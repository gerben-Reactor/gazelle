use crate::grammar::{Grammar, Symbol, SymbolId, SymbolTable};
use std::collections::{BTreeSet, HashMap, HashSet};

/// A bitset representing a set of terminals (including EOF at bit 0).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalSet {
    bits: Vec<u64>,
    /// Whether this set can derive epsilon (empty string).
    pub has_epsilon: bool,
}

impl TerminalSet {
    /// Create an empty terminal set.
    pub fn new(num_terminals: u32) -> Self {
        // +1 for EOF at position 0
        let num_bits = (num_terminals + 1) as usize;
        let num_words = num_bits.div_ceil(64);
        Self {
            bits: vec![0; num_words],
            has_epsilon: false,
        }
    }

    /// Insert a terminal ID into the set.
    pub fn insert(&mut self, id: SymbolId) -> bool {
        let idx = id.0 as usize;
        let word = idx / 64;
        let bit = idx % 64;
        if word < self.bits.len() {
            let mask = 1u64 << bit;
            let was_set = (self.bits[word] & mask) != 0;
            self.bits[word] |= mask;
            !was_set
        } else {
            false
        }
    }

    /// Check if a terminal ID is in the set.
    pub fn contains(&self, id: SymbolId) -> bool {
        let idx = id.0 as usize;
        let word = idx / 64;
        let bit = idx % 64;
        if word < self.bits.len() {
            (self.bits[word] & (1u64 << bit)) != 0
        } else {
            false
        }
    }

    /// Union this set with another, returning true if anything changed.
    pub fn union(&mut self, other: &TerminalSet) -> bool {
        let mut changed = false;
        for (w, &other_w) in self.bits.iter_mut().zip(other.bits.iter()) {
            let old = *w;
            *w |= other_w;
            if *w != old {
                changed = true;
            }
        }
        if other.has_epsilon && !self.has_epsilon {
            self.has_epsilon = true;
            changed = true;
        }
        changed
    }

    /// Iterate over all terminal IDs in the set.
    pub fn iter(&self) -> impl Iterator<Item = SymbolId> + '_ {
        self.bits.iter().enumerate().flat_map(|(word_idx, &word)| {
            (0..64).filter_map(move |bit| {
                if (word & (1u64 << bit)) != 0 {
                    Some(SymbolId((word_idx * 64 + bit) as u32))
                } else {
                    None
                }
            })
        })
    }
}

/// FIRST sets for all symbols, indexed by SymbolId.
#[derive(Debug, Clone)]
pub struct FirstSets {
    /// FIRST set for each symbol, indexed by symbol ID.
    sets: Vec<TerminalSet>,
    num_terminals: u32,
}

impl FirstSets {
    /// Compute FIRST sets for a grammar.
    pub fn compute(grammar: &Grammar) -> Self {
        let num_terminals = grammar.symbols.num_terminals();
        let num_symbols = grammar.symbols.num_symbols();

        let mut sets: Vec<TerminalSet> = (0..num_symbols)
            .map(|_| TerminalSet::new(num_terminals))
            .collect();

        // Initialize: terminals have FIRST = {self}
        for id in grammar.symbols.terminal_ids() {
            sets[id.0 as usize].insert(id);
        }

        // Fixed-point iteration for non-terminals
        let mut changed = true;
        while changed {
            changed = false;

            for rule in &grammar.rules {
                let lhs = rule.lhs.id();
                let rhs_ids: Vec<SymbolId> = rule.rhs.iter().map(|s| s.id()).collect();
                let rhs_first = Self::first_of_sequence(&rhs_ids, &sets, num_terminals, &grammar.symbols);

                // Add all terminals from rhs_first to sets[lhs]
                for id in rhs_first.iter() {
                    if sets[lhs.0 as usize].insert(id) {
                        changed = true;
                    }
                }
                if rhs_first.has_epsilon && !sets[lhs.0 as usize].has_epsilon {
                    sets[lhs.0 as usize].has_epsilon = true;
                    changed = true;
                }
            }
        }

        FirstSets { sets, num_terminals }
    }

    /// Compute FIRST of a sequence of symbols.
    fn first_of_sequence(
        symbols: &[SymbolId],
        sets: &[TerminalSet],
        num_terminals: u32,
        symbol_table: &SymbolTable,
    ) -> TerminalSet {
        let mut result = TerminalSet::new(num_terminals);

        if symbols.is_empty() {
            result.has_epsilon = true;
            return result;
        }

        for &sym in symbols {
            if symbol_table.is_terminal(sym) {
                // Terminal: add it and stop
                result.insert(sym);
                return result;
            }

            // Non-terminal: add its FIRST set (excluding epsilon)
            let sym_first = &sets[sym.0 as usize];
            for id in sym_first.iter() {
                result.insert(id);
            }

            // If this non-terminal can't derive epsilon, stop
            if !sym_first.has_epsilon {
                return result;
            }
        }

        // All symbols can derive epsilon
        result.has_epsilon = true;
        result
    }

    /// Get FIRST set for a symbol.
    pub fn get(&self, id: SymbolId) -> &TerminalSet {
        &self.sets[id.0 as usize]
    }

    /// Compute FIRST of a sequence followed by a lookahead.
    pub fn first_of_sequence_with_lookahead(
        &self,
        symbols: &[SymbolId],
        lookahead: SymbolId,
        symbol_table: &SymbolTable,
    ) -> TerminalSet {
        let mut result = TerminalSet::new(self.num_terminals);

        for &sym in symbols {
            if symbol_table.is_terminal(sym) {
                result.insert(sym);
                return result;
            }

            let sym_first = &self.sets[sym.0 as usize];
            for id in sym_first.iter() {
                result.insert(id);
            }

            if !sym_first.has_epsilon {
                return result;
            }
        }

        // All symbols can derive epsilon, add the lookahead
        result.insert(lookahead);
        result
    }
}

/// An LR(1) item: a rule with a dot position and lookahead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Item {
    /// Index of the rule in the grammar.
    pub rule: usize,
    /// Position of the dot (0 = before first symbol, len = after last).
    pub dot: usize,
    /// Lookahead terminal ID (EOF = SymbolId(0)).
    pub lookahead: SymbolId,
}

impl Item {
    pub fn new(rule: usize, dot: usize, lookahead: SymbolId) -> Self {
        Self { rule, dot, lookahead }
    }

    /// Returns the symbol immediately after the dot, if any.
    pub fn next_symbol(&self, grammar: &Grammar) -> Option<Symbol> {
        grammar.rules[self.rule].rhs.get(self.dot).copied()
    }

    /// Returns true if the dot is at the end (reduce item).
    pub fn is_complete(&self, grammar: &Grammar) -> bool {
        self.dot >= grammar.rules[self.rule].rhs.len()
    }

    /// Returns a new item with the dot advanced by one position.
    pub fn advance(&self) -> Self {
        Self {
            rule: self.rule,
            dot: self.dot + 1,
            lookahead: self.lookahead,
        }
    }
}

/// A set of LR(1) items representing a parser state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ItemSet {
    pub items: HashSet<Item>,
}

impl ItemSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_items(items: impl IntoIterator<Item = Item>) -> Self {
        Self { items: items.into_iter().collect() }
    }

    pub fn insert(&mut self, item: Item) -> bool {
        self.items.insert(item)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Item> {
        self.items.iter()
    }

    /// Returns the LR(0) core of this item set.
    pub fn core(&self) -> BTreeSet<(usize, usize)> {
        self.items.iter().map(|item| (item.rule, item.dot)).collect()
    }
}

/// Compute the closure of an item set.
pub fn closure(
    grammar: &Grammar,
    items: &ItemSet,
    first_sets: &FirstSets,
) -> ItemSet {
    let mut result = items.clone();
    let mut worklist: Vec<Item> = items.items.iter().copied().collect();

    while let Some(item) = worklist.pop() {
        let Some(next) = item.next_symbol(grammar) else { continue };

        if !next.is_non_terminal() {
            continue;
        }

        // Compute FIRST(β a) where β is everything after the non-terminal
        let beta: Vec<SymbolId> = grammar.rules[item.rule].rhs[item.dot + 1..]
            .iter()
            .map(|s| s.id())
            .collect();
        let lookaheads = first_sets.first_of_sequence_with_lookahead(
            &beta,
            item.lookahead,
            &grammar.symbols,
        );

        // Add items for each rule of the non-terminal
        for (rule_idx, _) in grammar.rules_for(next) {
            for la in lookaheads.iter() {
                let new_item = Item::new(rule_idx, 0, la);
                if result.insert(new_item) {
                    worklist.push(new_item);
                }
            }
        }
    }

    result
}

/// Compute the GOTO set: the closure of all items where we can advance past `symbol`.
pub fn goto(
    grammar: &Grammar,
    items: &ItemSet,
    symbol: Symbol,
    first_sets: &FirstSets,
) -> ItemSet {
    let mut kernel = ItemSet::new();

    for item in items.iter() {
        if item.next_symbol(grammar) == Some(symbol) {
            kernel.insert(item.advance());
        }
    }

    closure(grammar, &kernel, first_sets)
}

/// An LR(1) automaton: a collection of states with transitions.
#[derive(Debug)]
pub struct Automaton {
    /// The states (item sets) of the automaton.
    pub states: Vec<ItemSet>,
    /// Transitions: (from_state, symbol) -> to_state.
    pub transitions: HashMap<(usize, Symbol), usize>,
    /// The augmented grammar used to build this automaton.
    pub grammar: Grammar,
    /// Precomputed FIRST sets.
    pub first_sets: FirstSets,
}

impl Automaton {
    /// Build the LALR(1) automaton for a grammar.
    ///
    /// This builds states by LR(0) cores and merges lookaheads, producing
    /// an LALR(1) automaton which is smaller than canonical LR(1) but may
    /// have reduce-reduce conflicts that wouldn't exist in full LR(1).
    pub fn build(grammar: &Grammar) -> Self {
        let aug_grammar = grammar.augment();
        let first_sets = FirstSets::compute(&aug_grammar);

        // Initial state: closure of [__start -> • <original_start>, $]
        let initial_item = Item::new(0, 0, SymbolId::EOF);
        let initial_set = ItemSet::from_items([initial_item]);
        let state0 = closure(&aug_grammar, &initial_set, &first_sets);

        let mut states = vec![state0];
        let mut transitions = HashMap::new();
        let mut state_index: HashMap<BTreeSet<(usize, usize)>, usize> = HashMap::new();

        state_index.insert(states[0].core(), 0);

        let mut worklist = vec![0usize];

        while let Some(state_idx) = worklist.pop() {
            let state = states[state_idx].clone();

            // Collect all symbols we can transition on
            let symbols: HashSet<Symbol> = state.items.iter()
                .filter_map(|item| item.next_symbol(&aug_grammar))
                .collect();

            for symbol in symbols {
                let next_state = goto(&aug_grammar, &state, symbol, &first_sets);
                if next_state.is_empty() {
                    continue;
                }

                let next_core = next_state.core();

                let next_idx = if let Some(&idx) = state_index.get(&next_core) {
                    // LALR(1): merge lookaheads into existing state
                    let existing = &mut states[idx];
                    let mut merged_any = false;
                    for item in &next_state.items {
                        if existing.insert(*item) {
                            merged_any = true;
                        }
                    }
                    // If we added new lookaheads, reprocess this state
                    if merged_any && !worklist.contains(&idx) {
                        worklist.push(idx);
                    }
                    idx
                } else {
                    let idx = states.len();
                    state_index.insert(next_core, idx);
                    states.push(next_state);
                    worklist.push(idx);
                    idx
                };

                transitions.insert((state_idx, symbol), next_idx);
            }
        }

        Automaton {
            states,
            transitions,
            grammar: aug_grammar,
            first_sets,
        }
    }

    /// Returns the number of states in the automaton.
    pub fn num_states(&self) -> usize {
        self.states.len()
    }

    /// Returns the transition from a state on a symbol, if any.
    pub fn transition(&self, state: usize, symbol: Symbol) -> Option<usize> {
        self.transitions.get(&(state, symbol)).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::GrammarBuilder;

    fn expr_grammar() -> Grammar {
        let mut gb = GrammarBuilder::new();
        let plus = gb.t("+");
        let num = gb.t("NUM");
        let expr = gb.nt("expr");
        let term = gb.nt("term");

        gb.rule(expr, vec![expr, plus, term]);
        gb.rule(expr, vec![term]);
        gb.rule(term, vec![num]);

        gb.build()
    }

    #[test]
    fn test_terminal_set() {
        let mut set = TerminalSet::new(10);

        assert!(!set.contains(SymbolId(0)));
        assert!(!set.contains(SymbolId(5)));

        set.insert(SymbolId(0)); // EOF
        set.insert(SymbolId(5));

        assert!(set.contains(SymbolId(0)));
        assert!(set.contains(SymbolId(5)));
        assert!(!set.contains(SymbolId(3)));

        let ids: Vec<_> = set.iter().collect();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&SymbolId(0)));
        assert!(ids.contains(&SymbolId(5)));
    }

    #[test]
    fn test_first_sets() {
        let grammar = expr_grammar();
        let first_sets = FirstSets::compute(&grammar);

        // Get symbol IDs
        let num_id = grammar.symbols.get_id("NUM").unwrap();
        let term_id = grammar.symbols.get_id("term").unwrap();
        let expr_id = grammar.symbols.get_id("expr").unwrap();

        // FIRST(term) = {NUM}
        let term_first = first_sets.get(term_id);
        assert!(term_first.contains(num_id));

        // FIRST(expr) = {NUM}
        let expr_first = first_sets.get(expr_id);
        assert!(expr_first.contains(num_id));
    }

    #[test]
    fn test_item_next_symbol() {
        let grammar = expr_grammar();

        let expr = grammar.symbols.get("expr").unwrap();
        let plus = grammar.symbols.get("+").unwrap();

        // rule 0: expr -> expr '+' term
        let item = Item::new(0, 0, SymbolId::EOF);
        assert_eq!(item.next_symbol(&grammar), Some(expr));

        let item = Item::new(0, 1, SymbolId::EOF);
        assert_eq!(item.next_symbol(&grammar), Some(plus));

        let item = Item::new(0, 3, SymbolId::EOF);
        assert_eq!(item.next_symbol(&grammar), None);
        assert!(item.is_complete(&grammar));
    }

    #[test]
    fn test_closure() {
        let grammar = expr_grammar();
        let first = FirstSets::compute(&grammar);

        // Start with expr -> • expr '+' term, $
        let initial = ItemSet::from_items([Item::new(0, 0, SymbolId::EOF)]);
        let closed = closure(&grammar, &initial, &first);

        // Should include items for all expr and term rules
        assert!(closed.items.len() > 1);

        // Should have expr -> • term (rule 1)
        let has_expr_term = closed.items.iter().any(|item| {
            item.rule == 1 && item.dot == 0
        });
        assert!(has_expr_term);
    }

    #[test]
    fn test_goto() {
        let grammar = expr_grammar();
        let first = FirstSets::compute(&grammar);

        let num = grammar.symbols.get("NUM").unwrap();

        // rule 2: term -> 'NUM'
        let initial = ItemSet::from_items([Item::new(2, 0, SymbolId::EOF)]);
        let closed = closure(&grammar, &initial, &first);
        let after_num = goto(&grammar, &closed, num, &first);

        // Should have term -> NUM •
        let has_complete = after_num.items.iter().any(|item| {
            item.rule == 2 && item.dot == 1
        });
        assert!(has_complete);
    }

    #[test]
    fn test_automaton_construction() {
        let grammar = expr_grammar();
        let automaton = Automaton::build(&grammar);

        // Should have multiple states
        assert!(automaton.num_states() > 1);

        let expr = automaton.grammar.symbols.get("expr").unwrap();
        let term = automaton.grammar.symbols.get("term").unwrap();
        let num = automaton.grammar.symbols.get("NUM").unwrap();

        // State 0 should have transitions on expr, term, and NUM
        assert!(automaton.transition(0, expr).is_some());
        assert!(automaton.transition(0, term).is_some());
        assert!(automaton.transition(0, num).is_some());
    }

    #[test]
    fn test_automaton_simple() {
        let mut gb = GrammarBuilder::new();
        let a = gb.t("a");
        let s = gb.nt("S");
        gb.rule(s, vec![a]);
        let grammar = gb.build();

        let automaton = Automaton::build(&grammar);

        // Augmented: __start -> S, S -> 'a'
        // States: 3
        assert_eq!(automaton.num_states(), 3);

        let a_sym = automaton.grammar.symbols.get("a").unwrap();
        let s_sym = automaton.grammar.symbols.get("S").unwrap();

        assert!(automaton.transition(0, a_sym).is_some());
        assert!(automaton.transition(0, s_sym).is_some());
    }

    #[test]
    fn test_paren_grammar() {
        // Test that lookaheads are properly merged in LALR(1)
        let mut gb = GrammarBuilder::new();
        let num = gb.t("NUM");
        let lparen = gb.t("LPAREN");
        let rparen = gb.t("RPAREN");
        let expr = gb.nt("expr");

        // expr = NUM | LPAREN expr RPAREN
        gb.rule(expr, vec![num]);
        gb.rule(expr, vec![lparen, expr, rparen]);

        let grammar = gb.build();
        let automaton = Automaton::build(&grammar);

        // Build parse table
        use crate::table::ParseTable;
        let table = ParseTable::build(&automaton);

        assert!(!table.has_conflicts());

        let rparen_id = table.symbol_id("RPAREN").unwrap();
        let num_id = table.symbol_id("NUM").unwrap();

        // After shifting NUM inside parens, RPAREN should trigger a reduce
        // Find the state reached by shifting LPAREN then NUM
        let lparen_sym = table.symbol("LPAREN").unwrap();
        let num_sym = table.symbol("NUM").unwrap();

        let state_after_lparen = automaton.transition(0, lparen_sym).unwrap();
        let state_after_num = automaton.transition(state_after_lparen, num_sym).unwrap();

        // This state should have Reduce action for RPAREN
        match table.action(state_after_num, rparen_id) {
            crate::table::Action::Reduce(_) => {} // Good!
            other => panic!("Expected Reduce for RPAREN after LPAREN NUM, got {:?}", other),
        }
    }
}

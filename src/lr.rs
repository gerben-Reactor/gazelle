use crate::grammar::{Grammar, Symbol, SymbolId, InternedGrammar, SymbolTable};
use std::collections::{BTreeSet, HashMap, HashSet};

/// An LR(1) item: a rule with a dot position and lookahead.
///
/// For rule `A -> X Y Z` with dot at position 1: `A -> X • Y Z`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Item {
    /// Index of the rule in the grammar.
    pub rule: usize,
    /// Position of the dot (0 = before first symbol, len = after last).
    pub dot: usize,
    /// Lookahead terminal (None represents EOF/$).
    pub lookahead: Option<Symbol>,
}

impl Item {
    pub fn new(rule: usize, dot: usize, lookahead: Option<Symbol>) -> Self {
        Self { rule, dot, lookahead }
    }

    /// Returns the symbol immediately after the dot, if any.
    pub fn next_symbol<'a>(&self, grammar: &'a Grammar) -> Option<&'a Symbol> {
        grammar.rules[self.rule].rhs.get(self.dot)
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
            lookahead: self.lookahead.clone(),
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

    /// Returns the LR(0) core of this item set (items without lookahead).
    /// Used for comparing states in minimal LR construction.
    pub fn core(&self) -> BTreeSet<(usize, usize)> {
        self.items.iter().map(|item| (item.rule, item.dot)).collect()
    }
}

/// Compute the FIRST set for a sequence of symbols.
/// Returns the set of terminals that can begin strings derived from the sequence.
/// None in the result represents ε (empty).
pub fn first_of_sequence(
    _grammar: &Grammar,
    symbols: &[Symbol],
    first_sets: &HashMap<Symbol, HashSet<Option<Symbol>>>,
) -> HashSet<Option<Symbol>> {
    let mut result = HashSet::new();

    if symbols.is_empty() {
        result.insert(None); // ε
        return result;
    }

    for symbol in symbols {
        match symbol {
            Symbol::Terminal(_) | Symbol::PrecTerminal(_) => {
                result.insert(Some(symbol.clone()));
                return result; // Terminal doesn't derive ε
            }
            Symbol::NonTerminal(_) => {
                if let Some(first) = first_sets.get(symbol) {
                    for s in first {
                        if s.is_some() {
                            result.insert(s.clone());
                        }
                    }
                    // If this non-terminal can't derive ε, stop
                    if !first.contains(&None) {
                        return result;
                    }
                } else {
                    return result; // Unknown symbol, stop
                }
            }
        }
    }

    // All symbols can derive ε
    result.insert(None);
    result
}

/// Compute FIRST sets for all symbols in the grammar.
pub fn compute_first_sets(grammar: &Grammar) -> HashMap<Symbol, HashSet<Option<Symbol>>> {
    let mut first: HashMap<Symbol, HashSet<Option<Symbol>>> = HashMap::new();

    // Initialize: terminals have FIRST = {self}
    for rule in &grammar.rules {
        for symbol in &rule.rhs {
            if symbol.is_terminal() {
                first.entry(symbol.clone())
                    .or_default()
                    .insert(Some(symbol.clone()));
            }
        }
    }

    // Initialize non-terminals with empty sets
    for rule in &grammar.rules {
        first.entry(rule.lhs.clone()).or_default();
    }

    // Fixed-point iteration
    let mut changed = true;
    while changed {
        changed = false;

        for rule in &grammar.rules {
            let rhs_first = first_of_sequence(grammar, &rule.rhs, &first);
            let lhs_first = first.entry(rule.lhs.clone()).or_default();

            for s in rhs_first {
                if lhs_first.insert(s) {
                    changed = true;
                }
            }
        }
    }

    first
}

/// Compute the closure of an item set.
/// For each item `A -> α • B β, a` where B is a non-terminal,
/// add `B -> • γ, b` for each rule `B -> γ` and b in FIRST(βa).
pub fn closure(grammar: &Grammar, items: &ItemSet, first_sets: &HashMap<Symbol, HashSet<Option<Symbol>>>) -> ItemSet {
    let mut result = items.clone();
    let mut worklist: Vec<Item> = items.items.iter().cloned().collect();

    while let Some(item) = worklist.pop() {
        // Get symbol after the dot
        let Some(next) = item.next_symbol(grammar) else { continue };

        // Only process non-terminals
        if !next.is_non_terminal() {
            continue;
        }

        // Compute FIRST(β a) where β is everything after the non-terminal
        let rule = &grammar.rules[item.rule];
        let beta: Vec<Symbol> = rule.rhs[item.dot + 1..].to_vec();
        let mut beta_a = beta;
        if let Some(la) = &item.lookahead {
            beta_a.push(la.clone());
        }
        let lookaheads = first_of_sequence(grammar, &beta_a, first_sets);

        // Add items for each rule of the non-terminal
        for (rule_idx, _) in grammar.rules_for(next) {
            for la in &lookaheads {
                // Convert None (ε) to the original lookahead
                let new_la = if la.is_none() {
                    item.lookahead.clone()
                } else {
                    la.clone()
                };

                let new_item = Item::new(rule_idx, 0, new_la);
                if result.insert(new_item.clone()) {
                    worklist.push(new_item);
                }
            }
        }
    }

    result
}

/// Compute the GOTO set: the closure of all items where we can advance past `symbol`.
pub fn goto(grammar: &Grammar, items: &ItemSet, symbol: &Symbol, first_sets: &HashMap<Symbol, HashSet<Option<Symbol>>>) -> ItemSet {
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
    /// Rule 0 is always the augmented start rule: __start -> <original_start>
    pub grammar: Grammar,
}

impl Automaton {
    /// Build the canonical LR(1) automaton for a grammar.
    ///
    /// Automatically augments the grammar with: __start -> <original_start>
    /// This ensures there's a unique accept state (when rule 0 completes at EOF).
    pub fn build(grammar: &Grammar) -> Self {
        // Augment the grammar: add __start -> <original_start> as rule 0
        let aug_grammar = grammar.augment();
        let first_sets = compute_first_sets(&aug_grammar);

        // Initial state: closure of [__start -> • <original_start>, $]
        let initial_item = Item::new(0, 0, None);
        let initial_set = ItemSet::from_items([initial_item]);
        let state0 = closure(&aug_grammar, &initial_set, &first_sets);

        let mut states = vec![state0];
        let mut transitions = HashMap::new();
        let mut state_index: HashMap<BTreeSet<(usize, usize)>, usize> = HashMap::new();

        // Map first state by its core
        state_index.insert(states[0].core(), 0);

        let mut worklist = vec![0usize];

        while let Some(state_idx) = worklist.pop() {
            let state = states[state_idx].clone();

            // Collect all symbols we can transition on
            let symbols: HashSet<Symbol> = state.items.iter()
                .filter_map(|item| item.next_symbol(&aug_grammar).cloned())
                .collect();

            for symbol in symbols {
                let next_state = goto(&aug_grammar, &state, &symbol, &first_sets);
                if next_state.is_empty() {
                    continue;
                }

                let next_core = next_state.core();

                let next_idx = if let Some(&idx) = state_index.get(&next_core) {
                    // State with this core already exists
                    // For full LR(1), we need to check if the item sets are identical
                    // For now, we merge by core (this is LALR-like behavior)
                    // TODO: For minimal LR, we'll need to track and potentially split
                    idx
                } else {
                    // New state
                    let idx = states.len();
                    state_index.insert(next_core, idx);
                    states.push(next_state);
                    worklist.push(idx);
                    idx
                };

                transitions.insert((state_idx, symbol), next_idx);
            }
        }

        Automaton { states, transitions, grammar: aug_grammar }
    }

    /// Returns the number of states in the automaton.
    pub fn num_states(&self) -> usize {
        self.states.len()
    }

    /// Returns the transition from a state on a symbol, if any.
    pub fn transition(&self, state: usize, symbol: &Symbol) -> Option<usize> {
        self.transitions.get(&(state, symbol.clone())).copied()
    }
}

// ============================================================================
// Interned (integer-based) LR types for efficient parsing
// ============================================================================

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
    /// Compute FIRST sets for an interned grammar.
    pub fn compute(grammar: &InternedGrammar) -> Self {
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
                let lhs = rule.lhs;
                let rhs_first = Self::first_of_sequence(&rule.rhs, &sets, num_terminals, &grammar.symbols);

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

/// An LR(1) item using interned symbol IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InternedItem {
    /// Index of the rule in the grammar.
    pub rule: usize,
    /// Position of the dot (0 = before first symbol, len = after last).
    pub dot: usize,
    /// Lookahead terminal ID (EOF = SymbolId(0)).
    pub lookahead: SymbolId,
}

impl InternedItem {
    pub fn new(rule: usize, dot: usize, lookahead: SymbolId) -> Self {
        Self { rule, dot, lookahead }
    }

    /// Returns the symbol immediately after the dot, if any.
    pub fn next_symbol(&self, grammar: &InternedGrammar) -> Option<SymbolId> {
        grammar.rules[self.rule].rhs.get(self.dot).copied()
    }

    /// Returns true if the dot is at the end (reduce item).
    pub fn is_complete(&self, grammar: &InternedGrammar) -> bool {
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

/// A set of LR(1) items using interned symbols.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternedItemSet {
    pub items: HashSet<InternedItem>,
}

impl InternedItemSet {
    pub fn new() -> Self {
        Self { items: HashSet::new() }
    }

    pub fn from_items(items: impl IntoIterator<Item = InternedItem>) -> Self {
        Self { items: items.into_iter().collect() }
    }

    pub fn insert(&mut self, item: InternedItem) -> bool {
        self.items.insert(item)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &InternedItem> {
        self.items.iter()
    }

    /// Returns the LR(0) core of this item set.
    pub fn core(&self) -> BTreeSet<(usize, usize)> {
        self.items.iter().map(|item| (item.rule, item.dot)).collect()
    }
}

impl Default for InternedItemSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute the closure of an interned item set.
pub fn interned_closure(
    grammar: &InternedGrammar,
    items: &InternedItemSet,
    first_sets: &FirstSets,
) -> InternedItemSet {
    let mut result = items.clone();
    let mut worklist: Vec<InternedItem> = items.items.iter().copied().collect();

    while let Some(item) = worklist.pop() {
        let Some(next) = item.next_symbol(grammar) else { continue };

        if !grammar.symbols.is_non_terminal(next) {
            continue;
        }

        // Compute FIRST(β a) where β is everything after the non-terminal
        let beta = &grammar.rules[item.rule].rhs[item.dot + 1..];
        let lookaheads = first_sets.first_of_sequence_with_lookahead(
            beta,
            item.lookahead,
            &grammar.symbols,
        );

        // Add items for each rule of the non-terminal
        for (rule_idx, _) in grammar.rules_for(next) {
            for la in lookaheads.iter() {
                let new_item = InternedItem::new(rule_idx, 0, la);
                if result.insert(new_item) {
                    worklist.push(new_item);
                }
            }
        }
    }

    result
}

/// Compute the GOTO set for interned items.
pub fn interned_goto(
    grammar: &InternedGrammar,
    items: &InternedItemSet,
    symbol: SymbolId,
    first_sets: &FirstSets,
) -> InternedItemSet {
    let mut kernel = InternedItemSet::new();

    for item in items.iter() {
        if item.next_symbol(grammar) == Some(symbol) {
            kernel.insert(item.advance());
        }
    }

    interned_closure(grammar, &kernel, first_sets)
}

/// An LR(1) automaton using interned symbols.
#[derive(Debug)]
pub struct InternedAutomaton {
    /// The states (item sets) of the automaton.
    pub states: Vec<InternedItemSet>,
    /// Transitions: (from_state, symbol) -> to_state.
    pub transitions: HashMap<(usize, SymbolId), usize>,
    /// The augmented grammar used to build this automaton.
    pub grammar: InternedGrammar,
    /// Precomputed FIRST sets.
    pub first_sets: FirstSets,
}

impl InternedAutomaton {
    /// Build the canonical LR(1) automaton for an interned grammar.
    pub fn build(grammar: &InternedGrammar) -> Self {
        let aug_grammar = grammar.augment();
        let first_sets = FirstSets::compute(&aug_grammar);

        // Initial state: closure of [__start -> • <original_start>, $]
        let initial_item = InternedItem::new(0, 0, SymbolId::EOF);
        let initial_set = InternedItemSet::from_items([initial_item]);
        let state0 = interned_closure(&aug_grammar, &initial_set, &first_sets);

        let mut states = vec![state0];
        let mut transitions = HashMap::new();
        let mut state_index: HashMap<BTreeSet<(usize, usize)>, usize> = HashMap::new();

        state_index.insert(states[0].core(), 0);

        let mut worklist = vec![0usize];

        while let Some(state_idx) = worklist.pop() {
            let state = states[state_idx].clone();

            // Collect all symbols we can transition on
            let symbols: HashSet<SymbolId> = state.items.iter()
                .filter_map(|item| item.next_symbol(&aug_grammar))
                .collect();

            for symbol in symbols {
                let next_state = interned_goto(&aug_grammar, &state, symbol, &first_sets);
                if next_state.is_empty() {
                    continue;
                }

                let next_core = next_state.core();

                let next_idx = if let Some(&idx) = state_index.get(&next_core) {
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

        InternedAutomaton {
            states,
            transitions,
            grammar: aug_grammar,
            first_sets,
        }
    }

    /// Build from a string-based grammar.
    pub fn from_grammar(grammar: &Grammar) -> Self {
        let interned = grammar.intern();
        Self::build(&interned)
    }

    /// Returns the number of states in the automaton.
    pub fn num_states(&self) -> usize {
        self.states.len()
    }

    /// Returns the transition from a state on a symbol, if any.
    pub fn transition(&self, state: usize, symbol: SymbolId) -> Option<usize> {
        self.transitions.get(&(state, symbol)).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::{t, nt};
    use crate::meta::parse_grammar;

    fn expr_grammar() -> Grammar {
        parse_grammar(r#"
            expr = expr '+' term | term ;
            term = 'NUM' ;
        "#).unwrap()
    }

    #[test]
    fn test_item_next_symbol() {
        let grammar = expr_grammar();

        // rule 0: expr -> expr '+' term
        let item = Item::new(0, 0, None);
        assert_eq!(item.next_symbol(&grammar), Some(&nt("expr")));

        let item = Item::new(0, 1, None);
        assert_eq!(item.next_symbol(&grammar), Some(&t("+")));

        let item = Item::new(0, 3, None);
        assert_eq!(item.next_symbol(&grammar), None);
        assert!(item.is_complete(&grammar));
    }

    #[test]
    fn test_first_sets() {
        let grammar = expr_grammar();
        let first = compute_first_sets(&grammar);

        // FIRST(term) = {NUM}
        let term_first = first.get(&nt("term")).unwrap();
        assert!(term_first.contains(&Some(t("NUM"))));

        // FIRST(expr) = {NUM}
        let expr_first = first.get(&nt("expr")).unwrap();
        assert!(expr_first.contains(&Some(t("NUM"))));
    }

    #[test]
    fn test_closure() {
        let grammar = expr_grammar();
        let first = compute_first_sets(&grammar);

        // Start with expr -> • expr '+' term, $
        let initial = ItemSet::from_items([Item::new(0, 0, None)]);
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
        let first = compute_first_sets(&grammar);

        // rule 2: term -> 'NUM'
        let initial = ItemSet::from_items([Item::new(2, 0, None)]);
        let closed = closure(&grammar, &initial, &first);
        let after_num = goto(&grammar, &closed, &t("NUM"), &first);

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

        // State 0 should have transitions on expr, term, and NUM
        assert!(automaton.transition(0, &nt("expr")).is_some());
        assert!(automaton.transition(0, &nt("term")).is_some());
        assert!(automaton.transition(0, &t("NUM")).is_some());
    }

    #[test]
    fn test_automaton_simple() {
        let grammar = parse_grammar("S = 'a' ;").unwrap();
        let automaton = Automaton::build(&grammar);

        // Augmented: __start -> S, S -> 'a'
        // States:
        //   0: {__start -> • S, S -> • 'a'}
        //   1: {S -> 'a' •}
        //   2: {__start -> S •}
        assert_eq!(automaton.num_states(), 3);

        assert!(automaton.transition(0, &t("a")).is_some());
        assert!(automaton.transition(0, &nt("S")).is_some());
    }

    // Tests for interned LR types
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
    fn test_first_sets_interned() {
        let grammar = expr_grammar().intern();
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
    fn test_interned_automaton() {
        let grammar = expr_grammar();
        let automaton = InternedAutomaton::from_grammar(&grammar);

        // Should have the same number of states as the string-based automaton
        let string_automaton = Automaton::build(&grammar);
        assert_eq!(automaton.num_states(), string_automaton.num_states());

        // Check transitions exist
        let expr_id = automaton.grammar.symbols.get_id("expr").unwrap();
        let term_id = automaton.grammar.symbols.get_id("term").unwrap();
        let num_id = automaton.grammar.symbols.get_id("NUM").unwrap();

        assert!(automaton.transition(0, expr_id).is_some());
        assert!(automaton.transition(0, term_id).is_some());
        assert!(automaton.transition(0, num_id).is_some());
    }

    #[test]
    fn test_interned_automaton_simple() {
        let grammar = parse_grammar("S = 'a' ;").unwrap();
        let automaton = InternedAutomaton::from_grammar(&grammar);

        assert_eq!(automaton.num_states(), 3);

        let a_id = automaton.grammar.symbols.get_id("a").unwrap();
        let s_id = automaton.grammar.symbols.get_id("S").unwrap();

        assert!(automaton.transition(0, a_id).is_some());
        assert!(automaton.transition(0, s_id).is_some());
    }
}

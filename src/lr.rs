use crate::grammar::SymbolId;
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Convert snake_case or SCREAMING_SNAKE name to CamelCase type name.
/// e.g., "grammar_def" → "GrammarDef", "NAME" → "Name", "COMP_OP" → "CompOp"
pub(crate) fn to_camel_case(name: &str) -> String {
    name.split('_')
        .map(|seg| {
            let mut chars = seg.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
                None => String::new(),
            }
        })
        .collect()
}

// ============================================================================
// Internal grammar representation
// ============================================================================

/// A grammar symbol: terminal, precedence terminal, or non-terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Symbol {
    /// A regular terminal symbol.
    Terminal(SymbolId),
    /// A terminal that carries precedence at runtime (e.g., operators).
    PrecTerminal(SymbolId),
    /// A non-terminal symbol.
    NonTerminal(SymbolId),
}

impl Symbol {
    pub fn id(&self) -> SymbolId {
        match self {
            Symbol::Terminal(id) | Symbol::PrecTerminal(id) | Symbol::NonTerminal(id) => *id,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Symbol::Terminal(_) | Symbol::PrecTerminal(_))
    }

    pub fn is_non_terminal(&self) -> bool {
        matches!(self, Symbol::NonTerminal(_))
    }
}

/// Information about a terminal symbol.
#[derive(Debug, Clone)]
pub(crate) struct SymbolInfo {
    pub name: String,
    pub is_prec: bool,
}

/// Symbol table mapping names to IDs and vice versa.
#[derive(Debug, Clone)]
pub(crate) struct SymbolTable {
    /// Terminal info, indexed by id (0..num_terminals). EOF is at index 0.
    terminals: Vec<SymbolInfo>,
    /// Non-terminal names, indexed by id - num_terminals
    non_terminals: Vec<String>,
    /// Lookup from name to Symbol
    name_to_symbol: HashMap<String, Symbol>,
    /// Count of terminals (including EOF)
    num_terminals: u32,
}

impl SymbolTable {
    /// Create a new symbol table with EOF already interned as terminal 0.
    pub fn new() -> Self {
        let mut table = Self {
            terminals: Vec::new(),
            non_terminals: Vec::new(),
            name_to_symbol: HashMap::new(),
            num_terminals: 0,
        };
        // EOF is always terminal 0
        table.intern_terminal("$");
        table
    }

    /// Intern a terminal symbol, returning the Symbol.
    pub fn intern_terminal(&mut self, name: &str) -> Symbol {
        if let Some(&sym) = self.name_to_symbol.get(name) {
            return sym;
        }

        let id = SymbolId(self.terminals.len() as u32);
        self.terminals.push(SymbolInfo {
            name: name.to_string(),
            is_prec: false,
        });
        let sym = Symbol::Terminal(id);
        self.name_to_symbol.insert(name.to_string(), sym);
        sym
    }

    /// Intern a precedence terminal symbol, returning the Symbol.
    pub fn intern_prec_terminal(&mut self, name: &str) -> Symbol {
        if let Some(&sym) = self.name_to_symbol.get(name) {
            return sym;
        }

        let id = SymbolId(self.terminals.len() as u32);
        self.terminals.push(SymbolInfo {
            name: name.to_string(),
            is_prec: true,
        });
        let sym = Symbol::PrecTerminal(id);
        self.name_to_symbol.insert(name.to_string(), sym);
        sym
    }

    /// Intern a non-terminal symbol, returning the Symbol.
    pub fn intern_non_terminal(&mut self, name: &str) -> Symbol {
        if let Some(&sym) = self.name_to_symbol.get(name) {
            return sym;
        }

        let id = SymbolId(self.num_terminals + self.non_terminals.len() as u32);
        self.non_terminals.push(name.to_string());
        let sym = Symbol::NonTerminal(id);
        self.name_to_symbol.insert(name.to_string(), sym);
        sym
    }

    /// Finalize terminal interning. Call this after all terminals are added
    /// and before adding non-terminals.
    pub fn finalize_terminals(&mut self) {
        self.num_terminals = self.terminals.len() as u32;
    }

    /// Get the Symbol for a name, if it exists.
    pub fn get(&self, name: &str) -> Option<Symbol> {
        self.name_to_symbol.get(name).copied()
    }

    /// Get the SymbolId for a name, if it exists.
    pub fn get_id(&self, name: &str) -> Option<SymbolId> {
        self.name_to_symbol.get(name).map(|s| s.id())
    }

    /// Check if this is a terminal (including EOF).
    pub fn is_terminal(&self, id: SymbolId) -> bool {
        id.0 < self.num_terminals
    }

    /// Check if this terminal is a precedence terminal.
    pub fn is_prec_terminal(&self, id: SymbolId) -> bool {
        if id.0 >= self.num_terminals {
            return false;
        }
        self.terminals[id.0 as usize].is_prec
    }

    /// Get the name of a symbol.
    pub fn name(&self, id: SymbolId) -> &str {
        if id.0 < self.num_terminals {
            &self.terminals[id.0 as usize].name
        } else {
            let idx = (id.0 - self.num_terminals) as usize;
            &self.non_terminals[idx]
        }
    }

    /// Get the number of terminals (including EOF).
    pub fn num_terminals(&self) -> u32 {
        self.num_terminals
    }

    /// Get the number of non-terminals.
    pub fn num_non_terminals(&self) -> u32 {
        self.non_terminals.len() as u32
    }

    /// Get the total number of symbols.
    pub fn num_symbols(&self) -> u32 {
        self.num_terminals + self.num_non_terminals()
    }

    /// Iterate over all terminal IDs (including EOF at index 0).
    pub fn terminal_ids(&self) -> impl Iterator<Item = SymbolId> {
        (0..self.num_terminals).map(SymbolId)
    }

    /// Iterate over all non-terminal IDs.
    pub fn non_terminal_ids(&self) -> impl Iterator<Item = SymbolId> {
        let start = self.num_terminals;
        let end = start + self.non_terminals.len() as u32;
        (start..end).map(SymbolId)
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

/// The action to perform when a rule alternative is reduced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AltAction {
    /// User-defined action name (e.g., `=> binop`).
    Named(String),
    /// Synthetic: wrap value in `Some` (from `?` modifier).
    OptSome,
    /// Synthetic: produce `None` (from `?` modifier).
    OptNone,
    /// Synthetic: create empty `Vec` (from `*` modifier).
    VecEmpty,
    /// Synthetic: create `Vec` with single element (from `+`, `*`, `%` modifiers).
    VecSingle,
    /// Synthetic: append last element to `Vec` (from `+`, `*`, `%` modifiers).
    VecAppend,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Rule {
    pub lhs: Symbol,
    pub rhs: Vec<Symbol>,
    /// Action to perform when this rule is reduced.
    pub action: AltAction,
}

#[derive(Debug, Clone)]
pub(crate) struct GrammarInternal {
    pub rules: Vec<Rule>,
    pub symbols: SymbolTable,
    /// Type for each symbol (terminal payload or non-terminal result). None = unit type.
    pub types: BTreeMap<SymbolId, Option<String>>,
}

impl GrammarInternal {
    /// Returns all rules with the given non-terminal on the left-hand side.
    pub fn rules_for(&self, symbol: Symbol) -> impl Iterator<Item = (usize, &Rule)> {
        self.rules
            .iter()
            .enumerate()
            .filter(move |(_, rule)| rule.lhs == symbol)
    }
}

// ============================================================================
// Grammar conversion (AST -> Internal)
// ============================================================================

use crate::grammar::{Grammar, Term};

/// Convert Grammar AST to internal representation.
///
/// Desugars modifier symbols (?, *, +, %) into synthetic helper rules
/// with proper [`AltAction`]s, then builds the augmented grammar.
pub(crate) fn to_grammar_internal(grammar: &Grammar) -> Result<GrammarInternal, String> {
    if grammar.rules.is_empty() {
        return Err("Grammar has no rules".to_string());
    }

    let mut symbols = SymbolTable::new();
    let mut types: BTreeMap<SymbolId, Option<String>> = BTreeMap::new();

    // Register terminals + types
    for def in &grammar.terminals {
        let sym = if def.is_prec {
            symbols.intern_prec_terminal(&def.name)
        } else {
            symbols.intern_terminal(&def.name)
        };
        let type_name = if def.has_type {
            Some(to_camel_case(&def.name))
        } else {
            None
        };
        types.insert(sym.id(), type_name);
    }
    symbols.finalize_terminals();

    // Register user non-terminals + types
    // Every NT gets an auto-derived associated type from its name
    for rule in &grammar.rules {
        let sym = symbols.intern_non_terminal(&rule.name);
        types.insert(sym.id(), Some(to_camel_case(&rule.name)));
    }

    // Build rules, desugaring modifier terms inline
    let mut desugared: HashMap<Term, Symbol> = HashMap::new();
    let mut rules = Vec::new();

    for rule in &grammar.rules {
        let lhs = symbols.get(&rule.name).unwrap();

        for alt in &rule.alts {
            let has_empty = alt.terms.iter().any(|t| matches!(t, Term::Empty));

            let rhs: Vec<Symbol> = if has_empty {
                Vec::new()
            } else {
                alt.terms.iter().map(|term| {
                    resolve_term(term, &mut symbols, &mut types, &mut desugared, &mut rules)
                }).collect::<Result<Vec<_>, _>>()?
            };

            let action = AltAction::Named(alt.name.clone());

            rules.push(Rule { lhs, rhs, action });
        }
    }

    // Augment with __start -> <original_start>
    let start = symbols.get(&grammar.start)
        .ok_or_else(|| format!("Start symbol '{}' not found in grammar", grammar.start))?;
    let aug_start = symbols.intern_non_terminal("__start");
    let aug_rule = Rule {
        lhs: aug_start,
        rhs: vec![start],
        action: AltAction::Named(String::new()),
    };
    let mut aug_rules = vec![aug_rule];
    aug_rules.extend(rules);

    Ok(GrammarInternal {
        rules: aug_rules,
        symbols,
        types,
    })
}

fn resolve(symbols: &SymbolTable, name: &str) -> Result<Symbol, String> {
    symbols.get(name).ok_or_else(|| format!("Unknown symbol: {}", name))
}

fn resolve_term(
    term: &Term,
    symbols: &mut SymbolTable,
    types: &mut BTreeMap<SymbolId, Option<String>>,
    desugared: &mut HashMap<Term, Symbol>,
    rules: &mut Vec<Rule>,
) -> Result<Symbol, String> {
    if let Term::Symbol(name) = term {
        return resolve(symbols, name);
    }
    if let Some(&sym) = desugared.get(term) {
        return Ok(sym);
    }
    let lhs = match term {
        Term::Optional(name) => {
            let lhs = symbols.intern_non_terminal(&format!("__{}_opt", name.to_lowercase()));
            let inner = lookup_type(name, symbols, types);
            types.insert(lhs.id(), inner.map(|t| format!("Option<{}>", t)));
            let sym = resolve(symbols, name)?;
            rules.push(Rule { lhs, rhs: vec![sym], action: AltAction::OptSome });
            rules.push(Rule { lhs, rhs: vec![], action: AltAction::OptNone });
            lhs
        }
        Term::ZeroOrMore(name) => {
            let lhs = symbols.intern_non_terminal(&format!("__{}_star", name.to_lowercase()));
            let inner = lookup_type(name, symbols, types);
            types.insert(lhs.id(), inner.map(|t| format!("Vec<{}>", t)));
            let sym = resolve(symbols, name)?;
            rules.push(Rule { lhs, rhs: vec![lhs, sym], action: AltAction::VecAppend });
            rules.push(Rule { lhs, rhs: vec![], action: AltAction::VecEmpty });
            lhs
        }
        Term::OneOrMore(name) => {
            let lhs = symbols.intern_non_terminal(&format!("__{}_plus", name.to_lowercase()));
            let inner = lookup_type(name, symbols, types);
            types.insert(lhs.id(), inner.map(|t| format!("Vec<{}>", t)));
            let sym = resolve(symbols, name)?;
            rules.push(Rule { lhs, rhs: vec![lhs, sym], action: AltAction::VecAppend });
            rules.push(Rule { lhs, rhs: vec![sym], action: AltAction::VecSingle });
            lhs
        }
        Term::SeparatedBy { symbol, sep } => {
            let lhs = symbols.intern_non_terminal(
                &format!("__{}_sep_{}", symbol.to_lowercase(), sep.to_lowercase()));
            let inner = lookup_type(symbol, symbols, types);
            types.insert(lhs.id(), inner.map(|t| format!("Vec<{}>", t)));
            let sym = resolve(symbols, symbol)?;
            let sep_sym = resolve(symbols, sep)?;
            rules.push(Rule { lhs, rhs: vec![lhs, sep_sym, sym], action: AltAction::VecAppend });
            rules.push(Rule { lhs, rhs: vec![sym], action: AltAction::VecSingle });
            lhs
        }
        Term::Symbol(_) | Term::Empty => unreachable!(),
    };
    desugared.insert(term.clone(), lhs);
    Ok(lhs)
}

fn lookup_type(
    name: &str,
    symbols: &SymbolTable,
    types: &BTreeMap<SymbolId, Option<String>>,
) -> Option<String> {
    let id = symbols.get_id(name)?;
    let ty = types.get(&id)?;
    Some(ty.as_deref().unwrap_or("()").to_string())
}

// ============================================================================
// LR parsing
// ============================================================================

/// A bitset representing a set of terminals (including EOF at bit 0).
#[derive(Debug, Clone, PartialEq, Eq)]
struct TerminalSet {
    bits: Vec<u64>,
    /// Whether this set can derive epsilon (empty string).
    pub has_epsilon: bool,
}

impl TerminalSet {
    /// Create an empty terminal set.
    pub fn new(num_terminals: u32) -> Self {
        let num_bits = num_terminals as usize;
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

    #[cfg(test)]
    fn contains(&self, id: SymbolId) -> bool {
        let idx = id.0 as usize;
        let word = idx / 64;
        let bit = idx % 64;
        if word < self.bits.len() {
            (self.bits[word] & (1u64 << bit)) != 0
        } else {
            false
        }
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
struct FirstSets {
    /// FIRST set for each symbol, indexed by symbol ID.
    sets: Vec<TerminalSet>,
    num_terminals: u32,
}

impl FirstSets {
    /// Compute FIRST sets for a grammar.
    pub fn compute(grammar: &GrammarInternal) -> Self {
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

    #[cfg(test)]
    fn get(&self, id: SymbolId) -> &TerminalSet {
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

///// An LR(1) item: a rule with a dot position and lookahead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Item {
    /// Index of the rule in the grammar.
    pub(crate) rule: usize,
    /// Position of the dot (0 = before first symbol, len = after last).
    pub(crate) dot: usize,
    /// Lookahead terminal ID (EOF = SymbolId(0)).
    pub(crate) lookahead: SymbolId,
}

impl Item {
    fn new(rule: usize, dot: usize, lookahead: SymbolId) -> Self {
        Self { rule, dot, lookahead }
    }

    /// Returns the symbol immediately after the dot, if any.
    pub(crate) fn next_symbol(&self, grammar: &GrammarInternal) -> Option<Symbol> {
        grammar.rules[self.rule].rhs.get(self.dot).copied()
    }

    /// Returns true if the dot is at the end (reduce item).
    pub(crate) fn is_complete(&self, grammar: &GrammarInternal) -> bool {
        self.dot >= grammar.rules[self.rule].rhs.len()
    }

    /// Returns a new item with the dot advanced by one position.
    fn advance(&self) -> Self {
        Self {
            rule: self.rule,
            dot: self.dot + 1,
            lookahead: self.lookahead,
        }
    }
}

// ============================================================================
// NFA → DFA → Hopcroft minimization pipeline
// ============================================================================

/// NFA for LR(1) parsing.
/// States 0..num_items are item nodes (internal).
/// States num_items..num_items+num_rules are reduce/accept nodes.
struct Nfa {
    num_items: usize,
    items: Vec<Item>,
    /// Transitions: nfa_state -> [(symbol_id, target_nfa_state)]
    transitions: Vec<Vec<(u32, usize)>>,
    /// Epsilon transitions: nfa_state -> [target_nfa_state]
    epsilon: Vec<Vec<usize>>,
    /// Reverse mapping: virtual reduce ID -> real terminal ID
    reduce_to_real: HashMap<u32, u32>,
}

impl Nfa {
    fn is_reduce_node(&self, state: usize) -> bool {
        state >= self.num_items
    }

    fn reduce_rule(&self, state: usize) -> usize {
        debug_assert!(self.is_reduce_node(state));
        state - self.num_items
    }
}

fn build_nfa(grammar: &GrammarInternal, first_sets: &FirstSets) -> Nfa {
    let num_rules = grammar.rules.len();
    let num_terminals = grammar.symbols.num_terminals();
    let num_symbols = grammar.symbols.num_symbols();

    // Build prec terminal mapping: real ID -> virtual reduce ID
    let mut prec_to_reduce: Vec<Option<u32>> = vec![None; num_terminals as usize];
    let mut reduce_to_real: HashMap<u32, u32> = HashMap::new();
    let mut next_virtual = num_symbols;
    for id in grammar.symbols.terminal_ids() {
        if grammar.symbols.is_prec_terminal(id) {
            prec_to_reduce[id.0 as usize] = Some(next_virtual);
            reduce_to_real.insert(next_virtual, id.0);
            next_virtual += 1;
        }
    }
    // Phase 1: Enumerate all reachable items
    let mut item_index: HashMap<Item, usize> = HashMap::new();
    let mut items: Vec<Item> = Vec::new();

    fn intern(item: Item, items: &mut Vec<Item>, index: &mut HashMap<Item, usize>) -> usize {
        if let Some(&idx) = index.get(&item) {
            return idx;
        }
        let idx = items.len();
        index.insert(item, idx);
        items.push(item);
        idx
    }

    // Seed: (__start → • S, $)
    intern(Item::new(0, 0, SymbolId::EOF), &mut items, &mut item_index);

    // Discover all reachable items via closure + advance
    let mut i = 0;
    while i < items.len() {
        let item = items[i];
        i += 1;

        if item.is_complete(grammar) {
            continue;
        }

        // Advance past next symbol
        intern(item.advance(), &mut items, &mut item_index);

        // If next symbol is a non-terminal, add closure items
        let next_sym = item.next_symbol(grammar).unwrap();
        if next_sym.is_non_terminal() {
            let beta: Vec<SymbolId> = grammar.rules[item.rule].rhs[item.dot + 1..]
                .iter().map(|s| s.id()).collect();
            let lookaheads = first_sets.first_of_sequence_with_lookahead(
                &beta, item.lookahead, &grammar.symbols,
            );
            for (rule_idx, _) in grammar.rules_for(next_sym) {
                for la in lookaheads.iter() {
                    intern(Item::new(rule_idx, 0, la), &mut items, &mut item_index);
                }
            }
        }
    }

    let num_items = items.len();
    let total_states = num_items + num_rules;

    // Phase 2: Build transitions and epsilon edges
    let mut transitions: Vec<Vec<(u32, usize)>> = vec![Vec::new(); total_states];
    let mut epsilon: Vec<Vec<usize>> = vec![Vec::new(); total_states];

    for (idx, &item) in items.iter().enumerate() {
        if item.is_complete(grammar) {
            // Complete item: transition on lookahead to reduce node
            let la = item.lookahead;
            let reduce_node = num_items + item.rule;

            if let Some(&virtual_id) = prec_to_reduce.get(la.0 as usize).and_then(|x| x.as_ref()) {
                // Prec terminal: use virtual reduce ID
                transitions[idx].push((virtual_id, reduce_node));
            } else {
                transitions[idx].push((la.0, reduce_node));
            }
        } else {
            let next_sym = item.next_symbol(grammar).unwrap();
            let target = item_index[&item.advance()];

            if next_sym.is_terminal() {
                // Terminal: transition on the real symbol ID (shift flavor)
                transitions[idx].push((next_sym.id().0, target));
            } else {
                // Non-terminal: transition on NT id
                transitions[idx].push((next_sym.id().0, target));

                // Epsilon transitions for closure
                let beta: Vec<SymbolId> = grammar.rules[item.rule].rhs[item.dot + 1..]
                    .iter().map(|s| s.id()).collect();
                let lookaheads = first_sets.first_of_sequence_with_lookahead(
                    &beta, item.lookahead, &grammar.symbols,
                );
                for (rule_idx, _) in grammar.rules_for(next_sym) {
                    for la in lookaheads.iter() {
                        let closure_item = Item::new(rule_idx, 0, la);
                        epsilon[idx].push(item_index[&closure_item]);
                    }
                }
            }
        }
    }

    Nfa {
        num_items,
        items,
        transitions,
        epsilon,
        reduce_to_real,
    }
}

fn epsilon_closure(nfa: &Nfa, states: &BTreeSet<usize>) -> BTreeSet<usize> {
    let mut result = states.clone();
    let mut worklist: Vec<usize> = states.iter().copied().collect();

    while let Some(s) = worklist.pop() {
        if s < nfa.epsilon.len() {
            for &target in &nfa.epsilon[s] {
                if result.insert(target) {
                    worklist.push(target);
                }
            }
        }
    }

    result
}

/// Raw DFA from subset construction (before minimization).
struct RawDfa {
    /// Number of states
    num_states: usize,
    /// Transitions: state -> [(symbol_id, target_state)]
    transitions: Vec<Vec<(u32, usize)>>,
    /// For each state: reduce rules present (empty if pure internal state)
    reduce_rules: Vec<Vec<usize>>,
    /// Whether state has item nodes (non-reduce NFA states)
    has_items: Vec<bool>,
    /// NFA item indices per state (for error reporting)
    nfa_items: Vec<Vec<usize>>,
}

fn subset_construction(nfa: &Nfa) -> (RawDfa, Vec<crate::table::Conflict>) {
    let conflicts = Vec::new();

    // Initial DFA state: ε-closure of NFA state 0
    let initial: BTreeSet<usize> = [0].into_iter().collect();
    let initial_closed = epsilon_closure(nfa, &initial);

    let mut dfa_states: Vec<BTreeSet<usize>> = vec![initial_closed.clone()];
    let mut state_index: HashMap<BTreeSet<usize>, usize> = HashMap::new();
    state_index.insert(initial_closed, 0);

    let mut transitions: Vec<Vec<(u32, usize)>> = vec![Vec::new()];
    let mut worklist = vec![0usize];

    while let Some(dfa_idx) = worklist.pop() {
        let dfa_state = dfa_states[dfa_idx].clone();

        // Collect targets by symbol
        let mut symbol_targets: BTreeMap<u32, BTreeSet<usize>> = BTreeMap::new();
        for &nfa_state in &dfa_state {
            if nfa_state < nfa.transitions.len() {
                for &(sym, target) in &nfa.transitions[nfa_state] {
                    symbol_targets.entry(sym).or_default().insert(target);
                }
            }
        }

        for (sym, targets) in symbol_targets {
            let closed = epsilon_closure(nfa, &targets);
            if closed.is_empty() {
                continue;
            }

            let target_idx = if let Some(&idx) = state_index.get(&closed) {
                idx
            } else {
                let idx = dfa_states.len();
                state_index.insert(closed.clone(), idx);
                dfa_states.push(closed);
                transitions.push(Vec::new());
                worklist.push(idx);
                idx
            };

            transitions[dfa_idx].push((sym, target_idx));
        }
    }

    // Classify DFA states
    let num_states = dfa_states.len();
    let mut all_reduce_rules: Vec<Vec<usize>> = vec![Vec::new(); num_states];
    let mut has_items: Vec<bool> = vec![false; num_states];
    let mut nfa_items: Vec<Vec<usize>> = vec![Vec::new(); num_states];

    for (idx, state) in dfa_states.iter().enumerate() {
        let mut reduce_rules: Vec<usize> = Vec::new();
        let mut items = Vec::new();

        for &nfa_state in state {
            if nfa.is_reduce_node(nfa_state) {
                reduce_rules.push(nfa.reduce_rule(nfa_state));
            } else {
                items.push(nfa_state);
            }
        }

        reduce_rules.sort();
        reduce_rules.dedup();

        has_items[idx] = !items.is_empty();
        all_reduce_rules[idx] = reduce_rules;
        nfa_items[idx] = items;
    }

    (RawDfa { num_states, transitions, reduce_rules: all_reduce_rules, has_items, nfa_items }, conflicts)
}

/// Minimized DFA after Hopcroft partition refinement.
struct MinDfa {
    num_states: usize,
    transitions: Vec<Vec<(u32, usize)>>,
    reduce_rules: Vec<Vec<usize>>,
    has_items: Vec<bool>,
    /// Items per state (from representative of each partition)
    nfa_items: Vec<Vec<usize>>,
}

fn hopcroft_minimize(dfa: &RawDfa) -> MinDfa {
    // Initial partition:
    // - Each reduce state is its own singleton
    // - All internal states in one partition
    let mut partitions: Vec<Vec<usize>> = Vec::new();
    let mut state_to_partition: Vec<usize> = vec![0; dfa.num_states];

    // Initial partitions:
    // 1. Pure reduce states: grouped by rule
    // 2. Mixed states (items + reduces): grouped by reduce rule set
    //    (kept separate from pure-item states so Hopcroft doesn't merge away reduce info)
    // 3. Pure internal states (items only): one partition
    let mut reduce_partitions: HashMap<usize, usize> = HashMap::new();
    let mut mixed_partitions: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut internal_states: Vec<usize> = Vec::new();

    for state in 0..dfa.num_states {
        if !dfa.has_items[state] && !dfa.reduce_rules[state].is_empty() {
            // Pure reduce state — partition by rule
            let rule = dfa.reduce_rules[state][0];
            let partition = *reduce_partitions.entry(rule).or_insert_with(|| {
                let p = partitions.len();
                partitions.push(Vec::new());
                p
            });
            partitions[partition].push(state);
            state_to_partition[state] = partition;
        } else if dfa.has_items[state] && !dfa.reduce_rules[state].is_empty() {
            // Mixed state — partition by reduce rule set
            let partition = *mixed_partitions.entry(dfa.reduce_rules[state].clone()).or_insert_with(|| {
                let p = partitions.len();
                partitions.push(Vec::new());
                p
            });
            partitions[partition].push(state);
            state_to_partition[state] = partition;
        } else {
            internal_states.push(state);
        }
    }

    // All internal states in one partition
    if !internal_states.is_empty() {
        let p = partitions.len();
        for &s in &internal_states {
            state_to_partition[s] = p;
        }
        partitions.push(internal_states);
    }

    // Build reverse transition map: for each (symbol, target_partition),
    // which states have this transition?
    // Collect all symbols used
    let mut all_symbols: BTreeSet<u32> = BTreeSet::new();
    for row in &dfa.transitions {
        for &(sym, _) in row {
            all_symbols.insert(sym);
        }
    }

    // Hopcroft refinement loop
    let mut changed = true;
    while changed {
        changed = false;

        let num_partitions = partitions.len();
        let mut new_partitions: Vec<Vec<usize>> = Vec::new();
        let mut new_state_to_partition: Vec<usize> = vec![0; dfa.num_states];

        for partition in &partitions {
            if partition.len() <= 1 {
                let p = new_partitions.len();
                for &s in partition {
                    new_state_to_partition[s] = p;
                }
                new_partitions.push(partition.clone());
                continue;
            }

            // Try to split this partition based on transition signatures
            let mut signature_groups: BTreeMap<Vec<(u32, usize)>, Vec<usize>> = BTreeMap::new();

            for &state in partition {
                // Build signature: for each symbol, which partition does it go to?
                let mut sig: Vec<(u32, usize)> = Vec::new();
                for &(sym, target) in &dfa.transitions[state] {
                    sig.push((sym, state_to_partition[target]));
                }
                sig.sort();
                signature_groups.entry(sig).or_default().push(state);
            }

            if signature_groups.len() > 1 {
                changed = true;
            }

            for (_, group) in signature_groups {
                let p = new_partitions.len();
                for &s in &group {
                    new_state_to_partition[s] = p;
                }
                new_partitions.push(group);
            }
        }

        partitions = new_partitions;
        state_to_partition = new_state_to_partition;

        if partitions.len() == num_partitions && !changed {
            break;
        }
    }

    // Build minimized DFA
    let num_min_states = partitions.len();
    let mut min_transitions: Vec<Vec<(u32, usize)>> = vec![Vec::new(); num_min_states];
    let mut min_reduce_rules: Vec<Vec<usize>> = vec![Vec::new(); num_min_states];
    let mut min_has_items: Vec<bool> = vec![false; num_min_states];
    let mut min_nfa_items: Vec<Vec<usize>> = vec![Vec::new(); num_min_states];

    for (p_idx, partition) in partitions.iter().enumerate() {
        let representative = partition[0];

        min_reduce_rules[p_idx] = dfa.reduce_rules[representative].clone();
        min_has_items[p_idx] = dfa.has_items[representative];
        min_nfa_items[p_idx] = dfa.nfa_items[representative].clone();

        // Transitions (mapped to partition indices)
        for &(sym, target) in &dfa.transitions[representative] {
            min_transitions[p_idx].push((sym, state_to_partition[target]));
        }
    }

    // Ensure state 0 is the initial state
    let initial_partition = state_to_partition[0];
    if initial_partition != 0 {
        // Swap partitions 0 and initial_partition
        min_transitions.swap(0, initial_partition);
        min_reduce_rules.swap(0, initial_partition);
        min_has_items.swap(0, initial_partition);
        min_nfa_items.swap(0, initial_partition);

        // Update all transition targets
        for row in &mut min_transitions {
            for (_, target) in row.iter_mut() {
                if *target == 0 {
                    *target = initial_partition;
                } else if *target == initial_partition {
                    *target = 0;
                }
            }
        }
    }

    MinDfa {
        num_states: num_min_states,
        transitions: min_transitions,
        reduce_rules: min_reduce_rules,
        has_items: min_has_items,
        nfa_items: min_nfa_items,
    }
}

use crate::runtime::OpEntry;

/// Result of the automaton construction pipeline.
pub(crate) struct AutomatonResult {
    pub action_rows: Vec<Vec<(u32, OpEntry)>>,
    pub goto_rows: Vec<Vec<(u32, u32)>>,
    pub num_states: usize,
    pub state_items: Vec<Vec<(u16, u8)>>,
    pub state_symbols: Vec<u32>,
    pub conflicts: Vec<crate::table::Conflict>,
}

/// Build action/goto rows from the minimized DFA, merging prec terminal columns.
fn build_table_from_dfa(
    dfa: &MinDfa,
    nfa: &Nfa,
    grammar: &GrammarInternal,
    mut conflicts: Vec<crate::table::Conflict>,
) -> AutomatonResult {
    let num_terminals = grammar.symbols.num_terminals();
    let num_states = dfa.num_states;

    // Internal states go into the parse table. A state is internal if it has items.
    // Pure reduce states (no items, exactly one rule) are not table states.
    // Mixed states (items + reduces) are internal but also carry reduce info.
    let mut internal_states: Vec<usize> = Vec::new();
    let mut dfa_to_table: Vec<Option<usize>> = vec![None; num_states];

    for state in 0..num_states {
        if dfa.has_items[state] {
            let table_idx = internal_states.len();
            dfa_to_table[state] = Some(table_idx);
            internal_states.push(state);
        }
    }

    let num_table_states = internal_states.len();
    let mut action_rows: Vec<Vec<(u32, OpEntry)>> = vec![Vec::new(); num_table_states];
    let mut goto_rows: Vec<Vec<(u32, u32)>> = vec![Vec::new(); num_table_states];

    for (table_idx, &dfa_state) in internal_states.iter().enumerate() {
        // Track prec terminal actions for merging
        let mut prec_shifts: HashMap<u32, usize> = HashMap::new(); // real_id -> shift_state
        let mut prec_reduces: HashMap<u32, usize> = HashMap::new(); // real_id -> reduce_rule

        for &(sym, target) in &dfa.transitions[dfa_state] {
            // Check if this is a virtual reduce ID
            if let Some(&real_id) = nfa.reduce_to_real.get(&sym) {
                // Virtual prec reduce terminal — target should be a pure reduce state
                if let Some(&rule) = dfa.reduce_rules[target].first() {
                    prec_reduces.insert(real_id, rule);
                }
                continue;
            }

            let is_terminal = sym < num_terminals;
            let is_nt = sym >= num_terminals && sym < grammar.symbols.num_symbols();

            // Emit reduces for any reduce rules in the target
            if is_terminal {
                for &rule in &dfa.reduce_rules[target] {
                    insert_action_simple(
                        &mut action_rows[table_idx],
                        &mut conflicts,
                        table_idx,
                        sym,
                        OpEntry::reduce(rule),
                    );
                }
            }

            // Emit shift/goto if the target has items (is an internal state)
            if let Some(target_table) = dfa_to_table[target] {
                if is_terminal {
                    if grammar.symbols.is_prec_terminal(SymbolId(sym)) {
                        prec_shifts.insert(sym, target_table);
                    } else {
                        insert_action_simple(
                            &mut action_rows[table_idx],
                            &mut conflicts,
                            table_idx,
                            sym,
                            OpEntry::shift(target_table),
                        );
                    }
                } else if is_nt {
                    let nt_col = sym - num_terminals;
                    if !goto_rows[table_idx].iter().any(|(c, _)| *c == nt_col) {
                        goto_rows[table_idx].push((nt_col, target_table as u32));
                    }
                }
            }
        }

        // Merge prec terminal columns
        let prec_ids: BTreeSet<u32> = prec_shifts.keys().chain(prec_reduces.keys()).copied().collect();
        for real_id in prec_ids {
            match (prec_shifts.get(&real_id), prec_reduces.get(&real_id)) {
                (Some(&shift_state), Some(&reduce_rule)) => {
                    action_rows[table_idx].push((real_id, OpEntry::shift_or_reduce(shift_state, reduce_rule)));
                }
                (Some(&shift_state), None) => {
                    action_rows[table_idx].push((real_id, OpEntry::shift(shift_state)));
                }
                (None, Some(&reduce_rule)) => {
                    action_rows[table_idx].push((real_id, OpEntry::reduce(reduce_rule)));
                }
                (None, None) => unreachable!(),
            }
        }
    }

    // Build state items for error reporting
    let state_items: Vec<Vec<(u16, u8)>> = internal_states.iter().map(|&dfa_state| {
        dfa.nfa_items[dfa_state].iter().map(|&nfa_idx| {
            let item = &nfa.items[nfa_idx];
            (item.rule as u16, item.dot as u8)
        }).collect()
    }).collect();

    // Compute state symbols
    let state_symbols = compute_state_symbols(
        &action_rows, &goto_rows, num_table_states, num_terminals,
    );

    AutomatonResult {
        action_rows,
        goto_rows,
        num_states: num_table_states,
        state_items,
        state_symbols,
        conflicts,
    }
}

fn insert_action_simple(
    row: &mut Vec<(u32, OpEntry)>,
    conflicts: &mut Vec<crate::table::Conflict>,
    state: usize,
    col: u32,
    new_action: OpEntry,
) {
    if let Some(entry) = row.iter_mut().find(|(c, _)| *c == col) {
        if entry.1 != new_action {
            use crate::runtime::ParserOp;
            match (new_action.decode(), entry.1.decode()) {
                (ParserOp::Shift(s), ParserOp::Reduce(r))
                | (ParserOp::Reduce(r), ParserOp::Shift(s)) => {
                    conflicts.push(crate::table::Conflict::ShiftReduce {
                        state,
                        terminal: SymbolId(col),
                        shift_state: s,
                        reduce_rule: r,
                    });
                    entry.1 = OpEntry::shift(s);
                }
                (ParserOp::Reduce(r1), ParserOp::Reduce(r2)) => {
                    conflicts.push(crate::table::Conflict::ReduceReduce {
                        state,
                        terminal: SymbolId(col),
                        rule1: r1,
                        rule2: r2,
                    });
                }
                _ => {}
            }
        }
    } else {
        row.push((col, new_action));
    }
}

fn compute_state_symbols(
    action_rows: &[Vec<(u32, OpEntry)>],
    goto_rows: &[Vec<(u32, u32)>],
    num_states: usize,
    num_terminals: u32,
) -> Vec<u32> {
    use crate::runtime::ParserOp;

    let mut state_symbols = vec![0u32; num_states];

    for row in action_rows {
        for &(col, entry) in row {
            match entry.decode() {
                ParserOp::Shift(target) => state_symbols[target] = col,
                ParserOp::ShiftOrReduce { shift_state, .. } => state_symbols[shift_state] = col,
                _ => {}
            }
        }
    }

    for row in goto_rows {
        for &(col, target) in row {
            let nt_id = num_terminals + col;
            state_symbols[target as usize] = nt_id;
        }
    }

    state_symbols
}

/// Build a minimal LR(1) automaton for a grammar using NFA → DFA → Hopcroft.
pub(crate) fn build_minimal_automaton(grammar: &GrammarInternal) -> AutomatonResult {
    let first_sets = FirstSets::compute(grammar);
    let nfa = build_nfa(grammar, &first_sets);
    let (raw_dfa, conflicts) = subset_construction(&nfa);
    let min_dfa = hopcroft_minimize(&raw_dfa);
    build_table_from_dfa(&min_dfa, &nfa, grammar, conflicts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::parse_grammar;

    fn expr_grammar() -> GrammarInternal {
        to_grammar_internal(&parse_grammar(r#"
            start expr;
            terminals { PLUS, NUM }
            expr = expr PLUS term => add | term => term;
            term = NUM => num;
        "#).unwrap()).unwrap()
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
        let plus = grammar.symbols.get("PLUS").unwrap();

        // rule 0: __start -> expr (augmented)
        // rule 1: expr -> expr PLUS term
        let item = Item::new(1, 0, SymbolId::EOF);
        assert_eq!(item.next_symbol(&grammar), Some(expr));

        let item = Item::new(1, 1, SymbolId::EOF);
        assert_eq!(item.next_symbol(&grammar), Some(plus));

        let item = Item::new(1, 3, SymbolId::EOF);
        assert_eq!(item.next_symbol(&grammar), None);
        assert!(item.is_complete(&grammar));
    }

    #[test]
    fn test_paren_grammar() {
        let grammar = to_grammar_internal(&parse_grammar(r#"
            start expr;
            terminals { NUM, LPAREN, RPAREN }
            expr = NUM => num | LPAREN expr RPAREN => paren;
        "#).unwrap()).unwrap();

        use crate::table::CompiledTable;
        let compiled = CompiledTable::build_from_internal(&grammar);

        assert!(!compiled.has_conflicts());
    }
}

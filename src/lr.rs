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
}

// ============================================================================
// NFA → DFA → Hopcroft minimization pipeline
// ============================================================================

use crate::automaton::{self, Dfa};

/// LR-specific metadata kept alongside the generic NFA.
/// NFA states 0..items.len() are item nodes.
/// NFA states items.len()..items.len()+num_rules are reduce/accept nodes.
struct LrNfaInfo {
    items: Vec<Item>,
    /// Reverse mapping: virtual reduce ID -> real terminal ID
    reduce_to_real: HashMap<u32, u32>,
}

/// LR-specific metadata derived from DFA state classification.
struct DfaLrInfo {
    /// For each DFA state: reduce rules present (empty if pure item state)
    reduce_rules: Vec<Vec<usize>>,
    /// NFA item indices per state (for error reporting)
    nfa_items: Vec<Vec<usize>>,
}

impl DfaLrInfo {
    fn has_items(&self, state: usize) -> bool {
        !self.nfa_items[state].is_empty()
    }
}

/// Classify DFA states by inspecting which NFA states they contain.
fn classify_dfa_states(nfa_sets: &[Vec<usize>], num_items: usize) -> DfaLrInfo {
    let mut reduce_rules = vec![Vec::new(); nfa_sets.len()];
    let mut nfa_items = vec![Vec::new(); nfa_sets.len()];

    for (idx, nfa_set) in nfa_sets.iter().enumerate() {
        for &nfa_state in nfa_set {
            if nfa_state >= num_items {
                reduce_rules[idx].push(nfa_state - num_items);
            } else {
                nfa_items[idx].push(nfa_state);
            }
        }
        reduce_rules[idx].sort();
        reduce_rules[idx].dedup();
    }

    DfaLrInfo { reduce_rules, nfa_items }
}

/// Build prec terminal mapping: real terminal ID → virtual reduce symbol ID.
/// Returns (prec_to_reduce, reduce_to_real).
fn build_prec_mapping(grammar: &GrammarInternal) -> (Vec<Option<u32>>, HashMap<u32, u32>) {
    let num_terminals = grammar.symbols.num_terminals() as usize;
    let mut prec_to_reduce: Vec<Option<u32>> = vec![None; num_terminals];
    let mut reduce_to_real: HashMap<u32, u32> = HashMap::new();
    let mut next_virtual = grammar.symbols.num_symbols();
    for id in grammar.symbols.terminal_ids() {
        if grammar.symbols.is_prec_terminal(id) {
            prec_to_reduce[id.0 as usize] = Some(next_virtual);
            reduce_to_real.insert(next_virtual, id.0);
            next_virtual += 1;
        }
    }
    (prec_to_reduce, reduce_to_real)
}

/// Build LR(1) NFA by enumerating all (rule, dot, lookahead) triples.
/// Unreachable states are harmless — subset construction ignores them.
fn build_lr_nfa(grammar: &GrammarInternal, first_sets: &FirstSets) -> (automaton::Nfa, LrNfaInfo) {
    let num_rules = grammar.rules.len();
    let num_terminals = grammar.symbols.num_terminals();
    let (prec_to_reduce, reduce_to_real) = build_prec_mapping(grammar);

    // Compute flat index: item(rule, dot, la) and total item count
    let mut rule_offsets: Vec<usize> = Vec::with_capacity(num_rules);
    let mut num_items = 0;
    for rule in &grammar.rules {
        rule_offsets.push(num_items);
        num_items += (rule.rhs.len() + 1) * num_terminals as usize;
    }

    let item_state = |rule: usize, dot: usize, la: u32| -> usize {
        rule_offsets[rule] + dot * num_terminals as usize + la as usize
    };

    // Build items list for later classification
    let mut items: Vec<Item> = vec![Item::new(0, 0, SymbolId::EOF); num_items];
    for (rule_idx, rule) in grammar.rules.iter().enumerate() {
        for dot in 0..=rule.rhs.len() {
            for la in 0..num_terminals {
                let idx = item_state(rule_idx, dot, la);
                items[idx] = Item::new(rule_idx, dot, SymbolId(la));
            }
        }
    }

    let mut nfa = automaton::Nfa::new();
    // Pre-allocate: item states + reduce nodes
    for _ in 0..(num_items + num_rules) {
        nfa.add_state();
    }

    for (rule_idx, rule) in grammar.rules.iter().enumerate() {
        for dot in 0..=rule.rhs.len() {
            for la in 0..num_terminals {
                let idx = item_state(rule_idx, dot, la);

                if dot == rule.rhs.len() {
                    // Complete: transition on lookahead to reduce node
                    let reduce_node = num_items + rule_idx;
                    let sym = prec_to_reduce.get(la as usize)
                        .and_then(|x| *x)
                        .unwrap_or(la);
                    nfa.add_transition(idx, sym, reduce_node);
                } else {
                    let next_sym = rule.rhs[dot];
                    let advanced = item_state(rule_idx, dot + 1, la);
                    nfa.add_transition(idx, next_sym.id().0, advanced);

                    if next_sym.is_non_terminal() {
                        let beta: Vec<SymbolId> = rule.rhs[dot + 1..]
                            .iter().map(|s| s.id()).collect();
                        let lookaheads = first_sets.first_of_sequence_with_lookahead(
                            &beta, SymbolId(la), &grammar.symbols,
                        );
                        for (closure_rule, _) in grammar.rules_for(next_sym) {
                            for closure_la in lookaheads.iter() {
                                let target = item_state(closure_rule, 0, closure_la.0);
                                nfa.add_epsilon(idx, target);
                            }
                        }
                    }
                }
            }
        }
    }

    (nfa, LrNfaInfo { items, reduce_to_real })
}

/// Detect conflicts in the DFA without resolving them.
/// Returns conflict info with raw DFA state indices.
fn detect_conflicts(
    dfa: &Dfa,
    lr: &DfaLrInfo,
    nfa_info: &LrNfaInfo,
    grammar: &GrammarInternal,
) -> Vec<(usize, SymbolId, ConflictKind)> {
    let num_terminals = grammar.symbols.num_terminals() as u32;
    let mut conflicts = Vec::new();

    for source in 0..dfa.num_states() {
        if !lr.has_items(source) { continue; }
        for &(sym, target) in &dfa.transitions[source] {
            if sym >= num_terminals || nfa_info.reduce_to_real.contains_key(&sym) {
                continue;
            }
            if lr.has_items(target) && !lr.reduce_rules[target].is_empty() {
                for &rule in &lr.reduce_rules[target] {
                    conflicts.push((source, SymbolId(sym), ConflictKind::ShiftReduce(rule)));
                }
            }
            if lr.reduce_rules[target].len() > 1 {
                let rules = &lr.reduce_rules[target];
                for i in 1..rules.len() {
                    conflicts.push((source, SymbolId(sym), ConflictKind::ReduceReduce(rules[0], rules[i])));
                }
            }
        }
    }

    conflicts
}

/// Resolved DFA state: either a reduce state (single rule) or an item state.
#[derive(Clone)]
enum DfaStateKind {
    Reduce(usize),
    /// Items as (rule, dot) pairs.
    Items(Vec<(usize, usize)>),
}

/// Resolve conflicts and classify each DFA state:
/// - SR (mixed states with items + reduces): shift wins → Items
/// - RR (multiple reduces): lower rule wins → Reduce(winner)
/// - Pure reduce → Reduce(rule)
/// - Pure items → Items(nfa_items)
fn resolve_conflicts(lr: DfaLrInfo, nfa_info: &LrNfaInfo) -> Vec<DfaStateKind> {
    lr.reduce_rules.into_iter().zip(lr.nfa_items).map(|(mut reduces, nfa_items)| {
        if !nfa_items.is_empty() {
            // SR: shift wins
            let items = nfa_items.iter()
                .map(|&idx| (nfa_info.items[idx].rule, nfa_info.items[idx].dot))
                .collect();
            DfaStateKind::Items(items)
        } else if !reduces.is_empty() {
            // Pure reduce or RR: keep lowest-numbered rule
            reduces.sort();
            DfaStateKind::Reduce(reduces[0])
        } else {
            // Dead state (shouldn't normally happen)
            DfaStateKind::Items(Vec::new())
        }
    }).collect()
}

enum ConflictKind {
    ShiftReduce(usize),
    ReduceReduce(usize, usize),
}

/// Generate example input strings that demonstrate each conflict.
///
/// Works on the raw DFA before Hopcroft minimization. For each conflict,
/// BFS from state 0 to the conflict state to find the shortest viable prefix,
/// then shows how the same input can be parsed two ways.
fn conflict_examples(
    dfa: &Dfa,
    lr: &DfaLrInfo,
    nfa_info: &LrNfaInfo,
    grammar: &GrammarInternal,
    conflicts: Vec<(usize, SymbolId, ConflictKind)>,
) -> Vec<crate::table::Conflict> {
    // BFS from state 0 to find shortest path (grammar symbols) to each state.
    // Only follow transitions on real symbols between item states.
    let mut parent: Vec<Option<(usize, u32)>> = vec![None; dfa.num_states()];
    let mut visited = vec![false; dfa.num_states()];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0usize);
    visited[0] = true;

    while let Some(state) = queue.pop_front() {
        if !lr.has_items(state) { continue; }
        for &(sym, target) in &dfa.transitions[state] {
            // Skip virtual reduce symbols
            if nfa_info.reduce_to_real.contains_key(&sym) { continue; }
            // Skip reduce-only targets — follow item states
            if !lr.has_items(target) { continue; }
            if visited[target] { continue; }
            visited[target] = true;
            parent[target] = Some((state, sym));
            queue.push_back(target);
        }
    }

    // Reconstruct path from state 0 to a given state
    let path_to = |target: usize| -> Vec<u32> {
        let mut path = Vec::new();
        let mut s = target;
        while let Some((prev, sym)) = parent[s] {
            path.push(sym);
            s = prev;
        }
        path.reverse();
        path
    };

    let sym_name = |id: u32| -> &str {
        grammar.symbols.name(SymbolId(id))
    };

    let mut results = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for (source, terminal, kind) in &conflicts {
        let key = match kind {
            ConflictKind::ShiftReduce(rule) => (terminal.0, 0u8, *rule, 0),
            ConflictKind::ReduceReduce(r1, r2) => (terminal.0, 1, *r1, *r2),
        };
        if !seen.insert(key) { continue; }

        let prefix = path_to(*source);
        let prefix_str: Vec<&str> = prefix.iter().map(|&s| sym_name(s)).collect();
        let t_name = sym_name(terminal.0);

        match kind {
            ConflictKind::ShiftReduce(reduce_rule) => {
                let rule = &grammar.rules[*reduce_rule];
                let rhs_len = rule.rhs.len();
                let lhs_name = sym_name(rule.lhs.id().0);

                // The prefix is the viable prefix to the conflict state.
                // The last `rhs_len` symbols of the prefix form the RHS of the reduce rule.
                // The conflict terminal T follows.
                //
                // Shift:  prefix[..start] (prefix[start..] T suffix)
                // Reduce: prefix[..start] (prefix[start..]) T suffix
                let reduce_start = prefix.len().saturating_sub(rhs_len);

                // Build parser configs for both interpretations
                let sim = ParserSim::new(dfa, lr, nfa_info, grammar);
                let base_cfg = match sim.replay_prefix(&prefix) {
                    Some(c) => c,
                    None => continue,
                };

                // Shift config: shift T from conflict state
                let shift_cfg = match sim.shift_config(&base_cfg, terminal.0) {
                    Some(c) => c,
                    None => continue,
                };

                // Reduce config: reduce by conflict rule, then shift T
                let reduced_cfg = match sim.apply_reduce(&base_cfg, *reduce_rule) {
                    Some(c) => c,
                    None => continue,
                };
                let reduce_then_shift_cfg = match sim.shift_config(&reduced_cfg, terminal.0) {
                    Some(c) => c,
                    None => reduced_cfg.clone(),
                };

                // Joint BFS: find suffix valid for both interpretations
                let joint = find_joint_suffix(&sim, &shift_cfg, &reduce_then_shift_cfg);

                let example = if let Some(suffix) = joint {
                    // Unifying example: one string, two parses
                    let suffix_str: Vec<&str> = suffix.iter().map(|&s| sym_name(s)).collect();

                    // Input with dot at conflict point
                    let mut input_parts: Vec<String> = prefix_str.iter().map(|s| s.to_string()).collect();
                    input_parts.push(format!("\u{2022} {}", t_name));
                    for s in &suffix_str { input_parts.push(s.to_string()); }
                    let input_str = input_parts.join(" ");

                    // Shift bracketing
                    let shift_start = prefix.len().saturating_sub(1);
                    let mut shift_parts: Vec<String> = prefix_str[..shift_start].iter().map(|s| s.to_string()).collect();
                    let mut grouped: Vec<&str> = prefix_str[shift_start..].to_vec();
                    grouped.push(t_name);
                    grouped.extend_from_slice(&suffix_str);
                    shift_parts.push(format!("({})", grouped.join(" ")));
                    let shift_str = shift_parts.join(" ");

                    // Reduce bracketing
                    let mut reduce_parts: Vec<String> = prefix_str[..reduce_start].iter().map(|s| s.to_string()).collect();
                    let reduced: Vec<&str> = prefix_str[reduce_start..].to_vec();
                    reduce_parts.push(format!("({})", reduced.join(" ")));
                    reduce_parts.push(t_name.to_string());
                    reduce_parts.extend(suffix_str.iter().map(|s| s.to_string()));
                    let reduce_str = reduce_parts.join(" ");

                    format!(
                        "Example: {}\n  Shift:  {}\n  Reduce: {} (reduce to {})",
                        input_str, shift_str, reduce_str, lhs_name
                    )
                } else {
                    // Non-unifying: independent suffixes for each parse
                    let shift_suffix = find_independent_suffix(&sim, &shift_cfg);
                    let reduce_suffix = find_independent_suffix(&sim, &reduce_then_shift_cfg);
                    let shift_suffix_str: Vec<&str> = shift_suffix.iter().map(|&s| sym_name(s)).collect();
                    let reduce_suffix_str: Vec<&str> = reduce_suffix.iter().map(|&s| sym_name(s)).collect();

                    // Shift example
                    let mut shift_input: Vec<String> = prefix_str.iter().map(|s| s.to_string()).collect();
                    shift_input.push(format!("\u{2022} {}", t_name));
                    shift_input.extend(shift_suffix_str.iter().map(|s| s.to_string()));

                    let shift_start = prefix.len().saturating_sub(1);
                    let mut shift_bracket: Vec<String> = prefix_str[..shift_start].iter().map(|s| s.to_string()).collect();
                    let mut grouped: Vec<&str> = prefix_str[shift_start..].to_vec();
                    grouped.push(t_name);
                    grouped.extend_from_slice(&shift_suffix_str);
                    shift_bracket.push(format!("({})", grouped.join(" ")));

                    // Reduce example
                    let mut reduce_input: Vec<String> = prefix_str.iter().map(|s| s.to_string()).collect();
                    reduce_input.push(format!("\u{2022} {}", t_name));
                    reduce_input.extend(reduce_suffix_str.iter().map(|s| s.to_string()));

                    let mut reduce_bracket: Vec<String> = prefix_str[..reduce_start].iter().map(|s| s.to_string()).collect();
                    let reduced: Vec<&str> = prefix_str[reduce_start..].to_vec();
                    reduce_bracket.push(format!("({})", reduced.join(" ")));
                    reduce_bracket.push(t_name.to_string());
                    reduce_bracket.extend(reduce_suffix_str.iter().map(|s| s.to_string()));

                    format!(
                        "Shift example:  {}\n    {}\n  Reduce example: {}\n    {} (reduce to {})",
                        shift_input.join(" "), shift_bracket.join(" "),
                        reduce_input.join(" "), reduce_bracket.join(" "), lhs_name
                    )
                };

                results.push(crate::table::Conflict::ShiftReduce {
                    terminal: *terminal,
                    reduce_rule: *reduce_rule,
                    example,
                });
            }
            ConflictKind::ReduceReduce(rule1, rule2) => {
                let r1 = &grammar.rules[*rule1];
                let r2 = &grammar.rules[*rule2];
                let lhs1 = sym_name(r1.lhs.id().0);
                let lhs2 = sym_name(r2.lhs.id().0);
                let rhs1_len = r1.rhs.len();
                let rhs2_len = r2.rhs.len();

                let mut input: Vec<String> = prefix_str.iter().map(|s| s.to_string()).collect();
                input.push(format!("\u{2022} {}", t_name));

                let input_str = input.join(" ");

                // Show which parts of the prefix each rule would reduce
                let start1 = prefix.len().saturating_sub(rhs1_len);
                let start2 = prefix.len().saturating_sub(rhs2_len);

                let bracket = |start: usize, lhs: &str| -> String {
                    let mut s = String::new();
                    let mut opened = false;
                    for (i, &sym) in prefix_str.iter().enumerate() {
                        if i > 0 { s.push(' '); }
                        if i == start {
                            s.push('(');
                            opened = true;
                        }
                        s.push_str(sym);
                    }
                    if !opened {
                        // Epsilon reduction at end of prefix.
                        s.push('(');
                    }
                    s.push(')');
                    s.push_str(&format!(" {} [reduce to {}]", t_name, lhs));
                    s
                };

                results.push(crate::table::Conflict::ReduceReduce {
                    terminal: *terminal,
                    rule1: *rule1,
                    rule2: *rule2,
                    example: format!(
                        "Example: {}\n  Reduce 1: {}\n  Reduce 2: {}",
                        input_str,
                        bracket(start1, lhs1),
                        bracket(start2, lhs2),
                    ),
                });
            }
        }
    }

    results
}

/// A parser configuration: current state + stack of previous states.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct ParserConfig {
    state: usize,
    stack: Vec<usize>,
}

/// Helper functions for simulating an LR parser on the raw DFA.
struct ParserSim<'a> {
    dfa: &'a Dfa,
    lr: &'a DfaLrInfo,
    nfa_info: &'a LrNfaInfo,
    grammar: &'a GrammarInternal,
    num_terminals: u32,
}

impl<'a> ParserSim<'a> {
    fn new(dfa: &'a Dfa, lr: &'a DfaLrInfo, nfa_info: &'a LrNfaInfo, grammar: &'a GrammarInternal) -> Self {
        Self { dfa, lr, nfa_info, grammar, num_terminals: grammar.symbols.num_terminals() }
    }

    /// Shift a terminal: transition to an item state.
    fn shift(&self, state: usize, terminal: u32) -> Option<usize> {
        self.dfa.transitions[state].iter()
            .find(|&&(sym, target)| sym == terminal && self.lr.has_items(target))
            .map(|&(_, target)| target)
    }

    /// Goto on a non-terminal after a reduce.
    fn goto(&self, state: usize, nonterminal: u32) -> Option<usize> {
        self.dfa.transitions[state].iter()
            .find(|&&(sym, target)| sym == nonterminal && self.lr.has_items(target))
            .map(|&(_, target)| target)
    }

    /// Check if a state can accept (reduce on EOF).
    fn can_accept(&self, state: usize) -> bool {
        self.dfa.transitions[state].iter()
            .any(|&(sym, target)| {
                sym == 0 && !self.lr.has_items(target) && !self.lr.reduce_rules[target].is_empty()
            })
    }

    /// Get all reduce rules available on a given lookahead terminal.
    fn reduces_on(&self, state: usize, terminal: u32) -> Vec<usize> {
        self.dfa.transitions[state].iter()
            .filter(|&&(sym, target)| {
                sym == terminal && !self.lr.has_items(target)
            })
            .flat_map(|&(_, target)| self.lr.reduce_rules[target].iter().copied())
            .collect()
    }

    /// Get all symbols (terminals and non-terminals) that can be shifted from this state.
    fn shiftable_symbols(&self, state: usize) -> Vec<u32> {
        self.dfa.transitions[state].iter()
            .filter(|&&(sym, target)| {
                sym != 0
                    && !self.nfa_info.reduce_to_real.contains_key(&sym)
                    && self.lr.has_items(target)
            })
            .map(|&(sym, _)| sym)
            .collect()
    }

    /// Apply a reduce to a parser config. Returns None if stack is too short or goto fails.
    fn apply_reduce(&self, cfg: &ParserConfig, rule_idx: usize) -> Option<ParserConfig> {
        let rule = &self.grammar.rules[rule_idx];
        let rhs_len = rule.rhs.len();
        let lhs_id = rule.lhs.id().0;

        // Full state stack: stack ++ [state]
        let mut full = cfg.stack.clone();
        full.push(cfg.state);

        if full.len() <= rhs_len { return None; } // need at least one state remaining for goto

        full.truncate(full.len() - rhs_len);
        let goto_from = *full.last().unwrap();
        let goto_target = self.goto(goto_from, lhs_id)?;
        full.push(goto_target);

        let state = full.pop().unwrap();
        Some(ParserConfig { state, stack: full })
    }

    /// Build a parser config by replaying a sequence of symbols from state 0.
    fn replay_prefix(&self, prefix: &[u32]) -> Option<ParserConfig> {
        let mut state = 0usize;
        let mut stack = Vec::new();
        for &sym in prefix {
            let target = self.dfa.transitions[state].iter()
                .find(|&&(s, t)| s == sym && self.lr.has_items(t))
                .map(|&(_, t)| t)?;
            stack.push(state);
            state = target;
        }
        Some(ParserConfig { state, stack })
    }

    /// Shift a terminal in a config, returning the new config.
    fn shift_config(&self, cfg: &ParserConfig, terminal: u32) -> Option<ParserConfig> {
        let target = self.shift(cfg.state, terminal)?;
        let mut new_stack = cfg.stack.clone();
        new_stack.push(cfg.state);
        Some(ParserConfig { state: target, stack: new_stack })
    }
}

/// Advance a config on lookahead terminal `t`: apply reduces triggered by `t`,
/// then shift `t`. Returns all reachable configs (there may be multiple due to
/// reduce/reduce ambiguities, though typically just one).
fn advance_config(sim: &ParserSim, cfg: &ParserConfig, terminal: u32) -> Vec<ParserConfig> {
    let mut results = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    let mut visited = std::collections::HashSet::new();
    queue.push_back(cfg.clone());
    visited.insert(cfg.clone());

    while let Some(c) = queue.pop_front() {
        // Try shifting the terminal directly
        if let Some(shifted) = sim.shift_config(&c, terminal) {
            results.push(shifted);
        }
        // Try reduces on this lookahead, then recurse
        for rule in sim.reduces_on(c.state, terminal) {
            if let Some(reduced) = sim.apply_reduce(&c, rule) {
                if visited.insert(reduced.clone()) {
                    queue.push_back(reduced);
                }
            }
        }
    }
    results
}

/// Check if a config can accept: reduce on EOF until we reach acceptance.
fn can_accept_config(sim: &ParserSim, cfg: &ParserConfig) -> bool {
    let mut queue = std::collections::VecDeque::new();
    let mut visited = std::collections::HashSet::new();
    queue.push_back(cfg.clone());
    visited.insert(cfg.clone());

    while let Some(c) = queue.pop_front() {
        if sim.can_accept(c.state) {
            return true;
        }
        for rule in sim.reduces_on(c.state, 0) {
            if let Some(reduced) = sim.apply_reduce(&c, rule) {
                if visited.insert(reduced.clone()) {
                    queue.push_back(reduced);
                }
            }
        }
    }
    false
}

/// BFS budget: maximum number of entries explored before giving up.
const BFS_BUDGET: usize = 10_000;

/// Collect all symbols that could advance a config (direct shifts + terminals
/// that trigger reduces leading to shifts). Non-terminals sorted first for readability.
fn candidate_symbols(sim: &ParserSim, state: usize) -> Vec<u32> {
    let mut syms = sim.shiftable_symbols(state);
    for t in 0..sim.num_terminals {
        if !sim.reduces_on(state, t).is_empty() {
            syms.push(t);
        }
    }
    syms.sort();
    syms.dedup();
    syms.sort_by_key(|&sym| if sym >= sim.num_terminals { 0 } else { 1 });
    syms
}

/// Find a common suffix that drives both parser configs to acceptance.
///
/// BFS over pairs of configs, feeding the same symbol to both at each step.
/// Reduces are driven by lookahead (as in a real LR parser), not speculatively.
fn find_joint_suffix(
    sim: &ParserSim,
    cfg_a: &ParserConfig,
    cfg_b: &ParserConfig,
) -> Option<Vec<u32>> {
    use std::collections::{VecDeque, HashSet};

    struct Entry {
        a: ParserConfig,
        b: ParserConfig,
        suffix: Vec<u32>,
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    visited.insert((cfg_a.clone(), cfg_b.clone()));
    queue.push_back(Entry { a: cfg_a.clone(), b: cfg_b.clone(), suffix: Vec::new() });

    let mut explored = 0usize;

    while let Some(entry) = queue.pop_front() {
        explored += 1;
        if explored > BFS_BUDGET { break; }

        if can_accept_config(sim, &entry.a) && can_accept_config(sim, &entry.b) {
            return Some(entry.suffix);
        }

        for t in candidate_symbols(sim, entry.a.state) {
            let new_as = advance_config(sim, &entry.a, t);
            let new_bs = advance_config(sim, &entry.b, t);

            for new_a in &new_as {
                for new_b in &new_bs {
                    if visited.insert((new_a.clone(), new_b.clone())) {
                        let mut new_suffix = entry.suffix.clone();
                        new_suffix.push(t);
                        queue.push_back(Entry {
                            a: new_a.clone(),
                            b: new_b.clone(),
                            suffix: new_suffix,
                        });
                    }
                }
            }
        }
    }

    None
}

/// Find a suffix that drives a single config to acceptance (fallback).
fn find_independent_suffix(
    sim: &ParserSim,
    cfg: &ParserConfig,
) -> Vec<u32> {
    use std::collections::{VecDeque, HashSet};

    struct Entry {
        cfg: ParserConfig,
        suffix: Vec<u32>,
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    visited.insert(cfg.clone());
    queue.push_back(Entry { cfg: cfg.clone(), suffix: Vec::new() });

    let mut explored = 0usize;

    while let Some(entry) = queue.pop_front() {
        explored += 1;
        if explored > BFS_BUDGET { break; }

        if can_accept_config(sim, &entry.cfg) {
            return entry.suffix;
        }

        for t in candidate_symbols(sim, entry.cfg.state) {
            for new_cfg in advance_config(sim, &entry.cfg, t) {
                if visited.insert(new_cfg.clone()) {
                    let mut new_suffix = entry.suffix.clone();
                    new_suffix.push(t);
                    queue.push_back(Entry { cfg: new_cfg, suffix: new_suffix });
                }
            }
        }
    }

    Vec::new()
}


/// Result of the automaton construction pipeline.
pub(crate) struct AutomatonResult {
    /// Permuted DFA: states [0, num_item_states) are item states,
    /// state num_item_states + r reduces rule r.
    pub dfa: Dfa,
    pub num_item_states: usize,
    pub state_items: Vec<Vec<(u16, u8)>>,
    pub conflicts: Vec<crate::table::Conflict>,
    /// Virtual reduce symbol → real terminal ID (for prec terminals).
    pub reduce_to_real: HashMap<u32, u32>,
}


/// For each group of DFA states with the same LR(0) core, copy reduce
/// transitions from siblings to fill gaps. This makes same-core states
/// identical (when they don't conflict), so Hopcroft merges them — achieving
/// LALR-style minimization.
fn merge_lookaheads(dfa: &mut Dfa, states: &[DfaStateKind]) {
    // Group internal states by core: sorted (rule, dot) pairs
    let mut core_groups: HashMap<Vec<(usize, usize)>, Vec<usize>> = HashMap::new();
    for (state, kind) in states.iter().enumerate() {
        if let DfaStateKind::Items(items) = kind {
            let mut core = items.clone();
            core.sort();
            core.dedup();
            core_groups.entry(core).or_default().push(state);
        }
    }

    for (_, group) in &core_groups {
        if group.len() <= 1 {
            continue;
        }
        // Collect reduce transitions: only keep if all states that have
        // the transition agree on the target
        let mut sym_to_target: HashMap<u32, Option<usize>> = HashMap::new();
        for &state in group {
            for &(sym, target) in &dfa.transitions[state] {
                if matches!(states[target], DfaStateKind::Reduce(_)) {
                    sym_to_target.entry(sym)
                        .and_modify(|t| if *t != Some(target) { *t = None })
                        .or_insert(Some(target));
                }
            }
        }

        // Fill gaps: add transitions each state is missing
        for &state in group {
            let existing: BTreeSet<u32> = dfa.transitions[state]
                .iter()
                .map(|&(sym, _)| sym)
                .collect();
            for (&sym, &target) in &sym_to_target {
                if let Some(target) = target {
                    if !existing.contains(&sym) {
                        dfa.transitions[state].push((sym, target));
                    }
                }
            }
        }
    }
}

/// Build a minimal LR(1) automaton for a grammar using NFA → DFA → Hopcroft.
pub(crate) fn build_minimal_automaton(grammar: &GrammarInternal) -> AutomatonResult {
    let first_sets = FirstSets::compute(grammar);
    let (nfa, nfa_info) = build_lr_nfa(grammar, &first_sets);
    let num_items = nfa_info.items.len();

    let (mut raw_dfa, raw_nfa_sets) = automaton::subset_construction(&nfa);
    let dfa_lr_info = classify_dfa_states(&raw_nfa_sets, num_items);
    let dfa_conflicts = detect_conflicts(&raw_dfa, &dfa_lr_info, &nfa_info, grammar);
    let conflicts = conflict_examples(&raw_dfa, &dfa_lr_info, &nfa_info, grammar, dfa_conflicts);
    let resolved = resolve_conflicts(dfa_lr_info, &nfa_info);
    merge_lookaheads(&mut raw_dfa, &resolved);

    // Initial partition for Hopcroft: reduce states grouped by rule,
    // all item states in one partition. (Reduce states are leaves — Hopcroft
    // can't distinguish them by transitions alone.)
    let num_rules = grammar.rules.len();
    let initial_partition: Vec<usize> = resolved.iter().map(|kind| {
        match kind {
            DfaStateKind::Reduce(rule) => *rule,
            DfaStateKind::Items(_) => num_rules,
        }
    }).collect();

    let (min_dfa, state_map) = automaton::hopcroft_minimize(&raw_dfa, &initial_partition);

    // Map resolved classification through Hopcroft's state_map.
    let mut min_states = vec![const { DfaStateKind::Reduce(0) }; min_dfa.num_states()];
    for (raw_state, kind) in resolved.into_iter().enumerate() {
        min_states[state_map[raw_state]] = kind;
    }

    // Permute: item states first [0, num_item_states), then reduce states.
    // Reduce state for rule r = num_item_states + r.
    let mut permutation = vec![0usize; min_dfa.num_states()];
    let mut num_item_states = 0;
    for (state, kind) in min_states.iter().enumerate() {
        if let DfaStateKind::Items(_) = kind {
            permutation[state] = num_item_states;
            num_item_states += 1;
        }
    }
    for (state, kind) in min_states.iter().enumerate() {
        if let DfaStateKind::Reduce(rule) = kind {
            permutation[state] = num_item_states + rule;
        }
    }

    let total_states = num_item_states + num_rules;
    let mut new_transitions = vec![Vec::new(); total_states];
    for (old_state, trans) in min_dfa.transitions.iter().enumerate() {
        let new_state = permutation[old_state];
        new_transitions[new_state] = trans.iter()
            .map(|&(sym, target)| (sym, permutation[target]))
            .collect();
    }

    let permuted_dfa = Dfa {
        transitions: new_transitions,
    };

    let mut state_items = vec![Vec::new(); num_item_states];
    for (state, kind) in min_states.iter().enumerate() {
        if let DfaStateKind::Items(items) = kind {
            state_items[permutation[state]] = items.iter()
                .map(|&(rule, dot)| (rule as u16, dot as u8))
                .collect();
        }
    }

    AutomatonResult {
        dfa: permuted_dfa,
        num_item_states,
        state_items,
        conflicts,
        reduce_to_real: nfa_info.reduce_to_real,
    }
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

    /// Count LALR states by computing LR(0) core equivalence classes
    /// on the raw DFA item states.
    fn lalr_state_count(grammar: &GrammarInternal) -> usize {
        let first_sets = FirstSets::compute(grammar);
        let (nfa, nfa_info) = build_lr_nfa(grammar, &first_sets);
        let num_items = nfa_info.items.len();
        let (raw_dfa, raw_nfa_sets) = crate::automaton::subset_construction(&nfa);
        let lr = classify_dfa_states(&raw_nfa_sets, num_items);

        let num_terminals = grammar.symbols.num_terminals() as usize;

        // For each item-bearing DFA state, compute its LR(0) core
        // (the set of (rule, dot) pairs, stripping lookahead).
        let mut cores: std::collections::HashSet<Vec<usize>> = std::collections::HashSet::new();
        for state in 0..raw_dfa.num_states() {
            if !lr.has_items(state) { continue; }
            let mut core: Vec<usize> = raw_nfa_sets[state].iter()
                .filter(|&&s| s < num_items)
                .map(|&s| s / num_terminals)
                .collect();
            core.sort();
            core.dedup();
            cores.insert(core);
        }
        cores.len()
    }

    fn grammar_stats(grammar: &GrammarInternal) -> (usize, usize, usize, usize) {
        use crate::table::CompiledTable;
        let compiled = CompiledTable::build_from_internal(grammar);
        let rr = compiled.conflicts.iter()
            .filter(|c| matches!(c, crate::table::Conflict::ReduceReduce { .. }))
            .count();
        let sr = compiled.conflicts.iter()
            .filter(|c| matches!(c, crate::table::Conflict::ShiftReduce { .. }))
            .count();
        (compiled.num_states, lalr_state_count(grammar), rr, sr)
    }

    #[test]
    fn print_state_counts() {
        let grammars = [
            ("C11", "grammars/c11.gzl"),
            ("Python", "grammars/python.gzl"),
            ("Meta", "grammars/meta.gzl"),
        ];

        for (name, path) in grammars {
            let src = std::fs::read_to_string(path).unwrap();
            let grammar = to_grammar_internal(&parse_grammar(&src).unwrap()).unwrap();
            let (min_lr, lalr, rr, sr) = grammar_stats(&grammar);
            eprintln!("{}: minimal LR {} states, LALR {} states, {} rr, {} sr",
                name, min_lr, lalr, rr, sr);
            // LALR grammars: minimal LR must equal LALR
            assert_eq!(min_lr, lalr, "{} should have same state count", name);
        }

        // C11 has known conflicts
        let c11_src = std::fs::read_to_string("grammars/c11.gzl").unwrap();
        let c11 = to_grammar_internal(&parse_grammar(&c11_src).unwrap()).unwrap();
        let (_, _, rr, sr) = grammar_stats(&c11);
        assert_eq!(rr, 3);
        assert_eq!(sr, 1);

        // Classic LR(1)-but-not-LALR(1) grammar:
        // S → aEa | bEb | aFb | bFa; E → e; F → e
        // LALR merges the states for "E → e•" and "F → e•" (same core)
        // but they have incompatible lookaheads (a vs b), causing a
        // spurious reduce/reduce conflict.
        let non_lalr = to_grammar_internal(&parse_grammar(r#"
            start s;
            terminals { a, b, e }
            s = a ee a => aea | b ee b => beb | a f b => afb | b f a => bfa;
            ee = e => e;
            f = e => f;
        "#).unwrap()).unwrap();
        let (min_lr, lalr, rr, sr) = grammar_stats(&non_lalr);
        eprintln!("non-LALR: minimal LR {} states, LALR {} states, {} rr, {} sr",
            min_lr, lalr, rr, sr);
        // Minimal LR must have MORE states than LALR (it splits to avoid conflicts)
        assert!(min_lr > lalr, "minimal LR should have more states than LALR");
        // And no conflicts
        assert_eq!(rr, 0, "minimal LR should have no reduce/reduce conflicts");
        assert_eq!(sr, 0, "minimal LR should have no shift/reduce conflicts");
    }
}

use crate::grammar::SymbolId;
use std::collections::{BTreeMap, BTreeSet, HashMap};

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
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
pub enum AltAction {
    /// No action — auto-handle (passthrough or structural).
    None,
    /// User-defined action name (e.g., `@binop`).
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
        types.insert(sym.id(), def.type_name.clone());
    }
    symbols.finalize_terminals();

    // Register user non-terminals + types
    // NTs with @name on any alternative get an auto-derived associated type
    for rule in &grammar.rules {
        let sym = symbols.intern_non_terminal(&rule.name);
        let has_named_alt = rule.alts.iter().any(|a| a.name.is_some());
        let result_type = if has_named_alt {
            Some(capitalize(&rule.name))
        } else {
            None
        };
        types.insert(sym.id(), result_type);
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

            let action = match alt.name.as_deref() {
                Some(s) => AltAction::Named(s.to_string()),
                None => AltAction::None,
            };

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
        action: AltAction::None,
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

/// A set of LR(1) items representing a parser state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct ItemSet {
    pub items: BTreeSet<Item>,
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
fn closure(
    grammar: &GrammarInternal,
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
fn goto(
    grammar: &GrammarInternal,
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

/// LR algorithm variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LrAlgorithm {
    /// LALR(1): merge states by LR(0) core, may have spurious conflicts.
    #[default]
    Lalr1,
    /// Canonical LR(1): no merging, no spurious conflicts, more states.
    Lr1,
}

/// An LR(1) automaton: a collection of states with transitions.
#[derive(Debug)]
pub struct Automaton {
    /// The states (item sets) of the automaton.
    pub(crate) states: Vec<ItemSet>,
    /// Transitions: (from_state, symbol) -> to_state.
    pub(crate) transitions: HashMap<(usize, Symbol), usize>,
    /// The augmented grammar used to build this automaton.
    pub(crate) grammar: GrammarInternal,
}

impl Automaton {
    #[cfg(test)]
    fn build(grammar: &GrammarInternal) -> Self {
        Self::build_with_algorithm(grammar, LrAlgorithm::default())
    }

    /// Build an LR automaton for a grammar using the specified algorithm.
    /// The grammar must already be augmented (via to_grammar_internal).
    pub fn build_with_algorithm(grammar: &GrammarInternal, algorithm: LrAlgorithm) -> Self {
        let first_sets = FirstSets::compute(grammar);

        // Initial state: closure of [__start -> • <original_start>, $]
        let initial_item = Item::new(0, 0, SymbolId::EOF);
        let initial_set = ItemSet::from_items([initial_item]);
        let state0 = closure(grammar, &initial_set, &first_sets);

        let mut states = vec![state0];
        let mut transitions = HashMap::new();

        // For LALR: key by core. For LR(1): key by full item set.
        let mut core_index: HashMap<BTreeSet<(usize, usize)>, usize> = HashMap::new();
        let mut full_index: HashMap<BTreeSet<Item>, usize> = HashMap::new();

        match algorithm {
            LrAlgorithm::Lalr1 => { core_index.insert(states[0].core(), 0); }
            LrAlgorithm::Lr1 => { full_index.insert(states[0].items.clone(), 0); }
        }

        let mut worklist = vec![0usize];

        while let Some(state_idx) = worklist.pop() {
            let state = states[state_idx].clone();

            // Collect all symbols we can transition on
            let symbols: BTreeSet<Symbol> = state.items.iter()
                .filter_map(|item| item.next_symbol(grammar))
                .collect();

            for symbol in symbols {
                let next_state = goto(grammar, &state, symbol, &first_sets);
                if next_state.is_empty() {
                    continue;
                }

                let next_idx = match algorithm {
                    LrAlgorithm::Lalr1 => {
                        let next_core = next_state.core();
                        if let Some(&idx) = core_index.get(&next_core) {
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
                            core_index.insert(next_core, idx);
                            states.push(next_state);
                            worklist.push(idx);
                            idx
                        }
                    }
                    LrAlgorithm::Lr1 => {
                        if let Some(&idx) = full_index.get(&next_state.items) {
                            // Canonical LR(1): exact match required, no merging
                            idx
                        } else {
                            let idx = states.len();
                            full_index.insert(next_state.items.clone(), idx);
                            states.push(next_state);
                            worklist.push(idx);
                            idx
                        }
                    }
                };

                transitions.insert((state_idx, symbol), next_idx);
            }
        }

        Automaton {
            states,
            transitions,
            grammar: grammar.clone(),
        }
    }

    /// Returns the number of states in the automaton.
    pub(crate) fn num_states(&self) -> usize {
        self.states.len()
    }

    /// Returns the transition from a state on a symbol, if any.
    #[cfg(test)]
    fn transition(&self, state: usize, symbol: Symbol) -> Option<usize> {
        self.transitions.get(&(state, symbol)).copied()
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
            expr = expr PLUS term | term;
            term = NUM;
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

        // rule 0: __start -> expr (augmented)
        // rule 1: expr -> expr PLUS term
        // rule 2: expr -> term
        // rule 3: term -> NUM
        let initial = ItemSet::from_items([Item::new(3, 0, SymbolId::EOF)]);
        let closed = closure(&grammar, &initial, &first);
        let after_num = goto(&grammar, &closed, num, &first);

        // Should have term -> NUM •
        let has_complete = after_num.items.iter().any(|item| {
            item.rule == 3 && item.dot == 1
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
        let grammar = to_grammar_internal(&parse_grammar(r#"
            start s; terminals { a } s = a;
        "#).unwrap()).unwrap();

        let automaton = Automaton::build(&grammar);

        // Augmented: __start -> s, s -> 'a'
        // States: 3
        assert_eq!(automaton.num_states(), 3);

        let a_sym = automaton.grammar.symbols.get("a").unwrap();
        let s_sym = automaton.grammar.symbols.get("s").unwrap();

        assert!(automaton.transition(0, a_sym).is_some());
        assert!(automaton.transition(0, s_sym).is_some());
    }

    #[test]
    fn test_paren_grammar() {
        // Test that lookaheads are properly merged in LALR(1)
        let grammar = to_grammar_internal(&parse_grammar(r#"
            start expr;
            terminals { NUM, LPAREN, RPAREN }
            expr = NUM | LPAREN expr RPAREN;
        "#).unwrap()).unwrap();

        let automaton = Automaton::build(&grammar);

        // Build parse table
        use crate::table::CompiledTable;
        let compiled = CompiledTable::build_with_algorithm(&grammar, crate::lr::LrAlgorithm::default());
        let table = compiled.table();

        assert!(!compiled.has_conflicts());

        let rparen_id = compiled.symbol_id("RPAREN").unwrap();
        let _num_id = compiled.symbol_id("NUM").unwrap();

        // After shifting NUM inside parens, RPAREN should trigger a reduce
        // Find the state reached by shifting LPAREN then NUM
        let lparen_sym = grammar.symbols.get("LPAREN").unwrap();
        let num_sym = grammar.symbols.get("NUM").unwrap();

        let state_after_lparen = automaton.transition(0, lparen_sym).unwrap();
        let state_after_num = automaton.transition(state_after_lparen, num_sym).unwrap();

        // This state should have Reduce action for RPAREN
        match table.action(state_after_num, rparen_id) {
            crate::runtime::Action::Reduce(_) => {} // Good!
            other => panic!("Expected Reduce for RPAREN after LPAREN NUM, got {:?}", other),
        }
    }
}

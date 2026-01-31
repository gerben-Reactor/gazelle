use std::collections::HashMap;

/// Precedence information carried by a token at parse time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Precedence {
    Left(u8),
    Right(u8),
}

impl Precedence {
    /// Get the precedence level.
    pub fn level(&self) -> u8 {
        match self {
            Precedence::Left(l) | Precedence::Right(l) => *l,
        }
    }

    /// Get the associativity as u8 (0=left, 1=right).
    pub fn assoc(&self) -> u8 {
        match self {
            Precedence::Left(_) => 0,
            Precedence::Right(_) => 1,
        }
    }
}

/// An interned symbol ID for O(1) lookups.
/// Layout:
/// - IDs 0..num_terminals: terminals (EOF is always terminal 0)
/// - IDs num_terminals.. onwards: non-terminals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SymbolId(pub u32);

impl SymbolId {
    /// The EOF symbol ID (always 0).
    pub const EOF: SymbolId = SymbolId(0);
}

/// A grammar symbol: terminal, precedence terminal, or non-terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Symbol {
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

    pub fn is_prec_terminal(&self) -> bool {
        matches!(self, Symbol::PrecTerminal(_))
    }

    pub fn is_non_terminal(&self) -> bool {
        matches!(self, Symbol::NonTerminal(_))
    }
}

/// Information about a terminal symbol.
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub is_prec: bool,
}

/// Symbol table mapping names to IDs and vice versa.
#[derive(Debug, Clone)]
pub struct SymbolTable {
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
    pub(crate) fn finalize_terminals(&mut self) {
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

    /// Check if this is the EOF symbol.
    pub fn is_eof(&self, id: SymbolId) -> bool {
        id.0 == 0
    }

    /// Check if this is a terminal (including EOF).
    pub fn is_terminal(&self, id: SymbolId) -> bool {
        id.0 < self.num_terminals
    }

    /// Check if this is a non-terminal.
    pub fn is_non_terminal(&self, id: SymbolId) -> bool {
        id.0 >= self.num_terminals
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

    /// Iterate over all terminal IDs (including EOF).
    pub fn terminal_ids(&self) -> impl Iterator<Item = SymbolId> {
        (0..self.num_terminals).map(SymbolId)
    }

    /// Iterate over all non-terminal IDs.
    pub fn non_terminal_ids(&self) -> impl Iterator<Item = SymbolId> + '_ {
        let start = self.num_terminals;
        let end = start + self.num_non_terminals();
        (start..end).map(SymbolId)
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub lhs: Symbol,
    pub rhs: Vec<Symbol>,
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub rules: Vec<Rule>,
    pub start: Symbol,
    pub symbols: SymbolTable,
}

impl Grammar {
    /// Create a new grammar builder.
    pub fn builder() -> GrammarBuilder {
        GrammarBuilder::new()
    }

    /// Returns all rules with the given non-terminal on the left-hand side.
    pub fn rules_for(&self, symbol: Symbol) -> impl Iterator<Item = (usize, &Rule)> {
        self.rules
            .iter()
            .enumerate()
            .filter(move |(_, rule)| rule.lhs == symbol)
    }

    /// Create an augmented grammar with a new start rule: __start -> <original_start>
    pub(crate) fn augment(mut self) -> Grammar {
        let aug_start = self.symbols.intern_non_terminal("__start");

        let aug_rule = Rule {
            lhs: aug_start,
            rhs: vec![self.start],
        };

        let mut rules = vec![aug_rule];
        rules.extend(self.rules);

        Grammar {
            rules,
            start: aug_start,
            symbols: self.symbols,
        }
    }
}

/// Builder for constructing grammars with interned symbols.
#[derive(Debug, Clone)]
pub struct GrammarBuilder {
    /// The symbol table (public for macro access).
    pub symbols: SymbolTable,
    pub(crate) rules: Vec<Rule>,
    start: Option<Symbol>,
    terminals_finalized: bool,
}

impl GrammarBuilder {
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
            rules: Vec::new(),
            start: None,
            terminals_finalized: false,
        }
    }

    /// Intern a terminal symbol.
    pub fn t(&mut self, name: &str) -> Symbol {
        assert!(!self.terminals_finalized, "Cannot add terminals after adding non-terminals");
        self.symbols.intern_terminal(name)
    }

    /// Intern a precedence terminal symbol.
    pub fn pt(&mut self, name: &str) -> Symbol {
        assert!(!self.terminals_finalized, "Cannot add terminals after adding non-terminals");
        self.symbols.intern_prec_terminal(name)
    }

    /// Intern a non-terminal symbol.
    pub fn nt(&mut self, name: &str) -> Symbol {
        if !self.terminals_finalized {
            self.symbols.finalize_terminals();
            self.terminals_finalized = true;
        }
        self.symbols.intern_non_terminal(name)
    }

    /// Add a rule.
    pub fn rule(&mut self, lhs: Symbol, rhs: Vec<Symbol>) -> &mut Self {
        if self.start.is_none() {
            self.start = Some(lhs);
        }
        self.rules.push(Rule { lhs, rhs });
        self
    }

    /// Set the start symbol explicitly.
    pub fn start(&mut self, s: Symbol) -> &mut Self {
        self.start = Some(s);
        self
    }

    /// Build the grammar.
    pub fn build(mut self) -> Grammar {
        if !self.terminals_finalized {
            self.symbols.finalize_terminals();
        }
        Grammar {
            rules: self.rules,
            start: self.start.expect("No start symbol set"),
            symbols: self.symbols,
        }
    }
}

impl Default for GrammarBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_grammar() {
        let mut gb = GrammarBuilder::new();
        let plus = gb.t("+");
        let num = gb.t("NUM");
        let expr = gb.nt("expr");
        let term = gb.nt("term");

        gb.rule(expr, vec![expr, plus, term]);
        gb.rule(expr, vec![term]);
        gb.rule(term, vec![num]);

        let grammar = gb.build();

        assert_eq!(grammar.rules.len(), 3);
        assert_eq!(grammar.rules_for(expr).count(), 2);
        assert_eq!(grammar.rules_for(term).count(), 1);
    }

    #[test]
    fn test_symbol_table() {
        let mut gb = GrammarBuilder::new();
        let plus = gb.t("+");
        let num = gb.t("NUM");
        let expr = gb.nt("expr");
        let term = gb.nt("term");

        gb.rule(expr, vec![expr, plus, term]);
        gb.rule(expr, vec![term]);
        gb.rule(term, vec![num]);

        let grammar = gb.build();

        // Should have 3 terminals: $, +, NUM (EOF is included)
        assert_eq!(grammar.symbols.num_terminals(), 3);
        // Should have 2 non-terminals: expr, term
        assert_eq!(grammar.symbols.num_non_terminals(), 2);

        // Check IDs
        let plus_id = plus.id();
        let num_id = num.id();
        let expr_id = expr.id();
        let term_id = term.id();

        // Terminals have IDs 0..3
        assert!(grammar.symbols.is_terminal(plus_id));
        assert!(grammar.symbols.is_terminal(num_id));
        assert!(!grammar.symbols.is_non_terminal(plus_id));

        // Non-terminals have IDs >= 3
        assert!(grammar.symbols.is_non_terminal(expr_id));
        assert!(grammar.symbols.is_non_terminal(term_id));
        assert!(!grammar.symbols.is_terminal(expr_id));

        // EOF is ID 0
        assert!(grammar.symbols.is_eof(SymbolId::EOF));
        assert!(grammar.symbols.is_terminal(SymbolId::EOF));

        // Names round-trip
        assert_eq!(grammar.symbols.name(plus_id), "+");
        assert_eq!(grammar.symbols.name(num_id), "NUM");
        assert_eq!(grammar.symbols.name(expr_id), "expr");
        assert_eq!(grammar.symbols.name(term_id), "term");
        assert_eq!(grammar.symbols.name(SymbolId::EOF), "$");
    }

    #[test]
    fn test_prec_terminal() {
        let mut gb = GrammarBuilder::new();
        let op = gb.pt("OP");
        let num = gb.t("NUM");
        let expr = gb.nt("expr");

        gb.rule(expr, vec![expr, op, expr]);
        gb.rule(expr, vec![num]);

        let grammar = gb.build();

        assert!(grammar.symbols.is_prec_terminal(op.id()));
        assert!(!grammar.symbols.is_prec_terminal(num.id()));
        assert!(!grammar.symbols.is_prec_terminal(SymbolId::EOF));
    }

    #[test]
    fn test_grammar_augment() {
        let mut gb = GrammarBuilder::new();
        let a = gb.t("a");
        let s = gb.nt("S");
        gb.rule(s, vec![a]);

        let grammar = gb.build();
        let augmented = grammar.augment();

        // Should have 2 rules: __start -> S, S -> a
        assert_eq!(augmented.rules.len(), 2);

        // Rule 0 should be __start -> S
        let start_id = augmented.symbols.get_id("__start").unwrap();
        assert_eq!(augmented.rules[0].lhs.id(), start_id);
        assert_eq!(augmented.start.id(), start_id);
    }
}

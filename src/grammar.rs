use std::collections::HashMap;

/// Associativity for precedence-carrying operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Assoc {
    Left,
    Right,
}

/// Precedence information carried by a token at parse time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Precedence {
    pub level: u8,
    pub assoc: Assoc,
}

impl Precedence {
    pub fn left(level: u8) -> Self {
        Self { level, assoc: Assoc::Left }
    }

    pub fn right(level: u8) -> Self {
        Self { level, assoc: Assoc::Right }
    }
}

// ============================================================================
// Integer-based symbol system for efficient parsing
// ============================================================================

/// An interned symbol ID for O(1) lookups.
/// Layout:
/// - ID 0: EOF
/// - IDs 1..=num_terminals: regular terminals
/// - IDs num_terminals+1 onwards: non-terminals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId(pub u32);

impl SymbolId {
    /// The EOF symbol ID (always 0).
    pub const EOF: SymbolId = SymbolId(0);
}

/// Information about a terminal symbol.
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub is_prec: bool,  // true if this is a PrecTerminal
}

/// Symbol table mapping names to IDs and vice versa.
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// Terminal info, indexed by id-1 (id 1..=num_terminals)
    terminals: Vec<SymbolInfo>,
    /// Non-terminal names, indexed by id-num_terminals-1
    non_terminals: Vec<String>,
    /// Lookup from name to ID
    name_to_id: HashMap<String, SymbolId>,
    /// Count of terminals (not including EOF)
    num_terminals: u32,
}

impl SymbolTable {
    /// Create a new empty symbol table.
    pub fn new() -> Self {
        Self {
            terminals: Vec::new(),
            non_terminals: Vec::new(),
            name_to_id: HashMap::new(),
            num_terminals: 0,
        }
    }

    /// Build a symbol table from a grammar.
    pub fn from_grammar(grammar: &Grammar) -> Self {
        let mut table = Self::new();

        // Collect all terminals first
        for rule in &grammar.rules {
            for sym in &rule.rhs {
                if sym.is_terminal() {
                    table.intern_terminal(sym.name(), sym.is_prec_terminal());
                }
            }
        }

        // Mark terminal count
        table.num_terminals = table.terminals.len() as u32;

        // Collect all non-terminals
        for rule in &grammar.rules {
            table.intern_non_terminal(rule.lhs.name());
            for sym in &rule.rhs {
                if sym.is_non_terminal() {
                    table.intern_non_terminal(sym.name());
                }
            }
        }

        table
    }

    /// Intern a terminal symbol, returning its ID.
    fn intern_terminal(&mut self, name: &str, is_prec: bool) -> SymbolId {
        if let Some(&id) = self.name_to_id.get(name) {
            return id;
        }

        let id = SymbolId((self.terminals.len() + 1) as u32);
        self.terminals.push(SymbolInfo {
            name: name.to_string(),
            is_prec,
        });
        self.name_to_id.insert(name.to_string(), id);
        id
    }

    /// Intern a non-terminal symbol, returning its ID.
    fn intern_non_terminal(&mut self, name: &str) -> SymbolId {
        if let Some(&id) = self.name_to_id.get(name) {
            return id;
        }

        let id = SymbolId(self.num_terminals + 1 + self.non_terminals.len() as u32);
        self.non_terminals.push(name.to_string());
        self.name_to_id.insert(name.to_string(), id);
        id
    }

    /// Get the ID for a symbol, if it exists.
    pub fn get_id(&self, name: &str) -> Option<SymbolId> {
        self.name_to_id.get(name).copied()
    }

    /// Get the ID for a Symbol enum.
    pub fn symbol_to_id(&self, symbol: &Symbol) -> Option<SymbolId> {
        self.get_id(symbol.name())
    }

    /// Check if this is the EOF symbol.
    pub fn is_eof(&self, id: SymbolId) -> bool {
        id.0 == 0
    }

    /// Check if this is a terminal (including EOF).
    pub fn is_terminal(&self, id: SymbolId) -> bool {
        id.0 <= self.num_terminals
    }

    /// Check if this is a non-terminal.
    pub fn is_non_terminal(&self, id: SymbolId) -> bool {
        id.0 > self.num_terminals
    }

    /// Check if this terminal is a precedence terminal.
    pub fn is_prec_terminal(&self, id: SymbolId) -> bool {
        if id.0 == 0 || id.0 > self.num_terminals {
            return false;
        }
        self.terminals[(id.0 - 1) as usize].is_prec
    }

    /// Get the name of a symbol.
    pub fn name(&self, id: SymbolId) -> &str {
        if id.0 == 0 {
            "$"
        } else if id.0 <= self.num_terminals {
            &self.terminals[(id.0 - 1) as usize].name
        } else {
            let idx = (id.0 - self.num_terminals - 1) as usize;
            &self.non_terminals[idx]
        }
    }

    /// Get the number of terminals (not including EOF).
    pub fn num_terminals(&self) -> u32 {
        self.num_terminals
    }

    /// Get the number of non-terminals.
    pub fn num_non_terminals(&self) -> u32 {
        self.non_terminals.len() as u32
    }

    /// Get the total number of symbols (including EOF).
    pub fn num_symbols(&self) -> u32 {
        1 + self.num_terminals + self.num_non_terminals()
    }

    /// Iterate over all terminal IDs (including EOF).
    pub fn terminal_ids(&self) -> impl Iterator<Item = SymbolId> {
        (0..=self.num_terminals).map(SymbolId)
    }

    /// Iterate over all non-terminal IDs.
    pub fn non_terminal_ids(&self) -> impl Iterator<Item = SymbolId> + '_ {
        let start = self.num_terminals + 1;
        let end = start + self.num_non_terminals();
        (start..end).map(SymbolId)
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

/// A rule using interned symbol IDs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InternedRule {
    pub lhs: SymbolId,
    pub rhs: Vec<SymbolId>,
}

/// A grammar with interned symbols for efficient parsing.
#[derive(Debug, Clone)]
pub struct InternedGrammar {
    pub rules: Vec<InternedRule>,
    pub start: SymbolId,
    pub symbols: SymbolTable,
}

impl InternedGrammar {
    /// Get rules for a given non-terminal.
    pub fn rules_for(&self, symbol: SymbolId) -> impl Iterator<Item = (usize, &InternedRule)> {
        self.rules
            .iter()
            .enumerate()
            .filter(move |(_, rule)| rule.lhs == symbol)
    }

    /// Create an augmented grammar with a new start rule: __start -> <original_start>
    pub fn augment(&self) -> InternedGrammar {
        let mut symbols = self.symbols.clone();
        let aug_start = symbols.intern_non_terminal("__start");

        let aug_rule = InternedRule {
            lhs: aug_start,
            rhs: vec![self.start],
        };

        let mut rules = vec![aug_rule];
        rules.extend(self.rules.iter().cloned());

        InternedGrammar {
            rules,
            start: aug_start,
            symbols,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    /// A regular terminal symbol.
    Terminal(String),
    /// A terminal that carries precedence at runtime (e.g., operators).
    /// Shift/reduce conflicts on these are resolved by comparing precedences.
    PrecTerminal(String),
    /// A non-terminal symbol.
    NonTerminal(String),
}

impl Symbol {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Symbol::Terminal(_) | Symbol::PrecTerminal(_))
    }

    pub fn is_prec_terminal(&self) -> bool {
        matches!(self, Symbol::PrecTerminal(_))
    }

    pub fn is_non_terminal(&self) -> bool {
        matches!(self, Symbol::NonTerminal(_))
    }

    pub fn name(&self) -> &str {
        match self {
            Symbol::Terminal(s) | Symbol::PrecTerminal(s) | Symbol::NonTerminal(s) => s,
        }
    }
}

/// Helper to create a terminal symbol.
pub fn t(name: &str) -> Symbol {
    Symbol::Terminal(name.to_string())
}

/// Helper to create a precedence-carrying terminal symbol.
pub fn pt(name: &str) -> Symbol {
    Symbol::PrecTerminal(name.to_string())
}

/// Helper to create a non-terminal symbol.
pub fn nt(name: &str) -> Symbol {
    Symbol::NonTerminal(name.to_string())
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
}

impl Grammar {
    /// Returns all rules with the given non-terminal on the left-hand side.
    pub fn rules_for(&self, symbol: &Symbol) -> impl Iterator<Item = (usize, &Rule)> {
        self.rules
            .iter()
            .enumerate()
            .filter(move |(_, rule)| &rule.lhs == symbol)
    }

    /// Returns all non-terminal symbols in the grammar.
    pub fn non_terminals(&self) -> impl Iterator<Item = &Symbol> {
        self.rules.iter().map(|r| &r.lhs)
    }

    /// Returns all terminal symbols in the grammar.
    pub fn terminals(&self) -> impl Iterator<Item = &Symbol> {
        self.rules
            .iter()
            .flat_map(|r| r.rhs.iter())
            .filter(|s| s.is_terminal())
    }

    /// Create an augmented grammar with a new start rule: __start -> <original_start>
    /// Returns the augmented grammar and the index of the augmented rule (always 0).
    pub fn augment(&self) -> Grammar {
        let aug_start = nt("__start");
        let aug_rule = Rule {
            lhs: aug_start.clone(),
            rhs: vec![self.start.clone()],
        };

        let mut rules = vec![aug_rule];
        rules.extend(self.rules.iter().cloned());

        Grammar {
            rules,
            start: aug_start,
        }
    }

    /// Convert to an interned grammar with integer symbol IDs.
    pub fn intern(&self) -> InternedGrammar {
        let symbols = SymbolTable::from_grammar(self);

        let rules = self.rules.iter().map(|rule| {
            let lhs = symbols.symbol_to_id(&rule.lhs).unwrap();
            let rhs = rule.rhs.iter()
                .map(|s| symbols.symbol_to_id(s).unwrap())
                .collect();
            InternedRule { lhs, rhs }
        }).collect();

        let start = symbols.symbol_to_id(&self.start).unwrap();

        InternedGrammar { rules, start, symbols }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_grammar() {
        let grammar = Grammar {
            start: nt("expr"),
            rules: vec![
                Rule { lhs: nt("expr"), rhs: vec![nt("expr"), t("+"), nt("term")] },
                Rule { lhs: nt("expr"), rhs: vec![nt("term")] },
                Rule { lhs: nt("term"), rhs: vec![t("NUM")] },
            ],
        };

        assert_eq!(grammar.rules.len(), 3);
        assert_eq!(grammar.rules_for(&nt("expr")).count(), 2);
        assert_eq!(grammar.rules_for(&nt("term")).count(), 1);
    }

    #[test]
    fn test_symbol_interning() {
        let grammar = Grammar {
            start: nt("expr"),
            rules: vec![
                Rule { lhs: nt("expr"), rhs: vec![nt("expr"), t("+"), nt("term")] },
                Rule { lhs: nt("expr"), rhs: vec![nt("term")] },
                Rule { lhs: nt("term"), rhs: vec![t("NUM")] },
            ],
        };

        let interned = grammar.intern();

        // Should have 2 terminals: +, NUM
        assert_eq!(interned.symbols.num_terminals(), 2);
        // Should have 2 non-terminals: expr, term
        assert_eq!(interned.symbols.num_non_terminals(), 2);

        // Check IDs
        let plus_id = interned.symbols.get_id("+").unwrap();
        let num_id = interned.symbols.get_id("NUM").unwrap();
        let expr_id = interned.symbols.get_id("expr").unwrap();
        let term_id = interned.symbols.get_id("term").unwrap();

        // Terminals have IDs 1..=2
        assert!(interned.symbols.is_terminal(plus_id));
        assert!(interned.symbols.is_terminal(num_id));
        assert!(!interned.symbols.is_non_terminal(plus_id));

        // Non-terminals have IDs > 2
        assert!(interned.symbols.is_non_terminal(expr_id));
        assert!(interned.symbols.is_non_terminal(term_id));
        assert!(!interned.symbols.is_terminal(expr_id));

        // EOF is ID 0
        assert!(interned.symbols.is_eof(SymbolId::EOF));
        assert!(interned.symbols.is_terminal(SymbolId::EOF));

        // Names round-trip
        assert_eq!(interned.symbols.name(plus_id), "+");
        assert_eq!(interned.symbols.name(num_id), "NUM");
        assert_eq!(interned.symbols.name(expr_id), "expr");
        assert_eq!(interned.symbols.name(term_id), "term");
        assert_eq!(interned.symbols.name(SymbolId::EOF), "$");
    }

    #[test]
    fn test_prec_terminal_interning() {
        let grammar = Grammar {
            start: nt("expr"),
            rules: vec![
                Rule { lhs: nt("expr"), rhs: vec![nt("expr"), pt("OP"), nt("expr")] },
                Rule { lhs: nt("expr"), rhs: vec![t("NUM")] },
            ],
        };

        let interned = grammar.intern();

        let op_id = interned.symbols.get_id("OP").unwrap();
        let num_id = interned.symbols.get_id("NUM").unwrap();

        assert!(interned.symbols.is_prec_terminal(op_id));
        assert!(!interned.symbols.is_prec_terminal(num_id));
        assert!(!interned.symbols.is_prec_terminal(SymbolId::EOF));
    }

    #[test]
    fn test_interned_grammar_augment() {
        let grammar = Grammar {
            start: nt("S"),
            rules: vec![
                Rule { lhs: nt("S"), rhs: vec![t("a")] },
            ],
        };

        let interned = grammar.intern();
        let augmented = interned.augment();

        // Should have 2 rules: __start -> S, S -> a
        assert_eq!(augmented.rules.len(), 2);

        // Rule 0 should be __start -> S
        let start_id = augmented.symbols.get_id("__start").unwrap();
        assert_eq!(augmented.rules[0].lhs, start_id);
        assert_eq!(augmented.start, start_id);
    }
}

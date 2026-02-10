//! Grammar types - both public AST and internal representation types.

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

// ============================================================================
// Public AST types for grammar definitions
// ============================================================================

/// A grammar definition (AST).
#[derive(Debug, Clone)]
pub struct Grammar {
    /// Name of the grammar.
    pub name: String,
    /// Name of the start symbol.
    pub start: String,
    /// LR algorithm mode: "lalr" (default) or "lr".
    pub mode: String,
    /// Expected number of reduce/reduce conflicts.
    pub expect_rr: usize,
    /// Expected number of shift/reduce conflicts.
    pub expect_sr: usize,
    /// Terminal definitions.
    pub terminals: Vec<TerminalDef>,
    /// Grammar rules (productions).
    pub rules: Vec<Rule>,
}

/// Expected conflict declaration.
#[derive(Debug, Clone)]
pub struct ExpectDecl {
    /// Number of expected conflicts.
    pub count: usize,
    /// Conflict kind: "rr" (reduce/reduce) or "sr" (shift/reduce).
    pub kind: String,
}

/// A terminal definition in the grammar.
#[derive(Debug, Clone)]
pub struct TerminalDef {
    /// Terminal name (e.g., "NUM", "PLUS").
    pub name: String,
    /// Associated type name, if the terminal carries data.
    pub type_name: Option<String>,
    /// Whether this is a precedence terminal (`prec` keyword).
    pub is_prec: bool,
}

/// A rule (production) in the grammar.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Non-terminal name (left-hand side).
    pub name: String,
    /// Result type for this rule, if specified.
    pub result_type: Option<String>,
    /// Alternatives (right-hand sides).
    pub alts: Vec<Alt>,
}

/// An alternative (right-hand side) of a rule.
#[derive(Debug, Clone)]
pub struct Alt {
    /// Symbols in this alternative.
    pub symbols: Vec<SymbolRef>,
    /// Action name (e.g., `@foo`), if specified.
    pub name: Option<String>,
}

/// A symbol reference in a rule with optional modifier.
#[derive(Debug, Clone)]
pub struct SymbolRef {
    /// Symbol name.
    pub name: String,
    /// Modifier (?, *, +, %, or none).
    pub modifier: SymbolModifier,
}

/// Modifier for a symbol in a grammar rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SymbolModifier {
    /// No modifier - plain symbol
    None,
    /// `?` - optional (zero or one)
    Optional,
    /// `*` - zero or more
    ZeroOrMore,
    /// `+` - one or more
    OneOrMore,
    /// `%` - one or more separated by the given symbol
    SeparatedBy(String),
    /// `_` - empty production marker
    Empty,
}

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

/// A grammar definition, typically produced by [`parse_grammar`](crate::parse_grammar)
/// or built programmatically with fields.
#[derive(Debug, Clone)]
pub struct Grammar {
    /// Name of the start symbol.
    pub start: String,
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
    /// Whether this terminal carries a typed payload.
    pub has_type: bool,
    /// Whether this is a precedence terminal (`prec` keyword).
    pub is_prec: bool,
}

/// A rule (production) in the grammar.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Non-terminal name (left-hand side).
    pub name: String,
    /// Alternatives (right-hand sides).
    pub alts: Vec<Alt>,
}

/// An alternative (right-hand side) of a rule.
#[derive(Debug, Clone)]
pub struct Alt {
    /// Terms in this alternative.
    pub terms: Vec<Term>,
    /// Action name (e.g., `=> binop`).
    pub name: String,
}

/// A term in a grammar rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Term {
    /// Plain symbol reference.
    Symbol(String),
    /// `?` - optional (zero or one).
    Optional(String),
    /// `*` - zero or more.
    ZeroOrMore(String),
    /// `+` - one or more.
    OneOrMore(String),
    /// `%` - one or more separated by the given symbol.
    SeparatedBy { symbol: String, sep: String },
    /// `_` - empty production marker.
    Empty,
}
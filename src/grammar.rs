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
    pub name: String,
    pub start: String,
    pub mode: String,  // "lalr" or "lr", default "lalr"
    pub expect_rr: usize,
    pub expect_sr: usize,
    pub terminals: Vec<TerminalDef>,
    pub rules: Vec<Rule>,
}

/// Expected conflict declaration.
#[derive(Debug, Clone)]
pub struct ExpectDecl {
    pub count: usize,
    pub kind: String,  // "rr" or "sr"
}

/// A terminal definition in the grammar.
#[derive(Debug, Clone)]
pub struct TerminalDef {
    pub name: String,
    pub type_name: Option<String>,
    pub is_prec: bool,
}

/// A rule (production) in the grammar.
#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub result_type: Option<String>,
    pub alts: Vec<Alt>,
}

/// An alternative (right-hand side) of a rule.
#[derive(Debug, Clone)]
pub struct Alt {
    pub symbols: Vec<SymbolRef>,
    pub name: Option<String>,
}

/// A symbol reference in a rule with optional modifier.
#[derive(Debug, Clone)]
pub struct SymbolRef {
    pub name: String,
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

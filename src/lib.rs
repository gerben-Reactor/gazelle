pub mod lexer;
pub mod meta;

// Re-export core types from gazelle-core
pub use gazelle_core::grammar;
pub use gazelle_core::lr;
pub use gazelle_core::table;
pub use gazelle_core::runtime;

// Core grammar types
pub use gazelle_core::{
    Grammar, GrammarBuilder, Rule, Symbol, SymbolId, SymbolTable, SymbolInfo,
    Assoc, Precedence,
};

// LR automaton types
pub use gazelle_core::{Item, ItemSet, Automaton, TerminalSet, FirstSets, closure, goto};

// Parse table types
pub use gazelle_core::{ParseTable, Action, ActionEntry, Conflict};

// Runtime parser types
pub use gazelle_core::{Parser, Token, Event};

// Meta-grammar parser
pub use meta::{parse_grammar, parse_grammar_ast, Ast};

// Procedural macro for defining grammars
#[cfg(feature = "macros")]
pub use gazelle_macros::grammar;

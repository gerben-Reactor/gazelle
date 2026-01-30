//! Gazelle parser generator.
//!
//! A typed LR parser generator for Rust.

pub mod grammar;
mod lr;
pub mod table;

// LR algorithm selection
pub use lr::LrAlgorithm;
pub mod runtime;
pub mod lexer;
pub mod meta;

#[cfg(feature = "codegen")]
pub mod codegen;

// Core grammar types
pub use grammar::{
    Grammar, GrammarBuilder, Rule, Symbol, SymbolId, SymbolTable, SymbolInfo,
    Precedence,
};


// Parse table types
pub use table::{ParseTable, CompiledTable, Action, ActionEntry, Conflict, ErrorInfo, ErrorContext};

// Runtime parser types
pub use runtime::{Parser, Token, ParseError};

// Meta-grammar parser
pub use meta::{parse_grammar, parse_grammar_typed, GrammarDef};

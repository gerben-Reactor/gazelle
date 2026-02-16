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
#[doc(hidden)]
pub mod meta;

#[cfg(feature = "codegen")]
pub mod codegen;

// Core grammar types (AST)
pub use grammar::{SymbolId, Grammar, TerminalDef, Rule, Alt, Term, ExpectDecl};


// Parse table types
pub use table::{CompiledTable, Conflict, ErrorInfo};

// Runtime parser types
pub use runtime::{ParseTable, Parser, Token, ParseError, Precedence, ErrorContext, Cst, CstParser, IsNode, AstNode, ReduceFrom, Reducer, Ignore, RecoveryInfo, Repair};

// Meta-grammar parser
pub use meta::parse_grammar;

//! Gazelle parser generator.
//!
//! A typed LR parser generator for Rust.

pub mod grammar;
pub(crate) mod automaton;
mod lr;
pub mod table;

pub mod runtime;
pub mod lexer;
#[doc(hidden)]
pub mod meta;

#[doc(hidden)]
#[cfg(feature = "codegen")]
pub mod codegen;

// Core grammar types (AST)
pub use grammar::{SymbolId, Grammar, TerminalDef, Rule, Alt, Term};


// Parse table types
pub use table::{CompiledTable, Conflict, ErrorInfo};

// Runtime parser types
pub use runtime::{ParseTable, Parser, Token, ParseError, Precedence, ErrorContext, Cst, CstParser, AstNode, FromAstNode, Action, Ignore, RecoveryInfo, Repair};

// Meta-grammar parser
pub use meta::parse_grammar;

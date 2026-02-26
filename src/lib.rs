//! Gazelle parser generator.
//!
//! A typed LR parser generator for Rust.

pub mod automaton;
pub mod grammar;
mod lr;
pub mod table;

pub mod lexer;
#[doc(hidden)]
pub mod meta;
#[cfg(feature = "regex")]
pub mod regex;
pub mod runtime;

#[doc(hidden)]
#[cfg(feature = "codegen")]
pub mod codegen;

// Core grammar types (AST)
pub use grammar::{Alt, Grammar, Rule, SymbolId, Term, TerminalDef};

// Parse table types
pub use table::{CompiledTable, Conflict, ErrorInfo};

// Runtime parser types
pub use runtime::{
    Action, AstNode, Cst, CstParser, ErrorContext, FromAstNode, Ignore, ParseError, ParseTable,
    Parser, Precedence, RecoveryInfo, Repair, Token,
};

// Lexer DFA
#[cfg(feature = "regex")]
pub use lexer::LexerDfa;

// Meta-grammar parser
pub use meta::parse_grammar;

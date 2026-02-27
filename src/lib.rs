//! A typed LR(1) parser generator for Rust with runtime operator precedence
//! and a push-based API for natural lexer feedback.
//!
//! # Quick start
//!
//! Define a grammar with the [`gazelle_macros::gazelle!`] macro, implement the
//! generated `Types` trait to choose your output types, and implement
//! [`Action`] for any node you want to fold:
//!
//! ```rust
//! use gazelle_macros::gazelle;
//!
//! gazelle! {
//!     grammar calc {
//!         start expr;
//!         terminals { NUM: _, PLUS }
//!         expr = expr PLUS NUM => add | NUM => num;
//!     }
//! }
//!
//! struct Eval;
//!
//! impl calc::Types for Eval {
//!     type Error = gazelle::ParseError;
//!     type Num = i64;
//!     type Expr = i64;
//! }
//!
//! impl gazelle::Action<calc::Expr<Self>> for Eval {
//!     fn build(&mut self, node: calc::Expr<Self>) -> Result<i64, gazelle::ParseError> {
//!         Ok(match node {
//!             calc::Expr::Add(left, right) => left + right,
//!             calc::Expr::Num(n) => n,
//!         })
//!     }
//! }
//! ```
//!
//! Then push tokens and collect the result:
//!
//! ```rust,ignore
//! let mut parser = calc::Parser::<Eval>::new();
//! let mut actions = Eval;
//! for tok in tokens {
//!     parser.push(tok, &mut actions).map_err(|e| parser.format_error(&e, None, None))?;
//! }
//! let result = parser.finish(&mut actions).map_err(|(p, e)| p.format_error(&e, None, None))?;
//! ```
//!
//! See `examples/hello.rs` for a complete runnable version.
//!
//! # Key features
//!
//! - **Runtime operator precedence**: `prec` terminals carry [`Precedence`] at
//!   parse time, so one grammar rule handles any number of operator levels —
//!   including user-defined operators.
//! - **Push-based parsing**: you drive the loop, so the lexer can inspect
//!   parser state between tokens (solves C's typedef problem).
//! - **CST/AST continuum**: set associated types to the generated enum for a
//!   full CST, to a custom type for an AST, or to [`Ignore`] to discard.
//! - **Library API**: build [`CompiledTable`]s programmatically for dynamic
//!   grammars, analyzers, or conflict debuggers.

pub mod automaton;
pub mod grammar;
mod lr;
pub mod table;

pub mod lexer;
#[cfg(not(feature = "bootstrap"))]
#[doc(hidden)]
pub mod meta;
#[cfg(not(feature = "bootstrap_regex"))]
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
pub use lexer::LexerDfa;

// Meta-grammar parser
#[cfg(not(feature = "bootstrap"))]
pub use meta::parse_grammar;

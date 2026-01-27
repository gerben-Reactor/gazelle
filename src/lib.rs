//! Gazelle parser generator.
//!
//! A typed LR parser generator for Rust.

pub mod grammar;
pub mod lr;
pub mod table;
pub mod runtime;
pub mod lexer;
pub mod meta;

#[cfg(feature = "codegen")]
pub mod codegen;

// Core grammar types
pub use grammar::{
    Grammar, GrammarBuilder, Rule, Symbol, SymbolId, SymbolTable, SymbolInfo,
    Assoc, Precedence,
};

// LR automaton types
pub use lr::{Item, ItemSet, Automaton, TerminalSet, FirstSets, closure, goto};

// Parse table types
pub use table::{ParseTable, CompiledTable, Action, ActionEntry, Conflict};

// Runtime parser types
pub use runtime::{Parser, Token, Event};

// Meta-grammar parser
pub use meta::{parse_grammar, parse_grammar_typed, GrammarDef};

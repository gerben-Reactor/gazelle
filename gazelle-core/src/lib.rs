//! Core types and algorithms for Gazelle parser generator.
//!
//! This crate contains the grammar, LR automaton, and parse table
//! implementations used by both the main Gazelle crate and the
//! proc macro crate.

pub mod grammar;
pub mod lr;
pub mod table;
pub mod runtime;
#[cfg(feature = "codegen")]
pub mod codegen;
pub mod meta_bootstrap;

// Core grammar types
pub use grammar::{
    Grammar, GrammarBuilder, Rule, Symbol, SymbolId, SymbolTable, SymbolInfo,
    Assoc, Precedence,
};

// LR automaton types
pub use lr::{Item, ItemSet, Automaton, TerminalSet, FirstSets, closure, goto};

// Parse table types
pub use table::{ParseTable, Action, ActionEntry, Conflict};

// Runtime parser types
pub use runtime::{Parser, Token, Event};

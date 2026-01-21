pub mod grammar;
pub mod lexer;
pub mod lr;
pub mod table;
pub mod runtime;
pub mod meta;

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

// Meta-grammar parser
pub use meta::parse_grammar;

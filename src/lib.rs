pub mod grammar;
pub mod lexer;
pub mod lr;
pub mod table;
pub mod runtime;
pub mod meta;

// Original string-based types (for grammar construction)
pub use grammar::{Grammar, Rule, Symbol, Assoc, Precedence, t, pt, nt};
pub use lr::{Item, ItemSet, Automaton, closure, goto, compute_first_sets};
pub use table::{ParseTable, Action, Conflict};
pub use runtime::{Parser, Token, Event};
pub use meta::parse_grammar;

// New integer-based types (for efficient parsing)
pub use grammar::{SymbolId, SymbolTable, SymbolInfo, InternedGrammar, InternedRule};
pub use lr::{
    InternedItem, InternedItemSet, InternedAutomaton,
    TerminalSet, FirstSets,
    interned_closure, interned_goto,
};
pub use table::{CompactParseTable, ActionEntry, CompactConflict};
pub use runtime::{CompactParser, CompactToken, CompactEvent};
pub use meta::parse_grammar_compact;

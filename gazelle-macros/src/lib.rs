//! Procedural macros for Gazelle parser generator.
//!
//! This crate provides the `grammar!` macro that allows defining grammars
//! in Rust with type-safe parsers generated at compile time.
//!
//! # Example
//!
//! ```ignore
//! grammar! {
//!     pub grammar Calc {
//!         terminals {
//!             NUM: f64,
//!             LPAREN,
//!             RPAREN,
//!         }
//!
//!         prec_terminals {
//!             OP: Operator,
//!         }
//!
//!         expr: Expr = expr OP expr | atom;
//!         atom: Atom = NUM | LPAREN expr RPAREN;
//!     }
//! }
//! ```

use proc_macro::TokenStream;

mod codegen;
mod ir;
mod parse;
mod validate;

/// Define a grammar and generate a type-safe parser.
///
/// See the crate-level documentation for usage examples.
#[proc_macro]
pub fn grammar(input: TokenStream) -> TokenStream {
    let ir = match syn::parse::<ir::GrammarIr>(input) {
        Ok(ir) => ir,
        Err(err) => return err.to_compile_error().into(),
    };

    if let Err(err) = validate::validate(&ir) {
        return err.to_compile_error().into();
    }

    match codegen::generate(&ir) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

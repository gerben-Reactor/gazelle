//! Code generation for grammar parsers.
//!
//! This module generates Rust source code for type-safe LR parsers.

mod parser;
mod reduction;
mod table;
mod terminal;

use proc_macro2::TokenStream;
use quote::quote;

use crate::grammar::Grammar;
use crate::lr::{GrammarInternal, to_grammar_internal};

/// Context for code generation.
///
/// Contains all information needed to generate parser code:
/// - The grammar structure (with symbols, rules, type info)
/// - Naming information
#[derive(Debug, Clone)]
pub struct CodegenContext {
    /// The grammar with symbols, rules, and type info.
    pub(crate) grammar: GrammarInternal,
    /// Visibility for generated code (e.g., "pub", "pub(crate)", "").
    pub visibility: String,
    /// Grammar name (for naming generated types).
    pub name: String,

    /// If true, use absolute paths (`::gazelle::`). If false, use relative
    /// paths (`gazelle::`) which requires `use ... as gazelle;` in scope.
    pub use_absolute_path: bool,

    /// Start symbol name.
    pub start_symbol: String,

    /// Expected reduce/reduce conflicts (0 = none expected, error if different).
    pub expect_rr: usize,
    /// Expected shift/reduce conflicts (0 = none expected, error if different).
    pub expect_sr: usize,
    /// LR algorithm to use (LALR(1) or LR(1)).
    pub algorithm: crate::LrAlgorithm,
}

impl CodegenContext {
    /// Build a CodegenContext from a GrammarDef.
    pub fn from_grammar(
        grammar_def: &Grammar,
        name: &str,
        visibility: &str,
        use_absolute_path: bool,
    ) -> Result<Self, String> {
        let grammar = to_grammar_internal(grammar_def)?;

        let algorithm = match grammar_def.mode.as_str() {
            "lr" | "lr1" => crate::LrAlgorithm::Lr1,
            _ => crate::LrAlgorithm::Lalr1,
        };

        Ok(CodegenContext {
            grammar,
            visibility: visibility.to_string(),
            name: name.to_string(),
            use_absolute_path,
            start_symbol: grammar_def.start.clone(),
            expect_rr: grammar_def.expect_rr,
            expect_sr: grammar_def.expect_sr,
            algorithm,
        })
    }

    /// Get the gazelle path prefix as a string.
    pub fn core_path(&self) -> &'static str {
        if self.use_absolute_path {
            "::gazelle"
        } else {
            "gazelle"
        }
    }

    /// Get the gazelle path prefix as tokens.
    pub fn core_path_tokens(&self) -> TokenStream {
        if self.use_absolute_path {
            quote! { ::gazelle }
        } else {
            quote! { gazelle }
        }
    }

    /// Get a symbol's type by name.
    pub fn get_type(&self, name: &str) -> Option<&String> {
        let sym = self.grammar.symbols.get(name)?;
        self.grammar.types.get(&sym.id())?.as_ref()
    }
}

/// Generate all code for a grammar as a TokenStream.
pub fn generate_tokens(ctx: &CodegenContext) -> Result<TokenStream, String> {
    let (compiled, info) = table::build_table(ctx)?;

    let table_statics = table::generate_table_statics(ctx, &compiled, &info);
    let terminal_code = terminal::generate(ctx, &info);
    let parser_code = parser::generate(ctx, &info)?;

    Ok(quote! {
        #table_statics

        #terminal_code

        #parser_code
    })
}

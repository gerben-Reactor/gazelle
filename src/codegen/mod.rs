//! Code generation for grammar parsers.
//!
//! This module generates Rust source code for type-safe LR parsers.

mod parser;
mod reduction;
mod table;
mod terminal;

use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::grammar::{Grammar, SymbolId};
use crate::meta::{GrammarDef, desugar_modifiers, grammar_def_to_grammar};

/// The kind of action for a rule alternative.
#[derive(Debug, Clone)]
pub enum ActionKind {
    /// No action name - auto-handle (passthrough or structural)
    None,
    /// User-defined action name (e.g., @binop)
    Named(String),
    /// Synthetic: wrap value in Some (from `?` modifier)
    OptSome,
    /// Synthetic: create None (from `?` modifier)
    OptNone,
    /// Synthetic: create empty Vec (from `*` modifier)
    VecEmpty,
    /// Synthetic: create Vec with single element (from `+` modifier)
    VecSingle,
    /// Synthetic: append to Vec (from `*` or `+` modifier)
    VecAppend,
}

/// Information about a single alternative in a rule.
#[derive(Debug, Clone)]
pub struct AlternativeInfo {
    /// Action for this alternative.
    pub action: ActionKind,
    /// Symbols in the RHS: (symbol_name, type_if_any).
    /// Symbols without types won't appear in trait method parameters.
    pub symbols: Vec<(String, Option<String>)>,
}

/// Information about a rule (non-terminal).
#[derive(Debug, Clone)]
pub struct RuleInfo {
    /// Non-terminal name.
    pub name: String,
    /// Result type. None = structural (no user type).
    pub result_type: Option<String>,
    /// Alternatives for this rule.
    pub alternatives: Vec<AlternativeInfo>,
}

/// Context for code generation.
///
/// Contains all information needed to generate parser code:
/// - The grammar structure
/// - Type information for terminals and rules
/// - Naming information
#[derive(Debug, Clone)]
pub struct CodegenContext {
    /// The grammar with symbols and rules.
    pub grammar: Grammar,
    /// Visibility for generated code (e.g., "pub", "pub(crate)", "").
    pub visibility: String,
    /// Grammar name (for naming generated types).
    pub name: String,

    /// Payload types for regular terminals. None = unit type (no payload).
    pub terminal_types: BTreeMap<SymbolId, Option<String>>,
    /// Payload types for precedence terminals. None = unit type (no payload).
    pub prec_terminal_types: BTreeMap<SymbolId, Option<String>>,

    /// If true, use absolute paths (`::gazelle::`). If false, use relative
    /// paths (`gazelle::`) which requires `use ... as gazelle;` in scope.
    pub use_absolute_path: bool,

    /// Detailed rule information including alternatives and their names.
    pub rules: Vec<RuleInfo>,

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
    pub fn from_grammar_def(
        grammar_def: &GrammarDef,
        visibility: &str,
        use_absolute_path: bool,
    ) -> Result<Self, String> {
        // Clone and desugar modifiers first
        let mut grammar_def = grammar_def.clone();
        desugar_modifiers(&mut grammar_def);

        let grammar_name = grammar_def.name.clone();

        // Build the grammar using the shared function
        let grammar = grammar_def_to_grammar(grammar_def.clone())?;

        // Extract type information from grammar_def, using IDs from built grammar
        let mut terminal_types: BTreeMap<SymbolId, Option<String>> = BTreeMap::new();
        let mut prec_terminal_types: BTreeMap<SymbolId, Option<String>> = BTreeMap::new();

        for def in &grammar_def.terminals {
            let sym = grammar.symbols.get(&def.name).expect("terminal should exist");
            if def.is_prec {
                prec_terminal_types.insert(sym.id(), def.type_name.clone());
            } else {
                terminal_types.insert(sym.id(), def.type_name.clone());
            }
        }

        // Build detailed rule info with types
        let mut rules = Vec::new();
        for rule in &grammar_def.rules {
            let mut alternatives = Vec::new();
            for alt in &rule.alts {
                let symbols_with_types: Vec<_> = alt.symbols.iter().map(|sym| {
                    let sym_type = grammar_def.terminals.iter()
                        .find(|t| t.name == sym.name)
                        .and_then(|t| t.type_name.clone())
                        .or_else(|| {
                            grammar_def.rules.iter()
                                .find(|r| r.name == sym.name)
                                .and_then(|r| r.result_type.clone())
                        });
                    (sym.name.clone(), sym_type)
                }).collect();

                let action = match alt.name.as_deref() {
                    None => ActionKind::None,
                    Some("__some") => ActionKind::OptSome,
                    Some("__none") => ActionKind::OptNone,
                    Some("__empty") => ActionKind::VecEmpty,
                    Some("__single") => ActionKind::VecSingle,
                    Some("__append") => ActionKind::VecAppend,
                    Some(s) => ActionKind::Named(s.to_string()),
                };
                alternatives.push(AlternativeInfo {
                    action,
                    symbols: symbols_with_types,
                });
            }

            rules.push(RuleInfo {
                name: rule.name.clone(),
                result_type: rule.result_type.clone(),
                alternatives,
            });
        }

        // Parse mode string to algorithm
        let algorithm = match grammar_def.mode.as_str() {
            "lr" | "lr1" => crate::LrAlgorithm::Lr1,
            _ => crate::LrAlgorithm::Lalr1,  // default
        };

        Ok(CodegenContext {
            grammar,
            visibility: visibility.to_string(),
            name: grammar_name,
            terminal_types,
            prec_terminal_types,
            use_absolute_path,
            rules,
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

    /// Get a terminal's payload type by name.
    pub fn get_terminal_type(&self, name: &str) -> Option<&Option<String>> {
        let sym = self.grammar.symbols.get(name)?;
        self.terminal_types.get(&sym.id())
    }

    /// Get a prec_terminal's payload type by name.
    pub fn get_prec_terminal_type(&self, name: &str) -> Option<&Option<String>> {
        let sym = self.grammar.symbols.get(name)?;
        self.prec_terminal_types.get(&sym.id())
    }

    /// Get a rule's result type by name.
    pub fn get_rule_result_type(&self, name: &str) -> Option<&String> {
        self.rules.iter()
            .find(|r| r.name == name)
            .and_then(|r| r.result_type.as_ref())
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


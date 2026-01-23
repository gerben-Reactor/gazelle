//! Code generation for grammar parsers.
//!
//! This module generates Rust source code for type-safe LR parsers.

mod parser;
mod reduction;
mod table;
mod terminal;

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;

use crate::grammar::{Grammar, SymbolId};

/// Information about a single alternative in a rule.
#[derive(Debug, Clone)]
pub struct AlternativeInfo {
    /// Reduction name (from @name). None = auto-handle (passthrough or structural).
    pub name: Option<String>,
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
    pub terminal_types: HashMap<SymbolId, Option<String>>,
    /// Payload types for precedence terminals. None = unit type (no payload).
    pub prec_terminal_types: HashMap<SymbolId, Option<String>>,
    /// Result types for rules, indexed by rule index.
    pub rule_result_types: Vec<String>,

    /// Symbol names by ID.
    pub symbol_names: HashMap<SymbolId, String>,
    /// Rule names (non-terminal names for each rule's LHS).
    pub rule_names: Vec<String>,

    /// If true, use absolute paths (`::gazelle_core::`). If false, use relative
    /// paths (`gazelle_core::`) which requires `use ... as gazelle_core;` in scope.
    pub use_absolute_path: bool,

    /// Detailed rule information including alternatives and their names.
    pub rules: Vec<RuleInfo>,

    /// Start symbol name.
    pub start_symbol: String,
}

impl CodegenContext {
    /// Get the gazelle_core path prefix as a string.
    pub fn core_path(&self) -> &'static str {
        if self.use_absolute_path {
            "::gazelle_core"
        } else {
            "gazelle_core"
        }
    }

    /// Get the gazelle_core path prefix as tokens.
    pub fn core_path_tokens(&self) -> TokenStream {
        if self.use_absolute_path {
            quote! { ::gazelle_core }
        } else {
            quote! { gazelle_core }
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
        for (i, rule_name) in self.rule_names.iter().enumerate() {
            if rule_name == name {
                return self.rule_result_types.get(i);
            }
        }
        None
    }

    /// Convert a name to PascalCase for enum variants.
    pub fn to_pascal_case(s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in s.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c.to_ascii_lowercase());
            }
        }

        result
    }
}

/// Generate all code for a grammar.
///
/// Returns the complete Rust source code as a string.
pub fn generate(ctx: &CodegenContext) -> Result<String, String> {
    // Extract table data for code generation
    let table_data = table::extract_table_data(ctx)?;

    let table_statics = table::generate_table_statics(ctx, &table_data);
    let terminal_code = terminal::generate(ctx, &table_data);
    let parser_code = parser::generate(ctx, &table_data)?;

    let combined = quote! {
        #table_statics

        #terminal_code

        #parser_code
    };

    Ok(combined.to_string())
}

/// Check if a type name is likely Copy (simple heuristic).
pub(crate) fn is_copy_type(ty: &str) -> bool {
    matches!(
        ty,
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
            | "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
            | "f32" | "f64" | "bool" | "char" | "()"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(CodegenContext::to_pascal_case("NUM"), "Num");
        assert_eq!(CodegenContext::to_pascal_case("LPAREN"), "Lparen");
        assert_eq!(CodegenContext::to_pascal_case("LEFT_PAREN"), "LeftParen");
        assert_eq!(CodegenContext::to_pascal_case("foo"), "Foo");
        assert_eq!(CodegenContext::to_pascal_case("foo_bar"), "FooBar");
    }
}

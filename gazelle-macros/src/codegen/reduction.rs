//! Reduction enum code generation.

use quote::format_ident;
use syn::Type;

use crate::codegen::table::TableData;
use crate::ir::{GrammarIr, GrammarSymbol};

/// Information needed to map rule indices to reduction variants.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReductionInfo {
    /// Variant name for this reduction.
    pub variant_name: syn::Ident,
    /// Types of the values to pop from the stack (in reverse order).
    /// None means unit-type terminal (don't pop a value).
    pub value_types: Vec<Option<Type>>,
    /// Symbol kinds for the RHS (for determining what to pop).
    pub rhs_kinds: Vec<SymbolKind>,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    UnitTerminal,
    PayloadTerminal,
    PrecTerminal,
    NonTerminal,
}

/// Build reduction info for all rules.
pub fn build_reduction_info(grammar: &GrammarIr, table_data: &TableData) -> Vec<ReductionInfo> {
    let mut result = Vec::new();

    for (rule_idx, rule_info) in table_data.rule_mapping.iter().enumerate() {
        let variant_name = generate_variant_name(rule_info, rule_idx);

        let mut value_types = Vec::new();
        let mut rhs_kinds = Vec::new();

        for symbol in &rule_info.rhs_symbols {
            let name = symbol.name().to_string();

            match symbol {
                GrammarSymbol::Terminal(_) => {
                    if let Some(terminal) = grammar.get_terminal(&name) {
                        if let Some(ty) = &terminal.payload_type {
                            value_types.push(Some(ty.clone()));
                            rhs_kinds.push(SymbolKind::PayloadTerminal);
                        } else {
                            value_types.push(None);
                            rhs_kinds.push(SymbolKind::UnitTerminal);
                        }
                    } else if let Some(prec_terminal) = grammar.get_prec_terminal(&name) {
                        value_types.push(Some(prec_terminal.payload_type.clone()));
                        rhs_kinds.push(SymbolKind::PrecTerminal);
                    }
                }
                GrammarSymbol::NonTerminal(_) => {
                    if let Some(rule) = grammar.get_rule(&name) {
                        value_types.push(Some(rule.result_type.clone()));
                        rhs_kinds.push(SymbolKind::NonTerminal);
                    }
                }
            }
        }

        result.push(ReductionInfo {
            variant_name,
            value_types,
            rhs_kinds,
        });
    }

    result
}

/// Generate a variant name for a rule.
fn generate_variant_name(
    rule_info: &crate::codegen::table::RuleInfo,
    _rule_idx: usize,
) -> syn::Ident {
    let nt_name = to_pascal_case(&rule_info.non_terminal_name);

    // Build suffix from symbol names
    let suffix: String = rule_info
        .rhs_symbols
        .iter()
        .map(|sym| to_pascal_case(&sym.name().to_string()))
        .collect();

    if suffix.is_empty() {
        // Epsilon production
        format_ident!("{}Empty", nt_name)
    } else {
        format_ident!("{}{}", nt_name, suffix)
    }
}

/// Convert a name to PascalCase.
fn to_pascal_case(s: &str) -> String {
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

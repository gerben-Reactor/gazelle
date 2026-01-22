//! Terminal enum code generation.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::codegen::table::TableData;
use crate::ir::GrammarIr;

/// Generate the terminal enum and its implementations.
pub fn generate(grammar: &GrammarIr, table_data: &TableData) -> TokenStream {
    let vis = &grammar.visibility;
    let name = &grammar.name;
    let terminal_enum_name = format_ident!("{}Terminal", name);

    // Generate enum variants
    let variants = generate_variants(grammar);

    // Generate symbol_id() method body
    let symbol_id_arms = generate_symbol_id_arms(grammar, table_data);

    // Generate to_token() method body
    let to_token_arms = generate_to_token_arms(grammar);

    quote! {
        /// Terminal symbols for the parser.
        #[derive(Debug, Clone)]
        #vis enum #terminal_enum_name {
            #variants
        }

        impl #terminal_enum_name {
            /// Get the symbol ID for this terminal.
            pub fn symbol_id(&self) -> ::gazelle_core::SymbolId {
                match self {
                    #symbol_id_arms
                }
            }

            /// Convert to a gazelle Token for parsing.
            pub fn to_token(&self, symbol_ids: &impl Fn(&str) -> ::gazelle_core::SymbolId) -> ::gazelle_core::Token {
                match self {
                    #to_token_arms
                }
            }
        }
    }
}

/// Generate enum variants for all terminals.
fn generate_variants(grammar: &GrammarIr) -> TokenStream {
    let mut variants = Vec::new();

    // Regular terminals
    for terminal in &grammar.terminals {
        let variant_name = GrammarIr::terminal_variant_name(&terminal.name);

        if let Some(ty) = &terminal.payload_type {
            variants.push(quote! { #variant_name(#ty) });
        } else {
            variants.push(quote! { #variant_name });
        }
    }

    // Precedence terminals (add Precedence automatically)
    for prec_terminal in &grammar.prec_terminals {
        let variant_name = GrammarIr::terminal_variant_name(&prec_terminal.name);
        let ty = &prec_terminal.payload_type;
        variants.push(quote! { #variant_name(#ty, ::gazelle_core::Precedence) });
    }

    quote! { #(#variants,)* }
}

/// Generate match arms for symbol_id().
fn generate_symbol_id_arms(grammar: &GrammarIr, table_data: &TableData) -> TokenStream {
    let mut arms = Vec::new();

    // Use actual symbol IDs from the table
    for terminal in &grammar.terminals {
        let variant_name = GrammarIr::terminal_variant_name(&terminal.name);
        let name_str = terminal.name.to_string();

        // Find the actual ID from the table
        let id = table_data
            .terminal_ids
            .iter()
            .find(|(n, _)| n == &name_str)
            .map(|(_, id)| *id)
            .unwrap_or(0);

        if terminal.payload_type.is_some() {
            arms.push(quote! { Self::#variant_name(_) => ::gazelle_core::SymbolId(#id) });
        } else {
            arms.push(quote! { Self::#variant_name => ::gazelle_core::SymbolId(#id) });
        }
    }

    for prec_terminal in &grammar.prec_terminals {
        let variant_name = GrammarIr::terminal_variant_name(&prec_terminal.name);
        let name_str = prec_terminal.name.to_string();

        // Find the actual ID from the table
        let id = table_data
            .terminal_ids
            .iter()
            .find(|(n, _)| n == &name_str)
            .map(|(_, id)| *id)
            .unwrap_or(0);

        arms.push(quote! { Self::#variant_name(_, _) => ::gazelle_core::SymbolId(#id) });
    }

    quote! { #(#arms,)* }
}

/// Generate match arms for to_token().
fn generate_to_token_arms(grammar: &GrammarIr) -> TokenStream {
    let mut arms = Vec::new();

    for terminal in &grammar.terminals {
        let variant_name = GrammarIr::terminal_variant_name(&terminal.name);
        let name_str = terminal.name.to_string();

        if terminal.payload_type.is_some() {
            arms.push(quote! {
                Self::#variant_name(_) => ::gazelle_core::Token::new(symbol_ids(#name_str), #name_str)
            });
        } else {
            arms.push(quote! {
                Self::#variant_name => ::gazelle_core::Token::new(symbol_ids(#name_str), #name_str)
            });
        }
    }

    for prec_terminal in &grammar.prec_terminals {
        let variant_name = GrammarIr::terminal_variant_name(&prec_terminal.name);
        let name_str = prec_terminal.name.to_string();

        arms.push(quote! {
            Self::#variant_name(_, prec) => ::gazelle_core::Token::with_prec(symbol_ids(#name_str), #name_str, *prec)
        });
    }

    quote! { #(#arms,)* }
}

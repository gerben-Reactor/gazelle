//! Terminal enum code generation.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::table::TableData;
use super::CodegenContext;

/// Generate the terminal enum and its implementations.
pub fn generate(ctx: &CodegenContext, table_data: &TableData) -> TokenStream {
    let vis: TokenStream = ctx.visibility.parse().unwrap_or_default();
    let terminal_enum = format_ident!("{}Terminal", ctx.name);
    let actions_trait = format_ident!("{}Actions", ctx.name);
    let core_path = ctx.core_path_tokens();

    // Check if we have any typed terminals
    let has_typed_terminals = ctx.terminal_types.values().any(|t| t.is_some())
        || ctx.prec_terminal_types.values().any(|t| t.is_some());

    // Build enum variants
    let mut variants = Vec::new();

    // Regular terminals
    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = format_ident!("{}", CodegenContext::to_pascal_case(name));
            if payload_type.is_some() {
                // Use terminal NAME as associated type (e.g., A::Num, not A::i32)
                let assoc_type = format_ident!("{}", CodegenContext::to_pascal_case(name));
                variants.push(quote! { #variant_name(A::#assoc_type) });
            } else {
                variants.push(quote! { #variant_name });
            }
        }
    }

    // Precedence terminals
    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = format_ident!("{}", CodegenContext::to_pascal_case(name));
            if payload_type.is_some() {
                // Use terminal NAME as associated type
                let assoc_type = format_ident!("{}", CodegenContext::to_pascal_case(name));
                variants.push(quote! { #variant_name(A::#assoc_type, #core_path::Precedence) });
            } else {
                variants.push(quote! { #variant_name(#core_path::Precedence) });
            }
        }
    }

    // Always add phantom data to use the A parameter
    variants.push(quote! {
        #[doc(hidden)]
        __Phantom(std::marker::PhantomData<A>)
    });

    // Build symbol_id match arms
    let symbol_id_arms = build_symbol_id_arms(ctx, table_data, &core_path, has_typed_terminals);

    // Build to_token match arms
    let to_token_arms = build_to_token_arms(ctx, &core_path, has_typed_terminals);

    // Build precedence match arms
    let precedence_arms = build_precedence_arms(ctx, has_typed_terminals);

    quote! {
        /// Terminal symbols for the parser.
        #vis enum #terminal_enum<A: #actions_trait> {
            #(#variants),*
        }

        impl<A: #actions_trait> #terminal_enum<A> {
            /// Get the symbol ID for this terminal.
            pub fn symbol_id(&self) -> #core_path::SymbolId {
                match self {
                    #(#symbol_id_arms)*
                }
            }

            /// Convert to a gazelle Token for parsing.
            pub fn to_token(&self, symbol_ids: &impl Fn(&str) -> #core_path::SymbolId) -> #core_path::Token {
                match self {
                    #(#to_token_arms)*
                }
            }

            /// Get precedence for runtime precedence comparison.
            /// Returns (level, assoc) where assoc: 0=left, 1=right.
            pub fn precedence(&self) -> Option<(u8, u8)> {
                match self {
                    #(#precedence_arms)*
                }
            }
        }
    }
}

fn build_symbol_id_arms(ctx: &CodegenContext, table_data: &TableData, core_path: &TokenStream, has_typed_terminals: bool) -> Vec<TokenStream> {
    let mut arms = Vec::new();

    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = format_ident!("{}", CodegenContext::to_pascal_case(name));
            let table_id = table_data.terminal_ids.iter()
                .find(|(n, _)| n == name)
                .map(|(_, id)| *id)
                .unwrap_or(0);

            if payload_type.is_some() {
                arms.push(quote! { Self::#variant_name(_) => #core_path::SymbolId(#table_id), });
            } else {
                arms.push(quote! { Self::#variant_name => #core_path::SymbolId(#table_id), });
            }
        }
    }

    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = format_ident!("{}", CodegenContext::to_pascal_case(name));
            let table_id = table_data.terminal_ids.iter()
                .find(|(n, _)| n == name)
                .map(|(_, id)| *id)
                .unwrap_or(0);

            if payload_type.is_some() {
                arms.push(quote! { Self::#variant_name(_, _) => #core_path::SymbolId(#table_id), });
            } else {
                arms.push(quote! { Self::#variant_name(_) => #core_path::SymbolId(#table_id), });
            }
        }
    }

    // Always add __Phantom arm since we always include the variant
    arms.push(quote! { Self::__Phantom(_) => unreachable!(), });

    arms
}

fn build_to_token_arms(ctx: &CodegenContext, core_path: &TokenStream, _has_typed_terminals: bool) -> Vec<TokenStream> {
    let mut arms = Vec::new();

    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = format_ident!("{}", CodegenContext::to_pascal_case(name));
            if payload_type.is_some() {
                arms.push(quote! {
                    Self::#variant_name(_) => #core_path::Token::new(symbol_ids(#name), #name),
                });
            } else {
                arms.push(quote! {
                    Self::#variant_name => #core_path::Token::new(symbol_ids(#name), #name),
                });
            }
        }
    }

    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = format_ident!("{}", CodegenContext::to_pascal_case(name));
            if payload_type.is_some() {
                arms.push(quote! {
                    Self::#variant_name(_, prec) => #core_path::Token::with_prec(symbol_ids(#name), #name, *prec),
                });
            } else {
                arms.push(quote! {
                    Self::#variant_name(prec) => #core_path::Token::with_prec(symbol_ids(#name), #name, *prec),
                });
            }
        }
    }

    // Always add __Phantom arm
    arms.push(quote! { Self::__Phantom(_) => unreachable!(), });

    arms
}

fn build_precedence_arms(ctx: &CodegenContext, _has_typed_terminals: bool) -> Vec<TokenStream> {
    let mut arms = Vec::new();

    // Regular terminals have no precedence
    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = format_ident!("{}", CodegenContext::to_pascal_case(name));
            if payload_type.is_some() {
                arms.push(quote! { Self::#variant_name(_) => None, });
            } else {
                arms.push(quote! { Self::#variant_name => None, });
            }
        }
    }

    // Prec terminals extract precedence
    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = format_ident!("{}", CodegenContext::to_pascal_case(name));
            if payload_type.is_some() {
                arms.push(quote! {
                    Self::#variant_name(_, prec) => Some((prec.level(), prec.assoc())),
                });
            } else {
                arms.push(quote! {
                    Self::#variant_name(prec) => Some((prec.level(), prec.assoc())),
                });
            }
        }
    }

    // Always add __Phantom arm
    arms.push(quote! { Self::__Phantom(_) => unreachable!(), });

    arms
}

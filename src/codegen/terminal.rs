//! Terminal enum code generation.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::table::CodegenTableInfo;
use super::CodegenContext;

/// Generate the terminal enum and its implementations.
pub fn generate(ctx: &CodegenContext, info: &CodegenTableInfo) -> TokenStream {
    let vis: TokenStream = ctx.visibility.parse().unwrap_or_default();
    let terminal_enum = format_ident!("{}Terminal", ctx.name);
    let types_trait = format_ident!("{}Types", ctx.name);
    let core_path = ctx.core_path_tokens();

    // Check if we have any typed terminals
    let has_typed_terminals = ctx.grammar.terminal_types.values().any(|t| t.is_some())
        || ctx.grammar.prec_terminal_types.values().any(|t| t.is_some());

    // Build enum variants
    let mut variants = Vec::new();

    // Regular terminals
    for (&id, payload_type) in &ctx.grammar.terminal_types {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", name);
        if let Some(type_name) = payload_type {
            let assoc_type = format_ident!("{}", type_name);
            variants.push(quote! { #variant_name(A::#assoc_type) });
        } else {
            variants.push(quote! { #variant_name });
        }
    }

    // Precedence terminals
    for (&id, payload_type) in &ctx.grammar.prec_terminal_types {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", name);
        if let Some(type_name) = payload_type {
            let assoc_type = format_ident!("{}", type_name);
            variants.push(quote! { #variant_name(A::#assoc_type, #core_path::Precedence) });
        } else {
            variants.push(quote! { #variant_name(#core_path::Precedence) });
        }
    }

    // Always add phantom data to use the A parameter
    variants.push(quote! {
        #[doc(hidden)]
        __Phantom(std::marker::PhantomData<A>)
    });

    // Build symbol_id match arms
    let symbol_id_arms = build_symbol_id_arms(ctx, info, &core_path, has_typed_terminals);

    // Build to_token match arms
    let to_token_arms = build_to_token_arms(ctx, &core_path, has_typed_terminals);

    // Build precedence match arms
    let precedence_arms = build_precedence_arms(ctx, has_typed_terminals);

    quote! {
        /// Terminal symbols for the parser.
        #[allow(non_camel_case_types)]
        #vis enum #terminal_enum<A: #types_trait> {
            #(#variants),*
        }

        impl<A: #types_trait> #terminal_enum<A> {
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
            pub fn precedence(&self) -> Option<#core_path::Precedence> {
                match self {
                    #(#precedence_arms)*
                }
            }
        }
    }
}

fn build_symbol_id_arms(ctx: &CodegenContext, info: &CodegenTableInfo, core_path: &TokenStream, _has_typed_terminals: bool) -> Vec<TokenStream> {
    let mut arms = Vec::new();

    for (&id, payload_type) in &ctx.grammar.terminal_types {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", name);
        let table_id = info.terminal_ids.iter()
            .find(|(n, _)| n == name)
            .map(|(_, id)| *id)
            .unwrap_or(0);

        if payload_type.is_some() {
            arms.push(quote! { Self::#variant_name(_) => #core_path::SymbolId(#table_id), });
        } else {
            arms.push(quote! { Self::#variant_name => #core_path::SymbolId(#table_id), });
        }
    }

    for (&id, payload_type) in &ctx.grammar.prec_terminal_types {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", name);
        let table_id = info.terminal_ids.iter()
            .find(|(n, _)| n == name)
            .map(|(_, id)| *id)
            .unwrap_or(0);

        if payload_type.is_some() {
            arms.push(quote! { Self::#variant_name(_, _) => #core_path::SymbolId(#table_id), });
        } else {
            arms.push(quote! { Self::#variant_name(_) => #core_path::SymbolId(#table_id), });
        }
    }

    // Always add __Phantom arm since we always include the variant
    arms.push(quote! { Self::__Phantom(_) => unreachable!(), });

    arms
}

fn build_to_token_arms(ctx: &CodegenContext, core_path: &TokenStream, _has_typed_terminals: bool) -> Vec<TokenStream> {
    let mut arms = Vec::new();

    for (&id, payload_type) in &ctx.grammar.terminal_types {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", name);
        if payload_type.is_some() {
            arms.push(quote! {
                Self::#variant_name(_) => #core_path::Token::new(symbol_ids(#name)),
            });
        } else {
            arms.push(quote! {
                Self::#variant_name => #core_path::Token::new(symbol_ids(#name)),
            });
        }
    }

    for (&id, payload_type) in &ctx.grammar.prec_terminal_types {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", name);
        if payload_type.is_some() {
            arms.push(quote! {
                Self::#variant_name(_, prec) => #core_path::Token {
                    terminal: symbol_ids(#name),
                    prec: Some(*prec),
                },
            });
        } else {
            arms.push(quote! {
                Self::#variant_name(prec) => #core_path::Token {
                    terminal: symbol_ids(#name),
                    prec: Some(*prec),
                },
            });
        }
    }

    // Always add __Phantom arm
    arms.push(quote! { Self::__Phantom(_) => unreachable!(), });

    arms
}

fn build_precedence_arms(ctx: &CodegenContext, _has_typed_terminals: bool) -> Vec<TokenStream> {
    let mut arms = Vec::new();

    // Regular terminals have no precedence
    for (&id, payload_type) in &ctx.grammar.terminal_types {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", name);
        if payload_type.is_some() {
            arms.push(quote! { Self::#variant_name(_) => None, });
        } else {
            arms.push(quote! { Self::#variant_name => None, });
        }
    }

    // Prec terminals extract precedence
    for (&id, payload_type) in &ctx.grammar.prec_terminal_types {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", name);
        if payload_type.is_some() {
            arms.push(quote! {
                Self::#variant_name(_, prec) => Some(*prec),
            });
        } else {
            arms.push(quote! {
                Self::#variant_name(prec) => Some(*prec),
            });
        }
    }

    // Always add __Phantom arm
    arms.push(quote! { Self::__Phantom(_) => unreachable!(), });

    arms
}

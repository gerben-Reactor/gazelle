//! Terminal enum code generation.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::CodegenContext;
use super::table::CodegenTableInfo;

/// Generate the terminal enum and its implementations.
pub fn generate(ctx: &CodegenContext, info: &CodegenTableInfo) -> TokenStream {
    let vis: TokenStream = "pub".parse().unwrap();
    let terminal_enum = format_ident!("Terminal");
    let types_trait = format_ident!("Types");
    let gazelle_crate_path = ctx.gazelle_crate_path_tokens();

    // Check if we have any typed terminals
    let has_typed_terminals = ctx.grammar.symbols.terminal_ids().skip(1).any(|id| {
        ctx.grammar
            .types
            .get(&id)
            .and_then(|t| t.as_ref())
            .is_some()
    });

    // Build enum variants
    let mut variants = Vec::new();

    for id in ctx.grammar.symbols.terminal_ids().skip(1) {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", crate::lr::to_camel_case(name));
        let ty = ctx.grammar.types.get(&id).and_then(|t| t.as_ref());
        let is_prec = ctx.grammar.symbols.is_prec_terminal(id);

        match (is_prec, ty) {
            (false, Some(type_name)) => {
                let assoc_type = format_ident!("{}", type_name);
                variants.push(quote! { #variant_name(A::#assoc_type) });
            }
            (false, None) => {
                variants.push(quote! { #variant_name });
            }
            (true, Some(type_name)) => {
                let assoc_type = format_ident!("{}", type_name);
                variants.push(
                    quote! { #variant_name(A::#assoc_type, #gazelle_crate_path::Precedence) },
                );
            }
            (true, None) => {
                variants.push(quote! { #variant_name(#gazelle_crate_path::Precedence) });
            }
        }
    }

    // Always add phantom data to use the A parameter
    variants.push(quote! {
        #[doc(hidden)]
        __Phantom(std::marker::PhantomData<A>)
    });

    // Build symbol_id match arms
    let symbol_id_arms = build_symbol_id_arms(ctx, info, &gazelle_crate_path, has_typed_terminals);

    // Build to_token match arms
    let to_token_arms = build_to_token_arms(ctx, &gazelle_crate_path, has_typed_terminals);

    // Build precedence match arms
    let precedence_arms = build_precedence_arms(ctx, has_typed_terminals);

    quote! {
        /// Terminal symbols for the parser.
        #vis enum #terminal_enum<A: #types_trait> {
            #(#variants),*
        }

        impl<A: #types_trait> #terminal_enum<A> {
            /// Get the symbol ID for this terminal.
            pub fn symbol_id(&self) -> #gazelle_crate_path::SymbolId {
                match self {
                    #(#symbol_id_arms)*
                }
            }

            /// Convert to a gazelle Token for parsing.
            pub fn to_token(&self, symbol_ids: &impl Fn(&str) -> #gazelle_crate_path::SymbolId) -> #gazelle_crate_path::Token {
                match self {
                    #(#to_token_arms)*
                }
            }

            /// Get precedence for runtime precedence comparison.
            pub fn precedence(&self) -> Option<#gazelle_crate_path::Precedence> {
                match self {
                    #(#precedence_arms)*
                }
            }
        }
    }
}

fn build_symbol_id_arms(
    ctx: &CodegenContext,
    info: &CodegenTableInfo,
    gazelle_crate_path: &TokenStream,
    _has_typed_terminals: bool,
) -> Vec<TokenStream> {
    let mut arms = Vec::new();

    for id in ctx.grammar.symbols.terminal_ids().skip(1) {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", crate::lr::to_camel_case(name));
        let ty = ctx.grammar.types.get(&id).and_then(|t| t.as_ref());
        let is_prec = ctx.grammar.symbols.is_prec_terminal(id);
        let table_id = info
            .terminal_ids
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, id)| *id)
            .unwrap_or(0);

        match (is_prec, ty.is_some()) {
            (false, true) => arms.push(quote! { Self::#variant_name(_) => #gazelle_crate_path::SymbolId::new(#table_id), }),
            (false, false) => arms.push(quote! { Self::#variant_name => #gazelle_crate_path::SymbolId::new(#table_id), }),
            (true, true) => arms.push(quote! { Self::#variant_name(_, _) => #gazelle_crate_path::SymbolId::new(#table_id), }),
            (true, false) => arms.push(quote! { Self::#variant_name(_) => #gazelle_crate_path::SymbolId::new(#table_id), }),
        }
    }

    // Always add __Phantom arm since we always include the variant
    arms.push(quote! { Self::__Phantom(_) => unreachable!(), });

    arms
}

fn build_to_token_arms(
    ctx: &CodegenContext,
    gazelle_crate_path: &TokenStream,
    _has_typed_terminals: bool,
) -> Vec<TokenStream> {
    let mut arms = Vec::new();

    for id in ctx.grammar.symbols.terminal_ids().skip(1) {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", crate::lr::to_camel_case(name));
        let ty = ctx.grammar.types.get(&id).and_then(|t| t.as_ref());
        let is_prec = ctx.grammar.symbols.is_prec_terminal(id);

        match (is_prec, ty.is_some()) {
            (false, true) => arms.push(quote! {
                Self::#variant_name(_) => #gazelle_crate_path::Token::new(symbol_ids(#name)),
            }),
            (false, false) => arms.push(quote! {
                Self::#variant_name => #gazelle_crate_path::Token::new(symbol_ids(#name)),
            }),
            (true, true) => arms.push(quote! {
                Self::#variant_name(_, prec) => #gazelle_crate_path::Token {
                    terminal: symbol_ids(#name),
                    prec: Some(*prec),
                },
            }),
            (true, false) => arms.push(quote! {
                Self::#variant_name(prec) => #gazelle_crate_path::Token {
                    terminal: symbol_ids(#name),
                    prec: Some(*prec),
                },
            }),
        }
    }

    // Always add __Phantom arm
    arms.push(quote! { Self::__Phantom(_) => unreachable!(), });

    arms
}

fn build_precedence_arms(ctx: &CodegenContext, _has_typed_terminals: bool) -> Vec<TokenStream> {
    let mut arms = Vec::new();

    for id in ctx.grammar.symbols.terminal_ids().skip(1) {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", crate::lr::to_camel_case(name));
        let ty = ctx.grammar.types.get(&id).and_then(|t| t.as_ref());
        let is_prec = ctx.grammar.symbols.is_prec_terminal(id);

        match (is_prec, ty.is_some()) {
            (false, true) => arms.push(quote! { Self::#variant_name(_) => None, }),
            (false, false) => arms.push(quote! { Self::#variant_name => None, }),
            (true, true) => arms.push(quote! { Self::#variant_name(_, prec) => Some(*prec), }),
            (true, false) => arms.push(quote! { Self::#variant_name(prec) => Some(*prec), }),
        }
    }

    // Always add __Phantom arm
    arms.push(quote! { Self::__Phantom(_) => unreachable!(), });

    arms
}

//! Lexer code generation: next_token function and optional Lexed/RawToken enums.
//!
//! For patterned terminals that are unit (no type, no prec), `next_token`
//! returns `Terminal<A>` directly. For typed/prec terminals, the user matches
//! on `RawToken` to attach values and precedence.
//!
//! If ALL patterned terminals are unit: `next_token` returns `Terminal<A>`.
//! If some need user logic: `next_token` returns `Lexed<A>` which is either
//! `Token(Terminal<A>)` for unit terminals or `Raw(RawToken)` for the rest.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::CodegenContext;

/// Generate lexer code for terminals with regex patterns.
/// Returns `None` if no terminals have patterns.
pub fn generate(ctx: &CodegenContext) -> Option<Result<TokenStream, String>> {
    if ctx.terminal_patterns.is_empty() {
        return None;
    }

    Some(generate_inner(ctx))
}

fn generate_inner(ctx: &CodegenContext) -> Result<TokenStream, String> {
    let vis: TokenStream = "pub".parse().unwrap();
    let gazelle_crate = ctx.gazelle_crate_path_tokens();
    let types_trait = format_ident!("Types");
    let terminal_enum = format_ident!("Terminal");

    // Classify patterned terminals
    let mut all_unit = true;
    let mut pattern_literals = Vec::new();
    let mut tid_literals = Vec::new();

    for (i, tp) in ctx.terminal_patterns.iter().enumerate() {
        pattern_literals.push(tp.pattern.clone());
        tid_literals.push(i as u16);
        if tp.has_type || tp.is_prec {
            all_unit = false;
        }
    }

    let dfa_init = quote! {
        use std::sync::LazyLock;

        static DFA: LazyLock<#gazelle_crate::lexer::LexerDfa> = LazyLock::new(|| {
            #gazelle_crate::regex::build_lexer_dfa(&[
                #( (#tid_literals, #pattern_literals), )*
            ]).expect("invalid regex pattern in terminal definition")
        });
    };

    if all_unit {
        generate_all_unit(
            ctx,
            &vis,
            &gazelle_crate,
            &types_trait,
            &terminal_enum,
            &dfa_init,
        )
    } else {
        generate_mixed(
            ctx,
            &vis,
            &gazelle_crate,
            &types_trait,
            &terminal_enum,
            &dfa_init,
        )
    }
}

/// All patterned terminals are unit: `next_token` returns `Terminal<A>` directly.
fn generate_all_unit(
    ctx: &CodegenContext,
    vis: &TokenStream,
    gazelle_crate: &TokenStream,
    types_trait: &proc_macro2::Ident,
    terminal_enum: &proc_macro2::Ident,
    dfa_init: &TokenStream,
) -> Result<TokenStream, String> {
    let mut match_arms = Vec::new();
    for (i, tp) in ctx.terminal_patterns.iter().enumerate() {
        let tid = i as u16;
        let variant = format_ident!("{}", crate::lr::to_camel_case(&tp.name));
        match_arms.push(quote! { #tid => Some((#terminal_enum::#variant, span)), });
    }

    Ok(quote! {
        /// Read the next token from the scanner using the auto-generated lexer DFA.
        ///
        /// Returns a fully constructed `Terminal` and the byte span of the match.
        /// Returns `None` if no patterned terminal matches at the current position;
        /// the scanner is unchanged on `None`.
        #vis fn next_token<A: #types_trait, I: Iterator<Item = char>>(
            scanner: &mut #gazelle_crate::lexer::Scanner<I>,
        ) -> Option<(#terminal_enum<A>, std::ops::Range<usize>)> {
            #dfa_init

            let (tid, span) = DFA.read_token(scanner)?;
            match tid {
                #(#match_arms)*
                _ => None,
            }
        }
    })
}

/// Some patterned terminals are typed/prec: generate `RawToken` + `Lexed<A>`.
fn generate_mixed(
    ctx: &CodegenContext,
    vis: &TokenStream,
    gazelle_crate: &TokenStream,
    types_trait: &proc_macro2::Ident,
    terminal_enum: &proc_macro2::Ident,
    dfa_init: &TokenStream,
) -> Result<TokenStream, String> {
    // RawToken gets only typed/prec patterned terminals
    let mut raw_variants = Vec::new();
    let mut match_arms = Vec::new();

    for (i, tp) in ctx.terminal_patterns.iter().enumerate() {
        let tid = i as u16;
        let variant = format_ident!("{}", crate::lr::to_camel_case(&tp.name));

        if tp.has_type || tp.is_prec {
            raw_variants.push(variant.clone());
            match_arms.push(quote! {
                #tid => Some((Lexed::Raw(RawToken::#variant), span)),
            });
        } else {
            match_arms.push(quote! {
                #tid => Some((Lexed::Token(#terminal_enum::#variant), span)),
            });
        }
    }

    Ok(quote! {
        /// Raw token types that need user logic to construct a `Terminal`.
        ///
        /// These are patterned terminals that carry a typed payload or precedence.
        /// Use `&input[span]` to extract the matched text and construct the
        /// appropriate `Terminal` variant.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #vis enum RawToken {
            #(#raw_variants),*
        }

        /// Result of the auto-generated lexer.
        ///
        /// `Token` contains a fully constructed `Terminal` (for unit terminals).
        /// `Raw` contains a `RawToken` that needs user logic to attach values
        /// or precedence.
        #vis enum Lexed<A: #types_trait> {
            /// A complete terminal, ready to push into the parser.
            Token(#terminal_enum<A>),
            /// A raw token that needs user logic (typed or precedence terminal).
            Raw(RawToken),
        }

        /// Read the next token from the scanner using the auto-generated lexer DFA.
        ///
        /// Returns `None` if no patterned terminal matches at the current position;
        /// the scanner is unchanged on `None`.
        #vis fn next_token<A: #types_trait, I: Iterator<Item = char>>(
            scanner: &mut #gazelle_crate::lexer::Scanner<I>,
        ) -> Option<(Lexed<A>, std::ops::Range<usize>)> {
            #dfa_init

            let (tid, span) = DFA.read_token(scanner)?;
            match tid {
                #(#match_arms)*
                _ => None,
            }
        }
    })
}

//! Parser struct and trait code generation.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::lr::AltAction;
use super::reduction::{self, typed_symbol_indices, ReductionInfo, SymbolKind};
use super::table::CodegenTableInfo;
use super::CodegenContext;

/// Generate the parser wrapper, trait, and related types.
pub fn generate(ctx: &CodegenContext, info: &CodegenTableInfo) -> Result<TokenStream, String> {
    let vis: TokenStream = ctx.visibility.parse().unwrap_or_default();
    let name = &ctx.name;
    let terminal_enum = format_ident!("{}Terminal", name);
    let types_trait = format_ident!("{}Types", name);
    let actions_trait = format_ident!("{}Actions", name);
    let parser_struct = format_ident!("{}Parser", name);
    let value_union = format_ident!("__{}Value", name);
    let table_mod = format_ident!("__{}_table", name.to_lowercase());
    let core_path = ctx.core_path_tokens();

    // Analyze reductions
    let reductions = reduction::analyze_reductions(ctx)?;

    // Collect non-terminals with types (excluding synthetic rules)
    let typed_non_terminals: Vec<_> = ctx.grammar.symbols.non_terminal_ids()
        .filter_map(|id| {
            let ty = ctx.grammar.types.get(&id)?.as_ref()?;
            let name = ctx.grammar.symbols.name(id);
            if name.starts_with("__") { return None; }
            Some((name.to_string(), ty.clone()))
        })
        .collect();

    // All typed non-terminals including synthetic (for value union)
    let all_typed_non_terminals: Vec<_> = ctx.grammar.symbols.non_terminal_ids()
        .filter_map(|id| {
            let ty = ctx.grammar.types.get(&id)?.as_ref()?;
            Some((ctx.grammar.symbols.name(id).to_string(), ty.clone()))
        })
        .collect();

    // Get start non-terminal name and its type annotation
    let start_nt = &ctx.start_symbol;
    let start_type_annotation = typed_non_terminals.iter()
        .find(|(name, _)| name == start_nt)
        .map(|(_, ty)| ty.clone());
    let start_field = format_ident!("__{}", start_nt.to_lowercase());

    // Generate components
    let enum_code = generate_nonterminal_enums(ctx, &reductions, &typed_non_terminals, &types_trait, &vis);
    let traits_code = generate_traits(ctx, &types_trait, &actions_trait, &typed_non_terminals, &reductions, &vis, &core_path);
    let value_union_code = generate_value_union(ctx, &all_typed_non_terminals, &value_union, &types_trait);
    let shift_arms = generate_terminal_shift_arms(ctx, &terminal_enum, &value_union);
    let reduction_arms = generate_reduction_arms(ctx, &reductions, &value_union, &typed_non_terminals);
    let drop_arms = generate_drop_arms(ctx, info);

    // Generate finish method based on whether start symbol has a type
    let finish_method = if let Some(start_type) = start_type_annotation {
        let start_type_ident = format_ident!("{}", start_type);
        quote! {
            pub fn finish(mut self, actions: &mut A) -> Result<A::#start_type_ident, (Self, A::Error)> {
                loop {
                    match self.parser.maybe_reduce(None) {
                        Ok(Some((0, _, _))) => {
                            let union_val = self.value_stack.pop().unwrap();
                            return Ok(unsafe { std::mem::ManuallyDrop::into_inner(union_val.#start_field) });
                        }
                        Ok(Some((rule, _, start_idx))) => {
                            if let Err(e) = self.do_reduce(rule, start_idx, actions) {
                                return Err((self, e));
                            }
                        }
                        Ok(None) => unreachable!(),
                        Err(e) => return Err((self, e.into())),
                    }
                }
            }
        }
    } else {
        quote! {
            pub fn finish(mut self, actions: &mut A) -> Result<(), (Self, A::Error)> {
                loop {
                    match self.parser.maybe_reduce(None) {
                        Ok(Some((0, _, _))) => {
                            self.value_stack.pop();
                            return Ok(());
                        }
                        Ok(Some((rule, _, start_idx))) => {
                            if let Err(e) = self.do_reduce(rule, start_idx, actions) {
                                return Err((self, e));
                            }
                        }
                        Ok(None) => unreachable!(),
                        Err(e) => return Err((self, e.into())),
                    }
                }
            }
        }
    };

    Ok(quote! {
        #enum_code

        #traits_code

        #value_union_code

        /// Type-safe LR parser.
        #vis struct #parser_struct<A: #types_trait> {
            parser: #core_path::Parser<'static>,
            value_stack: Vec<#value_union<A>>,
        }

        impl<A: #types_trait> #parser_struct<A> {
            /// Create a new parser instance.
            pub fn new() -> Self {
                Self {
                    parser: #core_path::Parser::new(#table_mod::TABLE),
                    value_stack: Vec::new(),
                }
            }

            /// Get the current parser state.
            pub fn state(&self) -> usize {
                self.parser.state()
            }

            /// Format a parse error message.
            pub fn format_error(&self, err: &#core_path::ParseError) -> String {
                self.parser.format_error(err, &#table_mod::ERROR_INFO)
            }

            /// Format a parse error with display names and token texts.
            pub fn format_error_with(
                &self,
                err: &#core_path::ParseError,
                display_names: &std::collections::HashMap<&str, &str>,
                tokens: &[&str],
            ) -> String {
                self.parser.format_error_with(err, &#table_mod::ERROR_INFO, display_names, tokens)
            }

            /// Get the error info for custom error formatting.
            pub fn error_info() -> &'static #core_path::ErrorInfo<'static> {
                &#table_mod::ERROR_INFO
            }
        }

        #[allow(clippy::result_large_err)]
        impl<A: #actions_trait> #parser_struct<A> {
            /// Push a terminal, performing any reductions.
            pub fn push(&mut self, terminal: #terminal_enum<A>, actions: &mut A) -> Result<(), A::Error> {
                let token = #core_path::Token {
                    terminal: terminal.symbol_id(),
                    prec: terminal.precedence(),
                };

                // Reduce while possible
                while let Some((rule, _, start_idx)) = self.parser.maybe_reduce(Some(token))? {
                    self.do_reduce(rule, start_idx, actions)?;
                }

                // Shift the terminal
                self.parser.shift(token);

                match terminal {
                    #(#shift_arms)*
                }

                Ok(())
            }

            /// Finish parsing and return the result.
            #finish_method

            fn do_reduce(&mut self, rule: usize, start_idx: usize, actions: &mut A) -> Result<(), A::Error> {
                if rule == 0 { return Ok(()); }

                let original_rule_idx = rule - 1;

                let value = match original_rule_idx {
                    #(#reduction_arms)*
                    _ => return Ok(()),
                };

                self.value_stack.push(value);
                Ok(())
            }
        }

        impl<A: #types_trait> Default for #parser_struct<A> {
            fn default() -> Self { Self::new() }
        }

        impl<A: #types_trait> Drop for #parser_struct<A> {
            fn drop(&mut self) {
                for i in (0..self.value_stack.len()).rev() {
                    let union_val = self.value_stack.pop().unwrap();
                    let sym_id = #table_mod::STATE_SYMBOL[self.parser.state_at(i)];
                    unsafe {
                        match sym_id {
                            #(#drop_arms)*
                            _ => {}
                        }
                    }
                }
            }
        }
    })
}

/// Generate per-nonterminal enums for typed non-terminals.
fn generate_nonterminal_enums(
    ctx: &CodegenContext,
    reductions: &[ReductionInfo],
    typed_non_terminals: &[(String, String)],
    types_trait: &syn::Ident,
    vis: &TokenStream,
) -> TokenStream {
    let mut enums = Vec::new();

    // Map from terminal name to associated type name
    let terminal_assoc_types: std::collections::BTreeMap<&str, &str> = ctx.grammar.symbols.terminal_ids().skip(1)
        .filter_map(|id| {
            let type_name = ctx.grammar.types.get(&id)?.as_ref()?;
            Some((ctx.grammar.symbols.name(id), type_name.as_str()))
        })
        .collect();

    // Map from non-terminal name to result type
    let nt_result_types: std::collections::HashMap<&str, &str> = typed_non_terminals.iter()
        .map(|(name, result_type)| (name.as_str(), result_type.as_str()))
        .collect();

    // Group reductions by non-terminal, only for typed non-synthetic NTs with variants
    let mut nt_variants: std::collections::BTreeMap<&str, Vec<&ReductionInfo>> = std::collections::BTreeMap::new();
    for info in reductions {
        if info.variant_name.is_some() {
            nt_variants.entry(&info.non_terminal).or_default().push(info);
        }
    }

    for (nt_name, variants) in &nt_variants {
        let enum_ident = enum_name(&ctx.name, nt_name);

        let variant_defs: Vec<_> = variants.iter().map(|info| {
            let variant_name = format_ident!("{}", capitalize(info.variant_name.as_ref().unwrap()));
            let fields: Vec<_> = typed_symbol_indices(&info.rhs_symbols).iter()
                .map(|&idx| {
                    let sym = &info.rhs_symbols[idx];
                    symbol_to_field_type(sym, &nt_result_types, &terminal_assoc_types, ctx)
                })
                .collect();

            if fields.is_empty() {
                quote! { #variant_name }
            } else {
                quote! { #variant_name(#(#fields),*) }
            }
        }).collect();

        // Check if any variant field type actually references the type parameter A.
        // typed_symbol_indices alone is insufficient: synthetic types like Vec<()>
        // have a type but don't reference A.
        let uses_a = variants.iter().any(|info| {
            typed_symbol_indices(&info.rhs_symbols).iter().any(|&idx| {
                let sym = &info.rhs_symbols[idx];
                symbol_references_a(sym, &nt_result_types, &terminal_assoc_types, ctx)
            })
        });
        let phantom = if uses_a {
            quote! {}
        } else {
            quote! { , #[doc(hidden)] __Phantom(std::marker::PhantomData<fn() -> A>) }
        };

        let core_path = ctx.core_path_tokens();
        enums.push(quote! {
            #[allow(non_camel_case_types)]
            #vis enum #enum_ident<A: #types_trait> {
                #(#variant_defs),*
                #phantom
            }

            impl<A: #types_trait> #core_path::ReduceNode for #enum_ident<A> {}
        });
    }

    quote! { #(#enums)* }
}

/// Convert a symbol to its field type tokens for use in an enum variant.
fn symbol_to_field_type(
    sym: &reduction::SymbolInfo,
    nt_result_types: &std::collections::HashMap<&str, &str>,
    terminal_assoc_types: &std::collections::BTreeMap<&str, &str>,
    ctx: &CodegenContext,
) -> TokenStream {
    if sym.kind == SymbolKind::NonTerminal {
        if let Some(&result_type) = nt_result_types.get(sym.name.as_str()) {
            let assoc = format_ident!("{}", result_type);
            quote! { A::#assoc }
        } else if sym.name.starts_with("__") {
            if let Some(result_type) = ctx.get_type(&sym.name) {
                synthetic_type_to_tokens_with_prefix(result_type, false)
            } else {
                quote! { () }
            }
        } else {
            quote! { () }
        }
    } else if let Some(assoc_name) = terminal_assoc_types.get(sym.name.as_str()) {
        let assoc = format_ident!("{}", assoc_name);
        quote! { A::#assoc }
    } else {
        quote! { () }
    }
}

/// Check if a symbol's field type would reference the generic parameter A.
fn symbol_references_a(
    sym: &reduction::SymbolInfo,
    nt_result_types: &std::collections::HashMap<&str, &str>,
    terminal_assoc_types: &std::collections::BTreeMap<&str, &str>,
    ctx: &CodegenContext,
) -> bool {
    if sym.kind == SymbolKind::NonTerminal {
        if nt_result_types.contains_key(sym.name.as_str()) {
            // Non-synthetic typed NT -> A::ResultType
            true
        } else if sym.name.starts_with("__") {
            if let Some(result_type) = ctx.get_type(&sym.name) {
                // Synthetic types like Vec<Foo> reference A if Foo is not "()"
                let inner = result_type
                    .strip_prefix("Option<").and_then(|s| s.strip_suffix('>'))
                    .or_else(|| result_type.strip_prefix("Vec<").and_then(|s| s.strip_suffix('>')));
                match inner {
                    Some("()") => false,
                    Some(_) => true,
                    None => false,
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        terminal_assoc_types.contains_key(sym.name.as_str())
    }
}

fn generate_traits(
    ctx: &CodegenContext,
    types_trait: &syn::Ident,
    actions_trait: &syn::Ident,
    typed_non_terminals: &[(String, String)],
    reductions: &[ReductionInfo],
    vis: &TokenStream,
    core_path: &TokenStream,
) -> TokenStream {
    let mut assoc_types = Vec::new();
    let mut seen_types = std::collections::HashSet::new();

    // Terminal associated types - use payload type name directly
    for id in ctx.grammar.symbols.terminal_ids().skip(1) {
        if let Some(type_name) = ctx.grammar.types.get(&id).and_then(|t| t.as_ref()) {
            if seen_types.insert(type_name.as_str()) {
                let type_ident = format_ident!("{}", type_name);
                assoc_types.push(quote! { type #type_ident; });
            }
        }
    }

    // Non-terminal associated types (deduplicated by result_type)
    for (_, result_type) in typed_non_terminals {
        if seen_types.insert(result_type.as_str()) {
            let type_name = format_ident!("{}", result_type);
            assoc_types.push(quote! { type #type_name; });
        }
    }

    // Collect Reduce bounds for non-terminals that have enum variants
    let mut reduce_bounds = Vec::new();
    let mut reduce_bounds_for_blanket = Vec::new();
    let mut seen_nt = std::collections::HashSet::new();
    for info in reductions {
        if info.variant_name.is_some() && seen_nt.insert(&info.non_terminal) {
            let enum_ident = enum_name(&ctx.name, &info.non_terminal);
            if let Some((_, result_type)) = typed_non_terminals.iter().find(|(n, _)| n == &info.non_terminal) {
                let result_ident = format_ident!("{}", result_type);
                reduce_bounds.push(quote! { + #core_path::Reduce<#enum_ident<Self>, Self::#result_ident, Self::Error> });
                reduce_bounds_for_blanket.push(quote! { + #core_path::Reduce<#enum_ident<T>, T::#result_ident, T::Error> });
            } else {
                // Untyped NT with => name — side-effect enum, output is ()
                reduce_bounds.push(quote! { + #core_path::Reduce<#enum_ident<Self>, (), Self::Error> });
                reduce_bounds_for_blanket.push(quote! { + #core_path::Reduce<#enum_ident<T>, (), T::Error> });
            }
        }
    }

    quote! {
        /// Associated types for parser symbols.
        #vis trait #types_trait: Sized {
            type Error: From<#core_path::ParseError>;
            #(#assoc_types)*
        }

        /// Actions trait — automatically implemented for any type satisfying
        /// the Types and Reduce bounds.
        #vis trait #actions_trait: #types_trait #(#reduce_bounds)* {}

        impl<T: #types_trait #(#reduce_bounds_for_blanket)*> #actions_trait for T {}
    }
}

fn generate_value_union(
    ctx: &CodegenContext,
    typed_non_terminals: &[(String, String)],
    value_union: &syn::Ident,
    types_trait: &syn::Ident,
) -> TokenStream {
    let mut fields = Vec::new();

    // Terminals with payloads - use payload type name as associated type
    for id in ctx.grammar.symbols.terminal_ids().skip(1) {
        if let Some(type_name) = ctx.grammar.types.get(&id).and_then(|t| t.as_ref()) {
            let name = ctx.grammar.symbols.name(id);
            let field_name = format_ident!("__{}", name.to_lowercase());
            let assoc_type = format_ident!("{}", type_name);
            fields.push(quote! { #field_name: std::mem::ManuallyDrop<A::#assoc_type>, });
        }
    }

    // Typed non-terminals
    for (name, result_type) in typed_non_terminals {
        let field_name = format_ident!("__{}", name.to_lowercase());

        // Check if this is a synthetic rule
        if name.starts_with("__") {
            let field_type = synthetic_type_to_tokens_with_prefix(result_type, false);
            fields.push(quote! { #field_name: std::mem::ManuallyDrop<#field_type>, });
        } else {
            let assoc_type = format_ident!("{}", result_type);
            fields.push(quote! { #field_name: std::mem::ManuallyDrop<A::#assoc_type>, });
        }
    }

    quote! {
        #[doc(hidden)]
        union #value_union<A: #types_trait> {
            #(#fields)*
            __unit: (),
            __phantom: std::mem::ManuallyDrop<std::marker::PhantomData<A>>,
        }
    }
}

/// Convert a synthetic type like "Option<Foo>" or "Vec<Bar>" to tokens with associated type.
fn synthetic_type_to_tokens_with_prefix(type_str: &str, use_self: bool) -> TokenStream {
    if let Some(inner) = type_str.strip_prefix("Option<").and_then(|s| s.strip_suffix('>')) {
        if inner == "()" {
            quote! { Option<()> }
        } else {
            let inner_ident = format_ident!("{}", inner);
            if use_self {
                quote! { Option<Self::#inner_ident> }
            } else {
                quote! { Option<A::#inner_ident> }
            }
        }
    } else if let Some(inner) = type_str.strip_prefix("Vec<").and_then(|s| s.strip_suffix('>')) {
        if inner == "()" {
            quote! { Vec<()> }
        } else {
            let inner_ident = format_ident!("{}", inner);
            if use_self {
                quote! { Vec<Self::#inner_ident> }
            } else {
                quote! { Vec<A::#inner_ident> }
            }
        }
    } else {
        let ident = format_ident!("{}", type_str);
        if use_self {
            quote! { Self::#ident }
        } else {
            quote! { A::#ident }
        }
    }
}

fn generate_terminal_shift_arms(ctx: &CodegenContext, terminal_enum: &syn::Ident, value_union: &syn::Ident) -> Vec<TokenStream> {
    let mut arms = Vec::new();

    for id in ctx.grammar.symbols.terminal_ids().skip(1) {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", name);
        let ty = ctx.grammar.types.get(&id).and_then(|t| t.as_ref());
        let is_prec = ctx.grammar.symbols.is_prec_terminal(id);

        match (is_prec, ty.is_some()) {
            (false, true) => {
                let field_name = format_ident!("__{}", name.to_lowercase());
                arms.push(quote! {
                    #terminal_enum::#variant_name(v) => {
                        self.value_stack.push(
                            #value_union { #field_name: std::mem::ManuallyDrop::new(v) }
                        );
                    }
                });
            }
            (false, false) => {
                arms.push(quote! {
                    #terminal_enum::#variant_name => {
                        self.value_stack.push(#value_union { __unit: () });
                    }
                });
            }
            (true, true) => {
                let field_name = format_ident!("__{}", name.to_lowercase());
                arms.push(quote! {
                    #terminal_enum::#variant_name(v, _prec) => {
                        self.value_stack.push(
                            #value_union { #field_name: std::mem::ManuallyDrop::new(v) }
                        );
                    }
                });
            }
            (true, false) => {
                arms.push(quote! {
                    #terminal_enum::#variant_name(_prec) => {
                        self.value_stack.push(#value_union { __unit: () });
                    }
                });
            }
        }
    }

    // Always handle phantom variant
    arms.push(quote! {
        #terminal_enum::__Phantom(_) => unreachable!(),
    });

    arms
}

fn generate_reduction_arms(
    ctx: &CodegenContext,
    reductions: &[ReductionInfo],
    value_union: &syn::Ident,
    _typed_non_terminals: &[(String, String)],
) -> Vec<TokenStream> {
    let core_path = ctx.core_path_tokens();

    let mut arms = Vec::new();

    for (idx, info) in reductions.iter().enumerate() {
        let lhs_field = format_ident!("__{}", info.non_terminal.to_lowercase());
        let idx_lit = idx;

        // Build the pop and extract statements
        let mut stmts = Vec::new();

        for (i, sym) in info.rhs_symbols.iter().enumerate().rev() {
            let pop_expr = quote! { self.value_stack.pop().unwrap() };

            if sym.ty.is_some() {
                let field_name = match sym.kind {
                    SymbolKind::UnitTerminal => {
                        stmts.push(quote! { let _ = #pop_expr; });
                        continue;
                    }
                    SymbolKind::PayloadTerminal | SymbolKind::PrecTerminal => {
                        format_ident!("__{}", sym.name.to_lowercase())
                    }
                    SymbolKind::NonTerminal => {
                        format_ident!("__{}", sym.name.to_lowercase())
                    }
                };

                let var_name = format_ident!("v{}", i);
                let extract = quote! { std::mem::ManuallyDrop::into_inner(#pop_expr.#field_name) };

                stmts.push(quote! { let #var_name = unsafe { #extract }; });
            } else {
                stmts.push(quote! { let _ = #pop_expr; });
            }
        }

        // Check if NT has a result type
        let has_result_type = ctx.grammar.symbols.non_terminal_ids()
            .find(|&id| ctx.grammar.symbols.name(id) == info.non_terminal)
            .and_then(|id| ctx.grammar.types.get(&id)?.as_ref())
            .is_some();

        // Generate result based on reduction kind
        let result = if let Some(variant_name) = &info.variant_name {
            // Non-terminal with enum variant: construct variant, call reduce
            let enum_name = enum_name(&ctx.name, &info.non_terminal);
            let variant_ident = format_ident!("{}", capitalize(variant_name));
            let args: Vec<_> = typed_symbol_indices(&info.rhs_symbols).iter()
                .map(|sym_idx| format_ident!("v{}", sym_idx))
                .collect();

            let node_expr = if args.is_empty() {
                quote! { #enum_name::#variant_ident }
            } else {
                quote! { #enum_name::#variant_ident(#(#args),*) }
            };

            if has_result_type {
                quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(
                    #core_path::Reduce::reduce(actions, #node_expr)?
                ) } }
            } else {
                // Untyped NT with => name — side-effect reduction
                quote! { {
                    #core_path::Reduce::reduce(actions, #node_expr)?;
                    #value_union { __unit: () }
                } }
            }
        } else {
            match &info.action {
                AltAction::Named(_) | AltAction::None => {
                    quote! { #value_union { __unit: () } }
                }
                AltAction::OptSome => {
                    let is_unit = info.rhs_symbols.first().map(|s| s.ty.is_none()).unwrap_or(true);
                    if is_unit {
                        quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(Some(())) } }
                    } else {
                        quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(Some(v0)) } }
                    }
                }
                AltAction::OptNone => {
                    quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(None) } }
                }
                AltAction::VecEmpty => {
                    quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(Vec::new()) } }
                }
                AltAction::VecSingle => {
                    let is_unit = info.rhs_symbols.first().map(|s| s.ty.is_none()).unwrap_or(true);
                    if is_unit {
                        quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(vec![()]) } }
                    } else {
                        quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(vec![v0]) } }
                    }
                }
                AltAction::VecAppend => {
                    let last_idx = info.rhs_symbols.len() - 1;
                    let is_unit = info.rhs_symbols.get(last_idx).map(|s| s.ty.is_none()).unwrap_or(true);
                    if is_unit {
                        quote! { { let mut v0 = v0; v0.push(()); #value_union { #lhs_field: std::mem::ManuallyDrop::new(v0) } } }
                    } else {
                        let elem_var = format_ident!("v{}", last_idx);
                        quote! { { let mut v0 = v0; v0.push(#elem_var); #value_union { #lhs_field: std::mem::ManuallyDrop::new(v0) } } }
                    }
                }
            }
        };

        arms.push(quote! {
            #idx_lit => {
                #(#stmts)*
                #result
            }
        });
    }

    arms
}

fn generate_drop_arms(ctx: &CodegenContext, info: &CodegenTableInfo) -> Vec<TokenStream> {
    let mut arms = Vec::new();

    // Terminals with payloads
    for id in ctx.grammar.symbols.terminal_ids().skip(1) {
        if ctx.grammar.types.get(&id).and_then(|t| t.as_ref()).is_some() {
            let name = ctx.grammar.symbols.name(id);
            if let Some((_, table_id)) = info.terminal_ids.iter().find(|(n, _)| n == name) {
                let field_name = format_ident!("__{}", name.to_lowercase());
                arms.push(quote! {
                    #table_id => { std::mem::ManuallyDrop::into_inner(union_val.#field_name); }
                });
            }
        }
    }

    // Non-terminals
    for id in ctx.grammar.symbols.non_terminal_ids() {
        if ctx.grammar.types.get(&id).and_then(|t| t.as_ref()).is_some() {
            let name = ctx.grammar.symbols.name(id);
            let field_name = format_ident!("__{}", name.to_lowercase());
            if let Some((_, table_id)) = info.non_terminal_ids.iter().find(|(n, _)| n == name) {
                arms.push(quote! {
                    #table_id => { std::mem::ManuallyDrop::into_inner(union_val.#field_name); }
                });
            }
        }
    }

    arms
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn enum_name(grammar_name: &str, nt_name: &str) -> syn::Ident {
    format_ident!("{}{}", grammar_name, capitalize(nt_name))
}

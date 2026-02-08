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
    let typed_non_terminals: Vec<_> = ctx.grammar.nt_types.iter()
        .filter(|(_, ty)| ty.is_some())
        .filter(|(id, _)| !ctx.grammar.symbols.name(**id).starts_with("__"))
        .map(|(id, ty)| (ctx.grammar.symbols.name(*id).to_string(), ty.clone().unwrap()))
        .collect();

    // All typed non-terminals including synthetic (for value union)
    let all_typed_non_terminals: Vec<_> = ctx.grammar.nt_types.iter()
        .filter(|(_, ty)| ty.is_some())
        .map(|(id, ty)| (ctx.grammar.symbols.name(*id).to_string(), ty.clone().unwrap()))
        .collect();

    // Collect trait methods
    let trait_methods = reduction::collect_trait_methods(&reductions);

    // Get start non-terminal name and its type annotation
    let start_nt = &ctx.start_symbol;
    let start_type_annotation = typed_non_terminals.iter()
        .find(|(name, _)| name == start_nt)
        .map(|(_, ty)| ty.clone());
    let start_field = format_ident!("__{}", start_nt.to_lowercase());

    // Generate components
    let parse_error = quote! { #core_path::ParseError };
    let traits_code = generate_traits(ctx, &types_trait, &actions_trait, &typed_non_terminals, &trait_methods, &vis, &parse_error);
    let value_union_code = generate_value_union(ctx, &all_typed_non_terminals, &value_union, &types_trait);
    let shift_arms = generate_terminal_shift_arms(ctx, &terminal_enum, &value_union);
    let reduction_arms = generate_reduction_arms(ctx, &reductions, &value_union);
    let drop_arms = generate_drop_arms(ctx, info);

    // Generate finish method based on whether start symbol has a type
    // Returns (Self, E) on error so caller can still format it
    let finish_method = if let Some(start_type) = start_type_annotation {
        let start_type_ident = format_ident!("{}", start_type);
        quote! {
            pub fn finish(mut self, actions: &mut A) -> Result<A::#start_type_ident, (Self, E)> {
                loop {
                    match self.parser.maybe_reduce(None) {
                        Ok(Some((0, _, _))) => {
                            let union_val = self.value_stack.pop().unwrap();
                            return Ok(unsafe { std::mem::ManuallyDrop::into_inner(union_val.#start_field) });
                        }
                        Ok(Some((rule, _, start_idx))) => if let Err(e) = self.do_reduce(rule, start_idx, actions) { return Err((self, e)); },
                        Ok(None) => unreachable!(),
                        Err(e) => return Err((self, e.into())),
                    }
                }
            }
        }
    } else {
        quote! {
            pub fn finish(mut self, actions: &mut A) -> Result<(), (Self, E)> {
                loop {
                    match self.parser.maybe_reduce(None) {
                        Ok(Some((0, _, _))) => {
                            self.value_stack.pop();
                            return Ok(());
                        }
                        Ok(Some((rule, _, start_idx))) => if let Err(e) = self.do_reduce(rule, start_idx, actions) { return Err((self, e)); },
                        Ok(None) => unreachable!(),
                        Err(e) => return Err((self, e.into())),
                    }
                }
            }
        }
    };

    Ok(quote! {
        #traits_code

        #value_union_code

        /// Type-safe LR parser.
        #vis struct #parser_struct<A: #actions_trait<E>, E: From<#parse_error> = #parse_error> {
            parser: #core_path::Parser<'static>,
            value_stack: Vec<#value_union<A>>,
            _phantom: std::marker::PhantomData<E>,
        }

        #[allow(clippy::result_large_err)]
        impl<A: #actions_trait<E>, E: From<#parse_error>> #parser_struct<A, E> {
            /// Create a new parser instance.
            pub fn new() -> Self {
                Self {
                    parser: #core_path::Parser::new(#table_mod::TABLE),
                    value_stack: Vec::new(),
                    _phantom: std::marker::PhantomData,
                }
            }

            /// Push a terminal, performing any reductions.
            pub fn push(&mut self, terminal: #terminal_enum<A>, actions: &mut A) -> Result<(), E> {
                let token = #core_path::Token {
                    terminal: terminal.symbol_id(),
                    prec: terminal.precedence(),
                };

                // Reduce while possible
                while let Some((rule, _, start_idx)) = self.parser.maybe_reduce(Some(&token)).map_err(E::from)? {
                    self.do_reduce(rule, start_idx, actions)?;
                }

                // Shift the terminal
                self.parser.shift(&token);

                match terminal {
                    #(#shift_arms)*
                }

                Ok(())
            }

            /// Finish parsing and return the result.
            #finish_method

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

            fn do_reduce(&mut self, rule: usize, start_idx: usize, actions: &mut A) -> Result<(), E> {
                if rule == 0 { return Ok(()); }

                // Notify actions of token range [start_idx, end_idx)
                actions.set_token_range(start_idx, self.parser.token_count());

                let original_rule_idx = rule - 1;

                let value = match original_rule_idx {
                    #(#reduction_arms)*
                    _ => return Ok(()),
                };

                self.value_stack.push(value);
                Ok(())
            }
        }

        impl<A: #actions_trait<E>, E: From<#parse_error>> Default for #parser_struct<A, E> {
            fn default() -> Self { Self::new() }
        }

        impl<A: #actions_trait<E>, E: From<#parse_error>> Drop for #parser_struct<A, E> {
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

fn generate_traits(
    ctx: &CodegenContext,
    types_trait: &syn::Ident,
    actions_trait: &syn::Ident,
    typed_non_terminals: &[(String, String)],
    methods: &[reduction::TraitMethod],
    vis: &TokenStream,
    parse_error: &TokenStream,
) -> TokenStream {
    let mut assoc_types = Vec::new();
    let mut seen_types = std::collections::HashSet::new();

    // Terminal associated types - use payload type name directly
    for type_name in ctx.grammar.terminal_types.values().flatten() {
        if seen_types.insert(type_name.as_str()) {
            let type_ident = format_ident!("{}", type_name);
            assoc_types.push(quote! { type #type_ident; });
        }
    }

    // Prec terminal associated types
    for type_name in ctx.grammar.prec_terminal_types.values().flatten() {
        if seen_types.insert(type_name.as_str()) {
            let type_ident = format_ident!("{}", type_name);
            assoc_types.push(quote! { type #type_ident; });
        }
    }

    // Non-terminal associated types (deduplicated by result_type)
    for (_, result_type) in typed_non_terminals {
        if seen_types.insert(result_type.as_str()) {
            let type_name = format_ident!("{}", result_type);
            assoc_types.push(quote! { type #type_name; });
        }
    }

    // Map from rule name to result_type for quick lookup
    let nt_result_types: std::collections::HashMap<&str, &str> = typed_non_terminals.iter()
        .map(|(name, result_type)| (name.as_str(), result_type.as_str()))
        .collect();

    // Map from terminal name to associated type name
    let terminal_assoc_types: std::collections::BTreeMap<&str, &str> = ctx.grammar.terminal_types.iter()
        .filter_map(|(id, ty)| {
            ty.as_ref().map(|type_name| {
                (ctx.grammar.symbols.name(*id), type_name.as_str())
            })
        })
        .chain(
            ctx.grammar.prec_terminal_types.iter()
                .filter_map(|(id, ty)| {
                    ty.as_ref().map(|type_name| {
                        (ctx.grammar.symbols.name(*id), type_name.as_str())
                    })
                })
        )
        .collect();

    let method_defs: Vec<_> = methods.iter()
        .map(|method| {
            let method_name = format_ident!("{}", method.name);

            // Check if this non-terminal has a result type
            let return_type_tokens = if let Some(&result_type) = nt_result_types.get(method.non_terminal.as_str()) {
                let return_type = format_ident!("{}", result_type);
                quote! { Self::#return_type }
            } else {
                quote! { () }
            };

            let params: Vec<_> = typed_symbol_indices(&method.rhs_symbols).iter().enumerate()
                .map(|(param_idx, &sym_idx)| {
                    let sym = &method.rhs_symbols[sym_idx];
                    let param_name = format_ident!("v{}", param_idx);

                    let param_type: TokenStream = if sym.kind == SymbolKind::NonTerminal {
                        if let Some(&result_type) = nt_result_types.get(sym.name.as_str()) {
                            // Normal non-terminal - use associated type from result_type
                            let assoc = format_ident!("{}", result_type);
                            quote! { Self::#assoc }
                        } else if sym.name.starts_with("__") {
                            // Synthetic non-terminal - look up its result type and convert
                            if let Some(result_type) = ctx.get_rule_result_type(&sym.name) {
                                synthetic_type_to_tokens_with_prefix(result_type, true) // true = use Self::
                            } else {
                                quote! { () }
                            }
                        } else {
                            quote! { () }
                        }
                    } else {
                        // Terminal - use associated type if typed
                        if let Some(assoc_name) = terminal_assoc_types.get(sym.name.as_str()) {
                            let assoc = format_ident!("{}", assoc_name);
                            quote! { Self::#assoc }
                        } else {
                            quote! { () }
                        }
                    };

                    quote! { #param_name: #param_type }
                })
                .collect();

            quote! {
                fn #method_name(&mut self, #(#params),*) -> Result<#return_type_tokens, E>;
            }
        })
        .collect();

    quote! {
        /// Associated types for parser symbols.
        #vis trait #types_trait {
            #(#assoc_types)*
        }

        /// Actions trait for parser callbacks.
        #vis trait #actions_trait<E: From<#parse_error> = #parse_error>: #types_trait {
            /// Called before each reduction with the token range [start, end).
            /// Override to track source spans. Default is no-op.
            #[allow(unused_variables)]
            fn set_token_range(&mut self, start: usize, end: usize) {}

            #(#method_defs)*
        }
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
    for (&id, payload_type) in &ctx.grammar.terminal_types {
        if let Some(type_name) = payload_type {
            let name = ctx.grammar.symbols.name(id);
            let field_name = format_ident!("__{}", name.to_lowercase());
            let assoc_type = format_ident!("{}", type_name);
            fields.push(quote! { #field_name: std::mem::ManuallyDrop<A::#assoc_type>, });
        }
    }

    // Prec terminals with payloads - use payload type name as associated type
    for (&id, payload_type) in &ctx.grammar.prec_terminal_types {
        if let Some(type_name) = payload_type {
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
            // Synthetic rule - use concrete wrapper type with associated inner type
            // result_type is like "Option<Foo>" or "Vec<Foo>"
            let field_type = synthetic_type_to_tokens_with_prefix(result_type, false); // false = use A::
            fields.push(quote! { #field_name: std::mem::ManuallyDrop<#field_type>, });
        } else {
            // Normal non-terminal - use associated type from result_type
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
/// Uses `self_prefix` to generate either `Self::Foo` (for trait defs) or `A::Foo` (for impls).
fn synthetic_type_to_tokens_with_prefix(type_str: &str, use_self: bool) -> TokenStream {
    if let Some(inner) = type_str.strip_prefix("Option<").and_then(|s| s.strip_suffix('>')) {
        // Handle unit type specially - no prefix needed
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
        // Handle unit type specially - no prefix needed
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
        // Fallback - just use it as-is (shouldn't happen for valid synthetic rules)
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

    // Regular terminals
    for (&id, payload_type) in &ctx.grammar.terminal_types {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", name);
        let field_name = format_ident!("__{}", name.to_lowercase());

        if payload_type.is_some() {
            arms.push(quote! {
                #terminal_enum::#variant_name(v) => {
                    self.value_stack.push(
                        #value_union { #field_name: std::mem::ManuallyDrop::new(v) }
                    );
                }
            });
        } else {
            arms.push(quote! {
                #terminal_enum::#variant_name => {
                    self.value_stack.push(#value_union { __unit: () });
                }
            });
        }
    }

    // Prec terminals
    for (&id, payload_type) in &ctx.grammar.prec_terminal_types {
        let name = ctx.grammar.symbols.name(id);
        let variant_name = format_ident!("{}", name);

        if payload_type.is_some() {
            let field_name = format_ident!("__{}", name.to_lowercase());
            arms.push(quote! {
                #terminal_enum::#variant_name(v, _prec) => {
                    self.value_stack.push(
                        #value_union { #field_name: std::mem::ManuallyDrop::new(v) }
                    );
                }
            });
        } else {
            arms.push(quote! {
                #terminal_enum::#variant_name(_prec) => {
                    self.value_stack.push(#value_union { __unit: () });
                }
            });
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
) -> Vec<TokenStream> {
    // Track which non-terminals have result types
    let typed_nt_names: std::collections::HashSet<&str> = ctx.grammar.nt_types.iter()
        .filter(|(_, ty)| ty.is_some())
        .map(|(id, _)| ctx.grammar.symbols.name(*id))
        .collect();

    let mut arms = Vec::new();

    for (idx, info) in reductions.iter().enumerate() {
        let lhs_field = format_ident!("__{}", info.non_terminal.to_lowercase());
        let lhs_has_type = typed_nt_names.contains(info.non_terminal.as_str());
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
                // Extract value from union field's ManuallyDrop
                let extract = quote! { std::mem::ManuallyDrop::into_inner(#pop_expr.#field_name) };

                stmts.push(quote! { let #var_name = unsafe { #extract }; });
            } else {
                stmts.push(quote! { let _ = #pop_expr; });
            }
        }

        // Statements are already in correct LIFO pop order (built by iterating symbols in reverse)

        // Generate result based on reduction kind
        let result = match &info.action {
            AltAction::Named(method_name) => {
                let method = format_ident!("{}", method_name);
                let args: Vec<_> = typed_symbol_indices(&info.rhs_symbols).iter()
                    .map(|sym_idx| format_ident!("v{}", sym_idx))
                    .collect();
                if lhs_has_type {
                    quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(actions.#method(#(#args),*)?) } }
                } else {
                    quote! { { actions.#method(#(#args),*)?; #value_union { __unit: () } } }
                }
            }
            AltAction::None => {
                if let Some(symbol_index) = info.passthrough_index {
                    let var = format_ident!("v{}", symbol_index);
                    quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(#var) } }
                } else {
                    quote! { #value_union { __unit: () } }
                }
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

    // Terminals with payloads - always drop since we don't know if Copy
    for (&id, payload_type) in &ctx.grammar.terminal_types {
        if payload_type.is_some() {
            let name = ctx.grammar.symbols.name(id);
            if let Some((_, table_id)) = info.terminal_ids.iter().find(|(n, _)| n == name) {
                let field_name = format_ident!("__{}", name.to_lowercase());
                arms.push(quote! {
                    #table_id => { std::mem::ManuallyDrop::into_inner(union_val.#field_name); }
                });
            }
        }
    }

    // Prec terminals with payloads
    for (&id, payload_type) in &ctx.grammar.prec_terminal_types {
        if payload_type.is_some() {
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
    for (id, ty) in &ctx.grammar.nt_types {
        if ty.is_some() {
            let name = ctx.grammar.symbols.name(*id);
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

//! Parser struct and trait code generation.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::reduction::{self, typed_symbol_indices, ReductionInfo, ReductionKind, SymbolKind};
use super::table::CodegenTableInfo;
use super::CodegenContext;

/// Generate the parser wrapper, trait, and related types.
pub fn generate(ctx: &CodegenContext, info: &CodegenTableInfo) -> Result<TokenStream, String> {
    let vis: TokenStream = ctx.visibility.parse().unwrap_or_default();
    let name = &ctx.name;
    let terminal_enum = format_ident!("{}Terminal", name);
    let actions_trait = format_ident!("{}Actions", name);
    let parser_struct = format_ident!("{}Parser", name);
    let value_union = format_ident!("__{}Value", name);
    let table_mod = format_ident!("__{}_table", name.to_lowercase());
    let core_path = ctx.core_path_tokens();

    // Analyze reductions
    let reductions = reduction::analyze_reductions(ctx)?;

    // Collect non-terminals with types (excluding synthetic rules)
    let typed_non_terminals: Vec<_> = ctx.rules.iter()
        .filter(|r| r.result_type.is_some() && !r.name.starts_with("__"))
        .map(|r| (r.name.clone(), r.result_type.clone().unwrap()))
        .collect();

    // All typed non-terminals including synthetic (for value union)
    let all_typed_non_terminals: Vec<_> = ctx.rules.iter()
        .filter(|r| r.result_type.is_some())
        .map(|r| (r.name.clone(), r.result_type.clone().unwrap()))
        .collect();

    // Collect trait methods
    let trait_methods = reduction::collect_trait_methods(&reductions);

    // Get start non-terminal name and check if it has a type
    let start_nt = &ctx.start_symbol;
    let start_has_type = typed_non_terminals.iter().any(|(name, _)| name == start_nt);
    let start_field = format_ident!("__{}", start_nt.to_lowercase());

    // Generate components
    let actions_trait_code = generate_actions_trait(ctx, &actions_trait, &typed_non_terminals, &trait_methods, &vis);
    let value_union_code = generate_value_union(ctx, &all_typed_non_terminals, &value_union, &actions_trait);
    let shift_arms = generate_terminal_shift_arms(ctx, &terminal_enum, &value_union);
    let reduction_arms = generate_reduction_arms(ctx, &reductions, &value_union);
    let drop_arms = generate_drop_arms(ctx, info);

    // Fix the start_field type reference for finish return type
    let start_nt_type = format_ident!("{}", CodegenContext::to_pascal_case(start_nt));

    // Generate finish method based on whether start symbol has a type
    let finish_method = if start_has_type {
        quote! {
            pub fn finish(mut self, actions: &mut A) -> Result<A::#start_nt_type, #core_path::ParseError> {
                // Reduce until done
                loop {
                    match self.parser.maybe_reduce(None) {
                        Ok(Some((rule, _))) => self.do_reduce(rule, actions),
                        Ok(None) => break,
                        Err(e) => return Err(e),
                    }
                }

                if self.parser.is_accepted() {
                    let union_val = self.value_stack.pop().unwrap();
                    self.value_tags.pop();
                    Ok(unsafe { std::mem::ManuallyDrop::into_inner(union_val.#start_field) })
                } else {
                    Err(self.parser.make_error(#core_path::SymbolId::EOF))
                }
            }
        }
    } else {
        quote! {
            pub fn finish(mut self, actions: &mut A) -> Result<(), #core_path::ParseError> {
                // Reduce until done
                loop {
                    match self.parser.maybe_reduce(None) {
                        Ok(Some((rule, _))) => self.do_reduce(rule, actions),
                        Ok(None) => break,
                        Err(e) => return Err(e),
                    }
                }

                if self.parser.is_accepted() {
                    self.value_stack.pop();
                    self.value_tags.pop();
                    Ok(())
                } else {
                    Err(self.parser.make_error(#core_path::SymbolId::EOF))
                }
            }
        }
    };

    Ok(quote! {
        #actions_trait_code

        #value_union_code

        /// Type-safe LR parser.
        #vis struct #parser_struct<A: #actions_trait> {
            parser: #core_path::Parser<'static>,
            value_stack: Vec<#value_union<A>>,
            value_tags: Vec<u32>,
        }

        impl<A: #actions_trait> #parser_struct<A> {
            /// Create a new parser instance.
            pub fn new() -> Self {
                Self {
                    parser: #core_path::Parser::new(#table_mod::TABLE),
                    value_stack: Vec::new(),
                    value_tags: Vec::new(),
                }
            }

            /// Push a terminal, performing any reductions.
            pub fn push(&mut self, terminal: #terminal_enum<A>, actions: &mut A) -> Result<(), #core_path::ParseError> {
                let token = #core_path::Token {
                    terminal: terminal.symbol_id(),
                    prec: terminal.precedence(),
                };

                // Reduce while possible
                loop {
                    match self.parser.maybe_reduce(Some(&token)) {
                        Ok(Some((rule, _))) => self.do_reduce(rule, actions),
                        Ok(None) => break,
                        Err(e) => return Err(e),
                    }
                }

                // Shift the terminal
                let sym_id = token.terminal.0;
                self.parser.shift(&token);

                match terminal {
                    #(#shift_arms)*
                }
                self.value_tags.push(sym_id);

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
                self.parser.format_error(err)
            }

            fn do_reduce(&mut self, rule: usize, actions: &mut A) {
                if rule == 0 { return; }

                let (lhs_id, rhs_len) = #table_mod::RULES[rule];
                let rhs_len = rhs_len as usize;

                for _ in 0..rhs_len {
                    self.value_tags.pop();
                }

                let original_rule_idx = rule - 1;

                let value = match original_rule_idx {
                    #(#reduction_arms)*
                    _ => return,
                };

                self.value_tags.push(lhs_id);
                self.value_stack.push(value);
            }
        }

        impl<A: #actions_trait> Default for #parser_struct<A> {
            fn default() -> Self { Self::new() }
        }

        impl<A: #actions_trait> Drop for #parser_struct<A> {
            fn drop(&mut self) {
                while let Some(union_val) = self.value_stack.pop() {
                    let sym_id = self.value_tags.pop().unwrap();
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

fn generate_actions_trait(
    ctx: &CodegenContext,
    trait_name: &syn::Ident,
    typed_non_terminals: &[(String, String)],
    methods: &[reduction::TraitMethod],
    vis: &TokenStream,
) -> TokenStream {
    let mut assoc_types = Vec::new();

    // Terminal associated types - use payload type name directly
    for (_, payload_type) in &ctx.terminal_types {
        if let Some(type_name) = payload_type {
            let type_ident = format_ident!("{}", type_name);
            assoc_types.push(quote! { type #type_ident; });
        }
    }

    // Prec terminal associated types
    for (_, payload_type) in &ctx.prec_terminal_types {
        if let Some(type_name) = payload_type {
            let type_ident = format_ident!("{}", type_name);
            assoc_types.push(quote! { type #type_ident; });
        }
    }

    // Non-terminal associated types
    for (nt_name, _) in typed_non_terminals {
        let type_name = format_ident!("{}", CodegenContext::to_pascal_case(nt_name));
        assoc_types.push(quote! { type #type_name; });
    }

    // Create sets for quick lookup
    let typed_nt_names: std::collections::HashSet<&str> = typed_non_terminals.iter()
        .map(|(name, _)| name.as_str())
        .collect();

    // Map from terminal name to associated type name
    let terminal_assoc_types: std::collections::BTreeMap<&str, &str> = ctx.terminal_types.iter()
        .filter_map(|(id, ty)| {
            if let Some(type_name) = ty {
                ctx.symbol_names.get(id).map(|name| (name.as_str(), type_name.as_str()))
            } else {
                None
            }
        })
        .chain(
            ctx.prec_terminal_types.iter()
                .filter_map(|(id, ty)| {
                    if let Some(type_name) = ty {
                        ctx.symbol_names.get(id).map(|name| (name.as_str(), type_name.as_str()))
                    } else {
                        None
                    }
                })
        )
        .collect();

    let method_defs: Vec<_> = methods.iter()
        .map(|method| {
            let method_name = format_ident!("{}", method.name);

            // Check if this non-terminal has a result type
            let return_type_tokens = if typed_nt_names.contains(method.non_terminal.as_str()) {
                let return_type = format_ident!("{}", CodegenContext::to_pascal_case(&method.non_terminal));
                quote! { Self::#return_type }
            } else {
                quote! { () }
            };

            let params: Vec<_> = typed_symbol_indices(&method.rhs_symbols).iter().enumerate()
                .map(|(param_idx, &sym_idx)| {
                    let sym = &method.rhs_symbols[sym_idx];
                    let param_name = format_ident!("v{}", param_idx);

                    let param_type: TokenStream = if sym.kind == SymbolKind::NonTerminal {
                        if typed_nt_names.contains(sym.name.as_str()) {
                            // Normal non-terminal - use associated type
                            let assoc = format_ident!("{}", CodegenContext::to_pascal_case(&sym.name));
                            quote! { Self::#assoc }
                        } else if sym.name.starts_with("__") {
                            // Synthetic non-terminal - look up its result type and convert
                            if let Some(rule) = ctx.rules.iter().find(|r| r.name == sym.name) {
                                if let Some(result_type) = &rule.result_type {
                                    synthetic_type_to_tokens_with_prefix(result_type, true) // true = use Self::
                                } else {
                                    quote! { () }
                                }
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
                fn #method_name(&mut self, #(#params),*) -> #return_type_tokens;
            }
        })
        .collect();

    quote! {
        /// Actions trait for parser callbacks.
        #vis trait #trait_name {
            #(#assoc_types)*
            #(#method_defs)*
        }
    }
}

fn generate_value_union(
    ctx: &CodegenContext,
    typed_non_terminals: &[(String, String)],
    value_union: &syn::Ident,
    actions_trait: &syn::Ident,
) -> TokenStream {
    let mut fields = Vec::new();

    // Terminals with payloads - use payload type name as associated type
    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(type_name) = payload_type {
            if let Some(name) = ctx.symbol_names.get(&id) {
                let field_name = format_ident!("__{}", name.to_lowercase());
                let assoc_type = format_ident!("{}", type_name);
                fields.push(quote! { #field_name: std::mem::ManuallyDrop<A::#assoc_type>, });
            }
        }
    }

    // Prec terminals with payloads - use payload type name as associated type
    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(type_name) = payload_type {
            if let Some(name) = ctx.symbol_names.get(&id) {
                let field_name = format_ident!("__{}", name.to_lowercase());
                let assoc_type = format_ident!("{}", type_name);
                fields.push(quote! { #field_name: std::mem::ManuallyDrop<A::#assoc_type>, });
            }
        }
    }

    // Typed non-terminals
    for (name, result_type) in typed_non_terminals {
        let field_name = format_ident!("__{}", name.to_lowercase());

        // Check if this is a synthetic rule
        if name.starts_with("__") {
            // Synthetic rule - use concrete wrapper type with associated inner type
            // result_type is like "Option<Foo>" or "Vec<Foo>"
            let field_type = synthetic_type_to_tokens_with_prefix(&result_type, false); // false = use A::
            fields.push(quote! { #field_name: std::mem::ManuallyDrop<#field_type>, });
        } else {
            // Normal non-terminal - use associated type
            let assoc_type = format_ident!("{}", CodegenContext::to_pascal_case(name));
            fields.push(quote! { #field_name: std::mem::ManuallyDrop<A::#assoc_type>, });
        }
    }

    quote! {
        #[doc(hidden)]
        union #value_union<A: #actions_trait> {
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
    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = format_ident!("{}", CodegenContext::to_pascal_case(name));
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
    }

    // Prec terminals
    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = format_ident!("{}", CodegenContext::to_pascal_case(name));

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
    let typed_nt_names: std::collections::HashSet<&str> = ctx.rules.iter()
        .filter(|r| r.result_type.is_some())
        .map(|r| r.name.as_str())
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
        let result = match &info.kind {
            ReductionKind::Named { method_name } => {
                let method = format_ident!("{}", method_name);
                let args: Vec<_> = typed_symbol_indices(&info.rhs_symbols).iter()
                    .map(|sym_idx| format_ident!("v{}", sym_idx))
                    .collect();
                if lhs_has_type {
                    quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(actions.#method(#(#args),*)) } }
                } else {
                    quote! { { actions.#method(#(#args),*); #value_union { __unit: () } } }
                }
            }
            ReductionKind::Passthrough { symbol_index } => {
                let var = format_ident!("v{}", symbol_index);
                quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(#var) } }
            }
            ReductionKind::Structural => {
                quote! { #value_union { __unit: () } }
            }
            ReductionKind::SyntheticSome => {
                // Check if the symbol is untyped (unit)
                let is_unit = info.rhs_symbols.first().map(|s| s.ty.is_none()).unwrap_or(true);
                if is_unit {
                    quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(Some(())) } }
                } else {
                    quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(Some(v0)) } }
                }
            }
            ReductionKind::SyntheticNone => {
                quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(None) } }
            }
            ReductionKind::SyntheticEmpty => {
                quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(Vec::new()) } }
            }
            ReductionKind::SyntheticSingle => {
                // Check if the element is untyped (unit)
                let is_unit = info.rhs_symbols.first().map(|s| s.ty.is_none()).unwrap_or(true);
                if is_unit {
                    quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(vec![()]) } }
                } else {
                    quote! { #value_union { #lhs_field: std::mem::ManuallyDrop::new(vec![v0]) } }
                }
            }
            ReductionKind::SyntheticAppend => {
                // Check if the element (index 1) is untyped
                let is_unit = info.rhs_symbols.get(1).map(|s| s.ty.is_none()).unwrap_or(true);
                if is_unit {
                    quote! { { let mut v0 = v0; v0.push(()); #value_union { #lhs_field: std::mem::ManuallyDrop::new(v0) } } }
                } else {
                    quote! { { let mut v0 = v0; v0.push(v1); #value_union { #lhs_field: std::mem::ManuallyDrop::new(v0) } } }
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
    for (&id, payload_type) in &ctx.terminal_types {
        if payload_type.is_some()
            && let Some(name) = ctx.symbol_names.get(&id)
            && let Some((_, table_id)) = info.terminal_ids.iter().find(|(n, _)| n == name)
        {
            let field_name = format_ident!("__{}", name.to_lowercase());
            arms.push(quote! {
                #table_id => { std::mem::ManuallyDrop::into_inner(union_val.#field_name); }
            });
        }
    }

    // Prec terminals with payloads
    for (&id, payload_type) in &ctx.prec_terminal_types {
        if payload_type.is_some()
            && let Some(name) = ctx.symbol_names.get(&id)
            && let Some((_, table_id)) = info.terminal_ids.iter().find(|(n, _)| n == name)
        {
            let field_name = format_ident!("__{}", name.to_lowercase());
            arms.push(quote! {
                #table_id => { std::mem::ManuallyDrop::into_inner(union_val.#field_name); }
            });
        }
    }

    // Non-terminals
    for name in &ctx.rule_names {
        if ctx.rules.iter().any(|r| r.name == *name && r.result_type.is_some()) {
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

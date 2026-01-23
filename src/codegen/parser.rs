//! Parser struct and trait code generation.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::reduction::{self, ReductionInfo, ReductionKind, SymbolKind};
use super::table::TableData;
use super::{is_copy_type, CodegenContext};

/// Generate the parser wrapper, trait, and related types.
pub fn generate(ctx: &CodegenContext, table_data: &TableData) -> Result<TokenStream, String> {
    let vis: TokenStream = ctx.visibility.parse().unwrap_or_default();
    let name = &ctx.name;
    let terminal_enum = format_ident!("{}Terminal", name);
    let actions_trait = format_ident!("{}Actions", name);
    let parser_struct = format_ident!("{}Parser", name);
    let error_struct = format_ident!("{}Error", name);
    let value_union = format_ident!("__{}Value", name);
    let table_mod = format_ident!("__{}_table", name.to_lowercase());

    // Analyze reductions
    let reductions = reduction::analyze_reductions(ctx)?;

    // Collect non-terminals with types
    let typed_non_terminals: Vec<_> = ctx.rules.iter()
        .filter(|r| r.result_type.is_some())
        .map(|r| (r.name.clone(), r.result_type.clone().unwrap()))
        .collect();

    // Collect trait methods
    let trait_methods = reduction::collect_trait_methods(&reductions);

    // Get start non-terminal name and check if it has a type
    let start_nt = &ctx.start_symbol;
    let start_has_type = typed_non_terminals.iter().any(|(name, _)| name == start_nt);
    let start_nt_type = format_ident!("{}", CodegenContext::to_pascal_case(start_nt));
    let start_field = format_ident!("__{}", start_nt.to_lowercase());

    // Generate components
    let actions_trait_code = generate_actions_trait(&actions_trait, &typed_non_terminals, &trait_methods, &vis);
    let value_union_code = generate_value_union(ctx, &typed_non_terminals, &value_union, &actions_trait);
    let shift_arms = generate_terminal_shift_arms(ctx, &terminal_enum, &value_union);
    let reduction_arms = generate_reduction_arms(ctx, &reductions, &value_union);
    let drop_arms = generate_drop_arms(ctx, table_data);

    // Generate finish method based on whether start symbol has a type
    let finish_method = if start_has_type {
        quote! {
            pub fn finish(mut self, actions: &mut A) -> Result<A::#start_nt_type, #error_struct> {
                loop {
                    let state = self.current_state();
                    let action = self.lookup_action(state, 0);

                    match action & 3 {
                        2 => {
                            let rule = (action >> 2) as usize;
                            self.do_reduce(rule, actions);
                        }
                        3 => {
                            if action == 3 {
                                if let Some(value) = self.value_stack.pop() {
                                    self.state_stack.pop();
                                    let union_val = std::mem::ManuallyDrop::into_inner(value);
                                    return Ok(unsafe { std::mem::ManuallyDrop::into_inner(union_val.#start_field) });
                                }
                            } else {
                                let reduce_rule = (action >> 17) as usize;
                                self.do_reduce(reduce_rule, actions);
                            }
                        }
                        _ => return Err(#error_struct { state }),
                    }
                }
            }
        }
    } else {
        quote! {
            pub fn finish(mut self, actions: &mut A) -> Result<(), #error_struct> {
                loop {
                    let state = self.current_state();
                    let action = self.lookup_action(state, 0);

                    match action & 3 {
                        2 => {
                            let rule = (action >> 2) as usize;
                            self.do_reduce(rule, actions);
                        }
                        3 => {
                            if action == 3 {
                                if let Some(value) = self.value_stack.pop() {
                                    self.state_stack.pop();
                                    let _ = std::mem::ManuallyDrop::into_inner(value);
                                    return Ok(());
                                }
                            } else {
                                let reduce_rule = (action >> 17) as usize;
                                self.do_reduce(reduce_rule, actions);
                            }
                        }
                        _ => return Err(#error_struct { state }),
                    }
                }
            }
        }
    };

    Ok(quote! {
        /// Parse error.
        #[derive(Debug, Clone)]
        #vis struct #error_struct {
            /// The parser state when error occurred.
            pub state: usize,
        }

        #actions_trait_code

        #value_union_code

        /// Type-safe LR parser.
        #vis struct #parser_struct<A: #actions_trait> {
            state_stack: Vec<(usize, Option<(u8, u8)>)>,
            value_stack: Vec<std::mem::ManuallyDrop<#value_union<A>>>,
        }

        impl<A: #actions_trait> #parser_struct<A> {
            /// Create a new parser instance.
            pub fn new() -> Self {
                Self {
                    state_stack: vec![(0, None)],
                    value_stack: Vec::new(),
                }
            }

            /// Push a terminal, performing any reductions.
            pub fn push(&mut self, terminal: #terminal_enum, actions: &mut A) -> Result<(), #error_struct> {
                let token_prec = terminal.precedence();
                loop {
                    let (state, stack_prec) = self.current_state_and_prec();
                    let symbol_id = terminal.symbol_id().0;
                    let action = self.lookup_action(state, symbol_id);

                    match action & 3 {
                        0 => return Err(#error_struct { state }),
                        1 => {
                            let next_state = (action >> 2) as usize;
                            self.do_shift(&terminal, next_state, token_prec);
                            return Ok(());
                        }
                        2 => {
                            let rule = (action >> 2) as usize;
                            self.do_reduce(rule, actions);
                        }
                        3 if action != 3 => {
                            let shift_state = ((action >> 3) & 0x3FFF) as usize;
                            let reduce_rule = (action >> 17) as usize;

                            let should_shift = match (stack_prec, token_prec) {
                                (Some((sp, _)), Some((tp, assoc))) => {
                                    if tp > sp { true }
                                    else if tp < sp { false }
                                    else { assoc == 1 }
                                }
                                _ => true,
                            };

                            if should_shift {
                                self.do_shift(&terminal, shift_state, token_prec);
                                return Ok(());
                            } else {
                                self.do_reduce(reduce_rule, actions);
                            }
                        }
                        _ => return Err(#error_struct { state }),
                    }
                }
            }

            /// Finish parsing and return the result.
            #finish_method

            /// Get the current parser state.
            pub fn state(&self) -> usize {
                self.current_state()
            }

            fn current_state(&self) -> usize {
                self.state_stack.last().unwrap().0
            }

            fn current_state_and_prec(&self) -> (usize, Option<(u8, u8)>) {
                let (state, _) = *self.state_stack.last().unwrap();
                let prec = self.state_stack.iter().rev().find_map(|(_, p)| *p);
                (state, prec)
            }

            fn lookup_action(&self, state: usize, terminal: u32) -> u32 {
                let base = #table_mod::ACTION_BASE[state];
                let index = base.wrapping_add(terminal as i32) as usize;

                if index < #table_mod::ACTION_CHECK.len() && #table_mod::ACTION_CHECK[index] == state as u32 {
                    #table_mod::ACTION_DATA[index]
                } else {
                    0
                }
            }

            fn lookup_goto(&self, state: usize, non_terminal: u32) -> Option<usize> {
                let base = #table_mod::GOTO_BASE[state];
                let index = base.wrapping_add(non_terminal as i32) as usize;

                if index < #table_mod::GOTO_CHECK.len() && #table_mod::GOTO_CHECK[index] == state as u32 {
                    Some(#table_mod::GOTO_DATA[index] as usize)
                } else {
                    None
                }
            }

            fn do_shift(&mut self, terminal: &#terminal_enum, next_state: usize, prec: Option<(u8, u8)>) {
                self.state_stack.push((next_state, prec));
                match terminal {
                    #(#shift_arms)*
                }
            }

            fn do_reduce(&mut self, rule: usize, actions: &mut A) {
                if rule == 0 { return; }

                let (lhs_id, rhs_len) = #table_mod::RULES[rule];
                let rhs_len = rhs_len as usize;

                for _ in 0..rhs_len {
                    self.state_stack.pop();
                }

                let original_rule_idx = rule - 1;

                let value = match original_rule_idx {
                    #(#reduction_arms)*
                    _ => return,
                };

                self.value_stack.push(std::mem::ManuallyDrop::new(value));

                let goto_state = self.current_state();
                let nt_index = lhs_id - #table_mod::NUM_TERMINALS - 1;
                if let Some(next_state) = self.lookup_goto(goto_state, nt_index) {
                    self.state_stack.push((next_state, None));
                }
            }
        }

        impl<A: #actions_trait> Default for #parser_struct<A> {
            fn default() -> Self { Self::new() }
        }

        impl<A: #actions_trait> Drop for #parser_struct<A> {
            fn drop(&mut self) {
                while let Some(value) = self.value_stack.pop() {
                    let (state, _) = self.state_stack.pop().unwrap();
                    let sym_id = #table_mod::STATE_SYMBOL[state];
                    unsafe {
                        let union_val = std::mem::ManuallyDrop::into_inner(value);
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
    trait_name: &syn::Ident,
    typed_non_terminals: &[(String, String)],
    methods: &[reduction::TraitMethod],
    vis: &TokenStream,
) -> TokenStream {
    let assoc_types: Vec<_> = typed_non_terminals.iter()
        .map(|(nt_name, _)| {
            let type_name = format_ident!("{}", CodegenContext::to_pascal_case(nt_name));
            quote! { type #type_name; }
        })
        .collect();

    // Create a set of typed non-terminal names for quick lookup
    let typed_nt_names: std::collections::HashSet<&str> = typed_non_terminals.iter()
        .map(|(name, _)| name.as_str())
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

            let params: Vec<_> = method.params.iter().enumerate()
                .map(|(param_idx, (sym_idx, ty))| {
                    let sym = &method.rhs_symbols[*sym_idx];
                    let param_name = format_ident!("v{}", param_idx);

                    let param_type: TokenStream = if sym.kind == SymbolKind::NonTerminal {
                        // Check if this non-terminal has a type
                        if typed_nt_names.contains(sym.name.as_str()) {
                            let assoc = format_ident!("{}", CodegenContext::to_pascal_case(&sym.name));
                            quote! { Self::#assoc }
                        } else {
                            quote! { () }
                        }
                    } else {
                        ty.parse().unwrap()
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

    // Terminals with payloads
    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(ty) = payload_type {
            if let Some(name) = ctx.symbol_names.get(&id) {
                let field_name = format_ident!("__{}", name.to_lowercase());
                let ty: TokenStream = ty.parse().unwrap();
                if is_copy_type(&ty.to_string()) {
                    fields.push(quote! { #field_name: #ty, });
                } else {
                    fields.push(quote! { #field_name: std::mem::ManuallyDrop<#ty>, });
                }
            }
        }
    }

    // Prec terminals with payloads
    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(ty) = payload_type {
            if let Some(name) = ctx.symbol_names.get(&id) {
                let field_name = format_ident!("__{}", name.to_lowercase());
                let ty: TokenStream = ty.parse().unwrap();
                if is_copy_type(&ty.to_string()) {
                    fields.push(quote! { #field_name: #ty, });
                } else {
                    fields.push(quote! { #field_name: std::mem::ManuallyDrop<#ty>, });
                }
            }
        }
    }

    // Typed non-terminals
    for (name, _) in typed_non_terminals {
        let field_name = format_ident!("__{}", name.to_lowercase());
        let assoc_type = format_ident!("{}", CodegenContext::to_pascal_case(name));
        fields.push(quote! { #field_name: std::mem::ManuallyDrop<A::#assoc_type>, });
    }

    quote! {
        #[doc(hidden)]
        union #value_union<A: #actions_trait> {
            #(#fields)*
            __unit: (),
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

            if let Some(ty) = payload_type {
                let inner = if is_copy_type(ty) {
                    quote! { #value_union { #field_name: *v } }
                } else {
                    quote! { #value_union { #field_name: std::mem::ManuallyDrop::new(v.clone()) } }
                };
                arms.push(quote! {
                    #terminal_enum::#variant_name(v) => {
                        self.value_stack.push(std::mem::ManuallyDrop::new(#inner));
                    }
                });
            } else {
                arms.push(quote! {
                    #terminal_enum::#variant_name => {
                        self.value_stack.push(std::mem::ManuallyDrop::new(#value_union { __unit: () }));
                    }
                });
            }
        }
    }

    // Prec terminals
    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = format_ident!("{}", CodegenContext::to_pascal_case(name));

            if let Some(ty) = payload_type {
                let field_name = format_ident!("__{}", name.to_lowercase());
                let inner = if is_copy_type(ty) {
                    quote! { #value_union { #field_name: *v } }
                } else {
                    quote! { #value_union { #field_name: std::mem::ManuallyDrop::new(v.clone()) } }
                };
                arms.push(quote! {
                    #terminal_enum::#variant_name(v, _prec) => {
                        self.value_stack.push(std::mem::ManuallyDrop::new(#inner));
                    }
                });
            } else {
                arms.push(quote! {
                    #terminal_enum::#variant_name(_prec) => {
                        self.value_stack.push(std::mem::ManuallyDrop::new(#value_union { __unit: () }));
                    }
                });
            }
        }
    }

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
        let mut var_indices = Vec::new();

        for (i, sym) in info.rhs_symbols.iter().enumerate().rev() {
            let pop_expr = quote! { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };

            if let Some(ty) = &sym.ty {
                let field_name = match sym.kind {
                    SymbolKind::UnitTerminal => {
                        stmts.push(quote! { { #pop_expr }; });
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
                let extract = match sym.kind {
                    SymbolKind::PayloadTerminal | SymbolKind::PrecTerminal => {
                        if is_copy_type(ty) {
                            quote! { #pop_expr.#field_name }
                        } else {
                            quote! { std::mem::ManuallyDrop::into_inner(#pop_expr.#field_name) }
                        }
                    }
                    SymbolKind::NonTerminal => {
                        quote! { std::mem::ManuallyDrop::into_inner(#pop_expr.#field_name) }
                    }
                    SymbolKind::UnitTerminal => unreachable!(),
                };

                stmts.push(quote! { let #var_name = unsafe { #extract }; });
                var_indices.push(i);
            } else {
                stmts.push(quote! { { #pop_expr }; });
            }
        }

        // Statements are already in correct LIFO pop order (built by iterating symbols in reverse)

        // Generate result based on reduction kind
        let result = match &info.kind {
            ReductionKind::Named { method_name, params } => {
                let method = format_ident!("{}", method_name);
                let args: Vec<_> = params.iter()
                    .map(|(sym_idx, _)| format_ident!("v{}", sym_idx))
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

fn generate_drop_arms(ctx: &CodegenContext, table_data: &TableData) -> Vec<TokenStream> {
    let mut arms = Vec::new();

    // Terminals with non-Copy payloads
    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(ty) = payload_type {
            if !is_copy_type(ty) {
                if let Some(name) = ctx.symbol_names.get(&id) {
                    let field_name = format_ident!("__{}", name.to_lowercase());
                    if let Some((_, table_id)) = table_data.terminal_ids.iter().find(|(n, _)| n == name) {
                        arms.push(quote! {
                            #table_id => { std::mem::ManuallyDrop::into_inner(union_val.#field_name); }
                        });
                    }
                }
            }
        }
    }

    // Prec terminals with non-Copy payloads
    for (&id, payload_type) in &ctx.prec_terminal_types {
        if let Some(ty) = payload_type {
            if !is_copy_type(ty) {
                if let Some(name) = ctx.symbol_names.get(&id) {
                    let field_name = format_ident!("__{}", name.to_lowercase());
                    if let Some((_, table_id)) = table_data.terminal_ids.iter().find(|(n, _)| n == name) {
                        arms.push(quote! {
                            #table_id => { std::mem::ManuallyDrop::into_inner(union_val.#field_name); }
                        });
                    }
                }
            }
        }
    }

    // Non-terminals
    for name in &ctx.rule_names {
        if ctx.rules.iter().any(|r| r.name == *name && r.result_type.is_some()) {
            let field_name = format_ident!("__{}", name.to_lowercase());
            if let Some((_, table_id)) = table_data.non_terminal_ids.iter().find(|(n, _)| n == name) {
                arms.push(quote! {
                    #table_id => { std::mem::ManuallyDrop::into_inner(union_val.#field_name); }
                });
            }
        }
    }

    arms
}

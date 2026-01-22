//! Parser wrapper code generation.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::codegen::reduction::{self, ReductionInfo, SymbolKind};
use crate::codegen::table::{self, TableData};
use crate::ir::GrammarIr;

/// Generate the parser wrapper and event types.
pub fn generate(grammar: &GrammarIr, table_data: &TableData) -> TokenStream {
    let vis = &grammar.visibility;
    let name = &grammar.name;

    // Generate all the names
    let terminal_enum = format_ident!("{}Terminal", name);
    let reduction_enum = format_ident!("{}Reduction", name);
    let parser_struct = format_ident!("{}Parser", name);
    let error_struct = format_ident!("{}Error", name);
    let value_union = format_ident!("__{}Value", name);
    let result_struct = format_ident!("__{}ReductionResult", name);
    let table_mod = format_ident!("__{}_table", name.to_string().to_lowercase());

    // Get start symbol result type
    let start_type = grammar
        .rules
        .first()
        .map(|r| &r.result_type)
        .expect("grammar must have at least one rule");

    // Generate the table statics
    let table_statics = table::generate_table_statics(grammar, table_data);

    // Generate the internal value union
    let value_union_def = generate_value_union(grammar, &value_union);

    // Generate the reduction result struct (opaque to user)
    let result_struct_def = quote! {
        #[doc(hidden)]
        #vis struct #result_struct {
            value: #value_union,
            lhs_id: u32,
        }
    };

    // Generate error type
    let error_def = quote! {
        /// Parse error.
        #[derive(Debug, Clone)]
        #vis struct #error_struct {
            /// The parser state when error occurred.
            pub state: usize,
        }
    };

    // Build reduction info for generating match arms
    let reduction_info = reduction::build_reduction_info(grammar, table_data);

    // Generate reduction enum with constructor functions
    let reduction_enum_def = generate_reduction_enum(grammar, table_data, &reduction_info, &value_union, &result_struct);

    // Generate reduction handling code (for do_reduce)
    let reduction_arms = generate_reduction_arms(grammar, table_data, &reduction_info, &value_union, &result_struct);

    // Generate terminal shift handling
    let terminal_shift_arms = generate_terminal_shift_arms(grammar, &value_union);

    // Generate constructor functions for each non-terminal
    let constructor_fns = generate_constructor_fns(grammar, &value_union, &result_struct);

    // Generate accept body based on whether start_type is Copy
    let accept_body = if is_copy_type(start_type) {
        quote! {
            let state = self.current_state();
            let action = self.lookup_action(state, 0);

            if action == 3 {
                if let Some(value) = self.value_stack.pop() {
                    self.state_stack.pop();
                    let union_val = std::mem::ManuallyDrop::into_inner(value);
                    return Ok(unsafe { union_val.__start });
                }
            }
            Err(#error_struct { state })
        }
    } else {
        quote! {
            let state = self.current_state();
            let action = self.lookup_action(state, 0);

            if action == 3 {
                if let Some(value) = self.value_stack.pop() {
                    self.state_stack.pop();
                    let union_val = std::mem::ManuallyDrop::into_inner(value);
                    return Ok(unsafe { std::mem::ManuallyDrop::into_inner(union_val.__start) });
                }
            }
            Err(#error_struct { state })
        }
    };

    // Generate drop arms for each symbol type (keyed by state's accessing symbol)
    let drop_arms = generate_drop_arms(grammar, table_data, &value_union);

    quote! {
        #table_statics

        #value_union_def

        #result_struct_def

        #constructor_fns

        #error_def

        #reduction_enum_def

        /// Type-safe LR parser.
        ///
        /// Usage:
        /// ```ignore
        /// let mut parser = Parser::new();
        /// loop {
        ///     let tok = lexer.next(); // Option<Terminal>
        ///     while let Some(r) = parser.maybe_reduce(&tok) {
        ///         parser.reduce(match r {
        ///             Reduction::ExprBinOp(c, l, op, r) => c(eval(l, op, r)),
        ///             // ...
        ///         });
        ///     }
        ///     if tok.is_none() { break; }
        ///     parser.shift(tok.unwrap())?;
        /// }
        /// parser.accept()
        /// ```
        #vis struct #parser_struct {
            state_stack: Vec<usize>,
            value_stack: Vec<std::mem::ManuallyDrop<#value_union>>,
        }

        impl #parser_struct {
            /// Create a new parser instance.
            pub fn new() -> Self {
                Self {
                    state_stack: vec![0],
                    value_stack: Vec::new(),
                }
            }

            /// Check if a reduction is needed given the lookahead.
            /// Pass `None` for EOF.
            /// Returns `Some(reduction)` if a reduction should be performed.
            pub fn maybe_reduce(&mut self, lookahead: &Option<#terminal_enum>) -> Option<#reduction_enum> {
                let state = self.current_state();
                let symbol_id = match lookahead {
                    Some(t) => t.symbol_id().0,
                    None => 0, // EOF
                };
                let action = self.lookup_action(state, symbol_id);

                match action & 3 {
                    2 => {
                        // Reduce
                        let rule = (action >> 2) as usize;
                        self.do_reduce(rule)
                    }
                    3 if action != 3 => {
                        // ShiftOrReduce - at EOF, reduce; otherwise prefer shift
                        if lookahead.is_none() {
                            let reduce_rule = (action >> 17) as usize;
                            self.do_reduce(reduce_rule)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }

            /// Complete a reduction by pushing the result.
            pub fn reduce(&mut self, result: #result_struct) {
                self.value_stack.push(std::mem::ManuallyDrop::new(result.value));
                self.do_goto_after_reduce(result.lhs_id);
            }

            /// Shift (consume) a terminal.
            /// Call this after `maybe_reduce` returns `None`.
            pub fn shift(&mut self, terminal: #terminal_enum) -> Result<(), #error_struct> {
                let state = self.current_state();
                let symbol_id = terminal.symbol_id();
                let action = self.lookup_action(state, symbol_id.0);

                match action & 3 {
                    0 => {
                        Err(#error_struct { state })
                    }
                    1 => {
                        let next_state = (action >> 2) as usize;
                        self.do_shift(terminal, next_state);
                        Ok(())
                    }
                    3 if action != 3 => {
                        let shift_state = ((action >> 2) & 0x7FFF) as usize;
                        self.do_shift(terminal, shift_state);
                        Ok(())
                    }
                    _ => {
                        Err(#error_struct { state })
                    }
                }
            }

            /// Accept the parse result.
            pub fn accept(mut self) -> Result<#start_type, #error_struct> {
                #accept_body
            }

            /// Get the current parser state (for error reporting).
            pub fn state(&self) -> usize {
                self.current_state()
            }

            fn current_state(&self) -> usize {
                *self.state_stack.last().unwrap()
            }

            fn lookup_action(&self, state: usize, terminal: u32) -> u32 {
                let base = #table_mod::ACTION_BASE[state];
                let index = (base + terminal as i32) as usize;

                if index < #table_mod::ACTION_CHECK.len()
                    && #table_mod::ACTION_CHECK[index] == state as u32
                {
                    #table_mod::ACTION_DATA[index]
                } else {
                    0
                }
            }

            fn lookup_goto(&self, state: usize, non_terminal: u32) -> Option<usize> {
                let base = #table_mod::GOTO_BASE[state];
                let index = (base + non_terminal as i32) as usize;

                if index < #table_mod::GOTO_CHECK.len()
                    && #table_mod::GOTO_CHECK[index] == state as u32
                {
                    let val = #table_mod::GOTO_DATA[index];
                    if val != u32::MAX {
                        Some(val as usize)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }

            fn do_shift(&mut self, terminal: #terminal_enum, next_state: usize) {
                self.state_stack.push(next_state);
                #terminal_shift_arms
            }

            fn do_reduce(&mut self, rule: usize) -> Option<#reduction_enum> {
                if rule == 0 {
                    return None;
                }

                let (_, rhs_len) = #table_mod::RULES[rule];
                let rhs_len = rhs_len as usize;

                for _ in 0..rhs_len {
                    self.state_stack.pop();
                }

                let original_rule_idx = rule - 1;

                let reduction = match original_rule_idx {
                    #reduction_arms
                    _ => return None,
                };

                Some(reduction)
            }

            fn do_goto_after_reduce(&mut self, lhs_id: u32) {
                let goto_state = self.current_state();
                let nt_index = lhs_id - #table_mod::NUM_TERMINALS - 1;
                if let Some(next_state) = self.lookup_goto(goto_state, nt_index) {
                    self.state_stack.push(next_state);
                }
            }
        }

        impl Default for #parser_struct {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Drop for #parser_struct {
            fn drop(&mut self) {
                // Drop any remaining values on the stack
                // state_stack[0] is initial state (no value), state_stack[i] for i>0 corresponds to value_stack[i-1]
                while let Some(value) = self.value_stack.pop() {
                    let state = self.state_stack.pop().unwrap();
                    let sym_id = #table_mod::STATE_SYMBOL[state];
                    unsafe {
                        let union_val = std::mem::ManuallyDrop::into_inner(value);
                        match sym_id {
                            #drop_arms
                            _ => {} // Unit types or Copy types, nothing to drop
                        }
                    }
                }
            }
        }
    }
}

/// Generate the internal value union.
fn generate_value_union(grammar: &GrammarIr, value_union: &syn::Ident) -> TokenStream {
    let vis = &grammar.visibility;
    let mut variants = Vec::new();

    // Add variant for start symbol
    if let Some(start_rule) = grammar.rules.first() {
        let ty = &start_rule.result_type;
        if is_copy_type(ty) {
            variants.push(quote! { __start: #ty });
        } else {
            variants.push(quote! { __start: std::mem::ManuallyDrop<#ty> });
        }
    }

    // Add variants for terminals with payloads
    for terminal in &grammar.terminals {
        if let Some(ty) = &terminal.payload_type {
            let field_name = format_ident!("__{}", terminal.name.to_string().to_lowercase());
            if is_copy_type(ty) {
                variants.push(quote! { #field_name: #ty });
            } else {
                variants.push(quote! { #field_name: std::mem::ManuallyDrop<#ty> });
            }
        }
    }

    // Add variants for prec_terminals
    for prec_terminal in &grammar.prec_terminals {
        let field_name = format_ident!("__{}", prec_terminal.name.to_string().to_lowercase());
        let ty = &prec_terminal.payload_type;
        if is_copy_type(ty) {
            variants.push(quote! { #field_name: #ty });
        } else {
            variants.push(quote! { #field_name: std::mem::ManuallyDrop<#ty> });
        }
    }

    // Add variants for non-terminals (skip start, already covered)
    for rule in grammar.rules.iter().skip(1) {
        let field_name = format_ident!("__{}", rule.name.to_string().to_lowercase());
        let ty = &rule.result_type;
        if is_copy_type(ty) {
            variants.push(quote! { #field_name: #ty });
        } else {
            variants.push(quote! { #field_name: std::mem::ManuallyDrop<#ty> });
        }
    }

    // Add unit variant
    variants.push(quote! { __unit: () });

    quote! {
        #[doc(hidden)]
        #vis union #value_union {
            #(#variants,)*
        }
    }
}

/// Check if a type is likely Copy (simple heuristic).
fn is_copy_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(path) => {
            if let Some(ident) = path.path.get_ident() {
                let s = ident.to_string();
                matches!(s.as_str(),
                    "i8" | "i16" | "i32" | "i64" | "i128" | "isize" |
                    "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
                    "f32" | "f64" | "bool" | "char" | "()"
                )
            } else {
                false
            }
        }
        syn::Type::Tuple(tuple) if tuple.elems.is_empty() => true,
        _ => false,
    }
}

/// Generate constructor functions for each non-terminal.
fn generate_constructor_fns(
    grammar: &GrammarIr,
    value_union: &syn::Ident,
    result_struct: &syn::Ident,
) -> TokenStream {
    let table_mod = format_ident!("__{}_table", grammar.name.to_string().to_lowercase());
    let mut fns = Vec::new();

    for (idx, rule) in grammar.rules.iter().enumerate() {
        let fn_name = format_ident!("__construct_{}", rule.name);
        let ty = &rule.result_type;
        let nt_name = rule.name.to_string();

        let field_name = if idx == 0 {
            format_ident!("__start")
        } else {
            format_ident!("__{}", rule.name.to_string().to_lowercase())
        };

        let value_expr = if is_copy_type(ty) {
            quote! { #value_union { #field_name: value } }
        } else {
            quote! { #value_union { #field_name: std::mem::ManuallyDrop::new(value) } }
        };

        fns.push(quote! {
            #[doc(hidden)]
            fn #fn_name(value: #ty) -> #result_struct {
                #result_struct {
                    value: #value_expr,
                    lhs_id: #table_mod::symbol_id(#nt_name).0,
                }
            }
        });
    }

    quote! { #(#fns)* }
}

/// Generate the reduction enum with constructor functions.
fn generate_reduction_enum(
    grammar: &GrammarIr,
    table_data: &TableData,
    reduction_info: &[ReductionInfo],
    _value_union: &syn::Ident,
    result_struct: &syn::Ident,
) -> TokenStream {
    let vis = &grammar.visibility;
    let name = &grammar.name;
    let reduction_enum = format_ident!("{}Reduction", name);

    let mut variants = Vec::new();

    for (idx, info) in reduction_info.iter().enumerate() {
        let variant_name = &info.variant_name;
        let rule_info = &table_data.rule_mapping[idx];

        // Get the result type for this rule's LHS
        let lhs_name = &rule_info.non_terminal_name;
        let result_ty = grammar
            .get_rule(lhs_name)
            .map(|r| &r.result_type)
            .expect("rule must exist");

        // Build variant fields: constructor fn + values
        let mut field_types = vec![quote! { fn(#result_ty) -> #result_struct }];

        for (i, kind) in info.rhs_kinds.iter().enumerate() {
            let sym = &rule_info.rhs_symbols[i];
            match kind {
                SymbolKind::UnitTerminal => {}
                SymbolKind::PayloadTerminal => {
                    if let Some(terminal) = grammar.get_terminal(&sym.name().to_string()) {
                        if let Some(ty) = &terminal.payload_type {
                            field_types.push(quote! { #ty });
                        }
                    }
                }
                SymbolKind::PrecTerminal => {
                    if let Some(prec_terminal) = grammar.get_prec_terminal(&sym.name().to_string()) {
                        let ty = &prec_terminal.payload_type;
                        field_types.push(quote! { #ty });
                    }
                }
                SymbolKind::NonTerminal => {
                    if let Some(rule) = grammar.get_rule(&sym.name().to_string()) {
                        let ty = &rule.result_type;
                        field_types.push(quote! { #ty });
                    }
                }
            }
        }

        variants.push(quote! { #variant_name(#(#field_types),*) });
    }

    // Generate Debug impl manually since fn pointers can't derive Debug
    let debug_arms: Vec<_> = reduction_info.iter().map(|info| {
        let variant_name = &info.variant_name;
        let variant_str = variant_name.to_string();
        quote! { Self::#variant_name(..) => write!(f, #variant_str) }
    }).collect();

    quote! {
        /// Reduction variants with constructor functions.
        #vis enum #reduction_enum {
            #(#variants,)*
        }

        impl std::fmt::Debug for #reduction_enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#debug_arms,)*
                }
            }
        }
    }
}

/// Generate match arms for terminal shift.
fn generate_terminal_shift_arms(grammar: &GrammarIr, value_union: &syn::Ident) -> TokenStream {
    let terminal_enum = format_ident!("{}Terminal", grammar.name);
    let mut arms = Vec::new();

    for terminal in &grammar.terminals {
        let variant_name = GrammarIr::terminal_variant_name(&terminal.name);
        let field_name = format_ident!("__{}", terminal.name.to_string().to_lowercase());

        if let Some(ty) = &terminal.payload_type {
            let inner_value = if is_copy_type(ty) {
                quote! { #value_union { #field_name: v } }
            } else {
                quote! { #value_union { #field_name: std::mem::ManuallyDrop::new(v) } }
            };
            arms.push(quote! {
                #terminal_enum::#variant_name(v) => {
                    self.value_stack.push(std::mem::ManuallyDrop::new(#inner_value));
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

    for prec_terminal in &grammar.prec_terminals {
        let variant_name = GrammarIr::terminal_variant_name(&prec_terminal.name);
        let field_name = format_ident!("__{}", prec_terminal.name.to_string().to_lowercase());
        let ty = &prec_terminal.payload_type;

        let inner_value = if is_copy_type(ty) {
            quote! { #value_union { #field_name: v } }
        } else {
            quote! { #value_union { #field_name: std::mem::ManuallyDrop::new(v) } }
        };

        arms.push(quote! {
            #terminal_enum::#variant_name(v, _prec) => {
                self.value_stack.push(std::mem::ManuallyDrop::new(#inner_value));
            }
        });
    }

    quote! {
        match terminal {
            #(#arms)*
        }
    }
}

/// Generate match arms for reduction handling.
fn generate_reduction_arms(
    grammar: &GrammarIr,
    table_data: &TableData,
    reduction_info: &[ReductionInfo],
    _value_union: &syn::Ident,
    _result_struct: &syn::Ident,
) -> TokenStream {
    let reduction_enum = format_ident!("{}Reduction", grammar.name);

    let mut arms = Vec::new();

    for (idx, info) in reduction_info.iter().enumerate() {
        let variant_name = &info.variant_name;
        let rule_info = &table_data.rule_mapping[idx];

        // Get constructor function name
        let lhs_name = &rule_info.non_terminal_name;
        let constructor_fn = format_ident!("__construct_{}", lhs_name);

        // Generate code to pop values
        let mut pop_code = Vec::new();
        let mut reduction_args: Vec<TokenStream> = Vec::new();

        // Add constructor as first arg
        reduction_args.push(quote! { #constructor_fn });

        // Pop values in reverse order
        for (i, kind) in info.rhs_kinds.iter().enumerate().rev() {
            let val_name = format_ident!("v{}", i);
            let sym = &rule_info.rhs_symbols[i];

            match kind {
                SymbolKind::UnitTerminal => {
                    pop_code.push(quote! { self.value_stack.pop(); });
                }
                SymbolKind::PayloadTerminal => {
                    let field_name = format_ident!("__{}", sym.name().to_string().to_lowercase());
                    if let Some(terminal) = grammar.get_terminal(&sym.name().to_string()) {
                        if let Some(ty) = &terminal.payload_type {
                            // Stack holds ManuallyDrop<Union>, union field holds T or ManuallyDrop<T>
                            let extract = if is_copy_type(ty) {
                                quote! { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).#field_name }
                            } else {
                                quote! { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).#field_name) }
                            };
                            pop_code.push(quote! { let #val_name = unsafe { #extract }; });
                            reduction_args.push(quote! { #val_name });
                        }
                    }
                }
                SymbolKind::PrecTerminal => {
                    let field_name = format_ident!("__{}", sym.name().to_string().to_lowercase());
                    if let Some(prec_terminal) = grammar.get_prec_terminal(&sym.name().to_string()) {
                        let ty = &prec_terminal.payload_type;
                        let extract = if is_copy_type(ty) {
                            quote! { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).#field_name }
                        } else {
                            quote! { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).#field_name) }
                        };
                        pop_code.push(quote! { let #val_name = unsafe { #extract }; });
                        reduction_args.push(quote! { #val_name });
                    }
                }
                SymbolKind::NonTerminal => {
                    let nt_name = sym.name().to_string();
                    let is_start = grammar.rules.first().map(|r| r.name.to_string() == nt_name).unwrap_or(false);

                    let field_name = if is_start {
                        format_ident!("__start")
                    } else {
                        format_ident!("__{}", nt_name.to_lowercase())
                    };

                    if let Some(rule) = grammar.get_rule(&nt_name) {
                        let ty = &rule.result_type;
                        let extract = if is_copy_type(ty) {
                            quote! { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).#field_name }
                        } else {
                            quote! { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).#field_name) }
                        };
                        pop_code.push(quote! { let #val_name = unsafe { #extract }; });
                        reduction_args.push(quote! { #val_name });
                    }
                }
            }
        }

        // Reverse args (except constructor which stays first)
        let constructor = reduction_args.remove(0);
        reduction_args.reverse();
        reduction_args.insert(0, constructor);

        let arm = quote! {
            #idx => {
                #(#pop_code)*
                #reduction_enum::#variant_name(#(#reduction_args),*)
            }
        };

        arms.push(arm);
    }

    quote! { #(#arms,)* }
}

/// Generate match arms for dropping values by symbol ID.
fn generate_drop_arms(
    grammar: &GrammarIr,
    table_data: &TableData,
    _value_union: &syn::Ident,
) -> TokenStream {
    let mut arms = Vec::new();

    // Generate drop arms for terminals with non-Copy payloads
    for terminal in &grammar.terminals {
        if let Some(ty) = &terminal.payload_type {
            if !is_copy_type(ty) {
                let name_str = terminal.name.to_string();
                let field_name = format_ident!("__{}", name_str.to_lowercase());

                // Find the symbol ID for this terminal
                if let Some((_, id)) = table_data.terminal_ids.iter().find(|(n, _)| n == &name_str) {
                    arms.push(quote! {
                        #id => {
                            std::mem::ManuallyDrop::into_inner(union_val.#field_name);
                        }
                    });
                }
            }
        }
    }

    // Generate drop arms for prec_terminals with non-Copy payloads
    for prec_terminal in &grammar.prec_terminals {
        let ty = &prec_terminal.payload_type;
        if !is_copy_type(ty) {
            let name_str = prec_terminal.name.to_string();
            let field_name = format_ident!("__{}", name_str.to_lowercase());

            if let Some((_, id)) = table_data.terminal_ids.iter().find(|(n, _)| n == &name_str) {
                arms.push(quote! {
                    #id => {
                        std::mem::ManuallyDrop::into_inner(union_val.#field_name);
                    }
                });
            }
        }
    }

    // Generate drop arms for non-terminals with non-Copy result types
    for (idx, rule) in grammar.rules.iter().enumerate() {
        let ty = &rule.result_type;
        if !is_copy_type(ty) {
            let name_str = rule.name.to_string();
            let field_name = if idx == 0 {
                format_ident!("__start")
            } else {
                format_ident!("__{}", name_str.to_lowercase())
            };

            if let Some((_, id)) = table_data.non_terminal_ids.iter().find(|(n, _)| n == &name_str) {
                arms.push(quote! {
                    #id => {
                        std::mem::ManuallyDrop::into_inner(union_val.#field_name);
                    }
                });
            }
        }
    }

    quote! { #(#arms)* }
}

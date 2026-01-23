//! Parser struct and trait code generation.

use std::fmt::Write;

use super::reduction::{self, ReductionInfo, ReductionKind, SymbolKind};
use super::table::TableData;
use super::{is_copy_type, CodegenContext};

/// Generate the parser wrapper, trait, and related types.
pub fn generate(ctx: &CodegenContext, table_data: &TableData) -> String {
    let mut out = String::new();

    let vis = &ctx.visibility;
    let name = &ctx.name;
    let terminal_enum = format!("{}Terminal", name);
    let actions_trait = format!("{}Actions", name);
    let parser_struct = format!("{}Parser", name);
    let error_struct = format!("{}Error", name);
    let value_union = format!("__{}Value", name);
    let table_mod = format!("__{}_table", name.to_lowercase());

    // Analyze reductions
    let reductions = match reduction::analyze_reductions(ctx) {
        Ok(r) => r,
        Err(e) => {
            // Return an error as compile_error!
            return format!("compile_error!({:?});", e);
        }
    };

    // Collect non-terminals with types (for associated types)
    let typed_non_terminals: Vec<_> = ctx.rules.iter()
        .filter(|r| r.result_type.is_some())
        .map(|r| (r.name.clone(), r.result_type.clone().unwrap()))
        .collect();

    // Collect trait methods
    let trait_methods = reduction::collect_trait_methods(&reductions);

    // Get start non-terminal name
    let start_nt = ctx.rules.first().map(|r| r.name.as_str()).unwrap_or("start");

    // Error struct
    writeln!(out, "/// Parse error.").unwrap();
    writeln!(out, "#[derive(Debug, Clone)]").unwrap();
    writeln!(out, "{} struct {} {{", vis, error_struct).unwrap();
    writeln!(out, "    /// The parser state when error occurred.").unwrap();
    writeln!(out, "    pub state: usize,").unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    // Actions trait
    writeln!(out, "{}", generate_actions_trait(ctx, &actions_trait, &typed_non_terminals, &trait_methods, vis)).unwrap();
    writeln!(out).unwrap();

    // Value union
    writeln!(out, "{}", generate_value_union(ctx, &typed_non_terminals, &value_union, &actions_trait, vis)).unwrap();
    writeln!(out).unwrap();

    // Parser struct
    writeln!(out, "/// Type-safe LR parser.").unwrap();
    writeln!(out, "{}struct {}<A: {}> {{", vis, parser_struct, actions_trait).unwrap();
    writeln!(out, "    state_stack: Vec<(usize, Option<(u8, u8)>)>,  // (state, precedence: (level, assoc))").unwrap();
    writeln!(out, "    value_stack: Vec<std::mem::ManuallyDrop<{}<A>>>,", value_union).unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    // Parser impl
    writeln!(out, "impl<A: {}> {}<A> {{", actions_trait, parser_struct).unwrap();
    writeln!(out, "    /// Create a new parser instance.").unwrap();
    writeln!(out, "    pub fn new() -> Self {{").unwrap();
    writeln!(out, "        Self {{").unwrap();
    writeln!(out, "            state_stack: vec![(0, None)],").unwrap();
    writeln!(out, "            value_stack: Vec::new(),").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // push method
    writeln!(out, "    /// Push a terminal, performing any reductions.").unwrap();
    writeln!(out, "    pub fn push(&mut self, terminal: {}, actions: &mut A) -> Result<(), {}> {{", terminal_enum, error_struct).unwrap();
    writeln!(out, "        let token_prec = terminal.precedence();").unwrap();
    writeln!(out, "        // Reduce loop").unwrap();
    writeln!(out, "        loop {{").unwrap();
    writeln!(out, "            let (state, stack_prec) = self.current_state_and_prec();").unwrap();
    writeln!(out, "            let symbol_id = terminal.symbol_id().0;").unwrap();
    writeln!(out, "            let action = self.lookup_action(state, symbol_id);").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "            match action & 3 {{").unwrap();
    writeln!(out, "                0 => return Err({} {{ state }}),", error_struct).unwrap();
    writeln!(out, "                1 => {{").unwrap();
    writeln!(out, "                    // Shift").unwrap();
    writeln!(out, "                    let next_state = (action >> 2) as usize;").unwrap();
    writeln!(out, "                    self.do_shift(&terminal, next_state, token_prec);").unwrap();
    writeln!(out, "                    return Ok(());").unwrap();
    writeln!(out, "                }}").unwrap();
    writeln!(out, "                2 => {{").unwrap();
    writeln!(out, "                    // Reduce").unwrap();
    writeln!(out, "                    let rule = (action >> 2) as usize;").unwrap();
    writeln!(out, "                    self.do_reduce(rule, actions);").unwrap();
    writeln!(out, "                }}").unwrap();
    writeln!(out, "                3 if action != 3 => {{").unwrap();
    writeln!(out, "                    // Shift/reduce: compare precedences").unwrap();
    writeln!(out, "                    let shift_state = ((action >> 3) & 0x3FFF) as usize;").unwrap();
    writeln!(out, "                    let reduce_rule = (action >> 17) as usize;").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "                    let should_shift = match (stack_prec, token_prec) {{").unwrap();
    writeln!(out, "                        (Some((sp, _)), Some((tp, assoc))) => {{").unwrap();
    writeln!(out, "                            if tp > sp {{ true }}").unwrap();
    writeln!(out, "                            else if tp < sp {{ false }}").unwrap();
    writeln!(out, "                            else {{ assoc == 1 }}  // 1 = right-assoc = shift").unwrap();
    writeln!(out, "                        }}").unwrap();
    writeln!(out, "                        _ => true,  // default to shift").unwrap();
    writeln!(out, "                    }};").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "                    if should_shift {{").unwrap();
    writeln!(out, "                        self.do_shift(&terminal, shift_state, token_prec);").unwrap();
    writeln!(out, "                        return Ok(());").unwrap();
    writeln!(out, "                    }} else {{").unwrap();
    writeln!(out, "                        self.do_reduce(reduce_rule, actions);").unwrap();
    writeln!(out, "                    }}").unwrap();
    writeln!(out, "                }}").unwrap();
    writeln!(out, "                _ => return Err({} {{ state }}),", error_struct).unwrap();
    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // finish method
    let start_field = format!("__{}", start_nt.to_lowercase());
    // Non-terminal fields are always ManuallyDrop (since we don't know if associated type is Copy)
    let accept_extract = format!("std::mem::ManuallyDrop::into_inner(union_val.{})", start_field);

    writeln!(out, "    /// Finish parsing and return the result.").unwrap();
    writeln!(out, "    pub fn finish(mut self, actions: &mut A) -> Result<A::{}, {}> {{", CodegenContext::to_pascal_case(start_nt), error_struct).unwrap();
    writeln!(out, "        // Reduce until accept").unwrap();
    writeln!(out, "        loop {{").unwrap();
    writeln!(out, "            let state = self.current_state();").unwrap();
    writeln!(out, "            let action = self.lookup_action(state, 0); // EOF").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "            match action & 3 {{").unwrap();
    writeln!(out, "                2 => {{").unwrap();
    writeln!(out, "                    // Reduce").unwrap();
    writeln!(out, "                    let rule = (action >> 2) as usize;").unwrap();
    writeln!(out, "                    self.do_reduce(rule, actions);").unwrap();
    writeln!(out, "                }}").unwrap();
    writeln!(out, "                3 => {{").unwrap();
    writeln!(out, "                    if action == 3 {{").unwrap();
    writeln!(out, "                        // Accept").unwrap();
    writeln!(out, "                        if let Some(value) = self.value_stack.pop() {{").unwrap();
    writeln!(out, "                            self.state_stack.pop();").unwrap();
    writeln!(out, "                            let union_val = std::mem::ManuallyDrop::into_inner(value);").unwrap();
    writeln!(out, "                            return Ok(unsafe {{ {} }});", accept_extract).unwrap();
    writeln!(out, "                        }}").unwrap();
    writeln!(out, "                    }} else {{").unwrap();
    writeln!(out, "                        // Shift/reduce with EOF lookahead -> reduce").unwrap();
    writeln!(out, "                        let reduce_rule = (action >> 17) as usize;").unwrap();
    writeln!(out, "                        self.do_reduce(reduce_rule, actions);").unwrap();
    writeln!(out, "                    }}").unwrap();
    writeln!(out, "                }}").unwrap();
    writeln!(out, "                _ => return Err({} {{ state }}),", error_struct).unwrap();
    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // state method
    writeln!(out, "    /// Get the current parser state.").unwrap();
    writeln!(out, "    pub fn state(&self) -> usize {{").unwrap();
    writeln!(out, "        self.current_state()").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // Private methods
    writeln!(out, "    fn current_state(&self) -> usize {{").unwrap();
    writeln!(out, "        self.state_stack.last().unwrap().0").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "    fn current_state_and_prec(&self) -> (usize, Option<(u8, u8)>) {{").unwrap();
    writeln!(out, "        let (state, _) = *self.state_stack.last().unwrap();").unwrap();
    writeln!(out, "        // Find the most recent operator's precedence (for E OP E reductions)").unwrap();
    writeln!(out, "        // Search backwards through the stack for a state with precedence").unwrap();
    writeln!(out, "        let prec = self.state_stack.iter().rev()").unwrap();
    writeln!(out, "            .find_map(|(_, p)| *p);").unwrap();
    writeln!(out, "        (state, prec)").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "    fn lookup_action(&self, state: usize, terminal: u32) -> u32 {{").unwrap();
    writeln!(out, "        let base = {}::ACTION_BASE[state];", table_mod).unwrap();
    writeln!(out, "        let index = base.wrapping_add(terminal as i32) as usize;").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        if index < {}::ACTION_CHECK.len() && {}::ACTION_CHECK[index] == state as u32 {{", table_mod, table_mod).unwrap();
    writeln!(out, "            {}::ACTION_DATA[index]", table_mod).unwrap();
    writeln!(out, "        }} else {{").unwrap();
    writeln!(out, "            0").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    writeln!(out, "    fn lookup_goto(&self, state: usize, non_terminal: u32) -> Option<usize> {{").unwrap();
    writeln!(out, "        let base = {}::GOTO_BASE[state];", table_mod).unwrap();
    writeln!(out, "        let index = base.wrapping_add(non_terminal as i32) as usize;").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        if index < {}::GOTO_CHECK.len() && {}::GOTO_CHECK[index] == state as u32 {{", table_mod, table_mod).unwrap();
    writeln!(out, "            Some({}::GOTO_DATA[index] as usize)", table_mod).unwrap();
    writeln!(out, "        }} else {{").unwrap();
    writeln!(out, "            None").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // do_shift
    writeln!(out, "    fn do_shift(&mut self, terminal: &{}, next_state: usize, prec: Option<(u8, u8)>) {{", terminal_enum).unwrap();
    writeln!(out, "        // Store state with precedence (level and associativity)").unwrap();
    writeln!(out, "        self.state_stack.push((next_state, prec));").unwrap();
    writeln!(out, "{}", generate_terminal_shift_arms(ctx, &terminal_enum, &value_union)).unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out).unwrap();

    // do_reduce
    writeln!(out, "    fn do_reduce(&mut self, rule: usize, actions: &mut A) {{").unwrap();
    writeln!(out, "        if rule == 0 {{ return; }}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        let (lhs_id, rhs_len) = {}::RULES[rule];", table_mod).unwrap();
    writeln!(out, "        let rhs_len = rhs_len as usize;").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        for _ in 0..rhs_len {{").unwrap();
    writeln!(out, "            self.state_stack.pop();").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        let original_rule_idx = rule - 1;").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        let value = match original_rule_idx {{").unwrap();
    writeln!(out, "{}", generate_reduction_arms(ctx, &reductions, &value_union, &table_mod)).unwrap();
    writeln!(out, "            _ => return,").unwrap();
    writeln!(out, "        }};").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        self.value_stack.push(std::mem::ManuallyDrop::new(value));").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "        let goto_state = self.current_state();").unwrap();
    writeln!(out, "        let nt_index = lhs_id - {}::NUM_TERMINALS - 1;", table_mod).unwrap();
    writeln!(out, "        if let Some(next_state) = self.lookup_goto(goto_state, nt_index) {{").unwrap();
    writeln!(out, "            self.state_stack.push((next_state, None));").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();

    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    // Default impl
    writeln!(out, "impl<A: {}> Default for {}<A> {{", actions_trait, parser_struct).unwrap();
    writeln!(out, "    fn default() -> Self {{ Self::new() }}").unwrap();
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    // Drop impl
    writeln!(out, "impl<A: {}> Drop for {}<A> {{", actions_trait, parser_struct).unwrap();
    writeln!(out, "    fn drop(&mut self) {{").unwrap();
    writeln!(out, "        while let Some(value) = self.value_stack.pop() {{").unwrap();
    writeln!(out, "            let (state, _) = self.state_stack.pop().unwrap();").unwrap();
    writeln!(out, "            let sym_id = {}::STATE_SYMBOL[state];", table_mod).unwrap();
    writeln!(out, "            unsafe {{").unwrap();
    writeln!(out, "                let union_val = std::mem::ManuallyDrop::into_inner(value);").unwrap();
    writeln!(out, "                match sym_id {{").unwrap();
    writeln!(out, "{}", generate_drop_arms(ctx, table_data)).unwrap();
    writeln!(out, "                    _ => {{}}").unwrap();
    writeln!(out, "                }}").unwrap();
    writeln!(out, "            }}").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();
    writeln!(out, "}}").unwrap();

    out
}

fn generate_actions_trait(
    _ctx: &CodegenContext,
    trait_name: &str,
    typed_non_terminals: &[(String, String)],
    methods: &[reduction::TraitMethod],
    vis: &str,
) -> String {
    let mut out = String::new();

    writeln!(out, "/// Actions trait for parser callbacks.").unwrap();
    writeln!(out, "{}trait {} {{", vis, trait_name).unwrap();

    // Associated types for each typed non-terminal
    for (nt_name, _) in typed_non_terminals {
        let type_name = CodegenContext::to_pascal_case(nt_name);
        writeln!(out, "    type {};", type_name).unwrap();
    }

    if !typed_non_terminals.is_empty() && !methods.is_empty() {
        writeln!(out).unwrap();
    }

    // Methods for named reductions
    for method in methods {
        let return_type = CodegenContext::to_pascal_case(&method.non_terminal);

        // Build parameter list (only typed symbols)
        // Use associated types for non-terminals, concrete types for terminals
        let params: Vec<String> = method.params.iter().enumerate().map(|(param_idx, (sym_idx, ty))| {
            let sym = &method.rhs_symbols[*sym_idx];

            // For non-terminals, use associated type; for terminals, use concrete type
            let param_type = if sym.kind == SymbolKind::NonTerminal {
                // Use the non-terminal name for the associated type
                format!("Self::{}", CodegenContext::to_pascal_case(&sym.name))
            } else {
                ty.clone()
            };
            format!("v{}: {}", param_idx, param_type)
        }).collect();

        writeln!(out, "    fn {}(&mut self, {}) -> Self::{};",
            method.name,
            params.join(", "),
            return_type
        ).unwrap();
    }

    writeln!(out, "}}").unwrap();

    out
}

fn generate_value_union(
    ctx: &CodegenContext,
    typed_non_terminals: &[(String, String)],
    value_union: &str,
    actions_trait: &str,
    vis: &str,
) -> String {
    let mut out = String::new();

    writeln!(out, "#[doc(hidden)]").unwrap();
    writeln!(out, "{} union {}<A: {}> {{", vis, value_union, actions_trait).unwrap();

    // Terminals with payloads
    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(ty) = payload_type {
            if let Some(name) = ctx.symbol_names.get(&id) {
                let field_name = format!("__{}", name.to_lowercase());
                if is_copy_type(ty) {
                    writeln!(out, "    {}: {},", field_name, ty).unwrap();
                } else {
                    writeln!(out, "    {}: std::mem::ManuallyDrop<{}>,", field_name, ty).unwrap();
                }
            }
        }
    }

    // Prec terminals
    for (&id, ty) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let field_name = format!("__{}", name.to_lowercase());
            if is_copy_type(ty) {
                writeln!(out, "    {}: {},", field_name, ty).unwrap();
            } else {
                writeln!(out, "    {}: std::mem::ManuallyDrop<{}>,", field_name, ty).unwrap();
            }
        }
    }

    // Typed non-terminals (using associated types)
    for (name, _) in typed_non_terminals {
        let field_name = format!("__{}", name.to_lowercase());
        let assoc_type = format!("A::{}", CodegenContext::to_pascal_case(name));
        writeln!(out, "    {}: std::mem::ManuallyDrop<{}>,", field_name, assoc_type).unwrap();
    }

    writeln!(out, "    __unit: (),").unwrap();
    writeln!(out, "}}").unwrap();

    out
}

fn generate_terminal_shift_arms(ctx: &CodegenContext, terminal_enum: &str, value_union: &str) -> String {
    let mut out = String::new();

    writeln!(out, "        match terminal {{").unwrap();

    // Regular terminals
    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = CodegenContext::to_pascal_case(name);
            let field_name = format!("__{}", name.to_lowercase());

            if let Some(ty) = payload_type {
                let inner = if is_copy_type(ty) {
                    format!("{} {{ {}: *v }}", value_union, field_name)
                } else {
                    format!("{} {{ {}: std::mem::ManuallyDrop::new(v.clone()) }}", value_union, field_name)
                };
                writeln!(out, "            {}::{}(v) => {{", terminal_enum, variant_name).unwrap();
                writeln!(out, "                self.value_stack.push(std::mem::ManuallyDrop::new({}));", inner).unwrap();
                writeln!(out, "            }}").unwrap();
            } else {
                writeln!(out, "            {}::{} => {{", terminal_enum, variant_name).unwrap();
                writeln!(out, "                self.value_stack.push(std::mem::ManuallyDrop::new({} {{ __unit: () }}));", value_union).unwrap();
                writeln!(out, "            }}").unwrap();
            }
        }
    }

    // Prec terminals
    for (&id, ty) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            let variant_name = CodegenContext::to_pascal_case(name);
            let field_name = format!("__{}", name.to_lowercase());

            let inner = if is_copy_type(ty) {
                format!("{} {{ {}: *v }}", value_union, field_name)
            } else {
                format!("{} {{ {}: std::mem::ManuallyDrop::new(v.clone()) }}", value_union, field_name)
            };
            writeln!(out, "            {}::{}(v, _prec) => {{", terminal_enum, variant_name).unwrap();
            writeln!(out, "                self.value_stack.push(std::mem::ManuallyDrop::new({}));", inner).unwrap();
            writeln!(out, "            }}").unwrap();
        }
    }

    writeln!(out, "        }}").unwrap();

    out
}

fn generate_reduction_arms(
    _ctx: &CodegenContext,
    reductions: &[ReductionInfo],
    value_union: &str,
    _table_mod: &str,
) -> String {
    let mut out = String::new();

    for (idx, info) in reductions.iter().enumerate() {
        let lhs_field = format!("__{}", info.non_terminal.to_lowercase());

        writeln!(out, "            {} => {{", idx).unwrap();

        // Pop values in reverse order, collecting typed ones
        let mut var_decls = Vec::new();
        for (i, sym) in info.rhs_symbols.iter().enumerate().rev() {
            let pop = "std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap())";

            if let Some(ty) = &sym.ty {
                let field_name = match sym.kind {
                    SymbolKind::UnitTerminal => {
                        // Unit terminal - just pop
                        writeln!(out, "                {{ {} }};", pop).unwrap();
                        continue;
                    }
                    SymbolKind::PayloadTerminal | SymbolKind::PrecTerminal => {
                        format!("__{}", sym.name.to_lowercase())
                    }
                    SymbolKind::NonTerminal => {
                        format!("__{}", sym.name.to_lowercase())
                    }
                };

                // For terminals, we know the concrete type and can check if it's Copy.
                // For non-terminals, the union field is always ManuallyDrop<A::Type>
                // since we use associated types and can't know if they're Copy.
                let extract = match sym.kind {
                    SymbolKind::PayloadTerminal | SymbolKind::PrecTerminal => {
                        if is_copy_type(ty) {
                            format!("{}.{}", pop, field_name)
                        } else {
                            format!("std::mem::ManuallyDrop::into_inner({}.{})", pop, field_name)
                        }
                    }
                    SymbolKind::NonTerminal => {
                        // Always unwrap ManuallyDrop for non-terminals
                        format!("std::mem::ManuallyDrop::into_inner({}.{})", pop, field_name)
                    }
                    SymbolKind::UnitTerminal => unreachable!(),
                };

                writeln!(out, "                let v{} = unsafe {{ {} }};", i, extract).unwrap();
                var_decls.push((i, ty.clone()));
            } else {
                // Untyped - just pop
                writeln!(out, "                {{ {} }};", pop).unwrap();
            }
        }

        // Reverse to get correct order
        var_decls.reverse();

        // Generate result based on reduction kind
        // Non-terminal fields in the union are always ManuallyDrop<A::Type> since we
        // can't know at codegen time if the associated type is Copy
        match &info.kind {
            ReductionKind::Named { method_name, params } => {
                // Call trait method and wrap result
                let args: Vec<String> = params.iter()
                    .map(|(sym_idx, _)| format!("v{}", sym_idx))
                    .collect();

                writeln!(out, "                {} {{ {}: std::mem::ManuallyDrop::new(actions.{}({})) }}",
                    value_union, lhs_field, method_name, args.join(", ")).unwrap();
            }
            ReductionKind::Passthrough { symbol_index } => {
                // Pass through the single typed value, wrap in ManuallyDrop
                writeln!(out, "                {} {{ {}: std::mem::ManuallyDrop::new(v{}) }}",
                    value_union, lhs_field, symbol_index).unwrap();
            }
            ReductionKind::Structural => {
                // Structural - just use unit
                writeln!(out, "                {} {{ __unit: () }}", value_union).unwrap();
            }
        }

        writeln!(out, "            }}").unwrap();
    }

    out
}

fn generate_drop_arms(ctx: &CodegenContext, table_data: &TableData) -> String {
    let mut out = String::new();

    // Terminals with non-Copy payloads
    for (&id, payload_type) in &ctx.terminal_types {
        if let Some(ty) = payload_type {
            if !is_copy_type(ty) {
                if let Some(name) = ctx.symbol_names.get(&id) {
                    let field_name = format!("__{}", name.to_lowercase());
                    if let Some((_, table_id)) = table_data.terminal_ids.iter().find(|(n, _)| n == name) {
                        writeln!(out, "                    {} => {{ std::mem::ManuallyDrop::into_inner(union_val.{}); }}", table_id, field_name).unwrap();
                    }
                }
            }
        }
    }

    // Prec terminals with non-Copy payloads
    for (&id, ty) in &ctx.prec_terminal_types {
        if !is_copy_type(ty) {
            if let Some(name) = ctx.symbol_names.get(&id) {
                let field_name = format!("__{}", name.to_lowercase());
                if let Some((_, table_id)) = table_data.terminal_ids.iter().find(|(n, _)| n == name) {
                    writeln!(out, "                    {} => {{ std::mem::ManuallyDrop::into_inner(union_val.{}); }}", table_id, field_name).unwrap();
                }
            }
        }
    }

    // Non-terminals (always use ManuallyDrop for associated types since we don't know if they're Copy)
    for name in &ctx.rule_names {
        if ctx.rules.iter().any(|r| r.name == *name && r.result_type.is_some()) {
            let field_name = format!("__{}", name.to_lowercase());
            if let Some((_, table_id)) = table_data.non_terminal_ids.iter().find(|(n, _)| n == name) {
                writeln!(out, "                    {} => {{ std::mem::ManuallyDrop::into_inner(union_val.{}); }}", table_id, field_name).unwrap();
            }
        }
    }

    out
}

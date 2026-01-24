//! Code generation from parse tables.
//!
//! This module extracts data from compiled parse tables for code generation.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::grammar::SymbolId;
use crate::lr::Automaton;
use crate::table::{Action, ParseTable};

use super::CodegenContext;

/// Data extracted from a compiled parse table for code generation.
pub struct TableData {
    /// ACTION table data array.
    pub action_data: Vec<u32>,
    /// ACTION table base offsets.
    pub action_base: Vec<i32>,
    /// ACTION table check array.
    pub action_check: Vec<u32>,
    /// GOTO table data array.
    pub goto_data: Vec<u32>,
    /// GOTO table base offsets.
    pub goto_base: Vec<i32>,
    /// GOTO table check array.
    pub goto_check: Vec<u32>,
    /// Rule info: (lhs_symbol_id, rhs_length) for each rule.
    pub rules: Vec<(u32, u8)>,
    /// Number of parser states.
    pub num_states: usize,
    /// Number of terminals (including EOF).
    pub num_terminals: u32,
    /// Number of non-terminals.
    pub num_non_terminals: u32,
    /// Map from terminal names to symbol IDs.
    pub terminal_ids: Vec<(String, u32)>,
    /// Map from non-terminal names to symbol IDs.
    pub non_terminal_ids: Vec<(String, u32)>,
    /// Accessing symbol for each state.
    pub state_symbols: Vec<u32>,
}

/// Extract table data from the CodegenContext for code generation.
pub fn extract_table_data(ctx: &CodegenContext) -> Result<TableData, String> {
    let automaton = Automaton::build(&ctx.grammar);
    let table = ParseTable::build(&automaton);

    // Report conflicts (but allow them - resolved by rule order like bison)
    let reduce_reduce: Vec<_> = table.conflicts.iter()
        .filter(|c| matches!(c, crate::table::Conflict::ReduceReduce { .. }))
        .collect();
    let shift_reduce_count = table.conflicts.len() - reduce_reduce.len();

    if !reduce_reduce.is_empty() {
        eprintln!("Warning: {} reduce/reduce conflicts (resolved by rule order)", reduce_reduce.len());
    }
    if shift_reduce_count > 0 {
        eprintln!("Warning: {} shift/reduce conflicts (resolved by precedence at runtime)", shift_reduce_count);
    }

    let num_terminals = table.grammar.symbols.num_terminals();
    let num_non_terminals = table.grammar.symbols.num_non_terminals();

    // Build terminal ID map
    let mut terminal_ids = Vec::new();
    for &id in ctx.terminal_types.keys() {
        if let Some(name) = ctx.symbol_names.get(&id)
            && let Some(table_id) = table.grammar.symbols.get_id(name)
        {
            terminal_ids.push((name.clone(), table_id.0));
        }
    }
    for &id in ctx.prec_terminal_types.keys() {
        if let Some(name) = ctx.symbol_names.get(&id)
            && let Some(table_id) = table.grammar.symbols.get_id(name)
        {
            terminal_ids.push((name.clone(), table_id.0));
        }
    }

    // Build non-terminal ID map
    let mut non_terminal_ids = Vec::new();
    for name in &ctx.rule_names {
        if let Some(id) = table.grammar.symbols.get_id(name) {
            non_terminal_ids.push((name.clone(), id.0));
        }
    }

    // Extract rule info
    let rules: Vec<_> = table.grammar.rules.iter()
        .map(|r| (r.lhs.id().0, r.rhs.len() as u8))
        .collect();

    // Compute accessing symbols
    let state_symbols = compute_state_symbols(&table, num_terminals, num_non_terminals);

    Ok(TableData {
        action_data: table.action_data().to_vec(),
        action_base: table.action_base().to_vec(),
        action_check: table.action_check().to_vec(),
        goto_data: table.goto_data().to_vec(),
        goto_base: table.goto_base().to_vec(),
        goto_check: table.goto_check().to_vec(),
        rules,
        num_states: table.num_states,
        num_terminals,
        num_non_terminals,
        terminal_ids,
        non_terminal_ids,
        state_symbols,
    })
}

fn compute_state_symbols(table: &ParseTable, num_terminals: u32, num_non_terminals: u32) -> Vec<u32> {
    let num_states = table.num_states;
    let mut state_symbols = vec![0u32; num_states];

    for state in 0..num_states {
        for t in 0..=num_terminals {
            match table.action(state, SymbolId(t)) {
                Action::Shift(target) => state_symbols[target] = t,
                Action::ShiftOrReduce { shift_state, .. } => state_symbols[shift_state] = t,
                _ => {}
            }
        }
    }

    for state in 0..num_states {
        for nt in 0..num_non_terminals {
            let nt_id = SymbolId(num_terminals + 1 + nt);
            if let Some(target) = table.goto(state, nt_id) {
                state_symbols[target] = nt_id.0;
            }
        }
    }

    state_symbols
}

/// Generate static table data as Rust code.
pub fn generate_table_statics(ctx: &CodegenContext, table_data: &TableData) -> TokenStream {
    let mod_name = format_ident!("__{}_table", ctx.name.to_lowercase());
    let core_path = ctx.core_path_tokens();

    let action_data = &table_data.action_data;
    let action_base = &table_data.action_base;
    let action_check = &table_data.action_check;
    let goto_data = &table_data.goto_data;
    let goto_base = &table_data.goto_base;
    let goto_check = &table_data.goto_check;

    let rules: Vec<_> = table_data.rules.iter()
        .map(|(lhs, len)| quote! { (#lhs, #len) })
        .collect();

    let state_symbols = &table_data.state_symbols;
    let num_states = table_data.num_states;
    let num_terminals = table_data.num_terminals;
    let num_non_terminals = table_data.num_non_terminals;

    // Build symbol_id match arms
    let symbol_id_arms: Vec<_> = table_data.terminal_ids.iter()
        .chain(table_data.non_terminal_ids.iter())
        .map(|(name, id)| quote! { #name => #core_path::SymbolId(#id), })
        .collect();

    // Only include use statement for relative paths
    let use_stmt = if !ctx.use_absolute_path {
        quote! { use super::gazelle; }
    } else {
        quote! {}
    };

    quote! {
        #[doc(hidden)]
        mod #mod_name {
            #use_stmt

            pub static ACTION_DATA: &[u32] = &[#(#action_data),*];
            pub static ACTION_BASE: &[i32] = &[#(#action_base),*];
            pub static ACTION_CHECK: &[u32] = &[#(#action_check),*];
            pub static GOTO_DATA: &[u32] = &[#(#goto_data),*];
            pub static GOTO_BASE: &[i32] = &[#(#goto_base),*];
            pub static GOTO_CHECK: &[u32] = &[#(#goto_check),*];
            pub static RULES: &[(u32, u8)] = &[#(#rules),*];
            pub static STATE_SYMBOL: &[u32] = &[#(#state_symbols),*];
            pub const NUM_STATES: usize = #num_states;
            pub const NUM_TERMINALS: u32 = #num_terminals;
            #[allow(dead_code)]
            pub const NUM_NON_TERMINALS: u32 = #num_non_terminals;

            pub fn symbol_id(name: &str) -> #core_path::SymbolId {
                match name {
                    #(#symbol_id_arms)*
                    _ => panic!("unknown symbol: {}", name),
                }
            }
        }
    }
}

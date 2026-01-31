//! Code generation from parse tables.
//!
//! This module builds a [`CompiledTable`] and generates static Rust code from it.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::grammar::SymbolId;
use crate::table::CompiledTable;

use super::CodegenContext;

/// Extra codegen-specific data derived from a [`CompiledTable`] and [`CodegenContext`].
pub struct CodegenTableInfo {
    /// Map from terminal names to symbol IDs.
    pub terminal_ids: Vec<(String, u32)>,
    /// Map from non-terminal names to symbol IDs.
    pub non_terminal_ids: Vec<(String, u32)>,
}

/// Build parse tables and extract codegen info from a [`CodegenContext`].
pub fn build_table(ctx: &CodegenContext) -> Result<(CompiledTable, CodegenTableInfo), String> {
    let compiled = CompiledTable::build(&ctx.grammar);

    // Report conflicts (but allow them - resolved by rule order like bison)
    let reduce_reduce: Vec<_> = compiled.conflicts.iter()
        .filter(|c| matches!(c, crate::table::Conflict::ReduceReduce { .. }))
        .collect();
    let shift_reduce_count = compiled.conflicts.len() - reduce_reduce.len();

    if !reduce_reduce.is_empty() {
        eprintln!("Warning: {} reduce/reduce conflicts (resolved by rule order)", reduce_reduce.len());
    }
    if shift_reduce_count > 0 {
        eprintln!("Warning: {} shift/reduce conflicts (resolved by precedence at runtime)", shift_reduce_count);
    }

    let grammar = &compiled.grammar;

    // Build terminal ID map
    let mut terminal_ids = Vec::new();
    for &id in ctx.terminal_types.keys() {
        let name = ctx.grammar.symbols.name(id);
        terminal_ids.push((name.to_string(), id.0));
    }
    for &id in ctx.prec_terminal_types.keys() {
        let name = ctx.grammar.symbols.name(id);
        terminal_ids.push((name.to_string(), id.0));
    }

    // Build non-terminal ID map
    let mut non_terminal_ids = Vec::new();
    for rule in &ctx.rules {
        if let Some(id) = grammar.symbols.get_id(&rule.name) {
            non_terminal_ids.push((rule.name.clone(), id.0));
        }
    }

    let info = CodegenTableInfo {
        terminal_ids,
        non_terminal_ids,
    };

    Ok((compiled, info))
}

/// Generate static table data as Rust code.
pub fn generate_table_statics(ctx: &CodegenContext, compiled: &CompiledTable, info: &CodegenTableInfo) -> TokenStream {
    let mod_name = format_ident!("__{}_table", ctx.name.to_lowercase());
    let core_path = ctx.core_path_tokens();

    let action_data = compiled.action_data();
    let action_base = compiled.action_base();
    let action_check = compiled.action_check();
    let goto_data = compiled.goto_data();
    let goto_base = compiled.goto_base();
    let goto_check = compiled.goto_check();

    let rules: Vec<_> = compiled.rules().iter()
        .map(|(lhs, len)| quote! { (#lhs, #len) })
        .collect();

    let state_symbols = compiled.state_symbols();
    let num_states = compiled.num_states;
    let num_terminals = compiled.grammar.symbols.num_terminals();
    let num_non_terminals = compiled.grammar.symbols.num_non_terminals();

    // Build symbol_id match arms
    let symbol_id_arms: Vec<_> = info.terminal_ids.iter()
        .chain(info.non_terminal_ids.iter())
        .map(|(name, id)| quote! { #name => #core_path::SymbolId(#id), })
        .collect();

    // Only include use statement for relative paths
    let use_stmt = if !ctx.use_absolute_path {
        quote! { use super::gazelle; }
    } else {
        quote! {}
    };

    // Generate error info tables
    let grammar = &compiled.grammar;
    let num_symbols = grammar.symbols.num_symbols();

    // Symbol names indexed by SymbolId
    let symbol_names: Vec<_> = (0..num_symbols)
        .map(|i| grammar.symbols.name(SymbolId(i)))
        .collect();

    // Expected terminals per state
    let expected_terminals = compiled.expected_terminals();
    let expected_statics: Vec<_> = expected_terminals
        .iter()
        .enumerate()
        .map(|(i, terminals)| {
            let name = format_ident!("EXPECTED_{}", i);
            quote! { static #name: &[u32] = &[#(#terminals),*]; }
        })
        .collect();
    let expected_refs: Vec<_> = (0..num_states)
        .map(|i| {
            let name = format_ident!("EXPECTED_{}", i);
            quote! { #name }
        })
        .collect();

    // State items per state
    let state_items = compiled.state_items();
    let state_items_statics: Vec<_> = state_items
        .iter()
        .enumerate()
        .map(|(i, items)| {
            let name = format_ident!("STATE_ITEMS_{}", i);
            let items: Vec<_> = items.iter().map(|(r, d)| quote! { (#r, #d) }).collect();
            quote! { static #name: &[(u16, u8)] = &[#(#items),*]; }
        })
        .collect();
    let state_items_refs: Vec<_> = (0..num_states)
        .map(|i| {
            let name = format_ident!("STATE_ITEMS_{}", i);
            quote! { #name }
        })
        .collect();

    // Rule RHS symbol IDs
    let rule_rhs = compiled.rule_rhs();
    let rule_rhs_statics: Vec<_> = rule_rhs
        .iter()
        .enumerate()
        .map(|(i, rhs)| {
            let name = format_ident!("RULE_RHS_{}", i);
            quote! { static #name: &[u32] = &[#(#rhs),*]; }
        })
        .collect();
    let rule_rhs_refs: Vec<_> = (0..rule_rhs.len())
        .map(|i| {
            let name = format_ident!("RULE_RHS_{}", i);
            quote! { #name }
        })
        .collect();

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

            // Error info tables
            pub static SYMBOL_NAMES: &[&str] = &[#(#symbol_names),*];
            #(#expected_statics)*
            pub static EXPECTED: &[&[u32]] = &[#(#expected_refs),*];
            #(#state_items_statics)*
            pub static STATE_ITEMS: &[&[(u16, u8)]] = &[#(#state_items_refs),*];
            #(#rule_rhs_statics)*
            pub static RULE_RHS: &[&[u32]] = &[#(#rule_rhs_refs),*];

            pub fn symbol_id(name: &str) -> #core_path::SymbolId {
                match name {
                    #(#symbol_id_arms)*
                    _ => panic!("unknown symbol: {}", name),
                }
            }

            pub static TABLE: #core_path::ParseTable<'static> = #core_path::ParseTable::new(
                ACTION_DATA, ACTION_BASE, ACTION_CHECK,
                GOTO_DATA, GOTO_BASE, GOTO_CHECK,
                RULES, NUM_TERMINALS,
            );

            pub static ERROR_INFO: #core_path::ErrorInfo<'static> = #core_path::ErrorInfo {
                symbol_names: SYMBOL_NAMES,
                expected: EXPECTED,
                state_items: STATE_ITEMS,
                rule_rhs: RULE_RHS,
                state_symbols: STATE_SYMBOL,
                rules: RULES,
            };
        }
    }
}

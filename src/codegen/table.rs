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
    let compiled = CompiledTable::build_from_internal(&ctx.grammar);

    // Count conflicts by type
    let rr_count = compiled.conflicts.iter()
        .filter(|c| matches!(c, crate::table::Conflict::ReduceReduce { .. }))
        .count();
    let sr_count = compiled.conflicts.iter()
        .filter(|c| matches!(c, crate::table::Conflict::ShiftReduce { .. }))
        .count();

    // Check if conflict counts match expected
    let rr_mismatch = rr_count != ctx.expect_rr;
    let sr_mismatch = sr_count != ctx.expect_sr;

    if rr_mismatch || sr_mismatch {
        let messages = compiled.format_conflicts();
        let mut errors = Vec::new();

        if rr_mismatch {
            let rr_messages: Vec<_> = messages.iter()
                .filter(|m| m.starts_with("Reduce/reduce"))
                .cloned()
                .collect();
            if ctx.expect_rr == 0 {
                errors.push(format!(
                    "Grammar has {} reduce/reduce conflict(s) (expected 0):\n\n{}",
                    rr_count,
                    rr_messages.join("\n\n")
                ));
            } else {
                errors.push(format!(
                    "Grammar has {} reduce/reduce conflict(s) (expected {}):\n\n{}",
                    rr_count, ctx.expect_rr,
                    rr_messages.join("\n\n")
                ));
            }
        }

        if sr_mismatch {
            let sr_messages: Vec<_> = messages.iter()
                .filter(|m| m.starts_with("Shift/reduce"))
                .cloned()
                .collect();
            if ctx.expect_sr == 0 {
                errors.push(format!(
                    "Grammar has {} shift/reduce conflict(s) (expected 0):\n\n{}\n\n\
                     Hint: Use 'prec' terminals for operators to resolve by precedence at runtime.",
                    sr_count,
                    sr_messages.join("\n\n")
                ));
            } else {
                errors.push(format!(
                    "Grammar has {} shift/reduce conflict(s) (expected {}):\n\n{}",
                    sr_count, ctx.expect_sr,
                    sr_messages.join("\n\n")
                ));
            }
        }

        return Err(errors.join("\n\n"));
    }

    // Build terminal ID map (skip EOF at index 0)
    let mut terminal_ids = Vec::new();
    for id in ctx.grammar.symbols.terminal_ids().skip(1) {
        let name = ctx.grammar.symbols.name(id);
        terminal_ids.push((name.to_string(), id.0));
    }

    // Build non-terminal ID map
    let mut non_terminal_ids = Vec::new();
    for id in ctx.grammar.symbols.non_terminal_ids() {
        let name = ctx.grammar.symbols.name(id);
        non_terminal_ids.push((name.to_string(), id.0));
    }

    let info = CodegenTableInfo {
        terminal_ids,
        non_terminal_ids,
    };

    Ok((compiled, info))
}

/// Generate static table data as Rust code.
pub fn generate_table_statics(ctx: &CodegenContext, compiled: &CompiledTable, info: &CodegenTableInfo) -> TokenStream {
    let mod_name = format_ident!("__table");
    let gazelle_crate_path = ctx.gazelle_crate_path_tokens();

    let table_data = compiled.table_data();
    let table_check = compiled.table_check();
    let action_base = compiled.action_base();
    let goto_base = compiled.goto_base();

    let rules: Vec<_> = compiled.rules().iter()
        .map(|(lhs, len)| quote! { (#lhs, #len) })
        .collect();

    let state_symbols = compiled.state_symbols();
    let default_reduce = compiled.default_reduce();
    let default_goto = compiled.default_goto();
    let num_states = compiled.num_states();
    let num_terminals = compiled.grammar.symbols.num_terminals();
    let num_non_terminals = compiled.grammar.symbols.num_non_terminals();

    // Build symbol_id match arms
    let symbol_id_arms: Vec<_> = info.terminal_ids.iter()
        .chain(info.non_terminal_ids.iter())
        .map(|(name, id)| quote! { #name => #gazelle_crate_path::SymbolId::new(#id), })
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

            pub static DATA: &[u32] = &[#(#table_data),*];
            pub static CHECK: &[u32] = &[#(#table_check),*];
            pub static ACTION_BASE: &[i32] = &[#(#action_base),*];
            pub static GOTO_BASE: &[i32] = &[#(#goto_base),*];
            pub static RULES: &[(u32, u8)] = &[#(#rules),*];
            pub static STATE_SYMBOL: &[u32] = &[#(#state_symbols),*];
            pub static DEFAULT_REDUCE: &[u32] = &[#(#default_reduce),*];
            pub static DEFAULT_GOTO: &[u32] = &[#(#default_goto),*];
            pub const NUM_STATES: usize = #num_states;
            pub const NUM_TERMINALS: u32 = #num_terminals;
            #[allow(dead_code)]
            pub const NUM_NON_TERMINALS: u32 = #num_non_terminals;

            // Error info tables
            pub static SYMBOL_NAMES: &[&str] = &[#(#symbol_names),*];
            #(#state_items_statics)*
            pub static STATE_ITEMS: &[&[(u16, u8)]] = &[#(#state_items_refs),*];
            #(#rule_rhs_statics)*
            pub static RULE_RHS: &[&[u32]] = &[#(#rule_rhs_refs),*];

            pub fn symbol_id(name: &str) -> #gazelle_crate_path::SymbolId {
                match name {
                    #(#symbol_id_arms)*
                    _ => panic!("unknown symbol: {}", name),
                }
            }

            pub static TABLE: #gazelle_crate_path::ParseTable<'static> = #gazelle_crate_path::ParseTable::new(
                DATA, CHECK, ACTION_BASE, GOTO_BASE,
                RULES, NUM_TERMINALS, DEFAULT_REDUCE, DEFAULT_GOTO,
            );

            pub static ERROR_INFO: #gazelle_crate_path::ErrorInfo<'static> = #gazelle_crate_path::ErrorInfo {
                symbol_names: SYMBOL_NAMES,
                state_items: STATE_ITEMS,
                rule_rhs: RULE_RHS,
                state_symbols: STATE_SYMBOL,
            };
        }
    }
}

//! Code generation from parse tables.
//!
//! This module extracts data from compiled parse tables for code generation.
//! The actual table building and compression is done by `crate::table::ParseTable`.

use std::fmt::Write;

use crate::grammar::SymbolId;
use crate::lr::Automaton;
use crate::table::{Action, ParseTable};

use super::CodegenContext;

/// Data extracted from a compiled parse table for code generation.
pub struct TableData {
    /// ACTION table data array (from ParseTable).
    pub action_data: Vec<u32>,
    /// ACTION table base offsets (from ParseTable).
    pub action_base: Vec<i32>,
    /// ACTION table check array (from ParseTable).
    pub action_check: Vec<u32>,
    /// GOTO table data array (from ParseTable).
    pub goto_data: Vec<u32>,
    /// GOTO table base offsets (from ParseTable).
    pub goto_base: Vec<i32>,
    /// GOTO table check array (from ParseTable).
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

    // Check for conflicts
    if table.has_conflicts() {
        let mut error_msg = String::from("grammar has conflicts:\n");
        for conflict in &table.conflicts {
            error_msg.push_str(&format!("  {:?}\n", conflict));
        }
        return Err(error_msg);
    }

    let num_terminals = table.grammar.symbols.num_terminals();
    let num_non_terminals = table.grammar.symbols.num_non_terminals();

    // Build terminal ID map
    let mut terminal_ids = Vec::new();
    for (&id, _) in &ctx.terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            if let Some(table_id) = table.grammar.symbols.get_id(name) {
                terminal_ids.push((name.clone(), table_id.0));
            }
        }
    }
    for (&id, _) in &ctx.prec_terminal_types {
        if let Some(name) = ctx.symbol_names.get(&id) {
            if let Some(table_id) = table.grammar.symbols.get_id(name) {
                terminal_ids.push((name.clone(), table_id.0));
            }
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

    // Compute accessing symbols (which symbol was shifted/goto'd to reach each state)
    let state_symbols = compute_state_symbols(&table, num_terminals, num_non_terminals);

    Ok(TableData {
        // Copy already-compressed tables from ParseTable
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
pub fn generate_table_statics(ctx: &CodegenContext, table_data: &TableData) -> String {
    let mut out = String::new();
    let mod_name = format!("__{}_table", ctx.name.to_lowercase());
    let core = ctx.core_path();

    writeln!(out, "#[doc(hidden)]").unwrap();
    writeln!(out, "mod {} {{", mod_name).unwrap();
    // Only need the use statement for relative paths
    if !ctx.use_absolute_path {
        writeln!(out, "    use super::gazelle_core;").unwrap();
    }
    writeln!(out).unwrap();

    // ACTION table
    write!(out, "    pub static ACTION_DATA: &[u32] = &[").unwrap();
    for (i, v) in table_data.action_data.iter().enumerate() {
        if i > 0 { write!(out, ",").unwrap(); }
        write!(out, "{}", v).unwrap();
    }
    writeln!(out, "];").unwrap();

    write!(out, "    pub static ACTION_BASE: &[i32] = &[").unwrap();
    for (i, v) in table_data.action_base.iter().enumerate() {
        if i > 0 { write!(out, ",").unwrap(); }
        write!(out, "{}", v).unwrap();
    }
    writeln!(out, "];").unwrap();

    write!(out, "    pub static ACTION_CHECK: &[u32] = &[").unwrap();
    for (i, v) in table_data.action_check.iter().enumerate() {
        if i > 0 { write!(out, ",").unwrap(); }
        write!(out, "{}", v).unwrap();
    }
    writeln!(out, "];").unwrap();

    // GOTO table
    write!(out, "    pub static GOTO_DATA: &[u32] = &[").unwrap();
    for (i, v) in table_data.goto_data.iter().enumerate() {
        if i > 0 { write!(out, ",").unwrap(); }
        write!(out, "{}", v).unwrap();
    }
    writeln!(out, "];").unwrap();

    write!(out, "    pub static GOTO_BASE: &[i32] = &[").unwrap();
    for (i, v) in table_data.goto_base.iter().enumerate() {
        if i > 0 { write!(out, ",").unwrap(); }
        write!(out, "{}", v).unwrap();
    }
    writeln!(out, "];").unwrap();

    write!(out, "    pub static GOTO_CHECK: &[u32] = &[").unwrap();
    for (i, v) in table_data.goto_check.iter().enumerate() {
        if i > 0 { write!(out, ",").unwrap(); }
        write!(out, "{}", v).unwrap();
    }
    writeln!(out, "];").unwrap();

    // Rules
    write!(out, "    pub static RULES: &[(u32, u8)] = &[").unwrap();
    for (i, (lhs, len)) in table_data.rules.iter().enumerate() {
        if i > 0 { write!(out, ",").unwrap(); }
        write!(out, "({},{})", lhs, len).unwrap();
    }
    writeln!(out, "];").unwrap();

    // State symbols
    write!(out, "    pub static STATE_SYMBOL: &[u32] = &[").unwrap();
    for (i, v) in table_data.state_symbols.iter().enumerate() {
        if i > 0 { write!(out, ",").unwrap(); }
        write!(out, "{}", v).unwrap();
    }
    writeln!(out, "];").unwrap();

    // Constants
    writeln!(out, "    pub const NUM_STATES: usize = {};", table_data.num_states).unwrap();
    writeln!(out, "    pub const NUM_TERMINALS: u32 = {};", table_data.num_terminals).unwrap();
    writeln!(out, "    #[allow(dead_code)]").unwrap();
    writeln!(out, "    pub const NUM_NON_TERMINALS: u32 = {};", table_data.num_non_terminals).unwrap();

    // Symbol ID lookup
    writeln!(out).unwrap();
    writeln!(out, "    pub fn symbol_id(name: &str) -> {}::SymbolId {{", core).unwrap();
    writeln!(out, "        match name {{").unwrap();
    for (name, id) in &table_data.terminal_ids {
        writeln!(out, "            {:?} => {}::SymbolId({}),", name, core, id).unwrap();
    }
    for (name, id) in &table_data.non_terminal_ids {
        writeln!(out, "            {:?} => {}::SymbolId({}),", name, core, id).unwrap();
    }
    writeln!(out, "            _ => panic!(\"unknown symbol: {{}}\", name),").unwrap();
    writeln!(out, "        }}").unwrap();
    writeln!(out, "    }}").unwrap();

    writeln!(out, "}}").unwrap();

    out
}

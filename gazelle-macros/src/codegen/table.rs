//! Compile-time parse table generation.

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::Error;

use gazelle_core::{Automaton, GrammarBuilder, ParseTable, SymbolId};

use crate::ir::{GrammarIr, GrammarSymbol};

/// Data extracted from the compiled parse table.
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
    /// Map from our terminal names to symbol IDs.
    pub terminal_ids: Vec<(String, u32)>,
    /// Map from our non-terminal names to symbol IDs.
    pub non_terminal_ids: Vec<(String, u32)>,
    /// Rule mapping: (non_terminal_name, alternative_index, rhs_symbols).
    pub rule_mapping: Vec<RuleInfo>,
    /// Accessing symbol for each state (symbol that was shifted/goto'd to reach that state).
    /// state_symbols[0] is 0 (invalid - initial state has no accessing symbol).
    pub state_symbols: Vec<u32>,
}

/// Information about a grammar rule for reduction mapping.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RuleInfo {
    pub non_terminal_name: String,
    pub alternative_index: usize,
    pub rhs_symbols: Vec<GrammarSymbol>,
}

/// Build the parse table from the grammar IR at compile time.
pub fn build_table(grammar: &GrammarIr) -> Result<TableData, Error> {
    // Convert IR to Gazelle Grammar
    let mut builder = GrammarBuilder::new();

    // Add terminals
    for terminal in &grammar.terminals {
        builder.t(&terminal.name.to_string());
    }

    // Add prec_terminals
    for prec_terminal in &grammar.prec_terminals {
        builder.pt(&prec_terminal.name.to_string());
    }

    // Add non-terminals and rules
    let mut non_terminal_symbols = Vec::new();
    let mut rule_mapping = Vec::new();

    for rule_def in &grammar.rules {
        let nt_name = rule_def.name.to_string();
        let nt = builder.nt(&nt_name);
        non_terminal_symbols.push((nt_name.clone(), nt));
    }

    // Now add the rules (need to do this after all non-terminals are declared)
    for (_rule_idx, rule_def) in grammar.rules.iter().enumerate() {
        let nt_name = rule_def.name.to_string();
        let lhs = non_terminal_symbols
            .iter()
            .find(|(n, _)| n == &nt_name)
            .map(|(_, s)| *s)
            .unwrap();

        for (alt_idx, alt) in rule_def.alternatives.iter().enumerate() {
            let mut rhs = Vec::new();

            for sym in &alt.symbols {
                let sym_name = sym.name().to_string();
                let symbol = match sym {
                    GrammarSymbol::Terminal(_) => {
                        // Look up terminal
                        builder.symbols.get(&sym_name).ok_or_else(|| {
                            Error::new(Span::call_site(), format!("unknown terminal: {}", sym_name))
                        })?
                    }
                    GrammarSymbol::NonTerminal(_) => {
                        // Look up non-terminal
                        non_terminal_symbols
                            .iter()
                            .find(|(n, _)| n == &sym_name)
                            .map(|(_, s)| *s)
                            .ok_or_else(|| {
                                Error::new(
                                    Span::call_site(),
                                    format!("unknown non-terminal: {}", sym_name),
                                )
                            })?
                    }
                };
                rhs.push(symbol);
            }

            builder.rule(lhs, rhs);
            rule_mapping.push(RuleInfo {
                non_terminal_name: nt_name.clone(),
                alternative_index: alt_idx,
                rhs_symbols: alt.symbols.clone(),
            });
        }
    }

    // Set start symbol (first non-terminal)
    if let Some((_, start)) = non_terminal_symbols.first() {
        builder.start(*start);
    }

    let gazelle_grammar = builder.build();

    // Build automaton
    let automaton = Automaton::build(&gazelle_grammar);

    // Build parse table
    let table = ParseTable::build(&automaton);

    // Check for conflicts
    if table.has_conflicts() {
        let mut error_msg = String::from("grammar has conflicts:\n");
        for conflict in &table.conflicts {
            error_msg.push_str(&format!("  {:?}\n", conflict));
        }
        return Err(Error::new(Span::call_site(), error_msg));
    }

    // Extract table data using public accessors
    let num_terminals = table.grammar.symbols.num_terminals();
    let num_non_terminals = table.grammar.symbols.num_non_terminals();

    // Build terminal ID map
    let mut terminal_ids = Vec::new();
    for terminal in &grammar.terminals {
        let name = terminal.name.to_string();
        if let Some(id) = table.grammar.symbols.get_id(&name) {
            terminal_ids.push((name, id.0));
        }
    }
    for prec_terminal in &grammar.prec_terminals {
        let name = prec_terminal.name.to_string();
        if let Some(id) = table.grammar.symbols.get_id(&name) {
            terminal_ids.push((name, id.0));
        }
    }

    // Build non-terminal ID map
    let mut non_terminal_ids = Vec::new();
    for rule in &grammar.rules {
        let name = rule.name.to_string();
        if let Some(id) = table.grammar.symbols.get_id(&name) {
            non_terminal_ids.push((name, id.0));
        }
    }

    // Extract rule info
    let mut rules = Vec::new();
    for i in 0..table.grammar.rules.len() {
        let (lhs_id, len) = table.rule_info(i);
        rules.push((lhs_id.0, len as u8));
    }

    // Extract table arrays via serialization/reconstruction
    // We need to access the internal arrays, so we'll reconstruct action/goto lookups
    let table_data = extract_table_arrays(&table);

    // Compute accessing symbol for each state by inverting ACTION/GOTO tables
    let state_symbols = compute_state_symbols(&table, num_terminals, num_non_terminals);

    Ok(TableData {
        action_data: table_data.0,
        action_base: table_data.1,
        action_check: table_data.2,
        goto_data: table_data.3,
        goto_base: table_data.4,
        goto_check: table_data.5,
        rules,
        num_states: table.num_states,
        num_terminals,
        num_non_terminals,
        terminal_ids,
        non_terminal_ids,
        rule_mapping,
        state_symbols,
    })
}

/// Extract internal table arrays from ParseTable.
/// Since the internal fields aren't public, we reconstruct via lookups.
fn extract_table_arrays(
    table: &ParseTable,
) -> (Vec<u32>, Vec<i32>, Vec<u32>, Vec<u32>, Vec<i32>, Vec<u32>) {
    let num_states = table.num_states;
    let num_terminals = table.grammar.symbols.num_terminals();
    let num_non_terminals = table.grammar.symbols.num_non_terminals();

    // Reconstruct ACTION table as a dense matrix first
    let mut action_matrix = vec![vec![0u32; num_terminals as usize + 1]; num_states];

    for state in 0..num_states {
        for t in 0..=num_terminals {
            let terminal_id = SymbolId(t);
            let action = table.action(state, terminal_id);
            let entry = encode_action(&action);
            action_matrix[state][t as usize] = entry;
        }
    }

    // Reconstruct GOTO table as a dense matrix
    let mut goto_matrix = vec![vec![u32::MAX; num_non_terminals as usize]; num_states];

    for state in 0..num_states {
        for nt in 0..num_non_terminals {
            let nt_id = SymbolId(num_terminals + 1 + nt);
            if let Some(next_state) = table.goto(state, nt_id) {
                goto_matrix[state][nt as usize] = next_state as u32;
            }
        }
    }

    // Use row displacement to compress the tables
    let (action_data, action_base, action_check) =
        compress_table(&action_matrix, 0, num_terminals as usize + 1);
    let (goto_data, goto_base, goto_check) =
        compress_table_goto(&goto_matrix, num_non_terminals as usize);

    (
        action_data,
        action_base,
        action_check,
        goto_data,
        goto_base,
        goto_check,
    )
}

/// Encode an Action to a u32 (matching ActionEntry encoding).
fn encode_action(action: &gazelle_core::Action) -> u32 {
    match action {
        gazelle_core::Action::Error => 0,
        gazelle_core::Action::Shift(state) => 1 | ((*state as u32) << 2),
        gazelle_core::Action::Reduce(rule) => 2 | ((*rule as u32) << 2),
        gazelle_core::Action::Accept => 3,
        gazelle_core::Action::ShiftOrReduce {
            shift_state,
            reduce_rule,
        } => {
            // Encode as: type=3, shift_state in bits 2-16, reduce_rule in bits 17-31
            3 | ((*shift_state as u32) << 2) | ((*reduce_rule as u32) << 17)
        }
    }
}

/// Compress a table using row displacement.
fn compress_table(
    matrix: &[Vec<u32>],
    default_val: u32,
    row_width: usize,
) -> (Vec<u32>, Vec<i32>, Vec<u32>) {
    let num_rows = matrix.len();

    // Start with a reasonable size for the data array
    let mut data = vec![default_val; row_width * 2];
    let mut check = vec![u32::MAX; row_width * 2];
    let mut base = vec![0i32; num_rows];

    for (row_idx, row) in matrix.iter().enumerate() {
        // Find a position where this row fits
        let mut offset = 0i32;
        'find_offset: loop {
            let mut fits = true;
            for (col, &val) in row.iter().enumerate() {
                if val == default_val {
                    continue;
                }
                let pos = (offset + col as i32) as usize;
                if pos >= check.len() {
                    // Extend arrays
                    let new_size = pos + row_width;
                    data.resize(new_size, default_val);
                    check.resize(new_size, u32::MAX);
                }
                if check[pos] != u32::MAX {
                    fits = false;
                    break;
                }
            }
            if fits {
                break 'find_offset;
            }
            offset += 1;
        }

        // Place the row at this offset
        base[row_idx] = offset;
        for (col, &val) in row.iter().enumerate() {
            if val == default_val {
                continue;
            }
            let pos = (offset + col as i32) as usize;
            data[pos] = val;
            check[pos] = row_idx as u32;
        }
    }

    (data, base, check)
}

/// Compress GOTO table using row displacement (default is u32::MAX for no transition).
fn compress_table_goto(
    matrix: &[Vec<u32>],
    row_width: usize,
) -> (Vec<u32>, Vec<i32>, Vec<u32>) {
    compress_table(matrix, u32::MAX, row_width)
}

/// Compute the accessing symbol for each state.
/// The accessing symbol is the symbol (terminal or non-terminal) that was shifted/goto'd to reach that state.
fn compute_state_symbols(table: &ParseTable, num_terminals: u32, num_non_terminals: u32) -> Vec<u32> {
    use gazelle_core::Action;

    let num_states = table.num_states;
    let mut state_symbols = vec![0u32; num_states]; // 0 = no accessing symbol (for state 0)

    // Check ACTION table for shifts
    for state in 0..num_states {
        for t in 0..=num_terminals {
            let terminal_id = SymbolId(t);
            let action = table.action(state, terminal_id);
            match action {
                Action::Shift(target_state) => {
                    state_symbols[target_state] = t;
                }
                Action::ShiftOrReduce { shift_state, .. } => {
                    state_symbols[shift_state] = t;
                }
                _ => {}
            }
        }
    }

    // Check GOTO table for non-terminal transitions
    for state in 0..num_states {
        for nt in 0..num_non_terminals {
            let nt_id = SymbolId(num_terminals + 1 + nt);
            if let Some(target_state) = table.goto(state, nt_id) {
                state_symbols[target_state] = nt_id.0;
            }
        }
    }

    state_symbols
}

/// Generate static table data code.
pub fn generate_table_statics(grammar: &GrammarIr, table_data: &TableData) -> TokenStream {
    let name = &grammar.name;
    let mod_name = format_ident!("__{}_table", name.to_string().to_lowercase());

    let action_data = &table_data.action_data;
    let action_base = &table_data.action_base;
    let action_check = &table_data.action_check;
    let goto_data = &table_data.goto_data;
    let goto_base = &table_data.goto_base;
    let goto_check = &table_data.goto_check;

    let rules: Vec<_> = table_data
        .rules
        .iter()
        .map(|(lhs, len)| quote! { (#lhs, #len) })
        .collect();

    let num_states = table_data.num_states;
    let num_terminals = table_data.num_terminals;
    let num_non_terminals = table_data.num_non_terminals;
    let state_symbols = &table_data.state_symbols;

    // Generate symbol ID lookup
    let terminal_id_arms: Vec<_> = table_data
        .terminal_ids
        .iter()
        .map(|(name, id)| quote! { #name => ::gazelle_core::SymbolId(#id) })
        .collect();

    let non_terminal_id_arms: Vec<_> = table_data
        .non_terminal_ids
        .iter()
        .map(|(name, id)| quote! { #name => ::gazelle_core::SymbolId(#id) })
        .collect();

    quote! {
        #[doc(hidden)]
        mod #mod_name {
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
            pub const NUM_NON_TERMINALS: u32 = #num_non_terminals;

            pub fn symbol_id(name: &str) -> ::gazelle_core::SymbolId {
                match name {
                    #(#terminal_id_arms,)*
                    #(#non_terminal_id_arms,)*
                    _ => panic!("unknown symbol: {}", name),
                }
            }
        }
    }
}

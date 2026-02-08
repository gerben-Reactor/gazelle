use crate::grammar::{Grammar, SymbolId};
use crate::lr::{Automaton, GrammarInternal, to_grammar_internal};
use crate::runtime::{Action, ActionEntry, ErrorContext, ParseTable};

/// A conflict between two actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Conflict {
    ShiftReduce {
        state: usize,
        terminal: SymbolId,
        shift_state: usize,
        reduce_rule: usize,
    },
    ReduceReduce {
        state: usize,
        terminal: SymbolId,
        rule1: usize,
        rule2: usize,
    },
}

/// Grammar metadata for error reporting.
/// Only carries data not available through [`ParseTable`].
#[derive(Debug, Clone, Copy)]
pub struct ErrorInfo<'a> {
    /// Symbol names indexed by SymbolId.
    pub symbol_names: &'a [&'a str],
    /// Active items (rule, dot) per state.
    pub state_items: &'a [&'a [(u16, u8)]],
    /// RHS symbol IDs per rule.
    pub rule_rhs: &'a [&'a [u32]],
    /// Accessing symbol for each state.
    pub state_symbols: &'a [u32],
}

impl ErrorContext for ErrorInfo<'_> {
    fn symbol_name(&self, id: SymbolId) -> &str {
        self.symbol_names.get(id.0 as usize).copied().unwrap_or("<?>")
    }

    fn state_symbol(&self, state: usize) -> SymbolId {
        SymbolId(self.state_symbols.get(state).copied().unwrap_or(0))
    }

    fn state_items(&self, state: usize) -> &[(u16, u8)] {
        self.state_items.get(state).copied().unwrap_or(&[])
    }

    fn rule_rhs(&self, rule: usize) -> &[u32] {
        self.rule_rhs.get(rule).copied().unwrap_or(&[])
    }
}

/// Owned parse table data produced by [`CompiledTable::build`].
///
/// This holds the compressed table arrays, grammar, and conflict info.
/// Use [`CompiledTable::table`] to get a lightweight [`ParseTable`] for parsing.
#[derive(Debug)]
pub struct CompiledTable {
    // ACTION table (row displacement) — stored as raw u32 for ActionEntry
    action_data: Vec<u32>,
    action_base: Vec<i32>,
    action_check: Vec<u32>,

    // GOTO table (row displacement)
    goto_data: Vec<u32>,
    goto_base: Vec<i32>,
    goto_check: Vec<u32>,

    /// Rules: (lhs_id, rhs_len) for each rule.
    rules: Vec<(u32, u8)>,

    /// Number of terminals (including EOF) for goto column offset.
    num_terminals: u32,

    /// The augmented grammar.
    pub(crate) grammar: GrammarInternal,
    /// Number of states.
    pub num_states: usize,
    /// Conflicts detected during table construction.
    pub conflicts: Vec<Conflict>,

    // Error reporting data
    /// Active items (rule, dot) per state.
    state_items: Vec<Vec<(u16, u8)>>,
    /// RHS symbol IDs per rule.
    rule_rhs: Vec<Vec<u32>>,
    /// Accessing symbol for each state.
    state_symbols: Vec<u32>,
}

impl CompiledTable {
    /// Build parse tables from a grammar using the default algorithm (LALR(1)).
    pub fn build(grammar: &Grammar) -> Self {
        let algorithm = match grammar.mode.as_str() {
            "lr" | "lr1" => crate::lr::LrAlgorithm::Lr1,
            _ => crate::lr::LrAlgorithm::Lalr1,
        };
        let internal = to_grammar_internal(grammar.clone())
            .expect("grammar conversion failed");
        Self::build_with_algorithm(&internal, algorithm)
    }

    /// Build parse tables from internal grammar representation.
    pub(crate) fn build_with_algorithm(grammar: &GrammarInternal, algorithm: crate::lr::LrAlgorithm) -> Self {
        let automaton = Automaton::build_with_algorithm(grammar, algorithm);
        let grammar = &automaton.grammar;
        let num_states = automaton.num_states();
        let num_terminals = grammar.symbols.num_terminals();
        let num_non_terminals = grammar.symbols.num_non_terminals();

        // Build dense ACTION and GOTO tables first
        let mut action_rows: Vec<Vec<(u32, ActionEntry)>> = vec![Vec::new(); num_states];
        let mut goto_rows: Vec<Vec<(u32, u32)>> = vec![Vec::new(); num_states];
        let mut conflicts = Vec::new();

        const ACCEPT_RULE: usize = 0;

        for (state_idx, state) in automaton.states.iter().enumerate() {
            for item in state.iter() {
                if item.is_complete(grammar) {
                    let terminal_col = item.lookahead.0;

                    if item.rule == ACCEPT_RULE && item.lookahead == SymbolId::EOF {
                        Self::insert_action(
                            &mut action_rows[state_idx],
                            &mut conflicts,
                            state_idx,
                            terminal_col,
                            ActionEntry::reduce(ACCEPT_RULE),
                            &grammar.symbols,
                        );
                    } else {
                        Self::insert_action(
                            &mut action_rows[state_idx],
                            &mut conflicts,
                            state_idx,
                            terminal_col,
                            ActionEntry::reduce(item.rule),
                            &grammar.symbols,
                        );
                    }
                } else if let Some(next_symbol) = item.next_symbol(grammar)
                    && let Some(&next_state) = automaton.transitions.get(&(state_idx, next_symbol))
                {
                    if next_symbol.is_terminal() {
                        let terminal_col = next_symbol.id().0;
                        Self::insert_action(
                            &mut action_rows[state_idx],
                            &mut conflicts,
                            state_idx,
                            terminal_col,
                            ActionEntry::shift(next_state),
                            &grammar.symbols,
                        );
                    } else {
                        let nt_col = next_symbol.id().0 - grammar.symbols.num_terminals();
                        if !goto_rows[state_idx].iter().any(|(c, _)| *c == nt_col) {
                            goto_rows[state_idx].push((nt_col, next_state as u32));
                        }
                    }
                }
            }
        }

        // Compact the ACTION table
        let (action_data_entries, action_base, action_check) =
            Self::compact_table(&action_rows, num_terminals as usize);

        // Store action_data as raw u32
        let action_data: Vec<u32> = action_data_entries.iter().map(|e| e.0).collect();

        // Compact the GOTO table
        let (goto_data, goto_base, goto_check) =
            Self::compact_goto_table(&goto_rows, num_non_terminals as usize);

        // Extract rule info as (u32, u8)
        let rules: Vec<(u32, u8)> = grammar.rules.iter()
            .map(|r| (r.lhs.id().0, r.rhs.len() as u8))
            .collect();

        // Compute error reporting data
        // State items: active (rule, dot) pairs per state
        let state_items: Vec<Vec<(u16, u8)>> = automaton
            .states
            .iter()
            .map(|state| {
                state
                    .items
                    .iter()
                    .map(|item| (item.rule as u16, item.dot as u8))
                    .collect()
            })
            .collect();

        // Rule RHS: symbol IDs per rule
        let rule_rhs: Vec<Vec<u32>> = grammar
            .rules
            .iter()
            .map(|r| r.rhs.iter().map(|s| s.id().0).collect())
            .collect();

        // Compute state symbols (accessing symbol for each state)
        let state_symbols = Self::compute_state_symbols(
            &action_rows, &goto_rows, num_states, num_terminals
        );

        CompiledTable {
            action_data,
            action_base,
            action_check,
            goto_data,
            goto_base,
            goto_check,
            num_terminals,
            grammar: grammar.clone(),
            rules,
            num_states,
            conflicts,
            state_items,
            rule_rhs,
            state_symbols,
        }
    }

    fn compute_state_symbols(
        action_rows: &[Vec<(u32, ActionEntry)>],
        goto_rows: &[Vec<(u32, u32)>],
        num_states: usize,
        num_terminals: u32,
    ) -> Vec<u32> {
        let mut state_symbols = vec![0u32; num_states];

        // From action table: shifts tell us which terminal leads to which state
        for row in action_rows {
            for &(col, entry) in row {
                match entry.decode() {
                    Action::Shift(target) => state_symbols[target] = col,
                    Action::ShiftOrReduce { shift_state, .. } => state_symbols[shift_state] = col,
                    _ => {}
                }
            }
        }

        // From goto table: gotos tell us which non-terminal leads to which state
        for row in goto_rows {
            for &(col, target) in row {
                let nt_id = num_terminals + col;
                state_symbols[target as usize] = nt_id;
            }
        }

        state_symbols
    }

    /// Get a lightweight [`ParseTable`] borrowing from this compiled table.
    pub fn table(&self) -> ParseTable<'_> {
        ParseTable::new(
            &self.action_data,
            &self.action_base,
            &self.action_check,
            &self.goto_data,
            &self.goto_base,
            &self.goto_check,
            &self.rules,
            self.num_terminals,
        )
    }

    fn insert_action(
        row: &mut Vec<(u32, ActionEntry)>,
        conflicts: &mut Vec<Conflict>,
        state: usize,
        col: u32,
        new_action: ActionEntry,
        symbols: &crate::lr::SymbolTable,
    ) {
        if let Some(entry) = row.iter_mut().find(|(c, _)| *c == col) {
            let existing = entry.1;
            if existing != new_action {
                let is_prec = symbols.is_prec_terminal(SymbolId(col));

                match (new_action.decode(), existing.decode()) {
                    (Action::Shift(shift_state), Action::Reduce(reduce_rule))
                    | (Action::Reduce(reduce_rule), Action::Shift(shift_state)) => {
                        if is_prec {
                            entry.1 = ActionEntry::shift_or_reduce(shift_state, reduce_rule);
                        } else {
                            conflicts.push(Conflict::ShiftReduce {
                                state,
                                terminal: SymbolId(col),
                                shift_state,
                                reduce_rule,
                            });
                            // Resolve by shifting (standard behavior) to avoid duplicate detection
                            entry.1 = ActionEntry::shift(shift_state);
                        }
                    }
                    (Action::Reduce(rule1), Action::Reduce(rule2)) => {
                        conflicts.push(Conflict::ReduceReduce {
                            state,
                            terminal: SymbolId(col),
                            rule1,
                            rule2,
                        });
                    }
                    _ => {}
                }
            }
        } else {
            row.push((col, new_action));
        }
    }

    fn compact_table(
        rows: &[Vec<(u32, ActionEntry)>],
        num_cols: usize,
    ) -> (Vec<ActionEntry>, Vec<i32>, Vec<u32>) {
        let mut data = vec![ActionEntry::ERROR; num_cols * 2];
        let mut check: Vec<u32> = vec![u32::MAX; num_cols * 2];
        let mut base = vec![0i32; rows.len()];

        for (state, row) in rows.iter().enumerate() {
            if row.is_empty() {
                continue;
            }

            let min_col = row.iter().map(|(c, _)| *c).min().unwrap_or(0) as i32;

            let mut displacement = -min_col;
            'search: loop {
                let mut ok = true;
                for &(col, _) in row {
                    let idx = (displacement + col as i32) as usize;
                    if idx >= check.len() {
                        let new_size = (idx + 1).max(data.len() * 2);
                        data.resize(new_size, ActionEntry::ERROR);
                        check.resize(new_size, u32::MAX);
                    }
                    if check[idx] != u32::MAX {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    break 'search;
                }
                displacement += 1;
            }

            base[state] = displacement;
            for &(col, action) in row {
                let idx = (displacement + col as i32) as usize;
                data[idx] = action;
                check[idx] = state as u32;
            }
        }

        (data, base, check)
    }

    fn compact_goto_table(
        rows: &[Vec<(u32, u32)>],
        num_cols: usize,
    ) -> (Vec<u32>, Vec<i32>, Vec<u32>) {
        let mut data = vec![0u32; num_cols * 2];
        let mut check: Vec<u32> = vec![u32::MAX; num_cols * 2];
        let mut base = vec![0i32; rows.len()];

        for (state, row) in rows.iter().enumerate() {
            if row.is_empty() {
                continue;
            }

            let min_col = row.iter().map(|(c, _)| *c).min().unwrap_or(0) as i32;

            let mut displacement = -min_col;
            'search: loop {
                let mut ok = true;
                for &(col, _) in row {
                    let idx = (displacement + col as i32) as usize;
                    if idx >= check.len() {
                        let new_size = (idx + 1).max(data.len() * 2);
                        data.resize(new_size, 0);
                        check.resize(new_size, u32::MAX);
                    }
                    if check[idx] != u32::MAX {
                        ok = false;
                        break;
                    }
                }

                if ok {
                    break 'search;
                }
                displacement += 1;
            }

            base[state] = displacement;
            for &(col, value) in row {
                let idx = (displacement + col as i32) as usize;
                data[idx] = value;
                check[idx] = state as u32;
            }
        }

        (data, base, check)
    }

    /// Returns true if the table has conflicts.
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }

    /// Format a rule as "lhs -> rhs1 rhs2 ..."
    fn format_rule(&self, rule_idx: usize) -> String {
        let rule = &self.grammar.rules[rule_idx];
        let lhs_name = self.grammar.symbols.name(rule.lhs.id());
        let rhs_names: Vec<_> = rule.rhs.iter()
            .map(|s| self.grammar.symbols.name(s.id()))
            .collect();
        if rhs_names.is_empty() {
            format!("{} -> (empty)", lhs_name)
        } else {
            format!("{} -> {}", lhs_name, rhs_names.join(" "))
        }
    }

    /// Format conflicts as human-readable error messages.
    pub fn format_conflicts(&self) -> Vec<String> {
        self.conflicts.iter().map(|c| {
            match c {
                Conflict::ShiftReduce { state, terminal, reduce_rule, .. } => {
                    let term_name = self.grammar.symbols.name(*terminal);
                    let reduce_str = self.format_rule(*reduce_rule);
                    let context = self.format_state_context(*state, Some(*terminal));
                    format!(
                        "Shift/reduce conflict on '{}':\n  \
                         - Shift (continue parsing)\n  \
                         - Reduce by: {}\n\n\
                         Parser state when seeing '{}':\n{}",
                        term_name, reduce_str, term_name, context
                    )
                }
                Conflict::ReduceReduce { state, terminal, rule1, rule2 } => {
                    let term_name = self.grammar.symbols.name(*terminal);
                    let rule1_str = self.format_rule(*rule1);
                    let rule2_str = self.format_rule(*rule2);
                    let context = self.format_state_context(*state, Some(*terminal));
                    format!(
                        "Reduce/reduce conflict on '{}':\n  \
                         - Reduce by: {}\n  \
                         - Reduce by: {}\n\n\
                         Parser state when seeing '{}':\n{}",
                        term_name, rule1_str, rule2_str, term_name, context
                    )
                }
            }
        }).collect()
    }

    /// Format state context showing active items (deduplicated).
    fn format_state_context(&self, state: usize, terminal: Option<SymbolId>) -> String {
        let items = &self.state_items[state];
        let mut seen = std::collections::HashSet::new();
        let mut lines = Vec::new();

        for &(rule, dot) in items {
            let rule = rule as usize;
            let dot = dot as usize;

            let lhs = self.grammar.rules[rule].lhs;
            let lhs_name = self.grammar.symbols.name(lhs.id());
            let rhs = &self.rule_rhs[rule];

            let mut item_str = format!("  {} ->", lhs_name);
            for (i, &sym_id) in rhs.iter().enumerate() {
                if i == dot {
                    item_str.push_str(" •");
                }
                item_str.push(' ');
                item_str.push_str(self.grammar.symbols.name(SymbolId(sym_id)));
            }
            if dot == rhs.len() {
                item_str.push_str(" •");
                // This is a complete item (reduce)
                if let Some(term) = terminal {
                    item_str.push_str(&format!("  [reduce on {}]", self.grammar.symbols.name(term)));
                }
            } else if let Some(term) = terminal {
                // Check if this item can shift the terminal
                if rhs.get(dot) == Some(&term.0) {
                    item_str.push_str("  [shift]");
                }
            }

            // Deduplicate identical lines (e.g., same rule with different lookaheads)
            if seen.insert(item_str.clone()) {
                lines.push(item_str);
            }
        }

        lines.join("\n")
    }

    /// Lookup symbol ID by name.
    pub fn symbol_id(&self, name: &str) -> Option<SymbolId> {
        self.grammar.symbols.get_id(name)
    }

    /// Get the name of a symbol by ID.
    pub fn symbol_name(&self, id: SymbolId) -> &str {
        self.grammar.symbols.name(id)
    }

    /// Get the total number of symbols.
    pub fn num_symbols(&self) -> u32 {
        self.grammar.symbols.num_symbols()
    }

    /// Get the number of terminal symbols.
    pub fn num_terminals(&self) -> u32 {
        self.grammar.symbols.num_terminals()
    }

    // Accessors for compressed table arrays (for codegen/serialization)

    pub fn action_data(&self) -> &[u32] {
        &self.action_data
    }

    pub fn action_base(&self) -> &[i32] {
        &self.action_base
    }

    pub fn action_check(&self) -> &[u32] {
        &self.action_check
    }

    pub fn goto_data(&self) -> &[u32] {
        &self.goto_data
    }

    pub fn goto_base(&self) -> &[i32] {
        &self.goto_base
    }

    pub fn goto_check(&self) -> &[u32] {
        &self.goto_check
    }

    /// Get rule info as (u32, u8) pairs.
    pub fn rules(&self) -> &[(u32, u8)] {
        &self.rules
    }

    /// Get state items (rule, dot) per state.
    pub fn state_items(&self) -> &[Vec<(u16, u8)>] {
        &self.state_items
    }

    /// Get rule RHS symbol IDs.
    pub fn rule_rhs(&self) -> &[Vec<u32>] {
        &self.rule_rhs
    }

    /// Get the name of a rule (if it has one).
    pub fn rule_name(&self, rule: usize) -> Option<&str> {
        self.grammar.rules.get(rule).and_then(|r| r.name.as_deref())
    }

    /// Get accessing symbol for each state.
    pub fn state_symbols(&self) -> &[u32] {
        &self.state_symbols
    }
}

impl ErrorContext for CompiledTable {
    fn symbol_name(&self, id: SymbolId) -> &str {
        self.grammar.symbols.name(id)
    }

    fn state_symbol(&self, state: usize) -> SymbolId {
        SymbolId(self.state_symbols.get(state).copied().unwrap_or(0))
    }

    fn state_items(&self, state: usize) -> &[(u16, u8)] {
        self.state_items.get(state).map(|v| v.as_slice()).unwrap_or(&[])
    }

    fn rule_rhs(&self, rule: usize) -> &[u32] {
        self.rule_rhs.get(rule).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::parse_grammar;
    use crate::lr::to_grammar_internal;

    fn simple_grammar() -> GrammarInternal {
        to_grammar_internal(parse_grammar(r#"
            grammar Simple { start s; terminals { a } s = a; }
        "#).unwrap()).unwrap()
    }

    fn expr_grammar() -> GrammarInternal {
        to_grammar_internal(parse_grammar(r#"
            grammar Expr {
                start expr;
                terminals { PLUS, NUM }
                expr = expr PLUS term | term;
                term = NUM;
            }
        "#).unwrap()).unwrap()
    }

    fn ambiguous_grammar() -> GrammarInternal {
        to_grammar_internal(parse_grammar(r#"
            grammar Ambiguous {
                start expr;
                terminals { PLUS, NUM }
                expr = expr PLUS expr | NUM;
            }
        "#).unwrap()).unwrap()
    }

    fn prec_grammar() -> GrammarInternal {
        to_grammar_internal(parse_grammar(r#"
            grammar Prec {
                start expr;
                terminals { prec OP, NUM }
                expr = expr OP expr | NUM;
            }
        "#).unwrap()).unwrap()
    }

    #[test]
    fn test_simple_table() {
        let grammar = simple_grammar();
        let compiled = CompiledTable::build_with_algorithm(&grammar, crate::lr::LrAlgorithm::default());
        let table = compiled.table();

        assert!(!compiled.has_conflicts());

        let a_id = compiled.symbol_id("a").unwrap();
        match table.action(0, a_id) {
            Action::Shift(_) => {}
            other => panic!("Expected Shift, got {:?}", other),
        }
    }

    #[test]
    fn test_expr_table() {
        let grammar = expr_grammar();
        let compiled = CompiledTable::build_with_algorithm(&grammar, crate::lr::LrAlgorithm::default());
        let table = compiled.table();

        assert!(!compiled.has_conflicts(), "Unexpected conflicts: {:?}", compiled.conflicts);

        let num_id = compiled.symbol_id("NUM").unwrap();
        match table.action(0, num_id) {
            Action::Shift(_) => {}
            other => panic!("Expected Shift on NUM, got {:?}", other),
        }
    }

    #[test]
    fn test_ambiguous_grammar() {
        let grammar = ambiguous_grammar();
        let compiled = CompiledTable::build_with_algorithm(&grammar, crate::lr::LrAlgorithm::default());

        assert!(compiled.has_conflicts(), "Expected conflicts for ambiguous grammar");

        let has_sr_conflict = compiled.conflicts.iter().any(|c| {
            matches!(c, Conflict::ShiftReduce { .. })
        });
        assert!(has_sr_conflict, "Expected shift/reduce conflict");

        // Test conflict formatting
        let messages = compiled.format_conflicts();
        assert!(!messages.is_empty(), "Expected formatted conflict messages");
        assert!(messages[0].contains("Shift/reduce conflict"), "Message should describe conflict type: {}", messages[0]);
        assert!(messages[0].contains("'PLUS'"), "Message should mention the terminal: {}", messages[0]);
        assert!(messages[0].contains("expr -> expr PLUS expr"), "Message should show the rule: {}", messages[0]);
    }

    #[test]
    fn test_prec_terminal_no_conflict() {
        let grammar = prec_grammar();
        let compiled = CompiledTable::build_with_algorithm(&grammar, crate::lr::LrAlgorithm::default());
        let table = compiled.table();

        assert!(!compiled.has_conflicts(), "PrecTerminal should not report conflicts: {:?}", compiled.conflicts);

        // Find state with ShiftOrReduce
        let op_id = compiled.symbol_id("OP").unwrap();
        let mut found_shift_or_reduce = false;
        for state in 0..compiled.num_states {
            if let Action::ShiftOrReduce { .. } = table.action(state, op_id) {
                found_shift_or_reduce = true;
                break;
            }
        }
        assert!(found_shift_or_reduce, "Expected ShiftOrReduce action for OP");
    }

    #[test]
    fn test_goto() {
        let grammar = expr_grammar();
        let compiled = CompiledTable::build_with_algorithm(&grammar, crate::lr::LrAlgorithm::default());
        let table = compiled.table();

        let expr_id = compiled.symbol_id("expr").unwrap();
        let term_id = compiled.symbol_id("term").unwrap();

        assert!(table.goto(0, expr_id).is_some(), "Expected goto on expr from state 0");
        assert!(table.goto(0, term_id).is_some(), "Expected goto on term from state 0");
    }
}

use crate::grammar::{Grammar, Symbol, SymbolId};
use crate::lr::Automaton;

/// An action in the parse table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Shift the token and go to the given state.
    Shift(usize),
    /// Reduce using the given rule index. Reduce(0) means accept.
    Reduce(usize),
    /// Shift/reduce conflict resolved by precedence at runtime.
    ShiftOrReduce { shift_state: usize, reduce_rule: usize },
    /// Error (no valid action).
    Error,
}

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

/// Encoded action entry for compact parse tables.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ActionEntry(pub u32);

impl ActionEntry {
    pub const ERROR: ActionEntry = ActionEntry(0);

    pub fn shift(state: usize) -> Self {
        debug_assert!(state > 0, "Shift(0) is reserved for Error");
        debug_assert!(state < 0x80000000, "Shift state too large");
        ActionEntry(state as u32)
    }

    pub fn reduce(rule: usize) -> Self {
        debug_assert!(rule < 0x1000, "Reduce rule too large (max 4095)");
        ActionEntry(!(rule as u32))
    }

    pub fn shift_or_reduce(shift_state: usize, reduce_rule: usize) -> Self {
        debug_assert!(shift_state > 0, "Shift(0) is reserved for Error");
        debug_assert!(shift_state < 0x80000, "Shift state too large (max 19 bits)");
        debug_assert!(reduce_rule < 0x1000, "Reduce rule too large (max 4095)");
        ActionEntry(!((reduce_rule as u32) | ((shift_state as u32) << 12)))
    }

    pub fn decode(&self) -> Action {
        let v = self.0 as i32;
        if v > 0 {
            Action::Shift(v as usize)
        } else if v == 0 {
            Action::Error
        } else {
            let payload = !self.0;
            let r = (payload & 0xFFF) as usize;
            let s = ((payload >> 12) & 0x7FFFF) as usize;
            if s == 0 {
                Action::Reduce(r)
            } else {
                Action::ShiftOrReduce { shift_state: s, reduce_rule: r }
            }
        }
    }
}

/// Trait for providing error context (symbol names, expected terminals).
pub trait ErrorContext {
    /// Get the name for a symbol ID.
    fn symbol_name(&self, id: SymbolId) -> &str;
    /// Get expected terminal IDs for a state.
    fn expected_terminals(&self, state: usize) -> Vec<u32>;
}

/// Grammar metadata for error reporting.
#[derive(Debug, Clone, Copy)]
pub struct ErrorInfo<'a> {
    /// Symbol names indexed by SymbolId.
    pub symbol_names: &'a [&'a str],
    /// Expected terminal IDs per state.
    pub expected: &'a [&'a [u32]],
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

    fn expected_terminals(&self, state: usize) -> Vec<u32> {
        self.expected.get(state).copied().unwrap_or(&[]).to_vec()
    }
}

impl ErrorInfo<'_> {
    /// Get active items for a state.
    pub fn state_items(&self, state: usize) -> &[(u16, u8)] {
        self.state_items.get(state).copied().unwrap_or(&[])
    }

    /// Get RHS symbol IDs for a rule.
    pub fn rule_rhs(&self, rule: usize) -> &[u32] {
        self.rule_rhs.get(rule).copied().unwrap_or(&[])
    }

    /// Get the accessing symbol for a state.
    pub fn state_symbol(&self, state: usize) -> SymbolId {
        SymbolId(self.state_symbols.get(state).copied().unwrap_or(0))
    }
}

/// Lightweight parse table that borrows compressed table data.
///
/// This is the runtime representation used by the parser. It borrows slices
/// from either static data (generated code) or a [`CompiledTable`].
#[derive(Debug, Clone, Copy)]
pub struct ParseTable<'a> {
    action_data: &'a [u32],
    action_base: &'a [i32],
    action_check: &'a [u32],
    goto_data: &'a [u32],
    goto_base: &'a [i32],
    goto_check: &'a [u32],
    rules: &'a [(u32, u8)],
    num_terminals: u32,
    /// Optional error info for rich error messages.
    error_info: Option<ErrorInfo<'a>>,
}

impl<'a> ParseTable<'a> {
    /// Create a parse table from borrowed slices.
    pub const fn new(
        action_data: &'a [u32],
        action_base: &'a [i32],
        action_check: &'a [u32],
        goto_data: &'a [u32],
        goto_base: &'a [i32],
        goto_check: &'a [u32],
        rules: &'a [(u32, u8)],
        num_terminals: u32,
    ) -> Self {
        ParseTable {
            action_data,
            action_base,
            action_check,
            goto_data,
            goto_base,
            goto_check,
            rules,
            num_terminals,
            error_info: None,
        }
    }

    /// Add error info for rich error messages.
    pub const fn with_error_info(self, info: ErrorInfo<'a>) -> Self {
        Self {
            error_info: Some(info),
            ..self
        }
    }

    /// Get the error info, if available.
    pub fn error_info(&self) -> Option<&ErrorInfo<'a>> {
        self.error_info.as_ref()
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
    pub grammar: Grammar,
    /// Number of states.
    pub num_states: usize,
    /// Conflicts detected during table construction.
    pub conflicts: Vec<Conflict>,

    // Error reporting data
    /// Expected terminals per state.
    expected_terminals: Vec<Vec<u32>>,
    /// Active items (rule, dot) per state.
    state_items: Vec<Vec<(u16, u8)>>,
    /// RHS symbol IDs per rule.
    rule_rhs: Vec<Vec<u32>>,
}

impl CompiledTable {
    /// Build parse tables from a grammar using the default algorithm (LALR(1)).
    pub fn build(grammar: &Grammar) -> Self {
        Self::build_with_algorithm(grammar, crate::lr::LrAlgorithm::default())
    }

    /// Build parse tables from a grammar using the specified algorithm.
    pub fn build_with_algorithm(grammar: &Grammar, algorithm: crate::lr::LrAlgorithm) -> Self {
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
        // Expected terminals: terminals with non-error actions per state
        let expected_terminals: Vec<Vec<u32>> = action_rows
            .iter()
            .map(|row| row.iter().map(|(col, _)| *col).collect())
            .collect();

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
            expected_terminals,
            state_items,
            rule_rhs,
        }
    }

    /// Get a lightweight [`ParseTable`] borrowing from this compiled table.
    pub fn table(&self) -> ParseTable<'_> {
        ParseTable {
            action_data: &self.action_data,
            action_base: &self.action_base,
            action_check: &self.action_check,
            goto_data: &self.goto_data,
            goto_base: &self.goto_base,
            goto_check: &self.goto_check,
            rules: &self.rules,
            num_terminals: self.num_terminals,
            error_info: None,
        }
    }

    fn insert_action(
        row: &mut Vec<(u32, ActionEntry)>,
        conflicts: &mut Vec<Conflict>,
        state: usize,
        col: u32,
        new_action: ActionEntry,
        symbols: &crate::grammar::SymbolTable,
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

    /// Lookup symbol ID by name.
    pub fn symbol_id(&self, name: &str) -> Option<SymbolId> {
        self.grammar.symbols.get_id(name)
    }

    /// Lookup symbol by name.
    pub fn symbol(&self, name: &str) -> Option<Symbol> {
        self.grammar.symbols.get(name)
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

    /// Get expected terminals per state.
    pub fn expected_terminals(&self) -> &[Vec<u32>] {
        &self.expected_terminals
    }

    /// Get state items (rule, dot) per state.
    pub fn state_items(&self) -> &[Vec<(u16, u8)>] {
        &self.state_items
    }

    /// Get rule RHS symbol IDs.
    pub fn rule_rhs(&self) -> &[Vec<u32>] {
        &self.rule_rhs
    }
}

impl ErrorContext for CompiledTable {
    fn symbol_name(&self, id: SymbolId) -> &str {
        self.grammar.symbols.name(id)
    }

    fn expected_terminals(&self, state: usize) -> Vec<u32> {
        self.expected_terminals.get(state).cloned().unwrap_or_default()
    }
}

impl<'a> ParseTable<'a> {
    /// Get the action for a state and terminal. O(1) lookup.
    pub fn action(&self, state: usize, terminal: SymbolId) -> Action {
        let col = terminal.0 as i32;
        let displacement = self.action_base[state];
        let idx = displacement.wrapping_add(col) as usize;

        if idx < self.action_check.len() && self.action_check[idx] == state as u32 {
            ActionEntry(self.action_data[idx]).decode()
        } else {
            Action::Error
        }
    }

    /// Get the goto state for a state and non-terminal. O(1) lookup.
    pub fn goto(&self, state: usize, non_terminal: SymbolId) -> Option<usize> {
        let col = (non_terminal.0 - self.num_terminals) as i32;
        let displacement = self.goto_base[state];
        let idx = displacement.wrapping_add(col) as usize;

        if idx < self.goto_check.len() && self.goto_check[idx] == state as u32 {
            Some(self.goto_data[idx] as usize)
        } else {
            None
        }
    }

    /// Get rule info: (lhs symbol ID, rhs length).
    pub fn rule_info(&self, rule: usize) -> (SymbolId, usize) {
        let (lhs, len) = self.rules[rule];
        (SymbolId(lhs), len as usize)
    }

    /// Get all rules as (lhs_id, rhs_len) pairs.
    pub fn rules(&self) -> &[(u32, u8)] {
        self.rules
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::GrammarBuilder;

    fn simple_grammar() -> Grammar {
        let mut gb = GrammarBuilder::new();
        let a = gb.t("a");
        let s = gb.nt("S");
        gb.rule(s, vec![a]);
        gb.build()
    }

    fn expr_grammar() -> Grammar {
        let mut gb = GrammarBuilder::new();
        let plus = gb.t("+");
        let num = gb.t("NUM");
        let expr = gb.nt("expr");
        let term = gb.nt("term");

        gb.rule(expr, vec![expr, plus, term]);
        gb.rule(expr, vec![term]);
        gb.rule(term, vec![num]);

        gb.build()
    }

    fn ambiguous_grammar() -> Grammar {
        let mut gb = GrammarBuilder::new();
        let plus = gb.t("+");
        let num = gb.t("NUM");
        let expr = gb.nt("expr");

        gb.rule(expr, vec![expr, plus, expr]);
        gb.rule(expr, vec![num]);

        gb.build()
    }

    fn prec_grammar() -> Grammar {
        let mut gb = GrammarBuilder::new();
        let op = gb.pt("OP");
        let num = gb.t("NUM");
        let expr = gb.nt("expr");

        gb.rule(expr, vec![expr, op, expr]);
        gb.rule(expr, vec![num]);

        gb.build()
    }

    #[test]
    fn test_action_entry_encoding() {
        let shift = ActionEntry::shift(42);
        assert_eq!(shift.decode(), Action::Shift(42));

        let reduce = ActionEntry::reduce(7);
        assert_eq!(reduce.decode(), Action::Reduce(7));

        // Accept is Reduce(0)
        let accept = ActionEntry::reduce(0);
        assert_eq!(accept.decode(), Action::Reduce(0));

        let error = ActionEntry::ERROR;
        assert_eq!(error.decode(), Action::Error);

        let sor = ActionEntry::shift_or_reduce(10, 5);
        match sor.decode() {
            Action::ShiftOrReduce { shift_state, reduce_rule } => {
                assert_eq!(shift_state, 10);
                assert_eq!(reduce_rule, 5);
            }
            other => panic!("Expected ShiftOrReduce, got {:?}", other),
        }
    }

    #[test]
    fn test_simple_table() {
        let grammar = simple_grammar();
        let compiled = CompiledTable::build(&grammar);
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
        let compiled = CompiledTable::build(&grammar);
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
        let compiled = CompiledTable::build(&grammar);

        assert!(compiled.has_conflicts(), "Expected conflicts for ambiguous grammar");

        let has_sr_conflict = compiled.conflicts.iter().any(|c| {
            matches!(c, Conflict::ShiftReduce { .. })
        });
        assert!(has_sr_conflict, "Expected shift/reduce conflict");
    }

    #[test]
    fn test_prec_terminal_no_conflict() {
        let grammar = prec_grammar();
        let compiled = CompiledTable::build(&grammar);
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
        let compiled = CompiledTable::build(&grammar);
        let table = compiled.table();

        let expr_id = compiled.symbol_id("expr").unwrap();
        let term_id = compiled.symbol_id("term").unwrap();

        assert!(table.goto(0, expr_id).is_some(), "Expected goto on expr from state 0");
        assert!(table.goto(0, term_id).is_some(), "Expected goto on term from state 0");
    }
}

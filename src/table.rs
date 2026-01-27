use crate::grammar::{Grammar, Symbol, SymbolId};
use crate::lr::Automaton;

/// An action in the parse table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Shift the token and go to the given state.
    Shift(usize),
    /// Reduce using the given rule index.
    Reduce(usize),
    /// Shift/reduce conflict resolved by precedence at runtime.
    ShiftOrReduce { shift_state: usize, reduce_rule: usize },
    /// Accept the input.
    Accept,
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
        ActionEntry(1 | ((state as u32) << 2))
    }

    pub fn reduce(rule: usize) -> Self {
        ActionEntry(2 | ((rule as u32) << 2))
    }

    pub fn accept() -> Self {
        ActionEntry(3)
    }

    pub fn shift_or_reduce(shift_state: usize, reduce_rule: usize) -> Self {
        let payload = 1 | ((shift_state as u32) << 1) | ((reduce_rule as u32) << 15);
        ActionEntry(3 | (payload << 2))
    }

    pub fn decode(&self) -> Action {
        let action_type = self.0 & 3;
        let payload = self.0 >> 2;

        match action_type {
            0 => Action::Error,
            1 => Action::Shift(payload as usize),
            2 => Action::Reduce(payload as usize),
            3 => {
                if payload == 0 {
                    Action::Accept
                } else {
                    let shift_state = ((payload >> 1) & 0x3FFF) as usize;
                    let reduce_rule = (payload >> 15) as usize;
                    Action::ShiftOrReduce { shift_state, reduce_rule }
                }
            }
            _ => unreachable!(),
        }
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
}

impl<'a> ParseTable<'a> {
    /// Create a parse table from borrowed slices.
    ///
    /// `num_terminals` must include EOF (i.e., `count_of_user_terminals + 1`).
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
        }
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
}

impl CompiledTable {
    /// Build parse tables from an automaton.
    pub fn build(automaton: &Automaton) -> Self {
        let grammar = &automaton.grammar;
        let num_states = automaton.num_states();
        let num_terminals = grammar.symbols.num_terminals() + 1; // +1 for EOF
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
                            ActionEntry::accept(),
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
                        let nt_col = next_symbol.id().0 - grammar.symbols.num_terminals() - 1;
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

        let accept = ActionEntry::accept();
        assert_eq!(accept.decode(), Action::Accept);

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
        let automaton = Automaton::build(&grammar);
        let compiled = CompiledTable::build(&automaton);
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
        let automaton = Automaton::build(&grammar);
        let compiled = CompiledTable::build(&automaton);
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
        let automaton = Automaton::build(&grammar);
        let compiled = CompiledTable::build(&automaton);

        assert!(compiled.has_conflicts(), "Expected conflicts for ambiguous grammar");

        let has_sr_conflict = compiled.conflicts.iter().any(|c| {
            matches!(c, Conflict::ShiftReduce { .. })
        });
        assert!(has_sr_conflict, "Expected shift/reduce conflict");
    }

    #[test]
    fn test_prec_terminal_no_conflict() {
        let grammar = prec_grammar();
        let automaton = Automaton::build(&grammar);
        let compiled = CompiledTable::build(&automaton);
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
        let automaton = Automaton::build(&grammar);
        let compiled = CompiledTable::build(&automaton);
        let table = compiled.table();

        let expr_id = compiled.symbol_id("expr").unwrap();
        let term_id = compiled.symbol_id("term").unwrap();

        assert!(table.goto(0, expr_id).is_some(), "Expected goto on expr from state 0");
        assert!(table.goto(0, term_id).is_some(), "Expected goto on term from state 0");
    }
}

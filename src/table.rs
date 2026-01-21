use crate::grammar::{Grammar, Symbol, SymbolId, SymbolTable};
use crate::lr::{Automaton, InternedAutomaton};
use std::collections::HashMap;

/// An action in the parse table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Shift the token and go to the given state.
    Shift(usize),
    /// Reduce using the given rule index.
    Reduce(usize),
    /// Shift/reduce conflict resolved by precedence at runtime.
    /// Only generated for PrecTerminal symbols.
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
        terminal: Symbol,
        shift_state: usize,
        reduce_rule: usize,
    },
    ReduceReduce {
        state: usize,
        terminal: Symbol,
        rule1: usize,
        rule2: usize,
    },
}

/// The parse tables for an LR parser.
#[derive(Debug)]
pub struct ParseTable {
    /// Action table: (state, terminal) -> Action
    pub action: HashMap<(usize, Option<Symbol>), Action>,
    /// Goto table: (state, non-terminal) -> state
    pub goto: HashMap<(usize, Symbol), usize>,
    /// The augmented grammar (rule 0 is __start -> original_start).
    pub grammar: Grammar,
    /// Number of states.
    pub num_states: usize,
    /// Conflicts detected during table construction.
    pub conflicts: Vec<Conflict>,
}

impl ParseTable {
    /// Build parse tables from an automaton.
    ///
    /// Uses the augmented grammar stored in the automaton.
    /// Rule 0 is always the accept rule (__start -> original_start).
    pub fn build(automaton: &Automaton) -> Self {
        let grammar = &automaton.grammar;
        let mut action: HashMap<(usize, Option<Symbol>), Action> = HashMap::new();
        let mut goto: HashMap<(usize, Symbol), usize> = HashMap::new();
        let mut conflicts = Vec::new();

        // Rule 0 is always the augmented start rule: __start -> <original_start>
        // When this rule completes at EOF, we accept.
        const ACCEPT_RULE: usize = 0;

        for (state_idx, state) in automaton.states.iter().enumerate() {
            // Process each item in the state
            for item in state.iter() {
                if item.is_complete(grammar) {
                    // Reduce item: A -> α •, a
                    // Add reduce action for lookahead
                    let key = (state_idx, item.lookahead.clone());

                    // Check if this is the accept state (rule 0 complete at EOF)
                    if item.rule == ACCEPT_RULE && item.lookahead.is_none() {
                        insert_action(&mut action, &mut conflicts, key, Action::Accept);
                    } else {
                        insert_action(&mut action, &mut conflicts, key, Action::Reduce(item.rule));
                    }
                } else if let Some(next_symbol) = item.next_symbol(grammar) {
                    // Shift item: A -> α • X β
                    if let Some(&next_state) = automaton.transitions.get(&(state_idx, next_symbol.clone())) {
                        match next_symbol {
                            Symbol::Terminal(_) | Symbol::PrecTerminal(_) => {
                                let key = (state_idx, Some(next_symbol.clone()));
                                insert_action(&mut action, &mut conflicts, key, Action::Shift(next_state));
                            }
                            Symbol::NonTerminal(_) => {
                                goto.insert((state_idx, next_symbol.clone()), next_state);
                            }
                        }
                    }
                }
            }
        }

        ParseTable {
            action,
            goto,
            grammar: grammar.clone(),
            num_states: automaton.num_states(),
            conflicts,
        }
    }

    /// Get the action for a state and terminal.
    pub fn action(&self, state: usize, terminal: Option<&Symbol>) -> &Action {
        self.action.get(&(state, terminal.cloned())).unwrap_or(&Action::Error)
    }

    /// Get the goto state for a state and non-terminal.
    pub fn goto(&self, state: usize, non_terminal: &Symbol) -> Option<usize> {
        self.goto.get(&(state, non_terminal.clone())).copied()
    }

    /// Returns true if the table has conflicts.
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
}

// ============================================================================
// Compact Parse Table using Row Displacement Compaction
// ============================================================================

/// Encoded action entry for compact parse tables.
/// Bits 0-1: action type (0=Error, 1=Shift, 2=Reduce, 3=Accept/ShiftOrReduce)
/// Bits 2-31: payload (state for Shift, rule for Reduce)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// Encode ShiftOrReduce: we use type 3 with high bit of payload indicating this,
    /// and we pack both shift_state and reduce_rule.
    /// Format: type=3, bit 2 = 1 (ShiftOrReduce marker), bits 3-16 = shift_state, bits 17-30 = reduce_rule
    pub fn shift_or_reduce(shift_state: usize, reduce_rule: usize) -> Self {
        let payload = 1 | ((shift_state as u32) << 1) | ((reduce_rule as u32) << 15);
        ActionEntry(3 | (payload << 2))
    }

    /// Decode this entry into an Action.
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
                    // ShiftOrReduce
                    let shift_state = ((payload >> 1) & 0x3FFF) as usize;
                    let reduce_rule = (payload >> 15) as usize;
                    Action::ShiftOrReduce { shift_state, reduce_rule }
                }
            }
            _ => unreachable!(),
        }
    }
}

/// A compact conflict using SymbolId instead of Symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompactConflict {
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

/// A compact parse table using row displacement compaction for O(1) lookups.
#[derive(Debug)]
pub struct CompactParseTable {
    // ACTION table (row displacement)
    action_data: Vec<ActionEntry>,
    action_base: Vec<i32>,           // per-state offset into action_data
    action_check: Vec<u32>,          // verify entry belongs to state

    // GOTO table (row displacement)
    goto_data: Vec<u32>,
    goto_base: Vec<i32>,
    goto_check: Vec<u32>,

    // Metadata
    pub symbols: SymbolTable,
    /// Rules: (lhs, rhs_len) for each rule
    pub rules: Vec<(SymbolId, u8)>,
    pub num_states: usize,
    pub conflicts: Vec<CompactConflict>,
}

impl CompactParseTable {
    /// Build a compact parse table from an interned automaton.
    pub fn build(automaton: &InternedAutomaton) -> Self {
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
                    // Reduce item
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
                    if grammar.symbols.is_terminal(next_symbol) {
                        // Shift action
                        let terminal_col = next_symbol.0;
                        Self::insert_action(
                            &mut action_rows[state_idx],
                            &mut conflicts,
                            state_idx,
                            terminal_col,
                            ActionEntry::shift(next_state),
                            &grammar.symbols,
                        );
                    } else {
                        // GOTO
                        let nt_col = next_symbol.0 - grammar.symbols.num_terminals() - 1;
                        // Check if already exists
                        if !goto_rows[state_idx].iter().any(|(c, _)| *c == nt_col) {
                            goto_rows[state_idx].push((nt_col, next_state as u32));
                        }
                    }
                }
            }
        }

        // Compact the ACTION table using row displacement
        let (action_data, action_base, action_check) =
            Self::compact_table(&action_rows, num_terminals as usize);

        // Compact the GOTO table using row displacement
        let goto_rows_converted: Vec<Vec<(u32, u32)>> = goto_rows;
        let (goto_data, goto_base, goto_check) =
            Self::compact_goto_table(&goto_rows_converted, num_non_terminals as usize);

        // Extract rule info
        let rules = grammar.rules.iter()
            .map(|r| (r.lhs, r.rhs.len() as u8))
            .collect();

        CompactParseTable {
            action_data,
            action_base,
            action_check,
            goto_data,
            goto_base,
            goto_check,
            symbols: grammar.symbols.clone(),
            rules,
            num_states,
            conflicts,
        }
    }

    fn insert_action(
        row: &mut Vec<(u32, ActionEntry)>,
        conflicts: &mut Vec<CompactConflict>,
        state: usize,
        col: u32,
        new_action: ActionEntry,
        symbols: &SymbolTable,
    ) {
        // Check for existing action at this column
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
                            conflicts.push(CompactConflict::ShiftReduce {
                                state,
                                terminal: SymbolId(col),
                                shift_state,
                                reduce_rule,
                            });
                        }
                    }
                    (Action::Reduce(rule1), Action::Reduce(rule2)) => {
                        conflicts.push(CompactConflict::ReduceReduce {
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

    /// Compact a sparse table using row displacement.
    fn compact_table(
        rows: &[Vec<(u32, ActionEntry)>],
        num_cols: usize,
    ) -> (Vec<ActionEntry>, Vec<i32>, Vec<u32>) {
        // Start with a reasonable initial size
        let mut data = vec![ActionEntry::ERROR; num_cols * 2];
        let mut check: Vec<u32> = vec![u32::MAX; num_cols * 2];
        let mut base = vec![0i32; rows.len()];

        for (state, row) in rows.iter().enumerate() {
            if row.is_empty() {
                continue;
            }

            // Find a displacement that doesn't conflict
            let min_col = row.iter().map(|(c, _)| *c).min().unwrap_or(0) as i32;

            let mut displacement = -min_col;
            'search: loop {
                // Check if this displacement works
                let mut ok = true;
                for &(col, _) in row {
                    let idx = (displacement + col as i32) as usize;
                    if idx >= check.len() {
                        // Need to extend
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

            // Place the row at this displacement
            base[state] = displacement;
            for &(col, action) in row {
                let idx = (displacement + col as i32) as usize;
                data[idx] = action;
                check[idx] = state as u32;
            }
        }

        (data, base, check)
    }

    /// Compact the GOTO table.
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

    /// Get the action for a state and terminal. O(1) lookup.
    pub fn action(&self, state: usize, terminal: SymbolId) -> Action {
        let col = terminal.0 as i32;
        let displacement = self.action_base[state];
        let idx = displacement.wrapping_add(col) as usize;

        if idx < self.action_check.len() && self.action_check[idx] == state as u32 {
            self.action_data[idx].decode()
        } else {
            Action::Error
        }
    }

    /// Get the goto state for a state and non-terminal. O(1) lookup.
    pub fn goto(&self, state: usize, non_terminal: SymbolId) -> Option<usize> {
        // non_terminal.0 > num_terminals, so column = id - num_terminals - 1
        let col = (non_terminal.0 - self.symbols.num_terminals() - 1) as i32;
        let displacement = self.goto_base[state];
        let idx = displacement.wrapping_add(col) as usize;

        if idx < self.goto_check.len() && self.goto_check[idx] == state as u32 {
            Some(self.goto_data[idx] as usize)
        } else {
            None
        }
    }

    /// Returns true if the table has conflicts.
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }

    /// Get rule info: (lhs symbol ID, rhs length).
    pub fn rule_info(&self, rule: usize) -> (SymbolId, usize) {
        let (lhs, len) = self.rules[rule];
        (lhs, len as usize)
    }

    /// Lookup symbol ID by name.
    pub fn symbol_id(&self, name: &str) -> Option<SymbolId> {
        self.symbols.get_id(name)
    }

    /// Build from a string-based grammar via InternedAutomaton.
    pub fn from_grammar(grammar: &Grammar) -> Self {
        let automaton = InternedAutomaton::from_grammar(grammar);
        Self::build(&automaton)
    }
}

fn insert_action(
    action: &mut HashMap<(usize, Option<Symbol>), Action>,
    conflicts: &mut Vec<Conflict>,
    key: (usize, Option<Symbol>),
    new_action: Action,
) {
    if let Some(existing) = action.get(&key).cloned() {
        if existing != new_action {
            // Conflict detected - check if it can be resolved by precedence
            let is_prec_terminal = key.1.as_ref().is_some_and(|s| s.is_prec_terminal());

            match (&new_action, &existing) {
                (Action::Shift(shift_state), Action::Reduce(reduce_rule))
                | (Action::Reduce(reduce_rule), Action::Shift(shift_state)) => {
                    if is_prec_terminal {
                        // PrecTerminal: resolve at runtime via precedence
                        action.insert(key, Action::ShiftOrReduce {
                            shift_state: *shift_state,
                            reduce_rule: *reduce_rule,
                        });
                    } else {
                        // Regular terminal: report conflict
                        conflicts.push(Conflict::ShiftReduce {
                            state: key.0,
                            terminal: key.1.clone().unwrap_or_else(|| Symbol::Terminal("$".to_string())),
                            shift_state: *shift_state,
                            reduce_rule: *reduce_rule,
                        });
                    }
                }
                (Action::Reduce(rule1), Action::Reduce(rule2)) => {
                    // Reduce/reduce conflicts are always reported (can't resolve by precedence)
                    conflicts.push(Conflict::ReduceReduce {
                        state: key.0,
                        terminal: key.1.clone().unwrap_or_else(|| Symbol::Terminal("$".to_string())),
                        rule1: *rule1,
                        rule2: *rule2,
                    });
                }
                _ => {} // Same action or Accept, no real conflict
            }
        }
    } else {
        action.insert(key, new_action);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::t;
    use crate::meta::parse_grammar;

    #[test]
    fn test_simple_table() {
        let grammar = parse_grammar("S = 'a' ;").unwrap();
        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        assert!(!table.has_conflicts());

        match table.action(0, Some(&t("a"))) {
            Action::Shift(_) => {}
            other => panic!("Expected Shift, got {:?}", other),
        }
    }

    #[test]
    fn test_expr_table() {
        let grammar = parse_grammar(r#"
            expr = expr '+' term | term ;
            term = 'NUM' ;
        "#).unwrap();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        assert!(!table.has_conflicts(), "Unexpected conflicts: {:?}", table.conflicts);

        match table.action(0, Some(&t("NUM"))) {
            Action::Shift(_) => {}
            other => panic!("Expected Shift on NUM, got {:?}", other),
        }
    }

    #[test]
    fn test_ambiguous_grammar() {
        // expr -> expr + expr | NUM is ambiguous (shift/reduce on +)
        let grammar = parse_grammar(r#"
            expr = expr '+' expr | 'NUM' ;
        "#).unwrap();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        assert!(table.has_conflicts(), "Expected conflicts for ambiguous grammar");

        let has_sr_conflict = table.conflicts.iter().any(|c| {
            matches!(c, Conflict::ShiftReduce { terminal, .. } if terminal == &t("+"))
        });
        assert!(has_sr_conflict, "Expected shift/reduce conflict on +");
    }

    #[test]
    fn test_prec_terminal_no_conflict() {
        // Same ambiguous grammar but with <OP> precedence terminal
        // expr -> expr <OP> expr | NUM
        let grammar = parse_grammar(r#"
            expr = expr <OP> expr | 'NUM' ;
        "#).unwrap();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        // No reported conflicts - ShiftOrReduce is used instead
        assert!(!table.has_conflicts(), "PrecTerminal should not report conflicts: {:?}", table.conflicts);

        // Verify ShiftOrReduce action exists for OP
        let has_shift_or_reduce = table.action.values().any(|a| {
            matches!(a, Action::ShiftOrReduce { .. })
        });
        assert!(has_shift_or_reduce, "Expected ShiftOrReduce action for precedence terminal");
    }

    // Tests for CompactParseTable
    #[test]
    fn test_action_entry_encoding() {
        // Test Shift encoding
        let shift = ActionEntry::shift(42);
        assert_eq!(shift.decode(), Action::Shift(42));

        // Test Reduce encoding
        let reduce = ActionEntry::reduce(7);
        assert_eq!(reduce.decode(), Action::Reduce(7));

        // Test Accept encoding
        let accept = ActionEntry::accept();
        assert_eq!(accept.decode(), Action::Accept);

        // Test Error encoding
        let error = ActionEntry::ERROR;
        assert_eq!(error.decode(), Action::Error);

        // Test ShiftOrReduce encoding
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
    fn test_compact_table_simple() {
        let grammar = parse_grammar("S = 'a' ;").unwrap();
        let table = CompactParseTable::from_grammar(&grammar);

        assert!(!table.has_conflicts());

        let a_id = table.symbol_id("a").unwrap();
        match table.action(0, a_id) {
            Action::Shift(_) => {}
            other => panic!("Expected Shift, got {:?}", other),
        }
    }

    #[test]
    fn test_compact_table_expr() {
        let grammar = parse_grammar(r#"
            expr = expr '+' term | term ;
            term = 'NUM' ;
        "#).unwrap();

        let table = CompactParseTable::from_grammar(&grammar);

        assert!(!table.has_conflicts(), "Unexpected conflicts: {:?}", table.conflicts);

        let num_id = table.symbol_id("NUM").unwrap();
        match table.action(0, num_id) {
            Action::Shift(_) => {}
            other => panic!("Expected Shift on NUM, got {:?}", other),
        }
    }

    #[test]
    fn test_compact_table_same_conflict_status() {
        // Verify both table types agree on conflict status for various grammars
        let grammars = [
            "S = 'a' ;",
            "expr = expr '+' term | term ; term = 'NUM' ;",
            "expr = expr <OP> expr | 'NUM' ;",  // Uses precedence terminals
        ];

        for grammar_str in grammars {
            let grammar = parse_grammar(grammar_str).unwrap();
            let automaton = Automaton::build(&grammar);
            let hash_table = ParseTable::build(&automaton);
            let compact_table = CompactParseTable::from_grammar(&grammar);

            // Both should have the same number of states
            assert_eq!(
                hash_table.num_states,
                compact_table.num_states,
                "State count mismatch for grammar: {}", grammar_str
            );

            // Both should have the same conflict status
            assert_eq!(
                hash_table.has_conflicts(),
                compact_table.has_conflicts(),
                "Conflict status mismatch for grammar: {}", grammar_str
            );
        }
    }

    #[test]
    fn test_compact_table_prec_terminal() {
        let grammar = parse_grammar(r#"
            expr = expr <OP> expr | 'NUM' ;
        "#).unwrap();

        let table = CompactParseTable::from_grammar(&grammar);

        // No reported conflicts
        assert!(!table.has_conflicts(), "Unexpected conflicts: {:?}", table.conflicts);

        // Find state with ShiftOrReduce
        let op_id = table.symbol_id("OP").unwrap();
        let mut found_shift_or_reduce = false;
        for state in 0..table.num_states {
            if let Action::ShiftOrReduce { .. } = table.action(state, op_id) {
                found_shift_or_reduce = true;
                break;
            }
        }
        assert!(found_shift_or_reduce, "Expected ShiftOrReduce action for OP");
    }

    #[test]
    fn test_compact_table_goto() {
        let grammar = parse_grammar(r#"
            expr = expr '+' term | term ;
            term = 'NUM' ;
        "#).unwrap();

        let table = CompactParseTable::from_grammar(&grammar);

        let expr_id = table.symbol_id("expr").unwrap();
        let term_id = table.symbol_id("term").unwrap();

        // State 0 should have goto on expr and term
        assert!(table.goto(0, expr_id).is_some(), "Expected goto on expr from state 0");
        assert!(table.goto(0, term_id).is_some(), "Expected goto on term from state 0");
    }
}

use crate::grammar::{Grammar, SymbolId};
use crate::lr::{GrammarInternal, to_grammar_internal};
use crate::runtime::{OpEntry, ErrorContext, ParseTable};

/// A conflict between two actions in the parse table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Conflict {
    /// Shift/reduce conflict: can either shift the terminal or reduce by a rule.
    ShiftReduce {
        /// Parser state where the conflict occurs.
        state: usize,
        /// Terminal symbol that triggers the conflict.
        terminal: SymbolId,
        /// State to shift to.
        shift_state: usize,
        /// Rule index to reduce by.
        reduce_rule: usize,
    },
    /// Reduce/reduce conflict: can reduce by either of two rules.
    ReduceReduce {
        /// Parser state where the conflict occurs.
        state: usize,
        /// Terminal symbol that triggers the conflict.
        terminal: SymbolId,
        /// First rule index.
        rule1: usize,
        /// Second rule index.
        rule2: usize,
    },
}

/// Grammar metadata for error reporting.
/// Only carries data not available through [`ParseTable`].
#[doc(hidden)]
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
    // ACTION table (row displacement) — stored as raw u32 for OpEntry
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
    pub(crate) num_states: usize,
    /// Conflicts paired with example strings.
    pub(crate) conflicts: Vec<(Conflict, String)>,

    // Error reporting data
    /// Active items (rule, dot) per state.
    state_items: Vec<Vec<(u16, u8)>>,
    /// RHS symbol IDs per rule.
    rule_rhs: Vec<Vec<u32>>,
    /// Accessing symbol for each state.
    state_symbols: Vec<u32>,
}

impl CompiledTable {
    /// Build parse tables from a grammar using the minimal LR(1) pipeline.
    pub fn build(grammar: &Grammar) -> Self {
        let internal = to_grammar_internal(grammar)
            .expect("grammar conversion failed");
        Self::build_from_internal(&internal)
    }

    /// Build parse tables from internal grammar representation using NFA → DFA → Hopcroft.
    pub(crate) fn build_from_internal(grammar: &GrammarInternal) -> Self {
        let result = crate::lr::build_minimal_automaton(grammar);
        let num_terminals = grammar.symbols.num_terminals();
        let num_non_terminals = grammar.symbols.num_non_terminals();

        // Compact the ACTION table
        let (action_data_entries, action_base, action_check) =
            Self::compact_table(&result.action_rows, num_terminals as usize);
        let action_data: Vec<u32> = action_data_entries.iter().map(|e| e.0).collect();

        // Compact the GOTO table
        let (goto_data, goto_base, goto_check) =
            Self::compact_goto_table(&result.goto_rows, num_non_terminals as usize);

        // Extract rule info
        let rules: Vec<(u32, u8)> = grammar.rules.iter()
            .map(|r| (r.lhs.id().0, r.rhs.len() as u8))
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
            num_states: result.num_states,
            conflicts: result.conflicts,
            state_items: result.state_items,
            rule_rhs,
            state_symbols: result.state_symbols,
        }
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

    fn compact_table(
        rows: &[Vec<(u32, OpEntry)>],
        num_cols: usize,
    ) -> (Vec<OpEntry>, Vec<i32>, Vec<u32>) {
        let mut data = vec![OpEntry::ERROR; num_cols * 2];
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
                        data.resize(new_size, OpEntry::ERROR);
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

    /// Format conflicts as human-readable error messages (one string per conflict).
    pub fn format_conflicts(&self) -> Vec<String> {
        self.conflicts.iter().map(|(c, example)| {
            match c {
                Conflict::ShiftReduce { state, terminal, reduce_rule, .. } => {
                    let term_name = self.grammar.symbols.name(*terminal);
                    // Find the item that shifts the conflict terminal and format with dot
                    let shift_item = self.state_items[*state].iter()
                        .find(|&&(rule, dot)| {
                            let rhs = &self.rule_rhs[rule as usize];
                            (dot as usize) < rhs.len() && rhs[dot as usize] == terminal.0
                        })
                        .map(|&(rule, dot)| self.format_item(rule as usize, dot as usize))
                        .unwrap_or_else(|| "?".to_string());
                    let reduce_item = self.format_item(*reduce_rule, self.rule_rhs[*reduce_rule].len());
                    let mut msg = format!(
                        "Shift/reduce conflict on '{}':\n  \
                         Shift:  {} (wins)\n  \
                         Reduce: {}",
                        term_name, shift_item, reduce_item,
                    );
                    if !example.is_empty() {
                        msg.push_str(&format!("\n  {}", example));
                    }
                    msg
                }
                Conflict::ReduceReduce { state: _, terminal, rule1, rule2 } => {
                    let term_name = self.grammar.symbols.name(*terminal);
                    let item1 = self.format_item(*rule1, self.rule_rhs[*rule1].len());
                    let item2 = self.format_item(*rule2, self.rule_rhs[*rule2].len());
                    let mut msg = format!(
                        "Reduce/reduce conflict on '{}':\n  \
                         Reduce: {} (wins)\n  \
                         Reduce: {}",
                        term_name, item1, item2,
                    );
                    if !example.is_empty() {
                        msg.push_str(&format!("\n  {}", example));
                    }
                    msg
                }
            }
        }).collect()
    }

    /// Format an item as "lhs -> rhs1 rhs2 • rhs3 ..."
    fn format_item(&self, rule_idx: usize, dot: usize) -> String {
        let rule = &self.grammar.rules[rule_idx];
        let lhs_name = self.grammar.symbols.name(rule.lhs.id());
        let rhs = &self.rule_rhs[rule_idx];
        let mut s = format!("{} ->", lhs_name);
        for (i, &sym_id) in rhs.iter().enumerate() {
            if i == dot { s.push_str(" \u{2022}"); }
            s.push(' ');
            s.push_str(self.grammar.symbols.name(SymbolId(sym_id)));
        }
        if dot == rhs.len() { s.push_str(" \u{2022}"); }
        s
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

    /// Get the number of parser states.
    pub fn num_states(&self) -> usize {
        self.num_states
    }

    /// Get the conflicts detected during table construction, paired with examples.
    pub fn conflicts(&self) -> &[(Conflict, String)] {
        &self.conflicts
    }

    // Accessors for compressed table arrays (for codegen/serialization)

    #[doc(hidden)]
    pub fn action_data(&self) -> &[u32] {
        &self.action_data
    }

    #[doc(hidden)]
    pub fn action_base(&self) -> &[i32] {
        &self.action_base
    }

    #[doc(hidden)]
    pub fn action_check(&self) -> &[u32] {
        &self.action_check
    }

    #[doc(hidden)]
    pub fn goto_data(&self) -> &[u32] {
        &self.goto_data
    }

    #[doc(hidden)]
    pub fn goto_base(&self) -> &[i32] {
        &self.goto_base
    }

    #[doc(hidden)]
    pub fn goto_check(&self) -> &[u32] {
        &self.goto_check
    }

    #[doc(hidden)]
    pub fn rules(&self) -> &[(u32, u8)] {
        &self.rules
    }

    #[doc(hidden)]
    pub fn state_items(&self) -> &[Vec<(u16, u8)>] {
        &self.state_items
    }

    #[doc(hidden)]
    pub fn rule_rhs(&self) -> &[Vec<u32>] {
        &self.rule_rhs
    }

    #[doc(hidden)]
    pub fn rule_name(&self, rule: usize) -> Option<&str> {
        self.grammar.rules.get(rule).and_then(|r| {
            if let crate::lr::AltAction::Named(name) = &r.action {
                Some(name.as_str())
            } else {
                None
            }
        })
    }

    #[doc(hidden)]
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
    use crate::runtime::ParserOp;
    use crate::meta::parse_grammar;
    use crate::lr::to_grammar_internal;

    fn simple_grammar() -> GrammarInternal {
        to_grammar_internal(&parse_grammar(r#"
            start s; terminals { a } s = a => a;
        "#).unwrap()).unwrap()
    }

    fn expr_grammar() -> GrammarInternal {
        to_grammar_internal(&parse_grammar(r#"
            start expr;
            terminals { PLUS, NUM }
            expr = expr PLUS term => add | term => term;
            term = NUM => num;
        "#).unwrap()).unwrap()
    }

    fn ambiguous_grammar() -> GrammarInternal {
        to_grammar_internal(&parse_grammar(r#"
            start expr;
            terminals { PLUS, NUM }
            expr = expr PLUS expr => add | NUM => num;
        "#).unwrap()).unwrap()
    }

    fn prec_grammar() -> GrammarInternal {
        to_grammar_internal(&parse_grammar(r#"
            start expr;
            terminals { prec OP, NUM }
            expr = expr OP expr => binop | NUM => num;
        "#).unwrap()).unwrap()
    }

    #[test]
    fn test_simple_table() {
        let grammar = simple_grammar();
        let compiled = CompiledTable::build_from_internal(&grammar);
        let table = compiled.table();

        assert!(!compiled.has_conflicts());

        let a_id = compiled.symbol_id("a").unwrap();
        match table.action(0, a_id) {
            ParserOp::Shift(_) => {}
            other => panic!("Expected Shift, got {:?}", other),
        }
    }

    #[test]
    fn test_expr_table() {
        let grammar = expr_grammar();
        let compiled = CompiledTable::build_from_internal(&grammar);
        let table = compiled.table();

        assert!(!compiled.has_conflicts());

        let num_id = compiled.symbol_id("NUM").unwrap();
        match table.action(0, num_id) {
            ParserOp::Shift(_) => {}
            other => panic!("Expected Shift on NUM, got {:?}", other),
        }
    }

    #[test]
    fn test_ambiguous_grammar() {
        let grammar = ambiguous_grammar();
        let compiled = CompiledTable::build_from_internal(&grammar);

        assert!(compiled.has_conflicts(), "Expected conflicts for ambiguous grammar");

        let has_sr_conflict = compiled.conflicts.iter().any(|(c, _)| {
            matches!(c, Conflict::ShiftReduce { .. })
        });
        assert!(has_sr_conflict, "Expected shift/reduce conflict");

        // Test conflict formatting (includes examples inline)
        let messages = compiled.format_conflicts();
        assert!(!messages.is_empty(), "Expected formatted conflict messages");
        let msg = &messages[0];
        assert!(msg.contains("Shift/reduce conflict"), "Should describe conflict type: {}", msg);
        assert!(msg.contains("'PLUS'"), "Should mention the terminal: {}", msg);
        // Should contain items with dots
        assert!(msg.contains("\u{2022}"), "Should contain dot in item: {}", msg);
        // Should contain example inline
        assert!(msg.contains("Example:"), "Should contain example: {}", msg);
        assert!(msg.contains("expr"), "Should mention expr: {}", msg);
    }

    #[test]
    fn test_conflict_example_rr() {
        let grammar = to_grammar_internal(&parse_grammar(r#"
            start s;
            terminals { A }
            s = x => x | y => y;
            x = A => a;
            y = A => a;
        "#).unwrap()).unwrap();
        let compiled = CompiledTable::build_from_internal(&grammar);

        assert!(compiled.has_conflicts(), "Expected R/R conflict");
        let has_rr = compiled.conflicts.iter().any(|(c, _)| matches!(c, Conflict::ReduceReduce { .. }));
        assert!(has_rr, "Expected reduce/reduce conflict");

        // Examples are now inline in format_conflicts
        let messages = compiled.format_conflicts();
        let msg = &messages[0];
        assert!(msg.contains("Reduce/reduce conflict"), "Should describe R/R: {}", msg);
        assert!(msg.contains("Example:"), "Should contain example: {}", msg);
    }

    #[test]
    fn test_no_conflict_examples_for_clean_grammar() {
        let grammar = expr_grammar();
        let compiled = CompiledTable::build_from_internal(&grammar);
        assert!(!compiled.has_conflicts());
        assert!(compiled.conflicts().is_empty());
    }

    #[test]
    fn test_prec_terminal_no_conflict() {
        let grammar = prec_grammar();
        let compiled = CompiledTable::build_from_internal(&grammar);
        let table = compiled.table();

        assert!(!compiled.has_conflicts(), "PrecTerminal should not report conflicts");

        // Find state with ShiftOrReduce
        let op_id = compiled.symbol_id("OP").unwrap();
        let mut found_shift_or_reduce = false;
        for state in 0..compiled.num_states {
            if let ParserOp::ShiftOrReduce { .. } = table.action(state, op_id) {
                found_shift_or_reduce = true;
                break;
            }
        }
        assert!(found_shift_or_reduce, "Expected ShiftOrReduce action for OP");
    }

    #[test]
    fn test_goto() {
        let grammar = expr_grammar();
        let compiled = CompiledTable::build_from_internal(&grammar);
        let table = compiled.table();

        let expr_id = compiled.symbol_id("expr").unwrap();
        let term_id = compiled.symbol_id("term").unwrap();

        assert!(table.goto(0, expr_id).is_some(), "Expected goto on expr from state 0");
        assert!(table.goto(0, term_id).is_some(), "Expected goto on term from state 0");
    }
}

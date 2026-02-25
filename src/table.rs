use std::collections::{BTreeMap, HashMap};

use crate::grammar::{Grammar, SymbolId};
use crate::lr::{GrammarInternal, to_grammar_internal};
use crate::runtime::{ErrorContext, OpEntry, ParseTable};

/// A conflict between two actions in the parse table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Conflict {
    ShiftReduce {
        terminal: SymbolId,
        reduce_rule: usize,
        example: String,
    },
    ReduceReduce {
        terminal: SymbolId,
        rule1: usize,
        rule2: usize,
        example: String,
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
    // Shared data/check arrays (bison-style: action + goto share the same table)
    data: Vec<u32>,
    check: Vec<u32>,
    // Separate base arrays
    action_base: Vec<i32>,  // indexed by state
    goto_base: Vec<i32>,    // indexed by non-terminal

    /// Rules: (lhs_id, rhs_len) for each rule.
    rules: Vec<(u32, u8)>,

    /// Number of terminals (including EOF) for goto default indexing.
    num_terminals: u32,

    /// The augmented grammar.
    pub(crate) grammar: GrammarInternal,
    /// Number of states.
    pub(crate) num_states: usize,
    /// Conflicts paired with example strings.
    pub(crate) conflicts: Vec<Conflict>,

    // Error reporting data
    /// Active items (rule, dot) per state.
    state_items: Vec<Vec<(u16, u8)>>,
    /// RHS symbol IDs per rule.
    rule_rhs: Vec<Vec<u32>>,
    /// Accessing symbol for each state.
    state_symbols: Vec<u32>,
    /// Default reduce rule per state (0 = no default).
    default_reduce: Vec<u32>,
    /// Default goto target per non-terminal (u32::MAX = no default).
    default_goto: Vec<u32>,
}

/// Return the most frequent value, or u32::MAX if empty.
fn most_frequent(iter: impl Iterator<Item = u32>) -> u32 {
    let mut counts: BTreeMap<u32, usize> = BTreeMap::new();
    for v in iter {
        *counts.entry(v).or_default() += 1;
    }
    counts.into_iter().max_by_key(|&(_, c)| c).map(|(v, _)| v).unwrap_or(u32::MAX)
}

/// Pack rows of (col, value) into shared data/check arrays.
/// Returns `(data, check, bases)` where `bases[i]` is the displacement for row `i`.
/// Identical rows share the same base (row deduplication).
fn compact_rows(rows: &[Vec<(u32, u32)>]) -> (Vec<u32>, Vec<u32>, Vec<i32>) {
    let mut bases = vec![0i32; rows.len()];

    // Dedup: identical rows share the same base.
    let mut dedup: HashMap<Vec<(u32, u32)>, Vec<usize>> = HashMap::new();
    for (i, row) in rows.iter().enumerate() {
        dedup.entry(row.clone()).or_default().push(i);
    }

    let mut unique_rows: Vec<(Vec<(u32, u32)>, Vec<usize>)> = dedup.into_iter().collect();
    unique_rows.sort_by(|a, b| {
        b.0.len().cmp(&a.0.len())
            .then_with(|| a.1[0].cmp(&b.1[0]))
    });

    let init_size = rows.len() * 2;
    let mut data = vec![0u32; init_size];
    let mut check: Vec<u32> = vec![u32::MAX; init_size];
    let mut used_bases = std::collections::HashSet::new();

    for (row, members) in &unique_rows {
        if row.is_empty() {
            for &idx in members {
                let mut displacement = 0i32;
                while !used_bases.insert(displacement) {
                    displacement += 1;
                }
                bases[idx] = displacement;
            }
            continue;
        }

        let min_col = row.iter().map(|(c, _)| *c).min().unwrap_or(0) as i32;

        let mut displacement = -min_col;
        loop {
            if !used_bases.contains(&displacement) {
                let mut ok = true;
                for &(col, _) in row {
                    let slot = (displacement + col as i32) as usize;
                    if slot >= check.len() {
                        let new_size = (slot + 1).max(data.len() * 2);
                        data.resize(new_size, 0);
                        check.resize(new_size, u32::MAX);
                    }
                    if check[slot] != u32::MAX {
                        ok = false;
                        break;
                    }
                }
                if ok { break; }
            }
            displacement += 1;
        }

        used_bases.insert(displacement);
        for &(col, value) in row {
            let slot = (displacement + col as i32) as usize;
            data[slot] = value;
            check[slot] = col;
        }
        for &idx in members {
            bases[idx] = displacement;
        }
    }

    (data, check, bases)
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
        let num_item_states = result.num_item_states;
        let num_non_terminals = grammar.symbols.num_non_terminals() as usize;

        // Classify each DFA transition from item states.
        // For each item state: collect reduce rules (for default_reduce).
        // For each non-terminal: collect goto targets (for default_goto).
        let mut reduce_rules_per_state: Vec<Vec<u32>> = vec![Vec::new(); num_item_states];
        let mut goto_targets_per_nt: Vec<Vec<u32>> = vec![Vec::new(); num_non_terminals];

        for state in 0..num_item_states {
            for &(sym, target) in &result.dfa.transitions[state] {
                if result.reduce_to_real.contains_key(&sym) {
                    continue;
                }
                if sym < num_terminals && target >= num_item_states {
                    reduce_rules_per_state[state].push((target - num_item_states) as u32);
                } else if sym >= num_terminals && sym < grammar.symbols.num_symbols() && target < num_item_states {
                    let nt_idx = (sym - num_terminals) as usize;
                    goto_targets_per_nt[nt_idx].push(target as u32);
                }
            }
        }

        // Default reduce: most frequent reduce rule per state (skip accept = rule 0).
        let default_reduce: Vec<u32> = reduce_rules_per_state.iter().map(|rules| {
            let default = most_frequent(rules.iter().filter(|&&r| r > 0).copied());
            if default != u32::MAX { default } else { 0 }
        }).collect();

        // Default goto: most frequent target per non-terminal.
        let default_goto: Vec<u32> = goto_targets_per_nt.iter().map(|targets| {
            most_frequent(targets.iter().copied())
        }).collect();

        // State symbols: for each item state, what symbol reaches it.
        let mut state_symbols = vec![0u32; num_item_states];
        for state in 0..num_item_states {
            for &(sym, target) in &result.dfa.transitions[state] {
                if target < num_item_states {
                    state_symbols[target] = sym;
                }
            }
        }

        // Reverse map: real terminal → virtual reduce symbol.
        let mut real_to_virtual: HashMap<u32, u32> = HashMap::new();
        for (&virtual_id, &real_id) in &result.reduce_to_real {
            real_to_virtual.insert(real_id, virtual_id);
        }

        // Helper: look up transition target by symbol in a state.
        let find_target = |state: usize, sym: u32| -> Option<usize> {
            result.dfa.transitions[state].iter()
                .find(|&&(s, _)| s == sym)
                .map(|&(_, t)| t)
        };

        // Build rows: action rows (indexed by state), then goto rows (indexed by non-terminal).
        let mut rows: Vec<Vec<(u32, u32)>> = Vec::with_capacity(num_item_states + num_non_terminals);

        for state in 0..num_item_states {
            let mut row: Vec<(u32, u32)> = Vec::new();
            let dr = default_reduce[state];

            for sym in grammar.symbols.terminal_ids() {
                let shift = find_target(state, sym.0)
                    .filter(|&t| t < num_item_states);
                let reduce = if let Some(&virtual_id) = real_to_virtual.get(&sym.0) {
                    // Prec terminal: reduce comes from virtual symbol transition.
                    find_target(state, virtual_id)
                        .filter(|&t| t >= num_item_states)
                        .map(|t| t - num_item_states)
                } else {
                    // Non-prec: reduce comes from the same terminal's transition.
                    find_target(state, sym.0)
                        .filter(|&t| t >= num_item_states)
                        .map(|t| t - num_item_states)
                };

                let entry = match (shift, reduce) {
                    (Some(s), Some(r)) => OpEntry::shift_or_reduce(s, r),
                    (Some(s), None) => OpEntry::shift(s),
                    (None, Some(r)) => {
                        if dr > 0 && r as u32 == dr { continue; }
                        OpEntry::reduce(r)
                    }
                    (None, None) => continue,
                };
                row.push((sym.0, entry.0));
            }

            rows.push(row);
        }

        // Goto rows (transposed: indexed by non-terminal, col = state).
        for nt_idx in 0..num_non_terminals {
            let mut row: Vec<(u32, u32)> = Vec::new();
            let sym = num_terminals + nt_idx as u32;
            for state in 0..num_item_states {
                if let Some(target) = find_target(state, sym) {
                    if target < num_item_states && target as u32 != default_goto[nt_idx] {
                        row.push((state as u32, target as u32));
                    }
                }
            }
            rows.push(row);
        }

        let (data, check, bases) = compact_rows(&rows);
        let (action_base, goto_base) = bases.split_at(num_item_states);

        let rules: Vec<(u32, u8)> = grammar.rules.iter()
            .map(|r| (r.lhs.id().0, r.rhs.len() as u8))
            .collect();

        let rule_rhs: Vec<Vec<u32>> = grammar.rules.iter()
            .map(|r| r.rhs.iter().map(|s| s.id().0).collect())
            .collect();

        CompiledTable {
            data,
            check,
            action_base: action_base.to_vec(),
            goto_base: goto_base.to_vec(),
            num_terminals: grammar.symbols.num_terminals(),
            grammar: grammar.clone(),
            rules,
            num_states: num_item_states,
            conflicts: result.conflicts,
            state_items: result.state_items,
            rule_rhs,
            state_symbols,
            default_reduce,
            default_goto,
        }
    }

    /// Get a lightweight [`ParseTable`] borrowing from this compiled table.
    pub fn table(&self) -> ParseTable<'_> {
        ParseTable::new(
            &self.data,
            &self.check,
            &self.action_base,
            &self.goto_base,
            &self.rules,
            self.num_terminals,
            &self.default_reduce,
            &self.default_goto,
        )
    }

    /// Returns true if the table has conflicts.
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }

    /// Format conflicts as human-readable error messages (one string per conflict).
    pub fn format_conflicts(&self) -> Vec<String> {
        self.conflicts.iter().map(|c| {
            match c {
                Conflict::ShiftReduce { terminal, reduce_rule, example } => {
                    let term_name = self.grammar.symbols.name(*terminal);
                    let reduce_item = self.format_item(*reduce_rule, self.rule_rhs[*reduce_rule].len());
                    let mut msg = format!(
                        "Shift/reduce conflict on '{}':\n  \
                         Shift wins over: {}",
                        term_name, reduce_item,
                    );
                    if !example.is_empty() {
                        msg.push_str(&format!("\n  {}", example));
                    }
                    msg
                }
                Conflict::ReduceReduce { terminal, rule1, rule2, example } => {
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
    pub fn conflicts(&self) -> &[Conflict] {
        &self.conflicts
    }

    // Accessors for compressed table arrays (for codegen/serialization)

    #[doc(hidden)]
    pub fn table_data(&self) -> &[u32] {
        &self.data
    }

    #[doc(hidden)]
    pub fn table_check(&self) -> &[u32] {
        &self.check
    }

    #[doc(hidden)]
    pub fn action_base(&self) -> &[i32] {
        &self.action_base
    }

    #[doc(hidden)]
    pub fn goto_base(&self) -> &[i32] {
        &self.goto_base
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

    #[doc(hidden)]
    pub fn default_reduce(&self) -> &[u32] {
        &self.default_reduce
    }

    #[doc(hidden)]
    pub fn default_goto(&self) -> &[u32] {
        &self.default_goto
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

        let has_sr_conflict = compiled.conflicts.iter().any(|c| {
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
        let has_rr = compiled.conflicts.iter().any(|c| matches!(c, Conflict::ReduceReduce { .. }));
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

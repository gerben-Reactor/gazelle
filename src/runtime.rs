use crate::grammar::{Precedence, SymbolId};
use crate::table::{Action, ErrorContext, ParseTable};
use std::collections::{HashMap, HashSet};

/// Compute FIRST sets from ErrorContext (on demand for error reporting).
fn compute_first_sets(ctx: &impl ErrorContext) -> Vec<HashSet<u32>> {
    let num_terminals = ctx.num_terminals();
    let num_rules = ctx.num_rules();

    // Find max symbol ID by scanning rules
    let mut max_sym = num_terminals;
    for rule in 0..num_rules {
        let lhs = ctx.rule_lhs(rule).0 as usize;
        if lhs >= max_sym {
            max_sym = lhs + 1;
        }
        for sym in ctx.rule_rhs(rule) {
            let id = sym.0 as usize;
            if id >= max_sym {
                max_sym = id + 1;
            }
        }
    }

    let mut sets: Vec<HashSet<u32>> = (0..max_sym).map(|_| HashSet::new()).collect();
    let mut nullable: Vec<bool> = vec![false; max_sym];

    // Terminals have FIRST = {self}
    for t in 0..num_terminals {
        sets[t].insert(t as u32);
    }

    // Fixed-point iteration
    let mut changed = true;
    while changed {
        changed = false;

        for rule in 0..num_rules {
            let lhs = ctx.rule_lhs(rule).0 as usize;
            let rhs = ctx.rule_rhs(rule);

            // Compute FIRST of this RHS
            let mut all_nullable = true;
            for sym in &rhs {
                let id = sym.0 as usize;
                // Add FIRST(sym) to FIRST(lhs)
                let to_add: Vec<_> = sets[id].iter().copied().collect();
                for t in to_add {
                    if sets[lhs].insert(t) {
                        changed = true;
                    }
                }
                if !nullable[id] {
                    all_nullable = false;
                    break;
                }
            }

            // If RHS is empty or all nullable, LHS is nullable
            if all_nullable && !nullable[lhs] {
                nullable[lhs] = true;
                changed = true;
            }
        }
    }

    sets
}

/// Parse error with full context.
#[derive(Debug, Clone)]
pub struct ParseError {
    terminal: SymbolId,
    stack: Vec<StackEntry>,
    /// Token index where the error occurred.
    error_token_idx: usize,
}

impl ParseError {
    /// The unexpected terminal that caused the error.
    pub fn terminal(&self) -> SymbolId {
        self.terminal
    }

    /// The parser state at the error (for looking up expected terminals).
    pub fn state(&self) -> usize {
        self.stack.last().unwrap().state
    }

    /// Token index where the error occurred.
    ///
    /// Tokens `0..error_token_idx()` were successfully parsed.
    pub fn error_token_idx(&self) -> usize {
        self.error_token_idx
    }

    /// Stack entries as (start_token_idx, end_token_idx, state).
    ///
    /// Each entry spans tokens `start..end`. Use `ctx.state_symbol(state)`
    /// to get the non-terminal for display.
    pub fn stack_spans(&self) -> Vec<(usize, usize, usize)> {
        let mut spans = Vec::with_capacity(self.stack.len());
        for i in 0..self.stack.len() {
            let start = self.stack[i].token_idx;
            let end = if i + 1 < self.stack.len() {
                self.stack[i + 1].token_idx
            } else {
                self.error_token_idx
            };
            spans.push((start, end, self.stack[i].state));
        }
        spans
    }

    /// Find states with incomplete items by following reductions.
    ///
    /// If the current state only has completed items, simulate reductions
    /// and GOTO to find states that explain what's expected.
    fn find_incomplete_items_states(&self, ctx: &impl ErrorContext, state: usize) -> HashSet<usize> {
        let mut result = HashSet::new();
        let items = ctx.state_items(state);

        // Check if there are any incomplete items
        let has_incomplete = items.iter().any(|&(rule, dot)| {
            let rhs = ctx.rule_rhs(rule);
            dot < rhs.len()
        });

        if has_incomplete {
            result.insert(state);
            return result;
        }

        // All items complete - follow all reductions
        for &(rule, _) in &items {
            let lhs = ctx.rule_lhs(rule);
            let rhs_len = ctx.rule_rhs(rule).len();

            // Find the state we'd be in after popping rhs_len entries
            if self.stack.len() > rhs_len {
                let from_state = self.stack[self.stack.len() - rhs_len - 1].state;
                if let Some(goto_state) = ctx.goto(from_state, lhs) {
                    result.insert(goto_state);
                }
            }
        }

        // If we found nothing, return the original state
        if result.is_empty() {
            result.insert(state);
        }
        result
    }

    /// Format the error using the provided error context.
    pub fn format(&self, ctx: &impl ErrorContext) -> String {
        self.format_with(ctx, &HashMap::new(), &[])
    }

    /// Format with display names and token texts.
    ///
    /// - `display_names`: maps grammar names to user-friendly names (e.g., "SEMI" → "';'")
    /// - `tokens`: token texts by index (must include error token at index `error_token_idx()`)
    pub fn format_with(
        &self,
        ctx: &impl ErrorContext,
        display_names: &HashMap<&str, &str>,
        tokens: &[&str],
    ) -> String {
        let state = self.state();

        let display = |id: SymbolId| -> &str {
            let name = ctx.symbol_name(id);
            display_names.get(name).copied().unwrap_or(name)
        };

        // Find states with incomplete items (follow reductions if needed)
        let items_states = self.find_incomplete_items_states(ctx, state);
        let first_sets = compute_first_sets(ctx);
        let num_terminals = ctx.num_terminals();

        // Compute expected from incomplete items' next symbols using FIRST sets
        let mut expected_set = HashSet::new();
        for &items_state in &items_states {
            for (rule, dot) in ctx.state_items(items_state) {
                let rhs = ctx.rule_rhs(rule);
                if dot < rhs.len() {
                    let next_sym = rhs[dot].0 as usize;
                    // Add FIRST(next_sym)
                    if let Some(first) = first_sets.get(next_sym) {
                        for &t in first {
                            if (t as usize) < num_terminals {
                                expected_set.insert(display(SymbolId(t)));
                            }
                        }
                    }
                }
            }
        }

        let mut expected: Vec<_> = expected_set.into_iter().collect();
        expected.sort();

        // Show actual token text if available, otherwise display name
        let found_name = tokens.get(self.error_token_idx)
            .copied()
            .unwrap_or_else(|| display(self.terminal));

        let mut msg = format!("unexpected '{}'", found_name);
        if !expected.is_empty() {
            msg.push_str(&format!(", expected: {}", expected.join(", ")));
        }

        // Show parsed stack with token spans
        if !tokens.is_empty() && self.error_token_idx <= tokens.len() {
            let spans = self.stack_spans();
            // Skip state 0 (initial), show recent entries
            let relevant: Vec<_> = spans.into_iter()
                .skip(1)  // skip initial state
                .filter(|(start, end, _)| end > start)  // skip empty spans
                .collect();

            if !relevant.is_empty() {
                // Build two lines: tokens and underlines with names
                let mut token_line = String::new();
                let mut label_line = String::new();

                for (start, end, state) in relevant.iter().rev().take(4).rev() {
                    let sym = ctx.state_symbol(*state);
                    let name = display(sym);

                    // Get token text for this span
                    let span_text = if end - start == 1 {
                        tokens[*start].to_string()
                    } else if end - start <= 3 {
                        tokens[*start..*end].join(" ")
                    } else {
                        format!("{} ... {}", tokens[*start], tokens[end - 1])
                    };

                    let width = span_text.chars().count().max(name.len());

                    if !token_line.is_empty() {
                        token_line.push_str("  ");
                        label_line.push_str("  ");
                    }
                    token_line.push_str(&format!("{:^width$}", span_text, width = width));
                    label_line.push_str(&format!("{:^width$}", name, width = width));
                }

                msg.push_str(&format!("\n  {}\n  {}", token_line, label_line));
            }
        } else if self.stack.len() > 1 {
            // Fallback: show grammar symbols from stack
            let path: Vec<_> = self.stack[1..]
                .iter()
                .map(|e| display(ctx.state_symbol(e.state)))
                .collect();
            msg.push_str(&format!("\n  after: {}", path.join(" ")));
        }

        // Show incomplete items that explain what's expected
        let mut seen = HashSet::new();

        for &items_state in &items_states {
        for (rule, dot) in ctx.state_items(items_state) {
            let rhs = ctx.rule_rhs(rule);

            // Skip completed items
            if dot >= rhs.len() {
                continue;
            }

            let lhs = ctx.rule_lhs(rule);
            let lhs_name = display(lhs);

            // Skip internal generated rules (start with __)
            if lhs_name.starts_with("__") {
                continue;
            }

            // Convert __ generated names back to modifier syntax
            let format_sym = |s: &str| -> String {
                if let Some(base) = s.strip_prefix("__").and_then(|s| s.strip_suffix("_star")) {
                    format!("{}*", base)
                } else if let Some(base) = s.strip_prefix("__").and_then(|s| s.strip_suffix("_plus")) {
                    format!("{}+", base)
                } else if let Some(base) = s.strip_prefix("__").and_then(|s| s.strip_suffix("_opt")) {
                    format!("{}?", base)
                } else {
                    s.to_string()
                }
            };

            let before: Vec<_> = rhs[..dot]
                .iter()
                .map(|&id| format_sym(display(id)))
                .collect();
            let after: Vec<_> = rhs[dot..]
                .iter()
                .map(|&id| format_sym(display(id)))
                .collect();
            let line = format!(
                "\n  in {}: {} \u{2022} {}",
                lhs_name,
                before.join(" "),
                after.join(" ")
            );
            if seen.insert(line.clone()) {
                msg.push_str(&line);
            }
        }
        }
        msg
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unexpected terminal {:?}", self.terminal)
    }
}

impl std::error::Error for ParseError {}

/// A token with terminal symbol ID and optional precedence.
#[derive(Debug, Clone)]
pub struct Token {
    pub terminal: SymbolId,
    pub prec: Option<Precedence>,
}

impl Token {
    pub fn new(terminal: SymbolId) -> Self {
        Self { terminal, prec: None }
    }

    pub fn with_prec(terminal: SymbolId, prec: Precedence) -> Self {
        Self { terminal, prec: Some(prec) }
    }
}

/// Stack entry for the parser.
#[derive(Debug, Clone, Copy)]
struct StackEntry {
    state: usize,
    prec: Option<Precedence>,
    /// Start token index for this subtree (for span tracking).
    token_idx: usize,
}

/// An LR parser.
pub struct Parser<'a> {
    table: ParseTable<'a>,
    /// Current state (top of stack, kept in "register").
    state: StackEntry,
    /// Previous states (rest of stack).
    stack: Vec<StackEntry>,
    /// Count of tokens shifted (for span tracking).
    token_count: usize,
}

impl<'a> Parser<'a> {
    /// Create a new parser with the given parse table.
    pub fn new(table: ParseTable<'a>) -> Self {
        Self {
            table,
            state: StackEntry { state: 0, prec: None, token_idx: 0 },
            stack: Vec::new(),
            token_count: 0,
        }
    }

    /// Check if a reduction should happen for the given lookahead.
    ///
    /// Returns `Ok(Some((rule, len, start_idx)))` if a reduction should occur.
    /// The `start_idx` together with `token_count()` forms the half-open range `[start_idx, token_count())`.
    /// Returns `Ok(None)` if should shift or if accepted.
    /// Returns `Err(ParseError)` on parse error.
    pub fn maybe_reduce(&mut self, lookahead: Option<&Token>) -> Result<Option<(usize, usize, usize)>, ParseError> {
        let terminal = lookahead.map(|t| t.terminal).unwrap_or(SymbolId::EOF);
        let lookahead_prec = lookahead.and_then(|t| t.prec);

        match self.table.action(self.state.state, terminal) {
            Action::Reduce(rule) => {
                if rule == 0 {
                    Ok(Some((0, 0, 0))) // Accept
                } else {
                    let (len, start_idx) = self.do_reduce(rule);
                    Ok(Some((rule, len, start_idx)))
                }
            }
            Action::Shift(_) => Ok(None),
            Action::ShiftOrReduce { reduce_rule, .. } => {
                let should_reduce = match (self.state.prec, lookahead_prec) {
                    (Some(sp), Some(tp)) => {
                        if tp.level() > sp.level() {
                            false
                        } else if tp.level() < sp.level() {
                            true
                        } else {
                            matches!(sp, Precedence::Left(_))
                        }
                    }
                    _ => false,
                };

                if should_reduce {
                    let (len, start_idx) = self.do_reduce(reduce_rule);
                    Ok(Some((reduce_rule, len, start_idx)))
                } else {
                    Ok(None)
                }
            }
            Action::Error => {
                // Reconstruct full stack for error reporting
                let mut full_stack = self.stack.clone();
                full_stack.push(self.state);
                Err(ParseError {
                    terminal,
                    stack: full_stack,
                    error_token_idx: self.token_count,
                })
            }
        }
    }

    /// Shift a token onto the stack.
    pub fn shift(&mut self, token: &Token) {
        let next_state = match self.table.action(self.state.state, token.terminal) {
            Action::Shift(s) => s,
            Action::ShiftOrReduce { shift_state, .. } => shift_state,
            _ => panic!("shift called when action is not shift"),
        };

        let prec = token.prec.or(self.state.prec);
        self.stack.push(self.state);
        self.state = StackEntry {
            state: next_state,
            prec,
            token_idx: self.token_count,
        };
        self.token_count += 1;
    }

    fn do_reduce(&mut self, rule: usize) -> (usize, usize) {
        let (lhs, len) = self.table.rule_info(rule);

        // Compute start token index for this reduction
        let start_idx = match len {
            0 => self.token_count,  // epsilon: empty range at current position
            1 => self.state.token_idx,  // single symbol in register
            _ => self.stack[self.stack.len() - len + 1].token_idx,  // first symbol in stack
        };

        if len == 0 {
            // Epsilon: anchor is current state, push it, then set new state
            if let Some(next_state) = self.table.goto(self.state.state, lhs) {
                self.stack.push(self.state);
                self.state = StackEntry { state: next_state, prec: None, token_idx: start_idx };
            }
        } else {
            // Non-epsilon: capture prec from current state, pop len-1, goto from stack.last()
            let captured_prec = self.state.prec;
            for _ in 0..(len - 1) {
                self.stack.pop();
            }
            let anchor = self.stack.last().unwrap().state;
            if let Some(next_state) = self.table.goto(anchor, lhs) {
                self.state = StackEntry { state: next_state, prec: captured_prec, token_idx: start_idx };
            }
        }

        (len, start_idx)
    }

    /// Get the current state.
    pub fn state(&self) -> usize {
        self.state.state
    }

    /// Get the number of values on the stack.
    pub fn stack_depth(&self) -> usize {
        self.stack.len()
    }

    /// Get the count of tokens shifted so far.
    pub fn token_count(&self) -> usize {
        self.token_count
    }

    /// Get the state at a given depth (0 = bottom of value stack).
    pub fn state_at(&self, depth: usize) -> usize {
        let idx = depth + 1;
        if idx < self.stack.len() {
            self.stack[idx].state
        } else {
            self.state.state
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::GrammarBuilder;
    use crate::table::CompiledTable;

    #[test]
    fn test_parse_single_token() {
        let mut gb = GrammarBuilder::new();
        let a = gb.t("a");
        let s = gb.nt("S");
        gb.rule(s, vec![a]);
        let grammar = gb.build();

        let compiled = CompiledTable::build(&grammar);
        let mut parser = Parser::new(compiled.table());

        let a_id = compiled.symbol_id("a").unwrap();
        let token = Token::new(a_id);

        // Should not reduce before shifting
        assert!(matches!(parser.maybe_reduce(Some(&token)), Ok(None)));

        // Shift the token
        parser.shift(&token);

        // Now reduce with EOF lookahead
        let result = parser.maybe_reduce(None);
        assert!(matches!(result, Ok(Some((1, 1, 0))))); // rule 1, len 1, start_idx 0

        // Should be accepted now (rule 0)
        let result = parser.maybe_reduce(None);
        assert!(matches!(result, Ok(Some((0, 0, 0)))));
    }

    #[test]
    fn test_parse_error() {
        let mut gb = GrammarBuilder::new();
        let a = gb.t("a");
        let s = gb.nt("S");
        gb.rule(s, vec![a]);
        let grammar = gb.build();

        let compiled = CompiledTable::build(&grammar);
        let mut parser = Parser::new(compiled.table());

        let wrong_id = SymbolId(99);
        let token = Token::new(wrong_id);

        let result = parser.maybe_reduce(Some(&token));
        assert!(result.is_err());
    }

    #[test]
    fn test_format_error() {
        let mut gb = GrammarBuilder::new();
        let a = gb.t("a");
        gb.t("b");
        let s = gb.nt("S");
        gb.rule(s, vec![a]);
        let grammar = gb.build();

        let compiled = CompiledTable::build(&grammar);
        let mut parser = Parser::new(compiled.table());

        // Try to parse 'b' when only 'a' is expected
        let b_id = compiled.symbol_id("b").unwrap();
        let token = Token::new(b_id);

        let err = parser.maybe_reduce(Some(&token)).unwrap_err();
        let msg = err.format(&compiled);

        assert!(msg.contains("unexpected"), "msg: {}", msg);
        assert!(msg.contains("'b'"), "msg: {}", msg);
        assert!(msg.contains("a"), "msg: {}", msg);
    }
}

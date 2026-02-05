use crate::grammar::SymbolId;
use crate::table::{Action, ParseTable};
use std::collections::{HashMap, HashSet};

/// Trait for providing error context (symbol names, expected terminals).
pub trait ErrorContext {
    /// Get the name for a symbol ID.
    fn symbol_name(&self, id: SymbolId) -> &str;
    /// Get expected terminal IDs for a state.
    fn expected_terminals(&self, state: usize) -> Vec<u32>;
    /// Get the accessing symbol for a state (the symbol shifted/reduced to enter it).
    fn state_symbol(&self, state: usize) -> SymbolId;
    /// Get active items (rule, dot) for a state.
    fn state_items(&self, state: usize) -> Vec<(usize, usize)>;
    /// Get LHS symbol ID for a rule.
    fn rule_lhs(&self, rule: usize) -> SymbolId;
    /// Get RHS symbol IDs for a rule.
    fn rule_rhs(&self, rule: usize) -> Vec<SymbolId>;
    /// Get RHS length for a rule.
    fn rule_len(&self, rule: usize) -> usize {
        self.rule_rhs(rule).len()
    }
    /// GOTO lookup: given state and non-terminal, return next state (None if error).
    fn goto(&self, _state: usize, _non_terminal: SymbolId) -> Option<usize> {
        None  // Default: not available
    }
    /// Number of terminal symbols.
    fn num_terminals(&self) -> usize {
        0  // Default: unknown
    }
    /// Number of rules.
    fn num_rules(&self) -> usize {
        0  // Default: unknown
    }
}

/// Precedence information carried by a token at parse time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Precedence {
    Left(u8),
    Right(u8),
}

impl Precedence {
    /// Get the precedence level.
    pub fn level(&self) -> u8 {
        match self {
            Precedence::Left(l) | Precedence::Right(l) => *l,
        }
    }

    /// Get the associativity as u8 (0=left, 1=right).
    pub fn assoc(&self) -> u8 {
        match self {
            Precedence::Left(_) => 0,
            Precedence::Right(_) => 1,
        }
    }
}

/// Compute which symbols are nullable (can derive epsilon).
fn compute_nullable(ctx: &impl ErrorContext) -> Vec<bool> {
    let num_rules = ctx.num_rules();

    // Find max symbol ID by scanning rules
    let mut max_sym = ctx.num_terminals();
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

    let mut nullable: Vec<bool> = vec![false; max_sym];

    // Fixed-point iteration
    let mut changed = true;
    while changed {
        changed = false;

        for rule in 0..num_rules {
            let lhs = ctx.rule_lhs(rule).0 as usize;
            let rhs = ctx.rule_rhs(rule);

            // If RHS is empty or all nullable, LHS is nullable
            let all_nullable = rhs.iter().all(|sym| nullable[sym.0 as usize]);
            if all_nullable && !nullable[lhs] {
                nullable[lhs] = true;
                changed = true;
            }
        }
    }

    nullable
}

/// Compute FIRST sets for all symbols.
fn compute_first_sets(ctx: &impl ErrorContext, nullable: &[bool]) -> Vec<HashSet<usize>> {
    let num_terminals = ctx.num_terminals();
    let num_rules = ctx.num_rules();

    // Find max symbol ID
    let mut max_sym = num_terminals;
    for rule in 0..num_rules {
        let lhs = ctx.rule_lhs(rule).0 as usize;
        if lhs >= max_sym {
            max_sym = lhs + 1;
        }
        for sym in ctx.rule_rhs(rule) {
            if sym.0 as usize >= max_sym {
                max_sym = sym.0 as usize + 1;
            }
        }
    }

    let mut first: Vec<HashSet<usize>> = vec![HashSet::new(); max_sym];

    // Terminals: FIRST(t) = {t}
    for t in 0..num_terminals {
        first[t].insert(t);
    }

    // Fixed-point iteration for nonterminals
    let mut changed = true;
    while changed {
        changed = false;
        for rule in 0..num_rules {
            let lhs = ctx.rule_lhs(rule).0 as usize;
            let rhs = ctx.rule_rhs(rule);

            for sym in &rhs {
                let sym_id = sym.0 as usize;
                let sym_first: Vec<_> = first.get(sym_id)
                    .map(|s| s.iter().copied().collect())
                    .unwrap_or_default();
                let before = first[lhs].len();
                first[lhs].extend(sym_first);
                if first[lhs].len() > before {
                    changed = true;
                }
                if !nullable.get(sym_id).copied().unwrap_or(false) {
                    break;
                }
            }
        }
    }
    first
}

/// Compute FIRST of a sequence of symbols.
fn first_of_sequence(
    sequence: &[SymbolId],
    first_sets: &[HashSet<usize>],
    nullable: &[bool],
) -> HashSet<usize> {
    let mut result = HashSet::new();
    for sym in sequence {
        let sym_id = sym.0 as usize;
        if let Some(sym_first) = first_sets.get(sym_id) {
            result.extend(sym_first.iter().copied());
        }
        if !nullable.get(sym_id).copied().unwrap_or(false) {
            break;
        }
    }
    result
}

/// Check if a sequence is nullable.
fn is_sequence_nullable(sequence: &[SymbolId], nullable: &[bool]) -> bool {
    sequence.iter().all(|sym| nullable.get(sym.0 as usize).copied().unwrap_or(false))
}

/// Parse error containing the unexpected terminal.
///
/// The parser remains in a valid state after an error, so you can call
/// `parser.format_error()` to get a detailed error message.
#[derive(Debug, Clone)]
pub struct ParseError {
    terminal: SymbolId,
}

impl ParseError {
    /// The unexpected terminal that caused the error.
    pub fn terminal(&self) -> SymbolId {
        self.terminal
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
                Err(ParseError { terminal })
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
            // Pop first, then determine prec from anchor (the context before this reduction).
            for _ in 0..(len - 1) {
                self.stack.pop();
            }
            let anchor = self.stack.last().unwrap();
            // For single-symbol (len=1): preserve the symbol's own prec (e.g., PLUS → op)
            // For multi-symbol (len>1): use anchor's prec (the "waiting" context)
            // This handles both binary (expr OP expr) and unary (OP expr) correctly.
            let captured_prec = if len == 1 { self.state.prec } else { anchor.prec };
            if let Some(next_state) = self.table.goto(anchor.state, lhs) {
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

    /// Format a parse error using the provided error context.
    ///
    /// Call this after `maybe_reduce` returns an error to get a detailed message.
    pub fn format_error(&self, err: &ParseError, ctx: &impl ErrorContext) -> String {
        self.format_error_with(err, ctx, &HashMap::new(), &[])
    }

    /// Format a parse error with display names and token texts.
    ///
    /// - `display_names`: maps grammar names to user-friendly names (e.g., "SEMI" → "';'")
    /// - `tokens`: token texts by index (must include error token at index `token_count()`)
    pub fn format_error_with(
        &self,
        err: &ParseError,
        ctx: &impl ErrorContext,
        display_names: &HashMap<&str, &str>,
        tokens: &[&str],
    ) -> String {
        // Build full stack for error analysis
        let mut full_stack: Vec<StackEntry> = self.stack.clone();
        full_stack.push(self.state);
        let error_token_idx = self.token_count;

        let state = full_stack.last().unwrap().state;

        let display = |id: SymbolId| -> &str {
            let name = ctx.symbol_name(id);
            display_names.get(name).copied().unwrap_or(name)
        };

        // Helper: compute stack spans
        let stack_spans = || -> Vec<(usize, usize, usize)> {
            let mut spans = Vec::with_capacity(full_stack.len());
            for i in 0..full_stack.len() {
                let start = full_stack[i].token_idx;
                let end = if i + 1 < full_stack.len() {
                    full_stack[i + 1].token_idx
                } else {
                    error_token_idx
                };
                spans.push((start, end, full_stack[i].state));
            }
            spans
        };

        // Find states with incomplete items (follow reductions if needed)
        let items_states = self.find_incomplete_items_states(ctx, state, &full_stack);
        let nullable = compute_nullable(ctx);
        let first_sets = compute_first_sets(ctx, &nullable);
        let num_terminals = ctx.num_terminals();

        // Compute expected terminals using the stack for precise lookaheads
        let expected_syms = self.compute_expected_from_stack(ctx, &first_sets, &nullable, num_terminals);

        // Convert to display names
        let mut expected: Vec<_> = expected_syms.iter()
            .map(|&sym| display(SymbolId(sym as u32)))
            .collect();
        expected.sort();

        // Show actual token text if available, otherwise display name
        let found_name = tokens.get(error_token_idx)
            .copied()
            .unwrap_or_else(|| display(err.terminal));

        let mut msg = format!("unexpected '{}'", found_name);
        if !expected.is_empty() {
            msg.push_str(&format!(", expected: {}", expected.join(", ")));
        }

        // Show parsed stack with token spans
        if !tokens.is_empty() && error_token_idx <= tokens.len() {
            let spans = stack_spans();
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
        } else if full_stack.len() > 1 {
            // Fallback: show grammar symbols from stack
            let path: Vec<_> = full_stack[1..]
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

    /// Compute expected terminals using the stack for precise lookaheads.
    fn compute_expected_from_stack(
        &self,
        ctx: &impl ErrorContext,
        first_sets: &[HashSet<usize>],
        nullable: &[bool],
        num_terminals: usize,
    ) -> HashSet<usize> {
        let mut expected = HashSet::new();
        // stack.len() gives depth, +1 for current state in register
        let stack_len = self.stack.len() + 1;

        for (rule, dot) in ctx.state_items(self.state.state) {
            let rhs = ctx.rule_rhs(rule);
            let lhs = ctx.rule_lhs(rule);

            // For __start items, only add EOF if complete (accept state)
            if ctx.symbol_name(lhs) == "__start" {
                if dot >= rhs.len() {
                    expected.insert(0); // EOF - accept is valid
                }
                continue;
            }

            if dot < rhs.len() {
                // Incomplete item: add FIRST(suffix)
                let suffix = &rhs[dot..];
                expected.extend(first_of_sequence(suffix, first_sets, nullable));

                // If suffix is nullable, need follow from calling context
                if is_sequence_nullable(suffix, nullable) {
                    let consumed = dot;
                    if stack_len > consumed {
                        expected.extend(self.compute_follow_from_context(
                            ctx, lhs, stack_len - consumed,
                            first_sets, nullable, num_terminals, &mut HashSet::new(),
                        ));
                    }
                }
            } else {
                // Complete item: need follow from calling context
                let consumed = rhs.len();
                if stack_len > consumed {
                    expected.extend(self.compute_follow_from_context(
                        ctx, lhs, stack_len - consumed,
                        first_sets, nullable, num_terminals, &mut HashSet::new(),
                    ));
                } else {
                    expected.insert(0); // EOF
                }
            }
        }

        expected.retain(|&sym| sym < num_terminals);
        expected
    }

    /// Get state at a given stack index (0 = bottom, stack.len() = current state).
    fn state_at_idx(&self, idx: usize) -> usize {
        if idx < self.stack.len() {
            self.stack[idx].state
        } else {
            self.state.state
        }
    }

    /// Compute what follows a nonterminal using the stack as calling context.
    fn compute_follow_from_context(
        &self,
        ctx: &impl ErrorContext,
        nonterminal: SymbolId,
        caller_idx: usize,  // 1-based index into conceptual stack
        first_sets: &[HashSet<usize>],
        nullable: &[bool],
        num_terminals: usize,
        visited: &mut HashSet<(usize, u32)>,
    ) -> HashSet<usize> {
        // Rule 0 is __start → S, nothing follows __start
        if nonterminal == ctx.rule_lhs(0) {
            let mut result = HashSet::new();
            result.insert(0); // EOF
            return result;
        }

        if caller_idx == 0 {
            let mut result = HashSet::new();
            result.insert(0); // EOF
            return result;
        }

        let caller_state = self.state_at_idx(caller_idx - 1);

        // Use caller_idx in visited key to allow same state at different stack depths
        if !visited.insert((caller_idx, nonterminal.0)) {
            return HashSet::new();
        }

        let mut expected = HashSet::new();

        // Find items [B → γ • A δ] where A is our nonterminal
        for (rule, dot) in ctx.state_items(caller_state) {
            let rhs = ctx.rule_rhs(rule);
            if dot < rhs.len() && rhs[dot] == nonterminal {
                let suffix = &rhs[dot + 1..];
                let lhs = ctx.rule_lhs(rule);
                let consumed = dot;

                if suffix.is_empty() {
                    // Nothing after A, follow what follows B
                    if caller_idx > consumed {
                        expected.extend(self.compute_follow_from_context(
                            ctx, lhs, caller_idx - consumed,
                            first_sets, nullable, num_terminals, visited,
                        ));
                    } else {
                        expected.insert(0);
                    }
                } else {
                    // Add FIRST(suffix)
                    expected.extend(first_of_sequence(suffix, first_sets, nullable));

                    // If suffix nullable, also follow B
                    if is_sequence_nullable(suffix, nullable) {
                        if caller_idx > consumed {
                            expected.extend(self.compute_follow_from_context(
                                ctx, lhs, caller_idx - consumed,
                                first_sets, nullable, num_terminals, visited,
                            ));
                        } else {
                            expected.insert(0);
                        }
                    }
                }
            }
        }

        expected
    }

    /// Find states with incomplete items by following reductions recursively.
    fn find_incomplete_items_states(
        &self,
        ctx: &impl ErrorContext,
        state: usize,
        full_stack: &[StackEntry],
    ) -> HashSet<usize> {
        let mut result = HashSet::new();
        let mut visited = HashSet::new();
        // Track (current_state, virtual_stack) where virtual_stack extends full_stack
        let initial_stack: Vec<usize> = full_stack.iter().map(|e| e.state).collect();
        let mut worklist = vec![(state, initial_stack)];

        while let Some((current_state, virtual_stack)) = worklist.pop() {
            if !visited.insert(current_state) {
                continue;
            }

            let items = ctx.state_items(current_state);

            // Check if there are any incomplete items
            let has_incomplete = items.iter().any(|&(rule, dot)| {
                let rhs = ctx.rule_rhs(rule);
                dot < rhs.len()
            });

            if has_incomplete {
                result.insert(current_state);
                continue; // Don't follow reductions from states with incomplete items
            }

            // All items complete - follow all reductions recursively
            for &(rule, _) in &items {
                let lhs = ctx.rule_lhs(rule);
                let rhs_len = ctx.rule_rhs(rule).len();

                // Find the state we'd be in after popping rhs_len entries
                if virtual_stack.len() > rhs_len {
                    let from_state = virtual_stack[virtual_stack.len() - rhs_len - 1];
                    if let Some(goto_state) = ctx.goto(from_state, lhs) {
                        // Build new virtual stack: pop rhs_len, push goto_state
                        let mut new_stack = virtual_stack[..virtual_stack.len() - rhs_len].to_vec();
                        new_stack.push(goto_state);
                        worklist.push((goto_state, new_stack));
                    }
                }
            }
        }

        // If we found nothing, return the original state
        if result.is_empty() {
            result.insert(state);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::SymbolId;
    use crate::table::CompiledTable;
    use crate::meta::parse_grammar;
    use crate::lr::to_grammar_internal;

    #[test]
    fn test_parse_single_token() {
        let grammar = to_grammar_internal(parse_grammar(r#"
            grammar Simple { start s; terminals { a } s = a; }
        "#).unwrap()).unwrap();

        let compiled = CompiledTable::build_with_algorithm(&grammar, crate::lr::LrAlgorithm::default());
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
        let grammar = to_grammar_internal(parse_grammar(r#"
            grammar Simple { start s; terminals { a } s = a; }
        "#).unwrap()).unwrap();

        let compiled = CompiledTable::build_with_algorithm(&grammar, crate::lr::LrAlgorithm::default());
        let mut parser = Parser::new(compiled.table());

        let wrong_id = SymbolId(99);
        let token = Token::new(wrong_id);

        let result = parser.maybe_reduce(Some(&token));
        assert!(result.is_err());
    }

    #[test]
    fn test_format_error() {
        let grammar = to_grammar_internal(parse_grammar(r#"
            grammar Simple { start s; terminals { a, b } s = a; }
        "#).unwrap()).unwrap();

        let compiled = CompiledTable::build_with_algorithm(&grammar, crate::lr::LrAlgorithm::default());
        let mut parser = Parser::new(compiled.table());

        // Try to parse 'b' when only 'a' is expected
        let b_id = compiled.symbol_id("b").unwrap();
        let token = Token::new(b_id);

        let err = parser.maybe_reduce(Some(&token)).unwrap_err();
        let msg = parser.format_error(&err, &compiled);

        assert!(msg.contains("unexpected"), "msg: {}", msg);
        assert!(msg.contains("'b'"), "msg: {}", msg);
        assert!(msg.contains("a"), "msg: {}", msg);
    }
}

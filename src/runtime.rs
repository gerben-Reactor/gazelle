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

/// Collect expected symbols from a sequence, keeping nonterminal names.
/// Adds each symbol ID (terminal or nonterminal) until a non-nullable one.
fn expected_from_sequence(sequence: &[SymbolId], nullable: &[bool]) -> HashSet<usize> {
    let mut result = HashSet::new();
    for sym in sequence {
        let sym_id = sym.0 as usize;
        result.insert(sym_id);
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

        let nullable = compute_nullable(ctx);

        // Compute expected symbols using the stack for precise lookaheads
        let expected_syms = self.compute_expected_from_stack(ctx, &nullable);

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

        // Show informative items that explain what's expected
        let display_items = self.compute_display_items(ctx);
        let mut seen = HashSet::new();

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

        for (rule, dot) in display_items {
            let rhs = ctx.rule_rhs(rule);
            let lhs = ctx.rule_lhs(rule);
            let lhs_name = display(lhs);

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
        msg
    }

    /// Find informative items to display in error messages.
    /// Items with progress (0 < dot < len) are informative.
    /// Items at start (dot = 0) trace back to parent item if available.
    /// Complete items trace back to find what follows.
    fn compute_display_items(&self, ctx: &impl ErrorContext) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        let stack_len = self.stack.len() + 1;

        for (rule, dot) in ctx.state_items(self.state.state) {
            let rhs = ctx.rule_rhs(rule);
            let lhs = ctx.rule_lhs(rule);

            // Skip __start items
            if ctx.symbol_name(lhs) == "__start" {
                continue;
            }

            if dot > 0 && dot < rhs.len() {
                // Informative: shows progress and more to come
                result.push((rule, dot));
            } else if dot == 0 && !rhs.is_empty() {
                // Closure item: find parent item in same state with progress
                let mut found_parent = false;
                for (prule, pdot) in ctx.state_items(self.state.state) {
                    let prhs = ctx.rule_rhs(prule);
                    let plhs = ctx.rule_lhs(prule);
                    if ctx.symbol_name(plhs) == "__start" {
                        continue;
                    }
                    if pdot > 0 && pdot < prhs.len() && prhs[pdot] == lhs {
                        result.push((prule, pdot));
                        found_parent = true;
                    }
                }
                if !found_parent {
                    result.push((rule, dot));
                }
            } else if dot >= rhs.len() {
                // Complete: find caller item showing what follows this nonterminal
                let consumed = rhs.len();
                if stack_len > consumed {
                    let caller_state = self.state_at_idx(stack_len - consumed - 1);
                    for (prule, pdot) in ctx.state_items(caller_state) {
                        let prhs = ctx.rule_rhs(prule);
                        let plhs = ctx.rule_lhs(prule);
                        if ctx.symbol_name(plhs) == "__start" {
                            continue;
                        }
                        // Find item [B → γ • A δ] where A = lhs and δ is non-empty
                        if pdot < prhs.len() && prhs[pdot] == lhs {
                            // The item after goto is [B → γ A • δ]
                            let new_dot = pdot + 1;
                            if new_dot < prhs.len() {
                                result.push((prule, new_dot));
                            }
                        }
                    }
                }
            }
        }

        result
    }

    /// Compute expected symbols using the stack for precise lookaheads.
    /// Returns symbol IDs which may be terminals or nonterminals.
    /// Items at dot=0 are closure items predicted by a parent item
    /// that already contributes the nonterminal name, so they are skipped.
    fn compute_expected_from_stack(
        &self,
        ctx: &impl ErrorContext,
        nullable: &[bool],
    ) -> HashSet<usize> {
        let mut expected = HashSet::new();
        let stack_len = self.stack.len() + 1;

        for (rule, dot) in ctx.state_items(self.state.state) {
            let rhs = ctx.rule_rhs(rule);
            let lhs = ctx.rule_lhs(rule);

            // __start: add EOF if complete, add start symbol if incomplete
            if ctx.symbol_name(lhs) == "__start" {
                if dot >= rhs.len() {
                    expected.insert(0); // EOF
                } else {
                    expected.extend(expected_from_sequence(&rhs[dot..], nullable));
                }
                continue;
            }

            // Skip closure items; their parent already contributes the nonterminal
            if dot == 0 {
                continue;
            }

            if dot < rhs.len() {
                let suffix = &rhs[dot..];
                expected.extend(expected_from_sequence(suffix, nullable));

                if is_sequence_nullable(suffix, nullable) {
                    let consumed = dot;
                    if stack_len > consumed {
                        expected.extend(self.compute_follow_from_context(
                            ctx, lhs, stack_len - consumed,
                            nullable, &mut HashSet::new(),
                        ));
                    }
                }
            } else {
                let consumed = rhs.len();
                if stack_len > consumed {
                    expected.extend(self.compute_follow_from_context(
                        ctx, lhs, stack_len - consumed,
                        nullable, &mut HashSet::new(),
                    ));
                } else {
                    expected.insert(0); // EOF
                }
            }
        }

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
        caller_idx: usize,
        nullable: &[bool],
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
                            nullable, visited,
                        ));
                    } else {
                        expected.insert(0);
                    }
                } else {
                    expected.extend(expected_from_sequence(suffix, nullable));

                    if is_sequence_nullable(suffix, nullable) {
                        if caller_idx > consumed {
                            expected.extend(self.compute_follow_from_context(
                                ctx, lhs, caller_idx - consumed,
                                nullable, visited,
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

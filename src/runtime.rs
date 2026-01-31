use crate::grammar::{Precedence, SymbolId};
use crate::table::{Action, ErrorContext, ParseTable};

/// Parse error with full context.
#[derive(Debug, Clone)]
pub struct ParseError {
    terminal: SymbolId,
    stack: Vec<StackEntry>,
}

impl ParseError {
    /// Format the error using the provided error context.
    pub fn format(&self, ctx: &impl ErrorContext) -> String {
        let state = self.stack.last().unwrap().state;
        let found_name = ctx.symbol_name(self.terminal);
        let expected: Vec<_> = ctx.expected_terminals(state)
            .iter()
            .map(|&id| ctx.symbol_name(SymbolId(id)))
            .collect();

        let mut msg = format!("unexpected '{}'", found_name);
        if !expected.is_empty() {
            msg.push_str(&format!(", expected: {}", expected.join(", ")));
        }

        // Show parse path from stack (skip initial state 0)
        if self.stack.len() > 1 {
            let path: Vec<_> = self.stack[1..]
                .iter()
                .map(|e| ctx.symbol_name(ctx.state_symbol(e.state)))
                .collect();
            msg.push_str(&format!("\n  after: {}", path.join(" ")));
        }

        // Show active items (rules being parsed)
        let items = ctx.state_items(state);
        for (rule, dot) in items {
            let lhs = ctx.rule_lhs(rule);
            let rhs = ctx.rule_rhs(rule);
            let lhs_name = ctx.symbol_name(lhs);
            let before: Vec<_> = rhs[..dot]
                .iter()
                .map(|&id| ctx.symbol_name(id))
                .collect();
            let after: Vec<_> = rhs[dot..]
                .iter()
                .map(|&id| ctx.symbol_name(id))
                .collect();
            msg.push_str(&format!(
                "\n  in {}: {} \u{2022} {}",
                lhs_name,
                before.join(" "),
                after.join(" ")
            ));
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
}

/// An LR parser.
pub struct Parser<'a> {
    table: ParseTable<'a>,
    /// Current state (top of stack, kept in "register").
    state: StackEntry,
    /// Previous states (rest of stack).
    stack: Vec<StackEntry>,
}

impl<'a> Parser<'a> {
    /// Create a new parser with the given parse table.
    pub fn new(table: ParseTable<'a>) -> Self {
        Self {
            table,
            state: StackEntry { state: 0, prec: None },
            stack: Vec::new(),
        }
    }

    /// Check if a reduction should happen for the given lookahead.
    ///
    /// Returns `Ok(Some((rule, len)))` if a reduction should occur.
    /// Returns `Ok(None)` if should shift or if accepted.
    /// Returns `Err(ParseError)` on parse error.
    pub fn maybe_reduce(&mut self, lookahead: Option<&Token>) -> Result<Option<(usize, usize)>, ParseError> {
        let terminal = lookahead.map(|t| t.terminal).unwrap_or(SymbolId::EOF);
        let lookahead_prec = lookahead.and_then(|t| t.prec);

        match self.table.action(self.state.state, terminal) {
            Action::Reduce(rule) => {
                if rule == 0 {
                    Ok(Some((0, 0))) // Accept
                } else {
                    let len = self.do_reduce(rule);
                    Ok(Some((rule, len)))
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
                    let len = self.do_reduce(reduce_rule);
                    Ok(Some((reduce_rule, len)))
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
        self.state = StackEntry { state: next_state, prec };
    }

    fn do_reduce(&mut self, rule: usize) -> usize {
        let (lhs, len) = self.table.rule_info(rule);

        if len == 0 {
            // Epsilon: anchor is current state, push it, then set new state
            if let Some(next_state) = self.table.goto(self.state.state, lhs) {
                self.stack.push(self.state);
                self.state = StackEntry { state: next_state, prec: None };
            }
        } else {
            // Non-epsilon: capture prec from current state, pop len-1, goto from stack.last()
            let captured_prec = self.state.prec;
            for _ in 0..(len - 1) {
                self.stack.pop();
            }
            let anchor = self.stack.last().unwrap().state;
            if let Some(next_state) = self.table.goto(anchor, lhs) {
                self.state = StackEntry { state: next_state, prec: captured_prec };
            }
        }

        len
    }

    /// Get the current state.
    pub fn state(&self) -> usize {
        self.state.state
    }

    /// Get the number of values on the stack.
    pub fn stack_depth(&self) -> usize {
        self.stack.len()
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
        assert!(matches!(result, Ok(Some((1, 1)))));

        // Should be accepted now (rule 0)
        let result = parser.maybe_reduce(None);
        assert!(matches!(result, Ok(Some((0, 0)))));
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

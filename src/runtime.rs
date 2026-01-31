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
    stack: Vec<StackEntry>,
}

impl<'a> Parser<'a> {
    /// Create a new parser with the given parse table.
    pub fn new(table: ParseTable<'a>) -> Self {
        Self {
            table,
            stack: vec![StackEntry { state: 0, prec: None }],
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
        let state = self.stack.last().unwrap().state;

        match self.table.action(state, terminal) {
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
                let stack_prec = self.stack.last().unwrap().prec;

                let should_reduce = match (stack_prec, lookahead_prec) {
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
            Action::Error => Err(ParseError {
                terminal,
                stack: self.stack.clone(),
            }),
        }
    }

    /// Shift a token onto the stack.
    pub fn shift(&mut self, token: &Token) {
        let state = self.stack.last().unwrap().state;

        let next_state = match self.table.action(state, token.terminal) {
            Action::Shift(s) => s,
            Action::ShiftOrReduce { shift_state, .. } => shift_state,
            _ => panic!("shift called when action is not shift"),
        };

        let prec = token.prec.or(self.stack.last().unwrap().prec);
        self.stack.push(StackEntry { state: next_state, prec });
    }

    fn do_reduce(&mut self, rule: usize) -> usize {
        let (lhs, len) = self.table.rule_info(rule);

        let captured_prec = if len > 0 {
            self.stack.last().and_then(|e| e.prec)
        } else {
            None
        };

        for _ in 0..len {
            self.stack.pop();
        }

        let goto_state = self.stack.last().unwrap().state;
        if let Some(next_state) = self.table.goto(goto_state, lhs) {
            self.stack.push(StackEntry { state: next_state, prec: captured_prec });
        }

        len
    }

    /// Get the current state.
    pub fn state(&self) -> usize {
        self.stack.last().unwrap().state
    }

    /// Get the number of values on the stack (excluding initial state).
    pub fn stack_depth(&self) -> usize {
        self.stack.len() - 1
    }

    /// Get the state at a given depth (0 = bottom of value stack).
    pub fn state_at(&self, depth: usize) -> usize {
        self.stack[depth + 1].state
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

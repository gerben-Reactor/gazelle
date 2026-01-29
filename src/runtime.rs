use crate::grammar::{Precedence, SymbolId};
use crate::table::{Action, ParseTable};

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

    /// Create an EOF token.
    pub fn eof() -> Self {
        Self { terminal: SymbolId::EOF, prec: None }
    }
}

/// Events emitted by the parser during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// A shift occurred (terminal consumed).
    Shift,
    /// A reduction occurred.
    Reduce {
        /// Index of the rule that was reduced.
        rule: usize,
        /// Number of symbols on the right-hand side.
        len: usize,
    },
    /// The input was accepted.
    Accept,
    /// A parse error occurred.
    Error {
        /// The unexpected terminal (SymbolId::EOF for end of input).
        terminal: SymbolId,
        /// The current state.
        state: usize,
    },
}

/// Stack entry for the parser.
#[derive(Debug, Clone, Copy)]
struct StackEntry {
    state: usize,
    prec: Option<Precedence>,
}

impl StackEntry {
    fn new(state: usize) -> Self {
        Self { state, prec: None }
    }

    fn with_prec(state: usize, prec: Option<Precedence>) -> Self {
        Self { state, prec }
    }
}

/// A push-based LR parser.
pub struct Parser<'a> {
    table: ParseTable<'a>,
    stack: Vec<StackEntry>,
}

impl<'a> Parser<'a> {
    /// Create a new parser with the given parse table.
    pub fn new(table: ParseTable<'a>) -> Self {
        Self {
            table,
            stack: vec![StackEntry::new(0)],
        }
    }

    /// Push a token to the parser and return events.
    pub fn push(&mut self, token: &Token) -> Vec<Event> {
        let mut events = Vec::new();
        self.process(token, &mut events);
        events
    }

    /// Signal end of input and return final events.
    pub fn finish(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        let eof_token = Token::eof();
        self.process(&eof_token, &mut events);
        events
    }

    fn process(&mut self, token: &Token, events: &mut Vec<Event>) {
        let terminal = token.terminal;

        loop {
            let state = self.stack.last().unwrap().state;
            let action = self.table.action(state, terminal);

            match action {
                Action::Shift(next_state) => {
                    let prec = token.prec.or(self.stack.last().unwrap().prec);
                    self.stack.push(StackEntry::with_prec(next_state, prec));
                    events.push(Event::Shift);
                    break;
                }
                Action::Reduce(0) => {
                    events.push(Event::Accept);
                    break;
                }
                Action::Reduce(rule_idx) => {
                    self.do_reduce(rule_idx, events);
                }
                Action::ShiftOrReduce { shift_state, reduce_rule } => {
                    let stack_prec = self.stack.last().unwrap().prec;
                    let token_prec = token.prec;

                    let should_shift = match (stack_prec, token_prec) {
                        (Some(sp), Some(tp)) => {
                            if tp.level() > sp.level() {
                                true
                            } else if tp.level() < sp.level() {
                                false
                            } else {
                                matches!(tp, Precedence::Right(_))
                            }
                        }
                        _ => true,
                    };

                    if should_shift {
                        self.stack.push(StackEntry::with_prec(shift_state, token.prec));
                        events.push(Event::Shift);
                        break;
                    } else {
                        self.do_reduce(reduce_rule, events);
                    }
                }
                Action::Error => {
                    events.push(Event::Error {
                        terminal,
                        state,
                    });
                    break;
                }
            }
        }
    }

    fn do_reduce(&mut self, rule_idx: usize, events: &mut Vec<Event>) {
        let (lhs, len) = self.table.rule_info(rule_idx);

        // Capture precedence from the topmost RHS symbol before popping.
        // This propagates operator precedence through intermediate reductions
        // like `binary_op → PLUS`.
        let captured_prec = if len > 0 {
            self.stack.last().and_then(|e| e.prec)
        } else {
            None
        };

        for _ in 0..len {
            self.stack.pop();
        }

        debug_assert!(!self.stack.is_empty());

        let goto_entry = self.stack.last().unwrap();
        if let Some(next_state) = self.table.goto(goto_entry.state, lhs) {
            self.stack.push(StackEntry::with_prec(next_state, captured_prec));
        }

        events.push(Event::Reduce { rule: rule_idx, len });
    }

    /// Get the current state.
    pub fn state(&self) -> usize {
        self.stack.last().unwrap().state
    }

    /// Get the stack depth.
    pub fn stack_depth(&self) -> usize {
        self.stack.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::GrammarBuilder;

    #[test]
    fn test_parse_single_token() {
        use crate::table::CompiledTable;

        let mut gb = GrammarBuilder::new();
        let a = gb.t("a");
        let s = gb.nt("S");
        gb.rule(s, vec![a]);
        let grammar = gb.build();

        let compiled = CompiledTable::build(&grammar);
        let mut parser = Parser::new(compiled.table());

        let a_id = compiled.symbol_id("a").unwrap();
        let events = parser.push(&Token::new(a_id));
        assert!(events.iter().any(|e| matches!(e, Event::Shift)));

        let events = parser.finish();
        assert!(events.iter().any(|e| matches!(e, Event::Reduce { rule: 1, len: 1, .. })));
        assert!(events.iter().any(|e| matches!(e, Event::Accept)));
    }

    #[test]
    fn test_parse_error() {
        use crate::table::CompiledTable;

        let mut gb = GrammarBuilder::new();
        let a = gb.t("a");
        let s = gb.nt("S");
        gb.rule(s, vec![a]);
        let grammar = gb.build();

        let compiled = CompiledTable::build(&grammar);
        let mut parser = Parser::new(compiled.table());

        let wrong_id = SymbolId(99);
        let events = parser.push(&Token::new(wrong_id));
        assert!(events.iter().any(|e| matches!(e, Event::Error { .. })));
    }
}

use crate::grammar::{Precedence, SymbolId, Assoc};
use crate::table::{Action, ParseTable};

/// A token with terminal symbol ID, value, and optional precedence.
#[derive(Debug, Clone)]
pub struct Token {
    pub terminal: SymbolId,
    pub value: String,
    /// Precedence info for operators.
    pub prec: Option<Precedence>,
}

impl Token {
    pub fn new(terminal: SymbolId, value: impl Into<String>) -> Self {
        Self { terminal, value: value.into(), prec: None }
    }

    pub fn with_prec(terminal: SymbolId, value: impl Into<String>, prec: Precedence) -> Self {
        Self { terminal, value: value.into(), prec: Some(prec) }
    }

    /// Create an EOF token.
    pub fn eof() -> Self {
        Self { terminal: SymbolId::EOF, value: String::new(), prec: None }
    }
}

/// Events emitted by the parser during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
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
    prec: Option<u8>,
}

impl StackEntry {
    fn new(state: usize) -> Self {
        Self { state, prec: None }
    }

    fn with_prec(state: usize, prec: Option<u8>) -> Self {
        Self { state, prec }
    }
}

/// A push-based LR parser.
pub struct Parser<'a> {
    table: &'a ParseTable,
    stack: Vec<StackEntry>,
}

impl<'a> Parser<'a> {
    /// Create a new parser with the given parse table.
    pub fn new(table: &'a ParseTable) -> Self {
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
            let entry = *self.stack.last().unwrap();
            let action = self.table.action(entry.state, terminal);

            match action {
                Action::Shift(next_state) => {
                    let prec = token.prec.map(|p| p.level).or(entry.prec);
                    self.stack.push(StackEntry::with_prec(next_state, prec));
                    break;
                }
                Action::Reduce(rule_idx) => {
                    self.do_reduce(rule_idx, events);
                }
                Action::ShiftOrReduce { shift_state, reduce_rule } => {
                    let stack_prec = entry.prec;
                    let token_prec = token.prec;

                    let should_shift = match (stack_prec, token_prec) {
                        (Some(sp), Some(tp)) => {
                            if tp.level > sp {
                                true
                            } else if tp.level < sp {
                                false
                            } else {
                                match tp.assoc {
                                    Assoc::Right => true,
                                    Assoc::Left => false,
                                }
                            }
                        }
                        _ => true,
                    };

                    if should_shift {
                        let prec = token.prec.map(|p| p.level).or(entry.prec);
                        self.stack.push(StackEntry::with_prec(shift_state, prec));
                        break;
                    } else {
                        self.do_reduce(reduce_rule, events);
                    }
                }
                Action::Accept => {
                    events.push(Event::Accept);
                    break;
                }
                Action::Error => {
                    events.push(Event::Error {
                        terminal,
                        state: entry.state,
                    });
                    break;
                }
            }
        }
    }

    fn do_reduce(&mut self, rule_idx: usize, events: &mut Vec<Event>) {
        let (lhs, len) = self.table.rule_info(rule_idx);

        for _ in 0..len {
            self.stack.pop();
        }

        debug_assert!(!self.stack.is_empty());

        let goto_entry = self.stack.last().unwrap();
        if let Some(next_state) = self.table.goto(goto_entry.state, lhs) {
            self.stack.push(StackEntry::with_prec(next_state, goto_entry.prec));
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
    use crate::lr::Automaton;

    #[test]
    fn test_parse_single_token() {
        let mut gb = GrammarBuilder::new();
        let a = gb.t("a");
        let s = gb.nt("S");
        gb.rule(s, vec![a]);
        let grammar = gb.build();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);
        let mut parser = Parser::new(&table);

        let a_id = table.symbol_id("a").unwrap();
        let events = parser.push(&Token::new(a_id, "a"));
        assert!(events.is_empty());

        let events = parser.finish();
        assert!(events.iter().any(|e| matches!(e, Event::Reduce { rule: 1, len: 1, .. })));
        assert!(events.iter().any(|e| matches!(e, Event::Accept)));
    }

    #[test]
    fn test_parse_error() {
        let mut gb = GrammarBuilder::new();
        let a = gb.t("a");
        let s = gb.nt("S");
        gb.rule(s, vec![a]);
        let grammar = gb.build();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);
        let mut parser = Parser::new(&table);

        let wrong_id = SymbolId(99);
        let events = parser.push(&Token::new(wrong_id, "b"));
        assert!(events.iter().any(|e| matches!(e, Event::Error { .. })));
    }
}

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
    use crate::grammar::{GrammarBuilder, Precedence};
    use crate::lexer::{self, Token as LexToken};
    use crate::lr::Automaton;
    use std::collections::HashMap;

    /// Helper to parse expressions using the lexer.
    struct ExprParser<'a> {
        table: &'a ParseTable,
        ops: HashMap<String, Precedence>,
    }

    impl<'a> ExprParser<'a> {
        fn new(table: &'a ParseTable, ops: Vec<(&str, Precedence)>) -> Self {
            Self {
                table,
                ops: ops.into_iter().map(|(s, p)| (s.to_string(), p)).collect(),
            }
        }

        fn parse(&self, input: &str) -> (Vec<Event>, bool, String) {
            let lex_tokens = lexer::lex(input).unwrap();
            let mut parser = Parser::new(self.table);
            let mut all_events = Vec::new();
            let mut stack: Vec<String> = Vec::new();

            let num_id = self.table.symbol_id("NUM")
                .or_else(|| self.table.symbol_id("ID"));
            let op_id = self.table.symbol_id("OP");

            for tok in lex_tokens {
                let parser_tok = match &tok {
                    LexToken::Num(s) => {
                        if let Some(id) = num_id {
                            Token::new(id, s.clone())
                        } else {
                            continue;
                        }
                    }
                    LexToken::Ident(s) => {
                        if let Some(id) = self.table.symbol_id("ID") {
                            Token::new(id, s.clone())
                        } else {
                            continue;
                        }
                    }
                    LexToken::Op(s) => {
                        if let Some(&prec) = self.ops.get(s) {
                            if let Some(id) = op_id {
                                Token::with_prec(id, s.clone(), prec)
                            } else {
                                continue;
                            }
                        } else if let Some(id) = self.table.symbol_id(s) {
                            Token::new(id, s.clone())
                        } else {
                            continue;
                        }
                    }
                    _ => continue,
                };

                for event in parser.push(&parser_tok) {
                    if let Event::Reduce { rule, .. } = &event {
                        Self::apply_reduce(&mut stack, *rule);
                    }
                    all_events.push(event);
                }

                match &tok {
                    LexToken::Num(s) | LexToken::Ident(s) => stack.push(s.clone()),
                    LexToken::Op(s) => stack.push(s.clone()),
                    _ => {}
                }
            }

            for event in parser.finish() {
                if let Event::Reduce { rule, .. } = &event {
                    Self::apply_reduce(&mut stack, *rule);
                }
                all_events.push(event);
            }

            let accepted = all_events.iter().any(|e| matches!(e, Event::Accept));
            let tree = stack.pop().unwrap_or_default();
            (all_events, accepted, tree)
        }

        fn apply_reduce(stack: &mut Vec<String>, rule: usize) {
            match rule {
                0 => {}
                1 => {
                    if stack.len() >= 3 {
                        let right = stack.pop().unwrap();
                        let op = stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        stack.push(format!("({} {} {})", left, op, right));
                    }
                }
                2 => {}
                _ => {}
            }
        }
    }

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
    fn test_parse_expr() {
        let mut gb = GrammarBuilder::new();
        let plus = gb.t("+");
        let num = gb.t("NUM");
        let expr = gb.nt("expr");
        let term = gb.nt("term");

        gb.rule(expr, vec![expr, plus, term]);
        gb.rule(expr, vec![term]);
        gb.rule(term, vec![num]);
        let grammar = gb.build();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        let ep = ExprParser::new(&table, vec![]);
        let (events, accepted, _tree) = ep.parse("1 + 2");

        assert!(accepted);
        assert!(events.iter().any(|e| matches!(e, Event::Reduce { rule: 1, .. })));
        assert!(events.iter().any(|e| matches!(e, Event::Reduce { rule: 3, .. })));
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

    #[test]
    fn test_precedence_left_assoc() {
        let mut gb = GrammarBuilder::new();
        let op = gb.pt("OP");
        let num = gb.t("NUM");
        let expr = gb.nt("expr");

        gb.rule(expr, vec![expr, op, expr]);
        gb.rule(expr, vec![num]);
        let grammar = gb.build();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        let ep = ExprParser::new(&table, vec![
            ("+", Precedence::left(1)),
        ]);
        let (_events, accepted, tree) = ep.parse("1 + 2 + 3");

        assert!(accepted);
        assert_eq!(tree, "((1 + 2) + 3)");
    }

    #[test]
    fn test_precedence_right_assoc() {
        let mut gb = GrammarBuilder::new();
        let op = gb.pt("OP");
        let num = gb.t("NUM");
        let expr = gb.nt("expr");

        gb.rule(expr, vec![expr, op, expr]);
        gb.rule(expr, vec![num]);
        let grammar = gb.build();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        let ep = ExprParser::new(&table, vec![
            ("^", Precedence::right(1)),
        ]);
        let (_events, accepted, tree) = ep.parse("1 ^ 2 ^ 3");

        assert!(accepted);
        assert_eq!(tree, "(1 ^ (2 ^ 3))");
    }

    #[test]
    fn test_precedence_levels() {
        let mut gb = GrammarBuilder::new();
        let op = gb.pt("OP");
        let num = gb.t("NUM");
        let expr = gb.nt("expr");

        gb.rule(expr, vec![expr, op, expr]);
        gb.rule(expr, vec![num]);
        let grammar = gb.build();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        let ep = ExprParser::new(&table, vec![
            ("+", Precedence::left(1)),
            ("*", Precedence::left(2)),
        ]);
        let (_events, accepted, tree) = ep.parse("1 + 2 * 3");

        assert!(accepted);
        assert_eq!(tree, "(1 + (2 * 3))");
    }

    #[test]
    fn test_c_operator_precedence() {
        let mut gb = GrammarBuilder::new();
        let op = gb.pt("OP");
        let id = gb.t("ID");
        let expr = gb.nt("expr");

        gb.rule(expr, vec![expr, op, expr]);
        gb.rule(expr, vec![id]);
        let grammar = gb.build();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);
        assert!(!table.has_conflicts());

        let ep = ExprParser::new(&table, vec![
            ("=",  Precedence::right(1)),
            ("||", Precedence::left(2)),
            ("&&", Precedence::left(3)),
            ("|",  Precedence::left(4)),
            ("^",  Precedence::left(5)),
            ("&",  Precedence::left(6)),
            ("==", Precedence::left(7)),
            ("<",  Precedence::left(8)),
            ("+",  Precedence::left(9)),
            ("*",  Precedence::left(10)),
        ]);

        let (_events, accepted, tree) = ep.parse("a = b || c && d | e ^ f & g == h < i + j * k");

        assert!(accepted);
        assert_eq!(tree, "(a = (b || (c && (d | (e ^ (f & (g == (h < (i + (j * k))))))))))");
    }

    #[test]
    fn test_mixed_assoc() {
        let mut gb = GrammarBuilder::new();
        let op = gb.pt("OP");
        let id = gb.t("ID");
        let expr = gb.nt("expr");

        gb.rule(expr, vec![expr, op, expr]);
        gb.rule(expr, vec![id]);
        let grammar = gb.build();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        let ep = ExprParser::new(&table, vec![
            ("+", Precedence::left(2)),
            ("=", Precedence::right(1)),
        ]);

        let (_events, accepted, tree) = ep.parse("a + b + c = d = e");

        assert!(accepted);
        assert_eq!(tree, "(((a + b) + c) = (d = e))");
    }
}

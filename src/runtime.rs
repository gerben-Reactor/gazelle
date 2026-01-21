use crate::grammar::{Symbol, Precedence, SymbolId};
use crate::table::{Action, ParseTable, CompactParseTable};

/// A token with terminal symbol, value, and optional precedence.
#[derive(Debug, Clone)]
pub struct Token {
    pub terminal: Symbol,
    pub value: String,
    /// Precedence info for operators. Used to resolve shift/reduce conflicts at runtime.
    pub prec: Option<Precedence>,
}

impl Token {
    pub fn new(terminal: Symbol, value: impl Into<String>) -> Self {
        Self { terminal, value: value.into(), prec: None }
    }

    pub fn with_prec(terminal: Symbol, value: impl Into<String>, prec: Precedence) -> Self {
        Self { terminal, value: value.into(), prec: Some(prec) }
    }
}

/// Events emitted by the parser during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// A reduction occurred using the given rule.
    Reduce {
        /// Index of the rule that was reduced (in the augmented grammar).
        /// Rule 0 is __start -> original_start.
        /// User rules start at index 1.
        rule: usize,
        /// Number of symbols on the right-hand side.
        len: usize,
    },
    /// The input was accepted.
    Accept,
    /// A parse error occurred.
    Error {
        /// The unexpected token (None for EOF).
        token: Option<Symbol>,
        /// The current state.
        state: usize,
    },
}

use crate::grammar::Assoc;

/// Entry on the parser stack: state and optional precedence from shifted token.
#[derive(Debug, Clone, Copy)]
struct StackEntry {
    state: usize,
    /// Precedence level inherited from the token that caused this state to be pushed.
    /// Used for resolving shift/reduce conflicts on precedence terminals.
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
///
/// Tokens are pushed to the parser, and it emits events for each reduction,
/// accept, or error.
pub struct Parser<'a> {
    table: &'a ParseTable,
    /// Stack of (state, precedence) pairs.
    stack: Vec<StackEntry>,
}

impl<'a> Parser<'a> {
    /// Create a new parser with the given parse table.
    pub fn new(table: &'a ParseTable) -> Self {
        Self {
            table,
            stack: vec![StackEntry::new(0)], // Start in state 0
        }
    }

    /// Push a token to the parser and return events.
    ///
    /// This may return multiple events (e.g., several reductions followed by a shift).
    pub fn push(&mut self, token: &Token) -> Vec<Event> {
        let mut events = Vec::new();
        self.process(Some(token), &mut events);
        events
    }

    /// Signal end of input and return final events.
    pub fn finish(&mut self) -> Vec<Event> {
        let mut events = Vec::new();
        self.process(None, &mut events);
        events
    }

    fn process(&mut self, token: Option<&Token>, events: &mut Vec<Event>) {
        let terminal = token.map(|t| &t.terminal);

        loop {
            let entry = *self.stack.last().unwrap();
            let action = self.table.action(entry.state, terminal);

            match action {
                Action::Shift(next_state) => {
                    // Capture precedence from token, or inherit from previous top
                    let prec = token
                        .and_then(|t| t.prec.map(|p| p.level))
                        .or(entry.prec);
                    self.stack.push(StackEntry::with_prec(*next_state, prec));
                    break; // Consumed the token
                }
                Action::Reduce(rule_idx) => {
                    self.do_reduce(*rule_idx, events);
                    // Continue processing the same token
                }
                Action::ShiftOrReduce { shift_state, reduce_rule } => {
                    // Resolve based on precedence
                    let stack_prec = entry.prec;
                    let token_prec = token.and_then(|t| t.prec);

                    let should_shift = match (stack_prec, token_prec) {
                        (Some(sp), Some(tp)) => {
                            if tp.level > sp {
                                true // higher precedence: shift
                            } else if tp.level < sp {
                                false // lower precedence: reduce
                            } else {
                                // equal: use associativity
                                match tp.assoc {
                                    Assoc::Right => true,  // right-assoc: shift
                                    Assoc::Left => false,  // left-assoc: reduce
                                }
                            }
                        }
                        // No precedence info: default to shift (could also error)
                        _ => true,
                    };

                    if should_shift {
                        let prec = token
                            .and_then(|t| t.prec.map(|p| p.level))
                            .or(entry.prec);
                        self.stack.push(StackEntry::with_prec(*shift_state, prec));
                        break;
                    } else {
                        self.do_reduce(*reduce_rule, events);
                        // Continue processing the same token
                    }
                }
                Action::Accept => {
                    events.push(Event::Accept);
                    break;
                }
                Action::Error => {
                    events.push(Event::Error {
                        token: terminal.cloned(),
                        state: entry.state,
                    });
                    break;
                }
            }
        }
    }

    fn do_reduce(&mut self, rule_idx: usize, events: &mut Vec<Event>) {
        let rule = &self.table.grammar.rules[rule_idx];
        let len = rule.rhs.len();

        // Pop entries for the RHS symbols
        for _ in 0..len {
            self.stack.pop();
        }

        // Stack should never be empty here: the augmented start rule (__start -> S)
        // gets Action::Accept instead of Action::Reduce, so we never pop down to
        // an empty stack during reduction.
        debug_assert!(
            !self.stack.is_empty(),
            "stack empty after reduction - augmented start rule should use Accept, not Reduce"
        );

        let goto_entry = self.stack.last().unwrap();
        if let Some(next_state) = self.table.goto(goto_entry.state, &rule.lhs) {
            // Inherit precedence from the new top of stack
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

// ============================================================================
// Compact runtime types using SymbolId and CompactParseTable
// ============================================================================

/// A token using interned symbol IDs.
#[derive(Debug, Clone)]
pub struct CompactToken {
    pub terminal: SymbolId,
    pub value: String,
    /// Precedence info for operators.
    pub prec: Option<Precedence>,
}

impl CompactToken {
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

/// Events emitted by the compact parser.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompactEvent {
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

/// Stack entry for compact parser.
#[derive(Debug, Clone, Copy)]
struct CompactStackEntry {
    state: usize,
    prec: Option<u8>,
}

impl CompactStackEntry {
    fn new(state: usize) -> Self {
        Self { state, prec: None }
    }

    fn with_prec(state: usize, prec: Option<u8>) -> Self {
        Self { state, prec }
    }
}

/// A push-based LR parser using CompactParseTable for O(1) lookups.
pub struct CompactParser<'a> {
    table: &'a CompactParseTable,
    stack: Vec<CompactStackEntry>,
}

impl<'a> CompactParser<'a> {
    /// Create a new parser with the given compact parse table.
    pub fn new(table: &'a CompactParseTable) -> Self {
        Self {
            table,
            stack: vec![CompactStackEntry::new(0)],
        }
    }

    /// Push a token to the parser and return events.
    pub fn push(&mut self, token: &CompactToken) -> Vec<CompactEvent> {
        let mut events = Vec::new();
        self.process(token, &mut events);
        events
    }

    /// Signal end of input and return final events.
    pub fn finish(&mut self) -> Vec<CompactEvent> {
        let mut events = Vec::new();
        let eof_token = CompactToken::eof();
        self.process(&eof_token, &mut events);
        events
    }

    fn process(&mut self, token: &CompactToken, events: &mut Vec<CompactEvent>) {
        let terminal = token.terminal;

        loop {
            let entry = *self.stack.last().unwrap();
            let action = self.table.action(entry.state, terminal);

            match action {
                Action::Shift(next_state) => {
                    let prec = token.prec.map(|p| p.level).or(entry.prec);
                    self.stack.push(CompactStackEntry::with_prec(next_state, prec));
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
                                    crate::grammar::Assoc::Right => true,
                                    crate::grammar::Assoc::Left => false,
                                }
                            }
                        }
                        _ => true,
                    };

                    if should_shift {
                        let prec = token.prec.map(|p| p.level).or(entry.prec);
                        self.stack.push(CompactStackEntry::with_prec(shift_state, prec));
                        break;
                    } else {
                        self.do_reduce(reduce_rule, events);
                    }
                }
                Action::Accept => {
                    events.push(CompactEvent::Accept);
                    break;
                }
                Action::Error => {
                    events.push(CompactEvent::Error {
                        terminal,
                        state: entry.state,
                    });
                    break;
                }
            }
        }
    }

    fn do_reduce(&mut self, rule_idx: usize, events: &mut Vec<CompactEvent>) {
        let (lhs, len) = self.table.rule_info(rule_idx);

        for _ in 0..len {
            self.stack.pop();
        }

        debug_assert!(!self.stack.is_empty());

        let goto_entry = self.stack.last().unwrap();
        if let Some(next_state) = self.table.goto(goto_entry.state, lhs) {
            self.stack.push(CompactStackEntry::with_prec(next_state, goto_entry.prec));
        }

        events.push(CompactEvent::Reduce { rule: rule_idx, len });
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
    use crate::grammar::{t, pt, Precedence};
    use crate::lexer::{self, Token as LexToken};
    use crate::lr::Automaton;
    use crate::meta::parse_grammar;
    use std::collections::HashMap;

    /// Helper to parse expressions using the lexer.
    /// Builds a bracketed string showing the parse tree structure.
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

        /// Parse and return (events, accepted, bracketed_tree)
        fn parse(&self, input: &str) -> (Vec<Event>, bool, String) {
            let lex_tokens = lexer::lex(input).unwrap();
            let mut parser = Parser::new(self.table);
            let mut all_events = Vec::new();
            // Stack mirrors the parser stack: values and operators interleaved
            let mut stack: Vec<String> = Vec::new();

            for tok in lex_tokens {
                let parser_tok = match &tok {
                    LexToken::Num(s) => Token::new(t("NUM"), s.clone()),
                    LexToken::Ident(s) => Token::new(t("ID"), s.clone()),
                    LexToken::Op(s) => {
                        if let Some(&prec) = self.ops.get(s) {
                            Token::with_prec(pt("OP"), s.clone(), prec)
                        } else {
                            Token::new(t(s), s.clone())
                        }
                    }
                    _ => continue,
                };

                // Process reductions before pushing
                for event in parser.push(&parser_tok) {
                    if let Event::Reduce { rule, .. } = &event {
                        Self::apply_reduce(&mut stack, *rule);
                    }
                    all_events.push(event);
                }

                // Push value/op after reductions
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
            // Rule 0: __start -> expr (accept, no action)
            // Rule 1: expr -> expr OP expr (pop 3: right, op, left)
            // Rule 2: expr -> NUM/ID (pop 1, push back - no-op)
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
                2 => {
                    // Already on stack, nothing to do
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_parse_single_token() {
        let grammar = parse_grammar("S = 'a' ;").unwrap();
        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);
        let mut parser = Parser::new(&table);

        let events = parser.push(&Token::new(t("a"), "a"));
        assert!(events.is_empty());

        let events = parser.finish();
        assert!(events.iter().any(|e| matches!(e, Event::Reduce { rule: 1, len: 1, .. })));
        assert!(events.iter().any(|e| matches!(e, Event::Accept)));
    }

    #[test]
    fn test_parse_expr() {
        let grammar = parse_grammar(r#"
            expr = expr '+' term | term ;
            term = 'NUM' ;
        "#).unwrap();
        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        let ep = ExprParser::new(&table, vec![]);
        let (events, accepted, _tree) = ep.parse("1 + 2");

        assert!(accepted);
        assert!(events.iter().any(|e| matches!(e, Event::Reduce { rule: 1, .. }))); // expr -> expr + term
        assert!(events.iter().any(|e| matches!(e, Event::Reduce { rule: 3, .. }))); // term -> NUM
    }

    #[test]
    fn test_parse_error() {
        let grammar = parse_grammar("S = 'a' ;").unwrap();
        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);
        let mut parser = Parser::new(&table);

        let events = parser.push(&Token::new(t("b"), "b"));
        assert!(events.iter().any(|e| matches!(e, Event::Error { .. })));
    }

    #[test]
    fn test_precedence_left_assoc() {
        // "1 + 2 + 3" with left-assoc parses as "(1 + 2) + 3"
        let grammar = parse_grammar("expr = expr <OP> expr | 'NUM' ;").unwrap();
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
        // "1 ^ 2 ^ 3" with right-assoc parses as "1 ^ (2 ^ 3)"
        let grammar = parse_grammar("expr = expr <OP> expr | 'NUM' ;").unwrap();
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
        // "1 + 2 * 3" parses as "1 + (2 * 3)" because * has higher precedence
        let grammar = parse_grammar("expr = expr <OP> expr | 'NUM' ;").unwrap();
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
        // C-style expression with 10 precedence levels
        let grammar = parse_grammar("expr = expr <OP> expr | 'ID' ;").unwrap();
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
        // Each operator binds tighter than the one to its left
        assert_eq!(tree, "(a = (b || (c && (d | (e ^ (f & (g == (h < (i + (j * k))))))))))");
    }

    #[test]
    fn test_mixed_assoc() {
        // "a + b + c = d = e" with + left-assoc and = right-assoc
        // Parses as: ((a + b) + c) = (d = e)
        let grammar = parse_grammar("expr = expr <OP> expr | 'ID' ;").unwrap();
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

    // Tests for CompactParser
    use crate::table::CompactParseTable;

    #[test]
    fn test_compact_parse_single_token() {
        let grammar = parse_grammar("S = 'a' ;").unwrap();
        let table = CompactParseTable::from_grammar(&grammar);
        let mut parser = CompactParser::new(&table);

        let a_id = table.symbol_id("a").unwrap();
        let events = parser.push(&CompactToken::new(a_id, "a"));
        assert!(events.is_empty());

        let events = parser.finish();
        assert!(events.iter().any(|e| matches!(e, CompactEvent::Reduce { rule: 1, len: 1, .. })));
        assert!(events.iter().any(|e| matches!(e, CompactEvent::Accept)));
    }

    #[test]
    fn test_compact_parse_expr() {
        let grammar = parse_grammar(r#"
            expr = expr '+' term | term ;
            term = 'NUM' ;
        "#).unwrap();
        let table = CompactParseTable::from_grammar(&grammar);
        let mut parser = CompactParser::new(&table);

        let num_id = table.symbol_id("NUM").unwrap();
        let plus_id = table.symbol_id("+").unwrap();

        // Parse "1 + 2"
        let events = parser.push(&CompactToken::new(num_id, "1"));
        // Should get term -> NUM reduction
        let has_term_reduce = events.iter().any(|e| {
            matches!(e, CompactEvent::Reduce { len: 1, .. })
        });
        assert!(has_term_reduce || events.is_empty());

        let _events = parser.push(&CompactToken::new(plus_id, "+"));
        // Should get some reductions

        let _events = parser.push(&CompactToken::new(num_id, "2"));

        let events = parser.finish();
        assert!(events.iter().any(|e| matches!(e, CompactEvent::Accept)));
    }

    #[test]
    fn test_compact_parse_error() {
        let grammar = parse_grammar("S = 'a' ;").unwrap();
        let table = CompactParseTable::from_grammar(&grammar);
        let mut parser = CompactParser::new(&table);

        // Use an invalid terminal ID to trigger an error
        let wrong_id = SymbolId(99);  // Invalid terminal

        let events = parser.push(&CompactToken::new(wrong_id, "b"));
        assert!(events.iter().any(|e| matches!(e, CompactEvent::Error { .. })));
    }

    #[test]
    fn test_compact_precedence_left_assoc() {
        let grammar = parse_grammar("expr = expr <OP> expr | 'NUM' ;").unwrap();
        let table = CompactParseTable::from_grammar(&grammar);

        let num_id = table.symbol_id("NUM").unwrap();
        let op_id = table.symbol_id("OP").unwrap();

        // Parse "1 + 2 + 3" with left-associative +
        let mut parser = CompactParser::new(&table);
        let mut stack: Vec<String> = Vec::new();

        // Push 1
        for event in parser.push(&CompactToken::new(num_id, "1")) {
            apply_compact_reduce(&mut stack, &event);
        }
        stack.push("1".to_string());

        // Push +
        for event in parser.push(&CompactToken::with_prec(op_id, "+", Precedence::left(1))) {
            apply_compact_reduce(&mut stack, &event);
        }
        stack.push("+".to_string());

        // Push 2
        for event in parser.push(&CompactToken::new(num_id, "2")) {
            apply_compact_reduce(&mut stack, &event);
        }
        stack.push("2".to_string());

        // Push + (should trigger reduce due to left associativity)
        for event in parser.push(&CompactToken::with_prec(op_id, "+", Precedence::left(1))) {
            apply_compact_reduce(&mut stack, &event);
        }
        stack.push("+".to_string());

        // Push 3
        for event in parser.push(&CompactToken::new(num_id, "3")) {
            apply_compact_reduce(&mut stack, &event);
        }
        stack.push("3".to_string());

        // Finish
        for event in parser.finish() {
            apply_compact_reduce(&mut stack, &event);
        }

        let tree = stack.pop().unwrap_or_default();
        assert_eq!(tree, "((1 + 2) + 3)");
    }

    fn apply_compact_reduce(stack: &mut Vec<String>, event: &CompactEvent) {
        if let CompactEvent::Reduce { rule, .. } = event {
            match rule {
                0 => {} // __start -> expr
                1 => {
                    // expr -> expr OP expr
                    if stack.len() >= 3 {
                        let right = stack.pop().unwrap();
                        let op = stack.pop().unwrap();
                        let left = stack.pop().unwrap();
                        stack.push(format!("({} {} {})", left, op, right));
                    }
                }
                2 => {} // expr -> NUM (already on stack)
                _ => {}
            }
        }
    }
}

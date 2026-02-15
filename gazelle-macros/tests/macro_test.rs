//! Integration tests for the gazelle! macro.

use gazelle::Reduce;
use gazelle_macros::gazelle;

// Define a simple grammar for testing with the trait-based API
gazelle! {
    grammar Simple {
        start s;
        terminals {
            A
        }

        s = A => make_unit;
    }
}

// Implement the actions trait
struct SimpleActionsImpl;

impl SimpleTypes for SimpleActionsImpl {
    type Error = gazelle::ParseError;
    type S = ();
}

impl Reduce<SimpleS, (), gazelle::ParseError> for SimpleActionsImpl {
    fn reduce(&mut self, _node: SimpleS) -> Result<(), gazelle::ParseError> {
        Ok(())
    }
}

#[test]
fn test_simple_grammar_types() {
    let mut parser = SimpleParser::<SimpleActionsImpl>::new();
    let mut actions = SimpleActionsImpl;

    // Push the terminal - this handles reduction internally
    parser.push(SimpleTerminal::A, &mut actions).unwrap();

    // Finish and get result
    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert_eq!(result, ());
}

// Test a grammar with payload types
gazelle! {
    pub grammar NumParser {
        start value;
        terminals {
            NUM: Num
        }

        value = NUM => identity;
    }
}

struct NumActionsImpl;

impl NumParserTypes for NumActionsImpl {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Value = i32;
}

impl Reduce<NumParserValue<Self>, i32, gazelle::ParseError> for NumActionsImpl {
    fn reduce(&mut self, node: NumParserValue<Self>) -> Result<i32, gazelle::ParseError> {
        let NumParserValue::Identity(n) = node;
        Ok(n)
    }
}

#[test]
fn test_payload_grammar() {
    let mut parser = NumParserParser::<NumActionsImpl>::new();
    let mut actions = NumActionsImpl;

    // Push the terminal
    parser.push(NumParserTerminal::NUM(42), &mut actions).unwrap();

    // Finish and get result
    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert_eq!(result, 42);
}

// Test a more complex grammar with named reductions
gazelle! {
    grammar Expr {
        start expr;
        terminals {
            NUM: Num,
            PLUS
        }

        expr = expr PLUS term => add
             | term => term_to_expr;

        term = NUM => literal;
    }
}

struct ExprActionsImpl;

impl ExprTypes for ExprActionsImpl {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Expr = i32;
    type Term = i32;
}

impl Reduce<ExprExpr<Self>, i32, gazelle::ParseError> for ExprActionsImpl {
    fn reduce(&mut self, node: ExprExpr<Self>) -> Result<i32, gazelle::ParseError> {
        Ok(match node {
            ExprExpr::Add(left, right) => left + right,
            ExprExpr::Term_to_expr(t) => t,
        })
    }
}

impl Reduce<ExprTerm<Self>, i32, gazelle::ParseError> for ExprActionsImpl {
    fn reduce(&mut self, node: ExprTerm<Self>) -> Result<i32, gazelle::ParseError> {
        let ExprTerm::Literal(n) = node;
        Ok(n)
    }
}

#[test]
fn test_expr_grammar() {
    let mut parser = ExprParser::<ExprActionsImpl>::new();
    let mut actions = ExprActionsImpl;

    // Parse: 1 + 2
    parser.push(ExprTerminal::NUM(1), &mut actions).unwrap();
    parser.push(ExprTerminal::PLUS, &mut actions).unwrap();
    parser.push(ExprTerminal::NUM(2), &mut actions).unwrap();

    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert_eq!(result, 3);
}

// Test set_token_range callback
struct SpanTracker {
    spans: Vec<(usize, usize)>,
}

impl ExprTypes for SpanTracker {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Expr = i32;
    type Term = i32;

    fn set_token_range(&mut self, start: usize, end: usize) {
        self.spans.push((start, end));
    }
}

impl Reduce<ExprExpr<Self>, i32, gazelle::ParseError> for SpanTracker {
    fn reduce(&mut self, node: ExprExpr<Self>) -> Result<i32, gazelle::ParseError> {
        Ok(match node {
            ExprExpr::Add(left, right) => left + right,
            ExprExpr::Term_to_expr(t) => t,
        })
    }
}

impl Reduce<ExprTerm<Self>, i32, gazelle::ParseError> for SpanTracker {
    fn reduce(&mut self, node: ExprTerm<Self>) -> Result<i32, gazelle::ParseError> {
        let ExprTerm::Literal(n) = node;
        Ok(n)
    }
}

#[test]
fn test_set_token_range() {
    let mut parser = ExprParser::<SpanTracker>::new();
    let mut actions = SpanTracker { spans: Vec::new() };

    // Parse: 1 + 2   (tokens at indices 0, 1, 2)
    parser.push(ExprTerminal::NUM(1), &mut actions).unwrap();
    parser.push(ExprTerminal::PLUS, &mut actions).unwrap();
    parser.push(ExprTerminal::NUM(2), &mut actions).unwrap();

    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert_eq!(result, 3);

    // Reductions: term(0,1), expr(0,1), term(2,3), expr(0,3)
    assert!(actions.spans.contains(&(0, 1)), "term '1' span: {:?}", actions.spans);
    assert!(actions.spans.contains(&(2, 3)), "term '2' span: {:?}", actions.spans);
    assert!(actions.spans.contains(&(0, 3)), "expr '1+2' span: {:?}", actions.spans);
}

// Test separator list (%)
gazelle! {
    grammar CsvList {
        start items;
        terminals {
            NUM: Num,
            COMMA
        }

        items = (NUM % COMMA) => items;
    }
}

struct CsvActionsImpl;

impl CsvListTypes for CsvActionsImpl {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Items = Vec<i32>;
}

impl Reduce<CsvListItems<Self>, Vec<i32>, gazelle::ParseError> for CsvActionsImpl {
    fn reduce(&mut self, node: CsvListItems<Self>) -> Result<Vec<i32>, gazelle::ParseError> {
        let CsvListItems::Items(nums) = node;
        Ok(nums)
    }
}

#[test]
fn test_separator_single() {
    let mut parser = CsvListParser::<CsvActionsImpl>::new();
    let mut actions = CsvActionsImpl;
    parser.push(CsvListTerminal::NUM(42), &mut actions).unwrap();
    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert_eq!(result, vec![42]);
}

#[test]
fn test_separator_multiple() {
    let mut parser = CsvListParser::<CsvActionsImpl>::new();
    let mut actions = CsvActionsImpl;
    parser.push(CsvListTerminal::NUM(1), &mut actions).unwrap();
    parser.push(CsvListTerminal::COMMA, &mut actions).unwrap();
    parser.push(CsvListTerminal::NUM(2), &mut actions).unwrap();
    parser.push(CsvListTerminal::COMMA, &mut actions).unwrap();
    parser.push(CsvListTerminal::NUM(3), &mut actions).unwrap();
    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert_eq!(result, vec![1, 2, 3]);
}

// Test passthrough (same non-terminal to same non-terminal)
gazelle! {
    grammar Paren {
        start expr;
        terminals {
            NUM: Num,
            LPAREN,
            RPAREN
        }

        expr = LPAREN expr RPAREN => paren
             | NUM => literal;
    }
}

struct ParenActionsImpl;

impl ParenTypes for ParenActionsImpl {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Expr = i32;
}

impl Reduce<ParenExpr<Self>, i32, gazelle::ParseError> for ParenActionsImpl {
    fn reduce(&mut self, node: ParenExpr<Self>) -> Result<i32, gazelle::ParseError> {
        Ok(match node {
            ParenExpr::Paren(e) => e,
            ParenExpr::Literal(n) => n,
        })
    }
}

#[test]
fn test_passthrough() {
    let mut parser = ParenParser::<ParenActionsImpl>::new();
    let mut actions = ParenActionsImpl;

    // Parse: (42)
    parser.push(ParenTerminal::LPAREN, &mut actions).unwrap();
    parser.push(ParenTerminal::NUM(42), &mut actions).unwrap();
    parser.push(ParenTerminal::RPAREN, &mut actions).unwrap();

    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert_eq!(result, 42);
}

//! Integration tests for the grammar! macro.

use gazelle_macros::grammar;

// Define a simple grammar for testing with the trait-based API
grammar! {
    grammar Simple {
        start s;
        terminals {
            A,
        }

        s: S = A @make_unit;
    }
}

// Implement the actions trait
struct SimpleActionsImpl;

impl SimpleTypes for SimpleActionsImpl {
    type S = ();
}

impl SimpleActions for SimpleActionsImpl {
    fn make_unit(&mut self) -> Result<(), gazelle::ParseError> {
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
// Terminal to non-terminal needs @name with trait-based API
grammar! {
    pub grammar NumParser {
        start value;
        terminals {
            NUM: Num,
        }

        value: Value = NUM @identity;
    }
}

struct NumActionsImpl;

impl NumParserTypes for NumActionsImpl {
    type Num = i32;
    type Value = i32;
}

impl NumParserActions for NumActionsImpl {
    fn identity(&mut self, n: i32) -> Result<i32, gazelle::ParseError> {
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
// Note: Each non-terminal has its own associated type, so term->expr
// needs @name to convert between types
grammar! {
    grammar Expr {
        start expr;
        terminals {
            NUM: Num,
            PLUS,
        }

        expr: Expr = expr PLUS term @add
                   | term @term_to_expr;  // need @name for type conversion

        term: Term = NUM @literal;
    }
}

struct ExprActionsImpl;

impl ExprTypes for ExprActionsImpl {
    type Num = i32;
    type Expr = i32;
    type Term = i32;
}

impl ExprActions for ExprActionsImpl {
    fn add(&mut self, left: Self::Expr, right: Self::Term) -> Result<Self::Expr, gazelle::ParseError> {
        Ok(left + right)
    }

    fn term_to_expr(&mut self, t: Self::Term) -> Result<Self::Expr, gazelle::ParseError> {
        Ok(t)
    }

    fn literal(&mut self, n: i32) -> Result<Self::Term, gazelle::ParseError> {
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

// Test separator list (%)
grammar! {
    grammar CsvList {
        start items;
        terminals {
            NUM: Num,
            COMMA,
        }

        items: Items = NUM % COMMA @items;
    }
}

struct CsvActionsImpl;

impl CsvListTypes for CsvActionsImpl {
    type Num = i32;
    type Items = Vec<i32>;
}

impl CsvListActions for CsvActionsImpl {
    fn items(&mut self, nums: Vec<i32>) -> Result<Vec<i32>, gazelle::ParseError> {
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
grammar! {
    grammar Paren {
        start expr;
        terminals {
            NUM: Num,
            LPAREN,
            RPAREN,
        }

        expr: Expr = LPAREN expr RPAREN  // passthrough - expr to expr
                   | NUM @literal;
    }
}

struct ParenActionsImpl;

impl ParenTypes for ParenActionsImpl {
    type Num = i32;
    type Expr = i32;
}

impl ParenActions for ParenActionsImpl {
    fn literal(&mut self, n: i32) -> Result<Self::Expr, gazelle::ParseError> {
        Ok(n)
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

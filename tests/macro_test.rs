//! Integration tests for the grammar! macro.

use gazelle::grammar;

// Define a simple grammar for testing with the trait-based API
grammar! {
    grammar Simple {
        terminals {
            A,
        }

        s: () = A @make_unit;
    }
}

// Implement the actions trait
struct SimpleActionsImpl;

impl SimpleActions for SimpleActionsImpl {
    type S = ();

    fn make_unit(&mut self) -> () {
        ()
    }
}

#[test]
fn test_simple_grammar_types() {
    let mut parser = SimpleParser::<SimpleActionsImpl>::new();
    let mut actions = SimpleActionsImpl;

    // Push the terminal - this handles reduction internally
    parser.push(SimpleTerminal::A, &mut actions).unwrap();

    // Finish and get result
    let result = parser.finish(&mut actions).unwrap();
    assert_eq!(result, ());
}

// Test a grammar with payload types
// Terminal to non-terminal needs @name with trait-based API
grammar! {
    pub grammar NumParser {
        terminals {
            NUM: i32,
        }

        value: i32 = NUM @identity;
    }
}

struct NumActionsImpl;

impl NumParserActions for NumActionsImpl {
    type Value = i32;

    fn identity(&mut self, n: i32) -> i32 {
        n
    }
}

#[test]
fn test_payload_grammar() {
    let mut parser = NumParserParser::<NumActionsImpl>::new();
    let mut actions = NumActionsImpl;

    // Push the terminal
    parser.push(NumParserTerminal::Num(42), &mut actions).unwrap();

    // Finish and get result
    let result = parser.finish(&mut actions).unwrap();
    assert_eq!(result, 42);
}

// Test a more complex grammar with named reductions
// Note: Each non-terminal has its own associated type, so term->expr
// needs @name to convert between types
grammar! {
    grammar Expr {
        terminals {
            NUM: i32,
            PLUS,
        }

        expr: i32 = expr PLUS term @add
                  | term @term_to_expr;  // need @name for type conversion

        term: i32 = NUM @literal;
    }
}

struct ExprActionsImpl;

impl ExprActions for ExprActionsImpl {
    type Expr = i32;
    type Term = i32;

    fn add(&mut self, left: Self::Expr, right: Self::Term) -> Self::Expr {
        left + right
    }

    fn term_to_expr(&mut self, t: Self::Term) -> Self::Expr {
        t
    }

    fn literal(&mut self, n: i32) -> Self::Term {
        n
    }
}

#[test]
fn test_expr_grammar() {
    let mut parser = ExprParser::<ExprActionsImpl>::new();
    let mut actions = ExprActionsImpl;

    // Parse: 1 + 2
    parser.push(ExprTerminal::Num(1), &mut actions).unwrap();
    parser.push(ExprTerminal::Plus, &mut actions).unwrap();
    parser.push(ExprTerminal::Num(2), &mut actions).unwrap();

    let result = parser.finish(&mut actions).unwrap();
    assert_eq!(result, 3);
}

// Test passthrough (same non-terminal to same non-terminal)
grammar! {
    grammar Paren {
        terminals {
            NUM: i32,
            LPAREN,
            RPAREN,
        }

        expr: i32 = LPAREN expr RPAREN  // passthrough - expr to expr
                  | NUM @literal;
    }
}

struct ParenActionsImpl;

impl ParenActions for ParenActionsImpl {
    type Expr = i32;

    fn literal(&mut self, n: i32) -> Self::Expr {
        n
    }
}

#[test]
fn test_passthrough() {
    let mut parser = ParenParser::<ParenActionsImpl>::new();
    let mut actions = ParenActionsImpl;

    // Parse: (42)
    parser.push(ParenTerminal::Lparen, &mut actions).unwrap();
    parser.push(ParenTerminal::Num(42), &mut actions).unwrap();
    parser.push(ParenTerminal::Rparen, &mut actions).unwrap();

    let result = parser.finish(&mut actions).unwrap();
    assert_eq!(result, 42);
}

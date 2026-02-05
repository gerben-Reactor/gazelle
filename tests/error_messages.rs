//! Tests for parse error messages with toy grammars.

use gazelle::parse_grammar;
use gazelle::runtime::{Parser, Token};
use gazelle::table::CompiledTable;

/// Simple grammar: S -> a
#[test]
fn error_unexpected_token_simple() {
    let grammar = parse_grammar(r#"
        grammar Test {
            start S;
            terminals { a b }
            S = a;
        }
    "#).unwrap();

    let compiled = CompiledTable::build(&grammar);
    let mut parser = Parser::new(compiled.table());

    // Feed 'b' when 'a' is expected
    let b_id = compiled.symbol_id("b").unwrap();
    let token = Token::new(b_id);

    let err = parser.maybe_reduce(Some(&token)).unwrap_err();
    let msg = parser.format_error(&err, &compiled);

    assert_eq!(msg, "unexpected 'b', expected: a\n  in S:  \u{2022} a");
}

/// Simple grammar: S -> a, but we send EOF immediately
#[test]
fn error_unexpected_eof() {
    let grammar = parse_grammar(r#"
        grammar Test {
            start S;
            terminals { a }
            S = a;
        }
    "#).unwrap();

    let compiled = CompiledTable::build(&grammar);
    let mut parser = Parser::new(compiled.table());

    // Feed EOF when 'a' is expected
    let err = parser.maybe_reduce(None).unwrap_err();
    let msg = parser.format_error(&err, &compiled);

    assert_eq!(msg, "unexpected '$', expected: a\n  in S:  \u{2022} a");
}

/// Grammar with multiple expected tokens: S -> a | b
#[test]
fn error_multiple_expected() {
    let grammar = parse_grammar(r#"
        grammar Test {
            start S;
            terminals { a b c }
            S = a | b;
        }
    "#).unwrap();

    let compiled = CompiledTable::build(&grammar);
    let mut parser = Parser::new(compiled.table());

    // Feed 'c' when 'a' or 'b' is expected
    let c_id = compiled.symbol_id("c").unwrap();
    let token = Token::new(c_id);

    let err = parser.maybe_reduce(Some(&token)).unwrap_err();
    let msg = parser.format_error(&err, &compiled);

    assert_eq!(msg, "unexpected 'c', expected: a, b\n  in S:  \u{2022} a\n  in S:  \u{2022} b");
}

/// Sequence grammar: S -> a b c
#[test]
fn error_in_sequence() {
    let grammar = parse_grammar(r#"
        grammar Test {
            start S;
            terminals { a b c x }
            S = a b c;
        }
    "#).unwrap();

    let compiled = CompiledTable::build(&grammar);
    let mut parser = Parser::new(compiled.table());

    let a_id = compiled.symbol_id("a").unwrap();
    let x_id = compiled.symbol_id("x").unwrap();

    // Shift 'a'
    let token_a = Token::new(a_id);
    assert!(parser.maybe_reduce(Some(&token_a)).unwrap().is_none());
    parser.shift(&token_a);

    // Try 'x' when 'b' is expected
    let token_x = Token::new(x_id);
    let err = parser.maybe_reduce(Some(&token_x)).unwrap_err();
    let msg = parser.format_error(&err, &compiled);

    assert_eq!(msg, "unexpected 'x', expected: b\n  after: a\n  in S: a \u{2022} b c");
}

/// Expression grammar: E -> E PLUS NUM | NUM
#[test]
fn error_in_expression() {
    let grammar = parse_grammar(r#"
        grammar Test {
            start E;
            terminals { PLUS NUM STAR }
            E = E PLUS NUM | NUM;
        }
    "#).unwrap();

    let compiled = CompiledTable::build(&grammar);
    let mut parser = Parser::new(compiled.table());

    let num_id = compiled.symbol_id("NUM").unwrap();
    let plus_id = compiled.symbol_id("PLUS").unwrap();
    let star_id = compiled.symbol_id("STAR").unwrap();

    // Parse "NUM PLUS STAR" - error on STAR
    // Shift NUM
    let token_num = Token::new(num_id);
    while parser.maybe_reduce(Some(&token_num)).unwrap().is_some() {}
    parser.shift(&token_num);

    // Reduce E -> NUM, then check for PLUS
    let token_plus = Token::new(plus_id);
    while parser.maybe_reduce(Some(&token_plus)).unwrap().is_some() {}

    // Shift PLUS
    parser.shift(&token_plus);

    // Try STAR when NUM is expected after PLUS
    let token_star = Token::new(star_id);
    let err = parser.maybe_reduce(Some(&token_star)).unwrap_err();
    let msg = parser.format_error(&err, &compiled);

    assert_eq!(msg, "unexpected 'STAR', expected: NUM\n  after: E PLUS\n  in E: E PLUS \u{2022} NUM");
}

/// Test error at EOF after partial parse
#[test]
fn error_unexpected_eof_after_partial() {
    let grammar = parse_grammar(r#"
        grammar Test {
            start S;
            terminals { a b }
            S = a b;
        }
    "#).unwrap();

    let compiled = CompiledTable::build(&grammar);
    let mut parser = Parser::new(compiled.table());

    let a_id = compiled.symbol_id("a").unwrap();
    let token_a = Token::new(a_id);

    assert!(parser.maybe_reduce(Some(&token_a)).unwrap().is_none());
    parser.shift(&token_a);

    // Try EOF when 'b' is expected
    let err = parser.maybe_reduce(None).unwrap_err();
    let msg = parser.format_error(&err, &compiled);

    assert_eq!(msg, "unexpected '$', expected: b\n  after: a\n  in S: a \u{2022} b");
}

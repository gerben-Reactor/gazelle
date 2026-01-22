//! Integration tests for the grammar! macro.

use gazelle::grammar;

// Define a simple grammar for testing
grammar! {
    grammar Simple {
        terminals {
            A,
        }

        s: () = A;
    }
}

#[test]
fn test_simple_grammar_types() {
    let mut parser = SimpleParser::new();

    // Shift the terminal
    let tok: Option<SimpleTerminal> = Some(SimpleTerminal::A);
    assert!(parser.maybe_reduce(&tok).is_none());
    parser.shift(tok.unwrap()).unwrap();

    // At EOF, we should get a reduction
    let tok: Option<SimpleTerminal> = None;
    let reduction = parser.maybe_reduce(&tok).unwrap();

    // Use the constructor from the reduction
    let result = match reduction {
        SimpleReduction::SA(c) => c(()),
    };
    parser.reduce(result);

    // No more reductions
    assert!(parser.maybe_reduce(&tok).is_none());

    // Accept
    let result = parser.accept().unwrap();
    assert_eq!(result, ());
}

// Test a grammar with payload types
grammar! {
    pub grammar NumParser {
        terminals {
            NUM: i32,
        }

        value: i32 = NUM;
    }
}

#[test]
fn test_payload_grammar() {
    let mut parser = NumParserParser::new();

    // Shift
    let tok: Option<NumParserTerminal> = Some(NumParserTerminal::Num(42));
    assert!(parser.maybe_reduce(&tok).is_none());
    parser.shift(tok.unwrap()).unwrap();

    // At EOF, get reduction with constructor
    let tok: Option<NumParserTerminal> = None;
    let reduction = parser.maybe_reduce(&tok).unwrap();
    let result = match reduction {
        NumParserReduction::ValueNum(c, n) => {
            assert_eq!(n, 42);
            c(n)  // Just pass the number through
        }
    };
    parser.reduce(result);

    // Accept
    let result = parser.accept().unwrap();
    assert_eq!(result, 42);
}

//! Minimal example: parse and sum a list of numbers separated by '+'.
//!
//! ```text
//! cargo run --example hello
//! ```

use gazelle_macros::gazelle;

gazelle! {
    grammar sum {
        start expr;
        terminals {
            NUM: _,
            PLUS
        }

        expr = expr PLUS NUM => add
             | NUM => num;
    }
}

struct Eval;

impl sum::Types for Eval {
    type Error = gazelle::ParseError;
    type Num = i64;
    type Expr = i64;
}

impl gazelle::Action<sum::Expr<Self>> for Eval {
    fn build(&mut self, node: sum::Expr<Self>) -> Result<i64, gazelle::ParseError> {
        Ok(match node {
            sum::Expr::Add(left, right) => left + right,
            sum::Expr::Num(n) => n,
        })
    }
}

fn parse(input: &str) -> Result<i64, String> {
    use gazelle::lexer::Scanner;

    let mut src = Scanner::new(input);
    let mut parser = sum::Parser::<Eval>::new();
    let mut actions = Eval;

    loop {
        src.skip_whitespace();
        if src.at_end() {
            break;
        }
        let tok = if let Some(span) = src.read_digits() {
            let n: i64 = input[span].parse().map_err(|e| format!("{e}"))?;
            sum::Terminal::Num(n)
        } else if src.peek() == Some('+') {
            src.advance();
            sum::Terminal::Plus
        } else {
            return Err(format!("unexpected char: {:?}", src.peek()));
        };
        parser
            .push(tok, &mut actions)
            .map_err(|e| parser.format_error(&e, None, None))?;
    }

    parser
        .finish(&mut actions)
        .map_err(|(p, e)| p.format_error(&e, None, None))
}

fn main() {
    let input = "1 + 2 + 3";
    match parse(input) {
        Ok(result) => println!("{input} = {result}"),
        Err(e) => eprintln!("error: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        assert_eq!(parse("42").unwrap(), 42);
    }

    #[test]
    fn test_sum() {
        assert_eq!(parse("1 + 2 + 3").unwrap(), 6);
    }

    #[test]
    fn test_error() {
        assert!(parse("1 +").is_err());
    }
}

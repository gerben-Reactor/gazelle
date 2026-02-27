//! Expression evaluator to test dynamic precedence parsing.
//!
//! Uses the precedence-carrying non-terminal pattern: MINUS is a `prec`
//! terminal used both as binary subtract and unary negate. The grammar
//! handles the ambiguity, and precedence flows through the `binop`
//! non-terminal via single-symbol reduction.

use gazelle::Precedence;
use gazelle_macros::gazelle;

gazelle! {
    grammar expr {
        start expr;
        terminals {
            NUM: _,
            LPAREN, RPAREN,
            prec MINUS,
            prec OP: _
        }

        expr = term => term
             | expr binop expr => binop;

        binop = OP => op
              | MINUS => minus;

        term = NUM => num
             | LPAREN expr RPAREN => paren
             | MINUS term => neg;
    }
}

struct Eval;

impl expr::Types for Eval {
    type Error = gazelle::ParseError;
    type Num = i64;
    type Op = char;
    type Binop = char;
    type Term = i64;
    type Expr = i64;
}

impl gazelle::Action<expr::Binop<Self>> for Eval {
    fn build(&mut self, node: expr::Binop<Self>) -> Result<char, gazelle::ParseError> {
        Ok(match node {
            expr::Binop::Op(c) => c,
            expr::Binop::Minus => '-',
        })
    }
}

impl gazelle::Action<expr::Term<Self>> for Eval {
    fn build(&mut self, node: expr::Term<Self>) -> Result<i64, gazelle::ParseError> {
        Ok(match node {
            expr::Term::Num(n) => n,
            expr::Term::Paren(e) => e,
            expr::Term::Neg(e) => -e,
        })
    }
}

impl gazelle::Action<expr::Expr<Self>> for Eval {
    fn build(&mut self, node: expr::Expr<Self>) -> Result<i64, gazelle::ParseError> {
        Ok(match node {
            expr::Expr::Term(t) => t,
            expr::Expr::Binop(l, op, r) => match op {
                '|' => {
                    if l != 0 || r != 0 {
                        1
                    } else {
                        0
                    }
                }
                '&' => {
                    if l != 0 && r != 0 {
                        1
                    } else {
                        0
                    }
                }
                '=' => {
                    if l == r {
                        1
                    } else {
                        0
                    }
                }
                '!' => {
                    if l != r {
                        1
                    } else {
                        0
                    }
                }
                '<' => {
                    if l < r {
                        1
                    } else {
                        0
                    }
                }
                '>' => {
                    if l > r {
                        1
                    } else {
                        0
                    }
                }
                'L' => {
                    if l <= r {
                        1
                    } else {
                        0
                    }
                }
                'G' => {
                    if l >= r {
                        1
                    } else {
                        0
                    }
                }
                '+' => l + r,
                '-' => l - r,
                '*' => l * r,
                '/' => l / r,
                '%' => l % r,
                _ => panic!("unknown op: {}", op),
            },
        })
    }
}

fn eval(input: &str) -> Result<i64, String> {
    let mut parser = expr::Parser::<Eval>::new();
    let mut actions = Eval;

    let tokens = lex(input)?;
    for tok in tokens {
        parser
            .push(tok, &mut actions)
            .map_err(|e| parser.format_error(&e, None, None))?;
    }

    parser
        .finish(&mut actions)
        .map_err(|(p, e)| p.format_error(&e, None, None))
}

fn lex(input: &str) -> Result<Vec<expr::Terminal<Eval>>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '0'..='9' => {
                let mut num = 0i64;
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        num = num * 10 + (c as i64 - '0' as i64);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(expr::Terminal::Num(num));
            }
            '(' => {
                chars.next();
                tokens.push(expr::Terminal::Lparen);
            }
            ')' => {
                chars.next();
                tokens.push(expr::Terminal::Rparen);
            }
            '+' => {
                chars.next();
                tokens.push(expr::Terminal::Op('+', Precedence::Left(6)));
            }
            '-' => {
                chars.next();
                // Always MINUS with precedence — grammar handles unary vs binary.
                // For unary, the precedence doesn't matter: multi-symbol reduction
                // (MINUS term → neg) resets to anchor's precedence.
                tokens.push(expr::Terminal::Minus(Precedence::Left(6)));
            }
            '*' => {
                chars.next();
                tokens.push(expr::Terminal::Op('*', Precedence::Left(7)));
            }
            '/' => {
                chars.next();
                tokens.push(expr::Terminal::Op('/', Precedence::Left(7)));
            }
            '%' => {
                chars.next();
                tokens.push(expr::Terminal::Op('%', Precedence::Left(7)));
            }
            '|' => {
                chars.next();
                if chars.peek() == Some(&'|') {
                    chars.next();
                    tokens.push(expr::Terminal::Op('|', Precedence::Left(2)));
                } else {
                    return Err("Expected ||".into());
                }
            }
            '&' => {
                chars.next();
                if chars.peek() == Some(&'&') {
                    chars.next();
                    tokens.push(expr::Terminal::Op('&', Precedence::Left(3)));
                } else {
                    return Err("Expected &&".into());
                }
            }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(expr::Terminal::Op('=', Precedence::Left(4)));
                } else {
                    return Err("Expected ==".into());
                }
            }
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(expr::Terminal::Op('!', Precedence::Left(4)));
                } else {
                    return Err("Expected !=".into());
                }
            }
            '<' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(expr::Terminal::Op('L', Precedence::Left(5)));
                } else {
                    tokens.push(expr::Terminal::Op('<', Precedence::Left(5)));
                }
            }
            '>' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(expr::Terminal::Op('G', Precedence::Left(5)));
                } else {
                    tokens.push(expr::Terminal::Op('>', Precedence::Left(5)));
                }
            }
            _ => return Err(format!("Unexpected char: {}", c)),
        }
    }

    Ok(tokens)
}

fn main() {
    println!("Expression Evaluator - Dynamic Precedence Test");
    println!();

    let tests = [
        ("1 + 2 * 3", 7),      // * binds tighter: 1 + (2 * 3)
        ("2 * 3 + 1", 7),      // same result
        ("(1 + 2) * 3", 9),    // parens override
        ("10 - 3 - 2", 5),     // left-assoc: (10 - 3) - 2
        ("2 * 3 * 4", 24),     // left-assoc
        ("1 + 2 + 3 * 4", 15), // 1 + 2 + 12 = 15
        ("1 < 2", 1),          // comparison
        ("2 < 1", 0),
        ("1 + 1 == 2", 1),       // + before ==: (1+1) == 2
        ("1 == 1 && 2 == 2", 1), // == before &&
        ("1 || 0 && 0", 1),      // && before ||: 1 || (0 && 0) = 1
        ("0 || 0 && 1", 0),      // 0 || (0 && 1) = 0
        ("-5", -5),              // unary minus
        ("--5", 5),              // double negative
        ("2 * -3", -6),          // unary in expression
        ("1 - 2 - 3", -4),       // left-assoc: (1-2)-3 = -4
        ("100 / 10 / 2", 5),     // left-assoc: (100/10)/2 = 5
    ];

    let mut passed = 0;
    let mut failed = 0;

    for (expr, expected) in tests {
        match eval(expr) {
            Ok(result) if result == expected => {
                println!("PASS: {} = {}", expr, result);
                passed += 1;
            }
            Ok(result) => {
                println!("FAIL: {} = {} (expected {})", expr, result, expected);
                failed += 1;
            }
            Err(e) => {
                println!("ERROR: {} -> {}", expr, e);
                failed += 1;
            }
        }
    }

    println!();
    println!("{} passed, {} failed", passed, failed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precedence() {
        assert_eq!(eval("1 + 2 * 3").unwrap(), 7); // * before +
        assert_eq!(eval("2 * 3 + 1").unwrap(), 7);
        assert_eq!(eval("(1 + 2) * 3").unwrap(), 9);
    }

    #[test]
    fn test_associativity() {
        assert_eq!(eval("10 - 3 - 2").unwrap(), 5); // left-assoc: (10-3)-2
        assert_eq!(eval("2 * 3 * 4").unwrap(), 24);
        assert_eq!(eval("1 - 2 - 3").unwrap(), -4); // (1-2)-3 = -4
        assert_eq!(eval("100 / 10 / 2").unwrap(), 5); // (100/10)/2 = 5
    }

    #[test]
    fn test_comparison_precedence() {
        assert_eq!(eval("1 + 1 == 2").unwrap(), 1); // + before ==
        assert_eq!(eval("1 == 1 && 2 == 2").unwrap(), 1); // == before &&
        assert_eq!(eval("1 || 0 && 0").unwrap(), 1); // && before ||
    }

    #[test]
    fn test_unary() {
        assert_eq!(eval("-5").unwrap(), -5);
        assert_eq!(eval("--5").unwrap(), 5);
        assert_eq!(eval("2 * -3").unwrap(), -6);
        assert_eq!(eval("- 2 + 3").unwrap(), 1); // (-2)+3
        assert_eq!(eval("2 + -3 + 4").unwrap(), 3); // 2+(-3)+4
        assert_eq!(eval("2 * -3 + 4").unwrap(), -2); // (2*(-3))+4
        assert_eq!(eval("2 + -3 == -1").unwrap(), 1); // (2+(-3)) == (-1)
    }
}

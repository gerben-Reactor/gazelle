//! Expression evaluator to test dynamic precedence parsing.
//!
//! Extracts the expression precedence pattern from C11 and verifies
//! correct bracketing by evaluating arithmetic expressions.

use gazelle::Precedence;
use gazelle_macros::grammar;

grammar! {
    grammar Expr {
        start expr;
        terminals {
            NUM: i64,
            LPAREN, RPAREN, COLON,
            MINUS,  // unary minus (non-prec)
            // Single prec terminal for all binary ops
            prec OP: char,
        }

        // Single rule for all binary expressions + ternary
        expr: i64 = term @eval_term
                  | expr OP expr @eval_binop;

        term: i64 = NUM @eval_num
                  | LPAREN expr RPAREN @eval_paren
                  | MINUS term @eval_neg;
    }
}

struct Eval;

impl ExprActions for Eval {
    type Num = i64;
    type Op = char;
    type Term = i64;
    type Expr = i64;

    fn eval_num(&mut self, n: i64) -> i64 { n }
    fn eval_paren(&mut self, e: i64) -> i64 { e }
    fn eval_neg(&mut self, e: i64) -> i64 { -e }
    fn eval_term(&mut self, t: i64) -> i64 { t }

    fn eval_binop(&mut self, l: i64, op: char, r: i64) -> i64 {
        match op {
            // Ternary uses special handling below
            '?' => unreachable!("ternary handled specially"),
            // Logical
            '|' => if l != 0 || r != 0 { 1 } else { 0 },  // ||
            '&' => if l != 0 && r != 0 { 1 } else { 0 },  // &&
            // Comparison
            '=' => if l == r { 1 } else { 0 },  // ==
            '!' => if l != r { 1 } else { 0 },  // !=
            '<' => if l < r { 1 } else { 0 },
            '>' => if l > r { 1 } else { 0 },
            'L' => if l <= r { 1 } else { 0 },  // <=
            'G' => if l >= r { 1 } else { 0 },  // >=
            // Arithmetic
            '+' => l + r,
            '-' => l - r,
            '*' => l * r,
            '/' => l / r,
            '%' => l % r,
            _ => panic!("unknown op: {}", op),
        }
    }
}

fn eval(input: &str) -> Result<i64, String> {
    let mut parser = ExprParser::<Eval>::new();
    let mut actions = Eval;

    let tokens = lex(input)?;
    for tok in tokens {
        parser.push(tok, &mut actions).map_err(|e| format!("{:?}", e))?;
    }

    parser.finish(&mut actions).map_err(|e| format!("{:?}", e))
}

fn lex(input: &str) -> Result<Vec<ExprTerminal<Eval>>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => { chars.next(); }
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
                tokens.push(ExprTerminal::Num(num));
            }
            '(' => { chars.next(); tokens.push(ExprTerminal::Lparen); }
            ')' => { chars.next(); tokens.push(ExprTerminal::Rparen); }
            ':' => { chars.next(); tokens.push(ExprTerminal::Colon); }
            '+' => {
                chars.next();
                tokens.push(ExprTerminal::Op('+', Precedence::left(6)));
            }
            '-' => {
                chars.next();
                // Unary if start or after operator/lparen/unary-minus
                let is_unary = tokens.last().map(|t| matches!(t,
                    ExprTerminal::Op(_, _) | ExprTerminal::Lparen | ExprTerminal::Minus
                )).unwrap_or(true);
                if is_unary {
                    tokens.push(ExprTerminal::Minus);
                } else {
                    tokens.push(ExprTerminal::Op('-', Precedence::left(6)));
                }
            }
            '*' => { chars.next(); tokens.push(ExprTerminal::Op('*', Precedence::left(7))); }
            '/' => { chars.next(); tokens.push(ExprTerminal::Op('/', Precedence::left(7))); }
            '%' => { chars.next(); tokens.push(ExprTerminal::Op('%', Precedence::left(7))); }
            '|' => {
                chars.next();
                if chars.peek() == Some(&'|') {
                    chars.next();
                    tokens.push(ExprTerminal::Op('|', Precedence::left(2)));
                } else {
                    return Err("Expected ||".into());
                }
            }
            '&' => {
                chars.next();
                if chars.peek() == Some(&'&') {
                    chars.next();
                    tokens.push(ExprTerminal::Op('&', Precedence::left(3)));
                } else {
                    return Err("Expected &&".into());
                }
            }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(ExprTerminal::Op('=', Precedence::left(4)));
                } else {
                    return Err("Expected ==".into());
                }
            }
            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(ExprTerminal::Op('!', Precedence::left(4)));
                } else {
                    return Err("Expected !=".into());
                }
            }
            '<' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(ExprTerminal::Op('L', Precedence::left(5)));  // <=
                } else {
                    tokens.push(ExprTerminal::Op('<', Precedence::left(5)));
                }
            }
            '>' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(ExprTerminal::Op('G', Precedence::left(5)));  // >=
                } else {
                    tokens.push(ExprTerminal::Op('>', Precedence::left(5)));
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
        ("1 + 2 * 3", 7),           // * binds tighter: 1 + (2 * 3)
        ("2 * 3 + 1", 7),           // same result
        ("(1 + 2) * 3", 9),         // parens override
        ("10 - 3 - 2", 5),          // left-assoc: (10 - 3) - 2
        ("2 * 3 * 4", 24),          // left-assoc
        ("1 + 2 + 3 * 4", 15),      // 1 + 2 + 12 = 15
        ("1 < 2", 1),               // comparison
        ("2 < 1", 0),
        ("1 + 1 == 2", 1),          // + before ==: (1+1) == 2
        ("1 == 1 && 2 == 2", 1),    // == before &&
        ("1 || 0 && 0", 1),         // && before ||: 1 || (0 && 0) = 1
        ("0 || 0 && 1", 0),         // 0 || (0 && 1) = 0
        ("-5", -5),                 // unary minus
        ("--5", 5),                 // double negative
        ("2 * -3", -6),             // unary in expression
        ("1 - 2 - 3", -4),          // left-assoc: (1-2)-3 = -4
        ("100 / 10 / 2", 5),        // left-assoc: (100/10)/2 = 5
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
        assert_eq!(eval("1 + 2 * 3").unwrap(), 7);   // * before +
        assert_eq!(eval("2 * 3 + 1").unwrap(), 7);
        assert_eq!(eval("(1 + 2) * 3").unwrap(), 9);
    }

    #[test]
    fn test_associativity() {
        assert_eq!(eval("10 - 3 - 2").unwrap(), 5);  // left-assoc: (10-3)-2
        assert_eq!(eval("2 * 3 * 4").unwrap(), 24);
        assert_eq!(eval("1 - 2 - 3").unwrap(), -4);  // (1-2)-3 = -4
        assert_eq!(eval("100 / 10 / 2").unwrap(), 5); // (100/10)/2 = 5
    }

    #[test]
    fn test_comparison_precedence() {
        assert_eq!(eval("1 + 1 == 2").unwrap(), 1);      // + before ==
        assert_eq!(eval("1 == 1 && 2 == 2").unwrap(), 1); // == before &&
        assert_eq!(eval("1 || 0 && 0").unwrap(), 1);      // && before ||
    }

    #[test]
    fn test_unary() {
        assert_eq!(eval("-5").unwrap(), -5);
        assert_eq!(eval("--5").unwrap(), 5);
        assert_eq!(eval("2 * -3").unwrap(), -6);
    }
}

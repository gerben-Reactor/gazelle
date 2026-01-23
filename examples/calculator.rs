//! A simple calculator demonstrating the trait-based parser API.
//!
//! This example shows:
//! 1. Defining a grammar with the `grammar!` macro
//! 2. Implementing the generated Actions trait
//! 3. Using the parser with push/finish API
//!
//! The grammar supports:
//! - Numbers (integers and decimals)
//! - Binary operators with runtime precedence
//! - Parenthesized expressions

use gazelle::grammar;
use gazelle_core::Precedence;

grammar! {
    pub grammar Calc {
        start expr;
        terminals {
            NUM: f64,
            LPAREN,
            RPAREN,
            prec OP: char,
        }

        // Expression with runtime precedence
        expr: Expr = expr OP expr @binop
                   | NUM @literal
                   | LPAREN expr RPAREN;  // passthrough
    }
}

/// AST for expressions.
#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    BinOp(Box<Expr>, char, Box<Expr>),
}

/// Actions implementation - builds an AST then evaluates it.
struct Evaluator;

impl CalcActions for Evaluator {
    type Expr = f64;  // Evaluate directly to numbers

    fn binop(&mut self, left: f64, op: char, right: f64) -> f64 {
        match op {
            '+' => left + right,
            '-' => left - right,
            '*' => left * right,
            '/' => left / right,
            '^' => left.powf(right),
            _ => panic!("unknown operator: {}", op),
        }
    }

    fn literal(&mut self, n: f64) -> f64 {
        n
    }
}

/// Simple lexer that yields CalcTerminal tokens.
struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let c = self.input[self.pos..].chars().next().unwrap();
            if c.is_whitespace() {
                self.pos += c.len_utf8();
            } else {
                break;
            }
        }
    }

    fn next(&mut self) -> Option<CalcTerminal> {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return None;
        }

        let remaining = &self.input[self.pos..];
        let c = remaining.chars().next().unwrap();

        // Number
        if c.is_ascii_digit() || c == '.' {
            let end = remaining.find(|c: char| !c.is_ascii_digit() && c != '.')
                .unwrap_or(remaining.len());
            let num_str = &remaining[..end];
            self.pos += end;
            let num: f64 = num_str.parse().unwrap();
            return Some(CalcTerminal::Num(num));
        }

        // Single character tokens
        self.pos += 1;
        match c {
            '(' => Some(CalcTerminal::Lparen),
            ')' => Some(CalcTerminal::Rparen),
            '+' => Some(CalcTerminal::Op('+', Precedence::left(1))),
            '-' => Some(CalcTerminal::Op('-', Precedence::left(1))),
            '*' => Some(CalcTerminal::Op('*', Precedence::left(2))),
            '/' => Some(CalcTerminal::Op('/', Precedence::left(2))),
            '^' => Some(CalcTerminal::Op('^', Precedence::right(3))),  // right-assoc, high prec
            _ => panic!("unexpected character: {}", c),
        }
    }
}

fn eval(input: &str) -> f64 {
    let mut lexer = Lexer::new(input);
    let mut parser = CalcParser::<Evaluator>::new();
    let mut actions = Evaluator;

    // Feed tokens to parser
    while let Some(tok) = lexer.next() {
        parser.push(tok, &mut actions).expect("parse error");
    }

    // Finish and get result
    parser.finish(&mut actions).expect("parse error")
}

fn main() {
    let tests = [
        ("1 + 2", 3.0),
        ("2 * 3 + 4", 10.0),
        ("2 + 3 * 4", 14.0),
        ("(2 + 3) * 4", 20.0),
        ("2 ^ 3", 8.0),
        ("2 ^ 3 ^ 2", 512.0),  // right-assoc: 2^(3^2) = 2^9
        ("10 / 2 / 5", 1.0),   // left-assoc: (10/2)/5
    ];

    println!("Calculator Examples:");
    println!("====================\n");

    for (expr, expected) in tests {
        let result = eval(expr);
        let status = if (result - expected).abs() < 0.0001 { "✓" } else { "✗" };
        println!("{} {} = {} (expected {})", status, expr, result, expected);
    }
}

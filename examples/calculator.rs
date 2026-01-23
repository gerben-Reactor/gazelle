//! A simple calculator with variable assignments and user-defined operators.
//!
//! Demonstrates Gazelle's runtime operator precedence - operators like `^` can be
//! defined at runtime with custom precedence and associativity.

use std::cell::RefCell;
use std::collections::HashMap;
use gazelle::Precedence;
use gazelle_macros::grammar;

grammar! {
    grammar Calc {
        start stmts;
        terminals {
            NUM: f64,
            IDENT: String,
            LPAREN,
            RPAREN,
            COMMA,
            SEMI,
            OPERATOR,
            LEFT,
            RIGHT,
            prec OP: char,
        }

        stmts = stmts SEMI stmt | stmt | ;
        stmt = OPERATOR OP IDENT LEFT NUM @def_left
             | OPERATOR OP IDENT RIGHT NUM @def_right
             | expr @print;

        expr: Value = expr OP expr @binop
                    | NUM @literal
                    | IDENT LPAREN expr COMMA expr RPAREN @call
                    | IDENT @var
                    | LPAREN expr RPAREN;
    }
}

/// Value during evaluation - either a number or an unresolved variable (for assignment LHS).
#[derive(Clone)]
enum Value {
    Num(f64),
    Var(String),
}

impl Value {
    fn to_f64(&self, vars: &HashMap<String, f64>) -> f64 {
        match self {
            Value::Num(n) => *n,
            Value::Var(name) => *vars.get(name).unwrap_or(&f64::NAN),
        }
    }
}

/// User-defined operator.
#[derive(Clone)]
struct OpDef {
    func: String,
    prec: Precedence,
}

struct Evaluator<'a> {
    vars: &'a RefCell<HashMap<String, f64>>,
    ops: &'a RefCell<HashMap<char, OpDef>>,
}

impl<'a> CalcActions for Evaluator<'a> {
    type Expr = Value;

    fn def_left(&mut self, op: char, func: String, prec: f64) {
        println!("defined: {} = {} left {}", op, func, prec as u8);
        self.ops.borrow_mut().insert(op, OpDef { func, prec: Precedence::left(prec as u8) });
    }

    fn def_right(&mut self, op: char, func: String, prec: f64) {
        println!("defined: {} = {} right {}", op, func, prec as u8);
        self.ops.borrow_mut().insert(op, OpDef { func, prec: Precedence::right(prec as u8) });
    }

    fn print(&mut self, val: Value) {
        match val {
            Value::Num(n) => println!("{}", n),
            Value::Var(name) => {
                let val = self.vars.borrow().get(&name).copied().unwrap_or(f64::NAN);
                println!("{} = {}", name, val);
            }
        }
    }

    fn binop(&mut self, left: Value, op: char, right: Value) -> Value {
        if op == '=' {
            if let Value::Var(name) = left {
                let val = right.to_f64(&self.vars.borrow());
                self.vars.borrow_mut().insert(name.clone(), val);
                return Value::Var(name); // Return var so print shows "x = 18"
            }
        }

        let l = left.to_f64(&self.vars.borrow());
        let r = right.to_f64(&self.vars.borrow());

        // Check for user-defined operator
        let op_func = self.ops.borrow().get(&op).map(|d| d.func.clone());
        if let Some(func) = op_func {
            return Value::Num(builtin(&func, l, r));
        }

        Value::Num(match op {
            '+' => l + r,
            '-' => l - r,
            '*' => l * r,
            '/' => l / r,
            _ => f64::NAN,
        })
    }

    fn literal(&mut self, n: f64) -> Value {
        Value::Num(n)
    }

    fn call(&mut self, name: String, a: Value, b: Value) -> Value {
        let vars = self.vars.borrow();
        Value::Num(builtin(&name, a.to_f64(&vars), b.to_f64(&vars)))
    }

    fn var(&mut self, name: String) -> Value {
        Value::Var(name)
    }
}

fn builtin(name: &str, a: f64, b: f64) -> f64 {
    match name {
        "pow" => a.powf(b),
        "min" => a.min(b),
        "max" => a.max(b),
        _ => f64::NAN,
    }
}

struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn next(&mut self, custom_ops: &HashMap<char, OpDef>) -> Option<CalcTerminal> {
        // Skip whitespace
        while self.pos < self.input.len() {
            let c = self.input[self.pos..].chars().next().unwrap();
            if c.is_whitespace() { self.pos += 1; } else { break; }
        }
        if self.pos >= self.input.len() { return None; }

        let remaining = &self.input[self.pos..];
        let c = remaining.chars().next().unwrap();

        // Number
        if c.is_ascii_digit() || c == '.' {
            let end = remaining.find(|c: char| !c.is_ascii_digit() && c != '.').unwrap_or(remaining.len());
            self.pos += end;
            return Some(CalcTerminal::Num(remaining[..end].parse().unwrap()));
        }

        // Identifier or keyword
        if c.is_alphabetic() || c == '_' {
            let end = remaining.find(|c: char| !c.is_alphanumeric() && c != '_').unwrap_or(remaining.len());
            let ident = &remaining[..end];
            self.pos += end;
            return Some(match ident {
                "operator" => CalcTerminal::Operator,
                "left" => CalcTerminal::Left,
                "right" => CalcTerminal::Right,
                _ => CalcTerminal::Ident(ident.to_string()),
            });
        }

        // Single character
        self.pos += 1;
        Some(match c {
            '(' => CalcTerminal::Lparen,
            ')' => CalcTerminal::Rparen,
            ',' => CalcTerminal::Comma,
            ';' => CalcTerminal::Semi,
            '=' => CalcTerminal::Op('=', Precedence::right(0)),
            '+' => CalcTerminal::Op('+', Precedence::left(1)),
            '-' => CalcTerminal::Op('-', Precedence::left(1)),
            '*' => CalcTerminal::Op('*', Precedence::left(2)),
            '/' => CalcTerminal::Op('/', Precedence::left(2)),
            _ => CalcTerminal::Op(c, custom_ops.get(&c).map(|d| d.prec).unwrap_or(Precedence::left(0))),
        })
    }
}

fn main() {
    let input = r#"
        operator ^ pow right 3;
        2 ^ 3 ^ 2;
        x = 2 * 3 ^ 2;
        pow(2, 10);
        x + pow(x, 0.5)
    "#;

    println!("Input:");
    for line in input.lines() {
        let trimmed = line.trim();
        if !trimmed.is_empty() { println!("  {}", trimmed); }
    }
    println!();

    let vars = RefCell::new(HashMap::new());
    let ops = RefCell::new(HashMap::new());
    let mut lexer = Lexer::new(input);
    let mut parser = CalcParser::<Evaluator>::new();
    let mut actions = Evaluator { vars: &vars, ops: &ops };

    loop {
        // Clone ops to avoid holding borrow during push (which may call def_left/def_right)
        let ops_snapshot = ops.borrow().clone();
        match lexer.next(&ops_snapshot) {
            Some(tok) => parser.push(tok, &mut actions).expect("parse error"),
            None => break,
        }
    }
    parser.finish(&mut actions).expect("parse error");
}

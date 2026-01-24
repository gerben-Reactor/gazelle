//! A simple calculator with variable assignments and user-defined operators.
//!
//! Demonstrates Gazelle's runtime operator precedence - operators like `^` can be
//! defined at runtime with custom precedence and associativity.

use std::collections::HashMap;
use gazelle::Precedence;
use gazelle_macros::grammar;

grammar! {
    grammar Calc {
        start stmts;
        terminals {
            NUM: Num,
            IDENT: Ident,
            LPAREN,
            RPAREN,
            COMMA,
            SEMI,
            OPERATOR,
            LEFT,
            RIGHT,
            prec OP: Op,
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

struct Evaluator {
    vars: HashMap<String, f64>,
    ops: HashMap<char, OpDef>,
}

impl CalcActions for Evaluator {
    // Terminal types
    type Num = f64;
    type Ident = String;
    type Op = char;

    // Non-terminal types
    type Expr = Value;

    fn def_left(&mut self, op: char, func: String, prec: f64) {
        println!("defined: {} = {} left {}", op, func, prec as u8);
        self.ops.insert(op, OpDef { func, prec: Precedence::left(prec as u8) });
    }

    fn def_right(&mut self, op: char, func: String, prec: f64) {
        println!("defined: {} = {} right {}", op, func, prec as u8);
        self.ops.insert(op, OpDef { func, prec: Precedence::right(prec as u8) });
    }

    fn print(&mut self, val: Value) {
        match val {
            Value::Num(n) => println!("{}", n),
            Value::Var(name) => {
                let val = self.vars.get(&name).copied().unwrap_or(f64::NAN);
                println!("{} = {}", name, val);
            }
        }
    }

    fn binop(&mut self, left: Value, op: char, right: Value) -> Value {
        if op == '='
            && let Value::Var(name) = left
        {
            let val = right.to_f64(&self.vars);
            self.vars.insert(name.clone(), val);
            return Value::Var(name);
        }

        let l = left.to_f64(&self.vars);
        let r = right.to_f64(&self.vars);

        if let Some(op_def) = self.ops.get(&op) {
            return Value::Num(builtin(&op_def.func, l, r));
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
        Value::Num(builtin(&name, a.to_f64(&self.vars), b.to_f64(&self.vars)))
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

struct Tokenizer<'a> {
    lexer: gazelle::lexer::Lexer<'a>,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            lexer: gazelle::lexer::Lexer::new(input),
        }
    }

    fn next(&mut self, custom_ops: &HashMap<char, OpDef>) -> Result<Option<CalcTerminal<Evaluator>>, String> {
        use gazelle::lexer::Token;

        let tok = match self.lexer.next() {
            Some(Ok(t)) => t,
            Some(Err(e)) => return Err(e),
            None => return Ok(None),
        };

        Ok(Some(match tok {
            Token::Num(s) => CalcTerminal::Num(s.parse().unwrap()),
            Token::Ident(s) => match s.as_str() {
                "operator" => CalcTerminal::Operator,
                "left" => CalcTerminal::Left,
                "right" => CalcTerminal::Right,
                _ => CalcTerminal::Ident(s),
            },
            Token::Punct(c) => match c {
                '(' => CalcTerminal::Lparen,
                ')' => CalcTerminal::Rparen,
                ',' => CalcTerminal::Comma,
                ';' => CalcTerminal::Semi,
                _ => return self.next(custom_ops),
            },
            Token::Op(s) if s.len() == 1 => {
                self.op_terminal(s.chars().next().unwrap(), custom_ops)
            }
            Token::Op(s) => return Err(format!("Unknown operator: {}", s)),
            Token::Str(_) | Token::Char(_) => return self.next(custom_ops),
        }))
    }

    fn op_terminal(&self, c: char, custom_ops: &HashMap<char, OpDef>) -> CalcTerminal<Evaluator> {
        match c {
            '=' => CalcTerminal::Op('=', Precedence::right(0)),
            '+' => CalcTerminal::Op('+', Precedence::left(1)),
            '-' => CalcTerminal::Op('-', Precedence::left(1)),
            '*' => CalcTerminal::Op('*', Precedence::left(2)),
            '/' => CalcTerminal::Op('/', Precedence::left(2)),
            _ => CalcTerminal::Op(c, custom_ops.get(&c).map(|d| d.prec).unwrap_or(Precedence::left(0))),
        }
    }
}

fn main() {
    use std::io::{self, Write, BufRead};

    let mut parser = CalcParser::<Evaluator>::new();
    let mut actions = Evaluator {
        vars: HashMap::new(),
        ops: HashMap::new(),
    };

    println!("Calculator. Type expressions, 'operator ^ pow right 3' to define ops, or 'quit' to exit.");
    println!("End statements with ';' to see results.\n");

    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap() == 0 {
            break;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line == "quit" || line == "exit" {
            break;
        }

        // Tokenize and push to the single persistent parser
        let mut tokenizer = Tokenizer::new(line);
        loop {
            let ops_snapshot = actions.ops.clone();
            match tokenizer.next(&ops_snapshot) {
                Ok(Some(tok)) => {
                    if let Err(e) = parser.push(tok, &mut actions) {
                        eprintln!("Parse error: {:?}", e);
                        parser = CalcParser::new();
                        break;
                    }
                }
                Ok(None) => break,
                Err(e) => {
                    eprintln!("Lex error: {}", e);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator() {
        let mut parser = CalcParser::<Evaluator>::new();
        let mut actions = Evaluator {
            vars: HashMap::new(),
            ops: HashMap::new(),
        };

        let mut tokenizer = Tokenizer::new("operator ^ pow right 3; x = 2 * 3 ^ 2; y = x + 1;");
        loop {
            let ops_snapshot = actions.ops.clone();
            match tokenizer.next(&ops_snapshot).unwrap() {
                Some(tok) => parser.push(tok, &mut actions).unwrap(),
                None => break,
            }
        }

        assert!(actions.ops.contains_key(&'^'));
        assert_eq!(actions.vars.get("x"), Some(&18.0));
        assert_eq!(actions.vars.get("y"), Some(&19.0));
    }
}

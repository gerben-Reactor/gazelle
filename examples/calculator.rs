//! A simple calculator with variable assignments and user-defined operators.
//!
//! This example showcases Gazelle's key design features:
//!
//! ## 1. Runtime Operator Precedence (`prec_terminals`)
//!
//! Instead of encoding precedence in the grammar (which leads to grammar bloat),
//! we declare `OP` as a precedence terminal. The lexer provides precedence at
//! runtime:
//! - `=` gets `Precedence::right(0)` - lowest precedence, right associative
//! - `+` and `-` get `Precedence::left(1)`
//! - `*` and `/` get `Precedence::left(2)`
//!
//! Assignment is just another binary operator! `x = y = 1` parses as `x = (y = 1)`
//! because `=` is right-associative.
//!
//! ## 2. User-Defined Operators
//!
//! Because the lexer provides precedence at runtime, we can define new operators:
//! ```text
//!   operator ^ = pow right 3;
//! ```
//! This defines `^` as a right-associative operator with precedence 3 (higher than `*`)
//! that calls the built-in `pow` function. The lexer is updated dynamically!
//!
//! ## 3. Lexer-Driven Outer Loop
//!
//! The user drives the lexer, not the parser. This means:
//! - We control when to evaluate (after each `;` or at EOF)
//! - We can maintain state between expressions (variable bindings)
//! - We can define new operators that affect subsequent parsing
//! - We can handle interactive input, error recovery, etc.
//!
//! ## 4. Statement-Level Control
//!
//! When we see `;`, we complete the current parse, evaluate, store results,
//! then start fresh. The parser never "takes over" - we're always in control.
//!
//! Example:
//! ```text
//!   operator ^ = pow right 3;   -> defines ^ as exponentiation
//!   2 ^ 3 ^ 2;                  -> 512  (right-assoc: 2^(3^2) = 2^9)
//!   x = 2 * 3 ^ 2;              -> x = 18  (^ binds tighter: 2*(3^2))
//! ```

use std::collections::HashMap;
use gazelle::grammar;
use gazelle_core::Precedence;

grammar! {
    grammar Calc {
        terminals {
            NUM: f64,
            IDENT: String,
            LPAREN,
            RPAREN,
            COMMA,
            SEMI,
            // Keywords
            OPERATOR,
            LEFT,
            RIGHT,
        }

        prec_terminals {
            OP: char,
        }

        // Program is a sequence of statements separated by SEMI
        stmts: () = stmts SEMI stmt | stmt |;

        // Statements: operator definitions or expressions
        stmt: () = OPERATOR OP IDENT LEFT NUM
                 | OPERATOR OP IDENT RIGHT NUM
                 | expr;

        // Expressions with runtime precedence
        expr: Expr = expr OP expr | NUM | IDENT LPAREN expr COMMA expr RPAREN | IDENT | LPAREN expr RPAREN;
    }
}

/// AST for expressions.
#[derive(Debug, Clone)]
enum Expr {
    Num(f64),
    Var(String),
    BinOp(Box<Expr>, char, Box<Expr>),
    Call(String, Vec<Expr>),
}


/// User-defined operator: maps a char to a function name.
#[derive(Debug, Clone)]
struct OpDef {
    func: String,
    prec: Precedence,
}

impl Expr {
    /// Evaluate, performing assignments along the way.
    fn eval(&self, vars: &mut HashMap<String, f64>, ops: &HashMap<char, OpDef>) -> Result<f64, String> {
        match self {
            Expr::Num(n) => Ok(*n),
            Expr::Var(name) => vars.get(name).copied()
                .ok_or_else(|| format!("undefined variable: {}", name)),
            Expr::Call(name, args) => {
                let evaluated: Result<Vec<f64>, String> = args.iter()
                    .map(|a| a.eval(vars, ops))
                    .collect();
                let args = evaluated?;
                eval_builtin(name, &args)
            }
            Expr::BinOp(left, '=', right) => {
                // Assignment: left must be a variable
                match left.as_ref() {
                    Expr::Var(name) => {
                        let val = right.eval(vars, ops)?;
                        vars.insert(name.clone(), val);
                        Ok(val)
                    }
                    _ => Err("left side of assignment must be a variable".to_string()),
                }
            }
            Expr::BinOp(left, op, right) => {
                // Check for user-defined operator
                if let Some(def) = ops.get(op) {
                    let l = left.eval(vars, ops)?;
                    let r = right.eval(vars, ops)?;
                    return eval_builtin(&def.func, &[l, r]);
                }
                // Built-in operators
                let l = left.eval(vars, ops)?;
                let r = right.eval(vars, ops)?;
                Ok(match op {
                    '+' => l + r,
                    '-' => l - r,
                    '*' => l * r,
                    '/' => l / r,
                    _ => return Err(format!("unknown operator: {}", op)),
                })
            }
        }
    }
}

/// Evaluate a built-in function.
fn eval_builtin(name: &str, args: &[f64]) -> Result<f64, String> {
    match (name, args) {
        ("pow", [base, exp]) => Ok(base.powf(*exp)),
        ("sqrt", [x]) => Ok(x.sqrt()),
        ("sin", [x]) => Ok(x.sin()),
        ("cos", [x]) => Ok(x.cos()),
        ("abs", [x]) => Ok(x.abs()),
        ("min", [a, b]) => Ok(a.min(*b)),
        ("max", [a, b]) => Ok(a.max(*b)),
        _ => Err(format!("unknown function: {}({} args)", name, args.len())),
    }
}

/// Simple lexer that yields Option<CalcTerminal>.
/// Supports dynamically-defined operators via the `custom_ops` map passed to next().
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

    fn next(&mut self, custom_ops: &HashMap<char, OpDef>) -> Option<CalcTerminal> {
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

        // Identifier or keyword
        if c.is_alphabetic() || c == '_' {
            let end = remaining.find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(remaining.len());
            let ident = &remaining[..end];
            self.pos += end;
            return Some(match ident {
                "operator" => CalcTerminal::Operator,
                "left" => CalcTerminal::Left,
                "right" => CalcTerminal::Right,
                _ => CalcTerminal::Ident(ident.to_string()),
            });
        }

        // Single character tokens
        self.pos += 1;
        match c {
            '(' => Some(CalcTerminal::Lparen),
            ')' => Some(CalcTerminal::Rparen),
            ',' => Some(CalcTerminal::Comma),
            ';' => Some(CalcTerminal::Semi),
            '=' => Some(CalcTerminal::Op('=', Precedence::right(0))),  // lowest prec, right assoc
            '+' => Some(CalcTerminal::Op('+', Precedence::left(1))),
            '-' => Some(CalcTerminal::Op('-', Precedence::left(1))),
            '*' => Some(CalcTerminal::Op('*', Precedence::left(2))),
            '/' => Some(CalcTerminal::Op('/', Precedence::left(2))),
            _ => {
                // Custom operator (defined or being defined)
                let prec = custom_ops.get(&c)
                    .map(|def| def.prec)
                    .unwrap_or(Precedence::left(0));  // placeholder for definitions
                Some(CalcTerminal::Op(c, prec))
            }
        }
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
        if !trimmed.is_empty() {
            println!("  {}", trimmed);
        }
    }
    println!();

    let mut vars: HashMap<String, f64> = HashMap::new();
    let mut ops: HashMap<char, OpDef> = HashMap::new();
    let mut lexer = Lexer::new(input);
    let mut parser = CalcParser::new();

    // Helper closure to handle reductions with access to vars/ops
    let handle_reduce = |r: CalcReduction, vars: &mut HashMap<String, f64>, ops: &mut HashMap<char, OpDef>| {
        match r {
            // stmts = stmts SEMI stmt
            CalcReduction::StmtsStmtsSemiStmt(c, (), ()) => c(()),
            // stmts = stmt
            CalcReduction::StmtsStmt(c, ()) => c(()),
            // stmts = ε
            CalcReduction::StmtsEmpty(c) => c(()),
            // stmt = OPERATOR OP IDENT LEFT NUM
            CalcReduction::StmtOperatorOpIdentLeftNum(c, op, func, prec) => {
                let precedence = Precedence::left(prec as u8);
                println!("defined: {} = {} left {}", op, func, prec as u8);
                ops.insert(op, OpDef { func, prec: precedence });
                c(())
            }
            // stmt = OPERATOR OP IDENT RIGHT NUM
            CalcReduction::StmtOperatorOpIdentRightNum(c, op, func, prec) => {
                let precedence = Precedence::right(prec as u8);
                println!("defined: {} = {} right {}", op, func, prec as u8);
                ops.insert(op, OpDef { func, prec: precedence });
                c(())
            }
            // stmt = expr
            CalcReduction::StmtExpr(c, expr) => {
                eval_and_print(&expr, vars, ops);
                c(())
            }
            // expr reductions build the AST
            CalcReduction::ExprExprOpExpr(c, left, op, right) => {
                c(Expr::BinOp(Box::new(left), op, Box::new(right)))
            }
            CalcReduction::ExprNum(c, n) => c(Expr::Num(n)),
            CalcReduction::ExprIdentLparenExprCommaExprRparen(c, name, arg1, arg2) => {
                c(Expr::Call(name, vec![arg1, arg2]))
            }
            CalcReduction::ExprIdent(c, name) => c(Expr::Var(name)),
            CalcReduction::ExprLparenExprRparen(c, inner) => c(inner),
        }
    };

    // Parse token by token - this allows operator definitions to affect
    // subsequent lexing (the lexer gets updated ops on each next() call)
    loop {
        let tok = lexer.next(&ops);

        while let Some(r) = parser.maybe_reduce(&tok) {
            parser.reduce(handle_reduce(r, &mut vars, &mut ops));
        }

        if tok.is_none() {
            break;
        }
        parser.shift(tok.unwrap()).expect("parse error");
    }

    parser.accept().expect("parse error");
}

fn eval_and_print(expr: &Expr, vars: &mut HashMap<String, f64>, ops: &HashMap<char, OpDef>) {
    // Get assignments that will be made by this expression
    let assignments = find_assignments(expr);

    match expr.eval(vars, ops) {
        Ok(val) => {
            if assignments.is_empty() {
                println!("{}", val);
            } else {
                // Print each assignment
                for name in &assignments {
                    println!("{} = {}", name, vars.get(name).unwrap());
                }
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

/// Find all variable names that will be assigned by this expression.
fn find_assignments(expr: &Expr) -> Vec<String> {
    let mut names = Vec::new();
    collect_assignments(expr, &mut names);
    names
}

fn collect_assignments(expr: &Expr, names: &mut Vec<String>) {
    if let Expr::BinOp(left, '=', right) = expr {
        if let Expr::Var(name) = left.as_ref() {
            names.push(name.clone());
        }
        collect_assignments(right, names);
    }
}


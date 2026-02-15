//! Test that dynamic precedence parsing matches fixed grammar parsing.
//!
//! Generates all expressions with +, *, ^ operators up to 5 numbers
//! and verifies both approaches produce identical ASTs.

use gazelle::{Precedence, Reduce};
use gazelle_macros::gazelle;

// ============================================================================
// AST representation (shared between both parsers)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    Num(i32),
    BinOp(Box<Expr>, char, Box<Expr>),
}

impl Expr {
    fn binop(l: Expr, op: char, r: Expr) -> Expr {
        Expr::BinOp(Box::new(l), op, Box::new(r))
    }
}

// ============================================================================
// Dynamic precedence grammar (single rule with prec terminal)
// ============================================================================

gazelle! {
    grammar Dynamic {
        start expr;
        terminals {
            NUM: Num,
            prec OP: Op
        }

        expr = expr OP expr => Binop
                   | NUM => Num;
    }
}

struct DynBuilder;

impl DynamicTypes for DynBuilder {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Op = char;
    type Expr = Expr;
}

impl Reduce<DynamicExpr<Self>, Expr, gazelle::ParseError> for DynBuilder {
    fn reduce(&mut self, node: DynamicExpr<Self>) -> Result<Expr, gazelle::ParseError> {
        Ok(match node {
            DynamicExpr::Binop(l, op, r) => Expr::binop(l, op, r),
            DynamicExpr::Num(n) => Expr::Num(n),
        })
    }
}

fn parse_dynamic(input: &str) -> Result<Expr, String> {
    let tokens = lex_dynamic(input)?;
    let mut parser = DynamicParser::<DynBuilder>::new();
    let mut actions = DynBuilder;

    for tok in tokens {
        parser.push(tok, &mut actions).map_err(|e| format!("{:?}", e))?;
    }

    parser.finish(&mut actions).map_err(|(p, e)| p.format_error(&e))
}

fn lex_dynamic(input: &str) -> Result<Vec<DynamicTerminal<DynBuilder>>, String> {
    let mut tokens = Vec::new();
    for c in input.chars() {
        match c {
            ' ' => {}
            '0'..='9' => tokens.push(DynamicTerminal::NUM(c as i32 - '0' as i32)),
            '+' => tokens.push(DynamicTerminal::OP('+', Precedence::Left(1))),
            '*' => tokens.push(DynamicTerminal::OP('*', Precedence::Left(2))),
            '^' => tokens.push(DynamicTerminal::OP('^', Precedence::Right(3))),
            _ => return Err(format!("unexpected char: {}", c)),
        }
    }
    Ok(tokens)
}

// ============================================================================
// Fixed precedence grammar (explicit rule hierarchy)
// ============================================================================

gazelle! {
    grammar Fixed {
        start expr;
        terminals {
            NUM: Num,
            PLUS, STAR, CARET
        }

        // Lowest precedence: addition (left-associative)
        expr = expr PLUS term => Add
                   | term => Term;

        // Medium precedence: multiplication (left-associative)
        term = term STAR factor => Mul
                   | factor => Factor;

        // Highest precedence: exponentiation (right-associative)
        factor = base CARET factor => Pow
                       | base => Base;

        base = NUM => Num;
    }
}

struct FixedBuilder;

impl FixedTypes for FixedBuilder {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Expr = Expr;
    type Term = Expr;
    type Factor = Expr;
    type Base = Expr;
}

impl Reduce<FixedExpr<Self>, Expr, gazelle::ParseError> for FixedBuilder {
    fn reduce(&mut self, node: FixedExpr<Self>) -> Result<Expr, gazelle::ParseError> {
        Ok(match node {
            FixedExpr::Add(l, r) => Expr::binop(l, '+', r),
            FixedExpr::Term(t) => t,
        })
    }
}

impl Reduce<FixedTerm<Self>, Expr, gazelle::ParseError> for FixedBuilder {
    fn reduce(&mut self, node: FixedTerm<Self>) -> Result<Expr, gazelle::ParseError> {
        Ok(match node {
            FixedTerm::Mul(l, r) => Expr::binop(l, '*', r),
            FixedTerm::Factor(f) => f,
        })
    }
}

impl Reduce<FixedFactor<Self>, Expr, gazelle::ParseError> for FixedBuilder {
    fn reduce(&mut self, node: FixedFactor<Self>) -> Result<Expr, gazelle::ParseError> {
        Ok(match node {
            FixedFactor::Pow(l, r) => Expr::binop(l, '^', r),
            FixedFactor::Base(b) => b,
        })
    }
}

impl Reduce<FixedBase<Self>, Expr, gazelle::ParseError> for FixedBuilder {
    fn reduce(&mut self, node: FixedBase<Self>) -> Result<Expr, gazelle::ParseError> {
        Ok(match node {
            FixedBase::Num(n) => Expr::Num(n),
        })
    }
}

fn parse_fixed(input: &str) -> Result<Expr, String> {
    let tokens = lex_fixed(input)?;
    let mut parser = FixedParser::<FixedBuilder>::new();
    let mut actions = FixedBuilder;

    for tok in tokens {
        parser.push(tok, &mut actions).map_err(|e| format!("{:?}", e))?;
    }

    parser.finish(&mut actions).map_err(|(p, e)| p.format_error(&e))
}

fn lex_fixed(input: &str) -> Result<Vec<FixedTerminal<FixedBuilder>>, String> {
    let mut tokens = Vec::new();
    for c in input.chars() {
        match c {
            ' ' => {}
            '0'..='9' => tokens.push(FixedTerminal::NUM(c as i32 - '0' as i32)),
            '+' => tokens.push(FixedTerminal::PLUS),
            '*' => tokens.push(FixedTerminal::STAR),
            '^' => tokens.push(FixedTerminal::CARET),
            _ => return Err(format!("unexpected char: {}", c)),
        }
    }
    Ok(tokens)
}

// ============================================================================
// Expression generator
// ============================================================================

fn generate_expressions(max_nums: usize) -> Vec<String> {
    let ops = ['+', '*', '^'];
    let mut results = Vec::new();

    // Generate expressions with 1 to max_nums numbers
    for num_count in 1..=max_nums {
        generate_with_nums(num_count, &ops, &mut results);
    }

    results
}

fn generate_with_nums(num_count: usize, ops: &[char], results: &mut Vec<String>) {
    if num_count == 0 {
        return;
    }

    // Numbers 1-9 (single digit for simplicity)
    let nums: Vec<char> = (1..=9).take(num_count).map(|n| char::from_digit(n, 10).unwrap()).collect();

    if num_count == 1 {
        results.push(nums[0].to_string());
        return;
    }

    // For n numbers, we have n-1 operator positions
    // Each can be +, *, or ^
    let op_count = num_count - 1;
    let total_combinations = ops.len().pow(op_count as u32);

    for combo in 0..total_combinations {
        let mut expr = String::new();
        let mut remaining = combo;

        for i in 0..num_count {
            expr.push(nums[i]);
            if i < op_count {
                let op_idx = remaining % ops.len();
                remaining /= ops.len();
                expr.push(' ');
                expr.push(ops[op_idx]);
                expr.push(' ');
            }
        }

        results.push(expr);
    }
}

// ============================================================================
// Tests
// ============================================================================

#[test]
fn test_precedence_equivalence() {
    let expressions = generate_expressions(7);
    let mut passed = 0;
    let mut failed = 0;

    for expr in &expressions {
        let dynamic_result = parse_dynamic(expr);
        let fixed_result = parse_fixed(expr);

        match (&dynamic_result, &fixed_result) {
            (Ok(d), Ok(f)) if d == f => {
                passed += 1;
            }
            (Ok(d), Ok(f)) => {
                eprintln!("MISMATCH: {}", expr);
                eprintln!("  dynamic: {:?}", d);
                eprintln!("  fixed:   {:?}", f);
                failed += 1;
            }
            (Err(e1), Err(e2)) => {
                eprintln!("BOTH FAILED: {} -> {} / {}", expr, e1, e2);
                failed += 1;
            }
            (Err(e), Ok(_)) => {
                eprintln!("DYNAMIC FAILED: {} -> {}", expr, e);
                failed += 1;
            }
            (Ok(_), Err(e)) => {
                eprintln!("FIXED FAILED: {} -> {}", expr, e);
                failed += 1;
            }
        }
    }

    eprintln!("\nTotal: {} expressions, {} passed, {} failed", expressions.len(), passed, failed);
    assert_eq!(failed, 0, "Some expressions produced different ASTs");
}

#[test]
fn test_specific_cases() {
    // Test precedence: * binds tighter than +
    let expr = "1 + 2 * 3";
    let expected = Expr::binop(
        Expr::Num(1),
        '+',
        Expr::binop(Expr::Num(2), '*', Expr::Num(3))
    );
    assert_eq!(parse_dynamic(expr).unwrap(), expected);
    assert_eq!(parse_fixed(expr).unwrap(), expected);

    // Test left-associativity of +
    let expr = "1 + 2 + 3";
    let expected = Expr::binop(
        Expr::binop(Expr::Num(1), '+', Expr::Num(2)),
        '+',
        Expr::Num(3)
    );
    assert_eq!(parse_dynamic(expr).unwrap(), expected);
    assert_eq!(parse_fixed(expr).unwrap(), expected);

    // Test right-associativity of ^
    let expr = "2 ^ 3 ^ 4";
    let expected = Expr::binop(
        Expr::Num(2),
        '^',
        Expr::binop(Expr::Num(3), '^', Expr::Num(4))
    );
    assert_eq!(parse_dynamic(expr).unwrap(), expected);
    assert_eq!(parse_fixed(expr).unwrap(), expected);

    // Test mixed: 1 + 2 ^ 3 * 4 = 1 + ((2^3) * 4)
    let expr = "1 + 2 ^ 3 * 4";
    let expected = Expr::binop(
        Expr::Num(1),
        '+',
        Expr::binop(
            Expr::binop(Expr::Num(2), '^', Expr::Num(3)),
            '*',
            Expr::Num(4)
        )
    );
    assert_eq!(parse_dynamic(expr).unwrap(), expected);
    assert_eq!(parse_fixed(expr).unwrap(), expected);
}

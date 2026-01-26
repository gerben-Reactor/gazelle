//! C11 Expression Evaluator
//!
//! Tests the full C11 expression grammar (primary -> postfix -> unary -> cast -> assignment -> expression)
//! by evaluating arithmetic expressions. Uses the same lexer approach as the C11 parser.

use gazelle::Precedence;
use gazelle_macros::grammar;

// Operator enum for BINOP - must be defined before grammar! macro
#[derive(Clone, Copy, Debug)]
enum BinOp {
    Or, And,           // || &&
    BitOr, BitXor,     // | ^
    Eq, Ne,            // == !=
    Lt, Gt, Le, Ge,    // < > <= >=
    Shl, Shr,          // << >>
    Div, Mod,          // / %
}

grammar! {
    grammar C11Expr {
        start expression;
        terminals {
            NUM: Num,
            IDENT: Ident,
            LPAREN, RPAREN, LBRACK, RBRACK,
            COMMA, COLON,
            TILDE, BANG,
            INC, DEC,
            // Precedence terminals (same structure as C11 parser)
            // BINOP carries BinOp enum to distinguish operators in actions
            prec EQ,
            prec ASSIGN,
            prec QUESTION,
            prec STAR,
            prec AMP,
            prec PLUS,
            prec MINUS,
            prec BINOP: Binop,
        }

        // === Full C11 expression hierarchy ===

        primary_expression: PrimaryExpression = NUM @eval_num
                                              | IDENT @eval_ident
                                              | LPAREN expression RPAREN @eval_paren;

        postfix_expression: PostfixExpression = primary_expression @eval_primary
                                              | postfix_expression LBRACK expression RBRACK @eval_index
                                              | postfix_expression LPAREN RPAREN @eval_call0
                                              | postfix_expression LPAREN argument_expression_list RPAREN @eval_call
                                              | postfix_expression INC @eval_postinc
                                              | postfix_expression DEC @eval_postdec;

        argument_expression_list: ArgumentExpressionList = assignment_expression @eval_arg1
                                                         | argument_expression_list COMMA assignment_expression @eval_args;

        unary_expression: UnaryExpression = postfix_expression @eval_postfix
                                          | INC unary_expression @eval_preinc
                                          | DEC unary_expression @eval_predec
                                          | AMP cast_expression @eval_addr
                                          | STAR cast_expression @eval_deref
                                          | PLUS cast_expression @eval_uplus
                                          | MINUS cast_expression @eval_uminus
                                          | TILDE cast_expression @eval_bitnot
                                          | BANG cast_expression @eval_lognot;

        cast_expression: CastExpression = unary_expression @eval_unary;
        // Note: actual casts (LPAREN type_name RPAREN cast_expression) omitted - need type_name

        // Collapsed binary expression hierarchy with dynamic precedence
        assignment_expression: AssignmentExpression = cast_expression @eval_cast
                                                    | assignment_expression BINOP assignment_expression @eval_binop
                                                    | assignment_expression STAR assignment_expression @eval_mul
                                                    | assignment_expression AMP assignment_expression @eval_bitand
                                                    | assignment_expression PLUS assignment_expression @eval_add
                                                    | assignment_expression MINUS assignment_expression @eval_sub
                                                    | assignment_expression EQ assignment_expression @eval_assign
                                                    | assignment_expression ASSIGN assignment_expression @eval_compound
                                                    | assignment_expression QUESTION expression COLON assignment_expression @eval_ternary;

        expression: Expression = assignment_expression @eval_assign_expr
                               | expression COMMA assignment_expression @eval_comma;
    }
}

// Simple variable storage for evaluation
use std::collections::HashMap;

struct Eval {
    vars: HashMap<String, i64>,
    // Fake "memory" for address-of/deref testing
    mem: HashMap<i64, i64>,
    next_addr: i64,
}

impl Eval {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
            mem: HashMap::new(),
            next_addr: 1000,
        }
    }

    fn with_vars(vars: &[(&str, i64)]) -> Self {
        let mut e = Self::new();
        for (name, val) in vars {
            e.vars.insert(name.to_string(), *val);
        }
        e
    }
}

impl C11ExprActions for Eval {
    type Num = i64;
    type Ident = String;
    type Binop = BinOp;
    type ArgumentExpressionList = Vec<i64>;
    type PrimaryExpression = i64;
    type PostfixExpression = i64;
    type UnaryExpression = i64;
    type CastExpression = i64;
    type AssignmentExpression = i64;
    type Expression = i64;

    // Primary
    fn eval_num(&mut self, n: i64) -> i64 { n }
    fn eval_ident(&mut self, name: String) -> i64 {
        *self.vars.get(&name).unwrap_or(&0)
    }
    fn eval_paren(&mut self, e: i64) -> i64 { e }

    // Postfix
    fn eval_primary(&mut self, e: i64) -> i64 { e }
    fn eval_index(&mut self, arr: i64, idx: i64) -> i64 {
        // Simulate array: arr[idx] = *(arr + idx)
        *self.mem.get(&(arr + idx)).unwrap_or(&0)
    }
    fn eval_call0(&mut self, _func: i64) -> i64 { 0 }  // function call, return 0
    fn eval_call(&mut self, _func: i64, _args: Vec<i64>) -> i64 { 0 }
    fn eval_postinc(&mut self, e: i64) -> i64 { e }  // simplified: just return value
    fn eval_postdec(&mut self, e: i64) -> i64 { e }

    // Argument list
    fn eval_arg1(&mut self, e: i64) -> Vec<i64> { vec![e] }
    fn eval_args(&mut self, mut list: Vec<i64>, e: i64) -> Vec<i64> {
        list.push(e);
        list
    }

    // Unary
    fn eval_postfix(&mut self, e: i64) -> i64 { e }
    fn eval_preinc(&mut self, e: i64) -> i64 { e + 1 }
    fn eval_predec(&mut self, e: i64) -> i64 { e - 1 }
    fn eval_addr(&mut self, e: i64) -> i64 {
        let addr = self.next_addr;
        self.next_addr += 1;
        self.mem.insert(addr, e);
        addr
    }
    fn eval_deref(&mut self, addr: i64) -> i64 {
        *self.mem.get(&addr).unwrap_or(&0)
    }
    fn eval_uplus(&mut self, e: i64) -> i64 { e }
    fn eval_uminus(&mut self, e: i64) -> i64 { -e }
    fn eval_bitnot(&mut self, e: i64) -> i64 { !e }
    fn eval_lognot(&mut self, e: i64) -> i64 { if e == 0 { 1 } else { 0 } }

    // Cast (passthrough, no actual type casts)
    fn eval_unary(&mut self, e: i64) -> i64 { e }
    fn eval_cast(&mut self, e: i64) -> i64 { e }

    // Binary operators
    fn eval_binop(&mut self, l: i64, op: BinOp, r: i64) -> i64 {
        match op {
            BinOp::Or => if l != 0 || r != 0 { 1 } else { 0 },
            BinOp::And => if l != 0 && r != 0 { 1 } else { 0 },
            BinOp::BitOr => l | r,
            BinOp::BitXor => l ^ r,
            BinOp::Eq => if l == r { 1 } else { 0 },
            BinOp::Ne => if l != r { 1 } else { 0 },
            BinOp::Lt => if l < r { 1 } else { 0 },
            BinOp::Gt => if l > r { 1 } else { 0 },
            BinOp::Le => if l <= r { 1 } else { 0 },
            BinOp::Ge => if l >= r { 1 } else { 0 },
            BinOp::Shl => l << r,
            BinOp::Shr => l >> r,
            BinOp::Div => l / r,
            BinOp::Mod => l % r,
        }
    }
    fn eval_mul(&mut self, l: i64, r: i64) -> i64 { l * r }
    fn eval_bitand(&mut self, l: i64, r: i64) -> i64 { l & r }
    fn eval_add(&mut self, l: i64, r: i64) -> i64 { l + r }
    fn eval_sub(&mut self, l: i64, r: i64) -> i64 { l - r }

    // Assignment (simplified - just return RHS)
    fn eval_assign(&mut self, _l: i64, r: i64) -> i64 { r }
    fn eval_compound(&mut self, _l: i64, r: i64) -> i64 { r }

    // Ternary
    fn eval_ternary(&mut self, cond: i64, then_val: i64, else_val: i64) -> i64 {
        if cond != 0 { then_val } else { else_val }
    }

    // Expression
    fn eval_assign_expr(&mut self, e: i64) -> i64 { e }
    fn eval_comma(&mut self, _l: i64, r: i64) -> i64 { r }  // comma: evaluate both, return right
}

// =============================================================================
// Lexer (C11-style)
// =============================================================================

fn lex(input: &str) -> Result<Vec<C11ExprTerminal<Eval>>, String> {
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
                tokens.push(C11ExprTerminal::Num(num));
            }

            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(C11ExprTerminal::Ident(ident));
            }

            '(' => { chars.next(); tokens.push(C11ExprTerminal::Lparen); }
            ')' => { chars.next(); tokens.push(C11ExprTerminal::Rparen); }
            '[' => { chars.next(); tokens.push(C11ExprTerminal::Lbrack); }
            ']' => { chars.next(); tokens.push(C11ExprTerminal::Rbrack); }
            ',' => { chars.next(); tokens.push(C11ExprTerminal::Comma); }
            ':' => { chars.next(); tokens.push(C11ExprTerminal::Colon); }
            '~' => { chars.next(); tokens.push(C11ExprTerminal::Tilde); }

            '+' => {
                chars.next();
                if chars.peek() == Some(&'+') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Inc);
                } else if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Assign(Precedence::right(1)));
                } else {
                    tokens.push(C11ExprTerminal::Plus(Precedence::left(11)));
                }
            }

            '-' => {
                chars.next();
                if chars.peek() == Some(&'-') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Dec);
                } else if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Assign(Precedence::right(1)));
                } else {
                    tokens.push(C11ExprTerminal::Minus(Precedence::left(11)));
                }
            }

            '*' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Assign(Precedence::right(1)));
                } else {
                    tokens.push(C11ExprTerminal::Star(Precedence::left(12)));
                }
            }

            '/' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Assign(Precedence::right(1)));
                } else {
                    tokens.push(C11ExprTerminal::Binop(BinOp::Div, Precedence::left(12)));
                }
            }

            '%' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Assign(Precedence::right(1)));
                } else {
                    tokens.push(C11ExprTerminal::Binop(BinOp::Mod, Precedence::left(12)));
                }
            }

            '&' => {
                chars.next();
                if chars.peek() == Some(&'&') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Binop(BinOp::And, Precedence::left(4)));
                } else if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Assign(Precedence::right(1)));
                } else {
                    tokens.push(C11ExprTerminal::Amp(Precedence::left(7)));
                }
            }

            '|' => {
                chars.next();
                if chars.peek() == Some(&'|') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Binop(BinOp::Or, Precedence::left(3)));
                } else if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Assign(Precedence::right(1)));
                } else {
                    tokens.push(C11ExprTerminal::Binop(BinOp::BitOr, Precedence::left(5)));
                }
            }

            '^' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Assign(Precedence::right(1)));
                } else {
                    tokens.push(C11ExprTerminal::Binop(BinOp::BitXor, Precedence::left(6)));
                }
            }

            '!' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Binop(BinOp::Ne, Precedence::left(8)));
                } else {
                    tokens.push(C11ExprTerminal::Bang);
                }
            }

            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Binop(BinOp::Eq, Precedence::left(8)));
                } else {
                    tokens.push(C11ExprTerminal::Eq(Precedence::right(1)));
                }
            }

            '<' => {
                chars.next();
                if chars.peek() == Some(&'<') {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(C11ExprTerminal::Assign(Precedence::right(1)));
                    } else {
                        tokens.push(C11ExprTerminal::Binop(BinOp::Shl, Precedence::left(10)));
                    }
                } else if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Binop(BinOp::Le, Precedence::left(9)));
                } else {
                    tokens.push(C11ExprTerminal::Binop(BinOp::Lt, Precedence::left(9)));
                }
            }

            '>' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push(C11ExprTerminal::Assign(Precedence::right(1)));
                    } else {
                        tokens.push(C11ExprTerminal::Binop(BinOp::Shr, Precedence::left(10)));
                    }
                } else if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(C11ExprTerminal::Binop(BinOp::Ge, Precedence::left(9)));
                } else {
                    tokens.push(C11ExprTerminal::Binop(BinOp::Gt, Precedence::left(9)));
                }
            }

            '?' => {
                chars.next();
                tokens.push(C11ExprTerminal::Question(Precedence::right(2)));
            }

            _ => return Err(format!("Unexpected char: {}", c)),
        }
    }

    Ok(tokens)
}

fn eval(input: &str) -> Result<i64, String> {
    eval_with_vars(input, &[])
}

fn eval_with_vars(input: &str, vars: &[(&str, i64)]) -> Result<i64, String> {
    let mut parser = C11ExprParser::<Eval>::new();
    let mut actions = Eval::with_vars(vars);

    let tokens = lex(input)?;
    for tok in tokens {
        parser.push(tok, &mut actions).map_err(|e| format!("{:?}", e))?;
    }

    parser.finish(&mut actions).map_err(|e| format!("{:?}", e))
}

fn main() {
    println!("C11 Expression Evaluator - Full Grammar Test");
    println!("============================================");
    println!();

    let tests: &[(&str, i64)] = &[
        // Basic arithmetic
        ("1 + 2 * 3", 7),           // * binds tighter
        ("2 * 3 + 1", 7),
        ("(1 + 2) * 3", 9),         // parens override
        ("10 - 3 - 2", 5),          // left-assoc
        ("100 / 10 / 2", 5),        // left-assoc

        // All precedence levels (low to high)
        ("1, 2, 3", 3),                     // comma: returns last
        ("1 ? 2 : 3", 2),                   // ternary true
        ("0 ? 2 : 3", 3),                   // ternary false
        ("1 || 0", 1),                      // logical or
        ("0 && 1", 0),                      // logical and
        ("5 | 2", 7),                       // bitwise or
        ("7 ^ 3", 4),                       // bitwise xor
        ("7 & 3", 3),                       // bitwise and
        ("2 == 2", 1),                      // equality
        ("2 != 3", 1),
        ("1 < 2", 1),                       // relational
        ("2 <= 2", 1),
        ("3 > 2", 1),
        ("3 >= 3", 1),
        ("1 << 3", 8),                      // shift
        ("8 >> 2", 2),
        ("7 + 3", 10),                      // additive
        ("7 - 3", 4),
        ("3 * 4", 12),                      // multiplicative
        ("12 / 3", 4),
        ("10 % 3", 1),

        // Unary operators
        ("-5", -5),
        ("--5", 4),                         // pre-decrement (5-1=4, not double-minus)
        ("++5", 6),                         // pre-increment
        ("+5", 5),
        ("!0", 1),
        ("!1", 0),
        ("~0", -1),                         // bitwise not

        // Mixed precedence
        ("1 + 2 * 3 + 4", 11),              // 1 + 6 + 4
        ("1 || 0 && 0", 1),                 // && before ||
        ("1 + 1 == 2", 1),                  // + before ==
        ("2 * 3 == 6", 1),
        ("1 < 2 == 1", 1),                  // < before ==  (1<2)=1, 1==1
        ("1 + 2 < 4", 1),                   // + before <

        // Ternary associativity (right-assoc)
        ("1 ? 2 : 0 ? 3 : 4", 2),           // 1 ? 2 : (0 ? 3 : 4)
        ("0 ? 2 : 1 ? 3 : 4", 3),           // 0 ? 2 : (1 ? 3 : 4)

        // Postfix
        ("5++", 5),                         // post-increment returns original
        ("5--", 5),

        // Complex
        ("1 + 2 * 3 - 4 / 2", 5),           // 1 + 6 - 2 = 5
        ("(1 + 2) * (3 + 4)", 21),
        ("1 ? 2 + 3 : 4", 5),               // condition with expression
        ("1 + 1 ? 2 : 3", 2),               // + before ?
    ];

    let mut passed = 0;
    let mut failed = 0;

    for (expr, expected) in tests {
        match eval(expr) {
            Ok(result) if result == *expected => {
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
    fn test_c11_precedence() {
        // Multiplicative > Additive
        assert_eq!(eval("1 + 2 * 3").unwrap(), 7);
        assert_eq!(eval("2 * 3 + 1").unwrap(), 7);
        assert_eq!(eval("(1 + 2) * 3").unwrap(), 9);
    }

    #[test]
    fn test_c11_associativity() {
        // Left-associative
        assert_eq!(eval("10 - 3 - 2").unwrap(), 5);
        assert_eq!(eval("100 / 10 / 2").unwrap(), 5);
        // Right-associative ternary
        assert_eq!(eval("1 ? 2 : 0 ? 3 : 4").unwrap(), 2);
        assert_eq!(eval("0 ? 2 : 1 ? 3 : 4").unwrap(), 3);
    }

    #[test]
    fn test_c11_all_levels() {
        // Level 3: ||
        assert_eq!(eval("1 || 0").unwrap(), 1);
        // Level 4: &&
        assert_eq!(eval("1 && 1").unwrap(), 1);
        // Level 5: |
        assert_eq!(eval("5 | 2").unwrap(), 7);
        // Level 6: ^
        assert_eq!(eval("7 ^ 3").unwrap(), 4);
        // Level 7: &
        assert_eq!(eval("7 & 3").unwrap(), 3);
        // Level 8: == !=
        assert_eq!(eval("2 == 2").unwrap(), 1);
        // Level 9: < > <= >=
        assert_eq!(eval("1 < 2").unwrap(), 1);
        // Level 10: << >>
        assert_eq!(eval("1 << 3").unwrap(), 8);
        // Level 11: + -
        assert_eq!(eval("3 + 4").unwrap(), 7);
        // Level 12: * / %
        assert_eq!(eval("3 * 4").unwrap(), 12);
    }

    #[test]
    fn test_c11_unary() {
        assert_eq!(eval("-5").unwrap(), -5);
        assert_eq!(eval("!0").unwrap(), 1);
        assert_eq!(eval("~0").unwrap(), -1);
        assert_eq!(eval("++5").unwrap(), 6);
        assert_eq!(eval("--5").unwrap(), 4);
    }

    #[test]
    fn test_c11_mixed() {
        assert_eq!(eval("1 || 0 && 0").unwrap(), 1);  // && before ||
        assert_eq!(eval("1 + 1 == 2").unwrap(), 1);   // + before ==
        assert_eq!(eval("1 + 2 < 4").unwrap(), 1);    // + before <
    }

    #[test]
    fn test_c11_ternary() {
        assert_eq!(eval("1 ? 2 : 3").unwrap(), 2);
        assert_eq!(eval("0 ? 2 : 3").unwrap(), 3);
        assert_eq!(eval("1 + 1 ? 2 : 3").unwrap(), 2);  // + before ?
    }

    #[test]
    fn test_c11_comma() {
        assert_eq!(eval("1, 2, 3").unwrap(), 3);
        assert_eq!(eval("1 + 2, 3 + 4").unwrap(), 7);
    }
}

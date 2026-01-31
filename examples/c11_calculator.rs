//! C11 Calculator - full C11 expression syntax with variables, function calls,
//! and user-defined operators.
//!
//! Variables live in a flat array; `&x` returns the slot index, `*n` dereferences slot n.
//! All assignment operators (=, +=, <<=, etc.) are BINOP variants.
//! Builtins: pow(a, b), min(a, b), max(a, b).
//! Custom operators: `operator @ pow right 3;` binds `@` to pow with right-assoc prec 3.
//! Statements separated by `;`, each pushes its result for testing.

use gazelle::Precedence;
use gazelle_macros::grammar;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum BinOp {
    // Arithmetic
    Add, Sub, Mul, Div, Mod,
    // Bitwise
    BitAnd, BitOr, BitXor, Shl, Shr,
    // Logical
    And, Or,
    // Comparison
    Eq, Ne, Lt, Gt, Le, Ge,
    // Assignment
    Assign, AddAssign, SubAssign, MulAssign, DivAssign, ModAssign,
    ShlAssign, ShrAssign, BitAndAssign, BitOrAssign, BitXorAssign,
    // User-defined
    Custom(char),
}

grammar! {
    grammar C11Calc {
        start stmts;
        terminals {
            NUM: Num,
            IDENT: Ident,
            LPAREN, RPAREN, LBRACK, RBRACK,
            COMMA, COLON, SEMI,
            TILDE, BANG,
            INC, DEC,
            OPERATOR, LEFT, RIGHT,
            prec QUESTION,
            prec STAR,
            prec AMP,
            prec PLUS,
            prec MINUS,
            prec BINOP: Binop,
        }

        stmts = stmts SEMI stmt | stmt | _;

        assoc: Assoc = LEFT @left | RIGHT @right;
        stmt = OPERATOR BINOP IDENT assoc NUM @def_op
             | expression @print;

        primary_expression: Expr = NUM @eval_num
                                 | IDENT @eval_ident
                                 | LPAREN expression RPAREN;

        postfix_expression: Expr = primary_expression
                                 | postfix_expression LBRACK expression RBRACK @eval_index
                                 | postfix_expression LPAREN RPAREN @eval_call0
                                 | postfix_expression LPAREN argument_expression_list RPAREN @eval_call
                                 | postfix_expression INC @eval_postinc
                                 | postfix_expression DEC @eval_postdec;

        argument_expression_list: ArgumentExpressionList = assignment_expression @eval_arg1
                                                         | argument_expression_list COMMA assignment_expression @eval_args;

        unary_expression: Expr = postfix_expression
                               | INC unary_expression @eval_preinc
                               | DEC unary_expression @eval_predec
                               | AMP cast_expression @eval_addr
                               | STAR cast_expression @eval_deref
                               | PLUS cast_expression @eval_uplus
                               | MINUS cast_expression @eval_uminus
                               | TILDE cast_expression @eval_bitnot
                               | BANG cast_expression @eval_lognot;

        cast_expression: Expr = unary_expression;

        binary_op: Binop = BINOP
                         | STAR @op_mul
                         | AMP @op_bitand
                         | PLUS @op_add
                         | MINUS @op_sub;

        assignment_expression: Expr = cast_expression
                                    | assignment_expression binary_op assignment_expression @eval_binary
                                    | assignment_expression QUESTION expression COLON assignment_expression @eval_ternary;

        expression: Expr = assignment_expression
                         | expression COMMA assignment_expression @eval_comma;
    }
}

/// Value: either an rvalue (plain integer) or an lvalue (slot index into vars).
#[derive(Clone, Copy, Debug)]
enum Val {
    Rval(i64),
    Lval(usize),
}

#[derive(Clone)]
struct OpDef {
    func: String,
    prec: Precedence,
}

struct Eval {
    vars: Vec<i64>,
    slot_names: Vec<String>,
    names: HashMap<String, usize>,
    custom_ops: HashMap<char, OpDef>,
    results: Vec<i64>,
}

impl Eval {
    fn new() -> Self {
        Self {
            vars: Vec::new(),
            slot_names: Vec::new(),
            names: HashMap::new(),
            custom_ops: HashMap::new(),
            results: Vec::new(),
        }
    }

    fn ensure_slot(&mut self, slot: usize) {
        if slot >= self.vars.len() {
            self.vars.resize(slot + 1, 0);
            self.slot_names.resize(slot + 1, String::new());
        }
    }

    fn slot(&mut self, name: &str) -> usize {
        if let Some(&s) = self.names.get(name) {
            return s;
        }
        let s = self.vars.len();
        self.vars.push(0);
        self.slot_names.push(name.to_string());
        self.names.insert(name.to_string(), s);
        s
    }

    fn get(&mut self, v: Val) -> i64 {
        match v {
            Val::Rval(n) => n,
            Val::Lval(slot) => { self.ensure_slot(slot); self.vars[slot] }
        }
    }

    fn store(&mut self, lhs: Val, val: i64) -> Val {
        match lhs {
            Val::Lval(slot) => { self.ensure_slot(slot); self.vars[slot] = val; Val::Lval(slot) }
            Val::Rval(_) => panic!("assignment to rvalue"),
        }
    }
}

fn builtin(name: &str, a: i64, b: i64) -> i64 {
    match name {
        "pow" => {
            if b < 0 { return 0; }
            let mut r = 1i64;
            for _ in 0..b { r = r.wrapping_mul(a); }
            r
        }
        "min" => a.min(b),
        "max" => a.max(b),
        _ => panic!("unknown builtin: {}", name),
    }
}

impl C11CalcActions for Eval {
    type Num = i64;
    type Ident = String;
    type Binop = BinOp;
    type Assoc = fn(u8) -> Precedence;
    type ArgumentExpressionList = Vec<Val>;
    type Expr = Val;

    // Associativity
    fn left(&mut self) -> fn(u8) -> Precedence { Precedence::Left }
    fn right(&mut self) -> fn(u8) -> Precedence { Precedence::Right }

    // Operator definition
    fn def_op(&mut self, op: BinOp, func: String, assoc: fn(u8) -> Precedence, prec: i64) {
        if let BinOp::Custom(ch) = op {
            self.custom_ops.insert(ch, OpDef { func, prec: assoc(prec as u8) });
        }
    }

    // Primary
    fn eval_num(&mut self, n: i64) -> Val { Val::Rval(n) }
    fn eval_ident(&mut self, name: String) -> Val { Val::Lval(self.slot(&name)) }

    // Postfix
    fn eval_index(&mut self, arr: Val, idx: Val) -> Val {
        let base = self.get(arr) as usize;
        let i = self.get(idx) as usize;
        Val::Lval(base + i)
    }
    fn eval_call0(&mut self, _func: Val) -> Val { Val::Rval(0) }
    fn eval_call(&mut self, func: Val, args: Vec<Val>) -> Val {
        let name = match func {
            Val::Lval(slot) => self.slot_names[slot].clone(),
            Val::Rval(_) => panic!("call on rvalue"),
        };
        match args.len() {
            2 => {
                let a = self.get(args[0]);
                let b = self.get(args[1]);
                Val::Rval(builtin(&name, a, b))
            }
            _ => panic!("{}: expected 2 args, got {}", name, args.len()),
        }
    }
    fn eval_postinc(&mut self, e: Val) -> Val {
        let v = self.get(e);
        if let Val::Lval(_) = e { self.store(e, v + 1); }
        Val::Rval(v)
    }
    fn eval_postdec(&mut self, e: Val) -> Val {
        let v = self.get(e);
        if let Val::Lval(_) = e { self.store(e, v - 1); }
        Val::Rval(v)
    }

    // Argument list
    fn eval_arg1(&mut self, e: Val) -> Vec<Val> { vec![e] }
    fn eval_args(&mut self, mut list: Vec<Val>, e: Val) -> Vec<Val> {
        list.push(e);
        list
    }

    // Unary
    fn eval_preinc(&mut self, e: Val) -> Val {
        let v = self.get(e) + 1;
        self.store(e, v);
        Val::Rval(v)
    }
    fn eval_predec(&mut self, e: Val) -> Val {
        let v = self.get(e) - 1;
        self.store(e, v);
        Val::Rval(v)
    }
    fn eval_addr(&mut self, e: Val) -> Val {
        match e {
            Val::Lval(slot) => Val::Rval(slot as i64),
            Val::Rval(_) => panic!("address of rvalue"),
        }
    }
    fn eval_deref(&mut self, e: Val) -> Val {
        Val::Lval(self.get(e) as usize)
    }
    fn eval_uplus(&mut self, e: Val) -> Val { Val::Rval(self.get(e)) }
    fn eval_uminus(&mut self, e: Val) -> Val { Val::Rval(-self.get(e)) }
    fn eval_bitnot(&mut self, e: Val) -> Val { Val::Rval(!self.get(e)) }
    fn eval_lognot(&mut self, e: Val) -> Val {
        Val::Rval(if self.get(e) == 0 { 1 } else { 0 })
    }

    // Binary op non-terminal
    fn op_mul(&mut self) -> BinOp { BinOp::Mul }
    fn op_bitand(&mut self) -> BinOp { BinOp::BitAnd }
    fn op_add(&mut self) -> BinOp { BinOp::Add }
    fn op_sub(&mut self) -> BinOp { BinOp::Sub }

    // Unified binary expression
    fn eval_binary(&mut self, l: Val, op: BinOp, r: Val) -> Val {
        match op {
            // Assignment operators
            BinOp::Assign => {
                let v = self.get(r);
                self.store(l, v)
            }
            BinOp::AddAssign => {
                let v = self.get(l) + self.get(r);
                self.store(l, v)
            }
            BinOp::SubAssign => {
                let v = self.get(l) - self.get(r);
                self.store(l, v)
            }
            BinOp::MulAssign => {
                let v = self.get(l) * self.get(r);
                self.store(l, v)
            }
            BinOp::DivAssign => {
                let v = self.get(l) / self.get(r);
                self.store(l, v)
            }
            BinOp::ModAssign => {
                let v = self.get(l) % self.get(r);
                self.store(l, v)
            }
            BinOp::ShlAssign => {
                let v = self.get(l) << self.get(r);
                self.store(l, v)
            }
            BinOp::ShrAssign => {
                let v = self.get(l) >> self.get(r);
                self.store(l, v)
            }
            BinOp::BitAndAssign => {
                let v = self.get(l) & self.get(r);
                self.store(l, v)
            }
            BinOp::BitOrAssign => {
                let v = self.get(l) | self.get(r);
                self.store(l, v)
            }
            BinOp::BitXorAssign => {
                let v = self.get(l) ^ self.get(r);
                self.store(l, v)
            }
            // User-defined operator
            BinOp::Custom(ch) => {
                let func = self.custom_ops.get(&ch)
                    .unwrap_or_else(|| panic!("undefined operator: {}", ch))
                    .func.clone();
                let a = self.get(l);
                let b = self.get(r);
                Val::Rval(builtin(&func, a, b))
            }
            // Arithmetic / comparison / logic
            _ => {
                let lv = self.get(l);
                let rv = self.get(r);
                Val::Rval(match op {
                    BinOp::Add => lv + rv,
                    BinOp::Sub => lv - rv,
                    BinOp::Mul => lv * rv,
                    BinOp::Div => lv / rv,
                    BinOp::Mod => lv % rv,
                    BinOp::BitAnd => lv & rv,
                    BinOp::BitOr => lv | rv,
                    BinOp::BitXor => lv ^ rv,
                    BinOp::Shl => lv << rv,
                    BinOp::Shr => lv >> rv,
                    BinOp::And => if lv != 0 && rv != 0 { 1 } else { 0 },
                    BinOp::Or => if lv != 0 || rv != 0 { 1 } else { 0 },
                    BinOp::Eq => if lv == rv { 1 } else { 0 },
                    BinOp::Ne => if lv != rv { 1 } else { 0 },
                    BinOp::Lt => if lv < rv { 1 } else { 0 },
                    BinOp::Gt => if lv > rv { 1 } else { 0 },
                    BinOp::Le => if lv <= rv { 1 } else { 0 },
                    BinOp::Ge => if lv >= rv { 1 } else { 0 },
                    _ => unreachable!(),
                })
            }
        }
    }

    // Ternary
    fn eval_ternary(&mut self, cond: Val, then_val: Val, else_val: Val) -> Val {
        if self.get(cond) != 0 { then_val } else { else_val }
    }

    // Expression
    fn eval_comma(&mut self, _l: Val, r: Val) -> Val { r }

    // Statement
    fn print(&mut self, e: Val) {
        let v = self.get(e);
        self.results.push(v);
    }
}

// =============================================================================
// Tokenizer (wraps gazelle::lexer::Lexer over any char iterator)
// =============================================================================

struct Tokenizer<I: Iterator<Item = char>> {
    lexer: gazelle::lexer::Lexer<I>,
}

#[cfg(test)]
impl<'a> Tokenizer<std::str::Chars<'a>> {
    fn from_str(input: &'a str) -> Self {
        Self { lexer: gazelle::lexer::Lexer::new(input) }
    }
}

impl<I: Iterator<Item = char>> Tokenizer<I> {
    fn from_chars(iter: I) -> Self {
        Self { lexer: gazelle::lexer::Lexer::from_chars(iter) }
    }

    fn next(&mut self, custom_ops: &HashMap<char, OpDef>) -> Result<Option<C11CalcTerminal<Eval>>, String> {
        use gazelle::lexer::Token;

        let tok = match self.lexer.next() {
            Some(Ok(t)) => t,
            Some(Err(e)) => return Err(e),
            None => return Ok(None),
        };

        Ok(Some(match tok {
            Token::Num(s) => C11CalcTerminal::Num(s.parse().unwrap()),
            Token::Ident(s) => match s.as_str() {
                "operator" => C11CalcTerminal::Operator,
                "left" => C11CalcTerminal::Left,
                "right" => C11CalcTerminal::Right,
                _ => C11CalcTerminal::Ident(s),
            },
            Token::Punct(c) => match c {
                '(' => C11CalcTerminal::Lparen,
                ')' => C11CalcTerminal::Rparen,
                '[' => C11CalcTerminal::Lbrack,
                ']' => C11CalcTerminal::Rbrack,
                ',' => C11CalcTerminal::Comma,
                ';' => C11CalcTerminal::Semi,
                _ => return Err(format!("Unexpected punctuation: {}", c)),
            },
            Token::Op(s) => match s.as_str() {
                "+"  => C11CalcTerminal::Plus(Precedence::Left(11)),
                "-"  => C11CalcTerminal::Minus(Precedence::Left(11)),
                "*"  => C11CalcTerminal::Star(Precedence::Left(12)),
                "&"  => C11CalcTerminal::Amp(Precedence::Left(7)),
                "~"  => C11CalcTerminal::Tilde,
                "!"  => C11CalcTerminal::Bang,
                ":"  => C11CalcTerminal::Colon,
                "?"  => C11CalcTerminal::Question(Precedence::Right(2)),
                "++" => C11CalcTerminal::Inc,
                "--" => C11CalcTerminal::Dec,
                // Binary operators
                "/"  => C11CalcTerminal::Binop(BinOp::Div, Precedence::Left(12)),
                "%"  => C11CalcTerminal::Binop(BinOp::Mod, Precedence::Left(12)),
                "<<" => C11CalcTerminal::Binop(BinOp::Shl, Precedence::Left(10)),
                ">>" => C11CalcTerminal::Binop(BinOp::Shr, Precedence::Left(10)),
                "<"  => C11CalcTerminal::Binop(BinOp::Lt, Precedence::Left(9)),
                ">"  => C11CalcTerminal::Binop(BinOp::Gt, Precedence::Left(9)),
                "<=" => C11CalcTerminal::Binop(BinOp::Le, Precedence::Left(9)),
                ">=" => C11CalcTerminal::Binop(BinOp::Ge, Precedence::Left(9)),
                "==" => C11CalcTerminal::Binop(BinOp::Eq, Precedence::Left(8)),
                "!=" => C11CalcTerminal::Binop(BinOp::Ne, Precedence::Left(8)),
                "^"  => C11CalcTerminal::Binop(BinOp::BitXor, Precedence::Left(6)),
                "|"  => C11CalcTerminal::Binop(BinOp::BitOr, Precedence::Left(5)),
                "&&" => C11CalcTerminal::Binop(BinOp::And, Precedence::Left(4)),
                "||" => C11CalcTerminal::Binop(BinOp::Or, Precedence::Left(3)),
                // Assignment operators
                "="   => C11CalcTerminal::Binop(BinOp::Assign, Precedence::Right(1)),
                "+="  => C11CalcTerminal::Binop(BinOp::AddAssign, Precedence::Right(1)),
                "-="  => C11CalcTerminal::Binop(BinOp::SubAssign, Precedence::Right(1)),
                "*="  => C11CalcTerminal::Binop(BinOp::MulAssign, Precedence::Right(1)),
                "/="  => C11CalcTerminal::Binop(BinOp::DivAssign, Precedence::Right(1)),
                "%="  => C11CalcTerminal::Binop(BinOp::ModAssign, Precedence::Right(1)),
                "<<=" => C11CalcTerminal::Binop(BinOp::ShlAssign, Precedence::Right(1)),
                ">>=" => C11CalcTerminal::Binop(BinOp::ShrAssign, Precedence::Right(1)),
                "&="  => C11CalcTerminal::Binop(BinOp::BitAndAssign, Precedence::Right(1)),
                "|="  => C11CalcTerminal::Binop(BinOp::BitOrAssign, Precedence::Right(1)),
                "^="  => C11CalcTerminal::Binop(BinOp::BitXorAssign, Precedence::Right(1)),
                // Single-char custom operator
                s if s.len() == 1 => {
                    let ch = s.chars().next().unwrap();
                    let prec = custom_ops.get(&ch)
                        .map(|d| d.prec)
                        .unwrap_or(Precedence::Left(0));
                    C11CalcTerminal::Binop(BinOp::Custom(ch), prec)
                }
                _ => return Err(format!("Unknown operator: {}", s)),
            },
            Token::Str(_) | Token::Char(_) => return self.next(custom_ops),
        }))
    }
}

fn run<I: Iterator<Item = char>>(tokenizer: &mut Tokenizer<I>) -> Result<Vec<i64>, String> {
    let mut parser = C11CalcParser::<Eval>::new();
    let mut actions = Eval::new();

    loop {
        match tokenizer.next(&actions.custom_ops)? {
            Some(tok) => parser.push(tok, &mut actions).map_err(|e| format!("{:?}", e))?,
            None => break,
        }
    }
    parser.finish(&mut actions).map_err(|e| format!("{:?}", e))?;

    Ok(actions.results)
}

fn main() {
    use std::io::Read;
    let mut tokenizer = Tokenizer::from_chars(
        std::io::stdin().lock().bytes().map(|b| b.unwrap() as char)
    );
    match run(&mut tokenizer) {
        Ok(results) => {
            for r in &results {
                println!("{}", r);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c11_calculator() {
        let input = "
            // Arithmetic
            1 + 2 * 3;
            2 * 3 + 1;
            (1 + 2) * 3;
            10 - 3 - 2;
            100 / 10 / 2;
            10 % 3;

            // All precedence levels
            1 || 0;
            0 && 1;
            5 | 2;
            7 ^ 3;
            7 & 3;
            2 == 2;
            2 != 3;
            1 < 2;
            2 <= 2;
            3 > 2;
            3 >= 3;
            1 << 3;
            8 >> 2;

            // Mixed precedence
            1 + 2 * 3 + 4;
            1 || 0 && 0;
            1 + 1 == 2;
            2 * 3 == 6;
            1 < 2 == 1;
            1 + 2 < 4;

            // Unary
            -5;
            +5;
            !0;
            !1;
            ~0;

            // Ternary
            1 ? 2 : 3;
            0 ? 2 : 3;
            1 ? 2 : 0 ? 3 : 4;
            0 ? 2 : 1 ? 3 : 4;
            1 + 1 ? 2 : 3;

            // Comma
            1, 2, 3;
            1 + 2, 3 + 4;

            // Postfix on rvalue
            5++;
            5--;

            // Assignment
            x = 10; x;
            x = 3; y = 4; x + y;
            x = 0; y = 0; x = y = 42; x;

            // Compound assignment
            a = 10; a += 5; a;
            b = 10; b -= 3; b;
            c = 10; c *= 2; c;
            d = 20; d /= 4; d;
            e = 10; e %= 3; e;
            f = 1; f <<= 3; f;
            g = 8; g >>= 2; g;
            h = 7; h &= 3; h;
            i = 5; i |= 2; i;
            j = 7; j ^= 3; j;

            // Ref/deref, inc/dec
            k = 42; *&k;
            l = 5; ++l; l;
            m = 5; --m; m;
            n = 5; n++; n;
            o = 5; o--; o;

            // Complex sequences
            p = 1; q = 2; r = p + q * 3; r;
            s = 10; s += 5; s *= 2; s;
            t = 0; u = 1; t ? 10 : u ? 20 : 30;

            // Builtin functions
            pow(2, 10);
            min(3, 7);
            max(3, 7);
            pow(2, 3) + 1;
            min(1 + 2, 4);

            // Custom operators
            operator @ pow right 13; 2 @ 10;
            operator @ pow right 13; 2 @ 3 @ 2;
            operator @ pow right 13; 1 + 2 @ 3;
            operator @ max left 13;
            operator # min left 13;
            3 @ 5 # 4
        ";
        assert_eq!(run(&mut Tokenizer::from_str(input)).unwrap(), vec![
            // Arithmetic
            7, 7, 9, 5, 5, 1,
            // All precedence levels
            1, 0, 7, 4, 3, 1, 1, 1, 1, 1, 1, 8, 2,
            // Mixed precedence
            11, 1, 1, 1, 1, 1,
            // Unary
            -5, 5, 1, 0, -1,
            // Ternary
            2, 3, 2, 3, 2,
            // Comma
            3, 7,
            // Postfix on rvalue
            5, 5,
            // Assignment
            10, 10,
            3, 4, 7,
            0, 0, 42, 42,
            // Compound assignment
            10, 15, 15,
            10, 7, 7,
            10, 20, 20,
            20, 5, 5,
            10, 1, 1,
            1, 8, 8,
            8, 2, 2,
            7, 3, 3,
            5, 7, 7,
            7, 4, 4,
            // Ref/deref, inc/dec
            42, 42,
            5, 6, 6,
            5, 4, 4,
            5, 5, 6,
            5, 5, 4,
            // Complex sequences
            1, 2, 7, 7,
            10, 15, 30, 30,
            0, 1, 20,
            // Builtins
            1024, 3, 7, 9, 3,
            // Custom operators
            1024,
            512,
            9,
            4,
        ]);
    }
}

//! C11 Calculator - full C11 expression syntax with variables, function calls,
//! and user-defined operators.
//!
//! Variables live in a flat array; `&x` returns the slot index, `*n` dereferences slot n.
//! All assignment operators (=, +=, <<=, etc.) are BINOP variants.
//! Builtins: pow(a, b), min(a, b), max(a, b).
//! Custom operators: `operator @ pow right 3;` binds `@` to pow with right-assoc prec 3.
//! Statements separated by `;`, each pushes its result for testing.

use gazelle::{Ignore, Precedence, Reduce};
use gazelle_macros::gazelle;
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

gazelle! {
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
            prec BINOP: Binop
        }

        stmts = stmts SEMI stmt => append | stmt => single | _ => empty;

        assoc = LEFT => left | RIGHT => right;
        stmt = OPERATOR BINOP IDENT assoc NUM => def_op
             | expression => print;

        primary_expression = NUM => num
                                 | IDENT => ident
                                 | LPAREN expression RPAREN => paren;

        postfix_expression = primary_expression => primary
                                 | postfix_expression LBRACK expression RBRACK => index
                                 | postfix_expression LPAREN RPAREN => call0
                                 | postfix_expression LPAREN argument_expression_list RPAREN => call
                                 | postfix_expression INC => postinc
                                 | postfix_expression DEC => postdec;

        argument_expression_list = assignment_expression => single
                                                         | argument_expression_list COMMA assignment_expression => append;

        unary_expression = postfix_expression => postfix
                               | INC unary_expression => preinc
                               | DEC unary_expression => predec
                               | AMP cast_expression => addr
                               | STAR cast_expression => deref
                               | PLUS cast_expression => uplus
                               | MINUS cast_expression => uminus
                               | TILDE cast_expression => bitnot
                               | BANG cast_expression => lognot;

        cast_expression = unary_expression => unary;

        binary_op = BINOP => binop
                         | STAR => mul
                         | AMP => bitand
                         | PLUS => add
                         | MINUS => sub;

        assignment_expression = cast_expression => cast
                                    | assignment_expression binary_op assignment_expression => binary
                                    | assignment_expression QUESTION expression COLON assignment_expression => ternary;

        expression = assignment_expression => assign
                         | expression COMMA assignment_expression => comma;
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

impl C11CalcTypes for Eval {
    type Error = gazelle::ParseError;
    type Num = i64;
    type Ident = String;
    type Binop = BinOp;
    type Assoc = fn(u8) -> Precedence;
    type Stmts = Ignore;
    type Stmt = ();
    type Primary_expression = Val;
    type Postfix_expression = Val;
    type Argument_expression_list = Vec<Val>;
    type Unary_expression = Val;
    type Cast_expression = Val;
    type Binary_op = BinOp;
    type Assignment_expression = Val;
    type Expression = Val;
}

// Associativity
impl Reduce<C11CalcAssoc, fn(u8) -> Precedence, gazelle::ParseError> for Eval {
    fn reduce(&mut self, node: C11CalcAssoc) -> Result<fn(u8) -> Precedence, gazelle::ParseError> {
        Ok(match node {
            C11CalcAssoc::Left => Precedence::Left,
            C11CalcAssoc::Right => Precedence::Right,
        })
    }
}

// Statement (untyped NT with => name → output is ())
impl Reduce<C11CalcStmt<Self>, (), gazelle::ParseError> for Eval {
    fn reduce(&mut self, node: C11CalcStmt<Self>) -> Result<(), gazelle::ParseError> {
        match node {
            C11CalcStmt::Def_op(op, func, assoc, prec) => {
                if let BinOp::Custom(ch) = op {
                    self.custom_ops.insert(ch, OpDef { func, prec: assoc(prec as u8) });
                }
            }
            C11CalcStmt::Print(e) => {
                let v = self.get(e);
                self.results.push(v);
            }
        }
        Ok(())
    }
}

// Primary expression
impl Reduce<C11CalcPrimary_expression<Self>, Val, gazelle::ParseError> for Eval {
    fn reduce(&mut self, node: C11CalcPrimary_expression<Self>) -> Result<Val, gazelle::ParseError> {
        Ok(match node {
            C11CalcPrimary_expression::Num(n) => Val::Rval(n),
            C11CalcPrimary_expression::Ident(name) => Val::Lval(self.slot(&name)),
            C11CalcPrimary_expression::Paren(e) => e,
        })
    }
}

// Postfix expression
impl Reduce<C11CalcPostfix_expression<Self>, Val, gazelle::ParseError> for Eval {
    fn reduce(&mut self, node: C11CalcPostfix_expression<Self>) -> Result<Val, gazelle::ParseError> {
        Ok(match node {
            C11CalcPostfix_expression::Primary(e) => e,
            C11CalcPostfix_expression::Index(arr, idx) => {
                let base = self.get(arr) as usize;
                let i = self.get(idx) as usize;
                Val::Lval(base + i)
            }
            C11CalcPostfix_expression::Call0(_func) => Val::Rval(0),
            C11CalcPostfix_expression::Call(func, args) => {
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
            C11CalcPostfix_expression::Postinc(e) => {
                let v = self.get(e);
                if let Val::Lval(_) = e { self.store(e, v + 1); }
                Val::Rval(v)
            }
            C11CalcPostfix_expression::Postdec(e) => {
                let v = self.get(e);
                if let Val::Lval(_) = e { self.store(e, v - 1); }
                Val::Rval(v)
            }
        })
    }
}

// Argument expression list
impl Reduce<C11CalcArgument_expression_list<Self>, Vec<Val>, gazelle::ParseError> for Eval {
    fn reduce(&mut self, node: C11CalcArgument_expression_list<Self>) -> Result<Vec<Val>, gazelle::ParseError> {
        Ok(match node {
            C11CalcArgument_expression_list::Single(e) => vec![e],
            C11CalcArgument_expression_list::Append(mut list, e) => {
                list.push(e);
                list
            }
        })
    }
}

// Unary expression
impl Reduce<C11CalcUnary_expression<Self>, Val, gazelle::ParseError> for Eval {
    fn reduce(&mut self, node: C11CalcUnary_expression<Self>) -> Result<Val, gazelle::ParseError> {
        Ok(match node {
            C11CalcUnary_expression::Postfix(e) => e,
            C11CalcUnary_expression::Preinc(e) => {
                let v = self.get(e) + 1;
                self.store(e, v);
                Val::Rval(v)
            }
            C11CalcUnary_expression::Predec(e) => {
                let v = self.get(e) - 1;
                self.store(e, v);
                Val::Rval(v)
            }
            C11CalcUnary_expression::Addr(e) => {
                match e {
                    Val::Lval(slot) => Val::Rval(slot as i64),
                    Val::Rval(_) => panic!("address of rvalue"),
                }
            }
            C11CalcUnary_expression::Deref(e) => Val::Lval(self.get(e) as usize),
            C11CalcUnary_expression::Uplus(e) => Val::Rval(self.get(e)),
            C11CalcUnary_expression::Uminus(e) => Val::Rval(-self.get(e)),
            C11CalcUnary_expression::Bitnot(e) => Val::Rval(!self.get(e)),
            C11CalcUnary_expression::Lognot(e) => {
                Val::Rval(if self.get(e) == 0 { 1 } else { 0 })
            }
        })
    }
}

// Cast expression (passthrough)
impl Reduce<C11CalcCast_expression<Self>, Val, gazelle::ParseError> for Eval {
    fn reduce(&mut self, node: C11CalcCast_expression<Self>) -> Result<Val, gazelle::ParseError> {
        let C11CalcCast_expression::Unary(e) = node;
        Ok(e)
    }
}

// Binary op non-terminal
impl Reduce<C11CalcBinary_op<Self>, BinOp, gazelle::ParseError> for Eval {
    fn reduce(&mut self, node: C11CalcBinary_op<Self>) -> Result<BinOp, gazelle::ParseError> {
        Ok(match node {
            C11CalcBinary_op::Binop(op) => op,
            C11CalcBinary_op::Mul => BinOp::Mul,
            C11CalcBinary_op::Bitand => BinOp::BitAnd,
            C11CalcBinary_op::Add => BinOp::Add,
            C11CalcBinary_op::Sub => BinOp::Sub,
        })
    }
}

// Assignment expression (binary + ternary)
impl Reduce<C11CalcAssignment_expression<Self>, Val, gazelle::ParseError> for Eval {
    fn reduce(&mut self, node: C11CalcAssignment_expression<Self>) -> Result<Val, gazelle::ParseError> {
        Ok(match node {
            C11CalcAssignment_expression::Cast(e) => e,
            C11CalcAssignment_expression::Binary(l, op, r) => {
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
            C11CalcAssignment_expression::Ternary(cond, then_val, else_val) => {
                if self.get(cond) != 0 { then_val } else { else_val }
            }
        })
    }
}

// Expression (comma)
impl Reduce<C11CalcExpression<Self>, Val, gazelle::ParseError> for Eval {
    fn reduce(&mut self, node: C11CalcExpression<Self>) -> Result<Val, gazelle::ParseError> {
        Ok(match node {
            C11CalcExpression::Assign(e) => e,
            C11CalcExpression::Comma(_l, r) => r,
        })
    }
}

// =============================================================================
// Tokenizer (wraps gazelle::lexer::Source over any char iterator)
// =============================================================================

struct Tokenizer<I: Iterator<Item = char>> {
    src: gazelle::lexer::Source<I>,
}

#[cfg(test)]
impl<'a> Tokenizer<std::str::Chars<'a>> {
    fn from_str(input: &'a str) -> Self {
        Self { src: gazelle::lexer::Source::from_str(input) }
    }
}

impl<I: Iterator<Item = char>> Tokenizer<I> {
    fn from_chars(iter: I) -> Self {
        Self { src: gazelle::lexer::Source::new(iter) }
    }

    fn next(&mut self, custom_ops: &HashMap<char, OpDef>) -> Result<Option<C11CalcTerminal<Eval>>, String> {
        self.src.skip_whitespace();
        while self.src.skip_line_comment("//") || self.src.skip_block_comment("/*", "*/") {
            self.src.skip_whitespace();
        }

        if self.src.at_end() {
            return Ok(None);
        }

        // Number
        if self.src.peek().map_or(false, |c| c.is_ascii_digit()) {
            let mut s = String::new();
            while let Some(c) = self.src.peek() {
                if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '_' {
                    s.push(c);
                    self.src.advance();
                } else {
                    break;
                }
            }
            // Remove underscores for parsing
            s.retain(|c| c != '_');
            return Ok(Some(C11CalcTerminal::NUM(s.parse().unwrap_or(0))));
        }

        // Identifier or keyword
        if self.src.peek().map_or(false, |c| c.is_alphabetic() || c == '_') {
            let mut s = String::new();
            while let Some(c) = self.src.peek() {
                if c.is_alphanumeric() || c == '_' {
                    s.push(c);
                    self.src.advance();
                } else {
                    break;
                }
            }
            return Ok(Some(match s.as_str() {
                "operator" => C11CalcTerminal::OPERATOR,
                "left" => C11CalcTerminal::LEFT,
                "right" => C11CalcTerminal::RIGHT,
                _ => C11CalcTerminal::IDENT(s),
            }));
        }

        // Punctuation
        if let Some(c) = self.src.peek() {
            match c {
                '(' => { self.src.advance(); return Ok(Some(C11CalcTerminal::LPAREN)); }
                ')' => { self.src.advance(); return Ok(Some(C11CalcTerminal::RPAREN)); }
                '[' => { self.src.advance(); return Ok(Some(C11CalcTerminal::LBRACK)); }
                ']' => { self.src.advance(); return Ok(Some(C11CalcTerminal::RBRACK)); }
                ',' => { self.src.advance(); return Ok(Some(C11CalcTerminal::COMMA)); }
                ';' => { self.src.advance(); return Ok(Some(C11CalcTerminal::SEMI)); }
                _ => {}
            }
        }

        // Multi-char operators (longest first for maximal munch)
        const MULTI_OPS: &[&str] = &[
            "<<=", ">>=",  // 0-1: three-char
            "++", "--", "<<", ">>", "<=", ">=", "==", "!=",  // 2-9
            "&&", "||", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=",  // 10-19
        ];
        const MULTI_TERMINALS: &[fn() -> C11CalcTerminal<Eval>] = &[
            || C11CalcTerminal::BINOP(BinOp::ShlAssign, Precedence::Right(1)),
            || C11CalcTerminal::BINOP(BinOp::ShrAssign, Precedence::Right(1)),
            || C11CalcTerminal::INC,
            || C11CalcTerminal::DEC,
            || C11CalcTerminal::BINOP(BinOp::Shl, Precedence::Left(10)),
            || C11CalcTerminal::BINOP(BinOp::Shr, Precedence::Left(10)),
            || C11CalcTerminal::BINOP(BinOp::Le, Precedence::Left(9)),
            || C11CalcTerminal::BINOP(BinOp::Ge, Precedence::Left(9)),
            || C11CalcTerminal::BINOP(BinOp::Eq, Precedence::Left(8)),
            || C11CalcTerminal::BINOP(BinOp::Ne, Precedence::Left(8)),
            || C11CalcTerminal::BINOP(BinOp::And, Precedence::Left(4)),
            || C11CalcTerminal::BINOP(BinOp::Or, Precedence::Left(3)),
            || C11CalcTerminal::BINOP(BinOp::AddAssign, Precedence::Right(1)),
            || C11CalcTerminal::BINOP(BinOp::SubAssign, Precedence::Right(1)),
            || C11CalcTerminal::BINOP(BinOp::MulAssign, Precedence::Right(1)),
            || C11CalcTerminal::BINOP(BinOp::DivAssign, Precedence::Right(1)),
            || C11CalcTerminal::BINOP(BinOp::ModAssign, Precedence::Right(1)),
            || C11CalcTerminal::BINOP(BinOp::BitAndAssign, Precedence::Right(1)),
            || C11CalcTerminal::BINOP(BinOp::BitOrAssign, Precedence::Right(1)),
            || C11CalcTerminal::BINOP(BinOp::BitXorAssign, Precedence::Right(1)),
        ];

        if let Some((idx, _)) = self.src.read_one_of(MULTI_OPS) {
            return Ok(Some(MULTI_TERMINALS[idx]()));
        }

        // Single-char operators
        if let Some(c) = self.src.peek() {
            self.src.advance();
            return Ok(Some(match c {
                '+' => C11CalcTerminal::PLUS(Precedence::Left(11)),
                '-' => C11CalcTerminal::MINUS(Precedence::Left(11)),
                '*' => C11CalcTerminal::STAR(Precedence::Left(12)),
                '&' => C11CalcTerminal::AMP(Precedence::Left(7)),
                '~' => C11CalcTerminal::TILDE,
                '!' => C11CalcTerminal::BANG,
                ':' => C11CalcTerminal::COLON,
                '?' => C11CalcTerminal::QUESTION(Precedence::Right(2)),
                '/' => C11CalcTerminal::BINOP(BinOp::Div, Precedence::Left(12)),
                '%' => C11CalcTerminal::BINOP(BinOp::Mod, Precedence::Left(12)),
                '<' => C11CalcTerminal::BINOP(BinOp::Lt, Precedence::Left(9)),
                '>' => C11CalcTerminal::BINOP(BinOp::Gt, Precedence::Left(9)),
                '^' => C11CalcTerminal::BINOP(BinOp::BitXor, Precedence::Left(6)),
                '|' => C11CalcTerminal::BINOP(BinOp::BitOr, Precedence::Left(5)),
                '=' => C11CalcTerminal::BINOP(BinOp::Assign, Precedence::Right(1)),
                // Custom operator
                ch => {
                    let prec = custom_ops.get(&ch)
                        .map(|d| d.prec)
                        .unwrap_or(Precedence::Left(0));
                    C11CalcTerminal::BINOP(BinOp::Custom(ch), prec)
                }
            }));
        }

        Err("Unexpected end of input".to_string())
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
    parser.finish(&mut actions).map_err(|(p, e)| p.format_error(&e))?;

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

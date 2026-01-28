# Building a Calculator: From Simple to C11

This tutorial builds a calculator step by step, starting simple and adding features until we have full C11 expression syntax with user-defined operators. Each step introduces a Gazelle feature to solve a real problem.

## Step 1: A Basic Calculator

Let's start with arithmetic: `+`, `-`, `*`, `/`. We need precedence — `*` binds tighter than `+` — so we use the traditional cascading grammar:

```rust
use gazelle_macros::grammar;

grammar! {
    grammar Calc {
        start expr;
        terminals {
            NUM: Num,
            PLUS, MINUS, STAR, SLASH,
            LPAREN, RPAREN,
        }

        expr: Expr = add_expr;

        add_expr: Expr = add_expr PLUS mul_expr @add
                       | add_expr MINUS mul_expr @sub
                       | mul_expr;

        mul_expr: Expr = mul_expr STAR primary @mul
                       | mul_expr SLASH primary @div
                       | primary;

        primary: Expr = NUM @num
                      | LPAREN expr RPAREN;  // passthrough - parens don't transform the value
    }
}
```

This works. `1 + 2 * 3` parses as `1 + (2 * 3)` because `mul_expr` is lower in the cascade than `add_expr`.

All expression non-terminals share the same type (`Expr`). Alternatives without `@action` are passthroughs — the value flows through unchanged, no method call needed. Notice `LPAREN expr RPAREN` has no action: parentheses affect parsing (grouping) but don't transform the value, so no method is required.

### What Gets Generated

The `grammar!` macro generates several things. First, a trait for semantic actions:

```rust
trait CalcActions {
    type Num;
    type Expr;

    fn add(&mut self, l: Self::Expr, r: Self::Expr) -> Self::Expr;
    fn sub(&mut self, l: Self::Expr, r: Self::Expr) -> Self::Expr;
    fn mul(&mut self, l: Self::Expr, r: Self::Expr) -> Self::Expr;
    fn div(&mut self, l: Self::Expr, r: Self::Expr) -> Self::Expr;
    fn num(&mut self, n: Self::Num) -> Self::Expr;
}
```

No `paren` method — the parenthesized expression is a passthrough.

Only two associated types: `Num` for the terminal payload, `Expr` for all expression non-terminals. Passthrough alternatives (without `@action`) don't generate methods.

Second, a terminal enum generic over the Actions trait:

```rust
enum CalcTerminal<A: CalcActions> {
    Num(A::Num),
    Plus,
    Minus,
    Star,
    Slash,
    Lparen,
    Rparen,
}
```

Third, a parser struct `CalcParser<A: CalcActions>` with `push` and `finish` methods.

### Implementing the Trait

```rust
struct Eval;

impl CalcActions for Eval {
    type Num = i64;
    type Expr = i64;

    fn add(&mut self, l: i64, r: i64) -> i64 { l + r }
    fn sub(&mut self, l: i64, r: i64) -> i64 { l - r }
    fn mul(&mut self, l: i64, r: i64) -> i64 { l * r }
    fn div(&mut self, l: i64, r: i64) -> i64 { l / r }
    fn num(&mut self, n: i64) -> i64 { n }
}
```

Five methods — only the operations that transform values.

### The Lexer

Gazelle provides a default lexer that handles numbers, identifiers, operators, and punctuation. We wrap it and map its tokens to our terminal enum:

```rust
use gazelle::lexer::{Lexer, Token};

fn tokenize(input: &str) -> Result<Vec<CalcTerminal<Eval>>, String> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    while let Some(result) = lexer.next() {
        let tok = result?;
        tokens.push(match tok {
            Token::Num(s) => CalcTerminal::Num(s.parse().unwrap()),
            Token::Punct('(') => CalcTerminal::Lparen,
            Token::Punct(')') => CalcTerminal::Rparen,
            Token::Op(s) => match s.as_str() {
                "+" => CalcTerminal::Plus,
                "-" => CalcTerminal::Minus,
                "*" => CalcTerminal::Star,
                "/" => CalcTerminal::Slash,
                _ => return Err(format!("unknown operator: {}", s)),
            },
            _ => return Err(format!("unexpected token: {:?}", tok)),
        });
    }
    Ok(tokens)
}
```

`Lexer` returns `Token` variants: `Num(String)`, `Ident(String)`, `Op(String)`, `Punct(char)`. We map each to our `CalcTerminal`.

### Running the Parser

```rust
fn run(input: &str) -> Result<i64, String> {
    let tokens = tokenize(input)?;
    let mut parser = CalcParser::<Eval>::new();
    let mut actions = Eval;

    for token in tokens {
        parser.push(token, &mut actions).map_err(|e| format!("{:?}", e))?;
    }
    parser.finish(&mut actions).map_err(|e| format!("{:?}", e))
}

fn main() {
    let result = run("1 + 2 * 3").unwrap();
    println!("{}", result);  // prints: 7
}
```

The parser is push-based: you feed tokens one at a time. Each `push` may trigger reductions, calling methods on your `actions` impl. When input is exhausted, `finish` returns the final value — the result of reducing the start symbol (`expr`).

## Step 2: Multiple Statements

Let's allow multiple expressions separated by semicolons. We can simplify: the passthrough `expr = add_expr` is unnecessary since they share the same type.

```rust
grammar! {
    grammar Calc {
        start stmts;
        terminals {
            NUM: Num,
            PLUS, MINUS, STAR, SLASH,
            LPAREN, RPAREN,
            SEMI,
        }

        stmts = stmt*;
        stmt = add_expr @print SEMI;

        add_expr: Expr = add_expr PLUS mul_expr @add
                       | add_expr MINUS mul_expr @sub
                       | mul_expr;

        mul_expr: Expr = mul_expr STAR primary @mul
                       | mul_expr SLASH primary @div
                       | primary;

        primary: Expr = NUM @num
                      | LPAREN add_expr RPAREN;
    }
}
```

Gazelle supports modifiers on symbols: `*` (zero or more), `+` (one or more), `?` (optional). Here `stmt*` means zero or more statements. Each `stmt` prints an expression and expects a semicolon.

Since `stmts` is untyped, `finish` returns `Result<(), _>`. The `print` action just prints directly:

```rust
struct Eval;

impl CalcActions for Eval {
    type Num = i64;
    type Expr = i64;

    fn print(&mut self, e: i64) { println!("{}", e); }
    fn add(&mut self, l: i64, r: i64) -> i64 { l + r }
    fn sub(&mut self, l: i64, r: i64) -> i64 { l - r }
    fn mul(&mut self, l: i64, r: i64) -> i64 { l * r }
    fn div(&mut self, l: i64, r: i64) -> i64 { l / r }
    fn num(&mut self, n: i64) -> i64 { n }
}

fn run(input: &str) -> Result<(), String> {
    let tokens = tokenize(input)?;
    let mut parser = CalcParser::<Eval>::new();
    let mut actions = Eval;

    for token in tokens {
        parser.push(token, &mut actions).map_err(|e| format!("{:?}", e))?;
    }
    parser.finish(&mut actions).map_err(|e| format!("{:?}", e))
}
```

Now users can type several calculations:

```
> 1 + 2; 3 * 4; 5;
3
12
5
```

## Step 3: The Cascade Problem

Our grammar handles 2 precedence levels with 2 non-terminals. C11 has 15 levels. That means 15 non-terminals:

```
assignment_expr → conditional_expr → logical_or_expr → logical_and_expr →
bitwise_or_expr → bitwise_xor_expr → bitwise_and_expr → equality_expr →
relational_expr → shift_expr → additive_expr → multiplicative_expr →
cast_expr → unary_expr → postfix_expr → primary_expr
```

Each level has the same pattern: `this_level OP next_level`. Every rule produces the same semantic result — a value. The repetition is painful and obscures the actual structure: binary operation, two operands, one operator.

## Step 4: Runtime Precedence

Gazelle solves this with precedence terminals. Mark operators with `prec` and attach precedence values in the lexer:

```rust
grammar! {
    grammar Calc {
        start stmts;
        terminals {
            NUM: i64,
            LPAREN, RPAREN,
            SEMI,
            prec BINOP: BinOp,
        }

        stmts = stmts SEMI expr @stmt
              | expr @first
              | _;

        expr = expr BINOP expr @binary
             | primary;

        primary = NUM @num
                | LPAREN expr RPAREN @paren;
    }
}
```

One rule for all binary expressions. The lexer provides precedence:

```rust
fn tokenize(op: &str) -> CalcTerminal {
    match op {
        "+" => CalcTerminal::Binop(BinOp::Add, Precedence::left(11)),
        "-" => CalcTerminal::Binop(BinOp::Sub, Precedence::left(11)),
        "*" => CalcTerminal::Binop(BinOp::Mul, Precedence::left(12)),
        "/" => CalcTerminal::Binop(BinOp::Div, Precedence::left(12)),
        // ...
    }
}
```

Higher numbers bind tighter. `Precedence::left` for left-associative, `Precedence::right` for right-associative.

The trait simplifies:

```rust
impl CalcActions for Eval {
    type Expr = i64;
    type Primary = i64;
    type BinOp = BinOp;

    fn binary(&mut self, l: i64, op: BinOp, r: i64) -> i64 {
        match op {
            BinOp::Add => l + r,
            BinOp::Sub => l - r,
            BinOp::Mul => l * r,
            BinOp::Div => l / r,
        }
    }
    fn num(&mut self, n: i64) -> i64 { n }
    fn paren(&mut self, e: i64) -> i64 { e }
}
```

Now we can add all of C's binary operators by extending the `BinOp` enum and the `tokenize` function. The grammar doesn't change.

## Step 5: Variables

Let's add variables. We need to distinguish lvalues (assignable locations) from rvalues (plain values):

```rust
enum Val {
    Rval(i64),
    Lval(usize),  // index into variable storage
}
```

Update the grammar:

```rust
primary = NUM @num
        | IDENT @var
        | LPAREN expr RPAREN @paren;
```

And add assignment to the operators:

```rust
"=" => CalcTerminal::Binop(BinOp::Assign, Precedence::right(1)),
```

Assignment is right-associative (`x = y = 5` assigns right-to-left) and lowest precedence.

```rust
fn binary(&mut self, l: Val, op: BinOp, r: Val) -> Val {
    match op {
        BinOp::Assign => {
            let v = self.get(r);
            self.store(l, v)
        }
        BinOp::Add => Val::Rval(self.get(l) + self.get(r)),
        // ...
    }
}
```

Now: `x = 10; y = 20; x + y` → `30`

## Step 6: Unary Operators

C has unary `+`, `-`, `!`, `~`, and the dual-role `*` (dereference) and `&` (address-of). Here's the problem: `*` and `&` are also binary operators (multiply and bitwise AND).

If `STAR` is a `prec BINOP`, we can't use it in unary rules:

```rust
unary_expr = STAR expr @deref   // won't work - STAR is BINOP
```

Solution: declare them as separate precedence terminals:

```rust
terminals {
    // ...
    prec STAR,
    prec AMP,
    prec PLUS,
    prec MINUS,
    prec BINOP: BinOp,
}
```

Now we can use them in unary rules:

```rust
unary_expr = STAR unary_expr @deref
           | AMP unary_expr @addr
           | PLUS unary_expr @uplus
           | MINUS unary_expr @uminus
           | BANG unary_expr @lognot
           | TILDE unary_expr @bitnot
           | postfix_expr;
```

But wait — now binary expressions don't see `STAR` as an operator. We need to collect all binary operators into one place:

```rust
binary_op = BINOP @op_binop
          | STAR @op_mul
          | AMP @op_bitand
          | PLUS @op_add
          | MINUS @op_sub;

expr = expr binary_op expr @binary
     | unary_expr;
```

When `STAR` (precedence 12) reduces to `binary_op`, the non-terminal inherits that precedence. The parser resolves `1 + 2 * 3` correctly — the `binary_op` carrying `STAR`'s precedence wins over `PLUS`.

## Step 7: Postfix Expressions

Function calls, array indexing, post-increment/decrement:

```rust
postfix_expr = primary @primary
             | postfix_expr LPAREN RPAREN @call0
             | postfix_expr LPAREN args RPAREN @call
             | postfix_expr LBRACK expr RBRACK @index
             | postfix_expr INC @postinc
             | postfix_expr DEC @postdec;

args = expr @arg1
     | args COMMA expr @arg;
```

Implementation handles function calls — we'll support builtins like `pow(2, 10)`:

```rust
fn call(&mut self, func: Val, args: Vec<Val>) -> Val {
    let name = self.slot_name(func);
    match name.as_str() {
        "pow" => {
            let base = self.get(args[0]);
            let exp = self.get(args[1]);
            Val::Rval(base.pow(exp as u32))
        }
        "min" => Val::Rval(self.get(args[0]).min(self.get(args[1]))),
        "max" => Val::Rval(self.get(args[0]).max(self.get(args[1]))),
        _ => panic!("unknown function: {}", name),
    }
}
```

## Step 8: User-Defined Operators

The payoff. Let users define new operators:

```
operator @ pow right 13
2 @ 3 @ 2
```

This defines `@` as a right-associative operator at precedence 13, bound to the `pow` function. Then `2 @ 3 @ 2` computes `2^(3^2) = 2^9 = 512`.

Add statements for operator definition:

```rust
terminals {
    // ...
    LEFT, RIGHT,
}

assoc: Assoc = LEFT @left | RIGHT @right;

stmt = OPERATOR BINOP IDENT assoc NUM @def_op
     | add_expr @print SEMI;
```

`LEFT` and `RIGHT` are unit terminals — no payload. The actions return the precedence constructor:

```rust
type Assoc = fn(u8) -> Precedence;

fn left(&mut self) -> fn(u8) -> Precedence { Precedence::left }
fn right(&mut self) -> fn(u8) -> Precedence { Precedence::right }

fn def_op(&mut self, op: BinOp, func: String, assoc: fn(u8) -> Precedence, prec: i64) {
    if let BinOp::Custom(ch) = op {
        self.custom_ops.insert(ch, OpDef { func, prec: assoc(prec as u8) });
    }
}
```

The terminals distinguish left from right. The actions provide the behavior.

This shows the power of trait-based semantics: `type Assoc = fn(u8) -> Precedence` uses a function type as the associated type. You're not limited to simple data — any Rust type works.

Now the lexer needs to see this table. This is **lexer feedback** — information flowing from parser back to lexer.

The parse loop makes it natural:

```rust
let mut parser = CalcParser::new();
let mut actions = Eval::new();

loop {
    // Lexer sees the current custom_ops table
    match tokenizer.next(&actions.custom_ops)? {
        Some(tok) => parser.push(tok, &mut actions)?,
        None => break,
    }
}
```

The lexer consults `custom_ops` to get precedence for unknown single-character operators:

```rust
fn next(&mut self, custom_ops: &HashMap<char, OpDef>) -> Option<Token> {
    // ...
    if s.len() == 1 {
        let ch = s.chars().next().unwrap();
        let prec = custom_ops.get(&ch)
            .map(|d| d.prec)
            .unwrap_or(Precedence::left(0));
        return Some(Token::Binop(BinOp::Custom(ch), prec));
    }
}
```

An unknown operator gets precedence 0 (lowest) until defined. Once defined, subsequent uses get the registered precedence.

## The Complete Picture

We've built:

1. **Basic arithmetic** with cascading grammar
2. **Statements** for interactive use
3. **Runtime precedence** to collapse the cascade
4. **Variables** with lvalue/rvalue distinction
5. **Unary operators** including dual-role `*` and `&`
6. **Precedence-carrying non-terminals** to unify binary operators
7. **Postfix expressions** for calls and indexing
8. **User-defined operators** with lexer feedback

The final grammar is ~40 lines. It handles the full C11 expression syntax plus user-defined operators. The grammar is clean because precedence lives in tokens, not grammar structure. Lexer feedback works because you control the parse loop.

See `examples/c11_calculator.rs` for the complete implementation.

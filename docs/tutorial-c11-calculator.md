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

The `grammar!` macro generates several things. First, two traits — one for types, one for actions:

```rust
trait CalcTypes {
    type Num;
    type Expr;
}

trait CalcActions<E: From<ParseError> = ParseError>: CalcTypes {
    fn add(&mut self, l: Self::Expr, r: Self::Expr) -> Result<Self::Expr, E>;
    fn sub(&mut self, l: Self::Expr, r: Self::Expr) -> Result<Self::Expr, E>;
    fn mul(&mut self, l: Self::Expr, r: Self::Expr) -> Result<Self::Expr, E>;
    fn div(&mut self, l: Self::Expr, r: Self::Expr) -> Result<Self::Expr, E>;
    fn num(&mut self, n: Self::Num) -> Result<Self::Expr, E>;
}
```

No `paren` method — the parenthesized expression is a passthrough.

Only two associated types: `Num` for the terminal payload, `Expr` for all expression non-terminals. Passthrough alternatives (without `@action`) don't generate methods. Action methods return `Result` — the error type defaults to `ParseError` but can be customized for actions that can fail with domain errors.

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

### Implementing the Traits

```rust
struct Eval;

impl CalcTypes for Eval {
    type Num = i64;
    type Expr = i64;
}

impl CalcActions for Eval {
    fn add(&mut self, l: i64, r: i64) -> Result<i64, ParseError> { Ok(l + r) }
    fn sub(&mut self, l: i64, r: i64) -> Result<i64, ParseError> { Ok(l - r) }
    fn mul(&mut self, l: i64, r: i64) -> Result<i64, ParseError> { Ok(l * r) }
    fn div(&mut self, l: i64, r: i64) -> Result<i64, ParseError> { Ok(l / r) }
    fn num(&mut self, n: i64) -> Result<i64, ParseError> { Ok(n) }
}
```

Five methods — only the operations that transform values.

### The Lexer

Gazelle provides a `Source` type with composable methods for building lexers. We use its methods to read tokens and map them to our terminal enum:

```rust
use gazelle::lexer::Source;

fn tokenize(input: &str) -> Result<Vec<CalcTerminal<Eval>>, String> {
    let mut src = Source::from_str(input);
    let mut tokens = Vec::new();

    loop {
        src.skip_whitespace();
        if src.at_end() { break; }

        if let Some(span) = src.read_number() {
            let s = &input[span.start..span.end];
            tokens.push(CalcTerminal::Num(s.parse().unwrap()));
        } else if let Some(c) = src.peek() {
            src.advance();
            tokens.push(match c {
                '(' => CalcTerminal::Lparen,
                ')' => CalcTerminal::Rparen,
                '+' => CalcTerminal::Plus,
                '-' => CalcTerminal::Minus,
                '*' => CalcTerminal::Star,
                '/' => CalcTerminal::Slash,
                _ => return Err(format!("unexpected char: {}", c)),
            });
        }
    }
    Ok(tokens)
}
```

`Source` provides methods like `read_number()`, `read_ident()`, `skip_whitespace()` that return `Span` values you can use to extract text from the input.

### Running the Parser

```rust
fn run(input: &str) -> Result<i64, String> {
    let tokens = tokenize(input)?;
    let mut parser = CalcParser::<Eval>::new();
    let mut actions = Eval;

    for token in tokens {
        parser.push(token, &mut actions).map_err(|e| parser.format_error(&e))?;
    }
    parser.finish(&mut actions).map_err(|(p, e)| p.format_error(&e))
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

impl CalcTypes for Eval {
    type Num = i64;
    type Expr = i64;
}

impl CalcActions for Eval {
    fn print(&mut self, e: i64) -> Result<(), ParseError> { println!("{}", e); Ok(()) }
    fn add(&mut self, l: i64, r: i64) -> Result<i64, ParseError> { Ok(l + r) }
    fn sub(&mut self, l: i64, r: i64) -> Result<i64, ParseError> { Ok(l - r) }
    fn mul(&mut self, l: i64, r: i64) -> Result<i64, ParseError> { Ok(l * r) }
    fn div(&mut self, l: i64, r: i64) -> Result<i64, ParseError> { Ok(l / r) }
    fn num(&mut self, n: i64) -> Result<i64, ParseError> { Ok(n) }
}

fn run(input: &str) -> Result<(), String> {
    let tokens = tokenize(input)?;
    let mut parser = CalcParser::<Eval>::new();
    let mut actions = Eval;

    for token in tokens {
        parser.push(token, &mut actions).map_err(|e| parser.format_error(&e))?;
    }
    parser.finish(&mut actions).map_err(|(p, e)| p.format_error(&e))
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
            NUM: Num,
            LPAREN, RPAREN,
            SEMI,
            prec BINOP: Binop,
        }

        stmts = stmts SEMI expr @stmt
              | expr @first
              | _;

        expr: Expr = expr BINOP expr @binary
                   | primary;

        primary: Expr = NUM @num
                      | LPAREN expr RPAREN;  // passthrough - same type flows through
    }
}
```

One rule for all binary expressions. The lexer provides precedence:

```rust
fn tokenize(op: &str) -> CalcTerminal {
    match op {
        "+" => CalcTerminal::Binop(BinOp::Add, Precedence::Left(11)),
        "-" => CalcTerminal::Binop(BinOp::Sub, Precedence::Left(11)),
        "*" => CalcTerminal::Binop(BinOp::Mul, Precedence::Left(12)),
        "/" => CalcTerminal::Binop(BinOp::Div, Precedence::Left(12)),
        // ...
    }
}
```

Higher numbers bind tighter. `Precedence::Left` for left-associative, `Precedence::Right` for right-associative.

The trait simplifies:

```rust
impl CalcTypes for Eval {
    type Num = i64;
    type Binop = BinOp;
    type Expr = i64;
}

impl CalcActions for Eval {
    fn binary(&mut self, l: i64, op: BinOp, r: i64) -> Result<i64, ParseError> {
        Ok(match op {
            BinOp::Add => l + r,
            BinOp::Sub => l - r,
            BinOp::Mul => l * r,
            BinOp::Div => l / r,
        })
    }
    fn num(&mut self, n: i64) -> Result<i64, ParseError> { Ok(n) }
    // No paren method needed - passthrough!
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
fn binary(&mut self, l: Val, op: BinOp, r: Val) -> Result<Val, ParseError> {
    Ok(match op {
        BinOp::Assign => {
            let v = self.get(r);
            self.store(l, v)
        }
        BinOp::Add => Val::Rval(self.get(l) + self.get(r)),
        // ...
    })
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
unary_expr: Expr = STAR unary_expr @deref
                 | AMP unary_expr @addr
                 | PLUS unary_expr @uplus
                 | MINUS unary_expr @uminus
                 | BANG unary_expr @lognot
                 | TILDE unary_expr @bitnot
                 | postfix_expr;
```

But wait — now binary expressions don't see `STAR` as an operator. We need to collect all binary operators into one place:

```rust
binary_op: Binop = BINOP            // passthrough - BINOP already has type Binop
                 | STAR @op_mul
                 | AMP @op_bitand
                 | PLUS @op_add
                 | MINUS @op_sub;

expr: Expr = expr binary_op expr @binary
           | unary_expr;
```

When `STAR` (precedence 12) reduces to `binary_op`, the non-terminal inherits that precedence. The parser resolves `1 + 2 * 3` correctly — the `binary_op` carrying `STAR`'s precedence wins over `PLUS`.

Note that `BINOP` is a passthrough — it already has type `Binop`, so no action method is needed. The other operators (`STAR`, `AMP`, etc.) are untyped precedence terminals, so they need action methods to produce a `Binop` value.

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
fn call(&mut self, func: Val, args: Vec<Val>) -> Result<Val, ParseError> {
    let name = self.slot_name(func);
    Ok(match name.as_str() {
        "pow" => {
            let base = self.get(args[0]);
            let exp = self.get(args[1]);
            Val::Rval(base.pow(exp as u32))
        }
        "min" => Val::Rval(self.get(args[0]).min(self.get(args[1]))),
        "max" => Val::Rval(self.get(args[0]).max(self.get(args[1]))),
        _ => panic!("unknown function: {}", name),
    })
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

fn left(&mut self) -> Result<fn(u8) -> Precedence, ParseError> { Ok(Precedence::left) }
fn right(&mut self) -> Result<fn(u8) -> Precedence, ParseError> { Ok(Precedence::right) }

fn def_op(&mut self, op: BinOp, func: String, assoc: fn(u8) -> Precedence, prec: i64) -> Result<(), ParseError> {
    if let BinOp::Custom(ch) = op {
        self.custom_ops.insert(ch, OpDef { func, prec: assoc(prec as u8) });
    }
    Ok(())
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

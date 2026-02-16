# Gazelle Reference

Complete reference for the Gazelle parser generator.

## Project Status

**Working and tested:**
- Grammar definition (`.gzl` files and `gazelle!` macro)
- Minimal LR table generation
- Type-safe parser generation with `Types`/`Reducer` traits
- Precedence terminals (`prec`) for runtime operator precedence
- Modifiers: `?` (optional), `*` (zero+), `+` (one+), `%` (separated list)
- Expected conflict declarations (`expect N rr/sr`)
- Token range tracking for source spans
- Push-based parsing (you control the loop)
- Detailed error messages with parser state context
- Automatic error recovery (Dijkstra-based minimum-cost repair)

**Not yet implemented:**
- Debug dump via `GAZELLE_DEBUG` env var
- Grammar visualization (FIRST/FOLLOW sets)

**Tested on:**
- Calculator with user-defined operators
- C11 parser (complete grammar, 18 tests)

## Table of Contents

- [Grammar Syntax](#grammar-syntax)
- [The gazelle! Macro](#the-grammar-macro)
- [Generated Types](#generated-types)
- [Using the Parser](#using-the-parser)
- [Advanced Features](#advanced-features)

---

## Grammar Syntax

Gazelle grammars can be written in `.gzl` files or inline with the `gazelle!` macro.

### Basic Structure

`.gzl` files contain the grammar directly:

```
start rule_name;

terminals {
    // terminal declarations
}

// rule definitions
```

In the `gazelle!` macro, the grammar is wrapped with `grammar Name { ... }`:

```rust
gazelle! {
    grammar Name {
        start rule_name;
        terminals { ... }
        // rule definitions
    }
}
```

### Terminal Declarations

Terminals are tokens from your lexer. Declare them in the `terminals` block:

```
terminals {
    // Simple terminal (no payload, becomes () in generated code)
    LPAREN,
    RPAREN,

    // Terminal with payload type
    NUM: Number,        // Generates associated type `Number`
    IDENT: Identifier,

    // Precedence terminal (for operator precedence parsing)
    prec OP: Operator,  // Carries precedence at runtime
    prec BINOP,         // Prec terminal without payload
}
```

**Terminal naming convention:** Terminals should be UPPERCASE to distinguish from non-terminals.

### Rule Definitions

Rules define the grammar's structure. Every alternative requires `=> name`:

```
// Single alternative
expr = expr PLUS term => add;

// Multiple alternatives (separated by |)
expr = expr PLUS term => add
     | expr MINUS term => sub
     | term => term;
```

### Actions and Enum Generation

Each `=> name` generates an enum variant. Untyped symbols (no `: Type`) are omitted from variant fields:

```
expr = expr PLUS term => add;
// Generates enum variant: Expr::Add(A::Expr, A::Term)
// PLUS is untyped, so it's omitted — only typed symbols become fields
```

**Untyped rules** - if the non-terminal has no type annotation, output is `()`. The `Reducer` is still called for side effects:

```
// statement has no type annotation — output is ()
statement = expr SEMI => on_statement;
// Generates: Statement::OnStatement(A::Expr)
// Reducer<Statement<Self>> returns Result<(), Error>
```

### Modifiers

**Optional** (`?`) - zero or one:
```
trailing_comma = COMMA?;
// Generates Option<T> where T is the symbol's type
```

**Repetition** (`*`) - zero or more:
```
args = arg*;
// Generates Vec<T>
```

**One or more** (`+`) - at least one:
```
statements = statement+;
// Generates Vec<T>
```

**Separated list** (`%`) - one or more separated by a delimiter:
```
args = expr % COMMA;
// Generates Vec<T> where T is expr's type
// Parses: expr, expr COMMA expr, expr COMMA expr COMMA expr, ...
```

### Expect Declarations

Declare expected conflicts to suppress errors:

```
start translation_unit;
expect 3 rr;  // Expect 3 reduce/reduce conflicts
expect 1 sr;  // Expect 1 shift/reduce conflict

// ... rest of grammar
```

Use this for grammars with known ambiguities (like C's typedef or dangling else).

---

## The gazelle! Macro

The `gazelle!` macro generates a type-safe parser at compile time.

### Basic Usage

```rust
use gazelle_macros::gazelle;
use gazelle::Precedence;

gazelle! {
    grammar Calc {
        start expr;

        terminals {
            NUM: _,
            LPAREN, RPAREN,
            prec OP: _,
        }

        expr = expr OP expr => binop
             | NUM => literal
             | LPAREN expr RPAREN => paren;
    }
}
```

### Visibility

Add `pub` before `grammar` for public visibility:

```rust
gazelle! {
    pub grammar MyParser {
        // ...
    }
}
```

Or with restricted visibility:

```rust
gazelle! {
    pub(crate) grammar MyParser {
        // ...
    }
}
```

---

## Generated Types

The macro generates a module (snake_case of grammar name, e.g., `calc`) containing:

### Types and Actions Traits

`Types` declares associated types and the error type. `Actions` is auto-implemented for any type satisfying `Types` + all required `Reducer` bounds:

```rust
pub trait Types: Sized {
    type Error: From<ParseError>;
    type Num: Debug;
    type Op: Debug;
    type Expr: Debug;
    fn set_token_range(&mut self, start: usize, end: usize) {}
}

pub trait Actions: Types + Reducer<Expr<Self>> { }
impl<T: Types + Reducer<Expr<T>>> Actions for T { }
```

### Per-Node Enums

Each non-terminal with named alternatives generates an enum generic over `A: Types`:

```rust
pub enum Expr<A: Types> {
    Binop(A::Expr, A::Op, A::Expr),
    Literal(A::Num),
}

impl<A: Types> AstNode for Expr<A> {
    type Output = A::Expr;
    type Error = A::Error;
}
```

A blanket `Reducer` impl handles identity (CST), `Box<N>` (auto-boxing), and `Ignore` (discard) automatically. You only write a custom `Reducer` impl when you need custom logic.

### Terminal Enum

Represents input tokens:

```rust
pub enum Terminal<A: Types> {
    Num(A::Num),
    Lparen,
    Rparen,
    Op(A::Op, Precedence),  // prec terminals include Precedence
}
```

### Parser Struct

```rust
pub struct Parser<A: Types> { /* ... */ }

impl<A: Actions> Parser<A> {
    pub fn new() -> Self;
    pub fn push(&mut self, terminal: Terminal<A>, actions: &mut A) -> Result<(), A::Error>;
    pub fn finish(self, actions: &mut A) -> Result<A::Expr, (Self, A::Error)>;
    pub fn state(&self) -> usize;
    pub fn format_error(&self, err: &ParseError) -> String;
}
```

Note: `finish` returns `(Self, A::Error)` on error, giving back the parser so you can still call `format_error`.

---

## Using the Parser

### Step 1: Implement Types and Reducers

```rust
use gazelle::{ParseError, Reducer};

struct Evaluator;

impl calc::Types for Evaluator {
    type Error = ParseError;
    type Num = f64;
    type Op = char;
    type Expr = f64;
}

impl Reducer<calc::Expr<Self>> for Evaluator {
    fn reduce(&mut self, node: calc::Expr<Self>) -> Result<f64, ParseError> {
        Ok(match node {
            calc::Expr::Binop(left, op, right) => match op {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                _ => panic!("unknown operator"),
            },
            calc::Expr::Literal(n) => n,
        })
    }
}
```

### Step 2: Create Parser and Push Tokens

```rust
use gazelle::Precedence;

fn parse(input: &str) -> Result<f64, String> {
    let mut parser = calc::Parser::<Evaluator>::new();
    let mut actions = Evaluator;

    // Your lexer loop
    for token in lex(input) {
        let terminal = match token {
            Token::Num(n) => calc::Terminal::Num(n),
            Token::Plus => calc::Terminal::Op('+', Precedence::Left(1)),
            Token::Star => calc::Terminal::Op('*', Precedence::Left(2)),
            Token::LParen => calc::Terminal::Lparen,
            Token::RParen => calc::Terminal::Rparen,
        };

        parser.push(terminal, &mut actions)
            .map_err(|e| parser.format_error(&e))?;
    }

    parser.finish(&mut actions)
        .map_err(|(p, e)| p.format_error(&e))
}
```

### Step 3: Handle Errors

```rust
// Push errors - parser is still available for format_error
match parser.push(terminal, &mut actions) {
    Ok(()) => { /* continue */ }
    Err(e) => {
        let msg = parser.format_error(&e);
        // msg contains: "unexpected 'X', expected: A, B, C"
        //               "  after: tokens parsed so far"
        //               "  in rule: context"
        return Err(msg);
    }
}

// Finish errors - parser returned in the error tuple
match parser.finish(&mut actions) {
    Ok(result) => { /* use result */ }
    Err((parser, e)) => {
        let msg = parser.format_error(&e);
        return Err(msg);
    }
}
```

---

## Advanced Features

### Precedence Terminals

For expression grammars, `prec` terminals carry precedence at runtime instead of encoding it in grammar structure:

```rust
terminals {
    prec OP: _,
}

expr = expr OP expr => binop | atom => atom;
```

When lexing, attach precedence to each operator:

```rust
'+' => calc::Terminal::Op('+', Precedence::Left(1)),   // Lower precedence
'*' => calc::Terminal::Op('*', Precedence::Left(2)),   // Higher precedence
'^' => calc::Terminal::Op('^', Precedence::Right(3)),  // Right-associative
```

**Precedence values:**
- `Precedence::Left(n)` - left-associative with level n
- `Precedence::Right(n)` - right-associative with level n
- Higher n = tighter binding

This enables user-defined operators at runtime!

### Token Range Tracking (Spans)

Implement `set_token_range` to track source positions:

```rust
impl calc::Types for SpanTracker {
    type Error = ParseError;
    type Op = char;
    type Num = f64;
    type Expr = Expr;

    fn set_token_range(&mut self, start: usize, end: usize) {
        // Called before each reduction with [start, end) token indices
        self.current_span = self.token_spans[start].start..self.token_spans[end-1].end;
    }
}

impl Reducer<calc::Expr<Self>> for SpanTracker {
    fn reduce(&mut self, node: calc::Expr<Self>) -> Result<Expr, ParseError> {
        Ok(match node {
            calc::Expr::Binop(left, op, right) => Expr {
                kind: ExprKind::BinOp(Box::new(left), op, Box::new(right)),
                span: self.current_span.clone(),
            },
            // ...
        })
    }
}
```

### Lexer Feedback

For languages like C where lexing depends on parse state (typedef disambiguation):

```rust
struct CActions {
    typedefs: HashSet<String>,
}

impl Reducer<c11::DeclTypedef<Self>> for CActions {
    fn reduce(&mut self, node: c11::DeclTypedef<Self>) -> Result<...> {
        // Extract name, register it
        self.typedefs.insert(name.clone());
        // ...
    }
}

// In your parse loop:
loop {
    // Lexer sees current typedef set
    let token = lexer.next(&actions.typedefs)?;
    parser.push(token, &mut actions)?;
}
```

The push-based architecture makes this natural - you control the loop.

### Multiple Implementations

The same grammar can have multiple action implementations:

```rust
// Evaluator
impl calc::Types for Evaluator { type Expr = f64; /* ... */ }
impl Reducer<calc::Expr<Self>> for Evaluator {
    fn reduce(&mut self, node: calc::Expr<Self>) -> Result<f64, ParseError> { /* evaluate */ }
}

// AST Builder — set type Expr = calc::Expr<Self> for identity (CST mode)
// The blanket Reducer handles it automatically — no impl needed!
impl calc::Types for CstBuilder { type Expr = calc::Expr<Self>; /* ... */ }

// Boxing — set type Expr = Box<calc::Expr<Self>> for auto-boxing
impl calc::Types for BoxedBuilder { type Expr = Box<calc::Expr<Self>>; /* ... */ }

// Pretty Printer
impl calc::Types for Printer { type Expr = String; /* ... */ }
impl Reducer<calc::Expr<Self>> for Printer {
    fn reduce(&mut self, node: calc::Expr<Self>) -> Result<String, ParseError> { /* format */ }
}
```

### Runtime Grammar API

For dynamic grammars, use the library API directly:

```rust
use gazelle::{parse_grammar, GrammarBuilder};
use gazelle::table::CompiledTable;
use gazelle::runtime::{CstParser, Token};

// Parse from string
let grammar = parse_grammar(r#"
    start expr;
    terminals { NUM, PLUS }
    expr = expr PLUS expr => add | NUM => num;
"#)?;

// Or build programmatically
let mut gb = GrammarBuilder::new();
let num = gb.t("NUM");
let plus = gb.t("PLUS");
let expr = gb.nt("expr");
gb.rule(expr, vec![expr, plus, expr]);
gb.rule(expr, vec![num]);
let grammar = gb.build();

// Compile and parse
let compiled = CompiledTable::build(&grammar);
let mut parser = CstParser::new(compiled.table());

let num_id = compiled.symbol_id("NUM").unwrap();
let plus_id = compiled.symbol_id("PLUS").unwrap();

parser.push(Token::new(num_id))?;     // NUM
parser.push(Token::new(plus_id))?;    // PLUS
parser.push(Token::new(num_id))?;     // NUM

let tree = parser.finish().map_err(|(p, e)| p.format_error(&e, &compiled))?;
// tree is a Cst: Leaf nodes carry SymbolId + token index,
// Node carry rule index + children
```

`CstParser` mirrors the generated parser's `push`/`finish` pattern. `Cst::Leaf` includes a token index so you can map back to your own token data (values, source positions, etc.).

For building a custom AST instead of a CST, use `Parser` directly with `maybe_reduce`/`shift`:

```rust
use gazelle::runtime::{Parser, Token};

let compiled = CompiledTable::build(&grammar);
let mut parser = Parser::new(compiled.table());
let mut stack: Vec<MyAst> = Vec::new();
let mut iter = tokens.into_iter();

loop {
    let token = iter.next();
    // Reduce while possible (rule 0 = accept)
    while let Some((rule, len, _)) = parser.maybe_reduce(token)? {
        if rule == 0 { return stack.pop().unwrap(); }
        let children: Vec<MyAst> = stack.drain(stack.len() - len..).collect();
        stack.push(build_ast_node(rule, children));
    }
    let Some(token) = token else { unreachable!() };
    stack.push(MyAst::Leaf(token));
    parser.shift(token);
}
```

---

## Error Messages

### Parse Errors

```
unexpected 'STAR', expected: NUM, LPAREN
  after: expr PLUS
  in expr: expr • OP expr
```

### Conflict Errors

```
Shift/reduce conflict on 'IDENT':
  - Shift (continue parsing)
  - Reduce by: __ident_dot_star -> (empty)

Parser state when seeing 'IDENT':
  arg -> • IDENT EQ path  [shift]
  arg -> • path
  __ident_dot_star -> •  [reduce on IDENT]
```

Use `expect N rr;` / `expect N sr;` to acknowledge known conflicts.

# Gazelle Reference

Complete reference for the Gazelle parser generator.

## Table of Contents

- [Grammar Syntax](#grammar-syntax)
- [The grammar! Macro](#the-grammar-macro)
- [Generated Types](#generated-types)
- [Using the Parser](#using-the-parser)
- [Advanced Features](#advanced-features)

---

## Grammar Syntax

Gazelle grammars can be written in `.gzl` files or inline with the `grammar!` macro.

### Basic Structure

```
grammar Name {
    start rule_name;

    terminals {
        // terminal declarations
    }

    // rule definitions
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

Rules define the grammar's structure:

```
// Basic rule
expr = expr PLUS term;

// Multiple alternatives (separated by |)
expr = expr PLUS term
     | expr MINUS term
     | term;

// With result type annotation
expr: Expression = expr PLUS term | term;

// With action names
expr: Expr = expr PLUS term @add
           | expr MINUS term @sub
           | term @passthrough;
```

### Actions, Passthroughs, and Ignored Symbols

**Named actions** (`@name`) generate trait methods:

```
expr: Expr = expr PLUS term @add;
// Generates: fn add(&mut self, v0: Expr, v1: Term) -> Expr;
```

**Ignored symbols in actions** - untyped symbols (no `: Type`) are not passed to actions:

```
expr: Expr = expr PLUS term @add;
//               ^^^^
// PLUS has no type, so it's omitted from the parameter list
// Only the two typed symbols (expr, term) become v0 and v1
```

**Untyped rules** - if the non-terminal has no type, all RHS values are discarded. No action needed:

```
// statement has no type annotation
// The values of expr and SEMI are simply ignored
statement = expr SEMI;

// vs typed version that needs to produce a value
statement: Stmt = expr SEMI @make_stmt;
```

**Passthrough** - when a rule has exactly one typed symbol and no action, its value flows through automatically:

```
expr: Expr = LPAREN expr RPAREN;  // Inner expr value becomes outer expr
           | term @wrap_term;      // Explicit action needed here

// LPAREN and RPAREN are untyped (ignored)
// expr is the only typed symbol, so it passes through
// No trait method generated for this alternative
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

### Expect Declarations

Declare expected conflicts to suppress errors:

```
grammar C11 {
    expect 3 rr;  // Expect 3 reduce/reduce conflicts
    expect 1 sr;  // Expect 1 shift/reduce conflict

    // ... rest of grammar
}
```

Use this for grammars with known ambiguities (like C's typedef or dangling else).

---

## The grammar! Macro

The `grammar!` macro generates a type-safe parser at compile time.

### Basic Usage

```rust
use gazelle_macros::grammar;
use gazelle::Precedence;

grammar! {
    grammar Calc {
        start expr;

        terminals {
            NUM: Num,
            LPAREN, RPAREN,
            prec OP: Op,
        }

        expr: Expr = expr OP expr @binop
                   | NUM @literal
                   | LPAREN expr RPAREN;
    }
}
```

### Visibility

Add `pub` before `grammar` for public visibility:

```rust
grammar! {
    pub grammar MyParser {
        // ...
    }
}
```

Or with restricted visibility:

```rust
grammar! {
    pub(crate) grammar MyParser {
        // ...
    }
}
```

---

## Generated Types

The macro generates several types based on your grammar name (e.g., `Calc`):

### CalcActions Trait

Defines the semantic actions:

```rust
pub trait CalcActions {
    // Associated types for each payload type
    type Num;
    type Op;
    type Expr;

    // Optional: token range callback for span tracking
    fn set_token_range(&mut self, start: usize, end: usize) {}

    // Action methods from @name annotations
    fn binop(&mut self, v0: Self::Expr, v1: Self::Op, v2: Self::Expr) -> Self::Expr;
    fn literal(&mut self, v0: Self::Num) -> Self::Expr;
}
```

### CalcTerminal Enum

Represents input tokens:

```rust
pub enum CalcTerminal<A: CalcActions> {
    NUM(A::Num),
    LPAREN,
    RPAREN,
    OP(A::Op, Precedence),  // prec terminals include Precedence
}
```

### CalcParser Struct

The parser itself:

```rust
pub struct CalcParser<A: CalcActions> {
    // ...
}

impl<A: CalcActions> CalcParser<A> {
    pub fn new() -> Self;
    pub fn push(&mut self, terminal: CalcTerminal<A>, actions: &mut A) -> Result<(), ParseError>;
    pub fn finish(self, actions: &mut A) -> Result<A::Expr, ParseError>;
    pub fn state(&self) -> usize;
    pub fn format_error(&self, err: &ParseError) -> String;
}
```

---

## Using the Parser

### Step 1: Implement the Actions Trait

```rust
struct Evaluator;

impl CalcActions for Evaluator {
    type Num = f64;
    type Op = char;
    type Expr = f64;

    fn binop(&mut self, left: f64, op: char, right: f64) -> f64 {
        match op {
            '+' => left + right,
            '-' => left - right,
            '*' => left * right,
            '/' => left / right,
            _ => panic!("unknown operator"),
        }
    }

    fn literal(&mut self, n: f64) -> f64 {
        n
    }
}
```

### Step 2: Create Parser and Push Tokens

```rust
use gazelle::Precedence;

fn parse(input: &str) -> Result<f64, String> {
    let mut parser = CalcParser::<Evaluator>::new();
    let mut actions = Evaluator;

    // Your lexer loop
    for token in lex(input) {
        let terminal = match token {
            Token::Num(n) => CalcTerminal::NUM(n),
            Token::Plus => CalcTerminal::OP('+', Precedence::Left(1)),
            Token::Star => CalcTerminal::OP('*', Precedence::Left(2)),
            Token::LParen => CalcTerminal::LPAREN,
            Token::RParen => CalcTerminal::RPAREN,
        };

        parser.push(terminal, &mut actions)
            .map_err(|e| parser.format_error(&e))?;
    }

    parser.finish(&mut actions)
        .map_err(|e| parser.format_error(&e))
}
```

### Step 3: Handle Errors

```rust
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
```

---

## Advanced Features

### Precedence Terminals

For expression grammars, `prec` terminals carry precedence at runtime instead of encoding it in grammar structure:

```rust
terminals {
    prec OP: Operator,
}

expr: Expr = expr OP expr @binop | atom;
```

When lexing, attach precedence to each operator:

```rust
'+' => CalcTerminal::OP('+', Precedence::Left(1)),   // Lower precedence
'*' => CalcTerminal::OP('*', Precedence::Left(2)),   // Higher precedence
'^' => CalcTerminal::OP('^', Precedence::Right(3)),  // Right-associative
```

**Precedence values:**
- `Precedence::Left(n)` - left-associative with level n
- `Precedence::Right(n)` - right-associative with level n
- Higher n = tighter binding

This enables user-defined operators at runtime!

### Token Range Tracking (Spans)

Implement `set_token_range` to track source positions:

```rust
impl CalcActions for SpanTracker {
    fn set_token_range(&mut self, start: usize, end: usize) {
        // Called before each reduction with [start, end) token indices
        self.current_span = self.token_spans[start].start..self.token_spans[end-1].end;
    }

    fn binop(&mut self, left: Expr, op: char, right: Expr) -> Expr {
        Expr {
            kind: ExprKind::BinOp(Box::new(left), op, Box::new(right)),
            span: self.current_span.clone(),
        }
    }
    // ...
}
```

### Lexer Feedback

For languages like C where lexing depends on parse state (typedef disambiguation):

```rust
struct CActions {
    typedefs: HashSet<String>,
}

impl C11Actions for CActions {
    fn typedef_declaration(&mut self, name: String, ...) -> Declaration {
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
impl CalcActions for Evaluator {
    type Expr = f64;
    fn binop(&mut self, l: f64, op: char, r: f64) -> f64 { /* evaluate */ }
}

// AST Builder
impl CalcActions for AstBuilder {
    type Expr = AstNode;
    fn binop(&mut self, l: AstNode, op: char, r: AstNode) -> AstNode { /* build tree */ }
}

// Pretty Printer
impl CalcActions for Printer {
    type Expr = String;
    fn binop(&mut self, l: String, op: char, r: String) -> String { /* format */ }
}
```

### Runtime Grammar API

For dynamic grammars, use the library API directly:

```rust
use gazelle::{parse_grammar, GrammarBuilder};
use gazelle::table::CompiledTable;
use gazelle::runtime::{Parser, Token};

// Parse from string
let grammar = parse_grammar(r#"
    grammar Expr {
        start expr;
        terminals { NUM, PLUS }
        expr = expr PLUS expr | NUM;
    }
"#)?;

// Or build programmatically
let mut gb = GrammarBuilder::new();
let num = gb.t("NUM");
let plus = gb.t("PLUS");
let expr = gb.nt("expr");
gb.rule(expr, vec![expr, plus, expr]);
gb.rule(expr, vec![num]);
let grammar = gb.build();

// Compile and use
let compiled = CompiledTable::build(&grammar);
let mut parser = Parser::new(compiled.table());

// Push tokens with maybe_reduce/shift
let num_id = compiled.symbol_id("NUM").unwrap();
let token = Token::new(num_id);

while let Some((rule, len, start_idx)) = parser.maybe_reduce(Some(&token))? {
    // Handle reduction
}
parser.shift(&token);
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

# Trait-Based Parser API Design

## Overview

Replace the current callback-based reduction API with a trait-based approach. Users implement a trait with methods for each named reduction, and the parser calls these methods directly.

All types (terminals and non-terminals) are associated types on the trait. The parser is fully generic over the Actions trait.

## Grammar Syntax

### Symbol Type Annotations

Symbols can have optional type names. These become associated types on the trait - they are NOT concrete Rust types in the grammar.

```
terminals {
    NUM: Num,        // typed - becomes `type Num;` on the trait
    LPAREN,          // untyped - no associated type, invisible to trait methods
    RPAREN,          // untyped - invisible
    SEMI,            // untyped - invisible
}

prec_terminals {
    OP: Op,          // typed - becomes `type Op;` on the trait
}
```

The type name (`Num`, `Op`) is just an identifier for the associated type. The concrete Rust type is chosen by whoever implements the trait. This allows the same grammar to be used with different concrete types (e.g., `f64` for an interpreter, `TokenSpan` for an AST builder).

### Reduction Naming with `@name`

Each alternative can be named with `@name` suffix. Named reductions become trait methods.

```
expr: Expr = expr OP expr @binop
           | NUM @literal
           | LPAREN expr RPAREN @grouped;
```

### Unnamed Reductions

Reductions without names are handled automatically:

1. **Single typed symbol**: Passthrough - the value is returned directly
   ```
   expr: Expr = LPAREN expr RPAREN;  // returns the inner expr
   ```

2. **Multiple typed symbols**: Compile error - ambiguous what to return

3. **No typed symbols / unit result type**: Structural only - handled internally
   ```
   stmts = stmts SEMI stmt | stmt | ;  // pure structure, no user code
   ```

## Generated Trait

For this grammar:

```
grammar Calc {
    start expr;

    terminals {
        NUM: f64,
        LPAREN,
        RPAREN,
    }
    prec_terminals {
        OP: char,
    }

    expr: Expr = expr OP expr @binop
               | NUM @literal
               | LPAREN expr RPAREN;  // passthrough, no method

    stmts = stmts SEMI stmt | stmt | ;  // structural, no methods
}
```

Generate:

```rust
trait CalcActions {
    // Terminal types (associated types, not concrete)
    type Num;
    type Op;

    // Non-terminal result type
    type Expr;

    fn binop(&mut self, _: Self::Expr, _: Self::Op, _: Self::Expr) -> Self::Expr;
    fn literal(&mut self, _: Self::Num) -> Self::Expr;
}
```

Notes:
- One associated type per typed terminal
- One associated type per typed non-terminal
- One method per `@named` reduction
- Method parameters are the typed symbols in order (untyped symbols skipped)
- All typed symbols use associated types (both terminals and non-terminals)
- Return type matches the non-terminal's type
- `&mut self` for flexibility (e.g., updating symbol tables, lexer state)

## Generated Parser

```rust
struct CalcParser {
    state_stack: Vec<usize>,
    value_stack: Vec<CalcValue>,  // internal union type
}

impl CalcParser {
    /// Create a new parser.
    fn new() -> Self;

    /// Push a token, performing any reductions.
    fn push<A: CalcActions>(
        &mut self,
        token: CalcTerminal,
        actions: &mut A,
    ) -> Result<(), CalcError>;

    /// Finish parsing and return the result.
    fn finish<A: CalcActions>(
        self,
        actions: &mut A,
    ) -> Result<A::Expr, CalcError>;
}
```

The parser internally:
1. On `push`: reduce as needed (calling trait methods), then shift
2. On `finish`: reduce until accept, return final value

## User Implementation

### Example: Interpreter

```rust
struct Interpreter {
    vars: HashMap<String, f64>,
}

impl CalcActions for Interpreter {
    // Terminal types
    type Num = f64;
    type Op = char;

    // Non-terminal result type
    type Expr = f64;  // evaluate directly to numbers

    fn binop(&mut self, left: Self::Expr, op: Self::Op, right: Self::Expr) -> Self::Expr {
        match op {
            '+' => left + right,
            '-' => left - right,
            '*' => left * right,
            '/' => left / right,
            _ => panic!("unknown operator"),
        }
    }

    fn literal(&mut self, n: Self::Num) -> Self::Expr {
        n
    }
}
```

### Example: AST Builder

```rust
enum Expr {
    Num(f64),
    BinOp(Box<Expr>, char, Box<Expr>),
}

struct AstBuilder;

impl CalcActions for AstBuilder {
    // Terminal types
    type Num = f64;
    type Op = char;

    // Non-terminal result type
    type Expr = Box<Expr>;

    fn binop(&mut self, left: Self::Expr, op: Self::Op, right: Self::Expr) -> Self::Expr {
        Box::new(Expr::BinOp(left, op, right))
    }

    fn literal(&mut self, n: Self::Num) -> Self::Expr {
        Box::new(Expr::Num(n))
    }
}
```

### Usage

```rust
let mut parser = CalcParser::<Interpreter>::new();
let mut actions = Interpreter::new();

for token in lexer {
    // token is CalcTerminal<Interpreter>
    parser.push(token, &mut actions)?;
}

let result = parser.finish(&mut actions)?;
println!("Result: {}", result);
```

## Benefits

1. **Type safety**: Rust's type system enforces correct trait implementations
2. **Flexibility**: Associated types allow any representation (values, AST, etc.)
3. **Stateful**: `&mut self` enables symbol tables, scopes, lexer feedback
4. **Multiple implementations**: Same grammar, different actions (interpret, compile, pretty-print)
5. **Simple API**: Just `push` tokens, parser handles reduction loop internally
6. **Clean user code**: No manual reduction loop, no callback closures

## Migration from Current API

Current:
```rust
fn reduce(r: CalcReduction) -> CalcResult {
    match r {
        CalcReduction::ExprExprOpExpr(c, l, op, r) => c(Expr::BinOp(l, op, r)),
        CalcReduction::ExprNum(c, n) => c(Expr::Num(n)),
    }
}

loop {
    while let Some(r) = parser.maybe_reduce(&tok) {
        parser.reduce(reduce(r));
    }
    // ...
}
```

New:
```rust
impl CalcActions for AstBuilder {
    type Expr = Expr;
    fn binop(&mut self, l: Expr, op: char, r: Expr) -> Expr { Expr::BinOp(l, op, r) }
    fn literal(&mut self, n: f64) -> Expr { Expr::Num(n) }
}

for token in lexer {
    parser.push(token, &mut actions)?;
}
```

## Design Decisions

### Naming Conflicts

Duplicate `@name` across rules is an error. Alternatively, let Rust error on duplicate method names - the generated trait simply won't compile.

### Error Handling

`push` returns `Result<(), ParseError>`. No error recovery in the trait. User decides whether to abort or attempt recovery externally.

### Start Symbol

Explicit in grammar:

```
grammar Calc {
    start expr;

    terminals { ... }
    expr: Expr = ...;
}
```

The `finish` method returns the start symbol's associated type.

### Value Stack

Unchanged from current design - uses a union internally. The trait abstraction hides this from users.

## Generated Terminal Enum

Terminals become a generic enum parameterized by the Actions trait. Typed terminals use associated types for their payloads. Precedence terminals carry both value (as associated type) and precedence level.

```rust
enum CalcTerminal<A: CalcActions> {
    // From `terminals { NUM: Num, LPAREN, RPAREN }`
    Num(A::Num),
    LParen,
    RParen,

    // From `prec_terminals { OP: Op }`
    Op(A::Op, Precedence),

    // Hidden marker for unused type parameter
    #[doc(hidden)]
    _Phantom(std::marker::PhantomData<A>),
}

#[derive(Clone, Copy)]
struct Precedence {
    level: u8,
    assoc: Assoc,
}

#[derive(Clone, Copy)]
enum Assoc { Left, Right }
```

Usage:
```rust
// Lexer produces tokens - type parameter inferred from context
let token: CalcTerminal<Evaluator> = match ch {
    '+' | '-' => CalcTerminal::Op(ch, Precedence::left(1)),
    '*' | '/' => CalcTerminal::Op(ch, Precedence::left(2)),
    '^' => CalcTerminal::Op(ch, Precedence::right(3)),
    // ...
};
parser.push(token, &mut actions)?;
```

## Validation Strategy

Let the Rust compiler catch errors:
- Duplicate `@name` → duplicate method error
- Passthrough with wrong number of typed symbols → type mismatch error
- Missing trait method implementation → trait impl error

This keeps the grammar compiler simple. Verification can be added later for better error messages.

## Implementation Status

**Complete.** The trait-based API is fully implemented:

- `start <non_terminal>;` syntax supported in grammar
- `<Grammar>Actions` trait generated with associated types and methods
- `<Grammar>Parser` struct with `push()` and `finish()` API
- Value stack uses union internally (unchanged)
- Passthrough detection for single typed symbols (same type, not necessarily same non-terminal)
- Named reductions (`@name`) become trait methods
- Structural reductions handled internally
- Terminal enum with payload and precedence support

The meta grammar bootstrap (`meta.gzl`) has been converted to use this API, demonstrating that it works for real parsers.

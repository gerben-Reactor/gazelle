# Resume State

Current development state for continuing work.

## Architecture

```
gazelle/
├── meta.gzl                 # Grammar definition (with @name annotations)
├── src/
│   ├── meta.rs              # Grammar parsing, lexer, AST types
│   ├── meta_generated.rs    # Generated parser (self-hosted, trait-based)
│   ├── grammar.rs           # Grammar IR
│   ├── lr.rs                # LR automaton construction
│   ├── table.rs             # Parse table construction
│   ├── runtime.rs           # Parser runtime
│   └── codegen/
│       ├── mod.rs           # Code generation entry point
│       ├── parser.rs        # Trait + parser generation
│       ├── terminal.rs      # Terminal enum generation
│       ├── table.rs         # Table data generation
│       └── reduction.rs     # Reduction analysis (passthrough detection)
├── gazelle-macros/
│   └── src/lib.rs           # grammar! proc macro
├── examples/
│   ├── calculator.rs        # Simple calculator with user-defined operators
│   ├── c11_calculator.rs    # Full C11 expression syntax
│   └── c11/                 # Complete C11 parser
└── tests/
    └── error_messages.rs    # Error formatting tests
```

## Regenerating meta_generated.rs

```bash
cargo build --release
./target/release/gazelle --rust meta.gzl > src/meta_generated.rs
```

## Generated API Overview

For a grammar like:
```
grammar Calc {
    start expr;
    terminals { NUM: Num, LPAREN, RPAREN }
    prec_terminals { OP: Op }

    expr: Expr = expr OP expr @binop | NUM @literal | LPAREN expr RPAREN;
}
```

Generated trait:
```rust
trait CalcActions {
    type Num;   // Terminal type
    type Op;    // Terminal type
    type Expr;  // Non-terminal result type

    fn binop(&mut self, left: Self::Expr, op: Self::Op, right: Self::Expr) -> Self::Expr;
    fn literal(&mut self, n: Self::Num) -> Self::Expr;
    // No method for LPAREN expr RPAREN - it's a passthrough (single typed symbol with matching type)
}
```

Generated terminal enum:
```rust
enum CalcTerminal<A: CalcActions> {
    Num(A::Num),
    Op(A::Op, Precedence),
    Lparen,
    Rparen,
}
```

Usage:
```rust
let mut parser = CalcParser::<Evaluator>::new();
for tok in lexer {
    parser.push(tok, &mut actions)?;
}
let result = parser.finish(&mut actions)?;
```

## Key Features

- **Runtime precedence**: `prec` terminals carry precedence values, resolved at parse time
- **Passthrough detection**: Single typed symbol matching result type needs no action method
- **Type annotations determine associated types**: `expr: Expr` creates `type Expr;`, not `type Expr_` per rule
- **Push-based parsing**: You control the loop, enabling lexer feedback
- **Lexer feedback**: Parser state available to lexer (e.g., C typedef disambiguation)

## Next Steps

- [ ] Better error messages for grammar validation
- [ ] Counterexample generation for conflicts

# Resume State

Current development state for continuing work.

## Current Status: Flat AST Structure

The public AST types now use a flat structure with `Option` and `Vec`:

```rust
pub struct GrammarDef {
    pub name: String,
    pub start: Option<String>,
    pub terminals: TerminalsBlock,
    pub prec_terminals: PrecTerminalsBlock,
    pub rules: Vec<Rule>,
}
```

The intermediate `Sections` and `Section` types are hidden implementation details used only during parsing.

## Architecture

```
gazelle-core/
├── meta.gzl                 # Grammar definition (with @name annotations)
├── src/
│   ├── meta_bootstrap.rs    # Typed AST + AstBuilder (implements MetaActions)
│   ├── meta_generated.rs    # Generated parser (trait-based)
│   └── codegen/
│       ├── mod.rs           # CodegenContext with start_symbol
│       ├── parser.rs        # Trait + parser generation
│       ├── terminal.rs      # Terminal enum generation
│       └── reduction.rs     # Reduction analysis

src/
├── lexer.rs                 # Lexer (stays in main crate)
├── meta.rs                  # lex_grammar() with "start" keyword
└── main.rs                  # CLI with start_symbol extraction

gazelle-macros/
└── src/lib.rs               # Proc macro with start_symbol extraction
```

## Regenerating meta_generated.rs

```bash
cargo build --release
./target/release/gazelle --rust gazelle-core/meta.gzl > gazelle-core/src/meta_generated.rs
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
}
```

Generated terminal enum:
```rust
enum CalcTerminal<A: CalcActions> {
    Num(A::Num),
    Op(A::Op, Precedence),
    Lparen,
    Rparen,
    #[doc(hidden)]
    _Phantom(std::marker::PhantomData<A>),
}
```

Usage:
```rust
let mut parser = CalcParser::<Evaluator>::new();
for tok in lexer {
    // tok is CalcTerminal<Evaluator>
    parser.push(tok, &mut actions)?;
}
let result = parser.finish(&mut actions)?;
```

## Next Steps

- [ ] Add TokenStream lexer with parser feedback
- [ ] Better error messages for grammar validation
- [ ] Consider adding Clone bound to terminal associated types for ergonomics

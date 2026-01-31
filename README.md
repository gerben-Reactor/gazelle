# Gazelle

An LR parser generator for Rust with runtime operator precedence and natural lexer feedback.

## What Makes Gazelle Different

### 1. Runtime Operator Precedence

Traditional grammars encode precedence through structure - one rule per level:

```
expr: add_expr;
add_expr: add_expr '+' mul_expr | mul_expr;
mul_expr: mul_expr '*' unary_expr | unary_expr;
// ... 10 more levels for a full language
```

Gazelle's `prec` terminals carry precedence at runtime:

```rust
grammar! {
    grammar Calc {
        terminals {
            NUM: Num,
            prec OP: Op,  // precedence attached to each token
        }
        expr: Expr = expr OP expr @binop | NUM @literal;
    }
}
```

One rule. The lexer provides precedence per token:

```rust
'+' => CalcTerminal::Op('+', Precedence::Left(1)),
'*' => CalcTerminal::Op('*', Precedence::Left(2)),
```

This enables **user-defined operators** at runtime - see `examples/calculator.rs`.

### 2. Natural Lexer Feedback

The infamous C typedef problem: is `T * x` a multiplication or pointer declaration? The lexer needs parser state to decide.

Traditional parsers hide the parse loop, requiring globals or hacks. Gazelle uses a push-based API - you drive the loop:

```rust
loop {
    let token = lexer.next(&actions.ctx)?;  // lexer sees current state
    parser.push(token, &mut actions)?;       // actions update state
    // next iteration: lexer sees updated state
}
```

No magic. The lexer and parser share state through `actions`, and you control when each runs. See `examples/c11/` for a complete C11 parser using this for typedef disambiguation.

### 3. Clean Separation of Grammar and Actions

The grammar is pure grammar:

```rust
grammar! {
    grammar Calc {
        terminals { NUM: Num, prec OP: Op, LPAREN, RPAREN }

        expr: Expr = expr OP expr @binop
                   | NUM @literal
                   | LPAREN expr RPAREN;  // passthrough - inner expr flows through
    }
}
```

Actions are a normal Rust trait implementation:

```rust
impl CalcActions for Evaluator {
    type Num = f64;
    type Op = char;
    type Expr = f64;

    fn binop(&mut self, left: f64, op: char, right: f64) -> f64 {
        match op {
            '+' => left + right,
            '*' => left * right,
            _ => panic!("unknown op"),
        }
    }

    fn literal(&mut self, n: f64) -> f64 { n }
    // No paren method needed - passthrough!
}
```

This gives you:
- Full IDE support in action code (autocomplete, type hints, go-to-definition)
- Compile errors point to your code, not generated code
- Multiple implementations (interpreter, AST builder, pretty-printer)
- Grammars reusable across different backends

### 4. Parser Generator as a Library

Most parser generators are build tools. Gazelle exposes table construction as a library:

```rust
use gazelle::{parse_grammar, ErrorContext};
use gazelle::grammar::GrammarBuilder;
use gazelle::table::CompiledTable;
use gazelle::runtime::{Parser, Token};

// Option 1: Parse grammar from string
let grammar = parse_grammar(r#"
    grammar Expr {
        start expr;
        terminals { NUM, PLUS, STAR }
        expr = expr PLUS term | term;
        term = term STAR factor | factor;
        factor = NUM;
    }
"#).unwrap();

// Option 2: Build programmatically
let mut gb = GrammarBuilder::new();
let num = gb.t("NUM");
let plus = gb.t("PLUS");
let expr = gb.nt("expr");
gb.rule(expr, vec![expr, plus, expr]);
gb.rule(expr, vec![num]);
let grammar = gb.build();

// Build tables and parse
let compiled = CompiledTable::build(&grammar);
let mut parser = Parser::new(compiled.table());

let num_id = compiled.symbol_id("NUM").unwrap();
// ... push tokens with parser.maybe_reduce() and parser.shift()
```

Enables grammar analyzers, conflict debuggers, or parsers for dynamic grammars. See `examples/runtime_grammar.rs` for complete examples.

## Examples

### Calculator with User-Defined Operators

```
$ cargo run --example calculator

> operator ^ pow right 3;
defined: ^ = pow right 3

> 2 ^ 3 ^ 2;
512                        // right-assoc: 2^(3^2) = 2^9

> x = 2 * 3 ^ 2;
x = 18                     // ^ binds tighter than *
```

### C11 Parser

A complete C11 parser demonstrating:
- Lexer feedback for typedef disambiguation (Jourdan's approach)
- Dynamic precedence collapsing 10+ expression levels into one rule
- Full C11 test suite (41 tests)

```
$ cargo test --example c11
```

The expression grammar:
```rust
// Traditional C grammar: 10+ cascading rules
// Gazelle: one rule with prec terminals
assignment_expression = cast_expression
                      | assignment_expression BINOP assignment_expression
                      | assignment_expression STAR assignment_expression
                      | assignment_expression QUESTION expression COLON assignment_expression;
```

## Usage

```rust
use gazelle::Precedence;
use gazelle_macros::grammar;

grammar! {
    grammar MyParser {
        start expr;
        terminals {
            NUM: Num,  // terminal with payload
            LPAREN, RPAREN,
            prec OP: Op,  // precedence terminal with payload
        }

        expr: Expr = expr OP expr @binop
                   | NUM @num
                   | LPAREN expr RPAREN;  // passthrough
    }
}

struct Eval;

impl MyParserActions for Eval {
    type Num = i32;
    type Op = char;
    type Expr = i32;

    fn num(&mut self, n: i32) -> i32 { n }
    fn binop(&mut self, l: i32, op: char, r: i32) -> i32 {
        match op { '+' => l + r, '*' => l * r, _ => 0 }
    }
}

fn main() {
    let mut parser = MyParserParser::<Eval>::new();
    let mut actions = Eval;

    // Push tokens (you control the loop)
    parser.push(MyParserTerminal::Num(2), &mut actions).unwrap();
    parser.push(MyParserTerminal::Op('+', Precedence::Left(1)), &mut actions).unwrap();
    parser.push(MyParserTerminal::Num(3), &mut actions).unwrap();

    let result = parser.finish(&mut actions).unwrap();
    assert_eq!(result, 5);
}
```

## When to Use Gazelle

**Good fit:**
- Languages with user-definable operators or precedence
- C-family languages needing lexer feedback (typedef disambiguation)
- Complex expression grammars you want to simplify
- When you want full IDE support in semantic actions

**Consider alternatives for:**
- Simple formats (JSON, TOML) - nom or pest may be simpler
- Error recovery focus - chumsky or tree-sitter
- Maximum ecosystem maturity - lalrpop

## License

MIT

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
gazelle! {
    grammar Calc {
        terminals {
            NUM: _,
            prec OP: _,  // precedence attached to each token
        }
        expr = expr OP expr => binop | NUM => literal;
    }
}
```

One rule. The lexer provides precedence per token:

```rust
'+' => calc::Terminal::Op('+', Precedence::Left(1)),
'*' => calc::Terminal::Op('*', Precedence::Left(2)),
```

This enables **user-defined operators** at runtime - see `examples/c11_calculator.rs`.

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
gazelle! {
    grammar Calc {
        terminals { NUM: _, prec OP: _, LPAREN, RPAREN }

        expr = expr OP expr => binop
             | NUM => literal
             | LPAREN expr RPAREN => paren;
    }
}
```

Actions are split into a `Types` trait and per-node `Reducer` impls:

```rust
impl calc::Types for Evaluator {
    type Error = ParseError;
    type Num = f64;
    type Op = char;
    type Expr = f64;
}

impl gazelle::Reducer<calc::Expr<Self>> for Evaluator {
    fn reduce(&mut self, node: calc::Expr<Self>) -> Result<f64, ParseError> {
        Ok(match node {
            calc::Expr::Binop(left, op, right) => match op {
                '+' => left + right,
                '*' => left * right,
                _ => panic!("unknown op"),
            },
            calc::Expr::Literal(n) => n,
            calc::Expr::Paren(e) => e,
        })
    }
}
```

Reducer methods return `Result` - the error type is declared as `type Error: From<ParseError>` on the `Types` trait.

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
    start expr;
    terminals { NUM, PLUS, STAR }
    expr = expr PLUS term => add | term => term;
    term = term STAR factor => mul | factor => factor;
    factor = NUM => num;
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
$ cargo run --example c11_calculator

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
use gazelle::{ParseError, Precedence};
use gazelle_macros::gazelle;

gazelle! {
    grammar MyParser {
        start expr;
        terminals {
            NUM: _,        // terminal with payload
            LPAREN, RPAREN,
            prec OP: _,    // precedence terminal with payload
        }

        expr = expr OP expr => binop
             | NUM => num
             | LPAREN expr RPAREN => paren;
    }
}

struct Eval;

impl my_parser::Types for Eval {
    type Error = ParseError;
    type Num = i32;
    type Op = char;
    type Expr = i32;
}

impl gazelle::Reducer<my_parser::Expr<Self>> for Eval {
    fn reduce(&mut self, node: my_parser::Expr<Self>) -> Result<i32, ParseError> {
        Ok(match node {
            my_parser::Expr::Binop(l, op, r) => match op {
                '+' => l + r, '*' => l * r, _ => 0,
            },
            my_parser::Expr::Num(n) => n,
        })
    }
}

fn main() {
    let mut parser = my_parser::Parser::<Eval>::new();
    let mut actions = Eval;

    // Push tokens (you control the loop)
    parser.push(my_parser::Terminal::Num(2), &mut actions).unwrap();
    parser.push(my_parser::Terminal::Op('+', Precedence::Left(1)), &mut actions).unwrap();
    parser.push(my_parser::Terminal::Num(3), &mut actions).unwrap();

    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
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
- Maximum ecosystem maturity - lalrpop

## License

MIT

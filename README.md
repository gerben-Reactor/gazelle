# Gazelle

An LR parser generator for Rust with clean grammar separation and runtime operator precedence.

## Design Principles

### Clean Grammar, Separate Actions

Most parser generators mix grammar with semantic actions:

```yacc
expr: expr '+' expr { $$ = $1 + $3; }
    | expr '*' expr { $$ = $1 * $3; }
    ;
```

This interleaves *what* to parse with *what to do*, making grammars hard to read.

Gazelle keeps them separate. The grammar is pure grammar:

```rust
grammar! {
    grammar Calc {
        terminals {
            NUM: f64,
            IDENT: String,
            LPAREN, RPAREN, COMMA, SEMI,
        }

        prec_terminals {
            OP: char,
        }

        stmts: () = stmts SEMI stmt | stmt |;
        stmt: () = OPERATOR OP IDENT LEFT NUM
                 | OPERATOR OP IDENT RIGHT NUM
                 | expr;
        expr: Expr = expr OP expr
                   | NUM
                   | IDENT LPAREN expr COMMA expr RPAREN
                   | IDENT
                   | LPAREN expr RPAREN;
    }
}
```

Semantic actions are a normal Rust match on typed reductions:

```rust
match reduction {
    CalcReduction::StmtExpr(c, expr) => {
        eval(&expr);
        c(())
    }
    CalcReduction::ExprExprOpExpr(c, left, op, right) => {
        c(Expr::BinOp(Box::new(left), op, Box::new(right)))
    }
    // ...
}
```

### Runtime Operator Precedence

Traditional parser generators bake precedence into the grammar at compile time:

```yacc
%left '+' '-'
%left '*' '/'
```

This fixes the operator set. User-defined operators? Not possible.

Gazelle's `prec_terminals` let the lexer provide precedence per token:

```rust
prec_terminals {
    OP: char,  // each token carries its own precedence
}
```

The lexer returns operators with their precedence:

```rust
'+' => Some(CalcTerminal::Op('+', Precedence::left(1))),
'*' => Some(CalcTerminal::Op('*', Precedence::left(2))),
```

This enables **user-defined operators**. The calculator example supports:

```
operator ^ pow right 3;
2 ^ 3 ^ 2                   // 512 (right-assoc: 2^(3^2) = 2^9)
x = 2 * 3 ^ 2               // x = 18 (^ binds tighter than *)
```

The `operator` statement defines `^` as right-associative with precedence 3, calling the built-in `pow` function. The lexer is updated dynamically, and subsequent parsing uses the new precedence.

Unlike Haskell, which parses infix expressions flat then restructures them in a separate fixity resolution pass, Gazelle resolves precedence during parsing. The LR parser's shift/reduce decisions consult the token's precedence directly, building the correct tree in one pass.

### Push Architecture

Traditional parser generators use pull: the parser calls `yylex()` to get tokens. State sharing between lexer and parser requires globals or hacks (the infamous C typedef problem).

Gazelle uses push: you drive the loop.

```rust
let mut lexer = Lexer::new(input);
let mut parser = CalcParser::new();
let mut ops = HashMap::new();

loop {
    let tok = lexer.next(&ops);  // lexer sees current state

    while let Some(r) = parser.maybe_reduce(&tok) {
        // reductions can update state
        parser.reduce(handle_reduce(r, &mut vars, &mut ops));
    }

    if tok.is_none() { break; }
    parser.shift(tok.unwrap()).expect("parse error");
}
```

State flows naturally through your code:
- `lexer.next(&ops)` sees the current operator definitions
- Reductions can update `ops`
- The next `lexer.next(&ops)` sees the changes

No globals, no callbacks, no magic. Just Rust variables and control flow.

## Example

See `examples/calculator.rs` for a complete example demonstrating:
- Runtime operator precedence
- User-defined operators
- Assignment as a right-associative binary operator
- Function calls
- Clean grammar/action separation

```
$ cargo run --example calculator

Input:
  operator ^ pow right 3;
  2 ^ 3 ^ 2;
  x = 2 * 3 ^ 2;
  pow(2, 10);
  x + pow(x, 0.5)

defined: ^ = pow right 3
512
x = 18
1024
22.242640687119284
```

## Usage

```rust
use gazelle::grammar;

grammar! {
    grammar MyParser {
        terminals {
            // terminals with payload types
            NUM: i32,
            IDENT: String,
            // unit terminals (no payload)
            PLUS, MINUS, LPAREN, RPAREN,
        }

        // optional: terminals with runtime precedence
        prec_terminals {
            OP: MyOperator,
        }

        // rules: name: Type = alternatives;
        expr: Expr = expr OP expr | term;
        term: Term = NUM | LPAREN expr RPAREN;
    }
}
```

The macro generates:
- `MyParserTerminal` - enum of terminals
- `MyParserReduction` - enum of reductions with typed payloads
- `MyParserParser` - the LR parser

## License

MIT

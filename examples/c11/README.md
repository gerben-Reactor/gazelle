# C11 Parser POC for Gazelle

A proof-of-concept C11 parser demonstrating two key Gazelle innovations:

1. **Precedence terminals (`prec OP`)** - Collapse C's 13-level binary expression hierarchy into ONE rule
2. **Lexer feedback** - Jourdan's elegant typedef disambiguation via `NAME TYPE`/`NAME VARIABLE`

## Gazelle's Novel Parsing Extension: Precedence Terminals

Traditional C parsers require a cascade of ~15 rules for binary expressions:

```yacc
// Traditional approach - one rule per precedence level
multiplicative_expression: cast_expression
    | multiplicative_expression '*' cast_expression
    | multiplicative_expression '/' cast_expression;
additive_expression: multiplicative_expression
    | additive_expression '+' multiplicative_expression;
shift_expression: additive_expression
    | shift_expression '<<' additive_expression;
relational_expression: shift_expression ...
equality_expression: relational_expression ...
and_expression: equality_expression ...
xor_expression: and_expression ...
or_expression: xor_expression ...
logical_and_expression: or_expression ...
logical_or_expression: logical_and_expression ...
// ... continues for 13+ levels
```

**With Gazelle's `prec OP` terminal, this collapses to ONE rule:**

```rust
terminals {
    prec OP: Op,  // Precedence terminal - lexer provides (data, precedence)
}

// ALL binary operators in one rule!
binary_expr: Expr = binary_expr OP binary_expr @binop
                  | cast_expr;
```

The lexer returns each operator with its precedence:

```rust
fn op_terminal(&self, op: &str) -> C11Terminal<A> {
    let (data, prec) = match op {
        "*" | "/" | "%" => (BinOp::from(op), Precedence::left(13)),
        "+" | "-"       => (BinOp::from(op), Precedence::left(12)),
        "<<" | ">>"     => (BinOp::from(op), Precedence::left(11)),
        "<" | ">" | "<=" | ">=" => (BinOp::from(op), Precedence::left(10)),
        "==" | "!="     => (BinOp::from(op), Precedence::left(9)),
        "&"             => (BinOp::from(op), Precedence::left(8)),
        "^"             => (BinOp::from(op), Precedence::left(7)),
        "|"             => (BinOp::from(op), Precedence::left(6)),
        "&&"            => (BinOp::from(op), Precedence::left(5)),
        "||"            => (BinOp::from(op), Precedence::left(4)),
        // Assignment operators are right-associative
        "=" | "+=" | "-=" | ... => (BinOp::from(op), Precedence::right(2)),
        ","             => (BinOp::from(op), Precedence::left(1)),
        _ => ...
    };
    C11Terminal::Op(data, prec)
}
```

**Benefits:**
- ~15 grammar rules -> 1 rule
- No shift-reduce conflicts from precedence
- Precedence can be dynamic (like the calculator example with user-defined operators)

## The Typedef Problem & Solution

C has the classic "typedef ambiguity": `T * x;` could be a multiplication or a pointer declaration depending on whether `T` is a typedef name.

### Jourdan's Solution

1. **Two-token identifiers**: Lexer emits `NAME` followed by `TYPE` or `VARIABLE`
2. **Grammar disambiguates**: `typedef_name: NAME TYPE;` vs `var_name: NAME VARIABLE;`
3. **Empty productions for context**: `save_context: _;` triggers scope save/restore
4. **Parser steers lexer**: Actions update typedef table, lexer queries it

### Implementation

```rust
// Lexer feedback mechanism
pub struct C11Lexer<'a> {
    lexer: gazelle::lexer::Lexer<'a>,
    pending_type_token: Option<bool>,  // true = TYPE, false = VARIABLE
}

impl<'a> C11Lexer<'a> {
    fn next(&mut self, ctx: &TypedefContext) -> Result<Option<C11Terminal<A>>, String> {
        // If we have a pending TYPE/VARIABLE token, emit it
        if let Some(is_type) = self.pending_type_token.take() {
            return Ok(Some(if is_type {
                C11Terminal::Type
            } else {
                C11Terminal::Variable
            }));
        }

        let tok = self.lexer.next()?;

        match tok {
            Token::Ident(s) if !is_keyword(&s) => {
                // Check if typedef, queue TYPE/VARIABLE for next call
                self.pending_type_token = Some(ctx.is_typedef(&s));
                C11Terminal::Name(s)
            }
            // ... keywords, operators, etc.
        }
    }
}
```

### Typedef Context Management

```rust
struct TypedefContext {
    scopes: Vec<HashSet<String>>,  // Stack of scopes
}

impl TypedefContext {
    fn is_typedef(&self, name: &str) -> bool {
        self.scopes.iter().rev().any(|s| s.contains(name))
    }

    fn declare_typedef(&mut self, name: String) {
        self.scopes.last_mut().unwrap().insert(name);
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashSet::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}
```

## Key Grammar Patterns

```rust
// Typedef vs variable names - lexer inserts TYPE/VARIABLE after NAME
typedef_name = NAME TYPE @typedef_name;
var_name = NAME VARIABLE @var_name;

// Declarations split by whether they introduce typedefs
declaration = declaration_specifiers init_declarator_list? SEMICOLON @decl_var
            | declaration_specifiers_typedef init_declarator_list? SEMICOLON @decl_typedef;

// Empty production triggers context save at scope boundaries
save_context = _ @save_context;

// Scoped constructs wrap content with context save
scoped_compound_statement = save_context compound_statement @scoped_block;
```

## Grammar Size Comparison

Compared to a traditional Jourdan-style parser:
- **Original**: ~150 non-terminals, ~250 rules
- **With `prec OP`**: ~135 non-terminals, ~200 rules (15 expression rules -> 1)

## Running

```bash
# Build
cargo build --example c11

# Run tests
cargo test --example c11

# Run the example
cargo run --example c11
```

## References

- [Jourdan's C11 Parser (Menhir)](https://github.com/jhjourdan/C11parser)
- [C11 Standard (N1570)](http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf)

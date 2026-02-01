# Runtime Precedence in LR Parsing

If you've ever written a parser for a language with operators, you've seen this pattern:

```yacc
multiplicative_expression
    : cast_expression
    | multiplicative_expression '*' cast_expression
    | multiplicative_expression '/' cast_expression
    | multiplicative_expression '%' cast_expression
    ;

additive_expression
    : multiplicative_expression
    | additive_expression '+' multiplicative_expression
    | additive_expression '-' multiplicative_expression
    ;

shift_expression
    : additive_expression
    | shift_expression '<<' additive_expression
    | shift_expression '>>' additive_expression
    ;

relational_expression
    : shift_expression
    | relational_expression '<' shift_expression
    | relational_expression '>' shift_expression
    | relational_expression '<=' shift_expression
    | relational_expression '>=' shift_expression
    ;

// ... and so on for 10 more levels
```

This is the C expression grammar. Every precedence level gets its own non-terminal. Every non-terminal chains to the next. The grammar is correct, but it's also unreadable, fragile, and encodes knowledge (operator precedence) in the wrong place.

What if you could write this instead?

```
expr = expr OP expr | primary;
```

One rule for all binary expressions. The precedence information moves to where it belongs — the tokens themselves. The lexer returns `+` with precedence 12, `*` with precedence 13. The parser resolves ambiguity at runtime by comparing these values.

## Runtime Precedence

The insight is simple: **don't bake precedence into the parse table. Leave the shift/reduce conflicts in place, and resolve them at runtime.**

When the parser reaches a state where it could either shift an operator or reduce an expression, it compares:
1. The precedence of the operator on the stack (from the token that was shifted earlier)
2. The precedence of the incoming operator (the current token)

If the incoming operator has higher precedence, shift. If lower, reduce. If equal, use associativity to break the tie.

### How It Works

Declare a "precedence terminal" in your grammar:

```rust
grammar! {
    Calc;

    terminals {
        NUM: i64,
        prec OP: char,  // OP carries precedence
    }

    expr: Expr = expr OP expr @binary
               | NUM @literal;
}
```

The lexer returns each operator with its precedence:

```rust
fn operator_token(op: char) -> CalcTerminal {
    match op {
        '+' | '-' => CalcTerminal::Op(op, Precedence::left(1)),
        '*' | '/' => CalcTerminal::Op(op, Precedence::left(2)),
        '^'       => CalcTerminal::Op(op, Precedence::right(3)),
        // ...
    }
}
```

The grammar stays simple — one rule for all binary expressions. The precedence lives with the operators, where it belongs.

### Shunting-Yard Meets LR

If this reminds you of Pratt parsing, you're not wrong — but Pratt is the recursive formulation of Dijkstra's shunting-yard algorithm, a technique specifically for expressions. You typically embed a Pratt parser inside a larger recursive descent parser for the full language.

What we're doing here is different: it's the natural union of shunting-yard and canonical LR. The LR automaton handles the full grammar — statements, declarations, type expressions, everything. When it hits an expression with precedence conflicts, it resolves them shunting-yard style, comparing precedence values at runtime. You get the generality of LR with the elegance of shunting-yard for the parts that need it.

### User-Defined Operators

Because precedence comes from tokens at runtime, you can support user-defined operators:

```
> operator @ 14 left    // declare @ as precedence 14, left-associative
> 2 + 3 @ 4
14
```

The parser doesn't change. The lexer consults a table of known operators, and that table can grow during execution. When the parser reduces `operator @ 14 left`, a semantic action registers the new operator. Subsequent tokens include `@` with its precedence. No grammar changes, no parser regeneration.

This is how languages like Haskell handle user-defined operators. But traditionally they've required special parsing techniques — a Pratt parser for expressions, or post-parse tree rotation. With runtime precedence in an LR parser, it falls out naturally.

## Precedence-Carrying Non-Terminals

There's a wrinkle when applying runtime precedence to real languages like C. Some operators serve double duty:
- `*` is multiplication (binary) and dereference (unary)
- `&` is bitwise AND (binary) and address-of (unary)
- `+` and `-` are arithmetic (binary) and unary plus/minus

If you want runtime precedence for C expressions, you can't just have one `OP` terminal. These tokens appear in unary rules too:

```
unary_expr = STAR cast_expr @deref
           | AMP cast_expr @addr_of
           | ...;
```

The solution: let non-terminals carry precedence, not just terminals.

```
// Non-terminal collects all binary operators
binary_op = BINOP @op_binop
          | STAR @op_mul
          | AMP @op_bitand
          | PLUS @op_add
          | MINUS @op_sub;

// Single rule for all binary expressions
binary_expr = binary_expr binary_op binary_expr @binary
            | cast_expr;

// STAR, AMP, etc. still usable directly in unary rules
unary_expr = STAR cast_expr @deref
           | AMP cast_expr @addr_of
           | ...;
```

When `STAR` (precedence 13) reduces to `binary_op`, the resulting non-terminal inherits the precedence. The `binary_expr` rule sees `binary_op` with precedence 13 and resolves conflicts correctly.

The implementation is simple. When reducing, capture precedence from the rightmost symbol before popping it:

```rust
fn reduce(&mut self, rule: usize) {
    let (lhs, rhs_len) = RULES[rule];

    // Capture precedence before popping
    let prec = self.stack.last().and_then(|(_, p)| *p);

    for _ in 0..rhs_len {
        self.stack.pop();
    }

    // New non-terminal entry inherits the precedence
    self.stack.push((next_state, prec));
}
```

The parser stack carries optional precedence with each entry. Conflict resolution compares precedence values regardless of whether they came from terminals or non-terminals.

## Push-Based Parsing

Runtime precedence requires the lexer to annotate tokens with metadata. This is natural with a push-based parser — you control the token flow, so you control what information each token carries.

In a pull-based parser, the parser calls the lexer. Information flows one way. If the lexer needs to know something the parser has learned, you need globals, callbacks, or careful coordination through side channels.

In a push-based parser, you own the loop:

```rust
for token in lexer.tokens(&context) {
    parser.push(token, &mut actions)?;
    // actions may update context
}
```

You call the lexer, passing whatever context it needs. You call the parser with the token. Parser actions update the context. Next iteration, the lexer sees the update. It's just normal control flow.

### Lexer Feedback

This pattern — information flowing from parser back to lexer — is more common than people realize. The user-defined operators example requires it: the lexer consults a table of known operators, and that table grows during parsing.

The most famous instance is C's "lexer hack." In C, `T * x;` could be a multiplication (if `T` is a variable) or a pointer declaration (if `T` is a typedef). The parser can't know which without tracking declarations, but declarations are in the input being parsed. The lexer needs to know what the parser has seen.

With a pull-based parser, this genuinely feels like a hack — you're working against the tool's model of how parsing should flow. With a push-based parser, it's just normal control flow.

### Parsing as a Library

Push-based design also enables runtime grammar construction. Most parser generators are build tools — you write a grammar file, run a generator, get source code, compile it. If the grammar lives in a config file that users can modify, you're stuck.

But if the parser is a state machine you drive, you can construct that state machine at runtime:

```rust
let src = std::fs::read_to_string("expr.gzl")?;
let grammar = parse_grammar(&src)?;
let compiled = CompiledTable::build(&grammar);
let mut parser = Parser::new(compiled.table());
```

Load a grammar from a file, build the table, get a parser. No code generation, no build step. The grammar can come from anywhere — a config file, user input, a network request.

### Nested Parsers

Push-based parsers compose. Consider parsing a token stream using a grammar loaded at runtime — but the token stream format itself needs parsing. You have two parsers: one compiled (parses the token format), one runtime (parses according to the loaded grammar).

```rust
grammar! {
    grammar TokenFormat {
        start tokens;
        tokens = token*;
        token: Unit = IDENT colon_value? at_precedence? @token;
        // ...
    }
}
```

Each `@token` action drives the runtime parser:

```rust
fn token(&mut self, name: String, value: Option<String>, prec: Option<Precedence>) -> Unit {
    let id = self.compiled.symbol_id(&name).expect("unknown terminal");
    let token = Token::new(id, prec);

    self.reduce(Some(&token));  // Runtime parser: reduce
    self.stack.push(Ast::Leaf(name, value));
    self.parser.shift(&token);  // Runtime parser: shift
    Unit
}
```

The token format parser's semantic actions *are* the parse loop for the runtime grammar. There's no intermediate representation — parsing happens as tokens are recognized.

```
Input: "NUM:1 OP:+@<1 NUM:2 OP:*@<2 NUM:3"
  ↓
Token format parser (compiled)
  ↓ @token actions
Runtime grammar parser (loaded from file)
  ↓
AST
```

Both parsers are just state machines you drive. They compose because neither one owns the control flow.

## The State of Parser Generators

Parser generators have a reputation problem. Several high-profile projects — Go, Rust, Clang — use handwritten recursive descent. The mass exodus suggests something is wrong.

But the problem isn't the concept. Context-free grammars are a precise way to specify syntax. LR parsing is well-understood and efficient. Having a grammar as an artifact — something you can analyze, generate documentation from, reason about — is genuinely valuable. Handwritten parsers give you none of this.

The problem is that the dominant tools are stuck in 1975.

Yacc established patterns we're still living with: standalone code generators, grammars mixed with C code, semantic values accessed through cryptic `$$` and `$1` notation. You don't import yacc as a library — you run it as a build step, manage the generated files, debug "state 47" conflicts.

There's a reason for this, and it's not just inertia. A grammar by itself can only accept or reject input — to extract structured data, you need something more. The classic answer is to embed semantic actions: code fragments that run when rules are reduced. But this requires code generation, which requires a build step, which means your grammar is now polluted with host language snippets. The alternative — building a generic syntax tree with type-erased nodes — avoids the build step but just moves the problem downstream.

Modern tools have improved incrementally, but the core model persists. ANTLR generates code. Tree-sitter generates code. Parser combinators (nom, combine) avoid code generation but abandon grammars as declarative artifacts — the specification is the code. The middle ground — construct a grammar at runtime, get a parser back — is underexplored.

LR parsing has real limitations — error recovery is difficult, and incremental reparsing isn't natural. But for batch parsing, the core ideas remain solid. The tools just haven't evolved.

## Gazelle

[Gazelle](https://github.com/gerben-stavenga/gazelle) is an LR parser generator for Rust that implements these ideas. It's a library — no external build steps, no generated files. You can use a proc-macro for compile-time generation, or construct grammars programmatically at runtime.

The key to avoiding both embedded actions and type-erased trees is trait-based semantics. Gazelle generates a trait from your grammar, with one method per named reduction. You implement the trait, providing the types and the logic:

```rust
impl CalcActions for Evaluator {
    type Expr = i64;
    fn binary(&mut self, left: i64, op: char, right: i64) -> i64 {
        match op {
            '+' => left + right,
            '*' => left * right,
            // ...
        }
    }
}
```

The grammar stays clean. Your implementation is normal Rust code with full type safety. The parser is push-based, so lexer feedback and parser composition work naturally.

---

Good tools are sharp and give you freedom to wield them as you need.

---

## Appendix: Parsing C11

Jacques-Henri Jourdan and François Pottier developed a particularly clean formulation for their verified C11 parser (["A Simple, Possibly Correct LR Parser for C11"](https://dl.acm.org/doi/10.1145/3064848)). Their work is worth studying — not just for the technique, but for how it reframes what a grammar is.

The C lexer hack exists because `T * x;` could be a multiplication or a pointer declaration, depending on whether `T` is a typedef. The parser can't know without tracking declarations, but declarations are in the input being parsed. Information must flow from parser back to lexer.

Jourdan and Pottier's insight: augment both the token stream and the grammar to make context explicit.

When the lexer sees an identifier, it emits two tokens: `NAME` (the string) followed by `TYPE` or `VARIABLE` (based on whether it's a known typedef). The grammar has separate rules for each case:

```
typedef_name = NAME TYPE;
var_name = NAME VARIABLE;
```

The grammar also includes empty productions that exist solely to trigger actions at the right moment:

```
save_context = ;  // empty, but has a semantic action
scoped_block = save_context compound_statement;
```

Semantic actions update a typedef table. The lexer queries this table for each identifier. The parser's structure coordinates the whole dance.

The key insight: the grammar here — with its synthetic `TYPE` and `VARIABLE` tokens, its empty `save_context` productions — isn't the grammar of C11. It's a grammar *for parsing* C11. The token stream isn't what a naive lexer would produce; it's augmented with context markers. You're not feeding the official language specification into a black box. You're using the parser generator as a tool, programming the grammar and token stream to solve the actual problem.

This is the right way to think about parser generators. They're not validators for your language spec. They're tools for building parsers.

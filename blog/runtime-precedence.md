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

One rule to ring them all. The precedence information moves to where it belongs — the tokens themselves. The lexer returns `+` with precedence 12, `*` with precedence 13. The parser resolves ambiguity at runtime by comparing these values.

This post describes how this works, why it's not the same as Pratt parsing, and what it enables.

## The State of Parser Generators

Parser generators have a reputation problem. Ask around and you'll hear that they're outdated, inflexible, not worth the trouble. Several high-profile projects — Go, Rust, Clang, many JavaScript engines — use handwritten recursive descent. The mass exodus suggests something is wrong.

But the calculation is more nuanced than "handwritten is better." These are massive projects with decades-long horizons and teams who can afford to maintain a handwritten parser. At that scale, the flexibility of custom code can outweigh the benefits of a formal grammar — handwritten code is, by definition, maximally capable. You can do anything. For a project like LLVM, that matters.

Most projects aren't LLVM. Most languages don't have a team of compiler engineers maintaining the frontend for twenty years. For the rest of us, the tradeoffs point the other way: a grammar is a specification you can reason about, and maintaining a handwritten parser is a cost that doesn't pay off.

So what's actually wrong with parser generators? The theory is fine. Context-free grammars are a precise way to specify syntax. LR parsing is well-understood, efficient, and guarantees termination. Having a grammar as an artifact — something you can analyze for ambiguity, generate documentation from, reason about formally — is genuinely valuable. Handwritten parsers give you none of this. The specification exists only implicitly, spread across hundreds of functions.

The problem isn't the concept. It's that the dominant tools are stuck in 1975.

Yacc established patterns we're still living with: standalone code generators, grammars mixed with C code, semantic values accessed through cryptic `$$` and `$1` notation. You don't import yacc as a library — you run it as a build step, manage the generated files, debug "state 47" conflicts. The workflow assumes you're writing C, that code generation is cheap but compilation is expensive, that type safety is someone else's problem.

There's a reason for this, and it's not just inertia. A grammar by itself can only accept or reject input — to extract structured data, you need something more. The classic answer is to embed semantic actions: code fragments that run when rules are reduced. But this requires code generation, which requires a build step, which means your grammar is now polluted with host language snippets. The alternative — building a generic syntax tree with type-erased nodes — avoids the build step but just moves the problem downstream: now you're traversing strings and node names, converting them into whatever types you actually want.

Neither option is satisfying, and this tension is part of why parser generators haven't evolved into simple libraries.

Modern parser generators have improved incrementally, but the core model persists. ANTLR generates code. Tree-sitter generates code. Even newer Rust tools often expect you to embed actions in the grammar or run a build script. The idea that a parser generator could be a library you call — that you could construct a grammar programmatically, get a parser back, and run it — remains surprisingly rare.

And then there's the expression problem. Most grammar rules read naturally — statement syntax, declarations, type expressions all translate cleanly into BNF. But binary expressions require that 15-level cascade, or a dozen rules with `%left` and `%right` declarations, and they all do the same thing: reduce to a binary operation with a different operator attached. The repetition obscures what's actually a simple structure: two expressions, one operator, recurse.

The underlying concepts are sound. The tools just haven't evolved.

## Gazelle

[Gazelle](https://github.com/gerben-stavenga/gazelle) is an LR parser generator for Rust, designed to address these pain points.

It's a library. You add it to your `Cargo.toml` and call it from your code — no external build steps, no generated files to manage. You can use a proc-macro for compile-time parser generation, or construct grammars programmatically at runtime. Either way, the parser generator is just a dependency, not infrastructure.

The key to avoiding both embedded actions and type-erased trees is trait-based semantics. Gazelle generates a trait from your grammar, with one method per named reduction. You implement the trait, providing the types and the logic. The grammar describes syntax; your implementation describes what to do with it.

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

The grammar stays clean — no code fragments, no `$$` notation. Your implementation is normal Rust code with full type safety, auto-completion, and compile-time verification that your code matches the grammar. And because you control the types, you're not stuck interpreting a generic tree.

That's the high-level interface. But Gazelle also exposes the automaton directly for more dynamic use cases. At this level, parsing produces reduction signals in postfix order — you decide how to interpret them. Build a tree, evaluate directly, feed into another system. The library doesn't prescribe a representation; it gives you the structure and gets out of the way. Sharp but versatile.

Gazelle's parser is push-based: you control the loop. The parser is a state machine you drive, not a function that calls back into you. This turns out to matter for lexer feedback, streaming input, and anywhere you need fine-grained control over the parsing process.

Theoretically, Gazelle is a slight generalization of canonical LR parsing. The key extension: precedence lives with tokens, not grammar structure. Shift/reduce conflicts that would normally be errors are instead resolved at runtime by comparing precedence values. One rule for binary expressions instead of fifteen — but that's the subject of the next section.

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

The lexer returns each operator with its precedence. Simplified:

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

What Gazelle does is somewhat different: it's the natural union of shunting-yard and canonical LR. The LR automaton handles the full grammar — statements, declarations, type expressions, everything. When it hits an expression with precedence conflicts, it resolves them shunting-yard style, comparing precedence values at runtime. You get the generality of LR with the elegance of shunting-yard for the parts that need it.

### User-Defined Operators

Because precedence comes from tokens at runtime, you can support user-defined operators. Imagine a language where users can declare new operators:

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

## Lexer Feedback

You've already seen lexer feedback in this post, though I didn't call it that. The user-defined operators example requires it: the lexer consults a table of known operators, and that table can be modified during parsing. When a user declares `operator @ 14 left`, the parser processes that declaration and updates the table; subsequent tokens include `@` with its precedence. Information flows from parser back to lexer.

This pattern is more common than people realize. The most famous instance is C's "lexer hack" — the trick that lets C compilers distinguish typedef names from variable names. In C, `T * x;` could be a multiplication (if `T` is a variable) or a pointer declaration (if `T` is a typedef). The parser can't know which without tracking declarations, but declarations are in the input being parsed. The lexer needs to know what the parser has seen.

The technique has been called a "hack" since the earliest yacc-based C compilers, and with those tools it genuinely felt like one. The parser calls the lexer (pull-based), but the lexer needs parser state. So you thread global mutable state through the system, or set up callbacks, or carefully coordinate through side channels. You're working against the tool's model of how parsing should flow.

With a push-based parser, the feedback stops feeling like a hack. You control the loop. You call the lexer, passing whatever context it needs. You call the parser with the token. Parser actions update the context. Next iteration, the lexer sees the update. It's just normal control flow — no globals, no callbacks, no side channels.

Jacques-Henri Jourdan and François Pottier developed a particularly clean formulation for their verified C11 parser (["A Simple, Possibly Correct LR Parser for C11"](https://dl.acm.org/doi/10.1145/3064848)). Their insight: augment both the token stream and the grammar to make context explicit.

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

Gazelle doesn't invent this technique — Jourdan and Pottier did the hard work. But Gazelle's design makes implementing it straightforward:

```rust
let mut parser = Parser::new();

for token in lexer.tokens(&parser.ctx) {
    parser.push(token, &mut actions)?;  // may update parser.ctx
}
```

Empty reductions trigger trait methods. Trait methods update your context. The lexer reads that context. Everything composes cleanly because you own the loop.

This solution illustrates Gazelle's philosophy. The grammar here — with its synthetic `TYPE` and `VARIABLE` tokens, its empty `save_context` productions — isn't the grammar of C11. It's a grammar *for parsing* C11. The token stream isn't what a naive lexer would produce; it's augmented with context markers. You're not feeding the official language specification into a black box. You're using the LR parser generator as a tool, programming the grammar and token stream to solve the actual problem. Gazelle embraces this.

---

Good tools are sharp, lightweight, and give you freedom to wield them as you need. That's what Gazelle aims to be.

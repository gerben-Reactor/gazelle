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

One rule. The precedence information moves to where it belongs - the tokens themselves. The lexer returns `+` with precedence 12, `*` with precedence 13. The parser resolves ambiguity at runtime by comparing these values.

This post describes how this works, why it's not the same as Pratt parsing, and what it enables.

## Why Parser Generators Have a Bad Reputation

There are two dominant positions on parser generators, and I think both are wrong.

**"Just handwrite it."** This view has gained traction, especially after high-profile projects like Go and Rust moved to handwritten parsers. The argument: handwritten parsers are simpler, give better errors, and offer total control.

What this misses: a grammar is a specification. When you handwrite a parser, the grammar exists only implicitly, spread across hundreds of functions. You lose the ability to reason about your language systematically - to ask "is this grammar ambiguous?" or "what happens if I add this production?" You're trading long-term clarity for short-term convenience.

There's a telling moment in a [discussion between Jonathan Blow and Casey Muratori](https://youtu.be/fIPO4G42wYE?t=3826) where they dismiss parser theory while praising the simplicity of Pratt parsing. Casey asks whether you couldn't make the entire parser work like the Pratt recursion "loop" - but with precedence tables that differ depending on context. He's essentially rediscovering the core insight behind LR parsing: a state machine where each state has its own table of actions based on what's been seen so far. The literature they're dismissing contains exactly the ideas they're reaching for.

**"The parser should do everything."** The opposite extreme: the parser takes characters and outputs a complete AST. Lexing, parsing, tree construction, maybe even some semantic analysis - all driven by the grammar. This is the yacc vision, and it's equally flawed.

The reality is that parsing is one stage in a pipeline. The boundaries between stages - lexer, parser, semantic analysis - are design choices, not laws. Sometimes it makes sense to solve a problem in the lexer. Sometimes the parser. Sometimes later. A good parser generator should be a tool you leverage, not a monolith that owns your entire frontend.

**The real problem is the tools.** Yacc and bison established patterns in the 1970s that we're still stuck with: global state, cryptic `$$` and `$1` notation, poor type safety, error messages that reference "state 47". The frustration people feel with parser generators is largely frustration with these specific tools. The underlying concepts - grammar as specification, systematic ambiguity detection, generated parsing - are sound. The APIs just need to catch up.

## Parser as Leverage

Here's a different way to think about it. You have a pipeline:

```
source text → tokens → parse tree → AST → typed AST → ...
```

Each arrow is a boundary, and each boundary is a choice:

- **Tokens can carry metadata.** Not just "this is a PLUS token" but "this is a PLUS token with precedence 12 and source location (line 7, column 3)."

- **The lexer can see parser state.** In a push-based parser, you control the loop. The lexer can query the parser's context before deciding what token to emit.

- **The parse tree doesn't have to be the AST.** It can be a concrete syntax tree that you transform later. Or it can be a set of semantic actions that directly interpret the program. The grammar describes syntax, not the final representation.

- **Ambiguities can be resolved wherever it makes sense.** Some in the lexer (keywords vs identifiers). Some in the parser (precedence). Some in semantic analysis (overload resolution). The parser doesn't have to solve everything.

This is the philosophy behind Gazelle: the parser is a tool that does heavy lifting in your pipeline. It's not the whole pipeline.

Two examples will make this concrete.

## Runtime Precedence

The traditional way to handle operator precedence in LR parsing: encode it in the grammar structure (the 15-level cascade) or declare it to the parser generator (`%left`, `%right`) which bakes it into the parse table at generation time.

Both approaches have the same fundamental limitation: precedence is static. It's fixed when you build the parser. You can't have user-defined operators with custom precedence. You can't have precedence that varies by context. The information is frozen in either the grammar rules or the parse table.

The insight is simple: **don't bake precedence into the table. Leave the shift/reduce conflicts in place, and resolve them at runtime.**

When the parser reaches a state where it could either shift an operator or reduce an expression, it compares:
1. The precedence of the operator on the stack (from the token that was shifted earlier)
2. The precedence of the incoming operator (the current token)

If the incoming operator has higher precedence, shift. If lower, reduce. If equal, use associativity to break the tie.

### How It Works

Declare a "precedence terminal" in your grammar:

```rust
grammar Calc {
    terminals {
        NUM: f64,
        prec OP: char,  // OP carries precedence
    }

    expr: Expr = expr OP expr @binary
               | NUM @literal;
}
```

The lexer returns each operator with its precedence:

```rust
fn tokenize(c: char) -> CalcTerminal {
    match c {
        '+' | '-' => CalcTerminal::Op(c, Precedence::left(1)),
        '*' | '/' => CalcTerminal::Op(c, Precedence::left(2)),
        '^'       => CalcTerminal::Op(c, Precedence::right(3)),  // right-associative
        // ...
    }
}
```

The grammar stays simple - one rule for all binary expressions. The precedence lives with the operators, where it belongs.

### This Is Not Pratt Parsing

Pratt parsing (also called "top-down operator precedence" or TDOP) also handles precedence at runtime. But Pratt is a recursive descent technique limited to expression parsing. You typically embed a Pratt parser inside a larger recursive descent parser for the full language.

Runtime precedence in LR parsing is different:
- It's still LR. You get the full power of LR grammars, not just expressions.
- It integrates with the rest of your grammar. The expression rules are just rules, same as everything else.
- The parse table is generated normally. Only the conflict resolution changes.

You're not replacing LR with Pratt. You're extending LR to handle precedence dynamically.

### User-Defined Operators

Because precedence comes from tokens at runtime, you can define new operators on the fly:

```
> operator ** pow right 3
> 2 ** 3 ** 2
512
```

The lexer consults a table of known operators. The table can be modified during parsing (or before, or between files). No grammar changes needed.

This is how languages like Haskell handle user-defined operators. But traditionally they've required special parsing techniques - a Pratt parser for expressions, or post-parse tree rotation. With runtime precedence in an LR parser, it falls out naturally.

## The C11 Dual-Role Problem

C has operators that serve double duty:
- `*` is multiplication (binary) and dereference (unary)
- `&` is bitwise AND (binary) and address-of (unary)
- `+` is addition (binary) and unary plus
- `-` is subtraction (binary) and unary minus

If you want to use runtime precedence for C expressions, you can't just have one `OP` terminal for all operators. These tokens appear in the unary rules too:

```
unary_expression = STAR cast_expression @deref
                 | AMP cast_expression @addr_of
                 | PLUS cast_expression @unary_plus
                 | MINUS cast_expression @unary_minus
                 | ...;
```

The naive approach: use separate precedence terminals and separate binary rules for each dual-role operator. This works but results in multiple rules instead of one.

The cleaner solution: collect all binary operators into a non-terminal that inherits precedence.

### Precedence-Carrying Non-terminals

There's a natural extension that solves this: let non-terminals carry precedence too.

Think about what happens during LR parsing:
- **Shift**: push a terminal onto the stack
- **Reduce**: pop N symbols, push a non-terminal onto the stack

These operations are structurally similar. If terminals can carry precedence, why not non-terminals?

```
// Non-terminal collects all binary operators
binary_op = BINOP @op_binop
          | STAR @op_mul
          | AMP @op_bitand
          | PLUS @op_add
          | MINUS @op_sub;

// Single rule for all binary expressions
binary_expression = binary_expression binary_op binary_expression @binary
                  | cast_expression;

// STAR, AMP, etc. still usable directly in unary rules
unary_expression = STAR cast_expression @deref
                 | AMP cast_expression @addr_of
                 | ...;
```

When `STAR` (precedence 12) is reduced to `binary_op`, the resulting non-terminal inherits the precedence. The `binary_expression` rule sees `binary_op` and uses its precedence for conflict resolution - correctly preferring `*` over `+`.

The implementation is surprisingly simple. When reducing, capture the precedence from the rightmost RHS symbol before popping it from the stack:

```rust
fn do_reduce(&mut self, rule: usize, actions: &mut A) {
    let (lhs_id, rhs_len) = RULES[rule];

    // Capture precedence from topmost RHS symbol before popping
    let captured_prec = if rhs_len > 0 {
        self.state_stack.last().and_then(|(_, p)| *p)
    } else {
        None
    };

    for _ in 0..rhs_len {
        self.state_stack.pop();
    }

    // ... compute semantic value ...

    // Propagate captured precedence to the new non-terminal entry
    self.state_stack.push((next_state, captured_prec));
}
```

The parser stack now contains symbols (terminals or non-terminals) that optionally carry precedence. Conflict resolution doesn't care which kind of symbol it's looking at - it just compares precedence values.

This is implemented and working. The C11 expression evaluator example uses a unified `binary_op` non-terminal that handles all of C's binary operators (arithmetic, bitwise, logical, comparison) with correct precedence - while those same terminals (`*`, `&`, `+`, `-`) remain available for unary expressions.

## Lexer Feedback: The Typedef Problem

Runtime precedence is one example of parser-as-leverage. Here's another: solving C's infamous typedef ambiguity.

The problem: in C, `T * x;` could be:
- A multiplication expression (if `T` is a variable)
- A pointer declaration (if `T` is a typedef name)

The parser can't know which without tracking which identifiers are typedefs. But the typedef declarations are in the input being parsed. Chicken and egg.

This problem is as old as C itself. The traditional solution - the "lexer hack" - has been used since the earliest yacc/lex-based C compilers. The lexer maintains a symbol table and returns different token types for typedef names vs regular identifiers. It works, but it typically involves global state and awkward coupling between lexer and parser.

### The Jourdan-Pottier Solution

Jacques-Henri Jourdan and François Pottier developed a particularly elegant formulation for their verified C11 parser (see ["A Simple, Possibly Correct LR Parser for C11"](https://dl.acm.org/doi/10.1145/3064848)):

1. **Two-token identifiers.** When the lexer sees an identifier, it emits two tokens: `NAME` (the identifier itself) followed by either `TYPE` or `VARIABLE` (depending on whether it's a typedef).

2. **Grammar distinguishes them.** The grammar has separate rules:
   ```
   typedef_name = NAME TYPE;
   var_name = NAME VARIABLE;
   ```

3. **Empty productions for context.** The grammar includes "dummy" productions that trigger at scope boundaries:
   ```
   save_context = ;  // empty production, but has an action
   scoped_block = save_context compound_statement;
   ```

4. **Parser drives the lexer.** Semantic actions update a typedef table. The lexer queries this table to decide `TYPE` vs `VARIABLE`.

### Why Push-Based Parsing Feels Natural Here

The lexer hack works with traditional pull-based parsers - decades of C compilers prove that. But it typically involves global mutable state, awkward callbacks, or careful coordination between parser and lexer through side channels.

With a push-based parser, the feedback loop becomes explicit. You own the loop:

```rust
loop {
    let token = lexer.next(&context);    // lexer sees parser context
    parser.push(token, &mut actions);     // actions update context
}
```

Each iteration:
1. Lexer reads the current typedef context
2. Parser processes the token
3. Semantic actions update the context (new typedef declared, scope entered, etc.)
4. Next iteration sees updated context

The parser's control flow drives everything. The `save_context` production exists solely to trigger an action at the right point in the parse - it's using the grammar structure to hook into the parser's execution.

This is what I mean by "parser as leverage." You're not trying to solve the typedef problem purely in the grammar, or purely in the lexer, or purely in post-parse analysis. You're using the parser's structure to coordinate between lexer and semantic actions. The boundaries are permeable.

## What This Enables

**Readable grammars.** Instead of 15 precedence levels encoded as chained non-terminals, you have one rule and precedence metadata on tokens. The grammar describes structure; precedence is specified where operators are defined.

**Rapid iteration.** Changing operator precedence doesn't require modifying the grammar. Adding a new operator is a lexer change, not a grammar change. The parser doesn't need to be regenerated for every tweak.

**User-defined operators.** Languages with extensible syntax (Haskell, Raku, many research languages) can handle custom operators naturally, without special parsing phases or post-parse tree manipulation.

**Complex languages.** C and C++ have notorious parsing challenges: typedef ambiguity, template angle brackets, context-sensitive keywords. Lexer feedback makes these manageable. The parser becomes a tool you leverage to solve problems, not a black box you fight against.

**Multiple backends.** When grammar is separated from semantic actions (actions are a trait you implement, not code embedded in the grammar), you can have multiple implementations: interpreter, AST builder, pretty-printer, language server. Same grammar, different behaviors.

## Conclusion

Parser generators aren't obsolete. They're under-innovated.

The value is real: a grammar is a precise specification of your language's syntax. A parser generator gives you systematic ambiguity detection, guaranteed termination, and a formal artifact you can reason about. Handwritten parsers give you none of this.

The problems people encounter are largely problems with 1970s tool design: global state, poor APIs, inflexible models, cryptic error messages. These aren't fundamental to the concept.

Runtime precedence is one example of what's possible when you question the assumptions. Precedence doesn't have to be baked into the parse table. Conflicts can be resolved dynamically. Tokens can carry metadata. The lexer can see parser state.

The parser is a tool. Use it as leverage.

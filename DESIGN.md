# Gazelle Design Document

A lightweight parser generator library for Rust.

## Philosophy

**Parsing is just a tool** to transform strings into structured forms. It's never the end goal—you always need validation, type checking, semantic analysis afterward. Gazelle should do syntax well, then get out of the way.

**Parser generators have a bad reputation**, but for the wrong reasons:

| Complaint | Real cause | Gazelle's answer |
|-----------|-----------|------------------|
| "Hand-written is simpler" | Only true for stable grammars. Evolving languages benefit from grammar-driven generation | Focus on fast iteration |
| "Terrible error messages" | Tool problem, not fundamental | Counterexample generation for conflicts |
| "Ambiguities are hidden" | PEG's ordered choice silently picks first match | Use CFG + surface real conflicts |
| "Spurious conflicts" | LALR's aggressive state merging | Minimal LR algorithm |
| "Atrocious API" | Bison's globals, lex/yacc split, pull architecture | Library-first, push architecture, clean Rust API |

## Core Design Decisions

### Minimal LR

- **Not LALR**: LALR merges states aggressively, producing spurious reduce/reduce conflicts that aren't real ambiguities
- **Not full LR(1)**: Table sizes explode
- **Minimal LR**: Only split states where merging actually causes conflicts. LALR-sized tables without spurious conflicts

### Conflict Explanation via Counterexamples

When a real conflict exists, show a concrete ambiguous input:

```
Conflict: shift/reduce on '+'

  Parse 1: expr + expr • + expr  →  expr + (expr + expr)
  Parse 2: expr + expr • + expr  →  (expr + expr) + expr

  Ambiguous input: "1 + 2 + 3"
```

Not "state 47: shift/reduce conflict on '+'" with no explanation.

### Push Architecture

The parser is a state machine driven from the outside, not a function that pulls input.

```rust
let mut parser = Parser::new(&tables);
let mut lexer = Lexer::new(input);

while let Some(token) = lexer.next(&context) {
    for event in parser.push(token) {
        match event {
            Event::Reduce { rule, children } => { /* ... */ }
            Event::Accept => break,
            Event::Error { expected } => { /* ... */ }
        }
    }
}
```

**Benefits:**
- Streaming—no need to buffer entire input
- Pause/resume—parser is just state
- Incremental—feed updated tokens for editor integration
- User controls I/O—works with async, files, network
- Composable—parser doesn't own token source or AST construction
- Debuggable—inspect every step

**The lexer hack becomes elegant:** When parsing C, the `typedef` problem (is `foo` a type or identifier?) requires parser context during lexing. With push architecture:

```rust
for event in parser.push(token) {
    if let Event::Reduce { rule: Rule::Typedef, children } => {
        lexer.register_type(children.name());
    }
}
```

The outer loop sees typedef reductions and updates lexer context before the next token. No globals, no callbacks.

**Even cleaner:** Use a bogus token pattern:
```
type = id typetoken
```
Lexer always emits `id`, inserts `typetoken` after known types. The grammar explicitly shows the disambiguation.

### Library-First Architecture

Expose the interesting parts as reusable libraries, not a monolithic tool:

```
┌─────────────────────────────────┐
│  Proc macro / nice frontend     │  ← optional sugar
├─────────────────────────────────┤
│  Parser runtime                 │  ← table-driven, push-based
├─────────────────────────────────┤
│  Table construction library     │  ← THE INTERESTING PART
│  - Grammar representation       │
│  - LR automaton construction    │
│  - Minimal LR state splitting   │
│  - Conflict detection           │
│  - Counterexample generation    │
├─────────────────────────────────┤
│  Grammar AST / IR               │
└─────────────────────────────────┘
```

Users can:
- Build different frontends
- Visualize grammars/automata
- Analyze grammars without generating parsers
- Integrate into editor tooling
- Experiment with parsing research

## Novel Contribution: Unified LR + Operator Precedence

### The Problem

Traditional grammars explode for expressions:

```
expr     = add_expr
add_expr = add_expr '+' mul_expr | add_expr '-' mul_expr | mul_expr
mul_expr = mul_expr '*' unary    | mul_expr '/' unary    | unary
unary    = '-' unary | primary
primary  = number | '(' expr ')'
```

One rule per precedence level. All that machinery just encodes precedence.

### The Solution

Write:
```
expr = expr OP expr | atom
```

Where `OP` is a token that **carries precedence from the lexer**:

```rust
enum Token {
    Op { symbol: String, prec: u8, assoc: Assoc },
    Num(i64),
    // ...
}
```

At parse time, `expr OP1 expr • OP2` decisions use token data:
- `prec(OP2) > prec(OP1)` → shift
- `prec(OP2) < prec(OP1)` → reduce
- Equal → use associativity

The precedence table compresses into token metadata. The LR table for expressions collapses to nearly nothing.

### Implementation: Precedence-Carrying States

Standard LR parser state is just a table index:

```
action[state, token] -> shift(new_state) | reduce(rule) | error
```

Gazelle extends state to carry precedence:

```
state = (table_state, precedence)

action[table_state, token]:
  | shift only       -> shift(new_state, token.precedence)
  | reduce only      -> reduce(rule)
  | shift AND reduce ->
      if token.precedence > state.precedence: shift
      if token.precedence < state.precedence: reduce
      if equal: use token.associativity (left=reduce, right=shift)
```

When you shift an OP token, its precedence becomes part of the new state on the stack. When a shift/reduce conflict arises, compare the incoming token's precedence against the state's precedence.

**Example: `1 + 2 * 3`**

```
stack: [expr(1), +, expr(2)]    state.precedence = prec(+) = 1
token: * with precedence = 2

2 > 1 → shift (parse * first)

result: 1 + (2 * 3)
```

**Example: `1 * 2 + 3`**

```
stack: [expr(1), *, expr(2)]    state.precedence = prec(*) = 2
token: + with precedence = 1

1 < 2 → reduce (finish * first)

result: (1 * 2) + 3
```

**Why this matters:**

- The LR table stays small—one `expr = expr OP expr` rule, not N rules
- Conflicts are resolved at parse time with token data, not at table-generation time
- Same table works for any operators, including dynamically defined ones
- No grammar explosion, no static precedence declarations

The table generator marks `expr = expr OP expr` conflicts as "precedence-resolved" rather than erroring. The runtime knows to consult token precedence for these.

### Dynamic Operators (Haskell-style)

With push architecture, new operators can be defined mid-parse:

```rust
for event in parser.push(token) {
    if let Event::Reduce { rule: Rule::InfixDecl, children } => {
        // infixl 6 +++
        lexer.register_op("+++", prec: 6, assoc: Left);
    }
}
```

Grammar doesn't change. Lexer starts recognizing `+++` as `OP` with precedence 6.

### AST Implications

Don't encode each operator as a different node type:

```rust
// Bad: structure explosion
enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    // ... forever
}

// Good: operator is data
struct BinOp {
    left: Box<Expr>,
    op: Token,  // symbol, precedence, associativity
    right: Box<Expr>,
}

enum Expr {
    BinOp(BinOp),
    Atom(Token),
}
```

One grammar rule → one AST node type → downstream passes match on operator data, not node variants.

## Usage Modes

### Sharp-Knife Mode (Runtime Grammar)

The core library works with grammars provided at runtime. Everything is type-erased:

```rust
let grammar = Grammar::parse(grammar_str)?;
let tables = TableBuilder::new(&grammar).build()?;
let mut parser = Parser::new(&tables);

for event in parser.push(token) {
    match event {
        Event::Reduce { rule, children } => {
            // rule is a RuleId (number)
            // tokens are generic Token structs
            match grammar.rule_name(rule) {
                "expr" => { /* ... */ }
                "atom" => { /* ... */ }
                _ => { /* ... */ }
            }
        }
    }
}
```

**Use cases:**
- Tools that process arbitrary grammars
- User-defined DSLs (grammar comes from config/input)
- Grammar experimentation and visualization
- Building other parser generators on top

### Codegen Mode (Compile-Time Grammar)

A proc macro generates type-safe wrappers from a grammar string. Rules declare their output types, but **no code in the grammar**—builders are separate:

```rust
gazelle::grammar! {
    expr: Expr = expr OP expr
               | atom;

    atom: Expr = NUM
               | '(' expr ')';

    stmt: Stmt = 'let' IDENT '=' expr ';';
}
```

**The macro generates a builder trait:**

```rust
// Generated by macro
trait GrammarBuilder {
    // expr alternatives
    fn expr_op(&mut self, left: Expr, op: OpToken, right: Expr) -> Expr;
    fn expr_atom(&mut self, a: Expr) -> Expr;

    // atom alternatives
    fn atom_num(&mut self, n: NumToken) -> Expr;
    fn atom_paren(&mut self, l: LParen, e: Expr, r: RParen) -> Expr;

    // stmt alternatives
    fn stmt_let(&mut self, kw: LetKw, name: Ident, eq: Eq, val: Expr, semi: Semi) -> Stmt;
}

// Also generates token types, rule enum, parser wrapper, etc.
```

**You implement the builder as normal Rust:**

```rust
struct AstBuilder;

impl GrammarBuilder for AstBuilder {
    fn expr_op(&mut self, left: Expr, op: OpToken, right: Expr) -> Expr {
        Expr::BinOp {
            left: Box::new(left),
            op: op.into(),
            right: Box::new(right),
        }
    }

    fn expr_atom(&mut self, a: Expr) -> Expr {
        a
    }

    fn atom_num(&mut self, n: NumToken) -> Expr {
        Expr::Num(n.value)
    }

    fn atom_paren(&mut self, _l: LParen, e: Expr, _r: RParen) -> Expr {
        e
    }

    fn stmt_let(&mut self, _kw: LetKw, name: Ident, _eq: Eq, val: Expr, _semi: Semi) -> Stmt {
        Stmt::Let { name: name.text, value: val }
    }
}
```

**Benefits:**
- Grammar is clean—just structure and types, no code
- Builder code is real Rust with full IDE support
- Multiple builders possible (AST, pretty-printer, interpreter, validator)
- Refactoring works across grammar and builders
- Compile-time type checking between grammar and implementation
- No string interpolation or `$1` magic

**The proc macro:**
1. Parses grammar string at compile time
2. Validates types match between rules (e.g., `atom` returns `Expr`, used in `expr`)
3. Generates token types and rule enums
4. Generates builder trait with typed method signatures
5. Calls the core library to build tables
6. Embeds tables as static data
7. Generates parser wrapper that calls builder methods on reduce

Same library underneath both modes.

## Summary

Gazelle is:
1. **Minimal LR** for correctness without spurious conflicts
2. **Push-based** for composability and control
3. **Library-first** exposing table construction algorithms
4. **Unified LR + precedence parsing** as a novel contribution
5. **Practical** with clean Rust API and good error messages

The goal: make grammar iteration so painless it's obviously better than hand-writing parsers.

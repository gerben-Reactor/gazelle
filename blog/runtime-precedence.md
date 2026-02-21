# LR Parsing: Simpler, More Flexible, More Powerful Than You Think

LR parsing has a reputation for complexity. Multiple algorithms (SLR, LALR, canonical LR), inscrutable conflict messages ("state 47: shift/reduce conflict on ';'"), `%expect` declarations to suppress warnings you don't understand, yacc grammars polluted with embedded C code. High-profile projects — Go, Rust, Clang — abandoned parser generators for handwritten recursive descent. The exodus suggests something is fundamentally wrong.

It isn't. The theory is clean. The tools are the problem.

## Simpler Than You Think

### Simple table construction is NFA → DFA → minimize

Every textbook presents LR table construction as its own algorithm: compute closure, compute goto, build item sets iteratively. But it's not a special algorithm. It's the same NFA → DFA pipeline used to build lexers, applied to a different NFA.

The equivalence between LR item closure and NFA epsilon-closure is a known textbook observation — but every implementation I'm aware of skips the NFA and builds item sets directly via the worklist approach. Gazelle takes the observation literally: build an actual NFA, then apply the standard pipeline.

#### Items as NFA states

An LR(1) item `[A → α • β, a]` says: we've seen `α`, we expect `β`, and after the whole rule reduces, the next token should be `a`. The dot tracks progress through a rule. Each item is a state in a (very large) NFA.

Consider a trivial grammar:

```
expr = expr '+' expr | NUM
```

Some items for this grammar (ignoring lookaheads for now):

```
[expr → • expr '+' expr]     -- haven't seen anything yet
[expr → expr • '+' expr]     -- seen one expr, expecting '+'
[expr → expr '+' • expr]     -- seen expr '+', expecting another expr
[expr → expr '+' expr •]     -- complete, ready to reduce
[expr → • NUM]               -- expecting a number
[expr → NUM •]               -- seen a number, ready to reduce
```

The transitions between items follow two rules:

1. **Shift/goto**: seeing a symbol advances the dot. `[expr → • NUM]` transitions to `[expr → NUM •]` on symbol `NUM`. This is a labeled transition — the core of any NFA.

2. **Closure**: if the dot is before a non-terminal, we might start parsing any of that non-terminal's rules. `[expr → expr '+' • expr]` expands to `[expr → • expr '+' expr]` and `[expr → • NUM]`. These are epsilon transitions — you can be in the expanded states without consuming input.

That's it. Items are states. Advancing the dot is a transition. Closure is epsilon. This is an NFA.

#### Building the NFA

The traditional approach builds item sets incrementally with a worklist: compute closure, compute transitions, check if the target set already exists, repeat. It works, but it's doing subset construction inline, mixed with LR-specific closure logic.

Gazelle separates the two. First, build the NFA by enumerating all possible items — a flat triple loop over `(rule, dot, lookahead)`:

```rust
for (rule_idx, rule) in grammar.rules.iter().enumerate() {
    for dot in 0..=rule.rhs.len() {
        for la in 0..num_terminals {
            let idx = item_state(rule_idx, dot, la);

            if dot == rule.rhs.len() {
                // Complete item: transition on lookahead to reduce node
                nfa.add_transition(idx, la, reduce_node);
            } else {
                let next_sym = rule.rhs[dot];
                // Shift/goto: advance the dot
                nfa.add_transition(idx, next_sym, item_state(rule_idx, dot + 1, la));

                // Closure: if next symbol is a non-terminal, add epsilon edges
                // to all its rules' initial items, with appropriate lookaheads
                if next_sym.is_non_terminal() {
                    let lookaheads = first(rule.rhs[dot+1..], la);
                    for closure_rule in grammar.rules_for(next_sym) {
                        for closure_la in lookaheads {
                            nfa.add_epsilon(idx, item_state(closure_rule, 0, closure_la));
                        }
                    }
                }
            }
        }
    }
}
```

Most items are unreachable — subset construction ignores them. The simplicity is the point: no worklist, no duplicate checking, just enumerate and wire up.

The lookahead computation for closure edges uses FIRST sets — `first(β, a)` asks "what terminals can appear at the start of `β a`?" This is the standard fixed-point computation over the grammar, done once upfront.

#### The same pipeline as lexers

From here, construction uses the exact same algorithms as building a lexer from a regex:

1. **Subset construction** — NFA → DFA. Start from the epsilon-closure of the initial item. For each DFA state (a set of NFA states), group outgoing transitions by symbol, epsilon-close each target set. If the target set is new, it becomes a new DFA state. Repeat until no new states appear.

2. **Hopcroft minimization** — merge equivalent DFA states. Start with a partition (reduce states grouped by rule, everything else in one group). Repeatedly split groups where states disagree on which group their transitions lead to. When no more splits are possible, each remaining group becomes one state in the minimized DFA.

```rust
pub(crate) fn build_minimal_automaton(grammar: &GrammarInternal) -> AutomatonResult {
    let first_sets = FirstSets::compute(grammar);
    let (nfa, nfa_info) = build_lr_nfa(grammar, &first_sets);

    let raw_dfa = subset_construction(&nfa);
    // ... resolve conflicts, add spurious reductions ...
    let (min_dfa, _) = hopcroft_minimize(&raw_dfa, &initial_partition);
    // ... extract parse table from minimized DFA ...
}
```

The `Nfa`, `Dfa`, `subset_construction`, and `hopcroft_minimize` are generic — they know nothing about parsing. The same `automaton.rs` module works for lexers. LR-specific logic is only in how you *build* the NFA (the triple loop above) and how you *interpret* DFA states afterward — which ones are shift states, which are reduce states, what rule to reduce by.

Canonical LR(1) produces a lot of states. The lookahead component multiplies the state space — two items with the same rule and dot but different lookaheads become separate NFA states, producing separate DFA states. For C11, LALR produces ~400 states; canonical LR(1) produces thousands.

You might expect DFA minimization to fix this — after all, Hopcroft finds the *minimal* DFA. But Hopcroft preserves exact behavior, and states that differ only in lookahead sets are not equivalent: they have different transitions. Minimization merges genuinely redundant states but can't recover LALR-level compression. This is why the field moved to LALR instead — a separate construction that cannot be phrased as NFA → DFA (more on this below). And as far as I can tell, no existing parser generator applies DFA minimization to the LR automaton at all.

#### Parsing doesn't need exact behavior

A parser needs to:

1. **Parse valid inputs correctly** — produce the right sequence of reductions.
2. **Reject invalid inputs** — report an error at some point.

It does *not* need to reject invalid inputs at the earliest possible moment. If the parser performs some extra reductions before noticing the error, that's fine — the error is still detected, just a few steps later.

This is the key reframing. A DFA that accepts a regular language must behave correctly on *every* input — valid or invalid. A parser only needs correct behavior on valid parse strings. On invalid strings, it just needs to eventually error. This relaxation opens the door to *spurious reductions*.

#### Spurious reductions

Consider a DFA state that reduces rule 3 when it sees `+` or `)`. What happens if we also add a reduction on `$` — a token that can never actually appear at this point during a valid parse? Nothing changes for correct inputs: the parser never reaches this state with `$` as the lookahead, so the spurious reduction never fires.

For erroneous inputs, the parser might perform some extra reductions before detecting the error — but it still detects it. The spurious reduction produces a non-terminal and transitions via goto to a new state, but `$` is still the lookahead. If `$` were valid in this new context, it would have been a valid lookahead in the original state — and the reduction wouldn't have been spurious. So the error surfaces, just a few steps later.

This is the lever that makes DFA minimization effective for parsers. By adding reductions that are unreachable during valid parses, we can make states that were *almost* identical become *truly* identical — and then Hopcroft merges them.

#### LALR

LALR is a separate, weaker construction — it handles a smaller class of grammars (LALR(1) ⊂ LR(1)) in exchange for far fewer states. It builds LR(0) item sets (ignoring lookaheads), then computes lookaheads after the fact. This avoids the state explosion, but some grammars that are unambiguous under LR(1) produce conflicts under LALR — the tool rejects a valid grammar.

These false conflicts are particularly damaging because they're inexplicable from the grammar itself. The user stares at their rules, sees no ambiguity, and gets a reduce/reduce conflict — an artifact of the weaker construction, not of the language. There's nothing to fix in the grammar because the grammar is fine. This is a major reason people give up on LR parsing: the tool reports conflicts that aren't real, and the only remedies are restructuring a correct grammar to appease the algorithm or reaching for `%expect` to suppress the warning. It's not the theory that's hostile — it's the tooling.

In our framing, we can mimic LALR by adding spurious reductions to all same-core states unconditionally — union all their reduce transitions. If this doesn't introduce a reduce/reduce conflict (same core, different rules on the same lookahead), subsequent Hopcroft minimization produces an LALR-sized table. When it does introduce a conflict, LALR rejects the grammar. That's the LALR(1) ⊂ LR(1) gap.

#### Minimal LR and IELR

So you want LR(1) power — no false conflicts — but LALR-sized tables. The literature has two answers: minimal LR (Pager's algorithm) and IELR. Both try to merge states like LALR but avoid introducing conflicts. Minimal LR merges states during construction and splits them apart when merging would create a conflict — however, it has a subtle problem: when the grammar has *real* conflicts (which are common — think operator precedence), the merge-and-split process can produce a different parser than canonical LR(1) with the same conflict resolution applied. IELR fixes this by precisely tracking which lookaheads are "contributions" from merging versus original, so it can distinguish real conflicts from artifacts — but the algorithm is correspondingly more involved.

Both are specialized procedures that, like LALR, construct the smaller automaton directly. They interleave state construction with merging decisions, requiring purpose-built algorithms that don't decompose into reusable components.

#### The simple version

In the NFA → DFA → minimize framework, there's a simpler approach. The pipeline has three steps:

1. **Resolve conflicts.** After subset construction, identify shift/reduce and reduce/reduce conflicts in the raw DFA and resolve them (shift wins for S/R, lowest-numbered rule wins for R/R). This produces a clean, conflict-free automaton.

2. **Add spurious reductions.** Group DFA states by their LR(0) core. For each group, copy reduce transitions from siblings to fill gaps — but only when all states in the group agree. If state A reduces rule 3 on `)` and state B reduces rule 5 on `)`, that symbol is left alone. No spurious reduction is added that would introduce a conflict.

3. **Minimize.** Run Hopcroft. States that now have identical transitions get merged.

```rust
let raw_dfa = subset_construction(&nfa);
let mut lr = classify_dfa_states(&raw_dfa, num_items);

// Step 1: resolve conflicts in the raw DFA
resolve_conflicts(&raw_dfa, &mut lr, &nfa_info, grammar);

// Step 2: add spurious reductions where safe
merge_lookaheads(&mut raw_dfa, &lr, &nfa_info);

// Step 3: minimize
let (min_dfa, _) = hopcroft_minimize(&raw_dfa, &initial_partition);
```

The `merge_lookaheads` step is conservative: it only adds a spurious reduction when every state in the core group that already has a transition on that symbol agrees on the target. Disagreement means merging would create a conflict, so those states stay distinct.

```rust
// Collect reduce transitions: only keep if all states that have
// the transition agree on the target
for &state in group {
    for &(sym, target) in &dfa.transitions[state] {
        if !lr.has_items[target] {  // target is a reduce node
            sym_to_target.entry(sym)
                .and_modify(|t| if *t != Some(target) { *t = None })
                .or_insert(Some(target));
        }
    }
}

// Fill gaps: add transitions each state is missing
for &state in group {
    for (&sym, &target) in &sym_to_target {
        if let Some(target) = target {
            if !existing.contains(&sym) {
                dfa.transitions[state].push((sym, target));
            }
        }
    }
}
```

The result is correct by construction. Step 1 starts from the canonical LR(1) automaton — the gold standard — and resolves conflicts there. Steps 2 and 3 only add spurious reductions (which don't affect valid parses) and merge states that are now identical. At no point can the pipeline introduce a false conflict or change the parse of a valid input. There's nothing to prove — each step trivially preserves the LR(1) parse.

The result achieves LALR-level compression for grammars that are LALR, and gracefully keeps states distinct where LALR would reject the grammar.

### Simple grammar, orthogonal semantics

Everything so far has been about building the automaton — the machine that decides whether a string belongs to the language. In practice, acceptance alone is useless. You want to extract structured data — an AST, a value, a translation. The grammar imposes a tree structure on the input, and somehow that structure must reach user code that does something with it. This is where parser generators get messy.

An LR automaton by itself only accepts or rejects. To extract meaning, you need semantic actions. Yacc's answer is to embed them in the grammar:

```yacc
expr : expr '+' expr { $$ = $1 + $3; }
```

Now the grammar is a program in a bespoke language — `$$` and `$1` notation, no type safety, C fragments mixed into the specification. It can't be analyzed, reformatted, or used to generate documentation independently of the embedded code. Refactoring the grammar means refactoring the actions. The specification and the program are fused.

Parser combinators go the other direction: the code *is* the grammar. There's no separate specification at all. You gain composability and type safety but lose the grammar as an artifact.

A third approach — building a generic syntax tree with type-erased nodes — avoids the entanglement but just moves the problem downstream. You get an untyped tree and pattern-match your way through it, with no compiler help when the grammar changes.

Gazelle's answer is trait-based semantics. The grammar is declarative:

```rust
gazelle! {
    grammar calc {
        start expr;
        terminals {
            NUM: _,
            LPAREN, RPAREN,
            prec OP: _
        }

        expr = NUM => num
             | LPAREN expr RPAREN => paren
             | expr OP expr => binop;
    }
}
```

The `_` after a terminal means it carries a value (the type is determined by the user). `prec` marks a terminal that carries runtime precedence. Each `=> name` labels an alternative — this becomes a variant in the generated enum.

The macro generates two things. First, a `Types` trait with an associated type for each symbol:

```rust
// Generated (simplified)
trait Types {
    type Error;
    type Num;          // terminal payload
    type Op;           // terminal payload
    type Expr;         // non-terminal result
}
```

Second, an enum for each non-terminal, parameterized by `Types`:

```rust
// Generated (simplified)
enum Expr<A: Types> {
    Num(A::Num),
    Paren(A::Expr),
    Binop(A::Expr, A::Op, A::Expr),
}
```

These enums *are* the abstract syntax tree — the grammar's structure encoded in Rust's type system. Each variant is a grammar alternative, each field a symbol in the right-hand side. But the fields are associated types, so they don't have to hold tree nodes. Set them to the enum types themselves and you get a concrete syntax tree. Set them to `i64` and you get direct evaluation. Set them to your own IR nodes and you get a compiler front-end. The grammar's shape is fixed; what flows through it is up to you.

You implement `Types` to choose your concrete types, and `Action` to define what reductions do:

```rust
struct Eval;

impl calc::Types for Eval {
    type Error = gazelle::ParseError;
    type Num = i64;
    type Op = char;
    type Expr = i64;
}

impl gazelle::Action<calc::Expr<Self>> for Eval {
    fn build(&mut self, node: calc::Expr<Self>) -> Result<i64, gazelle::ParseError> {
        Ok(match node {
            calc::Expr::Num(n) => n,
            calc::Expr::Paren(e) => e,
            calc::Expr::Binop(l, op, r) => match op {
                '+' => l + r,
                '-' => l - r,
                '*' => l * r,
                '/' => l / r,
                _ => panic!("unknown op"),
            },
        })
    }
}
```

When the parser reduces `expr OP expr`, it constructs `Expr::Binop(left, op, right)` with the concrete types from your `Types` impl — `i64`, `char`, `i64` in this case — and calls `Action::build`. The grammar defines node shapes. Your code defines meaning. Full type safety, no `$$` notation, no type erasure.

#### Blanket implementations

For a CST, just set each associated type to the generated enum itself:

```rust
struct Cst;

impl calc::Types for Cst {
    type Error = gazelle::ParseError;
    type Num = String;
    type Op = String;
    type Expr = Box<calc::Expr<Self>>;  // boxed for recursion
}
```

No `Action` impl needed — blanket implementations handle identity, boxing for recursive types, and ignoring nodes you don't care about. Custom `Action` impls are only needed when you want to transform nodes into different types — evaluation, IR construction, declaration collection.

## More Flexible Than You Think

### Push-based parsing

Traditional parser generators use a pull model: the parser calls the lexer for the next token. Gazelle inverts this. You own the loop, you feed tokens to the parser:

```rust
for token in lex(&input) {
    parser.push(token, &mut actions)?;
}
parser.finish(&mut actions)?;
```

The generated parser has just two methods: `push` (feed a token) and `finish` (signal end of input). Internally it's an LR state machine — shift, reduce, goto — but the user never sees that.

### Lexer feedback and the C lexer hack

The lexer and parser are completely independent components. Neither calls the other. The user writes the loop that connects them — and that loop can do whatever it wants between lexing a token and pushing it. This is where context-sensitive behavior lives, in ordinary user code, not in hooks or callbacks baked into the tools.

The most famous instance is C's "lexer hack." In C, `T * x;` is a multiplication expression if `T` is a variable, or a pointer declaration if `T` is a typedef. The parser tracks declarations, but the lexer needs to know about them to emit the right tokens.

Jacques-Henri Jourdan and François Pottier found a clean formulation for their verified C11 parser (["A Simple, Possibly Correct LR Parser for C11"](https://dl.acm.org/doi/10.1145/3064848)). Their insight: augment the token stream and the grammar to make context explicit. When the lexer sees an identifier, it emits two tokens: `NAME` (the string) followed by `TYPE` or `VARIABLE` (based on the current typedef table). The grammar has separate rules for each case:

```
typedef_name = NAME TYPE;
var_name = NAME VARIABLE;
```

The grammar also includes empty productions that trigger context-saving actions at the right moment:

```
save_context = ;  // empty, but has a semantic action
scoped_block = save_context compound_statement;
```

With a push-based parser, the coordination lives in ordinary user code:

```rust
// The actions struct owns the typedef table.
// Parser reductions (via &mut actions) update it when typedef declarations are reduced.
let mut actions = C11Actions::new();

for raw_token in lexer.tokens(&input) {
    match raw_token {
        RawToken::Ident(name) => {
            parser.push(Terminal::Name(name.clone()), &mut actions)?;
            if actions.typedefs.contains(&name) {
                parser.push(Terminal::Type, &mut actions)?;
            } else {
                parser.push(Terminal::Variable, &mut actions)?;
            }
        }
        other => {
            parser.push(other.into(), &mut actions)?;
        }
    }
}
```

The lexer doesn't know about typedefs. The parser doesn't know about typedefs. The loop coordinates them — reading from the typedef table when emitting tokens, writing to it when semantic actions fire.

This pattern generalizes. Any time the parser learns something that affects how subsequent input should be tokenized — user-defined operators, context-sensitive keywords, indentation levels — the push loop is the natural place to handle it.

### Parser composition

Push-based parsers compose. In the `runtime_grammar` example, one compiled parser handles the token stream format, and its semantic actions directly drive a second parser (built at runtime from a loaded grammar):

```rust
gazelle! {
    grammar token_format {
        start sentences;
        terminals {
            IDENT: _, NUM: _,
            COLON, AT, LT, GT, SEMI
        }
        sentences = sentence* => sentences;
        sentence = tokens SEMI => sentence;
        tokens = _ => empty | tokens token => append;
        token = IDENT colon_value? at_precedence? => token;
        // ...
    }
}
```

The `=> empty` reduction creates a fresh runtime parser. Each `=> append` reduction constructs a runtime token and pushes it into that parser. Each `=> sentence` reduction finishes the runtime parse and prints the tree. Two state machines, composed through normal function calls — the token format parser's semantic actions *are* the parse loop for the runtime grammar:

```
Input: "NUM:1 OP:+@<1 NUM:2 OP:*@<2 NUM:3"
  ↓
Token format parser (compiled, from gazelle! macro)
  ↓ Semantic actions push tokens into...
Runtime grammar parser (loaded from .gzl file)
  ↓
AST
```

Neither parser knows about the other. They compose because neither one owns the control flow.

### Parser as library, not build tool

Most parser generators are build tools. You write a grammar file, run a code generator, manage the output. If the grammar lives in a config file or comes from user input, you're stuck.

Gazelle's table construction is a function call:

```rust
let src = std::fs::read_to_string("expr.gzl")?;
let grammar = parse_grammar(&src)?;
let compiled = CompiledTable::build(&grammar);
let mut parser = CstParser::new(compiled.table());
```

`parse_grammar` turns a grammar string into a grammar representation. `CompiledTable::build` runs the NFA → DFA → minimize pipeline and compresses the result into lookup tables. `CstParser::new` wraps the table in a push-based parser that builds a concrete syntax tree.

Load a grammar from anywhere — a file, a string, a network response. Build the table. Get a parser. No code generation, no build step.

The proc-macro is convenience for when you want compile-time tables and typed AST nodes. It generates the same tables as the runtime path, just baked into the binary. It's not the only path.

## More Powerful Than You Think

### Runtime precedence

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

// ... and so on for 10 more levels
```

Every precedence level gets its own non-terminal. Every non-terminal chains to the next. The grammar encodes operator precedence in the wrong place — it's structural information about operators, baked into the grammar's shape.

What if you could write this instead?

```
expr = expr OP expr => binop | term => term;
```

One rule for all binary expressions. One rule to rule them all. The precedence information moves to where it belongs — the tokens:

```rust
'+' => expr::Terminal::Op('+', Precedence::Left(6)),
'-' => expr::Terminal::Op('-', Precedence::Left(6)),
'*' => expr::Terminal::Op('*', Precedence::Left(7)),
'/' => expr::Terminal::Op('/', Precedence::Left(7)),
'^' => expr::Terminal::Op('^', Precedence::Right(8)),
```

#### The conflict

The rule `expr = expr OP expr` is ambiguous. `1 + 2 * 3` has two parses:

```
  1 + (2 * 3)       (1 + 2) * 3
```

In a standard LR parser, this ambiguity is a shift/reduce conflict — an error during table construction. Yacc resolves it statically with `%left`/`%right` declarations.

Gazelle takes a different approach: leave the conflict in the table. Instead of choosing shift or reduce at construction time, the table stores a `ShiftOrReduce` entry that defers the decision to runtime.

#### How prec terminals enter the NFA

Without special treatment, prec terminals would create ordinary shift/reduce conflicts in the DFA. A complete item `[expr → expr OP expr •, OP]` and a shift item `[expr → expr • OP expr]` both transition on `OP` in the NFA. Subset construction follows both, producing a DFA state that mixes reduce nodes (accept states) with regular item nodes — a shift/reduce conflict. The `resolve_conflicts` pass would need a special case: "this is a prec terminal, don't resolve it, keep both options." That's a special case in the middle of the pipeline.

The trick: each `prec` terminal gets a **virtual symbol** used exclusively for reduce transitions. When item `[A → α •, OP]` is complete and the lookahead `OP` is a prec terminal, the NFA transition uses the virtual symbol instead of the real terminal ID. Shift transitions still use the real ID.

Now prec terminals never create conflicts in the DFA at all. Shift transitions go on one edge (real ID), reduce transitions go on another (virtual ID). Subset construction, conflict resolution, spurious reduction merging, Hopcroft minimization — none of them need to know about prec terminals.

At the end, `build_table_from_dfa` merges the two edges back: if a state has both a shift edge (real ID) and a reduce edge (virtual ID) for the same prec terminal, the table entry becomes `ShiftOrReduce { shift_state, reduce_rule }`. If only one exists, it's a plain `Shift` or `Reduce`. The virtual symbols disappear — they were scaffolding for the pipeline.

The only prec-specific code is at the boundaries: symbol splitting before the NFA, merging after the DFA. Everything in between is generic.

#### Runtime resolution

When the parser hits a `ShiftOrReduce`, it compares two precedence values:

1. The precedence of the operator already on the stack (the one that was shifted earlier — `+` in our example)
2. The precedence of the incoming operator (the current lookahead — `*`)

```rust
ParserOp::ShiftOrReduce { shift_state, reduce_rule } => {
    let should_reduce = match (self.state.prec, lookahead_prec) {
        (Some(sp), Some(tp)) => {
            if tp.level() > sp.level() {
                false  // incoming binds tighter → shift
            } else if tp.level() < sp.level() {
                true   // stack binds tighter → reduce
            } else {
                // Equal precedence: use associativity
                matches!(sp, Precedence::Left(_))
            }
        }
        _ => false, // no precedence info → shift (default)
    };
}
```

Walk through `1 + 2 * 3`:

| Stack | Input | Action | Why |
|-------|-------|--------|-----|
| | `1 + 2 * 3` | shift `1` | |
| `1` | `+ 2 * 3` | reduce `1` → `expr` | |
| `expr` | `+ 2 * 3` | shift `+` (prec: Left(6)) | |
| `expr +` | `2 * 3` | shift `2` | |
| `expr + 2` | `* 3` | reduce `2` → `expr` | |
| `expr + expr` | `* 3` | **ShiftOrReduce**: stack has `+` (Left(6)), lookahead is `*` (Left(7)). 7 > 6, so **shift**. | `*` binds tighter |
| `expr + expr *` | `3` | shift `3` | |
| `expr + expr * 3` | EOF | reduce `3` → `expr` | |
| `expr + expr * expr` | EOF | reduce `expr * expr` → `expr` | |
| `expr + expr` | EOF | reduce `expr + expr` → `expr` | |
| `expr` | EOF | accept | |

The result is `1 + (2 * 3)` — correct precedence from runtime comparison, not from grammar structure.

For equal precedence with left-associativity (e.g., `1 - 2 - 3`): stack has `-` (Left(6)), lookahead is `-` (Left(6)). Equal levels, left-associative → reduce. Result: `(1 - 2) - 3`.

#### Shunting-yard meets LR

If this reminds you of Pratt parsing, you're not wrong — but Pratt is the recursive formulation of Dijkstra's shunting-yard algorithm, a technique specifically for expressions. You typically embed a Pratt parser inside a larger recursive descent parser for the full language.

What we're doing here is different: it's the natural union of shunting-yard and canonical LR. The LR automaton handles the full grammar — statements, declarations, type expressions, everything. When it hits an expression with precedence conflicts, it resolves them shunting-yard style, comparing precedence values at runtime. You get the generality of LR with the elegance of shunting-yard for the parts that need it.

Yacc's `%left`/`%right` declarations are a static version of this idea — they resolve conflicts at table construction time. Deferring the resolution to runtime, so the same table works for any precedence assignment, is new as far as I know.

#### User-defined operators

Because precedence comes from tokens at runtime, user-defined operators fall out naturally:

```
> operator @ 14 left    // declare @ as precedence 14, left-associative
> 2 + 3 @ 4
14
```

The parser doesn't change. The lexer consults a table of known operators, and that table can grow during execution. When the parser reduces `operator @ 14 left`, a semantic action registers the new operator. Subsequent tokens include `@` with its precedence. No grammar changes, no parser regeneration.

This is how languages like Haskell handle user-defined operators. But traditionally they've required special parsing techniques — a Pratt parser for expressions, or post-parse tree rotation. With runtime precedence in an LR parser, it falls out of the existing mechanism.

#### Precedence-carrying non-terminals

Real languages complicate this. In C, `*` is both multiplication (binary, precedence 13) and dereference (unary prefix). You can't funnel both uses through a single `prec OP` terminal because they appear in structurally different rules. The solution: make them separate `prec` terminals and collect them into a non-terminal for binary expressions:

```
terminals {
    prec OP: _,      // pure binary ops like ==, <<, etc.
    prec STAR,        // * as multiply OR dereference
    prec AMP,         // & as bitand OR address-of
    prec PLUS,        // + as add OR unary plus
    prec MINUS,       // - as subtract OR unary minus
    ...
}

binary_op = OP
          | STAR => op_mul
          | AMP => op_bitand
          | PLUS => op_add
          | MINUS => op_sub;

binary_expr = binary_expr binary_op binary_expr => binary
            | cast_expr => cast;

// STAR, AMP, etc. still usable directly in unary/prefix rules
unary_expr = STAR cast_expr => deref
           | AMP cast_expr => addr_of;
```

The lexer sends `STAR` with precedence regardless of context — the grammar sorts out which use applies. The question is how precedence propagates through reductions. When `STAR` reduces to `binary_op` (a single-symbol reduction), the precedence is preserved — it flows through the intermediate non-terminal. When `STAR` appears in `STAR cast_expr → deref` (a multi-symbol reduction), the anchor's precedence is used instead, discarding the operator's own precedence as it should. The `do_reduce` function manages this:

```rust
// For single-symbol reductions (like STAR → binary_op):
// preserve the symbol's own prec — it flows through
let captured_prec = if len == 1 {
    self.state.prec
// For multi-symbol reductions (like expr OP expr):
// use the anchor's prec — the context that was "waiting"
// before this sub-expression started
} else {
    anchor.prec
};
```

Three rules govern precedence propagation:

1. **Shifting a prec terminal** overwrites the stack entry's precedence with the token's value.
2. **Single-symbol reductions** (like `STAR → binary_op`) preserve precedence — it flows through intermediate non-terminals unchanged.
3. **Multi-symbol reductions** (like `expr OP expr` or `OP expr`) use the *anchor's* precedence — the stack entry that was waiting before this sub-expression began. This correctly resets the precedence context: after reducing `2 * 3` to `expr`, the precedence should reflect the context *around* `2 * 3`, not the `*` inside it.

For unary operators: in `2 * -3 + 4`, the lexer sends `MINUS` with Left(6) — same as for binary minus. The grammar routes it to `MINUS cast_expr → unary_expr` (multi-symbol), so Rule 3 resets to the anchor's precedence — `*`'s Left(7). Then `+` (Left(6)) correctly reduces `2 * (-3)` before adding `4`. The unary operator's own precedence is irrelevant; the grammar structure and Rule 3 handle it.

### Error reporting and recovery

Parsing theory is about recognizing valid inputs. Error handling — what happens when the input is *invalid* — tends to get far less attention in the literature and in parser generator tooling. But in practice, error reporting is what determines whether a parser is usable. A compiler that says "syntax error on line 42" and stops is barely better than one that crashes. Good diagnostics and the ability to recover and report multiple errors are what separate the toys from the real thing.

It's also genuinely hard. The best error messages require semantic understanding — "did you mean `==`?" or "missing return type" — which a parser generator doesn't have. Handwritten parsers *can* produce these, but at the cost of cataloging failure patterns one by one. It works but doesn't scale to every corner of the grammar.

A parser generator can't match that level of tailored diagnostics. But it has an advantage that handwritten parsers and parser combinators fundamentally lack: the full grammar and the full parse state are available as regular data. In a recursive descent parser, the parse state is the host language's call stack — opaque, not inspectable, not cloneable. You can't ask "what tokens would be valid here?" without manually maintaining that information. An LR parser with an explicit stack can answer that question directly from the parse table, and it can clone the parser to simulate speculative repairs. This makes automatic error reporting and recovery possible in a way that's difficult to replicate with hand-rolled logic.

#### Error messages

The parse table knows exactly which tokens are valid in each state. When the parser encounters an unexpected token, it can report what it expected instead. But a flat list of terminal names — "expected IDENT, NUM, LPAREN, MINUS, STAR, AMP" — isn't very helpful. It's technically correct but gives no context.

The parser has richer information: the LR items active in the current state tell you which rules are in progress and where the dot is. Gazelle uses these to produce structured error messages:

```
unexpected 'while', expected: expr, statement
  {            foo ( x )   ;       ???
  compound_statement        stmt*
  in compound_statement: '{' stmt* . '}'
```

Three layers of information:

1. **What went wrong**: the unexpected token and what the parser expected instead. But instead of listing raw terminals, the expected set is computed from the items' dot positions. If the dot is before a non-terminal like `expr`, the message says "expected expr" rather than enumerating every terminal that can start an expression.

2. **Where we are**: the parser stack, shown as token spans labeled with their grammar symbols. Each entry shows what has been parsed and how the grammar categorized it. This gives spatial context — you can see the `compound_statement` being built, the `stmt*` that was accumulating, the `{` that opened the block.

3. **What rule is in progress**: the active items with their dot positions, showing exactly which grammar rules are being matched and where the parser is within each one.

The expected set computation traces through nullable non-terminals (if `B` is optional and followed by `C`, both should appear as expected) and chases complete items back through the stack to find the calling context. This is what makes error messages reflect the actual parsing context rather than the raw state number.

#### Error recovery

Stopping at the first error is hostile. A parser that can recover and report multiple errors per file is far more useful.

The standard approach in yacc-family tools is `error` productions: you add rules like `stmt → error ';'` that match "skip everything until a semicolon." This works but it's manual — you have to anticipate where errors occur and what to skip to. Miss a case and the parser cascades into nonsense.

Gazelle's recovery is automatic. It uses Dijkstra's algorithm to search for the minimum-cost edit that gets the parser back on track. The search space has three operations:

- **Insert** a token (cost 1) — the parser acts as if a token appeared that wasn't in the input.
- **Delete** the current token (cost 1) — skip it and advance the input.
- **Shift** the current token (cost 0) — consume it normally.

The parser clones itself for each candidate repair, simulating the edit without modifying the real state. Dijkstra guarantees we explore lowest-cost repairs first. A repair is accepted when the parser can successfully consume three more real tokens from that point — enough confidence that we've genuinely resynchronized, not just patched one error to create another.

Typical repairs: inserting a missing semicolon (cost 1), deleting a stray token (cost 1), or inserting a closing brace and semicolon (cost 2). The search finds these automatically from the grammar — no `error` productions needed.

After applying a repair, the parser continues from the recovered state, consuming more input. If it hits another error, it runs the search again. The result is a list of `RecoveryInfo` entries, each with the position and the repairs applied. The caller formats these into messages like "inserted missing ';'" or "deleted unexpected 'while'".

The search is bounded to keep recovery fast. In practice, inserting or deleting one or two tokens covers the vast majority of real errors.

## Trade-Offs

Handwritten parsers are always more capable for bespoke requirements. You can craft error messages for specific situations ("did you mean `==` instead of `=`?"), implement incremental reparsing for IDE responsiveness, add context-sensitive hacks that no grammar formalism supports (C++ template angle brackets, contextual keywords). Some patterns that are trivial in a handwritten parser — treating a keyword as an identifier in certain positions, for instance — require grammar restructuring in LR. That flexibility costs engineering time — a handwritten parser for a real language is thousands of lines of careful code.

Parser generators are valuable for the many cases that don't need it. The grammar is a readable artifact you can analyze, generate documentation from, and reason about for correctness. The parser is correct by construction — if the grammar defines a language, the parser recognizes exactly that language. And if the tool is a library rather than a build step, the integration cost drops to a function call.

---

This is [Gazelle](https://github.com/gerben-stavenga/gazelle), available as [`gazelle-parser`](https://crates.io/crates/gazelle-parser) on crates.io. It currently parses a full C11 grammar.

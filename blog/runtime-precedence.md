# LR Parsing: Simpler, More Flexible, More Powerful Than You Think

LR parsing has a reputation for complexity. Multiple algorithms (SLR, LALR, canonical LR), inscrutable conflict messages ("state 47: shift/reduce conflict on ';'"), `%expect` declarations to suppress warnings you don't understand, yacc grammars polluted with embedded C code. High-profile projects — Go, Rust, Clang — abandoned parser generators for handwritten recursive descent. The exodus suggests something is fundamentally wrong.

It isn't. The theory is clean. The tools are the problem.

This post walks through five pain points that drive people away from LR parsing and shows that each one has a straightforward solution. The ideas are implemented in [Gazelle](https://github.com/gerben-stavenga/gazelle), a parser generator library for Rust.

## 1. "LR table construction is mysterious"

Every textbook presents LR table construction as its own algorithm: compute closure, compute goto, build item sets iteratively. But it's not a special algorithm. It's the same NFA → DFA pipeline used to build lexers, applied to a different NFA.

The equivalence between LR item closure and NFA epsilon-closure is a known textbook observation — but every implementation I'm aware of skips the NFA and builds item sets directly via the worklist approach. Gazelle takes the observation literally: build an actual NFA, then apply the standard pipeline.

### Items as NFA states

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

The transitions between items follow three rules:

1. **Shift/goto**: seeing a symbol advances the dot. `[expr → • NUM]` transitions to `[expr → NUM •]` on symbol `NUM`. This is a labeled transition — the core of any NFA.

2. **Closure**: if the dot is before a non-terminal, we might start parsing any of that non-terminal's rules. `[expr → expr '+' • expr]` expands to `[expr → • expr '+' expr]` and `[expr → • NUM]`. These are epsilon transitions — you can be in the expanded states without consuming input.

3. **Reduce**: when the dot reaches the end, the item is complete. `[expr → NUM •, +]` transitions on lookahead `+` to a reduce node — an accept state that says "reduce by this rule." Each rule gets its own reduce node, so the DFA can distinguish which reduction to perform.

Items are states. Advancing the dot is a shift transition. Closure is epsilon. Complete items transition to accept states on their lookahead. This is an NFA.

### Building the NFA

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

### The same pipeline as lexers

From here, construction uses the exact same algorithms as building a lexer from a regex:

1. **Subset construction** — NFA → DFA. Start from the epsilon-closure of the initial item. For each DFA state (a set of NFA states), group outgoing transitions by symbol, epsilon-close each target set. If the target set is new, it becomes a new DFA state. Repeat until no new states appear.

   Reduce nodes — the accept states from complete items — should always end up as singleton DFA states (a set containing just that one reduce node). Conflicts are naturally visible as violations of this property: if a DFA state contains two reduce nodes, that's a reduce/reduce conflict (two rules want to reduce on the same lookahead). If a DFA state contains both item nodes and a reduce node, that's a shift/reduce conflict (one rule wants to reduce while another wants to keep shifting). The generic subset construction algorithm produces these states without knowing what they mean — the LR interpretation comes afterward.

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

### The state explosion problem

Canonical LR(1) produces a lot of states. The lookahead component multiplies the state space — two items with the same rule and dot but different lookaheads become separate NFA states, producing separate DFA states. For C11, LALR produces ~400 states; canonical LR(1) produces thousands.

You might expect DFA minimization to fix this — after all, Hopcroft finds the *minimal* DFA. But Hopcroft preserves exact behavior, and states that differ only in lookahead sets are not equivalent: they have different transitions. Minimization merges genuinely redundant states but can't recover LALR-level compression. This is why the field moved to LALR instead — a separate construction that cannot be as neatly phrased as NFA → DFA.

### Parsing doesn't need exact behavior

A parser needs to:

1. **Parse valid inputs correctly** — produce the right sequence of reductions.
2. **Reject invalid inputs** — report an error at some point.

It does *not* need to reject invalid inputs at the earliest possible moment. If the parser performs some extra reductions before noticing the error, that's fine — the error is still detected, just a few steps later.

This is the key reframing. A DFA that accepts a regular language must behave correctly on *every* input — valid or invalid. A parser only needs correct behavior on valid parse strings. On invalid strings, it just needs to eventually error. This relaxation opens the door to *spurious reductions*.

### Spurious reductions enable minimization

Consider a DFA state that reduces rule 3 when it sees `+` or `)`. What happens if we also add a reduction on `$` — a token that can never actually appear at this point during a valid parse? Nothing changes for correct inputs: the parser never reaches this state with `$` as the lookahead, so the spurious reduction never fires.

For erroneous inputs, the parser might perform some extra reductions before detecting the error — but it still detects it. The spurious reduction produces a non-terminal and transitions via goto to a new state, but `$` is still the lookahead. If `$` were valid in this new context, it would have been a valid lookahead in the original state — and the reduction wouldn't have been spurious. So the error surfaces, just a few steps later.

This is the lever that makes DFA minimization effective for parsers. By adding reductions that are unreachable during valid parses, we can make states that were *almost* identical become *truly* identical — and then Hopcroft merges them.

### LALR, minimal LR, and IELR as special cases

LALR is a separate, weaker construction — it handles a smaller class of grammars (LALR(1) ⊂ LR(1)) in exchange for far fewer states. It builds LR(0) item sets (ignoring lookaheads), then computes lookaheads after the fact. This avoids the state explosion, but some grammars that are unambiguous under LR(1) produce conflicts under LALR — the tool rejects a valid grammar.

These false conflicts are particularly damaging because they're inexplicable from the grammar itself. The user stares at their rules, sees no ambiguity, and gets a reduce/reduce conflict — an artifact of the weaker construction, not of the language. There's nothing to fix in the grammar because the grammar is fine. This is a major reason people give up on LR parsing: the tool reports conflicts that aren't real, and the only remedies are restructuring a correct grammar to appease the algorithm or reaching for `%expect` to suppress the warning.

In our framing, we can mimic LALR by adding spurious reductions to all same-core states unconditionally — union all their reduce transitions. If this doesn't introduce a reduce/reduce conflict (same core, different rules on the same lookahead), subsequent Hopcroft minimization produces an LALR-sized table. When it does introduce a conflict, LALR rejects the grammar. That's the LALR(1) ⊂ LR(1) gap.

The literature has two attempts to bridge this gap: minimal LR (Pager's algorithm) and IELR. Both try to merge states like LALR but avoid introducing conflicts. Minimal LR merges states during construction and splits them apart when merging would create a conflict — however, it has a subtle problem: when the grammar has *real* conflicts (which are common — think operator precedence), the merge-and-split process can produce a different parser than canonical LR(1) with the same conflict resolution applied. IELR fixes this by precisely tracking which lookaheads are "contributions" from merging versus original, so it can distinguish real conflicts from artifacts — but the algorithm is correspondingly more involved.

Both are specialized procedures that, like LALR, construct the smaller automaton directly. They interleave state construction with merging decisions, requiring purpose-built algorithms that don't decompose into reusable components.

### The dead simple version

The NFA → DFA → minimize framework yields a simpler approach that seems to have escaped widespread attention, despite being obvious in hindsight. The pipeline has three steps:

1. **Detect and resolve conflicts.** After subset construction, identify shift/reduce and reduce/reduce conflicts in the raw DFA. Analyze them (see section 2), then resolve them (shift wins for S/R, lowest-numbered rule wins for R/R). This produces a clean, conflict-free automaton.

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

The result is correct by construction. Step 1 starts from the canonical LR(1) automaton — the gold standard — and resolves conflicts there. Steps 2 and 3 only add spurious reductions (which don't affect valid parses) and merge states that are now identical. At no point can the pipeline introduce a false conflict or change the parse of a valid input. There's nothing to prove — each step trivially preserves the LR(1) parse.

The result achieves LALR-level compression for grammars that are LALR, and gracefully keeps states distinct where LALR would reject the grammar. Every step is a standard, independently correct algorithm — no custom merging heuristics.

## 2. "Conflict messages are useless"

Consider the dangling else — the most famous shift/reduce conflict in parsing. The grammar says:

```
stmt = IF COND THEN stmt ELSE stmt
     | IF COND THEN stmt
     | other;
```

Bison reports:

```
State 5

    1 stmt: IF COND THEN stmt . ELSE stmt
    2     | IF COND THEN stmt .

    ELSE  shift, and go to state 6

    ELSE  [reduce using rule 2 (stmt)]
```

This tells you the state number, the terminal, and the two items involved. If you already know LR parsing intimately, you can reconstruct what's going on. If you don't — and most users don't — it's a wall of notation. What input causes this? Which interpretation is "right"? Should you fix the grammar or suppress the warning?

The fundamental problem is that conflicts are reported in terms of parser internals (states, items) rather than in terms of the grammar's language (input strings, parse trees).

### Concrete examples from the DFA

Gazelle shows the items *and* an example input for every conflict:

```
Shift/reduce conflict on 'ELSE':
  Shift:  stmt -> IF COND THEN stmt • ELSE stmt (wins)
  Reduce: stmt -> IF COND THEN stmt •
  Example: IF COND THEN IF COND THEN stmt • ELSE stmt
  Shift:  IF COND THEN IF COND THEN (stmt ELSE stmt)
  Reduce: IF COND THEN (IF COND THEN stmt) ELSE stmt (reduce to stmt)
```

The first two lines are the same items bison would show — which rules are in play and where the dot is. But below that is a concrete input: a nested `if` where the `else` is ambiguous. The bullet marks where the parser is — it has just seen the inner `IF COND THEN stmt` and the next token is `ELSE`. It can either shift (attach `ELSE` to the inner `if`) or reduce (close the inner `if`, attaching `ELSE` to the outer one). The brackets show the two resulting parse trees.

This is immediately actionable. You can see it's about which `if` owns the `else` — shift binds it to the nearest one, which is the standard resolution (and why shift wins). The items tell you *what* the parser is doing; the example tells you *why* it matters. No `%expect 1` needed to suppress a warning you don't understand.

### How the examples are found

The raw DFA — before conflict resolution or minimization — contains all the information needed. The algorithm has three phases:

**1. Find the shortest prefix to the conflict state.** BFS from the initial DFA state, following only transitions between item-bearing states (skip reduce nodes and virtual symbols). This gives the shortest sequence of grammar symbols that drives the parser to the conflict point. For the dangling else, the prefix is `IF COND THEN IF COND THEN stmt`.

**2. Simulate both interpretations.** The prefix is replayed through the DFA to reconstruct the full parser state (stack and current state). From there, two configs are created: one that shifts the conflict terminal, one that reduces by the conflict rule and then shifts. These represent the two different futures the parser could take.

**3. Find a joint suffix.** BFS over pairs of parser configs `(shift_config, reduce_config)`, feeding the *same* token to both at each step. When both configs can reach acceptance on the same remaining input, that suffix completes the example. The result is a single input string that has two valid parses — a concrete witness to the ambiguity.

The paired BFS is the key idea. By advancing both configs in lockstep on the same symbol, any suffix found is guaranteed to be a valid continuation for *both* interpretations. The search is bounded (10,000 states) to keep construction fast.

### When a single string isn't enough

Grammar ambiguity is undecidable in general — not every conflict has a single string with two parses reachable within the search budget. When the joint BFS doesn't find one, Gazelle falls back to independent suffixes: one completing string for the shift interpretation, another for the reduce:

```
Shift example:  prefix • T suffix₁
    prefix (sym T suffix₁)
Reduce example: prefix • T suffix₂
    prefix (reduced) T suffix₂ (reduce to X)
```

Two inputs that look identical up to and including the conflict lookahead but need different parser actions — making it clear why the grammar isn't LR(1).

### Reduce/reduce conflicts

For reduce/reduce conflicts — two rules want to reduce on the same lookahead — Gazelle shows the prefix with brackets marking what each rule would consume:

```
Reduce/reduce conflict on 'a':
  Example: b e • a
  Reduce 1: b (e) a [reduce to ee]
  Reduce 2: b (e) a [reduce to f]
```

The same input `e` is being claimed by two different rules. You can immediately see whether the grammar is genuinely ambiguous or whether restructuring would disambiguate it.

## 3. "Expression grammars explode"

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

Every precedence level gets its own non-terminal. Every non-terminal chains to the next. C11 has 15 levels of operator precedence — that's 15 non-terminals, each forwarding to the next, encoding what is logically a flat table of (operator, precedence, associativity) triples.

Yacc offers an escape hatch: write an ambiguous non-terminal and bolt on `%left`/`%right` declarations:

```yacc
%left '+' '-'
%left '*' '/' '%'
%right '^'

%%
expr : expr '+' expr
     | expr '-' expr
     | expr '*' expr
     | expr '/' expr
     | expr '%' expr
     | expr '^' expr
     | NUM
     ;
```

This collapses the ladder to one non-terminal, but precedence is still declared statically in the grammar file, separately from where tokens are defined. And it only works for operators with known, fixed precedence — user-defined operators are out of reach.

### Runtime precedence

Gazelle takes the idea further. Move precedence out of the grammar *and* out of the table, into the tokens themselves:

```
expr = expr OP expr => binop | term => term;
```

One rule replaces the entire precedence ladder. Precedence lives where it belongs — on the tokens at runtime:

```rust
'+' => Terminal::Op('+', Precedence::Left(6)),
'-' => Terminal::Op('-', Precedence::Left(6)),
'*' => Terminal::Op('*', Precedence::Left(7)),
'/' => Terminal::Op('/', Precedence::Left(7)),
'^' => Terminal::Op('^', Precedence::Right(8)),
```

### The conflict

The rule `expr = expr OP expr` is ambiguous. `1 + 2 * 3` has two parses:

```
  1 + (2 * 3)       (1 + 2) * 3
```

In a standard LR parser, this ambiguity is a shift/reduce conflict — an error during table construction. Yacc's `%left`/`%right` resolves it statically. Gazelle does something different: leave the conflict in the table. Instead of choosing shift or reduce at construction time, the table stores a `ShiftOrReduce` entry that defers the decision to runtime.

### How prec terminals enter the NFA

Without special treatment, prec terminals would create ordinary shift/reduce conflicts in the DFA. A complete item `[expr → expr OP expr •, OP]` and a shift item `[expr → expr • OP expr]` both transition on `OP` in the NFA. Subset construction follows both, producing a DFA state that mixes reduce nodes with regular item nodes — a shift/reduce conflict. The `resolve_conflicts` pass would need a special case: "this is a prec terminal, don't resolve it, keep both options." That's a special case in the middle of the pipeline.

The trick: each `prec` terminal gets a **virtual symbol** used exclusively for reduce transitions. When item `[A → α •, OP]` is complete and the lookahead `OP` is a prec terminal, the NFA transition uses the virtual symbol instead of the real terminal ID. Shift transitions still use the real ID.

Now prec terminals never create conflicts in the DFA at all. Shift transitions go on one edge (real ID), reduce transitions go on another (virtual ID). Subset construction, conflict resolution, spurious reduction merging, Hopcroft minimization — none of them need to know about prec terminals.

At the end, `build_table_from_dfa` merges the two edges back: if a state has both a shift edge (real ID) and a reduce edge (virtual ID) for the same prec terminal, the table entry becomes `ShiftOrReduce { shift_state, reduce_rule }`. If only one exists, it's a plain `Shift` or `Reduce`. The virtual symbols disappear — they were scaffolding for the pipeline.

The only prec-specific code is at the boundaries: symbol splitting before the NFA, merging after the DFA. Everything in between is generic.

### Runtime resolution

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

### Shunting-yard meets LR

If this reminds you of Pratt parsing, you're not wrong — but Pratt is the recursive formulation of Dijkstra's shunting-yard algorithm, a technique specifically for expressions. You typically embed a Pratt parser inside a larger recursive descent parser for the full language.

What we're doing here is different: it's the natural union of shunting-yard and canonical LR. The LR automaton handles the full grammar — statements, declarations, type expressions, everything. When it hits an expression with precedence conflicts, it resolves them shunting-yard style, comparing precedence values at runtime. You get the generality of LR with the elegance of shunting-yard for the parts that need it. Yacc's `%left`/`%right` is a static version of this — same idea, but baked into the table at construction time. Deferring the resolution to runtime is a simple extension that doesn't seem to have been explored.

### User-defined operators

Because precedence comes from tokens at runtime, user-defined operators fall out naturally:

```
> operator @ 14 left    // declare @ as precedence 14, left-associative
> 2 + 3 @ 4
14
```

The parser doesn't change. The lexer consults a table of known operators, and that table can grow during execution. When the parser reduces `operator @ 14 left`, a semantic action registers the new operator. Subsequent tokens include `@` with its precedence. No grammar changes, no parser regeneration.

This is how languages like Haskell handle user-defined operators. But traditionally they've required special parsing techniques — a Pratt parser for expressions, or post-parse tree rotation. With runtime precedence in an LR parser, it falls out of the existing mechanism.

### Precedence-carrying non-terminals

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
          | STAR => mul
          | AMP => bitand
          | PLUS => add
          | MINUS => sub;

// Unary rules use the same prec terminals directly
term = NUM => num
     | LPAREN expr RPAREN => paren
     | STAR term => deref
     | AMP term => addr
     | PLUS term => pos
     | MINUS term => neg;

expr = term => term
     | expr binary_op expr => binop;
```

The lexer sends `STAR` with precedence regardless of context — the grammar sorts out which use applies. In a unary rule like `STAR term`, the grammar consumes it structurally — no precedence ambiguity. In a binary expression, `STAR` reduces to `binary_op` and its precedence flows through. The question is how precedence propagates through reductions. The `do_reduce` function manages this:

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
3. **Multi-symbol reductions** (like `expr binary_op expr` or `MINUS term`) use the *anchor's* precedence — the stack entry that was waiting before this sub-expression began. This correctly resets the precedence context: after reducing `2 * 3` to `expr`, the precedence should reflect the context *around* `2 * 3`, not the `*` inside it.

Unary operators don't interact with runtime precedence at all — they're handled structurally by the grammar. In `2 * -3 + 4`, the parser shifts `MINUS` and then `3`, reduces `MINUS 3` to `term` (Rule 3 resets to the anchor's precedence — `*`'s Left(12)), then reduces `term` to `expr`. Now `+` (Left(6)) sees `*`'s precedence on the stack, correctly reduces `2 * (-3)` first, then adds `4`.

## 4. "Grammars are polluted with action code"

An LR automaton by itself only accepts or rejects. To extract meaning, you need semantic actions. Yacc's answer is to embed them in the grammar:

```yacc
expr : expr '+' expr { $$ = $1 + $3; }
```

Now the grammar is a program in a bespoke language — `$$` and `$1` notation, no type safety, C fragments mixed into the specification. It can't be analyzed, reformatted, or used to generate documentation independently of the embedded code. Refactoring the grammar means refactoring the actions. The specification and the program are fused.

Parser combinators go the other direction: the code *is* the grammar. There's no separate specification at all. You gain composability and type safety but lose the grammar as an artifact.

A third approach — building a generic syntax tree with type-erased nodes — avoids the entanglement but just moves the problem downstream. You get an untyped tree and pattern-match your way through it, with no compiler help when the grammar changes.

### Trait-based semantics

Gazelle's answer is to use Rust's trait system to separate grammar structure from semantic interpretation. The grammar is declarative:

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

### Why this matters

No other parser generator I'm aware of parameterizes the generated node types this way. lalrpop has typed actions but they're inline in the grammar. Tree-sitter gives you an untyped CST. Bison's `%type` declarations are per-symbol but monomorphic — each symbol has one fixed type.

By making the AST representation generic over the consumer, you get:

- **Multiple backends from one grammar.** Write an interpreter, a compiler, a pretty-printer, a linter — each is just a different `Types` impl. No grammar duplication.
- **Full IDE support in action code.** Autocomplete, type hints, go-to-definition — it's all plain Rust, not a code fragment inside a grammar file.
- **Compile errors point to your code,** not to generated code. When a type doesn't match, the error says your `Action::build` returns the wrong type, not that line 4723 of a generated file has a mismatch.
- **The grammar remains a readable artifact.** No code noise, just structure and action labels. You can analyze it, generate documentation from it, diff it across versions.

### Blanket implementations and the CST–AST continuum

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

This makes the CST/AST distinction a smooth continuum rather than a binary choice. Auto-box some nodes for a full tree, evaluate others inline, set yet others to `Ignore` to discard them entirely. The same grammar, the same generated enums — the representation depends entirely on what types you plug in.

## 5. "The parser and lexer can't talk to each other"

Traditional parser generators use a pull model: the parser calls the lexer for the next token. Gazelle uses a push-based API — you own the loop, you feed tokens to the parser:

```rust
for token in lex(&input) {
    parser.push(token, &mut actions)?;
}
parser.finish(&mut actions)?;
```

The generated parser has just two methods: `push` (feed a token) and `finish` (signal end of input). Internally it's an LR state machine — shift, reduce, goto — but the user never sees that. Push-based parsing isn't new — bison has supported it since version 2.4. Making it the only mode is a deliberate choice: it simplifies the entire design and makes lexer feedback the natural way to write things rather than a special configuration.

### Lexer feedback

The lexer and parser are completely independent components. Neither calls the other. The user writes the loop that connects them — and that loop can do whatever it wants between lexing a token and pushing it. This is where context-sensitive behavior lives, in ordinary user code, not in hooks or callbacks baked into the tools.

The most famous instance is C's "lexer hack." In C, `T * x;` is a multiplication expression if `T` is a variable, or a pointer declaration if `T` is a typedef. The parser tracks declarations, but the lexer needs to know about them to emit the right tokens.

Jacques-Henri Jourdan and François Pottier found a clean formulation for their verified C11 parser (["A Simple, Possibly Correct LR Parser for C11"](https://dl.acm.org/doi/10.1145/3064848)). Their insight: augment the token stream and the grammar to make context explicit. When the lexer sees an identifier, it emits two tokens: `NAME` (the string) followed by `TYPE` or `VARIABLE` (based on the current typedef table). The grammar has separate rules for each case:

```
typedef_name = NAME TYPE;
var_name = NAME VARIABLE;
```

With a push-based parser, the coordination lives in ordinary user code:

```rust
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

Push-based parsers compose. Since neither parser owns the control flow, one parser's semantic actions can drive another. In Gazelle's `runtime_grammar` example, a compiled parser handles the token stream format, and its reductions push tokens into a runtime grammar parser:

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

## Error reporting and recovery

Parsing theory is about recognizing valid inputs. Error handling — what happens when the input is *invalid* — tends to get far less attention in the literature and in parser generator tooling. But in practice, error reporting is what determines whether a parser is usable. A compiler that says "syntax error on line 42" and stops is barely better than one that crashes.

It's also genuinely hard. The best error messages require semantic understanding — "did you mean `==`?" or "missing return type" — which a parser generator doesn't have. Handwritten parsers *can* produce these, but at the cost of cataloging failure patterns one by one.

A parser generator can't match that level of tailored diagnostics. But it has an advantage that handwritten parsers and parser combinators fundamentally lack: the full grammar and the full parse state are available as regular data. In a recursive descent parser, the parse state is the host language's call stack — opaque, not inspectable, not cloneable. You can't ask "what tokens would be valid here?" without manually maintaining that information. An LR parser with an explicit stack can answer that question directly from the parse table, and it can clone the parser to simulate speculative repairs.

### Error messages

The parse table knows exactly which tokens are valid in each state. When the parser encounters an unexpected token, it can report what it expected instead. But a flat list of terminal names — "expected IDENT, NUM, LPAREN, MINUS, STAR, AMP" — isn't very helpful. It's technically correct but gives no context.

The parser has richer information: the LR items active in the current state tell you which rules are in progress and where the dot is. Gazelle uses these to produce structured error messages. Given `S = a b c` and input `a x`:

```
unexpected 'x', expected: b
  after: a
  in S: a • b c
```

Three layers of information:

1. **What went wrong**: the unexpected token and what was expected instead. Instead of listing raw terminals, the expected set is computed from the items' dot positions. If the dot is before a non-terminal like `expr`, the message says "expected expr" rather than enumerating every terminal that can start an expression.

2. **Where we are**: what has been parsed so far, shown as the grammar symbols on the stack.

3. **What rule is in progress**: the active item with its dot position, showing exactly where the parser is within the rule. Here `a • b c` means "matched `a`, expecting `b` next."

The expected set computation traces through nullable non-terminals (if `B` is optional and followed by `C`, both should appear as expected) and chases complete items back through the stack to find the calling context. This is what makes error messages reflect the actual parsing context rather than the raw state number.

### Error recovery

Stopping at the first error is hostile. A parser that can recover and report multiple errors per file is far more useful.

The standard approach in yacc-family tools is `error` productions: you add rules like `stmt → error ';'` that match "skip everything until a semicolon." This works but it's manual — you have to anticipate where errors occur and what to skip to. Miss a case and the parser cascades into nonsense.

Gazelle's recovery is automatic. It uses Dijkstra's algorithm to search for the minimum-cost edit that gets the parser back on track. The search space has three operations:

- **Insert** a token (cost 1) — the parser acts as if a token appeared that wasn't in the input.
- **Delete** the current token (cost 1) — skip it and advance the input.
- **Shift** the current token (cost 0) — consume it normally.

The parser clones itself for each candidate repair, simulating the edit without modifying the real state. Dijkstra guarantees we explore lowest-cost repairs first. A repair is accepted when the parser can consume all remaining tokens and reach acceptance — the search finds the minimum-cost edit sequence that makes the entire input valid.

Walk through a concrete example. Given a C-like grammar and the input `x = 1 + + 2;` (stray `+`):

| Step | Parser state | Input remaining | Action | Cost |
|------|-------------|-----------------|--------|------|
| 0 | `x = 1 +` | `+ 2 ;` | Error: unexpected `+` | |
| 1 | Try delete `+` | `2 ;` | Shift `2`, reduce, shift `;`, accept — all tokens consumed | cost 1 |
| 1' | Try insert `NUM` | `+ 2 ;` | Shift `NUM`, reduce, shift `+`, shift `2`, reduce, shift `;`, accept | cost 1 |

Both repairs have cost 1. The search finds delete-`+` first (fewer tokens to process). The parser resynchronizes and continues, reporting: "deleted unexpected '+'".

Typical repairs: inserting a missing semicolon (cost 1), deleting a stray token (cost 1), or inserting a closing brace and semicolon (cost 2). The search finds these automatically from the grammar — no `error` productions needed.

The search is bounded to keep recovery fast. In practice, inserting or deleting one or two tokens covers the vast majority of real errors.

---

A handwritten recursive descent parser will always be more capable. You can craft error messages for specific situations ("did you mean `==`?"), implement incremental reparsing for IDE responsiveness, handle context-sensitive hacks that no grammar formalism supports cleanly (C++ template angle brackets, contextual keywords). That flexibility costs engineering time — a handwritten parser for a real language is thousands of lines of careful code, and the error handling alone can dwarf the happy path.

Gazelle provides a readable grammar, structured error messages, automatic recovery, and type-safe semantic actions. It won't match the polish of a dedicated handwritten parser, but it provides a solid baseline that requires non-trivial engineering to match.

This is [Gazelle](https://github.com/gerben-stavenga/gazelle), available as [`gazelle-parser`](https://crates.io/crates/gazelle-parser) on crates.io.

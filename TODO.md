# Gazelle TODO

- Automatic grammar rewrites for conflict resolution: for unambiguous grammars
  (fake LR(1) conflicts), automatically rewrite the grammar to be LR(1) while
  keeping the original grammar as the user-facing AST and API. CPS-style inlining
  is one instance, but the mechanism should be more general — e.g. counting
  constraints (C11 declaration specifiers), interleaved lists, etc. The user
  writes what they mean, gazelle makes it work.

- Expose shift/reduce token variants to users: instead of runtime precedence
  tracking, double conflicted tokens into shift-leaning and reduce-leaning
  variants. The user/lexer picks which to push. Handles operator precedence,
  dangling-else, and any shift/reduce conflict with one mechanism and zero
  runtime logic. Replaces the prec field and comparison logic in maybe_reduce.


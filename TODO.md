# Gazelle TODO

Based on real-world usage feedback (replacing a 1364-line winnow parser with 150 lines of grammar).

## Diagnostics

- [x] **Conflict error messages** - Conflicts now show detailed error messages:
  - Messages show: state number, terminal, full rule (e.g., "expr -> expr + expr")
  - `expect N rr;` / `expect N sr;` syntax to declare expected conflicts
  - Only errors if actual count differs from expected

- [ ] **Debug dump (GAZELLE_DEBUG)** - Add env var to dump LR items during table construction. Shows items with dot position (e.g., `expr -> expr • + expr`).

- [ ] **Grammar visualization** - Dump computed FIRST/FOLLOW sets and LR items as text. Helps debug grammar ambiguities.

- [x] **Better proc-macro errors** - Replace opaque panics with clear error messages:
  - Unknown symbol references: "Unknown symbol: X"
  - Conflict errors with full rule details
  - Audited all panic paths - remaining ones are internal invariants

## Features

- [ ] **List pattern syntax** - Comma-separated lists are extremely common. Add sugar like `item % COMMA` (tree-sitter style) that expands to the standard recursive pattern. Reduces boilerplate.

## Documentation

- [x] **Grammar macro reference** - See `docs/reference.md` for complete documentation covering grammar syntax, the `grammar!` macro, generated types, parser usage, and advanced features.

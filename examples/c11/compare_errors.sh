#!/bin/bash
# Compare error recovery output: Gazelle vs GCC vs Clang
#
# Usage: bash examples/c11/compare_errors.sh
#        bash examples/c11/compare_errors.sh examples/c11/error_cases/missing_semi.c

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Build once
cargo build --example c11 --manifest-path="$ROOT/Cargo.toml" 2>/dev/null

GAZELLE="$ROOT/target/debug/examples/c11"

if [ $# -gt 0 ]; then
    FILES=("$@")
else
    FILES=("$SCRIPT_DIR"/error_cases/*.c)
fi

for file in "${FILES[@]}"; do
    name=$(basename "$file")
    # Print the source
    echo "================================================================"
    echo "  $name"
    echo "================================================================"
    head -1 "$file"  # comment describing the error
    echo "---"
    tail -n +2 "$file" | cat -n
    echo

    echo "--- Gazelle ---"
    "$GAZELLE" "$file" 2>&1 || true
    echo

    echo "--- GCC ---"
    gcc -fsyntax-only "$file" 2>&1 || true
    echo

    echo "--- Clang ---"
    clang -fsyntax-only "$file" 2>&1 || true
    echo
    echo
done

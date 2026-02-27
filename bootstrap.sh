#!/bin/sh
# Regenerate meta_generated.rs and regex_generated.rs from scratch.
#
# Step 1: Build with bootstrap (no meta, no regex). Use --bootstrap-meta
#   which has the meta grammar hardcoded in main.rs.
# Step 2: Build with bootstrap_regex (meta available, no regex).
#   parse_grammar can now parse regex.gzl.
# Step 3: Full build to regenerate meta_generated.rs from meta.gzl
#   (replacing the bootstrap version with the canonical one).
#   Must match step 1 output — verifies the hardcoded grammar is in sync.
set -e
cargo run -q --features bootstrap,codegen -- --bootstrap-meta > /tmp/meta_generated.rs
mv /tmp/meta_generated.rs src/meta_generated.rs
cargo run -q --features bootstrap_regex,codegen -- --rust grammars/regex.gzl > /tmp/regex_generated.rs
mv /tmp/regex_generated.rs src/regex_generated.rs
cargo run -q --features codegen -- --rust grammars/meta.gzl > /tmp/meta_generated.rs
diff -q src/meta_generated.rs /tmp/meta_generated.rs || {
    echo "ERROR: --bootstrap-meta output differs from grammars/meta.gzl output"
    echo "The hardcoded grammar in main.rs is out of sync with grammars/meta.gzl"
    exit 1
}
mv /tmp/meta_generated.rs src/meta_generated.rs

//! Verify that bootstrap.sh can regenerate both generated files from scratch.
//!
//! This test deletes `src/meta_generated.rs` and `src/regex_generated.rs`,
//! runs `bootstrap.sh`, and checks that the files are restored identically.
//!
//! Ignored by default since it's slow (multiple cargo builds).
//! Run with: cargo test --test bootstrap_from_scratch --features codegen -- --ignored

use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
#[ignore]
fn bootstrap_restores_generated_files() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let meta_path = dir.join("src/meta_generated.rs");
    let regex_path = dir.join("src/regex_generated.rs");

    // Save originals
    let meta_original = fs::read_to_string(&meta_path).expect("read meta_generated.rs");
    let regex_original = fs::read_to_string(&regex_path).expect("read regex_generated.rs");

    // Delete both
    fs::remove_file(&meta_path).expect("delete meta_generated.rs");
    fs::remove_file(&regex_path).expect("delete regex_generated.rs");

    // Run bootstrap.sh
    let output = Command::new("bash")
        .arg("bootstrap.sh")
        .current_dir(dir)
        .output()
        .expect("failed to run bootstrap.sh");

    // Restore originals on failure before panicking
    let restore = || {
        let _ = fs::write(&meta_path, &meta_original);
        let _ = fs::write(&regex_path, &regex_original);
    };

    if !output.status.success() {
        restore();
        panic!(
            "bootstrap.sh failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Read regenerated files
    let meta_new = match fs::read_to_string(&meta_path) {
        Ok(s) => s,
        Err(e) => {
            restore();
            panic!("meta_generated.rs not restored: {}", e);
        }
    };
    let regex_new = match fs::read_to_string(&regex_path) {
        Ok(s) => s,
        Err(e) => {
            restore();
            panic!("regex_generated.rs not restored: {}", e);
        }
    };

    // Compare
    let meta_matches = meta_new == meta_original;
    let regex_matches = regex_new == regex_original;

    // Restore originals (bootstrap may have produced them, but restore to be safe)
    restore();

    if !meta_matches {
        panic!("bootstrap.sh produced different meta_generated.rs");
    }
    if !regex_matches {
        panic!("bootstrap.sh produced different regex_generated.rs");
    }
}

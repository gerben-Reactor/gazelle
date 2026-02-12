use std::path::Path;
use std::process::Command;

/// Tests that fail the full pipeline. All other tests are expected to pass.
///
/// parse (3): missing syntax support
///   00213, 00214, 00216
///
/// typecheck (2):
///   00204 — missing __builtin_va_start
///   00219 — _Generic + function pointer typedef
///
/// crash/SIGSEGV (13): struct/pointer codegen bugs
///   00045, 00046, 00078, 00089, 00149, 00150, 00158, 00170, 00174, 00182,
///   00189, 00193, 00196
///
/// wrong exit code (10): codegen bugs (type conversions, control flow, etc.)
///   00048, 00049, 00050, 00092, 00093,
///   00117, 00118, 00124, 00143, 00147, 00148
///
/// wrong stdout (7): codegen bugs (sizeof=0, array init, etc.)
///   00132, 00175, 00179, 00185, 00197, 00205, 00220
const FAILING: &[&str] = &[
    // parse
    "00213.c", "00214.c", "00216.c",
    // typecheck
    "00204.c", "00219.c",
    // crash
    "00078.c", "00149.c", "00150.c",
    "00170.c", "00174.c", "00182.c",
    // wrong stdout
    "00220.c",
];

fn compile_and_run(c_path: &Path) -> Result<(i32, String), String> {
    let unit = c11::parse_file(c_path).map_err(|e| format!("parse: {}", e))?;
    let unit = c11::typecheck::check(unit).map_err(|e| format!("typecheck: {}", e))?;
    let asm = c11::codegen::codegen(&unit);

    let stem = c_path.file_stem().unwrap().to_str().unwrap();
    let asm_path = std::env::temp_dir().join(format!("c11test_{}.s", stem));
    let bin_path = std::env::temp_dir().join(format!("c11test_{}", stem));

    std::fs::write(&asm_path, &asm).map_err(|e| format!("write asm: {}", e))?;

    let status = Command::new("cc")
        .args([
            asm_path.to_str().unwrap(),
            "-o", bin_path.to_str().unwrap(),
            "-no-pie", "-lm",
        ])
        .output()
        .map_err(|e| format!("cc: {}", e))?;
    if !status.status.success() {
        let _ = std::fs::remove_file(&asm_path);
        return Err(format!("cc failed: {}", String::from_utf8_lossy(&status.stderr)));
    }

    let output = Command::new("timeout")
        .args(["10", bin_path.to_str().unwrap()])
        .output()
        .map_err(|e| format!("run: {}", e))?;

    let _ = std::fs::remove_file(&asm_path);
    let _ = std::fs::remove_file(&bin_path);

    let code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok((code, stdout))
}

#[test]
fn test_codegen() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/c-testsuite/tests/single-exec");

    let mut passed = 0;
    let mut failed = 0;
    let mut unexpected_failures = Vec::new();
    let mut unexpected_passes = Vec::new();

    let mut entries: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "c"))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in &entries {
        let name = entry.file_name();
        let name = name.to_str().unwrap();
        let expected_fail = FAILING.contains(&name);

        let expected_output = {
            let exp_path = entry.path().with_extension("c.expected");
            std::fs::read_to_string(&exp_path).unwrap_or_default()
        };

        let ok = match compile_and_run(&entry.path()) {
            Ok((0, stdout)) if stdout == expected_output => true,
            _ => false,
        };

        if ok {
            passed += 1;
            if expected_fail {
                unexpected_passes.push(name.to_string());
            }
        } else {
            failed += 1;
            if !expected_fail {
                unexpected_failures.push(name.to_string());
            }
        }
    }

    eprintln!("{} passed, {} failed, {} total",
        passed, failed, entries.len());

    for e in &unexpected_failures {
        eprintln!("UNEXPECTED FAIL: {}", e);
    }
    for p in &unexpected_passes {
        eprintln!("UNEXPECTED PASS (remove from FAILING): {}", p);
    }

    assert!(unexpected_failures.is_empty() && unexpected_passes.is_empty(),
        "{} unexpected failures, {} unexpected passes",
        unexpected_failures.len(), unexpected_passes.len());
}

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn compile_and_run(c_path: &Path) -> Result<i32, String> {
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let source = std::fs::read_to_string(c_path).map_err(|e| format!("read: {}", e))?;
    let unit = c11::parse(&source).map_err(|e| format!("parse: {}", e))?;
    let unit = c11::typecheck::check(unit).map_err(|e| format!("typecheck: {}", e))?;
    let asm = c11::codegen::codegen(&unit);

    let asm_path = std::env::temp_dir().join(format!("torture_{}.s", id));
    let bin_path = std::env::temp_dir().join(format!("torture_{}", id));

    std::fs::write(&asm_path, &asm).map_err(|e| format!("write asm: {}", e))?;

    let output = Command::new("cc")
        .args([
            asm_path.to_str().unwrap(),
            "-o", bin_path.to_str().unwrap(),
            "-no-pie", "-lm",
        ])
        .output()
        .map_err(|e| format!("cc: {}", e))?;
    let _ = std::fs::remove_file(&asm_path);
    if !output.status.success() {
        return Err(format!("assemble: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let output = Command::new("timeout")
        .args(["5", bin_path.to_str().unwrap()])
        .output()
        .map_err(|e| format!("run: {}", e))?;
    let _ = std::fs::remove_file(&bin_path);

    Ok(output.status.code().unwrap_or(-1))
}

#[test]
fn test_gcc_torture() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/gcc-torture-pp");

    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "c"))
        .collect();
    files.sort();

    let total = files.len();
    let pass = AtomicUsize::new(0);
    let parse_fail_n = AtomicUsize::new(0);
    let tc_fail_n = AtomicUsize::new(0);
    let asm_fail_n = AtomicUsize::new(0);
    let run_fail_n = AtomicUsize::new(0);
    let failures: std::sync::Mutex<Vec<(String, String)>> = std::sync::Mutex::new(Vec::new());

    std::thread::scope(|s| {
        let nthreads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(8);
        let chunk_size = (files.len() + nthreads - 1) / nthreads;
        for chunk in files.chunks(chunk_size) {
            let pass = &pass;
            let parse_fail_n = &parse_fail_n;
            let tc_fail_n = &tc_fail_n;
            let asm_fail_n = &asm_fail_n;
            let run_fail_n = &run_fail_n;
            let failures = &failures;
            s.spawn(move || {
                for path in chunk {
                    let name = path.file_name().unwrap().to_str().unwrap().to_string();
                    let result = std::panic::catch_unwind(|| compile_and_run(path));
                    let result = match result {
                        Ok(r) => r,
                        Err(_) => Err("typecheck: panic".to_string()),
                    };
                    match result {
                        Ok(0) => { pass.fetch_add(1, Ordering::Relaxed); }
                        Ok(code) => {
                            run_fail_n.fetch_add(1, Ordering::Relaxed);
                            failures.lock().unwrap().push((name, format!("exit {}", code)));
                        }
                        Err(e) if e.starts_with("parse:") => {
                            parse_fail_n.fetch_add(1, Ordering::Relaxed);
                            failures.lock().unwrap().push((name, e));
                        }
                        Err(e) if e.starts_with("typecheck:") => {
                            tc_fail_n.fetch_add(1, Ordering::Relaxed);
                            failures.lock().unwrap().push((name, e));
                        }
                        Err(e) => {
                            asm_fail_n.fetch_add(1, Ordering::Relaxed);
                            failures.lock().unwrap().push((name, e));
                        }
                    }
                }
            });
        }
    });

    let p = pass.load(Ordering::Relaxed);
    let pf = parse_fail_n.load(Ordering::Relaxed);
    let tf = tc_fail_n.load(Ordering::Relaxed);
    let af = asm_fail_n.load(Ordering::Relaxed);
    let rf = run_fail_n.load(Ordering::Relaxed);
    eprintln!("GCC torture: {} total, {} pass, {} parse-fail, {} typecheck-fail, {} asm-fail, {} run-fail",
        total, p, pf, tf, af, rf);

    let mut failures = failures.into_inner().unwrap();
    failures.sort();
    for (name, err) in &failures {
        let short = if err.len() > 120 { &err[..120] } else { err.as_str() };
        eprintln!("  {}: {}", name, short);
    }
}

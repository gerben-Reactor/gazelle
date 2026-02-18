use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Parse flags like cc: -c (compile only), -o <output>, -S (emit asm)
    // Ignore unknown flags (e.g. -ggdb3, -Os, -Wall, -D*, -W*)
    let mut input_path = None;
    let mut output_path = None;
    let mut compile_only = false;
    let mut emit_asm = false;
    let mut link_libs = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-c" => compile_only = true,
            "-S" => emit_asm = true,
            "-o" => { i += 1; output_path = Some(args[i].clone()); }
            s if s.starts_with("-l") => link_libs.push(s.to_string()),
            s if s.starts_with('-') => {} // ignore other flags
            s => input_path = Some(s.to_string()),
        }
        i += 1;
    }

    let input_str = match &input_path {
        Some(p) => p.as_str(),
        None => {
            eprintln!("Usage: c11 [flags] <input.c>");
            std::process::exit(1);
        }
    };
    let input = Path::new(input_str);
    let stem = input.file_stem().unwrap().to_str().unwrap();

    let unit = match c11::parse_file(input) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("{}: {}", input_str, e);
            std::process::exit(1);
        }
    };
    let unit = match c11::typecheck::check(unit) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("{}: {}", input_str, e);
            std::process::exit(1);
        }
    };

    let asm = c11::codegen::codegen(&unit);

    if emit_asm {
        let out = output_path.unwrap_or_else(|| format!("{}.s", stem));
        std::fs::write(&out, &asm).unwrap();
        return;
    }

    if compile_only {
        // -c: compile to .o via temp .s
        let asm_path = std::env::temp_dir().join(format!("{}.s", stem));
        std::fs::write(&asm_path, &asm).unwrap();
        let out = output_path.unwrap_or_else(|| format!("{}.o", stem));
        let status = Command::new("cc")
            .args(["-c", asm_path.to_str().unwrap(), "-o", &out])
            .status()
            .expect("failed to run cc");
        let _ = std::fs::remove_file(&asm_path);
        if !status.success() {
            std::process::exit(1);
        }
        return;
    }

    // Default: compile and link
    let asm_path = std::env::temp_dir().join(format!("{}.s", stem));
    std::fs::write(&asm_path, &asm).unwrap();
    let out = output_path.unwrap_or_else(|| stem.to_string());
    let mut cmd = Command::new("cc");
    cmd.args([asm_path.to_str().unwrap(), "-o", &out, "-no-pie"]);
    for lib in &link_libs {
        cmd.arg(lib);
    }
    let status = cmd.status().expect("failed to run cc");
    let _ = std::fs::remove_file(&asm_path);
    if !status.success() {
        std::process::exit(1);
    }
}

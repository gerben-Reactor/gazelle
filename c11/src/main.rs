use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: c11 <input.c>");
        std::process::exit(1);
    }
    let input = Path::new(&args[1]);
    let unit = match c11::parse_file(input) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };
    let unit = match c11::typecheck::check(unit) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("type error: {}", e);
            std::process::exit(1);
        }
    };

    let asm = c11::codegen::codegen(&unit);

    let stem = input.file_stem().unwrap().to_str().unwrap();
    let asm_path = format!("/tmp/{}.s", stem);
    let out_path = format!("/tmp/{}", stem);

    std::fs::write(&asm_path, &asm).unwrap();
    eprintln!("wrote {}", asm_path);

    let status = Command::new("cc")
        .args([&asm_path, "-o", &out_path, "-no-pie", "-lm"])
        .status()
        .expect("failed to run cc");
    if !status.success() {
        eprintln!("cc failed");
        std::process::exit(1);
    }
    eprintln!("wrote {}", out_path);
}

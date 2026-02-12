use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: c11 <input.c>");
        std::process::exit(1);
    }
    match c11::parse_file(Path::new(&args[1])) {
        Ok(unit) => {
            for f in &unit.functions {
                println!("fn {} ret={:?} {:?} params={:?}", f.name, f.return_specs, f.return_derived, f.params);
                println!("  body: {:?}", f.body);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: c11 <input.c>");
        std::process::exit(1);
    }
    if let Err(e) = c11::parse_file(Path::new(&args[1])) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: c11 <input.c>");
        std::process::exit(1);
    }
    match c11::parse_file(Path::new(&args[1])) {
        Ok(unit) => {
            let unit = match c11::typecheck::check(unit) {
                Ok(u) => u,
                Err(e) => {
                    eprintln!("type error: {}", e);
                    std::process::exit(1);
                }
            };
            for f in &unit.functions {
                let ret = c11::types::resolve_type(&f.return_specs, &f.return_derived);
                let params: Vec<_> = f.params.iter()
                    .map(|p| c11::types::resolve_type(&p.specs, &p.derived))
                    .collect();
                println!("fn {} -> {:?}  params={:?}", f.name, ret, params);
                println!("  body: {:#?}", f.body);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

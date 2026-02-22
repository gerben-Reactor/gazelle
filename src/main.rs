//! Gazelle CLI - parse grammar and output tables or Rust code.
//!
//! Usage:
//!   gazelle grammar.txt          # output JSON tables
//!   gazelle --rust grammar.txt   # output Rust code
//!   gazelle < grammar.txt        # read from stdin
//!   gazelle -                     # read from stdin (explicit)

use gazelle::{parse_grammar, CompiledTable, SymbolId};
#[cfg(feature = "codegen")]
use gazelle::codegen::{self, CodegenContext};
use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut rust_mode = false;
    let mut input_file: Option<&str> = None;

    for arg in args.iter().skip(1) {
        if arg == "--rust" {
            rust_mode = true;
        } else if arg != "-" {
            input_file = Some(arg);
        }
    }

    let input = if let Some(file) = input_file {
        fs::read_to_string(file).expect("Failed to read file")
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).expect("Failed to read stdin");
        buf
    };

    if rust_mode {
        #[cfg(feature = "codegen")]
        {
            output_rust(&input);
        }
        #[cfg(not(feature = "codegen"))]
        {
            eprintln!("--rust mode requires the 'codegen' feature");
            std::process::exit(1);
        }
    } else {
        output_json(&input);
    }
}

#[cfg(feature = "codegen")]
fn output_rust(input: &str) {
    let grammar_def = match gazelle::parse_grammar(input) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    };

    let ctx = match CodegenContext::from_grammar(&grammar_def, "", "pub ", false) {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    match codegen::generate_items(&ctx) {
        Ok(tokens) => {
            let syntax_tree: syn::File = syn::parse2(tokens).expect("generated invalid code");
            let formatted = prettyplease::unparse(&syntax_tree);
            println!("{}", formatted);
        }
        Err(e) => {
            eprintln!("Codegen error: {}", e);
            std::process::exit(1);
        }
    }
}

fn output_json(input: &str) {
    let grammar = match parse_grammar(input) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    };

    let table = CompiledTable::build(&grammar);

    if table.has_conflicts() {
        for c in table.conflicts() {
            eprintln!("Conflict: {:?}", c);
        }
        std::process::exit(1);
    }

    // Output JSON
    println!("{{");

    // Action encoding documentation
    println!("  \"_action_encoding\": \"0=error, (1|state<<2)=shift, (2|rule<<2)=reduce, 3=accept\",");

    // Symbol names
    println!("  \"symbols\": [");
    let num_symbols = table.num_symbols();
    for i in 0..num_symbols {
        let name = table.symbol_name(SymbolId::new(i));
        let comma = if i + 1 < num_symbols { "," } else { "" };
        println!("    \"{}\"{}", escape_json(name), comma);
    }
    println!("  ],");

    // Terminal count (symbols 0..num_terminals are terminals)
    println!("  \"num_terminals\": {},", table.num_terminals());

    // Rules: [lhs_symbol_id, rhs_length]
    println!("  \"rules\": [");
    let rules = table.rules();
    for (i, (lhs, len)) in rules.iter().enumerate() {
        let comma = if i + 1 < rules.len() { "," } else { "" };
        println!("    [{}, {}]{}", lhs, len, comma);
    }
    println!("  ],");

    // Number of states
    println!("  \"num_states\": {},", table.num_states());

    // ACTION table (row displacement compression)
    print!("  \"action_data\": [");
    print_u32_array(table.action_data());
    println!("],");

    print!("  \"action_base\": [");
    print_i32_array(table.action_base());
    println!("],");

    print!("  \"action_check\": [");
    print_u32_array(table.action_check());
    println!("],");

    // GOTO table (row displacement compression)
    print!("  \"goto_data\": [");
    print_u32_array(table.goto_data());
    println!("],");

    print!("  \"goto_base\": [");
    print_i32_array(table.goto_base());
    println!("],");

    print!("  \"goto_check\": [");
    print_u32_array(table.goto_check());
    println!("]");

    println!("}}");
}

fn print_u32_array(arr: &[u32]) {
    for (i, v) in arr.iter().enumerate() {
        if i > 0 {
            print!(",");
        }
        print!("{}", v);
    }
}

fn print_i32_array(arr: &[i32]) {
    for (i, v) in arr.iter().enumerate() {
        if i > 0 {
            print!(",");
        }
        print!("{}", v);
    }
}

fn escape_json(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

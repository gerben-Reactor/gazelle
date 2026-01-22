//! Gazelle CLI - parse grammar and output tables for use in any language.
//!
//! Usage:
//!   gazelle grammar.txt       # read from file
//!   gazelle < grammar.txt     # read from stdin
//!   gazelle -                  # read from stdin (explicit)
//!
//! Output is JSON with parse tables usable from any language.

use gazelle::{parse_grammar, Automaton, ParseTable, SymbolId};
use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if args.len() > 1 && args[1] != "-" {
        fs::read_to_string(&args[1]).expect("Failed to read file")
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).expect("Failed to read stdin");
        buf
    };

    let grammar = match parse_grammar(&input) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    };

    let automaton = Automaton::build(&grammar);
    let table = ParseTable::build(&automaton);

    if table.has_conflicts() {
        for c in &table.conflicts {
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
    let num_symbols = grammar.symbols.num_symbols();
    for i in 0..num_symbols {
        let name = grammar.symbols.name(SymbolId(i));
        let comma = if i + 1 < num_symbols { "," } else { "" };
        println!("    \"{}\"{}", escape_json(name), comma);
    }
    println!("  ],");

    // Terminal count (symbols 0..num_terminals are terminals)
    println!("  \"num_terminals\": {},", grammar.symbols.num_terminals());

    // Rules: [lhs_symbol_id, rhs_length]
    println!("  \"rules\": [");
    for (i, (lhs, len)) in table.rules.iter().enumerate() {
        let comma = if i + 1 < table.rules.len() { "," } else { "" };
        println!("    [{}, {}]{}", lhs.0, len, comma);
    }
    println!("  ],");

    // Number of states
    println!("  \"num_states\": {},", table.num_states);

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
        if i > 0 { print!(","); }
        print!("{}", v);
    }
}

fn print_i32_array(arr: &[i32]) {
    for (i, v) in arr.iter().enumerate() {
        if i > 0 { print!(","); }
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

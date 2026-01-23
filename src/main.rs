//! Gazelle CLI - parse grammar and output tables or Rust code.
//!
//! Usage:
//!   gazelle grammar.txt          # output JSON tables
//!   gazelle --rust grammar.txt   # output Rust code
//!   gazelle < grammar.txt        # read from stdin
//!   gazelle -                     # read from stdin (explicit)

use gazelle::{parse_grammar, parse_grammar_ast, Ast, Automaton, ParseTable, SymbolId, GrammarBuilder};
use gazelle_core::codegen::{self, CodegenContext, AlternativeInfo, RuleInfo};
use std::collections::HashMap;
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
        output_rust(&input);
    } else {
        output_json(&input);
    }
}

fn output_rust(input: &str) {
    // Parse to AST to get type information
    let ast = match parse_grammar_ast(input) {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    };

    // Build CodegenContext from AST
    let ctx = match build_codegen_context(&ast) {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Generate Rust code
    match codegen::generate(&ctx) {
        Ok(code) => println!("{}", code),
        Err(e) => {
            eprintln!("Codegen error: {}", e);
            std::process::exit(1);
        }
    }
}

fn build_codegen_context(ast: &Ast) -> Result<CodegenContext, String> {
    // Extract grammar definition
    let (grammar_name, sections) = match ast {
        Ast::GrammarDef { name, sections } => (name.clone(), sections.as_ref()),
        _ => return Err("Expected GrammarDef".to_string()),
    };

    let sections_vec = match sections {
        Ast::Sections(s) => s.clone(),
        other => vec![other.clone()],
    };

    let mut gb = GrammarBuilder::new();
    let mut terminal_types: HashMap<SymbolId, Option<String>> = HashMap::new();
    let mut prec_terminal_types: HashMap<SymbolId, String> = HashMap::new();
    let mut symbol_names: HashMap<SymbolId, String> = HashMap::new();
    let mut rule_names: Vec<String> = Vec::new();
    let mut rule_result_types: Vec<String> = Vec::new();

    // Collected data about a rule alternative
    struct AltData {
        symbols: Vec<String>,
        name: Option<String>,
    }

    // Collected data about a rule
    struct RuleData {
        name: String,
        result_type: Option<String>,
        alternatives: Vec<AltData>,
    }

    let mut rule_data: Vec<RuleData> = Vec::new();

    // First pass: process terminals and prec_terminals, collect rules
    for section in &sections_vec {
        match section {
            Ast::TerminalsBlock(defs) => {
                for def in defs {
                    if let Ast::TerminalDef { name, type_name } = def {
                        let sym = gb.t(name);
                        terminal_types.insert(sym.id(), type_name.clone());
                        symbol_names.insert(sym.id(), name.clone());
                    }
                }
            }
            Ast::PrecTerminalsBlock(defs) => {
                for def in defs {
                    if let Ast::PrecTerminalDef { name, type_name } = def {
                        let sym = gb.pt(name);
                        prec_terminal_types.insert(sym.id(), type_name.clone());
                        symbol_names.insert(sym.id(), name.clone());
                    }
                }
            }
            Ast::Rule { name, result_type, alts } => {
                let alts_vec = match alts.as_ref() {
                    Ast::Alts(a) => a.clone(),
                    other => vec![other.clone()],
                };

                let mut alternatives = Vec::new();
                for alt in alts_vec {
                    let (symbols, alt_name) = match alt {
                        Ast::Seq(s) => (s, None),
                        Ast::Alt { symbols, name } => (symbols, name),
                        _ => return Err("Expected Seq or Alt in alternatives".to_string()),
                    };
                    alternatives.push(AltData { symbols, name: alt_name });
                }
                rule_data.push(RuleData {
                    name: name.clone(),
                    result_type: result_type.clone(),
                    alternatives,
                });
            }
            _ => return Err(format!("Unexpected section type: {:?}", section)),
        }
    }

    // Second pass: intern non-terminals and collect rule info
    for rd in &rule_data {
        let nt = gb.nt(&rd.name);
        symbol_names.insert(nt.id(), rd.name.clone());
        rule_names.push(rd.name.clone());
        rule_result_types.push(rd.result_type.clone().unwrap_or_default());
    }

    // Third pass: build grammar rules
    for rd in &rule_data {
        let lhs = gb.symbols.get(&rd.name).ok_or_else(|| format!("Unknown non-terminal: {}", rd.name))?;

        for alt in &rd.alternatives {
            let rhs: Vec<_> = alt.symbols
                .iter()
                .map(|sym_name| {
                    gb.symbols
                        .get(sym_name)
                        .ok_or_else(|| format!("Unknown symbol: {}", sym_name))
                })
                .collect::<Result<_, _>>()?;

            gb.rule(lhs, rhs);
        }
    }

    if rule_data.is_empty() {
        return Err(format!("Grammar '{}' has no rules", grammar_name));
    }

    let grammar = gb.build();

    // Build detailed rule info with types
    let mut rules = Vec::new();
    for rd in &rule_data {
        let mut alternatives = Vec::new();
        for alt in &rd.alternatives {
            let symbols_with_types: Vec<_> = alt.symbols.iter().map(|sym_name| {
                // Look up type for this symbol
                let sym_type = if let Some(sym) = grammar.symbols.get(sym_name) {
                    if let Some(t) = terminal_types.get(&sym.id()) {
                        t.clone()
                    } else if let Some(t) = prec_terminal_types.get(&sym.id()) {
                        Some(t.clone())
                    } else {
                        // Non-terminal - look up its result type
                        rule_data.iter()
                            .find(|r| r.name == *sym_name)
                            .and_then(|r| r.result_type.clone())
                    }
                } else {
                    None
                };
                (sym_name.clone(), sym_type)
            }).collect();

            alternatives.push(AlternativeInfo {
                name: alt.name.clone(),
                symbols: symbols_with_types,
            });
        }

        rules.push(RuleInfo {
            name: rd.name.clone(),
            result_type: rd.result_type.clone(),
            alternatives,
        });
    }

    Ok(CodegenContext {
        grammar,
        visibility: "pub ".to_string(),
        name: grammar_name,
        terminal_types,
        prec_terminal_types,
        rule_result_types,
        symbol_names,
        rule_names,
        // Use relative paths (gazelle_core::) for CLI-generated code
        // since the user must provide `use crate as gazelle_core;` or similar
        use_absolute_path: false,
        rules,
    })
}

fn output_json(input: &str) {
    let grammar = match parse_grammar(input) {
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

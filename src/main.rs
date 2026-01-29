//! Gazelle CLI - parse grammar and output tables or Rust code.
//!
//! Usage:
//!   gazelle grammar.txt          # output JSON tables
//!   gazelle --rust grammar.txt   # output Rust code
//!   gazelle < grammar.txt        # read from stdin
//!   gazelle -                     # read from stdin (explicit)

use gazelle::{parse_grammar, CompiledTable, SymbolId, GrammarBuilder};
#[cfg(feature = "codegen")]
use gazelle::codegen::{self, CodegenContext, AlternativeInfo, RuleInfo, ActionKind};
#[cfg(feature = "codegen")]
use gazelle::meta::{GrammarDef, desugar_modifiers};
#[cfg(feature = "codegen")]
use std::collections::BTreeMap;
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
        output_rust(&input);
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
    // Parse to typed AST
    let grammar_def = match gazelle::parse_grammar_typed(input) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Parse error: {}", e);
            std::process::exit(1);
        }
    };

    // Build CodegenContext from typed AST
    let ctx = match build_codegen_context(&grammar_def) {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Generate Rust code
    match codegen::generate_tokens(&ctx) {
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

#[cfg(feature = "codegen")]
fn build_codegen_context(grammar_def: &GrammarDef) -> Result<CodegenContext, String> {
    // Clone and desugar modifiers first
    let mut grammar_def = grammar_def.clone();
    desugar_modifiers(&mut grammar_def);

    let grammar_name = grammar_def.name.clone();

    let mut gb = GrammarBuilder::new();
    let mut terminal_types: BTreeMap<SymbolId, Option<String>> = BTreeMap::new();
    let mut prec_terminal_types: BTreeMap<SymbolId, Option<String>> = BTreeMap::new();
    let mut symbol_names: BTreeMap<SymbolId, String> = BTreeMap::new();
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

    // Process terminals (unified - prec and non-prec in same list)
    for def in &grammar_def.terminals {
        let sym = if def.is_prec {
            gb.pt(&def.name)
        } else {
            gb.t(&def.name)
        };

        if def.is_prec {
            prec_terminal_types.insert(sym.id(), def.type_name.clone());
        } else {
            terminal_types.insert(sym.id(), def.type_name.clone());
        }
        symbol_names.insert(sym.id(), def.name.clone());
    }

    // Collect rules (extract symbol names after desugaring)
    for rule in &grammar_def.rules {
        let mut alternatives = Vec::new();
        for alt in &rule.alts {
            alternatives.push(AltData {
                symbols: alt.symbols.iter().map(|s| s.name.clone()).collect(),
                name: alt.name.clone(),
            });
        }
        rule_data.push(RuleData {
            name: rule.name.clone(),
            result_type: rule.result_type.clone(),
            alternatives,
        });
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

    // Set the start symbol
    if let Some(start_sym) = gb.symbols.get(&grammar_def.start) {
        gb.start(start_sym);
    } else {
        return Err(format!("Start symbol '{}' not found in grammar", grammar_def.start));
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
                        t.clone()
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

            let action = name_to_action_kind(&alt.name);
            alternatives.push(AlternativeInfo {
                action,
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
        start_symbol: grammar_def.start.clone(),
    })
}

/// Convert action name to ActionKind.
#[cfg(feature = "codegen")]
fn name_to_action_kind(name: &Option<String>) -> ActionKind {
    match name.as_deref() {
        None => ActionKind::None,
        Some("__some") => ActionKind::OptSome,
        Some("__none") => ActionKind::OptNone,
        Some("__empty") => ActionKind::VecEmpty,
        Some("__single") => ActionKind::VecSingle,
        Some("__append") => ActionKind::VecAppend,
        Some(s) => ActionKind::Named(s.to_string()),
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
    let rules = table.rules();
    for (i, (lhs, len)) in rules.iter().enumerate() {
        let comma = if i + 1 < rules.len() { "," } else { "" };
        println!("    [{}, {}]{}", lhs, len, comma);
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

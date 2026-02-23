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
    let mut yacc_mode = false;
    let mut bootstrap_meta = false;
    let mut input_file: Option<&str> = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--rust" => rust_mode = true,
            "--yacc" => yacc_mode = true,
            "--bootstrap-meta" => bootstrap_meta = true,
            "-" => {}
            _ => input_file = Some(arg),
        }
    }

    if bootstrap_meta {
        #[cfg(feature = "codegen")]
        {
            do_bootstrap_meta();
        }
        #[cfg(not(feature = "codegen"))]
        {
            eprintln!("--bootstrap-meta requires the 'codegen' feature");
            std::process::exit(1);
        }
        return;
    }

    let input = if let Some(file) = input_file {
        fs::read_to_string(file).expect("Failed to read file")
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).expect("Failed to read stdin");
        buf
    };

    if yacc_mode {
        output_yacc(&input);
    } else if rust_mode {
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

fn output_yacc(input: &str) {
    #[cfg(feature = "codegen")]
    {
        let grammar = match parse_grammar(input) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Parse error: {}", e);
                std::process::exit(1);
            }
        };
        match codegen::to_yacc(&grammar) {
            Ok(yacc) => print!("{}", yacc),
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
    #[cfg(not(feature = "codegen"))]
    {
        let _ = input;
        eprintln!("--yacc mode requires the 'codegen' feature");
        std::process::exit(1);
    }
}

#[cfg(feature = "codegen")]
fn do_bootstrap_meta() {
    use gazelle::grammar as g;

    let grammar = g::Grammar {
        start: "grammar_def".to_string(),
        expect_rr: 0,
        expect_sr: 0,
        terminals: vec![
            g::TerminalDef { name: "IDENT".into(), has_type: true, is_prec: false },
            g::TerminalDef { name: "NUM".into(), has_type: true, is_prec: false },
            g::TerminalDef { name: "KW_START".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "KW_TERMINALS".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "KW_PREC".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "KW_EXPECT".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "UNDERSCORE".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "LBRACE".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "RBRACE".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "LPAREN".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "RPAREN".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "COLON".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "COMMA".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "EQ".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "PIPE".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "SEMI".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "FAT_ARROW".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "QUESTION".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "STAR".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "PLUS".into(), has_type: false, is_prec: false },
            g::TerminalDef { name: "PERCENT".into(), has_type: false, is_prec: false },
        ],
        rules: vec![
            g::Rule {
                name: "grammar_def".into(),
                alts: vec![g::Alt {
                    terms: vec![
                        g::Term::Symbol("KW_START".into()),
                        g::Term::Symbol("IDENT".into()),
                        g::Term::Symbol("SEMI".into()),
                        g::Term::ZeroOrMore("expect_decl".into()),
                        g::Term::Symbol("KW_TERMINALS".into()),
                        g::Term::Symbol("LBRACE".into()),
                        g::Term::SeparatedBy { symbol: "terminal_item".into(), sep: "COMMA".into() },
                        g::Term::Symbol("RBRACE".into()),
                        g::Term::OneOrMore("rule".into()),
                    ],
                    name: "grammar_def".into(),
                }],
            },
            g::Rule {
                name: "expect_decl".into(),
                alts: vec![g::Alt {
                    terms: vec![
                        g::Term::Symbol("KW_EXPECT".into()),
                        g::Term::Symbol("NUM".into()),
                        g::Term::Symbol("IDENT".into()),
                        g::Term::Symbol("SEMI".into()),
                    ],
                    name: "expect_decl".into(),
                }],
            },
            g::Rule {
                name: "terminal_item".into(),
                alts: vec![g::Alt {
                    terms: vec![
                        g::Term::Optional("KW_PREC".into()),
                        g::Term::Symbol("IDENT".into()),
                        g::Term::Optional("type_annot".into()),
                    ],
                    name: "terminal_item".into(),
                }],
            },
            g::Rule {
                name: "type_annot".into(),
                alts: vec![g::Alt {
                    terms: vec![
                        g::Term::Symbol("COLON".into()),
                        g::Term::Symbol("UNDERSCORE".into()),
                    ],
                    name: "type_annot".into(),
                }],
            },
            g::Rule {
                name: "rule".into(),
                alts: vec![g::Alt {
                    terms: vec![
                        g::Term::Symbol("IDENT".into()),
                        g::Term::Symbol("EQ".into()),
                        g::Term::SeparatedBy { symbol: "alt".into(), sep: "PIPE".into() },
                        g::Term::Symbol("SEMI".into()),
                    ],
                    name: "rule".into(),
                }],
            },
            g::Rule {
                name: "alt".into(),
                alts: vec![g::Alt {
                    terms: vec![
                        g::Term::OneOrMore("term".into()),
                        g::Term::Symbol("variant".into()),
                    ],
                    name: "alt".into(),
                }],
            },
            g::Rule {
                name: "variant".into(),
                alts: vec![g::Alt {
                    terms: vec![
                        g::Term::Symbol("FAT_ARROW".into()),
                        g::Term::Symbol("IDENT".into()),
                    ],
                    name: "variant".into(),
                }],
            },
            g::Rule {
                name: "term".into(),
                alts: vec![
                    g::Alt {
                        terms: vec![
                            g::Term::Symbol("LPAREN".into()),
                            g::Term::Symbol("IDENT".into()),
                            g::Term::Symbol("PERCENT".into()),
                            g::Term::Symbol("IDENT".into()),
                            g::Term::Symbol("RPAREN".into()),
                        ],
                        name: "sym_sep".into(),
                    },
                    g::Alt {
                        terms: vec![
                            g::Term::Symbol("IDENT".into()),
                            g::Term::Symbol("QUESTION".into()),
                        ],
                        name: "sym_opt".into(),
                    },
                    g::Alt {
                        terms: vec![
                            g::Term::Symbol("IDENT".into()),
                            g::Term::Symbol("STAR".into()),
                        ],
                        name: "sym_star".into(),
                    },
                    g::Alt {
                        terms: vec![
                            g::Term::Symbol("IDENT".into()),
                            g::Term::Symbol("PLUS".into()),
                        ],
                        name: "sym_plus".into(),
                    },
                    g::Alt {
                        terms: vec![g::Term::Symbol("IDENT".into())],
                        name: "sym_plain".into(),
                    },
                    g::Alt {
                        terms: vec![g::Term::Symbol("UNDERSCORE".into())],
                        name: "sym_empty".into(),
                    },
                ],
            },
        ],
    };

    let ctx = CodegenContext::from_grammar(&grammar, "", "pub ", false)
        .expect("failed to build codegen context");

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
        for msg in table.format_conflicts() {
            eprintln!("{}\n", msg);
        }
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

    // Shared displacement table (bison-style: action + goto share data/check)
    print!("  \"data\": [");
    print_u32_array(table.table_data());
    println!("],");

    print!("  \"check\": [");
    print_u32_array(table.table_check());
    println!("],");

    print!("  \"action_base\": [");
    print_i32_array(table.action_base());
    println!("],");

    print!("  \"goto_base\": [");
    print_i32_array(table.goto_base());
    println!("],");

    // Default reduce per state
    print!("  \"default_reduce\": [");
    print_u32_array(table.default_reduce());
    println!("],");

    // Default goto per non-terminal
    print!("  \"default_goto\": [");
    print_u32_array(table.default_goto());
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

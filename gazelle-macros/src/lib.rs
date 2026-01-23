//! Procedural macros for Gazelle parser generator.
//!
//! This crate provides the `grammar!` macro that allows defining grammars
//! in Rust with type-safe parsers generated at compile time.
//!
//! # Example
//!
//! ```ignore
//! grammar! {
//!     pub grammar Calc {
//!         terminals {
//!             NUM: f64,
//!             LPAREN,
//!             RPAREN,
//!         }
//!
//!         prec_terminals {
//!             OP: Operator,
//!         }
//!
//!         expr: Expr = expr OP expr | atom;
//!         atom: Atom = NUM | LPAREN expr RPAREN;
//!     }
//! }
//! ```

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use std::collections::HashMap;

use gazelle_core::meta_bootstrap::{Ast, MetaTerminal};
use gazelle_core::{GrammarBuilder, SymbolId};

/// Define a grammar and generate a type-safe parser.
///
/// See the crate-level documentation for usage examples.
#[proc_macro]
pub fn grammar(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into();

    match parse_and_generate(input2) {
        Ok(code) => code.parse().unwrap(),
        Err(msg) => {
            let err = format!("compile_error!({:?});", msg);
            err.parse().unwrap()
        }
    }
}

fn parse_and_generate(input: proc_macro2::TokenStream) -> Result<String, String> {
    // Lex TokenStream into MetaTerminals
    let (visibility, tokens) = lex_token_stream(input)?;

    if tokens.is_empty() {
        return Err("Empty grammar".to_string());
    }

    // Parse using core's parser
    let ast = gazelle_core::meta_bootstrap::parse_tokens(tokens)?;

    // Convert AST to CodegenContext
    let ctx = ast_to_codegen_context(&ast, &visibility)?;

    // Generate code
    gazelle_core::codegen::generate(&ctx)
}

/// Lex a proc_macro2::TokenStream into MetaTerminals.
/// Returns (visibility_string, tokens).
fn lex_token_stream(input: proc_macro2::TokenStream) -> Result<(String, Vec<MetaTerminal>), String> {
    let mut tokens = Vec::new();
    let mut iter = input.into_iter().peekable();

    // Check for visibility (pub, pub(crate), etc.)
    let visibility = if matches!(iter.peek(), Some(TokenTree::Ident(id)) if id.to_string() == "pub") {
        iter.next(); // consume "pub"

        // Check for (crate) or (super) etc.
        if matches!(iter.peek(), Some(TokenTree::Group(g)) if matches!(g.delimiter(), proc_macro2::Delimiter::Parenthesis)) {
            let group = iter.next().unwrap();
            format!("pub{} ", group)
        } else {
            "pub ".to_string()
        }
    } else {
        String::new()
    };

    // Convert remaining tokens
    lex_tokens(&mut iter, &mut tokens)?;

    Ok((visibility, tokens))
}

fn lex_tokens(
    iter: &mut std::iter::Peekable<proc_macro2::token_stream::IntoIter>,
    tokens: &mut Vec<MetaTerminal>,
) -> Result<(), String> {
    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Ident(id) => {
                let s = id.to_string();
                match s.as_str() {
                    "grammar" => tokens.push(MetaTerminal::KwGrammar),
                    "terminals" => tokens.push(MetaTerminal::KwTerminals),
                    "prec_terminals" => tokens.push(MetaTerminal::KwPrecTerminals),
                    _ => tokens.push(MetaTerminal::Ident(s)),
                }
            }
            TokenTree::Punct(p) => {
                let c = p.as_char();
                match c {
                    '{' => tokens.push(MetaTerminal::Lbrace),
                    '}' => tokens.push(MetaTerminal::Rbrace),
                    ',' => tokens.push(MetaTerminal::Comma),
                    '|' => tokens.push(MetaTerminal::Pipe),
                    ';' => tokens.push(MetaTerminal::Semi),
                    '@' => tokens.push(MetaTerminal::At),
                    ':' => {
                        tokens.push(MetaTerminal::Colon);
                        // After colon, collect the type as a single IDENT
                        let type_str = collect_type(iter)?;
                        if !type_str.is_empty() {
                            tokens.push(MetaTerminal::Ident(type_str));
                        }
                    }
                    '=' => tokens.push(MetaTerminal::Eq),
                    _ => return Err(format!("Unexpected punctuation: {}", c)),
                }
            }
            TokenTree::Group(g) => {
                match g.delimiter() {
                    proc_macro2::Delimiter::Brace => {
                        tokens.push(MetaTerminal::Lbrace);
                        let mut inner_iter = g.stream().into_iter().peekable();
                        lex_tokens(&mut inner_iter, tokens)?;
                        tokens.push(MetaTerminal::Rbrace);
                    }
                    _ => return Err(format!("Unexpected group delimiter: {:?}", g.delimiter())),
                }
            }
            TokenTree::Literal(_) => {
                return Err("Unexpected literal in grammar".to_string());
            }
        }
    }
    Ok(())
}

/// Collect tokens that form a type (until we hit a delimiter like , = | ; { }).
fn collect_type(iter: &mut std::iter::Peekable<proc_macro2::token_stream::IntoIter>) -> Result<String, String> {
    let mut type_tokens = Vec::new();

    while let Some(tt) = iter.peek() {
        match tt {
            TokenTree::Punct(p) => {
                let c = p.as_char();
                // Stop at delimiters that end a type
                if matches!(c, ',' | '=' | '|' | ';' | '{' | '}') {
                    break;
                }
                // Include other punct in type (like < > ::)
                type_tokens.push(iter.next().unwrap());
            }
            TokenTree::Ident(_) | TokenTree::Literal(_) => {
                type_tokens.push(iter.next().unwrap());
            }
            TokenTree::Group(g) => {
                // Handle things like () or <T>
                match g.delimiter() {
                    proc_macro2::Delimiter::Parenthesis |
                    proc_macro2::Delimiter::Bracket |
                    proc_macro2::Delimiter::None => {
                        type_tokens.push(iter.next().unwrap());
                    }
                    proc_macro2::Delimiter::Brace => {
                        // Brace ends the type
                        break;
                    }
                }
            }
        }
    }

    // Stringify the collected tokens
    let type_str: String = type_tokens.iter().map(|t| t.to_string()).collect::<Vec<_>>().join("");

    // Clean up spacing issues from tokenization
    let type_str = type_str
        .replace(" < ", "<")
        .replace(" > ", ">")
        .replace(" ::", "::")
        .replace(":: ", "::")
        .replace(" ,", ",")
        .replace(", ", ",");

    Ok(type_str)
}

/// Collected data about a rule alternative from the AST.
struct AltData {
    symbols: Vec<String>,
    name: Option<String>,
}

/// Collected data about a rule from the AST.
struct RuleData {
    name: String,
    result_type: Option<String>,
    alternatives: Vec<AltData>,
}

/// Convert parsed AST to a CodegenContext for code generation.
fn ast_to_codegen_context(ast: &Ast, visibility: &str) -> Result<gazelle_core::codegen::CodegenContext, String> {
    use gazelle_core::codegen::{AlternativeInfo, RuleInfo};

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

    Ok(gazelle_core::codegen::CodegenContext {
        grammar,
        visibility: visibility.to_string(),
        name: grammar_name,
        terminal_types,
        prec_terminal_types,
        rule_result_types,
        symbol_names,
        rule_names,
        use_absolute_path: true,
        rules,
    })
}

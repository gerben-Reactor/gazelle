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
//!             prec OP: Operator,
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

use gazelle::meta::{AstBuilder, GrammarDef, MetaTerminal, desugar_modifiers};
use gazelle::{GrammarBuilder, SymbolId};

/// Define a grammar and generate a type-safe parser.
///
/// See the crate-level documentation for usage examples.
#[proc_macro]
pub fn grammar(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into();

    match parse_and_generate(input2) {
        Ok(tokens) => tokens.into(),
        Err(msg) => {
            let err = format!("compile_error!({:?});", msg);
            err.parse().unwrap()
        }
    }
}

fn parse_and_generate(input: proc_macro2::TokenStream) -> Result<proc_macro2::TokenStream, String> {
    // Lex TokenStream into MetaTerminals
    let (visibility, tokens) = lex_token_stream(input)?;

    if tokens.is_empty() {
        return Err("Empty grammar".to_string());
    }

    // Parse using core's parser
    let grammar_def = gazelle::meta::parse_tokens_typed(tokens)?;

    // Convert GrammarDef to CodegenContext
    let ctx = grammar_def_to_codegen_context(&grammar_def, &visibility)?;

    // Generate code directly as TokenStream
    gazelle::codegen::generate_tokens(&ctx)
}

/// Lex a proc_macro2::TokenStream into MetaTerminals.
/// Returns (visibility_string, tokens).
fn lex_token_stream(input: proc_macro2::TokenStream) -> Result<(String, Vec<MetaTerminal<AstBuilder>>), String> {
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
    tokens: &mut Vec<MetaTerminal<AstBuilder>>,
) -> Result<(), String> {
    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Ident(id) => {
                let s = id.to_string();
                match s.as_str() {
                    "grammar" => tokens.push(MetaTerminal::KwGrammar),
                    "start" => tokens.push(MetaTerminal::KwStart),
                    "terminals" => tokens.push(MetaTerminal::KwTerminals),
                    "prec" => tokens.push(MetaTerminal::KwPrec),
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
                    '?' => tokens.push(MetaTerminal::Question),
                    '*' => tokens.push(MetaTerminal::Star),
                    '+' => tokens.push(MetaTerminal::Plus),
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

/// Convert parsed GrammarDef to a CodegenContext for code generation.
fn grammar_def_to_codegen_context(grammar_def: &GrammarDef, visibility: &str) -> Result<gazelle::codegen::CodegenContext, String> {
    use gazelle::codegen::{AlternativeInfo, RuleInfo, ActionKind};

    // Clone and desugar modifiers first
    let mut grammar_def = grammar_def.clone();
    desugar_modifiers(&mut grammar_def);

    let grammar_name = grammar_def.name.clone();

    let mut gb = GrammarBuilder::new();
    let mut terminal_types: HashMap<SymbolId, Option<String>> = HashMap::new();
    let mut prec_terminal_types: HashMap<SymbolId, Option<String>> = HashMap::new();
    let mut symbol_names: HashMap<SymbolId, String> = HashMap::new();
    let mut rule_names: Vec<String> = Vec::new();
    let mut rule_result_types: Vec<String> = Vec::new();

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

    // Second pass: intern non-terminals and collect rule info
    for rule in &grammar_def.rules {
        let nt = gb.nt(&rule.name);
        symbol_names.insert(nt.id(), rule.name.clone());
        rule_names.push(rule.name.clone());
        rule_result_types.push(rule.result_type.clone().unwrap_or_default());
    }

    // Third pass: build grammar rules
    for rule in &grammar_def.rules {
        let lhs = gb.symbols.get(&rule.name).ok_or_else(|| format!("Unknown non-terminal: {}", rule.name))?;

        for alt in &rule.alts {
            let rhs: Vec<_> = alt.symbols
                .iter()
                .map(|sym| {
                    gb.symbols
                        .get(&sym.name)
                        .ok_or_else(|| format!("Unknown symbol: {}", sym.name))
                })
                .collect::<Result<_, _>>()?;

            gb.rule(lhs, rhs);
        }
    }

    if grammar_def.rules.is_empty() {
        return Err(format!("Grammar '{}' has no rules", grammar_name));
    }

    let grammar = gb.build();

    // Build detailed rule info with types
    let mut rules = Vec::new();
    for rule in &grammar_def.rules {
        let mut alternatives = Vec::new();
        for alt in &rule.alts {
            let symbols_with_types: Vec<_> = alt.symbols.iter().map(|sym| {
                let sym_name = &sym.name;
                // Look up type for this symbol
                let sym_type = if let Some(gsym) = grammar.symbols.get(sym_name) {
                    if let Some(t) = terminal_types.get(&gsym.id()) {
                        t.clone()
                    } else if let Some(t) = prec_terminal_types.get(&gsym.id()) {
                        t.clone()
                    } else {
                        // Non-terminal - look up its result type
                        grammar_def.rules.iter()
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
            name: rule.name.clone(),
            result_type: rule.result_type.clone(),
            alternatives,
        });
    }

    /// Convert action name to ActionKind.
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

    Ok(gazelle::codegen::CodegenContext {
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
        start_symbol: grammar_def.start.clone(),
    })
}

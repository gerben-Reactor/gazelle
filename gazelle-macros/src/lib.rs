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

use gazelle::meta::{AstBuilder, MetaTerminal};

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

    // Convert GrammarDef to CodegenContext and generate code
    let ctx = gazelle::codegen::CodegenContext::from_grammar_def(&grammar_def, &visibility, true)?;
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
                    "grammar" => tokens.push(MetaTerminal::KW_GRAMMAR),
                    "start" => tokens.push(MetaTerminal::KW_START),
                    "terminals" => tokens.push(MetaTerminal::KW_TERMINALS),
                    "prec" => tokens.push(MetaTerminal::KW_PREC),
                    "expect" => tokens.push(MetaTerminal::KW_EXPECT),
                    "mode" => tokens.push(MetaTerminal::KW_MODE),
                    "_" => tokens.push(MetaTerminal::UNDERSCORE),
                    _ => tokens.push(MetaTerminal::IDENT(s)),
                }
            }
            TokenTree::Punct(p) => {
                let c = p.as_char();
                match c {
                    '{' => tokens.push(MetaTerminal::LBRACE),
                    '}' => tokens.push(MetaTerminal::RBRACE),
                    ',' => tokens.push(MetaTerminal::COMMA),
                    '|' => tokens.push(MetaTerminal::PIPE),
                    ';' => tokens.push(MetaTerminal::SEMI),
                    '@' => tokens.push(MetaTerminal::AT),
                    '?' => tokens.push(MetaTerminal::QUESTION),
                    '*' => tokens.push(MetaTerminal::STAR),
                    '+' => tokens.push(MetaTerminal::PLUS),
                    ':' => {
                        tokens.push(MetaTerminal::COLON);
                        // After colon, collect the type as a single IDENT
                        let type_str = collect_type(iter)?;
                        if !type_str.is_empty() {
                            tokens.push(MetaTerminal::IDENT(type_str));
                        }
                    }
                    '=' => tokens.push(MetaTerminal::EQ),
                    _ => return Err(format!("Unexpected punctuation: {}", c)),
                }
            }
            TokenTree::Group(g) => {
                match g.delimiter() {
                    proc_macro2::Delimiter::Brace => {
                        tokens.push(MetaTerminal::LBRACE);
                        let mut inner_iter = g.stream().into_iter().peekable();
                        lex_tokens(&mut inner_iter, tokens)?;
                        tokens.push(MetaTerminal::RBRACE);
                    }
                    _ => return Err(format!("Unexpected group delimiter: {:?}", g.delimiter())),
                }
            }
            TokenTree::Literal(lit) => {
                // Handle numeric literals for expect declarations
                let s = lit.to_string();
                // Check if it's a number (integer literal)
                if s.chars().all(|c| c.is_ascii_digit()) {
                    tokens.push(MetaTerminal::NUM(s));
                } else {
                    return Err(format!("Unexpected literal in grammar: {}", s));
                }
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

//! Procedural macros for Gazelle parser generator.
//!
//! This crate provides the `gazelle!` macro that allows defining grammars
//! in Rust with type-safe parsers generated at compile time.
//!
//! # Example
//!
//! ```
//! use gazelle_macros::gazelle;
//! use gazelle::Precedence;
//!
//! gazelle! {
//!     grammar Expr {
//!         start expr;
//!         terminals {
//!             NUM: Num,
//!             LPAREN, RPAREN,
//!             prec OP: Op
//!         }
//!         expr: Num = NUM
//!                    | expr OP expr => binop
//!                    | LPAREN expr RPAREN;
//!     }
//! }
//!
//! struct Eval;
//! impl ExprTypes for Eval {
//!     type Num = f64;
//!     type Op = char;
//! }
//! impl ExprActions for Eval {
//!     fn binop(&mut self, l: f64, op: char, r: f64) -> Result<f64, gazelle::ParseError> {
//!         Ok(match op { '+' => l + r, '-' => l - r, '*' => l * r, '/' => l / r, _ => 0.0 })
//!     }
//! }
//!
//! let mut parser = ExprParser::<Eval>::new();
//! let mut eval = Eval;
//! parser.push(ExprTerminal::NUM(1.0), &mut eval).unwrap();
//! parser.push(ExprTerminal::OP('+', Precedence::Left(1)), &mut eval).unwrap();
//! parser.push(ExprTerminal::NUM(2.0), &mut eval).unwrap();
//! parser.push(ExprTerminal::OP('*', Precedence::Left(2)), &mut eval).unwrap();
//! parser.push(ExprTerminal::NUM(3.0), &mut eval).unwrap();
//! let result = parser.finish(&mut eval).map_err(|(_, e)| e).unwrap();
//! assert_eq!(result, 7.0);  // 1 + (2 * 3)
//! ```

use proc_macro::TokenStream;
use proc_macro2::TokenTree;

use gazelle::meta::{AstBuilder, MetaTerminal};

/// Define a grammar and generate a type-safe parser.
///
/// See the crate-level documentation for usage examples.
#[proc_macro]
pub fn gazelle(input: TokenStream) -> TokenStream {
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
    let (visibility, name, tokens) = lex_token_stream(input)?;

    if tokens.is_empty() {
        return Err("Empty grammar".to_string());
    }

    // Parse using core's parser
    let grammar_def = gazelle::meta::parse_tokens_typed(tokens)?;

    // Convert GrammarDef to CodegenContext and generate code
    let ctx = gazelle::codegen::CodegenContext::from_grammar(&grammar_def, &name, &visibility, true)?;
    gazelle::codegen::generate_tokens(&ctx)
}

/// Lex a proc_macro2::TokenStream into MetaTerminals.
/// Returns (visibility_string, name, tokens).
///
/// Expected format: `[pub] grammar Name { grammar_content... }`
fn lex_token_stream(input: proc_macro2::TokenStream) -> Result<(String, String, Vec<MetaTerminal<AstBuilder>>), String> {
    let mut tokens = Vec::new();
    let mut iter = input.into_iter().peekable();

    // Check for visibility (pub, pub(crate), etc.)
    let visibility = if matches!(iter.peek(), Some(TokenTree::Ident(id)) if *id == "pub") {
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

    // Expect `grammar` keyword
    match iter.next() {
        Some(TokenTree::Ident(id)) if id == "grammar" => {}
        other => return Err(format!("Expected `grammar` keyword, got {:?}", other)),
    }

    // Extract grammar name
    let name = match iter.next() {
        Some(TokenTree::Ident(id)) => id.to_string(),
        other => return Err(format!("Expected grammar name after `grammar`, got {:?}", other)),
    };

    // Extract braced content
    let content = match iter.next() {
        Some(TokenTree::Group(g)) if matches!(g.delimiter(), proc_macro2::Delimiter::Brace) => {
            g.stream()
        }
        other => return Err(format!("Expected {{ after grammar name, got {:?}", other)),
    };

    // Lex the content inside the braces
    let mut inner_iter = content.into_iter().peekable();
    lex_tokens(&mut inner_iter, &mut tokens)?;

    Ok((visibility, name, tokens))
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
                    '?' => tokens.push(MetaTerminal::QUESTION),
                    '*' => tokens.push(MetaTerminal::STAR),
                    '+' => tokens.push(MetaTerminal::PLUS),
                    '%' => tokens.push(MetaTerminal::PERCENT),
                    ':' => {
                        tokens.push(MetaTerminal::COLON);
                        // After colon, collect the type as a single IDENT
                        let type_str = collect_type(iter)?;
                        if !type_str.is_empty() {
                            tokens.push(MetaTerminal::IDENT(type_str));
                        }
                    }
                    '=' => {
                        // Check for => (fat arrow)
                        if p.spacing() == proc_macro2::Spacing::Joint {
                            if let Some(TokenTree::Punct(p2)) = iter.peek() {
                                if p2.as_char() == '>' {
                                    iter.next();
                                    tokens.push(MetaTerminal::FAT_ARROW);
                                    continue;
                                }
                            }
                        }
                        tokens.push(MetaTerminal::EQ);
                    }
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
                    proc_macro2::Delimiter::Parenthesis => {
                        tokens.push(MetaTerminal::LPAREN);
                        let mut inner_iter = g.stream().into_iter().peekable();
                        lex_tokens(&mut inner_iter, tokens)?;
                        tokens.push(MetaTerminal::RPAREN);
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

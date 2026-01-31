//! Runtime Grammar CLI
//!
//! Parses a token stream using a grammar loaded at runtime.
//! The token stream format itself is parsed by a compiled grammar.
//!
//! Usage:
//!   cargo run --example runtime_grammar <grammar.gzl> < tokens.txt
//!
//! Token format (space-separated):
//!   NAME           - terminal with no value
//!   NAME:value     - terminal with value
//!   NAME@L5        - terminal with left-assoc precedence 5
//!   NAME:value@R3  - terminal with value and right-assoc precedence 3
//!
//! Example:
//!   $ echo "NUM:1 PLUS NUM:2 STAR NUM:3" | cargo run --example runtime_grammar expr.gzl

use gazelle::grammar::Precedence;
use gazelle::lexer::{Lexer, Token as LexToken};
use gazelle::runtime::{Parser, Token};
use gazelle::table::CompiledTable;
use gazelle::{parse_grammar, ErrorContext};
use gazelle_macros::grammar;
use std::io::{self, Read};

// The token stream format is itself defined by a grammar!
// NAME:value@L5 etc.
grammar! {
    grammar TokenFormat {
        start tokens;
        terminals {
            IDENT: Ident,    // terminal name or value
            NUM: Num,        // numeric value
            COLON,           // :
            AT,              // @
        }

        tokens: Tokens = token* @collect;

        token: Tok = IDENT COLON value AT precedence @tok_val_prec
                   | IDENT AT precedence @tok_prec
                   | IDENT COLON value @tok_val
                   | IDENT @tok_name;

        value: Val = IDENT @val_ident | NUM @val_num;

        precedence: Prec = IDENT @parse_prec;
    }
}

/// Parsed token
struct Tok {
    name: String,
    value: Option<String>,
    prec: Option<Precedence>,
}

struct TokActions;

impl TokenFormatActions for TokActions {
    type Ident = String;
    type Num = String;
    type Tok = Tok;
    type Val = String;
    type Prec = Precedence;
    type Tokens = Vec<Tok>;

    fn collect(&mut self, tokens: Vec<Tok>) -> Vec<Tok> { tokens }

    fn tok_name(&mut self, name: String) -> Tok {
        Tok { name, value: None, prec: None }
    }

    fn tok_val(&mut self, name: String, value: String) -> Tok {
        Tok { name, value: Some(value), prec: None }
    }

    fn tok_prec(&mut self, name: String, prec: Precedence) -> Tok {
        Tok { name, value: None, prec: Some(prec) }
    }

    fn tok_val_prec(&mut self, name: String, value: String, prec: Precedence) -> Tok {
        Tok { name, value: Some(value), prec: Some(prec) }
    }

    fn val_ident(&mut self, s: String) -> String { s }
    fn val_num(&mut self, s: String) -> String { s }

    fn parse_prec(&mut self, s: String) -> Precedence {
        let (assoc, level) = s.split_at(1);
        let n: u8 = level.parse().unwrap_or(10);
        match assoc {
            "L" | "l" => Precedence::Left(n),
            "R" | "r" => Precedence::Right(n),
            _ => Precedence::Left(n),
        }
    }
}

/// Lex and parse the token format from stdin
fn parse_token_stream(input: &str) -> Result<Vec<Tok>, String> {
    let mut lexer = Lexer::new(input);
    let mut parser = TokenFormatParser::<TokActions>::new();
    let mut actions = TokActions;

    while let Some(result) = lexer.next() {
        let tok = result?;
        let terminal = match tok {
            LexToken::Ident(s) => TokenFormatTerminal::IDENT(s),
            LexToken::Num(s) => TokenFormatTerminal::NUM(s),
            LexToken::Op(ref s) if s == ":" => TokenFormatTerminal::COLON,
            LexToken::Op(ref s) if s == "@" => TokenFormatTerminal::AT,
            LexToken::Op(s) => TokenFormatTerminal::IDENT(s), // operators like +, * become IDENT
            LexToken::Punct(c) => TokenFormatTerminal::IDENT(c.to_string()), // punctuation too
            _ => continue, // skip whitespace etc
        };
        parser.push(terminal, &mut actions).map_err(|e| format!("{:?}", e))?;
    }

    parser.finish(&mut actions).map_err(|e| format!("{:?}", e))
}

/// Generic AST node
#[derive(Debug)]
enum Ast {
    Leaf(String, Option<String>),
    Node(String, Vec<Ast>),  // rule name (or lhs if unnamed), children
}

impl Ast {
    fn print(&self, indent: usize) {
        let pad = "  ".repeat(indent);
        match self {
            Ast::Leaf(name, None) => println!("{}{}", pad, name),
            Ast::Leaf(name, Some(v)) => println!("{}{}:{}", pad, name, v),
            Ast::Node(name, children) if children.len() == 1 => {
                print!("{}({} ", pad, name);
                match &children[0] {
                    Ast::Leaf(n, None) => println!("{})", n),
                    Ast::Leaf(n, Some(v)) => println!("{}:{})", n, v),
                    c => {
                        println!();
                        c.print(indent + 1);
                        println!("{})", pad);
                    }
                }
            }
            Ast::Node(name, children) => {
                println!("{}({}", pad, name);
                for c in children {
                    c.print(indent + 1);
                }
                println!("{})", pad);
            }
        }
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <grammar.gzl>", args[0]);
        eprintln!();
        eprintln!("Token format (stdin):");
        eprintln!("  NAME           - terminal");
        eprintln!("  NAME:value     - terminal with value");
        eprintln!("  NAME@L5        - left-assoc precedence 5");
        eprintln!("  NAME:val@R3    - value + right-assoc prec 3");
        std::process::exit(1);
    }

    // Load runtime grammar
    let src = std::fs::read_to_string(&args[1])
        .map_err(|e| format!("cannot read {}: {}", args[1], e))?;
    let grammar = parse_grammar(&src)?;
    let compiled = CompiledTable::build(&grammar);

    // Read and parse token stream using compiled grammar
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).map_err(|e| e.to_string())?;

    let tokens = parse_token_stream(&input)?;

    // Parse with runtime grammar
    let mut parser = Parser::new(compiled.table());
    let mut stack: Vec<Ast> = Vec::new();

    for tok in &tokens {
        let id = compiled.symbol_id(&tok.name)
            .ok_or_else(|| format!("unknown terminal: {}", tok.name))?;

        let token = match tok.prec {
            Some(p) => Token::with_prec(id, p),
            None => Token::new(id),
        };

        loop {
            match parser.maybe_reduce(Some(&token)) {
                Ok(Some((rule, len))) if rule > 0 => {
                    let (lhs, _) = compiled.table().rule_info(rule);
                    let lhs_name = compiled.symbol_name(lhs);
                    let name = compiled.rule_name(rule)
                        .unwrap_or(lhs_name)
                        .to_string();
                    let children: Vec<Ast> = stack.drain(stack.len() - len..).collect();
                    stack.push(Ast::Node(name, children));
                }
                Ok(_) => break,
                Err(e) => return Err(e.format(&compiled)),
            }
        }

        stack.push(Ast::Leaf(tok.name.clone(), tok.value.clone()));
        parser.shift(&token);
    }

    loop {
        match parser.maybe_reduce(None) {
            Ok(Some((0, _))) => break,
            Ok(Some((rule, len))) => {
                let (lhs, _) = compiled.table().rule_info(rule);
                let lhs_name = compiled.symbol_name(lhs);
                let name = compiled.rule_name(rule)
                    .unwrap_or(lhs_name)
                    .to_string();
                let children: Vec<Ast> = stack.drain(stack.len() - len..).collect();
                stack.push(Ast::Node(name, children));
            }
            Ok(None) => break,
            Err(e) => return Err(e.format(&compiled)),
        }
    }

    if stack.len() == 1 {
        stack.pop().unwrap().print(0);
        Ok(())
    } else {
        Err(format!("incomplete: {} on stack", stack.len()))
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

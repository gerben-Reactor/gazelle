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
//!   NAME@<5        - terminal with left-assoc precedence 5
//!   NAME:value@>3  - terminal with value and right-assoc precedence 3
//!
//! Example:
//!   $ echo "NUM:1 OP:+@<1 NUM:2 OP:*@<2 NUM:3" | cargo run --example runtime_grammar expr.gzl

use gazelle::grammar::Precedence;
use gazelle::lexer::{Lexer, Token as LexToken};
use gazelle::runtime::{Parser, Token};
use gazelle::table::CompiledTable;
use gazelle::{parse_grammar, ErrorContext};
use gazelle_macros::grammar;
use std::io::{self, Read};

// Token stream format parsed by a compiled grammar
grammar! {
    grammar TokenFormat {
        start tokens;
        terminals {
            IDENT: Ident,
            NUM: Num,
            COLON, AT, LT, GT,
        }

        tokens: Tokens = token* @collect;

        token: Tok = IDENT COLON value AT precedence @tok_val_prec
                   | IDENT AT precedence @tok_prec
                   | IDENT COLON value @tok_val
                   | IDENT @tok_name;

        value: Val = IDENT @val_ident | NUM @val_num;

        assoc: Assoc = LT @left | GT @right;
        precedence: Prec = assoc NUM @make_prec;
    }
}

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
    type Assoc = fn(u8) -> Precedence;
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

    fn left(&mut self) -> fn(u8) -> Precedence { Precedence::Left }
    fn right(&mut self) -> fn(u8) -> Precedence { Precedence::Right }
    fn make_prec(&mut self, assoc: fn(u8) -> Precedence, level: String) -> Precedence {
        assoc(level.parse().unwrap_or(10))
    }
}

fn parse_token_stream(input: &str) -> Result<Vec<Tok>, String> {
    let mut lexer = Lexer::new(input);
    let mut parser = TokenFormatParser::<TokActions>::new();
    let mut actions = TokActions;

    while let Some(result) = lexer.next() {
        let terminal = match result? {
            LexToken::Ident(s) => TokenFormatTerminal::IDENT(s),
            LexToken::Num(s) => TokenFormatTerminal::NUM(s),
            LexToken::Op(ref s) if s == ":" => TokenFormatTerminal::COLON,
            LexToken::Op(ref s) if s == "@" => TokenFormatTerminal::AT,
            LexToken::Op(ref s) if s == "<" => TokenFormatTerminal::LT,
            LexToken::Op(ref s) if s == ">" => TokenFormatTerminal::GT,
            LexToken::Op(s) => TokenFormatTerminal::IDENT(s),
            LexToken::Punct(c) => TokenFormatTerminal::IDENT(c.to_string()),
            _ => continue,
        };
        parser.push(terminal, &mut actions).map_err(|e| format!("{:?}", e))?;
    }
    parser.finish(&mut actions).map_err(|e| format!("{:?}", e))
}

/// Generic AST node for runtime-parsed grammars
enum Ast {
    Leaf(String, Option<String>),
    Node(String, Vec<Ast>),
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
                    _ => {
                        println!();
                        children[0].print(indent + 1);
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
        eprintln!("Reads token stream from stdin. Format: NAME:value@<prec");
        std::process::exit(1);
    }

    // Load grammar
    let src = std::fs::read_to_string(&args[1])
        .map_err(|e| format!("cannot read {}: {}", args[1], e))?;
    let grammar = parse_grammar(&src)?;
    let compiled = CompiledTable::build(&grammar);

    // Parse token stream
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).map_err(|e| e.to_string())?;
    let tokens = parse_token_stream(&input)?;

    // Convert to parser tokens
    let tokens: Vec<_> = tokens.iter().map(|tok| {
        let id = compiled.symbol_id(&tok.name)
            .ok_or_else(|| format!("unknown terminal: {}", tok.name))?;
        let token = match tok.prec {
            Some(p) => Token::with_prec(id, p),
            None => Token::new(id),
        };
        Ok((token, tok))
    }).collect::<Result<_, String>>()?;

    // Run parser
    let mut parser = Parser::new(compiled.table());
    let mut stack: Vec<Ast> = Vec::new();
    let mut i = 0;

    loop {
        let lookahead = tokens.get(i).map(|(t, _)| t);

        match parser.maybe_reduce(lookahead) {
            Ok(Some((0, _))) => break, // accept
            Ok(Some((rule, len))) => {
                let name = compiled.rule_name(rule)
                    .unwrap_or_else(|| compiled.symbol_name(compiled.table().rule_info(rule).0))
                    .to_string();
                let children: Vec<Ast> = stack.drain(stack.len() - len..).collect();
                stack.push(Ast::Node(name, children));
            }
            Ok(None) if i < tokens.len() => {
                let (token, tok) = &tokens[i];
                stack.push(Ast::Leaf(tok.name.clone(), tok.value.clone()));
                parser.shift(token);
                i += 1;
            }
            Ok(None) => return Err(format!("incomplete parse: {} items on stack", stack.len())),
            Err(e) => return Err(e.format(&compiled)),
        }
    }

    stack.pop().unwrap().print(0);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

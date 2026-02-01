//! Runtime Grammar CLI
//!
//! Parses a token stream using a grammar loaded at runtime.
//! The token stream format itself is parsed by a compiled grammar,
//! which directly drives the runtime parser - no intermediate storage.
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

// Token stream format - each @token action drives the runtime parser
grammar! {
    grammar TokenFormat {
        start tokens;
        terminals {
            IDENT: Val,
            NUM: Val,
            COLON, AT, LT, GT,
        }

        tokens = token*;
        token: Unit = IDENT colon_value? at_precedence? @token;

        colon_value: Val = COLON value;
        value: Val = IDENT | NUM;

        assoc: Assoc = LT @left | GT @right;
        at_precedence: Prec = AT assoc NUM @make_prec;
    }
}

struct Unit;

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

/// Actions that drive the runtime parser directly
struct Actions<'a> {
    compiled: &'a CompiledTable,
    parser: Parser<'a>,
    stack: Vec<Ast>,
}

impl Actions<'_> {
    fn reduce(&mut self, lookahead: Option<&Token>) {
        loop {
            match self.parser.maybe_reduce(lookahead) {
                Ok(Some((rule, len, _start_idx))) if rule > 0 => {
                    let name = self.compiled.rule_name(rule)
                        .unwrap_or_else(|| self.compiled.symbol_name(self.compiled.table().rule_info(rule).0))
                        .to_string();
                    let children: Vec<Ast> = self.stack.drain(self.stack.len() - len..).collect();
                    self.stack.push(Ast::Node(name, children));
                }
                Ok(_) => break,
                Err(e) => panic!("parse error: {}", e.format(self.compiled)),
            }
        }
    }
}

impl TokenFormatActions for Actions<'_> {
    type Val = String;
    type Assoc = fn(u8) -> Precedence;
    type Prec = Precedence;
    type Unit = Unit;

    // Each token action: reduce, shift - this IS the parse loop!
    fn token(&mut self, name: String, value: Option<String>, prec: Option<Precedence>) -> Unit {
        let id = self.compiled.symbol_id(&name).expect("unknown terminal");
        let token = match prec {
            Some(p) => Token::with_prec(id, p),
            None => Token::new(id),
        };

        self.reduce(Some(&token));
        self.stack.push(Ast::Leaf(name, value));
        self.parser.shift(&token);
        Unit
    }

    fn left(&mut self) -> fn(u8) -> Precedence { Precedence::Left }
    fn right(&mut self) -> fn(u8) -> Precedence { Precedence::Right }
    fn make_prec(&mut self, assoc: fn(u8) -> Precedence, level: String) -> Precedence {
        assoc(level.parse().unwrap_or(10))
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

    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).map_err(|e| e.to_string())?;

    // The token format parser drives the runtime parser via @token actions
    let mut actions = Actions {
        compiled: &compiled,
        parser: Parser::new(compiled.table()),
        stack: Vec::new(),
    };
    let mut lexer = Lexer::new(&input);
    let mut parser = TokenFormatParser::<Actions>::new();

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
    parser.finish(&mut actions).map_err(|e| format!("{:?}", e))?;

    // Final reductions (EOF)
    actions.reduce(None);

    if actions.stack.len() == 1 {
        actions.stack.pop().unwrap().print(0);
        Ok(())
    } else {
        Err(format!("incomplete parse: {} items on stack", actions.stack.len()))
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

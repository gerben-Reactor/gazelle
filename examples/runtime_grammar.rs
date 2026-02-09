//! Runtime Grammar CLI
//!
//! Parses a token stream using a grammar loaded at runtime.
//! The token stream format itself is parsed by a compiled grammar,
//! which directly drives the runtime parser - no intermediate storage.
//!
//! Usage:
//!   cargo run --example runtime_grammar <grammar.gzl> < tokens.txt
//!
//! Token format (space-separated, semicolon-separated expressions):
//!   NAME           - terminal with no value
//!   NAME:value     - terminal with value
//!   NAME@<5        - terminal with left-assoc precedence 5
//!   NAME:value@>3  - terminal with value and right-assoc precedence 3
//!   ;              - expression separator (prints result, resets parser)
//!
//! Example with included files:
//!   $ cat examples/expr_tokens.txt | cargo run --example runtime_grammar examples/expr.gzl
//!
//! Or inline:
//!   $ echo "NUM:1 OP:+@<1 NUM:2 OP:*@<2 NUM:3" | cargo run --example runtime_grammar examples/expr.gzl

use gazelle::lexer::Source;
use gazelle::runtime::{Parser, Token};
use gazelle::table::CompiledTable;
use gazelle::{parse_grammar, Precedence};
use gazelle_macros::grammar;
use std::io::{self, Read};

// Token stream format - each @token action drives the runtime parser
// Multiple expressions separated by SEMI, each printed separately
grammar! {
    grammar TokenFormat {
        start sentences;
        terminals {
            IDENT: Val,
            NUM: Val,
            COLON, AT, LT, GT, SEMI
        }

        sentences = sentence*;
        sentence = token* SEMI @sentence;
        token = IDENT colon_value? at_precedence? @token;

        colon_value: Val = COLON value;
        value: Val = IDENT | NUM;

        assoc: Assoc = LT @left | GT @right;
        at_precedence: Prec = AT assoc NUM @make_prec;
    }
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

/// Error type for runtime grammar actions.
#[derive(Debug)]
enum ActionError {
    Parse(gazelle::ParseError),
    Runtime(String),
}

impl From<gazelle::ParseError> for ActionError {
    fn from(e: gazelle::ParseError) -> Self { ActionError::Parse(e) }
}

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionError::Parse(e) => write!(f, "{}", e),
            ActionError::Runtime(s) => write!(f, "{}", s),
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
    fn reduce(&mut self, lookahead: Option<&Token>) -> Result<(), ActionError> {
        loop {
            match self.parser.maybe_reduce(lookahead) {
                Ok(Some((rule, len, _start_idx))) if rule > 0 => {
                    let name = [self.compiled.rule_name(rule)
                        .unwrap_or(&""), self.compiled.symbol_name(self.compiled.table().rule_info(rule).0)].join(":");
                    let children: Vec<Ast> = self.stack.drain(self.stack.len() - len..).collect();
                    self.stack.push(Ast::Node(name, children));
                }
                Ok(_) => break,
                Err(e) => return Err(ActionError::Runtime(
                    self.parser.format_error(&e, self.compiled)
                )),
            }
        }
        Ok(())
    }
}

impl TokenFormatTypes for Actions<'_> {
    type Val = String;
    type Assoc = fn(u8) -> Precedence;
    type Prec = Precedence;
}

impl TokenFormatActions<ActionError> for Actions<'_> {
    fn token(&mut self, name: String, value: Option<String>, prec: Option<Precedence>) -> Result<(), ActionError> {
        let id = self.compiled.symbol_id(&name)
            .ok_or_else(|| ActionError::Runtime(format!("unknown terminal '{}'", name)))?;
        let token = match prec {
            Some(p) => Token::with_prec(id, p),
            None => Token::new(id),
        };

        self.reduce(Some(&token))?;
        self.stack.push(Ast::Leaf(name, value));
        self.parser.shift(&token);
        Ok(())
    }

    fn sentence(&mut self, _:Vec<()>) -> Result<(), ActionError> {
        self.reduce(None)?;
        if self.stack.len() == 1 {
            self.stack.pop().unwrap().print(0);
            println!();
        } else if !self.stack.is_empty() {
            eprintln!("incomplete parse: {} items on stack", self.stack.len());
        }
        self.parser = Parser::new(self.compiled.table());
        self.stack.clear();
        Ok(())
    }

    fn left(&mut self) -> Result<fn(u8) -> Precedence, ActionError> { Ok(Precedence::Left) }
    fn right(&mut self) -> Result<fn(u8) -> Precedence, ActionError> { Ok(Precedence::Right) }
    fn make_prec(&mut self, assoc: fn(u8) -> Precedence, level: String) -> Result<Precedence, ActionError> {
        Ok(assoc(level.parse().unwrap_or(10)))
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
    let mut src = Source::from_str(&input);
    let mut parser = TokenFormatParser::<Actions, ActionError>::new();

    loop {
        src.skip_whitespace();
        while src.skip_line_comment("//") || src.skip_block_comment("/*", "*/") {
            src.skip_whitespace();
        }
        if src.at_end() {
            break;
        }

        let terminal = if let Some(span) = src.read_ident() {
            TokenFormatTerminal::IDENT(input[span].to_string())
        } else if let Some(span) = src.read_number() {
            TokenFormatTerminal::NUM(input[span].to_string())
        } else if let Some(c) = src.peek() {
            src.advance();
            match c {
                ':' => TokenFormatTerminal::COLON,
                '@' => TokenFormatTerminal::AT,
                '<' => TokenFormatTerminal::LT,
                '>' => TokenFormatTerminal::GT,
                ';' => TokenFormatTerminal::SEMI,
                _ => TokenFormatTerminal::IDENT(c.to_string()),
            }
        } else {
            break;
        };

        parser.push(terminal, &mut actions).map_err(|e|
            match e {
                ActionError::Parse(e) => format!("parse error: {}", parser.format_error(&e)),
                ActionError::Runtime(e) => format!("action error: {}", e),
            }
        )?;
    }
    parser.finish(&mut actions).map_err(|(p, e)|
        match e {
            ActionError::Parse(e) => format!("parse error at end: {}", p.format_error(&e)),
            ActionError::Runtime(e) => format!("action error at end: {}", e),
        }
    )
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

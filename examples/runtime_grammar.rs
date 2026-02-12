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
use gazelle::runtime::{Cst, Token, CstParser};
use gazelle::table::CompiledTable;
use gazelle::{Precedence, parse_grammar};
use gazelle_macros::gazelle;
use std::io::{self, Read};

// Token stream format - each @token action drives the runtime parser
// Multiple expressions separated by SEMI, each printed separately
gazelle! {
    grammar TokenFormat {
        start sentences;
        terminals {
            IDENT: Val,
            NUM: Val,
            COLON, AT, LT, GT, SEMI
        }

        sentences = sentence*;
        sentence = tokens SEMI @sentence;
        tokens: Parser = _ @new_parser | tokens token @push_token;
        token: Token = IDENT colon_value? at_precedence? @token;

        colon_value: Val = COLON value;
        value: Val = IDENT | NUM;

        assoc: Assoc = LT @left | GT @right;
        at_precedence: Prec = AT assoc NUM @make_prec;
    }
}

fn print_tree(tree: &Cst, indent: usize, compiled: &CompiledTable, values: &[Option<String>]) {
    let pad = "  ".repeat(indent);
    match *tree {
        Cst::Leaf(id, idx) => match values.get(idx).and_then(|v| v.as_ref()) {
            Some(v) => println!("{}{}:{}", pad, compiled.symbol_name(id), v),
            None => println!("{}{}", pad, compiled.symbol_name(id)),
        },
        Cst::Node(rule, ref children) => {
            let name = compiled.rule_name(rule).unwrap_or("?");
            println!("{}({}", pad, name);
            for c in children {
                print_tree(c, indent + 1, compiled, values);
            }
            println!("{})", pad);
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
}

struct RuntimeParser<'a> {
    cst: CstParser<'a>,
    values: Vec<Option<String>>,
}

impl<'a> TokenFormatTypes for Actions<'a> {
    type Val = String;
    type Assoc = fn(u8) -> Precedence;
    type Prec = Precedence;
    type Parser = RuntimeParser<'a>;
    type Token = (Token, Option<String>);
}

impl<'a> TokenFormatActions<ActionError> for Actions<'a> {
    fn token(&mut self, name: String, value: Option<String>, prec: Option<Precedence>) -> Result<Self::Token, ActionError> {
        let id = self.compiled.symbol_id(&name)
            .ok_or_else(|| ActionError::Runtime(format!("unknown terminal '{}'", name)))?;
        let token = match prec {
            Some(p) => Token::with_prec(id, p),
            None => Token::new(id),
        };
        Ok((token, value))
    }

    fn push_token(&mut self, mut parser: Self::Parser, (token, value): Self::Token) -> Result<Self::Parser, ActionError> {
        parser.values.push(value);
        parser.cst.push(token)?;
        Ok(parser)
    }

    fn new_parser(&mut self) -> Result<Self::Parser, ActionError> {
        Ok(RuntimeParser { cst: CstParser::new(self.compiled.table()), values: Vec::new() })
    }

    fn sentence(&mut self, parser: Self::Parser) -> Result<(), ActionError> {
        match parser.cst.finish() {
            Ok(tree) => {
                print_tree(&tree, 0, self.compiled, &parser.values);
                println!();
            }
            Err((_cst, e)) => return Err(e.into()),
        }
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
        } else if let Some(span) = src.read_digits() {
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

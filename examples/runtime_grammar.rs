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
use gazelle::{Ignore, Precedence, Reduce, parse_grammar};
use gazelle_macros::gazelle;
use std::io::{self, Read};

// Token stream format - each => token action drives the runtime parser
// Multiple expressions separated by SEMI, each printed separately
gazelle! {
    grammar TokenFormat {
        start sentences;
        terminals {
            IDENT: _,
            NUM: _,
            COLON, AT, LT, GT, SEMI
        }

        sentences = sentence* => sentences;
        sentence = tokens SEMI => sentence;
        tokens = _ => empty | tokens token => append;
        token = IDENT colon_value? at_precedence? => token;

        colon_value = COLON value => colon_value;
        value = IDENT => ident | NUM => num;

        assoc = LT => left | GT => right;
        at_precedence = AT assoc NUM => at_prec;
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
    type Error = ActionError;
    type Ident = String;
    type Num = String;
    // Identity types — ReduceNode blanket handles these
    type Assoc = TokenFormatAssoc;
    type At_precedence = TokenFormatAt_precedence<Self>;
    type Value = TokenFormatValue<Self>;
    type Colon_value = TokenFormatColon_value<Self>;
    // Identity types
    type Token = TokenFormatToken<Self>;
    // Custom types
    type Tokens = RuntimeParser<'a>;
    type Sentences = Ignore;
    type Sentence = ();
}

impl<'a> Reduce<TokenFormatSentence<Self>, (), ActionError> for Actions<'a> {
    fn reduce(&mut self, node: TokenFormatSentence<Self>) -> Result<(), ActionError> {
        let TokenFormatSentence::Sentence(parser) = node;
        match parser.cst.finish() {
            Ok(tree) => {
                print_tree(&tree, 0, self.compiled, &parser.values);
                println!();
            }
            Err((_cst, e)) => return Err(e.into()),
        }
        Ok(())
    }
}

impl<'a> Reduce<TokenFormatTokens<Self>, RuntimeParser<'a>, ActionError> for Actions<'a> {
    fn reduce(&mut self, node: TokenFormatTokens<Self>) -> Result<RuntimeParser<'a>, ActionError> {
        match node {
            TokenFormatTokens::Empty => {
                Ok(RuntimeParser { cst: CstParser::new(self.compiled.table()), values: Vec::new() })
            }
            TokenFormatTokens::Append(mut parser, token_cst) => {
                let TokenFormatToken::Token(name, colon_value, at_prec) = token_cst;

                let value = colon_value.map(|TokenFormatColon_value::Colon_value(v)| match v {
                    TokenFormatValue::Ident(s) | TokenFormatValue::Num(s) => s,
                });

                let prec = at_prec.map(|TokenFormatAt_precedence::At_prec(assoc, level)| {
                    let level: u8 = level.parse().unwrap_or(10);
                    match assoc {
                        TokenFormatAssoc::Left => Precedence::Left(level),
                        TokenFormatAssoc::Right => Precedence::Right(level),
                    }
                });

                let id = self.compiled.symbol_id(&name)
                    .ok_or_else(|| ActionError::Runtime(format!("unknown terminal '{}'", name)))?;
                let token = match prec {
                    Some(p) => Token::with_prec(id, p),
                    None => Token::new(id),
                };

                parser.values.push(value);
                parser.cst.push(token)?;
                Ok(parser)
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

    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).map_err(|e| e.to_string())?;

    // The token format parser drives the runtime parser via => token actions
    let mut actions = Actions {
        compiled: &compiled,
    };
    let mut src = Source::from_str(&input);
    let mut parser = TokenFormatParser::<Actions>::new();

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
    )?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

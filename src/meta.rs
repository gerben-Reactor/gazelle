//! Meta-grammar: parse grammar definitions using Gazelle itself.
//!
//! Grammar syntax:
//! ```text
//! rule = IDENT '=' alts ';'
//! alts = seq ('|' seq)*
//! seq = symbol+
//! symbol = IDENT | STRING | '<' IDENT '>'
//! ```
//!
//! - IDENT: non-terminal (alphanumeric, starts with letter)
//! - STRING: terminal (single-quoted, e.g., '+', 'if')
//! - '<' IDENT '>': precedence terminal (e.g., <OP>)

use crate::grammar::{Grammar, Rule, Symbol, nt, t, pt};
use crate::lexer::{self, Token as LexToken};
use crate::lr::Automaton;
use crate::table::ParseTable;
use crate::runtime::{Parser, Token, Event};

/// Tokens for the grammar syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarToken {
    Ident(String),  // non-terminal name
    String(String), // terminal (single-quoted)
    Lt,             // '<'
    Gt,             // '>'
    Eq,             // '='
    Pipe,           // '|'
    Semi,           // ';'
}

impl GrammarToken {
    fn to_terminal(&self) -> Symbol {
        match self {
            GrammarToken::Ident(_) => t("IDENT"),
            GrammarToken::String(_) => t("STRING"),
            GrammarToken::Lt => t("<"),
            GrammarToken::Gt => t(">"),
            GrammarToken::Eq => t("="),
            GrammarToken::Pipe => t("|"),
            GrammarToken::Semi => t(";"),
        }
    }
}

/// Lex grammar syntax using the general lexer.
pub fn lex_grammar(input: &str) -> Result<Vec<GrammarToken>, String> {
    let lex_tokens = lexer::lex(input)?;
    let mut tokens = Vec::new();

    for tok in lex_tokens {
        match tok {
            LexToken::Ident(s) => tokens.push(GrammarToken::Ident(s)),
            LexToken::Str(s) => tokens.push(GrammarToken::String(s)),
            LexToken::Op(s) => {
                // Split operators into single chars for grammar syntax
                for c in s.chars() {
                    match c {
                        '=' => tokens.push(GrammarToken::Eq),
                        '|' => tokens.push(GrammarToken::Pipe),
                        '<' => tokens.push(GrammarToken::Lt),
                        '>' => tokens.push(GrammarToken::Gt),
                        _ => return Err(format!("Unexpected operator in grammar: {}", c)),
                    }
                }
            }
            LexToken::Punct(c) => match c {
                ';' => tokens.push(GrammarToken::Semi),
                _ => return Err(format!("Unexpected punctuation in grammar: {}", c)),
            },
            LexToken::Num(s) => return Err(format!("Unexpected number in grammar: {}", s)),
        }
    }

    Ok(tokens)
}

/// Build the meta-grammar for parsing grammar definitions.
fn meta_grammar() -> Grammar {
    // grammar = rules
    // rules = rules rule | rule
    // rule = IDENT '=' alts ';'
    // alts = alts '|' seq | seq
    // seq = seq symbol | symbol
    // symbol = IDENT | STRING | '<' IDENT '>'
    Grammar {
        start: nt("grammar"),
        rules: vec![
            // grammar = rules
            Rule { lhs: nt("grammar"), rhs: vec![nt("rules")] },
            // rules = rules rule | rule
            Rule { lhs: nt("rules"), rhs: vec![nt("rules"), nt("rule")] },
            Rule { lhs: nt("rules"), rhs: vec![nt("rule")] },
            // rule = IDENT '=' alts ';'
            Rule { lhs: nt("rule"), rhs: vec![t("IDENT"), t("="), nt("alts"), t(";")] },
            // alts = alts '|' seq | seq
            Rule { lhs: nt("alts"), rhs: vec![nt("alts"), t("|"), nt("seq")] },
            Rule { lhs: nt("alts"), rhs: vec![nt("seq")] },
            // seq = seq symbol | symbol
            Rule { lhs: nt("seq"), rhs: vec![nt("seq"), nt("symbol")] },
            Rule { lhs: nt("seq"), rhs: vec![nt("symbol")] },
            // symbol = IDENT | STRING | '<' IDENT '>'
            Rule { lhs: nt("symbol"), rhs: vec![t("IDENT")] },
            Rule { lhs: nt("symbol"), rhs: vec![t("STRING")] },
            Rule { lhs: nt("symbol"), rhs: vec![t("<"), t("IDENT"), t(">")] },
        ],
    }
}

/// Parsed representation before conversion to Grammar.
#[derive(Debug)]
enum Ast {
    Ident(String),
    String(String),
    PrecString(String),  // precedence-carrying terminal
    Seq(Vec<Ast>),
    Alts(Vec<Ast>),
    Rule { name: String, alts: Box<Ast> },
    Rules(Vec<Ast>),
    Grammar(Box<Ast>),
}

/// Parse a grammar string into a Grammar.
pub fn parse_grammar(input: &str) -> Result<Grammar, String> {
    let tokens = lex_grammar(input)?;
    if tokens.is_empty() {
        return Err("Empty grammar".to_string());
    }

    let meta = meta_grammar();
    let automaton = Automaton::build(&meta);
    let table = ParseTable::build(&automaton);

    if table.has_conflicts() {
        return Err(format!("Meta-grammar has conflicts: {:?}", table.conflicts));
    }

    let mut parser = Parser::new(&table);
    let mut stack: Vec<(Ast, GrammarToken)> = Vec::new();

    // Push each token
    for tok in &tokens {
        let terminal = tok.to_terminal();
        let parser_token = Token::new(terminal, "");

        for event in parser.push(&parser_token) {
            match event {
                Event::Reduce { rule, len, .. } => {
                    reduce(&mut stack, rule, len, tok)?;
                }
                Event::Error { state, .. } => {
                    return Err(format!("Parse error at {:?}, state {}", tok, state));
                }
                Event::Accept => {}
            }
        }

        // Push current token onto stack
        let ast = match tok {
            GrammarToken::Ident(s) => Ast::Ident(s.clone()),
            GrammarToken::String(s) => Ast::String(s.clone()),
            _ => Ast::Ident(String::new()), // placeholder for punctuation
        };
        stack.push((ast, tok.clone()));
    }

    // Finish parsing
    for event in parser.finish() {
        match event {
            Event::Reduce { rule, len, .. } => {
                reduce(&mut stack, rule, len, &GrammarToken::Semi)?;
            }
            Event::Error { state, .. } => {
                return Err(format!("Parse error at end, state {}", state));
            }
            Event::Accept => {}
        }
    }

    // Convert AST to Grammar
    if stack.len() != 1 {
        return Err(format!("Parse incomplete, stack has {} items", stack.len()));
    }

    let (ast, _) = stack.pop().unwrap();
    ast_to_grammar(ast)
}

fn reduce(stack: &mut Vec<(Ast, GrammarToken)>, rule: usize, len: usize, _current_tok: &GrammarToken) -> Result<(), String> {
    // Rule indices in augmented grammar (rule 0 is __start -> grammar):
    // 1: grammar = rules
    // 2: rules = rules rule
    // 3: rules = rule
    // 4: rule = IDENT '=' alts ';'
    // 5: alts = alts '|' seq
    // 6: alts = seq
    // 7: seq = seq symbol
    // 8: seq = symbol
    // 9: symbol = IDENT
    // 10: symbol = STRING
    // 11: symbol = '<' IDENT '>'

    let mut children: Vec<(Ast, GrammarToken)> = Vec::new();
    for _ in 0..len {
        if let Some(item) = stack.pop() {
            children.push(item);
        }
    }
    children.reverse();

    let ast = match rule {
        0 => return Ok(()), // __start -> grammar (accept)
        1 => {
            // grammar = rules
            Ast::Grammar(Box::new(children.remove(0).0))
        }
        2 => {
            // rules = rules rule
            let mut rules = match children.remove(0).0 {
                Ast::Rules(r) => r,
                other => vec![other],
            };
            rules.push(children.remove(0).0);
            Ast::Rules(rules)
        }
        3 => {
            // rules = rule
            Ast::Rules(vec![children.remove(0).0])
        }
        4 => {
            // rule = IDENT '=' alts ';'
            let name = match &children[0].0 {
                Ast::Ident(s) => s.clone(),
                _ => return Err("Expected IDENT".to_string()),
            };
            let alts = children.remove(2).0;
            Ast::Rule { name, alts: Box::new(alts) }
        }
        5 => {
            // alts = alts '|' seq
            let mut alts = match children.remove(0).0 {
                Ast::Alts(a) => a,
                other => vec![other],
            };
            alts.push(children.remove(1).0); // skip '|'
            Ast::Alts(alts)
        }
        6 => {
            // alts = seq
            Ast::Alts(vec![children.remove(0).0])
        }
        7 => {
            // seq = seq symbol
            let mut seq = match children.remove(0).0 {
                Ast::Seq(s) => s,
                other => vec![other],
            };
            seq.push(children.remove(0).0);
            Ast::Seq(seq)
        }
        8 => {
            // seq = symbol
            Ast::Seq(vec![children.remove(0).0])
        }
        9 => {
            // symbol = IDENT
            children.remove(0).0
        }
        10 => {
            // symbol = STRING
            children.remove(0).0
        }
        11 => {
            // symbol = '<' IDENT '>'
            // children: [<, IDENT, >] - extract the IDENT in the middle
            let name = match &children[1].0 {
                Ast::Ident(s) => s.clone(),
                _ => return Err("Expected IDENT in <...>".to_string()),
            };
            Ast::PrecString(name)
        }
        _ => return Err(format!("Unknown rule {}", rule)),
    };

    let dummy_tok = GrammarToken::Ident(String::new());
    stack.push((ast, dummy_tok));
    Ok(())
}

fn ast_to_grammar(ast: Ast) -> Result<Grammar, String> {
    let rules_ast = match ast {
        Ast::Grammar(inner) => *inner,
        other => other,
    };

    let rule_asts = match rules_ast {
        Ast::Rules(r) => r,
        other => vec![other],
    };

    let mut rules = Vec::new();
    let mut start: Option<Symbol> = None;

    for rule_ast in rule_asts {
        let (name, alts_ast) = match rule_ast {
            Ast::Rule { name, alts } => (name, *alts),
            _ => return Err("Expected Rule".to_string()),
        };

        if start.is_none() {
            start = Some(nt(&name));
        }

        let alts = match alts_ast {
            Ast::Alts(a) => a,
            other => vec![other],
        };

        for alt in alts {
            let seq = match alt {
                Ast::Seq(s) => s,
                other => vec![other],
            };

            let rhs: Vec<Symbol> = seq.into_iter().map(|sym| {
                match sym {
                    Ast::Ident(s) => nt(&s),
                    Ast::String(s) => t(&s),
                    Ast::PrecString(s) => pt(&s),
                    _ => t("?"),
                }
            }).collect();

            rules.push(Rule { lhs: nt(&name), rhs });
        }
    }

    Ok(Grammar {
        start: start.ok_or("No rules")?,
        rules,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let tokens = lex_grammar("expr = expr '+' term | term ;").unwrap();
        assert_eq!(tokens.len(), 8);
        assert!(matches!(tokens[0], GrammarToken::Ident(ref s) if s == "expr"));
        assert!(matches!(tokens[2], GrammarToken::Ident(ref s) if s == "expr"));
        assert!(matches!(tokens[3], GrammarToken::String(ref s) if s == "+"));
    }

    #[test]
    fn test_parse_simple() {
        let grammar = parse_grammar("S = 'a' ;").unwrap();
        assert_eq!(grammar.rules.len(), 1);
        assert_eq!(grammar.start, nt("S"));
        assert_eq!(grammar.rules[0].rhs, vec![t("a")]);
    }

    #[test]
    fn test_parse_expr_grammar() {
        let grammar = parse_grammar(r#"
            expr = expr '+' term | term ;
            term = 'NUM' ;
        "#).unwrap();

        assert_eq!(grammar.rules.len(), 3);
        assert_eq!(grammar.start, nt("expr"));

        // expr -> expr '+' term
        assert_eq!(grammar.rules[0].lhs, nt("expr"));
        assert_eq!(grammar.rules[0].rhs, vec![nt("expr"), t("+"), nt("term")]);

        // expr -> term
        assert_eq!(grammar.rules[1].lhs, nt("expr"));
        assert_eq!(grammar.rules[1].rhs, vec![nt("term")]);

        // term -> NUM
        assert_eq!(grammar.rules[2].lhs, nt("term"));
        assert_eq!(grammar.rules[2].rhs, vec![t("NUM")]);
    }

    #[test]
    fn test_roundtrip() {
        // Parse a grammar, build parser, use it
        let grammar = parse_grammar(r#"
            expr = expr '+' term | term ;
            term = 'NUM' ;
        "#).unwrap();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        assert!(!table.has_conflicts());

        let mut parser = Parser::new(&table);

        // Parse "NUM + NUM"
        let events = parser.push(&Token::new(t("NUM"), "1"));
        assert!(events.is_empty());

        let events = parser.push(&Token::new(t("+"), "+"));
        assert!(!events.is_empty()); // reductions

        let events = parser.push(&Token::new(t("NUM"), "2"));
        assert!(events.is_empty());

        let events = parser.finish();
        assert!(events.iter().any(|e| matches!(e, Event::Accept)));
    }
}

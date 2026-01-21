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

use crate::grammar::{Grammar, GrammarBuilder, Rule, Symbol};
use crate::lexer::{self, Token as LexToken};
use crate::lr::Automaton;
use crate::table::ParseTable;
use crate::runtime::{Parser, Token, Event};

/// Tokens for the grammar syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarToken {
    Ident(String),
    String(String),
    Lt,
    Gt,
    Eq,
    Pipe,
    Semi,
}

impl GrammarToken {
    fn terminal_name(&self) -> &str {
        match self {
            GrammarToken::Ident(_) => "IDENT",
            GrammarToken::String(_) => "STRING",
            GrammarToken::Lt => "<",
            GrammarToken::Gt => ">",
            GrammarToken::Eq => "=",
            GrammarToken::Pipe => "|",
            GrammarToken::Semi => ";",
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
    let mut gb = GrammarBuilder::new();

    // Terminals
    let ident = gb.t("IDENT");
    let string = gb.t("STRING");
    let lt = gb.t("<");
    let gt = gb.t(">");
    let eq = gb.t("=");
    let pipe = gb.t("|");
    let semi = gb.t(";");

    // Non-terminals
    let grammar_nt = gb.nt("grammar");
    let rules = gb.nt("rules");
    let rule = gb.nt("rule");
    let alts = gb.nt("alts");
    let seq = gb.nt("seq");
    let symbol = gb.nt("symbol");

    // grammar = rules
    gb.rule(grammar_nt, vec![rules]);
    // rules = rules rule | rule
    gb.rule(rules, vec![rules, rule]);
    gb.rule(rules, vec![rule]);
    // rule = IDENT '=' alts ';'
    gb.rule(rule, vec![ident, eq, alts, semi]);
    // alts = alts '|' seq | seq
    gb.rule(alts, vec![alts, pipe, seq]);
    gb.rule(alts, vec![seq]);
    // seq = seq symbol | symbol
    gb.rule(seq, vec![seq, symbol]);
    gb.rule(seq, vec![symbol]);
    // symbol = IDENT | STRING | '<' IDENT '>'
    gb.rule(symbol, vec![ident]);
    gb.rule(symbol, vec![string]);
    gb.rule(symbol, vec![lt, ident, gt]);

    gb.build()
}

/// Parsed representation before conversion to Grammar.
#[derive(Debug)]
enum Ast {
    Ident(String),
    String(String),
    PrecString(String),
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

    for tok in &tokens {
        let terminal_name = tok.terminal_name();
        let terminal_id = table.symbol_id(terminal_name)
            .ok_or_else(|| format!("Unknown terminal: {}", terminal_name))?;
        let parser_token = Token::new(terminal_id, "");

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

        let ast = match tok {
            GrammarToken::Ident(s) => Ast::Ident(s.clone()),
            GrammarToken::String(s) => Ast::String(s.clone()),
            _ => Ast::Ident(String::new()),
        };
        stack.push((ast, tok.clone()));
    }

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

    if stack.len() != 1 {
        return Err(format!("Parse incomplete, stack has {} items", stack.len()));
    }

    let (ast, _) = stack.pop().unwrap();
    ast_to_grammar(ast)
}

fn reduce(stack: &mut Vec<(Ast, GrammarToken)>, rule: usize, len: usize, _current_tok: &GrammarToken) -> Result<(), String> {
    let mut children: Vec<(Ast, GrammarToken)> = Vec::new();
    for _ in 0..len {
        if let Some(item) = stack.pop() {
            children.push(item);
        }
    }
    children.reverse();

    let ast = match rule {
        0 => return Ok(()),
        1 => Ast::Grammar(Box::new(children.remove(0).0)),
        2 => {
            let mut rules = match children.remove(0).0 {
                Ast::Rules(r) => r,
                other => vec![other],
            };
            rules.push(children.remove(0).0);
            Ast::Rules(rules)
        }
        3 => Ast::Rules(vec![children.remove(0).0]),
        4 => {
            let name = match &children[0].0 {
                Ast::Ident(s) => s.clone(),
                _ => return Err("Expected IDENT".to_string()),
            };
            let alts = children.remove(2).0;
            Ast::Rule { name, alts: Box::new(alts) }
        }
        5 => {
            let mut alts = match children.remove(0).0 {
                Ast::Alts(a) => a,
                other => vec![other],
            };
            alts.push(children.remove(1).0);
            Ast::Alts(alts)
        }
        6 => Ast::Alts(vec![children.remove(0).0]),
        7 => {
            let mut seq = match children.remove(0).0 {
                Ast::Seq(s) => s,
                other => vec![other],
            };
            seq.push(children.remove(0).0);
            Ast::Seq(seq)
        }
        8 => Ast::Seq(vec![children.remove(0).0]),
        9 => children.remove(0).0,
        10 => children.remove(0).0,
        11 => {
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

    let mut gb = GrammarBuilder::new();
    let mut rule_data: Vec<(String, Vec<Vec<Ast>>)> = Vec::new();

    // First pass: collect all rule names and their alternatives
    for rule_ast in rule_asts {
        let (name, alts_ast) = match rule_ast {
            Ast::Rule { name, alts } => (name, *alts),
            _ => return Err("Expected Rule".to_string()),
        };

        let alts = match alts_ast {
            Ast::Alts(a) => a,
            other => vec![other],
        };

        let mut alt_seqs = Vec::new();
        for alt in alts {
            let seq = match alt {
                Ast::Seq(s) => s,
                other => vec![other],
            };
            alt_seqs.push(seq);
        }
        rule_data.push((name, alt_seqs));
    }

    // Second pass: intern all terminals first
    for (_, alt_seqs) in &rule_data {
        for seq in alt_seqs {
            for sym in seq {
                match sym {
                    Ast::String(s) => { gb.t(s); }
                    Ast::PrecString(s) => { gb.pt(s); }
                    _ => {}
                }
            }
        }
    }

    // Third pass: intern non-terminals and build rules
    let mut start: Option<Symbol> = None;
    let mut rules: Vec<Rule> = Vec::new();

    for (name, alt_seqs) in &rule_data {
        let lhs = gb.nt(name);
        if start.is_none() {
            start = Some(lhs);
        }

        for seq in alt_seqs {
            let rhs: Vec<Symbol> = seq.iter().map(|sym| {
                match sym {
                    Ast::Ident(s) => gb.nt(s),
                    Ast::String(s) => gb.symbols.get(s).unwrap(),
                    Ast::PrecString(s) => gb.symbols.get(s).unwrap(),
                    _ => panic!("Unexpected AST node"),
                }
            }).collect();

            rules.push(Rule { lhs, rhs });
        }
    }

    gb.start(start.ok_or("No rules")?);
    for rule in rules {
        gb.rules.push(rule);
    }

    Ok(gb.build())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::Symbol;

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

        let s_sym = grammar.symbols.get("S").unwrap();
        let a_sym = grammar.symbols.get("a").unwrap();

        assert_eq!(grammar.start, s_sym);
        assert_eq!(grammar.rules[0].rhs, vec![a_sym]);
    }

    #[test]
    fn test_parse_expr_grammar() {
        let grammar = parse_grammar(r#"
            expr = expr '+' term | term ;
            term = 'NUM' ;
        "#).unwrap();

        assert_eq!(grammar.rules.len(), 3);

        let expr = grammar.symbols.get("expr").unwrap();
        let term = grammar.symbols.get("term").unwrap();
        let plus = grammar.symbols.get("+").unwrap();
        let num = grammar.symbols.get("NUM").unwrap();

        assert_eq!(grammar.start, expr);

        // expr -> expr '+' term
        assert_eq!(grammar.rules[0].lhs, expr);
        assert_eq!(grammar.rules[0].rhs, vec![expr, plus, term]);

        // expr -> term
        assert_eq!(grammar.rules[1].lhs, expr);
        assert_eq!(grammar.rules[1].rhs, vec![term]);

        // term -> NUM
        assert_eq!(grammar.rules[2].lhs, term);
        assert_eq!(grammar.rules[2].rhs, vec![num]);
    }

    #[test]
    fn test_roundtrip() {
        let grammar = parse_grammar(r#"
            expr = expr '+' term | term ;
            term = 'NUM' ;
        "#).unwrap();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        assert!(!table.has_conflicts());

        let mut parser = Parser::new(&table);

        let num_id = table.symbol_id("NUM").unwrap();
        let plus_id = table.symbol_id("+").unwrap();

        let events = parser.push(&Token::new(num_id, "1"));
        assert!(events.is_empty());

        let events = parser.push(&Token::new(plus_id, "+"));
        assert!(!events.is_empty());

        let _events = parser.push(&Token::new(num_id, "2"));

        let events = parser.finish();
        assert!(events.iter().any(|e| matches!(e, Event::Accept)));
    }

    #[test]
    fn test_prec_terminal() {
        let grammar = parse_grammar("expr = expr <OP> expr | 'NUM' ;").unwrap();

        let op = grammar.symbols.get("OP").unwrap();
        assert!(matches!(op, Symbol::PrecTerminal(_)));
    }
}

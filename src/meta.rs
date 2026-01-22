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

use crate::grammar::{Grammar, GrammarBuilder, Symbol};
use crate::lexer::{self, Token as LexToken};

// Use the grammar! macro to define the meta-grammar parser
crate::grammar! {
    grammar Meta {
        terminals {
            IDENT: String,
            STRING: String,
            LT,    // <
            GT,    // >
            EQ,    // =
            PIPE,  // |
            SEMI,  // ;
        }

        grammar_: Ast = rules;
        rules: Ast = rules rule | rule;
        rule: Ast = IDENT EQ alts SEMI;
        alts: Ast = alts PIPE seq | seq;
        seq: Ast = seq symbol | symbol;
        symbol: Ast = IDENT | STRING | LT IDENT GT;
    }
}

/// Parsed representation before conversion to Grammar.
#[derive(Debug, Clone)]
pub enum Ast {
    Ident(String),
    String(String),
    PrecString(String),
    Seq(Vec<Ast>),
    Alts(Vec<Ast>),
    Rule { name: String, alts: Box<Ast> },
    Rules(Vec<Ast>),
    Grammar(Box<Ast>),
}

/// Lex grammar syntax using the general lexer.
pub fn lex_grammar(input: &str) -> Result<Vec<MetaTerminal>, String> {
    let lex_tokens = lexer::lex(input)?;
    let mut tokens = Vec::new();

    for tok in lex_tokens {
        match tok {
            LexToken::Ident(s) => tokens.push(MetaTerminal::Ident(s)),
            LexToken::Str(s) => tokens.push(MetaTerminal::String(s)),
            LexToken::Op(s) => {
                for c in s.chars() {
                    match c {
                        '=' => tokens.push(MetaTerminal::Eq),
                        '|' => tokens.push(MetaTerminal::Pipe),
                        '<' => tokens.push(MetaTerminal::Lt),
                        '>' => tokens.push(MetaTerminal::Gt),
                        _ => return Err(format!("Unexpected operator in grammar: {}", c)),
                    }
                }
            }
            LexToken::Punct(c) => match c {
                ';' => tokens.push(MetaTerminal::Semi),
                _ => return Err(format!("Unexpected punctuation in grammar: {}", c)),
            },
            LexToken::Num(s) => return Err(format!("Unexpected number in grammar: {}", s)),
        }
    }

    Ok(tokens)
}

/// Parse a grammar string into a Grammar.
pub fn parse_grammar(input: &str) -> Result<Grammar, String> {
    let tokens = lex_grammar(input)?;
    if tokens.is_empty() {
        return Err("Empty grammar".to_string());
    }

    let mut parser = MetaParser::new();
    let mut tokens = tokens.into_iter();

    loop {
        let tok = tokens.next();
        // Handle all reductions triggered by this lookahead
        while let Some(r) = parser.maybe_reduce(&tok) {
            parser.reduce(reduce(r));
        }
        if tok.is_none() {
            break;
        }
        // Consume the token
        parser.shift(tok.unwrap()).map_err(|e| format!("Parse error, state {}", e.state))?;
    }

    // Get the final result
    let ast = parser.accept().map_err(|e| format!("Parse error at end, state {}", e.state))?;
    ast_to_grammar(ast)
}

fn reduce(reduction: MetaReduction) -> __MetaReductionResult {
    match reduction {
        // grammar_ = rules
        MetaReduction::GrammarRules(c, rules) => {
            c(Ast::Grammar(Box::new(rules)))
        }
        // rules = rules rule
        MetaReduction::RulesRulesRule(c, rules_ast, rule_ast) => {
            let mut rules = match rules_ast {
                Ast::Rules(r) => r,
                other => vec![other],
            };
            rules.push(rule_ast);
            c(Ast::Rules(rules))
        }
        // rules = rule
        MetaReduction::RulesRule(c, rule_ast) => {
            c(Ast::Rules(vec![rule_ast]))
        }
        // rule = IDENT EQ alts SEMI
        MetaReduction::RuleIdentEqAltsSemi(c, name, alts) => {
            c(Ast::Rule { name, alts: Box::new(alts) })
        }
        // alts = alts PIPE seq
        MetaReduction::AltsAltsPipeSeq(c, alts_ast, seq_ast) => {
            let mut alts = match alts_ast {
                Ast::Alts(a) => a,
                other => vec![other],
            };
            alts.push(seq_ast);
            c(Ast::Alts(alts))
        }
        // alts = seq
        MetaReduction::AltsSeq(c, seq_ast) => {
            c(Ast::Alts(vec![seq_ast]))
        }
        // seq = seq symbol
        MetaReduction::SeqSeqSymbol(c, seq_ast, symbol_ast) => {
            let mut seq = match seq_ast {
                Ast::Seq(s) => s,
                other => vec![other],
            };
            seq.push(symbol_ast);
            c(Ast::Seq(seq))
        }
        // seq = symbol
        MetaReduction::SeqSymbol(c, symbol_ast) => {
            c(Ast::Seq(vec![symbol_ast]))
        }
        // symbol = IDENT
        MetaReduction::SymbolIdent(c, s) => {
            c(Ast::Ident(s))
        }
        // symbol = STRING
        MetaReduction::SymbolString(c, s) => {
            c(Ast::String(s))
        }
        // symbol = LT IDENT GT
        MetaReduction::SymbolLtIdentGt(c, name) => {
            c(Ast::PrecString(name))
        }
    }
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

    // Third pass: intern non-terminals
    let mut nt_symbols: Vec<(String, Symbol)> = Vec::new();
    for (name, _) in &rule_data {
        let lhs = gb.nt(name);
        nt_symbols.push((name.clone(), lhs));
    }

    // Fourth pass: build rules using the rule method
    for (name, alt_seqs) in &rule_data {
        let lhs = nt_symbols.iter().find(|(n, _)| n == name).map(|(_, s)| *s).unwrap();

        for seq in alt_seqs {
            let rhs: Vec<Symbol> = seq.iter().map(|sym| {
                match sym {
                    Ast::Ident(s) => nt_symbols.iter().find(|(n, _)| n == s).map(|(_, sym)| *sym).unwrap(),
                    Ast::String(s) => gb.symbols.get(s).unwrap(),
                    Ast::PrecString(s) => gb.symbols.get(s).unwrap(),
                    _ => panic!("Unexpected AST node"),
                }
            }).collect();

            gb.rule(lhs, rhs);
        }
    }

    Ok(gb.build())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lr::Automaton;
    use crate::table::ParseTable;
    use crate::runtime::{Parser, Token, Event};

    #[test]
    fn test_lex() {
        let tokens = lex_grammar("expr = expr '+' term | term ;").unwrap();
        assert_eq!(tokens.len(), 8);
        assert!(matches!(&tokens[0], MetaTerminal::Ident(s) if s == "expr"));
        assert!(matches!(&tokens[2], MetaTerminal::Ident(s) if s == "expr"));
        assert!(matches!(&tokens[3], MetaTerminal::String(s) if s == "+"));
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

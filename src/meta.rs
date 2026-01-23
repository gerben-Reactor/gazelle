//! Meta-grammar: parse grammar definitions using Gazelle itself.
//!
//! This module provides the high-level API for parsing grammar strings.
//! The actual parser is in `gazelle_core::meta_bootstrap`.

use crate::grammar::Grammar;
use crate::lexer::{self, Token as LexToken};

// Re-export types from core
pub use gazelle_core::meta_bootstrap::{AstBuilder, MetaTerminal, GrammarDef};

/// Lex grammar syntax using the general lexer.
fn lex_grammar(input: &str) -> Result<Vec<MetaTerminal>, String> {
    let lex_tokens = lexer::lex(input)?;
    let mut tokens = Vec::new();

    for tok in lex_tokens {
        match tok {
            LexToken::Ident(s) => {
                match s.as_str() {
                    "grammar" => tokens.push(MetaTerminal::KwGrammar),
                    "start" => tokens.push(MetaTerminal::KwStart),
                    "terminals" => tokens.push(MetaTerminal::KwTerminals),
                    "prec" => tokens.push(MetaTerminal::KwPrec),
                    _ => tokens.push(MetaTerminal::Ident(s)),
                }
            }
            LexToken::Str(s) => return Err(format!("Unexpected string literal: '{}'", s)),
            LexToken::Op(s) => {
                for c in s.chars() {
                    match c {
                        '=' => tokens.push(MetaTerminal::Eq),
                        '|' => tokens.push(MetaTerminal::Pipe),
                        ':' => tokens.push(MetaTerminal::Colon),
                        '@' => tokens.push(MetaTerminal::At),
                        _ => return Err(format!("Unexpected operator: {}", c)),
                    }
                }
            }
            LexToken::Punct(c) => match c {
                ';' => tokens.push(MetaTerminal::Semi),
                '{' => tokens.push(MetaTerminal::Lbrace),
                '}' => tokens.push(MetaTerminal::Rbrace),
                ',' => tokens.push(MetaTerminal::Comma),
                _ => return Err(format!("Unexpected punctuation: {}", c)),
            },
            LexToken::Num(s) => return Err(format!("Unexpected number: {}", s)),
        }
    }

    Ok(tokens)
}

/// Parse a grammar string into a Grammar.
pub fn parse_grammar(input: &str) -> Result<Grammar, String> {
    let grammar_def = parse_grammar_typed(input)?;
    gazelle_core::meta_bootstrap::grammar_def_to_grammar(grammar_def)
}

/// Parse a grammar string into a typed GrammarDef.
pub fn parse_grammar_typed(input: &str) -> Result<GrammarDef, String> {
    let tokens = lex_grammar(input)?;
    if tokens.is_empty() {
        return Err("Empty grammar".to_string());
    }
    gazelle_core::meta_bootstrap::parse_tokens_typed(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::Symbol;
    use crate::lr::Automaton;
    use crate::table::ParseTable;
    use crate::runtime::{Parser, Token, Event};

    #[test]
    fn test_lex() {
        let tokens = lex_grammar("grammar Test { start s; terminals { A } s: S = A; }").unwrap();
        assert!(matches!(&tokens[0], MetaTerminal::KwGrammar));
        assert!(matches!(&tokens[1], MetaTerminal::Ident(s) if s == "Test"));
        assert!(matches!(&tokens[2], MetaTerminal::Lbrace));
        assert!(matches!(&tokens[3], MetaTerminal::KwStart));
        assert!(matches!(&tokens[4], MetaTerminal::Ident(s) if s == "s"));
        assert!(matches!(&tokens[5], MetaTerminal::Semi));
        assert!(matches!(&tokens[6], MetaTerminal::KwTerminals));
    }

    #[test]
    fn test_parse_simple() {
        let grammar = parse_grammar(r#"
            grammar Simple {
                start s;
                terminals { A }
                s: S = A;
            }
        "#).unwrap();

        assert_eq!(grammar.rules.len(), 1);

        let s_sym = grammar.symbols.get("s").unwrap();
        let a_sym = grammar.symbols.get("A").unwrap();

        assert_eq!(grammar.start, s_sym);
        assert_eq!(grammar.rules[0].rhs, vec![a_sym]);
    }

    #[test]
    fn test_parse_expr_grammar() {
        let grammar = parse_grammar(r#"
            grammar Expr {
                start expr;
                terminals {
                    PLUS,
                    NUM
                }

                expr: Expr = expr PLUS term | term;
                term: Term = NUM;
            }
        "#).unwrap();

        assert_eq!(grammar.rules.len(), 3);
    }

    #[test]
    fn test_trailing_comma() {
        // Test that trailing commas are supported
        let grammar = parse_grammar(r#"
            grammar Test {
                start s;
                terminals {
                    A,
                    B,
                }
                s: S = A B;
            }
        "#).unwrap();

        assert_eq!(grammar.rules.len(), 1);
    }

    #[test]
    fn test_roundtrip() {
        let grammar = parse_grammar(r#"
            grammar Calc {
                start expr;
                terminals {
                    PLUS,
                    NUM,
                }

                expr: Expr = expr PLUS term | term;
                term: Term = NUM;
            }
        "#).unwrap();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        assert!(!table.has_conflicts());

        let mut parser = Parser::new(&table);

        let num_id = table.symbol_id("NUM").unwrap();
        let plus_id = table.symbol_id("PLUS").unwrap();

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
        let grammar = parse_grammar(r#"
            grammar Prec {
                start expr;
                terminals { NUM, prec OP: Operator }

                expr: Expr = expr OP expr | NUM;
            }
        "#).unwrap();

        let op = grammar.symbols.get("OP").unwrap();
        assert!(matches!(op, Symbol::PrecTerminal(_)));
    }

    #[test]
    fn test_terminals_with_types() {
        let grammar_def = parse_grammar_typed(r#"
            grammar Typed {
                start expr;
                terminals {
                    NUM: f64,
                    IDENT: String,
                    LPAREN,
                    RPAREN,
                }

                expr: Expr = NUM | IDENT | LPAREN expr RPAREN;
            }
        "#).unwrap();

        assert_eq!(grammar_def.name, "Typed");
        assert_eq!(grammar_def.terminals.len(), 4);
        assert_eq!(grammar_def.terminals[0].name, "NUM");
        assert_eq!(grammar_def.terminals[0].type_name, Some("f64".to_string()));
        assert_eq!(grammar_def.terminals[2].name, "LPAREN");
        assert_eq!(grammar_def.terminals[2].type_name, None);
    }

    #[test]
    fn test_named_reductions() {
        let grammar_def = parse_grammar_typed(r#"
            grammar Named {
                start expr;
                terminals {
                    NUM: f64,
                    LPAREN,
                    RPAREN,
                    prec OP: char,
                }

                expr: Expr = expr OP expr @binop
                           | NUM @literal
                           | LPAREN expr RPAREN;
            }
        "#).unwrap();

        // Get the rule
        let rule = &grammar_def.rules[0];

        assert_eq!(rule.name, "expr");
        assert_eq!(rule.result_type, Some("Expr".to_string()));
        assert_eq!(rule.alts.len(), 3);

        // First alt: expr OP expr @binop
        assert_eq!(rule.alts[0].symbols, vec!["expr", "OP", "expr"]);
        assert_eq!(rule.alts[0].name, Some("binop".to_string()));

        // Second alt: NUM @literal
        assert_eq!(rule.alts[1].symbols, vec!["NUM"]);
        assert_eq!(rule.alts[1].name, Some("literal".to_string()));

        // Third alt: LPAREN expr RPAREN (no name)
        assert_eq!(rule.alts[2].symbols, vec!["LPAREN", "expr", "RPAREN"]);
        assert_eq!(rule.alts[2].name, None);
    }

    #[test]
    fn test_rule_without_type() {
        let grammar_def = parse_grammar_typed(r#"
            grammar Untyped {
                start stmts;
                terminals { A, B, SEMI }

                stmts = stmts SEMI stmt | stmt | ;
                stmt = A | B;
            }
        "#).unwrap();

        // Find stmts rule (should have no type)
        let stmts_rule = grammar_def.rules.iter()
            .find(|r| r.name == "stmts")
            .unwrap();

        assert_eq!(stmts_rule.result_type, None);
    }

    #[test]
    fn test_named_empty_production() {
        let grammar_def = parse_grammar_typed(r#"
            grammar Optional {
                start item;
                terminals { KW_PREC, IDENT }

                prec_opt: PrecOpt = KW_PREC @has_prec | @no_prec;
                item: Item = prec_opt IDENT;
            }
        "#).unwrap();

        // Find prec_opt rule
        let prec_opt_rule = grammar_def.rules.iter()
            .find(|r| r.name == "prec_opt")
            .unwrap();

        assert_eq!(prec_opt_rule.result_type, Some("PrecOpt".to_string()));
        assert_eq!(prec_opt_rule.alts.len(), 2);

        // First alt: KW_PREC @has_prec
        assert_eq!(prec_opt_rule.alts[0].symbols, vec!["KW_PREC"]);
        assert_eq!(prec_opt_rule.alts[0].name, Some("has_prec".to_string()));

        // Second alt: @no_prec (empty production with name)
        assert_eq!(prec_opt_rule.alts[1].symbols, Vec::<String>::new());
        assert_eq!(prec_opt_rule.alts[1].name, Some("no_prec".to_string()));
    }
}

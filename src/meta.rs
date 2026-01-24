//! Meta-grammar: parse grammar definitions using Gazelle itself.
//!
//! This module provides the parser for Gazelle grammar syntax.
//! The parser is generated from `meta.gzl` using the CLI.
//!
//! To regenerate `meta_generated.rs`:
//! ```bash
//! cargo build --release
//! ./target/release/gazelle --rust meta.gzl > src/meta_generated.rs
//! ```

#![allow(dead_code)]

use crate as gazelle;
use crate::grammar::Grammar;
use crate::lexer::{self, Token as LexToken};

// Type alias for IDENT terminal payload
pub type Ident = String;

// ============================================================================
// Public AST types
// ============================================================================

#[derive(Debug, Clone)]
pub struct GrammarDef {
    pub name: String,
    pub start: String,
    pub terminals: Vec<TerminalDef>,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct TerminalDef {
    pub name: String,
    pub type_name: Option<String>,
    pub is_prec: bool,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub result_type: Option<String>,
    pub alts: Alts,
}

/// A list of alternatives for a rule.
pub type Alts = Vec<Alt>;

#[derive(Debug, Clone)]
pub struct Alt {
    pub symbols: Vec<String>,
    pub name: Option<String>,
}

pub type Seq = Vec<String>;

// ============================================================================
// Intermediate parsing types
// ============================================================================

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct TerminalList(Vec<TerminalDef>);

pub type Rules = Vec<Rule>;

// ============================================================================
// Generated parser
// ============================================================================

include!("meta_generated.rs");

// ============================================================================
// AST builder implementing MetaActions
// ============================================================================

pub struct AstBuilder;

impl MetaActions for AstBuilder {
    // Terminal types
    type Ident = Ident;

    // Non-terminal types
    type GrammarDef = GrammarDef;
    type Rules = Rules;
    type TerminalsBlock = Vec<TerminalDef>;
    type TerminalList = TerminalList;
    type TerminalItem = TerminalDef;
    type PrecOpt = bool;           // true = has prec keyword
    type TypeOpt = Option<String>; // Some(type) or None
    type Rule = Rule;
    type Alts = Vec<Alt>;
    type Alt = Alt;
    type NameOpt = Option<String>; // Some(name) or None
    type Seq = Seq;

    fn grammar_def(&mut self, name: Ident, start: Ident, terminals: Vec<TerminalDef>, rules: Rules) -> GrammarDef {
        GrammarDef { name, start, terminals, rules }
    }

    fn rules_append(&mut self, mut rules: Rules, rule: Rule) -> Rules {
        rules.push(rule);
        rules
    }

    fn rules_single(&mut self, rule: Rule) -> Rules {
        vec![rule]
    }

    fn terminals_trailing(&mut self, list: TerminalList) -> Vec<TerminalDef> {
        list.0
    }

    fn terminals_block(&mut self, list: TerminalList) -> Vec<TerminalDef> {
        list.0
    }

    fn terminals_empty(&mut self) -> Vec<TerminalDef> {
        vec![]
    }

    fn terminal_list_append(&mut self, mut list: TerminalList, item: TerminalDef) -> TerminalList {
        list.0.push(item);
        list
    }

    fn terminal_list_single(&mut self, item: TerminalDef) -> TerminalList {
        TerminalList(vec![item])
    }

    fn terminal_item(&mut self, is_prec: bool, name: Ident, type_name: Option<String>) -> TerminalDef {
        TerminalDef { name, type_name, is_prec }
    }

    fn prec_yes(&mut self) -> bool {
        true
    }

    fn prec_no(&mut self) -> bool {
        false
    }

    fn type_some(&mut self, type_name: Ident) -> Option<String> {
        Some(type_name)
    }

    fn type_none(&mut self) -> Option<String> {
        None
    }

    fn rule(&mut self, name: Ident, result_type: Option<String>, alts: Vec<Alt>) -> Rule {
        Rule { name, result_type, alts }
    }

    fn alts_append(&mut self, mut alts: Vec<Alt>, alt: Alt) -> Vec<Alt> {
        alts.push(alt);
        alts
    }

    fn alts_single(&mut self, alt: Alt) -> Vec<Alt> {
        vec![alt]
    }

    fn alt(&mut self, seq: Seq, name: Option<String>) -> Alt {
        Alt { symbols: seq, name }
    }

    fn alt_empty(&mut self, name: Option<String>) -> Alt {
        Alt { symbols: vec![], name }
    }

    fn name_some(&mut self, name: Ident) -> Option<String> {
        Some(name)
    }

    fn name_none(&mut self) -> Option<String> {
        None
    }

    fn seq_append(&mut self, mut seq: Seq, symbol: Ident) -> Seq {
        seq.push(symbol);
        seq
    }

    fn seq_single(&mut self, symbol: Ident) -> Seq {
        vec![symbol]
    }
}

// ============================================================================
// Lexer
// ============================================================================

/// Lex grammar syntax using the general lexer.
fn lex_grammar(input: &str) -> Result<Vec<MetaTerminal<AstBuilder>>, String> {
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
            LexToken::Char(c) => return Err(format!("Unexpected character literal: '{}'", c)),
        }
    }

    Ok(tokens)
}

// ============================================================================
// Parsing API
// ============================================================================

/// Parse tokens into typed AST.
pub fn parse_tokens_typed<I>(tokens: I) -> Result<GrammarDef, String>
where
    I: IntoIterator<Item = MetaTerminal<AstBuilder>>,
{
    let mut parser = MetaParser::<AstBuilder>::new();
    let mut actions = AstBuilder;

    for tok in tokens {
        parser.push(tok, &mut actions)
            .map_err(|e| format!("Parse error, state {}", e.state))?;
    }

    parser.finish(&mut actions)
        .map_err(|e| format!("Parse error at end, state {}", e.state))
}

/// Parse a grammar string into a Grammar.
pub fn parse_grammar(input: &str) -> Result<Grammar, String> {
    let grammar_def = parse_grammar_typed(input)?;
    grammar_def_to_grammar(grammar_def)
}

/// Parse a grammar string into a typed GrammarDef.
pub fn parse_grammar_typed(input: &str) -> Result<GrammarDef, String> {
    let tokens = lex_grammar(input)?;
    if tokens.is_empty() {
        return Err("Empty grammar".to_string());
    }
    parse_tokens_typed(tokens)
}

/// Convert typed AST to Grammar.
pub fn grammar_def_to_grammar(grammar_def: GrammarDef) -> Result<Grammar, String> {
    use crate::grammar::{GrammarBuilder, Symbol};

    let mut gb = GrammarBuilder::new();

    // Register terminals
    for def in &grammar_def.terminals {
        if def.is_prec {
            gb.pt(&def.name);
        } else {
            gb.t(&def.name);
        }
    }

    // Collect rule data
    let rule_data: Vec<(&Rule, Vec<Vec<String>>)> = grammar_def.rules.iter()
        .map(|rule| {
            let alt_seqs: Vec<Vec<String>> = rule.alts.iter()
                .map(|alt| alt.symbols.clone())
                .collect();
            (rule, alt_seqs)
        })
        .collect();

    // Register non-terminals
    let mut nt_symbols: Vec<(String, Symbol)> = Vec::new();
    for (rule, _) in &rule_data {
        let lhs = gb.nt(&rule.name);
        nt_symbols.push((rule.name.clone(), lhs));
    }

    // Build grammar rules
    for (rule, alt_seqs) in &rule_data {
        let lhs = nt_symbols.iter().find(|(n, _)| n == &rule.name).map(|(_, s)| *s).unwrap();

        for seq in alt_seqs {
            let rhs: Vec<Symbol> = seq.iter().map(|sym_name| {
                if let Some((_, sym)) = nt_symbols.iter().find(|(n, _)| n == sym_name) {
                    return *sym;
                }
                gb.symbols.get(sym_name)
                    .ok_or_else(|| format!("Unknown symbol: {}", sym_name))
                    .unwrap()
            }).collect();

            gb.rule(lhs, rhs);
        }
    }

    if grammar_def.rules.is_empty() {
        return Err(format!("Grammar '{}' has no rules", grammar_def.name));
    }

    Ok(gb.build())
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
        assert!(matches!(&tokens[0], MetaTerminal::<AstBuilder>::KwGrammar));
        assert!(matches!(&tokens[1], MetaTerminal::<AstBuilder>::Ident(s) if s == "Test"));
        assert!(matches!(&tokens[2], MetaTerminal::<AstBuilder>::Lbrace));
        assert!(matches!(&tokens[3], MetaTerminal::<AstBuilder>::KwStart));
        assert!(matches!(&tokens[4], MetaTerminal::<AstBuilder>::Ident(s) if s == "s"));
        assert!(matches!(&tokens[5], MetaTerminal::<AstBuilder>::Semi));
        assert!(matches!(&tokens[6], MetaTerminal::<AstBuilder>::KwTerminals));
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

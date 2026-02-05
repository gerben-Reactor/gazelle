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
use crate::grammar::{Grammar, ExpectDecl, TerminalDef, Rule, Alt, SymbolRef, SymbolModifier};
use crate::lexer::{self, Token as LexToken};

// Type alias for IDENT terminal payload
pub type Ident = String;

// ============================================================================
// Generated parser
// ============================================================================

include!("meta_generated.rs");

// ============================================================================
// AST builder implementing MetaActions
// ============================================================================

pub struct AstBuilder;

impl MetaActions for AstBuilder {
    // Types (from type annotations, shared across terminals and non-terminals)
    type Ident = Ident;
    type GrammarDef = Grammar;
    type ExpectDecl = ExpectDecl;
    type TerminalsBlock = Vec<TerminalDef>;
    type TerminalItem = TerminalDef;
    type Rule = Rule;
    type Alts = Vec<Alt>;
    type Alt = Alt;
    type Symbol = SymbolRef;

    fn grammar_def(&mut self, name: Ident, start: Ident, mode: Option<Ident>, expects: Vec<ExpectDecl>, terminals: Vec<TerminalDef>, rules: Vec<Rule>) -> Grammar {
        let mut expect_rr = 0;
        let mut expect_sr = 0;
        for e in expects {
            match e.kind.as_str() {
                "rr" => expect_rr = e.count,
                "sr" => expect_sr = e.count,
                _ => {} // ignore unknown kinds
            }
        }
        let mode = mode.unwrap_or_else(|| "lalr".to_string());
        Grammar { name, start, mode, expect_rr, expect_sr, terminals, rules }
    }

    fn mode_decl(&mut self, mode: Ident) -> Ident {
        mode
    }

    fn expect_decl(&mut self, count: Ident, kind: Ident) -> ExpectDecl {
        ExpectDecl {
            count: count.parse().unwrap_or(0),
            kind,
        }
    }

    fn terminals_block(&mut self, items: Vec<TerminalDef>) -> Vec<TerminalDef> {
        items
    }

    fn terminal_item(&mut self, is_prec: Option<()>, name: Ident, type_name: Option<Ident>, _comma: Option<()>) -> TerminalDef {
        TerminalDef { name, type_name, is_prec: is_prec.is_some() }
    }

    fn type_annot(&mut self, type_name: Ident) -> Ident {
        type_name
    }

    fn rule(&mut self, name: Ident, result_type: Option<Ident>, alts: Vec<Alt>) -> Rule {
        Rule { name, result_type, alts }
    }

    fn alts(&mut self, mut pipes: Vec<Alt>, final_alt: Alt) -> Vec<Alt> {
        pipes.push(final_alt);
        pipes
    }

    fn alt_pipe(&mut self, alt: Alt) -> Alt {
        alt
    }

    fn alt(&mut self, symbols: Vec<SymbolRef>, name: Option<Ident>) -> Alt {
        Alt { symbols, name }
    }

    fn action_name(&mut self, name: Ident) -> Ident {
        name
    }

    fn sym_opt(&mut self, name: Ident) -> SymbolRef {
        SymbolRef { name, modifier: SymbolModifier::Optional }
    }

    fn sym_star(&mut self, name: Ident) -> SymbolRef {
        SymbolRef { name, modifier: SymbolModifier::ZeroOrMore }
    }

    fn sym_plus(&mut self, name: Ident) -> SymbolRef {
        SymbolRef { name, modifier: SymbolModifier::OneOrMore }
    }

    fn sym_plain(&mut self, name: Ident) -> SymbolRef {
        SymbolRef { name, modifier: SymbolModifier::None }
    }

    fn sym_empty(&mut self) -> SymbolRef {
        SymbolRef { name: "_".to_string(), modifier: SymbolModifier::Empty }
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
                    "grammar" => tokens.push(MetaTerminal::KW_GRAMMAR),
                    "start" => tokens.push(MetaTerminal::KW_START),
                    "terminals" => tokens.push(MetaTerminal::KW_TERMINALS),
                    "prec" => tokens.push(MetaTerminal::KW_PREC),
                    "expect" => tokens.push(MetaTerminal::KW_EXPECT),
                    "mode" => tokens.push(MetaTerminal::KW_MODE),
                    "_" => tokens.push(MetaTerminal::UNDERSCORE),
                    _ => tokens.push(MetaTerminal::IDENT(s)),
                }
            }
            LexToken::Str(s) => return Err(format!("Unexpected string literal: '{}'", s)),
            LexToken::Op(s) => {
                for c in s.chars() {
                    match c {
                        '=' => tokens.push(MetaTerminal::EQ),
                        '|' => tokens.push(MetaTerminal::PIPE),
                        ':' => tokens.push(MetaTerminal::COLON),
                        '@' => tokens.push(MetaTerminal::AT),
                        '?' => tokens.push(MetaTerminal::QUESTION),
                        '*' => tokens.push(MetaTerminal::STAR),
                        '+' => tokens.push(MetaTerminal::PLUS),
                        _ => return Err(format!("Unexpected operator: {}", c)),
                    }
                }
            }
            LexToken::Punct(c) => match c {
                ';' => tokens.push(MetaTerminal::SEMI),
                '{' => tokens.push(MetaTerminal::LBRACE),
                '}' => tokens.push(MetaTerminal::RBRACE),
                ',' => tokens.push(MetaTerminal::COMMA),
                _ => return Err(format!("Unexpected punctuation: {}", c)),
            },
            LexToken::Num(s) => tokens.push(MetaTerminal::NUM(s)),
            LexToken::Char(c) => return Err(format!("Unexpected character literal: '{}'", c)),
        }
    }

    Ok(tokens)
}

// ============================================================================
// Parsing API
// ============================================================================

/// Parse tokens into typed AST.
pub fn parse_tokens_typed<I>(tokens: I) -> Result<Grammar, String>
where
    I: IntoIterator<Item = MetaTerminal<AstBuilder>>,
{
    let mut parser = MetaParser::<AstBuilder>::new();
    let mut actions = AstBuilder;

    for tok in tokens {
        if let Err(e) = parser.push(tok, &mut actions) {
            return Err(parser.format_error(&e));
        }
    }

    parser.finish(&mut actions)
        .map_err(|e| e.to_string())
}

/// Parse a grammar string into a Grammar AST.
pub fn parse_grammar(input: &str) -> Result<Grammar, String> {
    let tokens = lex_grammar(input)?;
    if tokens.is_empty() {
        return Err("Empty grammar".to_string());
    }
    parse_tokens_typed(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lr::to_grammar_internal;

    #[test]
    fn test_lex() {
        let tokens = lex_grammar("grammar Test { start s; terminals { A } s: S = A; }").unwrap();
        assert!(matches!(&tokens[0], MetaTerminal::<AstBuilder>::KW_GRAMMAR));
        assert!(matches!(&tokens[1], MetaTerminal::<AstBuilder>::IDENT(s) if s == "Test"));
        assert!(matches!(&tokens[2], MetaTerminal::<AstBuilder>::LBRACE));
        assert!(matches!(&tokens[3], MetaTerminal::<AstBuilder>::KW_START));
        assert!(matches!(&tokens[4], MetaTerminal::<AstBuilder>::IDENT(s) if s == "s"));
    }

    #[test]
    fn test_parse_simple() {
        let grammar = parse_grammar(r#"
            grammar Test {
                start expr;
                terminals { PLUS, NUM }
                expr = expr PLUS term | term;
                term = NUM;
            }
        "#).unwrap();

        assert_eq!(grammar.name, "Test");
        assert_eq!(grammar.start, "expr");
        assert_eq!(grammar.terminals.len(), 2);
        assert_eq!(grammar.rules.len(), 2);
    }

    #[test]
    fn test_parse_expr_grammar() {
        let grammar = parse_grammar(r#"
            grammar Expr {
                start expr;
                terminals { PLUS, STAR, NUM, LPAREN, RPAREN }
                expr = expr PLUS term | term;
                term = term STAR factor | factor;
                factor = NUM | LPAREN expr RPAREN;
            }
        "#).unwrap();

        assert_eq!(grammar.rules.len(), 3);
        assert_eq!(grammar.rules[0].alts.len(), 2); // expr has 2 alternatives
        assert_eq!(grammar.rules[1].alts.len(), 2); // term has 2 alternatives
        assert_eq!(grammar.rules[2].alts.len(), 2); // factor has 2 alternatives
    }

    #[test]
    fn test_parse_error_message() {
        let result = parse_grammar(r#"
            grammar Test {
                start foo;
                terminals { A }
                foo = A A A;
            }
        "#);

        assert!(result.is_ok());
    }

    #[test]
    fn test_prec_terminal() {
        let grammar = parse_grammar(r#"
            grammar Prec {
                start expr;
                terminals { prec OP, NUM }
                expr = expr OP expr | NUM;
            }
        "#).unwrap();

        assert_eq!(grammar.terminals.len(), 2);
        assert!(grammar.terminals[0].is_prec);
        assert!(!grammar.terminals[1].is_prec);
    }

    #[test]
    fn test_roundtrip() {
        let grammar = parse_grammar(r#"
            grammar Simple {
                start s;
                terminals { a }
                s = a;
            }
        "#).unwrap();

        let internal = to_grammar_internal(grammar).unwrap();
        // 2 rules: __start -> s, s -> a
        assert_eq!(internal.rules.len(), 2);
    }

    #[test]
    fn test_terminals_with_types() {
        let grammar = parse_grammar(r#"
            grammar TypedTerminals {
                start expr;
                terminals { NUM: i32, IDENT: String, PLUS }
                expr = NUM | IDENT | expr PLUS expr;
            }
        "#).unwrap();

        assert_eq!(grammar.terminals.len(), 3);
        assert_eq!(grammar.terminals[0].name, "NUM");
        assert_eq!(grammar.terminals[0].type_name, Some("i32".to_string()));
        assert_eq!(grammar.terminals[1].name, "IDENT");
        assert_eq!(grammar.terminals[1].type_name, Some("String".to_string()));
        assert_eq!(grammar.terminals[2].name, "PLUS");
        assert_eq!(grammar.terminals[2].type_name, None);
    }

    #[test]
    fn test_rule_without_type() {
        let grammar = parse_grammar(r#"
            grammar Untyped {
                start expr;
                terminals { NUM }
                expr = NUM;
            }
        "#).unwrap();

        assert_eq!(grammar.rules[0].result_type, None);
    }

    #[test]
    fn test_named_reductions() {
        let grammar = parse_grammar(r#"
            grammar Named {
                start expr;
                terminals { PLUS, NUM }
                expr = expr PLUS expr @binop | NUM @literal;
            }
        "#).unwrap();

        assert_eq!(grammar.rules[0].alts[0].name, Some("binop".to_string()));
        assert_eq!(grammar.rules[0].alts[1].name, Some("literal".to_string()));
    }

    #[test]
    fn test_modifier_parsing() {
        let grammar = parse_grammar(r#"
            grammar Modifiers {
                start s;
                terminals { A, B, C }
                s = A? B* C+;
            }
        "#).unwrap();

        assert_eq!(grammar.rules[0].alts[0].symbols.len(), 3);
        assert_eq!(grammar.rules[0].alts[0].symbols[0].modifier, SymbolModifier::Optional);
        assert_eq!(grammar.rules[0].alts[0].symbols[1].modifier, SymbolModifier::ZeroOrMore);
        assert_eq!(grammar.rules[0].alts[0].symbols[2].modifier, SymbolModifier::OneOrMore);
    }

    #[test]
    fn test_named_empty_production() {
        let grammar = parse_grammar(r#"
            grammar Empty {
                start s;
                terminals { A }
                s = A | _ @empty;
            }
        "#).unwrap();

        assert_eq!(grammar.rules[0].alts.len(), 2);
        assert_eq!(grammar.rules[0].alts[1].symbols.len(), 1);
        assert_eq!(grammar.rules[0].alts[1].symbols[0].modifier, SymbolModifier::Empty);
        assert_eq!(grammar.rules[0].alts[1].name, Some("empty".to_string()));
    }

    #[test]
    fn test_modifier_desugaring() {
        use crate::lr::desugar_modifiers;

        let mut grammar = parse_grammar(r#"
            grammar OptionalTest {
                start s;
                terminals { A: String }
                s: Result = A?;
            }
        "#).unwrap();

        desugar_modifiers(&mut grammar);

        // Should have 2 rules now: s and __a_opt
        assert_eq!(grammar.rules.len(), 2);

        // Find the synthetic rule
        let opt_rule = grammar.rules.iter().find(|r| r.name == "__a_opt").unwrap();
        assert_eq!(opt_rule.result_type, Some("Option<String>".to_string()));
        assert_eq!(opt_rule.alts.len(), 2);
        assert_eq!(opt_rule.alts[0].name, Some("__some".to_string()));
        assert_eq!(opt_rule.alts[1].name, Some("__none".to_string()));

        // The original rule should reference the synthetic rule
        let s_rule = grammar.rules.iter().find(|r| r.name == "s").unwrap();
        assert_eq!(s_rule.alts[0].symbols[0].name, "__a_opt");
        assert_eq!(s_rule.alts[0].symbols[0].modifier, SymbolModifier::None);
    }

    #[test]
    fn test_expect_declarations() {
        let grammar = parse_grammar(r#"
            grammar WithExpect {
                start s;
                expect 2 sr;
                expect 1 rr;
                terminals { A }
                s = A;
            }
        "#).unwrap();

        assert_eq!(grammar.expect_sr, 2);
        assert_eq!(grammar.expect_rr, 1);
    }

    #[test]
    fn test_trailing_comma() {
        let grammar = parse_grammar(r#"
            grammar TrailingComma {
                start s;
                terminals { A, B, C, }
                s = A;
            }
        "#).unwrap();

        assert_eq!(grammar.terminals.len(), 3);
    }

    #[test]
    fn test_unknown_symbol_error() {
        let grammar = parse_grammar(r#"
            grammar UnknownSymbol {
                start s;
                terminals { A }
                s = A B;
            }
        "#).unwrap();

        let result = to_grammar_internal(grammar);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown symbol: B"));
    }

    #[test]
    fn test_untyped_modifier_star() {
        use crate::lr::desugar_modifiers;

        let mut grammar = parse_grammar(r#"
            grammar UntypedStar {
                start s;
                terminals { A }
                s = A*;
            }
        "#).unwrap();

        desugar_modifiers(&mut grammar);

        let star_rule = grammar.rules.iter().find(|r| r.name == "__a_star").unwrap();
        assert_eq!(star_rule.result_type, Some("Vec<()>".to_string()));
    }

    #[test]
    fn test_untyped_nonterminal_modifier_optional() {
        use crate::lr::desugar_modifiers;

        let mut grammar = parse_grammar(r#"
            grammar UntypedNtOpt {
                start s;
                terminals { A }
                s = foo?;
                foo = A;
            }
        "#).unwrap();

        desugar_modifiers(&mut grammar);

        let opt_rule = grammar.rules.iter().find(|r| r.name == "__foo_opt").unwrap();
        assert_eq!(opt_rule.result_type, Some("Option<()>".to_string()));
    }

    #[test]
    fn test_untyped_nonterminal_modifier_star() {
        use crate::lr::desugar_modifiers;

        let mut grammar = parse_grammar(r#"
            grammar UntypedNtStar {
                start s;
                terminals { A }
                s = foo*;
                foo = A;
            }
        "#).unwrap();

        desugar_modifiers(&mut grammar);

        let star_rule = grammar.rules.iter().find(|r| r.name == "__foo_star").unwrap();
        assert_eq!(star_rule.result_type, Some("Vec<()>".to_string()));
    }
}

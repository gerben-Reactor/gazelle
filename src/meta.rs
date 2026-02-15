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
use crate::grammar::{Grammar, ExpectDecl, TerminalDef, Rule, Alt, Term};
use crate::lexer::Source;


// ============================================================================
// Generated parser
// ============================================================================

include!("meta_generated.rs");

// ============================================================================
// AST builder implementing MetaActions
// ============================================================================

#[doc(hidden)]
pub struct AstBuilder;

impl MetaTypes for AstBuilder {
    type Error = crate::ParseError;
    type Ident = String;
    type Grammar_def = Grammar;
    type Mode_decl = String;
    type Expect_decl = ExpectDecl;
    type Terminal_item = TerminalDef;
    type Type_annot = String;
    type Rule = Rule;
    type Alt = Alt;
    type Action_name = String;
    type Term = Term;
}

impl gazelle::Reduce<MetaMode_decl<Self>, String, crate::ParseError> for AstBuilder {
    fn reduce(&mut self, node: MetaMode_decl<Self>) -> Result<String, crate::ParseError> {
        let MetaMode_decl::Mode_decl(name) = node;
        Ok(name)
    }
}

impl gazelle::Reduce<MetaType_annot<Self>, String, crate::ParseError> for AstBuilder {
    fn reduce(&mut self, node: MetaType_annot<Self>) -> Result<String, crate::ParseError> {
        let MetaType_annot::Type_annot(name) = node;
        Ok(name)
    }
}

impl gazelle::Reduce<MetaAction_name<Self>, String, crate::ParseError> for AstBuilder {
    fn reduce(&mut self, node: MetaAction_name<Self>) -> Result<String, crate::ParseError> {
        let MetaAction_name::Action_name(name) = node;
        Ok(name)
    }
}

impl gazelle::Reduce<MetaGrammar_def<Self>, Grammar, crate::ParseError> for AstBuilder {
    fn reduce(&mut self, node: MetaGrammar_def<Self>) -> Result<Grammar, crate::ParseError> {
        let MetaGrammar_def::Grammar_def(start, mode, expects, terminals, rules) = node;
        let mut expect_rr = 0;
        let mut expect_sr = 0;
        for e in expects {
            match e.kind.as_str() {
                "rr" => expect_rr = e.count,
                "sr" => expect_sr = e.count,
                _ => {}
            }
        }
        let mode = mode.unwrap_or_else(|| "lalr".to_string());
        Ok(Grammar { start, mode, expect_rr, expect_sr, terminals, rules })
    }
}

impl gazelle::Reduce<MetaExpect_decl<Self>, ExpectDecl, crate::ParseError> for AstBuilder {
    fn reduce(&mut self, node: MetaExpect_decl<Self>) -> Result<ExpectDecl, crate::ParseError> {
        let MetaExpect_decl::Expect_decl(count, kind) = node;
        Ok(ExpectDecl { count: count.parse().unwrap_or(0), kind })
    }
}

impl gazelle::Reduce<MetaTerminal_item<Self>, TerminalDef, crate::ParseError> for AstBuilder {
    fn reduce(&mut self, node: MetaTerminal_item<Self>) -> Result<TerminalDef, crate::ParseError> {
        let MetaTerminal_item::Terminal_item(is_prec, name, type_name) = node;
        Ok(TerminalDef { name, type_name, is_prec: is_prec.is_some() })
    }
}

impl gazelle::Reduce<MetaRule<Self>, Rule, crate::ParseError> for AstBuilder {
    fn reduce(&mut self, node: MetaRule<Self>) -> Result<Rule, crate::ParseError> {
        let MetaRule::Rule(name, alts) = node;
        Ok(Rule { name, alts })
    }
}

impl gazelle::Reduce<MetaAlt<Self>, Alt, crate::ParseError> for AstBuilder {
    fn reduce(&mut self, node: MetaAlt<Self>) -> Result<Alt, crate::ParseError> {
        let MetaAlt::Alt(terms, name) = node;
        Ok(Alt { terms, name })
    }
}

impl gazelle::Reduce<MetaTerm<Self>, Term, crate::ParseError> for AstBuilder {
    fn reduce(&mut self, node: MetaTerm<Self>) -> Result<Term, crate::ParseError> {
        Ok(match node {
            MetaTerm::Sym_sep(name, sep) => Term::SeparatedBy { symbol: name, sep },
            MetaTerm::Sym_opt(name) => Term::Optional(name),
            MetaTerm::Sym_star(name) => Term::ZeroOrMore(name),
            MetaTerm::Sym_plus(name) => Term::OneOrMore(name),
            MetaTerm::Sym_plain(name) => Term::Symbol(name),
            MetaTerm::Sym_empty => Term::Empty,
        })
    }
}

// ============================================================================
// Lexer
// ============================================================================

/// Lex grammar syntax using the composable Source API.
fn lex_grammar(input: &str) -> Result<Vec<MetaTerminal<AstBuilder>>, String> {
    let mut src = Source::from_str(input);
    let mut tokens = Vec::new();

    loop {
        // Skip whitespace and comments
        src.skip_whitespace();
        while src.skip_line_comment("//") || src.skip_block_comment("/*", "*/") {
            src.skip_whitespace();
        }

        if src.at_end() {
            break;
        }

        // Identifier or keyword
        if let Some(span) = src.read_ident() {
            let s = &input[span];
            let tok = match s {
                "start" => MetaTerminal::KW_START,
                "terminals" => MetaTerminal::KW_TERMINALS,
                "prec" => MetaTerminal::KW_PREC,
                "expect" => MetaTerminal::KW_EXPECT,
                "mode" => MetaTerminal::KW_MODE,
                "_" => MetaTerminal::UNDERSCORE,
                _ => MetaTerminal::IDENT(s.to_string()),
            };
            tokens.push(tok);
            continue;
        }

        // Number
        if let Some(span) = src.read_digits() {
            let s = &input[span];
            tokens.push(MetaTerminal::NUM(s.to_string()));
            continue;
        }

        // Single-char operators and punctuation
        if let Some(c) = src.peek() {
            let tok = match c {
                '=' => { src.advance(); MetaTerminal::EQ }
                '|' => { src.advance(); MetaTerminal::PIPE }
                ':' => { src.advance(); MetaTerminal::COLON }
                '@' => { src.advance(); MetaTerminal::AT }
                '?' => { src.advance(); MetaTerminal::QUESTION }
                '*' => { src.advance(); MetaTerminal::STAR }
                '+' => { src.advance(); MetaTerminal::PLUS }
                '%' => { src.advance(); MetaTerminal::PERCENT }
                ';' => { src.advance(); MetaTerminal::SEMI }
                '{' => { src.advance(); MetaTerminal::LBRACE }
                '}' => { src.advance(); MetaTerminal::RBRACE }
                ',' => { src.advance(); MetaTerminal::COMMA }
                '(' => { src.advance(); MetaTerminal::LPAREN }
                ')' => { src.advance(); MetaTerminal::RPAREN }
                _ => {
                    let (line, col) = src.line_col(src.offset());
                    return Err(format!("{}:{}: unexpected character: {:?}", line, col, c));
                }
            };
            tokens.push(tok);
            continue;
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
        .map_err(|(p, e)| p.format_error(&e))
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
        let tokens = lex_grammar("start s; terminals { A } s: S = A;").unwrap();
        assert!(matches!(&tokens[0], MetaTerminal::<AstBuilder>::KW_START));
        assert!(matches!(&tokens[1], MetaTerminal::<AstBuilder>::IDENT(s) if s == "s"));
    }

    #[test]
    fn test_parse_simple() {
        let grammar = parse_grammar(r#"
            start expr;
            terminals { PLUS, NUM }
            expr = expr PLUS term @add | term @term;
            term = NUM @num;
        "#).unwrap();

        assert_eq!(grammar.start, "expr");
        assert_eq!(grammar.terminals.len(), 2);
        assert_eq!(grammar.rules.len(), 2);
    }

    #[test]
    fn test_parse_expr_grammar() {
        let grammar = parse_grammar(r#"
            start expr;
            terminals { PLUS, STAR, NUM, LPAREN, RPAREN }
            expr = expr PLUS term @add | term @term;
            term = term STAR factor @mul | factor @factor;
            factor = NUM @num | LPAREN expr RPAREN @paren;
        "#).unwrap();

        assert_eq!(grammar.rules.len(), 3);
        assert_eq!(grammar.rules[0].alts.len(), 2);
        assert_eq!(grammar.rules[1].alts.len(), 2);
        assert_eq!(grammar.rules[2].alts.len(), 2);
    }

    #[test]
    fn test_parse_error_message() {
        let result = parse_grammar(r#"
            start foo;
            terminals { A }
            foo = A A A @triple;
        "#);

        assert!(result.is_ok());
    }

    #[test]
    fn test_prec_terminal() {
        let grammar = parse_grammar(r#"
            start expr;
            terminals { prec OP, NUM }
            expr = expr OP expr @binop | NUM @num;
        "#).unwrap();

        assert_eq!(grammar.terminals.len(), 2);
        assert!(grammar.terminals[0].is_prec);
        assert!(!grammar.terminals[1].is_prec);
    }

    #[test]
    fn test_roundtrip() {
        let grammar = parse_grammar(r#"
            start s;
            terminals { a }
            s = a @a;
        "#).unwrap();

        let internal = to_grammar_internal(&grammar).unwrap();
        // 2 rules: __start -> s, s -> a
        assert_eq!(internal.rules.len(), 2);
    }

    #[test]
    fn test_terminals_with_types() {
        let grammar = parse_grammar(r#"
            start expr;
            terminals { NUM: i32, IDENT: String, PLUS }
            expr = NUM @num | IDENT @ident | expr PLUS expr @add;
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
    fn test_rule_without_action() {
        let grammar = parse_grammar(r#"
            start expr;
            terminals { NUM }
            expr = NUM @num;
        "#).unwrap();

        assert_eq!(grammar.rules[0].alts[0].name, "num");
    }

    #[test]
    fn test_named_reductions() {
        let grammar = parse_grammar(r#"
            start expr;
            terminals { PLUS, NUM }
            expr = expr PLUS expr @binop | NUM @literal;
        "#).unwrap();

        assert_eq!(grammar.rules[0].alts[0].name, "binop");
        assert_eq!(grammar.rules[0].alts[1].name, "literal");
    }

    #[test]
    fn test_modifier_parsing() {
        let grammar = parse_grammar(r#"
            start s;
            terminals { A, B, C }
            s = A? B* C+ @s;
        "#).unwrap();

        assert_eq!(grammar.rules[0].alts[0].terms.len(), 3);
        assert_eq!(grammar.rules[0].alts[0].terms[0], Term::Optional("A".to_string()));
        assert_eq!(grammar.rules[0].alts[0].terms[1], Term::ZeroOrMore("B".to_string()));
        assert_eq!(grammar.rules[0].alts[0].terms[2], Term::OneOrMore("C".to_string()));
    }

    #[test]
    fn test_named_empty_production() {
        let grammar = parse_grammar(r#"
            start s;
            terminals { A }
            s = A @a | _ @empty;
        "#).unwrap();

        assert_eq!(grammar.rules[0].alts.len(), 2);
        assert_eq!(grammar.rules[0].alts[1].terms.len(), 1);
        assert_eq!(grammar.rules[0].alts[1].terms[0], Term::Empty);
        assert_eq!(grammar.rules[0].alts[1].name, "empty");
    }

    #[test]
    fn test_modifier_desugaring() {
        use crate::lr::AltAction;

        let grammar = parse_grammar(r#"
            start s;
            terminals { A: String }
            s = A? @s;
        "#).unwrap();

        let internal = to_grammar_internal(&grammar).unwrap();

        // Check synthetic non-terminal has correct type
        let opt_id = internal.symbols.get_id("__a_opt").unwrap();
        assert_eq!(internal.types[&opt_id], Some("Option<String>".to_string()));

        // Find synthetic rules for __a_opt
        let opt_sym = internal.symbols.get("__a_opt").unwrap();
        let opt_rules: Vec<_> = internal.rules.iter()
            .filter(|r| r.lhs == opt_sym)
            .collect();
        assert_eq!(opt_rules.len(), 2);
        assert_eq!(opt_rules[0].action, AltAction::OptSome);
        assert_eq!(opt_rules[1].action, AltAction::OptNone);

        // The user rule should reference the synthetic non-terminal
        let s_sym = internal.symbols.get("s").unwrap();
        let s_rules: Vec<_> = internal.rules.iter()
            .filter(|r| r.lhs == s_sym)
            .collect();
        assert_eq!(s_rules.len(), 1);
        assert_eq!(s_rules[0].rhs, vec![opt_sym]);
    }

    #[test]
    fn test_expect_declarations() {
        let grammar = parse_grammar(r#"
            start s;
            expect 2 sr;
            expect 1 rr;
            terminals { A }
            s = A @a;
        "#).unwrap();

        assert_eq!(grammar.expect_sr, 2);
        assert_eq!(grammar.expect_rr, 1);
    }

    #[test]
    fn test_no_trailing_comma() {
        let grammar = parse_grammar(r#"
            start s;
            terminals { A, B, C }
            s = A @a;
        "#).unwrap();

        assert_eq!(grammar.terminals.len(), 3);
    }

    #[test]
    fn test_unknown_symbol_error() {
        let grammar = parse_grammar(r#"
            start s;
            terminals { A }
            s = A B @s;
        "#).unwrap();

        let result = to_grammar_internal(&grammar);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown symbol: B"));
    }

    #[test]
    fn test_untyped_modifier_star() {
        let grammar = parse_grammar(r#"
            start s;
            terminals { A }
            s = A* @s;
        "#).unwrap();

        let internal = to_grammar_internal(&grammar).unwrap();
        let star_id = internal.symbols.get_id("__a_star").unwrap();
        assert_eq!(internal.types[&star_id], Some("Vec<()>".to_string()));
    }

    #[test]
    fn test_untyped_nonterminal_modifier_optional() {
        let grammar = parse_grammar(r#"
            start s;
            terminals { A }
            s = foo? @s;
            foo = A @a;
        "#).unwrap();

        let internal = to_grammar_internal(&grammar).unwrap();
        let opt_id = internal.symbols.get_id("__foo_opt").unwrap();
        assert_eq!(internal.types[&opt_id], Some("Option<Foo>".to_string()));
    }

    #[test]
    fn test_untyped_nonterminal_modifier_star() {
        let grammar = parse_grammar(r#"
            start s;
            terminals { A }
            s = foo* @s;
            foo = A @a;
        "#).unwrap();

        let internal = to_grammar_internal(&grammar).unwrap();
        let star_id = internal.symbols.get_id("__foo_star").unwrap();
        assert_eq!(internal.types[&star_id], Some("Vec<Foo>".to_string()));
    }

    #[test]
    fn test_separator_modifier_parsing() {
        let grammar = parse_grammar(r#"
            start s;
            terminals { A, COMMA }
            s = (A % COMMA) @s;
        "#).unwrap();

        assert_eq!(grammar.rules[0].alts[0].terms.len(), 1);
        assert_eq!(grammar.rules[0].alts[0].terms[0], Term::SeparatedBy { symbol: "A".to_string(), sep: "COMMA".to_string() });
    }

    #[test]
    fn test_separator_modifier_desugaring() {
        use crate::lr::AltAction;

        let grammar = parse_grammar(r#"
            start s;
            terminals { A: String, COMMA }
            s = (A % COMMA) @s;
        "#).unwrap();

        let internal = to_grammar_internal(&grammar).unwrap();

        // Check synthetic type
        let sep_id = internal.symbols.get_id("__a_sep_comma").unwrap();
        assert_eq!(internal.types[&sep_id], Some("Vec<String>".to_string()));

        // Find synthetic rules
        let sep_sym = internal.symbols.get("__a_sep_comma").unwrap();
        let sep_rules: Vec<_> = internal.rules.iter()
            .filter(|r| r.lhs == sep_sym)
            .collect();
        assert_eq!(sep_rules.len(), 2);

        // First: __a_sep_comma -> __a_sep_comma COMMA A (VecAppend)
        let a_sym = internal.symbols.get("A").unwrap();
        let comma_sym = internal.symbols.get("COMMA").unwrap();
        assert_eq!(sep_rules[0].rhs, vec![sep_sym, comma_sym, a_sym]);
        assert_eq!(sep_rules[0].action, AltAction::VecAppend);

        // Second: __a_sep_comma -> A (VecSingle)
        assert_eq!(sep_rules[1].rhs, vec![a_sym]);
        assert_eq!(sep_rules[1].action, AltAction::VecSingle);

        // The user rule should reference the synthetic non-terminal
        let s_sym = internal.symbols.get("s").unwrap();
        let s_rules: Vec<_> = internal.rules.iter()
            .filter(|r| r.lhs == s_sym)
            .collect();
        assert_eq!(s_rules.len(), 1);
        assert_eq!(s_rules[0].rhs, vec![sep_sym]);
    }

    #[test]
    fn test_separator_end_to_end() {
        let grammar = parse_grammar(r#"
            start items;
            terminals { ITEM, COMMA }
            items = (ITEM % COMMA) @items;
        "#).unwrap();

        let internal = to_grammar_internal(&grammar).unwrap();
        use crate::table::CompiledTable;
        let compiled = CompiledTable::build_with_algorithm(&internal, crate::lr::LrAlgorithm::default());
        assert!(!compiled.has_conflicts());

        // Parse: ITEM
        let item_id = compiled.symbol_id("ITEM").unwrap();
        let comma_id = compiled.symbol_id("COMMA").unwrap();
        {
            use crate::runtime::{Parser, Token};
            let mut parser = Parser::new(compiled.table());
            let token = Token::new(item_id);
            assert!(parser.maybe_reduce(Some(token)).unwrap().is_none());
            parser.shift(token);
            // Reduce to accept
            while let Some((rule, _, _)) = parser.maybe_reduce(None).unwrap() {
                if rule == 0 { break; }
            }
        }

        // Parse: ITEM COMMA ITEM
        {
            use crate::runtime::{Parser, Token};
            let mut parser = Parser::new(compiled.table());
            let tokens = vec![Token::new(item_id), Token::new(comma_id), Token::new(item_id)];
            for tok in tokens {
                while let Some((rule, _, _)) = parser.maybe_reduce(Some(tok)).unwrap() {
                    if rule == 0 { break; }
                }
                parser.shift(tok);
            }
            // Finish
            while let Some((rule, _, _)) = parser.maybe_reduce(None).unwrap() {
                if rule == 0 { break; }
            }
        }

        // Parse: ITEM COMMA ITEM COMMA ITEM
        {
            use crate::runtime::{Parser, Token};
            let mut parser = Parser::new(compiled.table());
            let tokens = vec![
                Token::new(item_id), Token::new(comma_id),
                Token::new(item_id), Token::new(comma_id),
                Token::new(item_id),
            ];
            for tok in tokens {
                while let Some((rule, _, _)) = parser.maybe_reduce(Some(tok)).unwrap() {
                    if rule == 0 { break; }
                }
                parser.shift(tok);
            }
            while let Some((rule, _, _)) = parser.maybe_reduce(None).unwrap() {
                if rule == 0 { break; }
            }
        }
    }
}

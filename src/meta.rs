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
    pub symbols: Vec<Symbol>,
    pub name: Option<String>,
}

/// A symbol in a rule with optional modifier.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub modifier: SymbolModifier,
}

/// Modifier for a symbol in a grammar rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolModifier {
    /// No modifier - plain symbol
    None,
    /// `?` - optional (zero or one)
    Optional,
    /// `*` - zero or more
    ZeroOrMore,
    /// `+` - one or more
    OneOrMore,
}

// ============================================================================
// Intermediate parsing types
// ============================================================================

#[doc(hidden)]

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
    type TerminalsBlock = Vec<TerminalDef>;
    type TerminalItem = TerminalDef;
    type TypeAnnot = Ident;
    type Rule = Rule;
    type Alts = Vec<Alt>;
    type PipeAlt = Alt;
    type Alt = Alt;
    type EmptyAlt = Alt;
    type ActionName = Ident;
    type Symbol = Symbol;

    fn grammar_def(&mut self, name: Ident, start: Ident, terminals: Vec<TerminalDef>, rules: Vec<Rule>) -> GrammarDef {
        GrammarDef { name, start, terminals, rules }
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

    fn alts(&mut self, first: Alt, rest: Vec<Alt>) -> Vec<Alt> {
        let mut result = vec![first];
        result.extend(rest);
        result
    }

    fn alts_empty_first(&mut self, first: Alt, rest: Vec<Alt>) -> Vec<Alt> {
        let mut result = vec![first];
        result.extend(rest);
        result
    }

    fn pipe_alt(&mut self, alt: Alt) -> Alt {
        alt
    }

    fn alt(&mut self, symbols: Vec<Symbol>, name: Option<Ident>) -> Alt {
        Alt { symbols, name }
    }

    fn empty_alt(&mut self, name: Option<Ident>) -> Alt {
        Alt { symbols: vec![], name }
    }

    fn action_name(&mut self, name: Ident) -> Ident {
        name
    }

    fn sym_opt(&mut self, name: Ident) -> Symbol {
        Symbol { name, modifier: SymbolModifier::Optional }
    }

    fn sym_star(&mut self, name: Ident) -> Symbol {
        Symbol { name, modifier: SymbolModifier::ZeroOrMore }
    }

    fn sym_plus(&mut self, name: Ident) -> Symbol {
        Symbol { name, modifier: SymbolModifier::OneOrMore }
    }

    fn sym_plain(&mut self, name: Ident) -> Symbol {
        Symbol { name, modifier: SymbolModifier::None }
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
                        '?' => tokens.push(MetaTerminal::Question),
                        '*' => tokens.push(MetaTerminal::Star),
                        '+' => tokens.push(MetaTerminal::Plus),
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
pub fn grammar_def_to_grammar(mut grammar_def: GrammarDef) -> Result<Grammar, String> {
    use crate::grammar::{GrammarBuilder, Symbol as GrammarSymbol};

    // Desugar modifiers first
    desugar_modifiers(&mut grammar_def);

    let mut gb = GrammarBuilder::new();

    // Register terminals
    for def in &grammar_def.terminals {
        if def.is_prec {
            gb.pt(&def.name);
        } else {
            gb.t(&def.name);
        }
    }

    // Collect rule data - extract symbol names from Symbol structs
    let rule_data: Vec<(&Rule, Vec<Vec<String>>)> = grammar_def.rules.iter()
        .map(|rule| {
            let alt_seqs: Vec<Vec<String>> = rule.alts.iter()
                .map(|alt| alt.symbols.iter().map(|s| s.name.clone()).collect())
                .collect();
            (rule, alt_seqs)
        })
        .collect();

    // Register non-terminals
    let mut nt_symbols: Vec<(String, GrammarSymbol)> = Vec::new();
    for (rule, _) in &rule_data {
        let lhs = gb.nt(&rule.name);
        nt_symbols.push((rule.name.clone(), lhs));
    }

    // Build grammar rules
    for (rule, alt_seqs) in &rule_data {
        let lhs = nt_symbols.iter().find(|(n, _)| n == &rule.name).map(|(_, s)| *s).unwrap();

        for seq in alt_seqs {
            let rhs: Vec<GrammarSymbol> = seq.iter().map(|sym_name| {
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

/// Desugar modifier symbols (?, *, +) into synthetic helper rules.
pub fn desugar_modifiers(grammar_def: &mut GrammarDef) {
    use std::collections::HashMap;

    // Collect all symbols with modifiers and their types
    let mut synthetic_rules: Vec<Rule> = Vec::new();
    let mut synthetic_names: HashMap<(String, SymbolModifier), String> = HashMap::new();

    // Build a map from rule name to result type
    let rule_types: HashMap<String, Option<String>> = grammar_def.rules.iter()
        .map(|r| (r.name.clone(), r.result_type.clone()))
        .collect();

    // Build a map from terminal name to type
    let terminal_types: HashMap<String, Option<String>> = grammar_def.terminals.iter()
        .map(|t| (t.name.clone(), t.type_name.clone()))
        .collect();

    // First pass: identify all modified symbols and create synthetic rule names
    for rule in &grammar_def.rules {
        for alt in &rule.alts {
            for sym in &alt.symbols {
                if sym.modifier != SymbolModifier::None {
                    let key = (sym.name.clone(), sym.modifier);
                    if !synthetic_names.contains_key(&key) {
                        let suffix = match sym.modifier {
                            SymbolModifier::Optional => "opt",
                            SymbolModifier::ZeroOrMore => "star",
                            SymbolModifier::OneOrMore => "plus",
                            SymbolModifier::None => unreachable!(),
                        };
                        let synthetic_name = format!("__{sym_name}_{suffix}", sym_name = sym.name.to_lowercase());
                        synthetic_names.insert(key, synthetic_name);
                    }
                }
            }
        }
    }

    // Second pass: create synthetic rules
    for ((sym_name, modifier), synthetic_name) in &synthetic_names {
        // Convert to PascalCase (e.g., "type_annot" -> "TypeAnnot")
        fn to_pascal_case(s: &str) -> String {
            let mut result = String::new();
            let mut capitalize_next = true;
            for c in s.chars() {
                if c == '_' {
                    capitalize_next = true;
                } else if capitalize_next {
                    result.push(c.to_ascii_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(c.to_ascii_lowercase());
                }
            }
            result
        }

        // Look up the inner type
        // For terminals: use the terminal NAME (will become associated type)
        // For non-terminals: use the non-terminal NAME (will become associated type)
        let inner_type = if let Some(type_name) = terminal_types.get(sym_name) {
            if type_name.is_some() {
                // Typed terminal - use the terminal name as the type (for associated type)
                Some(to_pascal_case(sym_name))
            } else {
                // Untyped terminal - use unit type
                Some("()".to_string())
            }
        } else if rule_types.contains_key(sym_name) {
            // Non-terminal - use the non-terminal name (PascalCase) as the type
            Some(to_pascal_case(sym_name))
        } else {
            None
        };

        let (result_type, alts) = match modifier {
            SymbolModifier::Optional => {
                // __X_opt: Option<T> = X @__some | @__none;
                let wrapper_type = inner_type.as_ref()
                    .map(|t| format!("Option<{}>", t));
                let alts = vec![
                    Alt {
                        symbols: vec![Symbol { name: sym_name.clone(), modifier: SymbolModifier::None }],
                        name: Some("__some".to_string()),
                    },
                    Alt {
                        symbols: vec![],
                        name: Some("__none".to_string()),
                    },
                ];
                (wrapper_type, alts)
            }
            SymbolModifier::ZeroOrMore => {
                // __X_star: Vec<T> = __X_star X @__append | @__empty;
                let wrapper_type = inner_type.as_ref()
                    .map(|t| format!("Vec<{}>", t));
                let alts = vec![
                    Alt {
                        symbols: vec![
                            Symbol { name: synthetic_name.clone(), modifier: SymbolModifier::None },
                            Symbol { name: sym_name.clone(), modifier: SymbolModifier::None },
                        ],
                        name: Some("__append".to_string()),
                    },
                    Alt {
                        symbols: vec![],
                        name: Some("__empty".to_string()),
                    },
                ];
                (wrapper_type, alts)
            }
            SymbolModifier::OneOrMore => {
                // __X_plus: Vec<T> = __X_plus X @__append | X @__single;
                let wrapper_type = inner_type.as_ref()
                    .map(|t| format!("Vec<{}>", t));
                let alts = vec![
                    Alt {
                        symbols: vec![
                            Symbol { name: synthetic_name.clone(), modifier: SymbolModifier::None },
                            Symbol { name: sym_name.clone(), modifier: SymbolModifier::None },
                        ],
                        name: Some("__append".to_string()),
                    },
                    Alt {
                        symbols: vec![Symbol { name: sym_name.clone(), modifier: SymbolModifier::None }],
                        name: Some("__single".to_string()),
                    },
                ];
                (wrapper_type, alts)
            }
            SymbolModifier::None => unreachable!(),
        };

        synthetic_rules.push(Rule {
            name: synthetic_name.clone(),
            result_type,
            alts,
        });
    }

    // Third pass: replace modified symbols with synthetic non-terminal names
    for rule in &mut grammar_def.rules {
        for alt in &mut rule.alts {
            for sym in &mut alt.symbols {
                if sym.modifier != SymbolModifier::None {
                    let key = (sym.name.clone(), sym.modifier);
                    if let Some(synthetic_name) = synthetic_names.get(&key) {
                        sym.name = synthetic_name.clone();
                        sym.modifier = SymbolModifier::None;
                    }
                }
            }
        }
    }

    // Add synthetic rules to grammar
    grammar_def.rules.extend(synthetic_rules);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::Symbol as GrammarSymbol;
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
        assert!(matches!(op, GrammarSymbol::PrecTerminal(_)));
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

        // Helper to extract symbol names
        fn names(alt: &Alt) -> Vec<String> {
            alt.symbols.iter().map(|s| s.name.clone()).collect()
        }

        // First alt: expr OP expr @binop
        assert_eq!(names(&rule.alts[0]), vec!["expr", "OP", "expr"]);
        assert_eq!(rule.alts[0].name, Some("binop".to_string()));

        // Second alt: NUM @literal
        assert_eq!(names(&rule.alts[1]), vec!["NUM"]);
        assert_eq!(rule.alts[1].name, Some("literal".to_string()));

        // Third alt: LPAREN expr RPAREN (no name)
        assert_eq!(names(&rule.alts[2]), vec!["LPAREN", "expr", "RPAREN"]);
        assert_eq!(rule.alts[2].name, None);
    }

    #[test]
    fn test_rule_without_type() {
        let grammar_def = parse_grammar_typed(r#"
            grammar Untyped {
                start stmts;
                terminals { A, B, SEMI }

                stmts = | stmts SEMI stmt | stmt;
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

                prec_opt: PrecOpt = @no_prec | KW_PREC @has_prec;
                item: Item = prec_opt IDENT;
            }
        "#).unwrap();

        // Find prec_opt rule
        let prec_opt_rule = grammar_def.rules.iter()
            .find(|r| r.name == "prec_opt")
            .unwrap();

        assert_eq!(prec_opt_rule.result_type, Some("PrecOpt".to_string()));
        assert_eq!(prec_opt_rule.alts.len(), 2);

        // Helper to extract symbol names
        fn names(alt: &Alt) -> Vec<String> {
            alt.symbols.iter().map(|s| s.name.clone()).collect()
        }

        // First alt: @no_prec (empty production with name)
        assert!(names(&prec_opt_rule.alts[0]).is_empty());
        assert_eq!(prec_opt_rule.alts[0].name, Some("no_prec".to_string()));

        // Second alt: KW_PREC @has_prec
        assert_eq!(names(&prec_opt_rule.alts[1]), vec!["KW_PREC"]);
        assert_eq!(prec_opt_rule.alts[1].name, Some("has_prec".to_string()));
    }

    #[test]
    fn test_modifier_parsing() {
        let grammar_def = parse_grammar_typed(r#"
            grammar Modifiers {
                start expr;
                terminals { NUM: i32, OP, SEMI }

                expr: Expr = term tail*;
                tail: Tail = OP term;
                term: Term = NUM @num;
                stmts: Stmts = stmt+ @stmts;
                stmt: Stmt = expr SEMI @stmt;
                opt_expr: OptExpr = expr? SEMI;
            }
        "#).unwrap();

        // Helper to extract symbol info
        fn sym_info(alt: &Alt) -> Vec<(String, SymbolModifier)> {
            alt.symbols.iter().map(|s| (s.name.clone(), s.modifier)).collect()
        }

        // Find expr rule - should have tail*
        let expr_rule = grammar_def.rules.iter().find(|r| r.name == "expr").unwrap();
        assert_eq!(sym_info(&expr_rule.alts[0]), vec![
            ("term".to_string(), SymbolModifier::None),
            ("tail".to_string(), SymbolModifier::ZeroOrMore),
        ]);

        // Find stmts rule - should have stmt+
        let stmts_rule = grammar_def.rules.iter().find(|r| r.name == "stmts").unwrap();
        assert_eq!(sym_info(&stmts_rule.alts[0]), vec![
            ("stmt".to_string(), SymbolModifier::OneOrMore),
        ]);

        // Find opt_expr rule - should have expr?
        let opt_expr_rule = grammar_def.rules.iter().find(|r| r.name == "opt_expr").unwrap();
        assert_eq!(sym_info(&opt_expr_rule.alts[0]), vec![
            ("expr".to_string(), SymbolModifier::Optional),
            ("SEMI".to_string(), SymbolModifier::None),
        ]);
    }

    #[test]
    fn test_modifier_desugaring() {
        let mut grammar_def = parse_grammar_typed(r#"
            grammar Desugar {
                start items;
                terminals { A }

                items: Items = item* @items;
                item: Item = A @a;
            }
        "#).unwrap();

        // Before desugaring, items rule has item* modifier
        let items_rule = grammar_def.rules.iter().find(|r| r.name == "items").unwrap();
        assert_eq!(items_rule.alts[0].symbols[0].modifier, SymbolModifier::ZeroOrMore);

        // Apply desugaring
        desugar_modifiers(&mut grammar_def);

        // After desugaring:
        // - items rule should reference __item_star instead of item*
        // - A new __item_star rule should exist
        let items_rule = grammar_def.rules.iter().find(|r| r.name == "items").unwrap();
        assert_eq!(items_rule.alts[0].symbols[0].name, "__item_star");
        assert_eq!(items_rule.alts[0].symbols[0].modifier, SymbolModifier::None);

        // Check synthetic rule exists
        let synthetic = grammar_def.rules.iter().find(|r| r.name == "__item_star").unwrap();
        assert_eq!(synthetic.result_type, Some("Vec<Item>".to_string()));
        assert_eq!(synthetic.alts.len(), 2);
        // First alt: __item_star item @__append
        assert_eq!(synthetic.alts[0].name, Some("__append".to_string()));
        // Second alt: @__empty
        assert_eq!(synthetic.alts[1].name, Some("__empty".to_string()));
    }

    #[test]
    fn test_untyped_modifier_star() {
        // Test * modifier on untyped terminal -> Vec<()>
        let mut grammar_def = parse_grammar_typed(r#"
            grammar UntypedStar {
                start items;
                terminals { A, COMMA }

                items: Items = A COMMA* @items;
            }
        "#).unwrap();

        // Apply desugaring
        desugar_modifiers(&mut grammar_def);

        // Check synthetic rule for COMMA* has Vec<()> type
        let synthetic = grammar_def.rules.iter().find(|r| r.name == "__comma_star").unwrap();
        assert_eq!(synthetic.result_type, Some("Vec<()>".to_string()));
    }
}

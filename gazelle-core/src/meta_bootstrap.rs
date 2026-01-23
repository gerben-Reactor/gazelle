//! Bootstrap meta grammar parser.
//!
//! This module provides the meta grammar parser for parsing Gazelle grammar
//! definitions. The parser is generated from `meta.gzl` using the CLI.
//!
//! To regenerate `meta_generated.rs`:
//! ```bash
//! cargo build --release
//! ./target/release/gazelle --rust gazelle-core/meta.gzl > gazelle-core/src/meta_generated.rs
//! ```

#![allow(dead_code)]

use crate as gazelle_core;

/// Parsed representation of grammar elements.
#[derive(Debug, Clone)]
pub enum Ast {
    /// Sequence of symbol names
    Seq(Vec<String>),
    /// An alternative: sequence with optional reduction name
    Alt { symbols: Vec<String>, name: Option<String> },
    /// List of alternatives
    Alts(Vec<Ast>),
    /// A terminal definition: (name, optional type)
    TerminalDef { name: String, type_name: Option<String> },
    /// List of terminal definitions
    TerminalDefs(Vec<Ast>),
    /// A terminals block
    TerminalsBlock(Vec<Ast>),
    /// A prec terminal definition: (name, type)
    PrecTerminalDef { name: String, type_name: String },
    /// List of prec terminal definitions
    PrecTerminalDefs(Vec<Ast>),
    /// A prec terminals block
    PrecTerminalsBlock(Vec<Ast>),
    /// A rule: (name, optional result_type, alternatives)
    Rule { name: String, result_type: Option<String>, alts: Box<Ast> },
    /// List of sections
    Sections(Vec<Ast>),
    /// Full grammar definition
    GrammarDef { name: String, sections: Box<Ast> },
}

include!("meta_generated.rs");

/// Parse a sequence of tokens into an AST.
///
/// This is the core parsing function. Callers must provide tokens
/// (typically by lexing a grammar string).
pub fn parse_tokens<I>(tokens: I) -> Result<Ast, String>
where
    I: IntoIterator<Item = MetaTerminal>,
{
    let mut parser = MetaParser::new();
    let mut tokens = tokens.into_iter().peekable();

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
    parser.accept().map_err(|e| format!("Parse error at end, state {}", e.state))
}

/// Reduce a production to an AST node.
fn reduce(reduction: MetaReduction) -> __MetaReductionResult {
    match reduction {
        // grammar_def = KW_GRAMMAR IDENT LBRACE sections RBRACE
        MetaReduction::GrammarDefKwGrammarIdentLbraceSectionsRbrace(c, name, sections) => {
            c(Ast::GrammarDef { name, sections: Box::new(sections) })
        }

        // sections = sections section
        MetaReduction::SectionsSectionsSection(c, sections_ast, section_ast) => {
            let mut sections = match sections_ast {
                Ast::Sections(s) => s,
                other => vec![other],
            };
            sections.push(section_ast);
            c(Ast::Sections(sections))
        }
        // sections = section
        MetaReduction::SectionsSection(c, section_ast) => {
            c(Ast::Sections(vec![section_ast]))
        }

        // section = terminals_block | prec_terminals_block | rule
        MetaReduction::SectionTerminalsBlock(c, block) => c(block),
        MetaReduction::SectionPrecTerminalsBlock(c, block) => c(block),
        MetaReduction::SectionRule(c, rule) => c(rule),

        // terminals_block = KW_TERMINALS LBRACE terminal_list COMMA RBRACE (trailing comma)
        MetaReduction::TerminalsBlockKwTerminalsLbraceTerminalListCommaRbrace(c, list) => {
            let defs_vec = match list {
                Ast::TerminalDefs(d) => d,
                other => vec![other],
            };
            c(Ast::TerminalsBlock(defs_vec))
        }
        // terminals_block = KW_TERMINALS LBRACE terminal_list RBRACE
        MetaReduction::TerminalsBlockKwTerminalsLbraceTerminalListRbrace(c, list) => {
            let defs_vec = match list {
                Ast::TerminalDefs(d) => d,
                other => vec![other],
            };
            c(Ast::TerminalsBlock(defs_vec))
        }
        // terminals_block = KW_TERMINALS LBRACE RBRACE (empty)
        MetaReduction::TerminalsBlockKwTerminalsLbraceRbrace(c) => {
            c(Ast::TerminalsBlock(vec![]))
        }

        // terminal_list = terminal_list COMMA terminal_item
        MetaReduction::TerminalListTerminalListCommaTerminalItem(c, list_ast, item_ast) => {
            let mut defs = match list_ast {
                Ast::TerminalDefs(d) => d,
                other => vec![other],
            };
            defs.push(item_ast);
            c(Ast::TerminalDefs(defs))
        }
        // terminal_list = terminal_item
        MetaReduction::TerminalListTerminalItem(c, item_ast) => {
            c(Ast::TerminalDefs(vec![item_ast]))
        }

        // terminal_item = IDENT COLON IDENT
        MetaReduction::TerminalItemIdentColonIdent(c, name, type_name) => {
            c(Ast::TerminalDef { name, type_name: Some(type_name) })
        }
        // terminal_item = IDENT
        MetaReduction::TerminalItemIdent(c, name) => {
            c(Ast::TerminalDef { name, type_name: None })
        }

        // prec_terminals_block = KW_PREC_TERMINALS LBRACE prec_terminal_list COMMA RBRACE (trailing comma)
        MetaReduction::PrecTerminalsBlockKwPrecTerminalsLbracePrecTerminalListCommaRbrace(c, list) => {
            let defs_vec = match list {
                Ast::PrecTerminalDefs(d) => d,
                other => vec![other],
            };
            c(Ast::PrecTerminalsBlock(defs_vec))
        }
        // prec_terminals_block = KW_PREC_TERMINALS LBRACE prec_terminal_list RBRACE
        MetaReduction::PrecTerminalsBlockKwPrecTerminalsLbracePrecTerminalListRbrace(c, list) => {
            let defs_vec = match list {
                Ast::PrecTerminalDefs(d) => d,
                other => vec![other],
            };
            c(Ast::PrecTerminalsBlock(defs_vec))
        }
        // prec_terminals_block = KW_PREC_TERMINALS LBRACE RBRACE (empty)
        MetaReduction::PrecTerminalsBlockKwPrecTerminalsLbraceRbrace(c) => {
            c(Ast::PrecTerminalsBlock(vec![]))
        }

        // prec_terminal_list = prec_terminal_list COMMA prec_terminal_item
        MetaReduction::PrecTerminalListPrecTerminalListCommaPrecTerminalItem(c, list_ast, item_ast) => {
            let mut defs = match list_ast {
                Ast::PrecTerminalDefs(d) => d,
                other => vec![other],
            };
            defs.push(item_ast);
            c(Ast::PrecTerminalDefs(defs))
        }
        // prec_terminal_list = prec_terminal_item
        MetaReduction::PrecTerminalListPrecTerminalItem(c, item_ast) => {
            c(Ast::PrecTerminalDefs(vec![item_ast]))
        }

        // prec_terminal_item = IDENT COLON IDENT
        MetaReduction::PrecTerminalItemIdentColonIdent(c, name, type_name) => {
            c(Ast::PrecTerminalDef { name, type_name })
        }

        // rule = IDENT COLON IDENT EQ alts SEMI
        MetaReduction::RuleIdentColonIdentEqAltsSemi(c, name, result_type, alts) => {
            c(Ast::Rule { name, result_type: Some(result_type), alts: Box::new(alts) })
        }
        // rule = IDENT EQ alts SEMI (no type annotation)
        MetaReduction::RuleIdentEqAltsSemi(c, name, alts) => {
            c(Ast::Rule { name, result_type: None, alts: Box::new(alts) })
        }

        // alts = alts PIPE alt
        MetaReduction::AltsAltsPipeAlt(c, alts_ast, alt_ast) => {
            let mut alts = match alts_ast {
                Ast::Alts(a) => a,
                other => vec![other],
            };
            alts.push(alt_ast);
            c(Ast::Alts(alts))
        }
        // alts = alts PIPE (empty alternative)
        MetaReduction::AltsAltsPipe(c, alts_ast) => {
            let mut alts = match alts_ast {
                Ast::Alts(a) => a,
                other => vec![other],
            };
            alts.push(Ast::Alt { symbols: vec![], name: None }); // empty alternative
            c(Ast::Alts(alts))
        }
        // alts = alt
        MetaReduction::AltsAlt(c, alt_ast) => {
            c(Ast::Alts(vec![alt_ast]))
        }

        // alt = seq AT IDENT (named alternative)
        MetaReduction::AltSeqAtIdent(c, seq_ast, name) => {
            let symbols = match seq_ast {
                Ast::Seq(s) => s,
                _ => panic!("Expected Seq"),
            };
            c(Ast::Alt { symbols, name: Some(name) })
        }
        // alt = seq (unnamed alternative)
        MetaReduction::AltSeq(c, seq_ast) => {
            let symbols = match seq_ast {
                Ast::Seq(s) => s,
                _ => panic!("Expected Seq"),
            };
            c(Ast::Alt { symbols, name: None })
        }

        // seq = seq IDENT
        MetaReduction::SeqSeqIdent(c, seq_ast, symbol) => {
            let mut seq = match seq_ast {
                Ast::Seq(s) => s,
                _ => panic!("Expected Seq"),
            };
            seq.push(symbol);
            c(Ast::Seq(seq))
        }
        // seq = IDENT
        MetaReduction::SeqIdent(c, symbol) => {
            c(Ast::Seq(vec![symbol]))
        }
    }
}

/// Convert an AST to a Grammar.
pub fn ast_to_grammar(ast: Ast) -> Result<crate::Grammar, String> {
    use crate::{GrammarBuilder, Symbol};

    // Extract grammar definition
    let (grammar_name, sections) = match ast {
        Ast::GrammarDef { name, sections } => (name, *sections),
        _ => return Err("Expected GrammarDef".to_string()),
    };

    let sections_vec = match sections {
        Ast::Sections(s) => s,
        other => vec![other],
    };

    let mut gb = GrammarBuilder::new();
    let mut rule_data: Vec<(String, String, Vec<Vec<String>>)> = Vec::new();

    // First pass: process terminals and prec_terminals blocks, collect rules
    for section in &sections_vec {
        match section {
            Ast::TerminalsBlock(defs) => {
                for def in defs {
                    if let Ast::TerminalDef { name, .. } = def {
                        gb.t(name);
                    }
                }
            }
            Ast::PrecTerminalsBlock(defs) => {
                for def in defs {
                    if let Ast::PrecTerminalDef { name, .. } = def {
                        gb.pt(name);
                    }
                }
            }
            Ast::Rule { name, result_type, alts } => {
                let alts_vec = match alts.as_ref() {
                    Ast::Alts(a) => a.clone(),
                    other => vec![other.clone()],
                };

                let mut alt_seqs = Vec::new();
                for alt in alts_vec {
                    let seq = match alt {
                        Ast::Seq(s) => s,
                        Ast::Alt { symbols, .. } => symbols,
                        _ => return Err("Expected Seq or Alt in alternatives".to_string()),
                    };
                    alt_seqs.push(seq);
                }
                rule_data.push((name.clone(), result_type.clone().unwrap_or_default(), alt_seqs));
            }
            _ => return Err(format!("Unexpected section type: {:?}", section)),
        }
    }

    // Second pass: intern non-terminals
    let mut nt_symbols: Vec<(String, Symbol)> = Vec::new();
    for (name, _, _) in &rule_data {
        let lhs = gb.nt(name);
        nt_symbols.push((name.clone(), lhs));
    }

    // Third pass: build rules
    for (name, _, alt_seqs) in &rule_data {
        let lhs = nt_symbols.iter().find(|(n, _)| n == name).map(|(_, s)| *s).unwrap();

        for seq in alt_seqs {
            let rhs: Vec<Symbol> = seq.iter().map(|sym_name| {
                // First check if it's a non-terminal
                if let Some((_, sym)) = nt_symbols.iter().find(|(n, _)| n == sym_name) {
                    return *sym;
                }
                // Otherwise it should be a terminal
                gb.symbols.get(sym_name)
                    .ok_or_else(|| format!("Unknown symbol: {}", sym_name))
                    .unwrap()
            }).collect();

            gb.rule(lhs, rhs);
        }
    }

    if rule_data.is_empty() {
        return Err(format!("Grammar '{}' has no rules", grammar_name));
    }

    Ok(gb.build())
}

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

// Type alias for IDENT terminal payload
pub type Ident = String;

// ============================================================================
// Public AST types
// ============================================================================

#[derive(Debug, Clone)]
pub struct GrammarDef {
    pub name: String,
    pub start: Option<String>,
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
pub struct Sections {
    start: Option<String>,
    terminals: Vec<TerminalDef>,
    rules: Vec<Rule>,
}

impl Sections {
    fn new() -> Self {
        Self {
            start: None,
            terminals: vec![],
            rules: vec![],
        }
    }
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub enum Section {
    Terminals(Vec<TerminalDef>),
    Rule(Rule),
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct TerminalList(Vec<TerminalDef>);

// ============================================================================
// Generated parser
// ============================================================================

include!("meta_generated.rs");

// ============================================================================
// AST builder implementing MetaActions
// ============================================================================

pub struct AstBuilder;

impl MetaActions for AstBuilder {
    type GrammarDef = GrammarDef;
    type Sections = Sections;
    type Section = Section;
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

    fn grammar_def(&mut self, name: Ident, sections: Sections) -> GrammarDef {
        GrammarDef {
            name,
            start: sections.start,
            terminals: sections.terminals,
            rules: sections.rules,
        }
    }

    fn sections_append(&mut self, mut sections: Sections, section: Section) -> Sections {
        match section {
            Section::Terminals(defs) => sections.terminals.extend(defs),
            Section::Rule(rule) => sections.rules.push(rule),
        }
        sections
    }

    fn sections_single(&mut self, section: Section) -> Sections {
        let mut sections = Sections::new();
        match section {
            Section::Terminals(defs) => sections.terminals = defs,
            Section::Rule(rule) => sections.rules.push(rule),
        }
        sections
    }

    fn section_terminals(&mut self, defs: Vec<TerminalDef>) -> Section {
        Section::Terminals(defs)
    }

    fn section_rule(&mut self, rule: Rule) -> Section {
        Section::Rule(rule)
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

    fn rule_typed(&mut self, name: Ident, result_type: Ident, alts: Vec<Alt>) -> Rule {
        Rule { name, result_type: Some(result_type), alts }
    }

    fn rule_untyped(&mut self, name: Ident, alts: Vec<Alt>) -> Rule {
        Rule { name, result_type: None, alts }
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
// Parsing API
// ============================================================================

/// Parse tokens into typed AST.
pub fn parse_tokens_typed<I>(tokens: I) -> Result<GrammarDef, String>
where
    I: IntoIterator<Item = MetaTerminal>,
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

/// Convert typed AST to Grammar.
pub fn grammar_def_to_grammar(grammar_def: GrammarDef) -> Result<crate::Grammar, String> {
    use crate::{GrammarBuilder, Symbol};

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

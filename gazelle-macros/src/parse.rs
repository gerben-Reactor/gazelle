//! Parse the grammar! macro input syntax.
//!
//! Syntax:
//! ```ignore
//! grammar! {
//!     pub grammar Name {
//!         terminals {
//!             TERMINAL_NAME,
//!             TERMINAL_WITH_PAYLOAD: Type,
//!         }
//!
//!         prec_terminals {
//!             OP: OperatorType,  // Precedence added automatically
//!         }
//!
//!         rule_name: ResultType = symbol1 symbol2 | symbol3;
//!     }
//! }
//! ```

use syn::parse::{Parse, ParseStream};
use syn::{braced, Ident, Result, Token, Type, Visibility};

use crate::ir::{Alternative, GrammarIr, GrammarSymbol, PrecTerminalDef, RuleDef, TerminalDef};

/// Custom keywords for the grammar syntax.
mod kw {
    syn::custom_keyword!(grammar);
    syn::custom_keyword!(terminals);
    syn::custom_keyword!(prec_terminals);
}

/// Parse the entire grammar! macro input.
impl Parse for GrammarIr {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse visibility (optional)
        let visibility: Visibility = input.parse()?;

        // Parse "grammar" keyword
        input.parse::<kw::grammar>()?;

        // Parse grammar name
        let name: Ident = input.parse()?;

        // Parse the body in braces
        let content;
        braced!(content in input);

        let mut terminals = Vec::new();
        let mut prec_terminals = Vec::new();
        let mut rules = Vec::new();

        // Parse blocks in order: terminals, prec_terminals (optional), rules
        while !content.is_empty() {
            if content.peek(kw::terminals) {
                content.parse::<kw::terminals>()?;
                let terminal_content;
                braced!(terminal_content in content);
                terminals = parse_terminals(&terminal_content)?;
            } else if content.peek(kw::prec_terminals) {
                content.parse::<kw::prec_terminals>()?;
                let prec_content;
                braced!(prec_content in content);
                prec_terminals = parse_prec_terminals(&prec_content)?;
            } else {
                // Parse a rule
                let rule = parse_rule(&content)?;
                rules.push(rule);
            }
        }

        Ok(GrammarIr {
            visibility,
            name,
            terminals,
            prec_terminals,
            rules,
        })
    }
}

/// Parse terminal definitions.
fn parse_terminals(input: ParseStream) -> Result<Vec<TerminalDef>> {
    let mut terminals = Vec::new();

    while !input.is_empty() {
        let name: Ident = input.parse()?;

        let payload_type = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            Some(input.parse::<Type>()?)
        } else {
            None
        };

        terminals.push(TerminalDef { name, payload_type });

        // Consume trailing comma if present
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
    }

    Ok(terminals)
}

/// Parse precedence terminal definitions.
fn parse_prec_terminals(input: ParseStream) -> Result<Vec<PrecTerminalDef>> {
    let mut prec_terminals = Vec::new();

    while !input.is_empty() {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let payload_type: Type = input.parse()?;

        prec_terminals.push(PrecTerminalDef { name, payload_type });

        // Consume trailing comma if present
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }
    }

    Ok(prec_terminals)
}

/// Parse a grammar rule.
fn parse_rule(input: ParseStream) -> Result<RuleDef> {
    // rule_name: ResultType = alternatives;
    let name: Ident = input.parse()?;
    input.parse::<Token![:]>()?;
    let result_type: Type = input.parse()?;
    input.parse::<Token![=]>()?;

    let alternatives = parse_alternatives(input)?;

    input.parse::<Token![;]>()?;

    Ok(RuleDef {
        name,
        result_type,
        alternatives,
    })
}

/// Parse rule alternatives separated by |.
fn parse_alternatives(input: ParseStream) -> Result<Vec<Alternative>> {
    let mut alternatives = Vec::new();

    loop {
        let symbols = parse_symbols(input)?;
        alternatives.push(Alternative { symbols });

        if input.peek(Token![|]) {
            input.parse::<Token![|]>()?;
        } else {
            break;
        }
    }

    Ok(alternatives)
}

/// Parse symbols in an alternative (until | or ;).
fn parse_symbols(input: ParseStream) -> Result<Vec<GrammarSymbol>> {
    let mut symbols = Vec::new();

    while !input.peek(Token![|]) && !input.peek(Token![;]) {
        let ident: Ident = input.parse()?;

        // Determine if terminal or non-terminal by naming convention:
        // - UPPERCASE = terminal
        // - lowercase = non-terminal
        let symbol = if is_terminal_name(&ident.to_string()) {
            GrammarSymbol::Terminal(ident)
        } else {
            GrammarSymbol::NonTerminal(ident)
        };

        symbols.push(symbol);
    }

    Ok(symbols)
}

/// Check if a name follows terminal naming convention (all uppercase).
fn is_terminal_name(name: &str) -> bool {
    name.chars()
        .all(|c| c.is_ascii_uppercase() || c == '_' || c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;

    #[test]
    fn test_is_terminal_name() {
        assert!(is_terminal_name("NUM"));
        assert!(is_terminal_name("LPAREN"));
        assert!(is_terminal_name("LEFT_PAREN"));
        assert!(is_terminal_name("OP2"));
        assert!(!is_terminal_name("expr"));
        assert!(!is_terminal_name("Expr"));
        assert!(!is_terminal_name("foo_bar"));
    }

    #[test]
    fn test_parse_simple_grammar() {
        let input: TokenStream = quote::quote! {
            pub grammar Calc {
                terminals {
                    NUM: f64,
                    LPAREN,
                    RPAREN,
                }

                prec_terminals {
                    OP: Operator,
                }

                expr: Expr = expr OP expr | atom;
                atom: Atom = NUM | LPAREN expr RPAREN;
            }
        };

        let grammar: GrammarIr = syn::parse2(input).unwrap();

        assert_eq!(grammar.name.to_string(), "Calc");
        assert_eq!(grammar.terminals.len(), 3);
        assert_eq!(grammar.prec_terminals.len(), 1);
        assert_eq!(grammar.rules.len(), 2);

        // Check terminals
        assert_eq!(grammar.terminals[0].name.to_string(), "NUM");
        assert!(grammar.terminals[0].payload_type.is_some());
        assert_eq!(grammar.terminals[1].name.to_string(), "LPAREN");
        assert!(grammar.terminals[1].payload_type.is_none());

        // Check rules
        assert_eq!(grammar.rules[0].name.to_string(), "expr");
        assert_eq!(grammar.rules[0].alternatives.len(), 2);
        assert_eq!(grammar.rules[1].name.to_string(), "atom");
        assert_eq!(grammar.rules[1].alternatives.len(), 2);
    }
}

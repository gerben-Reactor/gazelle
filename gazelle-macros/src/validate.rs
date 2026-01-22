//! Grammar validation.

use std::collections::HashSet;
use syn::Error;

use crate::ir::{GrammarIr, GrammarSymbol};

/// Validate a grammar IR, returning an error if invalid.
pub fn validate(grammar: &GrammarIr) -> Result<(), Error> {
    check_no_duplicate_symbols(grammar)?;
    check_symbols_defined(grammar)?;
    check_has_rules(grammar)?;
    Ok(())
}

/// Check that no symbol name is defined more than once.
fn check_no_duplicate_symbols(grammar: &GrammarIr) -> Result<(), Error> {
    let mut seen = HashSet::new();

    for terminal in &grammar.terminals {
        if !seen.insert(terminal.name.to_string()) {
            return Err(Error::new(
                terminal.name.span(),
                format!("duplicate symbol `{}`", terminal.name),
            ));
        }
    }

    for prec_terminal in &grammar.prec_terminals {
        if !seen.insert(prec_terminal.name.to_string()) {
            return Err(Error::new(
                prec_terminal.name.span(),
                format!("duplicate symbol `{}`", prec_terminal.name),
            ));
        }
    }

    for rule in &grammar.rules {
        if !seen.insert(rule.name.to_string()) {
            return Err(Error::new(
                rule.name.span(),
                format!("duplicate symbol `{}`", rule.name),
            ));
        }
    }

    Ok(())
}

/// Check that all referenced symbols are defined.
fn check_symbols_defined(grammar: &GrammarIr) -> Result<(), Error> {
    for rule in &grammar.rules {
        for alt in &rule.alternatives {
            for symbol in &alt.symbols {
                let name = symbol.name();
                let name_str = name.to_string();

                match symbol {
                    GrammarSymbol::Terminal(_) => {
                        if !grammar.is_terminal(&name_str) {
                            return Err(Error::new(
                                name.span(),
                                format!("undefined terminal `{}`", name_str),
                            ));
                        }
                    }
                    GrammarSymbol::NonTerminal(_) => {
                        if !grammar.is_non_terminal(&name_str) {
                            return Err(Error::new(
                                name.span(),
                                format!("undefined non-terminal `{}`", name_str),
                            ));
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

/// Check that at least one rule is defined.
fn check_has_rules(grammar: &GrammarIr) -> Result<(), Error> {
    if grammar.rules.is_empty() {
        return Err(Error::new(
            grammar.name.span(),
            "grammar must have at least one rule",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;

    fn parse_grammar(input: TokenStream) -> GrammarIr {
        syn::parse2(input).unwrap()
    }

    #[test]
    fn test_valid_grammar() {
        let grammar = parse_grammar(quote::quote! {
            grammar Test {
                terminals { A }
                s: () = A;
            }
        });
        assert!(validate(&grammar).is_ok());
    }

    #[test]
    fn test_duplicate_terminal() {
        let grammar = parse_grammar(quote::quote! {
            grammar Test {
                terminals { A, A }
                s: () = A;
            }
        });
        let err = validate(&grammar).unwrap_err();
        assert!(err.to_string().contains("duplicate symbol"));
    }

    #[test]
    fn test_undefined_terminal() {
        let grammar = parse_grammar(quote::quote! {
            grammar Test {
                terminals { A }
                s: () = B;
            }
        });
        let err = validate(&grammar).unwrap_err();
        assert!(err.to_string().contains("undefined terminal"));
    }

    #[test]
    fn test_undefined_non_terminal() {
        let grammar = parse_grammar(quote::quote! {
            grammar Test {
                terminals { A }
                s: () = foo;
            }
        });
        let err = validate(&grammar).unwrap_err();
        assert!(err.to_string().contains("undefined non-terminal"));
    }

    #[test]
    fn test_no_rules() {
        let grammar = parse_grammar(quote::quote! {
            grammar Test {
                terminals { A }
            }
        });
        let err = validate(&grammar).unwrap_err();
        assert!(err.to_string().contains("at least one rule"));
    }
}

//! Intermediate representation for grammar definitions.

use syn::{Ident, Type, Visibility};

/// Complete grammar definition.
#[derive(Debug)]
pub struct GrammarIr {
    pub visibility: Visibility,
    pub name: Ident,
    pub terminals: Vec<TerminalDef>,
    pub prec_terminals: Vec<PrecTerminalDef>,
    pub rules: Vec<RuleDef>,
}

/// A regular terminal definition.
#[derive(Debug, Clone)]
pub struct TerminalDef {
    pub name: Ident,
    /// None means unit type (no payload).
    pub payload_type: Option<Type>,
}

/// A precedence terminal definition.
/// Precedence is added automatically to the generated type.
#[derive(Debug, Clone)]
pub struct PrecTerminalDef {
    pub name: Ident,
    /// The user-facing type (without Precedence).
    pub payload_type: Type,
}

/// A grammar rule (production) definition.
#[derive(Debug)]
pub struct RuleDef {
    pub name: Ident,
    pub result_type: Type,
    pub alternatives: Vec<Alternative>,
}

/// A single alternative in a rule.
#[derive(Debug, Clone)]
pub struct Alternative {
    pub symbols: Vec<GrammarSymbol>,
}

/// A symbol reference in a rule body.
#[derive(Debug, Clone)]
pub enum GrammarSymbol {
    Terminal(Ident),
    NonTerminal(Ident),
}

impl GrammarSymbol {
    pub fn name(&self) -> &Ident {
        match self {
            GrammarSymbol::Terminal(name) => name,
            GrammarSymbol::NonTerminal(name) => name,
        }
    }
}

#[allow(dead_code)]
impl GrammarIr {
    /// Get the start symbol (first non-terminal).
    pub fn start_symbol(&self) -> Option<&RuleDef> {
        self.rules.first()
    }

    /// Check if a name refers to a terminal.
    pub fn is_terminal(&self, name: &str) -> bool {
        self.terminals.iter().any(|t| t.name == name)
            || self.prec_terminals.iter().any(|t| t.name == name)
    }

    /// Check if a name refers to a prec_terminal.
    pub fn is_prec_terminal(&self, name: &str) -> bool {
        self.prec_terminals.iter().any(|t| t.name == name)
    }

    /// Check if a name refers to a non-terminal.
    pub fn is_non_terminal(&self, name: &str) -> bool {
        self.rules.iter().any(|r| r.name == name)
    }

    /// Get a terminal definition by name.
    pub fn get_terminal(&self, name: &str) -> Option<&TerminalDef> {
        self.terminals.iter().find(|t| t.name == name)
    }

    /// Get a prec_terminal definition by name.
    pub fn get_prec_terminal(&self, name: &str) -> Option<&PrecTerminalDef> {
        self.prec_terminals.iter().find(|t| t.name == name)
    }

    /// Get a rule definition by name.
    pub fn get_rule(&self, name: &str) -> Option<&RuleDef> {
        self.rules.iter().find(|r| r.name == name)
    }

    /// Iterate over all defined symbol names.
    pub fn all_symbol_names(&self) -> impl Iterator<Item = &Ident> {
        self.terminals
            .iter()
            .map(|t| &t.name)
            .chain(self.prec_terminals.iter().map(|t| &t.name))
            .chain(self.rules.iter().map(|r| &r.name))
    }

    /// Convert terminal name to PascalCase for enum variant.
    pub fn terminal_variant_name(name: &Ident) -> Ident {
        let s = name.to_string();
        let pascal = to_pascal_case(&s);
        Ident::new(&pascal, name.span())
    }
}

/// Convert SCREAMING_SNAKE_CASE or lowercase to PascalCase.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("NUM"), "Num");
        assert_eq!(to_pascal_case("LPAREN"), "Lparen");
        assert_eq!(to_pascal_case("LEFT_PAREN"), "LeftParen");
        assert_eq!(to_pascal_case("foo"), "Foo");
        assert_eq!(to_pascal_case("foo_bar"), "FooBar");
    }
}

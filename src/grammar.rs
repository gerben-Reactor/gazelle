/// Associativity for precedence-carrying operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Assoc {
    Left,
    Right,
}

/// Precedence information carried by a token at parse time.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Precedence {
    pub level: u8,
    pub assoc: Assoc,
}

impl Precedence {
    pub fn left(level: u8) -> Self {
        Self { level, assoc: Assoc::Left }
    }

    pub fn right(level: u8) -> Self {
        Self { level, assoc: Assoc::Right }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Symbol {
    /// A regular terminal symbol.
    Terminal(String),
    /// A terminal that carries precedence at runtime (e.g., operators).
    /// Shift/reduce conflicts on these are resolved by comparing precedences.
    PrecTerminal(String),
    /// A non-terminal symbol.
    NonTerminal(String),
}

impl Symbol {
    pub fn is_terminal(&self) -> bool {
        matches!(self, Symbol::Terminal(_) | Symbol::PrecTerminal(_))
    }

    pub fn is_prec_terminal(&self) -> bool {
        matches!(self, Symbol::PrecTerminal(_))
    }

    pub fn is_non_terminal(&self) -> bool {
        matches!(self, Symbol::NonTerminal(_))
    }

    pub fn name(&self) -> &str {
        match self {
            Symbol::Terminal(s) | Symbol::PrecTerminal(s) | Symbol::NonTerminal(s) => s,
        }
    }
}

/// Helper to create a terminal symbol.
pub fn t(name: &str) -> Symbol {
    Symbol::Terminal(name.to_string())
}

/// Helper to create a precedence-carrying terminal symbol.
pub fn pt(name: &str) -> Symbol {
    Symbol::PrecTerminal(name.to_string())
}

/// Helper to create a non-terminal symbol.
pub fn nt(name: &str) -> Symbol {
    Symbol::NonTerminal(name.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub lhs: Symbol,
    pub rhs: Vec<Symbol>,
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub rules: Vec<Rule>,
    pub start: Symbol,
}

impl Grammar {
    /// Returns all rules with the given non-terminal on the left-hand side.
    pub fn rules_for(&self, symbol: &Symbol) -> impl Iterator<Item = (usize, &Rule)> {
        self.rules
            .iter()
            .enumerate()
            .filter(move |(_, rule)| &rule.lhs == symbol)
    }

    /// Returns all non-terminal symbols in the grammar.
    pub fn non_terminals(&self) -> impl Iterator<Item = &Symbol> {
        self.rules.iter().map(|r| &r.lhs)
    }

    /// Returns all terminal symbols in the grammar.
    pub fn terminals(&self) -> impl Iterator<Item = &Symbol> {
        self.rules
            .iter()
            .flat_map(|r| r.rhs.iter())
            .filter(|s| s.is_terminal())
    }

    /// Create an augmented grammar with a new start rule: __start -> <original_start>
    /// Returns the augmented grammar and the index of the augmented rule (always 0).
    pub fn augment(&self) -> Grammar {
        let aug_start = nt("__start");
        let aug_rule = Rule {
            lhs: aug_start.clone(),
            rhs: vec![self.start.clone()],
        };

        let mut rules = vec![aug_rule];
        rules.extend(self.rules.iter().cloned());

        Grammar {
            rules,
            start: aug_start,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_grammar() {
        let grammar = Grammar {
            start: nt("expr"),
            rules: vec![
                Rule { lhs: nt("expr"), rhs: vec![nt("expr"), t("+"), nt("term")] },
                Rule { lhs: nt("expr"), rhs: vec![nt("term")] },
                Rule { lhs: nt("term"), rhs: vec![t("NUM")] },
            ],
        };

        assert_eq!(grammar.rules.len(), 3);
        assert_eq!(grammar.rules_for(&nt("expr")).count(), 2);
        assert_eq!(grammar.rules_for(&nt("term")).count(), 1);
    }
}

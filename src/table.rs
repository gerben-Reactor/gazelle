use crate::grammar::{Grammar, Symbol};
use crate::lr::Automaton;
use std::collections::HashMap;

/// An action in the parse table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Shift the token and go to the given state.
    Shift(usize),
    /// Reduce using the given rule index.
    Reduce(usize),
    /// Shift/reduce conflict resolved by precedence at runtime.
    /// Only generated for PrecTerminal symbols.
    ShiftOrReduce { shift_state: usize, reduce_rule: usize },
    /// Accept the input.
    Accept,
    /// Error (no valid action).
    Error,
}

/// A conflict between two actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Conflict {
    ShiftReduce {
        state: usize,
        terminal: Symbol,
        shift_state: usize,
        reduce_rule: usize,
    },
    ReduceReduce {
        state: usize,
        terminal: Symbol,
        rule1: usize,
        rule2: usize,
    },
}

/// The parse tables for an LR parser.
#[derive(Debug)]
pub struct ParseTable {
    /// Action table: (state, terminal) -> Action
    pub action: HashMap<(usize, Option<Symbol>), Action>,
    /// Goto table: (state, non-terminal) -> state
    pub goto: HashMap<(usize, Symbol), usize>,
    /// The augmented grammar (rule 0 is __start -> original_start).
    pub grammar: Grammar,
    /// Number of states.
    pub num_states: usize,
    /// Conflicts detected during table construction.
    pub conflicts: Vec<Conflict>,
}

impl ParseTable {
    /// Build parse tables from an automaton.
    ///
    /// Uses the augmented grammar stored in the automaton.
    /// Rule 0 is always the accept rule (__start -> original_start).
    pub fn build(automaton: &Automaton) -> Self {
        let grammar = &automaton.grammar;
        let mut action: HashMap<(usize, Option<Symbol>), Action> = HashMap::new();
        let mut goto: HashMap<(usize, Symbol), usize> = HashMap::new();
        let mut conflicts = Vec::new();

        // Rule 0 is always the augmented start rule: __start -> <original_start>
        // When this rule completes at EOF, we accept.
        const ACCEPT_RULE: usize = 0;

        for (state_idx, state) in automaton.states.iter().enumerate() {
            // Process each item in the state
            for item in state.iter() {
                if item.is_complete(grammar) {
                    // Reduce item: A -> α •, a
                    // Add reduce action for lookahead
                    let key = (state_idx, item.lookahead.clone());

                    // Check if this is the accept state (rule 0 complete at EOF)
                    if item.rule == ACCEPT_RULE && item.lookahead.is_none() {
                        insert_action(&mut action, &mut conflicts, key, Action::Accept);
                    } else {
                        insert_action(&mut action, &mut conflicts, key, Action::Reduce(item.rule));
                    }
                } else if let Some(next_symbol) = item.next_symbol(grammar) {
                    // Shift item: A -> α • X β
                    if let Some(&next_state) = automaton.transitions.get(&(state_idx, next_symbol.clone())) {
                        match next_symbol {
                            Symbol::Terminal(_) | Symbol::PrecTerminal(_) => {
                                let key = (state_idx, Some(next_symbol.clone()));
                                insert_action(&mut action, &mut conflicts, key, Action::Shift(next_state));
                            }
                            Symbol::NonTerminal(_) => {
                                goto.insert((state_idx, next_symbol.clone()), next_state);
                            }
                        }
                    }
                }
            }
        }

        ParseTable {
            action,
            goto,
            grammar: grammar.clone(),
            num_states: automaton.num_states(),
            conflicts,
        }
    }

    /// Get the action for a state and terminal.
    pub fn action(&self, state: usize, terminal: Option<&Symbol>) -> &Action {
        self.action.get(&(state, terminal.cloned())).unwrap_or(&Action::Error)
    }

    /// Get the goto state for a state and non-terminal.
    pub fn goto(&self, state: usize, non_terminal: &Symbol) -> Option<usize> {
        self.goto.get(&(state, non_terminal.clone())).copied()
    }

    /// Returns true if the table has conflicts.
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
}

fn insert_action(
    action: &mut HashMap<(usize, Option<Symbol>), Action>,
    conflicts: &mut Vec<Conflict>,
    key: (usize, Option<Symbol>),
    new_action: Action,
) {
    if let Some(existing) = action.get(&key).cloned() {
        if existing != new_action {
            // Conflict detected - check if it can be resolved by precedence
            let is_prec_terminal = key.1.as_ref().map_or(false, |s| s.is_prec_terminal());

            match (&new_action, &existing) {
                (Action::Shift(shift_state), Action::Reduce(reduce_rule))
                | (Action::Reduce(reduce_rule), Action::Shift(shift_state)) => {
                    if is_prec_terminal {
                        // PrecTerminal: resolve at runtime via precedence
                        action.insert(key, Action::ShiftOrReduce {
                            shift_state: *shift_state,
                            reduce_rule: *reduce_rule,
                        });
                    } else {
                        // Regular terminal: report conflict
                        conflicts.push(Conflict::ShiftReduce {
                            state: key.0,
                            terminal: key.1.clone().unwrap_or_else(|| Symbol::Terminal("$".to_string())),
                            shift_state: *shift_state,
                            reduce_rule: *reduce_rule,
                        });
                    }
                }
                (Action::Reduce(rule1), Action::Reduce(rule2)) => {
                    // Reduce/reduce conflicts are always reported (can't resolve by precedence)
                    conflicts.push(Conflict::ReduceReduce {
                        state: key.0,
                        terminal: key.1.clone().unwrap_or_else(|| Symbol::Terminal("$".to_string())),
                        rule1: *rule1,
                        rule2: *rule2,
                    });
                }
                _ => {} // Same action or Accept, no real conflict
            }
        }
    } else {
        action.insert(key, new_action);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::t;
    use crate::meta::parse_grammar;

    #[test]
    fn test_simple_table() {
        let grammar = parse_grammar("S = 'a' ;").unwrap();
        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        assert!(!table.has_conflicts());

        match table.action(0, Some(&t("a"))) {
            Action::Shift(_) => {}
            other => panic!("Expected Shift, got {:?}", other),
        }
    }

    #[test]
    fn test_expr_table() {
        let grammar = parse_grammar(r#"
            expr = expr '+' term | term ;
            term = 'NUM' ;
        "#).unwrap();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        assert!(!table.has_conflicts(), "Unexpected conflicts: {:?}", table.conflicts);

        match table.action(0, Some(&t("NUM"))) {
            Action::Shift(_) => {}
            other => panic!("Expected Shift on NUM, got {:?}", other),
        }
    }

    #[test]
    fn test_ambiguous_grammar() {
        // expr -> expr + expr | NUM is ambiguous (shift/reduce on +)
        let grammar = parse_grammar(r#"
            expr = expr '+' expr | 'NUM' ;
        "#).unwrap();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        assert!(table.has_conflicts(), "Expected conflicts for ambiguous grammar");

        let has_sr_conflict = table.conflicts.iter().any(|c| {
            matches!(c, Conflict::ShiftReduce { terminal, .. } if terminal == &t("+"))
        });
        assert!(has_sr_conflict, "Expected shift/reduce conflict on +");
    }

    #[test]
    fn test_prec_terminal_no_conflict() {
        // Same ambiguous grammar but with <OP> precedence terminal
        // expr -> expr <OP> expr | NUM
        let grammar = parse_grammar(r#"
            expr = expr <OP> expr | 'NUM' ;
        "#).unwrap();

        let automaton = Automaton::build(&grammar);
        let table = ParseTable::build(&automaton);

        // No reported conflicts - ShiftOrReduce is used instead
        assert!(!table.has_conflicts(), "PrecTerminal should not report conflicts: {:?}", table.conflicts);

        // Verify ShiftOrReduce action exists for OP
        let has_shift_or_reduce = table.action.values().any(|a| {
            matches!(a, Action::ShiftOrReduce { .. })
        });
        assert!(has_shift_or_reduce, "Expected ShiftOrReduce action for precedence terminal");
    }
}

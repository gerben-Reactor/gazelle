use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Generic NFA: states with labeled transitions and epsilon edges.
pub(crate) struct Nfa {
    transitions: Vec<Vec<(u32, usize)>>,
    epsilon: Vec<Vec<usize>>,
}

impl Nfa {
    pub fn new() -> Self {
        Self {
            transitions: Vec::new(),
            epsilon: Vec::new(),
        }
    }

    pub fn add_state(&mut self) -> usize {
        let id = self.transitions.len();
        self.transitions.push(Vec::new());
        self.epsilon.push(Vec::new());
        id
    }

    pub fn add_transition(&mut self, from: usize, sym: u32, to: usize) {
        self.transitions[from].push((sym, to));
    }

    pub fn add_epsilon(&mut self, from: usize, to: usize) {
        self.epsilon[from].push(to);
    }
}

/// Generic DFA: states with labeled transitions, plus the NFA state sets
/// from which each DFA state was constructed.
pub(crate) struct Dfa {
    pub num_states: usize,
    pub transitions: Vec<Vec<(u32, usize)>>,
    /// For each DFA state: the set of NFA states it contains.
    pub nfa_sets: Vec<Vec<usize>>,
}

fn epsilon_closure(nfa: &Nfa, states: &BTreeSet<usize>) -> BTreeSet<usize> {
    let mut result = states.clone();
    let mut worklist: Vec<usize> = states.iter().copied().collect();

    while let Some(s) = worklist.pop() {
        for &target in &nfa.epsilon[s] {
            if result.insert(target) {
                worklist.push(target);
            }
        }
    }

    result
}

/// Subset construction: NFA → DFA.
pub(crate) fn subset_construction(nfa: &Nfa) -> Dfa {
    let initial: BTreeSet<usize> = [0].into_iter().collect();
    let initial_closed = epsilon_closure(nfa, &initial);

    let mut dfa_nfa_sets: Vec<BTreeSet<usize>> = vec![initial_closed.clone()];
    let mut state_index: HashMap<BTreeSet<usize>, usize> = HashMap::new();
    state_index.insert(initial_closed, 0);

    let mut transitions: Vec<Vec<(u32, usize)>> = vec![Vec::new()];
    let mut worklist = vec![0usize];

    while let Some(dfa_idx) = worklist.pop() {
        let dfa_state = dfa_nfa_sets[dfa_idx].clone();

        let mut symbol_targets: BTreeMap<u32, BTreeSet<usize>> = BTreeMap::new();
        for &nfa_state in &dfa_state {
            for &(sym, target) in &nfa.transitions[nfa_state] {
                symbol_targets.entry(sym).or_default().insert(target);
            }
        }

        for (sym, targets) in symbol_targets {
            let closed = epsilon_closure(nfa, &targets);
            if closed.is_empty() {
                continue;
            }

            let target_idx = if let Some(&idx) = state_index.get(&closed) {
                idx
            } else {
                let idx = dfa_nfa_sets.len();
                state_index.insert(closed.clone(), idx);
                dfa_nfa_sets.push(closed);
                transitions.push(Vec::new());
                worklist.push(idx);
                idx
            };

            transitions[dfa_idx].push((sym, target_idx));
        }
    }

    let num_states = dfa_nfa_sets.len();
    let nfa_sets: Vec<Vec<usize>> = dfa_nfa_sets
        .into_iter()
        .map(|s| s.into_iter().collect())
        .collect();

    Dfa { num_states, transitions, nfa_sets }
}

/// Hopcroft DFA minimization.
///
/// `initial_partition[state]` assigns each state to a partition ID.
/// Returns `(minimized_dfa, state_map)` where `state_map[old_state] = new_state`.
pub(crate) fn hopcroft_minimize(dfa: &Dfa, initial_partition: &[usize]) -> (Dfa, Vec<usize>) {
    // Build partitions from the assignment
    let mut partition_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (state, &p) in initial_partition.iter().enumerate() {
        partition_map.entry(p).or_default().push(state);
    }

    let mut partition_ids: Vec<usize> = partition_map.keys().copied().collect();
    partition_ids.sort();

    let mut partitions: Vec<Vec<usize>> = partition_ids
        .iter()
        .map(|id| partition_map.remove(id).unwrap())
        .collect();

    let mut state_to_partition: Vec<usize> = vec![0; dfa.num_states];
    for (p_idx, partition) in partitions.iter().enumerate() {
        for &s in partition {
            state_to_partition[s] = p_idx;
        }
    }

    // Refinement loop
    let mut changed = true;
    while changed {
        changed = false;

        let num_partitions = partitions.len();
        let mut new_partitions: Vec<Vec<usize>> = Vec::new();
        let mut new_state_to_partition: Vec<usize> = vec![0; dfa.num_states];

        for partition in &partitions {
            if partition.len() <= 1 {
                let p = new_partitions.len();
                for &s in partition {
                    new_state_to_partition[s] = p;
                }
                new_partitions.push(partition.clone());
                continue;
            }

            let mut signature_groups: BTreeMap<Vec<(u32, usize)>, Vec<usize>> = BTreeMap::new();

            for &state in partition {
                let mut sig: Vec<(u32, usize)> = dfa.transitions[state]
                    .iter()
                    .map(|&(sym, target)| (sym, state_to_partition[target]))
                    .collect();
                sig.sort();
                signature_groups.entry(sig).or_default().push(state);
            }

            if signature_groups.len() > 1 {
                changed = true;
            }

            for (_, group) in signature_groups {
                let p = new_partitions.len();
                for &s in &group {
                    new_state_to_partition[s] = p;
                }
                new_partitions.push(group);
            }
        }

        partitions = new_partitions;
        state_to_partition = new_state_to_partition;

        if partitions.len() == num_partitions && !changed {
            break;
        }
    }

    // Build minimized DFA
    let num_min_states = partitions.len();
    let mut min_transitions: Vec<Vec<(u32, usize)>> = vec![Vec::new(); num_min_states];
    let mut min_nfa_sets: Vec<Vec<usize>> = vec![Vec::new(); num_min_states];

    for (p_idx, partition) in partitions.iter().enumerate() {
        let representative = partition[0];
        min_nfa_sets[p_idx] = dfa.nfa_sets[representative].clone();

        for &(sym, target) in &dfa.transitions[representative] {
            min_transitions[p_idx].push((sym, state_to_partition[target]));
        }
    }

    // Ensure state 0 is the initial state
    let initial_partition_id = state_to_partition[0];
    if initial_partition_id != 0 {
        min_transitions.swap(0, initial_partition_id);
        min_nfa_sets.swap(0, initial_partition_id);

        for row in &mut min_transitions {
            for (_, target) in row.iter_mut() {
                if *target == 0 {
                    *target = initial_partition_id;
                } else if *target == initial_partition_id {
                    *target = 0;
                }
            }
        }
    }

    // Build state map (accounting for swap)
    let mut state_map = state_to_partition;
    if initial_partition_id != 0 {
        for s in state_map.iter_mut() {
            if *s == 0 {
                *s = initial_partition_id;
            } else if *s == initial_partition_id {
                *s = 0;
            }
        }
    }

    (
        Dfa {
            num_states: num_min_states,
            transitions: min_transitions,
            nfa_sets: min_nfa_sets,
        },
        state_map,
    )
}

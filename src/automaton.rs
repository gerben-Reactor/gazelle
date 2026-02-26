use std::collections::{BTreeMap, BTreeSet, HashMap};

/// Generic NFA: states with labeled transitions and epsilon edges.
pub struct Nfa {
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

    pub fn num_states(&self) -> usize {
        self.transitions.len()
    }

    pub fn transitions(&self) -> &[Vec<(u32, usize)>] {
        &self.transitions
    }

    pub fn epsilons(&self) -> &[Vec<usize>] {
        &self.epsilon
    }
}

impl Default for Nfa {
    fn default() -> Self {
        Self::new()
    }
}

/// Generic DFA: states with labeled transitions.
pub struct Dfa {
    pub transitions: Vec<Vec<(u32, usize)>>,
}

impl Dfa {
    pub fn num_states(&self) -> usize {
        self.transitions.len()
    }
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
/// Returns the DFA and the NFA state sets for each DFA state.
pub fn subset_construction(nfa: &Nfa) -> (Dfa, Vec<Vec<usize>>) {
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

    let nfa_sets: Vec<Vec<usize>> = dfa_nfa_sets
        .into_iter()
        .map(|s| s.into_iter().collect())
        .collect();

    (Dfa { transitions }, nfa_sets)
}

/// Hopcroft DFA minimization.
///
/// `initial_partition[state]` assigns each state to a partition ID.
/// Returns `(minimized_dfa, state_map)` where `state_map[old_state] = new_state`.
pub fn hopcroft_minimize(dfa: &Dfa, initial_partition: &[usize]) -> (Dfa, Vec<usize>) {
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

    let mut state_to_partition: Vec<usize> = vec![0; dfa.num_states()];
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
        let mut new_state_to_partition: Vec<usize> = vec![0; dfa.num_states()];

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

    for (p_idx, partition) in partitions.iter().enumerate() {
        let representative = partition[0];
        for &(sym, target) in &dfa.transitions[representative] {
            min_transitions[p_idx].push((sym, state_to_partition[target]));
        }
    }

    // Ensure state 0 is the initial state
    let initial_partition_id = state_to_partition[0];
    if initial_partition_id != 0 {
        min_transitions.swap(0, initial_partition_id);
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
            transitions: min_transitions,
        },
        state_map,
    )
}

/// Compute symbol equivalence classes from DFA transition table.
/// Returns `(class_map, num_classes)` where `class_map[symbol] = class_id`.
/// Two symbols get the same class iff they have identical transitions in every state.
pub fn symbol_classes(dfa: &Dfa, num_symbols: usize) -> (Vec<u16>, usize) {
    // Build column signature: for each symbol, its transition target in every state.
    let mut col_map: HashMap<Vec<usize>, u16> = HashMap::new();
    let mut class_map = vec![0u16; num_symbols];
    let mut next_class = 0u16;

    // Pre-build a dense table: state × symbol → target (0 = no transition / dead)
    let mut table = vec![0usize; dfa.num_states() * num_symbols];
    for (state, trans) in dfa.transitions.iter().enumerate() {
        for &(sym, target) in trans {
            // target + 1 so that 0 means "no transition"
            table[state * num_symbols + sym as usize] = target + 1;
        }
    }

    for sym in 0..num_symbols {
        let col: Vec<usize> = (0..dfa.num_states())
            .map(|s| table[s * num_symbols + sym])
            .collect();

        let class = col_map.entry(col).or_insert_with(|| {
            let c = next_class;
            next_class += 1;
            c
        });
        class_map[sym] = *class;
    }

    (class_map, next_class as usize)
}

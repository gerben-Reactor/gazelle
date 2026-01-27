//! Debug script to understand the LR automaton bug.
//!
//! The issue: after shifting NAME, only TYPE is valid, not VARIABLE.
//! This test builds a minimal grammar with the same structure.

use gazelle::{GrammarBuilder, Automaton, Symbol};

fn main() {
    // Minimal grammar that reproduces the issue:
    //
    // typedef_name = NAME TYPE;
    // var_name = NAME VARIABLE;
    //
    // type_specifier = INT | typedef_name;
    //
    // declaration_specifiers = declaration_specifier+;
    // declaration_specifier = type_specifier;
    //
    // direct_declarator = var_name | LPAREN RPAREN;
    // declarator = direct_declarator;
    // init_declarator = declarator;
    // init_declarator_list = init_declarator;
    //
    // declaration = declaration_specifiers init_declarator_list? SEMI;
    // translation_unit = declaration+;

    let mut gb = GrammarBuilder::new();

    // Terminals
    let name = gb.t("NAME");
    let type_tok = gb.t("TYPE");
    let variable = gb.t("VARIABLE");
    let int = gb.t("INT");
    let semi = gb.t("SEMI");
    let lparen = gb.t("LPAREN");
    let rparen = gb.t("RPAREN");

    // Non-terminals
    let typedef_name = gb.nt("typedef_name");
    let var_name = gb.nt("var_name");
    let type_specifier = gb.nt("type_specifier");
    let declaration_specifier = gb.nt("declaration_specifier");
    let declaration_specifiers = gb.nt("declaration_specifiers");
    let direct_declarator = gb.nt("direct_declarator");
    let declarator = gb.nt("declarator");
    let init_declarator = gb.nt("init_declarator");
    let init_declarator_list = gb.nt("init_declarator_list");
    let init_declarator_list_opt = gb.nt("init_declarator_list_opt");
    let declaration = gb.nt("declaration");
    let translation_unit = gb.nt("translation_unit");
    let declaration_specifiers_aux = gb.nt("__decl_spec_plus");
    let translation_unit_aux = gb.nt("__translation_unit_plus");

    // Rules for typedef_name and var_name
    gb.rule(typedef_name, vec![name, type_tok]);  // typedef_name = NAME TYPE
    gb.rule(var_name, vec![name, variable]);      // var_name = NAME VARIABLE

    // type_specifier = INT | typedef_name
    gb.rule(type_specifier, vec![int]);
    gb.rule(type_specifier, vec![typedef_name]);

    // declaration_specifier = type_specifier
    gb.rule(declaration_specifier, vec![type_specifier]);

    // declaration_specifiers = declaration_specifier+ (desugared with RIGHT recursion)
    // Try RIGHT recursion: __decl_spec_plus = declaration_specifier __decl_spec_plus | declaration_specifier
    // This should allow both typedef_name and var_name in the same NAME state
    gb.rule(declaration_specifiers_aux, vec![declaration_specifier, declaration_specifiers_aux]);
    gb.rule(declaration_specifiers_aux, vec![declaration_specifier]);
    gb.rule(declaration_specifiers, vec![declaration_specifiers_aux]);

    // direct_declarator = var_name | LPAREN RPAREN
    gb.rule(direct_declarator, vec![var_name]);
    gb.rule(direct_declarator, vec![lparen, rparen]);

    // declarator = direct_declarator
    gb.rule(declarator, vec![direct_declarator]);

    // init_declarator = declarator
    gb.rule(init_declarator, vec![declarator]);

    // init_declarator_list = init_declarator
    gb.rule(init_declarator_list, vec![init_declarator]);

    // init_declarator_list? (desugared)
    gb.rule(init_declarator_list_opt, vec![init_declarator_list]);
    gb.rule(init_declarator_list_opt, vec![]);

    // declaration = declaration_specifiers init_declarator_list? SEMI
    gb.rule(declaration, vec![declaration_specifiers, init_declarator_list_opt, semi]);

    // translation_unit = declaration+ (desugared)
    gb.rule(translation_unit_aux, vec![translation_unit_aux, declaration]);
    gb.rule(translation_unit_aux, vec![declaration]);
    gb.rule(translation_unit, vec![translation_unit_aux]);

    // Set start symbol
    gb.start(translation_unit);

    let grammar = gb.build();

    println!("=== Grammar ===");
    println!("Terminals: {} + EOF", grammar.symbols.num_terminals());
    println!("Non-terminals: {}", grammar.symbols.num_non_terminals());
    println!();

    // Print all rules
    println!("=== Rules ===");
    for (i, rule) in grammar.rules.iter().enumerate() {
        let lhs_name = grammar.symbols.name(rule.lhs.id());
        let rhs_names: Vec<_> = rule.rhs.iter()
            .map(|s| grammar.symbols.name(s.id()))
            .collect();
        println!("{}: {} -> {}", i, lhs_name, rhs_names.join(" "));
    }
    println!();

    // Build automaton
    let automaton = Automaton::build(&grammar);

    println!("=== Automaton ===");
    println!("States: {}", automaton.num_states());
    println!();

    // Find the NAME and INT symbols in the augmented grammar
    let name_sym = automaton.grammar.symbols.get("NAME").unwrap();
    let int_sym = automaton.grammar.symbols.get("INT").unwrap();
    let type_sym = automaton.grammar.symbols.get("TYPE").unwrap();
    let var_sym = automaton.grammar.symbols.get("VARIABLE").unwrap();

    // Print items in state 0
    println!("=== Items in state 0 ===");
    for item in automaton.states[0].iter() {
        let rule = &automaton.grammar.rules[item.rule];
        let lhs_name = automaton.grammar.symbols.name(rule.lhs.id());
        let rhs_names: Vec<_> = rule.rhs.iter()
            .enumerate()
            .map(|(i, s)| {
                let name = automaton.grammar.symbols.name(s.id());
                if i == item.dot { format!("• {}", name) } else { name.to_string() }
            })
            .collect();
        let rhs_str = if item.dot == rule.rhs.len() {
            format!("{} •", rhs_names.join(" "))
        } else if rhs_names.is_empty() {
            "•".to_string()
        } else {
            rhs_names.join(" ")
        };
        let la_name = automaton.grammar.symbols.name(item.lookahead);
        println!("  [{}: {} -> {}, {}]", item.rule, lhs_name, rhs_str, la_name);
    }
    println!();

    // Find state reached after INT from state 0
    let state_after_int = automaton.transition(0, int_sym);
    println!("State 0 --INT--> {:?}", state_after_int);

    if let Some(state_int) = state_after_int {
        // Print all items in state_int
        println!();
        println!("=== Items in state {} (after INT) ===", state_int);
        for item in automaton.states[state_int].iter() {
            let rule = &automaton.grammar.rules[item.rule];
            let lhs_name = automaton.grammar.symbols.name(rule.lhs.id());
            let rhs_names: Vec<_> = rule.rhs.iter()
                .enumerate()
                .map(|(i, s)| {
                    let name = automaton.grammar.symbols.name(s.id());
                    if i == item.dot { format!("• {}", name) } else { name.to_string() }
                })
                .collect();
            let rhs_str = if item.dot == rule.rhs.len() {
                format!("{} •", rhs_names.join(" "))
            } else {
                rhs_names.join(" ")
            };
            let la_name = automaton.grammar.symbols.name(item.lookahead);
            println!("  [{}: {} -> {}, {}]", item.rule, lhs_name, rhs_str, la_name);
        }
        // Find state reached after NAME from that state
        let state_after_name = automaton.transition(state_int, name_sym);
        println!("State {} --NAME--> {:?}", state_int, state_after_name);

        if let Some(state_name) = state_after_name {
            // Check transitions for TYPE and VARIABLE
            let state_after_type = automaton.transition(state_name, type_sym);
            let state_after_var = automaton.transition(state_name, var_sym);
            println!("State {} --TYPE--> {:?}", state_name, state_after_type);
            println!("State {} --VARIABLE--> {:?}", state_name, state_after_var);

            // Print all items in the state after NAME
            println!();
            println!("=== Items in state {} (after NAME) ===", state_name);
            for item in automaton.states[state_name].iter() {
                let rule = &automaton.grammar.rules[item.rule];
                let lhs_name = automaton.grammar.symbols.name(rule.lhs.id());
                let rhs_names: Vec<_> = rule.rhs.iter()
                    .enumerate()
                    .map(|(i, s)| {
                        let name = automaton.grammar.symbols.name(s.id());
                        if i == item.dot { format!("• {}", name) } else { name.to_string() }
                    })
                    .collect();
                let rhs_str = if item.dot == rule.rhs.len() {
                    format!("{} •", rhs_names.join(" "))
                } else {
                    rhs_names.join(" ")
                };
                let la_name = automaton.grammar.symbols.name(item.lookahead);
                println!("  [{}: {} -> {}, {}]", item.rule, lhs_name, rhs_str, la_name);
            }
        }
    }

    // Also check state 0 for NAME transitions
    println!();
    println!("=== State 0 --NAME--> ? ===");
    let state0_name = automaton.transition(0, name_sym);
    println!("State 0 --NAME--> {:?}", state0_name);

    if let Some(s) = state0_name {
        println!();
        println!("=== Items in state {} (NAME from state 0) ===", s);
        for item in automaton.states[s].iter() {
            let rule = &automaton.grammar.rules[item.rule];
            let lhs_name = automaton.grammar.symbols.name(rule.lhs.id());
            let rhs_names: Vec<_> = rule.rhs.iter()
                .enumerate()
                .map(|(i, sym)| {
                    let name = automaton.grammar.symbols.name(sym.id());
                    if i == item.dot { format!("• {}", name) } else { name.to_string() }
                })
                .collect();
            let rhs_str = if item.dot == rule.rhs.len() {
                format!("{} •", rhs_names.join(" "))
            } else {
                rhs_names.join(" ")
            };
            let la_name = automaton.grammar.symbols.name(item.lookahead);
            println!("  [{}: {} -> {}, {}]", item.rule, lhs_name, rhs_str, la_name);
        }

        // Check TYPE and VARIABLE transitions
        let type_trans = automaton.transition(s, type_sym);
        let var_trans = automaton.transition(s, var_sym);
        println!();
        println!("State {} --TYPE--> {:?}", s, type_trans);
        println!("State {} --VARIABLE--> {:?}", s, var_trans);
    }

    // Let's also look at what states have items that expect VARIABLE
    println!();
    println!("=== States with items expecting VARIABLE ===");
    for (state_idx, state) in automaton.states.iter().enumerate() {
        for item in state.iter() {
            if let Some(next) = item.next_symbol(&automaton.grammar) {
                if next == var_sym {
                    let rule = &automaton.grammar.rules[item.rule];
                    let lhs_name = automaton.grammar.symbols.name(rule.lhs.id());
                    println!("State {}: {} rule (item {:?})", state_idx, lhs_name, item);
                }
            }
        }
    }

    // Print augmented grammar rules first to see numbering
    println!();
    println!("=== Augmented Grammar Rules ===");
    for (i, rule) in automaton.grammar.rules.iter().enumerate() {
        let lhs_name = automaton.grammar.symbols.name(rule.lhs.id());
        let rhs_names: Vec<_> = rule.rhs.iter()
            .map(|s| automaton.grammar.symbols.name(s.id()))
            .collect();
        println!("{}: {} -> {}", i, lhs_name, rhs_names.join(" "));
    }

    // Let's trace through what states have var_name -> • NAME VARIABLE
    println!();
    println!("=== States with var_name -> • NAME VARIABLE ===");
    // Find the rule index for var_name in augmented grammar
    let var_name_rule_idx = automaton.grammar.rules.iter()
        .position(|r| {
            let lhs = automaton.grammar.symbols.name(r.lhs.id());
            lhs == "var_name"
        })
        .expect("var_name rule not found");
    println!("var_name rule index in augmented grammar: {}", var_name_rule_idx);

    for (state_idx, state) in automaton.states.iter().enumerate() {
        for item in state.iter() {
            if item.rule == var_name_rule_idx && item.dot == 0 {
                let la_name = automaton.grammar.symbols.name(item.lookahead);
                println!("State {}: var_name -> • NAME VARIABLE, {}", state_idx, la_name);
            }
        }
    }

    // What about direct_declarator -> • var_name?
    println!();
    println!("=== States with direct_declarator -> • var_name ===");
    for (state_idx, state) in automaton.states.iter().enumerate() {
        for item in state.iter() {
            // Rule 8 is direct_declarator -> var_name
            if item.rule == 8 && item.dot == 0 {
                let la_name = automaton.grammar.symbols.name(item.lookahead);
                println!("State {}: direct_declarator -> • var_name, {}", state_idx, la_name);
            }
        }
    }

    // What about init_declarator_list_opt items?
    println!();
    println!("=== States with init_declarator_list_opt items ===");
    for (state_idx, state) in automaton.states.iter().enumerate() {
        for item in state.iter() {
            let rule = &automaton.grammar.rules[item.rule];
            let lhs_name = automaton.grammar.symbols.name(rule.lhs.id());
            if lhs_name == "init_declarator_list_opt" {
                let rhs_names: Vec<_> = rule.rhs.iter()
                    .map(|s| automaton.grammar.symbols.name(s.id()))
                    .collect();
                let la_name = automaton.grammar.symbols.name(item.lookahead);
                println!("State {}: init_declarator_list_opt -> {:?}, dot={}, la={}",
                    state_idx, rhs_names, item.dot, la_name);
            }
        }
    }

    // Let's manually compute GOTO(state 0, NAME) and see what we get
    println!();
    println!("=== Manual GOTO(state 0, NAME) computation ===");
    {
        use gazelle::goto;
        let state0 = &automaton.states[0];
        let goto_result = goto(&automaton.grammar, state0, name_sym, &automaton.first_sets);
        println!("GOTO(state 0, NAME) has {} items:", goto_result.len());
        for item in goto_result.iter() {
            let rule = &automaton.grammar.rules[item.rule];
            let lhs_name = automaton.grammar.symbols.name(rule.lhs.id());
            let rhs_names: Vec<_> = rule.rhs.iter()
                .enumerate()
                .map(|(i, s)| {
                    let name = automaton.grammar.symbols.name(s.id());
                    if i == item.dot { format!("• {}", name) } else { name.to_string() }
                })
                .collect();
            let rhs_str = if item.dot == rule.rhs.len() {
                format!("{} •", rhs_names.join(" "))
            } else {
                rhs_names.join(" ")
            };
            let la_name = automaton.grammar.symbols.name(item.lookahead);
            println!("  [{}: {} -> {}, {}]", item.rule, lhs_name, rhs_str, la_name);
        }
    }

    // Also check: what items in state 0 have NAME as next symbol?
    println!();
    println!("=== Items in state 0 with NAME as next symbol ===");
    for item in automaton.states[0].iter() {
        if let Some(next) = item.next_symbol(&automaton.grammar) {
            if next == name_sym {
                let rule = &automaton.grammar.rules[item.rule];
                let lhs_name = automaton.grammar.symbols.name(rule.lhs.id());
                let rhs_names: Vec<_> = rule.rhs.iter()
                    .map(|s| automaton.grammar.symbols.name(s.id()))
                    .collect();
                let la_name = automaton.grammar.symbols.name(item.lookahead);
                println!("  [{}: {} -> {:?}, dot={}, la={}]", item.rule, lhs_name, rhs_names, item.dot, la_name);
            }
        }
    }

    // Check if var_name is being recognized as a non-terminal
    let var_name_nt = automaton.grammar.symbols.get("var_name").unwrap();
    println!();
    println!("=== var_name symbol info ===");
    println!("var_name symbol: {:?}", var_name_nt);
    println!("is_non_terminal: {}", var_name_nt.is_non_terminal());
    println!("var_name symbol ID: {:?}", var_name_nt.id());

    // Check rules_for var_name
    println!();
    println!("=== Rules for var_name ===");
    for (idx, rule) in automaton.grammar.rules_for(var_name_nt) {
        let rhs_names: Vec<_> = rule.rhs.iter()
            .map(|s| automaton.grammar.symbols.name(s.id()))
            .collect();
        println!("  Rule {}: var_name -> {}", idx, rhs_names.join(" "));
    }

    // Check which items in state 0 have var_name as next symbol
    println!();
    println!("=== Items in state 0 with var_name as next symbol ===");
    for item in automaton.states[0].iter() {
        if let Some(next) = item.next_symbol(&automaton.grammar) {
            if next == var_name_nt {
                let rule = &automaton.grammar.rules[item.rule];
                let lhs_name = automaton.grammar.symbols.name(rule.lhs.id());
                let la_name = automaton.grammar.symbols.name(item.lookahead);
                println!("  Item: {} -> ... • var_name ..., la={}", lhs_name, la_name);
            }
        }
    }

    // Print items in states 1 and 3
    println!();
    println!("=== Items in state 1 (after __decl_spec_plus from state 0) ===");
    for item in automaton.states[1].iter() {
        let rule = &automaton.grammar.rules[item.rule];
        let lhs_name = automaton.grammar.symbols.name(rule.lhs.id());
        let rhs_names: Vec<_> = rule.rhs.iter()
            .enumerate()
            .map(|(i, s)| {
                let name = automaton.grammar.symbols.name(s.id());
                if i == item.dot { format!("• {}", name) } else { name.to_string() }
            })
            .collect();
        let rhs_str = if item.dot == rule.rhs.len() {
            format!("{} •", rhs_names.join(" "))
        } else if rhs_names.is_empty() {
            "•".to_string()
        } else {
            rhs_names.join(" ")
        };
        let la_name = automaton.grammar.symbols.name(item.lookahead);
        println!("  [{}: {} -> {}, {}]", item.rule, lhs_name, rhs_str, la_name);
    }

    println!();
    println!("=== Transitions from state 1 ===");
    // Iterate through all transitions from state 1
    for ((from_state, sym), to_state) in automaton.transitions.iter() {
        if *from_state == 1 {
            let name = automaton.grammar.symbols.name(sym.id());
            println!("  State 1 --{}--> State {}", name, to_state);
        }
    }

    // Get symbol IDs for parsing
    let name_id = automaton.grammar.symbols.get_id("NAME").unwrap();
    let type_id = automaton.grammar.symbols.get_id("TYPE").unwrap();
    let var_id = automaton.grammar.symbols.get_id("VARIABLE").unwrap();
    let int_id = automaton.grammar.symbols.get_id("INT").unwrap();
    let semi_id = automaton.grammar.symbols.get_id("SEMI").unwrap();

    // Now let's trace the actual parse of "INT NAME VARIABLE SEMI"
    // by following the automaton transitions
    println!();
    println!("=== Simulating parse of INT NAME VARIABLE SEMI ===");
    simulate_parse(&automaton, &[int_id, name_id, var_id, semi_id], &["INT", "NAME", "VARIABLE", "SEMI"]);

    // Also test "INT INT NAME VARIABLE SEMI" (two specifiers)
    println!();
    println!("=== Simulating parse of INT INT NAME VARIABLE SEMI ===");
    simulate_parse(&automaton, &[int_id, int_id, name_id, var_id, semi_id], &["INT", "INT", "NAME", "VARIABLE", "SEMI"]);

    // Also test "INT NAME TYPE SEMI" (typedef name in specifiers, then semicolon - like "int T;")
    println!();
    println!("=== Simulating parse of INT NAME TYPE SEMI ===");
    simulate_parse(&automaton, &[int_id, name_id, type_id, semi_id], &["INT", "NAME", "TYPE", "SEMI"]);
}

fn simulate_parse(automaton: &Automaton, input: &[gazelle::SymbolId], input_names: &[&str]) {
    use gazelle::{CompiledTable, Action};

    let compiled = CompiledTable::build(automaton);
    let table = compiled.table();
    let mut state_stack = vec![0usize];
    let mut input_pos = 0;

    while input_pos <= input.len() {
        let current_state = *state_stack.last().unwrap();
        let lookahead = if input_pos < input.len() { input[input_pos] } else { gazelle::SymbolId::EOF };
        let lookahead_name = if input_pos < input.len() { input_names[input_pos] } else { "$" };

        println!("State: {:?}, Lookahead: {}", state_stack, lookahead_name);

        let action = table.action(current_state, lookahead);

        match action {
            Action::Shift(next_state) => {
                state_stack.push(next_state);
                input_pos += 1;
                println!("  -> Shift to state {}", next_state);
            }
            Action::Reduce(rule_idx) => {
                let rule = &automaton.grammar.rules[rule_idx];
                let lhs_name = automaton.grammar.symbols.name(rule.lhs.id());
                let pop_count = rule.rhs.len();
                for _ in 0..pop_count {
                    state_stack.pop();
                }
                let goto_state = *state_stack.last().unwrap();
                if let Some(next) = table.goto(goto_state, rule.lhs.id()) {
                    state_stack.push(next);
                    println!("  -> Reduce {}, GOTO({}, {}) = {}", lhs_name, goto_state, lhs_name, next);
                } else {
                    println!("  -> Reduce {}, NO GOTO!", lhs_name);
                    break;
                }
            }
            Action::Accept => {
                println!("  -> ACCEPTED!");
                break;
            }
            Action::Error => {
                println!("  -> ERROR!");
                println!("  Available actions in state {}:", current_state);
                for t in 0..=automaton.grammar.symbols.num_terminals() {
                    let tid = gazelle::SymbolId(t);
                    let act = table.action(current_state, tid);
                    if !matches!(act, Action::Error) {
                        let name = automaton.grammar.symbols.name(tid);
                        println!("    {} -> {:?}", name, act);
                    }
                }
                break;
            }
            Action::ShiftOrReduce { .. } => {
                println!("  -> ShiftOrReduce - not handling precedence");
                break;
            }
        }
    }
}

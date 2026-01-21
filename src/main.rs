use gazelle::{parse_grammar, Automaton, ParseTable, Parser, Token, Event, t};

fn main() {
    // Parse a grammar from a string
    let grammar = parse_grammar(r#"
        expr = expr '+' term | expr '-' term | term ;
        term = term '*' factor | term '/' factor | factor ;
        factor = 'NUM' | '(' expr ')' ;
    "#).expect("Failed to parse grammar");

    println!("Parsed grammar with {} rules:", grammar.rules.len());
    for (i, rule) in grammar.rules.iter().enumerate() {
        let rhs: Vec<_> = rule.rhs.iter().map(|s| s.name()).collect();
        println!("  {}: {} -> {}", i, rule.lhs.name(), rhs.join(" "));
    }

    // Build parser
    let automaton = Automaton::build(&grammar);
    let table = ParseTable::build(&automaton);

    println!("\nAutomaton has {} states", automaton.num_states());

    if table.has_conflicts() {
        println!("Conflicts: {:?}", table.conflicts);
    } else {
        println!("No conflicts - grammar is unambiguous");
    }

    // Parse "1 + 2 * 3"
    println!("\nParsing: NUM + NUM * NUM");

    let mut parser = Parser::new(&table);
    let tokens = vec![
        Token::new(t("NUM"), "1"),
        Token::new(t("+"), "+"),
        Token::new(t("NUM"), "2"),
        Token::new(t("*"), "*"),
        Token::new(t("NUM"), "3"),
    ];

    for token in &tokens {
        println!("  push {:?}", token.terminal.name());
        for event in parser.push(token) {
            print_event(&event, &table);
        }
    }

    println!("  finish");
    for event in parser.finish() {
        print_event(&event, &table);
    }
}

fn print_event(event: &Event, table: &ParseTable) {
    match event {
        Event::Reduce { rule, len } => {
            let r = &table.grammar.rules[*rule];
            let rhs: Vec<_> = r.rhs.iter().map(|s| s.name()).collect();
            println!("    reduce: {} -> {} (pop {})", r.lhs.name(), rhs.join(" "), len);
        }
        Event::Accept => {
            println!("    accept!");
        }
        Event::Error { token, state } => {
            println!("    error: unexpected {:?} in state {}", token, state);
        }
    }
}

use gazelle::{parse_grammar, Automaton, ParseTable, Parser, Token, Event};

fn main() {
    // Parse a grammar from a string
    let grammar = parse_grammar(r#"
        expr = expr '+' term | expr '-' term | term ;
        term = term '*' factor | term '/' factor | factor ;
        factor = 'NUM' | '(' expr ')' ;
    "#).expect("Failed to parse grammar");

    println!("Parsed grammar with {} rules:", grammar.rules.len());
    for (i, rule) in grammar.rules.iter().enumerate() {
        let lhs_name = grammar.symbols.name(rule.lhs.id());
        let rhs: Vec<_> = rule.rhs.iter().map(|s| grammar.symbols.name(s.id())).collect();
        println!("  {}: {} -> {}", i, lhs_name, rhs.join(" "));
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

    let num_id = table.symbol_id("NUM").expect("NUM not found");
    let plus_id = table.symbol_id("+").expect("+ not found");
    let star_id = table.symbol_id("*").expect("* not found");

    let tokens = vec![
        Token::new(num_id, "1"),
        Token::new(plus_id, "+"),
        Token::new(num_id, "2"),
        Token::new(star_id, "*"),
        Token::new(num_id, "3"),
    ];

    for token in &tokens {
        println!("  push {:?}", table.grammar.symbols.name(token.terminal));
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
            let lhs_name = table.grammar.symbols.name(r.lhs.id());
            let rhs: Vec<_> = r.rhs.iter().map(|s| table.grammar.symbols.name(s.id())).collect();
            println!("    reduce: {} -> {} (pop {})", lhs_name, rhs.join(" "), len);
        }
        Event::Accept => {
            println!("    accept!");
        }
        Event::Error { terminal, state } => {
            println!("    error: unexpected {:?} in state {}", terminal, state);
        }
    }
}

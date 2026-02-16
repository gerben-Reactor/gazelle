//! Compare LALR(1) vs LR(1) state counts.

use gazelle::{CompiledTable, Grammar};

fn main() {
    // Test with the expression grammar
    println!("=== Expression Grammar ===");
    compare(&expr_grammar());

    // Test with a grammar that has LALR/LR(1) differences
    println!("\n=== Grammar with potential spurious conflict ===");
    compare(&spurious_conflict_grammar());

    // Test with a larger grammar (meta grammar)
    println!("\n=== Meta Grammar ===");
    let input = std::fs::read_to_string("grammars/meta.gzl").expect("failed to read grammars/meta.gzl");
    let grammar = gazelle::parse_grammar(&input).expect("failed to parse meta.gzl");
    compare(&grammar);

    // Test with C11 grammar
    println!("\n=== C11 Grammar ===");
    let input = std::fs::read_to_string("grammars/c11.gzl").expect("failed to read grammars/c11.gzl");
    let grammar = gazelle::parse_grammar(&input).expect("failed to parse c11.gzl");
    compare(&grammar);
}

fn compare(grammar: &Grammar) {
    // Build with LALR mode
    let mut lalr_grammar = grammar.clone();
    lalr_grammar.mode = "lalr".to_string();
    let lalr = CompiledTable::build(&lalr_grammar);

    // Build with LR mode
    let mut lr_grammar = grammar.clone();
    lr_grammar.mode = "lr".to_string();
    let lr1 = CompiledTable::build(&lr_grammar);

    println!("  LALR(1): {} states, {} conflicts", lalr.num_states, lalr.conflicts.len());
    println!("  LR(1):   {} states, {} conflicts", lr1.num_states, lr1.conflicts.len());
    println!("  Ratio:   {:.2}x", lr1.num_states as f64 / lalr.num_states as f64);
}

fn expr_grammar() -> Grammar {
    gazelle::parse_grammar(r#"
        start expr;
        terminals { PLUS, TIMES, NUM, LPAREN, RPAREN }
        expr = expr PLUS term => add | term => term;
        term = term TIMES factor => mul | factor => factor;
        factor = NUM => num | LPAREN expr RPAREN => paren;
    "#).expect("expr grammar")
}

fn spurious_conflict_grammar() -> Grammar {
    // Classic example: S → aEc | aFd | bEd | bFc, E → e, F → e
    gazelle::parse_grammar(r#"
        start s;
        terminals { A, B, C, D, E_TOK }
        s = A e C => s1 | A f D => s2 | B e D => s3 | B f C => s4;
        e = E_TOK => e;
        f = E_TOK => f;
    "#).expect("spurious grammar")
}

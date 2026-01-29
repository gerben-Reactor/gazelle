//! Compare LALR(1) vs LR(1) state counts.

use gazelle::{CompiledTable, LrAlgorithm, GrammarBuilder};

fn main() {
    // Test with the expression grammar
    println!("=== Expression Grammar ===");
    let grammar = expr_grammar();
    compare(&grammar);

    // Test with a grammar that has LALR/LR(1) differences
    println!("\n=== Grammar with potential spurious conflict ===");
    let grammar = spurious_conflict_grammar();
    compare(&grammar);

    // Test with a larger grammar (meta grammar)
    println!("\n=== Meta Grammar ===");
    let input = std::fs::read_to_string("meta.gzl").expect("failed to read meta.gzl");
    let grammar = gazelle::parse_grammar(&input).expect("failed to parse meta.gzl");
    compare(&grammar);

    // Test with C11 grammar
    println!("\n=== C11 Grammar ===");
    let input = std::fs::read_to_string("c11.gzl").expect("failed to read c11.gzl");
    let grammar = gazelle::parse_grammar(&input).expect("failed to parse c11.gzl");
    compare(&grammar);
}

fn compare(grammar: &gazelle::Grammar) {
    let lalr = CompiledTable::build_with_algorithm(grammar, LrAlgorithm::Lalr1);
    let lr1 = CompiledTable::build_with_algorithm(grammar, LrAlgorithm::Lr1);

    println!("  LALR(1): {} states, {} conflicts", lalr.num_states, lalr.conflicts.len());
    println!("  LR(1):   {} states, {} conflicts", lr1.num_states, lr1.conflicts.len());
    println!("  Ratio:   {:.2}x", lr1.num_states as f64 / lalr.num_states as f64);
}

fn expr_grammar() -> gazelle::Grammar {
    let mut gb = GrammarBuilder::new();
    let plus = gb.t("+");
    let times = gb.t("*");
    let num = gb.t("NUM");
    let lparen = gb.t("(");
    let rparen = gb.t(")");
    let expr = gb.nt("expr");
    let term = gb.nt("term");
    let factor = gb.nt("factor");

    gb.rule(expr, vec![expr, plus, term]);
    gb.rule(expr, vec![term]);
    gb.rule(term, vec![term, times, factor]);
    gb.rule(term, vec![factor]);
    gb.rule(factor, vec![num]);
    gb.rule(factor, vec![lparen, expr, rparen]);

    gb.build()
}

fn spurious_conflict_grammar() -> gazelle::Grammar {
    // Classic example: S → aEc | aFd | bEd | bFc, E → e, F → e
    let mut gb = GrammarBuilder::new();
    let a = gb.t("a");
    let b = gb.t("b");
    let c = gb.t("c");
    let d = gb.t("d");
    let e = gb.t("e");
    let s = gb.nt("S");
    let ee = gb.nt("E");
    let f = gb.nt("F");

    gb.rule(s, vec![a, ee, c]);
    gb.rule(s, vec![a, f, d]);
    gb.rule(s, vec![b, ee, d]);
    gb.rule(s, vec![b, f, c]);
    gb.rule(ee, vec![e]);
    gb.rule(f, vec![e]);

    gb.build()
}

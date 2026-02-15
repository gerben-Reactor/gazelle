//! Test that the Reduce-based pattern works with the generated parser.

use gazelle::Reduce;
use gazelle_macros::gazelle;

gazelle! {
    grammar Fallible {
        start expr;
        terminals { NUM: Num }
        expr = NUM => Num;
    }
}

struct CheckActions;

impl FallibleTypes for CheckActions {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Expr = i32;
}

impl Reduce<FallibleExpr<Self>, i32, gazelle::ParseError> for CheckActions {
    fn reduce(&mut self, node: FallibleExpr<Self>) -> Result<i32, gazelle::ParseError> {
        Ok(match node {
            FallibleExpr::Num(n) => n,
        })
    }
}

#[test]
fn test_action_ok() {
    let mut parser = FallibleParser::<CheckActions>::new();
    let mut actions = CheckActions;

    parser.push(FallibleTerminal::NUM(42), &mut actions).unwrap();
    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert_eq!(result, 42);
}

// Test that Ignore blanket works: type Expr = Ignore discards the node
struct DiscardActions;

impl FallibleTypes for DiscardActions {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Expr = gazelle::Ignore;
}

#[test]
fn test_discard_blanket() {
    let mut parser = FallibleParser::<DiscardActions>::new();
    let mut actions = DiscardActions;

    parser.push(FallibleTerminal::NUM(42), &mut actions).unwrap();
    let _result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
}

// Test that ReduceNode identity blanket works: type Expr = FallibleExpr<Self>
struct CstActions;

impl FallibleTypes for CstActions {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Expr = FallibleExpr<Self>;
}

#[test]
fn test_cst_blanket() {
    let mut parser = FallibleParser::<CstActions>::new();
    let mut actions = CstActions;

    parser.push(FallibleTerminal::NUM(42), &mut actions).unwrap();
    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert!(matches!(result, FallibleExpr::Num(42)));
}

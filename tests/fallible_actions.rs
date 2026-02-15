//! Test that the Reduce-based pattern works with the generated parser.

use gazelle::Reduce;
use gazelle_macros::gazelle;

gazelle! {
    grammar Fallible {
        start expr;
        terminals { NUM: Num }
        expr = NUM @Num;
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

// Test that identity blanket works: type Expr = FallibleExpr<Self>
struct CstActions;

impl FallibleTypes for CstActions {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Expr = FallibleExpr<Self>;
}

// No Reduce impl needed — identity blanket covers it!

#[test]
fn test_identity_blanket() {
    let mut parser = FallibleParser::<CstActions>::new();
    let mut actions = CstActions;

    parser.push(FallibleTerminal::NUM(42), &mut actions).unwrap();
    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert!(matches!(result, FallibleExpr::Num(42)));
}

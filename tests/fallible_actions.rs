//! Test that the Reduce-based pattern works with the generated parser.

use gazelle_macros::gazelle;

gazelle! {
    grammar fallible {
        start expr;
        terminals { NUM: _ }
        expr = NUM => num;
    }
}

struct CheckActions;

impl gazelle::ErrorType for CheckActions {
    type Error = core::convert::Infallible;
}

impl fallible::Types for CheckActions {
    type Num = i32;
    type Expr = i32;
}

impl gazelle::Action<fallible::Expr<Self>> for CheckActions {
    fn build(&mut self, node: fallible::Expr<Self>) -> Result<i32, Self::Error> {
        Ok(match node {
            fallible::Expr::Num(n) => n,
        })
    }
}

#[test]
fn test_action_ok() {
    let mut parser = fallible::Parser::<CheckActions>::new();
    let mut actions = CheckActions;

    parser
        .push(fallible::Terminal::Num(42), &mut actions)
        .unwrap();
    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert_eq!(result, 42);
}

// Test that Ignore blanket works: type Expr = Ignore discards the node
struct DiscardActions;

impl gazelle::ErrorType for DiscardActions {
    type Error = core::convert::Infallible;
}

impl fallible::Types for DiscardActions {
    type Num = i32;
    type Expr = gazelle::Ignore;
}

#[test]
fn test_discard_blanket() {
    let mut parser = fallible::Parser::<DiscardActions>::new();
    let mut actions = DiscardActions;

    parser
        .push(fallible::Terminal::Num(42), &mut actions)
        .unwrap();
    let _result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
}

// Test that ReduceNode identity blanket works: type Expr = fallible::Expr<Self>
struct CstActions;

impl gazelle::ErrorType for CstActions {
    type Error = core::convert::Infallible;
}

impl fallible::Types for CstActions {
    type Num = i32;
    type Expr = fallible::Expr<Self>;
}

#[test]
fn test_cst_blanket() {
    let mut parser = fallible::Parser::<CstActions>::new();
    let mut actions = CstActions;

    parser
        .push(fallible::Terminal::Num(42), &mut actions)
        .unwrap();
    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert!(matches!(result, fallible::Expr::Num(42)));
}

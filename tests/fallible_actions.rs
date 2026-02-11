//! Test that action methods can return custom error types.

use gazelle_macros::gazelle;

gazelle! {
    grammar Fallible {
        start expr;
        terminals { NUM: Num }
        expr: Expr = NUM @check_num;
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum MyError {
    Parse(gazelle::ParseError),
    TooLarge,
}

impl From<gazelle::ParseError> for MyError {
    fn from(e: gazelle::ParseError) -> Self {
        MyError::Parse(e)
    }
}

struct CheckActions;

impl FallibleTypes for CheckActions {
    type Num = i32;
    type Expr = i32;
}

impl FallibleActions<MyError> for CheckActions {
    fn check_num(&mut self, n: i32) -> Result<i32, MyError> {
        if n > 100 {
            Err(MyError::TooLarge)
        } else {
            Ok(n)
        }
    }
}

#[test]
fn test_fallible_action_ok() {
    let mut parser = FallibleParser::<CheckActions, MyError>::new();
    let mut actions = CheckActions;

    parser.push(FallibleTerminal::NUM(42), &mut actions).unwrap();
    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert_eq!(result, 42);
}

#[test]
fn test_fallible_action_err() {
    let mut parser = FallibleParser::<CheckActions, MyError>::new();
    let mut actions = CheckActions;

    // Push succeeds (shift), but reduction with the action error happens during finish
    parser.push(FallibleTerminal::NUM(200), &mut actions).unwrap();
    let result = parser.finish(&mut actions);
    assert!(result.is_err());
    let (_, err) = result.unwrap_err();
    assert!(matches!(err, MyError::TooLarge));
}

// Also test that the default E=ParseError still works
struct DefaultActions;

impl FallibleTypes for DefaultActions {
    type Num = i32;
    type Expr = i32;
}

impl FallibleActions for DefaultActions {
    fn check_num(&mut self, n: i32) -> Result<i32, gazelle::ParseError> {
        Ok(n)
    }
}

#[test]
fn test_default_error_type() {
    let mut parser = FallibleParser::<DefaultActions>::new();
    let mut actions = DefaultActions;

    parser.push(FallibleTerminal::NUM(42), &mut actions).unwrap();
    let result = parser.finish(&mut actions).map_err(|(_, e)| e).unwrap();
    assert_eq!(result, 42);
}

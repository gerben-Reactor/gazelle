pub mod grammar;
pub mod lexer;

use grammar::{python, PyActions};

pub fn parse(input: &str) -> Result<(), String> {
    let mut parser = python::Parser::<PyActions>::new();
    let mut actions = PyActions;
    lexer::lex(input, &mut parser, &mut actions)?;
    parser.finish(&mut actions).map_err(|(p, e)| format!("Finish error: {}", p.format_error(&e)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_assignment() {
        parse("x = 1\n").unwrap();
    }

    #[test]
    fn test_expression_stmt() {
        parse("x + y\n").unwrap();
    }

    #[test]
    fn test_arithmetic() {
        parse("x = 1 + 2 * 3\n").unwrap();
    }

    #[test]
    fn test_function_def() {
        parse("def foo(x, y):\n    return x + y\n").unwrap();
    }

    #[test]
    fn test_if_stmt() {
        parse("if x > 0:\n    pass\n").unwrap();
    }

    #[test]
    fn test_if_else() {
        parse("if x:\n    a\nelse:\n    b\n").unwrap();
    }

    #[test]
    fn test_while() {
        parse("while True:\n    break\n").unwrap();
    }

    #[test]
    fn test_for() {
        parse("for x in items:\n    pass\n").unwrap();
    }

    #[test]
    fn test_class() {
        parse("class Foo:\n    pass\n").unwrap();
    }

    #[test]
    fn test_import() {
        parse("import os\n").unwrap();
    }

    #[test]
    fn test_from_import() {
        parse("from os.path import join\n").unwrap();
    }

    #[test]
    fn test_list_literal() {
        parse("x = [1, 2, 3]\n").unwrap();
    }

    #[test]
    fn test_dict_literal() {
        parse("x = {1: 2, 3: 4}\n").unwrap();
    }

    #[test]
    fn test_nested_blocks() {
        parse("if True:\n    if False:\n        pass\n").unwrap();
    }

    #[test]
    fn test_multiline() {
        parse("x = 1\ny = 2\nz = x + y\n").unwrap();
    }

    #[test]
    fn test_pass() {
        parse("pass\n").unwrap();
    }

    #[test]
    fn test_return() {
        parse("return\n").unwrap();
    }

    #[test]
    fn test_return_value() {
        parse("return x\n").unwrap();
    }

    #[test]
    fn test_call() {
        parse("foo(x, y)\n").unwrap();
    }

    #[test]
    fn test_method_call() {
        parse("obj.method(x)\n").unwrap();
    }

    #[test]
    fn test_subscript() {
        parse("x[0]\n").unwrap();
    }

    #[test]
    fn test_tuple() {
        parse("x = (1, 2, 3)\n").unwrap();
    }

    #[test]
    fn test_string() {
        parse("x = \"hello\"\n").unwrap();
    }

    #[test]
    fn test_comparison() {
        parse("x = a < b\n").unwrap();
    }

    #[test]
    fn test_chained_comparison() {
        parse("x = a < b < c\n").unwrap();
    }

    #[test]
    fn test_logical() {
        parse("x = a and b or c\n").unwrap();
    }

    #[test]
    fn test_not() {
        parse("x = not a\n").unwrap();
    }

    #[test]
    fn test_ternary() {
        parse("x = a if b else c\n").unwrap();
    }

    #[test]
    fn test_lambda() {
        parse("f = lambda x: x + 1\n").unwrap();
    }

    #[test]
    fn test_augmented_assign() {
        parse("x += 1\n").unwrap();
    }

    #[test]
    fn test_try_except() {
        parse("try:\n    pass\nexcept:\n    pass\n").unwrap();
    }

    #[test]
    fn test_with() {
        parse("with open(f) as fh:\n    pass\n").unwrap();
    }

    #[test]
    fn test_decorator() {
        parse("@foo\ndef bar():\n    pass\n").unwrap();
    }

    #[test]
    fn test_multiple_dedent() {
        parse("if True:\n    if True:\n        pass\nx = 1\n").unwrap();
    }

    #[test]
    fn test_implicit_line_join() {
        parse("x = (1 +\n     2)\n").unwrap();
    }

    #[test]
    fn test_list_comprehension() {
        parse("x = [i for i in range(10)]\n").unwrap();
    }

    #[test]
    fn test_global() {
        parse("global x\n").unwrap();
    }

    #[test]
    fn test_nonlocal() {
        parse("nonlocal x\n").unwrap();
    }

    #[test]
    fn test_del() {
        parse("del x\n").unwrap();
    }

    #[test]
    fn test_assert() {
        parse("assert x\n").unwrap();
    }

    #[test]
    fn test_raise() {
        parse("raise ValueError()\n").unwrap();
    }

    #[test]
    fn test_star_unpack() {
        parse("a, *b = [1, 2, 3]\n").unwrap();
    }

    #[test]
    fn test_annotation() {
        parse("x: int = 5\n").unwrap();
    }

    #[test]
    fn test_empty() {
        parse("").unwrap();
    }

    #[test]
    fn test_only_newlines() {
        parse("\n\n\n").unwrap();
    }

    #[test]
    fn test_elif() {
        parse("if a:\n    pass\nelif b:\n    pass\nelse:\n    pass\n").unwrap();
    }

    #[test]
    fn test_yield() {
        parse("yield x\n").unwrap();
    }

    #[test]
    fn test_slice() {
        parse("x[1:2]\n").unwrap();
    }

    #[test]
    fn test_unary_minus() {
        parse("x = -1\n").unwrap();
    }

    #[test]
    fn test_power() {
        parse("x = 2 ** 3\n").unwrap();
    }

    #[test]
    fn test_in_comparison() {
        parse("x = a in b\n").unwrap();
    }

    #[test]
    fn test_is_comparison() {
        parse("x = a is b\n").unwrap();
    }

    #[test]
    fn test_is_not_comparison() {
        parse("x = a is not b\n").unwrap();
    }

    #[test]
    fn test_not_in_comparison() {
        parse("x = a not in b\n").unwrap();
    }

    #[test]
    fn test_no_trailing_newline() {
        parse("x = 1").unwrap_err();
    }

    #[test]
    fn test_fstring() {
        parse("x = f\"hello {name}\"\n").unwrap();
    }

    #[test]
    fn test_multiline_function() {
        let code = "\
def fibonacci(n):
    if n <= 1:
        return n
    a = 0
    b = 1
    for i in range(2, n + 1):
        a, b = b, a + b
    return b
";
        parse(code).unwrap();
    }
}

//! Test example for ?, *, + modifiers.
//!
//! Demonstrates the convenience syntax for optional, zero-or-more, and one-or-more.

use gazelle_macros::gazelle;

gazelle! {
    grammar list {
        start items;
        terminals {
            NUM: _,
            COMMA,
            SEMI
        }

        // items: zero or more item, separated by nothing
        items = item* => items;

        // item: a number followed by an optional comma
        item = NUM COMMA => with_comma | NUM => without_comma;

        // nums: one or more numbers (for testing +)
        nums = NUM+ => nums;

        // opt_num: optional number followed by semi
        opt_num = NUM? SEMI => opt;

        // semis: zero or more semicolons (untyped terminal with *)
        semis = SEMI* => semis;
    }
}

#[allow(dead_code)] // Only used in tests
struct Builder;

impl list::Types for Builder {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Items = Vec<i32>;
    type Item = i32;
    type Nums = Vec<i32>;
    type OptNum = Option<i32>;
    type Semis = usize; // count of semicolons
}

impl gazelle::Action<list::Items<Self>> for Builder {
    fn build(&mut self, node: list::Items<Self>) -> Result<Vec<i32>, gazelle::ParseError> {
        let list::Items::Items(items) = node;
        Ok(items)
    }
}

impl gazelle::Action<list::Semis<Self>> for Builder {
    fn build(&mut self, node: list::Semis<Self>) -> Result<usize, gazelle::ParseError> {
        match node {
            list::Semis::Semis(semis) => Ok(semis.len()),
        }
    }
}

impl gazelle::Action<list::Item<Self>> for Builder {
    fn build(&mut self, node: list::Item<Self>) -> Result<i32, gazelle::ParseError> {
        Ok(match node {
            list::Item::WithComma(n) => n,
            list::Item::WithoutComma(n) => n,
        })
    }
}

impl gazelle::Action<list::Nums<Self>> for Builder {
    fn build(&mut self, node: list::Nums<Self>) -> Result<Vec<i32>, gazelle::ParseError> {
        let list::Nums::Nums(nums) = node;
        Ok(nums)
    }
}

impl gazelle::Action<list::OptNum<Self>> for Builder {
    fn build(&mut self, node: list::OptNum<Self>) -> Result<Option<i32>, gazelle::ParseError> {
        let list::OptNum::Opt(opt) = node;
        Ok(opt)
    }
}

fn main() {
    println!("Modifier test example. Run with 'cargo test --example modifiers'.");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_items(input: &str) -> Result<Vec<i32>, String> {
        let tokens = lex(input)?;
        let mut parser = list::Parser::<Builder>::new();
        let mut actions = Builder;

        for tok in tokens {
            parser
                .push(tok, &mut actions)
                .map_err(|e| format!("Parse error: {:?}", e))?;
        }

        parser
            .finish(&mut actions)
            .map_err(|(p, e)| format!("Finish error: {}", p.format_error(&e)))
    }

    fn lex(input: &str) -> Result<Vec<list::Terminal<Builder>>, String> {
        use gazelle::lexer::Scanner;
        let mut src = Scanner::new(input);
        let mut tokens = Vec::new();

        loop {
            src.skip_whitespace();
            if src.at_end() {
                break;
            }

            if let Some(span) = src.read_digits() {
                let s = &input[span];
                tokens.push(list::Terminal::Num(s.parse().unwrap()));
            } else if let Some(c) = src.peek() {
                src.advance();
                match c {
                    ',' => tokens.push(list::Terminal::Comma),
                    ';' => tokens.push(list::Terminal::Semi),
                    _ => return Err(format!("Unexpected char: {}", c)),
                }
            }
        }
        Ok(tokens)
    }

    #[test]
    fn test_zero_items() {
        let result = parse_items("").unwrap();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_one_item() {
        let result = parse_items("42").unwrap();
        assert_eq!(result, vec![42]);
    }

    #[test]
    fn test_multiple_items() {
        let result = parse_items("1, 2, 3").unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_items_mixed_comma() {
        let result = parse_items("1, 2 3, 4").unwrap();
        assert_eq!(result, vec![1, 2, 3, 4]);
    }
}

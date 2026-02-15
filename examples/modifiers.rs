//! Test example for ?, *, + modifiers.
//!
//! Demonstrates the convenience syntax for optional, zero-or-more, and one-or-more.

use gazelle_macros::gazelle;

gazelle! {
    grammar List {
        start items;
        terminals {
            NUM: Num,
            COMMA,
            SEMI
        }

        // items: zero or more item, separated by nothing
        items = item* @items;

        // item: a number followed by an optional comma
        item = NUM COMMA @with_comma | NUM @without_comma;

        // nums: one or more numbers (for testing +)
        nums = NUM+ @nums;

        // opt_num: optional number followed by semi
        opt_num = NUM? SEMI @opt;

        // semis: zero or more semicolons (untyped terminal with *)
        semis = SEMI* @semis;
    }
}

#[allow(dead_code)]  // Only used in tests
struct Builder;

impl ListTypes for Builder {
    type Error = gazelle::ParseError;
    type Num = i32;
    type Items = Vec<i32>;
    type Item = i32;
    type Nums = Vec<i32>;
    type Opt_num = Option<i32>;
    type Semis = usize;  // count of semicolons
}

impl gazelle::Reduce<ListItems<Self>, Vec<i32>, gazelle::ParseError> for Builder {
    fn reduce(&mut self, node: ListItems<Self>) -> Result<Vec<i32>, gazelle::ParseError> {
        let ListItems::Items(items) = node;
        Ok(items)
    }
}

impl gazelle::Reduce<ListSemis<Self>, usize, gazelle::ParseError> for Builder {
    fn reduce(&mut self, node: ListSemis<Self>) -> Result<usize, gazelle::ParseError> {
        match node {
            ListSemis::Semis(semis) => Ok(semis.len()),
            ListSemis::__Phantom(_) => unreachable!(),
        }
    }
}

impl gazelle::Reduce<ListItem<Self>, i32, gazelle::ParseError> for Builder {
    fn reduce(&mut self, node: ListItem<Self>) -> Result<i32, gazelle::ParseError> {
        Ok(match node {
            ListItem::With_comma(n) => n,
            ListItem::Without_comma(n) => n,
        })
    }
}

impl gazelle::Reduce<ListNums<Self>, Vec<i32>, gazelle::ParseError> for Builder {
    fn reduce(&mut self, node: ListNums<Self>) -> Result<Vec<i32>, gazelle::ParseError> {
        let ListNums::Nums(nums) = node;
        Ok(nums)
    }
}

impl gazelle::Reduce<ListOpt_num<Self>, Option<i32>, gazelle::ParseError> for Builder {
    fn reduce(&mut self, node: ListOpt_num<Self>) -> Result<Option<i32>, gazelle::ParseError> {
        let ListOpt_num::Opt(opt) = node;
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
        let mut parser = ListParser::<Builder>::new();
        let mut actions = Builder;

        for tok in tokens {
            parser.push(tok, &mut actions).map_err(|e| format!("Parse error: {:?}", e))?;
        }

        parser.finish(&mut actions).map_err(|(p, e)| format!("Finish error: {}", p.format_error(&e)))
    }

    fn lex(input: &str) -> Result<Vec<ListTerminal<Builder>>, String> {
        use gazelle::lexer::Source;
        let mut src = Source::from_str(input);
        let mut tokens = Vec::new();

        loop {
            src.skip_whitespace();
            if src.at_end() {
                break;
            }

            if let Some(span) = src.read_digits() {
                let s = &input[span];
                tokens.push(ListTerminal::NUM(s.parse().unwrap()));
            } else if let Some(c) = src.peek() {
                src.advance();
                match c {
                    ',' => tokens.push(ListTerminal::COMMA),
                    ';' => tokens.push(ListTerminal::SEMI),
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

//! Simple lexer for tokenizing expressions.
//!
//! Produces tokens:
//! - Ident: alphanumeric starting with letter/underscore
//! - Num: sequence of digits
//! - Str: single-quoted string
//! - Op: sequence of operator characters (+, -, *, /, <, >, =, |, &, ^, !, ~, etc.)
//! - Punct: single punctuation (; , ( ) { } [ ])

/// A token from the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Num(String),
    Str(String),
    Op(String),
    Punct(char),
}

/// Lex input into tokens.
pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Whitespace
            ' ' | '\t' | '\n' | '\r' => {
                chars.next();
            }
            // Single-quoted string
            '\'' => {
                chars.next();
                let mut s = String::new();
                loop {
                    match chars.next() {
                        Some('\'') => break,
                        Some(c) => s.push(c),
                        None => return Err("Unterminated string".to_string()),
                    }
                }
                tokens.push(Token::Str(s));
            }
            // Punctuation
            ';' | ',' | '(' | ')' | '{' | '}' | '[' | ']' => {
                chars.next();
                tokens.push(Token::Punct(ch));
            }
            // Number
            c if c.is_ascii_digit() => {
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        s.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Num(s));
            }
            // Identifier
            c if c.is_alphabetic() || c == '_' => {
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        s.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Ident(s));
            }
            // Operator (sequence of operator chars)
            c if is_op_char(c) => {
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if is_op_char(c) {
                        s.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Op(s));
            }
            _ => {
                return Err(format!("Unexpected character: {:?}", ch));
            }
        }
    }

    Ok(tokens)
}

fn is_op_char(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '%' | '<' | '>' | '=' | '|' | '&' | '^' | '!' | '~' | '?' | ':' | '.' | '@' | '#' | '$')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_expr() {
        let tokens = lex("1 + 2 * 3").unwrap();
        assert_eq!(tokens, vec![
            Token::Num("1".into()),
            Token::Op("+".into()),
            Token::Num("2".into()),
            Token::Op("*".into()),
            Token::Num("3".into()),
        ]);
    }

    #[test]
    fn test_lex_compound_ops() {
        let tokens = lex("a << b || c && d").unwrap();
        assert_eq!(tokens, vec![
            Token::Ident("a".into()),
            Token::Op("<<".into()),
            Token::Ident("b".into()),
            Token::Op("||".into()),
            Token::Ident("c".into()),
            Token::Op("&&".into()),
            Token::Ident("d".into()),
        ]);
    }

    #[test]
    fn test_lex_string() {
        let tokens = lex("'hello' + 'world'").unwrap();
        assert_eq!(tokens, vec![
            Token::Str("hello".into()),
            Token::Op("+".into()),
            Token::Str("world".into()),
        ]);
    }

    #[test]
    fn test_lex_punct() {
        let tokens = lex("f(x, y);").unwrap();
        assert_eq!(tokens, vec![
            Token::Ident("f".into()),
            Token::Punct('('),
            Token::Ident("x".into()),
            Token::Punct(','),
            Token::Ident("y".into()),
            Token::Punct(')'),
            Token::Punct(';'),
        ]);
    }
}

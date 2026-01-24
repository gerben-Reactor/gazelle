//! General-purpose lexer for tokenizing programming languages.
//!
//! Produces tokens:
//! - Ident: alphanumeric starting with letter/underscore
//! - Num: integers, floats, hex (0x), binary (0b), octal (0o), with optional underscores
//! - Str: double-quoted string with escape sequences
//! - Char: single-quoted character
//! - Op: sequence of operator characters (+, -, *, /, <, >, =, |, &, ^, !, ~, etc.)
//! - Punct: single punctuation (; , ( ) { } [ ])
//!
//! Skips whitespace and comments (// line and /* block */).

use std::iter::Peekable;
use std::str::Chars;

/// A token from the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Num(String),
    Str(String),
    Char(char),
    Op(String),
    Punct(char),
}

/// Iterator-based lexer that returns one token at a time.
pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    line_comment_chars: &'a str,
    c_style_comments: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
            line_comment_chars: "",
            c_style_comments: true,
        }
    }

    /// Set single-character line comment starters (e.g., "#" or "#;").
    /// This disables the default C-style `//` and `/* */` comments.
    pub fn line_comments(mut self, chars: &'a str) -> Self {
        self.line_comment_chars = chars;
        self.c_style_comments = false;
        self
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace_and_comments();
        let &ch = self.chars.peek()?;

        Some(match ch {
            // Double-quoted string with escapes
            '"' => self.read_string(),
            // Single-quoted character
            '\'' => self.read_char(),
            // Punctuation
            ';' | ',' | '(' | ')' | '{' | '}' | '[' | ']' => {
                self.chars.next();
                Ok(Token::Punct(ch))
            }
            // Number
            c if c.is_ascii_digit() => self.read_number(),
            // Identifier
            c if c.is_alphabetic() || c == '_' => {
                let mut s = String::new();
                while let Some(&c) = self.chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        s.push(c);
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                Ok(Token::Ident(s))
            }
            // Operator - use maximal munch with known multi-char operators
            c if is_op_char(c) => {
                self.chars.next();
                let mut s = String::from(c);

                // Try to extend with known multi-char operators
                while let Some(&next) = self.chars.peek() {
                    if !is_op_char(next) {
                        break;
                    }
                    let candidate = format!("{}{}", s, next);
                    if is_valid_operator(&candidate) {
                        s.push(next);
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                Ok(Token::Op(s))
            }
            _ => Err(format!("Unexpected character: {:?}", ch)),
        })
    }
}

impl<'a> Lexer<'a> {
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Skip whitespace
            while let Some(&ch) = self.chars.peek() {
                if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                    self.chars.next();
                } else {
                    break;
                }
            }

            let Some(&ch) = self.chars.peek() else { break };

            // Single-char line comments (e.g., # or ;)
            if self.line_comment_chars.contains(ch) {
                self.skip_to_eol();
                continue;
            }

            // C-style comments
            if self.c_style_comments && ch == '/' {
                let mut tmp = self.chars.clone();
                tmp.next();
                match tmp.peek() {
                    Some(&'/') => {
                        self.chars.next();
                        self.chars.next();
                        self.skip_to_eol();
                        continue;
                    }
                    Some(&'*') => {
                        self.chars.next();
                        self.chars.next();
                        let mut prev = '\0';
                        while let Some(&c) = self.chars.peek() {
                            self.chars.next();
                            if prev == '*' && c == '/' {
                                break;
                            }
                            prev = c;
                        }
                        continue;
                    }
                    _ => {}
                }
            }
            break;
        }
    }

    fn skip_to_eol(&mut self) {
        while let Some(&c) = self.chars.peek() {
            self.chars.next();
            if c == '\n' {
                break;
            }
        }
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.chars.next(); // consume opening "
        let mut s = String::new();
        loop {
            match self.chars.next() {
                Some('"') => break,
                Some('\\') => match self.chars.next() {
                    Some('n') => s.push('\n'),
                    Some('t') => s.push('\t'),
                    Some('r') => s.push('\r'),
                    Some('\\') => s.push('\\'),
                    Some('"') => s.push('"'),
                    Some('\'') => s.push('\''),
                    Some('0') => s.push('\0'),
                    Some(c) => return Err(format!("Unknown escape sequence: \\{}", c)),
                    None => return Err("Unterminated string".to_string()),
                },
                Some(c) => s.push(c),
                None => return Err("Unterminated string".to_string()),
            }
        }
        Ok(Token::Str(s))
    }

    fn read_char(&mut self) -> Result<Token, String> {
        self.chars.next(); // consume opening '
        let c = match self.chars.next() {
            Some('\\') => match self.chars.next() {
                Some('n') => '\n',
                Some('t') => '\t',
                Some('r') => '\r',
                Some('\\') => '\\',
                Some('"') => '"',
                Some('\'') => '\'',
                Some('0') => '\0',
                Some('a') => '\x07', // bell
                Some('b') => '\x08', // backspace
                Some('f') => '\x0C', // form feed
                Some('v') => '\x0B', // vertical tab
                Some('?') => '?',
                Some('x') => {
                    // Hex escape sequence \xNN...
                    let mut val = 0u32;
                    while let Some(&c) = self.chars.peek() {
                        if let Some(d) = c.to_digit(16) {
                            val = val * 16 + d;
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    char::from_u32(val).unwrap_or('\0')
                }
                Some(c) if c.is_ascii_digit() => {
                    // Octal escape sequence \NNN
                    let mut val = c.to_digit(8).unwrap_or(0);
                    for _ in 0..2 {
                        if let Some(&c) = self.chars.peek() {
                            if let Some(d) = c.to_digit(8) {
                                val = val * 8 + d;
                                self.chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                    char::from_u32(val).unwrap_or('\0')
                }
                Some(c) => return Err(format!("Unknown escape sequence: \\{}", c)),
                None => return Err("Unterminated character literal".to_string()),
            },
            Some('\'') => return Err("Empty character literal".to_string()),
            Some(c) => c,
            None => return Err("Unterminated character literal".to_string()),
        };
        match self.chars.next() {
            Some('\'') => Ok(Token::Char(c)),
            Some(_) => Err("Character literal too long".to_string()),
            None => Err("Unterminated character literal".to_string()),
        }
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut s = String::new();
        let first = *self.chars.peek().unwrap();

        // Check for 0x, 0b, 0o prefixes
        if first == '0' {
            s.push('0');
            self.chars.next();

            if let Some(&prefix) = self.chars.peek() {
                match prefix {
                    'x' | 'X' => {
                        s.push(self.chars.next().unwrap());
                        while let Some(&c) = self.chars.peek() {
                            if c.is_ascii_hexdigit() || c == '_' {
                                if c != '_' { s.push(c); }
                                self.chars.next();
                            } else {
                                break;
                            }
                        }
                        return Ok(Token::Num(s));
                    }
                    'b' | 'B' => {
                        s.push(self.chars.next().unwrap());
                        while let Some(&c) = self.chars.peek() {
                            if c == '0' || c == '1' || c == '_' {
                                if c != '_' { s.push(c); }
                                self.chars.next();
                            } else {
                                break;
                            }
                        }
                        return Ok(Token::Num(s));
                    }
                    'o' | 'O' => {
                        s.push(self.chars.next().unwrap());
                        while let Some(&c) = self.chars.peek() {
                            if ('0'..='7').contains(&c) || c == '_' {
                                if c != '_' { s.push(c); }
                                self.chars.next();
                            } else {
                                break;
                            }
                        }
                        return Ok(Token::Num(s));
                    }
                    _ => {}
                }
            }
        }

        // Decimal integer part
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() || c == '_' {
                if c != '_' { s.push(c); }
                self.chars.next();
            } else {
                break;
            }
        }

        // Decimal part: only if '.' followed by digit
        if self.chars.peek() == Some(&'.') {
            let mut tmp = self.chars.clone();
            tmp.next();
            if tmp.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                s.push('.');
                self.chars.next();
                while let Some(&c) = self.chars.peek() {
                    if c.is_ascii_digit() || c == '_' {
                        if c != '_' { s.push(c); }
                        self.chars.next();
                    } else {
                        break;
                    }
                }
            }
        }

        // Exponent part: e/E followed by optional +/- and digits
        if self.chars.peek() == Some(&'e') || self.chars.peek() == Some(&'E') {
            let mut tmp = self.chars.clone();
            let e = tmp.next().unwrap();
            if tmp.peek() == Some(&'+') || tmp.peek() == Some(&'-') {
                tmp.next();
            }
            if tmp.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
                s.push(e);
                self.chars.next();
                if self.chars.peek() == Some(&'+') || self.chars.peek() == Some(&'-') {
                    s.push(self.chars.next().unwrap());
                }
                while let Some(&c) = self.chars.peek() {
                    if c.is_ascii_digit() || c == '_' {
                        if c != '_' { s.push(c); }
                        self.chars.next();
                    } else {
                        break;
                    }
                }
            }
        }

        Ok(Token::Num(s))
    }
}

/// Lex input into tokens.
pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    Lexer::new(input).collect()
}

fn is_op_char(c: char) -> bool {
    matches!(c, '+' | '-' | '*' | '/' | '%' | '<' | '>' | '=' | '|' | '&' | '^' | '!' | '~' | '?' | ':' | '.' | '@' | '#' | '$')
}

/// Check if a string is a valid (potentially multi-char) operator.
/// Uses common C-family operators for maximal munch.
fn is_valid_operator(s: &str) -> bool {
    matches!(s,
        // Single char (always valid start)
        "+" | "-" | "*" | "/" | "%" | "<" | ">" | "=" | "|" | "&" | "^" | "!" | "~" | "?" | ":" | "." | "@" | "#" | "$" |
        // Two char
        "++" | "--" | "+=" | "-=" | "*=" | "/=" | "%=" | "<<" | ">>" | "<=" | ">=" | "==" | "!=" |
        "&&" | "||" | "&=" | "|=" | "^=" | "->" | "::" | ".." |
        // Three char
        "<<=" | ">>=" | "..." | "<=>" | "->*"
    )
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
        let tokens = lex(r#""hello" + "world""#).unwrap();
        assert_eq!(tokens, vec![
            Token::Str("hello".into()),
            Token::Op("+".into()),
            Token::Str("world".into()),
        ]);
    }

    #[test]
    fn test_lex_string_escapes() {
        let tokens = lex(r#""line1\nline2\t\"quoted\"""#).unwrap();
        assert_eq!(tokens, vec![
            Token::Str("line1\nline2\t\"quoted\"".into()),
        ]);
    }

    #[test]
    fn test_lex_char() {
        let tokens = lex("'a' + '\\n'").unwrap();
        assert_eq!(tokens, vec![
            Token::Char('a'),
            Token::Op("+".into()),
            Token::Char('\n'),
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

    #[test]
    fn test_lex_floats() {
        let tokens = lex("3.14 + .5").unwrap();
        assert_eq!(tokens, vec![
            Token::Num("3.14".into()),
            Token::Op("+".into()),
            Token::Op(".".into()),
            Token::Num("5".into()),
        ]);
    }

    #[test]
    fn test_lex_scientific() {
        let tokens = lex("1e4 2.5e-3 1E+10").unwrap();
        assert_eq!(tokens, vec![
            Token::Num("1e4".into()),
            Token::Num("2.5e-3".into()),
            Token::Num("1E+10".into()),
        ]);
    }

    #[test]
    fn test_lex_field_access() {
        let tokens = lex("foo.bar 123.method").unwrap();
        assert_eq!(tokens, vec![
            Token::Ident("foo".into()),
            Token::Op(".".into()),
            Token::Ident("bar".into()),
            Token::Num("123".into()),
            Token::Op(".".into()),
            Token::Ident("method".into()),
        ]);
    }

    #[test]
    fn test_lex_hex_binary_octal() {
        let tokens = lex("0xFF 0b1010 0o77").unwrap();
        assert_eq!(tokens, vec![
            Token::Num("0xFF".into()),
            Token::Num("0b1010".into()),
            Token::Num("0o77".into()),
        ]);
    }

    #[test]
    fn test_lex_underscores_in_numbers() {
        let tokens = lex("1_000_000 0xFF_FF").unwrap();
        assert_eq!(tokens, vec![
            Token::Num("1000000".into()),
            Token::Num("0xFFFF".into()),
        ]);
    }

    #[test]
    fn test_lex_comments() {
        let tokens = lex("a // comment\n+ b /* block */ * c").unwrap();
        assert_eq!(tokens, vec![
            Token::Ident("a".into()),
            Token::Op("+".into()),
            Token::Ident("b".into()),
            Token::Op("*".into()),
            Token::Ident("c".into()),
        ]);
    }

    #[test]
    fn test_lex_hash_comments() {
        // # is comment, // becomes floor division operator
        let tokens: Result<Vec<_>, _> = Lexer::new("a // b # comment")
            .line_comments("#")
            .collect();
        assert_eq!(tokens.unwrap(), vec![
            Token::Ident("a".into()),
            Token::Op("//".into()),
            Token::Ident("b".into()),
        ]);
    }

    #[test]
    fn test_lex_semicolon_comments() {
        let tokens: Result<Vec<_>, _> = Lexer::new("(+ 1 2) ; comment\n(* 3 4)")
            .line_comments(";")
            .collect();
        assert_eq!(tokens.unwrap(), vec![
            Token::Punct('('),
            Token::Op("+".into()),
            Token::Num("1".into()),
            Token::Num("2".into()),
            Token::Punct(')'),
            Token::Punct('('),
            Token::Op("*".into()),
            Token::Num("3".into()),
            Token::Num("4".into()),
            Token::Punct(')'),
        ]);
    }
}

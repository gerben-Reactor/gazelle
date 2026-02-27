//! Regex-based tokenizer using LexerDfa.
//!
//! Demonstrates building a multi-pattern DFA lexer from regex patterns and
//! using it with Scanner to tokenize input.

use gazelle::lexer::{LexerDfa, Scanner};
use gazelle::regex::build_lexer_dfa;

const IDENT: u16 = 0;
const NUMBER: u16 = 1;
const PLUS: u16 = 2;
const STAR: u16 = 3;
const LPAREN: u16 = 4;
const RPAREN: u16 = 5;

fn build_lexer() -> LexerDfa {
    build_lexer_dfa(&[
        (IDENT, "[a-zA-Z_][a-zA-Z0-9_]*"),
        (NUMBER, "[0-9]+"),
        (PLUS, r"\+"),
        (STAR, r"\*"),
        (LPAREN, r"\("),
        (RPAREN, r"\)"),
    ])
    .expect("invalid regex")
}

fn token_name(id: u16) -> &'static str {
    match id {
        IDENT => "IDENT",
        NUMBER => "NUMBER",
        PLUS => "PLUS",
        STAR => "STAR",
        LPAREN => "LPAREN",
        RPAREN => "RPAREN",
        _ => "?",
    }
}

fn tokenize(input: &str) -> Result<Vec<(u16, String)>, String> {
    let dfa = build_lexer();
    let mut src = Scanner::new(input);
    let mut tokens = Vec::new();

    loop {
        src.skip_whitespace();
        if src.at_end() {
            break;
        }
        match dfa.read_token(&mut src) {
            Some((id, span)) => tokens.push((id, input[span].to_string())),
            None => {
                let (line, col) = src.line_col(src.offset());
                return Err(format!("{line}:{col}: unexpected character"));
            }
        }
    }
    Ok(tokens)
}

fn main() {
    let input = "foo + bar * (x + 123)";
    match tokenize(input) {
        Ok(tokens) => {
            for (id, text) in &tokens {
                println!("{:8} {:?}", token_name(*id), text);
            }
        }
        Err(e) => eprintln!("error: {e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("a + 1").unwrap();
        assert_eq!(
            tokens,
            vec![
                (IDENT, "a".into()),
                (PLUS, "+".into()),
                (NUMBER, "1".into()),
            ]
        );
    }

    #[test]
    fn test_keywords_vs_idents() {
        // Both match same pattern — LexerDfa uses longest match, same priority
        let tokens = tokenize("if ifx").unwrap();
        assert_eq!(tokens[0], (IDENT, "if".into()));
        assert_eq!(tokens[1], (IDENT, "ifx".into()));
    }

    #[test]
    fn test_error_on_unknown() {
        assert!(tokenize("a @ b").is_err());
    }
}

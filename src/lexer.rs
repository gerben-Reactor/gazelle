//! Composable lexer utilities with position tracking.
//!
//! A position-tracking wrapper over any char iterator with methods for reading tokens.
//! Users compose these methods to build lexers that produce their grammar's terminals.
//!
//! ```
//! use gazelle::lexer::{Source, Span};
//!
//! let input = "foo + 123";
//! let mut src = Source::new(input.chars());
//!
//! src.skip_whitespace();
//! if let Some(span) = src.read_ident() {
//!     let text = &input[span.start..span.end];  // "foo"
//! }
//! ```

use std::collections::VecDeque;

// ============================================================================
// Source - Composable lexer building blocks with position tracking
// ============================================================================

/// Position in source code.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Pos {
    /// Byte offset from start of input.
    pub offset: usize,
    /// Line number (1-indexed).
    pub line: usize,
    /// Column number (1-indexed, in characters not bytes).
    pub col: usize,
}

/// A span in source code (start and end byte offsets).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Create a new span.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Length of the span in bytes.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Whether the span is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// Error from lexer operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexError {
    pub message: String,
    pub pos: Pos,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.pos.line, self.pos.col, self.message)
    }
}

impl std::error::Error for LexError {}

/// Position-tracking source wrapper for building lexers.
///
/// Wraps any `Iterator<Item = char>` and tracks position (offset, line, column).
/// Provides composable methods for reading common token types, returning spans
/// that can be used to extract content from the original input.
///
/// # Example
///
/// ```
/// use gazelle::lexer::Source;
///
/// let input = "hello 123";
/// let mut src = Source::new(input.chars());
///
/// src.skip_whitespace();
/// let start = src.offset();
///
/// if let Some(span) = src.read_ident() {
///     let ident = &input[span.start..span.end];
///     assert_eq!(ident, "hello");
/// }
/// ```
pub struct Source<I: Iterator<Item = char>> {
    chars: I,
    /// Lookahead buffer for peeking without consuming.
    lookahead: VecDeque<char>,
    /// Current byte offset.
    offset: usize,
    /// Current line (1-indexed).
    line: usize,
    /// Current column (1-indexed).
    col: usize,
}

impl<'a> Source<std::str::Chars<'a>> {
    /// Create a new Source from a string slice.
    pub fn from_str(input: &'a str) -> Self {
        Self::new(input.chars())
    }
}

impl<I: Iterator<Item = char>> Source<I> {
    /// Create a new Source from any char iterator.
    pub fn new(iter: I) -> Self {
        Self {
            chars: iter,
            lookahead: VecDeque::new(),
            offset: 0,
            line: 1,
            col: 1,
        }
    }

    /// Current byte offset.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Current position (offset, line, column).
    pub fn pos(&self) -> Pos {
        Pos {
            offset: self.offset,
            line: self.line,
            col: self.col,
        }
    }

    /// Create a span from a start offset to the current position.
    pub fn span_from(&self, start: usize) -> Span {
        Span {
            start,
            end: self.offset,
        }
    }

    /// Peek at the next character without consuming it.
    pub fn peek(&mut self) -> Option<char> {
        if self.lookahead.is_empty() {
            if let Some(c) = self.chars.next() {
                self.lookahead.push_back(c);
            }
        }
        self.lookahead.front().copied()
    }

    /// Peek at the nth character ahead (0 = next char).
    pub fn peek_n(&mut self, n: usize) -> Option<char> {
        while self.lookahead.len() <= n {
            if let Some(c) = self.chars.next() {
                self.lookahead.push_back(c);
            } else {
                return None;
            }
        }
        self.lookahead.get(n).copied()
    }

    /// Consume and return the next character.
    pub fn advance(&mut self) -> Option<char> {
        let c = if let Some(c) = self.lookahead.pop_front() {
            c
        } else {
            self.chars.next()?
        };

        self.offset += c.len_utf8();
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }

        Some(c)
    }

    /// Check if we've reached the end of input.
    pub fn at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    // ========================================================================
    // Skipping methods
    // ========================================================================

    /// Skip whitespace characters (space, tab, newline, carriage return).
    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip characters while the predicate returns true.
    pub fn skip_while(&mut self, pred: impl Fn(char) -> bool) {
        while let Some(c) = self.peek() {
            if pred(c) {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip a line comment starting with the given prefix.
    /// Returns true if a comment was skipped.
    pub fn skip_line_comment(&mut self, prefix: &str) -> bool {
        if !self.starts_with(prefix) {
            return false;
        }
        // Consume the prefix
        for _ in 0..prefix.chars().count() {
            self.advance();
        }
        // Skip to end of line
        while let Some(c) = self.advance() {
            if c == '\n' {
                break;
            }
        }
        true
    }

    /// Skip a block comment with the given open/close delimiters.
    /// Returns true if a comment was skipped.
    pub fn skip_block_comment(&mut self, open: &str, close: &str) -> bool {
        if !self.starts_with(open) {
            return false;
        }
        // Consume the opening delimiter
        for _ in 0..open.chars().count() {
            self.advance();
        }
        // Find the closing delimiter
        let close_chars: Vec<char> = close.chars().collect();
        loop {
            if self.at_end() {
                break; // Unterminated comment
            }
            // Check for close delimiter
            let mut matched = true;
            for (i, &expected) in close_chars.iter().enumerate() {
                if self.peek_n(i) != Some(expected) {
                    matched = false;
                    break;
                }
            }
            if matched {
                for _ in 0..close_chars.len() {
                    self.advance();
                }
                break;
            }
            self.advance();
        }
        true
    }

    // ========================================================================
    // Reading methods - return Span on success
    // ========================================================================

    /// Read an identifier (letter/underscore followed by alphanumerics/underscores).
    /// Returns the span of the identifier, or None if not at an identifier.
    pub fn read_ident(&mut self) -> Option<Span> {
        let c = self.peek()?;
        if !c.is_alphabetic() && c != '_' {
            return None;
        }
        let start = self.offset;
        self.advance();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        Some(self.span_from(start))
    }

    /// Read an identifier with custom start/continue predicates.
    pub fn read_ident_where(
        &mut self,
        is_start: impl Fn(char) -> bool,
        is_continue: impl Fn(char) -> bool,
    ) -> Option<Span> {
        let c = self.peek()?;
        if !is_start(c) {
            return None;
        }
        let start = self.offset;
        self.advance();
        while let Some(c) = self.peek() {
            if is_continue(c) {
                self.advance();
            } else {
                break;
            }
        }
        Some(self.span_from(start))
    }

    /// Read a decimal integer (digits only, no prefix).
    pub fn read_integer(&mut self) -> Option<Span> {
        let c = self.peek()?;
        if !c.is_ascii_digit() {
            return None;
        }
        let start = self.offset;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        Some(self.span_from(start))
    }

    /// Read a number (integer or float, with optional hex/binary/octal prefix).
    /// Handles: 123, 3.14, 1e10, 0xFF, 0b1010, 0o77
    pub fn read_number(&mut self) -> Option<Span> {
        let c = self.peek()?;
        if !c.is_ascii_digit() {
            return None;
        }
        let start = self.offset;

        // Check for 0x, 0b, 0o prefixes
        if c == '0' {
            self.advance();
            match self.peek() {
                Some('x') | Some('X') => {
                    self.advance();
                    while let Some(c) = self.peek() {
                        if c.is_ascii_hexdigit() || c == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    return Some(self.span_from(start));
                }
                Some('b') | Some('B') => {
                    self.advance();
                    while let Some(c) = self.peek() {
                        if c == '0' || c == '1' || c == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    return Some(self.span_from(start));
                }
                Some('o') | Some('O') => {
                    self.advance();
                    while let Some(c) = self.peek() {
                        if ('0'..='7').contains(&c) || c == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    return Some(self.span_from(start));
                }
                _ => {}
            }
        }

        // Decimal digits
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        // Decimal part: only if '.' followed by digit
        if self.peek() == Some('.') && self.peek_n(1).map_or(false, |c| c.is_ascii_digit()) {
            self.advance(); // consume '.'
            while let Some(c) = self.peek() {
                if c.is_ascii_digit() || c == '_' {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // Exponent part
        if matches!(self.peek(), Some('e') | Some('E')) {
            let has_sign = matches!(self.peek_n(1), Some('+') | Some('-'));
            let digit_pos = if has_sign { 2 } else { 1 };
            if self.peek_n(digit_pos).map_or(false, |c| c.is_ascii_digit()) {
                self.advance(); // consume 'e'/'E'
                if has_sign {
                    self.advance(); // consume sign
                }
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() || c == '_' {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }

        Some(self.span_from(start))
    }

    /// Read a quoted string with escape sequences.
    /// The quote character is consumed but not included in the span.
    /// Returns the span of the string content (excluding quotes).
    pub fn read_string(&mut self, quote: char) -> Result<Span, LexError> {
        if self.peek() != Some(quote) {
            return Err(self.error(format!("expected '{}'", quote)));
        }
        self.advance(); // consume opening quote
        let start = self.offset;

        loop {
            match self.peek() {
                None => return Err(self.error("unterminated string")),
                Some(c) if c == quote => {
                    let end = self.offset;
                    self.advance(); // consume closing quote
                    return Ok(Span::new(start, end));
                }
                Some('\\') => {
                    self.advance(); // consume backslash
                    if self.advance().is_none() {
                        return Err(self.error("unterminated escape sequence"));
                    }
                }
                Some(_) => {
                    self.advance();
                }
            }
        }
    }

    /// Read characters while the predicate returns true.
    /// Returns the span of matched characters (may be empty).
    pub fn read_while(&mut self, pred: impl Fn(char) -> bool) -> Span {
        let start = self.offset;
        while let Some(c) = self.peek() {
            if pred(c) {
                self.advance();
            } else {
                break;
            }
        }
        self.span_from(start)
    }

    /// Try to consume an exact string. Returns the span if matched, None otherwise.
    /// Only consumes if the entire string matches.
    pub fn read_exact(&mut self, s: &str) -> Option<Span> {
        if !self.starts_with(s) {
            return None;
        }
        let start = self.offset;
        for _ in s.chars() {
            self.advance();
        }
        Some(self.span_from(start))
    }

    /// Try to match one of the given strings, checking in order.
    /// Returns the index of the matched string and its span on success.
    ///
    /// Use this for maximal munch: put longer options first.
    /// The index can be used to look up corresponding data in a parallel array.
    ///
    /// # Example
    /// ```
    /// use gazelle::lexer::Source;
    ///
    /// let input = "<<= foo";
    /// let mut src = Source::from_str(input);
    ///
    /// const OPS: &[&str] = &["<<=", "<<", "<=", "<"];
    ///
    /// // Longest first for maximal munch
    /// if let Some((idx, _span)) = src.read_one_of(OPS) {
    ///     assert_eq!(idx, 0);  // matched "<<=", first in list
    ///     assert_eq!(OPS[idx], "<<=");
    /// }
    /// ```
    pub fn read_one_of(&mut self, options: &[&str]) -> Option<(usize, Span)> {
        for (i, &option) in options.iter().enumerate() {
            if let Some(span) = self.read_exact(option) {
                return Some((i, span));
            }
        }
        None
    }

    /// Check if the remaining input starts with the given string.
    /// Does not consume any input.
    pub fn starts_with(&mut self, s: &str) -> bool {
        for (i, expected) in s.chars().enumerate() {
            if self.peek_n(i) != Some(expected) {
                return false;
            }
        }
        true
    }

    /// Create an error at the current position.
    pub fn error(&self, message: impl Into<String>) -> LexError {
        LexError {
            message: message.into(),
            pos: self.pos(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Source tests
    // ========================================================================

    #[test]
    fn test_source_position_tracking() {
        let input = "ab\ncd";
        let mut src = Source::from_str(input);

        assert_eq!(src.pos(), Pos { offset: 0, line: 1, col: 1 });
        assert_eq!(src.advance(), Some('a'));
        assert_eq!(src.pos(), Pos { offset: 1, line: 1, col: 2 });
        assert_eq!(src.advance(), Some('b'));
        assert_eq!(src.pos(), Pos { offset: 2, line: 1, col: 3 });
        assert_eq!(src.advance(), Some('\n'));
        assert_eq!(src.pos(), Pos { offset: 3, line: 2, col: 1 });
        assert_eq!(src.advance(), Some('c'));
        assert_eq!(src.pos(), Pos { offset: 4, line: 2, col: 2 });
    }

    #[test]
    fn test_source_peek() {
        let input = "abc";
        let mut src = Source::from_str(input);

        assert_eq!(src.peek(), Some('a'));
        assert_eq!(src.peek(), Some('a')); // peek doesn't consume
        assert_eq!(src.peek_n(0), Some('a'));
        assert_eq!(src.peek_n(1), Some('b'));
        assert_eq!(src.peek_n(2), Some('c'));
        assert_eq!(src.peek_n(3), None);

        assert_eq!(src.advance(), Some('a'));
        assert_eq!(src.peek(), Some('b'));
        assert_eq!(src.peek_n(1), Some('c'));
    }

    #[test]
    fn test_source_skip_whitespace() {
        let input = "  \t\n  hello";
        let mut src = Source::from_str(input);

        src.skip_whitespace();
        assert_eq!(src.peek(), Some('h'));
        assert_eq!(src.pos().offset, 6);
    }

    #[test]
    fn test_source_skip_line_comment() {
        let input = "// comment\nhello";
        let mut src = Source::from_str(input);

        assert!(src.skip_line_comment("//"));
        assert_eq!(src.peek(), Some('h'));
    }

    #[test]
    fn test_source_skip_block_comment() {
        let input = "/* block */hello";
        let mut src = Source::from_str(input);

        assert!(src.skip_block_comment("/*", "*/"));
        assert_eq!(src.peek(), Some('h'));
    }

    #[test]
    fn test_source_read_ident() {
        let input = "foo_bar123 + rest";
        let mut src = Source::from_str(input);

        let span = src.read_ident().unwrap();
        assert_eq!(&input[span.start..span.end], "foo_bar123");
        assert_eq!(src.peek(), Some(' '));
    }

    #[test]
    fn test_source_read_ident_where() {
        let input = "foo-bar-baz + rest";
        let mut src = Source::from_str(input);

        // Lisp-style identifiers with hyphens
        let span = src.read_ident_where(
            |c| c.is_alphabetic(),
            |c| c.is_alphanumeric() || c == '-',
        ).unwrap();
        assert_eq!(&input[span.start..span.end], "foo-bar-baz");
    }

    #[test]
    fn test_source_read_integer() {
        let input = "12345 rest";
        let mut src = Source::from_str(input);

        let span = src.read_integer().unwrap();
        assert_eq!(&input[span.start..span.end], "12345");
    }

    #[test]
    fn test_source_read_number_int() {
        let input = "42 rest";
        let mut src = Source::from_str(input);

        let span = src.read_number().unwrap();
        assert_eq!(&input[span.start..span.end], "42");
    }

    #[test]
    fn test_source_read_number_float() {
        let input = "3.14159 rest";
        let mut src = Source::from_str(input);

        let span = src.read_number().unwrap();
        assert_eq!(&input[span.start..span.end], "3.14159");
    }

    #[test]
    fn test_source_read_number_scientific() {
        let input = "1.5e-10 rest";
        let mut src = Source::from_str(input);

        let span = src.read_number().unwrap();
        assert_eq!(&input[span.start..span.end], "1.5e-10");
    }

    #[test]
    fn test_source_read_number_hex() {
        let input = "0xDEAD_BEEF rest";
        let mut src = Source::from_str(input);

        let span = src.read_number().unwrap();
        assert_eq!(&input[span.start..span.end], "0xDEAD_BEEF");
    }

    #[test]
    fn test_source_read_number_binary() {
        let input = "0b1010_1100 rest";
        let mut src = Source::from_str(input);

        let span = src.read_number().unwrap();
        assert_eq!(&input[span.start..span.end], "0b1010_1100");
    }

    #[test]
    fn test_source_read_string() {
        let input = r#""hello world" rest"#;
        let mut src = Source::from_str(input);

        let span = src.read_string('"').unwrap();
        assert_eq!(&input[span.start..span.end], "hello world");
        assert_eq!(src.peek(), Some(' '));
    }

    #[test]
    fn test_source_read_string_escapes() {
        let input = r#""hello\"world" rest"#;
        let mut src = Source::from_str(input);

        let span = src.read_string('"').unwrap();
        // The span includes the escape sequences as-is
        assert_eq!(&input[span.start..span.end], r#"hello\"world"#);
    }

    #[test]
    fn test_source_read_exact() {
        let input = "<<= rest";
        let mut src = Source::from_str(input);

        assert!(src.read_exact("<<=").is_some());
        assert_eq!(src.peek(), Some(' '));
    }

    #[test]
    fn test_source_read_exact_no_match() {
        let input = "<< rest";
        let mut src = Source::from_str(input);

        assert!(src.read_exact("<<=").is_none());
        // Nothing consumed
        assert_eq!(src.peek(), Some('<'));
        assert_eq!(src.offset(), 0);
    }

    #[test]
    fn test_source_read_one_of() {
        // Maximal munch: longer options first
        const OPS: &[&str] = &["<<=", "<<", "<=", "<"];

        let input = "<<= rest";
        let mut src = Source::from_str(input);
        let (idx, span) = src.read_one_of(OPS).unwrap();
        assert_eq!(idx, 0);
        assert_eq!(OPS[idx], "<<=");
        assert_eq!(&input[span.start..span.end], "<<=");

        let input = "<< rest";
        let mut src = Source::from_str(input);
        let (idx, span) = src.read_one_of(OPS).unwrap();
        assert_eq!(idx, 1);
        assert_eq!(OPS[idx], "<<");
        assert_eq!(&input[span.start..span.end], "<<");

        let input = "<= rest";
        let mut src = Source::from_str(input);
        let (idx, span) = src.read_one_of(OPS).unwrap();
        assert_eq!(idx, 2);
        assert_eq!(OPS[idx], "<=");
        assert_eq!(&input[span.start..span.end], "<=");

        let input = "< rest";
        let mut src = Source::from_str(input);
        let (idx, span) = src.read_one_of(OPS).unwrap();
        assert_eq!(idx, 3);
        assert_eq!(OPS[idx], "<");
        assert_eq!(&input[span.start..span.end], "<");

        // No match
        let input = "> rest";
        let mut src = Source::from_str(input);
        assert!(src.read_one_of(OPS).is_none());
        assert_eq!(src.offset(), 0); // Nothing consumed
    }

    #[test]
    fn test_source_starts_with() {
        let input = "hello world";
        let mut src = Source::from_str(input);

        assert!(src.starts_with("hello"));
        assert!(src.starts_with("hel"));
        assert!(!src.starts_with("world"));
        // Didn't consume anything
        assert_eq!(src.offset(), 0);
    }

    #[test]
    fn test_source_read_while() {
        let input = "aaabbbccc";
        let mut src = Source::from_str(input);

        let span = src.read_while(|c| c == 'a');
        assert_eq!(&input[span.start..span.end], "aaa");
        assert_eq!(src.peek(), Some('b'));
    }

    #[test]
    fn test_source_span_from() {
        let input = "hello world";
        let mut src = Source::from_str(input);

        let start = src.offset();
        for _ in 0..5 {
            src.advance();
        }
        let span = src.span_from(start);
        assert_eq!(&input[span.start..span.end], "hello");
    }

    #[test]
    fn test_source_complete_lexer() {
        // Example of composing Source methods into a simple lexer
        let input = "foo + 123";
        let mut src = Source::from_str(input);
        let mut tokens = Vec::new();

        loop {
            src.skip_whitespace();
            if src.at_end() {
                break;
            }

            let start = src.offset();

            if let Some(span) = src.read_ident() {
                tokens.push(("ident", &input[span.start..span.end]));
            } else if let Some(span) = src.read_number() {
                tokens.push(("number", &input[span.start..span.end]));
            } else if src.read_exact("+").is_some() {
                tokens.push(("op", "+"));
            } else {
                panic!("unexpected char at {}", start);
            }
        }

        assert_eq!(tokens, vec![
            ("ident", "foo"),
            ("op", "+"),
            ("number", "123"),
        ]);
    }

}

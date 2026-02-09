//! Composable lexer utilities with position tracking.
//!
//! A position-tracking wrapper over any char iterator with methods for reading tokens.
//! Users compose these methods to build lexers that produce their grammar's terminals.
//!
//! ```
//! use gazelle::lexer::Source;
//!
//! let input = "foo + 123";
//! let mut src = Source::new(input.chars());
//!
//! src.skip_whitespace();
//! if let Some(span) = src.read_ident() {
//!     let text = &input[span];  // "foo"
//!     // Line/col computed on demand: src.line_col(span.start)
//! }
//! ```

use std::collections::VecDeque;

// ============================================================================
// Source - Composable lexer building blocks with position tracking
// ============================================================================

/// A span in source code (start and end byte offsets).
/// Type alias for Range<usize> so it can be used directly for indexing.
pub type Span = std::ops::Range<usize>;

/// Error from lexer operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexError {
    pub message: String,
    pub offset: usize,
}

impl LexError {
    /// Format error with line/column from a Source.
    pub fn format<I: Iterator<Item = char>>(&self, src: &Source<I>) -> String {
        let (line, col) = src.line_col(self.offset);
        format!("{}:{}: {}", line, col, self.message)
    }
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "offset {}: {}", self.offset, self.message)
    }
}

impl std::error::Error for LexError {}

/// Position-tracking source wrapper for building lexers.
///
/// Wraps any `Iterator<Item = char>` and tracks byte offset and line starts.
/// Line/column are computed on demand from offset.
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
/// if let Some(span) = src.read_ident() {
///     let ident = &input[span];
///     assert_eq!(ident, "hello");
///     // Line/col on demand: src.line_col(span.start)
/// }
/// ```
pub struct Source<I: Iterator<Item = char>> {
    chars: I,
    /// Lookahead buffer for peeking without consuming.
    lookahead: VecDeque<char>,
    /// Current byte offset.
    offset: usize,
    /// Byte offsets where each line starts. line_starts[0] = 0 (line 1 starts at offset 0).
    line_starts: Vec<usize>,
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
            line_starts: vec![0],  // Line 1 starts at offset 0
        }
    }

    /// Current byte offset.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Compute line and column (1-indexed) from a byte offset.
    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        // Binary search for the line containing this offset
        let line = self.line_starts.partition_point(|&start| start <= offset);
        let line_start = self.line_starts[line - 1];
        let col = offset - line_start + 1;
        (line, col)
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
            self.line_starts.push(self.offset);
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
        Some(start..self.offset)
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
        Some(start..self.offset)
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
        Some(start..self.offset)
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
                    return Some(start..self.offset);
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
                    return Some(start..self.offset);
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
                    return Some(start..self.offset);
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

        Some(start..self.offset)
    }

    /// Read a quoted string with escape sequences.
    /// The quote character is consumed but not included in the returned span.
    /// Returns span of string content (excluding quotes).
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
                    return Ok(start..end);
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
    /// Returns span of matched characters (may be empty).
    pub fn read_while(&mut self, pred: impl Fn(char) -> bool) -> Span {
        let start = self.offset;
        while let Some(c) = self.peek() {
            if pred(c) {
                self.advance();
            } else {
                break;
            }
        }
        start..self.offset
    }

    /// Try to consume an exact string. Returns span if matched, None otherwise.
    /// Only consumes if the entire string matches.
    pub fn read_exact(&mut self, s: &str) -> Option<Span> {
        if !self.starts_with(s) {
            return None;
        }
        let start = self.offset;
        for _ in s.chars() {
            self.advance();
        }
        Some(start..self.offset)
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
    /// if let Some((idx, span)) = src.read_one_of(OPS) {
    ///     assert_eq!(idx, 0);  // matched "<<=", first in list
    ///     assert_eq!(&input[span], "<<=");
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

    /// Create an error at the current offset.
    pub fn error(&self, message: impl Into<String>) -> LexError {
        LexError {
            message: message.into(),
            offset: self.offset,
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
    fn test_source_line_col() {
        let input = "ab\ncd\nef";
        let mut src = Source::from_str(input);

        // Consume all chars to build line table
        while src.advance().is_some() {}

        // Test line/col computation
        assert_eq!(src.line_col(0), (1, 1));  // 'a'
        assert_eq!(src.line_col(1), (1, 2));  // 'b'
        assert_eq!(src.line_col(2), (1, 3));  // '\n'
        assert_eq!(src.line_col(3), (2, 1));  // 'c'
        assert_eq!(src.line_col(4), (2, 2));  // 'd'
        assert_eq!(src.line_col(5), (2, 3));  // '\n'
        assert_eq!(src.line_col(6), (3, 1));  // 'e'
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
        assert_eq!(src.offset(), 6);
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
        assert_eq!(&input[span], "foo_bar123");
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
        assert_eq!(&input[span], "foo-bar-baz");
    }

    #[test]
    fn test_source_read_integer() {
        let input = "12345 rest";
        let mut src = Source::from_str(input);

        let span = src.read_integer().unwrap();
        assert_eq!(&input[span], "12345");
    }

    #[test]
    fn test_source_read_number_int() {
        let input = "42 rest";
        let mut src = Source::from_str(input);

        let span = src.read_number().unwrap();
        assert_eq!(&input[span], "42");
    }

    #[test]
    fn test_source_read_number_float() {
        let input = "3.14159 rest";
        let mut src = Source::from_str(input);

        let span = src.read_number().unwrap();
        assert_eq!(&input[span], "3.14159");
    }

    #[test]
    fn test_source_read_number_scientific() {
        let input = "1.5e-10 rest";
        let mut src = Source::from_str(input);

        let span = src.read_number().unwrap();
        assert_eq!(&input[span], "1.5e-10");
    }

    #[test]
    fn test_source_read_number_hex() {
        let input = "0xDEAD_BEEF rest";
        let mut src = Source::from_str(input);

        let span = src.read_number().unwrap();
        assert_eq!(&input[span], "0xDEAD_BEEF");
    }

    #[test]
    fn test_source_read_number_binary() {
        let input = "0b1010_1100 rest";
        let mut src = Source::from_str(input);

        let span = src.read_number().unwrap();
        assert_eq!(&input[span], "0b1010_1100");
    }

    #[test]
    fn test_source_read_string() {
        let input = r#""hello world" rest"#;
        let mut src = Source::from_str(input);

        let span = src.read_string('"').unwrap();
        assert_eq!(&input[span], "hello world");
        assert_eq!(src.peek(), Some(' '));
    }

    #[test]
    fn test_source_read_string_escapes() {
        let input = r#""hello\"world" rest"#;
        let mut src = Source::from_str(input);

        let span = src.read_string('"').unwrap();
        assert_eq!(&input[span], r#"hello\"world"#);
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
        assert_eq!(&input[span], "<<=");

        let input = "<< rest";
        let mut src = Source::from_str(input);
        let (idx, span) = src.read_one_of(OPS).unwrap();
        assert_eq!(idx, 1);
        assert_eq!(OPS[idx], "<<");
        assert_eq!(&input[span], "<<");

        let input = "<= rest";
        let mut src = Source::from_str(input);
        let (idx, span) = src.read_one_of(OPS).unwrap();
        assert_eq!(idx, 2);
        assert_eq!(OPS[idx], "<=");
        assert_eq!(&input[span], "<=");

        let input = "< rest";
        let mut src = Source::from_str(input);
        let (idx, span) = src.read_one_of(OPS).unwrap();
        assert_eq!(idx, 3);
        assert_eq!(OPS[idx], "<");
        assert_eq!(&input[span], "<");

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
        assert_eq!(&input[span], "aaa");
        assert_eq!(src.peek(), Some('b'));
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

            if let Some(span) = src.read_ident() {
                tokens.push(("ident", &input[span]));
            } else if let Some(span) = src.read_number() {
                tokens.push(("number", &input[span]));
            } else if src.read_exact("+").is_some() {
                tokens.push(("op", "+"));
            } else {
                panic!("unexpected char at {}", src.offset());
            }
        }

        assert_eq!(tokens, vec![
            ("ident", "foo"),
            ("op", "+"),
            ("number", "123"),
        ]);
    }

}

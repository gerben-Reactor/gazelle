//! Composable lexer building blocks with position tracking.
//!
//! [`Scanner`] wraps any char iterator and tracks byte offsets and line starts.
//! Its methods are building blocks — compose them to build a lexer for your grammar's terminals.
//!
//! ```
//! use gazelle::lexer::Scanner;
//!
//! let input = "foo + 123";
//! let mut src = Scanner::new(input);
//!
//! src.skip_whitespace();
//! if let Some(span) = src.read_ident() {
//!     let text = &input[span];  // "foo"
//!     // Line/col computed on demand: src.line_col(span.start)
//! }
//! ```

use std::collections::VecDeque;

// ============================================================================
// Scanner - Composable lexer building blocks with position tracking
// ============================================================================

/// A span in source code (start and end byte offsets).
/// Type alias for `Range<usize>` so it can be used directly for indexing.
pub type Span = std::ops::Range<usize>;

/// Error from lexer operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LexError {
    /// The error message describing what went wrong.
    pub message: String,
    /// Byte offset in the source where the error occurred.
    pub offset: usize,
}

impl LexError {
    /// Format error with line/column from a Scanner.
    pub fn format<I: Iterator<Item = char>>(&self, src: &Scanner<I>) -> String {
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

/// Position-tracking character scanner for building lexers.
///
/// Wraps any `Iterator<Item = char>` and tracks byte offset and line starts.
/// Line/column are computed on demand from offset.
///
/// # Example
///
/// ```
/// use gazelle::lexer::Scanner;
///
/// let input = "hello 123";
/// let mut src = Scanner::new(input);
///
/// src.skip_whitespace();
/// if let Some(span) = src.read_ident() {
///     let ident = &input[span];
///     assert_eq!(ident, "hello");
///     // Line/col on demand: src.line_col(span.start)
/// }
/// ```
pub struct Scanner<I: Iterator<Item = char>> {
    chars: I,
    /// Lookahead buffer for peeking without consuming.
    lookahead: VecDeque<char>,
    /// Current byte offset.
    offset: usize,
    /// Byte offsets where each line starts. line_starts[0] = 0 (line 1 starts at offset 0).
    line_starts: Vec<usize>,
}

impl<'a> Scanner<std::str::Chars<'a>> {
    /// Create a new Scanner from a string slice.
    pub fn new(input: &'a str) -> Self {
        Self::from_iter(input.chars())
    }
}

impl<I: Iterator<Item = char>> Scanner<I> {
    /// Create a new Scanner from any char iterator.
    pub fn from_iter(iter: I) -> Self {
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

    /// Read decimal digits (0-9 and optional underscores).
    /// Returns None if not starting with a digit.
    pub fn read_digits(&mut self) -> Option<Span> {
        let c = self.peek()?;
        if !c.is_ascii_digit() {
            return None;
        }
        let start = self.offset;
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        Some(start..self.offset)
    }

    /// Read hex digits (0-9, a-f, A-F, and optional underscores).
    /// Returns None if not starting with a hex digit.
    pub fn read_hex_digits(&mut self) -> Option<Span> {
        let c = self.peek()?;
        if !c.is_ascii_hexdigit() {
            return None;
        }
        let start = self.offset;
        while let Some(c) = self.peek() {
            if c.is_ascii_hexdigit() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        Some(start..self.offset)
    }

    /// Read until any of the given characters (or EOF).
    /// Does not consume the stopping character.
    /// Returns span of content read (may be empty if immediately at a stop char).
    pub fn read_until_any(&mut self, chars: &[char]) -> Span {
        let start = self.offset;
        while let Some(c) = self.peek() {
            if chars.contains(&c) {
                break;
            }
            self.advance();
        }
        start..self.offset
    }

    /// Read a quoted string, skipping escape sequences without interpreting them.
    /// Returns span of raw content (excluding quotes).
    /// Use this when you just need to find the string boundary.
    pub fn read_string_raw(&mut self, quote: char) -> Result<Span, LexError> {
        if self.peek() != Some(quote) {
            return Err(self.error(format!("expected '{}'", quote)));
        }
        self.advance(); // opening quote
        let start = self.offset;

        loop {
            self.read_until_any(&[quote, '\\']);
            match self.peek() {
                Some(c) if c == quote => {
                    let end = self.offset;
                    self.advance(); // closing quote
                    return Ok(start..end);
                }
                Some('\\') => {
                    self.advance(); // consume backslash
                    self.advance(); // skip next char
                }
                None => {
                    return Err(self.error("unterminated string"));
                }
                Some(_) => unreachable!(),
            }
        }
    }

    /// Read a C-style string with standard escape sequences.
    /// Consumes opening and closing quotes, returns (span of raw content, interpreted value).
    /// Handles: \n \t \r \\ \' \" \0 \xNN
    pub fn read_c_string(&mut self, quote: char, input: &str) -> Result<(Span, String), LexError> {
        if self.peek() != Some(quote) {
            return Err(self.error(format!("expected '{}'", quote)));
        }
        self.advance(); // opening quote
        let start = self.offset;
        let mut value = String::new();

        loop {
            let span = self.read_until_any(&[quote, '\\']);
            value.push_str(&input[span]);

            match self.peek() {
                Some(c) if c == quote => {
                    let end = self.offset;
                    self.advance(); // closing quote
                    return Ok((start..end, value));
                }
                Some('\\') => {
                    let esc_offset = self.offset;
                    self.advance(); // consume backslash
                    match self.peek() {
                        Some('n') => { value.push('\n'); self.advance(); }
                        Some('t') => { value.push('\t'); self.advance(); }
                        Some('r') => { value.push('\r'); self.advance(); }
                        Some('\\') => { value.push('\\'); self.advance(); }
                        Some('\'') => { value.push('\''); self.advance(); }
                        Some('"') => { value.push('"'); self.advance(); }
                        Some('0') => { value.push('\0'); self.advance(); }
                        Some('x') => {
                            self.advance(); // consume 'x'
                            let h1 = self.peek().and_then(|c| c.to_digit(16));
                            let h2 = self.peek_n(1).and_then(|c| c.to_digit(16));
                            match (h1, h2) {
                                (Some(a), Some(b)) => {
                                    self.advance();
                                    self.advance();
                                    value.push(char::from((a * 16 + b) as u8));
                                }
                                _ => {
                                    return Err(LexError {
                                        message: "invalid \\xNN escape".into(),
                                        offset: esc_offset,
                                    });
                                }
                            }
                        }
                        Some(c) => {
                            return Err(LexError {
                                message: format!("invalid escape sequence: \\{}", c),
                                offset: esc_offset,
                            });
                        }
                        None => {
                            return Err(self.error("unterminated escape sequence"));
                        }
                    }
                }
                None => {
                    return Err(self.error("unterminated string"));
                }
                Some(_) => unreachable!("read_until_any should stop at quote or backslash"),
            }
        }
    }

    /// Read a Rust-style raw string: r"..." or r#"..."#
    /// Caller has consumed 'r' and passes the number of '#' consumed (0 for r"...").
    /// Returns span of content (excluding quotes and hashes).
    pub fn read_rust_raw_string(&mut self, hashes: usize) -> Result<Span, LexError> {
        if self.peek() != Some('"') {
            return Err(self.error("expected '\"' after r"));
        }
        self.advance(); // opening quote
        let start = self.offset;

        loop {
            self.read_until_any(&['"']);

            if self.at_end() {
                return Err(self.error("unterminated raw string"));
            }

            let potential_end = self.offset;
            self.advance(); // consume "

            // Count following #s
            let mut hash_count = 0;
            while self.peek() == Some('#') && hash_count < hashes {
                self.advance();
                hash_count += 1;
            }

            if hash_count == hashes {
                return Ok(start..potential_end);
            }
            // Otherwise the " and #s were part of content, continue
        }
    }

    /// Read a C++11 raw string: R"delim(...)delim"
    /// Caller has consumed 'R', this consumes the rest.
    /// Returns span of content (excluding delimiters).
    pub fn read_cpp_raw_string(&mut self, input: &str) -> Result<Span, LexError> {
        if self.peek() != Some('"') {
            return Err(self.error("expected '\"' after R"));
        }
        self.advance();

        // Read delimiter until '('
        let delim_start = self.offset;
        while self.peek() != Some('(') && !self.at_end() {
            self.advance();
        }
        if self.at_end() {
            return Err(self.error("expected '(' in raw string"));
        }
        let delimiter = &input[delim_start..self.offset];
        self.advance(); // consume (

        let content_start = self.offset;

        // Look for )delimiter"
        let closing = format!("){}\"", delimiter);

        loop {
            self.read_until_any(&[')']);

            if self.at_end() {
                return Err(self.error("unterminated raw string"));
            }

            let potential_end = self.offset;

            if self.starts_with(&closing) {
                // Consume the closing
                for _ in closing.chars() {
                    self.advance();
                }
                return Ok(content_start..potential_end);
            }

            self.advance(); // this ) wasn't the end, continue
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
    /// use gazelle::lexer::Scanner;
    ///
    /// let input = "<<= foo";
    /// let mut src = Scanner::new(input);
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
    // Scanner tests
    // ========================================================================

    #[test]
    fn test_source_line_col() {
        let input = "ab\ncd\nef";
        let mut src = Scanner::new(input);

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
        let mut src = Scanner::new(input);

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
        let mut src = Scanner::new(input);

        src.skip_whitespace();
        assert_eq!(src.peek(), Some('h'));
        assert_eq!(src.offset(), 6);
    }

    #[test]
    fn test_source_skip_line_comment() {
        let input = "// comment\nhello";
        let mut src = Scanner::new(input);

        assert!(src.skip_line_comment("//"));
        assert_eq!(src.peek(), Some('h'));
    }

    #[test]
    fn test_source_skip_block_comment() {
        let input = "/* block */hello";
        let mut src = Scanner::new(input);

        assert!(src.skip_block_comment("/*", "*/"));
        assert_eq!(src.peek(), Some('h'));
    }

    #[test]
    fn test_source_read_ident() {
        let input = "foo_bar123 + rest";
        let mut src = Scanner::new(input);

        let span = src.read_ident().unwrap();
        assert_eq!(&input[span], "foo_bar123");
        assert_eq!(src.peek(), Some(' '));
    }

    #[test]
    fn test_source_read_ident_where() {
        let input = "foo-bar-baz + rest";
        let mut src = Scanner::new(input);

        // Lisp-style identifiers with hyphens
        let span = src.read_ident_where(
            |c| c.is_alphabetic(),
            |c| c.is_alphanumeric() || c == '-',
        ).unwrap();
        assert_eq!(&input[span], "foo-bar-baz");
    }

    #[test]
    fn test_source_read_digits() {
        let input = "12345 rest";
        let mut src = Scanner::new(input);

        let span = src.read_digits().unwrap();
        assert_eq!(&input[span], "12345");
    }

    #[test]
    fn test_source_read_digits_with_underscores() {
        let input = "1_000_000 rest";
        let mut src = Scanner::new(input);

        let span = src.read_digits().unwrap();
        assert_eq!(&input[span], "1_000_000");
    }

    #[test]
    fn test_source_read_hex_digits() {
        let input = "DEAD_BEEF rest";
        let mut src = Scanner::new(input);

        let span = src.read_hex_digits().unwrap();
        assert_eq!(&input[span], "DEAD_BEEF");
    }

    #[test]
    fn test_source_read_until_any() {
        let input = "hello, world";
        let mut src = Scanner::new(input);

        let span = src.read_until_any(&[',', '!']);
        assert_eq!(&input[span], "hello");
        assert_eq!(src.peek(), Some(','));
    }

    #[test]
    fn test_source_read_c_string() {
        let input = r#""hello world" rest"#;
        let mut src = Scanner::new(input);

        let (span, value) = src.read_c_string('"', input).unwrap();
        assert_eq!(&input[span], "hello world");
        assert_eq!(value, "hello world");
        assert_eq!(src.peek(), Some(' '));
    }

    #[test]
    fn test_source_read_c_string_escapes() {
        let input = r#""hello\nworld\t!" rest"#;
        let mut src = Scanner::new(input);

        let (span, value) = src.read_c_string('"', input).unwrap();
        assert_eq!(&input[span], r#"hello\nworld\t!"#);
        assert_eq!(value, "hello\nworld\t!");
    }

    #[test]
    fn test_source_read_c_string_hex_escape() {
        let input = r#""\x41\x42\x43" rest"#;
        let mut src = Scanner::new(input);

        let (_span, value) = src.read_c_string('"', input).unwrap();
        assert_eq!(value, "ABC");
    }

    #[test]
    fn test_source_read_rust_raw_string() {
        let input = r#""hello world" rest"#;
        let mut src = Scanner::new(input);

        // Simulate: caller consumed 'r', hashes=0
        let span = src.read_rust_raw_string(0).unwrap();
        assert_eq!(&input[span], "hello world");
    }

    #[test]
    fn test_source_read_rust_raw_string_with_hashes() {
        // Input: "hello"world"# (what remains after caller consumed r#)
        let input = r##""hello"world"#"##;
        let mut src = Scanner::new(input);

        // Simulate: caller consumed 'r#', hashes=1
        let span = src.read_rust_raw_string(1).unwrap();
        assert_eq!(&input[span], r#"hello"world"#);
    }

    #[test]
    fn test_source_read_rust_raw_string_multiple_hashes() {
        // Input: "a"#b"## (what remains after caller consumed r##)
        let input = r###""a"#b"##"###;
        let mut src = Scanner::new(input);

        // Simulate: caller consumed 'r##', hashes=2
        let span = src.read_rust_raw_string(2).unwrap();
        assert_eq!(&input[span], r##"a"#b"##);
    }

    #[test]
    fn test_source_read_cpp_raw_string() {
        let input = r#""(hello world)" rest"#;
        let mut src = Scanner::new(input);

        // Simulate: caller consumed 'R'
        let span = src.read_cpp_raw_string(input).unwrap();
        assert_eq!(&input[span], "hello world");
    }

    #[test]
    fn test_source_read_cpp_raw_string_with_delimiter() {
        let input = r#""delim(hello)world)delim" rest"#;
        let mut src = Scanner::new(input);

        // Simulate: caller consumed 'R'
        let span = src.read_cpp_raw_string(input).unwrap();
        assert_eq!(&input[span], "hello)world");
    }

    #[test]
    fn test_source_read_exact() {
        let input = "<<= rest";
        let mut src = Scanner::new(input);

        assert!(src.read_exact("<<=").is_some());
        assert_eq!(src.peek(), Some(' '));
    }

    #[test]
    fn test_source_read_exact_no_match() {
        let input = "<< rest";
        let mut src = Scanner::new(input);

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
        let mut src = Scanner::new(input);
        let (idx, span) = src.read_one_of(OPS).unwrap();
        assert_eq!(idx, 0);
        assert_eq!(OPS[idx], "<<=");
        assert_eq!(&input[span], "<<=");

        let input = "<< rest";
        let mut src = Scanner::new(input);
        let (idx, span) = src.read_one_of(OPS).unwrap();
        assert_eq!(idx, 1);
        assert_eq!(OPS[idx], "<<");
        assert_eq!(&input[span], "<<");

        let input = "<= rest";
        let mut src = Scanner::new(input);
        let (idx, span) = src.read_one_of(OPS).unwrap();
        assert_eq!(idx, 2);
        assert_eq!(OPS[idx], "<=");
        assert_eq!(&input[span], "<=");

        let input = "< rest";
        let mut src = Scanner::new(input);
        let (idx, span) = src.read_one_of(OPS).unwrap();
        assert_eq!(idx, 3);
        assert_eq!(OPS[idx], "<");
        assert_eq!(&input[span], "<");

        // No match
        let input = "> rest";
        let mut src = Scanner::new(input);
        assert!(src.read_one_of(OPS).is_none());
        assert_eq!(src.offset(), 0); // Nothing consumed
    }

    #[test]
    fn test_source_starts_with() {
        let input = "hello world";
        let mut src = Scanner::new(input);

        assert!(src.starts_with("hello"));
        assert!(src.starts_with("hel"));
        assert!(!src.starts_with("world"));
        // Didn't consume anything
        assert_eq!(src.offset(), 0);
    }

    #[test]
    fn test_source_read_while() {
        let input = "aaabbbccc";
        let mut src = Scanner::new(input);

        let span = src.read_while(|c| c == 'a');
        assert_eq!(&input[span], "aaa");
        assert_eq!(src.peek(), Some('b'));
    }

    #[test]
    fn test_source_complete_lexer() {
        // Example of composing Scanner methods into a simple lexer
        let input = "foo + 123";
        let mut src = Scanner::new(input);
        let mut tokens = Vec::new();

        loop {
            src.skip_whitespace();
            if src.at_end() {
                break;
            }

            if let Some(span) = src.read_ident() {
                tokens.push(("ident", &input[span]));
            } else if let Some(span) = src.read_digits() {
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

//! Regex parser and byte-level Thompson NFA construction.
//!
//! Converts regex patterns to [`Nfa`] operating on byte values (0-255).
//! ASCII characters map to single transitions; multi-byte UTF-8 becomes byte chains.

use crate::automaton::Nfa;

/// Error from regex parsing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegexError {
    pub message: String,
    pub offset: usize,
}

impl std::fmt::Display for RegexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "regex error at {}: {}", self.offset, self.message)
    }
}

impl std::error::Error for RegexError {}

/// An NFA fragment: a start state and an end state (not yet connected).
struct Frag {
    start: usize,
    end: usize,
}

/// Parse a regex pattern and produce a byte-level NFA.
///
/// State 0 is the start state. The returned NFA's accept state is the last state added.
/// The caller determines which state is accepting (typically `nfa.num_states() - 1`
/// after construction, or tracked via the returned accept state).
///
/// Supported syntax:
/// - Literals: `a`, `\n`, `\t`, `\\`, `\xNN`
/// - Concatenation: `ab`
/// - Alternation: `a|b`
/// - Repetition: `*`, `+`, `?`
/// - Grouping: `(a|b)*`
/// - Character classes: `[a-z]`, `[^0-9]`, `[a-zA-Z_]`
/// - Dot: `.` (any byte except `\n`)
/// - Shorthand classes: `\d`, `\w`, `\s` and negations `\D`, `\W`, `\S`
pub fn regex_to_nfa(pattern: &str) -> Result<(Nfa, usize), RegexError> {
    let bytes = pattern.as_bytes();
    let mut parser = Parser { src: bytes, pos: 0 };
    let mut nfa = Nfa::new();
    // Reserve state 0 as the start state (subset_construction starts from 0).
    let state0 = nfa.add_state();
    debug_assert_eq!(state0, 0);
    let frag = parser.parse_alternation(&mut nfa)?;
    if parser.pos < parser.src.len() {
        return Err(RegexError {
            message: format!("unexpected character '{}'", parser.src[parser.pos] as char),
            offset: parser.pos,
        });
    }
    nfa.add_epsilon(0, frag.start);
    Ok((nfa, frag.end))
}

struct Parser<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<u8> {
        let b = self.src.get(self.pos).copied()?;
        self.pos += 1;
        Some(b)
    }

    fn expect(&mut self, ch: u8) -> Result<(), RegexError> {
        if self.next() == Some(ch) {
            Ok(())
        } else {
            Err(RegexError {
                message: format!("expected '{}'", ch as char),
                offset: self.pos.saturating_sub(1),
            })
        }
    }

    fn err(&self, msg: impl Into<String>) -> RegexError {
        RegexError { message: msg.into(), offset: self.pos }
    }

    // ========================================================================
    // Grammar: alternation = concat ('|' concat)*
    //          concat      = repetition*
    //          repetition  = atom ('*' | '+' | '?')?
    //          atom        = literal | '.' | '(' alternation ')' | '[' class ']'
    // ========================================================================

    fn parse_alternation(&mut self, nfa: &mut Nfa) -> Result<Frag, RegexError> {
        let mut frag = self.parse_concat(nfa)?;

        while self.peek() == Some(b'|') {
            self.next();
            let right = self.parse_concat(nfa)?;

            let start = nfa.add_state();
            let end = nfa.add_state();
            nfa.add_epsilon(start, frag.start);
            nfa.add_epsilon(start, right.start);
            nfa.add_epsilon(frag.end, end);
            nfa.add_epsilon(right.end, end);

            frag = Frag { start, end };
        }

        Ok(frag)
    }

    fn parse_concat(&mut self, nfa: &mut Nfa) -> Result<Frag, RegexError> {
        // Empty concat (e.g. in `(|b)` or at end)
        if matches!(self.peek(), None | Some(b'|') | Some(b')')) {
            let s = nfa.add_state();
            return Ok(Frag { start: s, end: s });
        }

        let mut frag = self.parse_repetition(nfa)?;

        while let Some(b) = self.peek() {
            if b == b'|' || b == b')' {
                break;
            }
            let right = self.parse_repetition(nfa)?;
            nfa.add_epsilon(frag.end, right.start);
            frag = Frag { start: frag.start, end: right.end };
        }

        Ok(frag)
    }

    fn parse_repetition(&mut self, nfa: &mut Nfa) -> Result<Frag, RegexError> {
        let mut frag = self.parse_atom(nfa)?;

        match self.peek() {
            Some(b'*') => {
                self.next();
                let start = nfa.add_state();
                let end = nfa.add_state();
                nfa.add_epsilon(start, frag.start);
                nfa.add_epsilon(start, end);
                nfa.add_epsilon(frag.end, frag.start);
                nfa.add_epsilon(frag.end, end);
                frag = Frag { start, end };
            }
            Some(b'+') => {
                self.next();
                let start = nfa.add_state();
                let end = nfa.add_state();
                nfa.add_epsilon(start, frag.start);
                nfa.add_epsilon(frag.end, frag.start);
                nfa.add_epsilon(frag.end, end);
                frag = Frag { start, end };
            }
            Some(b'?') => {
                self.next();
                let start = nfa.add_state();
                let end = nfa.add_state();
                nfa.add_epsilon(start, frag.start);
                nfa.add_epsilon(start, end);
                nfa.add_epsilon(frag.end, end);
                frag = Frag { start, end };
            }
            _ => {}
        }

        Ok(frag)
    }

    fn parse_atom(&mut self, nfa: &mut Nfa) -> Result<Frag, RegexError> {
        match self.peek() {
            Some(b'(') => {
                self.next();
                let frag = self.parse_alternation(nfa)?;
                self.expect(b')')?;
                Ok(frag)
            }
            Some(b'[') => self.parse_char_class(nfa),
            Some(b'.') => {
                self.next();
                // Match any byte except \n
                let start = nfa.add_state();
                let end = nfa.add_state();
                for b in 0u32..256 {
                    if b != b'\n' as u32 {
                        nfa.add_transition(start, b, end);
                    }
                }
                Ok(Frag { start, end })
            }
            Some(b'\\') => {
                let (frag, _) = self.parse_escape_or_class(nfa)?;
                Ok(frag)
            }
            Some(b'*') | Some(b'+') | Some(b'?') => {
                Err(self.err("quantifier without preceding element"))
            }
            Some(b) if b != b')' && b != b']' => {
                self.next();
                // For ASCII this is one byte. For multi-byte UTF-8 in the pattern source,
                // we need to find the full character.
                let ch = b as char;
                if ch.is_ascii() {
                    Ok(byte_chain(nfa, &[b]))
                } else {
                    // Back up and re-read as UTF-8 char
                    self.pos -= 1;
                    let s = std::str::from_utf8(&self.src[self.pos..])
                        .map_err(|_| self.err("invalid UTF-8"))?;
                    let ch = s.chars().next().unwrap();
                    self.pos += ch.len_utf8();
                    let mut buf = [0u8; 4];
                    let encoded = ch.encode_utf8(&mut buf);
                    Ok(byte_chain(nfa, encoded.as_bytes()))
                }
            }
            _ => Err(self.err("unexpected end of pattern")),
        }
    }

    /// Parse an escape sequence, returning the bytes it represents.
    /// For shorthand classes (\d, \w, \s, \D, \W, \S), returns an empty vec
    /// as a sentinel — handled specially by the caller... actually let's handle
    /// them inline. We'll return a Vec<u8> for literal escapes, but for classes
    /// we need a different path.
    fn parse_escape(&mut self) -> Result<Vec<u8>, RegexError> {
        self.expect(b'\\')?;
        match self.next() {
            Some(b'n') => Ok(vec![b'\n']),
            Some(b't') => Ok(vec![b'\t']),
            Some(b'r') => Ok(vec![b'\r']),
            Some(b'\\') => Ok(vec![b'\\']),
            Some(b'|') => Ok(vec![b'|']),
            Some(b'(') => Ok(vec![b'(']),
            Some(b')') => Ok(vec![b')']),
            Some(b'[') => Ok(vec![b'[']),
            Some(b']') => Ok(vec![b']']),
            Some(b'*') => Ok(vec![b'*']),
            Some(b'+') => Ok(vec![b'+']),
            Some(b'?') => Ok(vec![b'?']),
            Some(b'.') => Ok(vec![b'.']),
            Some(b'^') => Ok(vec![b'^']),
            Some(b'$') => Ok(vec![b'$']),
            Some(b'{') => Ok(vec![b'{']),
            Some(b'-') => Ok(vec![b'-']),
            Some(b'}') => Ok(vec![b'}']),
            Some(b'x') => {
                let h1 = self.next().ok_or_else(|| self.err("expected hex digit"))?;
                let h2 = self.next().ok_or_else(|| self.err("expected hex digit"))?;
                let v = hex_val(h1).ok_or_else(|| RegexError {
                    message: "invalid hex digit".into(),
                    offset: self.pos - 2,
                })?
                    * 16
                    + hex_val(h2).ok_or_else(|| RegexError {
                        message: "invalid hex digit".into(),
                        offset: self.pos - 1,
                    })?;
                Ok(vec![v])
            }
            Some(c) => Err(RegexError {
                message: format!("unknown escape '\\{}'", c as char),
                offset: self.pos - 1,
            }),
            None => Err(self.err("unexpected end after '\\'"))
        }
    }

    /// Parse escape that may be a shorthand class — returns byte set as a Frag.
    fn parse_escape_or_class(&mut self, nfa: &mut Nfa) -> Result<(Frag, Option<Vec<u8>>), RegexError> {
        let save = self.pos;
        // Check if it's a shorthand class
        if self.pos + 1 < self.src.len() && self.src[self.pos] == b'\\' {
            match self.src[self.pos + 1] {
                b'd' | b'D' | b'w' | b'W' | b's' | b'S' => {
                    let neg = self.src[self.pos + 1].is_ascii_uppercase();
                    let which = self.src[self.pos + 1].to_ascii_lowercase();
                    self.pos += 2;

                    let set = match which {
                        b'd' => byte_set_range(b'0', b'9'),
                        b'w' => {
                            let mut s = byte_set_range(b'a', b'z');
                            s.extend(byte_set_range(b'A', b'Z'));
                            s.extend(byte_set_range(b'0', b'9'));
                            s.push(b'_');
                            s
                        }
                        b's' => vec![b' ', b'\t', b'\n', b'\r', 0x0C, 0x0B],
                        _ => unreachable!(),
                    };

                    let start = nfa.add_state();
                    let end = nfa.add_state();
                    if neg {
                        for b in 0u8..=255 {
                            if !set.contains(&b) {
                                nfa.add_transition(start, b as u32, end);
                            }
                        }
                    } else {
                        for &b in &set {
                            nfa.add_transition(start, b as u32, end);
                        }
                    }
                    return Ok((Frag { start, end }, None));
                }
                _ => {}
            }
        }
        // Not a class, parse as literal escape
        self.pos = save;
        let bytes = self.parse_escape()?;
        let frag = byte_chain(nfa, &bytes);
        Ok((frag, Some(bytes)))
    }

    fn parse_char_class(&mut self, nfa: &mut Nfa) -> Result<Frag, RegexError> {
        self.expect(b'[')?;
        let negated = self.peek() == Some(b'^');
        if negated {
            self.next();
        }

        let mut bytes_in_class = Vec::new();

        // First char can be ']' as literal
        if self.peek() == Some(b']') {
            bytes_in_class.push(b']');
            self.next();
        }

        while self.peek() != Some(b']') {
            if self.peek().is_none() {
                return Err(self.err("unterminated character class"));
            }

            let byte = if self.peek() == Some(b'\\') {
                // Handle shorthand classes inside char class
                if self.pos + 1 < self.src.len() {
                    match self.src[self.pos + 1] {
                        b'd' => { self.pos += 2; bytes_in_class.extend(byte_set_range(b'0', b'9')); continue; }
                        b'w' => {
                            self.pos += 2;
                            bytes_in_class.extend(byte_set_range(b'a', b'z'));
                            bytes_in_class.extend(byte_set_range(b'A', b'Z'));
                            bytes_in_class.extend(byte_set_range(b'0', b'9'));
                            bytes_in_class.push(b'_');
                            continue;
                        }
                        b's' => { self.pos += 2; bytes_in_class.extend(&[b' ', b'\t', b'\n', b'\r', 0x0C, 0x0B]); continue; }
                        b'D' => {
                            self.pos += 2;
                            let digits = byte_set_range(b'0', b'9');
                            for b in 0u8..=255 { if !digits.contains(&b) { bytes_in_class.push(b); } }
                            continue;
                        }
                        b'W' => {
                            self.pos += 2;
                            let word: Vec<u8> = {
                                let mut s = byte_set_range(b'a', b'z');
                                s.extend(byte_set_range(b'A', b'Z'));
                                s.extend(byte_set_range(b'0', b'9'));
                                s.push(b'_');
                                s
                            };
                            for b in 0u8..=255 { if !word.contains(&b) { bytes_in_class.push(b); } }
                            continue;
                        }
                        b'S' => {
                            self.pos += 2;
                            let ws = [b' ', b'\t', b'\n', b'\r', 0x0Cu8, 0x0B];
                            for b in 0u8..=255 { if !ws.contains(&b) { bytes_in_class.push(b); } }
                            continue;
                        }
                        _ => {}
                    }
                }
                let esc = self.parse_escape()?;
                esc[0]
            } else {
                let b = self.next().unwrap();
                b
            };

            // Check for range: a-z
            if self.peek() == Some(b'-') && self.pos + 1 < self.src.len() && self.src[self.pos + 1] != b']' {
                self.next(); // consume '-'
                let end_byte = if self.peek() == Some(b'\\') {
                    let esc = self.parse_escape()?;
                    esc[0]
                } else {
                    self.next().ok_or_else(|| self.err("unterminated range"))?
                };
                if byte > end_byte {
                    return Err(RegexError {
                        message: format!("invalid range {}-{}", byte as char, end_byte as char),
                        offset: self.pos,
                    });
                }
                bytes_in_class.extend(byte_set_range(byte, end_byte));
            } else {
                bytes_in_class.push(byte);
            }
        }

        self.expect(b']')?;

        let start = nfa.add_state();
        let end = nfa.add_state();

        if negated {
            for b in 0u8..=255 {
                if !bytes_in_class.contains(&b) {
                    nfa.add_transition(start, b as u32, end);
                }
            }
        } else {
            // Deduplicate
            bytes_in_class.sort();
            bytes_in_class.dedup();
            for &b in &bytes_in_class {
                nfa.add_transition(start, b as u32, end);
            }
        }

        Ok(Frag { start, end })
    }
}

// Helper: build NFA fragment for a byte chain
fn byte_chain(nfa: &mut Nfa, bytes: &[u8]) -> Frag {
    assert!(!bytes.is_empty());
    let start = nfa.add_state();
    let mut prev = start;
    for &b in bytes {
        let next = nfa.add_state();
        nfa.add_transition(prev, b as u32, next);
        prev = next;
    }
    Frag { start, end: prev }
}

fn byte_set_range(lo: u8, hi: u8) -> Vec<u8> {
    (lo..=hi).collect()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::automaton;

    fn matches(pattern: &str, input: &[u8]) -> bool {
        let (nfa, accept) = regex_to_nfa(pattern).unwrap();
        let dfa = automaton::subset_construction(&nfa);
        // Walk DFA
        let mut state = 0usize;
        for &b in input {
            let mut next = None;
            for &(sym, target) in &dfa.transitions[state] {
                if sym == b as u32 {
                    next = Some(target);
                    break;
                }
            }
            match next {
                Some(s) => state = s,
                None => return false,
            }
        }
        // Check if current DFA state contains the NFA accept state
        dfa.nfa_sets[state].contains(&accept)
    }

    #[test]
    fn test_literal() {
        assert!(matches("abc", b"abc"));
        assert!(!matches("abc", b"ab"));
        assert!(!matches("abc", b"abcd"));
    }

    #[test]
    fn test_alternation() {
        assert!(matches("a|b", b"a"));
        assert!(matches("a|b", b"b"));
        assert!(!matches("a|b", b"c"));
        assert!(!matches("a|b", b"ab"));
    }

    #[test]
    fn test_star() {
        assert!(matches("a*", b""));
        assert!(matches("a*", b"a"));
        assert!(matches("a*", b"aaa"));
        assert!(!matches("a*", b"b"));
    }

    #[test]
    fn test_plus() {
        assert!(!matches("a+", b""));
        assert!(matches("a+", b"a"));
        assert!(matches("a+", b"aaa"));
    }

    #[test]
    fn test_question() {
        assert!(matches("a?", b""));
        assert!(matches("a?", b"a"));
        assert!(!matches("a?", b"aa"));
    }

    #[test]
    fn test_grouping() {
        assert!(matches("(ab)+", b"ab"));
        assert!(matches("(ab)+", b"abab"));
        assert!(!matches("(ab)+", b""));
        assert!(!matches("(ab)+", b"a"));
    }

    #[test]
    fn test_char_class() {
        assert!(matches("[abc]", b"a"));
        assert!(matches("[abc]", b"b"));
        assert!(matches("[abc]", b"c"));
        assert!(!matches("[abc]", b"d"));
    }

    #[test]
    fn test_char_class_range() {
        assert!(matches("[a-z]", b"a"));
        assert!(matches("[a-z]", b"m"));
        assert!(matches("[a-z]", b"z"));
        assert!(!matches("[a-z]", b"A"));
        assert!(!matches("[a-z]", b"0"));
    }

    #[test]
    fn test_char_class_negated() {
        assert!(!matches("[^a-z]", b"a"));
        assert!(matches("[^a-z]", b"0"));
        assert!(matches("[^a-z]", b"A"));
    }

    #[test]
    fn test_dot() {
        assert!(matches(".", b"a"));
        assert!(matches(".", b"0"));
        assert!(!matches(".", b"\n"));
        assert!(!matches(".", b""));
    }

    #[test]
    fn test_escape() {
        assert!(matches(r"\n", b"\n"));
        assert!(matches(r"\t", b"\t"));
        assert!(matches(r"\\", b"\\"));
        assert!(matches(r"\x41", b"A"));
    }

    #[test]
    fn test_complex_pattern() {
        // Identifier: [a-zA-Z_][a-zA-Z0-9_]*
        assert!(matches("[a-zA-Z_][a-zA-Z0-9_]*", b"foo"));
        assert!(matches("[a-zA-Z_][a-zA-Z0-9_]*", b"_bar"));
        assert!(matches("[a-zA-Z_][a-zA-Z0-9_]*", b"x1"));
        assert!(!matches("[a-zA-Z_][a-zA-Z0-9_]*", b"1x"));
        assert!(!matches("[a-zA-Z_][a-zA-Z0-9_]*", b""));
    }

    #[test]
    fn test_shorthand_digit() {
        assert!(matches(r"\d+", b"123"));
        assert!(!matches(r"\d+", b"abc"));
        assert!(!matches(r"\d+", b""));
    }

    #[test]
    fn test_shorthand_word() {
        assert!(matches(r"\w+", b"hello_123"));
        assert!(!matches(r"\w+", b""));
    }

    #[test]
    fn test_shorthand_space() {
        assert!(matches(r"\s+", b" \t\n"));
        assert!(!matches(r"\s+", b"a"));
    }

    #[test]
    fn test_escaped_metachar() {
        assert!(matches(r"\.", b"."));
        assert!(!matches(r"\.", b"a"));
        assert!(matches(r"\*", b"*"));
        assert!(matches(r"\+", b"+"));
    }

    #[test]
    fn test_empty_alternation() {
        assert!(matches("(|a)", b""));
        assert!(matches("(|a)", b"a"));
    }
}

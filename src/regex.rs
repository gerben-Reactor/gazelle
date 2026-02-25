//! Regex parser and byte-level Thompson NFA construction.
//!
//! Converts regex patterns to [`Nfa`] operating on byte values (0-255).
//! ASCII characters map to single transitions; multi-byte UTF-8 becomes byte chains.
//!
//! The parser is generated from `grammars/regex.gzl` using the CLI.
//!
//! To regenerate `regex_generated.rs`:
//! ```bash
//! cargo run -- --rust grammars/regex.gzl > src/regex_generated.rs
//! ```

#![allow(dead_code)]

use crate as gazelle;
use crate::automaton::Nfa;

// ============================================================================
// Generated parser
// ============================================================================

include!("regex_generated.rs");

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

#[derive(Debug, Clone, Copy)]
enum Shorthand {
    Digit, Word, Space, NotDigit, NotWord, NotSpace,
}

impl Shorthand {
    fn byte_set(&self) -> Vec<u8> {
        match self {
            Shorthand::Digit => (b'0'..=b'9').collect(),
            Shorthand::Word => {
                let mut s: Vec<u8> = (b'a'..=b'z').collect();
                s.extend(b'A'..=b'Z');
                s.extend(b'0'..=b'9');
                s.push(b'_');
                s
            }
            Shorthand::Space => vec![b' ', b'\t', b'\n', b'\r', 0x0C, 0x0B],
            Shorthand::NotDigit => {
                let set = Shorthand::Digit.byte_set();
                (0u8..=255).filter(|b| !set.contains(b)).collect()
            }
            Shorthand::NotWord => {
                let set = Shorthand::Word.byte_set();
                (0u8..=255).filter(|b| !set.contains(b)).collect()
            }
            Shorthand::NotSpace => {
                let set = Shorthand::Space.byte_set();
                (0u8..=255).filter(|b| !set.contains(b)).collect()
            }
        }
    }
}

// ============================================================================
// NFA builder implementing Actions
// ============================================================================

/// An NFA fragment: a start state and an end state (not yet connected).
#[derive(Debug)]
struct Frag {
    start: usize,
    end: usize,
}

struct NfaBuilder {
    nfa: Nfa,
}

impl NfaBuilder {
    fn byte_frag(&mut self, b: u8) -> Frag {
        let start = self.nfa.add_state();
        let end = self.nfa.add_state();
        self.nfa.add_transition(start, b as u32, end);
        Frag { start, end }
    }

    fn byte_set_frag(&mut self, bytes: &[u8]) -> Frag {
        let start = self.nfa.add_state();
        let end = self.nfa.add_state();
        for &b in bytes {
            self.nfa.add_transition(start, b as u32, end);
        }
        Frag { start, end }
    }
}

impl Types for NfaBuilder {
    type Error = RegexError;
    type Char = u8;
    type Shorthand = Shorthand;
    type Regex = Frag;
    type Concat = Frag;
    type Repetition = Frag;
    type Atom = Frag;
    type CharClass = Frag;
    type ClassItem = Vec<u8>;
    type ClassChar = u8;
}

impl From<crate::ParseError> for RegexError {
    fn from(e: crate::ParseError) -> Self {
        RegexError { message: format!("{:?}", e), offset: 0 }
    }
}

impl gazelle::Action<Regex<Self>> for NfaBuilder {
    fn build(&mut self, node: Regex<Self>) -> Result<Frag, RegexError> {
        let Regex::Regex(alts) = node;
        let mut iter = alts.into_iter();
        let mut frag = iter.next().unwrap();
        for alt in iter {
            let start = self.nfa.add_state();
            let end = self.nfa.add_state();
            self.nfa.add_epsilon(start, frag.start);
            self.nfa.add_epsilon(start, alt.start);
            self.nfa.add_epsilon(frag.end, end);
            self.nfa.add_epsilon(alt.end, end);
            frag = Frag { start, end };
        }
        Ok(frag)
    }
}

impl gazelle::Action<Concat<Self>> for NfaBuilder {
    fn build(&mut self, node: Concat<Self>) -> Result<Frag, RegexError> {
        let Concat::Concat(parts) = node;
        let mut iter = parts.into_iter();
        let mut frag = iter.next().unwrap();
        for part in iter {
            self.nfa.add_epsilon(frag.end, part.start);
            frag = Frag { start: frag.start, end: part.end };
        }
        Ok(frag)
    }
}

impl gazelle::Action<Repetition<Self>> for NfaBuilder {
    fn build(&mut self, node: Repetition<Self>) -> Result<Frag, RegexError> {
        Ok(match node {
            Repetition::Star(inner) => {
                let start = self.nfa.add_state();
                let end = self.nfa.add_state();
                self.nfa.add_epsilon(start, inner.start);
                self.nfa.add_epsilon(start, end);
                self.nfa.add_epsilon(inner.end, inner.start);
                self.nfa.add_epsilon(inner.end, end);
                Frag { start, end }
            }
            Repetition::Plus(inner) => {
                let start = self.nfa.add_state();
                let end = self.nfa.add_state();
                self.nfa.add_epsilon(start, inner.start);
                self.nfa.add_epsilon(inner.end, inner.start);
                self.nfa.add_epsilon(inner.end, end);
                Frag { start, end }
            }
            Repetition::Opt(inner) => {
                let start = self.nfa.add_state();
                let end = self.nfa.add_state();
                self.nfa.add_epsilon(start, inner.start);
                self.nfa.add_epsilon(start, end);
                self.nfa.add_epsilon(inner.end, end);
                Frag { start, end }
            }
            Repetition::Atom(a) => a,
        })
    }
}

impl gazelle::Action<Atom<Self>> for NfaBuilder {
    fn build(&mut self, node: Atom<Self>) -> Result<Frag, RegexError> {
        Ok(match node {
            Atom::Char(b) => self.byte_frag(b),
            Atom::Dash => self.byte_frag(b'-'),
            Atom::Caret => self.byte_frag(b'^'),
            Atom::Rbracket => self.byte_frag(b']'),
            Atom::Dot => {
                let start = self.nfa.add_state();
                let end = self.nfa.add_state();
                for b in 0u32..256 {
                    if b != b'\n' as u32 {
                        self.nfa.add_transition(start, b, end);
                    }
                }
                Frag { start, end }
            }
            Atom::Shorthand(s) => self.byte_set_frag(&s.byte_set()),
            Atom::Group(r) => r,
            Atom::Class(c) => c,
        })
    }
}

impl gazelle::Action<CharClass<Self>> for NfaBuilder {
    fn build(&mut self, node: CharClass<Self>) -> Result<Frag, RegexError> {
        let CharClass::Class(negated, items) = node;
        let mut bytes: Vec<u8> = items.into_iter().flatten().collect();
        if negated.is_some() {
            let set: std::collections::HashSet<u8> = bytes.drain(..).collect();
            bytes = (0u8..=255).filter(|b| !set.contains(b)).collect();
        } else {
            bytes.sort();
            bytes.dedup();
        }
        Ok(self.byte_set_frag(&bytes))
    }
}

impl gazelle::Action<ClassItem<Self>> for NfaBuilder {
    fn build(&mut self, node: ClassItem<Self>) -> Result<Vec<u8>, RegexError> {
        Ok(match node {
            ClassItem::Range(lo, hi) => {
                if lo > hi {
                    return Err(RegexError {
                        message: format!("invalid range {}-{}", lo as char, hi as char),
                        offset: 0,
                    });
                }
                (lo..=hi).collect()
            }
            ClassItem::Char(b) => vec![b],
            ClassItem::Shorthand(s) => s.byte_set(),
        })
    }
}

impl gazelle::Action<ClassChar<Self>> for NfaBuilder {
    fn build(&mut self, node: ClassChar<Self>) -> Result<u8, RegexError> {
        Ok(match node {
            ClassChar::Char(b) => b,
            ClassChar::Dot => b'.',
            ClassChar::Star => b'*',
            ClassChar::Plus => b'+',
            ClassChar::Question => b'?',
            ClassChar::Pipe => b'|',
            ClassChar::Lparen => b'(',
            ClassChar::Rparen => b')',
            ClassChar::Caret => b'^',
            ClassChar::Dash => b'-',
        })
    }
}

// ============================================================================
// Stateless lexer
// ============================================================================

fn lex_regex(input: &[u8]) -> Result<Vec<Terminal<NfaBuilder>>, RegexError> {
    let mut tokens = Vec::new();
    let mut pos = 0;

    while pos < input.len() {
        let b = input[pos];
        let tok = match b {
            b'*' => { pos += 1; Terminal::Star }
            b'+' => { pos += 1; Terminal::Plus }
            b'?' => { pos += 1; Terminal::Question }
            b'.' => { pos += 1; Terminal::Dot }
            b'|' => { pos += 1; Terminal::Pipe }
            b'(' => { pos += 1; Terminal::Lparen }
            b')' => { pos += 1; Terminal::Rparen }
            b'[' => { pos += 1; Terminal::Lbracket }
            b']' => { pos += 1; Terminal::Rbracket }
            b'^' => { pos += 1; Terminal::Caret }
            b'-' => { pos += 1; Terminal::Dash }
            b'\\' => {
                pos += 1;
                let c = *input.get(pos).ok_or_else(|| RegexError {
                    message: "unexpected end after '\\'".into(),
                    offset: pos,
                })?;
                pos += 1;
                match c {
                    b'd' => Terminal::Shorthand(Shorthand::Digit),
                    b'D' => Terminal::Shorthand(Shorthand::NotDigit),
                    b'w' => Terminal::Shorthand(Shorthand::Word),
                    b'W' => Terminal::Shorthand(Shorthand::NotWord),
                    b's' => Terminal::Shorthand(Shorthand::Space),
                    b'S' => Terminal::Shorthand(Shorthand::NotSpace),
                    b'n' => Terminal::Char(b'\n'),
                    b't' => Terminal::Char(b'\t'),
                    b'r' => Terminal::Char(b'\r'),
                    b'x' => {
                        let h1 = *input.get(pos).ok_or_else(|| RegexError {
                            message: "expected hex digit".into(), offset: pos,
                        })?;
                        let h2 = *input.get(pos + 1).ok_or_else(|| RegexError {
                            message: "expected hex digit".into(), offset: pos + 1,
                        })?;
                        let v = hex_val(h1).ok_or_else(|| RegexError {
                            message: "invalid hex digit".into(), offset: pos,
                        })? * 16
                            + hex_val(h2).ok_or_else(|| RegexError {
                                message: "invalid hex digit".into(), offset: pos + 1,
                            })?;
                        pos += 2;
                        Terminal::Char(v)
                    }
                    b'\\' | b'|' | b'(' | b')' | b'[' | b']'
                    | b'*' | b'+' | b'?' | b'.' | b'^' | b'$'
                    | b'{' | b'}' | b'-' => Terminal::Char(c),
                    _ => return Err(RegexError {
                        message: format!("unknown escape '\\{}'", c as char),
                        offset: pos - 1,
                    }),
                }
            }
            _ => {
                // Regular byte — could be multi-byte UTF-8 start
                if b.is_ascii() {
                    pos += 1;
                    Terminal::Char(b)
                } else {
                    // Multi-byte UTF-8: read the full character, emit byte chain as CHAR tokens
                    let s = std::str::from_utf8(&input[pos..])
                        .map_err(|_| RegexError {
                            message: "invalid UTF-8".into(), offset: pos,
                        })?;
                    let ch = s.chars().next().unwrap();
                    let len = ch.len_utf8();
                    for i in 0..len {
                        tokens.push(Terminal::Char(input[pos + i]));
                    }
                    pos += len;
                    continue;
                }
            }
        };
        tokens.push(tok);
    }

    Ok(tokens)
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

// ============================================================================
// Public API
// ============================================================================

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
    let tokens = lex_regex(pattern.as_bytes())?;

    let mut builder = NfaBuilder { nfa: Nfa::new() };
    let state0 = builder.nfa.add_state();
    debug_assert_eq!(state0, 0);

    let mut parser = Parser::<NfaBuilder>::new();
    for tok in tokens {
        parser.push(tok, &mut builder)?;
    }
    let frag = parser.finish(&mut builder).map_err(|(_, e)| e)?;

    builder.nfa.add_epsilon(0, frag.start);
    Ok((builder.nfa, frag.end))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::automaton;

    fn matches(pattern: &str, input: &[u8]) -> bool {
        let (nfa, accept) = regex_to_nfa(pattern).unwrap();
        let (dfa, nfa_sets) = automaton::subset_construction(&nfa);
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
        nfa_sets[state].contains(&accept)
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

    // Note: empty alternatives like (|a) are not supported by the grammar.
    // The grammar requires at least one repetition per concat branch.
}

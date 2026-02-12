use gazelle::Precedence;

use crate::ast::Op;
use crate::grammar::{C11Terminal, CActions};

pub struct C11Lexer<'a> {
    input: &'a str,
    src: gazelle::lexer::Source<std::str::Chars<'a>>,
    pending_ident: Option<String>,
}

impl<'a> C11Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            src: gazelle::lexer::Source::from_str(input),
            pending_ident: None,
        }
    }

    /// Skip balanced parentheses (for __attribute__, __asm__, etc.)
    fn skip_balanced_parens(&mut self) {
        self.src.skip_whitespace();
        if self.src.peek() != Some('(') { return; }
        self.src.advance();
        let mut depth = 1u32;
        while depth > 0 {
            match self.src.peek() {
                Some('(') => { depth += 1; self.src.advance(); }
                Some(')') => { depth -= 1; self.src.advance(); }
                Some('"') => { let _ = self.src.read_string_raw('"'); }
                Some('\'') => { let _ = self.src.read_string_raw('\''); }
                Some(_) => { self.src.advance(); }
                None => break,
            }
        }
    }

    /// Read a C numeric literal: decimal, hex, octal, float, with suffixes.
    fn read_number(&mut self) {
        if self.src.peek() == Some('0') {
            self.src.advance();
            match self.src.peek() {
                Some('x' | 'X') => {
                    self.src.advance();
                    self.src.read_hex_digits();
                    // Hex float: 0x1.2p3
                    if self.src.peek() == Some('.') {
                        self.src.advance();
                        self.src.read_hex_digits();
                    }
                    if matches!(self.src.peek(), Some('p' | 'P')) {
                        self.src.advance();
                        if matches!(self.src.peek(), Some('+' | '-')) { self.src.advance(); }
                        self.src.read_digits();
                    }
                }
                Some('0'..='9') => { self.src.read_digits(); }
                _ => {} // just "0"
            }
        } else if self.src.peek() == Some('.') {
            // .123 float
            self.src.advance();
            self.src.read_digits();
        } else {
            self.src.read_digits();
        }
        // Decimal float: 123.456e7
        if self.src.peek() == Some('.') {
            self.src.advance();
            self.src.read_digits();
        }
        if matches!(self.src.peek(), Some('e' | 'E')) {
            self.src.advance();
            if matches!(self.src.peek(), Some('+' | '-')) { self.src.advance(); }
            self.src.read_digits();
        }
        // Suffixes: u, l, ll, ul, ull, f, etc.
        while matches!(self.src.peek(), Some('u' | 'U' | 'l' | 'L' | 'f' | 'F')) {
            self.src.advance();
        }
    }

    pub(crate) fn next(&mut self, actions: &mut CActions) -> Result<Option<C11Terminal<CActions>>, String> {
        if let Some(id) = self.pending_ident.take() {
            return Ok(Some(if actions.ctx.is_typedef(&id) {
                C11Terminal::TYPE
            } else {
                C11Terminal::VARIABLE
            }));
        }

        self.src.skip_whitespace();
        while self.src.skip_line_comment("//") || self.src.skip_block_comment("/*", "*/") {
            self.src.skip_whitespace();
        }

        if self.src.at_end() {
            return Ok(None);
        }

        // Identifier or keyword
        if let Some(span) = self.src.read_ident() {
            let s = &self.input[span];

            // C-style prefixed string/char literals: L, u, U, u8
            if matches!(s, "L" | "u" | "U" | "u8") {
                if self.src.peek() == Some('\'') {
                    let span = self.src.read_string_raw('\'').map_err(|e| e.to_string())?;
                    return Ok(Some(C11Terminal::CONSTANT(self.input[span].to_string())));
                } else if self.src.peek() == Some('"') {
                    let span = self.src.read_string_raw('"').map_err(|e| e.to_string())?;
                    return Ok(Some(C11Terminal::STRING_LITERAL(self.input[span].to_string())));
                }
            }

            // GCC extensions: skip or map to standard tokens
            match s {
                "__attribute__" | "__attribute" => {
                    self.skip_balanced_parens();
                    return self.next(actions);
                }
                "__asm__" | "__asm" | "asm" => {
                    self.skip_balanced_parens();
                    return self.next(actions);
                }
                "__extension__" => return self.next(actions),
                "__builtin_va_arg" => return Ok(Some(C11Terminal::BUILTIN_VA_ARG)),
                _ => {}
            }

            return Ok(Some(match s {
                "auto" => C11Terminal::AUTO,
                "break" => C11Terminal::BREAK,
                "case" => C11Terminal::CASE,
                "char" => C11Terminal::CHAR,
                "const" | "__const" | "__const__" => C11Terminal::CONST,
                "continue" => C11Terminal::CONTINUE,
                "default" => C11Terminal::DEFAULT,
                "do" => C11Terminal::DO,
                "double" => C11Terminal::DOUBLE,
                "else" => C11Terminal::ELSE,
                "enum" => C11Terminal::ENUM,
                "extern" => C11Terminal::EXTERN,
                "float" => C11Terminal::FLOAT,
                "for" => C11Terminal::FOR,
                "goto" => C11Terminal::GOTO,
                "if" => C11Terminal::IF,
                "inline" | "__inline" | "__inline__" => C11Terminal::INLINE,
                "int" => C11Terminal::INT,
                "long" => C11Terminal::LONG,
                "register" => C11Terminal::REGISTER,
                "restrict" | "__restrict" | "__restrict__" => C11Terminal::RESTRICT,
                "return" => C11Terminal::RETURN,
                "short" => C11Terminal::SHORT,
                "signed" | "__signed__" => C11Terminal::SIGNED,
                "sizeof" => C11Terminal::SIZEOF,
                "static" => C11Terminal::STATIC,
                "struct" => C11Terminal::STRUCT,
                "switch" => C11Terminal::SWITCH,
                "typedef" => C11Terminal::TYPEDEF,
                "union" => C11Terminal::UNION,
                "unsigned" => C11Terminal::UNSIGNED,
                "void" => C11Terminal::VOID,
                "volatile" | "__volatile__" | "__volatile" => C11Terminal::VOLATILE,
                "while" => C11Terminal::WHILE,
                "_Alignas" => C11Terminal::ALIGNAS,
                "_Alignof" => C11Terminal::ALIGNOF,
                "_Atomic" => C11Terminal::ATOMIC,
                "_Bool" => C11Terminal::BOOL,
                "_Complex" => C11Terminal::COMPLEX,
                "_Generic" => C11Terminal::GENERIC,
                "_Imaginary" => C11Terminal::IMAGINARY,
                "_Noreturn" | "__noreturn__" => C11Terminal::NORETURN,
                "_Static_assert" => C11Terminal::STATIC_ASSERT,
                "_Thread_local" => C11Terminal::THREAD_LOCAL,
                // GCC builtin types: emit as NAME+TYPE (like a typedef)
                "__builtin_va_list" => {
                    self.pending_ident = Some(s.to_string());
                    C11Terminal::NAME(s.to_string())
                }
                _ => {
                    self.pending_ident = Some(s.to_string());
                    C11Terminal::NAME(s.to_string())
                }
            }));
        }

        // Number literal (decimal, hex, octal, float, with suffixes)
        if let Some(c) = self.src.peek() {
            if c.is_ascii_digit() || (c == '.' && self.src.peek_n(1).is_some_and(|c| c.is_ascii_digit())) {
                let start = self.src.offset();
                self.read_number();
                return Ok(Some(C11Terminal::CONSTANT(self.input[start..self.src.offset()].to_string())));
            }
        }

        // String literal
        if self.src.peek() == Some('"') {
            let span = self.src.read_string_raw('"').map_err(|e| e.to_string())?;
            return Ok(Some(C11Terminal::STRING_LITERAL(self.input[span].to_string())));
        }

        // Character literal — convert to integer value
        if self.src.peek() == Some('\'') {
            let span = self.src.read_string_raw('\'').map_err(|e| e.to_string())?;
            let inner = &self.input[span];
            let val = if inner.starts_with('\\') {
                match inner.as_bytes().get(1) {
                    Some(b'n') => 10, Some(b't') => 9, Some(b'r') => 13,
                    Some(b'0') => 0, Some(b'\\') => 92, Some(b'\'') => 39,
                    Some(b'"') => 34, Some(b'a') => 7, Some(b'b') => 8,
                    Some(b'f') => 12, Some(b'v') => 11,
                    Some(b'x') => i64::from_str_radix(&inner[2..], 16).unwrap_or(0),
                    Some(c) if c.is_ascii_digit() => i64::from_str_radix(&inner[1..], 8).unwrap_or(0),
                    _ => inner.as_bytes().get(1).copied().unwrap_or(0) as i64,
                }
            } else {
                inner.as_bytes().first().copied().unwrap_or(0) as i64
            };
            return Ok(Some(C11Terminal::CONSTANT(val.to_string())));
        }

        // Single-char punctuation (no operator overloading)
        if let Some(c) = self.src.peek() {
            match c {
                '(' => { self.src.advance(); return Ok(Some(C11Terminal::LPAREN)); }
                ')' => { self.src.advance(); return Ok(Some(C11Terminal::RPAREN)); }
                '{' => { self.src.advance(); return Ok(Some(C11Terminal::LBRACE)); }
                '}' => { self.src.advance(); return Ok(Some(C11Terminal::RBRACE)); }
                '[' => { self.src.advance(); return Ok(Some(C11Terminal::LBRACK)); }
                ']' => { self.src.advance(); return Ok(Some(C11Terminal::RBRACK)); }
                ';' => { self.src.advance(); return Ok(Some(C11Terminal::SEMICOLON)); }
                ',' => { self.src.advance(); return Ok(Some(C11Terminal::COMMA)); }
                _ => {}
            }
        }

        // Multi-char operators (longest first)
        const MULTI_OPS: &[&str] = &[
            "...", "<<=", ">>=",
            "->", "++", "--",
            "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=",
            "||", "&&", "==", "!=", "<=", ">=", "<<", ">>",
        ];
        const MULTI_OP_INFO: &[Option<(Op, Precedence)>] = &[
            None,
            Some((Op::ShlAssign, Precedence::Right(1))),
            Some((Op::ShrAssign, Precedence::Right(1))),
            None, None, None,
            Some((Op::AddAssign, Precedence::Right(1))),
            Some((Op::SubAssign, Precedence::Right(1))),
            Some((Op::MulAssign, Precedence::Right(1))),
            Some((Op::DivAssign, Precedence::Right(1))),
            Some((Op::ModAssign, Precedence::Right(1))),
            Some((Op::BitAndAssign, Precedence::Right(1))),
            Some((Op::BitOrAssign, Precedence::Right(1))),
            Some((Op::BitXorAssign, Precedence::Right(1))),
            Some((Op::Or, Precedence::Left(3))),
            Some((Op::And, Precedence::Left(4))),
            Some((Op::Eq, Precedence::Left(8))),
            Some((Op::Ne, Precedence::Left(8))),
            Some((Op::Le, Precedence::Left(9))),
            Some((Op::Ge, Precedence::Left(9))),
            Some((Op::Shl, Precedence::Left(10))),
            Some((Op::Shr, Precedence::Left(10))),
        ];

        if let Some((idx, _)) = self.src.read_one_of(MULTI_OPS) {
            return Ok(Some(match idx {
                0 => C11Terminal::ELLIPSIS,
                3 => C11Terminal::PTR,
                4 => C11Terminal::INC,
                5 => C11Terminal::DEC,
                _ => {
                    let (op, prec) = MULTI_OP_INFO[idx].unwrap();
                    C11Terminal::BINOP(op, prec)
                }
            }));
        }

        // Single-char operators
        if let Some(c) = self.src.peek() {
            self.src.advance();
            return Ok(Some(match c {
                ':' => C11Terminal::COLON,
                '.' => C11Terminal::DOT,
                '~' => C11Terminal::TILDE,
                '!' => C11Terminal::BANG,
                '=' => C11Terminal::EQ(Precedence::Right(1)),
                '?' => C11Terminal::QUESTION(Precedence::Right(2)),
                '|' => C11Terminal::BINOP(Op::BitOr, Precedence::Left(5)),
                '^' => C11Terminal::BINOP(Op::BitXor, Precedence::Left(6)),
                '&' => C11Terminal::AMP(Precedence::Left(7)),
                '<' => C11Terminal::BINOP(Op::Lt, Precedence::Left(9)),
                '>' => C11Terminal::BINOP(Op::Gt, Precedence::Left(9)),
                '+' => C11Terminal::PLUS(Precedence::Left(11)),
                '-' => C11Terminal::MINUS(Precedence::Left(11)),
                '*' => C11Terminal::STAR(Precedence::Left(12)),
                '/' => C11Terminal::BINOP(Op::Div, Precedence::Left(12)),
                '%' => C11Terminal::BINOP(Op::Mod, Precedence::Left(12)),
                _ => return Err(format!("Unknown character: {}", c)),
            }));
        }

        Ok(None)
    }
}

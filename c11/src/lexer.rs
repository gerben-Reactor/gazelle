use gazelle::Precedence;
use gazelle::lexer::Scanner;

use crate::ast::Op;
use crate::grammar::{C11, CActions};

type T = C11::Terminal<CActions>;

/// Read the next C11 token from the scanner.
/// Skips GCC extensions (__attribute__, __asm__, __extension__) internally.
/// Returns `T::Name(s)` for identifiers — caller must follow up with TYPE/VARIABLE.
pub(crate) fn next_token(src: &mut Scanner<std::str::Chars<'_>>, input: &str) -> Result<T, String> {
    // Identifier or keyword
    if let Some(span) = src.read_ident() {
        let s = &input[span];

        // C-style prefixed string/char literals: L, u, U, u8
        if matches!(s, "L" | "u" | "U" | "u8") {
            if src.peek() == Some('\'') {
                let span = src.read_string_raw('\'').map_err(|e| e.to_string())?;
                return Ok(T::Constant(input[span].to_string()));
            } else if src.peek() == Some('"') {
                let span = src.read_string_raw('"').map_err(|e| e.to_string())?;
                return Ok(T::StringLiteral(input[span].to_string()));
            }
        }

        return Ok(keyword(s).unwrap_or_else(|| T::Name(s.to_string())));
    }

    // Number literal
    if let Some(c) = src.peek() {
        if c.is_ascii_digit() || (c == '.' && src.peek_n(1).is_some_and(|c| c.is_ascii_digit())) {
            let start = src.offset();
            read_number(src);
            return Ok(T::Constant(input[start..src.offset()].to_string()));
        }
    }

    // String literal
    if src.peek() == Some('"') {
        let span = src.read_string_raw('"').map_err(|e| e.to_string())?;
        return Ok(T::StringLiteral(input[span].to_string()));
    }

    // Character literal
    if src.peek() == Some('\'') {
        let span = src.read_string_raw('\'').map_err(|e| e.to_string())?;
        return Ok(T::Constant(char_literal_value(&input[span]).to_string()));
    }

    // Operators and punctuation
    if let Some(tok) = read_operator(src) {
        return Ok(tok);
    }

    Err(format!("Unknown character: {}", src.peek().unwrap()))
}

fn keyword(s: &str) -> Option<T> {
    Some(match s {
        "auto" => T::Auto,
        "break" => T::Break,
        "case" => T::Case,
        "char" => T::Char,
        "const" | "__const" | "__const__" => T::Const,
        "continue" => T::Continue,
        "default" => T::Default,
        "do" => T::Do,
        "double" => T::Double,
        "else" => T::Else,
        "enum" => T::Enum,
        "extern" => T::Extern,
        "float" => T::Float,
        "for" => T::For,
        "goto" => T::Goto,
        "if" => T::If,
        "inline" | "__inline" | "__inline__" => T::Inline,
        "int" => T::Int,
        "long" => T::Long,
        "register" => T::Register,
        "restrict" | "__restrict" | "__restrict__" => T::Restrict,
        "return" => T::Return,
        "short" => T::Short,
        "signed" | "__signed__" => T::Signed,
        "sizeof" => T::Sizeof,
        "static" => T::Static,
        "struct" => T::Struct,
        "switch" => T::Switch,
        "typedef" => T::Typedef,
        "union" => T::Union,
        "unsigned" => T::Unsigned,
        "void" => T::Void,
        "volatile" | "__volatile__" | "__volatile" => T::Volatile,
        "while" => T::While,
        "_Alignas" => T::Alignas,
        "_Alignof" => T::Alignof,
        "_Atomic" => T::Atomic,
        "_Bool" => T::Bool,
        "_Complex" => T::Complex,
        "_Generic" => T::Generic,
        "_Imaginary" => T::Imaginary,
        "_Noreturn" | "__noreturn__" => T::Noreturn,
        "_Float128" | "_Float64" | "_Float32" | "_Float64x" | "_Float32x" => T::Double,
        "_Static_assert" => T::StaticAssert,
        "_Thread_local" => T::ThreadLocal,
        "__builtin_va_arg" => T::BuiltinVaArg,
        _ => return None,
    })
}

fn read_operator(src: &mut Scanner<std::str::Chars<'_>>) -> Option<T> {
    // Longest first for maximal munch
    const OPS: &[(&str, fn() -> T)] = &[
        ("...", || T::Ellipsis),
        ("<<=", || T::Binop(Op::ShlAssign, Precedence::Right(1))),
        (">>=", || T::Binop(Op::ShrAssign, Precedence::Right(1))),
        ("->",  || T::Ptr),
        ("++",  || T::Inc),
        ("--",  || T::Dec),
        ("+=",  || T::Binop(Op::AddAssign, Precedence::Right(1))),
        ("-=",  || T::Binop(Op::SubAssign, Precedence::Right(1))),
        ("*=",  || T::Binop(Op::MulAssign, Precedence::Right(1))),
        ("/=",  || T::Binop(Op::DivAssign, Precedence::Right(1))),
        ("%=",  || T::Binop(Op::ModAssign, Precedence::Right(1))),
        ("&=",  || T::Binop(Op::BitAndAssign, Precedence::Right(1))),
        ("|=",  || T::Binop(Op::BitOrAssign, Precedence::Right(1))),
        ("^=",  || T::Binop(Op::BitXorAssign, Precedence::Right(1))),
        ("||",  || T::Binop(Op::Or, Precedence::Left(3))),
        ("&&",  || T::Binop(Op::And, Precedence::Left(4))),
        ("==",  || T::Binop(Op::Eq, Precedence::Left(8))),
        ("!=",  || T::Binop(Op::Ne, Precedence::Left(8))),
        ("<=",  || T::Binop(Op::Le, Precedence::Left(9))),
        (">=",  || T::Binop(Op::Ge, Precedence::Left(9))),
        ("<<",  || T::Binop(Op::Shl, Precedence::Left(10))),
        (">>",  || T::Binop(Op::Shr, Precedence::Left(10))),
        ("(",   || T::Lparen),
        (")",   || T::Rparen),
        ("{",   || T::Lbrace),
        ("}",   || T::Rbrace),
        ("[",   || T::Lbrack),
        ("]",   || T::Rbrack),
        (";",   || T::Semicolon),
        (",",   || T::Comma),
        (":",   || T::Colon),
        ("~",   || T::Tilde),
        ("!",   || T::Bang),
        ("=",   || T::Eq(Precedence::Right(1))),
        ("?",   || T::Question(Precedence::Right(2))),
        ("|",   || T::Binop(Op::BitOr, Precedence::Left(5))),
        ("^",   || T::Binop(Op::BitXor, Precedence::Left(6))),
        ("&",   || T::Amp(Precedence::Left(7))),
        ("<",   || T::Binop(Op::Lt, Precedence::Left(9))),
        (">",   || T::Binop(Op::Gt, Precedence::Left(9))),
        ("+",   || T::Plus(Precedence::Left(11))),
        ("-",   || T::Minus(Precedence::Left(11))),
        ("*",   || T::Star(Precedence::Left(12))),
        ("/",   || T::Binop(Op::Div, Precedence::Left(12))),
        ("%",   || T::Binop(Op::Mod, Precedence::Left(12))),
        (".",   || T::Dot),
    ];
    for &(s, f) in OPS {
        if src.read_exact(s).is_some() {
            return Some(f());
        }
    }
    None
}

/// Skip balanced parentheses (for __attribute__, __asm__, etc.)
pub(crate) fn skip_balanced_parens(src: &mut Scanner<std::str::Chars<'_>>) {
    src.skip_whitespace();
    if src.peek() != Some('(') { return; }
    src.advance();
    let mut depth = 1u32;
    while depth > 0 {
        match src.peek() {
            Some('(') => { depth += 1; src.advance(); }
            Some(')') => { depth -= 1; src.advance(); }
            Some('"') => { let _ = src.read_string_raw('"'); }
            Some('\'') => { let _ = src.read_string_raw('\''); }
            Some(_) => { src.advance(); }
            None => break,
        }
    }
}

/// Read a C numeric literal: decimal, hex, octal, float, with suffixes.
fn read_number(src: &mut Scanner<std::str::Chars<'_>>) {
    if src.peek() == Some('0') {
        src.advance();
        match src.peek() {
            Some('x' | 'X') => {
                src.advance();
                src.read_hex_digits();
                if src.peek() == Some('.') {
                    src.advance();
                    src.read_hex_digits();
                }
                if matches!(src.peek(), Some('p' | 'P')) {
                    src.advance();
                    if matches!(src.peek(), Some('+' | '-')) { src.advance(); }
                    src.read_digits();
                }
            }
            Some('0'..='9') => { src.read_digits(); }
            _ => {}
        }
    } else if src.peek() == Some('.') {
        src.advance();
        src.read_digits();
    } else {
        src.read_digits();
    }
    if src.peek() == Some('.') {
        src.advance();
        src.read_digits();
    }
    if matches!(src.peek(), Some('e' | 'E')) {
        src.advance();
        if matches!(src.peek(), Some('+' | '-')) { src.advance(); }
        src.read_digits();
    }
    while matches!(src.peek(), Some('u' | 'U' | 'l' | 'L' | 'f' | 'F')) {
        src.advance();
    }
}

/// Evaluate a C character literal to its integer value.
fn char_literal_value(inner: &str) -> i64 {
    if inner.starts_with('\\') {
        match inner.as_bytes().get(1) {
            Some(b'n') => 10, Some(b't') => 9, Some(b'r') => 13,
            Some(b'\\') => 92, Some(b'\'') => 39,
            Some(b'"') => 34, Some(b'a') => 7, Some(b'b') => 8,
            Some(b'f') => 12, Some(b'v') => 11,
            Some(b'x') => i64::from_str_radix(&inner[2..], 16).unwrap_or(0),
            Some(c) if c.is_ascii_digit() => i64::from_str_radix(&inner[1..], 8).unwrap_or(0),
            _ => inner.as_bytes().get(1).copied().unwrap_or(0) as i64,
        }
    } else {
        inner.as_bytes().first().copied().unwrap_or(0) as i64
    }
}

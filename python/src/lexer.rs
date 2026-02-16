use gazelle::Precedence;
use gazelle::lexer::Source;

use crate::grammar::{PythonParser, PythonTerminal, PyActions, AugOp, CompOp, BinOp};

type Tok = PythonTerminal<PyActions>;
type Parser = PythonParser<PyActions>;

macro_rules! push {
    ($parser:expr, $actions:expr, $tok:expr) => {
        $parser.push($tok, $actions).map_err(|e| {
            format!("Parse error: {}", $parser.format_error(&e))
        })?
    };
}

pub(crate) fn lex(input: &str, parser: &mut Parser, actions: &mut PyActions) -> Result<(), String> {
    let mut src = Source::from_str(input);
    let mut indent_stack: Vec<usize> = vec![0];
    let mut bracket_depth: usize = 0;

    process_line_start(&mut src, &mut indent_stack, parser, actions)?;

    loop {
        // Skip horizontal whitespace, comments, and line continuations
        loop {
            src.skip_while(|c| c == ' ' || c == '\t');
            if src.peek() == Some('#') {
                src.read_until_any(&['\n']);
            }
            if src.peek() == Some('\\') && src.peek_n(1) == Some('\n') {
                src.advance();
                src.advance();
                continue;
            }
            break;
        }

        // Newline
        if matches!(src.peek(), Some('\n' | '\r')) {
            if src.peek() == Some('\r') { src.advance(); }
            if src.peek() == Some('\n') { src.advance(); }
            if bracket_depth > 0 {
                continue;
            }
            push!(parser, actions, Tok::Newline);
            process_line_start(&mut src, &mut indent_stack, parser, actions)?;
            continue;
        }

        // EOF
        if src.at_end() {
            return Ok(());
        }

        // Identifier or keyword
        if let Some(span) = src.read_ident() {
            let s = &input[span];
            if is_string_prefix(s) && matches!(src.peek(), Some('\'' | '"')) {
                let str_start = src.offset() - s.len();
                read_string(&mut src)?;
                push!(parser, actions, Tok::String(input[str_start..src.offset()].to_string()));
                continue;
            }
            push!(parser, actions, match s {
                "False" => Tok::False,
                "None" => Tok::None,
                "True" => Tok::True,
                "and" => Tok::And,
                "as" => Tok::As,
                "assert" => Tok::Assert,
                "async" => Tok::Async,
                "await" => Tok::Await,
                "break" => Tok::Break,
                "class" => Tok::Class,
                "continue" => Tok::Continue,
                "def" => Tok::Def,
                "del" => Tok::Del,
                "elif" => Tok::Elif,
                "else" => Tok::Else,
                "except" => Tok::Except,
                "finally" => Tok::Finally,
                "for" => Tok::For,
                "from" => Tok::From,
                "global" => Tok::Global,
                "if" => Tok::If,
                "import" => Tok::Import,
                "in" => Tok::In,
                "is" => Tok::Is,
                "lambda" => Tok::Lambda,
                "nonlocal" => Tok::Nonlocal,
                "not" => Tok::Not,
                "or" => Tok::Or,
                "pass" => Tok::Pass,
                "raise" => Tok::Raise,
                "return" => Tok::Return,
                "try" => Tok::Try,
                "while" => Tok::While,
                "with" => Tok::With,
                "yield" => Tok::Yield,
                _ => Tok::Name(s.to_string()),
            });
            continue;
        }

        // Number literal
        if src.peek().is_some_and(|c| c.is_ascii_digit())
            || (src.peek() == Some('.') && src.peek_n(1).is_some_and(|c| c.is_ascii_digit()))
        {
            let start = src.offset();
            read_number(&mut src);
            push!(parser, actions, Tok::Number(input[start..src.offset()].to_string()));
            continue;
        }

        // String literal (no prefix)
        if matches!(src.peek(), Some('\'' | '"')) {
            let start = src.offset();
            read_string(&mut src)?;
            push!(parser, actions, Tok::String(input[start..src.offset()].to_string()));
            continue;
        }

        // Brackets
        match src.peek() {
            Some('(' | '[' | '{') => {
                let c = src.peek().unwrap();
                src.advance();
                bracket_depth += 1;
                push!(parser, actions, match c {
                    '(' => Tok::Lparen, '[' => Tok::Lbrack, _ => Tok::Lbrace,
                });
                continue;
            }
            Some(')' | ']' | '}') => {
                let c = src.peek().unwrap();
                src.advance();
                bracket_depth = bracket_depth.saturating_sub(1);
                push!(parser, actions, match c {
                    ')' => Tok::Rparen, ']' => Tok::Rbrack, _ => Tok::Rbrace,
                });
                continue;
            }
            _ => {}
        }

        // Operators (longest first)
        if let Some((idx, _)) = src.read_one_of(&OPS.map(|(s, _)| s)) {
            push!(parser, actions, OPS[idx].1());
            continue;
        }

        if !src.at_end() {
            src.advance();
            return Err(format!("unexpected character: {:?}", &input[src.offset()-1..src.offset()]));
        }
    }
}

/// Skip blank lines and comments, measure indentation, push INDENT/DEDENTs.
fn process_line_start(
    src: &mut Source<std::str::Chars<'_>>,
    indent_stack: &mut Vec<usize>,
    parser: &mut Parser,
    actions: &mut PyActions,
) -> Result<(), String> {
    loop {
        let start = src.offset();
        src.skip_while(|c| c == ' ' || c == '\t');
        let indent = src.offset() - start;

        if src.peek() == Some('#') { src.read_until_any(&['\n']); }
        match src.peek() {
            Some('\r' | '\n') => {
                if src.peek() == Some('\r') { src.advance(); }
                if src.peek() == Some('\n') { src.advance(); }
                continue;
            }
            None => {
                while indent_stack.len() > 1 {
                    indent_stack.pop();
                    push!(parser, actions, Tok::Dedent);
                }
                return Ok(());
            }
            _ => {
                let current = *indent_stack.last().unwrap();
                if indent > current {
                    indent_stack.push(indent);
                    push!(parser, actions, Tok::Indent);
                } else if indent < current {
                    while *indent_stack.last().unwrap() > indent {
                        indent_stack.pop();
                        push!(parser, actions, Tok::Dedent);
                    }
                    if *indent_stack.last().unwrap() != indent {
                        return Err("dedent does not match any outer indentation level".into());
                    }
                }
                return Ok(());
            }
        }
    }
}

fn read_string_body(src: &mut Source<std::str::Chars<'_>>, quote: char, triple: bool) -> Result<(), String> {
    if triple {
        loop {
            match src.peek() {
                None => return Err("unterminated string".into()),
                Some('\\') => { src.advance(); src.advance(); }
                Some(c) if c == quote
                    && src.peek_n(1) == Some(quote)
                    && src.peek_n(2) == Some(quote) =>
                {
                    src.advance(); src.advance(); src.advance();
                    return Ok(());
                }
                _ => { src.advance(); }
            }
        }
    } else {
        loop {
            match src.peek() {
                None | Some('\n') => return Err("unterminated string".into()),
                Some('\\') => { src.advance(); src.advance(); }
                Some(c) if c == quote => { src.advance(); return Ok(()); }
                _ => { src.advance(); }
            }
        }
    }
}

fn read_string(src: &mut Source<std::str::Chars<'_>>) -> Result<(), String> {
    let quote = src.peek().unwrap();
    let triple = src.peek_n(1) == Some(quote) && src.peek_n(2) == Some(quote);
    if triple {
        src.advance(); src.advance(); src.advance();
    } else {
        src.advance();
    }
    read_string_body(src, quote, triple)
}

fn read_number(src: &mut Source<std::str::Chars<'_>>) {
    if src.peek() == Some('0') {
        src.advance();
        match src.peek() {
            Some('x' | 'X') => { src.advance(); src.read_hex_digits(); }
            Some('o' | 'O') => { src.advance(); src.read_while(|c| matches!(c, '0'..='7' | '_')); }
            Some('b' | 'B') => { src.advance(); src.read_while(|c| matches!(c, '0' | '1' | '_')); }
            _ => { src.read_digits(); }
        }
    } else {
        src.read_digits();
    }
    if src.peek() == Some('.') && src.peek_n(1).is_some_and(|c| c.is_ascii_digit()) {
        src.advance();
        src.read_digits();
    }
    if matches!(src.peek(), Some('e' | 'E')) {
        src.advance();
        if matches!(src.peek(), Some('+' | '-')) { src.advance(); }
        src.read_digits();
    }
    if matches!(src.peek(), Some('j' | 'J')) { src.advance(); }
}

fn is_string_prefix(s: &str) -> bool {
    matches!(s, "r" | "R" | "b" | "B" | "f" | "F" | "u" | "U"
        | "rb" | "Rb" | "rB" | "RB" | "br" | "Br" | "bR" | "BR"
        | "rf" | "Rf" | "rF" | "RF" | "fr" | "Fr" | "fR" | "FR")
}

// Operator table: longest first for correct matching.
const OPS: [(&str, fn() -> Tok); 41] = [
    ("...", || Tok::Ellipsis),
    ("**=", || Tok::Augassign(AugOp::Pow)),
    ("//=", || Tok::Augassign(AugOp::FloorDiv)),
    ("<<=", || Tok::Augassign(AugOp::Shl)),
    (">>=", || Tok::Augassign(AugOp::Shr)),
    ("**",  || Tok::Doublestar(Precedence::Right(12))),
    ("//",  || Tok::Binop(BinOp::FloorDiv, Precedence::Left(11))),
    ("<<",  || Tok::Binop(BinOp::Shl, Precedence::Left(8))),
    (">>",  || Tok::Binop(BinOp::Shr, Precedence::Left(8))),
    ("+=",  || Tok::Augassign(AugOp::Add)),
    ("-=",  || Tok::Augassign(AugOp::Sub)),
    ("*=",  || Tok::Augassign(AugOp::Mul)),
    ("/=",  || Tok::Augassign(AugOp::Div)),
    ("%=",  || Tok::Augassign(AugOp::Mod)),
    ("&=",  || Tok::Augassign(AugOp::BitAnd)),
    ("|=",  || Tok::Augassign(AugOp::BitOr)),
    ("^=",  || Tok::Augassign(AugOp::BitXor)),
    ("@=",  || Tok::Augassign(AugOp::MatMul)),
    ("==",  || Tok::CompOp(CompOp::Eq)),
    ("!=",  || Tok::CompOp(CompOp::Ne)),
    ("<=",  || Tok::CompOp(CompOp::Le)),
    (">=",  || Tok::CompOp(CompOp::Ge)),
    ("->",  || Tok::Arrow),
    (":=",  || Tok::Walrus),
    (".",   || Tok::Dot),
    (":",   || Tok::Colon),
    (";",   || Tok::Semicolon),
    (",",   || Tok::Comma),
    ("~",   || Tok::Tilde),
    ("@",   || Tok::At),
    ("=",   || Tok::Eq),
    ("<",   || Tok::CompOp(CompOp::Lt)),
    (">",   || Tok::CompOp(CompOp::Gt)),
    ("|",   || Tok::Binop(BinOp::BitOr, Precedence::Left(5))),
    ("^",   || Tok::Binop(BinOp::BitXor, Precedence::Left(6))),
    ("&",   || Tok::Binop(BinOp::BitAnd, Precedence::Left(7))),
    ("/",   || Tok::Binop(BinOp::Div, Precedence::Left(11))),
    ("%",   || Tok::Binop(BinOp::Mod, Precedence::Left(11))),
    ("+",   || Tok::Plus(Precedence::Left(9))),
    ("-",   || Tok::Minus(Precedence::Left(9))),
    ("*",   || Tok::Star(Precedence::Left(10))),
];

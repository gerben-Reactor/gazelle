use gazelle::Precedence;
use gazelle::lexer::Source;

use crate::grammar::{PythonParser, PythonTerminal, PyActions};

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
            push!(parser, actions, Tok::NEWLINE);
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
                push!(parser, actions, Tok::STRING(input[str_start..src.offset()].to_string()));
                continue;
            }
            push!(parser, actions, match s {
                "False" => Tok::FALSE,
                "None" => Tok::NONE,
                "True" => Tok::TRUE,
                "and" => Tok::AND,
                "as" => Tok::AS,
                "assert" => Tok::ASSERT,
                "async" => Tok::ASYNC,
                "await" => Tok::AWAIT,
                "break" => Tok::BREAK,
                "class" => Tok::CLASS,
                "continue" => Tok::CONTINUE,
                "def" => Tok::DEF,
                "del" => Tok::DEL,
                "elif" => Tok::ELIF,
                "else" => Tok::ELSE,
                "except" => Tok::EXCEPT,
                "finally" => Tok::FINALLY,
                "for" => Tok::FOR,
                "from" => Tok::FROM,
                "global" => Tok::GLOBAL,
                "if" => Tok::IF,
                "import" => Tok::IMPORT,
                "in" => Tok::IN,
                "is" => Tok::IS,
                "lambda" => Tok::LAMBDA,
                "nonlocal" => Tok::NONLOCAL,
                "not" => Tok::NOT,
                "or" => Tok::OR,
                "pass" => Tok::PASS,
                "raise" => Tok::RAISE,
                "return" => Tok::RETURN,
                "try" => Tok::TRY,
                "while" => Tok::WHILE,
                "with" => Tok::WITH,
                "yield" => Tok::YIELD,
                _ => Tok::NAME(s.to_string()),
            });
            continue;
        }

        // Number literal
        if src.peek().is_some_and(|c| c.is_ascii_digit())
            || (src.peek() == Some('.') && src.peek_n(1).is_some_and(|c| c.is_ascii_digit()))
        {
            let start = src.offset();
            read_number(&mut src);
            push!(parser, actions, Tok::NUMBER(input[start..src.offset()].to_string()));
            continue;
        }

        // String literal (no prefix)
        if matches!(src.peek(), Some('\'' | '"')) {
            let start = src.offset();
            read_string(&mut src)?;
            push!(parser, actions, Tok::STRING(input[start..src.offset()].to_string()));
            continue;
        }

        // Brackets
        match src.peek() {
            Some('(' | '[' | '{') => {
                let c = src.peek().unwrap();
                src.advance();
                bracket_depth += 1;
                push!(parser, actions, match c {
                    '(' => Tok::LPAREN, '[' => Tok::LBRACK, _ => Tok::LBRACE,
                });
                continue;
            }
            Some(')' | ']' | '}') => {
                let c = src.peek().unwrap();
                src.advance();
                bracket_depth = bracket_depth.saturating_sub(1);
                push!(parser, actions, match c {
                    ')' => Tok::RPAREN, ']' => Tok::RBRACK, _ => Tok::RBRACE,
                });
                continue;
            }
            _ => {}
        }

        // Operators (longest first)
        const OPS: &[&str] = &[
            "...", "**=", "//=", "<<=", ">>=",
            "**", "//", "<<", ">>",
            "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "@=",
            "==", "!=", "<=", ">=",
            "->", ":=",
            ".", ":", ";", ",", "~", "@", "=",
            "<", ">", "|", "^", "&", "/", "%", "+", "-", "*",
        ];
        if let Some((idx, _)) = src.read_one_of(OPS) {
            push!(parser, actions, match idx {
                0 => Tok::ELLIPSIS,
                1 => Tok::AUGASSIGN("**=".into()),
                2 => Tok::AUGASSIGN("//=".into()),
                3 => Tok::AUGASSIGN("<<=".into()),
                4 => Tok::AUGASSIGN(">>=".into()),
                5 => Tok::DOUBLESTAR(Precedence::Right(12)),
                6 => Tok::BINOP("//".into(), Precedence::Left(11)),
                7 => Tok::BINOP("<<".into(), Precedence::Left(8)),
                8 => Tok::BINOP(">>".into(), Precedence::Left(8)),
                9 => Tok::AUGASSIGN("+=".into()),
                10 => Tok::AUGASSIGN("-=".into()),
                11 => Tok::AUGASSIGN("*=".into()),
                12 => Tok::AUGASSIGN("/=".into()),
                13 => Tok::AUGASSIGN("%=".into()),
                14 => Tok::AUGASSIGN("&=".into()),
                15 => Tok::AUGASSIGN("|=".into()),
                16 => Tok::AUGASSIGN("^=".into()),
                17 => Tok::AUGASSIGN("@=".into()),
                18 => Tok::COMP_OP("==".into()),
                19 => Tok::COMP_OP("!=".into()),
                20 => Tok::COMP_OP("<=".into()),
                21 => Tok::COMP_OP(">=".into()),
                22 => Tok::ARROW,
                23 => Tok::WALRUS,
                24 => Tok::DOT,
                25 => Tok::COLON,
                26 => Tok::SEMICOLON,
                27 => Tok::COMMA,
                28 => Tok::TILDE,
                29 => Tok::AT,
                30 => Tok::EQ,
                31 => Tok::COMP_OP("<".into()),
                32 => Tok::COMP_OP(">".into()),
                33 => Tok::BINOP("|".into(), Precedence::Left(5)),
                34 => Tok::BINOP("^".into(), Precedence::Left(6)),
                35 => Tok::BINOP("&".into(), Precedence::Left(7)),
                36 => Tok::BINOP("/".into(), Precedence::Left(11)),
                37 => Tok::BINOP("%".into(), Precedence::Left(11)),
                38 => Tok::PLUS(Precedence::Left(9)),
                39 => Tok::MINUS(Precedence::Left(9)),
                40 => Tok::STAR(Precedence::Left(10)),
                _ => unreachable!(),
            });
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
                    push!(parser, actions, Tok::DEDENT);
                }
                return Ok(());
            }
            _ => {
                let current = *indent_stack.last().unwrap();
                if indent > current {
                    indent_stack.push(indent);
                    push!(parser, actions, Tok::INDENT);
                } else if indent < current {
                    while *indent_stack.last().unwrap() > indent {
                        indent_stack.pop();
                        push!(parser, actions, Tok::DEDENT);
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

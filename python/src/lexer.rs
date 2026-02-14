use gazelle::Precedence;
use gazelle::lexer::Source;

use crate::grammar::{PythonTerminal, PyActions};

pub struct PythonLexer<'a> {
    input: &'a str,
    src: Source<std::str::Chars<'a>>,
    indent_stack: Vec<usize>,
    pending_dedents: usize,
    bracket_depth: usize,
    pending_newline: bool,
    pending_indent: bool,
    initialized: bool,
}

type Tok<'a> = PythonTerminal<PyActions>;

impl<'a> PythonLexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            src: Source::from_str(input),
            indent_stack: vec![0],
            pending_dedents: 0,
            bracket_depth: 0,
            pending_newline: false,
            pending_indent: false,
            initialized: false,
        }
    }

    fn read_string_body(&mut self, quote: char, triple: bool) -> Result<(), String> {
        if triple {
            loop {
                match self.src.peek() {
                    None => return Err("unterminated string".into()),
                    Some('\\') => { self.src.advance(); self.src.advance(); }
                    Some(c) if c == quote
                        && self.src.peek_n(1) == Some(quote)
                        && self.src.peek_n(2) == Some(quote) =>
                    {
                        self.src.advance(); self.src.advance(); self.src.advance();
                        return Ok(());
                    }
                    _ => { self.src.advance(); }
                }
            }
        } else {
            loop {
                match self.src.peek() {
                    None | Some('\n') => return Err("unterminated string".into()),
                    Some('\\') => { self.src.advance(); self.src.advance(); }
                    Some(c) if c == quote => { self.src.advance(); return Ok(()); }
                    _ => { self.src.advance(); }
                }
            }
        }
    }

    fn read_string(&mut self) -> Result<(), String> {
        let quote = self.src.peek().unwrap();
        let triple = self.src.peek_n(1) == Some(quote) && self.src.peek_n(2) == Some(quote);
        if triple {
            self.src.advance(); self.src.advance(); self.src.advance();
        } else {
            self.src.advance();
        }
        self.read_string_body(quote, triple)
    }

    fn read_number(&mut self) {
        if self.src.peek() == Some('0') {
            self.src.advance();
            match self.src.peek() {
                Some('x' | 'X') => { self.src.advance(); self.src.read_hex_digits(); }
                Some('o' | 'O') => { self.src.advance(); self.src.read_while(|c| matches!(c, '0'..='7' | '_')); }
                Some('b' | 'B') => { self.src.advance(); self.src.read_while(|c| matches!(c, '0' | '1' | '_')); }
                _ => { self.src.read_digits(); }
            }
        } else {
            self.src.read_digits();
        }
        if self.src.peek() == Some('.') && self.src.peek_n(1).is_some_and(|c| c.is_ascii_digit()) {
            self.src.advance();
            self.src.read_digits();
        }
        if matches!(self.src.peek(), Some('e' | 'E')) {
            self.src.advance();
            if matches!(self.src.peek(), Some('+' | '-')) { self.src.advance(); }
            self.src.read_digits();
        }
        if matches!(self.src.peek(), Some('j' | 'J')) { self.src.advance(); }
    }

    /// Skip to next non-blank line, measuring its indentation.
    /// Queues INDENT/DEDENT tokens as needed.
    fn process_line_start(&mut self) -> Result<(), String> {
        loop {
            let start = self.src.offset();
            self.src.skip_while(|c| c == ' ' || c == '\t');
            let indent = self.src.offset() - start;

            if self.src.peek() == Some('#') { self.src.read_until_any(&['\n']); }
            match self.src.peek() {
                Some('\r' | '\n') => {
                    if self.src.peek() == Some('\r') { self.src.advance(); }
                    if self.src.peek() == Some('\n') { self.src.advance(); }
                    continue;
                }
                None => {
                    while self.indent_stack.len() > 1 {
                        self.indent_stack.pop();
                        self.pending_dedents += 1;
                    }
                    return Ok(());
                }
                _ => {
                    let current = *self.indent_stack.last().unwrap();
                    if indent > current {
                        self.indent_stack.push(indent);
                        self.pending_indent = true;
                    } else if indent < current {
                        while *self.indent_stack.last().unwrap() > indent {
                            self.indent_stack.pop();
                            self.pending_dedents += 1;
                        }
                        if *self.indent_stack.last().unwrap() != indent {
                            return Err("dedent does not match any outer indentation level".into());
                        }
                    }
                    return Ok(());
                }
            }
        }
    }

    pub(crate) fn next(&mut self) -> Result<Option<Tok<'a>>, String> {
      loop {
        // Drain pending tokens: NEWLINE before DEDENT before INDENT
        if self.pending_newline {
            self.pending_newline = false;
            return Ok(Some(PythonTerminal::NEWLINE));
        }
        if self.pending_dedents > 0 {
            self.pending_dedents -= 1;
            return Ok(Some(PythonTerminal::DEDENT));
        }
        if self.pending_indent {
            self.pending_indent = false;
            return Ok(Some(PythonTerminal::INDENT));
        }

        // First call: process initial indentation
        if !self.initialized {
            self.initialized = true;
            self.process_line_start()?;
            continue;
        }

        // Skip horizontal whitespace and line continuations
        loop {
            self.src.skip_while(|c| c == ' ' || c == '\t');
            if self.src.peek() == Some('\\') && self.src.peek_n(1) == Some('\n') {
                self.src.advance();
                self.src.advance();
                continue;
            }
            break;
        }
        if self.src.peek() == Some('#') {
            self.src.read_until_any(&['\n']);
        }

        // Newline
        if matches!(self.src.peek(), Some('\n' | '\r')) {
            if self.src.peek() == Some('\r') { self.src.advance(); }
            if self.src.peek() == Some('\n') { self.src.advance(); }
            if self.bracket_depth > 0 {
                continue;
            }
            self.process_line_start()?;
            self.pending_newline = true;
            continue;
        }

        // EOF
        if self.src.at_end() {
            return Ok(None);
        }

        // Identifier or keyword
        if let Some(span) = self.src.read_ident() {
            let s = &self.input[span];
            if is_string_prefix(s) && matches!(self.src.peek(), Some('\'' | '"')) {
                let str_start = self.src.offset() - s.len();
                self.read_string()?;
                return Ok(Some(PythonTerminal::STRING(self.input[str_start..self.src.offset()].to_string())));
            }
            return Ok(Some(match s {
                "False" => PythonTerminal::FALSE,
                "None" => PythonTerminal::NONE,
                "True" => PythonTerminal::TRUE,
                "and" => PythonTerminal::AND,
                "as" => PythonTerminal::AS,
                "assert" => PythonTerminal::ASSERT,
                "async" => PythonTerminal::ASYNC,
                "await" => PythonTerminal::AWAIT,
                "break" => PythonTerminal::BREAK,
                "class" => PythonTerminal::CLASS,
                "continue" => PythonTerminal::CONTINUE,
                "def" => PythonTerminal::DEF,
                "del" => PythonTerminal::DEL,
                "elif" => PythonTerminal::ELIF,
                "else" => PythonTerminal::ELSE,
                "except" => PythonTerminal::EXCEPT,
                "finally" => PythonTerminal::FINALLY,
                "for" => PythonTerminal::FOR,
                "from" => PythonTerminal::FROM,
                "global" => PythonTerminal::GLOBAL,
                "if" => PythonTerminal::IF,
                "import" => PythonTerminal::IMPORT,
                "in" => PythonTerminal::IN,
                "is" => PythonTerminal::IS,
                "lambda" => PythonTerminal::LAMBDA,
                "nonlocal" => PythonTerminal::NONLOCAL,
                "not" => PythonTerminal::NOT,
                "or" => PythonTerminal::OR,
                "pass" => PythonTerminal::PASS,
                "raise" => PythonTerminal::RAISE,
                "return" => PythonTerminal::RETURN,
                "try" => PythonTerminal::TRY,
                "while" => PythonTerminal::WHILE,
                "with" => PythonTerminal::WITH,
                "yield" => PythonTerminal::YIELD,
                _ => PythonTerminal::NAME(s.to_string()),
            }));
        }

        // Number literal
        if self.src.peek().is_some_and(|c| c.is_ascii_digit())
            || (self.src.peek() == Some('.') && self.src.peek_n(1).is_some_and(|c| c.is_ascii_digit()))
        {
            let start = self.src.offset();
            self.read_number();
            return Ok(Some(PythonTerminal::NUMBER(self.input[start..self.src.offset()].to_string())));
        }

        // String literal (no prefix)
        if matches!(self.src.peek(), Some('\'' | '"')) {
            let start = self.src.offset();
            self.read_string()?;
            return Ok(Some(PythonTerminal::STRING(self.input[start..self.src.offset()].to_string())));
        }

        // Dot/Ellipsis
        if self.src.peek() == Some('.') {
            if self.src.peek_n(1) == Some('.') && self.src.peek_n(2) == Some('.') {
                self.src.advance(); self.src.advance(); self.src.advance();
                return Ok(Some(PythonTerminal::ELLIPSIS));
            }
            self.src.advance();
            return Ok(Some(PythonTerminal::DOT));
        }

        // Brackets
        match self.src.peek() {
            Some('(') => { self.src.advance(); self.bracket_depth += 1; return Ok(Some(PythonTerminal::LPAREN)); }
            Some(')') => { self.src.advance(); self.bracket_depth = self.bracket_depth.saturating_sub(1); return Ok(Some(PythonTerminal::RPAREN)); }
            Some('[') => { self.src.advance(); self.bracket_depth += 1; return Ok(Some(PythonTerminal::LBRACK)); }
            Some(']') => { self.src.advance(); self.bracket_depth = self.bracket_depth.saturating_sub(1); return Ok(Some(PythonTerminal::RBRACK)); }
            Some('{') => { self.src.advance(); self.bracket_depth += 1; return Ok(Some(PythonTerminal::LBRACE)); }
            Some('}') => { self.src.advance(); self.bracket_depth = self.bracket_depth.saturating_sub(1); return Ok(Some(PythonTerminal::RBRACE)); }
            _ => {}
        }

        // Multi-char operators (longest first)
        const MULTI_OPS: &[&str] = &[
            "**=", "//=", "<<=", ">>=",
            "**", "//", "<<", ">>",
            "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "@=",
            "==", "!=", "<=", ">=",
            "->", ":=",
        ];
        if let Some((idx, _)) = self.src.read_one_of(MULTI_OPS) {
            return Ok(Some(match idx {
                0 => PythonTerminal::AUGASSIGN("**=".into()),
                1 => PythonTerminal::AUGASSIGN("//=".into()),
                2 => PythonTerminal::AUGASSIGN("<<=".into()),
                3 => PythonTerminal::AUGASSIGN(">>=".into()),
                4 => PythonTerminal::DOUBLESTAR(Precedence::Right(12)),
                5 => PythonTerminal::BINOP("//".into(), Precedence::Left(11)),
                6 => PythonTerminal::BINOP("<<".into(), Precedence::Left(8)),
                7 => PythonTerminal::BINOP(">>".into(), Precedence::Left(8)),
                8 => PythonTerminal::AUGASSIGN("+=".into()),
                9 => PythonTerminal::AUGASSIGN("-=".into()),
                10 => PythonTerminal::AUGASSIGN("*=".into()),
                11 => PythonTerminal::AUGASSIGN("/=".into()),
                12 => PythonTerminal::AUGASSIGN("%=".into()),
                13 => PythonTerminal::AUGASSIGN("&=".into()),
                14 => PythonTerminal::AUGASSIGN("|=".into()),
                15 => PythonTerminal::AUGASSIGN("^=".into()),
                16 => PythonTerminal::AUGASSIGN("@=".into()),
                17 => PythonTerminal::COMP_OP("==".into()),
                18 => PythonTerminal::COMP_OP("!=".into()),
                19 => PythonTerminal::COMP_OP("<=".into()),
                20 => PythonTerminal::COMP_OP(">=".into()),
                21 => PythonTerminal::ARROW,
                22 => PythonTerminal::WALRUS,
                _ => unreachable!(),
            }));
        }

        // Single-char operators
        if let Some(c) = self.src.peek() {
            self.src.advance();
            return Ok(Some(match c {
                ':' => PythonTerminal::COLON,
                ';' => PythonTerminal::SEMICOLON,
                ',' => PythonTerminal::COMMA,
                '~' => PythonTerminal::TILDE,
                '@' => PythonTerminal::AT,
                '=' => PythonTerminal::EQ,
                '<' => PythonTerminal::COMP_OP("<".into()),
                '>' => PythonTerminal::COMP_OP(">".into()),
                '|' => PythonTerminal::BINOP("|".into(), Precedence::Left(5)),
                '^' => PythonTerminal::BINOP("^".into(), Precedence::Left(6)),
                '&' => PythonTerminal::BINOP("&".into(), Precedence::Left(7)),
                '/' => PythonTerminal::BINOP("/".into(), Precedence::Left(11)),
                '%' => PythonTerminal::BINOP("%".into(), Precedence::Left(11)),
                '+' => PythonTerminal::PLUS(Precedence::Left(9)),
                '-' => PythonTerminal::MINUS(Precedence::Left(9)),
                '*' => PythonTerminal::STAR(Precedence::Left(10)),
                _ => return Err(format!("unexpected character: {:?}", c)),
            }));
        }

      } // loop
    }
}

fn is_string_prefix(s: &str) -> bool {
    matches!(s, "r" | "R" | "b" | "B" | "f" | "F" | "u" | "U"
        | "rb" | "Rb" | "rB" | "RB" | "br" | "Br" | "bR" | "BR"
        | "rf" | "Rf" | "rF" | "RF" | "fr" | "Fr" | "fR" | "FR")
}

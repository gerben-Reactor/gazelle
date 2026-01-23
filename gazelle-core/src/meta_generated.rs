#[doc(hidden)]
mod __meta_table {
    use super::gazelle_core;

    pub static ACTION_DATA: &[u32] = &[9,3,13,17,29,29,33,33,45,18,181,18,77,105,18,109,14,22,14,22,65,14,22,61,93,57,89,42,34,42,34,73,85,34,58,69,58,54,161,54,50,81,50,46,65,46,30,61,30,97,26,30,26,125,38,26,38,153,149,145,78,78,98,141,117,137,82,82,94,90,90,98,98,98,86,86,133,94,94,94,66,125,66,70,70,66,165,125,153,173,74,74,117,6,62,10,62,10,117,62,10,0,0,0];
    pub static ACTION_BASE: &[i32] = &[-2,1,1,-2,3,4,8,6,3,15,16,19,18,21,27,11,28,30,31,34,31,37,43,45,49,48,37,52,47,58,50,61,64,62,67,56,59,79,80,73,77,86,78,93,94,93];
    pub static ACTION_CHECK: &[u32] = &[0,1,2,3,4,5,4,5,8,6,5,6,15,7,6,7,9,10,9,10,11,9,10,11,12,11,12,13,14,13,14,17,20,14,16,16,16,18,26,18,19,19,19,21,22,21,23,22,23,22,24,23,24,27,25,24,25,28,28,29,30,30,31,33,27,32,35,35,34,36,36,31,31,31,32,32,32,34,34,34,37,38,37,39,39,37,40,41,42,42,38,38,38,45,43,44,43,44,41,43,44,4294967295,4294967295,4294967295];
    pub static GOTO_DATA: &[u32] = &[1,5,9,6,44,6,10,25,10,12,13,28,30,32,39,32,42,30,32,0,0,0];
    pub static GOTO_BASE: &[i32] = &[0,0,0,0,0,2,0,0,0,0,0,5,0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,4,0,0,0,0,0,0,0,0,0,0,6,0,0,9,0,0,0,0];
    pub static GOTO_CHECK: &[u32] = &[0,4,4,4,5,5,4,22,5,11,11,27,27,27,38,38,41,41,41,4294967295,4294967295,4294967295];
    pub static RULES: &[(u32, u8)] = &[(23,1),(13,5),(14,2),(14,1),(15,1),(15,1),(16,5),(16,4),(16,3),(17,3),(17,1),(18,4),(18,2),(18,3),(18,1),(19,6),(19,4),(20,3),(20,2),(20,1),(21,3),(21,1),(21,2),(22,2),(22,1)];
    pub static STATE_SYMBOL: &[u32] = &[0,13,2,1,5,14,16,1,3,15,19,5,17,18,6,4,1,7,1,1,7,1,8,6,6,18,7,9,20,12,21,1,22,12,1,1,1,11,10,21,1,9,20,11,15,6];
    pub const NUM_STATES: usize = 46;
    pub const NUM_TERMINALS: u32 = 12;
    #[allow(dead_code)]
    pub const NUM_NON_TERMINALS: u32 = 11;

    pub fn symbol_id(name: &str) -> gazelle_core::SymbolId {
        match name {
            "KW_TERMINALS" => gazelle_core::SymbolId(3),
            "LBRACE" => gazelle_core::SymbolId(5),
            "COLON" => gazelle_core::SymbolId(7),
            "PIPE" => gazelle_core::SymbolId(10),
            "IDENT" => gazelle_core::SymbolId(1),
            "SEMI" => gazelle_core::SymbolId(11),
            "EQ" => gazelle_core::SymbolId(9),
            "AT" => gazelle_core::SymbolId(12),
            "RBRACE" => gazelle_core::SymbolId(6),
            "KW_GRAMMAR" => gazelle_core::SymbolId(2),
            "KW_PREC" => gazelle_core::SymbolId(4),
            "COMMA" => gazelle_core::SymbolId(8),
            "grammar_def" => gazelle_core::SymbolId(13),
            "sections" => gazelle_core::SymbolId(14),
            "section" => gazelle_core::SymbolId(15),
            "terminals_block" => gazelle_core::SymbolId(16),
            "terminal_list" => gazelle_core::SymbolId(17),
            "terminal_item" => gazelle_core::SymbolId(18),
            "rule" => gazelle_core::SymbolId(19),
            "alts" => gazelle_core::SymbolId(20),
            "alt" => gazelle_core::SymbolId(21),
            "seq" => gazelle_core::SymbolId(22),
            _ => panic!("unknown symbol: {}", name),
        }
    }
}


/// Terminal symbols for the parser.
#[derive(Debug, Clone)]
pub  enum MetaTerminal {
    KwTerminals,
    Lbrace,
    Colon,
    Pipe,
    Ident(Ident),
    Semi,
    Eq,
    At,
    Rbrace,
    KwGrammar,
    KwPrec,
    Comma,
}

impl MetaTerminal {
    /// Get the symbol ID for this terminal.
    pub fn symbol_id(&self) -> gazelle_core::SymbolId {
        match self {
            Self::KwTerminals => gazelle_core::SymbolId(3),
            Self::Lbrace => gazelle_core::SymbolId(5),
            Self::Colon => gazelle_core::SymbolId(7),
            Self::Pipe => gazelle_core::SymbolId(10),
            Self::Ident(_) => gazelle_core::SymbolId(1),
            Self::Semi => gazelle_core::SymbolId(11),
            Self::Eq => gazelle_core::SymbolId(9),
            Self::At => gazelle_core::SymbolId(12),
            Self::Rbrace => gazelle_core::SymbolId(6),
            Self::KwGrammar => gazelle_core::SymbolId(2),
            Self::KwPrec => gazelle_core::SymbolId(4),
            Self::Comma => gazelle_core::SymbolId(8),
        }
    }

    /// Convert to a gazelle Token for parsing.
    pub fn to_token(&self, symbol_ids: &impl Fn(&str) -> gazelle_core::SymbolId) -> gazelle_core::Token {
        match self {
            Self::KwTerminals => gazelle_core::Token::new(symbol_ids("KW_TERMINALS"), "KW_TERMINALS"),
            Self::Lbrace => gazelle_core::Token::new(symbol_ids("LBRACE"), "LBRACE"),
            Self::Colon => gazelle_core::Token::new(symbol_ids("COLON"), "COLON"),
            Self::Pipe => gazelle_core::Token::new(symbol_ids("PIPE"), "PIPE"),
            Self::Ident(_) => gazelle_core::Token::new(symbol_ids("IDENT"), "IDENT"),
            Self::Semi => gazelle_core::Token::new(symbol_ids("SEMI"), "SEMI"),
            Self::Eq => gazelle_core::Token::new(symbol_ids("EQ"), "EQ"),
            Self::At => gazelle_core::Token::new(symbol_ids("AT"), "AT"),
            Self::Rbrace => gazelle_core::Token::new(symbol_ids("RBRACE"), "RBRACE"),
            Self::KwGrammar => gazelle_core::Token::new(symbol_ids("KW_GRAMMAR"), "KW_GRAMMAR"),
            Self::KwPrec => gazelle_core::Token::new(symbol_ids("KW_PREC"), "KW_PREC"),
            Self::Comma => gazelle_core::Token::new(symbol_ids("COMMA"), "COMMA"),
        }
    }

    /// Get precedence for runtime precedence comparison.
    /// Returns (level, assoc) where assoc: 0=left, 1=right.
    pub fn precedence(&self) -> Option<(u8, u8)> {
        match self {
            Self::KwTerminals => None,
            Self::Lbrace => None,
            Self::Colon => None,
            Self::Pipe => None,
            Self::Ident(_) => None,
            Self::Semi => None,
            Self::Eq => None,
            Self::At => None,
            Self::Rbrace => None,
            Self::KwGrammar => None,
            Self::KwPrec => None,
            Self::Comma => None,
        }
    }
}


/// Parse error.
#[derive(Debug, Clone)]
pub  struct MetaError {
    /// The parser state when error occurred.
    pub state: usize,
}

/// Actions trait for parser callbacks.
pub trait MetaActions {
    type GrammarDef;
    type Sections;
    type Section;
    type TerminalsBlock;
    type TerminalList;
    type TerminalItem;
    type Rule;
    type Alts;
    type Alt;
    type Seq;

    fn grammar_def(&mut self, v0: Ident, v1: Self::Sections) -> Self::GrammarDef;
    fn sections_append(&mut self, v0: Self::Sections, v1: Self::Section) -> Self::Sections;
    fn sections_single(&mut self, v0: Self::Section) -> Self::Sections;
    fn section_terminals(&mut self, v0: Self::TerminalsBlock) -> Self::Section;
    fn section_rule(&mut self, v0: Self::Rule) -> Self::Section;
    fn terminals_trailing(&mut self, v0: Self::TerminalList) -> Self::TerminalsBlock;
    fn terminals_block(&mut self, v0: Self::TerminalList) -> Self::TerminalsBlock;
    fn terminals_empty(&mut self, ) -> Self::TerminalsBlock;
    fn terminal_list_append(&mut self, v0: Self::TerminalList, v1: Self::TerminalItem) -> Self::TerminalList;
    fn terminal_list_single(&mut self, v0: Self::TerminalItem) -> Self::TerminalList;
    fn terminal_prec_typed(&mut self, v0: Ident, v1: Ident) -> Self::TerminalItem;
    fn terminal_prec_untyped(&mut self, v0: Ident) -> Self::TerminalItem;
    fn terminal_typed(&mut self, v0: Ident, v1: Ident) -> Self::TerminalItem;
    fn terminal_untyped(&mut self, v0: Ident) -> Self::TerminalItem;
    fn rule_typed(&mut self, v0: Ident, v1: Ident, v2: Self::Alts) -> Self::Rule;
    fn rule_untyped(&mut self, v0: Ident, v1: Self::Alts) -> Self::Rule;
    fn alts_append(&mut self, v0: Self::Alts, v1: Self::Alt) -> Self::Alts;
    fn alts_empty(&mut self, v0: Self::Alts) -> Self::Alts;
    fn alts_single(&mut self, v0: Self::Alt) -> Self::Alts;
    fn alt_named(&mut self, v0: Self::Seq, v1: Ident) -> Self::Alt;
    fn alt_unnamed(&mut self, v0: Self::Seq) -> Self::Alt;
    fn alt_empty_named(&mut self, v0: Ident) -> Self::Alt;
    fn seq_append(&mut self, v0: Self::Seq, v1: Ident) -> Self::Seq;
    fn seq_single(&mut self, v0: Ident) -> Self::Seq;
}


#[doc(hidden)]
pub  union __MetaValue<A: MetaActions> {
    __ident: std::mem::ManuallyDrop<Ident>,
    __grammar_def: std::mem::ManuallyDrop<A::GrammarDef>,
    __sections: std::mem::ManuallyDrop<A::Sections>,
    __section: std::mem::ManuallyDrop<A::Section>,
    __terminals_block: std::mem::ManuallyDrop<A::TerminalsBlock>,
    __terminal_list: std::mem::ManuallyDrop<A::TerminalList>,
    __terminal_item: std::mem::ManuallyDrop<A::TerminalItem>,
    __rule: std::mem::ManuallyDrop<A::Rule>,
    __alts: std::mem::ManuallyDrop<A::Alts>,
    __alt: std::mem::ManuallyDrop<A::Alt>,
    __seq: std::mem::ManuallyDrop<A::Seq>,
    __unit: (),
}


/// Type-safe LR parser.
pub struct MetaParser<A: MetaActions> {
    state_stack: Vec<(usize, Option<(u8, u8)>)>,  // (state, precedence: (level, assoc))
    value_stack: Vec<std::mem::ManuallyDrop<__MetaValue<A>>>,
}

impl<A: MetaActions> MetaParser<A> {
    /// Create a new parser instance.
    pub fn new() -> Self {
        Self {
            state_stack: vec![(0, None)],
            value_stack: Vec::new(),
        }
    }

    /// Push a terminal, performing any reductions.
    pub fn push(&mut self, terminal: MetaTerminal, actions: &mut A) -> Result<(), MetaError> {
        let token_prec = terminal.precedence();
        // Reduce loop
        loop {
            let (state, stack_prec) = self.current_state_and_prec();
            let symbol_id = terminal.symbol_id().0;
            let action = self.lookup_action(state, symbol_id);

            match action & 3 {
                0 => return Err(MetaError { state }),
                1 => {
                    // Shift
                    let next_state = (action >> 2) as usize;
                    self.do_shift(&terminal, next_state, token_prec);
                    return Ok(());
                }
                2 => {
                    // Reduce
                    let rule = (action >> 2) as usize;
                    self.do_reduce(rule, actions);
                }
                3 if action != 3 => {
                    // Shift/reduce: compare precedences
                    let shift_state = ((action >> 3) & 0x3FFF) as usize;
                    let reduce_rule = (action >> 17) as usize;

                    let should_shift = match (stack_prec, token_prec) {
                        (Some((sp, _)), Some((tp, assoc))) => {
                            if tp > sp { true }
                            else if tp < sp { false }
                            else { assoc == 1 }  // 1 = right-assoc = shift
                        }
                        _ => true,  // default to shift
                    };

                    if should_shift {
                        self.do_shift(&terminal, shift_state, token_prec);
                        return Ok(());
                    } else {
                        self.do_reduce(reduce_rule, actions);
                    }
                }
                _ => return Err(MetaError { state }),
            }
        }
    }

    /// Finish parsing and return the result.
    pub fn finish(mut self, actions: &mut A) -> Result<A::GrammarDef, MetaError> {
        // Reduce until accept
        loop {
            let state = self.current_state();
            let action = self.lookup_action(state, 0); // EOF

            match action & 3 {
                2 => {
                    // Reduce
                    let rule = (action >> 2) as usize;
                    self.do_reduce(rule, actions);
                }
                3 => {
                    if action == 3 {
                        // Accept
                        if let Some(value) = self.value_stack.pop() {
                            self.state_stack.pop();
                            let union_val = std::mem::ManuallyDrop::into_inner(value);
                            return Ok(unsafe { std::mem::ManuallyDrop::into_inner(union_val.__grammar_def) });
                        }
                    } else {
                        // Shift/reduce with EOF lookahead -> reduce
                        let reduce_rule = (action >> 17) as usize;
                        self.do_reduce(reduce_rule, actions);
                    }
                }
                _ => return Err(MetaError { state }),
            }
        }
    }

    /// Get the current parser state.
    pub fn state(&self) -> usize {
        self.current_state()
    }

    fn current_state(&self) -> usize {
        self.state_stack.last().unwrap().0
    }

    fn current_state_and_prec(&self) -> (usize, Option<(u8, u8)>) {
        let (state, _) = *self.state_stack.last().unwrap();
        // Find the most recent operator's precedence (for E OP E reductions)
        // Search backwards through the stack for a state with precedence
        let prec = self.state_stack.iter().rev()
            .find_map(|(_, p)| *p);
        (state, prec)
    }

    fn lookup_action(&self, state: usize, terminal: u32) -> u32 {
        let base = __meta_table::ACTION_BASE[state];
        let index = base.wrapping_add(terminal as i32) as usize;

        if index < __meta_table::ACTION_CHECK.len() && __meta_table::ACTION_CHECK[index] == state as u32 {
            __meta_table::ACTION_DATA[index]
        } else {
            0
        }
    }

    fn lookup_goto(&self, state: usize, non_terminal: u32) -> Option<usize> {
        let base = __meta_table::GOTO_BASE[state];
        let index = base.wrapping_add(non_terminal as i32) as usize;

        if index < __meta_table::GOTO_CHECK.len() && __meta_table::GOTO_CHECK[index] == state as u32 {
            Some(__meta_table::GOTO_DATA[index] as usize)
        } else {
            None
        }
    }

    fn do_shift(&mut self, terminal: &MetaTerminal, next_state: usize, prec: Option<(u8, u8)>) {
        // Store state with precedence (level and associativity)
        self.state_stack.push((next_state, prec));
        match terminal {
            MetaTerminal::KwTerminals => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Lbrace => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Colon => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Pipe => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Ident(v) => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __ident: std::mem::ManuallyDrop::new(v.clone()) }));
            }
            MetaTerminal::Semi => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Eq => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::At => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Rbrace => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::KwGrammar => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::KwPrec => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Comma => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
        }

    }

    fn do_reduce(&mut self, rule: usize, actions: &mut A) {
        if rule == 0 { return; }

        let (lhs_id, rhs_len) = __meta_table::RULES[rule];
        let rhs_len = rhs_len as usize;

        for _ in 0..rhs_len {
            self.state_stack.pop();
        }

        let original_rule_idx = rule - 1;

        let value = match original_rule_idx {
            0 => {
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v3 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__sections) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __grammar_def: std::mem::ManuallyDrop::new(actions.grammar_def(v1, v3)) }
            }
            1 => {
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__section) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__sections) };
                __MetaValue { __sections: std::mem::ManuallyDrop::new(actions.sections_append(v0, v1)) }
            }
            2 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__section) };
                __MetaValue { __sections: std::mem::ManuallyDrop::new(actions.sections_single(v0)) }
            }
            3 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminals_block) };
                __MetaValue { __section: std::mem::ManuallyDrop::new(actions.section_terminals(v0)) }
            }
            4 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__rule) };
                __MetaValue { __section: std::mem::ManuallyDrop::new(actions.section_rule(v0)) }
            }
            5 => {
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_list) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __terminals_block: std::mem::ManuallyDrop::new(actions.terminals_trailing(v2)) }
            }
            6 => {
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_list) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __terminals_block: std::mem::ManuallyDrop::new(actions.terminals_block(v2)) }
            }
            7 => {
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __terminals_block: std::mem::ManuallyDrop::new(actions.terminals_empty()) }
            }
            8 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_item) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_list) };
                __MetaValue { __terminal_list: std::mem::ManuallyDrop::new(actions.terminal_list_append(v0, v2)) }
            }
            9 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_item) };
                __MetaValue { __terminal_list: std::mem::ManuallyDrop::new(actions.terminal_list_single(v0)) }
            }
            10 => {
                let v3 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __terminal_item: std::mem::ManuallyDrop::new(actions.terminal_prec_typed(v1, v3)) }
            }
            11 => {
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __terminal_item: std::mem::ManuallyDrop::new(actions.terminal_prec_untyped(v1)) }
            }
            12 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                __MetaValue { __terminal_item: std::mem::ManuallyDrop::new(actions.terminal_typed(v0, v2)) }
            }
            13 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                __MetaValue { __terminal_item: std::mem::ManuallyDrop::new(actions.terminal_untyped(v0)) }
            }
            14 => {
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v4 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alts) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                __MetaValue { __rule: std::mem::ManuallyDrop::new(actions.rule_typed(v0, v2, v4)) }
            }
            15 => {
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alts) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                __MetaValue { __rule: std::mem::ManuallyDrop::new(actions.rule_untyped(v0, v2)) }
            }
            16 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alt) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alts) };
                __MetaValue { __alts: std::mem::ManuallyDrop::new(actions.alts_append(v0, v2)) }
            }
            17 => {
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alts) };
                __MetaValue { __alts: std::mem::ManuallyDrop::new(actions.alts_empty(v0)) }
            }
            18 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alt) };
                __MetaValue { __alts: std::mem::ManuallyDrop::new(actions.alts_single(v0)) }
            }
            19 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__seq) };
                __MetaValue { __alt: std::mem::ManuallyDrop::new(actions.alt_named(v0, v2)) }
            }
            20 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__seq) };
                __MetaValue { __alt: std::mem::ManuallyDrop::new(actions.alt_unnamed(v0)) }
            }
            21 => {
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __alt: std::mem::ManuallyDrop::new(actions.alt_empty_named(v1)) }
            }
            22 => {
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__seq) };
                __MetaValue { __seq: std::mem::ManuallyDrop::new(actions.seq_append(v0, v1)) }
            }
            23 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                __MetaValue { __seq: std::mem::ManuallyDrop::new(actions.seq_single(v0)) }
            }

            _ => return,
        };

        self.value_stack.push(std::mem::ManuallyDrop::new(value));

        let goto_state = self.current_state();
        let nt_index = lhs_id - __meta_table::NUM_TERMINALS - 1;
        if let Some(next_state) = self.lookup_goto(goto_state, nt_index) {
            self.state_stack.push((next_state, None));
        }
    }
}

impl<A: MetaActions> Default for MetaParser<A> {
    fn default() -> Self { Self::new() }
}

impl<A: MetaActions> Drop for MetaParser<A> {
    fn drop(&mut self) {
        while let Some(value) = self.value_stack.pop() {
            let (state, _) = self.state_stack.pop().unwrap();
            let sym_id = __meta_table::STATE_SYMBOL[state];
            unsafe {
                let union_val = std::mem::ManuallyDrop::into_inner(value);
                match sym_id {
                    1 => { std::mem::ManuallyDrop::into_inner(union_val.__ident); }
                    13 => { std::mem::ManuallyDrop::into_inner(union_val.__grammar_def); }
                    14 => { std::mem::ManuallyDrop::into_inner(union_val.__sections); }
                    15 => { std::mem::ManuallyDrop::into_inner(union_val.__section); }
                    16 => { std::mem::ManuallyDrop::into_inner(union_val.__terminals_block); }
                    17 => { std::mem::ManuallyDrop::into_inner(union_val.__terminal_list); }
                    18 => { std::mem::ManuallyDrop::into_inner(union_val.__terminal_item); }
                    19 => { std::mem::ManuallyDrop::into_inner(union_val.__rule); }
                    20 => { std::mem::ManuallyDrop::into_inner(union_val.__alts); }
                    21 => { std::mem::ManuallyDrop::into_inner(union_val.__alt); }
                    22 => { std::mem::ManuallyDrop::into_inner(union_val.__seq); }

                    _ => {}
                }
            }
        }
    }
}



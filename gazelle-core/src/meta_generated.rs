#[doc(hidden)]
mod __meta_table {
    use super::gazelle_core;

    pub static ACTION_DATA: &[u32] = &[9,3,13,17,41,22,37,22,109,14,22,14,93,41,14,37,61,18,169,18,69,45,18,49,85,81,94,78,78,86,86,73,90,77,97,94,94,94,82,82,61,90,90,90,66,61,66,70,70,66,85,105,157,137,74,74,62,125,62,165,113,62,121,141,34,145,34,6,0,34,58,133,58,42,54,42,54,30,125,30,0,113,30,149,26,38,26,38,0,26,50,161,50,46,10,46,10,0,0,10,0,0,0,0];
    pub static ACTION_BASE: &[i32] = &[-2,1,1,-2,3,4,8,12,16,3,14,11,15,14,19,25,17,31,32,28,43,44,37,25,39,40,55,56,51,57,63,64,67,52,68,76,77,83,79,84,58,87,67,93];
    pub static ACTION_CHECK: &[u32] = &[0,1,2,3,4,5,4,5,9,6,5,6,11,7,6,7,12,8,7,8,14,10,8,10,13,13,15,16,16,14,14,14,17,18,23,15,15,15,19,19,24,17,17,17,20,21,20,22,22,20,25,25,28,33,21,21,26,27,26,40,27,26,27,29,30,29,30,42,4294967295,30,31,31,31,32,34,32,34,35,36,35,4294967295,36,35,36,37,38,37,38,4294967295,37,39,39,39,41,43,41,43,4294967295,4294967295,43,4294967295,4294967295,4294967295,4294967295];
    pub static GOTO_DATA: &[u32] = &[1,7,6,8,43,8,5,38,5,13,16,14,22,14,25,16,14,29,32,0,0,0];
    pub static GOTO_BASE: &[i32] = &[0,0,0,0,0,0,0,2,0,0,0,0,2,0,0,0,0,0,0,0,0,4,0,0,7,0,0,13,0,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0];
    pub static GOTO_CHECK: &[u32] = &[0,4,4,4,7,7,4,36,7,12,12,12,21,21,24,24,24,27,27,4294967295,4294967295,4294967295];
    pub static RULES: &[(u32, u8)] = &[(23,1),(13,5),(14,2),(14,1),(15,1),(15,1),(16,5),(16,4),(16,3),(17,3),(17,1),(18,4),(18,2),(18,3),(18,1),(19,6),(19,4),(20,3),(20,2),(20,1),(21,3),(21,1),(22,2),(22,1)];
    pub static STATE_SYMBOL: &[u32] = &[0,13,2,1,5,19,15,14,16,3,1,7,9,20,22,1,21,1,12,1,11,10,21,1,9,20,11,5,4,17,6,1,18,7,1,6,8,6,18,1,7,1,6,15];
    pub const NUM_STATES: usize = 44;
    pub const NUM_TERMINALS: u32 = 12;
    #[allow(dead_code)]
    pub const NUM_NON_TERMINALS: u32 = 11;

    pub fn symbol_id(name: &str) -> gazelle_core::SymbolId {
        match name {
            "COMMA" => gazelle_core::SymbolId(8),
            "SEMI" => gazelle_core::SymbolId(11),
            "EQ" => gazelle_core::SymbolId(9),
            "KW_TERMINALS" => gazelle_core::SymbolId(3),
            "PIPE" => gazelle_core::SymbolId(10),
            "AT" => gazelle_core::SymbolId(12),
            "RBRACE" => gazelle_core::SymbolId(6),
            "KW_PREC" => gazelle_core::SymbolId(4),
            "COLON" => gazelle_core::SymbolId(7),
            "KW_GRAMMAR" => gazelle_core::SymbolId(2),
            "IDENT" => gazelle_core::SymbolId(1),
            "LBRACE" => gazelle_core::SymbolId(5),
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
    Comma,
    Semi,
    Eq,
    KwTerminals,
    Pipe,
    At,
    Rbrace,
    KwPrec,
    Colon,
    KwGrammar,
    Ident(Ident),
    Lbrace,
}

impl MetaTerminal {
    /// Get the symbol ID for this terminal.
    pub fn symbol_id(&self) -> gazelle_core::SymbolId {
        match self {
            Self::Comma => gazelle_core::SymbolId(8),
            Self::Semi => gazelle_core::SymbolId(11),
            Self::Eq => gazelle_core::SymbolId(9),
            Self::KwTerminals => gazelle_core::SymbolId(3),
            Self::Pipe => gazelle_core::SymbolId(10),
            Self::At => gazelle_core::SymbolId(12),
            Self::Rbrace => gazelle_core::SymbolId(6),
            Self::KwPrec => gazelle_core::SymbolId(4),
            Self::Colon => gazelle_core::SymbolId(7),
            Self::KwGrammar => gazelle_core::SymbolId(2),
            Self::Ident(_) => gazelle_core::SymbolId(1),
            Self::Lbrace => gazelle_core::SymbolId(5),
        }
    }

    /// Convert to a gazelle Token for parsing.
    pub fn to_token(&self, symbol_ids: &impl Fn(&str) -> gazelle_core::SymbolId) -> gazelle_core::Token {
        match self {
            Self::Comma => gazelle_core::Token::new(symbol_ids("COMMA"), "COMMA"),
            Self::Semi => gazelle_core::Token::new(symbol_ids("SEMI"), "SEMI"),
            Self::Eq => gazelle_core::Token::new(symbol_ids("EQ"), "EQ"),
            Self::KwTerminals => gazelle_core::Token::new(symbol_ids("KW_TERMINALS"), "KW_TERMINALS"),
            Self::Pipe => gazelle_core::Token::new(symbol_ids("PIPE"), "PIPE"),
            Self::At => gazelle_core::Token::new(symbol_ids("AT"), "AT"),
            Self::Rbrace => gazelle_core::Token::new(symbol_ids("RBRACE"), "RBRACE"),
            Self::KwPrec => gazelle_core::Token::new(symbol_ids("KW_PREC"), "KW_PREC"),
            Self::Colon => gazelle_core::Token::new(symbol_ids("COLON"), "COLON"),
            Self::KwGrammar => gazelle_core::Token::new(symbol_ids("KW_GRAMMAR"), "KW_GRAMMAR"),
            Self::Ident(_) => gazelle_core::Token::new(symbol_ids("IDENT"), "IDENT"),
            Self::Lbrace => gazelle_core::Token::new(symbol_ids("LBRACE"), "LBRACE"),
        }
    }

    /// Get precedence for runtime precedence comparison.
    /// Returns (level, assoc) where assoc: 0=left, 1=right.
    pub fn precedence(&self) -> Option<(u8, u8)> {
        match self {
            Self::Comma => None,
            Self::Semi => None,
            Self::Eq => None,
            Self::KwTerminals => None,
            Self::Pipe => None,
            Self::At => None,
            Self::Rbrace => None,
            Self::KwPrec => None,
            Self::Colon => None,
            Self::KwGrammar => None,
            Self::Ident(_) => None,
            Self::Lbrace => None,
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
            MetaTerminal::Comma => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Semi => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Eq => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::KwTerminals => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Pipe => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::At => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Rbrace => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::KwPrec => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Colon => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::KwGrammar => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Ident(v) => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __ident: std::mem::ManuallyDrop::new(v.clone()) }));
            }
            MetaTerminal::Lbrace => {
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
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__seq) };
                __MetaValue { __seq: std::mem::ManuallyDrop::new(actions.seq_append(v0, v1)) }
            }
            22 => {
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



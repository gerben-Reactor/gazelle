#[doc(hidden)]
mod __meta_table {
    use super::gazelle_core;

    pub static ACTION_DATA: &[u32] = &[5,13,3,17,21,25,29,37,105,41,46,34,65,34,57,81,61,85,42,26,54,73,54,38,77,38,50,22,50,50,46,18,14,30,57,30,89,105,14,73,113,54,133,169,161,66,66,74,74,153,141,145,82,82,117,90,58,6,133,82,82,117,58,62,62,90,90,90,82,82,117,86,70,70,78,78,0,0,10,0,0,86,86,86,10,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    pub static ACTION_BASE: &[i32] = &[-2,0,2,-3,1,4,-6,3,7,3,9,4,8,11,17,18,13,16,23,19,26,29,30,26,31,36,31,30,41,43,34,36,48,54,39,57,55,52,70,61,63,77,57];
    pub static ACTION_CHECK: &[u32] = &[0,1,2,3,4,5,6,7,8,9,10,11,13,11,10,12,10,12,14,15,16,16,16,17,18,17,19,20,19,19,21,22,24,23,21,23,21,25,24,26,27,26,28,25,29,30,30,31,31,32,34,34,28,28,28,33,36,42,35,32,32,32,36,37,37,33,33,33,35,35,35,38,39,39,40,40,4294967295,4294967295,41,4294967295,4294967295,38,38,38,41,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295];
    pub static GOTO_DATA: &[u32] = &[2,8,25,12,11,13,17,41,24,23,13,27,34,30,31,32,39,37,31,32,0,0,0,0,0,0];
    pub static GOTO_BASE: &[i32] = &[0,0,0,0,0,0,0,-1,1,0,0,0,0,0,0,0,0,0,0,0,0,5,0,0,0,0,5,0,4,0,0,0,6,0,0,8,0,0,0,0,0,0,0];
    pub static GOTO_CHECK: &[u32] = &[0,7,8,10,10,10,16,25,8,21,21,26,28,28,28,28,32,35,35,35,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295];
    pub static RULES: &[(u32, u8)] = &[(26,1),(14,9),(15,2),(15,1),(16,5),(16,4),(16,3),(17,3),(17,1),(18,3),(19,1),(19,0),(20,2),(20,0),(21,5),(22,3),(22,1),(23,2),(23,1),(24,2),(24,0),(25,2),(25,1)];
    pub static STATE_SYMBOL: &[u32] = &[0,2,14,1,6,3,1,12,16,4,6,18,17,19,5,7,1,20,8,1,7,9,7,18,21,15,1,20,10,13,23,24,25,1,22,11,12,23,1,24,1,21,7];
    pub const NUM_STATES: usize = 43;
    pub const NUM_TERMINALS: u32 = 13;
    #[allow(dead_code)]
    pub const NUM_NON_TERMINALS: u32 = 13;

    pub fn symbol_id(name: &str) -> gazelle_core::SymbolId {
        match name {
            "KW_TERMINALS" => gazelle_core::SymbolId(4),
            "IDENT" => gazelle_core::SymbolId(1),
            "KW_START" => gazelle_core::SymbolId(3),
            "COLON" => gazelle_core::SymbolId(8),
            "SEMI" => gazelle_core::SymbolId(12),
            "AT" => gazelle_core::SymbolId(13),
            "RBRACE" => gazelle_core::SymbolId(7),
            "PIPE" => gazelle_core::SymbolId(11),
            "KW_GRAMMAR" => gazelle_core::SymbolId(2),
            "COMMA" => gazelle_core::SymbolId(9),
            "LBRACE" => gazelle_core::SymbolId(6),
            "EQ" => gazelle_core::SymbolId(10),
            "KW_PREC" => gazelle_core::SymbolId(5),
            "grammar_def" => gazelle_core::SymbolId(14),
            "rules" => gazelle_core::SymbolId(15),
            "terminals_block" => gazelle_core::SymbolId(16),
            "terminal_list" => gazelle_core::SymbolId(17),
            "terminal_item" => gazelle_core::SymbolId(18),
            "prec_opt" => gazelle_core::SymbolId(19),
            "type_opt" => gazelle_core::SymbolId(20),
            "rule" => gazelle_core::SymbolId(21),
            "alts" => gazelle_core::SymbolId(22),
            "alt" => gazelle_core::SymbolId(23),
            "name_opt" => gazelle_core::SymbolId(24),
            "seq" => gazelle_core::SymbolId(25),
            _ => panic!("unknown symbol: {}", name),
        }
    }
}


/// Terminal symbols for the parser.
#[derive(Debug, Clone)]
pub  enum MetaTerminal {
    KwTerminals,
    Ident(Ident),
    KwStart,
    Colon,
    Semi,
    At,
    Rbrace,
    Pipe,
    KwGrammar,
    Comma,
    Lbrace,
    Eq,
    KwPrec,
}

impl MetaTerminal {
    /// Get the symbol ID for this terminal.
    pub fn symbol_id(&self) -> gazelle_core::SymbolId {
        match self {
            Self::KwTerminals => gazelle_core::SymbolId(4),
            Self::Ident(_) => gazelle_core::SymbolId(1),
            Self::KwStart => gazelle_core::SymbolId(3),
            Self::Colon => gazelle_core::SymbolId(8),
            Self::Semi => gazelle_core::SymbolId(12),
            Self::At => gazelle_core::SymbolId(13),
            Self::Rbrace => gazelle_core::SymbolId(7),
            Self::Pipe => gazelle_core::SymbolId(11),
            Self::KwGrammar => gazelle_core::SymbolId(2),
            Self::Comma => gazelle_core::SymbolId(9),
            Self::Lbrace => gazelle_core::SymbolId(6),
            Self::Eq => gazelle_core::SymbolId(10),
            Self::KwPrec => gazelle_core::SymbolId(5),
        }
    }

    /// Convert to a gazelle Token for parsing.
    pub fn to_token(&self, symbol_ids: &impl Fn(&str) -> gazelle_core::SymbolId) -> gazelle_core::Token {
        match self {
            Self::KwTerminals => gazelle_core::Token::new(symbol_ids("KW_TERMINALS"), "KW_TERMINALS"),
            Self::Ident(_) => gazelle_core::Token::new(symbol_ids("IDENT"), "IDENT"),
            Self::KwStart => gazelle_core::Token::new(symbol_ids("KW_START"), "KW_START"),
            Self::Colon => gazelle_core::Token::new(symbol_ids("COLON"), "COLON"),
            Self::Semi => gazelle_core::Token::new(symbol_ids("SEMI"), "SEMI"),
            Self::At => gazelle_core::Token::new(symbol_ids("AT"), "AT"),
            Self::Rbrace => gazelle_core::Token::new(symbol_ids("RBRACE"), "RBRACE"),
            Self::Pipe => gazelle_core::Token::new(symbol_ids("PIPE"), "PIPE"),
            Self::KwGrammar => gazelle_core::Token::new(symbol_ids("KW_GRAMMAR"), "KW_GRAMMAR"),
            Self::Comma => gazelle_core::Token::new(symbol_ids("COMMA"), "COMMA"),
            Self::Lbrace => gazelle_core::Token::new(symbol_ids("LBRACE"), "LBRACE"),
            Self::Eq => gazelle_core::Token::new(symbol_ids("EQ"), "EQ"),
            Self::KwPrec => gazelle_core::Token::new(symbol_ids("KW_PREC"), "KW_PREC"),
        }
    }

    /// Get precedence for runtime precedence comparison.
    /// Returns (level, assoc) where assoc: 0=left, 1=right.
    pub fn precedence(&self) -> Option<(u8, u8)> {
        match self {
            Self::KwTerminals => None,
            Self::Ident(_) => None,
            Self::KwStart => None,
            Self::Colon => None,
            Self::Semi => None,
            Self::At => None,
            Self::Rbrace => None,
            Self::Pipe => None,
            Self::KwGrammar => None,
            Self::Comma => None,
            Self::Lbrace => None,
            Self::Eq => None,
            Self::KwPrec => None,
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
    type Rules;
    type TerminalsBlock;
    type TerminalList;
    type TerminalItem;
    type PrecOpt;
    type TypeOpt;
    type Rule;
    type Alts;
    type Alt;
    type NameOpt;
    type Seq;

    fn grammar_def(&mut self, v0: Ident, v1: Ident, v2: Self::TerminalsBlock, v3: Self::Rules) -> Self::GrammarDef;
    fn rules_append(&mut self, v0: Self::Rules, v1: Self::Rule) -> Self::Rules;
    fn rules_single(&mut self, v0: Self::Rule) -> Self::Rules;
    fn terminals_trailing(&mut self, v0: Self::TerminalList) -> Self::TerminalsBlock;
    fn terminals_block(&mut self, v0: Self::TerminalList) -> Self::TerminalsBlock;
    fn terminals_empty(&mut self, ) -> Self::TerminalsBlock;
    fn terminal_list_append(&mut self, v0: Self::TerminalList, v1: Self::TerminalItem) -> Self::TerminalList;
    fn terminal_list_single(&mut self, v0: Self::TerminalItem) -> Self::TerminalList;
    fn terminal_item(&mut self, v0: Self::PrecOpt, v1: Ident, v2: Self::TypeOpt) -> Self::TerminalItem;
    fn prec_yes(&mut self, ) -> Self::PrecOpt;
    fn prec_no(&mut self, ) -> Self::PrecOpt;
    fn type_some(&mut self, v0: Ident) -> Self::TypeOpt;
    fn type_none(&mut self, ) -> Self::TypeOpt;
    fn rule(&mut self, v0: Ident, v1: Self::TypeOpt, v2: Self::Alts) -> Self::Rule;
    fn alts_append(&mut self, v0: Self::Alts, v1: Self::Alt) -> Self::Alts;
    fn alts_single(&mut self, v0: Self::Alt) -> Self::Alts;
    fn alt(&mut self, v0: Self::Seq, v1: Self::NameOpt) -> Self::Alt;
    fn alt_empty(&mut self, v0: Self::NameOpt) -> Self::Alt;
    fn name_some(&mut self, v0: Ident) -> Self::NameOpt;
    fn name_none(&mut self, ) -> Self::NameOpt;
    fn seq_append(&mut self, v0: Self::Seq, v1: Ident) -> Self::Seq;
    fn seq_single(&mut self, v0: Ident) -> Self::Seq;
}


#[doc(hidden)]
pub  union __MetaValue<A: MetaActions> {
    __ident: std::mem::ManuallyDrop<Ident>,
    __grammar_def: std::mem::ManuallyDrop<A::GrammarDef>,
    __rules: std::mem::ManuallyDrop<A::Rules>,
    __terminals_block: std::mem::ManuallyDrop<A::TerminalsBlock>,
    __terminal_list: std::mem::ManuallyDrop<A::TerminalList>,
    __terminal_item: std::mem::ManuallyDrop<A::TerminalItem>,
    __prec_opt: std::mem::ManuallyDrop<A::PrecOpt>,
    __type_opt: std::mem::ManuallyDrop<A::TypeOpt>,
    __rule: std::mem::ManuallyDrop<A::Rule>,
    __alts: std::mem::ManuallyDrop<A::Alts>,
    __alt: std::mem::ManuallyDrop<A::Alt>,
    __name_opt: std::mem::ManuallyDrop<A::NameOpt>,
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
            MetaTerminal::Ident(v) => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __ident: std::mem::ManuallyDrop::new(v.clone()) }));
            }
            MetaTerminal::KwStart => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Colon => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Semi => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::At => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Rbrace => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Pipe => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::KwGrammar => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Comma => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Lbrace => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Eq => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::KwPrec => {
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
                let v7 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__rules) };
                let v6 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminals_block) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v4 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __grammar_def: std::mem::ManuallyDrop::new(actions.grammar_def(v1, v4, v6, v7)) }
            }
            1 => {
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__rule) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__rules) };
                __MetaValue { __rules: std::mem::ManuallyDrop::new(actions.rules_append(v0, v1)) }
            }
            2 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__rule) };
                __MetaValue { __rules: std::mem::ManuallyDrop::new(actions.rules_single(v0)) }
            }
            3 => {
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_list) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __terminals_block: std::mem::ManuallyDrop::new(actions.terminals_trailing(v2)) }
            }
            4 => {
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_list) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __terminals_block: std::mem::ManuallyDrop::new(actions.terminals_block(v2)) }
            }
            5 => {
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __terminals_block: std::mem::ManuallyDrop::new(actions.terminals_empty()) }
            }
            6 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_item) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_list) };
                __MetaValue { __terminal_list: std::mem::ManuallyDrop::new(actions.terminal_list_append(v0, v2)) }
            }
            7 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_item) };
                __MetaValue { __terminal_list: std::mem::ManuallyDrop::new(actions.terminal_list_single(v0)) }
            }
            8 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__type_opt) };
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__prec_opt) };
                __MetaValue { __terminal_item: std::mem::ManuallyDrop::new(actions.terminal_item(v0, v1, v2)) }
            }
            9 => {
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __prec_opt: std::mem::ManuallyDrop::new(actions.prec_yes()) }
            }
            10 => {
                __MetaValue { __prec_opt: std::mem::ManuallyDrop::new(actions.prec_no()) }
            }
            11 => {
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __type_opt: std::mem::ManuallyDrop::new(actions.type_some(v1)) }
            }
            12 => {
                __MetaValue { __type_opt: std::mem::ManuallyDrop::new(actions.type_none()) }
            }
            13 => {
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v3 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alts) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__type_opt) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                __MetaValue { __rule: std::mem::ManuallyDrop::new(actions.rule(v0, v1, v3)) }
            }
            14 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alt) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alts) };
                __MetaValue { __alts: std::mem::ManuallyDrop::new(actions.alts_append(v0, v2)) }
            }
            15 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alt) };
                __MetaValue { __alts: std::mem::ManuallyDrop::new(actions.alts_single(v0)) }
            }
            16 => {
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__name_opt) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__seq) };
                __MetaValue { __alt: std::mem::ManuallyDrop::new(actions.alt(v0, v1)) }
            }
            17 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__name_opt) };
                __MetaValue { __alt: std::mem::ManuallyDrop::new(actions.alt_empty(v0)) }
            }
            18 => {
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                { std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()) };
                __MetaValue { __name_opt: std::mem::ManuallyDrop::new(actions.name_some(v1)) }
            }
            19 => {
                __MetaValue { __name_opt: std::mem::ManuallyDrop::new(actions.name_none()) }
            }
            20 => {
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__seq) };
                __MetaValue { __seq: std::mem::ManuallyDrop::new(actions.seq_append(v0, v1)) }
            }
            21 => {
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
                    14 => { std::mem::ManuallyDrop::into_inner(union_val.__grammar_def); }
                    15 => { std::mem::ManuallyDrop::into_inner(union_val.__rules); }
                    16 => { std::mem::ManuallyDrop::into_inner(union_val.__terminals_block); }
                    17 => { std::mem::ManuallyDrop::into_inner(union_val.__terminal_list); }
                    18 => { std::mem::ManuallyDrop::into_inner(union_val.__terminal_item); }
                    19 => { std::mem::ManuallyDrop::into_inner(union_val.__prec_opt); }
                    20 => { std::mem::ManuallyDrop::into_inner(union_val.__type_opt); }
                    21 => { std::mem::ManuallyDrop::into_inner(union_val.__rule); }
                    22 => { std::mem::ManuallyDrop::into_inner(union_val.__alts); }
                    23 => { std::mem::ManuallyDrop::into_inner(union_val.__alt); }
                    24 => { std::mem::ManuallyDrop::into_inner(union_val.__name_opt); }
                    25 => { std::mem::ManuallyDrop::into_inner(union_val.__seq); }

                    _ => {}
                }
            }
        }
    }
}



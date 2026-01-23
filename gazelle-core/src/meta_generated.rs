#[doc(hidden)]
mod __meta_table {
    use super::gazelle_core;

    pub static ACTION_DATA: &[u32] = &[9,3,13,17,33,169,21,29,14,125,14,14,61,14,65,22,6,22,22,33,22,21,29,18,53,18,18,26,18,26,26,10,26,10,10,109,10,73,97,98,98,114,85,89,73,90,90,106,106,101,114,114,114,94,94,86,110,86,86,105,86,102,102,113,73,110,110,110,85,121,82,129,82,82,161,82,133,66,165,66,66,149,66,145,74,129,74,209,0,62,153,62,62,58,62,58,58,70,58,70,78,177,78,0,177,38,173,38,38,197,38,54,205,54,193,46,189,46,34,0,34,34,30,34,30,30,42,30,42,50,0,50,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    pub static ACTION_BASE: &[i32] = &[-2,1,1,-2,3,0,7,4,5,14,18,22,26,16,30,34,36,37,40,29,32,43,54,35,55,58,51,54,63,58,69,70,67,76,75,78,84,88,92,91,77,94,100,104,105,108,109,103,117,121,120,86,123];
    pub static ACTION_CHECK: &[u32] = &[0,1,2,3,4,5,4,4,6,7,6,6,8,6,8,9,13,9,9,10,9,10,10,11,10,11,11,12,11,12,12,14,12,14,14,15,14,16,17,19,19,18,20,20,21,23,23,17,17,17,18,18,18,21,21,22,24,22,22,25,22,26,26,27,28,24,24,24,29,29,30,31,30,30,32,30,31,33,40,33,33,34,33,34,35,36,35,51,4294967295,37,36,37,37,38,37,38,38,39,38,39,41,42,41,4294967295,47,43,42,43,43,47,43,44,44,44,45,46,45,46,48,4294967295,48,48,49,48,49,49,50,49,50,52,4294967295,52,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295,4294967295];
    pub static GOTO_DATA: &[u32] = &[1,10,6,11,14,11,9,39,9,12,50,12,20,19,17,23,17,29,19,17,34,35,45,46,0,0,0,0];
    pub static GOTO_BASE: &[i32] = &[0,0,0,0,0,0,0,0,0,0,2,0,0,0,0,0,2,0,0,0,0,4,0,0,0,0,0,0,7,0,0,13,0,0,0,0,-1,0,0,0,0,0,18,0,0,0,0,5,0,0,0,0,0];
    pub static GOTO_CHECK: &[u32] = &[0,4,4,4,10,10,4,36,10,4,47,10,16,16,16,21,21,28,28,28,31,31,42,42,4294967295,4294967295,4294967295,4294967295];
    pub static RULES: &[(u32, u8)] = &[(26,1),(13,5),(14,2),(14,1),(15,1),(15,1),(15,1),(16,5),(16,4),(16,3),(17,3),(17,1),(18,3),(18,1),(19,5),(19,4),(19,3),(20,3),(20,1),(21,3),(22,6),(22,4),(23,3),(23,2),(23,1),(24,3),(24,1),(25,2),(25,1)];
    pub static STATE_SYMBOL: &[u32] = &[0,13,2,1,5,3,15,4,1,19,14,16,22,6,15,7,9,25,1,24,23,10,11,24,1,12,1,1,9,23,11,5,1,6,20,21,8,6,6,21,7,1,5,6,1,17,18,8,6,6,18,7,1];
    pub const NUM_STATES: usize = 53;
    pub const NUM_TERMINALS: u32 = 12;
    #[allow(dead_code)]
    pub const NUM_NON_TERMINALS: u32 = 14;

    pub fn symbol_id(name: &str) -> gazelle_core::SymbolId {
        match name {
            "SEMI" => gazelle_core::SymbolId(11),
            "IDENT" => gazelle_core::SymbolId(1),
            "KW_PREC_TERMINALS" => gazelle_core::SymbolId(4),
            "PIPE" => gazelle_core::SymbolId(10),
            "AT" => gazelle_core::SymbolId(12),
            "LBRACE" => gazelle_core::SymbolId(5),
            "KW_GRAMMAR" => gazelle_core::SymbolId(2),
            "COLON" => gazelle_core::SymbolId(7),
            "EQ" => gazelle_core::SymbolId(9),
            "KW_TERMINALS" => gazelle_core::SymbolId(3),
            "RBRACE" => gazelle_core::SymbolId(6),
            "COMMA" => gazelle_core::SymbolId(8),
            "grammar_def" => gazelle_core::SymbolId(13),
            "sections" => gazelle_core::SymbolId(14),
            "section" => gazelle_core::SymbolId(15),
            "terminals_block" => gazelle_core::SymbolId(16),
            "terminal_list" => gazelle_core::SymbolId(17),
            "terminal_item" => gazelle_core::SymbolId(18),
            "prec_terminals_block" => gazelle_core::SymbolId(19),
            "prec_terminal_list" => gazelle_core::SymbolId(20),
            "prec_terminal_item" => gazelle_core::SymbolId(21),
            "rule" => gazelle_core::SymbolId(22),
            "alts" => gazelle_core::SymbolId(23),
            "alt" => gazelle_core::SymbolId(24),
            "seq" => gazelle_core::SymbolId(25),
            _ => panic!("unknown symbol: {}", name),
        }
    }
}


/// Terminal symbols for the parser.
#[derive(Debug, Clone)]
pub  enum MetaTerminal {
    Semi,
    Ident(String),
    KwPrecTerminals,
    Pipe,
    At,
    Lbrace,
    KwGrammar,
    Colon,
    Eq,
    KwTerminals,
    Rbrace,
    Comma,
}

impl MetaTerminal {
    /// Get the symbol ID for this terminal.
    pub fn symbol_id(&self) -> gazelle_core::SymbolId {
        match self {
            Self::Semi => gazelle_core::SymbolId(11),
            Self::Ident(_) => gazelle_core::SymbolId(1),
            Self::KwPrecTerminals => gazelle_core::SymbolId(4),
            Self::Pipe => gazelle_core::SymbolId(10),
            Self::At => gazelle_core::SymbolId(12),
            Self::Lbrace => gazelle_core::SymbolId(5),
            Self::KwGrammar => gazelle_core::SymbolId(2),
            Self::Colon => gazelle_core::SymbolId(7),
            Self::Eq => gazelle_core::SymbolId(9),
            Self::KwTerminals => gazelle_core::SymbolId(3),
            Self::Rbrace => gazelle_core::SymbolId(6),
            Self::Comma => gazelle_core::SymbolId(8),
        }
    }

    /// Convert to a gazelle Token for parsing.
    pub fn to_token(&self, symbol_ids: &impl Fn(&str) -> gazelle_core::SymbolId) -> gazelle_core::Token {
        match self {
            Self::Semi => gazelle_core::Token::new(symbol_ids("SEMI"), "SEMI"),
            Self::Ident(_) => gazelle_core::Token::new(symbol_ids("IDENT"), "IDENT"),
            Self::KwPrecTerminals => gazelle_core::Token::new(symbol_ids("KW_PREC_TERMINALS"), "KW_PREC_TERMINALS"),
            Self::Pipe => gazelle_core::Token::new(symbol_ids("PIPE"), "PIPE"),
            Self::At => gazelle_core::Token::new(symbol_ids("AT"), "AT"),
            Self::Lbrace => gazelle_core::Token::new(symbol_ids("LBRACE"), "LBRACE"),
            Self::KwGrammar => gazelle_core::Token::new(symbol_ids("KW_GRAMMAR"), "KW_GRAMMAR"),
            Self::Colon => gazelle_core::Token::new(symbol_ids("COLON"), "COLON"),
            Self::Eq => gazelle_core::Token::new(symbol_ids("EQ"), "EQ"),
            Self::KwTerminals => gazelle_core::Token::new(symbol_ids("KW_TERMINALS"), "KW_TERMINALS"),
            Self::Rbrace => gazelle_core::Token::new(symbol_ids("RBRACE"), "RBRACE"),
            Self::Comma => gazelle_core::Token::new(symbol_ids("COMMA"), "COMMA"),
        }
    }
}


#[doc(hidden)]
pub  union __MetaValue {
    __start: std::mem::ManuallyDrop<Ast>,
    __ident: std::mem::ManuallyDrop<String>,
    __sections: std::mem::ManuallyDrop<Ast>,
    __section: std::mem::ManuallyDrop<Ast>,
    __terminals_block: std::mem::ManuallyDrop<Ast>,
    __terminal_list: std::mem::ManuallyDrop<Ast>,
    __terminal_item: std::mem::ManuallyDrop<Ast>,
    __prec_terminals_block: std::mem::ManuallyDrop<Ast>,
    __prec_terminal_list: std::mem::ManuallyDrop<Ast>,
    __prec_terminal_item: std::mem::ManuallyDrop<Ast>,
    __rule: std::mem::ManuallyDrop<Ast>,
    __alts: std::mem::ManuallyDrop<Ast>,
    __alt: std::mem::ManuallyDrop<Ast>,
    __seq: std::mem::ManuallyDrop<Ast>,
    __unit: (),
}


#[doc(hidden)]
pub  struct __MetaReductionResult {
    value: __MetaValue,
    lhs_id: u32,
}

#[doc(hidden)]
fn __construct_grammar_def(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __start: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("grammar_def").0,
    }
}
#[doc(hidden)]
fn __construct_sections(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __sections: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("sections").0,
    }
}
#[doc(hidden)]
fn __construct_section(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __section: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("section").0,
    }
}
#[doc(hidden)]
fn __construct_terminals_block(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __terminals_block: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("terminals_block").0,
    }
}
#[doc(hidden)]
fn __construct_terminal_list(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __terminal_list: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("terminal_list").0,
    }
}
#[doc(hidden)]
fn __construct_terminal_item(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __terminal_item: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("terminal_item").0,
    }
}
#[doc(hidden)]
fn __construct_prec_terminals_block(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __prec_terminals_block: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("prec_terminals_block").0,
    }
}
#[doc(hidden)]
fn __construct_prec_terminal_list(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __prec_terminal_list: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("prec_terminal_list").0,
    }
}
#[doc(hidden)]
fn __construct_prec_terminal_item(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __prec_terminal_item: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("prec_terminal_item").0,
    }
}
#[doc(hidden)]
fn __construct_rule(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __rule: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("rule").0,
    }
}
#[doc(hidden)]
fn __construct_alts(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __alts: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("alts").0,
    }
}
#[doc(hidden)]
fn __construct_alt(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __alt: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("alt").0,
    }
}
#[doc(hidden)]
fn __construct_seq(value: Ast) -> __MetaReductionResult {
    __MetaReductionResult {
        value: __MetaValue { __seq: std::mem::ManuallyDrop::new(value) },
        lhs_id: __meta_table::symbol_id("seq").0,
    }
}


/// Parse error.
#[derive(Debug, Clone)]
pub  struct MetaError {
    /// The parser state when error occurred.
    pub state: usize,
}

/// Reduction variants with constructor functions.
pub  enum MetaReduction {
    GrammarDefKwGrammarIdentLbraceSectionsRbrace(fn(Ast) -> __MetaReductionResult, String, Ast),
    SectionsSectionsSection(fn(Ast) -> __MetaReductionResult, Ast, Ast),
    SectionsSection(fn(Ast) -> __MetaReductionResult, Ast),
    SectionTerminalsBlock(fn(Ast) -> __MetaReductionResult, Ast),
    SectionPrecTerminalsBlock(fn(Ast) -> __MetaReductionResult, Ast),
    SectionRule(fn(Ast) -> __MetaReductionResult, Ast),
    TerminalsBlockKwTerminalsLbraceTerminalListCommaRbrace(fn(Ast) -> __MetaReductionResult, Ast),
    TerminalsBlockKwTerminalsLbraceTerminalListRbrace(fn(Ast) -> __MetaReductionResult, Ast),
    TerminalsBlockKwTerminalsLbraceRbrace(fn(Ast) -> __MetaReductionResult),
    TerminalListTerminalListCommaTerminalItem(fn(Ast) -> __MetaReductionResult, Ast, Ast),
    TerminalListTerminalItem(fn(Ast) -> __MetaReductionResult, Ast),
    TerminalItemIdentColonIdent(fn(Ast) -> __MetaReductionResult, String, String),
    TerminalItemIdent(fn(Ast) -> __MetaReductionResult, String),
    PrecTerminalsBlockKwPrecTerminalsLbracePrecTerminalListCommaRbrace(fn(Ast) -> __MetaReductionResult, Ast),
    PrecTerminalsBlockKwPrecTerminalsLbracePrecTerminalListRbrace(fn(Ast) -> __MetaReductionResult, Ast),
    PrecTerminalsBlockKwPrecTerminalsLbraceRbrace(fn(Ast) -> __MetaReductionResult),
    PrecTerminalListPrecTerminalListCommaPrecTerminalItem(fn(Ast) -> __MetaReductionResult, Ast, Ast),
    PrecTerminalListPrecTerminalItem(fn(Ast) -> __MetaReductionResult, Ast),
    PrecTerminalItemIdentColonIdent(fn(Ast) -> __MetaReductionResult, String, String),
    RuleIdentColonIdentEqAltsSemi(fn(Ast) -> __MetaReductionResult, String, String, Ast),
    RuleIdentEqAltsSemi(fn(Ast) -> __MetaReductionResult, String, Ast),
    AltsAltsPipeAlt(fn(Ast) -> __MetaReductionResult, Ast, Ast),
    AltsAltsPipe(fn(Ast) -> __MetaReductionResult, Ast),
    AltsAlt(fn(Ast) -> __MetaReductionResult, Ast),
    AltSeqAtIdent(fn(Ast) -> __MetaReductionResult, Ast, String),
    AltSeq(fn(Ast) -> __MetaReductionResult, Ast),
    SeqSeqIdent(fn(Ast) -> __MetaReductionResult, Ast, String),
    SeqIdent(fn(Ast) -> __MetaReductionResult, String),
}

impl std::fmt::Debug for MetaReduction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GrammarDefKwGrammarIdentLbraceSectionsRbrace(..) => write!(f, "GrammarDefKwGrammarIdentLbraceSectionsRbrace"),
            Self::SectionsSectionsSection(..) => write!(f, "SectionsSectionsSection"),
            Self::SectionsSection(..) => write!(f, "SectionsSection"),
            Self::SectionTerminalsBlock(..) => write!(f, "SectionTerminalsBlock"),
            Self::SectionPrecTerminalsBlock(..) => write!(f, "SectionPrecTerminalsBlock"),
            Self::SectionRule(..) => write!(f, "SectionRule"),
            Self::TerminalsBlockKwTerminalsLbraceTerminalListCommaRbrace(..) => write!(f, "TerminalsBlockKwTerminalsLbraceTerminalListCommaRbrace"),
            Self::TerminalsBlockKwTerminalsLbraceTerminalListRbrace(..) => write!(f, "TerminalsBlockKwTerminalsLbraceTerminalListRbrace"),
            Self::TerminalsBlockKwTerminalsLbraceRbrace(..) => write!(f, "TerminalsBlockKwTerminalsLbraceRbrace"),
            Self::TerminalListTerminalListCommaTerminalItem(..) => write!(f, "TerminalListTerminalListCommaTerminalItem"),
            Self::TerminalListTerminalItem(..) => write!(f, "TerminalListTerminalItem"),
            Self::TerminalItemIdentColonIdent(..) => write!(f, "TerminalItemIdentColonIdent"),
            Self::TerminalItemIdent(..) => write!(f, "TerminalItemIdent"),
            Self::PrecTerminalsBlockKwPrecTerminalsLbracePrecTerminalListCommaRbrace(..) => write!(f, "PrecTerminalsBlockKwPrecTerminalsLbracePrecTerminalListCommaRbrace"),
            Self::PrecTerminalsBlockKwPrecTerminalsLbracePrecTerminalListRbrace(..) => write!(f, "PrecTerminalsBlockKwPrecTerminalsLbracePrecTerminalListRbrace"),
            Self::PrecTerminalsBlockKwPrecTerminalsLbraceRbrace(..) => write!(f, "PrecTerminalsBlockKwPrecTerminalsLbraceRbrace"),
            Self::PrecTerminalListPrecTerminalListCommaPrecTerminalItem(..) => write!(f, "PrecTerminalListPrecTerminalListCommaPrecTerminalItem"),
            Self::PrecTerminalListPrecTerminalItem(..) => write!(f, "PrecTerminalListPrecTerminalItem"),
            Self::PrecTerminalItemIdentColonIdent(..) => write!(f, "PrecTerminalItemIdentColonIdent"),
            Self::RuleIdentColonIdentEqAltsSemi(..) => write!(f, "RuleIdentColonIdentEqAltsSemi"),
            Self::RuleIdentEqAltsSemi(..) => write!(f, "RuleIdentEqAltsSemi"),
            Self::AltsAltsPipeAlt(..) => write!(f, "AltsAltsPipeAlt"),
            Self::AltsAltsPipe(..) => write!(f, "AltsAltsPipe"),
            Self::AltsAlt(..) => write!(f, "AltsAlt"),
            Self::AltSeqAtIdent(..) => write!(f, "AltSeqAtIdent"),
            Self::AltSeq(..) => write!(f, "AltSeq"),
            Self::SeqSeqIdent(..) => write!(f, "SeqSeqIdent"),
            Self::SeqIdent(..) => write!(f, "SeqIdent"),
        }
    }
}


/// Type-safe LR parser.
pub struct MetaParser {
    state_stack: Vec<usize>,
    value_stack: Vec<std::mem::ManuallyDrop<__MetaValue>>,
}

impl MetaParser {
    /// Create a new parser instance.
    pub fn new() -> Self {
        Self {
            state_stack: vec![0],
            value_stack: Vec::new(),
        }
    }

    /// Check if a reduction is needed given the lookahead.
    pub fn maybe_reduce(&mut self, lookahead: &Option<MetaTerminal>) -> Option<MetaReduction> {
        let state = self.current_state();
        let symbol_id = match lookahead {
            Some(t) => t.symbol_id().0,
            None => 0,
        };
        let action = self.lookup_action(state, symbol_id);

        match action & 3 {
            2 => {
                let rule = (action >> 2) as usize;
                self.do_reduce(rule)
            }
            3 if action != 3 => {
                if lookahead.is_none() {
                    let reduce_rule = (action >> 17) as usize;
                    self.do_reduce(reduce_rule)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Complete a reduction by pushing the result.
    pub fn reduce(&mut self, result: __MetaReductionResult) {
        self.value_stack.push(std::mem::ManuallyDrop::new(result.value));
        self.do_goto_after_reduce(result.lhs_id);
    }

    /// Shift (consume) a terminal.
    pub fn shift(&mut self, terminal: MetaTerminal) -> Result<(), MetaError> {
        let state = self.current_state();
        let symbol_id = terminal.symbol_id();
        let action = self.lookup_action(state, symbol_id.0);

        match action & 3 {
            0 => Err(MetaError { state }),
            1 => {
                let next_state = (action >> 2) as usize;
                self.do_shift(terminal, next_state);
                Ok(())
            }
            3 if action != 3 => {
                let shift_state = ((action >> 3) & 0x3FFF) as usize;
                self.do_shift(terminal, shift_state);
                Ok(())
            }
            _ => Err(MetaError { state }),
        }
    }

    /// Accept the parse result.
    pub fn accept(mut self) -> Result<Ast, MetaError> {
        let state = self.current_state();
        let action = self.lookup_action(state, 0);

        if action == 3 {
            if let Some(value) = self.value_stack.pop() {
                self.state_stack.pop();
                let union_val = std::mem::ManuallyDrop::into_inner(value);
                return Ok(unsafe { std::mem::ManuallyDrop::into_inner(union_val.__start) });
            }
        }
        Err(MetaError { state })
    }

    /// Get the current parser state.
    pub fn state(&self) -> usize {
        self.current_state()
    }

    fn current_state(&self) -> usize {
        *self.state_stack.last().unwrap()
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

    fn do_shift(&mut self, terminal: MetaTerminal, next_state: usize) {
        self.state_stack.push(next_state);
        match terminal {
            MetaTerminal::Semi => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Ident(v) => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __ident: std::mem::ManuallyDrop::new(v) }));
            }
            MetaTerminal::KwPrecTerminals => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Pipe => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::At => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Lbrace => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::KwGrammar => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Colon => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Eq => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::KwTerminals => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Rbrace => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
            MetaTerminal::Comma => {
                self.value_stack.push(std::mem::ManuallyDrop::new(__MetaValue { __unit: () }));
            }
        }

    }

    fn do_reduce(&mut self, rule: usize) -> Option<MetaReduction> {
        if rule == 0 { return None; }

        let (_, rhs_len) = __meta_table::RULES[rule];
        let rhs_len = rhs_len as usize;

        for _ in 0..rhs_len {
            self.state_stack.pop();
        }

        let original_rule_idx = rule - 1;

        let reduction = match original_rule_idx {
            0 => {
                self.value_stack.pop();
                let v3 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__sections) };
                self.value_stack.pop();
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                self.value_stack.pop();
                MetaReduction::GrammarDefKwGrammarIdentLbraceSectionsRbrace(__construct_grammar_def, v1, v3)
            }
            1 => {
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__section) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__sections) };
                MetaReduction::SectionsSectionsSection(__construct_sections, v0, v1)
            }
            2 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__section) };
                MetaReduction::SectionsSection(__construct_sections, v0)
            }
            3 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminals_block) };
                MetaReduction::SectionTerminalsBlock(__construct_section, v0)
            }
            4 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__prec_terminals_block) };
                MetaReduction::SectionPrecTerminalsBlock(__construct_section, v0)
            }
            5 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__rule) };
                MetaReduction::SectionRule(__construct_section, v0)
            }
            6 => {
                self.value_stack.pop();
                self.value_stack.pop();
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_list) };
                self.value_stack.pop();
                self.value_stack.pop();
                MetaReduction::TerminalsBlockKwTerminalsLbraceTerminalListCommaRbrace(__construct_terminals_block, v2)
            }
            7 => {
                self.value_stack.pop();
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_list) };
                self.value_stack.pop();
                self.value_stack.pop();
                MetaReduction::TerminalsBlockKwTerminalsLbraceTerminalListRbrace(__construct_terminals_block, v2)
            }
            8 => {
                self.value_stack.pop();
                self.value_stack.pop();
                self.value_stack.pop();
                MetaReduction::TerminalsBlockKwTerminalsLbraceRbrace(__construct_terminals_block)
            }
            9 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_item) };
                self.value_stack.pop();
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_list) };
                MetaReduction::TerminalListTerminalListCommaTerminalItem(__construct_terminal_list, v0, v2)
            }
            10 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__terminal_item) };
                MetaReduction::TerminalListTerminalItem(__construct_terminal_list, v0)
            }
            11 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                self.value_stack.pop();
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                MetaReduction::TerminalItemIdentColonIdent(__construct_terminal_item, v0, v2)
            }
            12 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                MetaReduction::TerminalItemIdent(__construct_terminal_item, v0)
            }
            13 => {
                self.value_stack.pop();
                self.value_stack.pop();
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__prec_terminal_list) };
                self.value_stack.pop();
                self.value_stack.pop();
                MetaReduction::PrecTerminalsBlockKwPrecTerminalsLbracePrecTerminalListCommaRbrace(__construct_prec_terminals_block, v2)
            }
            14 => {
                self.value_stack.pop();
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__prec_terminal_list) };
                self.value_stack.pop();
                self.value_stack.pop();
                MetaReduction::PrecTerminalsBlockKwPrecTerminalsLbracePrecTerminalListRbrace(__construct_prec_terminals_block, v2)
            }
            15 => {
                self.value_stack.pop();
                self.value_stack.pop();
                self.value_stack.pop();
                MetaReduction::PrecTerminalsBlockKwPrecTerminalsLbraceRbrace(__construct_prec_terminals_block)
            }
            16 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__prec_terminal_item) };
                self.value_stack.pop();
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__prec_terminal_list) };
                MetaReduction::PrecTerminalListPrecTerminalListCommaPrecTerminalItem(__construct_prec_terminal_list, v0, v2)
            }
            17 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__prec_terminal_item) };
                MetaReduction::PrecTerminalListPrecTerminalItem(__construct_prec_terminal_list, v0)
            }
            18 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                self.value_stack.pop();
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                MetaReduction::PrecTerminalItemIdentColonIdent(__construct_prec_terminal_item, v0, v2)
            }
            19 => {
                self.value_stack.pop();
                let v4 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alts) };
                self.value_stack.pop();
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                self.value_stack.pop();
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                MetaReduction::RuleIdentColonIdentEqAltsSemi(__construct_rule, v0, v2, v4)
            }
            20 => {
                self.value_stack.pop();
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alts) };
                self.value_stack.pop();
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                MetaReduction::RuleIdentEqAltsSemi(__construct_rule, v0, v2)
            }
            21 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alt) };
                self.value_stack.pop();
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alts) };
                MetaReduction::AltsAltsPipeAlt(__construct_alts, v0, v2)
            }
            22 => {
                self.value_stack.pop();
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alts) };
                MetaReduction::AltsAltsPipe(__construct_alts, v0)
            }
            23 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__alt) };
                MetaReduction::AltsAlt(__construct_alts, v0)
            }
            24 => {
                let v2 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                self.value_stack.pop();
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__seq) };
                MetaReduction::AltSeqAtIdent(__construct_alt, v0, v2)
            }
            25 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__seq) };
                MetaReduction::AltSeq(__construct_alt, v0)
            }
            26 => {
                let v1 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__seq) };
                MetaReduction::SeqSeqIdent(__construct_seq, v0, v1)
            }
            27 => {
                let v0 = unsafe { std::mem::ManuallyDrop::into_inner(std::mem::ManuallyDrop::into_inner(self.value_stack.pop().unwrap()).__ident) };
                MetaReduction::SeqIdent(__construct_seq, v0)
            }

            _ => return None,
        };

        Some(reduction)
    }

    fn do_goto_after_reduce(&mut self, lhs_id: u32) {
        let goto_state = self.current_state();
        let nt_index = lhs_id - __meta_table::NUM_TERMINALS - 1;
        if let Some(next_state) = self.lookup_goto(goto_state, nt_index) {
            self.state_stack.push(next_state);
        }
    }
}

impl Default for MetaParser {
    fn default() -> Self { Self::new() }
}

impl Drop for MetaParser {
    fn drop(&mut self) {
        while let Some(value) = self.value_stack.pop() {
            let state = self.state_stack.pop().unwrap();
            let sym_id = __meta_table::STATE_SYMBOL[state];
            unsafe {
                let union_val = std::mem::ManuallyDrop::into_inner(value);
                match sym_id {
                    1 => { std::mem::ManuallyDrop::into_inner(union_val.__ident); }
                    13 => { std::mem::ManuallyDrop::into_inner(union_val.__start); }
                    14 => { std::mem::ManuallyDrop::into_inner(union_val.__sections); }
                    15 => { std::mem::ManuallyDrop::into_inner(union_val.__section); }
                    16 => { std::mem::ManuallyDrop::into_inner(union_val.__terminals_block); }
                    17 => { std::mem::ManuallyDrop::into_inner(union_val.__terminal_list); }
                    18 => { std::mem::ManuallyDrop::into_inner(union_val.__terminal_item); }
                    19 => { std::mem::ManuallyDrop::into_inner(union_val.__prec_terminals_block); }
                    20 => { std::mem::ManuallyDrop::into_inner(union_val.__prec_terminal_list); }
                    21 => { std::mem::ManuallyDrop::into_inner(union_val.__prec_terminal_item); }
                    22 => { std::mem::ManuallyDrop::into_inner(union_val.__rule); }
                    23 => { std::mem::ManuallyDrop::into_inner(union_val.__alts); }
                    24 => { std::mem::ManuallyDrop::into_inner(union_val.__alt); }
                    25 => { std::mem::ManuallyDrop::into_inner(union_val.__seq); }

                    _ => {}
                }
            }
        }
    }
}



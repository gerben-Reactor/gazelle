//! C11 Parser POC for Gazelle
//!
//! Demonstrates two key innovations:
//! 1. Jourdan's typedef disambiguation via NAME TYPE/NAME VARIABLE lexer feedback
//! 2. Dynamic precedence parsing via `prec` terminals - collapses 10 expression levels into one rule

use std::collections::HashSet;

use gazelle::Precedence;
use gazelle_macros::grammar;

// =============================================================================
// Grammar Definition
// =============================================================================

grammar! {
    grammar C11 {
        start translation_unit_file;
        terminals {
            NAME: Name, TYPE, VARIABLE,
            CONSTANT, STRING_LITERAL,
            AUTO, BREAK, CASE, CHAR, CONST, CONTINUE, DEFAULT, DO, DOUBLE,
            ELSE, ENUM, EXTERN, FLOAT, FOR, GOTO, IF, INLINE, INT, LONG,
            REGISTER, RESTRICT, RETURN, SHORT, SIGNED, SIZEOF, STATIC,
            STRUCT, SWITCH, TYPEDEF, UNION, UNSIGNED, VOID, VOLATILE, WHILE,
            ALIGNAS, ALIGNOF, ATOMIC, BOOL, COMPLEX, GENERIC, IMAGINARY,
            NORETURN, STATIC_ASSERT, THREAD_LOCAL,
            LPAREN, RPAREN, LBRACE, RBRACE, LBRACK, RBRACK,
            SEMICOLON, COLON, COMMA, DOT, PTR, ELLIPSIS,
            TILDE, BANG,  // unary-only
            INC, DEC,
            ATOMIC_LPAREN,
            // Precedence terminals - expression hierarchy in one rule!
            // COMMA handled by grammar (expression vs assignment_expression).
            // Levels (higher = tighter binding):
            //   1: = += etc (EQ and ASSIGN, right-assoc)
            //   2: ?:       (QUESTION, right-assoc)
            //   3-12: binary ops (BINOP, STAR, AMP, PLUS, MINUS)
            prec EQ,       // level 1, right-assoc (= also used in initializers)
            prec ASSIGN,   // level 1, right-assoc (+= -= etc.)
            prec QUESTION, // level 2, right-assoc (ternary ? :)
            prec STAR,     // level 12 (also pointer decl, unary deref)
            prec AMP,      // level 7  (also unary address-of)
            prec PLUS,     // level 11 (also unary plus)
            prec MINUS,    // level 11 (also unary minus)
            prec BINOP,    // all other binary ops
        }

        // === option_* (rules 1-40) ===
        option_anonymous_2_ = _ | COMMA ELLIPSIS;
        option_argument_expression_list_ = _ | argument_expression_list;
        option_assignment_expression_ = _ | assignment_expression;
        option_block_item_list_ = _ | block_item_list;
        option_declaration_list_ = _ | declaration_list;
        option_declarator_ = _ | declarator;
        option_designation_ = _ | designation;
        option_designator_list_ = _ | designator_list;
        option_expression_ = _ | expression;
        option_general_identifier_ = _ | general_identifier;
        option_identifier_list_ = _ | identifier_list;
        option_init_declarator_list_declarator_typedefname__ = _ | init_declarator_list_declarator_typedefname_;
        option_init_declarator_list_declarator_varname__ = _ | init_declarator_list_declarator_varname_;
        option_pointer_ = _ | pointer;
        option_scoped_parameter_type_list__ = _ | scoped_parameter_type_list_;
        option_struct_declarator_list_ = _ | struct_declarator_list;
        option_type_qualifier_list_ = _ | type_qualifier_list;

        // === list_* (rules 41-69) ===
        // 41-43: list___anonymous_0_
        list_anonymous_0_ = _ | type_qualifier list_anonymous_0_ | alignment_specifier list_anonymous_0_;
        // 44-46: list___anonymous_1_
        list_anonymous_1_ = _ | type_qualifier list_anonymous_1_ | alignment_specifier list_anonymous_1_;
        // 47-48: list_declaration_specifier_
        list_declaration_specifier_ = _ | declaration_specifier list_declaration_specifier_;
        // 49-50: list_eq1_TYPEDEF_declaration_specifier_
        list_eq1_TYPEDEF_declaration_specifier_ = TYPEDEF list_declaration_specifier_
                                                | declaration_specifier list_eq1_TYPEDEF_declaration_specifier_;
        // 51-53: list_eq1_type_specifier_unique___anonymous_0_
        list_eq1_type_specifier_unique_anonymous_0_ = type_specifier_unique list_anonymous_0_
                                                    | type_qualifier list_eq1_type_specifier_unique_anonymous_0_
                                                    | alignment_specifier list_eq1_type_specifier_unique_anonymous_0_;
        // 54-55: list_eq1_type_specifier_unique_declaration_specifier_
        list_eq1_type_specifier_unique_declaration_specifier_ = type_specifier_unique list_declaration_specifier_
                                                              | declaration_specifier list_eq1_type_specifier_unique_declaration_specifier_;
        // 56-59: list_ge1_type_specifier_nonunique___anonymous_1_
        list_ge1_type_specifier_nonunique_anonymous_1_ = type_specifier_nonunique list_anonymous_1_
                                                       | type_specifier_nonunique list_ge1_type_specifier_nonunique_anonymous_1_
                                                       | type_qualifier list_ge1_type_specifier_nonunique_anonymous_1_
                                                       | alignment_specifier list_ge1_type_specifier_nonunique_anonymous_1_;
        // 60-62: list_ge1_type_specifier_nonunique_declaration_specifier_
        list_ge1_type_specifier_nonunique_declaration_specifier_ = type_specifier_nonunique list_declaration_specifier_
                                                                 | type_specifier_nonunique list_ge1_type_specifier_nonunique_declaration_specifier_
                                                                 | declaration_specifier list_ge1_type_specifier_nonunique_declaration_specifier_;
        // 63-65: list_eq1_eq1_TYPEDEF_type_specifier_unique_declaration_specifier_
        list_eq1_eq1_TYPEDEF_type_specifier_unique_declaration_specifier_ = TYPEDEF list_eq1_type_specifier_unique_declaration_specifier_
                                                                          | type_specifier_unique list_eq1_TYPEDEF_declaration_specifier_
                                                                          | declaration_specifier list_eq1_eq1_TYPEDEF_type_specifier_unique_declaration_specifier_;
        // 66-69: list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_
        list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_ = TYPEDEF list_ge1_type_specifier_nonunique_declaration_specifier_
                                                                             | type_specifier_nonunique list_eq1_TYPEDEF_declaration_specifier_
                                                                             | type_specifier_nonunique list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_
                                                                             | declaration_specifier list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_;

        // === Names (rules 70-75) ===
        typedef_name: String = NAME TYPE @typedef_name;
        var_name: String = NAME VARIABLE @var_name;
        typedef_name_spec = typedef_name;
        general_identifier: String = typedef_name @gi_typedef | var_name @gi_var;
        // save_context for scoped wrappers (returns Context for restore)
        save_context: Context = _ @save_context;

        // === Scoped wrappers (rules 76-82) ===
        // Each scoped rule: save context, parse inner, restore context
        scoped_compound_statement_ = save_context compound_statement @restore_compound;
        scoped_iteration_statement_ = save_context iteration_statement @restore_iteration;
        // Parameters: save at start, parse (declares params), save end context, restore start
        // Returns the END context for use by function declarators
        scoped_parameter_type_list_: Context = save_context parameter_type_list @scoped_params;
        scoped_selection_statement_ = save_context selection_statement @restore_selection;
        scoped_statement_ = save_context statement @restore_statement;
        // Declarators now carry context for function declarators
        declarator_varname: Declarator = declarator @decl_varname;
        declarator_typedefname: Declarator = declarator @register_typedef;

        // === Strings (rules 83-84) ===
        string_literal = STRING_LITERAL | string_literal STRING_LITERAL;

        // === Expressions (rules 85-170) ===
        primary_expression = var_name | CONSTANT | string_literal | LPAREN expression RPAREN | generic_selection;
        generic_selection = GENERIC LPAREN assignment_expression COMMA generic_assoc_list RPAREN;
        generic_assoc_list = generic_association | generic_assoc_list COMMA generic_association;
        generic_association = type_name COLON assignment_expression | DEFAULT COLON assignment_expression;

        postfix_expression = primary_expression
                           | postfix_expression LBRACK expression RBRACK
                           | postfix_expression LPAREN option_argument_expression_list_ RPAREN
                           | postfix_expression DOT general_identifier
                           | postfix_expression PTR general_identifier
                           | postfix_expression INC
                           | postfix_expression DEC
                           | LPAREN type_name RPAREN LBRACE initializer_list COMMA? RBRACE;

        argument_expression_list = assignment_expression | argument_expression_list COMMA assignment_expression;

        unary_expression = postfix_expression
                         | INC unary_expression
                         | DEC unary_expression
                         | unary_operator cast_expression
                         | SIZEOF unary_expression
                         | SIZEOF LPAREN type_name RPAREN
                         | ALIGNOF LPAREN type_name RPAREN;

        unary_operator = AMP | STAR | PLUS | MINUS | TILDE | BANG;  // & * + - ~ !

        cast_expression = unary_expression | LPAREN type_name RPAREN cast_expression;

        // Expression hierarchy collapsed using dynamic precedence (prec terminals).
        // Assignment_expression excludes comma (needed for function args, etc.).
        // Ternary ?: included via QUESTION prec terminal.
        //
        // SEMANTIC NOTE: Original C grammar restricts assignment LHS:
        //   assignment_expression = unary_expression '=' assignment_expression | ...
        // Our collapsed grammar uses:
        //   assignment_expression = assignment_expression '=' assignment_expression | ...
        //
        // Difference: `a + b = 5` (no parens) is a syntax error in original C,
        // but parses as `(a + b) = 5` here. Both grammars accept `(a + b) = 5`
        // (parentheses make it primary -> unary). Lvalue checking is deferred
        // to semantic analysis, which is standard practice for modern compilers.
        assignment_expression = cast_expression
                              | assignment_expression BINOP assignment_expression
                              | assignment_expression STAR assignment_expression
                              | assignment_expression AMP assignment_expression
                              | assignment_expression PLUS assignment_expression
                              | assignment_expression MINUS assignment_expression
                              | assignment_expression EQ assignment_expression
                              | assignment_expression ASSIGN assignment_expression
                              | assignment_expression QUESTION expression COLON assignment_expression;

        expression = assignment_expression | expression COMMA assignment_expression;
        constant_expression = assignment_expression;

        // === Declarations (rules 171-240) ===
        declaration = declaration_specifiers option_init_declarator_list_declarator_varname__ SEMICOLON
                    | declaration_specifiers_typedef option_init_declarator_list_declarator_typedefname__ SEMICOLON
                    | static_assert_declaration;

        // 174-177: declaration_specifier (NO type_specifier!)
        declaration_specifier = storage_class_specifier | type_qualifier | function_specifier | alignment_specifier;

        // 178-179: declaration_specifiers
        declaration_specifiers = list_eq1_type_specifier_unique_declaration_specifier_
                               | list_ge1_type_specifier_nonunique_declaration_specifier_;

        // 180-181: declaration_specifiers_typedef
        declaration_specifiers_typedef = list_eq1_eq1_TYPEDEF_type_specifier_unique_declaration_specifier_
                                       | list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_;

        // 182-189: init_declarator_list variants
        init_declarator_list_declarator_typedefname_ = init_declarator_declarator_typedefname_
                                                     | init_declarator_list_declarator_typedefname_ COMMA init_declarator_declarator_typedefname_;
        init_declarator_list_declarator_varname_ = init_declarator_declarator_varname_
                                                 | init_declarator_list_declarator_varname_ COMMA init_declarator_declarator_varname_;
        init_declarator_declarator_typedefname_ = declarator_typedefname | declarator_typedefname EQ c_initializer;
        init_declarator_declarator_varname_ = declarator_varname | declarator_varname EQ c_initializer;

        // 190-194: storage_class_specifier
        storage_class_specifier = EXTERN | STATIC | THREAD_LOCAL | AUTO | REGISTER;

        // 195-203: type_specifier_nonunique
        type_specifier_nonunique = CHAR | SHORT | INT | LONG | FLOAT | DOUBLE | SIGNED | UNSIGNED | COMPLEX;

        // 204-209: type_specifier_unique
        type_specifier_unique = VOID | BOOL | atomic_type_specifier | struct_or_union_specifier | enum_specifier | typedef_name_spec;

        // 210-215: struct
        struct_or_union_specifier = struct_or_union option_general_identifier_ LBRACE struct_declaration_list RBRACE
                                  | struct_or_union general_identifier;
        struct_or_union = STRUCT | UNION;
        struct_declaration_list = struct_declaration | struct_declaration_list struct_declaration;
        struct_declaration = specifier_qualifier_list option_struct_declarator_list_ SEMICOLON | static_assert_declaration;

        // 218-219: specifier_qualifier_list
        specifier_qualifier_list = list_eq1_type_specifier_unique_anonymous_0_
                                 | list_ge1_type_specifier_nonunique_anonymous_1_;

        // 220-223: struct_declarator
        struct_declarator_list = struct_declarator | struct_declarator_list COMMA struct_declarator;
        struct_declarator = declarator | option_declarator_ COLON constant_expression;

        // 224-230: enum
        enum_specifier = ENUM option_general_identifier_ LBRACE enumerator_list COMMA? RBRACE
                       | ENUM general_identifier;
        enumerator_list = enumerator | enumerator_list COMMA enumerator;
        // Enumerator declares the constant as a variable (shadows typedef)
        enumerator = enumeration_constant @decl_enum | enumeration_constant EQ constant_expression @decl_enum_expr;
        enumeration_constant: String = general_identifier @enum_const;

        // 231-232: atomic_type_specifier
        atomic_type_specifier = ATOMIC LPAREN type_name RPAREN | ATOMIC ATOMIC_LPAREN type_name RPAREN;

        // 233-238: type_qualifier, function_specifier
        type_qualifier = CONST | RESTRICT | VOLATILE | ATOMIC;
        function_specifier = INLINE | NORETURN;
        alignment_specifier = ALIGNAS LPAREN type_name RPAREN | ALIGNAS LPAREN constant_expression RPAREN;

        // 241-252: declarators
        // Declarators carry both name and optional context (for function declarators)
        declarator: Declarator = direct_declarator @decl_direct | pointer direct_declarator @decl_ptr;
        direct_declarator: Declarator = general_identifier @dd_ident
                          | LPAREN save_context declarator RPAREN @dd_paren
                          | direct_declarator LBRACK option_type_qualifier_list_ option_assignment_expression_ RBRACK @dd_other
                          | direct_declarator LBRACK STATIC option_type_qualifier_list_ assignment_expression RBRACK @dd_other
                          | direct_declarator LBRACK type_qualifier_list STATIC assignment_expression RBRACK @dd_other
                          | direct_declarator LBRACK option_type_qualifier_list_ STAR RBRACK @dd_other
                          | direct_declarator LPAREN scoped_parameter_type_list_ RPAREN @dd_func
                          | direct_declarator LPAREN save_context option_identifier_list_ RPAREN @dd_other_kr;

        pointer = STAR option_type_qualifier_list_ option_pointer_;
        type_qualifier_list = option_type_qualifier_list_ type_qualifier;

        // 253-259: parameters
        // parameter_type_list returns the context at its END (with params declared)
        parameter_type_list: Context = parameter_list option_anonymous_2_ save_context @param_ctx;
        parameter_list = parameter_declaration | parameter_list COMMA parameter_declaration;
        parameter_declaration = declaration_specifiers declarator_varname | declaration_specifiers abstract_declarator?;
        identifier_list = var_name | identifier_list COMMA var_name;

        // 260-271: type_name, abstract_declarator
        type_name = specifier_qualifier_list abstract_declarator?;
        abstract_declarator = pointer | direct_abstract_declarator | pointer direct_abstract_declarator;
        direct_abstract_declarator = LPAREN save_context abstract_declarator RPAREN
                                   | direct_abstract_declarator? LBRACK option_assignment_expression_ RBRACK
                                   | direct_abstract_declarator? LBRACK type_qualifier_list option_assignment_expression_ RBRACK
                                   | direct_abstract_declarator? LBRACK STATIC option_type_qualifier_list_ assignment_expression RBRACK
                                   | direct_abstract_declarator? LBRACK type_qualifier_list STATIC assignment_expression RBRACK
                                   | direct_abstract_declarator? LBRACK STAR RBRACK
                                   | LPAREN option_scoped_parameter_type_list__ RPAREN
                                   | direct_abstract_declarator LPAREN option_scoped_parameter_type_list__ RPAREN;

        // 272-279: initializer, designation
        c_initializer = assignment_expression | LBRACE initializer_list COMMA? RBRACE;
        initializer_list = option_designation_ c_initializer | initializer_list COMMA option_designation_ c_initializer;
        designation = designator_list EQ;
        designator_list = option_designator_list_ designator;
        designator = LBRACK constant_expression RBRACK | DOT general_identifier;

        // 280: static_assert_declaration
        static_assert_declaration = STATIC_ASSERT LPAREN constant_expression COMMA string_literal RPAREN SEMICOLON;

        // === Statements (rules 281-305) ===
        statement = labeled_statement | scoped_compound_statement_ | expression_statement
                  | scoped_selection_statement_ | scoped_iteration_statement_ | jump_statement;
        labeled_statement = general_identifier COLON statement | CASE constant_expression COLON statement | DEFAULT COLON statement;
        compound_statement = LBRACE option_block_item_list_ RBRACE;
        block_item_list = option_block_item_list_ block_item;
        block_item = declaration | statement;
        expression_statement = option_expression_ SEMICOLON;

        selection_statement = IF LPAREN expression RPAREN scoped_statement_ ELSE scoped_statement_
                            | IF LPAREN expression RPAREN scoped_statement_
                            | SWITCH LPAREN expression RPAREN scoped_statement_;

        iteration_statement = WHILE LPAREN expression RPAREN scoped_statement_
                            | DO scoped_statement_ WHILE LPAREN expression RPAREN SEMICOLON
                            | FOR LPAREN option_expression_ SEMICOLON option_expression_ SEMICOLON option_expression_ RPAREN scoped_statement_
                            | FOR LPAREN declaration option_expression_ SEMICOLON option_expression_ RPAREN scoped_statement_;

        jump_statement = GOTO general_identifier SEMICOLON | CONTINUE SEMICOLON | BREAK SEMICOLON | RETURN option_expression_ SEMICOLON;

        // === Translation unit (rules 306-313) ===
        translation_unit_file = external_declaration translation_unit_file | external_declaration;
        external_declaration = function_definition | declaration;
        // function_definition1: save context, then reinstall function params
        function_definition1: Context = declaration_specifiers declarator_varname @func_def1;
        // function_definition: parse body, then restore original context
        function_definition = function_definition1 option_declaration_list_ compound_statement @func_def;
        declaration_list = declaration | declaration_list declaration;
    }
}

// =============================================================================
// Placeholder types (no AST for now, just validate parsing)
// =============================================================================

// =============================================================================
// Typedef Context (Jourdan's approach: flat set with save/restore)
// =============================================================================

/// A context snapshot - the set of typedef names visible at a point
pub type Context = HashSet<String>;

/// Declarator with optional context for function declarators (Jourdan's approach)
#[derive(Clone, Debug)]
pub enum Declarator {
    /// Simple identifier declarator
    Identifier(String),
    /// Function declarator with saved context at end of parameters
    Function(String, Context),
    /// Other declarator (array, pointer, etc.)
    Other(String),
}

impl Declarator {
    pub fn name(&self) -> &str {
        match self {
            Declarator::Identifier(s) => s,
            Declarator::Function(s, _) => s,
            Declarator::Other(s) => s,
        }
    }

    /// Convert identifier to function declarator with context
    pub fn to_function(self, ctx: Context) -> Self {
        match self {
            Declarator::Identifier(s) => Declarator::Function(s, ctx),
            other => other, // Already function or other, don't override
        }
    }

    /// Convert identifier to other declarator
    pub fn to_other(self) -> Self {
        match self {
            Declarator::Identifier(s) => Declarator::Other(s),
            other => other,
        }
    }
}

/// Typedef context for tracking declared typedef names.
/// Uses Jourdan's approach: a single mutable set with save/restore.
pub struct TypedefContext {
    current: HashSet<String>,
}

impl TypedefContext {
    pub fn new() -> Self {
        Self {
            current: HashSet::new(),
        }
    }

    /// Test if name is a typedef
    pub fn is_typedef(&self, name: &str) -> bool {
        self.current.contains(name)
    }

    /// Declare a typedef name (adds to set)
    pub fn declare_typedef(&mut self, name: &str) {
        self.current.insert(name.to_string());
    }

    /// Declare a variable name (removes from set, shadowing any typedef)
    pub fn declare_varname(&mut self, name: &str) {
        self.current.remove(name);
    }

    /// Save the current context (returns a snapshot)
    pub fn save(&self) -> Context {
        self.current.clone()
    }

    /// Restore a saved context
    pub fn restore(&mut self, snapshot: Context) {
        self.current = snapshot;
    }
}

impl Default for TypedefContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Actions for the C11 parser (empty - just validate parsing)
pub struct CActions {
    pub ctx: TypedefContext,
}

impl CActions {
    pub fn new() -> Self {
        Self {
            ctx: TypedefContext::new(),
        }
    }
}

impl Default for CActions {
    fn default() -> Self {
        Self::new()
    }
}

impl C11Actions for CActions {
    type Name = String;
    type TypedefName = String;
    type VarName = String;
    type GeneralIdentifier = String;
    type DirectDeclarator = Declarator;
    type Declarator = Declarator;
    type DeclaratorVarname = Declarator;
    type DeclaratorTypedefname = Declarator;
    type SaveContext = Context;
    type EnumerationConstant = String;
    type ParameterTypeList = Context;
    type ScopedParameterTypeList = Context;
    type FunctionDefinition1 = Context;

    // Names
    fn typedef_name(&mut self, name: String) -> String { name }
    fn var_name(&mut self, name: String) -> String { name }
    fn gi_typedef(&mut self, name: String) -> String { name }
    fn gi_var(&mut self, name: String) -> String { name }

    // Save context (returns snapshot for scoped wrappers)
    fn save_context(&mut self) -> Context {
        self.ctx.save()
    }

    // Restore context functions
    fn restore_compound(&mut self, ctx: Context) { self.ctx.restore(ctx); }
    fn restore_iteration(&mut self, ctx: Context) { self.ctx.restore(ctx); }
    fn restore_selection(&mut self, ctx: Context) { self.ctx.restore(ctx); }
    fn restore_statement(&mut self, ctx: Context) { self.ctx.restore(ctx); }

    // parameter_type_list returns context at its end (with params declared)
    fn param_ctx(&mut self, ctx: Context) -> Context { ctx }

    // scoped_parameter_type_list_: save at start, parse params, restore, return end context
    fn scoped_params(&mut self, start_ctx: Context, end_ctx: Context) -> Context {
        self.ctx.restore(start_ctx); // Restore context before params
        end_ctx // Return the context with params for function declarator
    }

    // Direct declarator constructors
    fn dd_ident(&mut self, name: String) -> Declarator { Declarator::Identifier(name) }
    fn dd_paren(&mut self, _ctx: Context, d: Declarator) -> Declarator { d }
    fn dd_other(&mut self, d: Declarator) -> Declarator { d.to_other() }
    fn dd_other_kr(&mut self, d: Declarator, _ctx: Context) -> Declarator { d.to_other() }
    fn dd_func(&mut self, d: Declarator, ctx: Context) -> Declarator { d.to_function(ctx) }

    // Declarator pass-through
    fn decl_direct(&mut self, d: Declarator) -> Declarator { d }
    fn decl_ptr(&mut self, d: Declarator) -> Declarator { d.to_other() }

    // declarator_varname: declare name as variable, return declarator
    fn decl_varname(&mut self, d: Declarator) -> Declarator {
        self.ctx.declare_varname(d.name());
        d
    }

    // declarator_typedefname: declare name as typedef, return declarator
    fn register_typedef(&mut self, d: Declarator) -> Declarator {
        self.ctx.declare_typedef(d.name());
        d
    }

    // function_definition1: save context, reinstall function params
    fn func_def1(&mut self, d: Declarator) -> Context {
        let saved = self.ctx.save();
        // If this is a function declarator, restore its parameter context
        if let Declarator::Function(name, param_ctx) = &d {
            self.ctx.restore(param_ctx.clone());
            self.ctx.declare_varname(name); // Declare function name as variable
        }
        saved
    }

    // function_definition: restore context after body
    fn func_def(&mut self, ctx: Context) {
        self.ctx.restore(ctx);
    }

    // Enumeration constant - just pass through name
    fn enum_const(&mut self, name: String) -> String { name }

    // Enumerator - declares enum constant as variable (shadows typedef)
    fn decl_enum(&mut self, name: String) { self.ctx.declare_varname(&name); }
    fn decl_enum_expr(&mut self, name: String) { self.ctx.declare_varname(&name); }
}

// =============================================================================
// C Lexer with Typedef Feedback
// =============================================================================

/// C11 lexer with lexer feedback for typedef disambiguation
pub struct C11Lexer<'a> {
    lexer: gazelle::lexer::Lexer<'a>,
    /// Pending identifier - when Some, next call returns TYPE or VARIABLE
    /// based on is_typedef check at that moment (delayed decision)
    pending_ident: Option<String>,
}

impl<'a> C11Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: gazelle::lexer::Lexer::new(input),
            pending_ident: None,
        }
    }

    pub fn next(&mut self, ctx: &TypedefContext) -> Result<Option<C11Terminal<CActions>>, String> {
        use gazelle::lexer::Token;

        // If we have a pending identifier, emit TYPE or VARIABLE based on current context
        // This is the key: the decision is made NOW, not when NAME was seen
        if let Some(id) = self.pending_ident.take() {
            return Ok(Some(if ctx.is_typedef(&id) {
                C11Terminal::Type
            } else {
                C11Terminal::Variable
            }));
        }

        let tok = match self.lexer.next() {
            Some(Ok(t)) => t,
            Some(Err(e)) => return Err(e),
            None => return Ok(None),
        };

        Ok(Some(match tok {
            Token::Ident(s) => match s.as_str() {
                // Keywords
                "auto" => C11Terminal::Auto,
                "break" => C11Terminal::Break,
                "case" => C11Terminal::Case,
                "char" => C11Terminal::Char,
                "const" => C11Terminal::Const,
                "continue" => C11Terminal::Continue,
                "default" => C11Terminal::Default,
                "do" => C11Terminal::Do,
                "double" => C11Terminal::Double,
                "else" => C11Terminal::Else,
                "enum" => C11Terminal::Enum,
                "extern" => C11Terminal::Extern,
                "float" => C11Terminal::Float,
                "for" => C11Terminal::For,
                "goto" => C11Terminal::Goto,
                "if" => C11Terminal::If,
                "inline" => C11Terminal::Inline,
                "int" => C11Terminal::Int,
                "long" => C11Terminal::Long,
                "register" => C11Terminal::Register,
                "restrict" => C11Terminal::Restrict,
                "return" => C11Terminal::Return,
                "short" => C11Terminal::Short,
                "signed" => C11Terminal::Signed,
                "sizeof" => C11Terminal::Sizeof,
                "static" => C11Terminal::Static,
                "struct" => C11Terminal::Struct,
                "switch" => C11Terminal::Switch,
                "typedef" => C11Terminal::Typedef,
                "union" => C11Terminal::Union,
                "unsigned" => C11Terminal::Unsigned,
                "void" => C11Terminal::Void,
                "volatile" => C11Terminal::Volatile,
                "while" => C11Terminal::While,
                // C11 keywords
                "_Alignas" => C11Terminal::Alignas,
                "_Alignof" => C11Terminal::Alignof,
                "_Atomic" => C11Terminal::Atomic,
                "_Bool" => C11Terminal::Bool,
                "_Complex" => C11Terminal::Complex,
                "_Generic" => C11Terminal::Generic,
                "_Imaginary" => C11Terminal::Imaginary,
                "_Noreturn" => C11Terminal::Noreturn,
                "_Static_assert" => C11Terminal::StaticAssert,
                "_Thread_local" => C11Terminal::ThreadLocal,
                // Identifier - queue TYPE/VARIABLE for next call
                _ => {
                    self.pending_ident = Some(s.clone());
                    C11Terminal::Name(s)
                }
            },
            Token::Num(_) => C11Terminal::Constant,
            Token::Str(_) => C11Terminal::StringLiteral,
            Token::Char(_) => C11Terminal::Constant,
            Token::Punct(c) => match c {
                '(' => C11Terminal::Lparen,
                ')' => C11Terminal::Rparen,
                '{' => C11Terminal::Lbrace,
                '}' => C11Terminal::Rbrace,
                '[' => C11Terminal::Lbrack,
                ']' => C11Terminal::Rbrack,
                ';' => C11Terminal::Semicolon,
                ',' => C11Terminal::Comma,
                _ => return self.next(ctx),
            },
            Token::Op(s) => match s.as_str() {
                // Non-expression operators
                ":" => C11Terminal::Colon,
                "." => C11Terminal::Dot,
                "->" => C11Terminal::Ptr,
                "..." => C11Terminal::Ellipsis,
                // Unary-only operators
                "~" => C11Terminal::Tilde,
                "!" => C11Terminal::Bang,
                "++" => C11Terminal::Inc,
                "--" => C11Terminal::Dec,
                // Precedence terminals for expressions
                // Level 1: assignment (right-assoc)
                // EQ is separate because = is also used in initializers, enums, designators
                "=" => C11Terminal::Eq(Precedence::right(1)),
                "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|=" | "^=" | "<<=" | ">>="
                    => C11Terminal::Assign(Precedence::right(1)),
                // Level 2: ternary (right-assoc)
                "?" => C11Terminal::Question(Precedence::right(2)),
                // Level 3: ||
                "||" => C11Terminal::Binop(Precedence::left(3)),
                // Level 4: &&
                "&&" => C11Terminal::Binop(Precedence::left(4)),
                // Level 5: |
                "|" => C11Terminal::Binop(Precedence::left(5)),
                // Level 6: ^
                "^" => C11Terminal::Binop(Precedence::left(6)),
                // Level 7: & (also unary address-of)
                "&" => C11Terminal::Amp(Precedence::left(7)),
                // Level 8: == !=
                "==" | "!=" => C11Terminal::Binop(Precedence::left(8)),
                // Level 9: < > <= >=
                "<" | ">" | "<=" | ">=" => C11Terminal::Binop(Precedence::left(9)),
                // Level 10: << >>
                "<<" | ">>" => C11Terminal::Binop(Precedence::left(10)),
                // Level 11: + - (also unary)
                "+" => C11Terminal::Plus(Precedence::left(11)),
                "-" => C11Terminal::Minus(Precedence::left(11)),
                // Level 12: * / % (STAR also pointer/unary deref)
                "*" => C11Terminal::Star(Precedence::left(12)),
                "/" | "%" => C11Terminal::Binop(Precedence::left(12)),
                _ => return Err(format!("Unknown operator: {}", s)),
            },
        }))
    }
}

// =============================================================================
// Parse Function
// =============================================================================

/// Parse C11 source code
pub fn parse(input: &str) -> Result<(), String> {
    // Strip preprocessor lines (lines starting with #)
    let preprocessed: String = input
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    parse_debug(&preprocessed, true)
}

/// Parse C11 source code with optional debug output
pub fn parse_debug(input: &str, debug: bool) -> Result<(), String> {
    let mut parser = C11Parser::<CActions>::new();
    let mut actions = CActions::new();
    let mut lexer = C11Lexer::new(input);
    let mut token_count = 0;

    loop {
        let tok = lexer.next(&actions.ctx)?;
        match tok {
            Some(t) => {
                if debug {
                    let name = match &t {
                        C11Terminal::Int => "INT",
                        C11Terminal::Name(_) => "NAME",
                        C11Terminal::Variable => "VARIABLE",
                        C11Terminal::Type => "TYPE",
                        C11Terminal::Semicolon => "SEMICOLON",
                        C11Terminal::Typedef => "Typedef",
                        _ => "Other",

                    };
                    eprintln!("Token {}: {} (before state={})", token_count, name, parser.state());
                }
                token_count += 1;
                parser.push(t, &mut actions).map_err(|e| {
                    format!("Parse error at token {}: {:?}", token_count, e)
                })?;
                if debug {
                    eprintln!("  -> after push, state={}", parser.state());
                }
            }
            None => break,
        }
    }

    parser.finish(&mut actions).map_err(|e| format!("Finish error: {:?}", e))?;
    Ok(())
}

// =============================================================================
// Main
// =============================================================================

fn main() {
    println!("C11 Parser POC for Gazelle");
    println!();
    println!("Key innovations demonstrated:");
    println!("1. prec OP terminal - collapses 13+ expression rules into ONE");
    println!("2. Lexer feedback - Jourdan's typedef disambiguation");
    println!();
    println!("Run tests with: cargo test --example c11");
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_expression() {
        let mut lexer = C11Lexer::new("int x;");
        let ctx = TypedefContext::new();

        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Int)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Name(_))));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Variable)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Semicolon)));
    }

    #[test]
    fn test_typedef_context() {
        let mut ctx = TypedefContext::new();
        assert!(!ctx.is_typedef("T"));

        ctx.declare_typedef("T");
        assert!(ctx.is_typedef("T"));

        // Save context, add S, then restore
        let saved = ctx.save();
        assert!(ctx.is_typedef("T"));
        ctx.declare_typedef("S");
        assert!(ctx.is_typedef("S"));

        ctx.restore(saved);
        assert!(!ctx.is_typedef("S"));
        assert!(ctx.is_typedef("T"));
    }

    #[test]
    fn test_typedef_shadowing() {
        let mut ctx = TypedefContext::new();
        ctx.declare_typedef("T");
        assert!(ctx.is_typedef("T"));

        // Shadow T with a variable declaration
        ctx.declare_varname("T");
        assert!(!ctx.is_typedef("T"));
    }

    #[test]
    fn test_local_scope_simple() {
        // Simplified version of local_scope.c
        let code = r#"
typedef int T;
void f(void) {
  T y = 1;
}
"#;
        parse(code).expect("basic function with typedef should parse");
    }

    #[test]
    fn test_local_scope_with_shadow() {
        // Local variable shadows typedef
        let code = r#"
typedef int T;
void f(void) {
  int T;
  T = 1;
}
"#;
        parse(code).expect("typedef shadow should parse");
    }

    #[test]
    fn test_local_scope_with_if() {
        // Scoped shadow in if block
        let code = r#"
typedef int T;
void f(void) {
  if(1) {
    int T;
    T = 1;
  }
  T x = 1;
}
"#;
        let preprocessed = code.lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n");
        parse_debug(&preprocessed, false).expect("scoped typedef shadow should parse");
    }

    // Note: argument_scope test requires tracking context through declarators
    // (like Jourdan's reinstall_function_context). This is a known limitation.

    #[test]
    fn test_typedef_lexer_feedback() {
        let mut ctx = TypedefContext::new();
        ctx.declare_typedef("MyType");

        let mut lexer = C11Lexer::new("MyType x");

        let tok1 = lexer.next(&ctx).unwrap();
        assert!(matches!(tok1, Some(C11Terminal::Name(_))));

        let tok2 = lexer.next(&ctx).unwrap();
        assert!(matches!(tok2, Some(C11Terminal::Type)));

        let tok3 = lexer.next(&ctx).unwrap();
        assert!(matches!(tok3, Some(C11Terminal::Name(_))));

        let tok4 = lexer.next(&ctx).unwrap();
        assert!(matches!(tok4, Some(C11Terminal::Variable)));
    }

    #[test]
    fn test_keywords() {
        let ctx = TypedefContext::new();
        let mut lexer = C11Lexer::new("int void struct typedef if while for");

        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Int)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Void)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Struct)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::Typedef)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::If)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::While)));
        assert!(matches!(lexer.next(&ctx).unwrap(), Some(C11Terminal::For)));
    }

    // =========================================================================
    // C11parser test suite
    // =========================================================================

    /// Helper to parse a C file and report success/failure
    fn parse_c_file(path: &str) -> Result<(), String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path, e))?;
        parse(&content).map_err(|e| format!("{}: {}", path, e))
    }

    #[test]
    fn test_simple_parse() {
        // Test "int;" first (declaration with no variables)
        eprintln!("\n--- Parsing 'int;' ---");
        let result1 = parse_debug("int;", true);
        if let Err(e) = &result1 {
            eprintln!("'int;' Error: {}", e);
        }

        // Test "typedef int T;" (typedef)
        eprintln!("\n--- Parsing 'typedef int T;' ---");
        let result2 = parse_debug("typedef int T;", true);
        if let Err(e) = &result2 {
            eprintln!("'typedef int T;' Error: {}", e);
        }

        result1.unwrap();
        result2.unwrap();
    }

    /// Test files that should parse successfully
    const C11_TEST_FILES: &[&str] = &[
        "examples/c11/C11parser/tests/typedef_star.c",
        "examples/c11/C11parser/tests/variable_star.c",
        "examples/c11/C11parser/tests/local_typedef.c",
        "examples/c11/C11parser/tests/block_scope.c",
        "examples/c11/C11parser/tests/declaration_ambiguity.c",
        "examples/c11/C11parser/tests/enum.c",
        "examples/c11/C11parser/tests/enum_shadows_typedef.c",
        "examples/c11/C11parser/tests/enum_constant_visibility.c",
        "examples/c11/C11parser/tests/namespaces.c",
        "examples/c11/C11parser/tests/local_scope.c",
        "examples/c11/C11parser/tests/if_scopes.c",
        "examples/c11/C11parser/tests/loop_scopes.c",
        "examples/c11/C11parser/tests/no_local_scope.c",
        "examples/c11/C11parser/tests/function_parameter_scope.c",
        "examples/c11/C11parser/tests/function_parameter_scope_extends.c",
        "examples/c11/C11parser/tests/argument_scope.c",
        "examples/c11/C11parser/tests/control-scope.c",
        "examples/c11/C11parser/tests/dangling_else.c",
        "examples/c11/C11parser/tests/dangling_else_lookahead.c",
        "examples/c11/C11parser/tests/dangling_else_lookahead.if.c",
        "examples/c11/C11parser/tests/parameter_declaration_ambiguity.c",
        "examples/c11/C11parser/tests/parameter_declaration_ambiguity.test.c",
        "examples/c11/C11parser/tests/bitfield_declaration_ambiguity.c",
        "examples/c11/C11parser/tests/bitfield_declaration_ambiguity.ok.c",
        "examples/c11/C11parser/tests/expressions.c",
        "examples/c11/C11parser/tests/statements.c",
        "examples/c11/C11parser/tests/types.c",
        "examples/c11/C11parser/tests/declarators.c",
        "examples/c11/C11parser/tests/designator.c",
        "examples/c11/C11parser/tests/function-decls.c",
        "examples/c11/C11parser/tests/struct-recursion.c",
        "examples/c11/C11parser/tests/long-long-struct.c",
        "examples/c11/C11parser/tests/c-namespace.c",
        "examples/c11/C11parser/tests/enum-trick.c",
        "examples/c11/C11parser/tests/char-literal-printing.c",
        // C11-specific features
        "examples/c11/C11parser/tests/c11-noreturn.c",
        "examples/c11/C11parser/tests/c1x-alignas.c",
        "examples/c11/C11parser/tests/atomic.c",
        "examples/c11/C11parser/tests/atomic_parenthesis.c",
        "examples/c11/C11parser/tests/aligned_struct_c18.c",

        "examples/c11/C11parser/tests/declarator_visibility.c",
    ];

    #[test]
    fn test_c11parser_suite() {
        let mut passed = 0;
        let mut failed = Vec::new();

        for file in C11_TEST_FILES {
            match parse_c_file(file) {
                Ok(()) => {
                    passed += 1;
                    println!("PASS: {}", file);
                }
                Err(e) => {
                    failed.push(e);
                    println!("FAIL: {}", file);
                }
            }
        }

        println!("\n{} passed, {} failed", passed, failed.len());

        if !failed.is_empty() {
            for err in &failed {
                eprintln!("FAIL: {}", err);
            }
            panic!("{} tests failed", failed.len());
        }
    }

    // =========================================================================
    // Expression evaluation tests - verify precedence using actual C11 lexer
    // =========================================================================

    /// Operator enum to distinguish BINOP operators.
    /// Note: Multiple C operators share precedence levels (e.g., == and != both at level 8).
    /// We pick one representative per level since we're testing precedence, not exact semantics.
    #[derive(Clone, Copy, Debug)]
    #[allow(dead_code)]
    enum BinOp {
        Or, And, BitOr, BitXor, Eq, Ne, Lt, Gt, Le, Ge, Shl, Shr, Div, Mod,
    }

    // Minimal expression grammar for evaluation testing
    // Simplified: all expression levels map to i64, just tests precedence
    grammar! {
        grammar Expr {
            start expr;
            terminals {
                NUM: Num,
                LPAREN, RPAREN,
                COLON,
                TILDE, BANG,
                INC, DEC,
                prec EQ,
                prec QUESTION,
                prec STAR,
                prec AMP,
                prec PLUS,
                prec MINUS,
                prec BINOP: Binop,
            }

            // Simplified: term handles primary/postfix/unary/cast
            term: Term = NUM @eval_num
                       | LPAREN expr RPAREN @eval_paren
                       | INC term @eval_preinc
                       | DEC term @eval_predec
                       | AMP term @eval_addr
                       | STAR term @eval_deref
                       | PLUS term @eval_uplus
                       | MINUS term @eval_neg
                       | TILDE term @eval_bitnot
                       | BANG term @eval_lognot
                       | term INC @eval_postinc
                       | term DEC @eval_postdec;

            // Binary expression with dynamic precedence
            expr: Expr = term @eval_term
                       | expr BINOP expr @eval_binop
                       | expr STAR expr @eval_mul
                       | expr AMP expr @eval_bitand
                       | expr PLUS expr @eval_add
                       | expr MINUS expr @eval_sub
                       | expr EQ expr @eval_assign
                       | expr QUESTION expr COLON expr @eval_ternary;
        }
    }

    struct Eval;

    impl ExprActions for Eval {
        type Num = i64;
        type Binop = BinOp;
        type Term = i64;
        type Expr = i64;

        fn eval_num(&mut self, n: i64) -> i64 { n }
        fn eval_paren(&mut self, e: i64) -> i64 { e }
        fn eval_term(&mut self, e: i64) -> i64 { e }
        fn eval_preinc(&mut self, e: i64) -> i64 { e + 1 }
        fn eval_predec(&mut self, e: i64) -> i64 { e - 1 }
        fn eval_postinc(&mut self, e: i64) -> i64 { e }
        fn eval_postdec(&mut self, e: i64) -> i64 { e }
        fn eval_addr(&mut self, e: i64) -> i64 { e }
        fn eval_deref(&mut self, e: i64) -> i64 { e }
        fn eval_uplus(&mut self, e: i64) -> i64 { e }
        fn eval_neg(&mut self, e: i64) -> i64 { -e }
        fn eval_bitnot(&mut self, e: i64) -> i64 { !e }
        fn eval_lognot(&mut self, e: i64) -> i64 { if e == 0 { 1 } else { 0 } }

        fn eval_binop(&mut self, l: i64, op: BinOp, r: i64) -> i64 {
            match op {
                BinOp::Or => if l != 0 || r != 0 { 1 } else { 0 },
                BinOp::And => if l != 0 && r != 0 { 1 } else { 0 },
                BinOp::BitOr => l | r,
                BinOp::BitXor => l ^ r,
                BinOp::Eq => if l == r { 1 } else { 0 },
                BinOp::Ne => if l != r { 1 } else { 0 },
                BinOp::Lt => if l < r { 1 } else { 0 },
                BinOp::Gt => if l > r { 1 } else { 0 },
                BinOp::Le => if l <= r { 1 } else { 0 },
                BinOp::Ge => if l >= r { 1 } else { 0 },
                BinOp::Shl => l << r,
                BinOp::Shr => l >> r,
                BinOp::Div => l / r,
                BinOp::Mod => l % r,
            }
        }
        fn eval_mul(&mut self, l: i64, r: i64) -> i64 { l * r }
        fn eval_bitand(&mut self, l: i64, r: i64) -> i64 { l & r }
        fn eval_add(&mut self, l: i64, r: i64) -> i64 { l + r }
        fn eval_sub(&mut self, l: i64, r: i64) -> i64 { l - r }
        fn eval_assign(&mut self, _l: i64, r: i64) -> i64 { r }
        fn eval_ternary(&mut self, c: i64, t: i64, e: i64) -> i64 { if c != 0 { t } else { e } }
    }

    /// Convert C11 terminal to expression terminal, using actual C11 lexer
    fn c11_to_expr(tok: C11Terminal<CActions>) -> Option<ExprTerminal<Eval>> {
        Some(match tok {
            C11Terminal::Constant => {
                // For simplicity, constants become 0 - we'll handle numbers specially
                ExprTerminal::Num(0)
            }
            C11Terminal::Lparen => ExprTerminal::Lparen,
            C11Terminal::Rparen => ExprTerminal::Rparen,
            C11Terminal::Colon => ExprTerminal::Colon,
            C11Terminal::Tilde => ExprTerminal::Tilde,
            C11Terminal::Bang => ExprTerminal::Bang,
            C11Terminal::Inc => ExprTerminal::Inc,
            C11Terminal::Dec => ExprTerminal::Dec,
            C11Terminal::Eq(p) => ExprTerminal::Eq(p),
            C11Terminal::Question(p) => ExprTerminal::Question(p),
            C11Terminal::Star(p) => ExprTerminal::Star(p),
            C11Terminal::Amp(p) => ExprTerminal::Amp(p),
            C11Terminal::Plus(p) => ExprTerminal::Plus(p),
            C11Terminal::Minus(p) => ExprTerminal::Minus(p),
            C11Terminal::Binop(p) => {
                // We need to figure out which binop from precedence level
                let op = match p.level() {
                    3 => BinOp::Or,
                    4 => BinOp::And,
                    5 => BinOp::BitOr,
                    6 => BinOp::BitXor,
                    8 => BinOp::Eq,  // or Ne - can't distinguish, but same prec
                    9 => BinOp::Lt,  // or Gt/Le/Ge
                    10 => BinOp::Shl, // or Shr
                    12 => BinOp::Div, // or Mod
                    _ => return None,
                };
                ExprTerminal::Binop(op, p)
            }
            // Skip tokens we don't need for expression evaluation
            _ => return None,
        })
    }

    /// Evaluate a C expression using the C11 lexer
    fn eval_c_expr(input: &str) -> Result<i64, String> {
        use gazelle::lexer::Token;

        let mut parser = ExprParser::<Eval>::new();
        let mut actions = Eval;
        let ctx = TypedefContext::new();

        // Use gazelle's lexer directly for number extraction
        let mut lexer = gazelle::lexer::Lexer::new(input);
        let mut c11_lexer = C11Lexer::new(input);

        // We need to handle numbers specially since C11Terminal::Constant loses the value
        // Rebuild tokens with actual number values
        let mut tokens = Vec::new();
        loop {
            // Get raw token for number values
            let raw = lexer.next();
            let c11 = c11_lexer.next(&ctx).map_err(|e| e)?;

            match (raw, c11) {
                (Some(Ok(Token::Num(s))), Some(C11Terminal::Constant)) => {
                    let n: i64 = s.parse().unwrap_or(0);
                    tokens.push(ExprTerminal::Num(n));
                }
                (_, Some(tok)) => {
                    if let Some(expr_tok) = c11_to_expr(tok) {
                        tokens.push(expr_tok);
                    }
                }
                (_, None) => break,
            }
        }

        for tok in tokens {
            parser.push(tok, &mut actions).map_err(|e| format!("{:?}", e))?;
        }

        parser.finish(&mut actions).map_err(|e| format!("{:?}", e))
    }

    #[test]
    fn test_expr_precedence() {
        // Multiplicative > Additive
        assert_eq!(eval_c_expr("1 + 2 * 3").unwrap(), 7);
        assert_eq!(eval_c_expr("2 * 3 + 1").unwrap(), 7);
        assert_eq!(eval_c_expr("(1 + 2) * 3").unwrap(), 9);
    }

    #[test]
    fn test_expr_associativity() {
        // Left-associative
        assert_eq!(eval_c_expr("10 - 3 - 2").unwrap(), 5);   // (10-3)-2
        assert_eq!(eval_c_expr("100 / 10 / 2").unwrap(), 5); // (100/10)/2
        // Right-associative ternary
        assert_eq!(eval_c_expr("1 ? 2 : 0 ? 3 : 4").unwrap(), 2);
        assert_eq!(eval_c_expr("0 ? 2 : 1 ? 3 : 4").unwrap(), 3);
    }

    #[test]
    fn test_expr_all_precedence_levels() {
        // Test each C precedence level
        assert_eq!(eval_c_expr("1 || 0").unwrap(), 1);       // level 3
        assert_eq!(eval_c_expr("1 && 1").unwrap(), 1);       // level 4
        assert_eq!(eval_c_expr("5 | 2").unwrap(), 7);        // level 5
        assert_eq!(eval_c_expr("7 ^ 3").unwrap(), 4);        // level 6
        assert_eq!(eval_c_expr("7 & 3").unwrap(), 3);        // level 7
        assert_eq!(eval_c_expr("2 == 2").unwrap(), 1);       // level 8
        assert_eq!(eval_c_expr("1 < 2").unwrap(), 1);        // level 9
        assert_eq!(eval_c_expr("1 << 3").unwrap(), 8);       // level 10
        assert_eq!(eval_c_expr("3 + 4").unwrap(), 7);        // level 11
        assert_eq!(eval_c_expr("3 * 4").unwrap(), 12);       // level 12
    }

    #[test]
    fn test_expr_mixed_precedence() {
        assert_eq!(eval_c_expr("1 || 0 && 0").unwrap(), 1);   // && before ||
        assert_eq!(eval_c_expr("1 + 1 == 2").unwrap(), 1);    // + before ==
        assert_eq!(eval_c_expr("1 + 2 < 4").unwrap(), 1);     // + before <
        assert_eq!(eval_c_expr("1 + 2 * 3 + 4").unwrap(), 11);
    }

    #[test]
    fn test_expr_unary() {
        assert_eq!(eval_c_expr("-5").unwrap(), -5);
        assert_eq!(eval_c_expr("!0").unwrap(), 1);
        assert_eq!(eval_c_expr("!1").unwrap(), 0);
        assert_eq!(eval_c_expr("~0").unwrap(), -1);
    }

    #[test]
    fn test_expr_ternary() {
        assert_eq!(eval_c_expr("1 ? 2 : 3").unwrap(), 2);
        assert_eq!(eval_c_expr("0 ? 2 : 3").unwrap(), 3);
        assert_eq!(eval_c_expr("1 + 1 ? 2 : 3").unwrap(), 2);  // + before ?
    }
}

use std::collections::HashSet;

use gazelle_macros::gazelle;

gazelle! {
    pub(crate) grammar C11 {
        start translation_unit_file;
        expect 3 rr;  // typedef_name ambiguity
        expect 1 sr;  // dangling else
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
            BUILTIN_VA_ARG,
            prec EQ,
            prec QUESTION,
            prec STAR,
            prec AMP,
            prec PLUS,
            prec MINUS,
            prec BINOP
        }

        // === option_* ===
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

        // === list_* ===
        list_anonymous_0_ = _ | type_qualifier list_anonymous_0_ | alignment_specifier list_anonymous_0_;
        list_anonymous_1_ = _ | type_qualifier list_anonymous_1_ | alignment_specifier list_anonymous_1_;
        list_declaration_specifier_ = _ | declaration_specifier list_declaration_specifier_;
        list_eq1_TYPEDEF_declaration_specifier_ = TYPEDEF list_declaration_specifier_
                                                | declaration_specifier list_eq1_TYPEDEF_declaration_specifier_;
        list_eq1_type_specifier_unique_anonymous_0_ = type_specifier_unique list_anonymous_0_
                                                    | type_qualifier list_eq1_type_specifier_unique_anonymous_0_
                                                    | alignment_specifier list_eq1_type_specifier_unique_anonymous_0_;
        list_eq1_type_specifier_unique_declaration_specifier_ = type_specifier_unique list_declaration_specifier_
                                                              | declaration_specifier list_eq1_type_specifier_unique_declaration_specifier_;
        list_ge1_type_specifier_nonunique_anonymous_1_ = type_specifier_nonunique list_anonymous_1_
                                                       | type_specifier_nonunique list_ge1_type_specifier_nonunique_anonymous_1_
                                                       | type_qualifier list_ge1_type_specifier_nonunique_anonymous_1_
                                                       | alignment_specifier list_ge1_type_specifier_nonunique_anonymous_1_;
        list_ge1_type_specifier_nonunique_declaration_specifier_ = type_specifier_nonunique list_declaration_specifier_
                                                                 | type_specifier_nonunique list_ge1_type_specifier_nonunique_declaration_specifier_
                                                                 | declaration_specifier list_ge1_type_specifier_nonunique_declaration_specifier_;
        list_eq1_eq1_TYPEDEF_type_specifier_unique_declaration_specifier_ = TYPEDEF list_eq1_type_specifier_unique_declaration_specifier_
                                                                          | type_specifier_unique list_eq1_TYPEDEF_declaration_specifier_
                                                                          | declaration_specifier list_eq1_eq1_TYPEDEF_type_specifier_unique_declaration_specifier_;
        list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_ = TYPEDEF list_ge1_type_specifier_nonunique_declaration_specifier_
                                                                             | type_specifier_nonunique list_eq1_TYPEDEF_declaration_specifier_
                                                                             | type_specifier_nonunique list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_
                                                                             | declaration_specifier list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_;

        // === Names ===
        typedef_name: Name = NAME TYPE;
        var_name: Name = NAME VARIABLE;
        typedef_name_spec = typedef_name;
        general_identifier: Name = typedef_name | var_name;
        save_context: Context = _ @save_context;

        // === Scoped wrappers ===
        scoped_compound_statement_ = save_context compound_statement @restore_compound;
        scoped_iteration_statement_ = save_context iteration_statement @restore_iteration;
        scoped_parameter_type_list_: Context = save_context parameter_type_list @scoped_params;
        scoped_selection_statement_ = save_context selection_statement @restore_selection;
        scoped_statement_ = save_context statement @restore_statement;
        declarator_varname: Declarator = declarator @decl_varname;
        declarator_typedefname: Declarator = declarator @register_typedef;

        // === Strings ===
        string_literal = STRING_LITERAL | string_literal STRING_LITERAL;

        // === Expressions ===
        primary_expression = var_name | CONSTANT | string_literal | LPAREN expression RPAREN | generic_selection
                           | BUILTIN_VA_ARG LPAREN assignment_expression COMMA type_name RPAREN;
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

        unary_operator = AMP | STAR | PLUS | MINUS | TILDE | BANG;

        cast_expression = unary_expression | LPAREN type_name RPAREN cast_expression;

        assignment_expression = cast_expression
                              | assignment_expression BINOP assignment_expression
                              | assignment_expression STAR assignment_expression
                              | assignment_expression AMP assignment_expression
                              | assignment_expression PLUS assignment_expression
                              | assignment_expression MINUS assignment_expression
                              | assignment_expression EQ assignment_expression
                              | assignment_expression QUESTION expression COLON assignment_expression;

        expression = assignment_expression | expression COMMA assignment_expression;
        constant_expression = assignment_expression;

        // === Declarations ===
        declaration = declaration_specifiers option_init_declarator_list_declarator_varname__ SEMICOLON
                    | declaration_specifiers_typedef option_init_declarator_list_declarator_typedefname__ SEMICOLON
                    | static_assert_declaration;

        declaration_specifier = storage_class_specifier | type_qualifier | function_specifier | alignment_specifier;

        declaration_specifiers = list_eq1_type_specifier_unique_declaration_specifier_
                               | list_ge1_type_specifier_nonunique_declaration_specifier_;

        declaration_specifiers_typedef = list_eq1_eq1_TYPEDEF_type_specifier_unique_declaration_specifier_
                                       | list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_;

        init_declarator_list_declarator_typedefname_ = init_declarator_declarator_typedefname_
                                                     | init_declarator_list_declarator_typedefname_ COMMA init_declarator_declarator_typedefname_;
        init_declarator_list_declarator_varname_ = init_declarator_declarator_varname_
                                                 | init_declarator_list_declarator_varname_ COMMA init_declarator_declarator_varname_;
        init_declarator_declarator_typedefname_ = declarator_typedefname | declarator_typedefname EQ c_initializer;
        init_declarator_declarator_varname_ = declarator_varname | declarator_varname EQ c_initializer;

        storage_class_specifier = EXTERN | STATIC | THREAD_LOCAL | AUTO | REGISTER;

        type_specifier_nonunique = CHAR | SHORT | INT | LONG | FLOAT | DOUBLE | SIGNED | UNSIGNED | COMPLEX;

        type_specifier_unique = VOID | BOOL | atomic_type_specifier | struct_or_union_specifier | enum_specifier | typedef_name_spec;

        struct_or_union_specifier = struct_or_union option_general_identifier_ LBRACE struct_declaration_list RBRACE
                                  | struct_or_union general_identifier;
        struct_or_union = STRUCT | UNION;
        struct_declaration_list = struct_declaration | struct_declaration_list struct_declaration;
        struct_declaration = specifier_qualifier_list option_struct_declarator_list_ SEMICOLON | static_assert_declaration;

        specifier_qualifier_list = list_eq1_type_specifier_unique_anonymous_0_
                                 | list_ge1_type_specifier_nonunique_anonymous_1_;

        struct_declarator_list = struct_declarator | struct_declarator_list COMMA struct_declarator;
        struct_declarator = declarator | option_declarator_ COLON constant_expression;

        enum_specifier = ENUM option_general_identifier_ LBRACE enumerator_list COMMA? RBRACE
                       | ENUM general_identifier;
        enumerator_list = enumerator | enumerator_list COMMA enumerator;
        enumerator = enumeration_constant @decl_enum | enumeration_constant EQ constant_expression @decl_enum_expr;
        enumeration_constant: Name = general_identifier;

        atomic_type_specifier = ATOMIC LPAREN type_name RPAREN | ATOMIC ATOMIC_LPAREN type_name RPAREN;

        type_qualifier = CONST | RESTRICT | VOLATILE | ATOMIC;
        function_specifier = INLINE | NORETURN;
        alignment_specifier = ALIGNAS LPAREN type_name RPAREN | ALIGNAS LPAREN constant_expression RPAREN;

        declarator: Declarator = direct_declarator | pointer direct_declarator @decl_ptr;
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

        parameter_type_list: Context = parameter_list option_anonymous_2_ save_context @param_ctx;
        parameter_list = parameter_declaration | parameter_list COMMA parameter_declaration;
        parameter_declaration = declaration_specifiers declarator_varname | declaration_specifiers abstract_declarator?;
        identifier_list = var_name | identifier_list COMMA var_name;

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

        c_initializer = assignment_expression | LBRACE initializer_list COMMA? RBRACE;
        initializer_list = option_designation_ c_initializer | initializer_list COMMA option_designation_ c_initializer;
        designation = designator_list EQ;
        designator_list = option_designator_list_ designator;
        designator = LBRACK constant_expression RBRACK | DOT general_identifier;

        static_assert_declaration = STATIC_ASSERT LPAREN constant_expression COMMA string_literal RPAREN SEMICOLON;

        // === Statements ===
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

        // === Translation unit ===
        translation_unit_file = external_declaration translation_unit_file | external_declaration;
        external_declaration = function_definition | declaration;
        function_definition1: Context = declaration_specifiers declarator_varname @func_def1;
        function_definition = function_definition1 option_declaration_list_ compound_statement @func_def;
        declaration_list = declaration | declaration_list declaration;
    }
}

// === Typedef context types ===

pub type Context = HashSet<String>;

#[derive(Clone, Debug)]
pub enum Declarator {
    Identifier(String),
    Function(String, Context),
    Other(String),
}

impl Declarator {
    pub fn name(&self) -> &str {
        match self {
            Self::Identifier(s) | Self::Function(s, _) | Self::Other(s) => s,
        }
    }

    pub fn to_function(self, ctx: Context) -> Self {
        match self {
            Self::Identifier(s) => Self::Function(s, ctx),
            other => other,
        }
    }

    pub fn to_other(self) -> Self {
        match self {
            Self::Identifier(s) => Self::Other(s),
            other => other,
        }
    }
}

pub struct TypedefContext {
    current: HashSet<String>,
}

impl TypedefContext {
    pub fn new() -> Self {
        Self { current: HashSet::new() }
    }

    pub fn is_typedef(&self, name: &str) -> bool {
        self.current.contains(name)
    }

    pub fn declare_typedef(&mut self, name: &str) {
        self.current.insert(name.to_string());
    }

    pub fn declare_varname(&mut self, name: &str) {
        self.current.remove(name);
    }

    pub fn save(&self) -> Context {
        self.current.clone()
    }

    pub fn restore(&mut self, snapshot: Context) {
        self.current = snapshot;
    }
}

// === Actions ===

pub struct CActions {
    pub ctx: TypedefContext,
}

impl CActions {
    pub fn new() -> Self {
        let mut ctx = TypedefContext::new();
        // GCC built-in types that appear in preprocessed system headers
        ctx.declare_typedef("__builtin_va_list");
        Self { ctx }
    }
}

impl C11Types for CActions {
    type Name = String;
    type Declarator = Declarator;
    type Context = Context;
}

impl C11Actions for CActions {
    fn save_context(&mut self) -> Result<Context, gazelle::ParseError> {
        Ok(self.ctx.save())
    }

    fn restore_compound(&mut self, ctx: Context) -> Result<(), gazelle::ParseError> { self.ctx.restore(ctx); Ok(()) }
    fn restore_iteration(&mut self, ctx: Context) -> Result<(), gazelle::ParseError> { self.ctx.restore(ctx); Ok(()) }
    fn restore_selection(&mut self, ctx: Context) -> Result<(), gazelle::ParseError> { self.ctx.restore(ctx); Ok(()) }
    fn restore_statement(&mut self, ctx: Context) -> Result<(), gazelle::ParseError> { self.ctx.restore(ctx); Ok(()) }

    fn param_ctx(&mut self, ctx: Context) -> Result<Context, gazelle::ParseError> { Ok(ctx) }

    fn scoped_params(&mut self, start_ctx: Context, end_ctx: Context) -> Result<Context, gazelle::ParseError> {
        self.ctx.restore(start_ctx);
        Ok(end_ctx)
    }

    fn dd_ident(&mut self, name: String) -> Result<Declarator, gazelle::ParseError> { Ok(Declarator::Identifier(name)) }
    fn dd_paren(&mut self, _ctx: Context, d: Declarator) -> Result<Declarator, gazelle::ParseError> { Ok(d) }
    fn dd_other(&mut self, d: Declarator) -> Result<Declarator, gazelle::ParseError> { Ok(d.to_other()) }
    fn dd_other_kr(&mut self, d: Declarator, _ctx: Context) -> Result<Declarator, gazelle::ParseError> { Ok(d.to_other()) }
    fn dd_func(&mut self, d: Declarator, ctx: Context) -> Result<Declarator, gazelle::ParseError> { Ok(d.to_function(ctx)) }

    fn decl_ptr(&mut self, d: Declarator) -> Result<Declarator, gazelle::ParseError> { Ok(d.to_other()) }

    fn decl_varname(&mut self, d: Declarator) -> Result<Declarator, gazelle::ParseError> {
        self.ctx.declare_varname(d.name());
        Ok(d)
    }

    fn register_typedef(&mut self, d: Declarator) -> Result<Declarator, gazelle::ParseError> {
        self.ctx.declare_typedef(d.name());
        Ok(d)
    }

    fn func_def1(&mut self, d: Declarator) -> Result<Context, gazelle::ParseError> {
        let saved = self.ctx.save();
        if let Declarator::Function(name, param_ctx) = &d {
            self.ctx.restore(param_ctx.clone());
            self.ctx.declare_varname(name);
        }
        Ok(saved)
    }

    fn func_def(&mut self, ctx: Context) -> Result<(), gazelle::ParseError> {
        self.ctx.restore(ctx);
        Ok(())
    }

    fn decl_enum(&mut self, name: String) -> Result<(), gazelle::ParseError> { self.ctx.declare_varname(&name); Ok(()) }
    fn decl_enum_expr(&mut self, name: String) -> Result<(), gazelle::ParseError> { self.ctx.declare_varname(&name); Ok(()) }
}

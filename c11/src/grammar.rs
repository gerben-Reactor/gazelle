use std::collections::HashSet;

use gazelle_macros::gazelle;

use crate::ast::*;

gazelle! {
    pub(crate) grammar C11 {
        start translation_unit_file;
        expect 3 rr;  // typedef_name ambiguity
        expect 1 sr;  // dangling else
        terminals {
            NAME: Name, TYPE, VARIABLE,
            CONSTANT: Name, STRING_LITERAL: Name,
            AUTO, BREAK, CASE, CHAR, CONST, CONTINUE, DEFAULT, DO, DOUBLE,
            ELSE, ENUM, EXTERN, FLOAT, FOR, GOTO, IF, INLINE, INT, LONG,
            REGISTER, RESTRICT, RETURN, SHORT, SIGNED, SIZEOF, STATIC,
            STRUCT, SWITCH, TYPEDEF, UNION, UNSIGNED, VOID, VOLATILE, WHILE,
            ALIGNAS, ALIGNOF, ATOMIC, BOOL, COMPLEX, GENERIC, IMAGINARY,
            NORETURN, STATIC_ASSERT, THREAD_LOCAL,
            LPAREN, RPAREN, LBRACE, RBRACE, LBRACK, RBRACK,
            SEMICOLON, COLON, COMMA, DOT, PTR, ELLIPSIS,
            TILDE, BANG,
            INC, DEC,
            ATOMIC_LPAREN,
            BUILTIN_VA_ARG,
            prec EQ,
            prec QUESTION,
            prec STAR,
            prec AMP,
            prec PLUS,
            prec MINUS,
            prec BINOP: Op
        }

        // Compound option (can't inline with ?)
        variadic_suffix = COMMA ELLIPSIS;

        // === Declaration specifier lists (Jourdan's typedef disambiguation) ===
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
        typedef_name_spec: Name = typedef_name;
        general_identifier: Name = typedef_name | var_name;
        save_context: Context = _ @save_context;

        // === Scoped wrappers ===
        scoped_compound_statement_: Stmt = save_context compound_statement @restore_compound;
        scoped_iteration_statement_: Stmt = save_context iteration_statement @restore_iteration;
        scoped_parameter_type_list_: Context = save_context parameter_type_list @scoped_params;
        scoped_selection_statement_: Stmt = save_context selection_statement @restore_selection;
        scoped_statement_: Stmt = save_context statement @restore_statement;
        declarator_varname: Declarator = declarator @decl_varname;
        declarator_typedefname: Declarator = declarator @register_typedef;

        // === Strings ===
        str_lit: Name = STRING_LITERAL | str_lit STRING_LITERAL @str_concat;

        // === Expressions ===
        primary_expression: ExprNode = var_name @prim_var
                           | CONSTANT @prim_const
                           | str_lit @prim_str
                           | LPAREN expression RPAREN
                           | generic_selection
                           | BUILTIN_VA_ARG LPAREN assignment_expression COMMA type_name RPAREN @prim_va_arg;
        generic_selection: ExprNode = GENERIC LPAREN assignment_expression COMMA (generic_association % COMMA) RPAREN @prim_generic;
        generic_association: GenericAssoc = type_name COLON assignment_expression @ga_type | DEFAULT COLON assignment_expression @ga_default;

        postfix_expression: ExprNode = primary_expression
                           | postfix_expression LBRACK expression RBRACK @post_index
                           | postfix_expression LPAREN RPAREN @post_call_empty
                           | postfix_expression LPAREN (assignment_expression % COMMA) RPAREN @post_call
                           | postfix_expression DOT general_identifier @post_member
                           | postfix_expression PTR general_identifier @post_ptr_member
                           | postfix_expression INC @post_inc
                           | postfix_expression DEC @post_dec
                           | LPAREN type_name RPAREN LBRACE initializer_list COMMA? RBRACE @post_compound_lit;

        unary_expression: ExprNode = postfix_expression
                         | INC unary_expression @pre_inc
                         | DEC unary_expression @pre_dec
                         | unary_operator cast_expression @unary_op
                         | SIZEOF unary_expression @sizeof_expr
                         | SIZEOF LPAREN type_name RPAREN @sizeof_type
                         | ALIGNOF LPAREN type_name RPAREN @alignof_type;

        unary_operator: UnaryOp = AMP @op_addr | STAR @op_deref | PLUS @op_plus | MINUS @op_neg | TILDE @op_bitnot | BANG @op_lognot;

        cast_expression: ExprNode = unary_expression | LPAREN type_name RPAREN cast_expression @cast;

        assignment_expression: ExprNode = cast_expression
                              | assignment_expression BINOP assignment_expression @binop
                              | assignment_expression STAR assignment_expression @mul
                              | assignment_expression AMP assignment_expression @bitand
                              | assignment_expression PLUS assignment_expression @add
                              | assignment_expression MINUS assignment_expression @sub
                              | assignment_expression EQ assignment_expression @assign
                              | assignment_expression QUESTION expression COLON assignment_expression @ternary;

        expression: ExprNode = assignment_expression | expression COMMA assignment_expression @comma;
        constant_expression: ExprNode = assignment_expression;

        // === Declarations ===
        init_declarator_typedefname = declarator_typedefname @push_td | declarator_typedefname EQ c_initializer @push_td_init;
        init_declarator_varname = declarator_varname @push_decl | declarator_varname EQ c_initializer @push_decl_init;
        init_declarator_list_typedefname = (init_declarator_typedefname % COMMA);
        init_declarator_list_varname = (init_declarator_varname % COMMA);

        declaration: Decl = declaration_specifiers init_declarator_list_varname? SEMICOLON @decl_var
                    | declaration_specifiers_typedef init_declarator_list_typedefname? SEMICOLON @decl_typedef
                    | static_assert_declaration @decl_static_assert;

        declaration_specifier = storage_class_specifier | type_qualifier | function_specifier | alignment_specifier;

        declaration_specifiers = list_eq1_type_specifier_unique_declaration_specifier_
                               | list_ge1_type_specifier_nonunique_declaration_specifier_;

        declaration_specifiers_typedef = list_eq1_eq1_TYPEDEF_type_specifier_unique_declaration_specifier_
                                       | list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_;

        storage_class_specifier = EXTERN @sc_extern | STATIC @sc_static | THREAD_LOCAL @sc_thread
                                | AUTO @sc_auto | REGISTER @sc_register;

        type_specifier_nonunique = CHAR @ts_char | SHORT @ts_short | INT @ts_int | LONG @ts_long
                                 | FLOAT @ts_float | DOUBLE @ts_double
                                 | SIGNED @ts_signed | UNSIGNED @ts_unsigned | COMPLEX @ts_complex;

        type_specifier_unique = VOID @ts_void | BOOL @ts_bool
                              | atomic_type_specifier @ts_pending
                              | struct_or_union_specifier @ts_pending
                              | enum_specifier @ts_pending
                              | typedef_name_spec @ts_typedef;

        struct_or_union_specifier = struct_or_union general_identifier? LBRACE struct_declaration+ RBRACE @sou_def
                                  | struct_or_union general_identifier @sou_ref;
        struct_or_union = STRUCT @sou_struct | UNION @sou_union;
        struct_declarator_list = (struct_declarator % COMMA);
        struct_declaration = specifier_qualifier_list struct_declarator_list? SEMICOLON @struct_member | static_assert_declaration;

        specifier_qualifier_list = list_eq1_type_specifier_unique_anonymous_0_
                                 | list_ge1_type_specifier_nonunique_anonymous_1_;

        struct_declarator = declarator @sd_decl | declarator? COLON constant_expression @sd_bitfield;

        enum_specifier = ENUM general_identifier? LBRACE (enumerator % COMMA) COMMA? RBRACE @enum_def
                       | ENUM general_identifier @enum_ref;
        enumerator = enumeration_constant @decl_enum | enumeration_constant EQ constant_expression @decl_enum_expr;
        enumeration_constant: Name = general_identifier;

        atomic_type_specifier = ATOMIC LPAREN type_name RPAREN @push_atomic
                              | ATOMIC ATOMIC_LPAREN type_name RPAREN @push_atomic;

        type_qualifier = CONST @tq_const | RESTRICT @tq_restrict | VOLATILE @tq_volatile | ATOMIC @tq_atomic;
        function_specifier = INLINE @fs_inline | NORETURN @fs_noreturn;
        alignment_specifier = ALIGNAS LPAREN type_name RPAREN @as_type
                            | ALIGNAS LPAREN constant_expression RPAREN @as_expr;

        declarator: Declarator = direct_declarator | pointer direct_declarator @decl_ptr;
        direct_declarator: Declarator = general_identifier @dd_ident
                          | LPAREN save_context declarator RPAREN @dd_paren
                          | direct_declarator LBRACK type_qualifier_list? assignment_expression? RBRACK @dd_array
                          | direct_declarator LBRACK STATIC type_qualifier_list? assignment_expression RBRACK @dd_array_s
                          | direct_declarator LBRACK type_qualifier_list STATIC assignment_expression RBRACK @dd_array_qs
                          | direct_declarator LBRACK type_qualifier_list? STAR RBRACK @dd_vla
                          | direct_declarator LPAREN scoped_parameter_type_list_ RPAREN @dd_func
                          | direct_declarator LPAREN save_context identifier_list? RPAREN @dd_kr;

        pointer: PtrDepth = STAR type_qualifier_list? pointer? @make_ptr;
        type_qualifier_list = type_qualifier+;

        parameter_type_list: Context = (parameter_declaration % COMMA) variadic_suffix? save_context @param_ctx;
        parameter_declaration = declaration_specifiers declarator_varname @push_param
                              | declaration_specifiers abstract_declarator? @push_anon_param;
        identifier_list = (var_name % COMMA);

        type_name: TypeName = specifier_qualifier_list abstract_declarator? @make_type_name;
        abstract_declarator: Derived = pointer @abs_ptr
                            | direct_abstract_declarator @abs_direct
                            | pointer direct_abstract_declarator @abs_ptr_direct;
        direct_abstract_declarator: Derived = LPAREN save_context abstract_declarator RPAREN @dabs_paren
                                   | direct_abstract_declarator? LBRACK assignment_expression? RBRACK @dabs_array
                                   | direct_abstract_declarator? LBRACK type_qualifier_list assignment_expression? RBRACK @dabs_array_q
                                   | direct_abstract_declarator? LBRACK STATIC type_qualifier_list? assignment_expression RBRACK @dabs_array_s
                                   | direct_abstract_declarator? LBRACK type_qualifier_list STATIC assignment_expression RBRACK @dabs_array_qs
                                   | direct_abstract_declarator? LBRACK STAR RBRACK @dabs_vla
                                   | LPAREN scoped_parameter_type_list_? RPAREN @dabs_func0
                                   | direct_abstract_declarator LPAREN scoped_parameter_type_list_? RPAREN @dabs_func;

        c_initializer: Init = assignment_expression @init_expr | LBRACE initializer_list COMMA? RBRACE @init_braced;
        initializer_list = designation? c_initializer @push_init | initializer_list COMMA designation? c_initializer @push_init;
        designation = designator_list EQ;
        designator_list = designator+;
        designator = LBRACK constant_expression RBRACK | DOT general_identifier;

        static_assert_declaration = STATIC_ASSERT LPAREN constant_expression COMMA str_lit RPAREN SEMICOLON;

        // === Statements ===
        statement: Stmt = labeled_statement | scoped_compound_statement_ | expression_statement
                  | scoped_selection_statement_ | scoped_iteration_statement_ | jump_statement;
        labeled_statement: Stmt = general_identifier COLON statement @labeled
                         | CASE constant_expression COLON statement @case_label
                         | DEFAULT COLON statement @default_label;
        compound_statement: Stmt = LBRACE block_item* RBRACE @compound;
        block_item: BlockItem = declaration @block_decl | statement @block_stmt;
        expression_statement: Stmt = expression? SEMICOLON @expr_stmt;

        selection_statement: Stmt = IF LPAREN expression RPAREN scoped_statement_ ELSE scoped_statement_ @if_else
                            | IF LPAREN expression RPAREN scoped_statement_ @if_stmt
                            | SWITCH LPAREN expression RPAREN scoped_statement_ @switch_stmt;

        iteration_statement: Stmt = WHILE LPAREN expression RPAREN scoped_statement_ @while_stmt
                            | DO scoped_statement_ WHILE LPAREN expression RPAREN SEMICOLON @do_while
                            | FOR LPAREN expression? SEMICOLON expression? SEMICOLON expression? RPAREN scoped_statement_ @for_expr
                            | FOR LPAREN declaration expression? SEMICOLON expression? RPAREN scoped_statement_ @for_decl;

        jump_statement: Stmt = GOTO general_identifier SEMICOLON @goto_stmt
                      | CONTINUE SEMICOLON @continue_stmt
                      | BREAK SEMICOLON @break_stmt
                      | RETURN expression? SEMICOLON @return_stmt;

        // === Translation unit ===
        translation_unit_file = external_declaration+;
        external_declaration = function_definition | declaration @top_decl;
        function_definition1: Context = declaration_specifiers declarator_varname @func_def1;
        function_definition = function_definition1 declaration* compound_statement @func_def;
    }
}

// === Typedef context types ===

pub type Context = HashSet<String>;

pub struct Declarator {
    name: String,
    derived: Vec<DerivedType>,
    kind: DeclKind,
}

enum DeclKind { Ident, Func(Context), Other }

impl Declarator {
    fn new(name: String) -> Self { Self { name, derived: vec![], kind: DeclKind::Ident } }
    pub fn name(&self) -> &str { &self.name }
    // Only set kind when it's still Ident — preserves the innermost function context
    // through nested declarators like (*f(params))(return_params)
    fn set_func(&mut self, ctx: Context) {
        if matches!(self.kind, DeclKind::Ident) { self.kind = DeclKind::Func(ctx); }
    }
    fn set_other(&mut self) {
        if matches!(self.kind, DeclKind::Ident) { self.kind = DeclKind::Other; }
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
    // Specifier accumulation stack — frames pushed by save_context, sou_struct/sou_union
    spec_stack: Vec<Vec<DeclSpec>>,
    // Pending type specifier (struct/enum/atomic → ts_pending)
    pending_type_spec: Option<TypeSpec>,
    // Pending struct/union flag
    pending_sou: StructOrUnion,
    // Declaration accumulation
    pending_decls: Vec<InitDeclarator>,
    // Struct member accumulation
    pending_struct_members: Vec<StructMember>,
    pending_member_decls: Vec<MemberDeclarator>,
    // Enum accumulation
    pending_enumerators: Vec<Enumerator>,
    // Initializer list accumulation
    pending_inits: Vec<InitItem>,
    // Parameter accumulation
    pending_params: Vec<Param>,
    current_variadic: bool,
    // Function definition state
    current_func: Option<String>,
    current_return_specs: Vec<DeclSpec>,
    current_return_derived: Vec<DerivedType>,
    current_params: Vec<Param>,
    // Result
    pub unit: TranslationUnit,
}

impl CActions {
    pub fn new() -> Self {
        let mut ctx = TypedefContext::new();
        ctx.declare_typedef("__builtin_va_list");
        Self {
            ctx,
            spec_stack: vec![vec![]],
            pending_type_spec: None,
            pending_sou: StructOrUnion::Struct,
            pending_decls: vec![],
            pending_struct_members: vec![],
            pending_member_decls: vec![],
            pending_enumerators: vec![],
            pending_inits: vec![],
            pending_params: vec![],
            current_variadic: false,
            current_func: None,
            current_return_specs: vec![],
            current_return_derived: vec![],
            current_params: vec![],
            unit: TranslationUnit { functions: vec![] },
        }
    }

    fn push_spec(&mut self, spec: DeclSpec) {
        self.spec_stack.last_mut().unwrap().push(spec);
    }

    // Take specs from the top frame (without popping the frame)
    fn drain_specs(&mut self) -> Vec<DeclSpec> {
        std::mem::take(self.spec_stack.last_mut().unwrap())
    }

    fn push_spec_frame(&mut self) {
        self.spec_stack.push(vec![]);
    }

    fn pop_spec_frame(&mut self) {
        self.spec_stack.pop();
    }
}

impl C11Types for CActions {
    type Name = String;
    type Declarator = Declarator;
    type Context = Context;
    type Op = Op;
    type ExprNode = ExprNode;
    type UnaryOp = UnaryOp;
    type Stmt = Stmt;
    type BlockItem = BlockItem;
    type Init = Init;
    type Decl = Decl;
    type TypeName = TypeName;
    type PtrDepth = u32;
    type Derived = Vec<DerivedType>;
    type GenericAssoc = GenericAssoc;
}

type R<T> = Result<T, gazelle::ParseError>;

impl C11Actions for CActions {
    // === Struct members ===
    fn struct_member(&mut self, _decls: Option<()>) -> R<()> {
        let specs = self.drain_specs();
        let declarators = std::mem::take(&mut self.pending_member_decls);
        self.pending_struct_members.push(StructMember { specs, declarators });
        Ok(())
    }
    fn sd_decl(&mut self, d: Declarator) -> R<()> {
        self.pending_member_decls.push(MemberDeclarator {
            name: Some(d.name.clone()), derived: d.derived, bitfield: None,
        });
        Ok(())
    }
    fn sd_bitfield(&mut self, d: Option<Declarator>, bits: ExprNode) -> R<()> {
        let (name, derived) = match d {
            Some(d) => (Some(d.name.clone()), d.derived),
            None => (None, vec![]),
        };
        self.pending_member_decls.push(MemberDeclarator { name, derived, bitfield: Some(bits) });
        Ok(())
    }

    // === Storage class specifiers ===
    fn sc_extern(&mut self) -> R<()> { self.push_spec(DeclSpec::Storage(StorageClass::Extern)); Ok(()) }
    fn sc_static(&mut self) -> R<()> { self.push_spec(DeclSpec::Storage(StorageClass::Static)); Ok(()) }
    fn sc_thread(&mut self) -> R<()> { self.push_spec(DeclSpec::Storage(StorageClass::ThreadLocal)); Ok(()) }
    fn sc_auto(&mut self) -> R<()> { self.push_spec(DeclSpec::Storage(StorageClass::Auto)); Ok(()) }
    fn sc_register(&mut self) -> R<()> { self.push_spec(DeclSpec::Storage(StorageClass::Register)); Ok(()) }

    // === Type specifiers (nonunique) ===
    fn ts_char(&mut self) -> R<()> { self.push_spec(DeclSpec::Type(TypeSpec::Char)); Ok(()) }
    fn ts_short(&mut self) -> R<()> { self.push_spec(DeclSpec::Type(TypeSpec::Short)); Ok(()) }
    fn ts_int(&mut self) -> R<()> { self.push_spec(DeclSpec::Type(TypeSpec::Int)); Ok(()) }
    fn ts_long(&mut self) -> R<()> { self.push_spec(DeclSpec::Type(TypeSpec::Long)); Ok(()) }
    fn ts_float(&mut self) -> R<()> { self.push_spec(DeclSpec::Type(TypeSpec::Float)); Ok(()) }
    fn ts_double(&mut self) -> R<()> { self.push_spec(DeclSpec::Type(TypeSpec::Double)); Ok(()) }
    fn ts_signed(&mut self) -> R<()> { self.push_spec(DeclSpec::Type(TypeSpec::Signed)); Ok(()) }
    fn ts_unsigned(&mut self) -> R<()> { self.push_spec(DeclSpec::Type(TypeSpec::Unsigned)); Ok(()) }
    fn ts_complex(&mut self) -> R<()> { self.push_spec(DeclSpec::Type(TypeSpec::Complex)); Ok(()) }

    // === Type specifiers (unique) ===
    fn ts_void(&mut self) -> R<()> { self.push_spec(DeclSpec::Type(TypeSpec::Void)); Ok(()) }
    fn ts_bool(&mut self) -> R<()> { self.push_spec(DeclSpec::Type(TypeSpec::Bool)); Ok(()) }
    fn ts_pending(&mut self) -> R<()> {
        let ts = self.pending_type_spec.take().unwrap();
        self.push_spec(DeclSpec::Type(ts));
        Ok(())
    }
    fn ts_typedef(&mut self, name: String) -> R<()> {
        self.push_spec(DeclSpec::Type(TypeSpec::TypedefName(name)));
        Ok(())
    }

    // === Type qualifiers ===
    fn tq_const(&mut self) -> R<()> { self.push_spec(DeclSpec::Qual(TypeQualifier::Const)); Ok(()) }
    fn tq_restrict(&mut self) -> R<()> { self.push_spec(DeclSpec::Qual(TypeQualifier::Restrict)); Ok(()) }
    fn tq_volatile(&mut self) -> R<()> { self.push_spec(DeclSpec::Qual(TypeQualifier::Volatile)); Ok(()) }
    fn tq_atomic(&mut self) -> R<()> { self.push_spec(DeclSpec::Qual(TypeQualifier::Atomic)); Ok(()) }

    // === Function specifiers ===
    fn fs_inline(&mut self) -> R<()> { self.push_spec(DeclSpec::Func(FuncSpec::Inline)); Ok(()) }
    fn fs_noreturn(&mut self) -> R<()> { self.push_spec(DeclSpec::Func(FuncSpec::Noreturn)); Ok(()) }

    // === Alignment specifiers ===
    fn as_type(&mut self, tn: TypeName) -> R<()> { self.push_spec(DeclSpec::Align(AlignSpec::Type(tn))); Ok(()) }
    fn as_expr(&mut self, e: ExprNode) -> R<()> { self.push_spec(DeclSpec::Align(AlignSpec::Expr(e))); Ok(()) }

    // === Struct/Union ===
    fn sou_struct(&mut self) -> R<()> { self.pending_sou = StructOrUnion::Struct; self.push_spec_frame(); Ok(()) }
    fn sou_union(&mut self) -> R<()> { self.pending_sou = StructOrUnion::Union; self.push_spec_frame(); Ok(()) }
    fn sou_def(&mut self, name: Option<String>, _members: Vec<()>) -> R<()> {
        self.pop_spec_frame();
        let members = std::mem::take(&mut self.pending_struct_members);
        self.pending_type_spec = Some(TypeSpec::Struct(self.pending_sou, StructSpec { name, members }));
        Ok(())
    }
    fn sou_ref(&mut self, name: String) -> R<()> {
        self.pop_spec_frame();
        self.pending_type_spec = Some(TypeSpec::Struct(self.pending_sou, StructSpec { name: Some(name), members: vec![] }));
        Ok(())
    }

    // === Enum ===
    fn enum_def(&mut self, name: Option<String>, _enumerators: Vec<()>, _comma: Option<()>) -> R<()> {
        let enumerators = std::mem::take(&mut self.pending_enumerators);
        self.pending_type_spec = Some(TypeSpec::Enum(EnumSpec { name, enumerators }));
        Ok(())
    }
    fn enum_ref(&mut self, name: String) -> R<()> {
        self.pending_type_spec = Some(TypeSpec::Enum(EnumSpec { name: Some(name), enumerators: vec![] }));
        Ok(())
    }
    fn decl_enum(&mut self, name: String) -> R<()> {
        self.ctx.declare_varname(&name);
        self.pending_enumerators.push(Enumerator { name, value: None });
        Ok(())
    }
    fn decl_enum_expr(&mut self, name: String, e: ExprNode) -> R<()> {
        self.ctx.declare_varname(&name);
        self.pending_enumerators.push(Enumerator { name, value: Some(e) });
        Ok(())
    }

    // === Atomic ===
    fn push_atomic(&mut self, tn: TypeName) -> R<()> {
        self.pending_type_spec = Some(TypeSpec::Atomic(tn));
        Ok(())
    }

    // === Pointer ===
    fn make_ptr(&mut self, _quals: Option<()>, inner: Option<u32>) -> R<u32> {
        Ok(1 + inner.unwrap_or(0))
    }

    // === Abstract declarator ===
    fn abs_ptr(&mut self, n: u32) -> R<Vec<DerivedType>> {
        Ok(vec![DerivedType::Pointer; n as usize])
    }
    fn abs_direct(&mut self, d: Vec<DerivedType>) -> R<Vec<DerivedType>> {
        Ok(d)
    }
    fn abs_ptr_direct(&mut self, n: u32, mut d: Vec<DerivedType>) -> R<Vec<DerivedType>> {
        d.extend(std::iter::repeat_n(DerivedType::Pointer, n as usize));
        Ok(d)
    }

    // === Direct abstract declarator ===
    fn dabs_paren(&mut self, _ctx: Context, abs: Vec<DerivedType>) -> R<Vec<DerivedType>> {
        self.pop_spec_frame();
        Ok(abs)
    }
    fn dabs_array(&mut self, d: Option<Vec<DerivedType>>, size: Option<ExprNode>) -> R<Vec<DerivedType>> {
        let mut d = d.unwrap_or_default();
        d.push(DerivedType::Array(size));
        Ok(d)
    }
    fn dabs_array_q(&mut self, d: Option<Vec<DerivedType>>, size: Option<ExprNode>) -> R<Vec<DerivedType>> {
        let mut d = d.unwrap_or_default();
        d.push(DerivedType::Array(size));
        Ok(d)
    }
    fn dabs_array_s(&mut self, d: Option<Vec<DerivedType>>, _quals: Option<()>, size: ExprNode) -> R<Vec<DerivedType>> {
        let mut d = d.unwrap_or_default();
        d.push(DerivedType::Array(Some(size)));
        Ok(d)
    }
    fn dabs_array_qs(&mut self, d: Option<Vec<DerivedType>>, size: ExprNode) -> R<Vec<DerivedType>> {
        let mut d = d.unwrap_or_default();
        d.push(DerivedType::Array(Some(size)));
        Ok(d)
    }
    fn dabs_vla(&mut self, d: Option<Vec<DerivedType>>) -> R<Vec<DerivedType>> {
        let mut d = d.unwrap_or_default();
        d.push(DerivedType::Array(None));
        Ok(d)
    }
    fn dabs_func0(&mut self, params: Option<Context>) -> R<Vec<DerivedType>> {
        let (params, variadic) = if params.is_some() {
            (std::mem::take(&mut self.current_params), self.current_variadic)
        } else {
            (vec![], false)
        };
        self.current_variadic = false;
        Ok(vec![DerivedType::Function(params, variadic)])
    }
    fn dabs_func(&mut self, mut d: Vec<DerivedType>, params: Option<Context>) -> R<Vec<DerivedType>> {
        let (params, variadic) = if params.is_some() {
            (std::mem::take(&mut self.current_params), self.current_variadic)
        } else {
            (vec![], false)
        };
        self.current_variadic = false;
        d.push(DerivedType::Function(params, variadic));
        Ok(d)
    }

    // === Type names ===
    fn make_type_name(&mut self, abs: Option<Vec<DerivedType>>) -> R<TypeName> {
        Ok(TypeName { specs: self.drain_specs(), derived: abs.unwrap_or_default() })
    }

    // === Context ===
    fn save_context(&mut self) -> R<Context> { self.push_spec_frame(); Ok(self.ctx.save()) }
    fn restore_compound(&mut self, ctx: Context, stmt: Stmt) -> R<Stmt> { self.pop_spec_frame(); self.ctx.restore(ctx); Ok(stmt) }
    fn restore_iteration(&mut self, ctx: Context, stmt: Stmt) -> R<Stmt> { self.pop_spec_frame(); self.ctx.restore(ctx); Ok(stmt) }
    fn restore_selection(&mut self, ctx: Context, stmt: Stmt) -> R<Stmt> { self.pop_spec_frame(); self.ctx.restore(ctx); Ok(stmt) }
    fn restore_statement(&mut self, ctx: Context, stmt: Stmt) -> R<Stmt> { self.pop_spec_frame(); self.ctx.restore(ctx); Ok(stmt) }

    fn param_ctx(&mut self, _params: Vec<()>, variadic: Option<()>, ctx: Context) -> R<Context> {
        self.pop_spec_frame();
        self.current_params = std::mem::take(&mut self.pending_params);
        self.current_variadic = variadic.is_some();
        Ok(ctx)
    }
    fn scoped_params(&mut self, start_ctx: Context, end_ctx: Context) -> R<Context> {
        self.pop_spec_frame();
        self.ctx.restore(start_ctx);
        Ok(end_ctx)
    }

    // === Parameters ===
    fn push_param(&mut self, d: Declarator) -> R<()> {
        let specs = self.drain_specs();
        self.pending_params.push(Param { specs, name: Some(d.name.clone()), derived: d.derived });
        Ok(())
    }
    fn push_anon_param(&mut self, abs: Option<Vec<DerivedType>>) -> R<()> {
        let specs = self.drain_specs();
        self.pending_params.push(Param { specs, name: None, derived: abs.unwrap_or_default() });
        Ok(())
    }

    // === Declarators ===
    fn dd_ident(&mut self, name: String) -> R<Declarator> { Ok(Declarator::new(name)) }
    fn dd_paren(&mut self, _ctx: Context, d: Declarator) -> R<Declarator> { self.pop_spec_frame(); Ok(d) }
    fn dd_array(&mut self, mut d: Declarator, _quals: Option<()>, size: Option<ExprNode>) -> R<Declarator> {
        d.derived.push(DerivedType::Array(size));
        d.set_other();
        Ok(d)
    }
    fn dd_array_s(&mut self, mut d: Declarator, _quals: Option<()>, size: ExprNode) -> R<Declarator> {
        d.derived.push(DerivedType::Array(Some(size)));
        d.set_other();
        Ok(d)
    }
    fn dd_array_qs(&mut self, mut d: Declarator, size: ExprNode) -> R<Declarator> {
        d.derived.push(DerivedType::Array(Some(size)));
        d.set_other();
        Ok(d)
    }
    fn dd_vla(&mut self, mut d: Declarator, _quals: Option<()>) -> R<Declarator> {
        d.derived.push(DerivedType::Array(None));
        d.set_other();
        Ok(d)
    }
    fn dd_func(&mut self, mut d: Declarator, ctx: Context) -> R<Declarator> {
        let params = std::mem::take(&mut self.current_params);
        d.derived.push(DerivedType::Function(params, self.current_variadic));
        self.current_variadic = false;
        d.set_func(ctx);
        Ok(d)
    }
    fn dd_kr(&mut self, mut d: Declarator, _ctx: Context, _ids: Option<()>) -> R<Declarator> {
        self.pop_spec_frame();
        d.derived.push(DerivedType::Function(vec![], false));
        d.set_other();
        Ok(d)
    }
    fn decl_ptr(&mut self, n: u32, mut d: Declarator) -> R<Declarator> {
        for _ in 0..n { d.derived.push(DerivedType::Pointer); }
        d.set_other();
        Ok(d)
    }

    fn decl_varname(&mut self, d: Declarator) -> R<Declarator> {
        self.ctx.declare_varname(&d.name);
        Ok(d)
    }
    fn register_typedef(&mut self, d: Declarator) -> R<Declarator> {
        self.ctx.declare_typedef(&d.name);
        Ok(d)
    }

    // === Function definitions ===
    fn func_def1(&mut self, d: Declarator) -> R<Context> {
        let saved = self.ctx.save();
        let Declarator { name, mut derived, kind } = d;
        self.current_func = Some(name.clone());
        self.current_return_specs = self.drain_specs();
        // Strip the function's own Function entry; extract params, rest is return derived
        if let Some(pos) = derived.iter().position(|d| matches!(d, DerivedType::Function(..))) {
            if let DerivedType::Function(params, variadic) = derived.remove(pos) {
                self.current_params = params;
                self.current_variadic = variadic;
            }
        }
        self.current_return_derived = derived;
        if let DeclKind::Func(ctx) = kind {
            self.ctx.restore(ctx);
            self.ctx.declare_varname(&name);
        }
        Ok(saved)
    }

    fn func_def(&mut self, ctx: Context, _decls: Vec<Decl>, body: Stmt) -> R<()> {
        self.ctx.restore(ctx);
        let name = self.current_func.take().unwrap_or_default();
        let return_specs = std::mem::take(&mut self.current_return_specs);
        let return_derived = std::mem::take(&mut self.current_return_derived);
        let params = std::mem::take(&mut self.current_params);
        self.unit.functions.push(FunctionDef { name, return_specs, return_derived, params, body });
        Ok(())
    }

    // === Declaration accumulation ===
    fn push_decl(&mut self, d: Declarator) -> R<()> {
        self.pending_decls.push(InitDeclarator { name: d.name.clone(), derived: d.derived, init: None });
        Ok(())
    }
    fn push_decl_init(&mut self, d: Declarator, init: Init) -> R<()> {
        self.pending_decls.push(InitDeclarator { name: d.name.clone(), derived: d.derived, init: Some(init) });
        Ok(())
    }
    fn push_td(&mut self, d: Declarator) -> R<()> {
        self.pending_decls.push(InitDeclarator { name: d.name.clone(), derived: d.derived, init: None });
        Ok(())
    }
    fn push_td_init(&mut self, d: Declarator, init: Init) -> R<()> {
        self.pending_decls.push(InitDeclarator { name: d.name.clone(), derived: d.derived, init: Some(init) });
        Ok(())
    }
    fn decl_var(&mut self, _list: Option<()>) -> R<Decl> {
        Ok(Decl { specs: self.drain_specs(), is_typedef: false, declarators: std::mem::take(&mut self.pending_decls) })
    }
    fn decl_typedef(&mut self, _list: Option<()>) -> R<Decl> {
        Ok(Decl { specs: self.drain_specs(), is_typedef: true, declarators: std::mem::take(&mut self.pending_decls) })
    }
    fn decl_static_assert(&mut self) -> R<Decl> {
        Ok(Decl { specs: vec![], is_typedef: false, declarators: vec![] })
    }
    fn top_decl(&mut self, _d: Decl) -> R<()> { Ok(()) }

    // === Initializers ===
    fn init_expr(&mut self, e: ExprNode) -> R<Init> { Ok(Init::Expr(e)) }
    fn init_braced(&mut self, _comma: Option<()>) -> R<Init> {
        Ok(Init::List(std::mem::take(&mut self.pending_inits)))
    }
    fn push_init(&mut self, _desig: Option<()>, init: Init) -> R<()> {
        self.pending_inits.push(InitItem { designation: vec![], init });
        Ok(())
    }

    // === Expression actions ===
    fn prim_var(&mut self, name: String) -> R<ExprNode> { Ok(expr(Expr::Var(name))) }
    fn prim_const(&mut self, val: String) -> R<ExprNode> { Ok(expr(Expr::Constant(val))) }
    fn prim_str(&mut self, val: String) -> R<ExprNode> { Ok(expr(Expr::StringLit(val))) }
    fn str_concat(&mut self, a: String, b: String) -> R<String> { Ok(a + &b) }
    fn prim_generic(&mut self, ctrl: ExprNode, assocs: Vec<GenericAssoc>) -> R<ExprNode> { Ok(expr(Expr::Generic(ctrl, assocs))) }
    fn ga_type(&mut self, tn: TypeName, e: ExprNode) -> R<GenericAssoc> { Ok(GenericAssoc { type_name: Some(tn), expr: e }) }
    fn ga_default(&mut self, e: ExprNode) -> R<GenericAssoc> { Ok(GenericAssoc { type_name: None, expr: e }) }
    fn prim_va_arg(&mut self, e: ExprNode, tn: TypeName) -> R<ExprNode> { Ok(expr(Expr::VaArg(e, tn))) }

    fn post_index(&mut self, arr: ExprNode, idx: ExprNode) -> R<ExprNode> { Ok(expr(Expr::Index(arr, idx))) }
    fn post_call(&mut self, func: ExprNode, args: Vec<ExprNode>) -> R<ExprNode> { Ok(expr(Expr::Call(func, args))) }
    fn post_call_empty(&mut self, func: ExprNode) -> R<ExprNode> { Ok(expr(Expr::Call(func, vec![]))) }
    fn post_member(&mut self, obj: ExprNode, name: String) -> R<ExprNode> { Ok(expr(Expr::Member(obj, name))) }
    fn post_ptr_member(&mut self, obj: ExprNode, name: String) -> R<ExprNode> { Ok(expr(Expr::PtrMember(obj, name))) }
    fn post_inc(&mut self, e: ExprNode) -> R<ExprNode> { Ok(expr(Expr::UnaryOp(UnaryOp::PostInc, e))) }
    fn post_dec(&mut self, e: ExprNode) -> R<ExprNode> { Ok(expr(Expr::UnaryOp(UnaryOp::PostDec, e))) }
    fn post_compound_lit(&mut self, tn: TypeName, _comma: Option<()>) -> R<ExprNode> {
        let items = std::mem::take(&mut self.pending_inits);
        Ok(expr(Expr::CompoundLiteral(tn, items)))
    }

    fn pre_inc(&mut self, e: ExprNode) -> R<ExprNode> { Ok(expr(Expr::UnaryOp(UnaryOp::PreInc, e))) }
    fn pre_dec(&mut self, e: ExprNode) -> R<ExprNode> { Ok(expr(Expr::UnaryOp(UnaryOp::PreDec, e))) }
    fn unary_op(&mut self, op: UnaryOp, e: ExprNode) -> R<ExprNode> { Ok(expr(Expr::UnaryOp(op, e))) }

    fn op_addr(&mut self) -> R<UnaryOp> { Ok(UnaryOp::AddrOf) }
    fn op_deref(&mut self) -> R<UnaryOp> { Ok(UnaryOp::Deref) }
    fn op_plus(&mut self) -> R<UnaryOp> { Ok(UnaryOp::Plus) }
    fn op_neg(&mut self) -> R<UnaryOp> { Ok(UnaryOp::Neg) }
    fn op_bitnot(&mut self) -> R<UnaryOp> { Ok(UnaryOp::BitNot) }
    fn op_lognot(&mut self) -> R<UnaryOp> { Ok(UnaryOp::LogNot) }
    fn sizeof_expr(&mut self, e: ExprNode) -> R<ExprNode> { Ok(expr(Expr::SizeofExpr(e))) }
    fn sizeof_type(&mut self, tn: TypeName) -> R<ExprNode> { Ok(expr(Expr::SizeofType(tn))) }
    fn alignof_type(&mut self, tn: TypeName) -> R<ExprNode> { Ok(expr(Expr::AlignofType(tn))) }
    fn cast(&mut self, tn: TypeName, e: ExprNode) -> R<ExprNode> { Ok(expr(Expr::Cast(tn, e))) }

    fn binop(&mut self, l: ExprNode, op: Op, r: ExprNode) -> R<ExprNode> { Ok(expr(Expr::BinOp(op, l, r))) }
    fn mul(&mut self, l: ExprNode, r: ExprNode) -> R<ExprNode> { Ok(expr(Expr::BinOp(Op::Mul, l, r))) }
    fn bitand(&mut self, l: ExprNode, r: ExprNode) -> R<ExprNode> { Ok(expr(Expr::BinOp(Op::BitAnd, l, r))) }
    fn add(&mut self, l: ExprNode, r: ExprNode) -> R<ExprNode> { Ok(expr(Expr::BinOp(Op::Add, l, r))) }
    fn sub(&mut self, l: ExprNode, r: ExprNode) -> R<ExprNode> { Ok(expr(Expr::BinOp(Op::Sub, l, r))) }
    fn assign(&mut self, l: ExprNode, r: ExprNode) -> R<ExprNode> { Ok(expr(Expr::BinOp(Op::Assign, l, r))) }
    fn ternary(&mut self, cond: ExprNode, then: ExprNode, else_: ExprNode) -> R<ExprNode> { Ok(expr(Expr::Ternary(cond, then, else_))) }
    fn comma(&mut self, l: ExprNode, r: ExprNode) -> R<ExprNode> { Ok(expr(Expr::Comma(l, r))) }

    // === Statement actions ===
    fn compound(&mut self, items: Vec<BlockItem>) -> R<Stmt> { Ok(Stmt::Compound(items)) }
    fn block_decl(&mut self, d: Decl) -> R<BlockItem> { Ok(BlockItem::Decl(d)) }
    fn block_stmt(&mut self, s: Stmt) -> R<BlockItem> { Ok(BlockItem::Stmt(s)) }
    fn expr_stmt(&mut self, e: Option<ExprNode>) -> R<Stmt> { Ok(Stmt::Expr(e)) }
    fn labeled(&mut self, name: String, s: Stmt) -> R<Stmt> { Ok(Stmt::Labeled(name, Box::new(s))) }
    fn case_label(&mut self, e: ExprNode, s: Stmt) -> R<Stmt> { Ok(Stmt::Case(e, Box::new(s))) }
    fn default_label(&mut self, s: Stmt) -> R<Stmt> { Ok(Stmt::Default(Box::new(s))) }
    fn if_else(&mut self, cond: ExprNode, then: Stmt, else_: Stmt) -> R<Stmt> { Ok(Stmt::If(cond, Box::new(then), Some(Box::new(else_)))) }
    fn if_stmt(&mut self, cond: ExprNode, then: Stmt) -> R<Stmt> { Ok(Stmt::If(cond, Box::new(then), None)) }
    fn switch_stmt(&mut self, e: ExprNode, s: Stmt) -> R<Stmt> { Ok(Stmt::Switch(e, Box::new(s))) }
    fn while_stmt(&mut self, cond: ExprNode, body: Stmt) -> R<Stmt> { Ok(Stmt::While(cond, Box::new(body))) }
    fn do_while(&mut self, body: Stmt, cond: ExprNode) -> R<Stmt> { Ok(Stmt::DoWhile(Box::new(body), cond)) }
    fn for_expr(&mut self, init: Option<ExprNode>, cond: Option<ExprNode>, step: Option<ExprNode>, body: Stmt) -> R<Stmt> {
        Ok(Stmt::For(ForInit::Expr(init), cond, step, Box::new(body)))
    }
    fn for_decl(&mut self, decl: Decl, cond: Option<ExprNode>, step: Option<ExprNode>, body: Stmt) -> R<Stmt> {
        Ok(Stmt::For(ForInit::Decl(decl), cond, step, Box::new(body)))
    }
    fn goto_stmt(&mut self, name: String) -> R<Stmt> { Ok(Stmt::Goto(name)) }
    fn continue_stmt(&mut self) -> R<Stmt> { Ok(Stmt::Continue) }
    fn break_stmt(&mut self) -> R<Stmt> { Ok(Stmt::Break) }
    fn return_stmt(&mut self, e: Option<ExprNode>) -> R<Stmt> { Ok(Stmt::Return(e)) }
}

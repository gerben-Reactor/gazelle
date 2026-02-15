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
        specs_nil: Specs = _ @specs_empty;
        list_anonymous_0_: Specs = specs_nil
            | type_qualifier list_anonymous_0_ @sc0
            | alignment_specifier list_anonymous_0_ @sc0;
        list_anonymous_1_: Specs = specs_nil
            | type_qualifier list_anonymous_1_ @sc1
            | alignment_specifier list_anonymous_1_ @sc1;
        list_declaration_specifier_: Specs = specs_nil
            | declaration_specifier list_declaration_specifier_ @sc2;
        list_eq1_TYPEDEF_declaration_specifier_: Specs = TYPEDEF list_declaration_specifier_
            | declaration_specifier list_eq1_TYPEDEF_declaration_specifier_ @sc3;
        list_eq1_type_specifier_unique_anonymous_0_: Specs = type_specifier_unique list_anonymous_0_ @sc4
            | type_qualifier list_eq1_type_specifier_unique_anonymous_0_ @sc4
            | alignment_specifier list_eq1_type_specifier_unique_anonymous_0_ @sc4;
        list_eq1_type_specifier_unique_declaration_specifier_: Specs = type_specifier_unique list_declaration_specifier_ @sc5
            | declaration_specifier list_eq1_type_specifier_unique_declaration_specifier_ @sc5;
        list_ge1_type_specifier_nonunique_anonymous_1_: Specs = type_specifier_nonunique list_anonymous_1_ @sc6
            | type_specifier_nonunique list_ge1_type_specifier_nonunique_anonymous_1_ @sc6
            | type_qualifier list_ge1_type_specifier_nonunique_anonymous_1_ @sc6
            | alignment_specifier list_ge1_type_specifier_nonunique_anonymous_1_ @sc6;
        list_ge1_type_specifier_nonunique_declaration_specifier_: Specs = type_specifier_nonunique list_declaration_specifier_ @sc7
            | type_specifier_nonunique list_ge1_type_specifier_nonunique_declaration_specifier_ @sc7
            | declaration_specifier list_ge1_type_specifier_nonunique_declaration_specifier_ @sc7;
        list_eq1_eq1_TYPEDEF_type_specifier_unique_declaration_specifier_: Specs = TYPEDEF list_eq1_type_specifier_unique_declaration_specifier_
            | type_specifier_unique list_eq1_TYPEDEF_declaration_specifier_ @sc8
            | declaration_specifier list_eq1_eq1_TYPEDEF_type_specifier_unique_declaration_specifier_ @sc8;
        list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_: Specs = TYPEDEF list_ge1_type_specifier_nonunique_declaration_specifier_
            | type_specifier_nonunique list_eq1_TYPEDEF_declaration_specifier_ @sc9
            | type_specifier_nonunique list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_ @sc9
            | declaration_specifier list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_ @sc9;

        // === Names ===
        typedef_name: Name = NAME TYPE;
        var_name: Name = NAME VARIABLE;
        typedef_name_spec: Name = typedef_name;
        general_identifier: Name = typedef_name | var_name;
        save_context: Context = _ @save_context;

        // === Scoped wrappers ===
        scoped_compound_statement_: Stmt = save_context compound_statement @restore_compound;
        scoped_iteration_statement_: Stmt = save_context iteration_statement @restore_iteration;
        scoped_parameter_type_list_: ParamCtx = save_context parameter_type_list @scoped_params;
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
                           | LPAREN type_name RPAREN LBRACE (initializer_list % COMMA) COMMA? RBRACE @post_compound_lit;

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
        init_declarator_typedefname: InitDeclarator = declarator_typedefname @push_td | declarator_typedefname EQ c_initializer @push_td_init;
        init_declarator_varname: InitDeclarator = declarator_varname @push_decl | declarator_varname EQ c_initializer @push_decl_init;

        declaration: Decl = declaration_specifiers (init_declarator_varname % COMMA) SEMICOLON @decl_var
                    | declaration_specifiers SEMICOLON @decl_var_empty
                    | declaration_specifiers_typedef (init_declarator_typedefname % COMMA) SEMICOLON @decl_typedef
                    | declaration_specifiers_typedef SEMICOLON @decl_typedef_empty
                    | static_assert_declaration @decl_static_assert;

        declaration_specifier: DeclSpec = storage_class_specifier | type_qualifier | function_specifier | alignment_specifier;

        declaration_specifiers: Specs = list_eq1_type_specifier_unique_declaration_specifier_
                               | list_ge1_type_specifier_nonunique_declaration_specifier_;

        declaration_specifiers_typedef: Specs = list_eq1_eq1_TYPEDEF_type_specifier_unique_declaration_specifier_
                                       | list_eq1_ge1_TYPEDEF_type_specifier_nonunique_declaration_specifier_;

        storage_class_specifier: DeclSpec = EXTERN @sc_extern | STATIC @sc_static | THREAD_LOCAL @sc_thread
                                | AUTO @sc_auto | REGISTER @sc_register;

        type_specifier_nonunique: DeclSpec = CHAR @ts_char | SHORT @ts_short | INT @ts_int | LONG @ts_long
                                 | FLOAT @ts_float | DOUBLE @ts_double
                                 | SIGNED @ts_signed | UNSIGNED @ts_unsigned | COMPLEX @ts_complex;

        type_specifier_unique: DeclSpec = VOID @ts_void | BOOL @ts_bool
                              | atomic_type_specifier @ts_pending
                              | struct_or_union_specifier @ts_pending
                              | enum_specifier @ts_pending
                              | typedef_name_spec @ts_typedef;

        struct_or_union_specifier: TypeSpec = struct_or_union general_identifier? LBRACE struct_declaration+ RBRACE @sou_def
                                  | struct_or_union general_identifier @sou_ref;
        struct_or_union: StructOrUnion = STRUCT @sou_struct | UNION @sou_union;
        struct_declaration: StructMember = specifier_qualifier_list (struct_declarator % COMMA) SEMICOLON @struct_member
                                       | specifier_qualifier_list SEMICOLON @struct_member_anon;

        specifier_qualifier_list: Specs = list_eq1_type_specifier_unique_anonymous_0_
                                 | list_ge1_type_specifier_nonunique_anonymous_1_;

        struct_declarator: MemberDeclarator = declarator @sd_decl | declarator? COLON constant_expression @sd_bitfield;

        enum_specifier: TypeSpec = ENUM general_identifier? LBRACE (enumerator % COMMA) COMMA? RBRACE @enum_def
                       | ENUM general_identifier @enum_ref;
        enumerator: Enumerator = enumeration_constant @decl_enum | enumeration_constant EQ constant_expression @decl_enum_expr;
        enumeration_constant: Name = general_identifier;

        atomic_type_specifier: TypeSpec = ATOMIC LPAREN type_name RPAREN @push_atomic
                              | ATOMIC ATOMIC_LPAREN type_name RPAREN @push_atomic;

        type_qualifier: DeclSpec = CONST @tq_const | RESTRICT @tq_restrict | VOLATILE @tq_volatile | ATOMIC @tq_atomic;
        function_specifier: DeclSpec = INLINE @fs_inline | NORETURN @fs_noreturn;
        alignment_specifier: DeclSpec = ALIGNAS LPAREN type_name RPAREN @as_type
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

        parameter_type_list: ParamCtx = (parameter_declaration % COMMA) variadic_suffix? save_context @param_ctx;
        parameter_declaration: Param = declaration_specifiers declarator_varname @make_param
                              | declaration_specifiers abstract_declarator? @make_anon_param;
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

        c_initializer: Init = assignment_expression @init_expr | LBRACE (initializer_list % COMMA) COMMA? RBRACE @init_braced;
        initializer_list: InitItem = designation? c_initializer @make_init_item;
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
        function_definition1: FuncHeader = declaration_specifiers declarator_varname @func_def1;
        function_definition = function_definition1 declaration* compound_statement @func_def;
    }
}

// === Typedef context types ===

pub type Context = HashSet<String>;

pub struct ParamCtx {
    ctx: Context,
    params: Vec<Param>,
    variadic: bool,
}

pub struct FuncHeader {
    ctx: Context,
    name: String,
    return_specs: Vec<DeclSpec>,
    return_derived: Vec<DerivedType>,
    params: Vec<Param>,
}

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
    pub unit: TranslationUnit,
}

impl CActions {
    pub fn new() -> Self {
        let mut ctx = TypedefContext::new();
        ctx.declare_typedef("__builtin_va_list");
        Self {
            ctx,
            unit: TranslationUnit { decls: vec![], functions: vec![], structs: Default::default(), globals: Default::default() },
        }
    }
}

impl C11Types for CActions {
    type Error = gazelle::ParseError;
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
    type StructOrUnion = StructOrUnion;
    type StructMember = StructMember;
    type TypeSpec = TypeSpec;
    type InitDeclarator = InitDeclarator;
    type MemberDeclarator = MemberDeclarator;
    type Enumerator = Enumerator;
    type Param = Param;
    type InitItem = InitItem;
    type ParamCtx = ParamCtx;
    type FuncHeader = FuncHeader;
    type DeclSpec = DeclSpec;
    type Specs = Vec<DeclSpec>;
}

type R<T> = Result<T, gazelle::ParseError>;

macro_rules! specs_cons_impl {
    ($($name:ident),*) => {
        $(fn $name(&mut self, spec: DeclSpec, mut specs: Vec<DeclSpec>) -> R<Vec<DeclSpec>> {
            specs.push(spec);
            Ok(specs)
        })*
    }
}

impl C11Actions for CActions {
    // === Struct members ===
    fn struct_member(&mut self, specs: Vec<DeclSpec>, decls: Vec<MemberDeclarator>) -> R<StructMember> {
        Ok(StructMember { specs, declarators: decls })
    }
    fn struct_member_anon(&mut self, specs: Vec<DeclSpec>) -> R<StructMember> {
        Ok(StructMember { specs, declarators: vec![] })
    }
    fn sd_decl(&mut self, d: Declarator) -> R<MemberDeclarator> {
        Ok(MemberDeclarator { name: Some(d.name), derived: d.derived, bitfield: None })
    }
    fn sd_bitfield(&mut self, d: Option<Declarator>, bits: ExprNode) -> R<MemberDeclarator> {
        let (name, derived) = match d {
            Some(d) => (Some(d.name), d.derived),
            None => (None, vec![]),
        };
        Ok(MemberDeclarator { name, derived, bitfield: Some(bits) })
    }

    // === Specifier list construction ===
    fn specs_empty(&mut self) -> R<Vec<DeclSpec>> { Ok(vec![]) }
    specs_cons_impl!(sc0, sc1, sc2, sc3, sc4, sc5, sc6, sc7, sc8, sc9);

    // === Storage class specifiers ===
    fn sc_extern(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Storage(StorageClass::Extern)) }
    fn sc_static(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Storage(StorageClass::Static)) }
    fn sc_thread(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Storage(StorageClass::ThreadLocal)) }
    fn sc_auto(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Storage(StorageClass::Auto)) }
    fn sc_register(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Storage(StorageClass::Register)) }

    // === Type specifiers (nonunique) ===
    fn ts_char(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Type(TypeSpec::Char)) }
    fn ts_short(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Type(TypeSpec::Short)) }
    fn ts_int(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Type(TypeSpec::Int)) }
    fn ts_long(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Type(TypeSpec::Long)) }
    fn ts_float(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Type(TypeSpec::Float)) }
    fn ts_double(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Type(TypeSpec::Double)) }
    fn ts_signed(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Type(TypeSpec::Signed)) }
    fn ts_unsigned(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Type(TypeSpec::Unsigned)) }
    fn ts_complex(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Type(TypeSpec::Complex)) }

    // === Type specifiers (unique) ===
    fn ts_void(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Type(TypeSpec::Void)) }
    fn ts_bool(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Type(TypeSpec::Bool)) }
    fn ts_pending(&mut self, ts: TypeSpec) -> R<DeclSpec> { Ok(DeclSpec::Type(ts)) }
    fn ts_typedef(&mut self, name: String) -> R<DeclSpec> { Ok(DeclSpec::Type(TypeSpec::TypedefName(name))) }

    // === Type qualifiers ===
    fn tq_const(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Qual(TypeQualifier::Const)) }
    fn tq_restrict(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Qual(TypeQualifier::Restrict)) }
    fn tq_volatile(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Qual(TypeQualifier::Volatile)) }
    fn tq_atomic(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Qual(TypeQualifier::Atomic)) }

    // === Function specifiers ===
    fn fs_inline(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Func(FuncSpec::Inline)) }
    fn fs_noreturn(&mut self) -> R<DeclSpec> { Ok(DeclSpec::Func(FuncSpec::Noreturn)) }

    // === Alignment specifiers ===
    fn as_type(&mut self, tn: TypeName) -> R<DeclSpec> { Ok(DeclSpec::Align(AlignSpec::Type(tn))) }
    fn as_expr(&mut self, e: ExprNode) -> R<DeclSpec> { Ok(DeclSpec::Align(AlignSpec::Expr(e))) }

    // === Struct/Union ===
    fn sou_struct(&mut self) -> R<StructOrUnion> { Ok(StructOrUnion::Struct) }
    fn sou_union(&mut self) -> R<StructOrUnion> { Ok(StructOrUnion::Union) }
    fn sou_def(&mut self, sou: StructOrUnion, name: Option<String>, members: Vec<StructMember>) -> R<TypeSpec> {
        Ok(TypeSpec::Struct(sou, StructSpec { name, members }))
    }
    fn sou_ref(&mut self, sou: StructOrUnion, name: String) -> R<TypeSpec> {
        Ok(TypeSpec::Struct(sou, StructSpec { name: Some(name), members: vec![] }))
    }

    // === Enum ===
    fn enum_def(&mut self, name: Option<String>, enumerators: Vec<Enumerator>, _comma: Option<()>) -> R<TypeSpec> {
        Ok(TypeSpec::Enum(EnumSpec { name, enumerators }))
    }
    fn enum_ref(&mut self, name: String) -> R<TypeSpec> {
        Ok(TypeSpec::Enum(EnumSpec { name: Some(name), enumerators: vec![] }))
    }
    fn decl_enum(&mut self, name: String) -> R<Enumerator> {
        self.ctx.declare_varname(&name);
        Ok(Enumerator { name, value: None })
    }
    fn decl_enum_expr(&mut self, name: String, e: ExprNode) -> R<Enumerator> {
        self.ctx.declare_varname(&name);
        Ok(Enumerator { name, value: Some(e) })
    }

    // === Atomic ===
    fn push_atomic(&mut self, tn: TypeName) -> R<TypeSpec> {
        Ok(TypeSpec::Atomic(tn))
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
    fn dabs_func0(&mut self, pctx: Option<ParamCtx>) -> R<Vec<DerivedType>> {
        let (params, variadic) = match pctx {
            Some(p) => (p.params, p.variadic),
            None => (vec![], false),
        };
        Ok(vec![DerivedType::Function(params, variadic)])
    }
    fn dabs_func(&mut self, mut d: Vec<DerivedType>, pctx: Option<ParamCtx>) -> R<Vec<DerivedType>> {
        let (params, variadic) = match pctx {
            Some(p) => (p.params, p.variadic),
            None => (vec![], false),
        };
        d.push(DerivedType::Function(params, variadic));
        Ok(d)
    }

    // === Type names ===
    fn make_type_name(&mut self, specs: Vec<DeclSpec>, abs: Option<Vec<DerivedType>>) -> R<TypeName> {
        Ok(TypeName { specs, derived: abs.unwrap_or_default() })
    }

    // === Context ===
    fn save_context(&mut self) -> R<Context> { Ok(self.ctx.save()) }
    fn restore_compound(&mut self, ctx: Context, stmt: Stmt) -> R<Stmt> { self.ctx.restore(ctx); Ok(stmt) }
    fn restore_iteration(&mut self, ctx: Context, stmt: Stmt) -> R<Stmt> { self.ctx.restore(ctx); Ok(stmt) }
    fn restore_selection(&mut self, ctx: Context, stmt: Stmt) -> R<Stmt> { self.ctx.restore(ctx); Ok(stmt) }
    fn restore_statement(&mut self, ctx: Context, stmt: Stmt) -> R<Stmt> { self.ctx.restore(ctx); Ok(stmt) }

    fn param_ctx(&mut self, params: Vec<Param>, variadic: Option<()>, ctx: Context) -> R<ParamCtx> {
        Ok(ParamCtx { ctx, params, variadic: variadic.is_some() })
    }
    fn scoped_params(&mut self, start_ctx: Context, pctx: ParamCtx) -> R<ParamCtx> {
        self.ctx.restore(start_ctx);
        Ok(pctx)
    }

    // === Parameters ===
    fn make_param(&mut self, specs: Vec<DeclSpec>, d: Declarator) -> R<Param> {
        Ok(Param { specs, name: Some(d.name), derived: d.derived })
    }
    fn make_anon_param(&mut self, specs: Vec<DeclSpec>, abs: Option<Vec<DerivedType>>) -> R<Param> {
        Ok(Param { specs, name: None, derived: abs.unwrap_or_default() })
    }

    // === Declarators ===
    fn dd_ident(&mut self, name: String) -> R<Declarator> { Ok(Declarator::new(name)) }
    fn dd_paren(&mut self, _ctx: Context, d: Declarator) -> R<Declarator> { Ok(d) }
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
    fn dd_func(&mut self, mut d: Declarator, pctx: ParamCtx) -> R<Declarator> {
        d.derived.push(DerivedType::Function(pctx.params, pctx.variadic));
        d.set_func(pctx.ctx);
        Ok(d)
    }
    fn dd_kr(&mut self, mut d: Declarator, _ctx: Context, _ids: Option<()>) -> R<Declarator> {
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
    fn func_def1(&mut self, return_specs: Vec<DeclSpec>, d: Declarator) -> R<FuncHeader> {
        let ctx = self.ctx.save();
        let Declarator { name, mut derived, kind } = d;
        let params = if let Some(pos) = derived.iter().position(|d| matches!(d, DerivedType::Function(..))) {
            if let DerivedType::Function(params, _) = derived.remove(pos) { params } else { vec![] }
        } else { vec![] };
        let return_derived = derived;
        if let DeclKind::Func(ref fctx) = kind {
            self.ctx.restore(fctx.clone());
            self.ctx.declare_varname(&name);
        }
        Ok(FuncHeader { ctx, name, return_specs, return_derived, params })
    }

    fn func_def(&mut self, header: FuncHeader, _decls: Vec<Decl>, body: Stmt) -> R<()> {
        self.ctx.restore(header.ctx);
        self.unit.functions.push(FunctionDef {
            name: header.name, return_specs: header.return_specs,
            return_derived: header.return_derived, params: header.params, body,
        });
        Ok(())
    }

    // === Declaration accumulation ===
    fn push_decl(&mut self, d: Declarator) -> R<InitDeclarator> {
        Ok(InitDeclarator { name: d.name, derived: d.derived, init: None })
    }
    fn push_decl_init(&mut self, d: Declarator, init: Init) -> R<InitDeclarator> {
        Ok(InitDeclarator { name: d.name, derived: d.derived, init: Some(init) })
    }
    fn push_td(&mut self, d: Declarator) -> R<InitDeclarator> {
        Ok(InitDeclarator { name: d.name, derived: d.derived, init: None })
    }
    fn push_td_init(&mut self, d: Declarator, init: Init) -> R<InitDeclarator> {
        Ok(InitDeclarator { name: d.name, derived: d.derived, init: Some(init) })
    }
    fn decl_var(&mut self, specs: Vec<DeclSpec>, list: Vec<InitDeclarator>) -> R<Decl> {
        Ok(Decl { specs, is_typedef: false, declarators: list })
    }
    fn decl_var_empty(&mut self, specs: Vec<DeclSpec>) -> R<Decl> {
        Ok(Decl { specs, is_typedef: false, declarators: vec![] })
    }
    fn decl_typedef(&mut self, specs: Vec<DeclSpec>, list: Vec<InitDeclarator>) -> R<Decl> {
        Ok(Decl { specs, is_typedef: true, declarators: list })
    }
    fn decl_typedef_empty(&mut self, specs: Vec<DeclSpec>) -> R<Decl> {
        Ok(Decl { specs, is_typedef: true, declarators: vec![] })
    }
    fn decl_static_assert(&mut self) -> R<Decl> {
        Ok(Decl { specs: vec![], is_typedef: false, declarators: vec![] })
    }
    fn top_decl(&mut self, d: Decl) -> R<()> { self.unit.decls.push(d); Ok(()) }

    // === Initializers ===
    fn init_expr(&mut self, e: ExprNode) -> R<Init> { Ok(Init::Expr(e)) }
    fn init_braced(&mut self, items: Vec<InitItem>, _comma: Option<()>) -> R<Init> {
        Ok(Init::List(items))
    }
    fn make_init_item(&mut self, _desig: Option<()>, init: Init) -> R<InitItem> {
        Ok(InitItem { designation: vec![], init })
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
    fn post_compound_lit(&mut self, tn: TypeName, items: Vec<InitItem>, _comma: Option<()>) -> R<ExprNode> {
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

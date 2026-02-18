use std::collections::HashSet;

use gazelle::Ignore;
use gazelle_macros::gazelle;

use crate::ast::*;

gazelle! {
    pub(crate) grammar C11 = "../grammars/c11.gzl"
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
    variadic: bool,
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
    fn set_func(&mut self, ctx: Context) {
        if matches!(self.kind, DeclKind::Ident) { self.kind = DeclKind::Func(ctx); }
    }
    fn set_other(&mut self) {
        if matches!(self.kind, DeclKind::Ident) { self.kind = DeclKind::Other; }
    }
}

impl std::fmt::Debug for Declarator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Declarator").field("name", &self.name).field("derived", &self.derived).finish()
    }
}

impl std::fmt::Debug for ParamCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParamCtx").field("params", &self.params).field("variadic", &self.variadic).finish()
    }
}

impl std::fmt::Debug for FuncHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FuncHeader").field("name", &self.name).finish()
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
    pub enums: std::collections::HashMap<String, i64>,
    next_enum_val: i64,
}

impl CActions {
    pub fn new() -> Self {
        let mut ctx = TypedefContext::new();
        ctx.declare_typedef("__builtin_va_list");
        Self {
            ctx,
            enums: Default::default(),
            next_enum_val: 0,
        }
    }
}

type R<T> = Result<T, gazelle::ParseError>;

// === Types impl ===

impl C11::Types for CActions {
    type Error = gazelle::ParseError;
    // Terminal payloads
    type Name = String;
    type Constant = String;
    type StringLiteral = String;
    type Binop = Op;
    // Ignored (no semantic value)
    type Variadic = Ignore;
    type TypeQualifierList = Ignore;
    type IdentifierList = Ignore;
    type StaticAssertDeclaration = Ignore;
    type EnumerationConstant = String;  // reuse GeneralIdentifier output
    // Spec lists — all collapse to Vec<DeclSpec>
    type ListAnonymous0 = Vec<DeclSpec>;
    type ListAnonymous1 = Vec<DeclSpec>;
    type ListDeclarationSpecifier = Vec<DeclSpec>;
    type ListEq1TypedefDeclarationSpecifier = Vec<DeclSpec>;
    type ListEq1TypeSpecifierUniqueAnonymous0 = Vec<DeclSpec>;
    type ListEq1TypeSpecifierUniqueDeclarationSpecifier = Vec<DeclSpec>;
    type ListGe1TypeSpecifierNonuniqueAnonymous1 = Vec<DeclSpec>;
    type ListGe1TypeSpecifierNonuniqueDeclarationSpecifier = Vec<DeclSpec>;
    type ListEq1Eq1TypedefTypeSpecifierUniqueDeclarationSpecifier = Vec<DeclSpec>;
    type ListEq1Ge1TypedefTypeSpecifierNonuniqueDeclarationSpecifier = Vec<DeclSpec>;
    // Names
    type TypedefName = String;
    type VarName = String;
    type TypedefNameSpec = String;
    type GeneralIdentifier = String;
    // Context
    type SaveContext = Context;
    // Scoped wrappers
    type ScopedCompoundStatement = Stmt;
    type ScopedIterationStatement = Stmt;
    type ScopedParameterTypeList = ParamCtx;
    type ScopedSelectionStatement = Stmt;
    type ScopedStatement = Stmt;
    // Declarator wrappers
    type DeclaratorVarname = Declarator;
    type DeclaratorTypedefname = Declarator;
    // Expressions — all collapse to ExprNode
    type PrimaryExpression = ExprNode;
    type GenericSelection = ExprNode;
    type GenericAssociation = GenericAssoc;
    type ArgumentExpressionList = Vec<ExprNode>;
    type PostfixExpression = ExprNode;
    type UnaryExpression = ExprNode;
    type UnaryOperator = UnaryOp;
    type CastExpression = ExprNode;
    type AssignmentExpression = ExprNode;
    type Expression = ExprNode;
    type ConstantExpression = ExprNode;
    // Declarations
    type InitDeclaratorListVarname = Vec<InitDeclarator>;
    type InitDeclaratorListTypedef = Vec<InitDeclarator>;
    type Declaration = Decl;
    type DeclarationSpecifier = DeclSpec;
    type DeclarationSpecifiers = Vec<DeclSpec>;
    type DeclarationSpecifiersTypedef = Vec<DeclSpec>;
    type InitDeclaratorDeclaratorTypedefname = InitDeclarator;
    type InitDeclaratorDeclaratorVarname = InitDeclarator;
    // Specifiers
    type StorageClassSpecifier = DeclSpec;
    type TypeSpecifierNonunique = DeclSpec;
    type TypeSpecifierUnique = DeclSpec;
    type StructOrUnionSpecifier = TypeSpec;
    type StructOrUnion = StructOrUnion;
    type StructDeclaratorList = Vec<MemberDeclarator>;
    type StructDeclaration = StructMember;
    type SpecifierQualifierList = Vec<DeclSpec>;
    type StructDeclarator = MemberDeclarator;
    type EnumSpecifier = TypeSpec;
    type Enumerator = Enumerator;
    type AtomicTypeSpecifier = TypeSpec;
    type TypeQualifier = DeclSpec;
    type FunctionSpecifier = DeclSpec;
    type AlignmentSpecifier = DeclSpec;
    // Declarators
    type Declarator = Declarator;
    type DirectDeclarator = Declarator;
    type Pointer = u32;
    // Parameters
    type ParameterTypeList = ParamCtx;
    type ParameterDeclaration = Param;
    // Types
    type TypeName = CType;
    type AbstractDeclarator = Vec<DerivedType>;
    type DirectAbstractDeclarator = Vec<DerivedType>;
    // Initializers
    type CInitializer = Init;
    type InitializerList = Vec<InitItem>;
    type Designation = Vec<Designator>;
    type Designator = Designator;
    // Statements
    type Statement = Stmt;
    type LabeledStatement = Stmt;
    type CompoundStatement = Stmt;
    type BlockItem = BlockItem;
    type ExpressionStatement = Stmt;
    type SelectionStatement = Stmt;
    type IterationStatement = Stmt;
    type JumpStatement = Stmt;
    // Top-level
    type TranslationUnitFile = Vec<C11::ExternalDeclaration<Self>>;
    type ExternalDeclaration = C11::ExternalDeclaration<Self>;
    type FunctionDefinition1 = FuncHeader;
    type FunctionDefinition = FunctionDef;
}

// === Reducer impls ===

// --- Names ---

impl gazelle::Reducer<C11::TypedefName<Self>> for CActions {
    fn reduce(&mut self, node: C11::TypedefName<Self>) -> R<String> {
        let C11::TypedefName::TypedefName(name) = node;
        Ok(name)
    }
}

impl gazelle::Reducer<C11::VarName<Self>> for CActions {
    fn reduce(&mut self, node: C11::VarName<Self>) -> R<String> {
        let C11::VarName::VarName(name) = node;
        Ok(name)
    }
}

impl gazelle::Reducer<C11::TypedefNameSpec<Self>> for CActions {
    fn reduce(&mut self, node: C11::TypedefNameSpec<Self>) -> R<String> {
        let C11::TypedefNameSpec::TypedefNameSpec(name) = node;
        Ok(name)
    }
}

impl gazelle::Reducer<C11::GeneralIdentifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::GeneralIdentifier<Self>) -> R<String> {
        match node {
            C11::GeneralIdentifier::Typedef(name) | C11::GeneralIdentifier::Var(name) => Ok(name),
        }
    }
}

impl gazelle::Reducer<C11::EnumerationConstant<Self>> for CActions {
    fn reduce(&mut self, node: C11::EnumerationConstant<Self>) -> R<String> {
        let C11::EnumerationConstant::EnumConst(name) = node;
        Ok(name)
    }
}

// --- Context ---

impl gazelle::Reducer<C11::SaveContext<Self>> for CActions {
    fn reduce(&mut self, _node: C11::SaveContext<Self>) -> R<Context> {
        Ok(self.ctx.save())
    }
}

// --- Scoped wrappers ---

impl gazelle::Reducer<C11::ScopedCompoundStatement<Self>> for CActions {
    fn reduce(&mut self, node: C11::ScopedCompoundStatement<Self>) -> R<Stmt> {
        let C11::ScopedCompoundStatement::RestoreCompound(ctx, stmt) = node;
        self.ctx.restore(ctx);
        Ok(stmt)
    }
}

impl gazelle::Reducer<C11::ScopedIterationStatement<Self>> for CActions {
    fn reduce(&mut self, node: C11::ScopedIterationStatement<Self>) -> R<Stmt> {
        let C11::ScopedIterationStatement::RestoreIteration(ctx, stmt) = node;
        self.ctx.restore(ctx);
        Ok(stmt)
    }
}

impl gazelle::Reducer<C11::ScopedSelectionStatement<Self>> for CActions {
    fn reduce(&mut self, node: C11::ScopedSelectionStatement<Self>) -> R<Stmt> {
        let C11::ScopedSelectionStatement::RestoreSelection(ctx, stmt) = node;
        self.ctx.restore(ctx);
        Ok(stmt)
    }
}

impl gazelle::Reducer<C11::ScopedStatement<Self>> for CActions {
    fn reduce(&mut self, node: C11::ScopedStatement<Self>) -> R<Stmt> {
        let C11::ScopedStatement::RestoreStatement(ctx, stmt) = node;
        self.ctx.restore(ctx);
        Ok(stmt)
    }
}

impl gazelle::Reducer<C11::ScopedParameterTypeList<Self>> for CActions {
    fn reduce(&mut self, node: C11::ScopedParameterTypeList<Self>) -> R<ParamCtx> {
        let C11::ScopedParameterTypeList::ScopedParams(start_ctx, pctx) = node;
        self.ctx.restore(start_ctx);
        Ok(pctx)
    }
}

// --- Declarator wrappers ---

impl gazelle::Reducer<C11::DeclaratorVarname<Self>> for CActions {
    fn reduce(&mut self, node: C11::DeclaratorVarname<Self>) -> R<Declarator> {
        let C11::DeclaratorVarname::DeclVarname(d) = node;
        self.ctx.declare_varname(&d.name);
        Ok(d)
    }
}

impl gazelle::Reducer<C11::DeclaratorTypedefname<Self>> for CActions {
    fn reduce(&mut self, node: C11::DeclaratorTypedefname<Self>) -> R<Declarator> {
        let C11::DeclaratorTypedefname::RegisterTypedef(d) = node;
        self.ctx.declare_typedef(&d.name);
        Ok(d)
    }
}

// --- Spec lists ---
// All spec list NTs collapse to Vec<DeclSpec> by consing/prepending.

fn specs_cons(spec: DeclSpec, mut rest: Vec<DeclSpec>) -> R<Vec<DeclSpec>> {
    rest.push(spec);
    Ok(rest)
}

impl gazelle::Reducer<C11::ListAnonymous0<Self>> for CActions {
    fn reduce(&mut self, node: C11::ListAnonymous0<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::ListAnonymous0::Empty => Ok(vec![]),
            C11::ListAnonymous0::Tq(spec, rest) => specs_cons(spec, rest),
            C11::ListAnonymous0::Align(spec, rest) => specs_cons(spec, rest),
        }
    }
}

impl gazelle::Reducer<C11::ListAnonymous1<Self>> for CActions {
    fn reduce(&mut self, node: C11::ListAnonymous1<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::ListAnonymous1::Empty => Ok(vec![]),
            C11::ListAnonymous1::Tq(spec, rest) => specs_cons(spec, rest),
            C11::ListAnonymous1::Align(spec, rest) => specs_cons(spec, rest),
        }
    }
}

impl gazelle::Reducer<C11::ListDeclarationSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::ListDeclarationSpecifier<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::ListDeclarationSpecifier::Empty => Ok(vec![]),
            C11::ListDeclarationSpecifier::Cons(spec, rest) => specs_cons(spec, rest),
        }
    }
}

impl gazelle::Reducer<C11::ListEq1TypedefDeclarationSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::ListEq1TypedefDeclarationSpecifier<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::ListEq1TypedefDeclarationSpecifier::Td(rest) => Ok(rest),
            C11::ListEq1TypedefDeclarationSpecifier::Cons(spec, rest) => specs_cons(spec, rest),
        }
    }
}

impl gazelle::Reducer<C11::ListEq1TypeSpecifierUniqueAnonymous0<Self>> for CActions {
    fn reduce(&mut self, node: C11::ListEq1TypeSpecifierUniqueAnonymous0<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::ListEq1TypeSpecifierUniqueAnonymous0::Ts(spec, rest) => specs_cons(spec, rest),
            C11::ListEq1TypeSpecifierUniqueAnonymous0::Tq(spec, rest) => specs_cons(spec, rest),
            C11::ListEq1TypeSpecifierUniqueAnonymous0::Align(spec, rest) => specs_cons(spec, rest),
        }
    }
}

impl gazelle::Reducer<C11::ListEq1TypeSpecifierUniqueDeclarationSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::ListEq1TypeSpecifierUniqueDeclarationSpecifier<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::ListEq1TypeSpecifierUniqueDeclarationSpecifier::Ts(spec, rest) => specs_cons(spec, rest),
            C11::ListEq1TypeSpecifierUniqueDeclarationSpecifier::Cons(spec, rest) => specs_cons(spec, rest),
        }
    }
}

impl gazelle::Reducer<C11::ListGe1TypeSpecifierNonuniqueAnonymous1<Self>> for CActions {
    fn reduce(&mut self, node: C11::ListGe1TypeSpecifierNonuniqueAnonymous1<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::ListGe1TypeSpecifierNonuniqueAnonymous1::Ts(spec, rest) => specs_cons(spec, rest),
            C11::ListGe1TypeSpecifierNonuniqueAnonymous1::Ts2(spec, rest) => specs_cons(spec, rest),
            C11::ListGe1TypeSpecifierNonuniqueAnonymous1::Tq(spec, rest) => specs_cons(spec, rest),
            C11::ListGe1TypeSpecifierNonuniqueAnonymous1::Align(spec, rest) => specs_cons(spec, rest),
        }
    }
}

impl gazelle::Reducer<C11::ListGe1TypeSpecifierNonuniqueDeclarationSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::ListGe1TypeSpecifierNonuniqueDeclarationSpecifier<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::ListGe1TypeSpecifierNonuniqueDeclarationSpecifier::Ts(spec, rest) => specs_cons(spec, rest),
            C11::ListGe1TypeSpecifierNonuniqueDeclarationSpecifier::Ts2(spec, rest) => specs_cons(spec, rest),
            C11::ListGe1TypeSpecifierNonuniqueDeclarationSpecifier::Cons(spec, rest) => specs_cons(spec, rest),
        }
    }
}

impl gazelle::Reducer<C11::ListEq1Eq1TypedefTypeSpecifierUniqueDeclarationSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::ListEq1Eq1TypedefTypeSpecifierUniqueDeclarationSpecifier<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::ListEq1Eq1TypedefTypeSpecifierUniqueDeclarationSpecifier::Td(rest) => Ok(rest),
            C11::ListEq1Eq1TypedefTypeSpecifierUniqueDeclarationSpecifier::Ts(spec, rest) => specs_cons(spec, rest),
            C11::ListEq1Eq1TypedefTypeSpecifierUniqueDeclarationSpecifier::Cons(spec, rest) => specs_cons(spec, rest),
        }
    }
}

impl gazelle::Reducer<C11::ListEq1Ge1TypedefTypeSpecifierNonuniqueDeclarationSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::ListEq1Ge1TypedefTypeSpecifierNonuniqueDeclarationSpecifier<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::ListEq1Ge1TypedefTypeSpecifierNonuniqueDeclarationSpecifier::Td(rest) => Ok(rest),
            C11::ListEq1Ge1TypedefTypeSpecifierNonuniqueDeclarationSpecifier::Ts(spec, rest) => specs_cons(spec, rest),
            C11::ListEq1Ge1TypedefTypeSpecifierNonuniqueDeclarationSpecifier::Ts2(spec, rest) => specs_cons(spec, rest),
            C11::ListEq1Ge1TypedefTypeSpecifierNonuniqueDeclarationSpecifier::Cons(spec, rest) => specs_cons(spec, rest),
        }
    }
}

// --- Declaration specifiers passthrough ---

impl gazelle::Reducer<C11::DeclarationSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::DeclarationSpecifier<Self>) -> R<DeclSpec> {
        match node {
            C11::DeclarationSpecifier::Storage(s) => Ok(s),
            C11::DeclarationSpecifier::Tq(s) => Ok(s),
            C11::DeclarationSpecifier::Func(s) => Ok(s),
            C11::DeclarationSpecifier::Align(s) => Ok(s),
        }
    }
}

impl gazelle::Reducer<C11::DeclarationSpecifiers<Self>> for CActions {
    fn reduce(&mut self, node: C11::DeclarationSpecifiers<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::DeclarationSpecifiers::Unique(s) | C11::DeclarationSpecifiers::Nonunique(s) => Ok(s),
        }
    }
}

impl gazelle::Reducer<C11::DeclarationSpecifiersTypedef<Self>> for CActions {
    fn reduce(&mut self, node: C11::DeclarationSpecifiersTypedef<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::DeclarationSpecifiersTypedef::Unique(s) | C11::DeclarationSpecifiersTypedef::Nonunique(s) => Ok(s),
        }
    }
}

impl gazelle::Reducer<C11::SpecifierQualifierList<Self>> for CActions {
    fn reduce(&mut self, node: C11::SpecifierQualifierList<Self>) -> R<Vec<DeclSpec>> {
        match node {
            C11::SpecifierQualifierList::Unique(s) | C11::SpecifierQualifierList::Nonunique(s) => Ok(s),
        }
    }
}

// --- Storage class specifiers ---

impl gazelle::Reducer<C11::StorageClassSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::StorageClassSpecifier<Self>) -> R<DeclSpec> {
        Ok(DeclSpec::Storage(match node {
            C11::StorageClassSpecifier::Extern => StorageClass::Extern,
            C11::StorageClassSpecifier::Static => StorageClass::Static,
            C11::StorageClassSpecifier::ThreadLocal => StorageClass::ThreadLocal,
            C11::StorageClassSpecifier::Auto => StorageClass::Auto,
            C11::StorageClassSpecifier::Register => StorageClass::Register,
            _ => unreachable!(),
        }))
    }
}

// --- Type specifiers ---

impl gazelle::Reducer<C11::TypeSpecifierNonunique<Self>> for CActions {
    fn reduce(&mut self, node: C11::TypeSpecifierNonunique<Self>) -> R<DeclSpec> {
        Ok(DeclSpec::Type(match node {
            C11::TypeSpecifierNonunique::Char => TypeSpec::Char,
            C11::TypeSpecifierNonunique::Short => TypeSpec::Short,
            C11::TypeSpecifierNonunique::Int => TypeSpec::Int,
            C11::TypeSpecifierNonunique::Long => TypeSpec::Long,
            C11::TypeSpecifierNonunique::Float => TypeSpec::Float,
            C11::TypeSpecifierNonunique::Double => TypeSpec::Double,
            C11::TypeSpecifierNonunique::Signed => TypeSpec::Signed,
            C11::TypeSpecifierNonunique::Unsigned => TypeSpec::Unsigned,
            C11::TypeSpecifierNonunique::Complex => TypeSpec::Complex,
            _ => unreachable!(),
        }))
    }
}

impl gazelle::Reducer<C11::TypeSpecifierUnique<Self>> for CActions {
    fn reduce(&mut self, node: C11::TypeSpecifierUnique<Self>) -> R<DeclSpec> {
        Ok(DeclSpec::Type(match node {
            C11::TypeSpecifierUnique::Void => TypeSpec::Void,
            C11::TypeSpecifierUnique::Bool => TypeSpec::Bool,
            C11::TypeSpecifierUnique::Atomic(ts) => ts,
            C11::TypeSpecifierUnique::StructOrUnion(ts) => ts,
            C11::TypeSpecifierUnique::Enum(ts) => ts,
            C11::TypeSpecifierUnique::Typedef(name) => TypeSpec::TypedefName(name),
        }))
    }
}

// --- Type qualifiers ---

impl gazelle::Reducer<C11::TypeQualifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::TypeQualifier<Self>) -> R<DeclSpec> {
        Ok(DeclSpec::Qual(match node {
            C11::TypeQualifier::Const => TypeQualifier::Const,
            C11::TypeQualifier::Restrict => TypeQualifier::Restrict,
            C11::TypeQualifier::Volatile => TypeQualifier::Volatile,
            C11::TypeQualifier::Atomic => TypeQualifier::Atomic,
            _ => unreachable!(),
        }))
    }
}

// --- Function specifiers ---

impl gazelle::Reducer<C11::FunctionSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::FunctionSpecifier<Self>) -> R<DeclSpec> {
        Ok(DeclSpec::Func(match node {
            C11::FunctionSpecifier::Inline => FuncSpec::Inline,
            C11::FunctionSpecifier::Noreturn => FuncSpec::Noreturn,
            _ => unreachable!(),
        }))
    }
}

// --- Alignment specifiers ---

impl gazelle::Reducer<C11::AlignmentSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::AlignmentSpecifier<Self>) -> R<DeclSpec> {
        Ok(match node {
            C11::AlignmentSpecifier::AlignType(ty) => DeclSpec::Align(AlignSpec::Type(ty)),
            C11::AlignmentSpecifier::AlignExpr(e) => DeclSpec::Align(AlignSpec::Expr(e)),
        })
    }
}

// --- Struct/Union ---

impl gazelle::Reducer<C11::StructOrUnion<Self>> for CActions {
    fn reduce(&mut self, node: C11::StructOrUnion<Self>) -> R<StructOrUnion> {
        Ok(match node {
            C11::StructOrUnion::Struct => StructOrUnion::Struct,
            C11::StructOrUnion::Union => StructOrUnion::Union,
            _ => unreachable!(),
        })
    }
}

impl gazelle::Reducer<C11::StructOrUnionSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::StructOrUnionSpecifier<Self>) -> R<TypeSpec> {
        Ok(match node {
            C11::StructOrUnionSpecifier::Def(sou, name, members) => {
                TypeSpec::Struct(sou, StructSpec { name, members })
            }
            C11::StructOrUnionSpecifier::Ref(sou, name) => {
                TypeSpec::Struct(sou, StructSpec { name: Some(name), members: vec![] })
            }
        })
    }
}

impl gazelle::Reducer<C11::StructDeclaration<Self>> for CActions {
    fn reduce(&mut self, node: C11::StructDeclaration<Self>) -> R<StructMember> {
        Ok(match node {
            C11::StructDeclaration::Field(specs, decls) => {
                StructMember { specs, declarators: decls.unwrap_or_default() }
            }
            C11::StructDeclaration::StaticAssert(_) => {
                StructMember { specs: vec![], declarators: vec![] }
            }
        })
    }
}

impl gazelle::Reducer<C11::StructDeclaratorList<Self>> for CActions {
    fn reduce(&mut self, node: C11::StructDeclaratorList<Self>) -> R<Vec<MemberDeclarator>> {
        let C11::StructDeclaratorList::List(list) = node;
        Ok(list)
    }
}

impl gazelle::Reducer<C11::StructDeclarator<Self>> for CActions {
    fn reduce(&mut self, node: C11::StructDeclarator<Self>) -> R<MemberDeclarator> {
        Ok(match node {
            C11::StructDeclarator::Decl(d) => {
                MemberDeclarator { name: Some(d.name), derived: d.derived, bitfield: None }
            }
            C11::StructDeclarator::Bitfield(d, bits) => {
                let (name, derived) = match d {
                    Some(d) => (Some(d.name), d.derived),
                    None => (None, vec![]),
                };
                MemberDeclarator { name, derived, bitfield: Some(bits) }
            }
        })
    }
}

// --- Enum ---

impl gazelle::Reducer<C11::EnumSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::EnumSpecifier<Self>) -> R<TypeSpec> {
        Ok(match node {
            C11::EnumSpecifier::Def(name, enumerators, _comma) => {
                self.next_enum_val = 0;
                TypeSpec::Enum(EnumSpec { name, enumerators })
            }
            C11::EnumSpecifier::Ref(name) => {
                TypeSpec::Enum(EnumSpec { name: Some(name), enumerators: vec![] })
            }
        })
    }
}

impl gazelle::Reducer<C11::Enumerator<Self>> for CActions {
    fn reduce(&mut self, node: C11::Enumerator<Self>) -> R<Enumerator> {
        match node {
            C11::Enumerator::DeclEnum(name) => {
                let val = self.next_enum_val;
                self.enums.insert(name.clone(), val);
                self.next_enum_val = val + 1;
                self.ctx.declare_varname(&name);
                Ok(Enumerator { name, value: None })
            }
            C11::Enumerator::DeclEnumExpr(name, e) => {
                let val = crate::typecheck::eval_const_i64(&e, &self.enums);
                self.enums.insert(name.clone(), val);
                self.next_enum_val = val + 1;
                self.ctx.declare_varname(&name);
                Ok(Enumerator { name, value: Some(e) })
            }
        }
    }
}

// --- Atomic ---

impl gazelle::Reducer<C11::AtomicTypeSpecifier<Self>> for CActions {
    fn reduce(&mut self, node: C11::AtomicTypeSpecifier<Self>) -> R<TypeSpec> {
        match node {
            C11::AtomicTypeSpecifier::Atomic(ty) | C11::AtomicTypeSpecifier::AtomicLparen(ty) => {
                Ok(TypeSpec::Atomic(ty))
            }
        }
    }
}

// --- Pointer ---

impl gazelle::Reducer<C11::Pointer<Self>> for CActions {
    fn reduce(&mut self, node: C11::Pointer<Self>) -> R<u32> {
        let C11::Pointer::Pointer(_quals, inner) = node;
        Ok(1 + inner.unwrap_or(0))
    }
}

// --- Declarators ---

impl gazelle::Reducer<C11::Declarator<Self>> for CActions {
    fn reduce(&mut self, node: C11::Declarator<Self>) -> R<Declarator> {
        Ok(match node {
            C11::Declarator::DeclDirect(d) => d,
            C11::Declarator::DeclPtr(n, mut d) => {
                for _ in 0..n { d.derived.push(DerivedType::Pointer); }
                d.set_other();
                d
            }
        })
    }
}

impl gazelle::Reducer<C11::DirectDeclarator<Self>> for CActions {
    fn reduce(&mut self, node: C11::DirectDeclarator<Self>) -> R<Declarator> {
        Ok(match node {
            C11::DirectDeclarator::DdIdent(name) => Declarator::new(name),
            C11::DirectDeclarator::DdParen(_ctx, d) => d,
            C11::DirectDeclarator::DdOther(mut d, _quals, size) => {
                d.derived.push(DerivedType::Array(size));
                d.set_other();
                d
            }
            C11::DirectDeclarator::DdOther1(mut d, _quals, size) => {
                d.derived.push(DerivedType::Array(Some(size)));
                d.set_other();
                d
            }
            C11::DirectDeclarator::DdOther2(mut d, _quals, size) => {
                d.derived.push(DerivedType::Array(Some(size)));
                d.set_other();
                d
            }
            C11::DirectDeclarator::DdOther3(mut d, _quals) => {
                d.derived.push(DerivedType::Array(None));
                d.set_other();
                d
            }
            C11::DirectDeclarator::DdFunc(mut d, pctx) => {
                d.derived.push(DerivedType::Function(pctx.params, pctx.variadic));
                d.set_func(pctx.ctx);
                d
            }
            C11::DirectDeclarator::DdOtherKr(mut d, _ctx, _ids) => {
                d.derived.push(DerivedType::Function(vec![], false));
                d.set_other();
                d
            }
        })
    }
}

// --- Abstract declarators ---

impl gazelle::Reducer<C11::AbstractDeclarator<Self>> for CActions {
    fn reduce(&mut self, node: C11::AbstractDeclarator<Self>) -> R<Vec<DerivedType>> {
        Ok(match node {
            C11::AbstractDeclarator::Ptr(n) => vec![DerivedType::Pointer; n as usize],
            C11::AbstractDeclarator::Direct(d) => d,
            C11::AbstractDeclarator::PtrDirect(n, mut d) => {
                d.extend(std::iter::repeat_n(DerivedType::Pointer, n as usize));
                d
            }
        })
    }
}

impl gazelle::Reducer<C11::DirectAbstractDeclarator<Self>> for CActions {
    fn reduce(&mut self, node: C11::DirectAbstractDeclarator<Self>) -> R<Vec<DerivedType>> {
        Ok(match node {
            C11::DirectAbstractDeclarator::Paren(_ctx, abs) => abs,
            C11::DirectAbstractDeclarator::Array(d, size) => {
                let mut d = d.unwrap_or_default();
                d.push(DerivedType::Array(size));
                d
            }
            C11::DirectAbstractDeclarator::ArrayQual(d, _quals, size) => {
                let mut d = d.unwrap_or_default();
                d.push(DerivedType::Array(size));
                d
            }
            C11::DirectAbstractDeclarator::ArrayStatic(d, _quals, size) => {
                let mut d = d.unwrap_or_default();
                d.push(DerivedType::Array(Some(size)));
                d
            }
            C11::DirectAbstractDeclarator::ArrayQualStatic(d, _quals, size) => {
                let mut d = d.unwrap_or_default();
                d.push(DerivedType::Array(Some(size)));
                d
            }
            C11::DirectAbstractDeclarator::ArrayVla(d) => {
                let mut d = d.unwrap_or_default();
                d.push(DerivedType::Array(None));
                d
            }
            C11::DirectAbstractDeclarator::Func(pctx) => {
                let (params, variadic) = match pctx {
                    Some(p) => (p.params, p.variadic),
                    None => (vec![], false),
                };
                vec![DerivedType::Function(params, variadic)]
            }
            C11::DirectAbstractDeclarator::FuncSuffix(mut d, pctx) => {
                let (params, variadic) = match pctx {
                    Some(p) => (p.params, p.variadic),
                    None => (vec![], false),
                };
                d.push(DerivedType::Function(params, variadic));
                d
            }
        })
    }
}

// --- Parameters ---

impl gazelle::Reducer<C11::ParameterTypeList<Self>> for CActions {
    fn reduce(&mut self, node: C11::ParameterTypeList<Self>) -> R<ParamCtx> {
        let C11::ParameterTypeList::ParamCtx(params, variadic, ctx) = node;
        Ok(ParamCtx { ctx, params, variadic: variadic.is_some() })
    }
}

impl gazelle::Reducer<C11::ParameterDeclaration<Self>> for CActions {
    fn reduce(&mut self, node: C11::ParameterDeclaration<Self>) -> R<Param> {
        Ok(match node {
            C11::ParameterDeclaration::Named(specs, d) => {
                Param { specs, name: Some(d.name), derived: d.derived, ty: None }
            }
            C11::ParameterDeclaration::Abstract(specs, abs) => {
                Param { specs, name: None, derived: abs.unwrap_or_default(), ty: None }
            }
        })
    }
}

// --- Type names ---

impl gazelle::Reducer<C11::TypeName<Self>> for CActions {
    fn reduce(&mut self, node: C11::TypeName<Self>) -> R<CType> {
        let C11::TypeName::TypeName(specs, abs) = node;
        let derived = abs.unwrap_or_default();
        crate::types::resolve_type(&specs, &derived, &self.enums).map_err(|e| panic!("type resolution: {}", e))
    }
}

// --- Expressions ---

impl gazelle::Reducer<C11::PrimaryExpression<Self>> for CActions {
    fn reduce(&mut self, node: C11::PrimaryExpression<Self>) -> R<ExprNode> {
        Ok(match node {
            C11::PrimaryExpression::Name(name) => expr(Expr::Var(name)),
            C11::PrimaryExpression::Const(val) => expr(Expr::Constant(val)),
            C11::PrimaryExpression::Str(parts) => {
                let s = parts.join("");
                expr(Expr::StringLit(s))
            }
            C11::PrimaryExpression::Paren(e) => e,
            C11::PrimaryExpression::Generic(e) => e,
            C11::PrimaryExpression::VaArg(e, ty) => expr(Expr::VaArg(e, ty)),
        })
    }
}

impl gazelle::Reducer<C11::GenericSelection<Self>> for CActions {
    fn reduce(&mut self, node: C11::GenericSelection<Self>) -> R<ExprNode> {
        let C11::GenericSelection::GenericSelection(ctrl, assocs) = node;
        Ok(expr(Expr::Generic(ctrl, assocs)))
    }
}

impl gazelle::Reducer<C11::GenericAssociation<Self>> for CActions {
    fn reduce(&mut self, node: C11::GenericAssociation<Self>) -> R<GenericAssoc> {
        Ok(match node {
            C11::GenericAssociation::Typed(ty, e) => GenericAssoc { ty: Some(ty), expr: e },
            C11::GenericAssociation::Default(e) => GenericAssoc { ty: None, expr: e },
        })
    }
}

impl gazelle::Reducer<C11::ArgumentExpressionList<Self>> for CActions {
    fn reduce(&mut self, node: C11::ArgumentExpressionList<Self>) -> R<Vec<ExprNode>> {
        let C11::ArgumentExpressionList::List(list) = node;
        Ok(list)
    }
}

impl gazelle::Reducer<C11::PostfixExpression<Self>> for CActions {
    fn reduce(&mut self, node: C11::PostfixExpression<Self>) -> R<ExprNode> {
        Ok(match node {
            C11::PostfixExpression::Primary(e) => e,
            C11::PostfixExpression::Index(arr, idx) => expr(Expr::Index(arr, idx)),
            C11::PostfixExpression::Call(func, args) => {
                expr(Expr::Call(func, args.unwrap_or_default()))
            }
            C11::PostfixExpression::Dot(obj, name) => expr(Expr::Member(obj, name)),
            C11::PostfixExpression::Arrow(obj, name) => expr(Expr::PtrMember(obj, name)),
            C11::PostfixExpression::Postinc(e) => expr(Expr::UnaryOp(UnaryOp::PostInc, e)),
            C11::PostfixExpression::Postdec(e) => expr(Expr::UnaryOp(UnaryOp::PostDec, e)),
            C11::PostfixExpression::CompoundLit(ty, items, _comma) => {
                expr(Expr::CompoundLiteral(ty, items))
            }
        })
    }
}

impl gazelle::Reducer<C11::UnaryExpression<Self>> for CActions {
    fn reduce(&mut self, node: C11::UnaryExpression<Self>) -> R<ExprNode> {
        Ok(match node {
            C11::UnaryExpression::Postfix(e) => e,
            C11::UnaryExpression::Preinc(e) => expr(Expr::UnaryOp(UnaryOp::PreInc, e)),
            C11::UnaryExpression::Predec(e) => expr(Expr::UnaryOp(UnaryOp::PreDec, e)),
            C11::UnaryExpression::UnaryOp(op, e) => expr(Expr::UnaryOp(op, e)),
            C11::UnaryExpression::SizeofExpr(e) => expr(Expr::SizeofExpr(e)),
            C11::UnaryExpression::SizeofType(ty) => expr(Expr::SizeofType(ty)),
            C11::UnaryExpression::Alignof(ty) => expr(Expr::AlignofType(ty)),
        })
    }
}

impl gazelle::Reducer<C11::UnaryOperator<Self>> for CActions {
    fn reduce(&mut self, node: C11::UnaryOperator<Self>) -> R<UnaryOp> {
        Ok(match node {
            C11::UnaryOperator::Addr => UnaryOp::AddrOf,
            C11::UnaryOperator::Deref => UnaryOp::Deref,
            C11::UnaryOperator::Pos => UnaryOp::Plus,
            C11::UnaryOperator::Neg => UnaryOp::Neg,
            C11::UnaryOperator::Bitnot => UnaryOp::BitNot,
            C11::UnaryOperator::Lognot => UnaryOp::LogNot,
            _ => unreachable!(),
        })
    }
}

impl gazelle::Reducer<C11::CastExpression<Self>> for CActions {
    fn reduce(&mut self, node: C11::CastExpression<Self>) -> R<ExprNode> {
        Ok(match node {
            C11::CastExpression::Unary(e) => e,
            C11::CastExpression::Cast(ty, e) => expr(Expr::Cast(ty, e)),
        })
    }
}

impl gazelle::Reducer<C11::AssignmentExpression<Self>> for CActions {
    fn reduce(&mut self, node: C11::AssignmentExpression<Self>) -> R<ExprNode> {
        Ok(match node {
            C11::AssignmentExpression::Cast(e) => e,
            C11::AssignmentExpression::Binop(l, op, r) => expr(Expr::BinOp(op, l, r)),
            C11::AssignmentExpression::Mul(l, r) => expr(Expr::BinOp(Op::Mul, l, r)),
            C11::AssignmentExpression::Bitand(l, r) => expr(Expr::BinOp(Op::BitAnd, l, r)),
            C11::AssignmentExpression::Add(l, r) => expr(Expr::BinOp(Op::Add, l, r)),
            C11::AssignmentExpression::Sub(l, r) => expr(Expr::BinOp(Op::Sub, l, r)),
            C11::AssignmentExpression::Assign(l, r) => expr(Expr::BinOp(Op::Assign, l, r)),
            C11::AssignmentExpression::Ternary(cond, then, else_) => expr(Expr::Ternary(cond, then, else_)),
        })
    }
}

impl gazelle::Reducer<C11::Expression<Self>> for CActions {
    fn reduce(&mut self, node: C11::Expression<Self>) -> R<ExprNode> {
        Ok(match node {
            C11::Expression::Single(e) => e,
            C11::Expression::Comma(l, r) => expr(Expr::Comma(l, r)),
        })
    }
}

impl gazelle::Reducer<C11::ConstantExpression<Self>> for CActions {
    fn reduce(&mut self, node: C11::ConstantExpression<Self>) -> R<ExprNode> {
        let C11::ConstantExpression::ConstExpr(e) = node;
        Ok(e)
    }
}

// --- Initializers ---

impl gazelle::Reducer<C11::CInitializer<Self>> for CActions {
    fn reduce(&mut self, node: C11::CInitializer<Self>) -> R<Init> {
        Ok(match node {
            C11::CInitializer::Expr(e) => Init::Expr(e),
            C11::CInitializer::Brace(items, _comma) => Init::List(items),
        })
    }
}

impl gazelle::Reducer<C11::InitializerList<Self>> for CActions {
    fn reduce(&mut self, node: C11::InitializerList<Self>) -> R<Vec<InitItem>> {
        Ok(match node {
            C11::InitializerList::Single(desig, init) => {
                vec![InitItem { designation: desig.unwrap_or_default(), init }]
            }
            C11::InitializerList::Append(mut list, desig, init) => {
                list.push(InitItem { designation: desig.unwrap_or_default(), init });
                list
            }
        })
    }
}

impl gazelle::Reducer<C11::Designation<Self>> for CActions {
    fn reduce(&mut self, node: C11::Designation<Self>) -> R<Vec<Designator>> {
        let C11::Designation::Designation(desigs) = node;
        Ok(desigs)
    }
}

impl gazelle::Reducer<C11::Designator<Self>> for CActions {
    fn reduce(&mut self, node: C11::Designator<Self>) -> R<Designator> {
        Ok(match node {
            C11::Designator::Index(e) => Designator::Index(e),
            C11::Designator::Field(name) => Designator::Field(name),
        })
    }
}

// --- Init declarators ---

impl gazelle::Reducer<C11::InitDeclaratorDeclaratorVarname<Self>> for CActions {
    fn reduce(&mut self, node: C11::InitDeclaratorDeclaratorVarname<Self>) -> R<InitDeclarator> {
        Ok(match node {
            C11::InitDeclaratorDeclaratorVarname::Decl(d) => {
                InitDeclarator { name: d.name, derived: d.derived, init: None, ty: None }
            }
            C11::InitDeclaratorDeclaratorVarname::DeclInit(d, init) => {
                InitDeclarator { name: d.name, derived: d.derived, init: Some(init), ty: None }
            }
        })
    }
}

impl gazelle::Reducer<C11::InitDeclaratorDeclaratorTypedefname<Self>> for CActions {
    fn reduce(&mut self, node: C11::InitDeclaratorDeclaratorTypedefname<Self>) -> R<InitDeclarator> {
        Ok(match node {
            C11::InitDeclaratorDeclaratorTypedefname::Decl(d) => {
                InitDeclarator { name: d.name, derived: d.derived, init: None, ty: None }
            }
            C11::InitDeclaratorDeclaratorTypedefname::DeclInit(d, init) => {
                InitDeclarator { name: d.name, derived: d.derived, init: Some(init), ty: None }
            }
        })
    }
}

impl gazelle::Reducer<C11::InitDeclaratorListVarname<Self>> for CActions {
    fn reduce(&mut self, node: C11::InitDeclaratorListVarname<Self>) -> R<Vec<InitDeclarator>> {
        let C11::InitDeclaratorListVarname::List(list) = node;
        Ok(list)
    }
}

impl gazelle::Reducer<C11::InitDeclaratorListTypedef<Self>> for CActions {
    fn reduce(&mut self, node: C11::InitDeclaratorListTypedef<Self>) -> R<Vec<InitDeclarator>> {
        let C11::InitDeclaratorListTypedef::List(list) = node;
        Ok(list)
    }
}

// --- Declarations ---

impl gazelle::Reducer<C11::Declaration<Self>> for CActions {
    fn reduce(&mut self, node: C11::Declaration<Self>) -> R<Decl> {
        Ok(match node {
            C11::Declaration::VarDecl(specs, list) => {
                Decl { specs, is_typedef: false, declarators: list.unwrap_or_default() }
            }
            C11::Declaration::TypedefDecl(specs, list) => {
                Decl { specs, is_typedef: true, declarators: list.unwrap_or_default() }
            }
            C11::Declaration::StaticAssert(_) => {
                Decl { specs: vec![], is_typedef: false, declarators: vec![] }
            }
        })
    }
}

// --- Statements ---

impl gazelle::Reducer<C11::Statement<Self>> for CActions {
    fn reduce(&mut self, node: C11::Statement<Self>) -> R<Stmt> {
        Ok(match node {
            C11::Statement::Labeled(s) => s,
            C11::Statement::Compound(s) => s,
            C11::Statement::Expr(s) => s,
            C11::Statement::Selection(s) => s,
            C11::Statement::Iteration(s) => s,
            C11::Statement::Jump(s) => s,
        })
    }
}

impl gazelle::Reducer<C11::LabeledStatement<Self>> for CActions {
    fn reduce(&mut self, node: C11::LabeledStatement<Self>) -> R<Stmt> {
        Ok(match node {
            C11::LabeledStatement::Label(name, s) => Stmt::Labeled(name, Box::new(s)),
            C11::LabeledStatement::Case(e, s) => Stmt::Case(e, Box::new(s)),
            C11::LabeledStatement::Default(s) => Stmt::Default(Box::new(s)),
        })
    }
}

impl gazelle::Reducer<C11::CompoundStatement<Self>> for CActions {
    fn reduce(&mut self, node: C11::CompoundStatement<Self>) -> R<Stmt> {
        let C11::CompoundStatement::Compound(items) = node;
        Ok(Stmt::Compound(items))
    }
}

impl gazelle::Reducer<C11::BlockItem<Self>> for CActions {
    fn reduce(&mut self, node: C11::BlockItem<Self>) -> R<BlockItem> {
        Ok(match node {
            C11::BlockItem::Decl(d) => BlockItem::Decl(d),
            C11::BlockItem::Stmt(s) => BlockItem::Stmt(s),
        })
    }
}

impl gazelle::Reducer<C11::ExpressionStatement<Self>> for CActions {
    fn reduce(&mut self, node: C11::ExpressionStatement<Self>) -> R<Stmt> {
        let C11::ExpressionStatement::ExprStmt(e) = node;
        Ok(Stmt::Expr(e))
    }
}

impl gazelle::Reducer<C11::SelectionStatement<Self>> for CActions {
    fn reduce(&mut self, node: C11::SelectionStatement<Self>) -> R<Stmt> {
        Ok(match node {
            C11::SelectionStatement::IfElse(cond, then, else_) => {
                Stmt::If(cond, Box::new(then), Some(Box::new(else_)))
            }
            C11::SelectionStatement::If(cond, then) => {
                Stmt::If(cond, Box::new(then), None)
            }
            C11::SelectionStatement::Switch(e, s) => {
                Stmt::Switch(e, Box::new(s))
            }
        })
    }
}

impl gazelle::Reducer<C11::IterationStatement<Self>> for CActions {
    fn reduce(&mut self, node: C11::IterationStatement<Self>) -> R<Stmt> {
        Ok(match node {
            C11::IterationStatement::While(cond, body) => Stmt::While(cond, Box::new(body)),
            C11::IterationStatement::DoWhile(body, cond) => Stmt::DoWhile(Box::new(body), cond),
            C11::IterationStatement::ForExpr(init, cond, step, body) => {
                Stmt::For(ForInit::Expr(init), cond, step, Box::new(body))
            }
            C11::IterationStatement::ForDecl(decl, cond, step, body) => {
                Stmt::For(ForInit::Decl(decl), cond, step, Box::new(body))
            }
        })
    }
}

impl gazelle::Reducer<C11::JumpStatement<Self>> for CActions {
    fn reduce(&mut self, node: C11::JumpStatement<Self>) -> R<Stmt> {
        Ok(match node {
            C11::JumpStatement::Goto(name) => Stmt::Goto(name),
            C11::JumpStatement::Continue => Stmt::Continue,
            C11::JumpStatement::Break => Stmt::Break,
            C11::JumpStatement::Return(e) => Stmt::Return(e),
        })
    }
}

// --- Top level ---

impl gazelle::Reducer<C11::TranslationUnitFile<Self>> for CActions {
    fn reduce(&mut self, node: C11::TranslationUnitFile<Self>) -> R<Vec<C11::ExternalDeclaration<Self>>> {
        let C11::TranslationUnitFile::File(defs) = node;
        Ok(defs)
    }
}

impl gazelle::Reducer<C11::FunctionDefinition1<Self>> for CActions {
    fn reduce(&mut self, node: C11::FunctionDefinition1<Self>) -> R<FuncHeader> {
        let C11::FunctionDefinition1::FuncDef1(return_specs, d) = node;
        let ctx = self.ctx.save();
        let Declarator { name, mut derived, kind } = d;
        let (params, variadic) = if let Some(pos) = derived.iter().position(|d| matches!(d, DerivedType::Function(..))) {
            if let DerivedType::Function(params, variadic) = derived.remove(pos) { (params, variadic) } else { (vec![], false) }
        } else { (vec![], false) };
        let return_derived = derived;
        if let DeclKind::Func(ref fctx) = kind {
            self.ctx.restore(fctx.clone());
            self.ctx.declare_varname(&name);
        }
        Ok(FuncHeader { ctx, name, return_specs, return_derived, params, variadic })
    }
}

impl gazelle::Reducer<C11::FunctionDefinition<Self>> for CActions {
    fn reduce(&mut self, node: C11::FunctionDefinition<Self>) -> R<FunctionDef> {
        let C11::FunctionDefinition::FuncDef(header, _decls, body) = node;
        self.ctx.restore(header.ctx);
        Ok(FunctionDef {
            name: header.name, return_specs: header.return_specs,
            return_derived: header.return_derived, params: header.params, body,
            variadic: header.variadic,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Sign { Signed, Unsigned }

#[derive(Clone, Debug, PartialEq)]
pub enum CType {
    Void,
    Bool,
    Char(Sign),
    Short(Sign),
    Int(Sign),
    Long(Sign),
    LongLong(Sign),
    Float,
    Double,
    LongDouble,
    Pointer(Box<CType>),
    Array(Box<CType>, Option<u64>),
    Function { ret: Box<CType>, params: Vec<CType>, variadic: bool },
    Struct(String),
    Union(String),
    Enum(String),
    Typedef(String),
}

#[derive(Clone, Copy, Debug)]
pub enum Op {
    // Arithmetic
    Add, Sub, Mul, Div, Mod,
    // Bitwise
    BitAnd, BitOr, BitXor, Shl, Shr,
    // Logical
    And, Or,
    // Comparison
    Eq, Ne, Lt, Gt, Le, Ge,
    // Assignment
    Assign,
    AddAssign, SubAssign, MulAssign, DivAssign, ModAssign,
    ShlAssign, ShrAssign, BitAndAssign, BitOrAssign, BitXorAssign,
}

/// Unary operator
#[derive(Clone, Copy, Debug)]
pub enum UnaryOp {
    AddrOf, Deref, Plus, Neg, BitNot, LogNot,
    PreInc, PreDec, PostInc, PostDec,
}

/// Derived type (pointer, array, function) — ordered inside-out from variable
#[derive(Clone, Debug)]
pub enum DerivedType {
    Pointer,
    Array(Option<ExprNode>),
    Function(Vec<Param>, bool), // params, is_variadic
}

/// Expression with metadata. The box lives here so Expr variants stay clean.
#[derive(Clone, Debug)]
pub struct ExprNode {
    pub expr: Box<Expr>,
    pub ty: Option<CType>,
}

pub fn expr(e: Expr) -> ExprNode {
    ExprNode { expr: Box::new(e), ty: None }
}

#[derive(Clone, Debug)]
pub enum Expr {
    Var(String),
    Constant(String),
    StringLit(String),
    BinOp(Op, ExprNode, ExprNode),
    UnaryOp(UnaryOp, ExprNode),
    Call(ExprNode, Vec<ExprNode>),
    Index(ExprNode, ExprNode),
    Member(ExprNode, String),
    PtrMember(ExprNode, String),
    Ternary(ExprNode, ExprNode, ExprNode),
    Cast(CType, ExprNode),
    SizeofExpr(ExprNode),
    SizeofType(CType),
    AlignofType(CType),
    CompoundLiteral(CType, Vec<InitItem>),
    VaArg(ExprNode, CType),
    Generic(ExprNode, Vec<GenericAssoc>),
    Comma(ExprNode, ExprNode),

    // Conversion nodes (inserted by type pass, Phase 4)
    Load(ExprNode),
    Widen(ExprNode),
    Decay(ExprNode),
    FuncToPtr(ExprNode),
    ImplicitCast(CType, ExprNode),
}

// === Statements ===

#[derive(Clone, Debug)]
pub enum Stmt {
    Compound(Vec<BlockItem>),
    Expr(Option<ExprNode>),
    If(ExprNode, Box<Stmt>, Option<Box<Stmt>>),
    Switch(ExprNode, Box<Stmt>),
    While(ExprNode, Box<Stmt>),
    DoWhile(Box<Stmt>, ExprNode),
    For(ForInit, Option<ExprNode>, Option<ExprNode>, Box<Stmt>),
    Goto(String),
    Continue,
    Break,
    Return(Option<ExprNode>),
    Labeled(String, Box<Stmt>),
    Case(ExprNode, Box<Stmt>),
    Default(Box<Stmt>),
}

#[derive(Clone, Debug)]
pub enum ForInit {
    Expr(Option<ExprNode>),
    Decl(Decl),
}

#[derive(Clone, Debug)]
pub enum BlockItem {
    Decl(Decl),
    Stmt(Stmt),
}

// === Declarations ===

#[derive(Clone, Debug)]
pub struct Decl {
    pub specs: Vec<DeclSpec>,
    pub is_typedef: bool,
    pub declarators: Vec<InitDeclarator>,
}

#[derive(Clone, Debug)]
pub struct InitDeclarator {
    pub name: String,
    pub derived: Vec<DerivedType>,
    pub init: Option<Init>,
    pub ty: Option<CType>,
}

#[derive(Clone, Debug)]
pub enum DeclSpec {
    Storage(StorageClass),
    Type(TypeSpec),
    Qual(TypeQualifier),
    Func(FuncSpec),
    Align(AlignSpec),
}

#[derive(Clone, Copy, Debug)]
pub enum StorageClass {
    Extern, Static, ThreadLocal, Auto, Register,
}

#[derive(Clone, Debug)]
pub enum TypeSpec {
    Void, Char, Short, Int, Long, Float, Double,
    Signed, Unsigned, Bool, Complex,
    Struct(StructOrUnion, StructSpec),
    Enum(EnumSpec),
    TypedefName(String),
    Atomic(CType),
}

#[derive(Clone, Copy, Debug)]
pub enum StructOrUnion { Struct, Union }

#[derive(Clone, Copy, Debug)]
pub enum TypeQualifier {
    Const, Restrict, Volatile, Atomic,
}

#[derive(Clone, Copy, Debug)]
pub enum FuncSpec {
    Inline, Noreturn,
}

#[derive(Clone, Debug)]
pub enum AlignSpec {
    Type(CType),
    Expr(ExprNode),
}

// === Structs/Unions ===

#[derive(Clone, Debug)]
pub struct StructSpec {
    pub name: Option<String>,
    pub members: Vec<StructMember>,
}

#[derive(Clone, Debug)]
pub struct StructMember {
    pub specs: Vec<DeclSpec>,
    pub declarators: Vec<MemberDeclarator>,
}

#[derive(Clone, Debug)]
pub struct MemberDeclarator {
    pub name: Option<String>,
    pub derived: Vec<DerivedType>,
    pub bitfield: Option<ExprNode>,
}

// === Enums ===

#[derive(Clone, Debug)]
pub struct EnumSpec {
    pub name: Option<String>,
    pub enumerators: Vec<Enumerator>,
}

#[derive(Clone, Debug)]
pub struct Enumerator {
    pub name: String,
    pub value: Option<ExprNode>,
}

// === Initializers ===

#[derive(Clone, Debug)]
pub enum Init {
    Expr(ExprNode),
    List(Vec<InitItem>),
}

#[derive(Clone, Debug)]
pub struct InitItem {
    pub designation: Vec<Designator>,
    pub init: Init,
}

#[derive(Clone, Debug)]
pub enum Designator {
    Index(ExprNode),
    Field(String),
}

// === Generic selection ===

#[derive(Clone, Debug)]
pub struct GenericAssoc {
    pub ty: Option<CType>, // None = default
    pub expr: ExprNode,
}

// === Parameters ===

#[derive(Clone, Debug)]
pub struct Param {
    pub specs: Vec<DeclSpec>,
    pub name: Option<String>,
    pub derived: Vec<DerivedType>,
    pub ty: Option<CType>,
}

// === Top level ===

#[derive(Clone, Debug)]
pub struct FunctionDef {
    pub name: String,
    pub return_specs: Vec<DeclSpec>,
    pub return_derived: Vec<DerivedType>,
    pub params: Vec<Param>,
    pub body: Stmt,
    pub variadic: bool,
}

#[derive(Clone, Debug)]
pub struct TranslationUnit {
    pub decls: Vec<Decl>,
    pub functions: Vec<FunctionDef>,
    /// Struct/union tag → (is_union, field list), populated by the type checker.
    /// Each field is (name, type, bitfield_width) where bitfield_width is Some(n) for `:n` fields.
    pub structs: std::collections::HashMap<String, (bool, Vec<(String, CType, Option<u8>)>)>,
}

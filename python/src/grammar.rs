use gazelle_macros::gazelle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AugOp {
    Add, Sub, Mul, Div, FloorDiv, Mod, Pow, Shl, Shr, BitAnd, BitOr, BitXor, MatMul,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompOp {
    Eq, Ne, Lt, Gt, Le, Ge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Div, FloorDiv, Mod, Shl, Shr, BitAnd, BitOr, BitXor,
}

gazelle! {
    pub(crate) grammar Python = "../grammars/python.gzl"
}

// Dummy actions — all types are ()
pub struct PyActions;

impl PythonTypes for PyActions {
    type Error = gazelle::ParseError;
    type Name = String;
    type Number = String;
    type String = String;
    type Augassign = AugOp;
    type CompOp = CompOp;
    type Binop = BinOp;
    type FileInput = gazelle::Ignore;
    type Statements = gazelle::Ignore;
    type Statement = gazelle::Ignore;
    type SimpleStmts = gazelle::Ignore;
    type SimpleStmt = gazelle::Ignore;
    type AssertMsg = gazelle::Ignore;
    type AssignRhs = gazelle::Ignore;
    type YieldExpr = gazelle::Ignore;
    type YieldArg = gazelle::Ignore;
    type RaiseArgs = gazelle::Ignore;
    type RaiseFrom = gazelle::Ignore;
    type DottedName = gazelle::Ignore;
    type DottedAsName = gazelle::Ignore;
    type DottedAsNames = gazelle::Ignore;
    type AsName = gazelle::Ignore;
    type ImportFromPath = gazelle::Ignore;
    type Dots = gazelle::Ignore;
    type ImportTargets = gazelle::Ignore;
    type ImportAsName = gazelle::Ignore;
    type ArithExpr = gazelle::Ignore;
    type StarTarget = gazelle::Ignore;
    type StarTargets = gazelle::Ignore;
    type StarExpression = gazelle::Ignore;
    type StarExpressions = gazelle::Ignore;
    type StarNamedExpression = gazelle::Ignore;
    type Comparison = gazelle::Ignore;
    type CompPair = gazelle::Ignore;
    type Inversion = gazelle::Ignore;
    type Conjunction = gazelle::Ignore;
    type Disjunction = gazelle::Ignore;
    type NamedExpression = gazelle::Ignore;
    type Expression = gazelle::Ignore;
    type LambdaExpr = gazelle::Ignore;
    type LambdaParams = gazelle::Ignore;
    type LambdaParam = gazelle::Ignore;
    type LambdaDefault = gazelle::Ignore;
    type Primary = gazelle::Ignore;
    type Atom = gazelle::Ignore;
    type StringConcat = gazelle::Ignore;
    type ParenBody = gazelle::Ignore;
    type ListBody = gazelle::Ignore;
    type Slices = gazelle::Ignore;
    type Slice = gazelle::Ignore;
    type SliceStep = gazelle::Ignore;
    type Arguments = gazelle::Ignore;
    type Arg = gazelle::Ignore;
    type DictOrSet = gazelle::Ignore;
    type DictItems = gazelle::Ignore;
    type Kvpair = gazelle::Ignore;
    type DictComp = gazelle::Ignore;
    type SetComp = gazelle::Ignore;
    type CompFor = gazelle::Ignore;
    type Filter = gazelle::Ignore;
    type CompoundStmt = gazelle::Ignore;
    type Block = gazelle::Ignore;
    type IfStmt = gazelle::Ignore;
    type ElifClause = gazelle::Ignore;
    type ElseClause = gazelle::Ignore;
    type WhileStmt = gazelle::Ignore;
    type ForStmt = gazelle::Ignore;
    type TryStmt = gazelle::Ignore;
    type ExceptClause = gazelle::Ignore;
    type ExceptAs = gazelle::Ignore;
    type FinallyClause = gazelle::Ignore;
    type WithStmt = gazelle::Ignore;
    type WithItem = gazelle::Ignore;
    type WithAs = gazelle::Ignore;
    type FuncDef = gazelle::Ignore;
    type ReturnAnnot = gazelle::Ignore;
    type Decorators = gazelle::Ignore;
    type Decorator = gazelle::Ignore;
    type Params = gazelle::Ignore;
    type Param = gazelle::Ignore;
    type ParamAnnot = gazelle::Ignore;
    type ParamDefault = gazelle::Ignore;
    type ClassDef = gazelle::Ignore;
    type ClassArgs = gazelle::Ignore;
    type AsyncStmt = gazelle::Ignore;
}

// PythonActions is auto-implemented via blanket impl

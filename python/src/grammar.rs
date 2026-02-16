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
    type Comp_op = CompOp;
    type Binop = BinOp;
    type File_input = gazelle::Ignore;
    type Statements = gazelle::Ignore;
    type Statement = gazelle::Ignore;
    type Simple_stmts = gazelle::Ignore;
    type Simple_stmt = gazelle::Ignore;
    type Assert_msg = gazelle::Ignore;
    type Assign_rhs = gazelle::Ignore;
    type Yield_expr = gazelle::Ignore;
    type Yield_arg = gazelle::Ignore;
    type Raise_args = gazelle::Ignore;
    type Raise_from = gazelle::Ignore;
    type Dotted_name = gazelle::Ignore;
    type Dotted_as_name = gazelle::Ignore;
    type Dotted_as_names = gazelle::Ignore;
    type As_name = gazelle::Ignore;
    type Import_from_path = gazelle::Ignore;
    type Dots = gazelle::Ignore;
    type Import_targets = gazelle::Ignore;
    type Import_as_name = gazelle::Ignore;
    type Arith_expr = gazelle::Ignore;
    type Star_target = gazelle::Ignore;
    type Star_targets = gazelle::Ignore;
    type Star_expression = gazelle::Ignore;
    type Star_expressions = gazelle::Ignore;
    type Star_named_expression = gazelle::Ignore;
    type Comparison = gazelle::Ignore;
    type Comp_pair = gazelle::Ignore;
    type Inversion = gazelle::Ignore;
    type Conjunction = gazelle::Ignore;
    type Disjunction = gazelle::Ignore;
    type Named_expression = gazelle::Ignore;
    type Expression = gazelle::Ignore;
    type Lambda_expr = gazelle::Ignore;
    type Lambda_params = gazelle::Ignore;
    type Lambda_param = gazelle::Ignore;
    type Lambda_default = gazelle::Ignore;
    type Primary = gazelle::Ignore;
    type Atom = gazelle::Ignore;
    type String_concat = gazelle::Ignore;
    type Paren_body = gazelle::Ignore;
    type List_body = gazelle::Ignore;
    type Slices = gazelle::Ignore;
    type Slice = gazelle::Ignore;
    type Slice_step = gazelle::Ignore;
    type Arguments = gazelle::Ignore;
    type Arg = gazelle::Ignore;
    type Dict_or_set = gazelle::Ignore;
    type Dict_items = gazelle::Ignore;
    type Kvpair = gazelle::Ignore;
    type Dict_comp = gazelle::Ignore;
    type Set_comp = gazelle::Ignore;
    type Comp_for = gazelle::Ignore;
    type Filter = gazelle::Ignore;
    type Compound_stmt = gazelle::Ignore;
    type Block = gazelle::Ignore;
    type If_stmt = gazelle::Ignore;
    type Elif_clause = gazelle::Ignore;
    type Else_clause = gazelle::Ignore;
    type While_stmt = gazelle::Ignore;
    type For_stmt = gazelle::Ignore;
    type Try_stmt = gazelle::Ignore;
    type Except_clause = gazelle::Ignore;
    type Except_as = gazelle::Ignore;
    type Finally_clause = gazelle::Ignore;
    type With_stmt = gazelle::Ignore;
    type With_item = gazelle::Ignore;
    type With_as = gazelle::Ignore;
    type Func_def = gazelle::Ignore;
    type Return_annot = gazelle::Ignore;
    type Decorators = gazelle::Ignore;
    type Decorator = gazelle::Ignore;
    type Params = gazelle::Ignore;
    type Param = gazelle::Ignore;
    type Param_annot = gazelle::Ignore;
    type Param_default = gazelle::Ignore;
    type Class_def = gazelle::Ignore;
    type Class_args = gazelle::Ignore;
    type Async_stmt = gazelle::Ignore;
}

// PythonActions is auto-implemented via blanket impl


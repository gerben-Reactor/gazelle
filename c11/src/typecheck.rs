use std::collections::HashMap;
use crate::ast::*;
use crate::types::resolve_type;

fn typed(e: Expr, ty: CType) -> ExprNode {
    ExprNode { expr: Box::new(e), ty: Some(ty) }
}

fn is_lvalue(e: &Expr) -> bool {
    matches!(e, Expr::Var(_) | Expr::UnaryOp(UnaryOp::Deref, _)
        | Expr::Index(..) | Expr::Member(..) | Expr::PtrMember(..))
}

fn is_arith(ty: &CType) -> bool {
    matches!(ty, CType::Bool | CType::Char(_) | CType::Short(_) | CType::Int(_)
        | CType::Long(_) | CType::LongLong(_) | CType::Float | CType::Double | CType::LongDouble)
}

fn rank(ty: &CType) -> u8 {
    match ty {
        CType::Bool => 0,
        CType::Char(_) => 1,
        CType::Short(_) => 2,
        CType::Int(_) => 3,
        CType::Long(_) => 4,
        CType::LongLong(_) => 5,
        CType::Float => 6,
        CType::Double => 7,
        CType::LongDouble => 8,
        _ => 0,
    }
}

/// Integer promotion (C11 6.3.1.1): Bool, Char, Short → Int(Signed).
fn promote(ty: &CType) -> CType {
    match ty {
        CType::Bool | CType::Char(_) | CType::Short(_) => CType::Int(Sign::Signed),
        other => other.clone(),
    }
}

/// Usual arithmetic conversions (C11 6.3.1.8).
fn usual_arith(a: &CType, b: &CType) -> CType {
    let (a, b) = (promote(a), promote(b));
    if rank(&a) != rank(&b) {
        return if rank(&a) > rank(&b) { a } else { b };
    }
    // Same rank: unsigned wins for integer types
    match (&a, &b) {
        (CType::Int(s1), CType::Int(s2)) => CType::Int(unsigned_wins(*s1, *s2)),
        (CType::Long(s1), CType::Long(s2)) => CType::Long(unsigned_wins(*s1, *s2)),
        (CType::LongLong(s1), CType::LongLong(s2)) => CType::LongLong(unsigned_wins(*s1, *s2)),
        _ => a,
    }
}

fn unsigned_wins(a: Sign, b: Sign) -> Sign {
    if a == Sign::Unsigned || b == Sign::Unsigned { Sign::Unsigned } else { Sign::Signed }
}

fn parse_constant_type(s: &str) -> CType {
    if s.starts_with('\'') {
        return CType::Int(Sign::Signed);
    }
    let lower = s.to_ascii_lowercase();
    let is_hex = lower.starts_with("0x");
    let is_float = lower.contains('.')
        || (!is_hex && lower.contains('e'))
        || (is_hex && lower.contains('p'));
    if is_float {
        if lower.ends_with('f') { return CType::Float; }
        if lower.ends_with('l') { return CType::LongDouble; }
        return CType::Double;
    }
    if lower.ends_with("ull") || lower.ends_with("llu") {
        return CType::LongLong(Sign::Unsigned);
    }
    if lower.ends_with("ll") { return CType::LongLong(Sign::Signed); }
    if lower.ends_with("ul") || lower.ends_with("lu") {
        return CType::Long(Sign::Unsigned);
    }
    if lower.ends_with('l') { return CType::Long(Sign::Signed); }
    if lower.ends_with('u') { return CType::Int(Sign::Unsigned); }
    CType::Int(Sign::Signed)
}

struct TypeChecker {
    scopes: Vec<HashMap<String, CType>>,
    structs: HashMap<String, Vec<(String, CType)>>,
    ret_type: CType,
}

impl TypeChecker {
    fn new(ret_type: CType) -> Self {
        Self { scopes: vec![HashMap::new()], structs: HashMap::new(), ret_type }
    }

    fn push_scope(&mut self) { self.scopes.push(HashMap::new()); }
    fn pop_scope(&mut self) { self.scopes.pop(); }

    fn define(&mut self, name: String, ty: CType) {
        self.scopes.last_mut().unwrap().insert(name, ty);
    }

    fn lookup(&self, name: &str) -> Result<CType, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Ok(ty.clone());
            }
        }
        Err(format!("undefined variable: {}", name))
    }

    fn register_struct_from_specs(&mut self, specs: &[DeclSpec]) {
        for spec in specs {
            if let DeclSpec::Type(TypeSpec::Struct(_, ss)) = spec {
                if let Some(name) = &ss.name {
                    if !ss.members.is_empty() {
                        let fields = ss.members.iter()
                            .flat_map(|m| m.declarators.iter().filter_map(|d| {
                                let ty = resolve_type(&m.specs, &d.derived).ok()?;
                                Some((d.name.clone()?, ty))
                            }))
                            .collect();
                        self.structs.insert(name.clone(), fields);
                    }
                }
            }
        }
    }

    fn lookup_field(&self, struct_name: &str, field: &str) -> Result<CType, String> {
        let fields = self.structs.get(struct_name)
            .ok_or_else(|| format!("unknown struct: {}", struct_name))?;
        fields.iter()
            .find(|(n, _)| n == field)
            .map(|(_, ty)| ty.clone())
            .ok_or_else(|| format!("no field '{}' in struct {}", field, struct_name))
    }

    // === Core: rvalue and check_expr ===

    fn rvalue(&mut self, e: ExprNode) -> Result<ExprNode, String> {
        let e = self.check_expr(e)?;
        let ty = e.ty.clone().unwrap();
        match &ty {
            CType::Array(elem, _) => {
                let ptr_ty = CType::Pointer(elem.clone());
                Ok(typed(Expr::Decay(e), ptr_ty))
            }
            CType::Function { .. } => {
                Ok(typed(Expr::FuncToPtr(e), CType::Pointer(Box::new(ty))))
            }
            _ if is_lvalue(&e.expr) => Ok(typed(Expr::Load(e), ty)),
            _ => Ok(e),
        }
    }

    fn check_expr(&mut self, e: ExprNode) -> Result<ExprNode, String> {
        if e.ty.is_some() { return Ok(e); } // already typed
        let (expr, ty) = match *e.expr {
            Expr::Var(name) => {
                let ty = self.lookup(&name)?;
                (Expr::Var(name), ty)
            }
            Expr::Constant(s) => {
                let ty = parse_constant_type(&s);
                (Expr::Constant(s), ty)
            }
            Expr::StringLit(s) => {
                (Expr::StringLit(s), CType::Pointer(Box::new(CType::Char(Sign::Signed))))
            }
            Expr::BinOp(op, l, r) => return self.check_binop(op, l, r),
            Expr::UnaryOp(op, inner) => return self.check_unaryop(op, inner),
            Expr::Call(func, args) => {
                let func = self.rvalue(func)?;
                let ret_ty = match func.ty.as_ref().unwrap() {
                    CType::Pointer(inner) => match inner.as_ref() {
                        CType::Function { ret, .. } => ret.as_ref().clone(),
                        _ => return Err("call on non-function pointer".into()),
                    },
                    _ => return Err("call on non-function".into()),
                };
                let args = args.into_iter()
                    .map(|a| self.rvalue(a))
                    .collect::<Result<Vec<_>, _>>()?;
                (Expr::Call(func, args), ret_ty)
            }
            Expr::Index(arr, idx) => {
                let arr = self.rvalue(arr)?;
                let idx = self.rvalue(idx)?;
                let elem_ty = match arr.ty.as_ref().unwrap() {
                    CType::Pointer(inner) => inner.as_ref().clone(),
                    _ => return Err("index on non-pointer".into()),
                };
                (Expr::Index(arr, idx), elem_ty)
            }
            Expr::Member(obj, field) => {
                let obj = self.check_expr(obj)?;
                let struct_name = match obj.ty.as_ref().unwrap() {
                    CType::Struct(n) => n.clone(),
                    _ => return Err("member access on non-struct".into()),
                };
                let field_ty = self.lookup_field(&struct_name, &field)?;
                (Expr::Member(obj, field), field_ty)
            }
            Expr::PtrMember(obj, field) => {
                let obj = self.rvalue(obj)?;
                let struct_name = match obj.ty.as_ref().unwrap() {
                    CType::Pointer(inner) => match inner.as_ref() {
                        CType::Struct(n) => n.clone(),
                        _ => return Err("-> on non-struct pointer".into()),
                    },
                    _ => return Err("-> on non-pointer".into()),
                };
                let field_ty = self.lookup_field(&struct_name, &field)?;
                (Expr::PtrMember(obj, field), field_ty)
            }
            Expr::Ternary(cond, then, else_) => {
                let cond = self.rvalue(cond)?;
                let then = self.rvalue(then)?;
                let else_ = self.rvalue(else_)?;
                let ty = usual_arith(then.ty.as_ref().unwrap(), else_.ty.as_ref().unwrap());
                (Expr::Ternary(cond, then, else_), ty)
            }
            Expr::Cast(tn, inner) => {
                let inner = self.rvalue(inner)?;
                let ty = resolve_type(&tn.specs, &tn.derived)?;
                (Expr::Cast(tn, inner), ty)
            }
            Expr::SizeofExpr(inner) => {
                let inner = self.check_expr(inner)?;
                (Expr::SizeofExpr(inner), CType::Long(Sign::Unsigned))
            }
            Expr::SizeofType(tn) => (Expr::SizeofType(tn), CType::Long(Sign::Unsigned)),
            Expr::AlignofType(tn) => (Expr::AlignofType(tn), CType::Long(Sign::Unsigned)),
            Expr::Comma(l, r) => {
                let l = self.rvalue(l)?;
                let r = self.rvalue(r)?;
                let ty = r.ty.clone().unwrap();
                (Expr::Comma(l, r), ty)
            }
            Expr::CompoundLiteral(tn, items) => {
                let ty = resolve_type(&tn.specs, &tn.derived)?;
                (Expr::CompoundLiteral(tn, items), ty)
            }
            Expr::VaArg(inner, tn) => {
                let inner = self.rvalue(inner)?;
                let ty = resolve_type(&tn.specs, &tn.derived)?;
                (Expr::VaArg(inner, tn), ty)
            }
            Expr::Generic(ctrl, assocs) => {
                let ctrl = self.rvalue(ctrl)?;
                (Expr::Generic(ctrl, assocs), CType::Int(Sign::Signed)) // placeholder
            }
            // Conversion nodes — already typed, pass through
            other => return Ok(ExprNode { expr: Box::new(other), ty: e.ty }),
        };
        Ok(typed(expr, ty))
    }

    fn check_binop(&mut self, op: Op, l: ExprNode, r: ExprNode) -> Result<ExprNode, String> {
        match op {
            Op::Assign => {
                let l = self.check_expr(l)?;
                let mut r = self.rvalue(r)?;
                let ty = l.ty.clone().unwrap();
                if r.ty.as_ref().unwrap() != &ty {
                    r = typed(Expr::ImplicitCast(ty.clone(), r), ty.clone());
                }
                Ok(typed(Expr::BinOp(Op::Assign, l, r), ty))
            }
            Op::AddAssign | Op::SubAssign | Op::MulAssign | Op::DivAssign | Op::ModAssign |
            Op::ShlAssign | Op::ShrAssign | Op::BitAndAssign | Op::BitOrAssign | Op::BitXorAssign => {
                let l = self.check_expr(l)?;
                let r = self.rvalue(r)?;
                let ty = l.ty.clone().unwrap();
                Ok(typed(Expr::BinOp(op, l, r), ty))
            }
            Op::And | Op::Or => {
                let l = self.rvalue(l)?;
                let r = self.rvalue(r)?;
                Ok(typed(Expr::BinOp(op, l, r), CType::Int(Sign::Signed)))
            }
            Op::Eq | Op::Ne | Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                let l = self.rvalue(l)?;
                let r = self.rvalue(r)?;
                let (l, r) = self.apply_arith_conv(l, r);
                Ok(typed(Expr::BinOp(op, l, r), CType::Int(Sign::Signed)))
            }
            Op::Shl | Op::Shr => {
                let l = self.rvalue(l)?;
                let r = self.rvalue(r)?;
                let ty = promote(l.ty.as_ref().unwrap());
                let l = if *l.ty.as_ref().unwrap() != ty {
                    typed(Expr::Widen(l), ty.clone())
                } else { l };
                Ok(typed(Expr::BinOp(op, l, r), ty))
            }
            Op::Add | Op::Sub => {
                let l = self.rvalue(l)?;
                let r = self.rvalue(r)?;
                // Pointer arithmetic
                if let CType::Pointer(_) = l.ty.as_ref().unwrap() {
                    let ty = l.ty.clone().unwrap();
                    return Ok(typed(Expr::BinOp(op, l, r), ty));
                }
                if matches!(op, Op::Add) {
                    if let CType::Pointer(_) = r.ty.as_ref().unwrap() {
                        let ty = r.ty.clone().unwrap();
                        return Ok(typed(Expr::BinOp(op, l, r), ty));
                    }
                }
                let (l, r) = self.apply_arith_conv(l, r);
                let ty = l.ty.clone().unwrap();
                Ok(typed(Expr::BinOp(op, l, r), ty))
            }
            _ => {
                // Mul, Div, Mod, BitAnd, BitOr, BitXor
                let l = self.rvalue(l)?;
                let r = self.rvalue(r)?;
                let (l, r) = self.apply_arith_conv(l, r);
                let ty = l.ty.clone().unwrap();
                Ok(typed(Expr::BinOp(op, l, r), ty))
            }
        }
    }

    fn apply_arith_conv(&self, l: ExprNode, r: ExprNode) -> (ExprNode, ExprNode) {
        let lt = l.ty.as_ref().unwrap();
        let rt = r.ty.as_ref().unwrap();
        if !is_arith(lt) || !is_arith(rt) { return (l, r); }
        let common = usual_arith(lt, rt);
        let l = if *lt != common { typed(Expr::Widen(l), common.clone()) } else { l };
        let r = if *rt != common { typed(Expr::Widen(r), common) } else { r };
        (l, r)
    }

    fn check_unaryop(&mut self, op: UnaryOp, inner: ExprNode) -> Result<ExprNode, String> {
        match op {
            UnaryOp::AddrOf => {
                let inner = self.check_expr(inner)?;
                let ty = CType::Pointer(Box::new(inner.ty.clone().unwrap()));
                Ok(typed(Expr::UnaryOp(UnaryOp::AddrOf, inner), ty))
            }
            UnaryOp::Deref => {
                let inner = self.rvalue(inner)?;
                let ty = match inner.ty.as_ref().unwrap() {
                    CType::Pointer(inner_ty) => inner_ty.as_ref().clone(),
                    _ => return Err("dereference of non-pointer".into()),
                };
                Ok(typed(Expr::UnaryOp(UnaryOp::Deref, inner), ty))
            }
            UnaryOp::LogNot => {
                let inner = self.rvalue(inner)?;
                Ok(typed(Expr::UnaryOp(UnaryOp::LogNot, inner), CType::Int(Sign::Signed)))
            }
            UnaryOp::PreInc | UnaryOp::PreDec | UnaryOp::PostInc | UnaryOp::PostDec => {
                let inner = self.check_expr(inner)?;
                let ty = inner.ty.clone().unwrap();
                Ok(typed(Expr::UnaryOp(op, inner), ty))
            }
            _ => {
                // Plus, Neg, BitNot — integer promotion
                let inner = self.rvalue(inner)?;
                let ty = promote(inner.ty.as_ref().unwrap());
                let inner = if *inner.ty.as_ref().unwrap() != ty {
                    typed(Expr::Widen(inner), ty.clone())
                } else { inner };
                Ok(typed(Expr::UnaryOp(op, inner), ty))
            }
        }
    }

    // === Statements and declarations ===

    fn check_stmt(&mut self, stmt: Stmt) -> Result<Stmt, String> {
        Ok(match stmt {
            Stmt::Compound(items) => {
                self.push_scope();
                let items = items.into_iter()
                    .map(|i| self.check_block_item(i))
                    .collect::<Result<Vec<_>, _>>()?;
                self.pop_scope();
                Stmt::Compound(items)
            }
            Stmt::Expr(Some(e)) => Stmt::Expr(Some(self.rvalue(e)?)),
            Stmt::Expr(None) => Stmt::Expr(None),
            Stmt::If(cond, then, else_) => {
                let cond = self.rvalue(cond)?;
                let then = Box::new(self.check_stmt(*then)?);
                let else_ = else_.map(|e| self.check_stmt(*e)).transpose()?.map(Box::new);
                Stmt::If(cond, then, else_)
            }
            Stmt::Switch(cond, body) => {
                Stmt::Switch(self.rvalue(cond)?, Box::new(self.check_stmt(*body)?))
            }
            Stmt::While(cond, body) => {
                Stmt::While(self.rvalue(cond)?, Box::new(self.check_stmt(*body)?))
            }
            Stmt::DoWhile(body, cond) => {
                Stmt::DoWhile(Box::new(self.check_stmt(*body)?), self.rvalue(cond)?)
            }
            Stmt::For(init, cond, incr, body) => {
                let init = match init {
                    ForInit::Expr(Some(e)) => ForInit::Expr(Some(self.rvalue(e)?)),
                    ForInit::Expr(None) => ForInit::Expr(None),
                    ForInit::Decl(d) => ForInit::Decl(self.check_decl(d)?),
                };
                let cond = cond.map(|e| self.rvalue(e)).transpose()?;
                let incr = incr.map(|e| self.rvalue(e)).transpose()?;
                Stmt::For(init, cond, incr, Box::new(self.check_stmt(*body)?))
            }
            Stmt::Return(Some(e)) => {
                let mut e = self.rvalue(e)?;
                let ret = self.ret_type.clone();
                if *e.ty.as_ref().unwrap() != ret {
                    e = typed(Expr::ImplicitCast(ret.clone(), e), ret);
                }
                Stmt::Return(Some(e))
            }
            Stmt::Return(None) => Stmt::Return(None),
            Stmt::Labeled(name, s) => Stmt::Labeled(name, Box::new(self.check_stmt(*s)?)),
            Stmt::Case(e, s) => Stmt::Case(self.rvalue(e)?, Box::new(self.check_stmt(*s)?)),
            Stmt::Default(s) => Stmt::Default(Box::new(self.check_stmt(*s)?)),
            Stmt::Goto(label) => Stmt::Goto(label),
            Stmt::Continue => Stmt::Continue,
            Stmt::Break => Stmt::Break,
        })
    }

    fn check_block_item(&mut self, item: BlockItem) -> Result<BlockItem, String> {
        match item {
            BlockItem::Decl(d) => Ok(BlockItem::Decl(self.check_decl(d)?)),
            BlockItem::Stmt(s) => Ok(BlockItem::Stmt(self.check_stmt(s)?)),
        }
    }

    fn check_decl(&mut self, d: Decl) -> Result<Decl, String> {
        self.register_struct_from_specs(&d.specs);
        let mut declarators = Vec::new();
        for mut id in d.declarators {
            let ty = resolve_type(&d.specs, &id.derived)?;
            self.define(id.name.clone(), ty);
            id.init = match id.init {
                Some(Init::Expr(e)) => Some(Init::Expr(self.rvalue(e)?)),
                other => other,
            };
            declarators.push(id);
        }
        Ok(Decl { specs: d.specs, is_typedef: d.is_typedef, declarators })
    }
}

pub fn check(unit: TranslationUnit) -> Result<TranslationUnit, String> {
    // Build global scope from top-level declarations and function definitions
    let mut global = HashMap::new();
    let mut structs = HashMap::<String, Vec<(String, CType)>>::new();
    // Process top-level declarations (globals, forward decls, struct defs)
    for d in &unit.decls {
        register_struct_fields(&d.specs, &mut structs);
        for id in &d.declarators {
            let ty = resolve_type(&d.specs, &id.derived)?;
            global.insert(id.name.clone(), ty);
        }
    }
    // Register all function definitions
    for f in &unit.functions {
        let ret = resolve_type(&f.return_specs, &f.return_derived)?;
        let params = f.params.iter()
            .map(|p| resolve_type(&p.specs, &p.derived))
            .collect::<Result<Vec<_>, _>>()?;
        global.insert(f.name.clone(), CType::Function {
            ret: Box::new(ret), params, variadic: false,
        });
    }
    // Type-check each function body
    let functions = unit.functions.into_iter().map(|f| {
        let ret = resolve_type(&f.return_specs, &f.return_derived)?;
        let mut tc = TypeChecker::new(ret);
        tc.structs = structs.clone();
        for (name, ty) in &global {
            tc.define(name.clone(), ty.clone());
        }
        tc.register_struct_from_specs(&f.return_specs);
        for p in &f.params {
            tc.register_struct_from_specs(&p.specs);
            let ty = resolve_type(&p.specs, &p.derived)?;
            if let Some(name) = &p.name {
                tc.define(name.clone(), ty);
            }
        }
        let body = tc.check_stmt(f.body)?;
        Ok(FunctionDef { body, ..f })
    }).collect::<Result<Vec<_>, String>>()?;
    Ok(TranslationUnit { decls: unit.decls, functions })
}

fn register_struct_fields(specs: &[DeclSpec], structs: &mut HashMap<String, Vec<(String, CType)>>) {
    for spec in specs {
        if let DeclSpec::Type(TypeSpec::Struct(_, ss)) = spec {
            if let Some(name) = &ss.name {
                if !ss.members.is_empty() {
                    let fields = ss.members.iter()
                        .flat_map(|m| m.declarators.iter().filter_map(|d| {
                            let ty = resolve_type(&m.specs, &d.derived).ok()?;
                            Some((d.name.clone()?, ty))
                        }))
                        .collect();
                    structs.insert(name.clone(), fields);
                }
            }
        }
    }
}

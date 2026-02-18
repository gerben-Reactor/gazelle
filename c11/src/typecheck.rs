use std::collections::{HashMap, HashSet};
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
    // No suffix: determine type from value (C11 6.4.4.1)
    let s_clean = lower.trim_end_matches(|c: char| c == 'u' || c == 'l');
    let val = if s_clean.starts_with("0x") {
        u64::from_str_radix(&s_clean[2..], 16).unwrap_or(0)
    } else if s_clean.starts_with('0') && s_clean.len() > 1 {
        u64::from_str_radix(&s_clean[1..], 8).unwrap_or(0)
    } else {
        s_clean.parse::<u64>().unwrap_or(0)
    };
    if is_hex {
        // Hex: int → unsigned int → long → unsigned long → long long → unsigned long long
        if val <= i32::MAX as u64 { CType::Int(Sign::Signed) }
        else if val <= u32::MAX as u64 { CType::Int(Sign::Unsigned) }
        else if val <= i64::MAX as u64 { CType::Long(Sign::Signed) }
        else { CType::Long(Sign::Unsigned) }
    } else {
        // Decimal: int → long → long long
        if val <= i32::MAX as u64 { CType::Int(Sign::Signed) }
        else if val <= i64::MAX as u64 { CType::Long(Sign::Signed) }
        else { CType::Long(Sign::Unsigned) }
    }
}

pub fn eval_const_i64(e: &ExprNode, enums: &HashMap<String, i64>) -> i64 {
    match e.expr.as_ref() {
        Expr::Constant(s) => {
            let s = s.to_ascii_lowercase();
            let s = s.trim_end_matches(|c: char| c == 'u' || c == 'l');
            if s.starts_with("0x") {
                i64::from_str_radix(&s[2..], 16).unwrap_or_else(|_| panic!("bad hex constant: {}", s))
            } else if s.starts_with('\'') {
                s.as_bytes().get(1).map(|&b| b as i64).unwrap_or_else(|| panic!("bad char constant: {}", s))
            } else {
                s.parse().unwrap_or_else(|_| panic!("bad integer constant: {}", s))
            }
        }
        Expr::Var(name) => *enums.get(name).unwrap_or_else(|| panic!("undefined constant '{}'", name)),
        Expr::UnaryOp(UnaryOp::Neg, a) => -eval_const_i64(a, enums),
        Expr::UnaryOp(UnaryOp::BitNot, a) => !eval_const_i64(a, enums),
        Expr::UnaryOp(UnaryOp::LogNot, a) => (eval_const_i64(a, enums) == 0) as i64,
        Expr::BinOp(op, a, b) => {
            let a = eval_const_i64(a, enums);
            let b = eval_const_i64(b, enums);
            match op {
                Op::Add => a + b,
                Op::Sub => a - b,
                Op::Mul => a * b,
                Op::Div => a / b,
                Op::Mod => a % b,
                Op::Shl => a << b,
                Op::Shr => a >> b,
                Op::BitAnd => a & b,
                Op::BitOr => a | b,
                Op::BitXor => a ^ b,
                Op::Lt => (a < b) as i64,
                Op::Gt => (a > b) as i64,
                Op::Le => (a <= b) as i64,
                Op::Ge => (a >= b) as i64,
                Op::Eq => (a == b) as i64,
                Op::Ne => (a != b) as i64,
                Op::And => ((a != 0) && (b != 0)) as i64,
                Op::Or => ((a != 0) || (b != 0)) as i64,
                _ => panic!("unsupported binary op in constant expression: {:?}", op),
            }
        }
        Expr::Ternary(cond, then, else_) => {
            if eval_const_i64(cond, enums) != 0 {
                eval_const_i64(then, enums)
            } else {
                eval_const_i64(else_, enums)
            }
        }
        Expr::Cast(_, inner) => eval_const_i64(inner, enums),
        Expr::SizeofType(ty) => {
            match ty {
                CType::Bool | CType::Char(_) => 1,
                CType::Short(_) => 2,
                CType::Int(_) | CType::Float | CType::Enum(_) => 4,
                CType::Long(_) | CType::LongLong(_) | CType::Double
                | CType::Pointer(_) | CType::LongDouble => 8,
                _ => panic!("unsupported sizeof in constant expression: {:?}", ty),
            }
        }
        other => panic!("unsupported constant expression {:?}", other),
    }
}

fn scalar_init_slots_free(ty: &CType, structs: &HashMap<String, Vec<(String, CType, Option<u8>)>>) -> u64 {
    match ty {
        CType::Struct(n) | CType::Union(n) => {
            structs.get(n).map_or(1, |fields| {
                fields.iter().map(|(_, fty, _)| scalar_init_slots_free(fty, structs)).sum()
            })
        }
        CType::Array(elem, Some(n)) => n * scalar_init_slots_free(elem, structs),
        _ => 1,
    }
}

/// Infer the size of an unsized array from its initializer list, accounting for designated indices.
/// `fields_per_elem` is the number of scalar init slots if the element is a struct (for brace elision), or 1.
fn infer_init_list_size(items: &[InitItem], fields_per_elem: u64) -> u64 {
    let mut max_idx: u64 = 0;
    let mut next_idx: u64 = 0;
    for item in items {
        let idx = if let Some(Designator::Index(e)) = item.designation.first() {
            eval_const_i64(e, &HashMap::new()) as u64
        } else {
            next_idx
        };
        next_idx = idx + 1;
        max_idx = max_idx.max(next_idx);
    }
    // Only apply brace elision division when all items are flat scalars (no sub-lists)
    let all_flat = fields_per_elem > 1 && items.iter().all(|i| matches!(i.init, Init::Expr(_)));
    if all_flat {
        (max_idx + fields_per_elem - 1) / fields_per_elem
    } else {
        max_idx
    }
}

/// Recursively collect enum constant (name, value) pairs from specs.
fn collect_enum_constants(specs: &[DeclSpec], enums: &mut HashMap<String, i64>) {
    for spec in specs {
        match spec {
            DeclSpec::Type(TypeSpec::Enum(es)) => {
                let mut next_val: i64 = 0;
                for en in &es.enumerators {
                    let val = match &en.value {
                        Some(e) => eval_const_i64(e, enums),
                        None => next_val,
                    };
                    enums.insert(en.name.clone(), val);
                    next_val = val + 1;
                }
            }
            DeclSpec::Type(TypeSpec::Struct(_, ss)) => {
                for m in &ss.members {
                    collect_enum_constants(&m.specs, enums);
                }
            }
            _ => {}
        }
    }
}

/// Resolve bare struct/union tag names in a type using a resolver function.
fn resolve_tags_in_type(ty: CType, resolve: &impl Fn(&str) -> String) -> CType {
    match ty {
        CType::Struct(ref name) => {
            let resolved = resolve(name);
            if resolved == *name { ty } else { CType::Struct(resolved) }
        }
        CType::Union(ref name) => {
            let resolved = resolve(name);
            if resolved == *name { ty } else { CType::Union(resolved) }
        }
        CType::Pointer(inner) => CType::Pointer(Box::new(resolve_tags_in_type(*inner, resolve))),
        CType::Array(inner, n) => CType::Array(Box::new(resolve_tags_in_type(*inner, resolve)), n),
        CType::Function { ret, params, variadic } => CType::Function {
            ret: Box::new(resolve_tags_in_type(*ret, resolve)),
            params: params.into_iter().map(|p| resolve_tags_in_type(p, resolve)).collect(),
            variadic,
        },
        _ => ty,
    }
}

/// Resolve typedefs and enums deeply using a flat name→type map.
fn resolve_typedef_deep(ty: CType, global: &HashMap<String, CType>) -> CType {
    match ty {
        CType::Typedef(ref name) => {
            if let Some(resolved) = global.get(name) {
                resolve_typedef_deep(resolved.clone(), global)
            } else {
                ty
            }
        }
        CType::Enum(_) => CType::Int(Sign::Signed),
        CType::Pointer(inner) => CType::Pointer(Box::new(resolve_typedef_deep(*inner, global))),
        CType::Array(inner, size) => CType::Array(Box::new(resolve_typedef_deep(*inner, global)), size),
        CType::Function { ret, params, variadic } => CType::Function {
            ret: Box::new(resolve_typedef_deep(*ret, global)),
            params: params.into_iter().map(|p| resolve_typedef_deep(p, global)).collect(),
            variadic,
        },
        _ => ty,
    }
}

struct TypeChecker {
    scopes: Vec<HashMap<String, CType>>,
    structs: HashMap<String, Vec<(String, CType, Option<u8>)>>,
    /// Scoped mapping from struct/union tag names to unique internal names.
    tag_scopes: Vec<HashMap<String, String>>,
    enum_constants: HashMap<String, i64>,
    unions: HashSet<String>,
    anon_count: usize,
    ret_type: CType,
}

impl TypeChecker {
    fn new(ret_type: CType) -> Self {
        Self { scopes: vec![HashMap::new()], structs: HashMap::new(), tag_scopes: vec![HashMap::new()], enum_constants: HashMap::new(), unions: HashSet::new(), anon_count: 0, ret_type }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.tag_scopes.push(HashMap::new());
    }
    fn pop_scope(&mut self) {
        self.scopes.pop();
        self.tag_scopes.pop();
    }

    /// Look up the unique internal name for a struct/union tag.
    fn resolve_tag(&self, tag: &str) -> String {
        for scope in self.tag_scopes.iter().rev() {
            if let Some(internal) = scope.get(tag) {
                return internal.clone();
            }
        }
        tag.to_string()
    }

    fn define(&mut self, name: String, ty: CType) {
        self.scopes.last_mut().unwrap().insert(name, ty);
    }

    /// Count the number of flat scalar init slots for a type (for brace elision).
    fn scalar_init_slots(&self, ty: &CType) -> u64 {
        match ty {
            CType::Struct(n) | CType::Union(n) => {
                self.structs.get(n).map_or(1, |fields| {
                    fields.iter().map(|(_, fty, _)| self.scalar_init_slots(fty)).sum()
                })
            }
            CType::Array(elem, Some(n)) => n * self.scalar_init_slots(elem),
            _ => 1,
        }
    }

    fn lookup(&self, name: &str) -> Result<CType, String> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Ok(ty.clone());
            }
        }
        Err(format!("undefined variable: {}", name))
    }

    /// Resolve `CType::Typedef(name)` and `CType::Enum` deeply.
    fn resolve_typedef(&self, ty: CType) -> CType {
        match ty {
            CType::Typedef(ref name) => {
                if let Ok(resolved) = self.lookup(name) {
                    self.resolve_typedef(resolved)
                } else {
                    ty
                }
            }
            CType::Enum(_) => CType::Int(Sign::Signed),
            CType::Pointer(inner) => CType::Pointer(Box::new(self.resolve_typedef(*inner))),
            CType::Array(inner, size) => CType::Array(Box::new(self.resolve_typedef(*inner)), size),
            CType::Function { ret, params, variadic } => CType::Function {
                ret: Box::new(self.resolve_typedef(*ret)),
                params: params.into_iter().map(|p| self.resolve_typedef(p)).collect(),
                variadic,
            },
            _ => ty,
        }
    }

    fn fresh_anon(&mut self) -> String {
        let n = self.anon_count;
        self.anon_count += 1;
        format!("__anon_{}", n)
    }

    fn register_struct_from_specs(&mut self, specs: &[DeclSpec]) {
        for spec in specs {
            if let DeclSpec::Type(TypeSpec::Struct(sou, ss)) = spec {
                if !ss.members.is_empty() {
                    let tag = ss.name.clone().unwrap_or_else(|| self.fresh_anon());
                    let internal = format!("{}#{}", tag, self.anon_count);
                    self.anon_count += 1;
                    self.tag_scopes.last_mut().unwrap().insert(tag.clone(), internal.clone());
                    let fields = collect_struct_fields(&ss.members, &mut self.structs, &mut self.unions, &mut self.anon_count, &self.enum_constants);
                    // Resolve bare tag names in field types to unique internal names
                    let fields = fields.into_iter().map(|(name, ty, bf)| {
                        (name, resolve_tags_in_type(ty, &|t| self.resolve_tag(t)), bf)
                    }).collect();
                    if matches!(sou, StructOrUnion::Union) {
                        self.unions.insert(internal.clone());
                    }
                    self.structs.insert(internal, fields);
                }
            }
        }
    }

    /// Resolve specs, assigning synthetic names to anonymous structs and resolving tags.
    fn resolve_specs_with_anon(&mut self, specs: &[DeclSpec]) -> Result<CType, String> {
        let ty = crate::types::resolve_specs(specs)?;
        match &ty {
            CType::Struct(name) if name.is_empty() => {
                for spec in specs {
                    if let DeclSpec::Type(TypeSpec::Struct(_, ss)) = spec {
                        if ss.name.is_none() && !ss.members.is_empty() {
                            let anon_name = self.fresh_anon();
                            let fields = collect_struct_fields(&ss.members, &mut self.structs, &mut self.unions, &mut self.anon_count, &self.enum_constants);
                            let fields = fields.into_iter().map(|(name, ty, bf)| {
                                (name, resolve_tags_in_type(ty, &|t| self.resolve_tag(t)), bf)
                            }).collect();
                            self.structs.insert(anon_name.clone(), fields);
                            return Ok(CType::Struct(anon_name));
                        }
                    }
                }
            }
            CType::Union(name) if name.is_empty() => {
                for spec in specs {
                    if let DeclSpec::Type(TypeSpec::Struct(StructOrUnion::Union, ss)) = spec {
                        if ss.name.is_none() && !ss.members.is_empty() {
                            let anon_name = self.fresh_anon();
                            let fields = collect_struct_fields(&ss.members, &mut self.structs, &mut self.unions, &mut self.anon_count, &self.enum_constants);
                            let fields = fields.into_iter().map(|(name, ty, bf)| {
                                (name, resolve_tags_in_type(ty, &|t| self.resolve_tag(t)), bf)
                            }).collect();
                            self.unions.insert(anon_name.clone());
                            self.structs.insert(anon_name.clone(), fields);
                            return Ok(CType::Union(anon_name));
                        }
                    }
                }
            }
            CType::Struct(name) => return Ok(CType::Struct(self.resolve_tag(name))),
            CType::Union(name) => return Ok(CType::Union(self.resolve_tag(name))),
            _ => {}
        }
        Ok(ty)
    }

    /// Resolve type from specs + derived, handling anonymous structs and typedefs.
    fn resolve_type_full(&mut self, specs: &[DeclSpec], derived: &[DerivedType]) -> Result<CType, String> {
        self.register_struct_from_specs(specs);
        let base = self.resolve_specs_with_anon(specs)?;
        let ty = crate::types::apply_derived(base, derived, &self.enum_constants)?;
        Ok(self.resolve_typedef(ty))
    }

    fn struct_name_of(&self, ty: &CType) -> Option<String> {
        match ty {
            CType::Struct(n) | CType::Union(n) => Some(n.clone()),
            CType::Typedef(name) => {
                if let Ok(resolved) = self.lookup(name) {
                    self.struct_name_of(&resolved)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn lookup_field(&self, struct_name: &str, field: &str) -> Result<CType, String> {
        let resolved = self.resolve_tag(struct_name);
        let fields = self.structs.get(&resolved)
            .or_else(|| self.structs.get(struct_name))
            .ok_or_else(|| format!("unknown struct: {}", struct_name))?;
        // Direct lookup
        if let Some((_, ty, _)) = fields.iter().find(|(n, _, _)| n == field) {
            return Ok(self.resolve_typedef(ty.clone()));
        }
        // Search through anonymous nested structs/unions
        for (n, ty, _) in fields {
            if let CType::Struct(inner) | CType::Union(inner) = ty {
                if n.starts_with("__anon_") {
                    if let Ok(ty) = self.lookup_field(inner, field) {
                        return Ok(ty);
                    }
                }
            }
        }
        Err(format!("no field '{}' in struct {}", field, struct_name))
    }

    /// Insert ImplicitCast if `e`'s type differs from `target`.
    fn coerce(e: ExprNode, target: &CType) -> ExprNode {
        if e.ty.as_ref().unwrap() == target {
            e
        } else {
            typed(Expr::ImplicitCast(target.clone(), e), target.clone())
        }
    }

    /// Default argument promotions (C11 6.5.2.2p6): float → double, char/short → int.
    fn default_promote(e: ExprNode) -> ExprNode {
        match e.ty.as_ref().unwrap() {
            CType::Float => Self::coerce(e, &CType::Double),
            CType::Bool | CType::Char(_) | CType::Short(_) => Self::coerce(e, &CType::Int(Sign::Signed)),
            _ => e,
        }
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
                if let Some(&val) = self.enum_constants.get(&name) {
                    (Expr::Constant(val.to_string()), CType::Int(Sign::Signed))
                } else {
                    let ty = self.lookup(&name)?;
                    (Expr::Var(name), ty)
                }
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
                let resolved_ty = self.resolve_typedef(func.ty.clone().unwrap());
                let (ret_ty, param_types) = match &resolved_ty {
                    CType::Pointer(inner) => match inner.as_ref() {
                        CType::Function { ret, params, .. } => (ret.as_ref().clone(), params.clone()),
                        _ => return Err("call on non-function pointer".into()),
                    },
                    _ => return Err(format!("call on non-function: {:?}", resolved_ty)),
                };
                let args = args.into_iter().enumerate()
                    .map(|(i, a)| {
                        let a = self.rvalue(a)?;
                        Ok::<_, String>(if i < param_types.len() {
                            Self::coerce(a, &param_types[i])
                        } else {
                            // Default argument promotions (C11 6.5.2.2p6):
                            // float → double, char/short → int
                            Self::default_promote(a)
                        })
                    })
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
                let struct_name = self.struct_name_of(obj.ty.as_ref().unwrap())
                    .ok_or("member access on non-struct")?;
                let field_ty = self.lookup_field(&struct_name, &field)?;
                (Expr::Member(obj, field), field_ty)
            }
            Expr::PtrMember(obj, field) => {
                let obj = self.rvalue(obj)?;
                let struct_name = match obj.ty.as_ref().unwrap() {
                    CType::Pointer(inner) => self.struct_name_of(inner)
                        .ok_or("-> on non-struct pointer")?,
                    _ => return Err("-> on non-pointer".into()),
                };
                let field_ty = self.lookup_field(&struct_name, &field)?;
                (Expr::PtrMember(obj, field), field_ty)
            }
            Expr::Ternary(cond, then, else_) => {
                let cond = self.rvalue(cond)?;
                let then = self.rvalue(then)?;
                let else_ = self.rvalue(else_)?;
                let lt = then.ty.as_ref().unwrap();
                let rt = else_.ty.as_ref().unwrap();
                let ty = if is_arith(lt) && is_arith(rt) {
                    usual_arith(lt, rt)
                } else {
                    lt.clone()
                };
                let then = Self::coerce(then, &ty);
                let else_ = Self::coerce(else_, &ty);
                (Expr::Ternary(cond, then, else_), ty)
            }
            Expr::Cast(cast_ty, inner) => {
                let inner = self.rvalue(inner)?;
                let ty = self.resolve_typedef(cast_ty.clone());
                (Expr::Cast(cast_ty, inner), ty)
            }
            Expr::SizeofExpr(inner) => {
                let inner = self.check_expr(inner)?;
                (Expr::SizeofExpr(inner), CType::Long(Sign::Unsigned))
            }
            Expr::SizeofType(ty) => {
                let resolved = self.resolve_typedef(ty);
                (Expr::SizeofType(resolved), CType::Long(Sign::Unsigned))
            }
            Expr::AlignofType(ty) => {
                let resolved = self.resolve_typedef(ty);
                (Expr::AlignofType(resolved), CType::Long(Sign::Unsigned))
            }
            Expr::Comma(l, r) => {
                let l = self.rvalue(l)?;
                let r = self.rvalue(r)?;
                let ty = r.ty.clone().unwrap();
                (Expr::Comma(l, r), ty)
            }
            Expr::CompoundLiteral(cl_ty, items) => {
                let ty = self.resolve_typedef(cl_ty.clone());
                (Expr::CompoundLiteral(cl_ty, items), ty)
            }
            Expr::VaArg(inner, va_ty) => {
                let inner = self.rvalue(inner)?;
                let ty = self.resolve_typedef(va_ty.clone());
                (Expr::VaArg(inner, va_ty), ty)
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
                // Keep compound assignment — codegen evaluates the lvalue once.
                // Apply usual arithmetic conversions to r so codegen gets the right type.
                let l = self.check_expr(l)?;
                let mut r = self.rvalue(r)?;
                let lty = l.ty.clone().unwrap();
                // For pointer += int, no conversion needed
                if !matches!(&lty, CType::Pointer(_)) {
                    let common = usual_arith(&lty, r.ty.as_ref().unwrap());
                    if *r.ty.as_ref().unwrap() != common {
                        r = typed(Expr::Widen(r), common);
                    }
                }
                Ok(typed(Expr::BinOp(op, l, r), lty))
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
        // Register enum constants (including from nested struct members)
        collect_enum_constants(&d.specs, &mut self.enum_constants);
        let names: Vec<_> = self.enum_constants.keys().cloned().collect();
        for name in names {
            self.define(name, CType::Int(Sign::Signed));
        }
        let mut declarators = Vec::new();
        for mut id in d.declarators {
            let mut ty = self.resolve_type_full(&d.specs, &id.derived)?;
            self.define(id.name.clone(), ty.clone());
            id.init = match id.init {
                Some(Init::Expr(e)) => {
                    let e = self.rvalue(e)?;
                    Some(Init::Expr(Self::coerce(e, &ty)))
                }
                Some(Init::List(items)) => {
                    let items = self.check_init_list(items)?;
                    // Infer unsized array size from initializer list
                    if let CType::Array(ref elem, None) = ty {
                        let fpe = self.scalar_init_slots(elem);
                        ty = CType::Array(elem.clone(), Some(infer_init_list_size(&items, fpe)));
                    }
                    Some(Init::List(items))
                }
                None => None,
            };
            id.ty = Some(ty);
            declarators.push(id);
        }
        Ok(Decl { specs: d.specs, is_typedef: d.is_typedef, declarators })
    }

    fn check_init_list(&mut self, items: Vec<InitItem>) -> Result<Vec<InitItem>, String> {
        items.into_iter().map(|item| {
            let init = match item.init {
                Init::Expr(e) => Init::Expr(self.rvalue(e)?),
                Init::List(sub) => Init::List(self.check_init_list(sub)?),
            };
            Ok(InitItem { designation: item.designation, init })
        }).collect()
    }
}

/// Collect struct fields, flattening anonymous nested struct/union members.
/// Registers any anonymous nested struct types into `structs`.
fn collect_struct_fields(
    members: &[StructMember],
    structs: &mut HashMap<String, Vec<(String, CType, Option<u8>)>>,
    unions: &mut HashSet<String>,
    anon_count: &mut usize,
    enums: &HashMap<String, i64>,
) -> Vec<(String, CType, Option<u8>)> {
    let mut fields = Vec::new();
    for m in members {
        // Register any named nested struct/union definitions in member specs
        register_struct_fields(&m.specs, structs, unions, anon_count, enums);
        if m.declarators.is_empty() {
            // Anonymous nested struct/union — keep as single field for correct layout
            for spec in &m.specs {
                if let DeclSpec::Type(TypeSpec::Struct(sou, ss)) = spec {
                    if !ss.members.is_empty() {
                        let nested = collect_struct_fields(&ss.members, structs, unions, anon_count, enums);
                        let anon_name = format!("__anon_{}", *anon_count);
                        *anon_count += 1;
                        if matches!(sou, StructOrUnion::Union) {
                            unions.insert(anon_name.clone());
                        }
                        structs.insert(anon_name.clone(), nested);
                        let ty = match sou {
                            StructOrUnion::Struct => CType::Struct(anon_name.clone()),
                            StructOrUnion::Union => CType::Union(anon_name.clone()),
                        };
                        fields.push((anon_name, ty, None));
                    } else if let Some(name) = &ss.name {
                        if let Some(f) = structs.get(name).cloned() {
                            // Named anonymous member — flatten its fields
                            fields.extend(f);
                        }
                    }
                }
            }
        } else {
            for d in &m.declarators {
                if let Some(mut ty) = crate::types::resolve_type(&m.specs, &d.derived, enums).ok() {
                    // Register anonymous struct types used as named fields
                    if let CType::Struct(ref name) | CType::Union(ref name) = ty {
                        if name.is_empty() {
                            for spec in &m.specs {
                                if let DeclSpec::Type(TypeSpec::Struct(sou, ss)) = spec {
                                    if ss.name.is_none() && !ss.members.is_empty() {
                                        let anon_name = format!("__anon_{}", *anon_count);
                                        *anon_count += 1;
                                        let nested = collect_struct_fields(&ss.members, structs, unions, anon_count, enums);
                                        if matches!(sou, StructOrUnion::Union) {
                                            unions.insert(anon_name.clone());
                                        }
                                        structs.insert(anon_name.clone(), nested);
                                        ty = match sou {
                                            StructOrUnion::Struct => CType::Struct(anon_name),
                                            StructOrUnion::Union => CType::Union(anon_name),
                                        };
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    // Extract bitfield width
                    let bf_width = d.bitfield.as_ref().map(|e| {
                        eval_const_i64(e, enums) as u8
                    });
                    // Enum bitfields: treat as unsigned int (GCC-compatible behavior)
                    if bf_width.is_some() {
                        if let CType::Enum(_) = &ty {
                            ty = CType::Int(Sign::Unsigned);
                        }
                    }
                    if let Some(name) = &d.name {
                        fields.push((name.clone(), ty, bf_width));
                    }
                }
            }
        }
    }
    fields
}

pub fn check(unit: TranslationUnit) -> Result<TranslationUnit, String> {
    let mut global: HashMap<String, CType> = HashMap::new();
    let mut structs = HashMap::<String, Vec<(String, CType, Option<u8>)>>::new();
    let mut unions = HashSet::<String>::new();
    let mut enum_constants = HashMap::<String, i64>::new();
    let mut anon_count = 0usize;

    // Define __builtin_va_list as a struct (x86-64 ABI: 24 bytes)
    structs.insert("__va_list_tag".into(), vec![
        ("gp_offset".into(), CType::Int(Sign::Unsigned), None),
        ("fp_offset".into(), CType::Int(Sign::Unsigned), None),
        ("overflow_arg_area".into(), CType::Pointer(Box::new(CType::Void)), None),
        ("reg_save_area".into(), CType::Pointer(Box::new(CType::Void)), None),
    ]);
    // __builtin_va_list is __va_list_tag[1]
    global.insert("__builtin_va_list".into(),
        CType::Array(Box::new(CType::Struct("__va_list_tag".into())), Some(1)));

    // Register GCC builtin functions
    global.insert("__builtin_bswap16".into(), CType::Function {
        ret: Box::new(CType::Short(Sign::Unsigned)), params: vec![CType::Short(Sign::Unsigned)], variadic: false,
    });
    global.insert("__builtin_bswap32".into(), CType::Function {
        ret: Box::new(CType::Int(Sign::Unsigned)), params: vec![CType::Int(Sign::Unsigned)], variadic: false,
    });
    global.insert("__builtin_bswap64".into(), CType::Function {
        ret: Box::new(CType::Long(Sign::Unsigned)), params: vec![CType::Long(Sign::Unsigned)], variadic: false,
    });
    // va_start/va_end: take va_list pointer + last named param (treated as variadic)
    let va_list_ptr = CType::Pointer(Box::new(CType::Void));
    global.insert("__builtin_va_start".into(), CType::Function {
        ret: Box::new(CType::Void), params: vec![va_list_ptr.clone()], variadic: true,
    });
    global.insert("__builtin_va_end".into(), CType::Function {
        ret: Box::new(CType::Void), params: vec![va_list_ptr], variadic: false,
    });

    // Pre-register function definitions so global init expressions can reference them
    for f in &unit.functions {
        if let Ok(ret) = resolve_type(&f.return_specs, &f.return_derived, &enum_constants) {
            let params = f.params.iter()
                .filter_map(|p| resolve_type(&p.specs, &p.derived, &enum_constants).ok())
                .collect::<Vec<_>>();
            let variadic = f.params.last().map_or(false, |p|
                p.specs.is_empty() && p.name.is_none() && p.derived.is_empty());
            global.insert(f.name.clone(), CType::Function {
                ret: Box::new(ret), params, variadic,
            });
        }
    }

    // Process top-level declarations, setting id.ty on each declarator
    let decls = unit.decls.into_iter().map(|mut d| {
        // Register enum constants first so struct field array sizes can reference them
        collect_enum_constants(&d.specs, &mut enum_constants);
        register_struct_fields(&d.specs, &mut structs, &mut unions, &mut anon_count, &enum_constants);
        for (name, _) in &enum_constants {
            global.entry(name.clone()).or_insert(CType::Int(Sign::Signed));
        }
        // Create a TypeChecker for global scope to type-check init expressions
        let mut tc = TypeChecker::new(CType::Void);
        for (name, cty) in &global { tc.define(name.clone(), cty.clone()); }
        for (name, val) in &enum_constants { tc.enum_constants.insert(name.clone(), *val); }
        tc.structs = structs.clone();
        tc.unions = unions.clone();
        tc.anon_count = anon_count;

        for id in &mut d.declarators {
            // Resolve base type from specs, handling anonymous structs
            let mut base_ty = crate::types::resolve_specs(&d.specs)?;
            if let CType::Struct(ref name) | CType::Union(ref name) = base_ty {
                if name.is_empty() {
                    for spec in &d.specs {
                        if let DeclSpec::Type(TypeSpec::Struct(sou, ss)) = spec {
                            if ss.name.is_none() && !ss.members.is_empty() {
                                let anon_name = format!("__anon_{}", anon_count);
                                anon_count += 1;
                                let fields = collect_struct_fields(&ss.members, &mut structs, &mut unions, &mut anon_count, &enum_constants);
                                if matches!(sou, StructOrUnion::Union) {
                                    unions.insert(anon_name.clone());
                                }
                                structs.insert(anon_name.clone(), fields);
                                base_ty = match sou {
                                    StructOrUnion::Struct => CType::Struct(anon_name),
                                    StructOrUnion::Union => CType::Union(anon_name),
                                };
                                break;
                            }
                        }
                    }
                }
            }
            let mut ty = crate::types::apply_derived(base_ty, &id.derived, &enum_constants)?;
            ty = resolve_typedef_deep(ty, &global);
            // Type-check initializer expressions
            id.init = match id.init.take() {
                Some(Init::Expr(e)) => {
                    let e = tc.rvalue(e)?;
                    Some(Init::Expr(TypeChecker::coerce(e, &ty)))
                }
                Some(Init::List(items)) => {
                    let items = tc.check_init_list(items)?;
                    // Infer unsized array size from initializer list
                    if let CType::Array(ref elem, None) = ty {
                        let fpe = scalar_init_slots_free(elem, &structs);
                        ty = CType::Array(elem.clone(), Some(infer_init_list_size(&items, fpe)));
                    }
                    Some(Init::List(items))
                }
                None => None,
            };
            id.ty = Some(ty.clone());
            global.insert(id.name.clone(), ty.clone());
            tc.define(id.name.clone(), ty);
        }
        anon_count = anon_count.max(tc.anon_count);
        Ok(d)
    }).collect::<Result<Vec<_>, String>>()?;

    // Register all function definitions
    for f in &unit.functions {
        let ret = resolve_typedef_deep(resolve_type(&f.return_specs, &f.return_derived, &enum_constants)?, &global);
        let params = f.params.iter()
            .map(|p| Ok(resolve_typedef_deep(resolve_type(&p.specs, &p.derived, &enum_constants)?, &global)))
            .collect::<Result<Vec<_>, String>>()?;
        let variadic = f.params.last().map_or(false, |p|
            p.specs.is_empty() && p.name.is_none() && p.derived.is_empty());
        global.insert(f.name.clone(), CType::Function {
            ret: Box::new(ret), params, variadic,
        });
    }

    // Type-check each function body, setting p.ty on each parameter
    let functions = unit.functions.into_iter().map(|mut f| {
        let ret_raw = resolve_type(&f.return_specs, &f.return_derived, &enum_constants)?;
        let mut tc = TypeChecker::new(resolve_typedef_deep(ret_raw, &global));
        tc.structs = structs.clone();
        tc.unions = unions.clone();
        tc.enum_constants = enum_constants.clone();
        tc.anon_count = anon_count;
        for (name, ty) in &global {
            tc.define(name.clone(), ty.clone());
        }
        tc.register_struct_from_specs(&f.return_specs);
        for p in &mut f.params {
            tc.register_struct_from_specs(&p.specs);
            let mut ty = tc.resolve_type_full(&p.specs, &p.derived)?;
            // C11 6.7.6.3p7: array parameters decay to pointers
            if let CType::Array(elem, _) = ty {
                ty = CType::Pointer(elem);
            }
            p.ty = Some(ty.clone());
            if let Some(name) = &p.name {
                tc.define(name.clone(), ty);
            }
        }
        let body = tc.check_stmt(f.body)?;
        // Merge any struct defs discovered inside this function back
        structs.extend(tc.structs.into_iter());
        unions.extend(tc.unions.into_iter());
        Ok(FunctionDef { body, ..f })
    }).collect::<Result<Vec<_>, String>>()?;

    // Build (is_union, fields) map for TranslationUnit, resolving typedefs in field types
    let struct_map = structs.into_iter()
        .map(|(name, fields)| {
            let is_union = unions.contains(&name);
            let fields = fields.into_iter()
                .map(|(fname, fty, bf)| (fname, resolve_typedef_deep(fty, &global), bf))
                .collect();
            (name, (is_union, fields))
        })
        .collect();

    Ok(TranslationUnit { decls, functions, structs: struct_map })
}

fn register_struct_fields(
    specs: &[DeclSpec],
    structs: &mut HashMap<String, Vec<(String, CType, Option<u8>)>>,
    unions: &mut HashSet<String>,
    anon_count: &mut usize,
    enums: &HashMap<String, i64>,
) {
    for spec in specs {
        if let DeclSpec::Type(TypeSpec::Struct(sou, ss)) = spec {
            let name = if let Some(n) = &ss.name {
                n.clone()
            } else if !ss.members.is_empty() {
                let n = format!("__anon_{}", *anon_count);
                *anon_count += 1;
                n
            } else {
                continue;
            };
            if !ss.members.is_empty() {
                let fields = collect_struct_fields(&ss.members, structs, unions, anon_count, enums);
                if matches!(sou, StructOrUnion::Union) {
                    unions.insert(name.clone());
                }
                structs.insert(name, fields);
            }
        }
    }
}

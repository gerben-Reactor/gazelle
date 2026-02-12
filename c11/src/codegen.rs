use std::alloc::Layout;
use std::collections::{HashMap, HashSet};
use crate::ast::*;
use crate::types::resolve_type;

/// Computed struct layout: field name → (byte offset, field type).
struct StructLayout {
    fields: Vec<(String, i32, CType)>,
    size: i32,
    align: i32,
}

struct Codegen {
    out: String,
    label_count: usize,
    locals: HashMap<String, i32>,
    globals: HashSet<String>,
    stack_size: i32,
    strings: Vec<String>,
    func_name: String,
    break_label: Option<String>,
    continue_label: Option<String>,
    struct_layouts: HashMap<String, StructLayout>,
}

impl Codegen {
    fn new() -> Self {
        Self {
            out: String::new(),
            label_count: 0,
            locals: HashMap::new(),
            globals: HashSet::new(),
            stack_size: 0,
            strings: Vec::new(),
            func_name: String::new(),
            break_label: None,
            continue_label: None,
            struct_layouts: HashMap::new(),
        }
    }

    fn emit(&mut self, s: &str) {
        self.out.push('\t');
        self.out.push_str(s);
        self.out.push('\n');
    }

    fn label(&mut self, s: &str) {
        self.out.push_str(s);
        self.out.push_str(":\n");
    }

    fn fresh(&mut self, prefix: &str) -> String {
        let n = self.label_count;
        self.label_count += 1;
        format!(".L{}_{}", prefix, n)
    }

    // === Type helpers ===

    fn type_size(&self, ty: &CType) -> i32 {
        match ty {
            CType::Void => 0,
            CType::Bool | CType::Char(_) => 1,
            CType::Short(_) => 2,
            CType::Int(_) | CType::Float | CType::Enum(_) => 4,
            CType::Long(_) | CType::LongLong(_) | CType::Double
            | CType::Pointer(_) | CType::LongDouble => 8,
            CType::Array(elem, Some(n)) => self.type_size(elem) * *n as i32,
            CType::Array(_, None) => 8,
            CType::Function { .. } => 0,
            CType::Struct(name) | CType::Union(name) => {
                self.struct_layouts.get(name).map_or(0, |l| l.size)
            }
            _ => 8,
        }
    }

    fn field_offset(&self, struct_name: &str, field: &str) -> (i32, CType) {
        self.find_field_offset(struct_name, field)
            .unwrap_or((0, CType::Int(Sign::Signed)))
    }

    fn find_field_offset(&self, struct_name: &str, field: &str) -> Option<(i32, CType)> {
        let layout = self.struct_layouts.get(struct_name)?;
        // Direct lookup
        for (name, offset, ty) in &layout.fields {
            if name == field {
                return Some((*offset, ty.clone()));
            }
        }
        // Search through anonymous nested structs/unions
        for (name, offset, ty) in &layout.fields {
            if let CType::Struct(inner) | CType::Union(inner) = ty {
                if name.starts_with("__anon_") {
                    if let Some((inner_offset, inner_ty)) = self.find_field_offset(inner, field) {
                        return Some((offset + inner_offset, inner_ty));
                    }
                }
            }
        }
        None
    }

    fn is_float(ty: &CType) -> bool {
        matches!(ty, CType::Float | CType::Double | CType::LongDouble)
    }

    fn is_unsigned(ty: &CType) -> bool {
        matches!(ty, CType::Char(Sign::Unsigned) | CType::Short(Sign::Unsigned)
            | CType::Int(Sign::Unsigned) | CType::Long(Sign::Unsigned)
            | CType::LongLong(Sign::Unsigned) | CType::Bool | CType::Pointer(_))
    }

    // === Stack allocation ===

    fn alloc_locals(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Compound(items) => {
                for item in items {
                    match item {
                        BlockItem::Decl(d) => {
                            for id in &d.declarators {
                                if d.is_typedef { continue; }
                                let ty = id.ty.as_ref().unwrap();
                                let size = self.type_size(ty).max(8);
                                self.stack_size += size;
                                self.locals.insert(id.name.clone(), -self.stack_size);
                            }
                        }
                        BlockItem::Stmt(s) => self.alloc_locals(s),
                    }
                }
            }
            Stmt::If(_, then, else_) => {
                self.alloc_locals(then);
                if let Some(e) = else_ { self.alloc_locals(e); }
            }
            Stmt::While(_, body) | Stmt::DoWhile(body, _) | Stmt::Switch(_, body) => {
                self.alloc_locals(body);
            }
            Stmt::For(init, _, _, body) => {
                if let ForInit::Decl(d) = init {
                    for id in &d.declarators {
                        let ty = id.ty.as_ref().unwrap();
                        let size = self.type_size(ty).max(8);
                        self.stack_size += size;
                        self.locals.insert(id.name.clone(), -self.stack_size);
                    }
                }
                self.alloc_locals(body);
            }
            Stmt::Labeled(_, s) | Stmt::Case(_, s) | Stmt::Default(s) => {
                self.alloc_locals(s);
            }
            _ => {}
        }
    }

    // === Load/Store helpers ===

    /// Emit a load from (%rax) into %rax or %xmm0, sized by `ty`.
    fn emit_load(&mut self, ty: &CType) {
        match ty {
            CType::Bool | CType::Char(Sign::Unsigned) => self.emit("movzbl (%rax), %eax"),
            CType::Char(Sign::Signed) => self.emit("movsbl (%rax), %eax"),
            CType::Short(Sign::Unsigned) => self.emit("movzwl (%rax), %eax"),
            CType::Short(Sign::Signed) => self.emit("movswl (%rax), %eax"),
            CType::Int(_) | CType::Enum(_) => self.emit("movl (%rax), %eax"),
            CType::Float => self.emit("movss (%rax), %xmm0"),
            CType::Double | CType::LongDouble => self.emit("movsd (%rax), %xmm0"),
            _ => self.emit("movq (%rax), %rax"), // Long, LongLong, Pointer
        }
    }

    /// Emit a store from %rcx/%xmm0 to (%rax), sized by `ty`.
    fn emit_store(&mut self, ty: &CType) {
        match ty {
            CType::Bool | CType::Char(_) => self.emit("movb %cl, (%rax)"),
            CType::Short(_) => self.emit("movw %cx, (%rax)"),
            CType::Int(_) | CType::Enum(_) => self.emit("movl %ecx, (%rax)"),
            CType::Float => self.emit("movss %xmm0, (%rax)"),
            CType::Double | CType::LongDouble => self.emit("movsd %xmm0, (%rax)"),
            _ => self.emit("movq %rcx, (%rax)"),
        }
    }

    fn push_int(&mut self) {
        self.emit("pushq %rax");
    }

    fn pop_int(&mut self, reg: &str) {
        self.emit(&format!("popq {}", reg));
    }

    fn push_float(&mut self) {
        self.emit("subq $8, %rsp");
        self.emit("movsd %xmm0, (%rsp)");
    }

    fn pop_float(&mut self, xmm: &str) {
        self.emit(&format!("movsd (%rsp), {}", xmm));
        self.emit("addq $8, %rsp");
    }

    // === Expression codegen ===

    fn emit_expr(&mut self, e: &ExprNode) {
        let ty = e.ty.as_ref().unwrap();
        match e.expr.as_ref() {
            Expr::Constant(s) => self.emit_constant(s, ty),
            Expr::StringLit(s) => {
                let idx = self.strings.len();
                self.strings.push(s.clone());
                self.emit(&format!("leaq .Lstr_{}(%rip), %rax", idx));
            }
            Expr::Var(name) => {
                if let Some(&offset) = self.locals.get(name) {
                    self.emit(&format!("leaq {}(%rbp), %rax", offset));
                } else {
                    self.emit(&format!("leaq {}(%rip), %rax", name));
                }
            }
            Expr::Load(inner) => {
                self.emit_expr(inner);
                self.emit_load(ty);
            }
            Expr::Decay(inner) | Expr::FuncToPtr(inner) => {
                self.emit_expr(inner);
            }
            Expr::Widen(inner) => {
                self.emit_expr(inner);
                let from = inner.ty.as_ref().unwrap();
                self.emit_widen(from, ty);
            }
            Expr::ImplicitCast(target, inner) => {
                self.emit_expr(inner);
                let from = inner.ty.as_ref().unwrap();
                self.emit_cast(from, target);
            }
            Expr::Cast(_tn, inner) => {
                self.emit_expr(inner);
                let from = inner.ty.as_ref().unwrap();
                self.emit_cast(from, ty);
            }
            Expr::BinOp(op, l, r) => self.emit_binop(*op, l, r, ty),
            Expr::UnaryOp(op, inner) => self.emit_unaryop(*op, inner, ty),
            Expr::Call(func, args) => self.emit_call(func, args, ty),
            Expr::Index(arr, idx) => {
                // Result is an address (lvalue). elem_ty = ty, need elem size.
                let elem_size = self.type_size(ty);
                self.emit_expr(idx);
                if elem_size != 1 {
                    self.emit(&format!("imulq ${}, %rax", elem_size));
                }
                self.push_int();
                self.emit_expr(arr);
                self.pop_int("%rcx");
                self.emit("addq %rcx, %rax");
            }
            Expr::Ternary(cond, then, else_) => {
                let lelse = self.fresh("else");
                let lend = self.fresh("end");
                self.emit_expr(cond);
                self.emit_test_zero(cond.ty.as_ref().unwrap());
                self.emit(&format!("je {}", lelse));
                self.emit_expr(then);
                self.emit(&format!("jmp {}", lend));
                self.label(&lelse);
                self.emit_expr(else_);
                self.label(&lend);
            }
            Expr::Comma(l, r) => {
                self.emit_expr(l);
                self.emit_expr(r);
            }
            Expr::SizeofExpr(inner) => {
                let sz = self.type_size(inner.ty.as_ref().unwrap());
                self.emit(&format!("movq ${}, %rax", sz));
            }
            Expr::SizeofType(tn) => {
                let t = resolve_type(&tn.specs, &tn.derived).unwrap_or(CType::Int(Sign::Signed));
                let sz = self.type_size(&t);
                self.emit(&format!("movq ${}, %rax", sz));
            }
            Expr::Member(obj, field) => {
                // obj is an lvalue (struct address)
                self.emit_expr(obj);
                let struct_name = match obj.ty.as_ref().unwrap() {
                    CType::Struct(n) | CType::Union(n) => n.clone(),
                    _ => String::new(),
                };
                let (offset, _) = self.field_offset(&struct_name, field);
                if offset != 0 {
                    self.emit(&format!("addq ${}, %rax", offset));
                }
            }
            Expr::PtrMember(obj, field) => {
                // obj is a pointer to struct (rvalue)
                self.emit_expr(obj);
                let struct_name = match obj.ty.as_ref().unwrap() {
                    CType::Pointer(inner) => match inner.as_ref() {
                        CType::Struct(n) | CType::Union(n) => n.clone(),
                        _ => String::new(),
                    },
                    _ => String::new(),
                };
                let (offset, _) = self.field_offset(&struct_name, field);
                if offset != 0 {
                    self.emit(&format!("addq ${}, %rax", offset));
                }
            }
            _ => {
                // Fallback for unhandled expr variants
            }
        }
    }

    fn emit_constant(&mut self, s: &str, ty: &CType) {
        if Self::is_float(ty) {
            // Float/double constant: emit as integer bits moved to xmm
            let s_clean = s.trim_end_matches(|c: char| c == 'f' || c == 'F' || c == 'l' || c == 'L');
            if matches!(ty, CType::Float) {
                let val: f32 = s_clean.parse().unwrap_or(0.0);
                let bits = val.to_bits();
                self.emit(&format!("movl ${}, %eax", bits as i32));
                self.emit("movd %eax, %xmm0");
            } else {
                let val: f64 = s_clean.parse().unwrap_or(0.0);
                let bits = val.to_bits();
                self.emit(&format!("movabsq ${}, %rax", bits as i64));
                self.emit("movq %rax, %xmm0");
            }
            return;
        }
        if s.starts_with('\'') {
            // Character literal
            let val = parse_char_literal(s);
            self.emit(&format!("movl ${}, %eax", val));
            return;
        }
        // Integer constant
        let val = parse_int_constant(s);
        if val as u64 <= i32::MAX as u64 {
            self.emit(&format!("movl ${}, %eax", val as i32));
        } else {
            self.emit(&format!("movabsq ${}, %rax", val));
        }
    }

    fn emit_test_zero(&mut self, ty: &CType) {
        if Self::is_float(ty) {
            self.emit("xorpd %xmm1, %xmm1");
            self.emit("ucomisd %xmm1, %xmm0");
            // Set rax = (xmm0 != 0) using setnp+setne trick
            self.emit("movl $0, %eax");
            self.emit("setne %al");
            // Now test %rax for zero
            self.emit("testl %eax, %eax");
        } else {
            self.emit("testq %rax, %rax");
        }
    }

    fn emit_widen(&mut self, from: &CType, to: &CType) {
        // from → to widening conversion
        match (from, to) {
            (_, _) if from == to => {}
            // Small int → Int: already sign/zero-extended from Load
            (CType::Bool | CType::Char(_) | CType::Short(_), CType::Int(_)) => {}
            // Int → Long/LongLong
            (CType::Int(Sign::Signed), CType::Long(_) | CType::LongLong(_)) => {
                self.emit("movslq %eax, %rax");
            }
            (CType::Int(Sign::Unsigned), CType::Long(_) | CType::LongLong(_)) => {
                // movl %eax, %eax zero-extends to 64 bits
                self.emit("movl %eax, %eax");
            }
            // Int → Double
            (CType::Int(_), CType::Double | CType::LongDouble) => {
                self.emit("cvtsi2sdl %eax, %xmm0");
            }
            // Int → Float
            (CType::Int(_), CType::Float) => {
                self.emit("cvtsi2ssl %eax, %xmm0");
            }
            // Long → Double
            (CType::Long(_) | CType::LongLong(_), CType::Double | CType::LongDouble) => {
                self.emit("cvtsi2sdq %rax, %xmm0");
            }
            // Long → Float
            (CType::Long(_) | CType::LongLong(_), CType::Float) => {
                self.emit("cvtsi2ssq %rax, %xmm0");
            }
            // Float → Double
            (CType::Float, CType::Double | CType::LongDouble) => {
                self.emit("cvtss2sd %xmm0, %xmm0");
            }
            // Small int → Long
            (CType::Bool | CType::Char(_) | CType::Short(_), CType::Long(_) | CType::LongLong(_)) => {
                self.emit("movslq %eax, %rax");
            }
            // Small int → Double
            (CType::Bool | CType::Char(_) | CType::Short(_), CType::Double | CType::LongDouble) => {
                self.emit("cvtsi2sdl %eax, %xmm0");
            }
            // Small int → Float
            (CType::Bool | CType::Char(_) | CType::Short(_), CType::Float) => {
                self.emit("cvtsi2ssl %eax, %xmm0");
            }
            _ => {} // Same size or unsupported, no-op
        }
    }

    fn emit_cast(&mut self, from: &CType, to: &CType) {
        if from == to { return; }
        match (from, to) {
            // Float/Double → Int
            (CType::Float, CType::Int(_) | CType::Long(_) | CType::LongLong(_)
                | CType::Char(_) | CType::Short(_) | CType::Bool) => {
                self.emit("cvttss2sil %xmm0, %eax");
            }
            (CType::Double | CType::LongDouble, CType::Int(_) | CType::Char(_)
                | CType::Short(_) | CType::Bool) => {
                self.emit("cvttsd2sil %xmm0, %eax");
            }
            (CType::Double | CType::LongDouble, CType::Long(_) | CType::LongLong(_)) => {
                self.emit("cvttsd2siq %xmm0, %rax");
            }
            // Double → Float
            (CType::Double | CType::LongDouble, CType::Float) => {
                self.emit("cvtsd2ss %xmm0, %xmm0");
            }
            _ => self.emit_widen(from, to),
        }
    }

    // === Binary operations ===

    fn emit_binop(&mut self, op: Op, l: &ExprNode, r: &ExprNode, result_ty: &CType) {
        match op {
            Op::Assign => self.emit_assign(l, r),
            Op::AddAssign | Op::SubAssign | Op::MulAssign | Op::DivAssign | Op::ModAssign
            | Op::ShlAssign | Op::ShrAssign | Op::BitAndAssign | Op::BitOrAssign | Op::BitXorAssign => {
                self.emit_compound_assign(op, l, r);
            }
            Op::And => self.emit_logical_and(l, r),
            Op::Or => self.emit_logical_or(l, r),
            _ if Self::is_float(result_ty) => self.emit_float_binop(op, l, r, result_ty),
            _ => self.emit_int_binop(op, l, r, result_ty),
        }
    }

    fn emit_assign(&mut self, l: &ExprNode, r: &ExprNode) {
        let lty = l.ty.as_ref().unwrap();
        if Self::is_float(r.ty.as_ref().unwrap()) {
            self.emit_expr(r);
            self.push_float();
            self.emit_expr(l);
            self.pop_float("%xmm0");
            self.emit_store(lty);
        } else {
            self.emit_expr(r);
            self.push_int();
            self.emit_expr(l);
            self.pop_int("%rcx");
            self.emit_store(lty);
            self.emit("movq %rcx, %rax");
        }
    }

    fn emit_compound_assign(&mut self, op: Op, l: &ExprNode, r: &ExprNode) {
        let lty = l.ty.as_ref().unwrap();
        let is_ptr = matches!(lty, CType::Pointer(_));
        // Compute the address of l
        self.emit_expr(l);
        self.push_int(); // save address
        // Load current value
        self.emit_load(lty);
        // Save current value
        if Self::is_float(lty) {
            self.push_float();
        } else {
            self.push_int();
        }
        // Evaluate r
        self.emit_expr(r);
        // Now: r in rax/xmm0, l_value on stack, l_addr below that
        if Self::is_float(lty) {
            self.emit("movapd %xmm0, %xmm1"); // r in xmm1
            self.pop_float("%xmm0"); // l_value in xmm0
            let suffix = if matches!(lty, CType::Float) { "ss" } else { "sd" };
            match op {
                Op::AddAssign => self.emit(&format!("add{} %xmm1, %xmm0", suffix)),
                Op::SubAssign => self.emit(&format!("sub{} %xmm1, %xmm0", suffix)),
                Op::MulAssign => self.emit(&format!("mul{} %xmm1, %xmm0", suffix)),
                Op::DivAssign => self.emit(&format!("div{} %xmm1, %xmm0", suffix)),
                _ => {}
            }
            self.pop_int("%rax"); // address
            self.emit_store(lty);
        } else {
            self.emit("movq %rax, %rcx"); // r in rcx
            self.pop_int("%rax"); // l_value in rax
            if is_ptr {
                if let CType::Pointer(inner) = lty {
                    let elem_size = self.type_size(inner);
                    if elem_size != 1 {
                        self.emit(&format!("imulq ${}, %rcx", elem_size));
                    }
                }
            }
            match op {
                Op::AddAssign => self.emit("addq %rcx, %rax"),
                Op::SubAssign => self.emit("subq %rcx, %rax"),
                Op::MulAssign => self.emit("imulq %rcx, %rax"),
                Op::DivAssign => {
                    self.emit("cqo");
                    self.emit("idivq %rcx");
                }
                Op::ModAssign => {
                    self.emit("cqo");
                    self.emit("idivq %rcx");
                    self.emit("movq %rdx, %rax");
                }
                Op::ShlAssign => self.emit("shlq %cl, %rax"),
                Op::ShrAssign => {
                    if Self::is_unsigned(lty) {
                        self.emit("shrq %cl, %rax");
                    } else {
                        self.emit("sarq %cl, %rax");
                    }
                }
                Op::BitAndAssign => self.emit("andq %rcx, %rax"),
                Op::BitOrAssign => self.emit("orq %rcx, %rax"),
                Op::BitXorAssign => self.emit("xorq %rcx, %rax"),
                _ => {}
            }
            self.emit("movq %rax, %rcx"); // result in rcx
            self.pop_int("%rax"); // address
            self.emit_store(lty);
            self.emit("movq %rcx, %rax"); // result in rax
        }
    }

    fn emit_logical_and(&mut self, l: &ExprNode, r: &ExprNode) {
        let lfalse = self.fresh("and_false");
        let lend = self.fresh("and_end");
        self.emit_expr(l);
        self.emit_test_zero(l.ty.as_ref().unwrap());
        self.emit(&format!("je {}", lfalse));
        self.emit_expr(r);
        self.emit_test_zero(r.ty.as_ref().unwrap());
        self.emit(&format!("je {}", lfalse));
        self.emit("movl $1, %eax");
        self.emit(&format!("jmp {}", lend));
        self.label(&lfalse);
        self.emit("xorl %eax, %eax");
        self.label(&lend);
    }

    fn emit_logical_or(&mut self, l: &ExprNode, r: &ExprNode) {
        let ltrue = self.fresh("or_true");
        let lend = self.fresh("or_end");
        self.emit_expr(l);
        self.emit_test_zero(l.ty.as_ref().unwrap());
        self.emit(&format!("jne {}", ltrue));
        self.emit_expr(r);
        self.emit_test_zero(r.ty.as_ref().unwrap());
        self.emit(&format!("jne {}", ltrue));
        self.emit("xorl %eax, %eax");
        self.emit(&format!("jmp {}", lend));
        self.label(&ltrue);
        self.emit("movl $1, %eax");
        self.label(&lend);
    }

    fn emit_int_binop(&mut self, op: Op, l: &ExprNode, r: &ExprNode, result_ty: &CType) {
        // Pointer arithmetic: scale index by element size
        let is_ptr_arith = matches!(
            (l.ty.as_ref().unwrap(), op),
            (CType::Pointer(_), Op::Add | Op::Sub)
        );
        let ptr_sub = matches!(op, Op::Sub)
            && matches!(l.ty.as_ref().unwrap(), CType::Pointer(_))
            && matches!(r.ty.as_ref().unwrap(), CType::Pointer(_));

        self.emit_expr(l);
        self.push_int();
        self.emit_expr(r);
        self.emit("movq %rax, %rcx"); // right in rcx
        self.pop_int("%rax"); // left in rax

        if ptr_sub {
            // ptr - ptr → difference / elem_size
            self.emit("subq %rcx, %rax");
            if let CType::Pointer(inner) = l.ty.as_ref().unwrap() {
                let elem_size = self.type_size(inner);
                if elem_size != 1 {
                    self.emit("cqo");
                    self.emit(&format!("movq ${}, %rcx", elem_size));
                    self.emit("idivq %rcx");
                }
            }
            return;
        }

        if is_ptr_arith && !ptr_sub {
            if let CType::Pointer(inner) = l.ty.as_ref().unwrap() {
                let elem_size = self.type_size(inner);
                if elem_size != 1 {
                    self.emit(&format!("imulq ${}, %rcx", elem_size));
                }
            }
        }

        // Also handle int + ptr (commuted)
        let int_plus_ptr = matches!(op, Op::Add)
            && matches!(r.ty.as_ref().unwrap(), CType::Pointer(_));
        if int_plus_ptr {
            if let CType::Pointer(inner) = r.ty.as_ref().unwrap() {
                let elem_size = self.type_size(inner);
                if elem_size != 1 {
                    self.emit(&format!("imulq ${}, %rax", elem_size));
                }
            }
        }

        let unsigned = Self::is_unsigned(result_ty);

        match op {
            Op::Add => self.emit("addq %rcx, %rax"),
            Op::Sub => self.emit("subq %rcx, %rax"),
            Op::Mul => self.emit("imulq %rcx, %rax"),
            Op::Div => {
                if unsigned {
                    self.emit("xorl %edx, %edx");
                    self.emit("divq %rcx");
                } else {
                    self.emit("cqo");
                    self.emit("idivq %rcx");
                }
            }
            Op::Mod => {
                if unsigned {
                    self.emit("xorl %edx, %edx");
                    self.emit("divq %rcx");
                } else {
                    self.emit("cqo");
                    self.emit("idivq %rcx");
                }
                self.emit("movq %rdx, %rax");
            }
            Op::BitAnd => self.emit("andq %rcx, %rax"),
            Op::BitOr => self.emit("orq %rcx, %rax"),
            Op::BitXor => self.emit("xorq %rcx, %rax"),
            Op::Shl => self.emit("shlq %cl, %rax"),
            Op::Shr => {
                if unsigned {
                    self.emit("shrq %cl, %rax");
                } else {
                    self.emit("sarq %cl, %rax");
                }
            }
            Op::Eq | Op::Ne | Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                self.emit("cmpq %rcx, %rax");
                let cc = match op {
                    Op::Eq => "sete",
                    Op::Ne => "setne",
                    Op::Lt => if unsigned { "setb" } else { "setl" },
                    Op::Gt => if unsigned { "seta" } else { "setg" },
                    Op::Le => if unsigned { "setbe" } else { "setle" },
                    Op::Ge => if unsigned { "setae" } else { "setge" },
                    _ => unreachable!(),
                };
                self.emit(&format!("{} %al", cc));
                self.emit("movzbl %al, %eax");
            }
            _ => {}
        }
    }

    fn emit_float_binop(&mut self, op: Op, l: &ExprNode, r: &ExprNode, result_ty: &CType) {
        let suffix = if matches!(result_ty, CType::Float) { "ss" } else { "sd" };

        self.emit_expr(l);
        self.push_float();
        self.emit_expr(r);
        self.emit(&format!("movap{} %xmm0, %xmm1", if suffix == "ss" { "s" } else { "d" }));
        self.pop_float("%xmm0");

        match op {
            Op::Add => self.emit(&format!("add{} %xmm1, %xmm0", suffix)),
            Op::Sub => self.emit(&format!("sub{} %xmm1, %xmm0", suffix)),
            Op::Mul => self.emit(&format!("mul{} %xmm1, %xmm0", suffix)),
            Op::Div => self.emit(&format!("div{} %xmm1, %xmm0", suffix)),
            Op::Eq | Op::Ne | Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                self.emit(&format!("ucomi{} %xmm1, %xmm0", suffix));
                let cc = match op {
                    Op::Eq => "sete",
                    Op::Ne => "setne",
                    Op::Lt => "setb",
                    Op::Gt => "seta",
                    Op::Le => "setbe",
                    Op::Ge => "setae",
                    _ => unreachable!(),
                };
                self.emit(&format!("{} %al", cc));
                if matches!(op, Op::Eq) {
                    // For float ==, also need setnp (not NaN)
                    self.emit("setnp %cl");
                    self.emit("andb %cl, %al");
                }
                if matches!(op, Op::Ne) {
                    // For float !=, also set on NaN
                    self.emit("setp %cl");
                    self.emit("orb %cl, %al");
                }
                self.emit("movzbl %al, %eax");
            }
            _ => {}
        }
    }

    // === Unary operations ===

    fn emit_unaryop(&mut self, op: UnaryOp, inner: &ExprNode, result_ty: &CType) {
        match op {
            UnaryOp::Neg => {
                self.emit_expr(inner);
                if Self::is_float(result_ty) {
                    // Negate by XOR with sign bit
                    if matches!(result_ty, CType::Float) {
                        self.emit("movd %xmm0, %eax");
                        self.emit("xorl $0x80000000, %eax");
                        self.emit("movd %eax, %xmm0");
                    } else {
                        self.emit("movq %xmm0, %rax");
                        self.emit("movabsq $-9223372036854775808, %rcx"); // 0x8000000000000000
                        self.emit("xorq %rcx, %rax");
                        self.emit("movq %rax, %xmm0");
                    }
                } else {
                    self.emit("negq %rax");
                }
            }
            UnaryOp::Plus => {
                self.emit_expr(inner);
            }
            UnaryOp::BitNot => {
                self.emit_expr(inner);
                self.emit("notq %rax");
            }
            UnaryOp::LogNot => {
                self.emit_expr(inner);
                self.emit_test_zero(inner.ty.as_ref().unwrap());
                self.emit("sete %al");
                self.emit("movzbl %al, %eax");
            }
            UnaryOp::AddrOf => {
                // inner is already an lvalue producing an address
                self.emit_expr(inner);
            }
            UnaryOp::Deref => {
                // Produces an lvalue (address). The inner expr is a pointer value.
                self.emit_expr(inner);
            }
            UnaryOp::PreInc | UnaryOp::PreDec => {
                // inner is lvalue (address)
                let lty = inner.ty.as_ref().unwrap();
                self.emit_expr(inner);
                self.push_int(); // save address
                self.emit_load(lty);
                let delta = if matches!(lty, CType::Pointer(..)) {
                    if let CType::Pointer(inner_ty) = lty {
                        self.type_size(inner_ty) as i64
                    } else { 1 }
                } else { 1 };
                if Self::is_float(lty) {
                    // float inc/dec
                    let suffix = if matches!(lty, CType::Float) { "ss" } else { "sd" };
                    let one = if matches!(lty, CType::Float) {
                        let bits = 1.0f32.to_bits();
                        self.emit(&format!("movl ${}, %eax", bits as i32));
                        self.emit("movd %eax, %xmm1");
                        suffix
                    } else {
                        let bits = 1.0f64.to_bits();
                        self.emit(&format!("movabsq ${}, %rax", bits as i64));
                        self.emit("movq %rax, %xmm1");
                        suffix
                    };
                    if matches!(op, UnaryOp::PreInc) {
                        self.emit(&format!("add{} %xmm1, %xmm0", one));
                    } else {
                        self.emit(&format!("sub{} %xmm1, %xmm0", one));
                    }
                    self.pop_int("%rax");
                    self.emit_store(lty);
                } else {
                    if matches!(op, UnaryOp::PreInc) {
                        self.emit(&format!("addq ${}, %rax", delta));
                    } else {
                        self.emit(&format!("subq ${}, %rax", delta));
                    }
                    self.emit("movq %rax, %rcx");
                    self.pop_int("%rax"); // address
                    self.emit_store(lty);
                    self.emit("movq %rcx, %rax"); // result = new value
                }
            }
            UnaryOp::PostInc | UnaryOp::PostDec => {
                let lty = inner.ty.as_ref().unwrap();
                let delta = if let CType::Pointer(inner_ty) = lty {
                    self.type_size(inner_ty) as i64
                } else { 1 };
                self.emit_expr(inner); // address in rax
                self.push_int(); // save address
                self.emit_load(lty); // old value in rax/xmm0
                if Self::is_float(lty) {
                    let sfx = if matches!(lty, CType::Float) { "ss" } else { "sd" };
                    let ap = if sfx == "ss" { "s" } else { "d" };
                    self.emit(&format!("movap{} %xmm0, %xmm2", ap)); // save old
                    if matches!(lty, CType::Float) {
                        let bits = 1.0f32.to_bits();
                        self.emit(&format!("movl ${}, %eax", bits as i32));
                        self.emit("movd %eax, %xmm1");
                    } else {
                        let bits = 1.0f64.to_bits();
                        self.emit(&format!("movabsq ${}, %rax", bits as i64));
                        self.emit("movq %rax, %xmm1");
                    }
                    if matches!(op, UnaryOp::PostInc) {
                        self.emit(&format!("add{} %xmm1, %xmm0", sfx));
                    } else {
                        self.emit(&format!("sub{} %xmm1, %xmm0", sfx));
                    }
                    self.pop_int("%rax"); // address
                    self.emit_store(lty); // store new
                    self.emit(&format!("movap{} %xmm2, %xmm0", ap)); // return old
                } else {
                    self.emit("movq %rax, %rcx"); // old in rcx
                    if matches!(op, UnaryOp::PostInc) {
                        self.emit(&format!("addq ${}, %rax", delta));
                    } else {
                        self.emit(&format!("subq ${}, %rax", delta));
                    }
                    // rax = new value, rcx = old value
                    self.emit("pushq %rcx"); // save old
                    self.emit("movq %rax, %rcx"); // new in rcx for store
                    // stack: [old_value, address]
                    self.emit("movq 8(%rsp), %rax"); // address
                    self.emit_store(lty);
                    self.emit("popq %rax"); // old value = result
                    self.emit("addq $8, %rsp"); // pop address
                }
            }
        }
    }

    // === Function calls ===

    fn emit_call(&mut self, func: &ExprNode, args: &[ExprNode], _result_ty: &CType) {
        let int_regs = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
        // Determine which args go in int regs vs xmm regs
        let mut int_idx = 0usize;
        let mut xmm_idx = 0usize;
        let mut arg_slots: Vec<(bool, usize)> = Vec::new(); // (is_float, reg_idx)

        for arg in args {
            let aty = arg.ty.as_ref().unwrap();
            if Self::is_float(aty) {
                arg_slots.push((true, xmm_idx));
                xmm_idx += 1;
            } else {
                arg_slots.push((false, int_idx));
                int_idx += 1;
            }
        }

        let xmm_count = xmm_idx;

        // How many stack args?
        let stack_int_args = if int_idx > 6 { int_idx - 6 } else { 0 };
        let stack_xmm_args = if xmm_idx > 8 { xmm_idx - 8 } else { 0 };
        let stack_args = stack_int_args + stack_xmm_args;

        // Align stack to 16 bytes if needed
        let needs_align = stack_args % 2 != 0;
        if needs_align {
            self.emit("subq $8, %rsp");
        }

        // Push stack args in reverse order
        // (For simplicity, only handle register args for now)
        // Evaluate all args and push onto stack (reverse order for register assignment)
        for arg in args.iter().rev() {
            self.emit_expr(arg);
            if Self::is_float(arg.ty.as_ref().unwrap()) {
                self.push_float();
            } else {
                self.push_int();
            }
        }

        // Evaluate function address before popping args into registers,
        // because the function expression may itself be a call that clobbers
        // argument registers.
        self.emit_expr(func);
        self.push_int(); // save function address on stack

        // Pop function address into %r11 (caller-saved, not used for args)
        self.pop_int("%r11");

        // Pop args into registers
        for (i, (is_float, idx)) in arg_slots.iter().enumerate() {
            let _ = i;
            if *is_float {
                if *idx < 8 {
                    self.pop_float(&format!("%xmm{}", idx));
                } else {
                    // Stack arg — leave it
                    self.emit("addq $8, %rsp"); // skip for now
                }
            } else {
                if *idx < 6 {
                    self.pop_int(int_regs[*idx]);
                } else {
                    // Stack arg — leave on stack
                    self.emit("addq $8, %rsp"); // skip for now
                }
            }
        }

        // For variadic functions, set %al to number of xmm args
        // We always set it since we don't easily know if variadic at this point
        self.emit(&format!("movl ${}, %eax", xmm_count.min(8)));

        // Call via %r11
        self.emit("callq *%r11");

        // Clean up stack alignment
        if needs_align {
            self.emit("addq $8, %rsp");
        }
    }

    // === Statements ===

    fn emit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Compound(items) => {
                for item in items {
                    match item {
                        BlockItem::Decl(d) => self.emit_decl(d),
                        BlockItem::Stmt(s) => self.emit_stmt(s),
                    }
                }
            }
            Stmt::Expr(Some(e)) => self.emit_expr(e),
            Stmt::Expr(None) => {}
            Stmt::Return(Some(e)) => {
                self.emit_expr(e);
                let func = self.func_name.clone();
                self.emit(&format!("jmp .Lret_{}", func));
            }
            Stmt::Return(None) => {
                let func = self.func_name.clone();
                self.emit(&format!("jmp .Lret_{}", func));
            }
            Stmt::If(cond, then, else_) => {
                let lelse = self.fresh("else");
                let lend = self.fresh("endif");
                self.emit_expr(cond);
                self.emit_test_zero(cond.ty.as_ref().unwrap());
                self.emit(&format!("je {}", lelse));
                self.emit_stmt(then);
                self.emit(&format!("jmp {}", lend));
                self.label(&lelse);
                if let Some(else_stmt) = else_ {
                    self.emit_stmt(else_stmt);
                }
                self.label(&lend);
            }
            Stmt::While(cond, body) => {
                let lcont = self.fresh("while_cont");
                let lbreak = self.fresh("while_break");
                let old_break = self.break_label.replace(lbreak.clone());
                let old_cont = self.continue_label.replace(lcont.clone());
                self.label(&lcont);
                self.emit_expr(cond);
                self.emit_test_zero(cond.ty.as_ref().unwrap());
                self.emit(&format!("je {}", lbreak));
                self.emit_stmt(body);
                self.emit(&format!("jmp {}", lcont));
                self.label(&lbreak);
                self.break_label = old_break;
                self.continue_label = old_cont;
            }
            Stmt::DoWhile(body, cond) => {
                let ltop = self.fresh("do_top");
                let lcont = self.fresh("do_cont");
                let lbreak = self.fresh("do_break");
                let old_break = self.break_label.replace(lbreak.clone());
                let old_cont = self.continue_label.replace(lcont.clone());
                self.label(&ltop);
                self.emit_stmt(body);
                self.label(&lcont);
                self.emit_expr(cond);
                self.emit_test_zero(cond.ty.as_ref().unwrap());
                self.emit(&format!("jne {}", ltop));
                self.label(&lbreak);
                self.break_label = old_break;
                self.continue_label = old_cont;
            }
            Stmt::For(init, cond, incr, body) => {
                let lcond = self.fresh("for_cond");
                let lcont = self.fresh("for_cont");
                let lbreak = self.fresh("for_break");
                let old_break = self.break_label.replace(lbreak.clone());
                let old_cont = self.continue_label.replace(lcont.clone());
                match init {
                    ForInit::Expr(Some(e)) => self.emit_expr(e),
                    ForInit::Decl(d) => self.emit_decl(d),
                    ForInit::Expr(None) => {}
                }
                self.label(&lcond);
                if let Some(c) = cond {
                    self.emit_expr(c);
                    self.emit_test_zero(c.ty.as_ref().unwrap());
                    self.emit(&format!("je {}", lbreak));
                }
                self.emit_stmt(body);
                self.label(&lcont);
                if let Some(inc) = incr {
                    self.emit_expr(inc);
                }
                self.emit(&format!("jmp {}", lcond));
                self.label(&lbreak);
                self.break_label = old_break;
                self.continue_label = old_cont;
            }
            Stmt::Break => {
                if let Some(lbl) = &self.break_label {
                    let lbl = lbl.clone();
                    self.emit(&format!("jmp {}", lbl));
                }
            }
            Stmt::Continue => {
                if let Some(lbl) = &self.continue_label {
                    let lbl = lbl.clone();
                    self.emit(&format!("jmp {}", lbl));
                }
            }
            Stmt::Goto(name) => {
                let func = self.func_name.clone();
                self.emit(&format!("jmp .Llabel_{}_{}", func, name));
            }
            Stmt::Labeled(name, body) => {
                let func = self.func_name.clone();
                self.label(&format!(".Llabel_{}_{}", func, name));
                self.emit_stmt(body);
            }
            Stmt::Switch(cond, body) => {
                // Simple implementation: evaluate cond, then walk body looking for Case nodes
                // For now, implement as chained if-else by collecting cases first
                let lbreak = self.fresh("sw_break");
                let old_break = self.break_label.replace(lbreak.clone());
                self.emit_expr(cond);
                self.push_int(); // save switch value
                self.emit_switch_body(body);
                self.pop_int("%rax"); // clean up switch value
                self.label(&lbreak);
                self.break_label = old_break;
            }
            Stmt::Case(val, body) => {
                // Case labels are handled by emit_switch_body; if we reach here
                // outside switch context, just emit the body
                self.emit_expr(val);
                self.emit_stmt(body);
            }
            Stmt::Default(body) => {
                self.emit_stmt(body);
            }
        }
    }

    fn emit_switch_body(&mut self, stmt: &Stmt) {
        // Collect case labels and default, emit as chained comparisons
        let mut cases: Vec<(&ExprNode, String)> = Vec::new();
        let mut default_label: Option<String> = None;
        let end_label = self.break_label.clone().unwrap();

        // First pass: assign labels to all case/default
        self.collect_cases(stmt, &mut cases, &mut default_label);

        // Emit comparisons: switch value is on top of stack
        for (val, lbl) in &cases {
            // Load switch value from stack (peek)
            self.emit("movq (%rsp), %rax");
            self.push_int();
            self.emit_expr(val);
            self.emit("movq %rax, %rcx");
            self.pop_int("%rax");
            self.emit("cmpq %rcx, %rax");
            self.emit(&format!("je {}", lbl));
        }
        if let Some(ref dl) = default_label {
            self.emit(&format!("jmp {}", dl));
        } else {
            self.emit(&format!("jmp {}", end_label));
        }

        // Second pass: emit the body with labels
        self.emit_switch_stmt(stmt, &cases, &default_label);
    }

    fn collect_cases<'a>(
        &mut self,
        stmt: &'a Stmt,
        cases: &mut Vec<(&'a ExprNode, String)>,
        default: &mut Option<String>,
    ) {
        match stmt {
            Stmt::Case(val, body) => {
                let lbl = self.fresh("case");
                cases.push((val, lbl));
                self.collect_cases(body, cases, default);
            }
            Stmt::Default(body) => {
                *default = Some(self.fresh("default"));
                self.collect_cases(body, cases, default);
            }
            Stmt::Compound(items) => {
                for item in items {
                    if let BlockItem::Stmt(s) = item {
                        self.collect_cases(s, cases, default);
                    }
                }
            }
            Stmt::Labeled(_, body) => self.collect_cases(body, cases, default),
            Stmt::DoWhile(body, _) | Stmt::While(_, body) => {
                self.collect_cases(body, cases, default);
            }
            Stmt::For(_, _, _, body) => {
                self.collect_cases(body, cases, default);
            }
            Stmt::If(_, then, else_) => {
                self.collect_cases(then, cases, default);
                if let Some(e) = else_ {
                    self.collect_cases(e, cases, default);
                }
            }
            _ => {}
        }
    }

    fn emit_switch_stmt(
        &mut self,
        stmt: &Stmt,
        cases: &[(&ExprNode, String)],
        default: &Option<String>,
    ) {
        match stmt {
            Stmt::Case(val, body) => {
                // Find the label for this case
                if let Some((_, lbl)) = cases.iter().find(|(v, _)| std::ptr::eq(*v, val)) {
                    self.label(lbl);
                }
                self.emit_switch_stmt(body, cases, default);
            }
            Stmt::Default(body) => {
                if let Some(lbl) = default {
                    self.label(lbl);
                }
                self.emit_switch_stmt(body, cases, default);
            }
            Stmt::Compound(items) => {
                for item in items {
                    match item {
                        BlockItem::Stmt(s) => self.emit_switch_stmt(s, cases, default),
                        BlockItem::Decl(d) => self.emit_decl(d),
                    }
                }
            }
            Stmt::Labeled(name, body) => {
                let func = self.func_name.clone();
                self.label(&format!(".Llabel_{}_{}", func, name));
                self.emit_switch_stmt(body, cases, default);
            }
            Stmt::DoWhile(body, cond) => {
                let ltop = self.fresh("do_top");
                let lcont = self.fresh("do_cont");
                let lbreak = self.fresh("do_break");
                let old_break = self.break_label.replace(lbreak.clone());
                let old_cont = self.continue_label.replace(lcont.clone());
                self.label(&ltop);
                self.emit_switch_stmt(body, cases, default);
                self.label(&lcont);
                self.emit_expr(cond);
                self.emit_test_zero(cond.ty.as_ref().unwrap());
                self.emit(&format!("jne {}", ltop));
                self.label(&lbreak);
                self.break_label = old_break;
                self.continue_label = old_cont;
            }
            Stmt::While(cond, body) => {
                let lcont = self.fresh("while_cont");
                let lbreak = self.fresh("while_break");
                let old_break = self.break_label.replace(lbreak.clone());
                let old_cont = self.continue_label.replace(lcont.clone());
                self.label(&lcont);
                self.emit_expr(cond);
                self.emit_test_zero(cond.ty.as_ref().unwrap());
                self.emit(&format!("je {}", lbreak));
                self.emit_switch_stmt(body, cases, default);
                self.emit(&format!("jmp {}", lcont));
                self.label(&lbreak);
                self.break_label = old_break;
                self.continue_label = old_cont;
            }
            Stmt::For(init, cond, incr, body) => {
                let lcond = self.fresh("for_cond");
                let lcont = self.fresh("for_cont");
                let lbreak = self.fresh("for_break");
                let old_break = self.break_label.replace(lbreak.clone());
                let old_cont = self.continue_label.replace(lcont.clone());
                match init {
                    ForInit::Expr(Some(e)) => self.emit_expr(e),
                    ForInit::Decl(d) => self.emit_decl(d),
                    ForInit::Expr(None) => {}
                }
                self.label(&lcond);
                if let Some(c) = cond {
                    self.emit_expr(c);
                    self.emit_test_zero(c.ty.as_ref().unwrap());
                    self.emit(&format!("je {}", lbreak));
                }
                self.emit_switch_stmt(body, cases, default);
                self.label(&lcont);
                if let Some(inc) = incr {
                    self.emit_expr(inc);
                }
                self.emit(&format!("jmp {}", lcond));
                self.label(&lbreak);
                self.break_label = old_break;
                self.continue_label = old_cont;
            }
            Stmt::If(cond, then, else_) => {
                let lelse = self.fresh("else");
                let lend = self.fresh("endif");
                self.emit_expr(cond);
                self.emit_test_zero(cond.ty.as_ref().unwrap());
                self.emit(&format!("je {}", lelse));
                self.emit_switch_stmt(then, cases, default);
                self.emit(&format!("jmp {}", lend));
                self.label(&lelse);
                if let Some(else_stmt) = else_ {
                    self.emit_switch_stmt(else_stmt, cases, default);
                }
                self.label(&lend);
            }
            other => self.emit_stmt(other),
        }
    }

    fn emit_decl(&mut self, d: &Decl) {
        if d.is_typedef { return; }
        for id in &d.declarators {
            if let Some(init) = &id.init {
                let ty = id.ty.as_ref().unwrap();
                let offset = self.locals[&id.name];
                match init {
                    Init::Expr(e) => {
                        self.emit_expr(e);
                        if Self::is_float(ty) {
                            self.emit(&format!("leaq {}(%rbp), %rax", offset));
                            self.emit_store(ty);
                        } else {
                            self.emit(&format!("movq %rax, {}(%rbp)", offset));
                        }
                    }
                    Init::List(items) => {
                        self.emit_local_init_list(offset, ty, items);
                    }
                }
            }
        }
    }

    fn emit_local_init_list(&mut self, base_offset: i32, ty: &CType, items: &[InitItem]) {
        // Zero-fill the entire variable first
        let size = self.type_size(ty);
        if size > 0 {
            self.emit(&format!("leaq {}(%rbp), %rdi", base_offset));
            self.emit("xorl %eax, %eax");
            self.emit(&format!("movl ${}, %ecx", size));
            self.emit("rep stosb");
        }
        // Store each initializer item
        match ty {
            CType::Array(elem, _) => {
                let elem_size = self.type_size(elem);
                let mut next_idx = 0usize;
                for item in items {
                    let idx = if let Some(Designator::Index(e)) = item.designation.first() {
                        eval_const_init(e).unwrap_or(next_idx as i64) as usize
                    } else {
                        next_idx
                    };
                    let item_offset = base_offset + idx as i32 * elem_size;
                    self.emit_local_store_init(&item.init, elem, item_offset);
                    next_idx = idx + 1;
                }
            }
            CType::Struct(name) | CType::Union(name) => {
                let fields: Vec<(String, i32, CType)> = self.struct_layouts.get(name)
                    .map(|l| l.fields.clone())
                    .unwrap_or_default();
                let mut next_field = 0usize;
                for item in items {
                    let field_idx = if let Some(Designator::Field(fname)) = item.designation.first() {
                        fields.iter().position(|(n, _, _)| n == fname).unwrap_or(next_field)
                    } else {
                        next_field
                    };
                    if field_idx < fields.len() {
                        let (_, field_offset, ref field_ty) = fields[field_idx];
                        self.emit_local_store_init(&item.init, field_ty, base_offset + field_offset);
                        next_field = field_idx + 1;
                    }
                }
            }
            _ => {
                if let Some(item) = items.first() {
                    self.emit_local_store_init(&item.init, ty, base_offset);
                }
            }
        }
    }

    fn emit_local_store_init(&mut self, init: &Init, ty: &CType, offset: i32) {
        match init {
            Init::Expr(e) => {
                self.emit_expr(e);
                if Self::is_float(ty) {
                    self.emit(&format!("leaq {}(%rbp), %rax", offset));
                    self.emit_store(ty);
                } else {
                    self.emit("movq %rax, %rcx");
                    self.emit(&format!("leaq {}(%rbp), %rax", offset));
                    self.emit_store(ty);
                }
            }
            Init::List(items) => {
                // Nested init list — dispatch without re-zeroing
                match ty {
                    CType::Array(elem, _) => {
                        let elem_size = self.type_size(elem);
                        for (i, item) in items.iter().enumerate() {
                            self.emit_local_store_init(&item.init, elem, offset + i as i32 * elem_size);
                        }
                    }
                    CType::Struct(name) | CType::Union(name) => {
                        let fields: Vec<(String, i32, CType)> = self.struct_layouts.get(name)
                            .map(|l| l.fields.clone())
                            .unwrap_or_default();
                        for (i, item) in items.iter().enumerate() {
                            if i >= fields.len() { break; }
                            let (_, field_offset, ref field_ty) = fields[i];
                            self.emit_local_store_init(&item.init, field_ty, offset + field_offset);
                        }
                    }
                    _ => {
                        if let Some(item) = items.first() {
                            self.emit_local_store_init(&item.init, ty, offset);
                        }
                    }
                }
            }
        }
    }

    // === Function emission ===

    fn emit_function(&mut self, f: &FunctionDef) {
        self.func_name = f.name.clone();
        self.locals.clear();
        self.stack_size = 0;
        self.break_label = None;
        self.continue_label = None;

        // Allocate parameter slots first
        let int_regs = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
        let mut param_slots = Vec::new();
        for p in &f.params {
            if let Some(name) = &p.name {
                let ty = p.ty.as_ref().unwrap().clone();
                self.stack_size += 8;
                let offset = -self.stack_size;
                self.locals.insert(name.clone(), offset);
                param_slots.push((name.clone(), offset, ty));
            }
        }

        // Allocate local variable slots
        self.alloc_locals(&f.body);

        // Align stack to 16 bytes
        self.stack_size = (self.stack_size + 15) & !15;

        // Emit function header
        self.out.push_str(&format!("\t.globl {}\n", f.name));
        self.out.push_str(&format!("\t.type {}, @function\n", f.name));
        self.label(&f.name);

        // Prologue
        self.emit("pushq %rbp");
        self.emit("movq %rsp, %rbp");
        if self.stack_size > 0 {
            self.emit(&format!("subq ${}, %rsp", self.stack_size));
        }

        // Copy parameters from registers to stack
        let mut int_idx = 0usize;
        let mut xmm_idx = 0usize;
        for (_name, offset, ty) in &param_slots {
            if Self::is_float(ty) {
                if xmm_idx < 8 {
                    if matches!(ty, CType::Float) {
                        self.emit(&format!("movss %xmm{}, {}(%rbp)", xmm_idx, offset));
                    } else {
                        self.emit(&format!("movsd %xmm{}, {}(%rbp)", xmm_idx, offset));
                    }
                    xmm_idx += 1;
                }
            } else {
                if int_idx < 6 {
                    self.emit(&format!("movq {}, {}(%rbp)", int_regs[int_idx], offset));
                    int_idx += 1;
                }
            }
        }

        // Emit body
        self.emit_stmt(&f.body);

        // Emit implicit return 0 for main (C11 5.1.2.2.3)
        if f.name == "main" {
            self.emit("xorl %eax, %eax");
        }

        // Epilogue
        self.label(&format!(".Lret_{}", f.name));
        self.emit("leave");
        self.emit("retq");
        self.out.push('\n');
    }

    /// Emit a string literal as data, zero-padding to `size` bytes.
    fn emit_string_data(&mut self, s: &str, size: i32) {
        // Emit the string bytes (with null terminator)
        self.out.push_str(&format!("\t.string \"{}\"\n", s));
        // Zero-pad if the array is larger than the string + null
        let string_size = s.len() as i32 + 1;
        if size > string_size {
            self.out.push_str(&format!("\t.zero {}\n", size - string_size));
        }
    }

    fn emit_data_value(&mut self, size: i32, val: i64) {
        match size {
            1 => self.out.push_str(&format!("\t.byte {}\n", val)),
            2 => self.out.push_str(&format!("\t.short {}\n", val)),
            4 => self.out.push_str(&format!("\t.long {}\n", val)),
            8 => self.out.push_str(&format!("\t.quad {}\n", val)),
            _ => self.out.push_str(&format!("\t.zero {}\n", size)),
        }
    }

    fn emit_init_item(&mut self, init: &Init, ty: &CType, size: i32) {
        match init {
            Init::Expr(e) => {
                if let Some(s) = extract_string_init(e) {
                    self.emit_string_data(s, size);
                } else if let Some(val) = eval_const_init(e) {
                    self.emit_data_value(size, val);
                } else if let Some(sym) = extract_global_addr(e) {
                    self.out.push_str(&format!("\t.quad {}\n", sym));
                } else {
                    self.out.push_str(&format!("\t.zero {}\n", size));
                }
            }
            Init::List(sub) => {
                self.emit_init_list(ty, sub);
            }
        }
    }

    fn emit_init_list(&mut self, ty: &CType, items: &[InitItem]) {
        match ty {
            CType::Struct(name) | CType::Union(name) => {
                let fields: Vec<(String, i32, CType)> = self.struct_layouts.get(name)
                    .map(|l| l.fields.clone())
                    .unwrap_or_default();
                let total_size = self.struct_layouts.get(name).map_or(0, |l| l.size);

                // Map each init item to a field index, respecting designators
                let mut field_inits: Vec<Option<&Init>> = vec![None; fields.len()];
                let mut next_field = 0usize;
                for item in items {
                    let field_idx = if let Some(Designator::Field(fname)) = item.designation.first() {
                        fields.iter().position(|(n, _, _)| n == fname).unwrap_or(next_field)
                    } else {
                        next_field
                    };
                    if field_idx < fields.len() {
                        field_inits[field_idx] = Some(&item.init);
                        next_field = field_idx + 1;
                    }
                }

                // Emit in field order
                let mut offset = 0;
                for (i, (_, field_offset, field_ty)) in fields.iter().enumerate() {
                    if *field_offset > offset {
                        self.out.push_str(&format!("\t.zero {}\n", field_offset - offset));
                    }
                    let field_size = self.type_size(field_ty);
                    if let Some(init) = field_inits[i] {
                        self.emit_init_item(init, field_ty, field_size);
                    } else {
                        self.out.push_str(&format!("\t.zero {}\n", field_size));
                    }
                    offset = field_offset + field_size;
                }
                if offset < total_size {
                    self.out.push_str(&format!("\t.zero {}\n", total_size - offset));
                }
            }
            CType::Array(elem, count) => {
                let elem_size = self.type_size(elem);
                let n = count.unwrap_or(items.len() as u64) as usize;

                // Map each init item to an array index, respecting designators
                let mut elem_inits: Vec<Option<&Init>> = vec![None; n];
                let mut next_idx = 0usize;
                for item in items {
                    let idx = if let Some(Designator::Index(e)) = item.designation.first() {
                        eval_const_init(e).unwrap_or(next_idx as i64) as usize
                    } else {
                        next_idx
                    };
                    if idx < n {
                        elem_inits[idx] = Some(&item.init);
                        next_idx = idx + 1;
                    }
                }

                // Emit in index order
                for i in 0..n {
                    if let Some(init) = elem_inits[i] {
                        self.emit_init_item(init, elem, elem_size);
                    } else {
                        self.out.push_str(&format!("\t.zero {}\n", elem_size));
                    }
                }
            }
            _ => {
                let size = self.type_size(ty);
                if let Some(item) = items.first() {
                    self.emit_init_item(&item.init, ty, size);
                } else {
                    self.out.push_str(&format!("\t.zero {}\n", size));
                }
            }
        }
    }
}

fn parse_char_literal(s: &str) -> i32 {
    let inner = &s[1..s.len()-1]; // strip quotes
    if inner.starts_with('\\') {
        match inner.as_bytes().get(1) {
            Some(b'n') => 10,
            Some(b't') => 9,
            Some(b'r') => 13,
            Some(b'0') => 0,
            Some(b'\\') => 92,
            Some(b'\'') => 39,
            Some(b'"') => 34,
            Some(b'a') => 7,
            Some(b'b') => 8,
            Some(b'f') => 12,
            Some(b'v') => 11,
            Some(b'x') => {
                i32::from_str_radix(&inner[2..], 16).unwrap_or(0)
            }
            Some(c) if c.is_ascii_digit() => {
                i32::from_str_radix(&inner[1..], 8).unwrap_or(0)
            }
            _ => inner.as_bytes()[1] as i32,
        }
    } else {
        inner.as_bytes()[0] as i32
    }
}

fn type_layout(ty: &CType, layouts: &HashMap<String, StructLayout>) -> Layout {
    let size = match ty {
        CType::Void => 0,
        CType::Bool | CType::Char(_) => 1,
        CType::Short(_) => 2,
        CType::Int(_) | CType::Float | CType::Enum(_) => 4,
        CType::Long(_) | CType::LongLong(_) | CType::Double
        | CType::Pointer(_) | CType::LongDouble => 8,
        CType::Array(elem, Some(n)) => {
            type_layout(elem, layouts).size() * *n as usize
        }
        CType::Array(_, None) => 8,
        CType::Struct(name) | CType::Union(name) => {
            return layouts.get(name)
                .map_or(Layout::new::<()>(), |l| Layout::from_size_align(l.size as usize, l.align as usize).unwrap());
        }
        _ => 8,
    };
    let align = size.min(8).max(1);
    Layout::from_size_align(size, align).unwrap()
}

fn compute_struct_layout(
    fields: &[(String, CType)],
    layouts: &HashMap<String, StructLayout>,
    is_union: bool,
) -> StructLayout {
    let mut result = Vec::new();
    let mut compound = Layout::from_size_align(0, 1).unwrap();
    for (fname, fty) in fields {
        let field_layout = type_layout(fty, layouts);
        if is_union {
            result.push((fname.clone(), 0, fty.clone()));
            compound = Layout::from_size_align(
                compound.size().max(field_layout.size()),
                compound.align().max(field_layout.align()),
            ).unwrap();
        } else {
            let (new_layout, offset) = compound.extend(field_layout).unwrap();
            result.push((fname.clone(), offset as i32, fty.clone()));
            compound = new_layout;
        }
    }
    let padded = compound.pad_to_align();
    StructLayout { fields: result, size: padded.size() as i32, align: padded.align() as i32 }
}

pub fn codegen(unit: &TranslationUnit) -> String {
    let mut cg = Codegen::new();

    // Compute struct layouts (multi-pass to resolve dependencies in order)
    {
        let all: Vec<_> = unit.structs.iter().collect();
        let mut remaining: Vec<_> = all.iter().map(|(n, v)| (n.as_str(), v)).collect();
        while !remaining.is_empty() {
            let before = remaining.len();
            remaining.retain(|(name, (is_union, fields))| {
                for (_, fty) in fields.iter() {
                    if let CType::Struct(n) | CType::Union(n) = fty {
                        if !cg.struct_layouts.contains_key(n) {
                            return true; // dependency not ready yet
                        }
                    }
                }
                let layout = compute_struct_layout(fields, &cg.struct_layouts, *is_union);
                cg.struct_layouts.insert(name.to_string(), layout);
                false
            });
            if remaining.len() == before {
                // No progress; compute remaining anyway to avoid infinite loop
                for (name, (is_union, fields)) in remaining {
                    let layout = compute_struct_layout(fields, &cg.struct_layouts, *is_union);
                    cg.struct_layouts.insert(name.to_string(), layout);
                }
                break;
            }
        }
    }

    // Collect global names
    for d in &unit.decls {
        for id in &d.declarators {
            cg.globals.insert(id.name.clone());
        }
    }
    for f in &unit.functions {
        cg.globals.insert(f.name.clone());
    }

    cg.out.push_str("\t.section .note.GNU-stack,\"\",@progbits\n");
    cg.out.push_str("\t.text\n");

    // Emit functions
    for f in &unit.functions {
        cg.emit_function(f);
    }

    // Emit string literals
    if !cg.strings.is_empty() || !unit.globals.is_empty() {
        cg.out.push_str("\t.section .rodata\n");
    }
    for (i, s) in cg.strings.iter().enumerate() {
        cg.out.push_str(&format!(".Lstr_{}:\n", i));
        // s is the raw C string content (without quotes); re-wrap in quotes for .string directive
        cg.out.push_str(&format!("\t.string \"{}\"\n", s));
    }

    // Deduplicate global variables: keep the definition with an initializer, or the last tentative.
    let mut global_decls: std::collections::HashMap<String, (&Decl, &InitDeclarator)> = Default::default();
    for d in &unit.decls {
        if d.is_typedef { continue; }
        let is_extern = d.specs.iter().any(|s| matches!(s, DeclSpec::Storage(StorageClass::Extern)));
        if is_extern { continue; }
        for id in &d.declarators {
            let entry = global_decls.entry(id.name.clone());
            match entry {
                std::collections::hash_map::Entry::Vacant(e) => { e.insert((d, id)); }
                std::collections::hash_map::Entry::Occupied(mut e) => {
                    // Prefer the one with an initializer
                    if id.init.is_some() { e.insert((d, id)); }
                }
            }
        }
    }
    // Emit in declaration order
    let mut emitted = HashSet::new();
    for d in &unit.decls {
        if d.is_typedef { continue; }
        for id in &d.declarators {
            if !emitted.insert(id.name.clone()) { continue; }
            let Some(&(_decl, id)) = global_decls.get(&id.name) else { continue };
            let ty = id.ty.as_ref().unwrap();
            if matches!(ty, CType::Function { .. }) { continue; }
            let size = cg.type_size(ty);
            if size == 0 { continue; }
            cg.out.push_str(&format!("\t.data\n"));
            cg.out.push_str(&format!("\t.globl {}\n", id.name));
            cg.out.push_str(&format!("{}:\n", id.name));
            match &id.init {
                Some(Init::Expr(e)) => {
                    if let Some(s) = extract_string_init(e) {
                        cg.emit_string_data(s, size);
                    } else if let Some(val) = eval_const_init(e) {
                        cg.emit_data_value(size, val);
                    } else if let Some(sym) = extract_global_addr(e) {
                        cg.out.push_str(&format!("\t.quad {}\n", sym));
                    } else {
                        cg.out.push_str(&format!("\t.zero {}\n", size));
                    }
                }
                Some(Init::List(items)) => {
                    cg.emit_init_list(&ty, items);
                }
                None => {
                    cg.out.push_str(&format!("\t.zero {}\n", size));
                }
            }
        }
    }

    cg.out
}

/// Evaluate a constant initializer expression (handles Cast, ImplicitCast, Constant, unary minus).
fn eval_const_init(e: &ExprNode) -> Option<i64> {
    match e.expr.as_ref() {
        Expr::Constant(s) => Some(parse_int_constant(s)),
        Expr::Cast(_, inner) | Expr::ImplicitCast(_, inner) => eval_const_init(inner),
        Expr::UnaryOp(UnaryOp::Neg, inner) => eval_const_init(inner).map(|v| -v),
        _ => None,
    }
}

/// Extract a string literal from an initializer expression (unwrapping Decay/ImplicitCast/Load).
fn extract_string_init(e: &ExprNode) -> Option<&str> {
    match e.expr.as_ref() {
        Expr::StringLit(s) => Some(s),
        Expr::Decay(inner) | Expr::Load(inner) | Expr::ImplicitCast(_, inner) | Expr::Cast(_, inner) => {
            extract_string_init(inner)
        }
        _ => None,
    }
}

/// Extract a global symbol address from an initializer (e.g., &x, func_name).
fn extract_global_addr(e: &ExprNode) -> Option<String> {
    match e.expr.as_ref() {
        Expr::UnaryOp(UnaryOp::AddrOf, inner) => match inner.expr.as_ref() {
            Expr::Var(name) => Some(name.clone()),
            Expr::Load(inner2) => match inner2.expr.as_ref() {
                Expr::Var(name) => Some(name.clone()),
                _ => None,
            },
            _ => None,
        },
        Expr::Cast(_, inner) | Expr::ImplicitCast(_, inner)
        | Expr::Decay(inner) | Expr::FuncToPtr(inner) => extract_global_addr(inner),
        _ => None,
    }
}

fn parse_int_constant(s: &str) -> i64 {
    let s_lower = s.to_ascii_lowercase();
    let s_clean = s_lower.trim_end_matches(|c: char| c == 'u' || c == 'l');
    let val: u64 = if s_clean.starts_with("0x") {
        u64::from_str_radix(&s_clean[2..], 16).unwrap_or(0)
    } else if s_clean.starts_with('0') && s_clean.len() > 1 {
        u64::from_str_radix(&s_clean[1..], 8).unwrap_or(0)
    } else {
        s_clean.parse().unwrap_or(0)
    };
    val as i64
}

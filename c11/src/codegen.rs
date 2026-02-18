use std::alloc::Layout;
use std::collections::{HashMap, HashSet};
use crate::ast::*;

/// Computed struct layout: field name → (byte offset, field type, optional bitfield info).
/// Bitfield info is (bit_offset, bit_width) within the storage unit at byte_offset.
struct StructLayout {
    fields: Vec<(String, i32, CType, Option<(u8, u8)>)>,
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
    /// Static locals: variable name → global label
    statics: HashMap<String, String>,
    /// Deferred static variable definitions to emit in .data
    static_defs: Vec<(String, CType, Option<Init>)>,
    /// For variadic functions: stack offset of register save area, and number of named int params
    va_info: Option<(i32, usize)>,
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
            statics: HashMap::new(),
            static_defs: Vec::new(),
            va_info: None,
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
            CType::Typedef(_) => panic!("ICE: unresolved typedef in codegen"),
        }
    }

    fn field_offset(&self, struct_name: &str, field: &str) -> (i32, CType) {
        let (off, ty, _) = self.field_info(struct_name, field);
        (off, ty)
    }

    /// Returns (byte_offset, type, bitfield_info) for a field.
    fn field_info(&self, struct_name: &str, field: &str) -> (i32, CType, Option<(u8, u8)>) {
        self.find_field_info(struct_name, field)
            .unwrap_or((0, CType::Int(Sign::Signed), None))
    }

    fn find_field_info(&self, struct_name: &str, field: &str) -> Option<(i32, CType, Option<(u8, u8)>)> {
        let layout = self.struct_layouts.get(struct_name)?;
        // Direct lookup
        for (name, offset, ty, bf) in &layout.fields {
            if name == field {
                return Some((*offset, ty.clone(), *bf));
            }
        }
        // Search through anonymous nested structs/unions
        for (name, offset, ty, _) in &layout.fields {
            if let CType::Struct(inner) | CType::Union(inner) = ty {
                if name.starts_with("__anon_") {
                    if let Some((inner_offset, inner_ty, inner_bf)) = self.find_field_info(inner, field) {
                        return Some((offset + inner_offset, inner_ty, inner_bf));
                    }
                }
            }
        }
        None
    }

    /// If the expression is a Member or PtrMember that is a bitfield, return (bit_offset, bit_width, field_type).
    fn get_bitfield_info(&self, e: &ExprNode) -> Option<(u8, u8, CType)> {
        match e.expr.as_ref() {
            Expr::Member(obj, field) => {
                let struct_name = match obj.ty.as_ref().unwrap() {
                    CType::Struct(n) | CType::Union(n) => n.clone(),
                    _ => return None,
                };
                let (_, fty, bf) = self.field_info(&struct_name, field);
                bf.map(|(off, w)| (off, w, fty))
            }
            Expr::PtrMember(obj, field) => {
                let struct_name = match obj.ty.as_ref().unwrap() {
                    CType::Pointer(inner) => match inner.as_ref() {
                        CType::Struct(n) | CType::Union(n) => n.clone(),
                        _ => return None,
                    },
                    _ => return None,
                };
                let (_, fty, bf) = self.field_info(&struct_name, field);
                bf.map(|(off, w)| (off, w, fty))
            }
            _ => None,
        }
    }

    fn is_float(ty: &CType) -> bool {
        matches!(ty, CType::Float | CType::Double | CType::LongDouble)
    }

    fn is_unsigned(ty: &CType) -> bool {
        matches!(ty, CType::Char(Sign::Unsigned) | CType::Short(Sign::Unsigned)
            | CType::Int(Sign::Unsigned) | CType::Long(Sign::Unsigned)
            | CType::LongLong(Sign::Unsigned) | CType::Bool | CType::Pointer(_))
    }

    fn is_long_or_ptr(ty: &CType) -> bool {
        matches!(ty, CType::Long(_) | CType::LongLong(_) | CType::Pointer(_))
    }

    /// Count scalar init slots for brace elision (array fields expand to their element count).
    fn scalar_init_slots(&self, ty: &CType) -> usize {
        match ty {
            CType::Struct(name) | CType::Union(name) => {
                self.struct_layouts.get(name).map_or(1, |layout| {
                    layout.fields.iter().map(|(_, _, fty, _)| self.scalar_init_slots(fty)).sum()
                })
            }
            CType::Array(elem, Some(n)) => *n as usize * self.scalar_init_slots(elem),
            CType::Typedef(_) => panic!("ICE: unresolved typedef"),
            _ => 1,
        }
    }

    fn is_static_decl(d: &Decl) -> bool {
        d.specs.iter().any(|s| matches!(s, DeclSpec::Storage(StorageClass::Static)))
    }

    fn is_extern_decl(d: &Decl) -> bool {
        d.specs.iter().any(|s| matches!(s, DeclSpec::Storage(StorageClass::Extern)))
    }

    // === Stack allocation ===

    fn alloc_locals(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Compound(items) => {
                for item in items {
                    match item {
                        BlockItem::Decl(d) => {
                            if d.is_typedef || Self::is_extern_decl(d) { continue; }
                            for id in &d.declarators {
                                let ty = id.ty.as_ref().unwrap();
                                // Skip function declarations (forward decls inside function bodies)
                                if matches!(ty, CType::Function { .. }) { continue; }
                                if Self::is_static_decl(d) {
                                    let label = format!("{}.{}", self.func_name, id.name);
                                    self.statics.insert(id.name.clone(), label.clone());
                                    self.static_defs.push((label, ty.clone(), id.init.clone()));
                                    continue;
                                }
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
            CType::Long(_) | CType::LongLong(_) | CType::Pointer(_) => self.emit("movq (%rax), %rax"),
            CType::Void | CType::Array(..) | CType::Function { .. }
            | CType::Struct(_) | CType::Union(_) => {}
            CType::Typedef(_) => panic!("ICE: unresolved typedef"),
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
            CType::Long(_) | CType::LongLong(_) | CType::Pointer(_) => self.emit("movq %rcx, (%rax)"),
            CType::Void | CType::Array(..) | CType::Function { .. }
            | CType::Struct(_) | CType::Union(_) => {}
            CType::Typedef(_) => panic!("ICE: unresolved typedef"),
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
                if let Some(label) = self.statics.get(name) {
                    self.emit(&format!("leaq {}(%rip), %rax", label));
                } else if let Some(&offset) = self.locals.get(name) {
                    self.emit(&format!("leaq {}(%rbp), %rax", offset));
                } else {
                    self.emit(&format!("leaq {}(%rip), %rax", name));
                }
            }
            Expr::Load(inner) => {
                if let Some((bit_off, bit_width, field_ty)) = self.get_bitfield_info(inner) {
                    // Bitfield read: load storage unit, extract field
                    self.emit_expr(inner);
                    let storage_ty = inner.ty.as_ref().unwrap();
                    self.emit_load(storage_ty);
                    let is_wide = Self::is_long_or_ptr(storage_ty);
                    // Use the declared field type for signedness (not storage type)
                    // Enum bitfields are treated as unsigned
                    let is_signed = match &field_ty {
                        CType::Char(s) | CType::Short(s) | CType::Int(s) |
                        CType::Long(s) | CType::LongLong(s) => *s == Sign::Signed,
                        _ => false, // enums, bool, etc. -> unsigned
                    };
                    let storage_bits: u8 = if is_wide { 64 } else { 32 };
                    if is_signed {
                        // Sign-extend: shl to put sign bit at MSB, then sar
                        let shl = storage_bits - bit_width - bit_off;
                        let sar = storage_bits - bit_width;
                        if is_wide {
                            if shl != 0 { self.emit(&format!("shlq ${}, %rax", shl)); }
                            self.emit(&format!("sarq ${}, %rax", sar));
                        } else {
                            if shl != 0 { self.emit(&format!("shll ${}, %eax", shl)); }
                            self.emit(&format!("sarl ${}, %eax", sar));
                        }
                    } else {
                        // Zero-extend: shr then mask
                        if bit_off != 0 {
                            if is_wide {
                                self.emit(&format!("shrq ${}, %rax", bit_off));
                            } else {
                                self.emit(&format!("shrl ${}, %eax", bit_off));
                            }
                        }
                        let mask = (1u64 << bit_width) - 1;
                        if is_wide {
                            self.emit(&format!("movabsq ${}, %rcx", mask as i64));
                            self.emit("andq %rcx, %rax");
                        } else {
                            self.emit(&format!("andl ${}, %eax", mask as i32));
                        }
                    }
                } else {
                    self.emit_expr(inner);
                    self.emit_load(ty);
                }
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
                // Sign-extend int index to 64-bit for pointer arithmetic
                if !Self::is_long_or_ptr(idx.ty.as_ref().unwrap()) {
                    self.emit("movslq %eax, %rax");
                }
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
            Expr::SizeofType(t) => {
                let sz = self.type_size(t);
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
            Expr::VaArg(va_list_expr, _tn) => {
                // x86-64 SysV ABI va_arg for integer/pointer types
                // va_list is a pointer to __va_list_tag struct:
                //   0: gp_offset (u32)
                //   4: fp_offset (u32)
                //   8: overflow_arg_area (ptr)
                //  16: reg_save_area (ptr)
                let va_ptr = match va_list_expr.expr.as_ref() {
                    Expr::Load(inner) | Expr::Decay(inner) => inner,
                    _ => va_list_expr,
                };
                self.emit_expr(va_ptr);
                // %rax = pointer to __va_list_tag
                let lreg = self.fresh("va_reg");
                let lstack = self.fresh("va_stack");
                let lend = self.fresh("va_end");
                // Save va_list ptr
                self.push_int();
                // Check if gp_offset < 48
                self.emit("movl (%rax), %ecx");       // gp_offset
                self.emit("cmpl $48, %ecx");
                self.emit(&format!("jae {}", lstack));
                // Register path: load from reg_save_area + gp_offset
                self.label(&lreg);
                self.emit("movq 16(%rax), %rdx");      // reg_save_area
                self.emit("movslq %ecx, %rcx");
                self.emit("addq %rcx, %rdx");           // reg_save_area + gp_offset
                // Increment gp_offset by 8
                self.emit("movl (%rax), %ecx");
                self.emit("addl $8, %ecx");
                self.emit("movl %ecx, (%rax)");
                // Load the value
                self.emit("movq (%rdx), %rax");
                self.emit(&format!("jmp {}", lend));
                // Stack path: load from overflow_arg_area
                self.label(&lstack);
                self.pop_int("%rax");                   // restore va_list ptr
                self.push_int();                        // keep it on stack
                self.emit("movq 8(%rax), %rdx");       // overflow_arg_area
                // Load the value
                self.emit("movq (%rdx), %rcx");
                // Increment overflow_arg_area by 8
                self.emit("addq $8, %rdx");
                self.emit("movq %rdx, 8(%rax)");
                self.emit("movq %rcx, %rax");
                self.label(&lend);
                // Clean up saved va_list ptr
                self.pop_int("%rcx");
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
            // Narrowing: truncate to smaller integer and re-extend
            (_, CType::Bool) => {
                self.emit("testq %rax, %rax");
                self.emit("setne %al");
                self.emit("movzbl %al, %eax");
            }
            (_, CType::Char(Sign::Signed)) => self.emit("movsbl %al, %eax"),
            (_, CType::Char(Sign::Unsigned)) => self.emit("movzbl %al, %eax"),
            (_, CType::Short(Sign::Signed)) => self.emit("movswl %ax, %eax"),
            (_, CType::Short(Sign::Unsigned)) => self.emit("movzwl %ax, %eax"),
            (_, CType::Int(_)) if Self::is_long_or_ptr(from) => {
                // long/ptr → int: upper 32 bits are discarded, eax already has low 32
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
                self.emit_compound_assign(op, l, r, result_ty);
            }
            Op::And => self.emit_logical_and(l, r),
            Op::Or => self.emit_logical_or(l, r),
            _ if Self::is_float(result_ty) || Self::is_float(l.ty.as_ref().unwrap()) => {
                let operand_ty = l.ty.as_ref().unwrap();
                self.emit_float_binop(op, l, r, operand_ty);
            }
            _ => self.emit_int_binop(op, l, r, result_ty),
        }
    }

    fn emit_assign(&mut self, l: &ExprNode, r: &ExprNode) {
        let lty = l.ty.as_ref().unwrap();
        if let Some((bit_off, bit_width, _field_ty)) = self.get_bitfield_info(l) {
            // Bitfield write: read-modify-write the storage unit
            let mask = (1u64 << bit_width) - 1;
            let is_wide = Self::is_long_or_ptr(lty);
            // Evaluate RHS
            self.emit_expr(r);
            self.push_int(); // save new value
            // Evaluate LHS address (storage unit address)
            self.emit_expr(l);
            self.push_int(); // save address
            // Mask and shift new value into position
            self.emit("movq 8(%rsp), %rcx"); // new value
            if is_wide {
                self.emit(&format!("movabsq ${}, %rdx", mask as i64));
                self.emit("andq %rdx, %rcx");
                if bit_off != 0 {
                    self.emit(&format!("shlq ${}, %rcx", bit_off));
                }
                // Load current storage unit
                self.emit("movq (%rax), %rdx");
                // Clear target bits
                let clear_mask = !(mask << bit_off) as i64;
                self.emit(&format!("movabsq ${}, %rsi", clear_mask));
                self.emit("andq %rsi, %rdx");
            } else {
                self.emit(&format!("andl ${}, %ecx", mask as i32));
                if bit_off != 0 {
                    self.emit(&format!("shll ${}, %ecx", bit_off));
                }
                // Load current storage unit
                self.emit("movl (%rax), %edx");
                // Clear target bits
                let clear_mask = !((mask << bit_off) as u32) as i32;
                self.emit(&format!("andl ${}, %edx", clear_mask));
            }
            // Merge and store
            if is_wide {
                self.emit("orq %rcx, %rdx");
                self.emit("movq %rdx, (%rax)");
            } else {
                self.emit("orl %ecx, %edx");
                self.emit("movl %edx, (%rax)");
            }
            self.pop_int("%rax"); // discard address
            self.pop_int("%rax"); // result = original new value
            return;
        }
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

    fn emit_compound_assign(&mut self, op: Op, l: &ExprNode, r: &ExprNode, _result_ty: &CType) {
        // Bitfield compound assign: read field, compute, write back via bitfield assign path
        if let Some((bit_off, bit_width, field_ty)) = self.get_bitfield_info(l) {
            let mask = (1u64 << bit_width) - 1;
            // Read current bitfield value (extracted)
            self.emit_expr(l); // address of storage unit
            self.emit("movl (%rax), %eax");
            if bit_off != 0 {
                self.emit(&format!("shrl ${}, %eax", bit_off));
            }
            self.emit(&format!("andl ${}, %eax", mask as i32));
            // Sign-extend if signed bitfield
            if !Self::is_unsigned(&field_ty) {
                let shl = 32 - bit_width;
                self.emit(&format!("shll ${}, %eax", shl));
                self.emit(&format!("sarl ${}, %eax", shl));
            }
            self.push_int(); // save current value
            // Evaluate RHS
            self.emit_expr(r);
            self.emit("movq %rax, %rcx"); // r in rcx
            self.pop_int("%rax"); // current value in rax
            // Compute
            let simple_op = match op {
                Op::AddAssign => Op::Add, Op::SubAssign => Op::Sub,
                Op::MulAssign => Op::Mul, Op::DivAssign => Op::Div,
                Op::ModAssign => Op::Mod, Op::ShlAssign => Op::Shl,
                Op::ShrAssign => Op::Shr, Op::BitAndAssign => Op::BitAnd,
                Op::BitOrAssign => Op::BitOr, Op::BitXorAssign => Op::BitXor,
                _ => Op::Add,
            };
            match simple_op {
                Op::Add => self.emit("addl %ecx, %eax"),
                Op::Sub => self.emit("subl %ecx, %eax"),
                Op::Mul => self.emit("imull %ecx, %eax"),
                Op::Shl => self.emit("shll %cl, %eax"),
                Op::Shr => self.emit("shrl %cl, %eax"),
                Op::BitAnd => self.emit("andl %ecx, %eax"),
                Op::BitOr => self.emit("orl %ecx, %eax"),
                Op::BitXor => self.emit("xorl %ecx, %eax"),
                _ => self.emit("addl %ecx, %eax"),
            }
            // Write back: mask, shift, read-modify-write
            self.emit(&format!("andl ${}, %eax", mask as i32));
            self.push_int(); // save new value
            self.emit_expr(l); // address of storage unit
            self.emit("movq %rax, %rdx"); // address in rdx
            self.pop_int("%rcx"); // new value in ecx
            if bit_off != 0 {
                self.emit(&format!("shll ${}, %ecx", bit_off));
            }
            let clear_mask = !((mask << bit_off) as u32) as i32;
            self.emit(&format!("movl (%rdx), %eax"));
            self.emit(&format!("andl ${}, %eax", clear_mask));
            self.emit("orl %ecx, %eax");
            self.emit("movl %eax, (%rdx)");
            return;
        }
        let lty = l.ty.as_ref().unwrap();
        let rty = r.ty.as_ref().unwrap();
        let is_ptr = matches!(lty, CType::Pointer(_));
        // The computation type is r's type (after usual arithmetic conversions in typecheck).
        let comp_float = Self::is_float(rty);
        // Compute the address of l
        self.emit_expr(l);
        self.push_int(); // save address
        // Load current value and widen to computation type if needed
        self.emit_load(lty);
        if comp_float && !Self::is_float(lty) {
            // int lval, float rhs: convert loaded int to float
            self.emit_cast(lty, rty);
        } else if Self::is_float(lty) && Self::is_float(rty) && lty != rty {
            // float → double widening
            self.emit("cvtss2sd %xmm0, %xmm0");
        }
        // Save current value in computation type
        if comp_float {
            self.push_float();
        } else {
            self.push_int();
        }
        // Evaluate r
        self.emit_expr(r);
        // Now: r in rax/xmm0, l_value on stack, l_addr below that
        if comp_float {
            self.emit("movapd %xmm0, %xmm1"); // r in xmm1
            self.pop_float("%xmm0"); // l_value in xmm0
            let suffix = if matches!(rty, CType::Float) { "ss" } else { "sd" };
            match op {
                Op::AddAssign => self.emit(&format!("add{} %xmm1, %xmm0", suffix)),
                Op::SubAssign => self.emit(&format!("sub{} %xmm1, %xmm0", suffix)),
                Op::MulAssign => self.emit(&format!("mul{} %xmm1, %xmm0", suffix)),
                Op::DivAssign => self.emit(&format!("div{} %xmm1, %xmm0", suffix)),
                _ => {}
            }
            // Convert back to lvalue type if needed
            if lty != rty {
                self.emit_cast(rty, lty);
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
            // Sign-extend int index to 64-bit for pointer arithmetic
            if !Self::is_long_or_ptr(r.ty.as_ref().unwrap()) {
                self.emit("movslq %ecx, %rcx");
            }
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
            // Sign-extend int index to 64-bit for pointer arithmetic
            if !Self::is_long_or_ptr(l.ty.as_ref().unwrap()) {
                self.emit("movslq %eax, %rax");
            }
            if let CType::Pointer(inner) = r.ty.as_ref().unwrap() {
                let elem_size = self.type_size(inner);
                if elem_size != 1 {
                    self.emit(&format!("imulq ${}, %rax", elem_size));
                }
            }
        }

        let unsigned = Self::is_unsigned(result_ty);
        let wide = Self::is_long_or_ptr(result_ty) || is_ptr_arith || int_plus_ptr;
        let (sx, r1, r2) = if wide { ("q", "%rcx", "%rax") } else { ("l", "%ecx", "%eax") };

        match op {
            Op::Add => self.emit(&format!("add{} {}, {}", sx, r1, r2)),
            Op::Sub => self.emit(&format!("sub{} {}, {}", sx, r1, r2)),
            Op::Mul => self.emit(&format!("imul{} {}, {}", sx, r1, r2)),
            Op::Div => {
                if unsigned {
                    self.emit("xorl %edx, %edx");
                    self.emit(&format!("div{} {}", sx, r1));
                } else {
                    self.emit(if wide { "cqo" } else { "cdq" });
                    self.emit(&format!("idiv{} {}", sx, r1));
                }
            }
            Op::Mod => {
                if unsigned {
                    self.emit("xorl %edx, %edx");
                    self.emit(&format!("div{} {}", sx, r1));
                } else {
                    self.emit(if wide { "cqo" } else { "cdq" });
                    self.emit(&format!("idiv{} {}", sx, r1));
                }
                self.emit("movq %rdx, %rax");
            }
            Op::BitAnd => self.emit(&format!("and{} {}, {}", sx, r1, r2)),
            Op::BitOr => self.emit(&format!("or{} {}, {}", sx, r1, r2)),
            Op::BitXor => self.emit(&format!("xor{} {}, {}", sx, r1, r2)),
            Op::Shl => self.emit(&format!("shl{} %cl, {}", sx, r2)),
            Op::Shr => {
                if unsigned {
                    self.emit(&format!("shr{} %cl, {}", sx, r2));
                } else {
                    self.emit(&format!("sar{} %cl, {}", sx, r2));
                }
            }
            Op::Eq | Op::Ne | Op::Lt | Op::Gt | Op::Le | Op::Ge => {
                // Use operand type for signedness/width, not result type (which is always int)
                let operand_ty = l.ty.as_ref().unwrap();
                let op_unsigned = Self::is_unsigned(operand_ty);
                let cmp = if Self::is_long_or_ptr(operand_ty) { "cmpq" } else { "cmpl" };
                self.emit(&format!("{} %{}, %{}", cmp,
                    if cmp == "cmpl" { "ecx" } else { "rcx" },
                    if cmp == "cmpl" { "eax" } else { "rax" }));
                let cc = match op {
                    Op::Eq => "sete",
                    Op::Ne => "setne",
                    Op::Lt => if op_unsigned { "setb" } else { "setl" },
                    Op::Gt => if op_unsigned { "seta" } else { "setg" },
                    Op::Le => if op_unsigned { "setbe" } else { "setle" },
                    Op::Ge => if op_unsigned { "setae" } else { "setge" },
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
                } else if Self::is_long_or_ptr(result_ty) {
                    self.emit("negq %rax");
                } else {
                    self.emit("negl %eax");
                }
            }
            UnaryOp::Plus => {
                self.emit_expr(inner);
            }
            UnaryOp::BitNot => {
                self.emit_expr(inner);
                if Self::is_long_or_ptr(result_ty) {
                    self.emit("notq %rax");
                } else {
                    self.emit("notl %eax");
                }
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
        // Inline GCC builtins
        let builtin_name = match func.expr.as_ref() {
            Expr::Var(n) => Some(n.as_str()),
            Expr::FuncToPtr(inner) => if let Expr::Var(n) = inner.expr.as_ref() { Some(n.as_str()) } else { None },
            _ => None,
        };
        if let Some(name) = builtin_name {
            match name {
                "__builtin_bswap16" if args.len() == 1 => {
                    self.emit_expr(&args[0]);
                    self.out.push_str("\trolw $8, %ax\n\tmovzwl %ax, %eax\n");
                    return;
                }
                "__builtin_bswap32" if args.len() == 1 => {
                    self.emit_expr(&args[0]);
                    self.out.push_str("\tbswapl %eax\n");
                    return;
                }
                "__builtin_bswap64" if args.len() == 1 => {
                    self.emit_expr(&args[0]);
                    self.out.push_str("\tbswapq %rax\n");
                    return;
                }
                "__builtin_va_end" => { return; } // no-op on x86-64
                "__builtin_va_start" if args.len() >= 1 => {
                    // Initialize va_list struct for x86-64 ABI.
                    // Unwrap Load to get the address of the va_list struct
                    let va_ptr = match args[0].expr.as_ref() {
                        Expr::Load(inner) | Expr::Decay(inner) => inner,
                        _ => &args[0],
                    };
                    self.emit_expr(va_ptr); // va_list pointer in %rax
                    if let Some((save_offset, named_int)) = self.va_info {
                        let gp_offset = named_int * 8;
                        self.out.push_str(&format!("\tmovl ${}, (%rax)\n", gp_offset)); // gp_offset
                        self.out.push_str("\tmovl $48, 4(%rax)\n"); // fp_offset = 48 (no FP regs used)
                        self.out.push_str("\tleaq 16(%rbp), %rcx\n"); // overflow_arg_area
                        self.out.push_str("\tmovq %rcx, 8(%rax)\n");
                        self.out.push_str(&format!("\tleaq {}(%rbp), %rcx\n", save_offset)); // reg_save_area
                        self.out.push_str("\tmovq %rcx, 16(%rax)\n");
                    } else {
                        // Non-variadic function calling va_start (shouldn't happen, but handle gracefully)
                        self.out.push_str("\tmovl $48, (%rax)\n");
                        self.out.push_str("\tmovl $48, 4(%rax)\n");
                        self.out.push_str("\tleaq 16(%rbp), %rcx\n");
                        self.out.push_str("\tmovq %rcx, 8(%rax)\n");
                        self.out.push_str("\tmovq %rcx, 16(%rax)\n");
                    }
                    return;
                }
                _ => {}
            }
        }
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

        // Pop register args; stack args are already in position
        for (_i, (is_float, idx)) in arg_slots.iter().enumerate() {
            if *is_float {
                if *idx < 8 {
                    self.pop_float(&format!("%xmm{}", idx));
                }
                // else: stack arg, already in position
            } else {
                if *idx < 6 {
                    self.pop_int(int_regs[*idx]);
                }
                // else: stack arg, already in position
            }
        }

        // For variadic functions, set %al to number of xmm args
        // We always set it since we don't easily know if variadic at this point
        self.emit(&format!("movl ${}, %eax", xmm_count.min(8)));

        // Align stack to 16 bytes and call
        if stack_args > 0 {
            // Stack args are positioned relative to rsp; can't use andq.
            // Total on stack = stack_args; pad if odd for 16-byte alignment.
            // (The frame is already 16-aligned, and we pushed an even number
            //  of 8-byte values to get here if stack_args is even.)
            // But pending pushes from enclosing expressions may misalign,
            // so save/restore rsp around the whole sequence.
            // For stack args, we've already done all pushes/pops above.
            // Just align and call.
            self.emit("movq %rsp, %rbx");
            self.emit("andq $-16, %rsp");
            // Re-push stack args onto the aligned stack (reverse order so callee sees correct layout)
            for i in (0..stack_args).rev() {
                self.emit(&format!("pushq {}(%rbx)", i * 8));
            }
            self.emit("callq *%r11");
            self.emit("movq %rbx, %rsp");
            self.emit(&format!("addq ${}, %rsp", stack_args * 8));
        } else {
            self.emit("movq %rsp, %rbx");
            self.emit("andq $-16, %rsp");
            self.emit("callq *%r11");
            self.emit("movq %rbx, %rsp");
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
        if d.is_typedef || Self::is_static_decl(d) || Self::is_extern_decl(d) { return; }
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
                        self.eval_const_init_inner(e).unwrap_or(next_idx as i64) as usize
                    } else {
                        next_idx
                    };
                    let item_offset = base_offset + idx as i32 * elem_size;
                    self.emit_local_store_init(&item.init, elem, item_offset);
                    next_idx = idx + 1;
                }
            }
            CType::Struct(name) | CType::Union(name) => {
                let fields: Vec<(String, i32, CType, Option<(u8, u8)>)> = self.struct_layouts.get(name)
                    .map(|l| l.fields.clone())
                    .unwrap_or_default();
                let mut next_field = 0usize;
                for item in items {
                    let field_idx = if let Some(Designator::Field(fname)) = item.designation.first() {
                        fields.iter().position(|(n, _, _, _)| n == fname).unwrap_or(next_field)
                    } else {
                        next_field
                    };
                    if field_idx < fields.len() {
                        let (_, field_offset, ref field_ty, ref bf_info) = fields[field_idx];
                        if let Some((bit_off, bit_width)) = bf_info {
                            // Bitfield: evaluate value, mask, shift, OR into storage unit
                            let mask = (1u64 << bit_width) - 1;
                            let bit_off = *bit_off;
                            if let Init::Expr(e) = &item.init {
                                self.emit_expr(e);
                                self.emit(&format!("andl ${}, %eax", mask as i32));
                                if bit_off != 0 {
                                    self.emit(&format!("shll ${}, %eax", bit_off));
                                }
                                self.emit("movl %eax, %ecx");
                                self.emit(&format!("leaq {}(%rbp), %rax", base_offset + field_offset));
                                self.emit("orl %ecx, (%rax)");
                            }
                        } else {
                            self.emit_local_store_init(&item.init, field_ty, base_offset + field_offset);
                        }
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
                        let fields: Vec<(String, i32, CType, Option<(u8, u8)>)> = self.struct_layouts.get(name)
                            .map(|l| l.fields.clone())
                            .unwrap_or_default();
                        for (i, item) in items.iter().enumerate() {
                            if i >= fields.len() { break; }
                            let (_, field_offset, ref field_ty, _) = fields[i];
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
        self.statics.clear();
        self.stack_size = 8; // reserve -8(%rbp) for saved %rbx
        self.break_label = None;
        self.continue_label = None;

        // Allocate parameter slots first
        let int_regs = ["%rdi", "%rsi", "%rdx", "%rcx", "%r8", "%r9"];
        let mut param_slots = Vec::new();
        let mut named_int_params = 0usize;
        let mut int_param_count = 0usize;
        let mut xmm_param_count = 0usize;
        let mut stack_arg_offset = 16i32; // first stack arg is at 16(%rbp)
        for p in &f.params {
            if let Some(name) = &p.name {
                let ty = p.ty.as_ref().unwrap().clone();
                let is_float = Self::is_float(&ty);
                let in_reg = if is_float { xmm_param_count < 8 } else { int_param_count < 6 };
                if is_float { xmm_param_count += 1; } else { int_param_count += 1; }
                if in_reg {
                    let size = if matches!(ty, CType::Struct(_) | CType::Union(_)) {
                        self.type_size(&ty).max(8)
                    } else {
                        8
                    };
                    self.stack_size += size;
                    let offset = -self.stack_size;
                    self.locals.insert(name.clone(), offset);
                } else {
                    // Stack-passed arg: lives in caller's frame
                    self.locals.insert(name.clone(), stack_arg_offset);
                    stack_arg_offset += 8;
                }
                if !is_float { named_int_params += 1; }
                param_slots.push((name.clone(), *self.locals.get(name).unwrap(), ty));
            }
        }

        // For variadic functions, allocate register save area (6 * 8 = 48 bytes)
        if f.variadic {
            self.stack_size += 48;
            let offset = -self.stack_size; // start of save area
            self.va_info = Some((offset, named_int_params));
        } else {
            self.va_info = None;
        }
        let va_save_area_offset = self.va_info.map(|(off, _)| off);

        // Allocate local variable slots
        self.alloc_locals(&f.body);

        // Align stack: push %rbx adds 8, so need (stack_size + 8) ≡ 0 (mod 16)
        self.stack_size = ((self.stack_size + 8 + 15) & !15) - 8;

        // Emit function header
        let is_static = f.return_specs.iter().any(|s| matches!(s, DeclSpec::Storage(StorageClass::Static)));
        if !is_static {
            self.out.push_str(&format!("\t.globl {}\n", f.name));
        }
        self.out.push_str(&format!("\t.type {}, @function\n", f.name));
        self.label(&f.name);

        // Prologue
        self.emit("pushq %rbp");
        self.emit("movq %rsp, %rbp");
        self.emit("pushq %rbx");
        if self.stack_size > 0 {
            self.emit(&format!("subq ${}, %rsp", self.stack_size));
        }

        // Copy parameters from registers to stack.
        // First pass: save all register args to temp slots (struct pointers too).
        // We need a temp slot for struct pointer args since rep movsb clobbers rdi/rsi/rcx.
        let mut int_idx = 0usize;
        let mut xmm_idx = 0usize;
        let mut struct_copies = Vec::new();
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
            } else if matches!(ty, CType::Struct(_) | CType::Union(_)) {
                if int_idx < 6 {
                    // Save source pointer to the end of the struct's local slot (last 8 bytes)
                    // We'll overwrite it with the actual copy below.
                    self.emit(&format!("movq {}, {}(%rbp)", int_regs[int_idx], offset));
                    let size = type_layout(ty, &self.struct_layouts).size();
                    struct_copies.push((*offset, size));
                    int_idx += 1;
                }
            } else {
                if int_idx < 6 {
                    self.emit(&format!("movq {}, {}(%rbp)", int_regs[int_idx], offset));
                    int_idx += 1;
                }
            }
        }
        // Second pass: copy struct data from saved pointers
        for (offset, size) in struct_copies {
            self.emit(&format!("movq {}(%rbp), %rsi", offset)); // source pointer
            self.emit(&format!("leaq {}(%rbp), %rdi", offset)); // dest = local slot
            self.emit(&format!("movl ${}, %ecx", size));
            self.emit("rep movsb");
        }

        // For variadic functions, save remaining register args to the save area
        if let Some(save_offset) = va_save_area_offset {
            for i in 0..6 {
                self.emit(&format!("movq {}, {}(%rbp)", int_regs[i], save_offset + (i as i32) * 8));
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
        self.emit("movq -8(%rbp), %rbx");
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
                    if matches!(ty, CType::Pointer(_)) {
                        // Pointer to string: add to string pool, emit quad reference
                        let idx = self.strings.len();
                        self.strings.push(s.to_string());
                        self.out.push_str(&format!("\t.quad .Lstr_{}\n", idx));
                    } else {
                        self.emit_string_data(s, size);
                    }
                } else if let Some(val) = self.eval_const_init_inner(e) {
                    self.emit_data_value(size, val);
                } else if let Some(sym) = extract_global_addr(e) {
                    self.out.push_str(&format!("\t.quad {}\n", sym));
                } else if let Some((cl_ty, cl_items)) = extract_compound_literal_addr(e) {
                    let label = format!(".Lcl_{}", self.label_count);
                    self.label_count += 1;
                    self.out.push_str(&format!("\t.quad {}\n", label));
                    self.static_defs.push((label, cl_ty, Some(Init::List(cl_items))));
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
                let fields: Vec<(String, i32, CType, Option<(u8, u8)>)> = self.struct_layouts.get(name)
                    .map(|l| l.fields.clone())
                    .unwrap_or_default();
                let total_size = self.struct_layouts.get(name).map_or(0, |l| l.size);

                // Check for brace elision: more flat scalar items than fields
                let has_designators = items.iter().any(|i| !i.designation.is_empty());
                let has_sublists = items.iter().any(|i| matches!(i.init, Init::List(_)));
                let flat_scalars = !has_designators && !has_sublists && items.len() > fields.len();

                if flat_scalars {
                    // Brace elision: consume items sequentially across fields,
                    // with array fields consuming multiple items
                    let mut item_idx = 0usize;
                    let mut offset = 0;
                    for (_, field_offset, field_ty, _) in &fields {
                        if *field_offset > offset {
                            self.out.push_str(&format!("\t.zero {}\n", field_offset - offset));
                        }
                        let field_size = self.type_size(field_ty);
                        if let CType::Array(elem, count) = field_ty {
                            let n = count.unwrap_or(0) as usize;
                            let esz = self.type_size(elem);
                            for _ in 0..n {
                                if item_idx < items.len() {
                                    self.emit_init_item(&items[item_idx].init, elem, esz);
                                    item_idx += 1;
                                } else {
                                    self.out.push_str(&format!("\t.zero {}\n", esz));
                                }
                            }
                        } else if item_idx < items.len() {
                            self.emit_init_item(&items[item_idx].init, field_ty, field_size);
                            item_idx += 1;
                        } else {
                            self.out.push_str(&format!("\t.zero {}\n", field_size));
                        }
                        offset = field_offset + field_size;
                    }
                    if offset < total_size {
                        self.out.push_str(&format!("\t.zero {}\n", total_size - offset));
                    }
                } else {
                    // Normal: one init item per field, with designator support
                    let mut field_inits: Vec<Option<&Init>> = vec![None; fields.len()];
                    let mut next_field = 0usize;
                    for item in items {
                        let field_idx = if let Some(Designator::Field(fname)) = item.designation.first() {
                            fields.iter().position(|(n, _, _, _)| n == fname).unwrap_or(next_field)
                        } else {
                            next_field
                        };
                        if field_idx < fields.len() {
                            field_inits[field_idx] = Some(&item.init);
                            next_field = field_idx + 1;
                        }
                    }

                    let mut offset = 0;
                    for (i, (_, field_offset, field_ty, _)) in fields.iter().enumerate() {
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
            }
            CType::Array(elem, count) => {
                let elem_size = self.type_size(elem);
                let n = count.unwrap_or(items.len() as u64) as usize;

                // Detect brace elision: flat scalar inits for array of structs
                let slots = self.scalar_init_slots(elem);
                if slots > 1 && items.len() > n && items.iter().all(|i| i.designation.is_empty() && matches!(i.init, Init::Expr(_))) {
                    // Brace elision: group flat items into struct-sized chunks
                    for i in 0..n {
                        let start = i * slots;
                        let end = (start + slots).min(items.len());
                        if start < items.len() {
                            let chunk = &items[start..end];
                            self.emit_init_list(elem, chunk);
                        } else {
                            self.out.push_str(&format!("\t.zero {}\n", elem_size));
                        }
                    }
                } else {
                    // Normal: each init item is one array element
                    let mut elem_inits: Vec<Option<&Init>> = vec![None; n];
                    let mut next_idx = 0usize;
                    for item in items {
                        let idx = if let Some(Designator::Index(e)) = item.designation.first() {
                            self.eval_const_init_inner(e).unwrap_or(next_idx as i64) as usize
                        } else {
                            next_idx
                        };
                        if idx < n {
                            elem_inits[idx] = Some(&item.init);
                            next_idx = idx + 1;
                        }
                    }

                    for i in 0..n {
                        if let Some(init) = elem_inits[i] {
                            self.emit_init_item(init, elem, elem_size);
                        } else {
                            self.out.push_str(&format!("\t.zero {}\n", elem_size));
                        }
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
    let size: usize = match ty {
        CType::Void => 0,
        CType::Bool | CType::Char(_) => 1,
        CType::Short(_) => 2,
        CType::Int(_) | CType::Float | CType::Enum(_) => 4,
        CType::Long(_) | CType::LongLong(_) | CType::Double
        | CType::Pointer(_) | CType::LongDouble => 8,
        CType::Array(elem, Some(n)) => {
            let elem_layout = type_layout(elem, layouts);
            return Layout::from_size_align(
                elem_layout.size() * *n as usize,
                elem_layout.align(),
            ).unwrap();
        }
        CType::Array(elem, None) => {
            let elem_layout = type_layout(elem, layouts);
            return Layout::from_size_align(8, elem_layout.align().max(1)).unwrap();
        }
        CType::Struct(name) | CType::Union(name) => {
            return layouts.get(name)
                .map_or(Layout::new::<()>(), |l| Layout::from_size_align(l.size as usize, l.align as usize).unwrap());
        }
        _ => 8,
    };
    let align = size.min(8).max(1).next_power_of_two();
    Layout::from_size_align(size, align).unwrap()
}

fn compute_struct_layout(
    fields: &[(String, CType, Option<u8>)],
    layouts: &HashMap<String, StructLayout>,
    is_union: bool,
) -> StructLayout {
    let mut result = Vec::new();
    let mut compound = Layout::from_size_align(0, 1).unwrap();
    // Bitfield packing state: (storage_unit_byte_offset, storage_unit_size_bits, next_bit_offset, storage_type)
    let mut bf_state: Option<(usize, u8, u8, CType)> = None;
    for (fname, fty, bf_width) in fields {
        let field_layout = type_layout(fty, layouts);
        if is_union {
            if let Some(width) = bf_width {
                // Union bitfields: all start at bit 0 of offset 0
                result.push((fname.clone(), 0, fty.clone(), Some((0, *width))));
                compound = Layout::from_size_align(
                    compound.size().max(field_layout.size()),
                    compound.align().max(field_layout.align()),
                ).unwrap();
            } else {
                result.push((fname.clone(), 0, fty.clone(), None));
                compound = Layout::from_size_align(
                    compound.size().max(field_layout.size()),
                    compound.align().max(field_layout.align()),
                ).unwrap();
            }
        } else if let Some(width) = bf_width {
            let storage_bits = (field_layout.size() * 8) as u8;
            // Try to pack into current storage unit
            if let Some((unit_offset, unit_bits, next_bit, ref unit_ty)) = bf_state {
                if storage_bits == unit_bits && next_bit + width <= unit_bits {
                    // Fits in current unit
                    result.push((fname.clone(), unit_offset as i32, fty.clone(), Some((next_bit, *width))));
                    bf_state = Some((unit_offset, unit_bits, next_bit + width, unit_ty.clone()));
                    continue;
                }
            }
            // Start a new storage unit
            // Align for new storage unit
            let (new_layout, offset) = compound.extend(field_layout).unwrap();
            result.push((fname.clone(), offset as i32, fty.clone(), Some((0, *width))));
            compound = new_layout;
            bf_state = Some((offset, storage_bits, *width, fty.clone()));
        } else {
            // Non-bitfield — close bitfield state
            bf_state = None;
            let (new_layout, offset) = compound.extend(field_layout).unwrap();
            result.push((fname.clone(), offset as i32, fty.clone(), None));
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
                for (_, fty, _) in fields.iter() {
                    let inner = match fty {
                        CType::Array(elem, _) => elem.as_ref(),
                        other => other,
                    };
                    if let CType::Struct(n) | CType::Union(n) = inner {
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
    let deferred_compound_literals: Vec<(String, CType, Vec<InitItem>)> = Vec::new();
    for d in &unit.decls {
        if d.is_typedef { continue; }
        for id in &d.declarators {
            if !emitted.insert(id.name.clone()) { continue; }
            let Some(&(_decl, id)) = global_decls.get(&id.name) else { continue };
            let ty = id.ty.as_ref().unwrap();
            if matches!(ty, CType::Function { .. }) { continue; }
            let size = cg.type_size(ty);
            if size == 0 { continue; }
            let is_static = d.specs.iter().any(|s| matches!(s, DeclSpec::Storage(StorageClass::Static)));
            cg.out.push_str(&format!("\t.data\n"));
            if !is_static {
                cg.out.push_str(&format!("\t.globl {}\n", id.name));
            }
            cg.out.push_str(&format!("{}:\n", id.name));
            match &id.init {
                Some(Init::Expr(e)) => {
                    if let Some(s) = extract_string_init(e) {
                        cg.emit_string_data(s, size);
                    } else {
                        cg.emit_init_item(&Init::Expr(e.clone()), ty, size);
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

    // Emit deferred compound literals (file-scope compound literals used via &)
    for (label, ty, items) in &deferred_compound_literals {
        cg.out.push_str("\t.data\n");
        cg.out.push_str(&format!("{}:\n", label));
        cg.emit_init_list(ty, items);
    }

    // Emit static local variables
    let static_defs = std::mem::take(&mut cg.static_defs);
    for (label, ty, init) in &static_defs {
        let size = cg.type_size(&ty);
        if size == 0 { continue; }
        cg.out.push_str("\t.data\n");
        cg.out.push_str(&format!("{}:\n", label));
        match init {
            Some(init) => {
                cg.emit_init_item(init, &ty, size);
            }
            None => {
                cg.out.push_str(&format!("\t.zero {}\n", size));
            }
        }
    }

    // Emit all string literals (from code + from global/static initializers)
    if !cg.strings.is_empty() {
        cg.out.push_str("\t.section .rodata\n");
        for (i, s) in cg.strings.iter().enumerate() {
            cg.out.push_str(&format!(".Lstr_{}:\n", i));
            cg.out.push_str(&format!("\t.string \"{}\"\n", s));
        }
    }

    cg.out
}

/// Evaluate a constant initializer expression, returning the raw bits as i64.
/// For float/double types, returns IEEE 754 bit representation.
impl Codegen {
fn eval_const_init_inner(&self, e: &ExprNode) -> Option<i64> {
    Self::eval_const_init_static(e, &self.struct_layouts)
}

fn eval_const_init_static(e: &ExprNode, struct_layouts: &HashMap<String, StructLayout>) -> Option<i64> {
    let rec = |e: &ExprNode| Self::eval_const_init_static(e, struct_layouts);
    let ty = e.ty.as_ref();
    let is_float_ty = matches!(ty, Some(CType::Float | CType::Double | CType::LongDouble));

    match e.expr.as_ref() {
        Expr::Constant(s) => {
            if is_float_ty {
                let s_clean = s.trim_end_matches(|c: char| c == 'f' || c == 'F' || c == 'l' || c == 'L');
                let fval: f64 = s_clean.parse().unwrap_or(0.0);
                if matches!(ty, Some(CType::Float)) {
                    Some((fval as f32).to_bits() as i32 as i64)
                } else {
                    Some(fval.to_bits() as i64)
                }
            } else {
                Some(parse_int_constant(s))
            }
        }
        Expr::Cast(_, inner) | Expr::ImplicitCast(_, inner) => {
            let inner_val = rec(inner)?;
            let inner_ty = inner.ty.as_ref();
            let inner_is_float = matches!(inner_ty, Some(CType::Float | CType::Double | CType::LongDouble));
            if is_float_ty && !inner_is_float {
                if matches!(ty, Some(CType::Float)) {
                    Some((inner_val as f32).to_bits() as i32 as i64)
                } else {
                    Some((inner_val as f64).to_bits() as i64)
                }
            } else if !is_float_ty && inner_is_float {
                if matches!(inner_ty, Some(CType::Float)) {
                    Some(f32::from_bits(inner_val as u32) as i64)
                } else {
                    Some(f64::from_bits(inner_val as u64) as i64)
                }
            } else if is_float_ty && inner_is_float {
                if matches!(inner_ty, Some(CType::Float)) && !matches!(ty, Some(CType::Float)) {
                    let fval = f32::from_bits(inner_val as u32) as f64;
                    Some(fval.to_bits() as i64)
                } else if !matches!(inner_ty, Some(CType::Float)) && matches!(ty, Some(CType::Float)) {
                    let fval = f64::from_bits(inner_val as u64) as f32;
                    Some(fval.to_bits() as i32 as i64)
                } else {
                    Some(inner_val)
                }
            } else {
                Some(inner_val)
            }
        }
        Expr::SizeofExpr(inner) => {
            let t = inner.ty.as_ref()?;
            Some(type_size_static(t, struct_layouts) as i64)
        }
        Expr::SizeofType(t) => {
            Some(type_size_static(t, struct_layouts) as i64)
        }
        Expr::UnaryOp(UnaryOp::Plus, inner) => rec(inner),
        Expr::UnaryOp(UnaryOp::Neg, inner) => {
            let v = rec(inner)?;
            if is_float_ty {
                if matches!(ty, Some(CType::Float)) {
                    Some((-f32::from_bits(v as u32)).to_bits() as i32 as i64)
                } else {
                    Some((-f64::from_bits(v as u64)).to_bits() as i64)
                }
            } else {
                Some(-v)
            }
        }
        Expr::UnaryOp(UnaryOp::BitNot, inner) => rec(inner).map(|v| !v),
        Expr::BinOp(op, l, r) => {
            let l = rec(l)?;
            let r = rec(r)?;
            Some(match op {
                Op::Add => l.wrapping_add(r),
                Op::Sub => l.wrapping_sub(r),
                Op::Mul => l.wrapping_mul(r),
                Op::Div => l.checked_div(r).unwrap_or(0),
                Op::Mod => l.checked_rem(r).unwrap_or(0),
                Op::BitAnd => l & r,
                Op::BitOr => l | r,
                Op::BitXor => l ^ r,
                Op::Shl => l.wrapping_shl(r as u32),
                Op::Shr => l.wrapping_shr(r as u32),
                _ => return None,
            })
        }
        _ => None,
    }
}
} // impl CodeGen

fn type_size_static(ty: &CType, struct_layouts: &HashMap<String, StructLayout>) -> i32 {
    match ty {
        CType::Void => 0,
        CType::Bool | CType::Char(_) => 1,
        CType::Short(_) => 2,
        CType::Int(_) | CType::Float | CType::Enum(_) => 4,
        CType::Long(_) | CType::LongLong(_) | CType::Double
        | CType::Pointer(_) | CType::LongDouble => 8,
        CType::Array(elem, Some(n)) => type_size_static(elem, struct_layouts) * *n as i32,
        CType::Array(_, None) => 8,
        CType::Function { .. } => 0,
        CType::Struct(name) | CType::Union(name) => {
            struct_layouts.get(name).map_or(0, |l| l.size)
        }
        CType::Typedef(_) => panic!("ICE: unresolved typedef"),
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

/// Extract a global symbol address from an initializer (e.g., &x, func_name, &arr[N]).
fn extract_global_addr(e: &ExprNode) -> Option<String> {
    match e.expr.as_ref() {
        Expr::UnaryOp(UnaryOp::AddrOf, inner) => match inner.expr.as_ref() {
            Expr::Var(name) => Some(name.clone()),
            Expr::Load(inner2) => match inner2.expr.as_ref() {
                Expr::Var(name) => Some(name.clone()),
                _ => None,
            },
            Expr::Index(base, idx) => {
                // &array[N] → symbol + N * elem_size
                let sym = extract_global_addr(base)?;
                let n = extract_const_int(idx)?;
                let elem_size = match base.ty.as_ref()? {
                    CType::Pointer(inner) => simple_type_size(inner)?,
                    _ => return None,
                };
                let byte_offset = n * elem_size;
                if byte_offset == 0 {
                    Some(sym)
                } else {
                    Some(format!("{}+{}", sym, byte_offset))
                }
            }
            _ => None,
        },
        Expr::Var(name) => Some(name.clone()),
        Expr::Cast(_, inner) | Expr::ImplicitCast(_, inner)
        | Expr::Decay(inner) | Expr::FuncToPtr(inner) => extract_global_addr(inner),
        Expr::BinOp(Op::Add, base, offset) => {
            // Handle global_ptr + const (e.g., &array[N] → array + N*elem_size)
            let sym = extract_global_addr(base)?;
            let n = extract_const_int(offset)?;
            let elem_size = match base.ty.as_ref()? {
                CType::Pointer(inner) => simple_type_size(inner)?,
                _ => return None,
            };
            let byte_offset = n * elem_size;
            if byte_offset == 0 {
                Some(sym)
            } else {
                Some(format!("{}+{}", sym, byte_offset))
            }
        }
        _ => None,
    }
}

fn extract_const_int(e: &ExprNode) -> Option<i64> {
    Codegen::eval_const_init_static(e, &HashMap::new())
}

fn simple_type_size(ty: &CType) -> Option<i64> {
    match ty {
        CType::Bool | CType::Char(_) => Some(1),
        CType::Short(_) => Some(2),
        CType::Int(_) | CType::Float | CType::Enum(_) => Some(4),
        CType::Long(_) | CType::LongLong(_) | CType::Double
        | CType::Pointer(_) | CType::LongDouble => Some(8),
        CType::Void | CType::Array(..) | CType::Function { .. }
        | CType::Struct(_) | CType::Union(_) => None,
        CType::Typedef(_) => panic!("ICE: unresolved typedef"),
    }
}

/// Extract a compound literal from an address-of expression for file-scope init.
/// Returns (type, init_items) if the expression is &(Type){...} (possibly through casts).
fn extract_compound_literal_addr(e: &ExprNode) -> Option<(CType, Vec<InitItem>)> {
    match e.expr.as_ref() {
        Expr::UnaryOp(UnaryOp::AddrOf, inner) => extract_compound_literal(inner),
        Expr::Cast(_, inner) | Expr::ImplicitCast(_, inner) => extract_compound_literal_addr(inner),
        _ => None,
    }
}

fn extract_compound_literal(e: &ExprNode) -> Option<(CType, Vec<InitItem>)> {
    match e.expr.as_ref() {
        Expr::CompoundLiteral(_tn, items) => {
            let ty = e.ty.as_ref().cloned()?;
            Some((ty, items.clone()))
        }
        Expr::Load(inner) => extract_compound_literal(inner),
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

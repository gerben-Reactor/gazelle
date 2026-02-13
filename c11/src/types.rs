use crate::ast::*;

fn eval_const_size(e: &ExprNode) -> Option<u64> {
    match e.expr.as_ref() {
        Expr::Constant(s) => {
            let s = s.to_ascii_lowercase();
            let s = s.trim_end_matches(|c: char| c == 'u' || c == 'l');
            if s.starts_with("0x") {
                u64::from_str_radix(&s[2..], 16).ok()
            } else {
                s.parse().ok()
            }
        }
        Expr::BinOp(op, l, r) => {
            let l = eval_const_size(l)?;
            let r = eval_const_size(r)?;
            Some(match op {
                Op::Add => l.wrapping_add(r),
                Op::Sub => l.wrapping_sub(r),
                Op::Mul => l.wrapping_mul(r),
                Op::Div => l.checked_div(r)?,
                Op::Mod => l.checked_rem(r)?,
                Op::Shl => l.wrapping_shl(r as u32),
                Op::Shr => l.wrapping_shr(r as u32),
                _ => return None,
            })
        }
        Expr::UnaryOp(UnaryOp::Plus, inner) => eval_const_size(inner),
        Expr::Cast(_, inner) | Expr::ImplicitCast(_, inner) => eval_const_size(inner),
        _ => None,
    }
}

/// Resolve declaration specifiers into a base CType per C11 6.7.2.
pub fn resolve_specs(specs: &[DeclSpec]) -> Result<CType, String> {
    let mut void = false;
    let mut bool_ = false;
    let mut char_ = false;
    let mut short = 0u8;
    let mut long = 0u8;
    let mut float_ = false;
    let mut double_ = false;
    let mut signed = false;
    let mut unsigned = false;
    let mut tag: Option<CType> = None;

    for spec in specs {
        match spec {
            DeclSpec::Type(ts) => match ts {
                TypeSpec::Void => void = true,
                TypeSpec::Bool => bool_ = true,
                TypeSpec::Char => char_ = true,
                TypeSpec::Short => short += 1,
                TypeSpec::Int => {}
                TypeSpec::Long => long += 1,
                TypeSpec::Float => float_ = true,
                TypeSpec::Double => double_ = true,
                TypeSpec::Signed => signed = true,
                TypeSpec::Unsigned => unsigned = true,
                TypeSpec::Struct(sou, ss) => {
                    let name = ss.name.clone().unwrap_or_default();
                    tag = Some(match sou {
                        StructOrUnion::Struct => CType::Struct(name),
                        StructOrUnion::Union => CType::Union(name),
                    });
                }
                TypeSpec::Enum(es) => {
                    tag = Some(CType::Enum(es.name.clone().unwrap_or_default()));
                }
                TypeSpec::TypedefName(name) => {
                    tag = Some(CType::Typedef(name.clone()));
                }
                TypeSpec::Atomic(_) | TypeSpec::Complex => {}
            },
            _ => {}
        }
    }

    if let Some(t) = tag { return Ok(t); }
    if void { return Ok(CType::Void); }
    if bool_ { return Ok(CType::Bool); }

    if signed && unsigned {
        return Err("both signed and unsigned".into());
    }
    let sign = if unsigned { Sign::Unsigned } else { Sign::Signed };

    if char_ { return Ok(CType::Char(sign)); }
    if float_ { return Ok(CType::Float); }
    if double_ && long >= 1 { return Ok(CType::LongDouble); }
    if double_ { return Ok(CType::Double); }
    if short >= 1 { return Ok(CType::Short(sign)); }
    if long >= 2 { return Ok(CType::LongLong(sign)); }
    if long == 1 { return Ok(CType::Long(sign)); }

    // bare `signed`, `unsigned`, `int`, or implicit int
    Ok(CType::Int(sign))
}

/// Apply derived types (pointer, array, function) to a base type.
pub fn apply_derived(mut ty: CType, derived: &[DerivedType]) -> Result<CType, String> {
    for d in derived.iter().rev() {
        ty = match d {
            DerivedType::Pointer => CType::Pointer(Box::new(ty)),
            DerivedType::Array(size_expr) => {
                let size = size_expr.as_ref().and_then(eval_const_size);
                CType::Array(Box::new(ty), size)
            }
            DerivedType::Function(params, variadic) => {
                let param_types: Vec<CType> = params.iter()
                    .map(|p| resolve_type(&p.specs, &p.derived))
                    .collect::<Result<_, _>>()?;
                CType::Function {
                    ret: Box::new(ty),
                    params: param_types,
                    variadic: *variadic,
                }
            }
        };
    }
    Ok(ty)
}

/// Resolve specifiers + derived type list into a fully wrapped CType.
pub fn resolve_type(specs: &[DeclSpec], derived: &[DerivedType]) -> Result<CType, String> {
    apply_derived(resolve_specs(specs)?, derived)
}

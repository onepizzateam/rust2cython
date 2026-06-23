use crate::ir::{PrimKind, TypeRef};

pub fn to_cython_type(ty: &crate::ir::TypeRef) -> String {
    match ty {
        TypeRef::Primitive(
            PrimKind::I8
            | PrimKind::I16
            | PrimKind::I32
            | PrimKind::U8
            | PrimKind::U16
            | PrimKind::U32
            | PrimKind::Usize,
        ) => "int".to_string(),
        TypeRef::Primitive(PrimKind::I64 | PrimKind::U64) => "long long".to_string(),
        TypeRef::Primitive(PrimKind::F32) => "float".to_string(),
        TypeRef::Primitive(PrimKind::F64) => "double".to_string(),
        TypeRef::Primitive(PrimKind::Bool) => "bint".to_string(),
        TypeRef::Str => "bytes".to_string(),
        TypeRef::Vec(inner) => {
            match &**inner {
                TypeRef::Primitive(
                    PrimKind::I8
                    | PrimKind::I16
                    | PrimKind::I32
                    | PrimKind::I64
                    | PrimKind::U8
                    | PrimKind::U16
                    | PrimKind::U32
                    | PrimKind::U64
                    | PrimKind::Usize
                    | PrimKind::F32
                    | PrimKind::F64,
                ) => {
                    // use typed memoryview
                    let t = to_cython_type(inner);
                    format!("{}[:]", t)
                }
                _ => "list".to_string(),
            }
        }
        TypeRef::Option(inner) => to_cython_type(inner),
        TypeRef::Result(ok, _err) => to_cython_type(ok),
        TypeRef::Named(s) => s.clone(),
        TypeRef::Ptr(_) => "void*".to_string(),
        TypeRef::Void => "void".to_string(),
    }
}

pub fn needs_wrapper(ty: &crate::ir::TypeRef) -> bool {
    matches!(
        ty,
        TypeRef::Str | TypeRef::Vec(_) | TypeRef::Option(_) | TypeRef::Result(_, _)
    )
}

pub fn python_return_expr(ty: &crate::ir::TypeRef, raw_expr: &str) -> String {
    match ty {
        TypeRef::Result(_, _) => raw_expr.to_string(),
        TypeRef::Option(_) => format!("None if {} is NULL else {}", raw_expr, raw_expr),
        TypeRef::Str => format!("{}.decode('utf-8')", raw_expr),
        _ => raw_expr.to_string(),
    }
}

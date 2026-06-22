use crate::ir::{TypeRef, PrimKind};

pub fn to_cython_type(ty: &crate::ir::TypeRef) -> String {
    match ty {
        TypeRef::Primitive(k) => match k {
            PrimKind::I8 | PrimKind::I16 | PrimKind::I32 | PrimKind::U8 | PrimKind::U16 | PrimKind::U32 | PrimKind::Usize => "int".to_string(),
            PrimKind::I64 | PrimKind::U64 => "long long".to_string(),
            PrimKind::F32 => "float".to_string(),
            PrimKind::F64 => "double".to_string(),
            PrimKind::Bool => "bint".to_string(),
        },
        TypeRef::Str => "bytes".to_string(),
        TypeRef::Vec(inner) => {
            match &**inner {
                TypeRef::Primitive(p) => match p {
                    PrimKind::I8 | PrimKind::I16 | PrimKind::I32 | PrimKind::I64 | PrimKind::U8 | PrimKind::U16 | PrimKind::U32 | PrimKind::U64 | PrimKind::Usize | PrimKind::F32 | PrimKind::F64 => {
                        // use typed memoryview
                        let t = to_cython_type(&*inner);
                        format!("{}[:]", t)
                    }
                    _ => "list".to_string(),
                },
                _ => "list".to_string(),
            }
        }
        TypeRef::Option(inner) => to_cython_type(&*inner),
        TypeRef::Result(ok, _err) => to_cython_type(&*ok),
        TypeRef::Named(s) => s.clone(),
        TypeRef::Ptr(_) => "void*".to_string(),
        TypeRef::Void => "void".to_string(),
    }
}

pub fn needs_wrapper(ty: &crate::ir::TypeRef) -> bool {
    match ty {
        TypeRef::Str => true,
        TypeRef::Vec(_) => true,
        TypeRef::Option(_) => true,
        TypeRef::Result(_, _) => true,
        _ => false,
    }
}

pub fn python_return_expr(ty: &crate::ir::TypeRef, raw_expr: &str) -> String {
    match ty {
        TypeRef::Result(_, _) => format!("raise_if_err({})", raw_expr),
        TypeRef::Option(_) => format!("None if {} is NULL else {}", raw_expr, raw_expr),
        TypeRef::Str => format!("{}.decode('utf-8')", raw_expr),
        _ => raw_expr.to_string(),
    }
}

use crate::ir::{FfiType, FnDef, ShimFn, ShimParam, TypeRef};

pub fn plan_shim(fn_def: &FnDef) -> ShimFn {
    // helper to map parameter types
    fn map_param_ty(ty: &TypeRef) -> FfiType {
        match ty {
            TypeRef::Primitive(_) => FfiType::Direct(ty.clone()),
            TypeRef::Str => FfiType::CStr,
            TypeRef::Vec(inner) => {
                let inner_ty = *inner.clone();
                match inner_ty {
                    TypeRef::Primitive(_) => FfiType::SlicePtr { inner: inner_ty },
                    TypeRef::Str => FfiType::StringSlicePtr,
                    _ => FfiType::Unsupported("Vec<non-primitive> not supported".into()),
                }
            }
            TypeRef::Option(inner) => {
                let inner_ty = *inner.clone();
                match inner_ty {
                    TypeRef::Primitive(_) => FfiType::OptionPtr { inner: inner_ty },
                    _ => FfiType::Unsupported("Option<non-primitive> not supported".into()),
                }
            }
            TypeRef::Named(_) => FfiType::Direct(ty.clone()),
            TypeRef::Result(_, _) => FfiType::Unsupported("Result as param not supported".into()),
            other => FfiType::Unsupported(format!("unsupported param type: {:?}", other)),
        }
    }

    // helper to map return types
    fn map_return_ty(ty: &TypeRef) -> FfiType {
        match ty {
            TypeRef::Primitive(_) => FfiType::Direct(ty.clone()),
            TypeRef::Void => FfiType::Direct(ty.clone()),
            TypeRef::Str => FfiType::CStr,
            TypeRef::Vec(inner) => {
                let inner_ty = *inner.clone();
                match inner_ty {
                    TypeRef::Primitive(_) => FfiType::SliceOut { inner: inner_ty },
                    TypeRef::Str => FfiType::StringArrayOut,
                    _ => FfiType::Unsupported("Vec<non-primitive> not supported".into()),
                }
            }
            TypeRef::Option(inner) => {
                let inner_ty = *inner.clone();
                match inner_ty {
                    TypeRef::Primitive(_) => FfiType::OptionPtr { inner: inner_ty },
                    _ => FfiType::Unsupported("Option<non-primitive> not supported".into()),
                }
            }
            TypeRef::Named(_) => FfiType::Direct(ty.clone()),
            TypeRef::Result(ok, _err) => {
                let ok_ty = *ok.clone();
                match ok_ty {
                    TypeRef::Primitive(_) => FfiType::ResultWithErrOut { ok: ok_ty },
                    _ => FfiType::Unsupported(format!("unsupported return type: {:?}", ty)),
                }
            }
            other => FfiType::Unsupported(format!("unsupported return type: {:?}", other)),
        }
    }

    let params = fn_def
        .params
        .iter()
        .map(|p| ShimParam {
            name: p.name.clone(),
            original_ty: p.ty.clone(),
            ffi_ty: map_param_ty(&p.ty),
        })
        .collect();

    let ffi_ret = map_return_ty(&fn_def.ret);

    ShimFn {
        original_name: fn_def.name.clone(),
        shim_name: fn_def.name.clone(),
        params,
        ret: fn_def.ret.clone(),
        ffi_ret,
    }
}

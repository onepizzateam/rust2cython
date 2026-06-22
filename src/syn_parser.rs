use anyhow::Context;

pub fn parse_rust_file(path: &std::path::Path) -> anyhow::Result<crate::ir::Module> {
    let src = std::fs::read_to_string(path).with_context(|| format!("reading file {}", path.display()))?;
    let file = syn::parse_file(&src).with_context(|| format!("parsing file {}", path.display()))?;

    fn extract_doc(attrs: &[syn::Attribute]) -> Option<String> {
        let mut docs = Vec::new();
        for attr in attrs {
            if attr.path().is_ident("doc") {
                if let Ok(litstr) = attr.parse_args::<syn::LitStr>() {
                    docs.push(litstr.value());
                }
            }
        }
        if docs.is_empty() { None } else { Some(docs.join("\n")) }
    }

    fn convert(ty: &syn::Type) -> crate::ir::TypeRef {
        use crate::ir::{TypeRef, PrimKind};
        match ty {
            syn::Type::Path(tp) if tp.qself.is_none() => {
                if let Some(seg) = tp.path.segments.last() {
                    let ident = seg.ident.to_string();
                    match ident.as_str() {
                        "i8" => TypeRef::Primitive(PrimKind::I8),
                        "i16" => TypeRef::Primitive(PrimKind::I16),
                        "i32" => TypeRef::Primitive(PrimKind::I32),
                        "i64" => TypeRef::Primitive(PrimKind::I64),
                        "u8" => TypeRef::Primitive(PrimKind::U8),
                        "u16" => TypeRef::Primitive(PrimKind::U16),
                        "u32" => TypeRef::Primitive(PrimKind::U32),
                        "u64" => TypeRef::Primitive(PrimKind::U64),
                        "f32" => TypeRef::Primitive(PrimKind::F32),
                        "f64" => TypeRef::Primitive(PrimKind::F64),
                        "bool" => TypeRef::Primitive(PrimKind::Bool),
                        "usize" => TypeRef::Primitive(PrimKind::Usize),
                        "String" => TypeRef::Str,
                        "Vec" => {
                            if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                                if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                                    return TypeRef::Vec(Box::new(convert(inner)));
                                }
                            }
                            TypeRef::Named(ident)
                        }
                        "Option" => {
                            if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                                if let Some(syn::GenericArgument::Type(inner)) = args.args.first() {
                                    return TypeRef::Option(Box::new(convert(inner)));
                                }
                            }
                            TypeRef::Named(ident)
                        }
                        "Result" => {
                            if let syn::PathArguments::AngleBracketed(args) = &seg.arguments {
                                let mut iter = args.args.iter().filter_map(|ga| {
                                    if let syn::GenericArgument::Type(t) = ga { Some(t) } else { None }
                                });
                                if let (Some(t1), Some(t2)) = (iter.next(), iter.next()) {
                                    return TypeRef::Result(Box::new(convert(t1)), Box::new(convert(t2)));
                                }
                            }
                            TypeRef::Named(ident)
                        }
                        other => TypeRef::Named(other.to_string()),
                    }
                } else {
                    TypeRef::Named("unknown".to_string())
                }
            }
            syn::Type::Reference(r) => {
                if let syn::Type::Path(tp) = &*r.elem {
                    if let Some(seg) = tp.path.segments.last() {
                        if seg.ident == "str" { return TypeRef::Str; }
                    }
                }
                convert(&*r.elem)
            }
            syn::Type::Ptr(p) => TypeRef::Ptr(Box::new(convert(&*p.elem))),
            syn::Type::Tuple(t) if t.elems.is_empty() => TypeRef::Void,
            _ => TypeRef::Named("unknown".to_string()),
        }
    }

    let mut module = crate::ir::Module { functions: Vec::new(), structs: Vec::new(), enums: Vec::new() };

    for item in file.items {
        match item {
            syn::Item::Fn(f) => {
                if matches!(f.vis, syn::Visibility::Public(_)) {
                    let name = f.sig.ident.to_string();
                    let doc = extract_doc(&f.attrs);
                    let mut params = Vec::new();
                    for input in f.sig.inputs.iter() {
                        if let syn::FnArg::Typed(pt) = input {
                            let pname = match &*pt.pat {
                                syn::Pat::Ident(pi) => pi.ident.to_string(),
                                _ => "_".to_string(),
                            };
                            let pty = convert(&*pt.ty);
                            params.push(crate::ir::Param { name: pname, ty: pty });
                        }
                    }
                    let ret = match &f.sig.output {
                        syn::ReturnType::Default => crate::ir::TypeRef::Void,
                        syn::ReturnType::Type(_, ty) => convert(&*ty),
                    };
                    module.functions.push(crate::ir::FnDef { name, params, ret, doc });
                }
            }
            syn::Item::Struct(s) => {
                if matches!(s.vis, syn::Visibility::Public(_)) {
                    let name = s.ident.to_string();
                    let doc = extract_doc(&s.attrs);
                    let mut fields = Vec::new();
                    for field in s.fields.iter() {
                        let fname = field.ident.as_ref().map(|id| id.to_string()).unwrap_or_else(|| "_".to_string());
                        let fty = convert(&field.ty);
                        fields.push(crate::ir::FieldDef { name: fname, ty: fty });
                    }
                    module.structs.push(crate::ir::StructDef { name, fields, doc });
                }
            }
            syn::Item::Enum(e) => {
                if matches!(e.vis, syn::Visibility::Public(_)) {
                    let name = e.ident.to_string();
                    let doc = extract_doc(&e.attrs);
                    let variants = e.variants.into_iter().map(|v| crate::ir::EnumVariant { name: v.ident.to_string() }).collect();
                    module.enums.push(crate::ir::EnumDef { name, variants, doc });
                }
            }
            _ => {}
        }
    }

    Ok(module)
}

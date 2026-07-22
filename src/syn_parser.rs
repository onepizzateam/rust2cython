use anyhow::Context;

pub fn parse_rust_file(path: &std::path::Path) -> anyhow::Result<crate::ir::Module> {
    let src = std::fs::read_to_string(path)
        .with_context(|| format!("reading file {}", path.display()))?;
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
        if docs.is_empty() {
            None
        } else {
            Some(docs.join("\n"))
        }
    }

    fn convert(ty: &syn::Type) -> crate::ir::TypeRef {
        use crate::ir::{PrimKind, TypeRef};
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
                        "isize" => TypeRef::Primitive(PrimKind::Isize),
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
                                    if let syn::GenericArgument::Type(t) = ga {
                                        Some(t)
                                    } else {
                                        None
                                    }
                                });
                                if let (Some(t1), Some(t2)) = (iter.next(), iter.next()) {
                                    return TypeRef::Result(
                                        Box::new(convert(t1)),
                                        Box::new(convert(t2)),
                                    );
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
                if let syn::Type::Slice(slice) = &*r.elem {
                    return TypeRef::Vec(Box::new(convert(&slice.elem)));
                }
                if let syn::Type::Path(tp) = &*r.elem {
                    if let Some(seg) = tp.path.segments.last() {
                        if seg.ident == "str" {
                            return TypeRef::Str;
                        }
                    }
                }
                convert(&r.elem)
            }
            syn::Type::Ptr(p) => TypeRef::Ptr(Box::new(convert(&p.elem)), p.mutability.is_some()),
            syn::Type::Tuple(t) if t.elems.is_empty() => TypeRef::Void,
            syn::Type::Tuple(_) => TypeRef::Tuple,
            _ => TypeRef::Named("unknown".to_string()),
        }
    }

    let mut module = crate::ir::Module {
        functions: Vec::new(),
        structs: Vec::new(),
        enums: Vec::new(),
    };

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
                            let pty = if matches!(&*pt.ty, syn::Type::Tuple(t) if !t.elems.is_empty())
                            {
                                // Tuple parameters remain on the established unsupported path;
                                // only tuple *returns* receive the explicit stub diagnostic.
                                crate::ir::TypeRef::Named("unknown".to_string())
                            } else {
                                convert(&pt.ty)
                            };
                            params.push(crate::ir::Param {
                                name: pname,
                                ty: pty,
                                is_slice: matches!(&*pt.ty, syn::Type::Reference(r) if matches!(&*r.elem, syn::Type::Slice(_))),
                            });
                        }
                    }
                    let ret = match &f.sig.output {
                        syn::ReturnType::Default => crate::ir::TypeRef::Void,
                        syn::ReturnType::Type(_, ty) => convert(ty),
                    };
                    module.functions.push(crate::ir::FnDef {
                        original_name: name.clone(),
                        name,
                        params,
                        ret,
                        doc,
                    });
                }
            }
            syn::Item::Impl(item_impl) => {
                let Some(struct_name) = (match &*item_impl.self_ty {
                    syn::Type::Path(tp) => tp
                        .path
                        .segments
                        .last()
                        .map(|segment| segment.ident.to_string()),
                    _ => None,
                }) else {
                    continue;
                };

                for impl_item in item_impl.items {
                    let syn::ImplItem::Fn(method) = impl_item else {
                        continue;
                    };
                    if !matches!(method.vis, syn::Visibility::Public(_)) {
                        continue;
                    }
                    let method_name = method.sig.ident.to_string();
                    if method
                        .sig
                        .inputs
                        .iter()
                        .any(|input| matches!(input, syn::FnArg::Receiver(_)))
                    {
                        println!(
                            "  skipped impl method {}::{} (instance method, requires self)",
                            struct_name, method_name
                        );
                        continue;
                    }

                    let mut params = Vec::new();
                    for input in &method.sig.inputs {
                        let syn::FnArg::Typed(pt) = input else {
                            continue;
                        };
                        let pname = match &*pt.pat {
                            syn::Pat::Ident(pi) => pi.ident.to_string(),
                            _ => "_".to_string(),
                        };
                        params.push(crate::ir::Param {
                            name: pname,
                            ty: if matches!(&*pt.ty, syn::Type::Tuple(t) if !t.elems.is_empty()) {
                                crate::ir::TypeRef::Named("unknown".to_string())
                            } else {
                                convert(&pt.ty)
                            },
                            is_slice: matches!(&*pt.ty, syn::Type::Reference(r) if matches!(&*r.elem, syn::Type::Slice(_))),
                        });
                    }
                    let ret = match &method.sig.output {
                        syn::ReturnType::Default => crate::ir::TypeRef::Void,
                        syn::ReturnType::Type(_, ty) => convert(ty),
                    };
                    let name = format!("{}_{}", struct_name, method_name);
                    println!("  found impl fn: {}::{}", struct_name, method_name);
                    module.functions.push(crate::ir::FnDef {
                        name,
                        original_name: format!("{}::{}", struct_name, method_name),
                        params,
                        ret,
                        doc: extract_doc(&method.attrs),
                    });
                }
            }
            syn::Item::Struct(s) => {
                if matches!(s.vis, syn::Visibility::Public(_)) {
                    let name = s.ident.to_string();
                    let doc = extract_doc(&s.attrs);
                    let mut fields = Vec::new();
                    for field in s.fields.iter() {
                        let fname = field
                            .ident
                            .as_ref()
                            .map(|id| id.to_string())
                            .unwrap_or_else(|| "_".to_string());
                        let fty = convert(&field.ty);
                        fields.push(crate::ir::FieldDef {
                            name: fname,
                            ty: fty,
                        });
                    }
                    module
                        .structs
                        .push(crate::ir::StructDef { name, fields, doc });
                }
            }
            syn::Item::Enum(e) => {
                if matches!(e.vis, syn::Visibility::Public(_)) {
                    let name = e.ident.to_string();
                    let doc = extract_doc(&e.attrs);
                    let variants = e
                        .variants
                        .into_iter()
                        .map(|v| crate::ir::EnumVariant {
                            name: v.ident.to_string(),
                        })
                        .collect();
                    module.enums.push(crate::ir::EnumDef {
                        name,
                        variants,
                        doc,
                    });
                }
            }
            _ => {}
        }
    }

    // Report discovered public functions to stdout
    if module.functions.is_empty() {
        println!("  WARNING: no pub fn found. Are your functions marked pub?");
    } else {
        for fn_def in &module.functions {
            println!("  found pub fn: {}", fn_def.name);
        }
    }

    Ok(module)
}

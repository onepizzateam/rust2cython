#[derive(Debug, Clone, PartialEq)]
pub enum PrimKind {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    Isize,
    Usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeRef {
    Primitive(PrimKind),
    Str,
    Vec(Box<TypeRef>),
    Option(Box<TypeRef>),
    Result(Box<TypeRef>, Box<TypeRef>),
    Named(String),
    Ptr(Box<TypeRef>),
    Void,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: TypeRef,
    pub is_slice: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: String,
    pub original_name: String,
    pub params: Vec<Param>,
    pub ret: TypeRef,
    pub doc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub ty: TypeRef,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<FieldDef>,
    pub doc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EnumVariant {
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub doc: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub functions: Vec<FnDef>,
    pub structs: Vec<StructDef>,
    pub enums: Vec<EnumDef>,
}

impl Module {
    /// Merge shallowly parsed modules while preserving the first public item
    /// with a given exported name. Re-exports can otherwise expose the same
    /// source item through more than one parsed module.
    pub fn merge_modules(mods: Vec<Module>) -> Module {
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut enums = Vec::new();
        let mut seen_fns = std::collections::HashSet::new();
        let mut seen_structs = std::collections::HashSet::new();
        let mut seen_enums = std::collections::HashSet::new();
        for m in mods {
            for function in m.functions {
                if seen_fns.insert(function.name.clone()) {
                    functions.push(function);
                } else {
                    eprintln!(
                        "[INFO] fn `{}` already emitted — skipping duplicate",
                        function.name
                    );
                }
            }
            for structure in m.structs {
                if seen_structs.insert(structure.name.clone()) {
                    structs.push(structure);
                }
            }
            for enumeration in m.enums {
                if seen_enums.insert(enumeration.name.clone()) {
                    enums.push(enumeration);
                }
            }
        }
        Module {
            functions,
            structs,
            enums,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShimParam {
    pub name: String,
    pub original_ty: TypeRef,
    pub ffi_ty: FfiType,
    pub is_slice: bool,
}

#[derive(Debug, Clone)]
pub struct ShimFn {
    pub original_name: String,
    pub shim_name: String,
    pub params: Vec<ShimParam>,
    pub ret: TypeRef,
    pub ffi_ret: FfiType,
}

#[derive(Debug, Clone)]
pub enum FfiType {
    Direct(TypeRef),
    CStr,
    SlicePtr { inner: TypeRef },
    OptionPtr { inner: TypeRef },
    ResultWithErrOut { ok: TypeRef },
    SliceOut { inner: TypeRef },
    StringSlicePtr,
    StringArrayOut,
    Unsupported(String),
}

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
    Usize,
}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
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
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: String,
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

#[derive(Debug, Clone)]
pub struct ShimParam {
    pub name: String,
    pub original_ty: TypeRef,
    pub ffi_ty: FfiType,
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

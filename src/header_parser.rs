use anyhow::Context;

pub fn parse_c_header(path: &std::path::Path) -> anyhow::Result<crate::ir::Module> {
    let src = std::fs::read_to_string(path)
        .with_context(|| format!("reading file {}", path.display()))?;

    let mut module = crate::ir::Module {
        functions: Vec::new(),
        structs: Vec::new(),
        enums: Vec::new(),
    };

    // Helper: convert C type string to TypeRef
    fn map_type(s: &str) -> crate::ir::TypeRef {
        let orig = s.trim();
        let low = orig.to_lowercase();
        let stripped = low.replace("const", "").replace("volatile", "");
        let stripped = stripped.trim();
        let compact = stripped.replace("  ", " ");

        use crate::ir::{PrimKind, TypeRef};

        if compact == "int" || compact == "int32_t" {
            return TypeRef::Primitive(PrimKind::I32);
        }
        if compact == "int64_t" || compact == "long" {
            return TypeRef::Primitive(PrimKind::I64);
        }
        if compact == "uint32_t" || compact == "unsigned int" {
            return TypeRef::Primitive(PrimKind::U32);
        }
        if compact == "float" {
            return TypeRef::Primitive(PrimKind::F32);
        }
        if compact == "double" {
            return TypeRef::Primitive(PrimKind::F64);
        }
        if compact == "bool" {
            return TypeRef::Primitive(PrimKind::Bool);
        }
        if compact == "void" {
            return TypeRef::Void;
        }
        if compact == "void*" || compact == "void *" {
            return TypeRef::Ptr(Box::new(TypeRef::Void));
        }
        if compact == "const char*"
            || compact == "const char *"
            || compact == "char *"
            || compact == "char*"
        {
            return TypeRef::Str;
        }

        // fallback
        TypeRef::Named(orig.trim().to_string())
    }

    // Helper: strip parameter or field name from a declaration fragment
    fn strip_name(token: &str) -> &str {
        let s = token.trim();
        let mut i = s.len();
        // move left over identifier characters
        while i > 0 {
            let c = s.as_bytes()[i - 1] as char;
            if c.is_ascii_alphanumeric() || c == '_' {
                i -= 1;
            } else {
                break;
            }
        }
        if i < s.len() {
            // we found a trailing identifier; trim spaces before it
            while i > 0 && s.as_bytes()[i - 1].is_ascii_whitespace() {
                i -= 1;
            }
            &s[..i]
        } else {
            s
        }
    }

    // Parse typedef struct { ... } name;
    let mut pos = 0usize;
    while let Some(idx) = src[pos..].find("typedef struct") {
        let start = pos + idx;
        if let Some(brace_open) = src[start..].find('{') {
            let brace_open = start + brace_open;
            if let Some(brace_close) = src[brace_open..].find('}') {
                let brace_close = brace_open + brace_close;
                // find semicolon after brace_close
                if let Some(semicol) = src[brace_close..].find(';') {
                    let semicol = brace_close + semicol;
                    // extract name between brace_close and semicol
                    let name = src[brace_close + 1..semicol].trim();
                    let struct_name = name
                        .split_whitespace()
                        .last()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    let fields_text = &src[brace_open + 1..brace_close];
                    let mut fields = Vec::new();
                    for part in fields_text.split(';') {
                        let part = part.trim();
                        if part.is_empty() {
                            continue;
                        }
                        // each field like: "int a" or "const char *name"
                        let typ = strip_name(part);
                        let ty = map_type(typ);
                        // field name: take last token
                        let fname = part
                            .split_whitespace()
                            .last()
                            .unwrap_or("_")
                            .trim()
                            .to_string();
                        fields.push(crate::ir::FieldDef { name: fname, ty });
                    }
                    module.structs.push(crate::ir::StructDef {
                        name: struct_name,
                        fields,
                        doc: None,
                    });
                    pos = semicol + 1;
                    continue;
                }
            }
        }
        // if we couldn't parse, advance to avoid infinite loop
        pos = start + "typedef struct".len();
    }

    // Parse function declarations line by line
    for line in src.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with("//") || line.starts_with('#') {
            continue;
        }
        // only simple one-line prototypes: must end with ';' and contain '(' and ')'
        if !line.ends_with(';') {
            continue;
        }
        if !line.contains('(') || !line.contains(')') {
            continue;
        }
        // avoid typedefs
        if line.starts_with("typedef") {
            continue;
        }
        // extract before '('
        if let Some(p_idx) = line.find('(') {
            let before = &line[..p_idx];
            // separate return type and name
            let before = before.trim();
            let mut parts: Vec<&str> = before.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            let name = parts.pop().unwrap().to_string();
            let ret_type_str = parts.join(" ");
            let ret_ty = if ret_type_str.is_empty() {
                crate::ir::TypeRef::Void
            } else {
                map_type(&ret_type_str)
            };

            // params between () up to closing ')'
            if let Some(close_idx) = line.find(')') {
                let params_str = &line[p_idx + 1..close_idx];
                let mut params = Vec::new();
                let params_str = params_str.trim();
                if !params_str.is_empty() && params_str != "void" {
                    for p in params_str.split(',') {
                        let p = p.trim();
                        if p.is_empty() {
                            continue;
                        }
                        let typ_part = strip_name(p);
                        let mut typ_s = typ_part.trim().to_string();
                        // if star present in original token but lost, keep it
                        if !typ_s.contains('*') && p.contains('*') {
                            typ_s.push('*');
                        }
                        let pty = map_type(&typ_s);
                        // param name attempt
                        let pname = p
                            .split_whitespace()
                            .last()
                            .unwrap_or("_")
                            .trim()
                            .to_string();
                        params.push(crate::ir::Param {
                            name: pname,
                            ty: pty,
                        });
                    }
                }

                module.functions.push(crate::ir::FnDef {
                    name,
                    params,
                    ret: ret_ty,
                    doc: None,
                });
            }
        }
    }

    Ok(module)
}

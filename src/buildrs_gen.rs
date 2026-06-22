pub fn generate_build_rs(
    lib_name: &str,
    rs_source: Option<&str>,
    h_source: Option<&str>,
) -> String {
    // choose input: prefer rs_source
    let input = rs_source.or(h_source).unwrap_or_default();

    let mut out = String::new();
    out.push_str("use std::process::Command;\n\n");
    out.push_str("fn main() {\n");
    out.push_str(&format!(
        "    println!(\"cargo:rerun-if-changed={}\");\n",
        input
    ));
    out.push_str("    let status = Command::new(\"rust2cython\")\n");
    out.push_str(&format!("        .arg(\"{}\")\n", input));
    out.push_str(&format!(
        "        .arg(\"-n\")\n        .arg(\"{}\")\n",
        lib_name
    ));
    out.push_str("        .arg(\"-o\")\n        .arg(\"./\")\n        .output()\n        .expect(\"failed to execute rust2cython\");\n\n");
    out.push_str("    if !status.status.success() {\n");
    out.push_str("        let stderr = String::from_utf8_lossy(&status.stderr);\n");
    out.push_str("        panic!(\"rust2cython failed: {}\", stderr);\n");
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

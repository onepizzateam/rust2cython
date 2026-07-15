use std::path::{Path, PathBuf};

// Minimal crate parser stubs to satisfy the CLI's shallow-crate mode.
// These return errors so runtime behavior is unchanged, but the module
// compiles without requiring the full crate-parsing implementation.

pub fn find_lib_rs(cargo_toml: &Path) -> Result<PathBuf, String> {
    Err(format!("crate_path support not implemented: {}", cargo_toml.display()))
}

pub fn collect_module_files(_lib_rs: &Path) -> Result<Vec<PathBuf>, String> {
    Err("collect_module_files not implemented".into())
}

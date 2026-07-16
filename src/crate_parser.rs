use anyhow::{bail, Context};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn find_lib_rs(cargo_toml: &Path) -> anyhow::Result<PathBuf> {
    let cargo_dir = cargo_toml
        .parent()
        .context("Cargo.toml path must have a parent directory")?;
    let cargo_text = std::fs::read_to_string(cargo_toml)
        .with_context(|| format!("reading Cargo.toml at {}", cargo_toml.display()))?;

    let mut in_lib = false;
    let mut lib_path = None;
    for line in cargo_text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_lib = trimmed == "[lib]";
            continue;
        }
        if in_lib && trimmed.starts_with("path") {
            if let Some(eq) = trimmed.find('=') {
                let value = trimmed[eq + 1..].trim().trim_matches('"');
                lib_path = Some(cargo_dir.join(value));
                break;
            }
        }
    }

    let candidate = lib_path.unwrap_or_else(|| cargo_dir.join("src/lib.rs"));
    if candidate.exists() {
        Ok(candidate)
    } else {
        bail!(
            "could not find lib.rs for crate at {}",
            cargo_toml.display()
        )
    }
}

pub fn collect_module_files(lib_rs: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let crate_root = lib_rs
        .parent()
        .context("lib.rs path must have a parent directory")?;
    let src =
        std::fs::read_to_string(lib_rs).with_context(|| format!("reading {}", lib_rs.display()))?;
    let mut files = vec![lib_rs.to_path_buf()];
    let mut seen = HashSet::from([lib_rs.to_path_buf()]);

    for line in src.lines() {
        let trimmed = line.trim();
        let mod_name = if let Some(rest) = trimmed.strip_prefix("pub mod ") {
            rest
        } else if let Some(rest) = trimmed.strip_prefix("mod ") {
            rest
        } else {
            continue;
        };

        let mod_name = mod_name
            .split(|c: char| c == ';' || c == '{' || c.is_whitespace())
            .next()
            .unwrap_or("")
            .trim();
        if mod_name.is_empty() {
            continue;
        }

        let candidates = [
            crate_root.join(format!("{mod_name}.rs")),
            crate_root.join(mod_name).join("mod.rs"),
        ];
        if let Some(path) = candidates.into_iter().find(|p| p.exists()) {
            if seen.insert(path.clone()) {
                files.push(path);
            }
        }
    }

    Ok(files)
}

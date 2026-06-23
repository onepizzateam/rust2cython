mod buildrs_gen;
mod header_parser;
mod header_gen;
mod ir;
mod shim_planner;
mod shim_gen;
mod pxd_gen;
mod pyx_gen;
mod setuptools_gen;
mod syn_parser;
mod translator;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "Generate Cython wrappers from Rust/C headers", long_about = None)]
struct Args {
    /// Path to a .rs source file or a .h C header file
    #[arg(value_name = "INPUT")]
    input: PathBuf,

    /// Output directory (default: current dir)
    #[arg(short, long, value_name = "DIR", default_value = ".")]
    output: PathBuf,

    /// Library name used in generated files (default: stem of INPUT)
    #[arg(short, long, value_name = "NAME")]
    name: Option<String>,

    /// Input format: auto, rust, c (default: auto)
    #[arg(long, value_name = "FORMAT", default_value = "auto")]
    format: String,

    /// Instead of generating .pxd/.pyx, print a build.rs snippet to stdout
    #[arg(long, action = clap::ArgAction::SetTrue)]
    emit_buildrs: bool,

    /// Skip generating setup.py / pyproject.toml / BUILD.sh
    #[arg(long, action = clap::ArgAction::SetTrue)]
    no_setup: bool,

    /// Skip generating the Rust shim (_ffi.rs)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    no_shim: bool,
    /// Skip injecting the generated shim into the original crate (mod declaration)
    #[arg(long, action = clap::ArgAction::SetTrue)]
    no_inject: bool,
}

fn main() {
    let args = Args::parse();

    let fmt = args.format.to_lowercase();

    let detected = if fmt == "auto" {
        match args.input.extension().and_then(|s| s.to_str()) {
            Some(ext) if ext.eq_ignore_ascii_case("rs") => "rust".to_string(),
            Some(ext) if ext.eq_ignore_ascii_case("h") => "c".to_string(),
            _ => {
                eprintln!("Error: could not auto-detect input format from extension");
                std::process::exit(1);
            }
        }
    } else if fmt == "rust" || fmt == "c" {
        fmt
    } else {
        eprintln!("Error: unknown format '{}', expected auto|rust|c", fmt);
        std::process::exit(1);
    };

    let module = match detected.as_str() {
        "rust" => match syn_parser::parse_rust_file(&args.input) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        "c" => match header_parser::parse_c_header(&args.input) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        _ => unreachable!(),
    };

    let name = args.name.clone().unwrap_or_else(|| {
        args.input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("lib")
            .to_string()
    });

    if args.emit_buildrs {
        let rs_src = if detected == "rust" {
            args.input.to_str()
        } else {
            None
        };
        let h_src = if detected == "c" {
            args.input.to_str()
        } else {
            None
        };
        let snippet = buildrs_gen::generate_build_rs(&name, rs_src, h_src);
        println!("{}", snippet);
        return;
    }

    if let Err(e) = std::fs::create_dir_all(&args.output) {
        eprintln!("Error: failed to create output dir: {}", e);
        std::process::exit(1);
    }

    let pxd = pxd_gen::generate_pxd(&module, &name);
    let pyx = pyx_gen::generate_pyx(&module, &name);
    let header_content = header_gen::generate_header(&module, &name);

    let pxd_path = args.output.join(format!("{}.pxd", name));
    let pyx_path = args.output.join(format!("{}.pyx", name));

    if let Err(e) = std::fs::write(&pxd_path, pxd) {
        eprintln!("Error: failed to write {}: {}", pxd_path.display(), e);
        std::process::exit(1);
    }
    if let Err(e) = std::fs::write(&pyx_path, pyx) {
        eprintln!("Error: failed to write {}: {}", pyx_path.display(), e);
        std::process::exit(1);
    }
    let header_path = args.output.join(format!("{}.h", name));
    if let Err(e) = std::fs::write(&header_path, header_content) {
        eprintln!("Error: failed to write {}: {}", header_path.display(), e);
        std::process::exit(1);
    }

    let mut shim_written = false;
    if !args.no_shim {
        // derive src_dir from input file
        let src_dir = args.input.parent().unwrap_or(std::path::Path::new("."));

        // write shim to src_dir
        let shim_content = shim_gen::generate_shim(&module);
        let shim_path = src_dir.join(format!("{}_ffi.rs", name));
        if let Err(e) = std::fs::write(&shim_path, &shim_content) {
            eprintln!("Error: failed to write {}: {}", shim_path.display(), e);
            std::process::exit(1);
        }
        shim_written = true;

        // patch lib.rs — insert mod declaration before first pub fn
        if args.no_inject {
            println!("--no-inject set, skipping shim injection");
        } else {
            let lib_rs_content = match std::fs::read_to_string(&args.input) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error: failed to read {}: {}", args.input.display(), e);
                    std::process::exit(1);
                }
            };
            let mod_line = format!("mod {}_ffi;", name);
            // remove any existing occurrences of the mod line, then insert it before first pub fn
            let mut new_content = lib_rs_content.clone();
            while let Some(pos) = new_content.find(&mod_line) {
                let rem_end = pos + mod_line.len();
                let mut tail = rem_end;
                while tail < new_content.len() && (new_content.as_bytes()[tail] == b'\n' || new_content.as_bytes()[tail] == b'\r') {
                    tail += 1;
                }
                new_content.replace_range(pos..tail, "");
            }
            let insert_pos = new_content.find("pub fn").unwrap_or(new_content.len());
            new_content.insert_str(insert_pos, &format!("{}\n\n", mod_line));
            if let Err(e) = std::fs::write(&args.input, new_content) {
                eprintln!("Error: failed to write {}: {}", args.input.display(), e);
                std::process::exit(1);
            }
            println!(
                "Patched lib.rs — inserted mod {}_ffi before first pub fn",
                name
            );
            println!("Injected {}_ffi.rs into {}", name, src_dir.display());
        }
    }

    if !args.no_setup {
        let rs_source = args.input.to_str().unwrap_or("");
        let (setup_py, pyproject) = setuptools_gen::generate_setup_files(&name, rs_source);

        let build_sh = setuptools_gen::generate_build_instructions(&name);

        let setup_path = args.output.join("setup.py");
        let pyproject_path = args.output.join("pyproject.toml");
        let build_path = args.output.join("BUILD.sh");
            let requirements_path = args.output.join("requirements.txt");

        if let Err(e) = std::fs::write(&setup_path, setup_py) {
            eprintln!("Error: failed to write {}: {}", setup_path.display(), e);
            std::process::exit(1);
        }
        if let Err(e) = std::fs::write(&pyproject_path, pyproject) {
            eprintln!("Error: failed to write {}: {}", pyproject_path.display(), e);
            std::process::exit(1);
        }
        if let Err(e) = std::fs::write(&build_path, build_sh) {
            eprintln!("Error: failed to write {}: {}", build_path.display(), e);
            std::process::exit(1);
        }
            // write requirements.txt
            if let Err(e) = std::fs::write(&requirements_path, setuptools_gen::generate_requirements()) {
                eprintln!("Error: failed to write {}: {}", requirements_path.display(), e);
                std::process::exit(1);
            }

        if shim_written {
            println!(
                "Generated {}.pxd, {}.pyx, {}.h, {}_ffi.rs, setup.py, pyproject.toml, BUILD.sh in {}",
                name,
                name,
                name,
                name,
                args.output.display()
            );
        } else {
            println!(
                "Generated {}.pxd, {}.pyx, {}.h, setup.py, pyproject.toml, BUILD.sh in {}",
                name,
                name,
                name,
                args.output.display()
            );
        }
    } else {
        if shim_written {
            println!(
                "Generated {}.pxd, {}.pyx, {}_ffi.rs in {}",
                name,
                name,
                name,
                args.output.display()
            );
        } else {
            println!(
                "Generated {}.pxd and {}.pyx in {}",
                name,
                name,
                args.output.display()
            );
        }
    }
}

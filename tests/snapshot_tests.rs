use std::fs;
use std::path::{Path, PathBuf};

#[test]
fn rust_fixtures_generate_snapshots() {
    let fixtures_dir = Path::new("tests/fixtures");
    let mut fixtures: Vec<PathBuf> = fs::read_dir(fixtures_dir)
        .expect("failed to read tests/fixtures")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("rs"))
        .collect();

    fixtures.sort();

    for fixture in fixtures {
        let stem = fixture
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("fixture stem should be valid UTF-8")
            .to_string();

        let module = rust2cython::syn_parser::parse_rust_file(&fixture)
            .unwrap_or_else(|e| panic!("failed to parse {}: {e}", fixture.display()));

        let pxd = rust2cython::pxd_gen::generate_pxd(&module, &stem);
        let pyx = rust2cython::pyx_gen::generate_pyx(&module, &stem);
        let header = rust2cython::header_gen::generate_header(&module, &stem);

        insta::assert_snapshot!(format!("{}_pxd", stem), pxd);
        insta::assert_snapshot!(format!("{}_pyx", stem), pyx);
        insta::assert_snapshot!(format!("{}_h", stem), header);
    }
}

#[test]
fn parse_rust_file_invalid_rs_returns_err() {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "rust2cython_invalid_{}_{}.rs",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be after unix epoch")
            .as_nanos()
    ));

    fs::write(&path, "pub fn broken( -> i32 { 1 }").expect("failed to write temp invalid rs");
    let parsed = rust2cython::syn_parser::parse_rust_file(&path);
    let _ = fs::remove_file(&path);

    assert!(parsed.is_err(), "expected invalid Rust fixture to return Err");
}

#[test]
fn parse_c_header_invalid_h_returns_err() {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "rust2cython_invalid_{}_{}.h",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be after unix epoch")
            .as_nanos()
    ));

    // Intentionally do not create the file so parser takes the error path.
    let parsed = rust2cython::header_parser::parse_c_header(&path);

    assert!(parsed.is_err(), "expected invalid C header fixture to return Err");
}

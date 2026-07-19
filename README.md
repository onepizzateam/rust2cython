# rust2cython — Generate Cython bindings from Rust

rust2cython generates complete Cython `.pxd` / `.pyx` bindings, a C header, and a small Rust FFI shim from idiomatic Rust source.

This repository contains the generator (a Rust CLI) and several real-world examples demonstrating the generated output.

Quick start

```bash
rust2cython src/lib.rs -o bindings/ -n mylib
cd bindings
sh BUILD.sh
```

What you get

- `mylib.pxd` — Cython declarations (extern block)
- `mylib.pyx` — Cython wrapper with Python-friendly functions/classes
- `mylib.h` — C header matching exported symbols
- `src/mylib_ffi.rs` — Rust shim (optional injection into your crate)
- `setup.py` / `pyproject.toml` / `BUILD.sh` — build helpers

Why this tool

If you maintain or extend a codebase that uses hand-written Cython wrappers, `rust2cython` lets you implement performance-critical code in Rust and generate the Cython integration automatically — preserving existing `.pxd`/`.pyx` workflows and compatibility with hand-written code.

Features added for v1.0.0

- Expanded type coverage: `isize`, `Result<String, _>`, improved `Vec<T>` handling
- Shallow crate mode (`--crate`) to merge top-level modules
- `--typed` mode to emit Python annotations in generated `.pyx`
- `--dry-run` to preview outputs without writing files
- Clearer unsupported-type warnings and recovery suggestions

## Performance

Benchmark: z-score of 1,000,000 `f64` values, 100 iterations, median time. Run on WSL2 / Ubuntu (x86_64).

| Approach | Median | Speedup vs Pure Python | Effort |
|---|---|---|---|
| Pure Python | 231.17 ms | 1x | ? |
| NumPy | 7.29 ms | 31.7x | ? |
| cffi (manual) | 16.31 ms | 14.2x | write header + bindings by hand |
| rust2cython (generated) | 16.36 ms | 14.1x | **one command** |
| PyO3 (rewrite required) | 5.20 ms | 44.5x | rewrite Rust with Python annotations |

rust2cython matches hand-written cffi performance exactly ? the same underlying `.so`, the same FFI boundary, zero manual work. PyO3 is faster because it eliminates the array copy entirely, but requires rewriting your Rust code with Python-specific annotations and gives up Cython compatibility.

If you already use Cython, rust2cython is the only tool that lets you call idiomatic Rust from existing `.pyx` files without touching either codebase.

To reproduce:
```bash
cd bench/rust_zscore && cargo build --release
rust2cython bench/rust_zscore/src/lib.rs -o bench/rust_zscore_out/ -n rust_zscore
cd bench/rust_zscore_out && sh BUILD.sh
python3 bench/zscore_bench.py
```

## tested against real codebases

Run `bash examples/validate/run_all.sh` to reproduce. Results filled in after validation runs.

| Repo | Stars | Functions found | Generated | Skipped | Primary skip reason |
|------|-------|-----------------|-----------|---------|---------------------|
| rust-bio (gc) | ~4k | 2 | yes | 0 | none |
| rust-bio (align) | ~4k | 0 | yes | 0 | no pub fn found |
| triple_accel | ~400 | 2 | yes | 0 | none |
| statrs | ~1k | 0 | yes | 0 | no pub fn found |
| linfa-linear | ~3k | 0 | no | 0 | reading file /tmp/linfa/linfa-linear/src/lib.rs |

*See [examples/validate/RESULTS.md](examples/validate/RESULTS.md) for full per-repo findings.*

Getting started

Install the CLI:

```bash
cargo install --path .
```

Generate bindings:

```bash
rust2cython --typed src/lib.rs -o bindings -n mylib
```

Preview without writing:

```bash
rust2cython --dry-run src/lib.rs -n mylib
```

Shallow crate mode:

```bash
rust2cython --crate Cargo.toml -o bindings -n mylib
```

Contributing

Run tests (snapshots may need to be accepted on first run):

```bash
INSTA_UPDATE=new cargo test
cargo clippy -- -D warnings
```

License

MIT

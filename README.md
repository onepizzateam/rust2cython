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

Performance

There is an included benchmark script at `bench/zscore_bench.py` that measures z-score performance across multiple approaches. This repository does not contain measured numbers — please run the benchmark locally on your target machine and commit the results.

To run the benchmark script:

```bash
python bench/zscore_bench.py
```

The intended benchmark procedure is to measure the following approaches on 1M f64 values, 100 iterations each, and record the median times:

- Pure Python (list-based implementation)
- NumPy vectorized
- rust2cython-generated binding (build via `examples/linear_stats/BUILD.sh`)
- PyO3/maturin implementation (optional)
- Hand-written Cython wrapper (baseline)

After running, populate the performance table in this README with the measured values.

Examples

This repo includes four standalone examples under `examples/` — each has a minimal Rust crate and committed generated outputs so you can inspect the `.pxd`, `.pyx`, `.h`, and shim files without running the generator.

- `examples/rust_bio_gc` — GC content, reverse complement, hamming distance (Result return)
- `examples/linear_stats` — `Vec<f64>` in/out, numpy memoryview paths and a `bench.py` for local benchmarking
- `examples/sequence_struct` — struct return and `Vec<String>` input demo
- `examples/signal_processing` — numeric arrays and a documented skipped `Option<Vec<T>>` case

Type support (v1.0.0)

| Rust type | Python | Notes |
|-----------|--------|-------|
| `i8` `i16` `i32` `u8` `u16` `u32` `usize` `isize` | `int` | |
| `i64` `u64` | `int` | mapped to `long long` in C |
| `f32` | `float` | |
| `f64` | `float` | |
| `bool` | `bool` | |
| `&str`, `String` | `str` | encode/decode handled automatically |
| `Vec<f64>`, `Vec<i32>`, etc. | `np.ndarray` | zero-copy via typed memoryview |
| `Vec<String>` | `list[str]` | |
| `Option<primitive>` | `T \| None` | |
| `Option<String>` | `str \| None` | |
| `Result<primitive, _>` | `T` (raises `RuntimeError`) | |
| `Result<String, _>` | `str` (raises `RuntimeError`) | |
| `pub struct` with primitive/str fields | `cdef class` | |
| C-style `pub enum` | `cpdef enum` | |
| `Vec<Struct>` | ❌ skipped | flatten to parallel arrays or use a primitive wrapper |
| `Option<Vec<T>>` | ❌ skipped | use `*const T` + len param pattern |
| `HashMap`, `BTreeMap` | ❌ skipped | serialize to `Vec<(K,V)>` |
| Tuple `(A, B)` | ❌ skipped | use a named struct |
| `u128`, `i128` | ❌ skipped | no C equivalent; use `u64` |

## tested against real codebases

Run `bash examples/validate/run_all.sh` to reproduce. Results filled in after validation runs.

| Repo | Stars | Functions found | Generated | Skipped | Primary skip reason |
|------|-------|-----------------|-----------|---------|---------------------|
| rust-bio (gc) | ~4k | — | — | — | — |
| rust-bio (align) | ~4k | — | — | — | — |
| triple_accel | ~400 | — | — | — | — |
| statrs | ~1k | — | — | — | — |
| linfa-linear | ~3k | — | — | — | — |

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

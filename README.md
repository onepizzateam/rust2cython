# rust2cython — Generate Cython bindings from Rust

rust2cython generates Cython `.pxd` / `.pyx` bindings, a C header, and a small Rust FFI shim from idiomatic Rust source.

## Quick start

```bash
rust2cython src/lib.rs -o bindings -n mylib
python build.py --name mylib --bindings bindings
```

Generated output includes `.pxd`, `.pyx`, a matching C header, an optional Rust shim, and setuptools metadata.

## Benchmark table

| Implementation | Mean (ms) | Std (ms) | vs Pure Python |
|----------------|-----------|----------|----------------|
| Pure Python    | 122.030   | 69.525   | 1.0×           |
| NumPy          | 3.236     | 1.400    | 37.7×          |
| cffi (manual)  | 1.014     | 0.104    | 120.3×         |
| rust2cython    | 5.123     | 1.973    | 23.8×          |
| PyO3           | N/A       | N/A      | N/A (PyO3 0.21 does not support Python 3.13) |

Benchmarked on: Windows 11 Home Single Language, 11th Gen Intel(R) Core(TM) i3-1115G4 @ 3.00GHz, Python 3.13.5, rustc 1.89.0, N=100_000.

## Type support

See [TYPES.md](TYPES.md) for the full supported/stub/unsupported matrix. Unsupported types emit a `[WARN]` at generation time with a suggested workaround.

## Building

### Linux / macOS / Windows

```bash
python build.py
```

On Windows, ensure Rust and Python are in PATH. WSL2 also works.

## Validation table

| Crate        | Pub fns before | Pub fns after | .pyx compiles | .pyx imports |
|--------------|---------------:|--------------:|---------------|--------------|
| linfa-linear | 0 | 0 | yes | N/A — no public free functions generated, so no extension target |
| statrs       | 0 | 0 | yes | N/A — no public free functions generated, so no extension target |

## Getting started

Install the CLI with `cargo install --path .`, then generate bindings with:

```bash
rust2cython --typed src/lib.rs -o bindings -n mylib
```

Use `rust2cython --dry-run src/lib.rs -n mylib` to preview files, or `rust2cython --crate Cargo.toml -o bindings -n mylib` for shallow crate traversal.

## CI

The repository includes GitHub Actions workflows. The main branch status is shown below.

[![CI](https://github.com/onepizzateam/rust2cython/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/onepizzateam/rust2cython/actions/workflows/ci.yml)

## License

MIT

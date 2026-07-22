# rust2cython — Generate Cython bindings from Rust

![CI](https://github.com/onepizzateam/rust2cython/actions/workflows/ci.yml/badge.svg)

rust2cython generates Cython bindings from Rust source. Unlike PyO3, it is non-invasive: it works from unmodified Rust source and does not require attributes or proc macros, which is useful when you cannot or do not want to change the Rust code. It generates `.pxd` and `.pyx` files, a C header, a Rust FFI shim, and setuptools metadata.

## Installation

```bash
cargo install --path .
```

Prerequisites: Rust 1.70+, Python 3.10+, and Cython 3.x.

## Quick start

This example uses the z-score implementation in `bench/rust_zscore/src/lib.rs`.

### Step 1 — Rust source

```rust
pub fn zscore(values: Vec<f64>) -> Vec<f64> {
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let std = (values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n).sqrt();
    values.iter().map(|x| (x - mean) / std).collect()
}
```

### Step 2 — Generate bindings

```bash
rust2cython src/lib.rs -o bindings -n zscore
```

### Step 3 — Build

```bash
python build.py --name zscore --bindings bindings
```

### Step 4 — Use from Python

```python
import zscore

result = zscore.zscore(data)
```

The generated wrapper derives and passes the pointer and length for the `Vec<f64>` input.

## CLI reference

| Flag | Description |
|------|-------------|
| `-o, --output <DIR>` | Output directory. |
| `-n, --name <NAME>` | Library name used in generated files. |
| `--format <FORMAT>` | Input format: `auto`, `rust`, or `c`. |
| `--emit-buildrs` | Print a `build.rs` snippet instead of generating bindings. |
| `--no-setup` | Skip `setup.py`, `pyproject.toml`, and `BUILD.sh` generation. |
| `--no-shim` | Skip Rust FFI shim generation. |
| `--no-inject` | Skip injecting the generated shim module declaration. |
| `--crate <CARGO_TOML>` | Parse a full crate from a Cargo.toml path; shallow traversal. |
| `--crate-path` | Alias for `--crate`. |
| `--typed` | Emit Python type annotations in generated `.pyx`. |
| `--dry-run` | Print generated files without writing them. |
| `--platform <PLATFORM>` | Platform for rpath and library extension: `auto`, `linux`, or `macos`. |
| `--lib-version <VERSION>` | Version for the generated library. |
| `--wheel` | Generate a distributable wheel. |
| `--no-wheel` | Disable wheel generation. |
| `-h, --help` | Print help. |
| `-V, --version` | Print version. |

## Benchmark table

| Implementation | Mean (ms) | Std (ms) | vs Pure Python |
|----------------|-----------|----------|----------------|
| Pure Python    | 122.030   | 69.525   | 1.0×           |
| NumPy          | 3.236     | 1.400    | 37.7×          |
| cffi (manual)  | 1.014     | 0.104    | 120.3×         |
| rust2cython    | 5.123     | 1.973    | 23.8×          |
| PyO3           | N/A       | N/A      | N/A — PyO3 not installed in benchmark environment; skipped this session. |

> cffi outperforms rust2cython on this microbenchmark because cffi has lower per-call overhead for simple scalar loops; rust2cython's advantage compounds on larger APIs where generated type-safe wrappers reduce integration code.

Benchmarked on: Windows 11 Home Single Language, 11th Gen Intel(R) Core(TM) i3-1115G4 @ 3.00GHz, Python 3.13.5, rustc 1.89.0, N=100_000.

## Type support

See [TYPES.md](TYPES.md) for the full supported/stub/unsupported matrix. Unsupported types emit a `[WARN]` at generation time with a suggested workaround.

## Validation table

| Crate        | Pub fns (before) | Pub fns (after) | .pyx compiles | .pyx imports |
|--------------|-----------------:|----------------:|---------------|--------------|
| linfa-linear | 0 | 2 | ✓ | N/A¹ |
| statrs       | 0 | 9 | ✓ | N/A¹ |

¹ Import test N/A: neither crate includes a cdylib build target.
> To use rust2cython output from these crates, add `crate-type = ["cdylib"]` to their Cargo.toml and rebuild.

## Limitations

- **Generic types** are not resolved at FFI boundaries. Functions or structs with unresolved generic parameters are emitted as stubs with a `[WARN]` and a suggested workaround.
- **Owned `Vec<T>`** has no stable C ABI. Use `*mut T` + length params instead; rust2cython handles these correctly.
- **Tuple returns** have no C ABI. The generator emits a stub; use a `#[repr(C)]` newtype struct as a workaround.
- **`dyn Trait`** (trait objects) are emitted as opaque pointers with a warning.
- **Windows**: native extension linking requires Rust `.dll` and a matching import library. `build.py` handles this; `BUILD.sh` is Unix-only and deprecated.
- **`--crate` mode** performs shallow traversal. Deeply nested private modules or complex re-export chains may not be fully visited.

See [TYPES.md](TYPES.md) for the full matrix of supported, stub-only, and unsupported types.

## Building

### Linux / macOS / Windows

```bash
python build.py
```

On Windows, ensure Rust and Python are in PATH. WSL2 also works.

## Contributing

1. Fork the repository and create a branch.
2. Add a test for any new type mapping in `tests/smoke/`.
3. Run `cargo test && cargo clippy -- -D warnings` before submitting.
4. If you add a new type mapping, update `TYPES.md` to match. The matrix must stay in sync with the CLI warning output.

Run the full test suite:

```bash
cargo test
cargo clippy -- -D warnings
```

## License

MIT

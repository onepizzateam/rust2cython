# rust2cython 🦀→🐍

> Rust library → pip-installable Python package. Automatically.

```bash
rust2cython src/lib.rs -o bindings/ -n mylib
cd bindings && sh BUILD.sh
LD_LIBRARY_PATH="$PWD/bindings" PYTHONPATH="$PWD/bindings" python3 your_script.py
```

That's it. No glue code. No manual `.pxd`. No FFI boilerplate.

---

## the problem

You have a Rust library. You want to call it from Python.

PyO3 is great if you're starting fresh. But if you have an existing Cython codebase — common in scientific Python, bioinformatics, and numerical computing — you're stuck writing `.pxd` and `.pyx` wrappers by hand. That's tedious, error-prone, and existing solutions focus on PyO3 or manual bindings — not Cython.

`rust2cython` fills that gap. For one function you could write the wrapper yourself. For a large Rust API with structs, Options, and Results across dozens of functions, that's days of work. `rust2cython` makes it one command regardless of scale.

---

## what it generates

Given this idiomatic Rust:

```rust
pub struct Point { pub x: f64, pub y: f64 }

pub fn distance(p: Point) -> f64 {
    (p.x * p.x + p.y * p.y).sqrt()
}

pub fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err("divide by zero".into()) } else { Ok(a / b) }
}
```

You get this Python:

```python
import mylib

p = mylib.Point(3.0, 4.0)
print(mylib.distance(p))      # 5.0

print(mylib.safe_div(10, 2))  # 5
mylib.safe_div(10, 0)         # raises RuntimeError: divide by zero
```

`rust2cython` generates six files:

```
bindings/
  mylib.pxd       ← Cython declaration file
  mylib.pyx       ← Cython wrapper with Python classes
  mylib.h         ← C header matching the exported symbols
  setup.py        ← builds the Cython extension
  pyproject.toml
  BUILD.sh        ← one script that does everything
```

It also patches your `src/lib.rs` to add `mod mylib_ffi;` and writes `src/mylib_ffi.rs` — the Rust FFI shim that exports your idiomatic functions over C ABI. You never write `#[no_mangle]` or `extern "C"` by hand.

---

## installation

```bash
cargo install rust2cython
```

Or from source:

```bash
git clone https://github.com/onepizzateam/rust2cython
cd rust2cython
cargo build --release
```

---

## how it works

Unlike tools that use regex or string matching, `rust2cython` is built on [`syn`](https://github.com/dtolnay/syn) — the same AST parser used by the Rust compiler's proc-macro ecosystem. It walks the full Rust type tree: generics, references, nested types, visibility — all handled at the AST level.

```
pub fn safe_div(a: i32, b: i32) -> Result<i32, String>
                                   ^^^^^^^^^^^^^^^^^^^
                                   syn parses this as a proper AST node,
                                   not a string — so nested types
                                   work just as well.
```

The generated bindings are as accurate as what `rustc` itself sees, not a best-effort guess from pattern matching.

---

## usage

```
rust2cython [OPTIONS] <INPUT>

Arguments:
  <INPUT>    Path to a .rs source file or .h C header

Options:
  -o, --output <DIR>      Output directory [default: current dir]
  -n, --name <NAME>       Library name [default: input filename stem]
  --format <FORMAT>       Input format: auto, rust, c [default: auto]
  --no-setup              Only generate .pxd and .pyx, skip setup files
  --no-inject             Skip patching lib.rs and writing the FFI shim
  --emit-buildrs          Print a build.rs snippet to stdout
```

### typical workflow

```bash
# generate everything
rust2cython src/lib.rs -o bindings/ -n mylib

# build (handles cargo build, Cython compile, import verification)
cd bindings && sh BUILD.sh

# run
LD_LIBRARY_PATH="$PWD/bindings" PYTHONPATH="$PWD/bindings" python3 your_script.py
```

### from a C header

```bash
cbindgen --output mylib.h
rust2cython mylib.h -o bindings/ -n mylib
```

### only the Cython files

```bash
rust2cython src/lib.rs --no-setup
```

---

## type support

| Rust | Python |
|------|--------|
| `i32`, `u32`, `i64`, `f64`, `bool`, etc. | native numeric types |
| `&str`, `String` | `str` (encode/decode handled automatically) |
| `Vec<f64>` / `Vec<i32>` etc. | `numpy` array via memoryview |
| `Option<T>` | `None` or value |
| `Result<T, E>` | return value or `RuntimeError` |
| `pub struct Foo` | `cdef class Foo` with typed constructor |

---

## current limitations

v0.1 — works, but with known gaps:

- `Vec<T>` supports numeric primitives only (`Vec<String>`, `Vec<Struct>` skipped with a warning)
- `Option<String>` not yet supported
- Nested generics (e.g. `Option<Vec<f64>>`) are skipped with a `# WARNING` comment in the output
- Enums with data not supported — C-style enums only
- Linux only — macOS users need to swap `.so` for `.dylib` in `BUILD.sh`
- String-returning functions require the Rust side to use `CString::into_raw()` — the generated shim handles this but memory must be freed manually (a `rust2cython_free_string` helper is generated in the shim)

Skipped functions are reported on stderr so you always know what wasn't handled.

---

## examples

- [`examples/linear_algebra/`](examples/linear_algebra/) — dot product, norm, scale, matrix determinant
- [`examples/bio_sequence/`](examples/bio_sequence/) — GC content, reverse complement, motif search

---

## contributing

Most useful contributions right now:

- Real `.rs` files that produce wrong or missing output — open an issue with the file
- `Vec<String>` and `Option<String>` support
- macOS `BUILD.sh` variant
- Auto-freeing string wrappers

---

## license

MIT
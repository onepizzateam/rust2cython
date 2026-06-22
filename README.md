# rust2cython 🦀→🐍

> Rust library → pip-installable Python package. Automatically.

```bash
rust2cython src/lib.rs -o bindings/ -n mylib
# → mylib.pxd
# → mylib.pyx
# → setup.py
# → pyproject.toml
# → BUILD.sh
```

Then:

```bash
sh bindings/BUILD.sh
python -c "import mylib; print(mylib.distance(mylib.Point(3.0, 4.0)))"
# 5.0
```

That's it.

---

## the problem

You have a Rust library. You want to call it from Python.

PyO3 is great if you're starting fresh. But if you have an existing Cython codebase — common in scientific Python, bioinformatics, and numerical computing — you're stuck writing `.pxd` and `.pyx` wrappers by hand. That's tedious, error-prone, and existing solutions focus on PyO3 or manual bindings — not Cython.

`rust2cython` fills that gap.

---

## what it generates

Given this Rust:

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

No glue code. No manual `.pxd`. No wrapper boilerplate.

---

## installation

```bash
cargo install rust2cython
```

Or from source:

```bash
git clone https://github.com/you/rust2cython
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
                                   not a string — so Result<Vec<T>, E>
                                   works just as well.
```

This means the generated bindings are as accurate as what `rustc` itself sees, not a best-effort guess from pattern matching.

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
  --emit-buildrs          Print a build.rs snippet to stdout
```

### from Rust source

```bash
rust2cython src/lib.rs -o bindings/ -n mylib
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

### auto-regenerate on compile

```bash
rust2cython src/lib.rs --emit-buildrs >> build.rs
```

---

## type support

| Rust | Python |
|------|--------|
| `i32`, `u32`, `i64`, `f64`, `bool` | native numeric types |
| `&str`, `String` | `str` (encode/decode handled) |
| `Vec<f64>` / `Vec<i32>` etc. | `numpy` array via memoryview |
| `Option<T>` | `None` or value |
| `Result<T, E>` | return value or `RuntimeError` |
| `struct Foo` | `cdef class Foo` with typed constructor |

---

## FFI conventions

`rust2cython` generates the Python side automatically. Your Rust functions need to follow these conventions — the generated files include comments telling you exactly what's needed.

### strings

Return `*const c_char` via `CString::into_raw()`:

```rust
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn greet(name: *const c_char) -> *const c_char {
    let name = unsafe { std::ffi::CStr::from_ptr(name).to_str().unwrap() };
    CString::new(format!("hello {}", name)).unwrap().into_raw()
}
```

### `Option<T>`

Return or accept `*const T`. `NULL` means `None`:

```rust
#[no_mangle]
pub extern "C" fn maybe_sqrt(x: f64) -> *const f64 {
    if x >= 0.0 { Box::into_raw(Box::new(x.sqrt())) }
    else { std::ptr::null() }
}
```

### `Result<T, E>`

Accept `*mut *mut c_char` as the last param. Write `NULL` on success, an error string on failure:

```rust
#[no_mangle]
pub extern "C" fn safe_div(a: i32, b: i32, error_out: *mut *mut c_char) -> i32 {
    if b == 0 {
        unsafe { *error_out = CString::new("divide by zero").unwrap().into_raw(); }
        return 0;
    }
    unsafe { *error_out = std::ptr::null_mut(); }
    a / b
}
```

### memory: freeing Rust strings

Add this to your lib and call it from Python after consuming a returned string:

```rust
#[no_mangle]
pub extern "C" fn free_rust_string(ptr: *mut c_char) {
    if !ptr.is_null() { unsafe { drop(CString::from_raw(ptr)); } }
}
```

---

## current limitations

v0.1 — works, but rough around the edges:

- `Vec<T>` supports numeric primitives only (no `Vec<String>`, `Vec<Struct>`)
- `Option<String>` not yet supported
- Nested generics (e.g. `Option<Vec<f64>>`) are skipped with a `# WARNING` comment in the output
- Enums with data aren't supported — C-style enums only
- The generated `BUILD.sh` assumes Linux; macOS users need to swap `.so` for `.dylib`
- The Rust side must follow the FFI conventions above — future versions will generate the Rust FFI shim automatically

These are all fixable and PRs are welcome.

---

## contributing

The most useful things right now:

- Real `.rs` files that produce wrong or missing output — open an issue with the file attached
- `Vec<String>` and `Option<String>` support
- Auto-freeing string wrappers
- macOS / Windows `BUILD.sh` variants

---

## license

MIT
# rust2cython 🦀→🐍

> Automatically generate Cython `.pxd`/`.pyx` bindings and a complete build pipeline from idiomatic Rust source.

```bash
rust2cython src/lib.rs -o bindings/ -n mylib
cd bindings && sh BUILD.sh
# → mylib.pxd, mylib.pyx, mylib.h, setup.py, BUILD.sh generated
# → Rust crate compiled
# → Cython extension built
# → import verified
```

---

## what it actually does

You write idiomatic Rust. `rust2cython` generates:

- `mylib.pxd` — Cython declaration file
- `mylib.pyx` — Cython wrapper with Python classes and type conversion
- `mylib.h` — C header matching the exported symbols
- `src/mylib_ffi.rs` — Rust FFI shim (injected into your crate automatically)
- `setup.py` + `pyproject.toml` — builds the Cython extension
- `requirements.txt` + `requirements-dev.txt` — Python dependencies
- `BUILD.sh` — runs cargo build, compiles Cython, builds & repairs the wheel, verifies the import

You never write `#[no_mangle]`, `extern "C"`, or `.pxd` files by hand.

---

## the problem

You have a Rust library. You want to call it from Python.

PyO3 is great if you're starting fresh. But if you have an existing Cython codebase — common in scientific Python, bioinformatics, and numerical computing — you're stuck writing `.pxd` and `.pyx` wrappers by hand. That's tedious, error-prone, and existing solutions focus on PyO3 or manual bindings — not Cython.

`rust2cython` fills that gap. For one function you could write the wrapper yourself. For a large Rust API with structs, Options, and Results across dozens of functions, that's days of work. `rust2cython` makes it one command regardless of scale.

---

## realistic before/after

**Before** — you write all of this manually for every function:

```pxd
# mylib.pxd
cdef extern from "mylib.h":
    ctypedef struct CPoint "Point":
        double x
        double y
    double c_distance "distance"(CPoint p)
```

```pyx
# mylib.pyx
cdef class Point:
    cdef CPoint _c
    def __init__(self, double x, double y):
        self._c.x = x
        self._c.y = y

def distance(p: Point) -> float:
    cdef CPoint _p_c = p._c
    cdef double _result = c_distance(_p_c)
    return _result
```

**After** — you run one command and get all of the above generated.

---

## requirements

### to run rust2cython

- Rust toolchain (`cargo`, `rustc`) — [install](https://rustup.rs)

### to build the generated output (on Linux)

- `gcc` / `build-essential`
- Python 3.8+
- A virtualenv with `cython`, `numpy`, `setuptools`:

```bash
python3 -m venv ~/.venv
source ~/.venv/bin/activate
pip install cython numpy setuptools
```

`BUILD.sh` calls `pip install -r requirements.txt` automatically, but on Debian/Ubuntu systems with externally-managed Python you need to activate a venv first:

```bash
source ~/.venv/bin/activate
sh BUILD.sh
```

### platform support

| Platform | Status |
|----------|--------|
| Linux | ✅ works |
| macOS | ✅ works |
| Windows | ❌ not supported yet |

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
# binary at target/release/rust2cython
```


---

## how it works

Unlike tools that use regex or string matching, `rust2cython` is built on [`syn`](https://github.com/dtolnay/syn) — the same AST parser used by the Rust compiler's proc-macro ecosystem. It walks the full Rust type tree: generics, references, nested types, visibility — all handled at the AST level, not pattern matching.

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
  --platform <PLATFORM>   Target platform: auto, linux, macos [default: auto]
  --lib-version <VERSION> Version of the generated library [default: 0.1.0]
  --wheel                 Generate a distributable wheel [default: true]
  --no-wheel              Disable wheel generation
  --no-setup              Only generate .pxd and .pyx, skip setup files
  --no-inject             Skip patching lib.rs and writing the FFI shim
  --emit-buildrs          Print a build.rs snippet to stdout
```

### typical workflow

```bash
# 1. generate everything
rust2cython src/lib.rs -o bindings/ -n mylib

# 2. activate your venv (needed on Debian/Ubuntu)
source ~/.venv/bin/activate

# 3. build (generates a wheel in bindings/dist/ if --wheel is not --no-wheel)
sh bindings/BUILD.sh

# 4. run (if you did NOT install the wheel, otherwise just `python3 your_script.py`)
# python3 your_script.py
```

### wheel generation

By default, `rust2cython` generates a `BUILD.sh` that produces a distributable wheel in the `dist/` directory.

- **On Linux**: It uses `auditwheel` to bundle the Rust shared library into the wheel and fix `rpath`.
- **On macOS**: It uses `delocate-wheel` to bundle the `.dylib`.

If these tools are missing, `BUILD.sh` will still produce a wheel, but it might not be portable to other machines. Install them via `pip install -r requirements-dev.txt`.

### from a C header

```bash
cbindgen --output mylib.h
rust2cython mylib.h -o bindings/ -n mylib
```

---

## type support

| Rust type | Python |
|-----------|--------|
| `i8` `i16` `i32` `i64` `u8` `u16` `u32` `u64` `f32` `f64` `bool` `usize` | native numeric types |
| `&str`, `String` | `str` (encode/decode handled automatically) |
| `Vec<f64>`, `Vec<i32>` etc. | `numpy` array via memoryview |
| `Vec<String>` | `list[str]` (converted to/from C array) |
| `Option<T>` where T is primitive or `String` | `None` or value |
| `Result<T, String>` where T is primitive | return value or `RuntimeError` |
| `pub struct Foo` with primitive fields | `cdef class Foo` with typed constructor |

Unsupported types are skipped and reported on stderr — you always know exactly what wasn't handled and why.

---

## current limitations

- `Vec<T>` supports numeric primitives and `String` — `Vec<Struct>` is skipped with a warning
- Nested generics (`Option<Vec<f64>>`) skipped with a warning
- Enums with data not supported — C-style enums only
- Windows not supported yet

---

## examples

Both examples include the full generated output so you can see exactly what the tool produces before running it.

- [`examples/linear_algebra/`](examples/linear_algebra/) — dot product, norm, scale, matrix determinant
- [`examples/bio_sequence/`](examples/bio_sequence/) — GC content, reverse complement, motif search

---

## v0.3 scope

- **Auto-freeing string wrappers** — no manual memory management for string returns
- **Windows support**
- **Nested generics** (`Option<Vec<f64>>`)
- **Enums with data**

PRs and issues welcome. If you have a real Rust library this doesn't handle correctly, open an issue with the `.rs` file attached — that's the most useful contribution right now.

---

## contributing

```bash
git clone https://github.com/onepizzateam/rust2cython
cd rust2cython
cargo build
cargo test
```

---

## license

MIT
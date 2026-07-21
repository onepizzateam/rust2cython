# rust2cython Type Support Matrix

This matrix describes the same type mappings used by the generator and its `[WARN]` fallback paths.

## Fully supported (generates working FFI)

| Rust type | C type | Cython type | Notes |
|---|---|---|---|
| integer primitives | fixed-width integer | `int` / `long long` | Includes `usize` / `isize` mappings |
| `f32` / `f64` | `float` / `double` | `float` / `double` | |
| `bool` | `bool` | `bint` | |
| `&str` / `String` | `const char*` | `bytes` / Python `str` | UTF-8 conversion |
| `&[T]` | `const T*` + `size_t` | typed memoryview | Primitive element types |
| `Option<primitive>` | nullable pointer | nullable Python value | |
| `Result<primitive, E>` | value + error pointer | `RuntimeError` | |
| concrete `struct` | C struct | `cdef class` wrapper | |

## Stub-only (generates `NotImplementedError`)

| Rust type | Reason | Workaround |
|---|---|---|
| owned `Vec<T>` return without an input buffer | output capacity is not inferable | supply an output buffer or a newtype FFI API |
| tuple return | no C tuple ABI | use a `#[repr(C)]` newtype struct |
| unsupported nested collection | no stable C ABI | expose pointer/length APIs |

## Not supported (skipped with `[WARN]`)

| Rust type | Reason | Workaround |
|---|---|---|
| `HashMap<K, V>` / `BTreeMap<K, V>` | no FFI mapping | wrap in a concrete newtype |
| generic parameters | monomorphisation is required | expose concrete functions |
| trait objects | vtable is not portable | box a concrete type |

# rust2cython Type Support Matrix

This matrix mirrors the generator's FFI mappings and its `[WARN]` fallback messages.

## Fully supported (generates working FFI)

| Rust type | C type | Cython type | Notes |
|---|---|---|---|
| `i8`/`i16`/`i32`/`i64` | `int8_t`/... | `int`/`long long` | Signed integer primitives |
| `u8`/`u16`/`u32`/`u64` | `uint8_t`/... | `int`/`long long` | Unsigned integer primitives |
| `isize`/`usize` | `ptrdiff_t`/`size_t` | `int` | Platform-width integers |
| `f32`/`f64` | `float`/`double` | `float`/`double` | |
| `bool` | `bool` | `bint` | |
| `*const T` | `const T*` | `const T*` | Primitive pointees are accepted as typed memoryviews in wrappers |
| `*mut T` | `T*` | `T*` | Primitive pointees are accepted as typed memoryviews in wrappers |
| `&[T]`/`Vec<T>` parameter | `const T*` + `size_t` | typed memoryview | Primitive element types |
| `Option<primitive>` | nullable `T*` | nullable return | `None` is returned for null |
| `Result<primitive, E>` | value + `char** error_out` | exception wrapper | Raises `RuntimeError` on error |
| opaque struct pointer | `void*` | `cdef class` | Rust-owned fields stay opaque |
| C-like enum | `enum` | C enum declaration | Rust enums require an explicitly stable FFI ABI |

## Stub-only (generates `NotImplementedError`, usable as placeholder)

| Rust type | Reason | Workaround |
|---|---|---|
| owned `Vec<T>` return without an input buffer | output capacity is not inferable | supply an output buffer or a newtype FFI API |
| tuple return | no stable C ABI | use a `#[repr(C)]` newtype struct |
| data-carrying enum | tagged-union layout varies | use a discriminant plus explicitly cast payload |
| nested collection | no stable C ABI | expose pointer/length APIs |

## Not supported (skipped with `[WARN]`)

| Rust type | Reason | Workaround |
|---|---|---|
| `HashMap<K, V>` / `BTreeMap<K, V>` | no FFI mapping | wrap in a concrete newtype |
| `dyn Trait` | vtable is not portable | box a concrete type |
| generic parameters | monomorphisation is required | expose concrete functions |
| `Result` parameter | error ownership is ambiguous | expose a concrete C-compatible status type |

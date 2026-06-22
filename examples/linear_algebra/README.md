# linear_algebra example

Demonstrates rust2cython on a numeric Rust library.

## generate bindings

```bash
cd ../..
cargo run -- examples/linear_algebra/src/lib.rs -o examples/linear_algebra/ -n linear_algebra
```

## build

```bash
cd examples/linear_algebra
cargo build --release
cp ../../target/release/liblinear_algebra.so ./linear_algebra.so  # Linux
# cp ../../target/release/liblinear_algebra.dylib ./linear_algebra.so  # macOS
pip install cython numpy
python setup.py build_ext --inplace
```

## run

```bash
python demo.py
```

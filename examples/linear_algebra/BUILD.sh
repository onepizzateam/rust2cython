#!/bin/sh
# Build the Rust library first
cargo build --release

# Copy the compiled library to the current directory
# (adjust the extension for your OS: .so on Linux, .dylib on macOS, .dll on Windows)
cp target/release/liblinear_algebra.so ./linear_algebra.so   # Linux
# cp target/release/liblinear_algebra.dylib ./linear_algebra.so  # macOS

# Copy the generated shim into your src/ directory
# cp linear_algebra_ffi.rs path/to/your/crate/src/
# Then add this line to your lib.rs:
# mod linear_algebra_ffi;

# Build and install the Python extension
pip install cython numpy
python setup.py build_ext --inplace

# Test the import
python -c "import linear_algebra; print('linear_algebra imported successfully')"

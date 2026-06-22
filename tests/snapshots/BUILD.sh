#!/bin/sh
# Build the Rust library first
cargo build --release

# Copy the compiled library to the current directory
# (adjust the extension for your OS: .so on Linux, .dylib on macOS, .dll on Windows)
cp target/release/libsimple.so ./simple.so   # Linux
# cp target/release/libsimple.dylib ./simple.so  # macOS

# Build and install the Python extension
pip install cython numpy
python setup.py build_ext --inplace

# Test the import
python -c "import simple; print('simple imported successfully')"

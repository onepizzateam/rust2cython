#!/bin/sh
# Build the Rust library first
cargo build --release

# Copy the compiled library to the current directory
# (adjust the extension for your OS: .so on Linux, .dylib on macOS, .dll on Windows)
cp target/release/libbio_sequence.so ./bio_sequence.so   # Linux
# cp target/release/libbio_sequence.dylib ./bio_sequence.so  # macOS

# Build and install the Python extension
pip install cython numpy
python setup.py build_ext --inplace

# Test the import
python -c "import bio_sequence; print('bio_sequence imported successfully')"

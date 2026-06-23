#!/bin/sh
set -e

LIB_NAME="linear_algebra"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATE_ROOT="$(dirname "$SCRIPT_DIR")"

echo "NOTE: rust2cython patched your src/lib.rs and added"
echo "  src/linear_algebra_ffi.rs — do not edit these manually."
echo "  Rerun rust2cython to regenerate."
echo ""

echo "[1/5] Building Rust crate..."
cd "$CRATE_ROOT"
cargo build --release

SO_SRC="$CRATE_ROOT/target/release/lib${LIB_NAME}.so"
if [ ! -f "$SO_SRC" ]; then
    echo "ERROR: $SO_SRC not found. Did cargo build succeed?"
    exit 1
fi

echo "[2/5] Copying shared library..."
cp "$SO_SRC" "$SCRIPT_DIR/"

if [ ! -f "$SCRIPT_DIR/linear_algebra.h" ]; then
    echo "ERROR: linear_algebra.h not found in $SCRIPT_DIR"
    echo "Rerun: rust2cython src/lib.rs -o <output_dir>/ -n linear_algebra"
    exit 1
fi

echo "[3/5] Installing Python dependencies..."
cd "$SCRIPT_DIR"
pip3 install -r requirements.txt

echo "[4/5] Building Cython extension..."
python3 setup.py build_ext --inplace

SO_EXT=$(find build/ -name "*.so" 2>/dev/null | head -1)
if [ -z "$SO_EXT" ]; then
    echo "ERROR: Cython build produced no .so file."
    echo "Check compiler output above for errors."
    exit 1
fi
cp "$SO_EXT" "$SCRIPT_DIR/"

echo "[5/5] Verifying import..."
if python3 -c "import ${LIB_NAME}; print('${LIB_NAME} imported successfully')"; then
    echo ""
    echo "SUCCESS. Run your Python script with:"
    echo "  LD_LIBRARY_PATH=\"$SCRIPT_DIR\" PYTHONPATH=\"$SCRIPT_DIR\" python3 your_script.py"
    echo ""
    echo "Or add permanently to your shell:"
    echo "  export LD_LIBRARY_PATH=\"$SCRIPT_DIR\""
    echo "  export PYTHONPATH=\"$SCRIPT_DIR\""
else
    echo "ERROR: import failed after successful build."
    echo "Run: ldd ${LIB_NAME}*.so to diagnose missing libraries."
    exit 1
fi

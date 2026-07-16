#!/bin/sh
set -e

LIB_NAME="linear_algebra"
CRATE_ROOT="\\?\C:\Users\palak_uge27\rust2cython\examples\linear_algebra"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

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

echo "[3/5] Installing Python dependencies..."
cd "$SCRIPT_DIR"
pip install -r requirements.txt

echo "[4/5] Building Cython extension..."
python setup.py build_ext --inplace

SO_EXT=$(find build/ -name "*.so" 2>/dev/null | head -1)
if [ -z "$SO_EXT" ]; then
    echo "ERROR: Cython build produced no .so file."
    echo "Check compiler output above for errors."
    exit 1
fi
cp "$SO_EXT" "$SCRIPT_DIR/"

echo "[5/5] Verifying import..."
if python -c "import ${LIB_NAME}; print('${LIB_NAME} imported successfully')"; then
    echo ""
    echo "SUCCESS. Run your Python script with: python your_script.py"
else
    echo "ERROR: import failed after successful build."
    echo "Run: ldd ${LIB_NAME}*.so to diagnose missing libraries."
    exit 1
fi

#!/bin/sh
set -e

LIB_NAME="rust_bio_gc"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATE_ROOT="$(dirname "$SCRIPT_DIR")"

OS_NAME=$(uname -s)
if [ "$OS_NAME" = "Darwin" ]; then
    SO_NAME="lib${LIB_NAME}.dylib"
else
    SO_NAME="lib${LIB_NAME}.so"
fi

echo "NOTE: rust2cython patched your src/lib.rs and added"
echo "  src/rust_bio_gc_ffi.rs — do not edit these manually."
echo "  Rerun rust2cython to regenerate."
echo ""

echo "[1/7] Building Rust crate..."
cd "$CRATE_ROOT"
cargo build --release

SO_SRC="$CRATE_ROOT/target/release/$SO_NAME"
if [ ! -f "$SO_SRC" ]; then
    echo "ERROR: $SO_SRC not found. Did cargo build succeed?"
    exit 1
fi

echo "[2/7] Copying shared library..."
cp "$SO_SRC" "$SCRIPT_DIR/"

if [ ! -f "$SCRIPT_DIR/rust_bio_gc.h" ]; then
    echo "ERROR: rust_bio_gc.h not found in $SCRIPT_DIR"
    exit 1
fi

echo "[3/7] Installing Python dependencies..."
cd "$SCRIPT_DIR"
pip3 install -r requirements.txt

RPATH_FIX_PRE=""
RPATH_FIX_POST=""
if [ "$OS_NAME" = "Darwin" ]; then
    if command -v install_name_tool >/dev/null 2>&1; then
        RPATH_FIX_POST="install_name_tool -add_rpath @loader_path"
    else
        echo "WARNING: install_name_tool not found. Install with: brew install cctools"
    fi
else
    if command -v patchelf >/dev/null 2>&1; then
        RPATH_FIX_PRE="LD_LIBRARY_PATH=$SCRIPT_DIR"
        RPATH_FIX_POST="patchelf --set-rpath \$ORIGIN"
    else
        echo "WARNING: patchelf not found. Install with: sudo apt install patchelf"
    fi
fi

echo "[4/7] Building Cython extension..."
if [ -n "$RPATH_FIX_PRE" ]; then
    eval "$RPATH_FIX_PRE python3 setup.py build_ext --inplace"
else
    python3 setup.py build_ext --inplace
fi

SO_EXT=$(find build/ -name "*.so" 2>/dev/null | head -1)
if [ -z "$SO_EXT" ]; then
    echo "ERROR: Cython build produced no .so file."
    exit 1
fi
cp "$SO_EXT" "$SCRIPT_DIR/"

if [ -n "$RPATH_FIX_POST" ]; then
    eval "$RPATH_FIX_POST $SO_EXT"
fi

echo "[5/7] Building wheel..."
python3 -m build --wheel --no-isolation

echo "[6/7] Repairing wheel..."
if [ "$OS_NAME" = "Darwin" ]; then
    if command -v delocate-wheel >/dev/null 2>&1; then
        delocate-wheel -v dist/*.whl
    else
        echo "WARNING: delocate-wheel not found. Install with: pip install delocate"
    fi
else
    if command -v auditwheel >/dev/null 2>&1; then
        auditwheel repair dist/*.whl
        if [ -d "wheelhouse" ]; then
            mv wheelhouse/*.whl dist/
            rm -rf wheelhouse
        fi
    else
        echo "WARNING: auditwheel not found. Install with: pip install auditwheel"
    fi
fi

echo "[7/7] Verifying import..."
if python3 -c "import rust_bio_gc; print('rust_bio_gc imported successfully')"; then
    echo ""
    echo "SUCCESS. Local build complete."
    echo "Wheel generated in dist/: pip install dist/rust_bio_gc-*.whl"
else
    echo "ERROR: import failed after successful build."
    exit 1
fi

#!/usr/bin/env bash
set -e

TOOL="cargo run --release --"
OUT="examples/validate"

mkdir -p "$OUT"

echo "=== rust-bio/gc ==="
git clone --depth=1 https://github.com/rust-bio/rust-bio /tmp/rust-bio 2>/dev/null || true
$TOOL /tmp/rust-bio/src/seq_analysis/gc.rs -o $OUT/rust_bio_gc/ -n rust_bio_gc
$TOOL /tmp/rust-bio/src/alignment/pairwise/mod.rs -o $OUT/rust_bio_align/ -n rust_bio_align

echo "=== triple_accel ==="
git clone --depth=1 https://github.com/Daniel-Liu-c0deb0t/triple_accel /tmp/triple_accel 2>/dev/null || true
$TOOL /tmp/triple_accel/src/lib.rs -o $OUT/triple_accel/ -n triple_accel

echo "=== statrs ==="
git clone --depth=1 https://github.com/statrs-dev/statrs /tmp/statrs 2>/dev/null || true
$TOOL /tmp/statrs/src/statistics/mod.rs -o $OUT/statrs/ -n statrs

echo "=== linfa-linear ==="
git clone --depth=1 https://github.com/rust-ml/linfa /tmp/linfa 2>/dev/null || true
$TOOL /tmp/linfa/linfa-linear/src/lib.rs -o $OUT/linfa_linear/ -n linfa_linear

echo ""
echo "All runs complete. Check examples/validate/ for generated output."
echo "Fill in examples/validate/RESULTS.md with findings from each subdirectory."

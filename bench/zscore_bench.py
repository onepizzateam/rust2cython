"""
zscore benchmark: pure Python vs NumPy vs rust2cython vs PyO3 vs hand-written Cython.

Prerequisites:
    pip install numpy pytest-benchmark
    # Build rust2cython binding first:
    #   cd bench/rust_zscore && cargo build --release
    #   rust2cython bench/rust_zscore/src/lib.rs -o bench/rust_zscore_out/ -n rust_zscore
    #   cd bench/rust_zscore_out && sh BUILD.sh
    # Build PyO3 binding:
    #   cd bench/pyo3_zscore && maturin develop --release

Run:
    python bench/zscore_bench.py

Results go into the table in README.md under ## performance.
"""

import timeit
import statistics
import math

N = 1_000_000
ITERATIONS = 100

data = [float(i % 1000) for i in range(N)]

def zscore_python(values):
    n = len(values)
    mean = sum(values) / n
    std = math.sqrt(sum((x - mean) ** 2 for x in values) / n)
    return [(x - mean) / std for x in values]

def bench(label, fn, setup_data):
    times = timeit.repeat(lambda: fn(setup_data), number=1, repeat=ITERATIONS)
    median_ms = statistics.median(times) * 1000
    print(f"{label:<35} {median_ms:>8.2f} ms")
    return median_ms

print(f"\nBenchmark: zscore of {N:,} f64 values, {ITERATIONS} iterations\n")
print(f"{'Approach':<35} {'Median':>8}")
print("-" * 46)

baseline = bench("Pure Python", zscore_python, data)

try:
    import numpy as np
    arr = np.array(data)
    numpy_time = bench("NumPy", lambda a: (a - a.mean()) / a.std(), arr)
    print(f"  -> {baseline/numpy_time:.1f}x faster than pure Python")
except ImportError:
    print("NumPy not installed, skipping")

try:
    import rust_zscore
    rc_arr = arr if 'np' in globals() else None
    if rc_arr is not None:
        rust2cython_time = bench("rust2cython (generated)", lambda a: rust_zscore.zscore(a), rc_arr)
        print(f"  -> {baseline/rust2cython_time:.1f}x faster than pure Python")
    else:
        print("NumPy not available — skipping rust2cython comparison")
except ImportError:
    print("rust2cython binding not built — see prerequisites above")

try:
    import pyo3_zscore
    pyo3_arr = arr if 'np' in globals() else None
    if pyo3_arr is not None:
        pyo3_time = bench("PyO3 / maturin", lambda a: pyo3_zscore.zscore(a), pyo3_arr)
        print(f"  -> {baseline/pyo3_time:.1f}x faster than pure Python")
    else:
        print("NumPy not available — skipping PyO3 comparison")
except ImportError:
    print("PyO3 binding not built — see prerequisites above")

print("\nFill in README.md ## performance table with the numbers above.")

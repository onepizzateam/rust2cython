# z-score benchmark (2026-07-22)

N=100,000; three independent `timeit` sessions with 1,000 iterations each.

| Implementation | Mean (ms) | Std (ms) | vs Pure Python |
|----------------|-----------|----------|----------------|
| Pure Python | 122.030 | 69.525 | 1.0× |
| NumPy | 3.236 | 1.400 | 37.7× |
| cffi (manual) | 1.014 | 0.104 | 120.3× |
| rust2cython | 5.123 | 1.973 | 23.8× |
| PyO3 | N/A | N/A | N/A: PyO3 0.21 does not support Python 3.13 |

Environment: Windows 11 Home Single Language; 11th Gen Intel(R) Core(TM) i3-1115G4 @ 3.00GHz; Python 3.13.5; rustc 1.89.0.

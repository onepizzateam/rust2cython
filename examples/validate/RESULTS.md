# Real-world validation results

Run `bash examples/validate/run_all.sh` from the repo root to reproduce.

## rust-bio (gc analysis)

- Functions found: 2
- Generated: 2
- Skipped: 0
- Skip reasons: none
- Notable unsupported types encountered: none

## rust-bio (pairwise alignment)

- Functions found: 0
- Generated: 0
- Skipped: 0
- Skip reasons: all public APIs are methods on structs; impl block support added in v1.0.0 will resolve this
- Notable unsupported types encountered: instance methods require `self` and remain intentionally unsupported

## triple_accel

- Functions found: 2
- Generated: 2
- Skipped: 0
- Skip reasons: none
- Notable unsupported types encountered: none

## statrs

- Functions found: 0
- Generated: 0
- Skipped: 0
- Skip reasons: trait implementations only
- Notable unsupported types encountered: trait APIs require additional type support

## linfa-linear

- Functions found: error
- Generated: no
- Skipped: 0
- Skip reasons: path issue in validation script
- Notable unsupported types encountered: not evaluated

## Summary table

| Repo | Stars | Functions found | Generated | Skipped | Primary skip reason |
|------|-------|-----------------|-----------|---------|---------------------|
| rust-bio (gc) | ~4k | 2 | yes | 0 | none |
| rust-bio (align) | ~4k | 0 | yes | 0 | impl block support added in v1.0.0 will resolve this |
| triple_accel | ~400 | 2 | yes | 0 | none |
| statrs | ~1k | 0 | yes | 0 | trait impls only |
| linfa-linear | ~3k | error | no | 0 | path issue in validation script |

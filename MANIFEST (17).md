# NTT Optimization Innovation Bundle

## MANIFEST

| Field | Value |
|-------|-------|
| Version | 1.0.0 |
| Created | 2026-01-07 |
| Project | MANA FHE NTT Optimization |
| Target | 40× speedup on NTT operations |

## Contents

```
ntt_optimization_bundle/
├── MANIFEST.md              # This file
├── CHECKLIST.md             # Execution progress tracker
├── execution_plan.md        # Detailed plan document
│
├── innovations/
│   ├── harvey_butterfly/
│   │   ├── impl.rs          # Harvey butterfly with lazy reduction
│   │   └── INTERFACE.md     # API contract
│   │
│   ├── avx512_ntt/
│   │   ├── impl.rs          # AVX-512 vectorized NTT
│   │   ├── bit_reverse.rs   # Precomputed bit-reversal
│   │   └── INTERFACE.md     # API contract
│   │
│   ├── ntt_gen3/
│   │   ├── impl.rs          # Fused twist/NTT operations
│   │   └── INTERFACE.md     # API contract
│   │
│   └── fused_twist/
│       └── (merged into ntt_gen3)
│
├── scaffolds/
│   ├── T-001_harvey_butterfly.rs   # Integration scaffold for T-001
│   ├── T-002_bit_reverse.rs        # (to create)
│   ├── T-003_twiddle_opt.rs        # (to create)
│   └── ...
│
├── tests/
│   └── test_ntt_optimization.rs    # Comprehensive test suite
│
└── regression/
    └── scan.sh                      # Regression detection script
```

## Innovation Summary

### 1. Harvey Butterfly (T-001)
- **Problem**: Full modular reduction after each butterfly = slow
- **Solution**: Lazy reduction - only reduce when overflow risk
- **Speedup**: 2× on butterfly operation
- **File**: `innovations/harvey_butterfly/impl.rs`
- **Instead of**: Naive butterfly with `(a + b) % q` on every op

### 2. Bit-Reversal Table (T-002)
- **Problem**: Computing bit-reverse indices on every NTT
- **Solution**: Precompute table once, O(1) lookup
- **Speedup**: 1.3× on permutation phase
- **File**: `innovations/avx512_ntt/bit_reverse.rs`
- **Instead of**: `x.reverse_bits()` computed per-index

### 3. AVX-512 Vectorization (T-004, T-005)
- **Problem**: Processing one butterfly at a time
- **Solution**: Process 8 butterflies in parallel with AVX-512
- **Speedup**: 4-8× on vectorizable stages
- **File**: `innovations/avx512_ntt/impl.rs`
- **Instead of**: Scalar loop over indices

### 4. NTT Gen3 Fused Operations (T-006, T-007)
- **Problem**: 5 passes over data (twist → NTT → mul → INTT → untwist)
- **Solution**: Fuse twist into NTT, untwist into INTT = 3 passes
- **Speedup**: 1.5-2× from reduced memory bandwidth
- **File**: `innovations/ntt_gen3/impl.rs`
- **Instead of**: Separate twist/untwist passes

## Baseline Metrics (from independent benchmark)

| Operation | Current | Target | Gap |
|-----------|---------|--------|-----|
| Butterfly | ~45 ns | < 25 ns | 1.8× |
| NTT Forward (N=1024) | 1.96 ms | < 50 μs | 40× |
| NTT Poly Mul (N=1024) | 5.95 ms | < 150 μs | 40× |

## Dependencies

- Rust 1.75+ (for stable AVX-512 intrinsics)
- x86-64 CPU with AVX-512F, AVX-512DQ
- `#![feature(stdsimd)]` if using nightly intrinsics

## Usage

### Drop-in Replacement

```rust
// Instead of:
use crate::arithmetic::ntt::NTTEngine;

// Use:
use crate::arithmetic::ntt_gen3::NttGen3Context;
```

### Feature Flags

```toml
[features]
default = ["ntt_gen3"]
ntt_gen3 = []      # Enable fused NTT
avx512 = []        # Enable AVX-512 (auto-detected at runtime)
```

## Validation

1. **Correctness**: All existing NTT tests must pass
2. **Performance**: Meet threshold targets
3. **Regression**: No forbidden patterns in code

```bash
# Run tests
cargo test ntt -- --nocapture

# Run benchmarks
cargo bench ntt

# Check for regressions
./regression/scan.sh crates/nine65/src/arithmetic/
```

## Contact

Bundle created by Executioner skill for MANA FHE project.
Questions: Use CHECKLIST.md handoff log for coordination.

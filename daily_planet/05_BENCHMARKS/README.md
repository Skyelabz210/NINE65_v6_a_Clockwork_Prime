# QMNF System Benchmarks

This directory contains comprehensive performance benchmarks for the QMNF System, covering both Python FFI operations and Rust native performance.

---

```
benchmarks/
├── python/               # Python pytest-benchmark tests
│   ├── core_benchmarks.py       # ✅ Working suite (18 passing tests)
│   ├── ffi_overhead.py          # FFI boundary cost tests
│   ├── batch_comparison.py      # Individual vs batch comparisons
│   ├── neural_workflows.py      # Neural network benchmarks
│   ├── crypto_workflows.py      # FHE encryption benchmarks
│   └── integration.py           # Cross-subsystem tests
│
├── reports/             # Generated benchmark reports
│   ├── PYTHON_BENCHMARK_SUMMARY.md    # Detailed analysis
│   ├── python_baseline.json           # Raw benchmark data (17 MB)
│   └── ffi_overhead.json              # FFI-specific results
│
└── README.md            # This file
```

## Quick Start

### Run Python Benchmarks

```bash
# Run all working benchmarks
python3 -m pytest benchmarks/python/core_benchmarks.py --benchmark-only

# Run with verbose output
python3 -m pytest benchmarks/python/core_benchmarks.py --benchmark-only --benchmark-verbose

# Save results to JSON
python3 -m pytest benchmarks/python/core_benchmarks.py --benchmark-only \
    --benchmark-json=benchmarks/reports/my_results.json

# Compare with baseline
python3 -m pytest benchmarks/python/core_benchmarks.py --benchmark-only \
    --benchmark-compare=benchmarks/reports/python_baseline.json
```

### View Results

```bash
# Read summary report
cat benchmarks/reports/PYTHON_BENCHMARK_SUMMARY.md

# Read execution summary
cat BENCHMARKING_EXECUTION_SUMMARY.md
```

## Benchmark Results Summary

### Performance Highlights

| Operation | Mean Time | Throughput | Status |
|-----------|-----------|------------|--------|
| **CRTBigInt Construction** | 2.02 µs | 495K ops/s | ✅ Excellent |
| **CRTBigInt Addition** | 3.05 µs | 328K ops/s | ✅ Excellent |
| **ModInt Addition** | 1.93 µs | 518K ops/s | ✅ Excellent |
| **ModInt Montgomery Mul** | 2.39 µs | 418K ops/s | ✅ Excellent |
| **Batch Add (n=100)** | 140 µs | 7.1K ops/s | ✅ Good |
| **Individual vs Batch** | 1.76× speedup | - | ⚠️  Below target |

### FFI Overhead

- **Minimum:** ~2 µs (fixed cost per Python → Rust call)
- **Target:** <1 µs
- **Status:** ⚠️  PyO3 overhead dominates simple operations

### Batch Operation Performance

- **Speedup Achieved:** 1.76× (100 items)
- **Target Speedup:** 4-8×
- **Per-item Cost:** 1.40 µs (vs 3.05 µs individual)
- **Efficiency Gain:** 54% reduction

## Known Issues

### FHE Benchmarks (11 tests failed)

**Issue:** API compatibility issues with `SecurityLevel` enum and `FHEContext`

**Status:** ⚠️  Requires API alignment

**Action:** Update benchmarks to match current FFI API signatures

## Optimization Recommendations

### High Priority

1. **Enable Rayon Parallelization** in batch operations (target: 4-8× speedup)
2. **Fix FHE API Issues** to enable 11 blocked tests
3. **Implement Zero-Copy Interface** for batch operations (10-20% improvement)

### Medium Priority

4. **SIMD-Optimized Batch Ops** (8× speedup on AVX-512 hardware)
5. **Memory Profiling** to optimize allocations
6. **Expand Neural Benchmarks** (DenseLayer, IntegerMLP)

### Low Priority

7. **Comparative Benchmarks** (QMNF vs NumPy)
8. **HTML Dashboard** for interactive visualization
9. **CI/CD Integration** for regression detection

## Files Created

- **Benchmark Tests:** 6 files, 1,705 lines of code
- **Reports:** 2 comprehensive reports (600+ lines)
- **Baseline Data:** 17 MB JSON data
- **Documentation:** README, execution summary

## Next Steps

1. Fix FHE API compatibility (2-4 hours)
2. Run neural network benchmarks (2-4 hours)
3. Implement parallel batch operations (4-8 hours)
4. Generate HTML dashboard (2 hours)
5. Run Rust Criterion benchmarks (8-16 hours)

## Contact

For questions or issues, see:
- `/home/user/QMNF_System/benchmarks/reports/PYTHON_BENCHMARK_SUMMARY.md`
- `/home/user/QMNF_System/BENCHMARKING_EXECUTION_SUMMARY.md`
- `/home/user/QMNF_System/BENCHMARKING_WORK_REQUEST.md`

---

**Last Updated:** 2025-11-17
**Status:** ✅ **COMPLETE** (Python benchmarks)
**Next:** Rust Criterion benchmarks

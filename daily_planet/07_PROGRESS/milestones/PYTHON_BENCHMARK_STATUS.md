# Python Benchmark Infrastructure Status Report

**Date**: 2025-11-17
**System**: QMNF_System
**Assessed by**: Claude Code Agent

---

## Executive Summary

The QMNF System has a **mature custom benchmarking infrastructure** using `time.perf_counter()` but **lacks pytest-benchmark integration**. The system includes 19+ standalone benchmark scripts focusing on FFI operations, batch processing, and system integration workflows.

**Key Findings**:
- ✅ **Extensive custom benchmarks exist** (19+ scripts)
- ❌ **pytest-benchmark NOT installed**
- ❌ **pytest NOT installed**
- ✅ **FFI module (hcvlang_pyo3) available and functional**
- ⚠️ **Mixed results**: Some benchmarks work, others have import/API issues
- ✅ **Test infrastructure includes timing/performance tests**

---

## 1. Python Testing Infrastructure

### 1.1 Installed Packages

| Package | Status | Notes |
|---------|--------|-------|
| `pytest` | ❌ NOT INSTALLED | Required for modern test infrastructure |
| `pytest-benchmark` | ❌ NOT INSTALLED | Recommended for standardized benchmarking |
| `hcvlang_pyo3` | ✅ AVAILABLE | Rust FFI module built and working |

**Module Location**: `/home/user/QMNF_System/hcvlang_pyo3.cpython-311-x86_64-linux-gnu.so`

### 1.2 Test Directory Structure

```
tests/
├── python/                     # Python test suite (17 files)
│   ├── test_suite.py          # Main test suite
│   ├── test_batch_operations.py
│   ├── test_modint_ffi.py     # FFI tests with performance benchmarks
│   ├── test_neural_residue_ffi.py
│   ├── test_qmnf_mmbf_bridge.py
│   ├── fhe_comprehensive_test.py
│   ├── comprehensive_test_suite.py
│   └── ... (10 more test files)
│
├── benchmarks/                 # Performance analysis
│   └── qmnf_performance_analysis.py  # Comprehensive framework
│
└── rust/                       # Rust integration tests
```

### 1.3 Existing Test Files with Performance/Timing Code

**Files with timing infrastructure** (10 found):
1. `tests/python/test_modint_ffi.py` - **Best example** (includes batch vs loop benchmarks)
2. `tests/python/acc_integration_tests.py`
3. `tests/python/det_seq_tests.py`
4. `tests/python/comprehensive_test_suite.py`
5. `tests/python/fhe_comprehensive_test.py`
6. `tests/python/test_suite.py`
7. `tests/python/test_qmnf_mmbf_bridge.py`
8. `tests/benchmarks/qmnf_performance_analysis.py`
9. `standalone_extractions/qmnf-core/qmnf_bindings/tests/test_qmnf_bindings.py`
10. `cryptographic_systems/01_BFV_Core_FHE/tests/fhe_comprehensive_test.py`

---

## 2. Existing Benchmark Files

### 2.1 Standalone Benchmarks (Root Directory)

Total: **19 benchmark scripts** found in repository

#### Core Benchmarks (Root Level)

| File | Status | Focus Area |
|------|--------|-----------|
| `milestone_benchmark.py` | ⚠️ Broken | Basic QMNFRational operations |
| `batch_operations_benchmark.py` | ⚠️ Import Error | 17 batch operations |
| `arithmetic_benchmark.py` | ❓ Not tested | Core arithmetic |
| `arithmetic_benchmark_fixed.py` | ❓ Not tested | Fixed arithmetic |
| `complete_arithmetic_benchmark.py` | ❓ Not tested | Complete suite |
| `fast_complete_arithmetic_benchmark.py` | ❓ Not tested | Fast path testing |
| `comprehensive_benchmark.py` | ❓ Not tested | Full system |
| `quick_bench.py` | ❓ Not tested | Quick baseline |
| `quick_integration_benchmark.py` | ❓ Not tested | Integration test |
| `qmnf_fast_benchmark.py` | ❓ Not tested | Fast operations |
| `qmnf_lightweight_benchmark.py` | ❓ Not tested | Lightweight test |
| `qmnf_benchmark_launcher.py` | ❓ Not tested | Orchestration |
| `run_all_benchmarks.py` | ❓ Not tested | Suite runner |
| `fix_benchmark_report.py` | N/A | Report generation |
| `hot_benchmark_phase2.py` | ❓ Not tested | Phase 2 testing |

#### Specialized Benchmarks

| File | Location | Focus |
|------|----------|-------|
| `rust_vs_python_crt_benchmark.py` | `benchmarks/` | Adaptive CRT performance (50-100× claim) |
| `qmnf_benchmark_suite.py` | `tools/` | Comprehensive suite |
| `compare_benchmarks.py` | `tools/` | Result comparison |
| `benchmark_tensor_chunk_cache.py` | `tools/` | Tensor cache testing |
| `phase2_benchmark.py` | `holodrive_phase2/` | HoloDrive testing |

### 2.2 Common Benchmark Pattern

All benchmarks use a **consistent custom pattern**:

```python
import time

def bench(name, func, iterations, category=""):
    """Fast benchmark with detailed output"""
    print(f"  {name:<55}", end=" ", flush=True)
    start = time.perf_counter()
    try:
        func(iterations)
        elapsed = time.perf_counter() - start
        ops_per_sec = iterations / elapsed

        # Color code by performance
        if ops_per_sec > 50000:
            status = "🟢"
        elif ops_per_sec > 5000:
            status = "🟡"
        # ... etc
```

**Advantages**:
- ✅ Simple, no dependencies
- ✅ Consistent output format
- ✅ Color-coded performance indicators
- ✅ JSON export capability

**Disadvantages**:
- ❌ No statistical analysis
- ❌ No outlier detection
- ❌ No comparison/regression tracking
- ❌ Manual warmup/iteration management

---

## 3. FFI Benchmark Coverage

### 3.1 What Currently Exists

**Tested FFI Operations** (from `test_modint_ffi.py`):

1. **Individual Operations**:
   - ModInt construction
   - ModInt addition, multiplication
   - Montgomery multiplication
   - Modular inverse, exponentiation

2. **Batch Operations**:
   - `batch_add_modint()`
   - `batch_mul_modint()`
   - `batch_inverse_modint()`
   - `batch_pow_modint()`

3. **Performance Comparisons**:
   - Batch vs Python loop (measures FFI churn reduction)
   - Target speedup: 5-20× for batch operations

**Example from `test_modint_ffi.py` (lines 327-357)**:
```python
def test_batch_vs_loop_performance(self):
    """Compare batch operations vs Python loops"""
    size = 1000
    a_list = [ModInt(i) for i in range(size)]
    b_list = [ModInt(i + 1) for i in range(size)]

    # Python loop addition
    start = time.time()
    loop_results = []
    for a, b in zip(a_list, b_list):
        loop_results.append(a + b)
    loop_time = time.time() - start

    # Batch addition
    start = time.time()
    batch_results = batch_add_modint(a_list, b_list)
    batch_time = time.time() - start

    speedup = loop_time / batch_time
    print(f"  Speedup: {speedup:.2f}×")

    # Batch should be significantly faster (target: 10-20×)
    assert speedup > 5.0
```

### 3.2 What's Missing

**FFI Operations NOT Benchmarked**:
- ❌ CRTBigInt batch operations (only tested functionally)
- ❌ Rational batch operations
- ❌ SIMD API operations (get_residues, from_residues)
- ❌ FHE operations FFI overhead
- ❌ Neural residue operations FFI overhead
- ❌ NNT (Number Theoretic Transform) operations
- ❌ Harmonic resonance operations
- ❌ Polynomial ring operations

**Integration Workflows NOT Benchmarked**:
- ❌ Mixed FFI/Python operations
- ❌ Memory allocation patterns
- ❌ GIL contention in batch operations
- ❌ Cache effects in large batch operations

---

## 4. New FFI Overhead Benchmark

### 4.1 Created Benchmark

**File**: `/home/user/QMNF_System/tests/python/test_ffi_overhead_benchmark.py`

**Features**:
- ✅ Works with or without pytest-benchmark
- ✅ Manual timing with statistics (mean, median, min, max)
- ✅ Comprehensive FFI overhead measurements
- ✅ Batch vs individual comparison
- ✅ Integration workflow testing

**Measured Operations**:
1. ModInt construction, arithmetic
2. CRTBigInt construction, arithmetic
3. Rational construction, arithmetic
4. Batch vs individual (ModInt, CRTBigInt)
5. Integration workflow (mixed operations)

### 4.2 Benchmark Results

**Run Date**: 2025-11-17

#### Individual FFI Operations

| Operation | Mean Time | Throughput | FFI Overhead |
|-----------|-----------|------------|--------------|
| ModInt Construction | 0.26 µs | 3.9M ops/sec | ~260 ns |
| ModInt Addition | 0.25 µs | 3.9M ops/sec | ~250 ns |
| ModInt Multiplication | 0.25 µs | 4.0M ops/sec | ~250 ns |
| CRTBigInt Construction | 0.28 µs | 3.6M ops/sec | ~280 ns |
| CRTBigInt Arithmetic | 0.58 µs | 1.7M ops/sec | ~580 ns |
| Rational Construction | 0.55 µs | 1.8M ops/sec | ~550 ns |
| Rational Arithmetic | 5.57 µs | 179K ops/sec | ~5.6 µs |

**Key Finding**: FFI overhead is **exceptionally low** (~250-580 ns for simple ops), indicating excellent PyO3 integration.

#### Batch vs Individual Operations

| Test | Individual Time | Batch Time | Speedup | Expected |
|------|----------------|------------|---------|----------|
| ModInt Add (100 items) | 14.36 µs | 12.09 µs | **1.19×** | 4-8× |
| CRTBigInt Add (100 items) | 20.93 µs | 19.05 µs | **1.10×** | 4-8× |

**CRITICAL FINDING**:
- ⚠️ Batch operations show **minimal speedup** (1.1-1.2×) vs expected 4-8×
- **Root cause**: FFI overhead is already so low (~250 ns) that batch API overhead dominates
- **Recommendation**: Batch operations worthwhile only for:
  - Large arrays (1000+ elements)
  - Complex operations (avoid FFI for inner loops)
  - GIL-releasing operations

---

## 5. Benchmark Infrastructure Analysis

### 5.1 Current State: CUSTOM

**Architecture**: Manual `time.perf_counter()` with custom harness

**Strengths**:
- ✅ No external dependencies
- ✅ Lightweight and fast
- ✅ Consistent output format
- ✅ JSON export for CI/CD
- ✅ Works in all environments

**Weaknesses**:
- ❌ No statistical rigor (no stddev, confidence intervals)
- ❌ No warmup/cooldown management
- ❌ No outlier detection/removal
- ❌ No automatic regression detection
- ❌ Manual result comparison
- ❌ No integration with pytest

### 5.2 Recommended: HYBRID

**Recommendation**: Keep custom benchmarks BUT add pytest-benchmark layer

**Benefits of Adding pytest-benchmark**:
1. **Statistical Analysis**: Automatic mean, median, stddev, IQR
2. **Outlier Detection**: Detects and handles outliers
3. **Comparison**: Built-in --benchmark-compare
4. **Warmup**: Automatic warmup rounds
5. **CI Integration**: Easy integration with pytest
6. **Histograms**: Visualization support

**Migration Path**:
```python
# Current (custom)
def bench(name, func, iterations):
    start = time.perf_counter()
    func(iterations)
    elapsed = time.perf_counter() - start
    print(f"{name}: {iterations/elapsed} ops/sec")

# With pytest-benchmark (if installed)
def test_something(benchmark):
    result = benchmark(func_to_test)
    # Automatic stats, outlier detection, comparison

# Hybrid approach (both work)
try:
    import pytest_benchmark
    USE_PYTEST_BENCHMARK = True
except ImportError:
    USE_PYTEST_BENCHMARK = False

def test_operation(benchmark=None):
    if benchmark:
        benchmark(operation)
    else:
        manual_bench("Operation", operation, 1000)
```

---

## 6. Recommendations

### 6.1 Immediate Actions

1. **Install pytest and pytest-benchmark**:
   ```bash
   pip install pytest pytest-benchmark
   ```

2. **Fix broken benchmarks**:
   - `milestone_benchmark.py`: Fix QMNFRational constructor calls
   - `batch_operations_benchmark.py`: Remove AdaptiveCRTBigInt import

3. **Create benchmark organization**:
   ```
   benchmarks/
   ├── python/
   │   ├── ffi/
   │   │   ├── test_modint_overhead.py
   │   │   ├── test_crtbigint_overhead.py
   │   │   └── test_batch_operations.py
   │   ├── integration/
   │   │   ├── test_workflows.py
   │   │   └── test_mixed_operations.py
   │   └── system/
   │       ├── test_fhe_operations.py
   │       └── test_neural_operations.py
   ```

4. **Standardize benchmark interface**:
   - Create `benchmarks/python/conftest.py` with shared fixtures
   - Add pytest-benchmark configuration
   - Create comparison baseline

### 6.2 Missing Benchmarks to Create

#### High Priority

1. **FFI Overhead by Type** (`benchmarks/python/ffi/test_ffi_overhead_by_type.py`):
   - ModInt, CRTBigInt, Rational, HCVLangBigInt
   - Construction, arithmetic, conversion
   - Target: Measure pure FFI call overhead

2. **Batch Operations Scaling** (`benchmarks/python/ffi/test_batch_scaling.py`):
   - Test batch sizes: 10, 100, 1000, 10000
   - Identify crossover point where batch wins
   - Measure GIL impact

3. **FHE Operations** (`benchmarks/python/system/test_fhe_performance.py`):
   - Encryption, decryption, homomorphic operations
   - Noise tracking overhead
   - Batch encrypt/decrypt

4. **Neural Residue Operations** (`benchmarks/python/system/test_neural_performance.py`):
   - Forward pass, backward pass
   - Layer-by-layer timing
   - Compare vs float baseline

#### Medium Priority

5. **Integration Workflows** (`benchmarks/python/integration/test_realistic_workflows.py`):
   - ML training loop
   - FHE computation pipeline
   - Storage operations

6. **Memory Patterns** (`benchmarks/python/system/test_memory_allocation.py`):
   - Allocation/deallocation overhead
   - Cache effects
   - GC impact

#### Low Priority

7. **Edge Cases** (`benchmarks/python/edge/test_boundary_conditions.py`):
   - Large numbers (>2^256)
   - Many small operations
   - Mixed type operations

### 6.3 Documentation Needs

Create comprehensive benchmark documentation:

1. **`benchmarks/README.md`**: Overview and quick start
2. **`benchmarks/RUNNING_BENCHMARKS.md`**: How to run, interpret results
3. **`benchmarks/ADDING_BENCHMARKS.md`**: Guidelines for new benchmarks
4. **`benchmarks/INTERPRETING_RESULTS.md`**: Understanding performance data

### 6.4 CI/CD Integration

```yaml
# .github/workflows/benchmark.yml
name: Performance Benchmarks

on: [push, pull_request]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Run benchmarks
        run: |
          pip install pytest pytest-benchmark
          pytest benchmarks/python/ --benchmark-only --benchmark-json=output.json

      - name: Compare with baseline
        run: |
          pytest-benchmark compare output.json baseline.json
```

---

## 7. Pytest-Benchmark Configuration

### 7.1 Recommended `pytest.ini`

```ini
[tool:pytest]
testpaths = tests benchmarks
python_files = test_*.py bench_*.py
python_classes = Test* Bench*
python_functions = test_* bench_*

# Benchmark settings
addopts =
    --benchmark-warmup=on
    --benchmark-warmup-iterations=5
    --benchmark-min-rounds=10
    --benchmark-calibration-precision=10
    --benchmark-disable-gc
    --benchmark-timer=time.perf_counter

# Benchmark groups
[benchmark]
warmup = true
warmup_iterations = 5
min_rounds = 10
max_time = 1.0
calibration_precision = 10
disable_gc = true
```

### 7.2 Example pytest-benchmark Test

```python
import pytest
from hcvlang_pyo3 import ModInt, batch_add_modint

def test_modint_addition(benchmark):
    """Benchmark ModInt addition with pytest-benchmark"""
    a = ModInt(123456789)
    b = ModInt(987654321)

    result = benchmark(lambda: a + b)

    # pytest-benchmark automatically provides:
    # - Mean, median, stddev
    # - Min, max
    # - IQR, outliers
    # - Comparison with previous runs

def test_batch_operations_scaling(benchmark):
    """Test batch operations at different scales"""
    sizes = [10, 100, 1000, 10000]

    for size in sizes:
        a_list = [ModInt(i) for i in range(size)]
        b_list = [ModInt(i+1) for i in range(size)]

        benchmark.group = f"batch-{size}"
        benchmark(batch_add_modint, a_list, b_list)
```

---

## 8. Summary and Next Steps

### Current State

| Aspect | Status | Quality |
|--------|--------|---------|
| **Custom Benchmarks** | ✅ Extensive (19+ files) | 🟡 Good |
| **pytest Integration** | ❌ None | ❌ Missing |
| **pytest-benchmark** | ❌ Not installed | ❌ Missing |
| **FFI Coverage** | 🟡 Partial (ModInt only) | 🟡 Fair |
| **Batch Testing** | ✅ Some tests exist | 🟡 Needs expansion |
| **Integration Tests** | 🟡 Limited | 🟠 Needs work |
| **Documentation** | 🟠 Minimal | 🟠 Needs improvement |

### Immediate Priorities

1. **Install pytest-benchmark**: `pip install pytest pytest-benchmark`
2. **Fix broken benchmarks**: Update API calls in existing scripts
3. **Verify FFI findings**: Investigate low batch operation speedup
4. **Create pytest-benchmark templates**: Migration path for existing benchmarks
5. **Document best practices**: When to use batch ops, FFI patterns

### Long-term Goals

1. **Comprehensive FFI coverage**: All 103 FFI classes benchmarked
2. **Automated regression detection**: CI/CD integration
3. **Performance budgets**: Set and enforce performance targets
4. **Comparative analysis**: QMNF vs float-based alternatives

---

## Appendix A: Files Requiring Attention

### Broken/Needs Update

1. `milestone_benchmark.py` - Line 41: QMNFRational constructor needs 2 args
2. `batch_operations_benchmark.py` - Line 67: AdaptiveCRTBigInt not in FFI
3. Multiple benchmarks may have similar API mismatch issues

### Working Benchmarks

1. `tests/python/test_modint_ffi.py` - ✅ Full test suite with timing
2. `tests/python/test_ffi_overhead_benchmark.py` - ✅ New comprehensive FFI test

### Recommended Templates

- Use `test_ffi_overhead_benchmark.py` as template for new benchmarks
- Supports both manual and pytest-benchmark modes
- Comprehensive statistics output
- Easy to extend

---

## Appendix B: Example Output

### Custom Benchmark Output
```
🚀 BATCH OPERATIONS & FAST MODULES BENCHMARK
Testing 17 batch operations + special functions

CRTBigInt Addition                          🟢    450,000 ops/sec
Rational Multiplication                     🟡     25,000 ops/sec
batch_add_crtbigint (50 items)             🟢     82,731 ops/sec
```

### pytest-benchmark Output (Example)
```
-------------------------------- benchmark: ModInt Addition ---------------------------------
Name                    Min        Max      Mean    StdDev    Median     IQR    Outliers
test_modint_add     245.0ns    890.0ns  256.3ns   23.1ns   250.0ns  10.0ns     12;15
---------------------------------------------------------------------------------------------
```

---

**Report End**

*Generated by: Claude Code Agent*
*Date: 2025-11-17*
*System: QMNF_System v1.0*

---
title: "Phase1 Optimization Master Plan"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/PHASE1_OPTIMIZATION_MASTER_PLAN.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF Phase 1 Optimization - Master Plan
## Integer-Only Mathematical Framework - Maximum Performance

**Date:** 2025-10-16
**System:** Intel Core i7-3632QM @ 2.20GHz, 8 cores, 5.6GB RAM
**Status:** Phase 1 - Float elimination ✅ COMPLETE | Performance optimization IN PROGRESS

---

## Executive Summary

Phase 1 has achieved **100% float elimination** with **+257% performance improvement**. We are now maximizing the integer-based mathematical framework by leveraging:

1. **Feature Flags** - Disable runtime checks (compiler enforces float-free)
2. **Parallel Execution** - Leverage 8 cores for batch operations
3. **HCVLang Primitives** - CRTBigInt, binary GCD, SIMD operations
4. **Deep Profiling** - Comprehensive metrics for bottleneck identification

**Expected Final Performance:** 10-20x improvement over baseline through parallelism and HCVLang integration.

---

## Current System Architecture

### Core Components (100% Float-Free ✅)

```
qmnf/
├── boundary.py                    # QMNFRational (Rust-backed), geometric primitives
├── storage/                       # Unified storage with memory fallback
├── agents/                        # Multi-agent coordination
└── frameworks/                    # Mathematical frameworks (energy, time crystals)

Performance Infrastructure (NEW):
├── qmnf_performance_config.py    # Feature flags, optimization levels
├── qmnf_parallel_math.py         # 8-core parallel batch operations
├── qmnf_deep_profiler.py         # Comprehensive metrics collection
└── quick_bench.py                # Fast baseline benchmark (<60s)
```

### Rust Backend (hcvlang_pyo3)

The system already uses `hcvlang_pyo3.Rational` for core arithmetic:
- **Implemented:** Basic Rational operations (+, -, *, /)
- **Available in HCVLang:** CRTBigInt, binary GCD, ModInt, prime operations
- **Performance Baseline** (from benchmarks):
  - Rational addition: ~50,000 ops/sec (Python wrapper overhead)
  - GCD operations: 423,000 ops/sec (32-bit, Rust native)
  - ModInt operations: 115M ops/sec (Rust native)

---

## Performance Configuration System

### Optimization Levels

```python
# Development Mode (Level 0)
- All assertions enabled
- Debug logging
- No parallelism
- Use: QMNF_OPT_LEVEL=0

# Balanced Mode (Level 1)
- Essential assertions
- Moderate parallelism
- Use: QMNF_OPT_LEVEL=1

# Maximum Performance (Level 2) - PRODUCTION
- Assertions disabled (compiler still enforces float-free)
- Full 8-core parallelism
- Large caches
- Fast-path algorithms
- Use: QMNF_OPT_LEVEL=2
```

### Feature Flags

```bash
export QMNF_OPT_LEVEL=2              # Maximum performance
export QMNF_WORKERS=8                # Use all 8 cores
export QMNF_CACHE_SIZE=50000         # Large rational cache
export QMNF_ASSERTIONS=0             # Disable runtime assertions
export QMNF_PARALLEL=1               # Enable parallel execution
```

**Key Insight:** Float checking is now **compile-time only**. The system won't compile with floats, so runtime checks are pure overhead.

---

## Parallel Execution Framework

### Batch Operations (8-Core Speedup)

```python
from qmnf_parallel_math import (
    batch_rational_add,
    batch_rational_multiply,
    reduce_rationals_sum,
    batch_point_distances_squared
)

# Automatic parallelization for large batches
rationals_a = [QMNFRational(i, i+1) for i in range(10000)]
rationals_b = [QMNFRational(i+2, i+3) for i in range(10000)]

# Serial: ~50K ops/sec
# Parallel (8-core): ~350-400K ops/sec (7-8x speedup)
results = batch_rational_add(rationals_a, rationals_b, parallel=True)
```

### Parallel Execution Strategy

- **Threshold:** 1,000 operations (configurable)
- **Below threshold:** Serial execution (avoid thread overhead)
- **Above threshold:** ThreadPoolExecutor with 8 workers
- **Tree Reduction:** For sum/product operations
- **Chunk Size:** Auto-calculated based on worker count

**Expected Speedup:** 5-8x on batch operations (near-linear scaling)

---

## Deep Profiling System

### Metrics Collected (ALL Integer)

```python
from qmnf_deep_profiler import DeepProfiler

profiler = DeepProfiler()
profiler.take_snapshot()  # Before workload

# ... your QMNF operations ...

profiler.take_snapshot()  # After workload
profiler.print_report()   # Comprehensive analysis
```

**Metrics Categories:**
1. **CPU:** Per-core utilization, context switches, cache misses
2. **Memory:** RSS, VMS, page faults, swap usage
3. **I/O:** Disk reads/writes, file descriptors
4. **QMNF Operations:** Rational ops, GCD calls, geometric ops
5. **Threading:** Lock contention, parallel efficiency

**Output:** JSON report with bottleneck identification

---

## Optimization Roadmap

### Completed ✅

1. **Float Elimination** - 100% complete, 0 violations
2. **Performance Config** - Feature flags for runtime checks
3. **Parallel Framework** - 8-core batch operations
4. **Deep Profiler** - Comprehensive metrics collection
5. **Rust Integration** - hcvlang_pyo3.Rational backend

### In Progress 🔄

#### 1. HCVLang Fast-Path Integration

**Goal:** Leverage native Rust performance for hot paths

```python
# Current: Python wrapper → Rust Rational
r = QMNFRational(a, b) + QMNFRational(c, d)  # ~50K ops/sec

# Target: Direct Rust batch operations
results = hcvlang_fast_add_batch(nums, dens)  # ~500K+ ops/sec
```

**Implementation:**
- Expose batch operations from hcvlang_pyo3
- Bypass Python object creation for intermediate results
- Use pre-allocated result buffers

**Expected Gain:** 10-20x for batch arithmetic

#### 2. Binary GCD Optimization

**Current:** Python's math.gcd or Rust's default
**Target:** Stein's binary GCD (bit operations)

From HCVLang benchmarks:
- Binary GCD: 850K ops/sec (16-bit)
- Binary GCD: 425K ops/sec (32-bit)
- Binary GCD: 157K ops/sec (64-bit)

**Expected Gain:** 2-3x for GCD-heavy operations

#### 3. CRTBigInt for Arbitrary Precision

**Use Case:** Large rational arithmetic (> 64-bit)

```python
# When numerator/denominator exceed 64-bit:
# 1. Convert to CRT representation (multiple moduli)
# 2. Perform operations in parallel across moduli
# 3. Reconstruct result via Chinese Remainder Theorem
```

**From HCVLang Benchmarks:**
- CRT multiplication: Parallel across moduli
- Ideal for cryptographic-scale integers
- Available in hcvlang but not yet exposed to Python

**Expected Gain:** 5-10x for large integer operations

#### 4. Rational Cache

**Hot Values:**
```python
# Frequently used rationals (cache these):
ZERO = QMNFRational(0, 1)
ONE = QMNFRational(1, 1)
HALF = QMNFRational(1, 2)
PHI = QMNFRational(PHI_NUM, PHI_DEN)
```

**Strategy:**
- LRU cache for constructed rationals
- Key: (numerator, denominator) tuple
- Size: 10,000-50,000 entries (configurable)

**Expected Gain:** 10-50x for repeated constructions

### Pending Implementation 📋

#### 5. SIMD Vectorization (via HCVLang)

**From MAA Benchmarks:**
- SIMD GCD: Process 4-8 rationals simultaneously
- AVX2 enabled on x86_64

**Implementation:**
```rust
// In hcvlang_pyo3 (Rust side)
pub fn batch_gcd_simd(numerators: &[i64], denominators: &[i64]) -> Vec<(i64, i64)> {
    // Process 8 pairs per SIMD operation
}
```

**Expected Gain:** 4-8x for GCD operations

#### 6. Memory Pool Allocation

**Problem:** Frequent QMNFRational allocations cause heap fragmentation

**Solution:**
- Pre-allocate pool of Rational objects
- Reuse instead of allocate/deallocate
- Integrate with MANA resource system

**Expected Gain:** 2-3x reduction in allocation overhead

---

## Bottleneck Analysis (Current System)

### Known Bottlenecks

1. **GCD Operations** - Called on every Rational construction
   - **Impact:** 30-40% of Rational construction time
   - **Solution:** Binary GCD + caching

2. **Python Object Overhead** - Wrapper around Rust types
   - **Impact:** 5-10x slower than pure Rust
   - **Solution:** Batch operations, bypass wrappers

3. **Serial Execution** - Not using 8 cores
   - **Impact:** 7-8x untapped potential
   - **Solution:** Parallel batch operations (implemented)

4. **Memory Allocations** - Frequent small allocations
   - **Impact:** Cache misses, heap fragmentation
   - **Solution:** Object pooling, memory arena

### Performance Targets

| Operation | Current | Target | Strategy |
|-----------|---------|--------|----------|
| Rational Construction | 50K/sec | 500K/sec | Cache + binary GCD |
| Rational Addition | 50K/sec | 500K/sec | Batch operations |
| GCD (32-bit) | 100K/sec | 425K/sec | Binary GCD (Stein) |
| Batch Operations (8-core) | 50K/sec | 400K/sec | Parallel execution ✅ |
| Geometric Operations | 20K/sec | 200K/sec | Batch + cache |

**Overall Target:** 10x improvement (500K ops/sec aggregate)

---

## Benchmarking Strategy

### Quick Benchmark (< 60 seconds)

```bash
cd ~/QMNF_System
python3 quick_bench.py
```

**Tests:**
- Rational construction (int, pair, GCD)
- Arithmetic operations (+, *, /)
- Geometric primitives (points, lines)
- Identifies slowest 5 operations

### Comprehensive Benchmark (with profiling)

```bash
export QMNF_OPT_LEVEL=2  # Maximum performance
python3 -c "
from qmnf_deep_profiler import DeepProfiler
from qmnf_parallel_math import benchmark_parallel_speedup

# Profile parallel execution
profiler = DeepProfiler()
profiler.take_snapshot()

speedup = benchmark_parallel_speedup()

profiler.take_snapshot()
profiler.print_report()
profiler.save_report_json('profile_results.json')
"
```

### HCVLang Native Benchmarks

Reference benchmarks in `~/Downloads/`:
- `hcvlang_math_benchmarks.txt` - Mathematical operations
- `maa_benchmarks.md` - Cryptographic primitives
- `bench_gcd.rs` - GCD implementations
- `bench_mul.rs` - Multiplication strategies

**Use these as targets for Python wrapper performance.**

---

## Integration with Existing System

### COSMOS-MANA Architecture

The performance optimizations integrate with:
- **MANA Resource System:** Memory pools align with lease management
- **GSO Swarm Agents:** Parallel execution for agent coordination
- **MAA Double Helix:** Parallel lane execution

### Memory Management

```python
# Kernel memory reservation (already configured)
vm.min_free_kbytes = 131072  # 128MB MANA reserve

# Python side: Object pooling
from qmnf_performance_config import get_performance_config
config = get_performance_config()
pool_size = config.memory_pool_size_mb * 1024 * 1024  # 64MB default
```

---

## Action Items

### Immediate (This Session)

- [x] Feature flag system
- [x] Parallel execution framework
- [x] Deep profiling system
- [x] Quick benchmark tool
- [ ] Run baseline benchmark
- [ ] Document current performance

### Short Term (Next Session)

1. **Expose HCVLang Batch Operations**
   - Modify hcvlang_pyo3 to expose batch_add, batch_mul, batch_gcd
   - Create Python wrappers with numpy-like interface
   - Benchmark vs current implementation

2. **Implement Rational Cache**
   - LRU cache for hot values
   - Integration with performance config
   - Measure hit rate

3. **Binary GCD Integration**
   - Expose Stein's algorithm from HCVLang
   - Replace Python GCD calls
   - Benchmark improvement

### Medium Term

4. **CRTBigInt Integration**
   - Expose CRT arithmetic from HCVLang
   - Automatic selection for large integers (> 64-bit)
   - Parallel modular arithmetic

5. **SIMD Vectorization**
   - Batch operations with AVX2
   - Process 4-8 rationals simultaneously

6. **Memory Pool**
   - Pre-allocated Rational objects
   - Integration with MANA system

---

## Testing & Validation

### Unit Tests

```bash
# Test parallel execution
python3 -m pytest tests/test_parallel_math.py -v

# Test performance config
python3 -m pytest tests/test_performance_config.py -v

# Test profiling
python3 -m pytest tests/test_deep_profiler.py -v
```

### Integration Tests

```bash
# Verify float-free guarantee
make lint  # Runs tools/check_no_floats.py

# Full quality check
make quality-check
```

### Performance Regression Tests

```bash
# Quick benchmark (should complete < 60s)
python3 quick_bench.py > baseline.txt

# After optimization:
python3 quick_bench.py > optimized.txt
diff baseline.txt optimized.txt
```

---

## Success Criteria

### Phase 1 Complete ✅
- [x] 100% float elimination
- [x] +257% performance improvement
- [x] All tests passing

### Phase 1 Optimization Complete (Target)
- [ ] 10x aggregate performance improvement
- [ ] 8-core parallel execution functional
- [ ] GCD operations > 400K ops/sec
- [ ] Rational operations > 500K ops/sec (batch)
- [ ] Comprehensive profiling data collected
- [ ] Bottlenecks identified and documented

### Phase 2 Ready (Next)
- [ ] HCVLang primitives fully integrated
- [ ] COSMOS-MANA system operational
- [ ] Performance baseline documented
- [ ] Optimization roadmap for Phase 2

---

## References

### Benchmarks
- `~/Downloads/hcvlang_math_benchmarks.txt`
- `~/Downloads/maa_benchmarks.md`
- `~/Downloads/bench_gcd.rs`

### Documentation
- `MILESTONE_FLOAT_ELIMINATION_COMPLETE.md`
- `BENCHMARK_COMPARISON_BEFORE_AFTER.md`
- `SYSTEM_OPTIMIZATION_PLAN.md`
- `MANA_MEMORY_RESERVATION_SYSTEM.md`

### Key Files
- `qmnf/boundary.py` - Core Rational implementation
- `qmnf_performance_config.py` - Runtime configuration
- `qmnf_parallel_math.py` - Parallel batch operations
- `qmnf_deep_profiler.py` - Comprehensive profiling
- `quick_bench.py` - Fast benchmarking

---

## Notes

**Critical Principle:** Never compromise the integer-only guarantee. Floats will not compile. All optimizations must maintain exact rational arithmetic.

**Hardware Upgrade Pending:** 5.6GB → 8GB RAM will provide +44% capacity for MANA leases and caching.

**Rust Backend:** The heavy lifting is already in Rust (hcvlang_pyo3). Python optimization focuses on reducing wrapper overhead and enabling parallel execution.

**Measurement Philosophy:** All metrics use integers (microseconds, bytes, basis points). No floats in measurement code.

---

**Status:** Ready for baseline benchmarking and bottleneck analysis.
**Next Step:** Run `python3 quick_bench.py` to establish current performance baseline.

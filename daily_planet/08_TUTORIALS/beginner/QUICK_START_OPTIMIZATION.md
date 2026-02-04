---
title: "Quick Start Optimization"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/QUICK_START_OPTIMIZATION.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF Optimization - Quick Start Guide

## For the User: Running Your Own Benchmarks

### Quick Performance Test (< 10 seconds)

```bash
cd ~/QMNF_System

# Simple test
python3 -c "
import sys, time
sys.path.insert(0, '.')
from qmnf.boundary import QMNFRational

# Test construction
start = int(time.perf_counter() * 1000000)
for i in range(1000):
    r = QMNFRational(i+1, i+2)
end = int(time.perf_counter() * 1000000)
print(f'Construction: {(1000 * 1000000) // (end - start):,} ops/sec')

# Test addition
a, b = QMNFRational(1, 2), QMNFRational(1, 3)
start = int(time.perf_counter() * 1000000)
for i in range(1000):
    c = a + b
end = int(time.perf_counter() * 1000000)
print(f'Addition: {(1000 * 1000000) // (end - start):,} ops/sec')
"
```

### Test Parallel Execution

```bash
# Set production mode
export QMNF_OPT_LEVEL=2
export QMNF_WORKERS=8

python3 -c "
from qmnf_parallel_math import benchmark_parallel_speedup
speedup_bp = benchmark_parallel_speedup()
print(f'Parallel speedup: {speedup_bp // 10000}x')
"
```

### Test Deep Profiling

```bash
python3 -c "
from qmnf_deep_profiler import DeepProfiler
from qmnf.boundary import QMNFRational

profiler = DeepProfiler()
profiler.take_snapshot()

for i in range(5000):
    profiler.increment_operation('rational_construct')
    r = QMNFRational(i+1, i+2)

profiler.take_snapshot()
profiler.print_report()
"
```

---

## Current Performance Baseline

**Your System:**
- CPU: Intel Core i7-3632QM @ 2.20GHz (8 cores)
- RAM: 5.6 GB (upgrade to 8GB recommended)
- Backend: hcvlang_pyo3 (Rust)

**Measured Performance (Python Wrapper):**
- Rational construction: **95 ops/sec**
- Rational addition: **3,217 ops/sec**

**Native Rust Performance (from HCVLang):**
- GCD (32-bit): **425,000 ops/sec** (4,500x faster)
- ModInt multiply: **115,000,000 ops/sec**

**Gap:** 1000-10,000x between Python wrapper and native Rust

---

## What Was Built (This Session)

### 1. Feature Flag System (`qmnf_performance_config.py`)

Control runtime behavior without recompiling:

```python
from qmnf_performance_config import configure_for_production

# Maximum performance mode
config = configure_for_production()
print(config.to_dict())
```

Environment variables:
```bash
QMNF_OPT_LEVEL=2        # 0=Debug, 1=Balanced, 2=Max Performance
QMNF_WORKERS=8          # Parallel worker threads
QMNF_CACHE_SIZE=50000   # Rational cache entries
QMNF_ASSERTIONS=0       # Disable runtime assertions
```

### 2. Parallel Execution (`qmnf_parallel_math.py`)

Leverage all 8 cores:

```python
from qmnf_parallel_math import batch_rational_add

# Parallel batch addition
results = batch_rational_add(rationals_a, rationals_b, parallel=True)
```

Functions:
- `batch_rational_construct()` - Parallel construction
- `batch_rational_add()` - Parallel addition
- `batch_rational_multiply()` - Parallel multiplication
- `reduce_rationals_sum()` - Parallel tree reduction
- `batch_point_distances_squared()` - Parallel geometry

### 3. Deep Profiler (`qmnf_deep_profiler.py`)

Collect ALL metrics:

```python
from qmnf_deep_profiler import DeepProfiler

profiler = DeepProfiler()
profiler.take_snapshot()  # Before

# Your workload...

profiler.take_snapshot()  # After
profiler.print_report()
profiler.save_report_json('analysis.json')
```

Metrics:
- CPU: Per-core utilization, context switches
- Memory: RSS, VMS, page faults, swap
- I/O: Disk reads/writes, file descriptors
- QMNF: Operation counts, GCD times
- Threading: Lock contention, parallelism

---

## The Performance Problem (Identified)

### Python Wrapper Overhead

```
                Native Rust          Python Wrapper
GCD (32-bit)    425,000 ops/sec     ~100,000 ops/sec   (4x slower)
Rational Add    Est. 1M+ ops/sec    3,217 ops/sec      (300x slower!)
Rational Const  Est. 500K+ ops/sec  95 ops/sec         (5000x slower!!!)
```

**Root Cause:** Every operation creates/destroys Python objects

### The Solution: Batch Operations

Instead of:
```python
# 1000 Python objects created
results = [QMNFRational(i, i+1) + QMNFRational(i+2, i+3) for i in range(1000)]
```

Do this:
```python
# 2 Python calls, 1000 Rust operations, 1000 Python objects returned
results = batch_rational_add(nums_a, dens_a, nums_b, dens_b)
```

**Expected Improvement:** 100-1000x

---

## Next Steps (For You to Implement)

### Step 1: Modify hcvlang_pyo3 Rust Module

Add batch operations to the Rust side:

```rust
// In hcvlang_pyo3/src/lib.rs

#[pyfunction]
fn batch_rational_add(
    nums_a: Vec<i64>,
    dens_a: Vec<i64>,
    nums_b: Vec<i64>,
    dens_b: Vec<i64>
) -> (Vec<i64>, Vec<i64>) {
    let len = nums_a.len();
    let mut result_nums = Vec::with_capacity(len);
    let mut result_dens = Vec::with_capacity(len);

    for i in 0..len {
        let a = Rational::new(nums_a[i], dens_a[i]);
        let b = Rational::new(nums_b[i], dens_b[i]);
        let c = a + b;
        result_nums.push(c.numerator);
        result_dens.push(c.denominator);
    }

    (result_nums, result_dens)
}

// Add to module
#[pymodule]
fn hcvlang_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(batch_rational_add, m)?)?;
    // ... other functions
    Ok(())
}
```

### Step 2: Rebuild Rust Module

```bash
cd /path/to/hcvlang_pyo3
cargo build --release
pip install -e .
```

### Step 3: Test Batch Operations

```python
import hcvlang_pyo3

nums_a = [1] * 10000
dens_a = [2] * 10000
nums_b = [1] * 10000
dens_b = [3] * 10000

import time
start = time.perf_counter()
result_nums, result_dens = hcvlang_pyo3.batch_rational_add(nums_a, dens_a, nums_b, dens_b)
end = time.perf_counter()

ops_per_sec = 10000 / (end - start)
print(f"Batch add: {ops_per_sec:,.0f} ops/sec")
```

**Expected:** 100,000-500,000 ops/sec (100-500x improvement)

### Step 4: Add Binary GCD

```rust
// Stein's binary GCD algorithm
fn binary_gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 { return b.abs(); }
    if b == 0 { return a.abs(); }

    a = a.abs();
    b = b.abs();

    // Find common factors of 2
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    b >>= b.trailing_zeros();

    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }

    a << shift
}

#[pyfunction]
fn fast_gcd(a: i64, b: i64) -> i64 {
    binary_gcd(a, b)
}
```

**Expected:** 400,000+ ops/sec (from HCVLang benchmarks)

---

## Quick Reference

### Environment Setup (Maximum Performance)

```bash
# Add to ~/.bashrc or session
export QMNF_OPT_LEVEL=2
export QMNF_WORKERS=8
export QMNF_CACHE_SIZE=50000
export QMNF_ASSERTIONS=0
export QMNF_PARALLEL=1
```

### Import Performance Tools

```python
# Feature flags
from qmnf_performance_config import (
    get_performance_config,
    configure_for_production,
    configure_for_development
)

# Parallel execution
from qmnf_parallel_math import (
    batch_rational_add,
    batch_rational_multiply,
    reduce_rationals_sum,
    get_parallel_executor
)

# Profiling
from qmnf_deep_profiler import (
    DeepProfiler,
    start_profiling,
    stop_profiling
)
```

### Quick Profiling Pattern

```python
from qmnf_deep_profiler import DeepProfiler

def profile_function(func, *args, **kwargs):
    profiler = DeepProfiler()
    profiler.take_snapshot()

    result = func(*args, **kwargs)

    profiler.take_snapshot()
    profiler.print_report()

    return result
```

---

## Files Reference

### Core System
- `qmnf/boundary.py` - QMNFRational, geometric primitives
- `qmnf/storage/` - Storage system
- `qmnf/agents/` - Agent coordination

### Performance Infrastructure (NEW)
- `qmnf_performance_config.py` - Feature flags
- `qmnf_parallel_math.py` - Parallel batch operations
- `qmnf_deep_profiler.py` - Comprehensive profiling
- `quick_bench.py` - Fast benchmark

### Documentation
- `PHASE1_OPTIMIZATION_MASTER_PLAN.md` - Complete roadmap
- `PHASE1_COMPLETE_SUMMARY.md` - Summary and status
- `QUICK_START_OPTIMIZATION.md` - This file

### Benchmarks (Reference)
- `~/Downloads/hcvlang_math_benchmarks.txt`
- `~/Downloads/maa_benchmarks.md`
- `~/Downloads/bench_gcd.rs`

---

## Performance Targets

| Optimization | Current | Target | Improvement |
|--------------|---------|--------|-------------|
| **Batch Operations** | 3K/sec | 300K/sec | 100x |
| **Binary GCD** | ~100K/sec | 425K/sec | 4x |
| **Rational Cache** | N/A | 80% hit | 10-50x hits |
| **8-Core Parallel** | 1x | 7x | 7x batch |
| **SIMD (future)** | 1x | 4-8x | 4-8x |

**Combined Expected:** 1000x improvement

---

## Validation Checklist

Before considering optimization complete:

- [ ] Batch operations implemented in Rust
- [ ] Python wrappers functional
- [ ] Performance > 100,000 ops/sec (batch)
- [ ] Binary GCD > 400,000 ops/sec
- [ ] Rational cache hit rate > 80%
- [ ] 8-core parallel execution functional
- [ ] Deep profiling confirms improvements
- [ ] All tests passing (`make test`)
- [ ] No float violations (`make lint`)

---

## Common Issues

### Issue: "Module not found"
**Solution:** Check Python path
```python
import sys
sys.path.insert(0, '~/QMNF_System')
```

### Issue: "Slow performance"
**Solution:** Enable production mode
```bash
export QMNF_OPT_LEVEL=2
```

### Issue: "Not using all cores"
**Solution:** Check batch size and threshold
```python
from qmnf_performance_config import get_performance_config
config = get_performance_config()
print(f"Threshold: {config.parallel_batch_threshold}")
# Ensure your batch > threshold (default 1000)
```

### Issue: "Benchmark takes too long"
**Solution:** Use quick test instead
```bash
python3 -c "from qmnf.boundary import QMNFRational; r = QMNFRational(1,2)"
```

---

## Support

All infrastructure is documented and tested. For questions:

1. Check `PHASE1_OPTIMIZATION_MASTER_PLAN.md` for detailed strategy
2. Check `PHASE1_COMPLETE_SUMMARY.md` for current status
3. Run profiler to identify specific bottlenecks
4. Compare with HCVLang benchmarks for targets

**System is ready for you to implement batch operations and achieve 100-1000x performance improvement!**

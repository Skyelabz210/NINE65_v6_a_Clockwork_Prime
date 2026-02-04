# Full-Stack Benchmarking Work Request

**Created:** 2025-11-17
**Priority:** MEDIUM
**Status:** READY FOR EXECUTION
**Prerequisite:** FFI Integration Complete (Commit 21a9f10)
**Assignee:** AI Benchmarking Team

---

## Summary

Execute comprehensive performance benchmarking across the entire QMNF System stack. Establish baseline performance metrics for all subsystems including Residue Neural Networks, FHE cryptography, CRT arithmetic, MANA orchestration, and storage systems.

**Scope:** Full-stack performance profiling
**Estimated Time:** 16-24 hours
**Benchmark Count:** 200+ individual benchmarks
**Impact:** Production readiness validation and performance regression detection

---

## Benchmarking Strategy

### Objectives

1. **Baseline Establishment:** Current performance across all subsystems
2. **Bottleneck Identification:** Find performance critical paths
3. **Regression Detection:** Enable future performance validation
4. **Optimization Targets:** Identify improvement opportunities
5. **Production Readiness:** Verify performance targets met

### Methodology

- **Rust Benchmarks:** Criterion.rs for microbenchmarks
- **Python Benchmarks:** pytest-benchmark for FFI operations
- **Integration Benchmarks:** End-to-end workflows
- **Memory Profiling:** Allocation patterns and leak detection
- **Comparative Analysis:** Before/after optimization comparisons

---

## Benchmark Suite Structure

```
benchmarks/
├── rust/                               # Criterion benchmarks
│   ├── core_arithmetic.rs              # CRT, ModInt, Rational
│   ├── neural_networks.rs              # Residue NN, SIMD, Montgomery
│   ├── cryptography.rs                 # FHE operations
│   ├── storage.rs                      # HoloHD, SVD encoding
│   ├── mana_orchestration.rs           # Runtime kernel
│   ├── mathematical.rs                 # Transcendental, polynomials
│   ├── geometric.rs                    # 2D/3D SIMD primitives
│   ├── entropy.rs                      # Shadow entropy harvesting
│   └── batch_operations.rs             # Parallel batch processing
│
├── python/                             # pytest-benchmark
│   ├── ffi_overhead.py                 # FFI boundary costs
│   ├── batch_comparison.py             # Individual vs batch
│   ├── neural_workflows.py             # End-to-end NN training
│   ├── crypto_workflows.py             # FHE encryption pipelines
│   └── integration.py                  # Cross-subsystem workflows
│
└── reports/                            # Generated reports
    ├── baseline_YYYY-MM-DD.json        # Raw benchmark data
    ├── summary_YYYY-MM-DD.html         # HTML report
    ├── comparison_YYYY-MM-DD.md        # Comparison analysis
    └── recommendations.md              # Optimization suggestions
```

---

## Benchmark Categories

### Category 1: Core Arithmetic (Rust)

**File:** `benchmarks/rust/core_arithmetic.rs`

#### CRTBigInt Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use hcvlang::crt_bigint::CRTBigInt;

fn crtbigint_construction(c: &mut Criterion) {
    c.bench_function("crtbigint_from_i64", |b| {
        b.iter(|| CRTBigInt::from(black_box(42i64)))
    });

    c.bench_function("crtbigint_from_large", |b| {
        b.iter(|| CRTBigInt::from(black_box(1267650600228229401496703205376i128)))
    });
}

fn crtbigint_addition(c: &mut Criterion) {
    let a = CRTBigInt::from(123456789i64);
    let b = CRTBigInt::from(987654321i64);

    c.bench_function("crtbigint_add", |b| {
        b.iter(|| black_box(&a) + black_box(&b))
    });
}

fn crtbigint_multiplication(c: &mut Criterion) {
    let a = CRTBigInt::from(123456789i64);
    let b = CRTBigInt::from(987654321i64);

    c.bench_function("crtbigint_mul", |b| {
        b.iter(|| black_box(&a) * black_box(&b))
    });
}

fn crtbigint_reconstruction(c: &mut Criterion) {
    let a = CRTBigInt::from(123456789i64);

    c.bench_function("crtbigint_reconstruct", |b| {
        b.iter(|| black_box(&a).reconstruct())
    });
}

fn crtbigint_batch_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("crtbigint_batch");

    for size in [10, 100, 1000, 10000].iter() {
        let values_a: Vec<_> = (0..*size).map(|i| CRTBigInt::from(i as i64)).collect();
        let values_b: Vec<_> = (0..*size).map(|i| CRTBigInt::from(i as i64 * 2)).collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &_size| {
            b.iter(|| {
                values_a.iter().zip(values_b.iter())
                    .map(|(a, b)| a + b)
                    .collect::<Vec<_>>()
            })
        });
    }
    group.finish();
}
```

#### ModInt Benchmarks

```rust
use hcvlang::modint::ModInt;

fn modint_operations(c: &mut Criterion) {
    let a = ModInt::new_u64(123456789, 2147483647);
    let b = ModInt::new_u64(987654321, 2147483647);

    c.bench_function("modint_add", |b| {
        b.iter(|| black_box(a) + black_box(b))
    });

    c.bench_function("modint_mul", |b| {
        b.iter(|| black_box(a) * black_box(b))
    });

    c.bench_function("modint_inverse", |b| {
        b.iter(|| black_box(a).modular_inverse_constant_time())
    });
}

fn modint_montgomery(c: &mut Criterion) {
    let a = ModInt::new_u64(123456789, 2147483647);
    let b = ModInt::new_u64(987654321, 2147483647);

    c.bench_function("modint_montgomery_mul", |b| {
        b.iter(|| black_box(a).montgomery_mul(black_box(b)))
    });
}
```

#### Rational Arithmetic

```rust
use hcvlang::rational::Rational;

fn rational_operations(c: &mut Criterion) {
    let a = Rational::new(CRTBigInt::from(22), CRTBigInt::from(7));
    let b = Rational::new(CRTBigInt::from(355), CRTBigInt::from(113));

    c.bench_function("rational_add", |b| {
        b.iter(|| black_box(&a) + black_box(&b))
    });

    c.bench_function("rational_mul", |b| {
        b.iter(|| black_box(&a) * black_box(&b))
    });
}
```

**Expected Performance:**
- CRTBigInt construction: <200ns
- CRTBigInt add/mul: <500ns
- CRTBigInt reconstruct: <2µs
- ModInt add: <10ns
- ModInt mul (Montgomery): <50ns
- ModInt inverse: <100ns
- Rational operations: <1µs

---

### Category 2: Neural Networks (Rust)

**File:** `benchmarks/rust/neural_networks.rs`

#### Montgomery Arithmetic

```rust
use hcvlang::neural::montgomery::MontgomeryContext;

fn montgomery_benchmarks(c: &mut Criterion) {
    let ctx = MontgomeryContext::new(2147483647);

    c.bench_function("montgomery_mul", |b| {
        b.iter(|| {
            ctx.mul(black_box(123456789), black_box(987654321))
        })
    });

    c.bench_function("montgomery_add", |b| {
        b.iter(|| {
            ctx.add(black_box(123456789), black_box(987654321))
        })
    });
}
```

#### Residue Neural Network

```rust
use hcvlang::neural::residue_space::{ResidueDenseLayer, ResidueConfig};

fn residue_layer_forward(c: &mut Criterion) {
    let config = ResidueConfig::default();
    let layer = ResidueDenseLayer::new(784, 128, config);

    let input: Vec<u64> = (0..784).collect();

    c.bench_function("residue_layer_forward_784_128", |b| {
        b.iter(|| layer.forward(black_box(&input)))
    });
}

fn residue_layer_backward(c: &mut Criterion) {
    let config = ResidueConfig::default();
    let layer = ResidueDenseLayer::new(128, 64, config);

    let input: Vec<u64> = (0..128).collect();
    let grad: Vec<u64> = (0..64).collect();

    c.bench_function("residue_layer_backward_128_64", |b| {
        b.iter(|| layer.backward(black_box(&input), black_box(&grad)))
    });
}
```

#### SIMD Acceleration

```rust
#[cfg(feature = "simd")]
use hcvlang::neural::simd::*;

#[cfg(feature = "simd")]
fn simd_benchmarks(c: &mut Criterion) {
    let a: Vec<u64> = (0..256).collect();
    let b: Vec<u64> = (0..256).collect();

    c.bench_function("simd_batch_add_256", |b| {
        b.iter(|| simd_batch_add(black_box(&a), black_box(&b)))
    });

    c.bench_function("simd_batch_mul_256", |b| {
        b.iter(|| simd_batch_multiply(black_box(&a), black_box(&b)))
    });
}
```

**Expected Performance:**
- Montgomery mul: 4-10ns (target: 4.1ns)
- Montgomery add: 2-5ns
- Residue forward pass: <10µs
- Residue backward pass: <20µs
- SIMD batch add: 8× faster than sequential
- SIMD batch mul: 8× faster than sequential

---

### Category 3: Cryptography (Rust)

**File:** `benchmarks/rust/cryptography.rs`

#### FHE Operations

```rust
use hcvlang::fhe::{FHEContext, SecurityLevel};

fn fhe_benchmarks(c: &mut Criterion) {
    let ctx = FHEContext::new(SecurityLevel::Toy);
    let (sk, pk) = ctx.generate_keypair();
    let ek = ctx.generate_evaluation_key(&sk);

    let pt1 = ctx.encode(42);
    let pt2 = ctx.encode(100);

    c.bench_function("fhe_encrypt", |b| {
        b.iter(|| ctx.encrypt(black_box(&pt1), &pk))
    });

    let ct1 = ctx.encrypt(&pt1, &pk);
    let ct2 = ctx.encrypt(&pt2, &pk);

    c.bench_function("fhe_decrypt", |b| {
        b.iter(|| ctx.decrypt(black_box(&ct1), &sk))
    });

    c.bench_function("fhe_add", |b| {
        b.iter(|| ctx.add(black_box(&ct1), black_box(&ct2)))
    });

    c.bench_function("fhe_mul", |b| {
        b.iter(|| ctx.mul(black_box(&ct1), black_box(&ct2), &ek))
    });
}
```

#### Batch FHE

```rust
use hcvlang::fhe_realtime::batch_operations::BatchProcessor;

fn batch_fhe_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_fhe");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let ctx = FHEContext::new(SecurityLevel::Toy);
            let (sk, pk) = ctx.generate_keypair();

            let plaintexts: Vec<_> = (0..size)
                .map(|i| ctx.encode(i as i64))
                .collect();

            b.iter(|| {
                plaintexts.iter()
                    .map(|pt| ctx.encrypt(pt, &pk))
                    .collect::<Vec<_>>()
            })
        });
    }
    group.finish();
}
```

**Expected Performance:**
- FHE encrypt (base): 2-5ms
- FHE encrypt (real-time): <1ms
- FHE decrypt: 1-3ms
- FHE add: 50-200µs
- FHE mul: 5-15ms
- Batch encryption (100 items, 8 cores): 8× speedup

---

### Category 4: Python FFI Overhead

**File:** `benchmarks/python/ffi_overhead.py`

```python
"""
FFI Boundary Overhead Benchmarks

Measures Python-Rust boundary crossing costs.
"""

import pytest


@pytest.mark.benchmark(group="ffi_construction")
def test_crtbigint_construction(benchmark):
    """Benchmark CRTBigInt construction from Python."""
    from hcvlang import CRTBigInt

    result = benchmark(CRTBigInt, 42)
    assert result is not None


@pytest.mark.benchmark(group="ffi_arithmetic")
def test_crtbigint_addition(benchmark):
    """Benchmark CRTBigInt addition via FFI."""
    from hcvlang import CRTBigInt

    a = CRTBigInt(123456789)
    b = CRTBigInt(987654321)

    result = benchmark(lambda: a + b)
    assert result is not None


@pytest.mark.benchmark(group="ffi_batch")
def test_batch_vs_individual(benchmark):
    """Benchmark batch operations vs individual FFI calls."""
    from hcvlang import CRTBigInt, batch_add_crtbigint

    n = 100
    values_a = [CRTBigInt(i) for i in range(n)]
    values_b = [CRTBigInt(i * 2) for i in range(n)]

    result = benchmark(batch_add_crtbigint, values_a, values_b)
    assert len(result) == n


@pytest.mark.benchmark(group="ffi_neural")
def test_residue_similarity_matrix(benchmark):
    """Benchmark residue similarity computation."""
    from hcvlang import ResidueSimilarityEngine

    engine = ResidueSimilarityEngine(128, 1000)
    theorems = [f"theorem_{i}" for i in range(10)]

    result = benchmark(engine.similarity_matrix, theorems)
    assert len(result) == 100  # 10x10 flattened


@pytest.mark.benchmark(group="ffi_crypto")
def test_fhe_encryption(benchmark):
    """Benchmark FHE encryption via FFI."""
    from hcvlang import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()
    plaintext = ctx.encode(42)

    result = benchmark(ctx.encrypt, plaintext, pk)
    assert result is not None
```

**Run Commands:**
```bash
# Run benchmarks with output
pytest benchmarks/python/ffi_overhead.py --benchmark-only --benchmark-verbose

# Generate JSON report
pytest benchmarks/python/ffi_overhead.py --benchmark-only \
    --benchmark-json=benchmarks/reports/ffi_overhead.json

# Compare with baseline
pytest benchmarks/python/ffi_overhead.py --benchmark-only \
    --benchmark-compare=benchmarks/reports/baseline.json
```

---

### Category 5: Integration Benchmarks

**File:** `benchmarks/python/integration.py`

```python
"""
End-to-End Integration Benchmarks

Tests complete workflows across subsystems.
"""

import pytest


@pytest.mark.benchmark(group="integration")
@pytest.mark.slow
def test_neural_network_training_epoch(benchmark):
    """Benchmark one training epoch of residue neural network."""
    from hcvlang import ResidueConfidenceNetwork

    network = ResidueConfidenceNetwork(512, [256, 128])

    # Create training batch
    batch_size = 32
    inputs = [[i % 100 for i in range(512)] for _ in range(batch_size)]
    targets = [1 if i % 2 == 0 else 0 for i in range(batch_size)]

    def train_epoch():
        for input_vec, target in zip(inputs, targets):
            output = network.forward(input_vec)
            # In real training, would compute loss and backprop here

    result = benchmark(train_epoch)


@pytest.mark.benchmark(group="integration")
def test_fhe_encrypted_computation_pipeline(benchmark):
    """Benchmark end-to-end encrypted computation."""
    from hcvlang import FHEContext, SecurityLevel

    def encrypted_pipeline():
        ctx = FHEContext(SecurityLevel.Toy)
        sk, pk = ctx.generate_keypair()
        ek = ctx.generate_evaluation_key(sk)

        # Encrypt inputs
        ct1 = ctx.encrypt(ctx.encode(10), pk)
        ct2 = ctx.encrypt(ctx.encode(32), pk)

        # Compute: (10 + 32) * 2 = 84
        ct_sum = ctx.add(ct1, ct2)
        ct_two = ctx.encrypt(ctx.encode(2), pk)
        ct_result = ctx.mul(ct_sum, ct_two, ek)

        # Decrypt
        result = ctx.decode(ctx.decrypt(ct_result, sk))
        assert result == 84

    benchmark(encrypted_pipeline)


@pytest.mark.benchmark(group="integration")
def test_storage_encode_decode_pipeline(benchmark):
    """Benchmark holographic storage encode/decode."""
    from hcvlang import HolographicEncoder, IntegerMatrix

    encoder = HolographicEncoder(1024)

    # Create test data
    data = IntegerMatrix(32, 32)
    for i in range(32):
        for j in range(32):
            data.set(i, j, i * 32 + j)

    def encode_decode():
        encoded = encoder.encode(data)
        decoded = encoder.decode(encoded)
        return decoded

    result = benchmark(encode_decode)
```

**Expected Integration Times:**
- NN training epoch (32 samples): <100ms
- FHE encrypted pipeline: <50ms
- Storage encode/decode: <10ms
- Cross-subsystem workflows: <200ms

---

## Benchmark Execution Plan

### Setup Phase

```bash
# Install Criterion for Rust benchmarks
cd hcvlang
cargo install cargo-criterion

# Install pytest-benchmark for Python
pip3 install pytest-benchmark --break-system-packages

# Create benchmark directories
mkdir -p benchmarks/rust
mkdir -p benchmarks/python
mkdir -p benchmarks/reports
```

### Rust Benchmark Execution

```bash
# Run all Rust benchmarks
cd hcvlang
cargo bench --features python

# Run specific benchmark
cargo bench --bench core_arithmetic

# Save baseline
cargo bench --save-baseline current

# Compare with baseline
cargo bench --baseline current

# Generate HTML report
cargo criterion
```

### Python Benchmark Execution

```bash
# Run all Python benchmarks
pytest benchmarks/python/ --benchmark-only --benchmark-verbose

# Save baseline
pytest benchmarks/python/ --benchmark-only \
    --benchmark-save=baseline_2025_11_17

# Compare with baseline
pytest benchmarks/python/ --benchmark-only \
    --benchmark-compare=baseline_2025_11_17

# Generate HTML report
pytest benchmarks/python/ --benchmark-only \
    --benchmark-histogram=benchmarks/reports/histogram
```

### Memory Profiling

```bash
# Profile Rust memory usage
cargo install cargo-profiler
cd hcvlang
cargo profiler cachegrind --bench core_arithmetic

# Profile Python memory
pip3 install memory-profiler --break-system-packages
python -m memory_profiler benchmarks/python/integration.py
```

---

## Performance Targets

### Core Arithmetic
| Operation | Target | Critical |
|-----------|--------|----------|
| CRTBigInt add | <500ns | <1µs |
| CRTBigInt mul | <500ns | <1µs |
| ModInt add | <10ns | <50ns |
| ModInt mul (Montgomery) | <50ns | <100ns |
| Rational ops | <1µs | <5µs |

### Neural Networks
| Operation | Target | Critical |
|-----------|--------|----------|
| Montgomery mul | <10ns | <50ns |
| Residue forward pass | <10µs | <50µs |
| SIMD speedup | 6-8× | 4× minimum |
| Training epoch (32 samples) | <100ms | <500ms |

### Cryptography
| Operation | Target | Critical |
|-----------|--------|----------|
| FHE encrypt (standard) | <5ms | <10ms |
| FHE encrypt (real-time) | <1ms | <2ms |
| FHE add | <200µs | <1ms |
| FHE mul | <10ms | <20ms |
| Batch encryption (100, 8 cores) | 6-8× speedup | 4× minimum |

### FFI Overhead
| Operation | Target | Critical |
|-----------|--------|----------|
| Simple construction | <1µs | <5µs |
| Arithmetic operation | <2µs | <10µs |
| Batch operation (100 items) | 4× speedup | 2× minimum |

### Integration
| Workflow | Target | Critical |
|-----------|--------|----------|
| NN training epoch | <100ms | <500ms |
| FHE pipeline | <50ms | <200ms |
| Storage encode/decode | <10ms | <50ms |

---

## Reporting Format

### Summary Report Structure

```markdown
# QMNF System Performance Benchmark Report

**Date:** YYYY-MM-DD
**Commit:** [hash]
**Platform:** [CPU, RAM, OS]
**Rust Version:** [version]
**Python Version:** [version]

## Executive Summary

- **Total Benchmarks:** XXX
- **Passed Targets:** XX%
- **Critical Failures:** X
- **Performance Rating:** [Excellent/Good/Needs Improvement]

## Results by Category

### Core Arithmetic
| Benchmark | Time | Target | Status |
|-----------|------|--------|--------|
| CRTBigInt add | XXns | <500ns | ✅/❌ |
| ... | | | |

### Neural Networks
[Similar table]

### Cryptography
[Similar table]

### FFI Overhead
[Similar table]

### Integration
[Similar table]

## Performance Highlights

- ✅ [Achievement 1]
- ✅ [Achievement 2]
- ⚠️  [Concern 1]

## Bottlenecks Identified

1. [Bottleneck description]
   - Impact: [High/Medium/Low]
   - Recommendation: [Action item]

## Comparison with Baseline

[Table showing improvements/regressions]

## Recommendations

1. [Optimization suggestion 1]
2. [Optimization suggestion 2]

## Detailed Results

[Link to full criterion reports]
[Link to pytest-benchmark JSON]
```

---

## Deliverables

1. **Rust Benchmark Results:** Criterion HTML reports
2. **Python Benchmark Results:** pytest-benchmark JSON/HTML
3. **Summary Report:** Markdown with key findings
4. **Performance Dashboard:** HTML interactive charts
5. **Baseline Data:** JSON files for future comparisons
6. **Optimization Recommendations:** Prioritized action items
7. **Memory Profile:** Allocation patterns and hotspots

---

## Execution Timeline

| Phase | Duration | Tasks |
|-------|----------|-------|
| Setup | 2 hours | Install tools, create benchmarks |
| Rust Microbenchmarks | 4-6 hours | Run all Criterion benchmarks |
| Python FFI Benchmarks | 3-4 hours | Run pytest-benchmark suite |
| Integration Benchmarks | 3-4 hours | End-to-end workflows |
| Memory Profiling | 2-3 hours | Profile allocations |
| Report Generation | 2-3 hours | Create summary reports |
| Analysis | 2-3 hours | Identify bottlenecks |
| **Total** | **16-24 hours** | |

---

## Success Criteria

### Critical (Must Pass)
- [ ] All benchmarks complete without crashes
- [ ] 90% of targets met
- [ ] Zero critical failures
- [ ] Baseline established for all subsystems
- [ ] Memory leaks < 1% in extended runs

### Performance (Should Meet)
- [ ] Montgomery arithmetic <50ns
- [ ] SIMD speedup ≥6×
- [ ] Batch operations ≥4× speedup
- [ ] FHE real-time <2ms
- [ ] Integration workflows <500ms

### Deliverables (Must Provide)
- [ ] Complete benchmark suite
- [ ] HTML reports generated
- [ ] JSON baseline data saved
- [ ] Summary markdown report
- [ ] Optimization recommendations

---

## Notes

- Run benchmarks on dedicated hardware (no background tasks)
- Use release builds only (`--release` flag)
- Warm up CPU (run benchmarks twice, use second run)
- Record system specs (CPU, RAM, OS) in reports
- Archive baseline data for historical comparison
- Document any anomalies or unexpected results

---

**Status:** READY FOR EXECUTION
**Assignee:** AI Benchmarking Team
**Expected Completion:** 16-24 hours
**Deliverable:** Complete performance profile of QMNF System

# PLMG Test Implementation Guide

**Document:** PLMG_TEST_IMPLEMENTATION_GUIDE.md  
**Date:** December 4, 2025  
**Status:** Implementation Roadmap  
**Version:** 1.0

This companion document provides step-by-step guidance for implementing the test strategy defined in `PLMG_COMPREHENSIVE_TEST_STRATEGY.md`.

---

## Phase 1: Foundation Setup (Week 1)

### Step 1.1: Create Test Module Structure

```bash
# Create theorem test modules
touch hcvlang/tests/plmg_theorem_1_k_elimination.rs
touch hcvlang/tests/plmg_theorem_2_phase_locked_periodicity.rs
touch hcvlang/tests/plmg_theorem_3_exact_division.rs
touch hcvlang/tests/plmg_theorem_4_magnitude_comparison.rs
touch hcvlang/tests/plmg_theorem_5_sign_encoding.rs
touch hcvlang/tests/plmg_theorem_6_polynomial_division.rs
touch hcvlang/tests/plmg_theorem_7_hierarchical_gearing.rs
touch hcvlang/tests/plmg_theorem_8_zero_churn_addition.rs
touch hcvlang/tests/plmg_theorem_9_deterministic_property.rs
touch hcvlang/tests/plmg_theorem_10_zero_error.rs
touch hcvlang/tests/plmg_property_tests.rs

# Create integration test modules
touch hcvlang/tests/plmg_integration_with_crt_bigint.rs
touch hcvlang/tests/plmg_integration_with_adaptive_crt.rs
touch hcvlang/tests/plmg_integration_with_fhe.rs
touch hcvlang/tests/plmg_integration_with_neural_primitives.rs
touch hcvlang/tests/plmg_integration_with_modint.rs
touch hcvlang/tests/plmg_integration_with_rational.rs
```

### Step 1.2: Create Benchmark Module Structure

```bash
# Create benchmark files
touch hcvlang/benches/plmg_core_benchmarks.rs
touch hcvlang/benches/plmg_division_benchmark.rs
touch hcvlang/benches/plmg_comparison_benchmark.rs
touch hcvlang/benches/plmg_hierarchical_benchmark.rs
touch hcvlang/benches/plmg_ffi_benchmark.rs
touch hcvlang/benches/plmg_batch_operations_benchmark.rs
```

### Step 1.3: Register Tests in Cargo.toml

```toml
# Add to hcvlang/Cargo.toml [dev-dependencies]
proptest = "1.0"
quickcheck = "1.0"
quickcheck_macros = "1.0"

# Add test registrations
[[test]]
name = "plmg_theorem_1_k_elimination"
harness = true

[[test]]
name = "plmg_theorem_2_phase_locked_periodicity"
harness = true

# ... add remaining 8 theorem tests ...

[[test]]
name = "plmg_property_tests"
harness = true

# Add benchmark registrations
[[bench]]
name = "plmg_core_benchmarks"
harness = false

[[bench]]
name = "plmg_division_benchmark"
harness = false

# ... add remaining benchmarks ...
```

### Step 1.4: Create Module Template

Create a standard template for each theorem test:

```rust
// hcvlang/tests/plmg_theorem_N_XXXX.rs

//! Tests for Theorem N: [Theorem Title]
//!
//! This module validates all properties of Theorem N as defined in
//! PLMG_Extended_Theorems.md.
//!
//! Test Count: X test cases
//! Coverage: > 95% of theorem-related code

#[cfg(test)]
mod plmg_theorem_n_tests {
    use super::*;
    use hcvlang::plmg::*;  // Import PLMG implementation

    // Standard test data
    fn setup() -> (Vec<u64>, Vec<u64>) {
        let primary_moduli = vec![2u64.pow(31) - 1, 2u64.pow(31) - 19];
        let reference_moduli = vec![2u64.pow(32) - 5];
        (primary_moduli, reference_moduli)
    }

    #[test]
    fn test_n_1_core_property() {
        let (primary, reference) = setup();
        // Test implementation
    }

    // ... additional tests ...
}
```

---

## Phase 2: Implement Unit Tests (Weeks 2-4)

### Implementation Priority Order

```
Week 2:
  [ ] Theorem 1: K-Elimination (7 tests)
  [ ] Theorem 2: Phase-Locked Periodicity (8 tests)
  [ ] Theorem 3: Exact Division (7 tests)

Week 3:
  [ ] Theorem 4: Magnitude Comparison (4 tests)
  [ ] Theorem 5: Sign Encoding (5 tests)
  [ ] Theorem 6: Polynomial Division (6 tests)

Week 4:
  [ ] Theorem 7: Hierarchical Gearing (4 tests)
  [ ] Theorem 8: Zero-Churn Addition (3 tests)
  [ ] Theorem 9: Deterministic Property (6 tests)
  [ ] Theorem 10: Zero Error (7 tests)
  [ ] Property-based tests (8 properties)
```

### Checklist for Each Theorem Test

For each theorem implementation:

- [ ] **Read** the theorem proof in PLMG_Extended_Theorems.md
- [ ] **Identify** 3-5 key properties to test
- [ ] **Design** test cases covering:
  - Core property validation
  - Edge cases
  - Multiple configurations
  - Complexity bounds
  - Error conditions
- [ ] **Implement** test functions
  - Use descriptive names (test_XXX_property_description)
  - Add documentation comments
  - Include assertions with helpful messages
- [ ] **Run locally** with `cargo test`
- [ ] **Verify** 100% pass rate

### Testing Theorem 1 (K-Elimination) - Complete Example

```rust
// hcvlang/tests/plmg_theorem_1_k_elimination.rs

#[cfg(test)]
mod plmg_theorem_1_tests {
    use super::*;
    use hcvlang::plmg::*;

    fn setup() -> (Vec<u64>, Vec<u64>) {
        (vec![2u64.pow(31) - 1, 2u64.pow(31) - 19],
         vec![2u64.pow(32) - 5])
    }

    #[test]
    fn test_phase_differential_equals_overflow_count() {
        let (primary, reference) = setup();
        let c_p = primary.iter().product::<u64>();
        let c_r = reference.iter().product::<u64>();
        let c_total = c_p.saturating_mul(c_r);

        for x in (0..std::cmp::min(c_total, 1_000_000)).step_by(1000) {
            let phase = PLMGSystem::phase_differential(x, &primary, &reference);
            let overflow = PLMGSystem::overflow_count(x, &primary);

            assert_eq!(phase, overflow,
                "Mismatch at x={}: phase={}, overflow={}", x, phase, overflow);
        }
    }

    #[test]
    fn test_reconstruction_from_phase_exact() {
        let (primary, reference) = setup();

        for x in [0u64, 1, 42, 12345678, u64::MAX / 2] {
            let phase = PLMGSystem::phase_differential(x, &primary, &reference);
            let reconstructed = PLMGSystem::reconstruct(phase, &primary, &reference);

            assert_eq!(x, reconstructed,
                "Reconstruction failed for x={}", x);
        }
    }

    // ... implement remaining 5 tests from PLMG_COMPREHENSIVE_TEST_STRATEGY.md ...
}
```

---

## Phase 3: Implement Integration Tests (Week 5)

### Integration Test Checklist

For each integration (6 total):

- [ ] **CRTBigInt Integration**
  - [ ] PLMG ↔ CRTBigInt conversion
  - [ ] Arithmetic compatibility
  - [ ] Value range testing

- [ ] **Adaptive CRT Integration**
  - [ ] Tier promotion with PLMG
  - [ ] Hierarchical gearing extension
  - [ ] Phase differential preservation

- [ ] **FHE Integration**
  - [ ] Exact polynomial division in encrypted domain
  - [ ] Zero error in FHE operations
  - [ ] Ciphertext compatibility

- [ ] **Neural Primitive Integration**
  - [ ] Zero-drift weight updates
  - [ ] Activation function precision
  - [ ] Gradient computation exactness

- [ ] **ModInt Integration**
  - [ ] PLMG + modular arithmetic
  - [ ] Mersenne prime compatibility
  - [ ] Batch modular operations

- [ ] **Rational Integration**
  - [ ] PLMG rational numbers
  - [ ] Exact fraction arithmetic
  - [ ] GCD computations

### Example Integration Test

```rust
// hcvlang/tests/plmg_integration_with_crt_bigint.rs

#[cfg(test)]
mod plmg_crt_integration {
    use super::*;
    use hcvlang::plmg::*;
    use hcvlang::crt_bigint::CRTBigInt;

    #[test]
    fn test_plmg_to_crtbigint_exact_conversion() {
        let (primary, reference) = setup();

        for x in [123u64, 456, 789, 999_999] {
            let plmg_val = PLMGSystem::new(x, &primary, &reference);
            let crt_val = CRTBigInt::from_plmg(&plmg_val);
            let plmg_restored = PLMGSystem::from_crt(&crt_val, &primary, &reference);

            assert_eq!(plmg_val, plmg_restored,
                "Round-trip conversion failed for x={}", x);
        }
    }

    #[test]
    fn test_plmg_arithmetic_equals_crt() {
        let (primary, reference) = setup();

        for (a, b) in [(10u64, 3), (100, 7), (1000, 13)] {
            let plmg_sum = PLMGSystem::add(a, b, &primary, &reference);
            let crt_sum = CRTBigInt::from(a) + CRTBigInt::from(b);

            assert_eq!(plmg_sum, CRTBigInt::to_u64(&crt_sum),
                "PLMG and CRT addition differ for {}, {}", a, b);
        }
    }

    // ... implement 8+ more integration tests ...
}
```

---

## Phase 4: Implement Benchmarks (Weeks 5-7)

### Benchmark Implementation Priority

```
Priority 1 (Week 5):
  [ ] plmg_core_benchmarks.rs - Phase differential, reconstruction
  [ ] plmg_division_benchmark.rs - 40× speedup validation

Priority 2 (Week 6):
  [ ] plmg_comparison_benchmark.rs - O(n+m) confirmation
  [ ] plmg_hierarchical_benchmark.rs - Multi-level performance

Priority 3 (Week 7):
  [ ] plmg_ffi_benchmark.rs - Python FFI overhead
  [ ] plmg_batch_operations_benchmark.rs - Vectorization
```

### Benchmark Template (Criterion)

```rust
// hcvlang/benches/plmg_core_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hcvlang::plmg::*;

fn setup() -> (Vec<u64>, Vec<u64>) {
    (vec![2u64.pow(31) - 1, 2u64.pow(31) - 19],
     vec![2u64.pow(32) - 5])
}

fn bench_phase_differential(c: &mut Criterion) {
    let (primary, reference) = setup();

    c.bench_function("phase_differential_small", |b| {
        b.iter(|| {
            PLMGSystem::phase_differential(black_box(42u64), &primary, &reference)
        });
    });

    c.bench_function("phase_differential_large", |b| {
        b.iter(|| {
            PLMGSystem::phase_differential(black_box(u64::MAX / 2), &primary, &reference)
        });
    });
}

fn bench_reconstruction(c: &mut Criterion) {
    let (primary, reference) = setup();
    let phase = 17u64;

    c.bench_function("reconstruct_from_phase", |b| {
        b.iter(|| {
            PLMGSystem::reconstruct(black_box(phase), &primary, &reference)
        });
    });
}

fn bench_division_piggyback_vs_naive(c: &mut Criterion) {
    let (primary, reference) = setup();

    c.bench_function("piggyback_division", |b| {
        b.iter(|| {
            PLMGSystem::divide_piggyback(
                black_box(12_345_678u64),
                black_box(97u64),
                &primary,
                &reference
            )
        });
    });

    c.bench_function("naive_crt_division", |b| {
        b.iter(|| {
            PLMGSystem::divide_naive_crt(
                black_box(12_345_678u64),
                black_box(97u64),
                &primary,
                &reference
            )
        });
    });
}

criterion_group!(benches, bench_phase_differential, bench_reconstruction, bench_division_piggyback_vs_naive);
criterion_main!(benches);
```

### Running Benchmarks

```bash
# Run all PLMG benchmarks
cd hcvlang
cargo bench --bench plmg_core_benchmarks

# Generate comparison report
cargo bench --bench plmg_core_benchmarks -- --output-format bencher | tee /tmp/bench_latest.txt

# Compare against baseline
git show main:hcvlang/benches/plmg_core_benchmarks.rs > /tmp/bench_baseline.rs
# (analyze deltas)
```

---

## Phase 5: Property-Based Testing (Week 4)

### Using proptest

```rust
// hcvlang/tests/plmg_property_tests.rs

use proptest::prelude::*;

#[cfg(test)]
mod plmg_properties {
    use super::*;
    use hcvlang::plmg::*;

    fn setup() -> (Vec<u64>, Vec<u64>) {
        (vec![2u64.pow(31) - 1, 2u64.pow(31) - 19],
         vec![2u64.pow(32) - 5])
    }

    proptest! {
        #[test]
        fn prop_phase_differential_in_range(
            x in 0u64..1_000_000
        ) {
            let (primary, reference) = setup();
            let c_r = reference.iter().product::<u64>();
            let phase = PLMGSystem::phase_differential(x, &primary, &reference);

            prop_assert!(phase < c_r,
                "Phase {} out of range [0, {})", phase, c_r);
        }

        #[test]
        fn prop_division_exactness(
            a in 1u64..1_000_000,
            b in 1u64..10_000
        ) {
            let (primary, reference) = setup();
            let (quotient, remainder) = PLMGSystem::divide(a, b, &primary, &reference);
            let reconstructed = quotient.saturating_mul(b).saturating_add(remainder);

            prop_assert_eq!(a, reconstructed,
                "Division inexact for {}/{}", a, b);
        }

        #[test]
        fn prop_addition_commutative(
            a in 0u64..100_000,
            b in 0u64..100_000
        ) {
            let (primary, reference) = setup();
            let left = PLMGSystem::add(a, b, &primary, &reference);
            let right = PLMGSystem::add(b, a, &primary, &reference);

            prop_assert_eq!(left, right,
                "Addition not commutative: {} + {} != {} + {}",
                a, b, b, a);
        }

        #[test]
        fn prop_comparison_transitive(
            x in 0u64..10_000,
            y in 0u64..10_000,
            z in 0u64..10_000
        ) {
            let (primary, reference) = setup();

            let x_lt_y = PLMGSystem::compare(x, y, &primary, &reference) == std::cmp::Ordering::Less;
            let y_lt_z = PLMGSystem::compare(y, z, &primary, &reference) == std::cmp::Ordering::Less;
            let x_lt_z = PLMGSystem::compare(x, z, &primary, &reference) == std::cmp::Ordering::Less;

            if x_lt_y && y_lt_z {
                prop_assert!(x_lt_z, "Transitivity violated");
            }
        }
    }
}
```

---

## Phase 6: Regression Testing & CI Setup (Weeks 7-8)

### Step 6.1: Establish Baseline

```bash
# Run existing test suite (PLMG disabled)
cd /home/acid/Projects/QMNF_System
cargo test --release --no-default-features 2>&1 | tee /tmp/baseline_tests.log
baseline_count=$(grep -c "test result:" /tmp/baseline_tests.log)
echo "Baseline: $baseline_count tests"
```

### Step 6.2: Create Regression Suite

```bash
# tests/regression_suite.sh

#!/bin/bash
set -e

BASELINE_TESTS=500  # Adjust to actual count

echo "=== PLMG Regression Test Suite ==="
echo "Running full test suite with PLMG features..."

cargo test --release 2>&1 | tee /tmp/plmg_test_results.log

# Extract test count
test_count=$(grep -o "[0-9]* test" /tmp/plmg_test_results.log | head -1 | grep -o "[0-9]*")
passed=$(grep "test result: ok" /tmp/plmg_test_results.log | wc -l)

echo ""
echo "Results:"
echo "  Total tests: $test_count"
echo "  Passed: $passed"
echo "  Baseline: $BASELINE_TESTS"

if [ "$passed" -ge "$BASELINE_TESTS" ]; then
    echo "✅ No regression detected"
    exit 0
else
    echo "❌ Regression! Expected ≥$BASELINE_TESTS, got $passed"
    exit 1
fi
```

### Step 6.3: Create GitHub Actions Workflow

```yaml
# .github/workflows/plmg_tests.yml

name: PLMG Comprehensive Tests

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        theorem: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Test Theorem ${{ matrix.theorem }}
        run: cargo test --test plmg_theorem_${{ matrix.theorem }}_ --release

  integration-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Run integration tests
        run: cargo test --test plmg_integration_* --release

  benchmarks:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Run benchmarks
        run: cargo bench --bench plmg_* --no-run && cargo bench --bench plmg_*

  regression:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Run regression suite
        run: bash tests/regression_suite.sh

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Install tarpaulin
        run: cargo install cargo-tarpaulin
      - name: Generate coverage
        run: cargo tarpaulin --out Xml --output-dir coverage
      - name: Upload coverage
        uses: codecov/codecov-action@v3
        with:
          files: ./coverage/cobertura.xml
```

---

## Phase 7: Coverage Analysis (Weeks 8-10)

### Generate Coverage Reports

```bash
# Install coverage tools
cargo install cargo-tarpaulin
pip install pytest-cov

# Generate Rust coverage
cd hcvlang
cargo tarpaulin --out Html --output-dir coverage/rust --timeout 300

# Generate Python coverage  
cd ../
python3 -m pytest tests/python/ --cov=hcvlang --cov-report=html:coverage/python

# View reports
open coverage/rust/tarpaulin-report.html
open coverage/python/index.html
```

### Identify and Fix Coverage Gaps

```bash
# Find uncovered lines
grep 'uncovered' coverage/rust/tarpaulin-report.html | head -20

# Add targeted tests for gaps
# Example: If phase_differential has uncovered branches,
#          add specific test cases exercising those branches
```

---

## Phase 8: Final Validation & Documentation (Weeks 10-12)

### Validation Checklist

- [ ] All 10 theorem test modules complete
- [ ] All integration tests passing
- [ ] Benchmark suite running with baselines established
- [ ] Coverage ≥85% across all modules
- [ ] Property tests passing
- [ ] Regression suite green
- [ ] CI/CD pipeline functional
- [ ] Cross-platform tests (Linux, macOS, Windows) passing
- [ ] Documentation complete

### Generate Final Test Report

```python
# tools/generate_plmg_test_report.py

import json
import subprocess
from datetime import datetime

def run_command(cmd):
    """Run shell command and capture output"""
    result = subprocess.run(cmd, shell=True, capture_output=True, text=True)
    return result.stdout + result.stderr

def generate_report():
    """Generate comprehensive PLMG test report"""

    report = {
        "timestamp": datetime.now().isoformat(),
        "test_summary": {
            "unit_tests": run_command("cargo test --test plmg_theorem_* -- --list | wc -l"),
            "integration_tests": run_command("cargo test --test plmg_integration_* -- --list | wc -l"),
            "benchmarks": run_command("cargo bench --bench plmg_* -- --list | wc -l"),
            "property_tests": run_command("cargo test --test plmg_property_tests -- --list | wc -l"),
        },
        "coverage": {
            "rust": extract_coverage("coverage/rust/tarpaulin-report.html"),
            "python": extract_coverage("coverage/python/index.html"),
        },
        "performance": {
            "phase_differential_ns": measure_operation("phase_differential"),
            "division_speedup": measure_speedup("division"),
            "comparison_complexity": verify_complexity("comparison"),
        },
        "validation": {
            "determinism": validate_determinism(1000),
            "zero_error": validate_zero_error(1_000_000),
            "cross_platform": validate_platforms(["linux", "macos", "windows"]),
        },
    }

    # Write report
    with open("test_results/PLMG_Test_Report_Final.json", "w") as f:
        json.dump(report, f, indent=2)

    # Generate markdown summary
    print_markdown_summary(report)

if __name__ == "__main__":
    generate_report()
```

---

## Quick Reference Commands

```bash
# Run all PLMG tests
cargo test --test plmg_* --release

# Run specific theorem
cargo test --test plmg_theorem_1 --release

# Run benchmarks
cargo bench --bench plmg_*

# Generate coverage
cargo tarpaulin --out Html --output-dir coverage

# Run property tests
cargo test --test plmg_property_tests --release

# Run full regression suite
bash tests/regression_suite.sh

# Run with output
cargo test --test plmg_theorem_1 -- --nocapture

# Parallel testing
cargo test --test plmg_* --release -- --test-threads=8

# Generate test report
python3 tools/generate_plmg_test_report.py
```

---

## Common Issues & Solutions

### Issue 1: Tests Timeout

**Solution:**
```bash
# Increase timeout for long-running tests
cargo test --test plmg_theorem_10_zero_error --release -- --test-threads=1

# Or add timeout attribute
#[test]
#[ignore]  // Run separately if too slow
fn test_million_operations() {
    // ...
}
```

### Issue 2: Flaky Benchmarks

**Solution:**
```bash
# Run benchmarks in isolation
cargo bench --bench plmg_core_benchmarks

# Use Criterion's statistical analysis
# Results automatically skip outliers and compute variance
```

### Issue 3: Coverage Not Increasing

**Solution:**
```bash
# Run with verbose output to identify untested paths
RUST_LOG=debug cargo test --test plmg_theorem_1 -- --nocapture

# Add `#[cfg(test)]` blocks for test-only code
#[cfg(test)]
mod tests {
    // Test implementation
}
```

---

## Success Metrics Dashboard

Track these metrics weekly:

```markdown
| Metric | Target | Week 2 | Week 4 | Week 6 | Week 8 | Week 10 | Week 12 |
|--------|--------|--------|--------|---------|---------|----------|----------|
| Unit Tests | 1200 | 250 | 600 | 900 | 1100 | 1200 | 1200 |
| Integration Tests | 50 | 5 | 15 | 25 | 35 | 45 | 50 |
| Coverage % | 85% | 60% | 70% | 75% | 80% | 85% | 85% |
| Benchmarks | 200+ | 30 | 80 | 150 | 180 | 200+ | 200+ |
| Determinism | 1000 runs | - | - | - | - | ✅ | ✅ |
| Zero Error | 1M ops | - | - | - | ✅ | ✅ | ✅ |
| CI/CD Green | 100% | - | - | ✅ | ✅ | ✅ | ✅ |
```

---

**Document Status:** Final - Implementation Roadmap  
**Last Updated:** December 4, 2025

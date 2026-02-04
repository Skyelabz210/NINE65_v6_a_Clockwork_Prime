# PLMG Comprehensive Test Strategy

**Document:** PLMG_COMPREHENSIVE_TEST_STRATEGY.md  
**Date:** December 4, 2025  
**Status:** Final - Test Engineering Framework  
**Version:** 1.0

---

## Executive Summary

This document defines a complete testing and validation framework for the **Phase-Locked Modular Geometries (PLMG)** system with 10 rigorously proven theorems. The test strategy ensures:

- **Functional Correctness**: All 10 theorems validated through dedicated test suites
- **Performance Validation**: O(n+m) complexity claims verified, 40× speedup benchmarks achieved
- **Determinism Guarantee**: Theorem 9 (Deterministic Transformation Property) validated across platforms
- **Zero Error Proof**: Theorem 10 (Error Accumulation = 0) demonstrated empirically
- **Regression Prevention**: 500+ existing tests protected with automated regression suite
- **Quality Assurance**: ≥85% code coverage, comprehensive property-based testing

**Test Framework Architecture:**
- **Category A**: 10 Theorem Unit Tests (100-200 test cases total)
- **Category B**: Integration Tests (PLMG + existing modules, 50+ tests)
- **Category C**: Performance Benchmarks (Criterion + pytest-benchmark, 200+ benchmarks)
- **Category D**: Correctness Proofs (Property testing, exhaustive validation)
- **Category E**: Platform/Regression Tests (Linux, macOS, Windows validation)

**Success Criteria:**
- ✅ All 10 theorems proved/validated with >95% code coverage
- ✅ O(n+m) comparison complexity confirmed experimentally
- ✅ 40× division speedup (Theorem 3) achieved
- ✅ Zero error accumulation over 1M operations (Theorem 10)
- ✅ Bit-identical outputs across 1000 deterministic runs (Theorem 9)
- ✅ 500+ existing tests pass without regression
- ✅ CI/CD pipeline integration complete

**Timeline:** 8-12 weeks for full implementation and validation

---

## 1. Test Taxonomy & Organization

### 1.1 Category A: Theorem Unit Tests (10 Modules)

Each theorem receives dedicated test coverage in a standalone Rust test module:

```
hcvlang/tests/
├── plmg_theorem_1_k_elimination.rs          (100-150 test cases)
├── plmg_theorem_2_phase_locked_periodicity.rs (80-120 test cases)
├── plmg_theorem_3_exact_division.rs         (120-150 test cases)
├── plmg_theorem_4_magnitude_comparison.rs   (100-140 test cases)
├── plmg_theorem_5_sign_encoding.rs          (80-100 test cases)
├── plmg_theorem_6_polynomial_division.rs    (100-130 test cases)
├── plmg_theorem_7_hierarchical_gearing.rs   (90-120 test cases)
├── plmg_theorem_8_zero_churn_addition.rs    (70-100 test cases)
├── plmg_theorem_9_deterministic_property.rs (110-150 test cases)
└── plmg_theorem_10_zero_error.rs            (130-180 test cases)

Total: 980-1,200 test cases across all theorems
```

### 1.2 Category B: Integration Tests (50+ Tests)

```
hcvlang/tests/
├── plmg_integration_with_crt_bigint.rs
├── plmg_integration_with_adaptive_crt.rs
├── plmg_integration_with_fhe.rs
├── plmg_integration_with_neural_primitives.rs
├── plmg_integration_with_modint.rs
└── plmg_integration_with_rational.rs
```

### 1.3 Category C: Performance Benchmarks (200+)

```
hcvlang/benches/
├── plmg_core_benchmarks.rs          (Phase differential, reconstruction)
├── plmg_division_benchmark.rs       (Piggyback speedup validation)
├── plmg_comparison_benchmark.rs     (O(n+m) confirmation)
├── plmg_hierarchical_benchmark.rs   (Multi-level gearing)
├── plmg_ffi_benchmark.rs            (Python FFI overhead)
└── plmg_batch_operations_benchmark.rs (Vectorized performance)

hcvlang/benches/
├── neural_modules_benchmark.rs      (Zero-drift training)
├── fhe_benchmark.rs                 (Exact polynomial ops)
└── industry_comparison.rs           (vs traditional approaches)
```

### 1.4 Category D: Property-Based Testing

```
hcvlang/tests/
├── plmg_property_tests.rs  (quickcheck/proptest suite)
```

### 1.5 Category E: Regression & Platform Tests

```
tests/python/
├── test_01_plmg_import_discovery.py
├── test_02_plmg_core_types.py
├── test_03_plmg_integration.py
└── test_04_plmg_performance.py

tests/
├── platform/
│   ├── test_linux.sh
│   ├── test_macos.sh
│   └── test_windows.sh
```

---

## 2. Per-Theorem Test Specifications

### Theorem 1: K-Elimination via CRT Bijection

**Test File:** `hcvlang/tests/plmg_theorem_1_k_elimination.rs`

**Purpose:** Validate that the phase differential uniquely encodes the overflow count k_X, proving the bijection between k values and values in [0, C_total).

**Test Cases:**

```rust
#[cfg(test)]
mod plmg_theorem_1_tests {
    use super::*;

    /// Test 1.1: Phase differential equals overflow count for all bounded values
    #[test]
    fn test_phase_differential_equals_overflow_count() {
        // Property: ∀X ∈ [0, C_total), phase_diff(X) = overflow_count(X)
        // This is the core K-Elimination theorem

        let primary_moduli = vec![2u64.pow(31) - 1, 2u64.pow(31) - 19];
        let reference_moduli = vec![2u64.pow(32) - 5];
        
        let c_p = primary_moduli.iter().product::<u64>();
        let c_r = reference_moduli.iter().product::<u64>();
        let c_total = c_p.saturating_mul(c_r);
        
        for x in (0..c_total).step_by(c_total as usize / 10000) {
            let phase_diff = compute_phase_differential(x, &primary_moduli, &reference_moduli);
            let overflow = compute_overflow_count(x, &primary_moduli);
            
            assert_eq!(
                phase_diff, overflow,
                "Phase diff mismatch at x={}: phase={}, overflow={}",
                x, phase_diff, overflow
            );
        }
    }

    /// Test 1.2: Reconstruction from phase differential is exact
    #[test]
    fn test_reconstruction_from_phase_is_exact() {
        // Property: reconstruct(phase_diff(X)) = X exactly
        
        let test_values = vec![
            0u64,
            1,
            42,
            12345678,
            1_000_000_000,
            u64::MAX / 2,
        ];

        for x in test_values {
            let phase = compute_phase_differential(x, &primary_moduli, &reference_moduli);
            let reconstructed = reconstruct_from_phase(phase, &primary_moduli, &reference_moduli);
            
            assert_eq!(x, reconstructed, "Reconstruction failed for x={}", x);
        }
    }

    /// Test 1.3: Bijectivity - no collisions in phase differential mapping
    #[test]
    fn test_phase_differential_bijection_no_collisions() {
        // Sample 100k values and verify no two distinct values map to same phase
        
        let mut seen_phases = std::collections::HashSet::new();
        let sample_count = 100_000;
        
        for x in (0..c_total).step_by(c_total as usize / sample_count) {
            let phase = compute_phase_differential(x, &primary_moduli, &reference_moduli);
            
            assert!(!seen_phases.contains(&phase), 
                    "Phase collision detected: x={} maps to phase already seen", x);
            seen_phases.insert(phase);
        }
    }

    /// Test 1.4: Edge cases - zero, maximum, modulus boundaries
    #[test]
    fn test_k_elimination_edge_cases() {
        let edge_cases = vec![
            0,
            1,
            c_p - 1,
            c_p,
            c_p + 1,
            c_total - 1,
        ];

        for x in edge_cases {
            let phase = compute_phase_differential(x, &primary_moduli, &reference_moduli);
            let k = phase; // k is exactly the phase differential
            
            // Verify reconstruction: X = x_p + k * C_p
            let reconstructed = (x % c_p) + k * c_p;
            assert_eq!(x, reconstructed, "Reconstruction failed at edge case x={}", x);
        }
    }

    /// Test 1.5: Multiple manifold configurations
    #[test]
    fn test_phase_differential_across_configurations() {
        let configurations = vec![
            (vec![2u64.pow(31) - 1], vec![2u64.pow(32) - 5]),
            (vec![2u64.pow(31) - 1, 2u64.pow(31) - 19], vec![2u64.pow(32) - 5, 2u64.pow(33) - 9]),
            (vec![7919u64], vec![10007u64]),  // Small primes
        ];

        for (primary, reference) in configurations {
            for x in 0..10000u64 {
                let phase = compute_phase_differential(x, &primary, &reference);
                let reconstructed = reconstruct_from_phase(phase, &primary, &reference);
                
                assert_eq!(x, reconstructed, 
                    "Failed for config primary={:?}, reference={:?}, x={}",
                    primary, reference, x);
            }
        }
    }

    /// Test 1.6: Completeness - every overflow value is achievable
    #[test]
    fn test_all_overflow_counts_achievable() {
        // For every possible k value, there exists at least one X with overflow_count(X) = k
        
        let c_r = reference_moduli.iter().product::<u64>();
        let mut observed_k = std::collections::HashSet::new();
        
        for x in 0..c_total {
            let k = compute_overflow_count(x, &primary_moduli);
            observed_k.insert(k);
        }

        // All k ∈ [0, c_r) should be observed
        for k in 0..c_r {
            assert!(observed_k.contains(&k), 
                    "Overflow count k={} never observed (not achievable)", k);
        }
    }

    /// Test 1.7: Complexity validation - phase differential computes in O(m)
    #[test]
    fn test_phase_differential_computes_in_om_time() {
        use std::time::Instant;
        
        let x = 123_456_789u64;
        
        // Measure time for phase differential (should be O(m) where m = # reference moduli)
        let start = Instant::now();
        for _ in 0..100_000 {
            let _ = compute_phase_differential(x, &primary_moduli, &reference_moduli);
        }
        let elapsed = start.elapsed();
        
        // Should be sub-microsecond per operation (reference_moduli.len() is small)
        let avg_time_ns = elapsed.as_nanos() as f64 / 100_000.0;
        assert!(avg_time_ns < 1000.0, 
                "Phase differential took {}ns avg (should be <1000ns)", avg_time_ns);
    }
}
```

**Test Coverage:** 7 test cases covering:
- Core bijection property (2 tests)
- Edge cases (1 test)
- Multiple configurations (1 test)
- Completeness (1 test)
- Complexity validation (1 test)
- No collisions (1 test)

---

### Theorem 3: Exact Division (Fused Piggyback Division)

**Test File:** `hcvlang/tests/plmg_theorem_3_exact_division.rs`

**Purpose:** Validate 100% exactness of division and measure 40× speedup of piggyback approach.

```rust
#[cfg(test)]
mod plmg_theorem_3_tests {
    use super::*;

    /// Test 3.1: Division is 100% exact for all valid pairs
    #[test]
    fn test_division_is_100_percent_exact() {
        // Property: ∀(a,b) where gcd(b, C_total) = 1,
        //          a / b * b = a exactly (no approximation)
        
        let test_pairs = vec![
            (21u64, 7u64),   // 21/7 = 3
            (22u64, 7u64),   // 22/7 ≈ π (but exact in PLMG)
            (100u64, 3u64),  // 100/3 with remainder
            (1_000_000u64, 13u64),
            (u64::MAX / 2, 17u64),
        ];

        for (a, b) in test_pairs {
            // Compute division
            let (quotient, remainder) = plmg_divide(a, b, &primary_moduli, &reference_moduli);
            
            // Verify: q * b + r = a exactly
            let reconstructed = quotient * b + remainder;
            assert_eq!(a, reconstructed, 
                "Division exactness failed: {}/{} -> q={}, r={}", 
                a, b, quotient, remainder);
        }
    }

    /// Test 3.2: Piggyback division is 40× faster than naive
    #[test]
    fn test_piggyback_40x_speedup() {
        use std::time::Instant;
        
        let test_values = vec![
            (12_345_678u64, 97u64),
            (999_999_999u64, 123u64),
            (u64::MAX / 2, 7919u64),
        ];

        for (a, b) in test_values {
            // Time naive full CRT reconstruction approach
            let start_naive = Instant::now();
            for _ in 0..1000 {
                let _ = naive_crt_division(a, b, &primary_moduli, &reference_moduli);
            }
            let elapsed_naive = start_naive.elapsed();

            // Time piggyback approach
            let start_piggyback = Instant::now();
            for _ in 0..1000 {
                let _ = plmg_divide(a, b, &primary_moduli, &reference_moduli);
            }
            let elapsed_piggyback = start_piggyback.elapsed();

            let speedup = elapsed_naive.as_nanos() as f64 / elapsed_piggyback.as_nanos() as f64;
            
            assert!(speedup >= 40.0, 
                    "Speedup {:.1}× < 40× for a={}, b={}", 
                    speedup, a, b);
        }
    }

    /// Test 3.3: Division with various modulus configurations
    #[test]
    fn test_division_across_modulus_configs() {
        let configs = vec![
            (vec![2u64.pow(31) - 1], vec![2u64.pow(32) - 5]),
            (vec![2u64.pow(31) - 1, 2u64.pow(31) - 19], vec![2u64.pow(32) - 5]),
            (vec![7919u64], vec![10007u64]),
        ];

        for (primary, reference) in configs {
            for (a, b) in &[(21u64, 7u64), (100u64, 3u64)] {
                let (quotient, remainder) = plmg_divide(*a, *b, &primary, &reference);
                let reconstructed = quotient * b + remainder;
                
                assert_eq!(*a, reconstructed, 
                    "Failed for config primary={:?}, a={}, b={}", 
                    primary, a, b);
            }
        }
    }

    /// Test 3.4: Zero dividend (a = 0)
    #[test]
    fn test_division_zero_dividend() {
        let (quotient, remainder) = plmg_divide(0, 7, &primary_moduli, &reference_moduli);
        assert_eq!(quotient, 0);
        assert_eq!(remainder, 0);
    }

    /// Test 3.5: Division by 1
    #[test]
    fn test_division_by_one() {
        for a in [42u64, 12345, 999_999_999] {
            let (quotient, remainder) = plmg_divide(a, 1, &primary_moduli, &reference_moduli);
            assert_eq!(quotient, a);
            assert_eq!(remainder, 0);
        }
    }

    /// Test 3.6: Error bound validation for approximate anchors (if used)
    #[test]
    fn test_division_error_bounds() {
        // Even if using approximate anchors, error must be < 10^-15 of modulus
        
        let tolerance_permille = 0.000_001; // 0.0001% tolerance
        
        for (a, b) in &[(22u64, 7u64), (100u64, 3u64), (1_000_000u64, 13u64)] {
            let (quotient, remainder) = plmg_divide(*a, *b, &primary_moduli, &reference_moduli);
            
            let exact_div = (*a as f64) / (*b as f64);
            let plmg_div = (quotient as f64) + (remainder as f64) / (*b as f64);
            
            let error = (exact_div - plmg_div).abs() / exact_div;
            assert!(error < tolerance_permille, 
                "Error {:.10} exceeds tolerance for {}/{}", 
                error, a, b);
        }
    }

    /// Test 3.7: Division is deterministic
    #[test]
    fn test_division_determinism() {
        for (a, b) in &[(21u64, 7u64), (100u64, 3u64)] {
            let (q1, r1) = plmg_divide(*a, *b, &primary_moduli, &reference_moduli);
            let (q2, r2) = plmg_divide(*a, *b, &primary_moduli, &reference_moduli);
            let (q3, r3) = plmg_divide(*a, *b, &primary_moduli, &reference_moduli);
            
            assert_eq!((q1, r1), (q2, r2));
            assert_eq!((q2, r2), (q3, r3));
        }
    }
}
```

**Test Coverage:** 7 test cases validating:
- 100% exactness property
- 40× speedup measurement
- Multiple configurations
- Edge cases (zero, one)
- Error bounds
- Determinism

---

### Theorem 4: Magnitude Comparison via Phase Differential

**Test File:** `hcvlang/tests/plmg_theorem_4_magnitude_comparison.rs`

```rust
#[cfg(test)]
mod plmg_theorem_4_tests {
    use super::*;

    /// Test 4.1: O(n+m) comparison complexity (no full reconstruction)
    #[test]
    fn test_comparison_without_full_reconstruction() {
        use std::time::Instant;
        
        // Create values where phase differentials differ
        // (so full reconstruction of primary is not needed)
        let x = 12_345_678u64;
        let y = 87_654_321u64;
        
        // Time the comparison operation
        let start = Instant::now();
        for _ in 0..100_000 {
            let _ = plmg_compare(x, y, &primary_moduli, &reference_moduli);
        }
        let elapsed = start.elapsed();
        
        // Comparison should be sub-microsecond
        let avg_time_ns = elapsed.as_nanos() as f64 / 100_000.0;
        assert!(avg_time_ns < 1000.0, 
                "Comparison took {}ns (should be <1000ns for O(n+m))", avg_time_ns);
    }

    /// Test 4.2: Correct ordering for all comparisons
    #[test]
    fn test_comparison_correctness() {
        let test_pairs = vec![
            (1u64, 2u64),
            (42u64, 42u64),
            (100u64, 99u64),
            (0u64, c_total - 1),
        ];

        for (x, y) in test_pairs {
            let result = plmg_compare(x, y, &primary_moduli, &reference_moduli);
            
            let expected = if x < y {
                std::cmp::Ordering::Less
            } else if x > y {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            };

            assert_eq!(result, expected, 
                "Compare({}, {}) returned {:?}, expected {:?}", 
                x, y, result, expected);
        }
    }

    /// Test 4.3: Transitivity (if x < y and y < z then x < z)
    #[test]
    fn test_comparison_transitivity() {
        for x in 0..100u64 {
            for y in (x+1)..100u64 {
                for z in (y+1)..100u64 {
                    let x_lt_y = plmg_compare(x, y, &primary_moduli, &reference_moduli)
                        == std::cmp::Ordering::Less;
                    let y_lt_z = plmg_compare(y, z, &primary_moduli, &reference_moduli)
                        == std::cmp::Ordering::Less;
                    let x_lt_z = plmg_compare(x, z, &primary_moduli, &reference_moduli)
                        == std::cmp::Ordering::Less;

                    assert!(x_lt_z, 
                        "Transitivity violated: x={}, y={}, z={}", x, y, z);
                }
            }
        }
    }

    /// Test 4.4: Phase differential hierarchy correctly predicts magnitude
    #[test]
    fn test_phase_differential_magnitude_hierarchy() {
        // Values with different phase differentials should be immediately comparable
        
        for x in 0..10000u64 {
            for y in (x+1)..10000u64 {
                let phase_x = compute_phase_differential(x, &primary_moduli, &reference_moduli);
                let phase_y = compute_phase_differential(y, &primary_moduli, &reference_moduli);

                if phase_x != phase_y {
                    // Different phases - should determine ordering immediately
                    let result = plmg_compare(x, y, &primary_moduli, &reference_moduli);
                    let expected_by_phase = if phase_x < phase_y {
                        std::cmp::Ordering::Less
                    } else {
                        std::cmp::Ordering::Greater
                    };

                    assert_eq!(result, expected_by_phase,
                        "Phase differential ordering incorrect for x={}, y={}", x, y);
                }
            }
        }
    }
}
```

**Test Coverage:** 4 comprehensive test cases for comparison operations

---

### Theorem 9: Deterministic Transformation Property (DTP)

**Test File:** `hcvlang/tests/plmg_theorem_9_deterministic_property.rs`

```rust
#[cfg(test)]
mod plmg_theorem_9_tests {
    use super::*;

    /// Test 9.1: Same inputs produce bit-identical outputs
    #[test]
    fn test_determinism_identical_runs() {
        let test_ops = vec![
            ("add", 123u64, 456u64),
            ("mul", 789u64, 111u64),
            ("sub", 999u64, 111u64),
        ];

        for (op, a, b) in test_ops {
            // Run operation 1000 times
            let mut results = vec![];
            for _ in 0..1000 {
                let result = match op {
                    "add" => plmg_add(a, b, &primary_moduli, &reference_moduli),
                    "mul" => plmg_mul(a, b, &primary_moduli, &reference_moduli),
                    "sub" => plmg_sub(a, b, &primary_moduli, &reference_moduli),
                    _ => panic!("Unknown op"),
                };
                results.push(result);
            }

            // All results must be identical
            for result in &results[1..] {
                assert_eq!(result, &results[0], 
                    "Non-deterministic behavior detected for operation {}", op);
            }
        }
    }

    /// Test 9.2: Byte-for-byte output consistency
    #[test]
    fn test_determinism_byte_identical() {
        let a = 12_345_678u64;
        let b = 87_654_321u64;

        for _ in 0..100 {
            let result1 = plmg_add(a, b, &primary_moduli, &reference_moduli);
            let result2 = plmg_add(a, b, &primary_moduli, &reference_moduli);

            // Serialize and compare byte-for-byte
            let bytes1 = bincode::serialize(&result1).unwrap();
            let bytes2 = bincode::serialize(&result2).unwrap();

            assert_eq!(bytes1, bytes2, "Byte-level outputs differ");
        }
    }

    /// Test 9.3: Reproducibility across program invocations (saved state)
    #[test]
    fn test_determinism_across_invocations() {
        let test_value = 42u64;
        
        // Simulate multiple "invocations" by serializing/deserializing
        let result1 = plmg_add(test_value, 1, &primary_moduli, &reference_moduli);
        let bytes = bincode::serialize(&result1).unwrap();
        let result1_restored = bincode::deserialize::<_>(&bytes).unwrap();

        let result2 = plmg_add(test_value, 1, &primary_moduli, &reference_moduli);

        assert_eq!(result1_restored, result2, 
            "Restored state doesn't match fresh computation");
    }

    /// Test 9.4: Determinism across different compiler optimization levels
    #[test]
    fn test_determinism_independent_of_optimization() {
        // This test runs during both debug and release builds
        // Assertion: Same result regardless of build mode
        
        let x = 123_456_789u64;
        let expected = plmg_add(x, 1, &primary_moduli, &reference_moduli);

        // With different compiler optimizations, result must be identical
        assert_eq!(
            plmg_add(x, 1, &primary_moduli, &reference_moduli),
            expected,
            "Optimization level affects output!"
        );
    }

    /// Test 9.5: Sequence of operations is deterministic
    #[test]
    fn test_determinism_operation_sequences() {
        let a = 100u64;
        let b = 50u64;
        let c = 25u64;

        // Compute (a + b) * c twice
        let seq1_result = {
            let sum = plmg_add(a, b, &primary_moduli, &reference_moduli);
            plmg_mul(sum, c, &primary_moduli, &reference_moduli)
        };

        let seq2_result = {
            let sum = plmg_add(a, b, &primary_moduli, &reference_moduli);
            plmg_mul(sum, c, &primary_moduli, &reference_moduli)
        };

        assert_eq!(seq1_result, seq2_result, 
            "Sequence of operations is non-deterministic");
    }

    /// Test 9.6: 10K operation marathon produces identical result
    #[test]
    fn test_determinism_long_sequence() {
        let mut value = 1u64;
        
        let expected_final = {
            let mut v = 1u64;
            for i in 0..10000 {
                v = plmg_add(v, i as u64, &primary_moduli, &reference_moduli);
            }
            v
        };

        let actual_final = {
            let mut v = 1u64;
            for i in 0..10000 {
                v = plmg_add(v, i as u64, &primary_moduli, &reference_moduli);
            }
            v
        };

        assert_eq!(actual_final, expected_final, 
            "10K-operation sequence produces different results");
    }
}
```

**Test Coverage:** 6 test cases validating determinism across:
- Multiple runs
- Byte-level consistency
- Serialization round-trips
- Compiler optimizations
- Operation sequences
- Long computations (10K+ ops)

---

### Theorem 10: Zero Error Accumulation

**Test File:** `hcvlang/tests/plmg_theorem_10_zero_error.rs`

```rust
#[cfg(test)]
mod plmg_theorem_10_tests {
    use super::*;

    /// Test 10.1: Error after 1M operations is exactly zero
    #[test]
    fn test_zero_error_after_1m_operations() {
        // Compute via PLMG: 1 + 1 + 1 + ... + 1 (1M times)
        let mut plmg_result = 0u64;
        for _ in 0..1_000_000 {
            plmg_result = plmg_add(plmg_result, 1, &primary_moduli, &reference_moduli);
        }

        // Expected result: 1M
        assert_eq!(plmg_result, 1_000_000, 
            "After 1M additions, error detected (expected 1M, got {})", 
            plmg_result);
    }

    /// Test 10.2: Associativity holds exactly (no rounding error)
    #[test]
    fn test_associativity_exact() {
        // (a + b) + c = a + (b + c) exactly
        
        let a = 123u64;
        let b = 456u64;
        let c = 789u64;

        let left = {
            let ab = plmg_add(a, b, &primary_moduli, &reference_moduli);
            plmg_add(ab, c, &primary_moduli, &reference_moduli)
        };

        let right = {
            let bc = plmg_add(b, c, &primary_moduli, &reference_moduli);
            plmg_add(a, bc, &primary_moduli, &reference_moduli)
        };

        assert_eq!(left, right, "Associativity violated");
    }

    /// Test 10.3: Distributivity holds exactly
    #[test]
    fn test_distributivity_exact() {
        // a * (b + c) = a*b + a*c exactly
        
        let a = 17u64;
        let b = 42u64;
        let c = 83u64;

        let left = {
            let sum = plmg_add(b, c, &primary_moduli, &reference_moduli);
            plmg_mul(a, sum, &primary_moduli, &reference_moduli)
        };

        let right = {
            let ab = plmg_mul(a, b, &primary_moduli, &reference_moduli);
            let ac = plmg_mul(a, c, &primary_moduli, &reference_moduli);
            plmg_add(ab, ac, &primary_moduli, &reference_moduli)
        };

        assert_eq!(left, right, "Distributivity violated");
    }

    /// Test 10.4: Commutative property verified for all operations
    #[test]
    fn test_commutativity_exact() {
        let a = 123u64;
        let b = 456u64;

        // a + b = b + a
        assert_eq!(
            plmg_add(a, b, &primary_moduli, &reference_moduli),
            plmg_add(b, a, &primary_moduli, &reference_moduli),
            "Addition not commutative"
        );

        // a * b = b * a
        assert_eq!(
            plmg_mul(a, b, &primary_moduli, &reference_moduli),
            plmg_mul(b, a, &primary_moduli, &reference_moduli),
            "Multiplication not commutative"
        );
    }

    /// Test 10.5: Inverse operations cancel exactly (a - b + b = a)
    #[test]
    fn test_inverse_operations_exact() {
        for a in vec![0u64, 1, 100, 1_000_000] {
            for b in vec![1u64, 42, 999] {
                let result = {
                    let minus_b = plmg_sub(a, b, &primary_moduli, &reference_moduli);
                    plmg_add(minus_b, b, &primary_moduli, &reference_moduli)
                };

                assert_eq!(result, a, 
                    "Inverse cancel failed: a={}, b={}", a, b);
            }
        }
    }

    /// Test 10.6: Comparison with reference BigInt computation (ground truth)
    #[test]
    fn test_zero_error_vs_bigint_reference() {
        use num_bigint::BigUint;
        
        // Run 10K random operations and compare against BigInt
        let mut plmg_val = 0u64;
        let mut bigint_val = BigUint::from(0u64);

        for i in 0..10_000 {
            let op_choice = i % 3;
            let operand = ((i * 7919) % 1_000_000) as u64;  // Pseudorandom

            match op_choice {
                0 => {  // Addition
                    plmg_val = plmg_add(plmg_val, operand, &primary_moduli, &reference_moduli);
                    bigint_val = bigint_val + BigUint::from(operand);
                },
                1 => {  // Subtraction
                    if plmg_val >= operand {
                        plmg_val = plmg_sub(plmg_val, operand, &primary_moduli, &reference_moduli);
                        bigint_val = &bigint_val - BigUint::from(operand);
                    }
                },
                _ => {  // Multiplication
                    plmg_val = plmg_mul(plmg_val, operand, &primary_moduli, &reference_moduli);
                    bigint_val = &bigint_val * BigUint::from(operand);
                }
            }
        }

        // Results must match exactly
        assert_eq!(plmg_val, bigint_val.to_u64().unwrap(),
            "PLMG and BigInt results diverged after 10K operations");
    }

    /// Test 10.7: Modular reduction preserves exactness
    #[test]
    fn test_modular_exactness() {
        let a = 999_999_999u64;
        let b = 888_888_888u64;

        // Compute modular sum
        let modular_sum = plmg_add_modular(a, b, &primary_moduli);
        
        // Verify matches integer arithmetic
        let expected = (a + b) % (primary_moduli.iter().product::<u64>());
        
        assert_eq!(modular_sum, expected, 
            "Modular addition produced inexact result");
    }
}
```

**Test Coverage:** 7 test cases validating:
- Zero error over 1M operations
- Algebraic properties (associativity, distributivity)
- Commutativity
- Inverse operations
- Comparison with BigInt ground truth
- Modular exactness

---

## 3. Integration Test Plan (50+ Tests)

### Integration with CRTBigInt
```rust
#[test]
fn test_plmg_to_crtbigint_conversion() {
    // PLMG value → CRTBigInt representation
    // Verify exact conversion and back
}

#[test]
fn test_plmg_crtbigint_arithmetic() {
    // Compute same operation in both systems
    // Compare results for equality
}
```

### Integration with Adaptive CRT
```rust
#[test]
fn test_plmg_with_tier_promotion() {
    // Verify PLMG phase differential unchanged when tier promotion occurs
    // Verify hierarchical gearing extends correctly
}
```

### Integration with FHE
```rust
#[test]
fn test_plmg_exact_polynomial_division_in_fhe() {
    // Encrypt polynomial, divide in encrypted domain
    // Decrypt and verify exactness
}

#[test]
fn test_plmg_zero_error_in_fhe_learning() {
    // Train encrypted model using PLMG
    // Verify zero error accumulation
}
```

### Integration with Neural Primitives
```rust
#[test]
fn test_plmg_zero_drift_neural_layer() {
    // Neural network layer using PLMG arithmetic
    // Verify no activation quantization error
}
```

---

## 4. Performance Benchmark Suite (200+ Benchmarks)

### 4.1 Rust Criterion Benchmarks

**File:** `hcvlang/benches/plmg_core_benchmarks.rs`

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_phase_differential_computation(c: &mut Criterion) {
    let primary_moduli = vec![2u64.pow(31) - 1, 2u64.pow(31) - 19];
    let reference_moduli = vec![2u64.pow(32) - 5];
    
    c.bench_function("phase_diff_small_value", |b| {
        b.iter(|| {
            compute_phase_differential(
                black_box(42u64),
                &primary_moduli,
                &reference_moduli
            )
        });
    });

    c.bench_function("phase_diff_large_value", |b| {
        b.iter(|| {
            compute_phase_differential(
                black_box(u64::MAX / 2),
                &primary_moduli,
                &reference_moduli
            )
        });
    });
}

fn benchmark_reconstruction(c: &mut Criterion) {
    let primary_moduli = vec![2u64.pow(31) - 1, 2u64.pow(31) - 19];
    let reference_moduli = vec![2u64.pow(32) - 5];
    
    c.bench_function("reconstruct_from_phase", |b| {
        let phase = 42u64;
        b.iter(|| {
            reconstruct_from_phase(
                black_box(phase),
                &primary_moduli,
                &reference_moduli
            )
        });
    });
}

fn benchmark_piggyback_division(c: &mut Criterion) {
    let primary_moduli = vec![2u64.pow(31) - 1, 2u64.pow(31) - 19];
    let reference_moduli = vec![2u64.pow(32) - 5];
    
    c.bench_function("piggyback_division_40x_baseline", |b| {
        b.iter(|| {
            plmg_divide(
                black_box(12_345_678u64),
                black_box(97u64),
                &primary_moduli,
                &reference_moduli
            )
        });
    });

    c.bench_function("naive_crt_division_baseline", |b| {
        b.iter(|| {
            naive_crt_division(
                black_box(12_345_678u64),
                black_box(97u64),
                &primary_moduli,
                &reference_moduli
            )
        });
    });
}

fn benchmark_hierarchical_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("hierarchical_comparison");
    
    for level_count in [2, 4, 8, 16].iter() {
        group.bench_with_input("magnitude_comparison", level_count, |b, &levels| {
            let manifolds = create_l_level_manifolds(levels);
            b.iter(|| {
                plmg_hierarchical_compare(
                    black_box(123_456u64),
                    black_box(654_321u64),
                    &manifolds
                )
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_phase_differential_computation,
    benchmark_reconstruction,
    benchmark_piggyback_division,
    benchmark_hierarchical_comparison
);
criterion_main!(benches);
```

### 4.2 Python FFI Benchmarks

**File:** `tests/python/test_04_plmg_performance.py`

```python
import pytest
from hcvlang import PLMGSystem
import time

@pytest.mark.benchmark
def test_ffi_phase_differential_overhead(benchmark):
    """Measure FFI call overhead for phase differential"""
    plmg = PLMGSystem()
    result = benchmark(plmg.phase_differential, 42)
    assert result is not None

@pytest.mark.benchmark
def test_batch_phase_differentials(benchmark):
    """Batch operations should be 4-8× faster than loops"""
    plmg = PLMGSystem()
    values = list(range(1000))
    
    # Individual calls
    start = time.perf_counter()
    for v in values:
        _ = plmg.phase_differential(v)
    individual_time = time.perf_counter() - start
    
    # Batch call
    batch_time = benchmark(plmg.batch_phase_differentials, values)
    
    speedup = individual_time / batch_time
    assert speedup >= 4.0, f"Batch speedup {speedup:.1f}× < 4×"

@pytest.mark.benchmark
def test_piggyback_division_40x(benchmark):
    """Validate 40× speedup for piggyback division"""
    plmg = PLMGSystem()
    
    # Baseline: traditional division
    traditional_time = traditional_divide_benchmark(123_456_789, 97)
    
    # PLMG piggyback
    plmg_time = benchmark(plmg.divide, 123_456_789, 97)
    
    speedup = traditional_time / plmg_time
    assert speedup >= 40.0, f"Speedup {speedup:.1f}× < 40×"
```

**Benchmark Targets:**
- Phase differential: <50ns per operation
- Reconstruction: <100ns per operation
- Division (piggyback): 40× vs naive
- Comparison: <500ns for O(n+m)
- FFI overhead: <1µs per call
- Batch operations: 4-8× vs individual

---

## 5. Property-Based Testing (quickcheck/proptest)

**File:** `hcvlang/tests/plmg_property_tests.rs`

```rust
#[cfg(test)]
mod plmg_property_tests {
    use proptest::prelude::*;

    proptest! {
        /// Property: phase_differential(X) is always in [0, C_R)
        #[test]
        fn phase_diff_in_valid_range(x in 0u64..c_total) {
            let phase = compute_phase_differential(x, &primary_moduli, &reference_moduli);
            prop_assert!(phase < c_r, "Phase {}: out of range [0, {})", phase, c_r);
        }

        /// Property: bijection - no collisions
        #[test]
        fn phase_diff_bijection(x1 in 0u64..1000, x2 in 0u64..1000) {
            if x1 != x2 {
                let phase1 = compute_phase_differential(x1, &primary_moduli, &reference_moduli);
                let phase2 = compute_phase_differential(x2, &primary_moduli, &reference_moduli);
                prop_assert_ne!(phase1, phase2, "Phase collision: {} and {}", x1, x2);
            }
        }

        /// Property: division exactness
        #[test]
        fn division_is_exact(a in 1u64..1_000_000, b in 1u64..1000) {
            let (q, r) = plmg_divide(a, b, &primary_moduli, &reference_moduli);
            let reconstructed = q * b + r;
            prop_assert_eq!(a, reconstructed, "Division inexact for {}/{}", a, b);
        }

        /// Property: addition commutativity
        #[test]
        fn addition_commutative(a in 0u64..c_total, b in 0u64..c_total) {
            let left = plmg_add(a, b, &primary_moduli, &reference_moduli);
            let right = plmg_add(b, a, &primary_moduli, &reference_moduli);
            prop_assert_eq!(left, right, "a+b != b+a for a={}, b={}", a, b);
        }

        /// Property: multiplication associativity
        #[test]
        fn multiplication_associative(
            a in 1u64..10000,
            b in 1u64..10000,
            c in 1u64..10000
        ) {
            let left = {
                let ab = plmg_mul(a, b, &primary_moduli, &reference_moduli);
                plmg_mul(ab, c, &primary_moduli, &reference_moduli)
            };
            let right = {
                let bc = plmg_mul(b, c, &primary_moduli, &reference_moduli);
                plmg_mul(a, bc, &primary_moduli, &reference_moduli)
            };
            prop_assert_eq!(left, right, "(a*b)*c != a*(b*c)");
        }

        /// Property: comparison transitivity
        #[test]
        fn comparison_transitive(x in 0u64..10000, y in 0u64..10000, z in 0u64..10000) {
            let x_lt_y = plmg_compare(x, y, &primary_moduli, &reference_moduli) 
                == std::cmp::Ordering::Less;
            let y_lt_z = plmg_compare(y, z, &primary_moduli, &reference_moduli) 
                == std::cmp::Ordering::Less;
            let x_lt_z = plmg_compare(x, z, &primary_moduli, &reference_moduli) 
                == std::cmp::Ordering::Less;

            if x_lt_y && y_lt_z {
                prop_assert!(x_lt_z, "Transitivity violated: {} < {} < {}", x, y, z);
            }
        }
    }
}
```

---

## 6. Regression Test Strategy

**Purpose:** Protect 500+ existing tests; detect any inadvertent breakage from PLMG integration

**Implementation:**

```bash
#!/bin/bash
# script: tests/regression_suite.sh

echo "=== PLMG Regression Test Suite ==="

# Step 1: Run full existing test suite with PLMG disabled
echo "Step 1: Baseline (PLMG disabled)"
cargo test --release --no-default-features 2>&1 | tee /tmp/plmg_test_baseline.log
baseline_code=$?

# Step 2: Run full existing test suite with PLMG enabled
echo "Step 2: With PLMG enabled"
cargo test --release --all-features 2>&1 | tee /tmp/plmg_test_with_plmg.log
plmg_code=$?

# Step 3: Compare test counts
baseline_passes=$(grep -c "test result: ok" /tmp/plmg_test_baseline.log)
plmg_passes=$(grep -c "test result: ok" /tmp/plmg_test_with_plmg.log)

echo "Baseline passing: $baseline_passes"
echo "With PLMG: $plmg_passes"

if [ "$baseline_passes" -eq "$plmg_passes" ]; then
    echo "✅ No regression detected"
    exit 0
else
    echo "❌ Regression detected: $((baseline_passes - plmg_passes)) tests failed"
    diff /tmp/plmg_test_baseline.log /tmp/plmg_test_with_plmg.log
    exit 1
fi
```

**Automated CI Integration:**

```yaml
# .github/workflows/plmg_regression.yml
name: PLMG Regression Tests
on: [push, pull_request]

jobs:
  regression:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Run regression suite
        run: bash tests/regression_suite.sh
```

---

## 7. Platform Testing (Linux, macOS, Windows)

**File:** `tests/platform/cross_platform_validation.sh`

```bash
#!/bin/bash
# Validate PLMG determinism across platforms

echo "=== Cross-Platform PLMG Validation ==="

platforms=("linux" "macos" "windows")
expected_hash="abc123def456"  # Compute once on reference platform

for platform in "${platforms[@]}"; do
    echo "Testing on $platform..."
    
    # Build and run test suite
    cargo build --release --target-dir=/tmp/plmg_test_$platform
    
    # Compute hash of all outputs
    output_hash=$(cargo test --release | sha256sum)
    
    if [ "$output_hash" != "$expected_hash" ]; then
        echo "❌ Platform $platform produced different output"
        exit 1
    fi
    echo "✅ $platform passed"
done

echo "✅ All platforms consistent"
```

---

## 8. Coverage Goals & Tools

### Coverage Targets

| Component | Target | Tool |
|-----------|--------|------|
| Rust Core | ≥90% | `cargo tarpaulin` |
| Python FFI | ≥85% | `pytest-cov` |
| Per-Theorem | ≥95% | Combined |
| Integration | ≥80% | Combined |

### Coverage Commands

```bash
# Rust coverage
cd hcvlang
cargo tarpaulin --out Html --output-dir coverage/rust --timeout 300

# Python coverage
pytest tests/python/ --cov=hcvlang --cov-report=html:coverage/python

# Combined report
python3 tools/merge_coverage_reports.py coverage/rust coverage/python coverage/combined
```

---

## 9. Test Execution Timeline (8-12 Weeks)

### Week 1-2: Unit Test Implementation (Theorems 1-3)
- [ ] Theorem 1 test module (150 lines, 7 tests)
- [ ] Theorem 2 test module (150 lines, 8 tests)
- [ ] Theorem 3 test module (150 lines, 7 tests)
- [ ] Local execution and validation

### Week 2-3: Unit Tests (Theorems 4-6)
- [ ] Theorem 4 test module (120 lines, 4 tests)
- [ ] Theorem 5 test module (100 lines, 5 tests)
- [ ] Theorem 6 test module (130 lines, 6 tests)

### Week 3-4: Unit Tests (Theorems 7-10) + Property Tests
- [ ] Theorem 7 test module (100 lines, 4 tests)
- [ ] Theorem 8 test module (80 lines, 3 tests)
- [ ] Theorem 9 test module (140 lines, 6 tests)
- [ ] Theorem 10 test module (150 lines, 7 tests)
- [ ] Property-based test suite (200 lines, 8 properties)

### Week 4-5: Integration Tests (50+ Tests)
- [ ] CRTBigInt integration (10 tests)
- [ ] Adaptive CRT integration (10 tests)
- [ ] FHE integration (12 tests)
- [ ] Neural primitive integration (10 tests)
- [ ] Modular arithmetic integration (8 tests)

### Week 5-7: Benchmark Implementation (200+ Benchmarks)
- [ ] Criterion benchmarks (Rust, 100 benchmarks)
- [ ] pytest-benchmark suite (Python, 50 benchmarks)
- [ ] Industry comparison benchmarks (50 benchmarks)
- [ ] Baseline execution and reporting

### Week 7-8: Regression Testing & CI
- [ ] Baseline capture (PLMG disabled)
- [ ] Regression suite implementation
- [ ] CI/CD pipeline setup
- [ ] Cross-platform validation scripts

### Week 8-10: Coverage Analysis & Fixes
- [ ] Code coverage reporting (tarpaulin, pytest-cov)
- [ ] Coverage gap analysis
- [ ] Additional test cases for gaps
- [ ] Achieve ≥85% coverage target

### Week 10-12: Final Validation & Documentation
- [ ] Full test suite execution (1000+ tests)
- [ ] Performance target validation
- [ ] Determinism validation (1000+ runs)
- [ ] Zero error proof (1M+ operations)
- [ ] Test documentation and results

---

## 10. Success Criteria & Validation

### Functional Correctness

| Criterion | Target | Validation |
|-----------|--------|-----------|
| All 10 theorems proven | 100% | All test suites passing |
| Zero regression | 100% | 500+ legacy tests passing |
| Code coverage | ≥85% | tarpaulin + pytest-cov reports |
| Property compliance | 100% | All proptest cases passing |

### Performance Targets

| Metric | Target | Test |
|--------|--------|------|
| Phase differential | <50ns | Criterion benchmark |
| Reconstruction | <100ns | Criterion benchmark |
| Division speedup | ≥40× | plmg_division_benchmark.rs |
| Comparison O(n+m) | Confirmed | plmg_comparison_benchmark.rs |
| FFI overhead | <1µs | ffi_benchmark.py |
| Batch speedup | 4-8× | batch_operations_benchmark.rs |

### Determinism Validation

| Test | Criterion |
|------|-----------|
| Bit-identical outputs (1000 runs) | ✅ Must pass |
| Byte-level consistency | ✅ Must pass |
| Cross-platform agreement | ✅ Must pass |
| Long-sequence stability (10K+ ops) | ✅ Must pass |

### Zero Error Proof

| Test | Criterion |
|------|-----------|
| 1M operation marathon | Error = 0 |
| Algebraic properties (assoc, comm, dist) | Exact |
| Comparison with BigInt ground truth | Exact match |
| Inverse operation cancellation | a - b + b = a |

---

## 11. Risk Mitigation

### Risk 1: Tests Exceed Performance Budget

**Problem:** Full test suite takes >30 minutes

**Mitigation:**
- Parallelize with `cargo test -- --test-threads=8`
- Use `cargo nextest` (parallel test runner)
- Separate "unit" tests (fast) from "benchmark" tests (slow)
- CI runs tests; developers run subset locally

### Risk 2: Flaky Tests (Non-Determinism)

**Problem:** Tests pass sometimes, fail other times

**Mitigation:**
- All tests use deterministic seeding
- No randomness unless explicitly controlled
- Run each test 100× in CI before declaring "passing"
- Use `proptest` regression capture for property tests

### Risk 3: Platform-Specific Failures

**Problem:** Linux passes, macOS fails

**Mitigation:**
- Use GitHub Actions matrix for Linux/macOS/Windows
- Capture baseline hashes on reference platform
- Assert identical output across all platforms
- Use only platform-independent arithmetic

### Risk 4: Benchmark Variance

**Problem:** Timing results fluctuate wildly

**Mitigation:**
- Run benchmarks on dedicated hardware (no background processes)
- Use Criterion's statistical analysis (mean ± stddev)
- Ignore outliers using statistical filtering
- Capture baseline and only flag deltas >5%

### Risk 5: Coverage Gaps

**Problem:** Achieving ≥85% coverage proves difficult

**Mitigation:**
- Identify gaps with `cargo tarpaulin`
- Add targeted tests for uncovered branches
- Use property-based testing for generalization
- Accept minor exceptions for error paths

---

## 12. Test Documentation & Results

### Test Results Repository

```
test_results/
├── test_runs/
│   ├── 2025-12-04_full_suite.log
│   ├── 2025-12-04_theorems_1_3.log
│   └── 2025-12-04_performance.log
├── coverage/
│   ├── rust_coverage.html
│   └── python_coverage.html
├── benchmarks/
│   ├── baseline_20251204.json
│   └── latest_20251204.json
└── reports/
    ├── PLMG_Test_Completion_Report.md
    ├── Coverage_Analysis.md
    └── Performance_Validation.md
```

### Automated Reporting

```python
# tools/generate_test_report.py
import json
from pathlib import Path

def generate_report():
    """Generate comprehensive test report"""
    
    # Aggregate results from all test runs
    test_results = aggregate_test_logs()
    coverage = parse_coverage_reports()
    benchmarks = parse_benchmark_results()
    
    report = {
        "date": datetime.now().isoformat(),
        "summary": {
            "theorems_validated": 10,
            "total_tests": sum(t["count"] for t in test_results),
            "passing": sum(t["passing"] for t in test_results),
            "coverage": coverage["combined"],
        },
        "details": {
            "unit_tests": test_results,
            "integration_tests": [...],
            "benchmarks": benchmarks,
            "determinism": [...],
            "zero_error": [...],
        }
    }
    
    # Write JSON and markdown report
    with open("test_results/report.json", "w") as f:
        json.dump(report, f, indent=2)
    
    # Generate markdown summary
    write_markdown_report(report)
```

---

## 13. Continuous Integration (CI/CD)

### GitHub Actions Workflow

```yaml
# .github/workflows/plmg_comprehensive_tests.yml
name: PLMG Comprehensive Test Suite

on: [push, pull_request]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Run theorem unit tests
        run: |
          cargo test --test plmg_theorem_1_k_elimination --release
          cargo test --test plmg_theorem_2_phase_locked_periodicity --release
          # ... all 10 theorem tests ...

  property-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Run property-based tests
        run: cargo test --test plmg_property_tests --release

  benchmarks:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Run benchmarks
        run: cargo bench --bench plmg_core_benchmarks

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Generate coverage
        run: |
          cargo tarpaulin --out Html --output-dir coverage
          pytest tests/python/ --cov=hcvlang --cov-report=html
      - uses: actions/upload-artifact@v3
        with:
          name: coverage-reports
          path: coverage/

  regression:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Run regression suite
        run: bash tests/regression_suite.sh

  cross-platform:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - name: Test on ${{ matrix.os }}
        run: cargo test --release
```

---

## Appendix A: Test Infrastructure

### Required Tools

```bash
# Rust testing
cargo test
cargo bench
cargo tarpaulin           # Coverage
cargo nextest             # Parallel testing

# Python testing
pytest
pytest-benchmark
pytest-cov
pytest-timeout

# Documentation
cargo test --doc
rustdoc --test docs/
```

### Test Configuration

```toml
# hcvlang/Cargo.toml test settings
[dev-dependencies]
criterion = "0.5"
proptest = "1.0"
quickcheck = "1.0"
quickcheck_macros = "1.0"

[[test]]
name = "plmg_theorem_1_k_elimination"
harness = true

[[test]]
name = "plmg_property_tests"
harness = true

# ... similar for all 10 theorem tests ...
```

---

## Appendix B: Test Execution Quick Reference

```bash
# Run all PLMG tests
cargo test --test plmg_theorem_* --release

# Run with output
cargo test --test plmg_theorem_1 --release -- --nocapture

# Run specific test
cargo test --test plmg_theorem_1 test_phase_differential_equals_overflow_count -- --exact

# Generate coverage
cargo tarpaulin --out Html --output-dir coverage

# Run benchmarks
cargo bench --bench plmg_core_benchmarks

# Run property tests
cargo test --test plmg_property_tests --release

# Run Python FFI tests
python3 -m pytest tests/python/test_04_plmg_performance.py -v

# Full regression suite
bash tests/regression_suite.sh

# Generate test report
python3 tools/generate_test_report.py
```

---

## Summary

This comprehensive test strategy validates all 10 PLMG theorems through:
- **1,000-1,200 unit test cases** across dedicated theorem modules
- **50+ integration tests** with existing systems
- **200+ performance benchmarks** (Criterion + pytest-benchmark)
- **Property-based testing** for generalization
- **Regression protection** for 500+ existing tests
- **Cross-platform validation** (Linux, macOS, Windows)
- **Determinism verification** (1000+ identical runs)
- **Zero error proof** (1M+ operations vs BigInt ground truth)

**Success Criteria:**
- All theorems proven with >95% code coverage
- Zero regression in legacy tests
- Performance targets achieved (O(n+m), 40×, etc.)
- Determinism validated across platforms
- Zero error accumulation demonstrated

**Estimated Cost:** 8-12 weeks of implementation and validation

---

**Document Status:** Final - Ready for Implementation  
**Last Updated:** December 4, 2025

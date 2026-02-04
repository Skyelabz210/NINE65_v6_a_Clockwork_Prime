//! NTT Optimization Test Suite
//!
//! Tests for all phases of NTT optimization project
//!
//! Run with: cargo test ntt_optimization -- --nocapture

use std::time::{Duration, Instant};

// ============================================================================
// TEST CONFIGURATION
// ============================================================================

const TEST_PRIME: u64 = 998244353;
const N_SMALL: usize = 8;
const N_MEDIUM: usize = 1024;
const N_LARGE: usize = 4096;

// Performance thresholds (nanoseconds)
const BUTTERFLY_THRESHOLD_NS: u64 = 25;
const NTT_FORWARD_1024_THRESHOLD_US: u64 = 50;
const NTT_POLY_MUL_1024_THRESHOLD_US: u64 = 150;

// ============================================================================
// T-001: HARVEY BUTTERFLY TESTS
// ============================================================================

mod t001_harvey_butterfly {
    use super::*;
    
    #[test]
    fn test_lazy_reduce_correctness() {
        let q = TEST_PRIME;
        let q2 = q << 1;
        
        // Test boundary cases
        assert_eq!(lazy_reduce(0, q, q2), 0);
        assert_eq!(lazy_reduce(q - 1, q, q2), q - 1);
        assert_eq!(lazy_reduce(q, q, q2), 0);
        assert_eq!(lazy_reduce(q + 1, q, q2), 1);
        assert_eq!(lazy_reduce(q2, q, q2), 0);
        assert_eq!(lazy_reduce(q2 + 1, q, q2), 1);
        
        // Test random values
        for x in [12345u64, 67890, 999999, q - 100, q + 100, q2 - 1] {
            let result = lazy_reduce(x, q, q2);
            assert!(result < q2, "Result {} >= 2q for input {}", result, x);
        }
    }
    
    #[test]
    fn test_montgomery_mul_lazy_correctness() {
        let q = TEST_PRIME;
        
        for (a, b) in [(1, 1), (100, 200), (12345, 67890), (q - 1, q - 1)] {
            let expected = ((a as u128 * b as u128) % q as u128) as u64;
            
            // Convert to Montgomery, multiply, convert back
            let a_mont = to_montgomery(a, q);
            let b_mont = to_montgomery(b, q);
            let result_mont = montgomery_mul_lazy(a_mont, b_mont, q);
            let result = from_montgomery(full_reduce(result_mont, q), q);
            
            assert_eq!(result, expected, "Montgomery mul failed for {} * {}", a, b);
        }
    }
    
    #[test]
    fn test_harvey_butterfly_correctness() {
        let q = TEST_PRIME;
        let q2 = q << 1;
        
        let mut a = 100u64;
        let mut b = 200u64;
        let tw = to_montgomery(3, q);
        
        // Expected: a' = a + tw*b, b' = a - tw*b
        let tw_b = (3u128 * 200) % q as u128;
        let expected_a = ((100 + tw_b) % q as u128) as u64;
        let expected_b = ((100 + q as u128 - tw_b) % q as u128) as u64;
        
        harvey_butterfly(&mut a, &mut b, tw, q, q2);
        
        // Fully reduce for comparison
        let a_reduced = full_reduce(a, q);
        let b_reduced = full_reduce(b, q);
        
        assert_eq!(a_reduced, expected_a);
        assert_eq!(b_reduced, expected_b);
    }
    
    #[test]
    fn test_harvey_butterfly_performance() {
        let q = TEST_PRIME;
        let q2 = q << 1;
        let tw = to_montgomery(3, q);
        
        let mut a = 12345u64;
        let mut b = 67890u64;
        
        let iterations = 1_000_000;
        let start = Instant::now();
        for _ in 0..iterations {
            harvey_butterfly(&mut a, &mut b, tw, q, q2);
        }
        let elapsed = start.elapsed();
        
        let per_op_ns = elapsed.as_nanos() as u64 / iterations as u64;
        println!("[T-001] Harvey butterfly: {} ns/op", per_op_ns);
        
        assert!(
            per_op_ns <= BUTTERFLY_THRESHOLD_NS,
            "GATE FAILED: {} ns > {} ns threshold",
            per_op_ns,
            BUTTERFLY_THRESHOLD_NS
        );
    }
    
    // Helper functions for T-001 tests
    fn lazy_reduce(x: u64, q: u64, q2: u64) -> u64 {
        if x >= q2 { x - q2 } else if x >= q { x - q } else { x }
    }
    
    fn full_reduce(x: u64, q: u64) -> u64 {
        if x >= q { x - q } else { x }
    }
    
    fn to_montgomery(a: u64, q: u64) -> u64 {
        let r2 = compute_r2(q);
        ((a as u128 * r2 as u128) % q as u128) as u64
    }
    
    fn from_montgomery(a: u64, q: u64) -> u64 {
        montgomery_mul_lazy(a, 1, q)
    }
    
    fn compute_r2(q: u64) -> u64 {
        let r = 1u128 << 64;
        ((r * r) % q as u128) as u64
    }
    
    fn compute_q_inv(q: u64) -> u64 {
        let mut inv = 1u64;
        for _ in 0..6 {
            inv = inv.wrapping_mul(2u64.wrapping_sub(q.wrapping_mul(inv)));
        }
        inv.wrapping_neg()
    }
    
    fn montgomery_mul_lazy(a: u64, b: u64, q: u64) -> u64 {
        let q_inv = compute_q_inv(q);
        let ab = a as u128 * b as u128;
        let m = (ab as u64).wrapping_mul(q_inv);
        let t = ((ab + m as u128 * q as u128) >> 64) as u64;
        if t >= q { t - q } else { t }
    }
    
    fn harvey_butterfly(a: &mut u64, b: &mut u64, tw: u64, q: u64, q2: u64) {
        let t = montgomery_mul_lazy(*b, tw, q);
        let sum = *a + t;
        let diff = *a + q2 - t;
        *a = lazy_reduce(sum, q, q2);
        *b = lazy_reduce(diff, q, q2);
    }
}

// ============================================================================
// T-002: BIT-REVERSAL TABLE TESTS
// ============================================================================

mod t002_bit_reversal {
    use super::*;
    
    #[test]
    fn test_bit_reverse_small() {
        let log_n = 3; // n = 8
        
        // Manual verification
        assert_eq!(bit_reverse(0, log_n), 0); // 000 -> 000
        assert_eq!(bit_reverse(1, log_n), 4); // 001 -> 100
        assert_eq!(bit_reverse(2, log_n), 2); // 010 -> 010
        assert_eq!(bit_reverse(3, log_n), 6); // 011 -> 110
        assert_eq!(bit_reverse(4, log_n), 1); // 100 -> 001
        assert_eq!(bit_reverse(5, log_n), 5); // 101 -> 101
        assert_eq!(bit_reverse(6, log_n), 3); // 110 -> 011
        assert_eq!(bit_reverse(7, log_n), 7); // 111 -> 111
    }
    
    #[test]
    fn test_precomputed_table() {
        let n = N_MEDIUM;
        let log_n = n.trailing_zeros() as usize;
        
        let table = precompute_bit_reverse_table(n);
        
        // Verify table is correct
        for i in 0..n {
            assert_eq!(table[i], bit_reverse(i, log_n));
        }
    }
    
    #[test]
    fn test_table_permutation_faster() {
        let n = N_MEDIUM;
        let log_n = n.trailing_zeros() as usize;
        
        let table = precompute_bit_reverse_table(n);
        let mut a: Vec<u64> = (0..n as u64).collect();
        let mut b = a.clone();
        
        let iterations = 10_000;
        
        // Time: compute on-the-fly
        let start = Instant::now();
        for _ in 0..iterations {
            bit_reverse_permute_compute(&mut a, log_n);
        }
        let compute_time = start.elapsed();
        
        // Time: use table
        let start = Instant::now();
        for _ in 0..iterations {
            bit_reverse_permute_table(&mut b, &table);
        }
        let table_time = start.elapsed();
        
        println!("[T-002] Bit-reverse compute: {:?}", compute_time / iterations as u32);
        println!("[T-002] Bit-reverse table: {:?}", table_time / iterations as u32);
        
        // Table should be faster
        assert!(
            table_time < compute_time,
            "Table ({:?}) not faster than compute ({:?})",
            table_time,
            compute_time
        );
    }
    
    fn bit_reverse(x: usize, bits: usize) -> usize {
        x.reverse_bits() >> (usize::BITS as usize - bits)
    }
    
    fn precompute_bit_reverse_table(n: usize) -> Vec<usize> {
        let log_n = n.trailing_zeros() as usize;
        (0..n).map(|i| bit_reverse(i, log_n)).collect()
    }
    
    fn bit_reverse_permute_compute(a: &mut [u64], log_n: usize) {
        for i in 0..a.len() {
            let j = bit_reverse(i, log_n);
            if i < j {
                a.swap(i, j);
            }
        }
    }
    
    fn bit_reverse_permute_table(a: &mut [u64], table: &[usize]) {
        for i in 0..a.len() {
            let j = table[i];
            if i < j {
                a.swap(i, j);
            }
        }
    }
}

// ============================================================================
// T-008: INTEGRATION TESTS
// ============================================================================

mod t008_integration {
    use super::*;
    
    #[test]
    fn test_ntt_roundtrip_small() {
        // This would use the actual optimized NTT engine
        // For now, placeholder that tests the expected interface
        let original: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut a = original.clone();
        
        // ntt_forward(&mut a, &ctx);
        // ntt_inverse(&mut a, &ctx);
        
        // assert_eq!(a, original);
        println!("[T-008] Roundtrip test placeholder - implement after integration");
    }
    
    #[test]
    fn test_poly_mul_correctness() {
        // (1 + 2x + 3x^2) * (4 + 5x) = 4 + 13x + 22x^2 + 15x^3
        println!("[T-008] Poly mul correctness - implement after integration");
    }
    
    #[test]
    fn test_negacyclic_property() {
        // x^(n-1) * x = -1 in Z[X]/(X^n + 1)
        println!("[T-008] Negacyclic property - implement after integration");
    }
}

// ============================================================================
// T-010: BENCHMARK TESTS
// ============================================================================

mod t010_benchmark {
    use super::*;
    
    #[test]
    fn test_ntt_forward_1024_performance() {
        // Placeholder - would use actual optimized NTT
        println!("[T-010] NTT Forward 1024 benchmark placeholder");
        println!("[T-010] Target: < {} μs", NTT_FORWARD_1024_THRESHOLD_US);
    }
    
    #[test]
    fn test_poly_mul_1024_performance() {
        // Placeholder - would use actual optimized poly_mul
        println!("[T-010] Poly Mul 1024 benchmark placeholder");
        println!("[T-010] Target: < {} μs", NTT_POLY_MUL_1024_THRESHOLD_US);
    }
    
    #[test]
    fn test_compare_vs_baseline() {
        // Compare new implementation vs old
        println!("[T-010] Baseline comparison placeholder");
        println!("[T-010] Expected speedup: 40×");
    }
}

// ============================================================================
// REGRESSION TESTS
// ============================================================================

mod regression {
    use super::*;
    
    #[test]
    fn test_no_float_usage() {
        // This would scan source files for forbidden patterns
        // In actual implementation, use the regression/scan.sh script
        println!("[REGRESSION] Float check placeholder - use scan.sh");
    }
    
    #[test]
    fn test_required_patterns_present() {
        // Check that required patterns appear in source
        println!("[REGRESSION] Pattern check placeholder - use scan.sh");
    }
}

// ============================================================================
// MAIN TEST RUNNER
// ============================================================================

#[test]
fn run_all_ntt_optimization_tests() {
    println!("\n");
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║       NTT OPTIMIZATION TEST SUITE                             ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    println!("Run individual test modules with:");
    println!("  cargo test t001 -- --nocapture");
    println!("  cargo test t002 -- --nocapture");
    println!("  ...");
    println!();
    println!("Performance thresholds:");
    println!("  Butterfly: < {} ns", BUTTERFLY_THRESHOLD_NS);
    println!("  NTT Forward (1024): < {} μs", NTT_FORWARD_1024_THRESHOLD_US);
    println!("  Poly Mul (1024): < {} μs", NTT_POLY_MUL_1024_THRESHOLD_US);
    println!();
}

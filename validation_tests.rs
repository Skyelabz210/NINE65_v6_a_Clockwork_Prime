//! QMNF Validation Test Suite
//! 
//! Machine-verifiable validation identities for all QMNF innovations.
//! Each test corresponds to a formal theorem in the technical paper.
//! 
//! HackFate Research - January 2026

#![allow(dead_code)]

// ============================================================================
// Validation Identity Functions (V1-V8)
// ============================================================================

/// V1: Division Algorithm Identity
/// X = q × d + r where 0 ≤ r < d
#[inline]
pub fn validate_v1(x: u128, d: u64, q: u128, r: u64) -> bool {
    d > 0 && x == q * d as u128 + r as u128 && r < d
}

/// V2: Residue Bound
/// All residues satisfy 0 ≤ rᵢ < mᵢ
#[inline]
pub fn validate_v2(residues: &[u64], moduli: &[u64]) -> bool {
    residues.len() == moduli.len() 
        && residues.iter().zip(moduli).all(|(&r, &m)| m > 0 && r < m)
}

/// V3: Key Congruence (K-Elimination Core)
/// k ≡ (v_A - v_M) × M⁻¹ (mod A)
#[inline]
pub fn validate_v3(
    k: u128, 
    v_anchor: u128, 
    v_main: u128, 
    _main_cap: u128,  // M - not directly used in validation
    anchor_cap: u128, 
    m_inv_a: u128
) -> bool {
    let diff = if v_anchor >= (v_main % anchor_cap) {
        v_anchor - (v_main % anchor_cap)
    } else {
        anchor_cap - ((v_main % anchor_cap) - v_anchor)
    };
    let expected_k = (diff * m_inv_a) % anchor_cap;
    k == expected_k
}

/// V4: K Range Bound
/// 0 ≤ k < A
#[inline]
pub fn validate_v4(k: u128, anchor_cap: u128) -> bool {
    k < anchor_cap
}

/// V5: K Uniqueness (implicit - if two k values pass V3 and V4, they're equal)
#[inline]
pub fn validate_v5(k1: u128, k2: u128, anchor_cap: u128) -> bool {
    // Both in range and congruent implies equal
    k1 < anchor_cap && k2 < anchor_cap && k1 % anchor_cap == k2 % anchor_cap
}

/// V6: Reconstruction Identity
/// X = v_M + k × M
#[inline]
pub fn validate_v6(x: u128, v_main: u128, k: u128, main_cap: u128) -> bool {
    x == v_main + k * main_cap
}

/// V7: Montgomery Correctness
/// Mont(a, b) ≡ a × b × R⁻¹ (mod m)
#[inline]
pub fn validate_v7(mont_result: u64, a: u64, b: u64, m: u64, r_inv: u64) -> bool {
    if m == 0 { return false; }
    let expected = ((a as u128 * b as u128 % m as u128) * r_inv as u128 % m as u128) as u64;
    mont_result == expected
}

/// V8: GSO Noise Bound
/// N_k ≤ α × Q_k where α < 0.5
#[inline]
pub fn validate_v8(noise: u64, modulus: u64, alpha_numerator: u64, alpha_denominator: u64) -> bool {
    if alpha_denominator == 0 { return false; }
    // Check: noise * denominator ≤ numerator * modulus
    // And: numerator * 2 < denominator (ensures α < 0.5)
    let lhs = noise as u128 * alpha_denominator as u128;
    let rhs = alpha_numerator as u128 * modulus as u128;
    lhs <= rhs && alpha_numerator * 2 < alpha_denominator
}


// ============================================================================
// Runtime Assertion Macros
// ============================================================================

/// Assert V1: Division algorithm
#[macro_export]
macro_rules! assert_v1 {
    ($x:expr, $d:expr, $q:expr, $r:expr) => {
        debug_assert_eq!($x, $q * $d as u128 + $r as u128, "V1 violated: X ≠ q·d + r");
        debug_assert!($r < $d, "V1 remainder bound violated: r ≥ d");
    };
}

/// Assert V3: K-Elimination congruence
#[macro_export]
macro_rules! assert_v3 {
    ($k:expr, $v_a:expr, $v_m:expr, $m_cap:expr, $a_cap:expr, $m_inv:expr) => {{
        let diff = if $v_a >= ($v_m % $a_cap) {
            $v_a - ($v_m % $a_cap)
        } else {
            $a_cap - (($v_m % $a_cap) - $v_a)
        };
        let expected = (diff * $m_inv) % $a_cap;
        debug_assert_eq!($k, expected, "V3 violated: k={}, expected={}", $k, expected);
    }};
}

/// Assert V6: Reconstruction
#[macro_export]
macro_rules! assert_v6 {
    ($x:expr, $v_m:expr, $k:expr, $m_cap:expr) => {
        debug_assert_eq!($x, $v_m + $k * $m_cap, "V6 violated: X ≠ v_M + k·M");
    };
}


// ============================================================================
// Core Algorithm Implementations for Testing
// ============================================================================

/// Binary GCD (Stein's Algorithm) - Reference Implementation
pub fn binary_gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    
    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    
    loop {
        b >>= b.trailing_zeros();
        if a > b { std::mem::swap(&mut a, &mut b); }
        b -= a;
        if b == 0 { break; }
    }
    
    a << shift
}

/// Extended Euclidean Algorithm
pub fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (g, x1, y1) = extended_gcd(b % a, a);
    let x = y1 - (b / a) * x1;
    let y = x1;
    (g, x, y)
}

/// Modular inverse using extended GCD
pub fn mod_inverse(a: u64, m: u64) -> Option<u64> {
    let (g, x, _) = extended_gcd(a as i128, m as i128);
    if g != 1 {
        None
    } else {
        Some(((x % m as i128 + m as i128) % m as i128) as u64)
    }
}

/// Montgomery context
pub struct Montgomery {
    pub modulus: u64,
    pub r_mod: u64,
    pub r2_mod: u64,
    pub m_prime: u64,
}

impl Montgomery {
    pub fn new(modulus: u64) -> Option<Self> {
        if modulus < 2 || modulus % 2 == 0 {
            return None;
        }
        
        let r: u128 = 1u128 << 64;
        let r_mod = (r % modulus as u128) as u64;
        let r2_mod = ((r % modulus as u128) * (r % modulus as u128) % modulus as u128) as u64;
        
        // Hensel lifting
        let mut x: u64 = 1;
        for _ in 0..6 {
            x = x.wrapping_mul(2u64.wrapping_sub(modulus.wrapping_mul(x)));
        }
        let m_prime = x.wrapping_neg();
        
        Some(Self { modulus, r_mod, r2_mod, m_prime })
    }
    
    #[inline]
    pub fn mont_mul(&self, a: u64, b: u64) -> u64 {
        let t: u128 = a as u128 * b as u128;
        let m: u64 = (t as u64).wrapping_mul(self.m_prime);
        let t_high: u128 = t + (m as u128 * self.modulus as u128);
        let u: u64 = (t_high >> 64) as u64;
        if u >= self.modulus { u - self.modulus } else { u }
    }
    
    #[inline]
    pub fn to_mont(&self, a: u64) -> u64 {
        self.mont_mul(a, self.r2_mod)
    }
    
    #[inline]
    pub fn from_mont(&self, a: u64) -> u64 {
        self.mont_mul(a, 1)
    }
}

/// K-Elimination Configuration
pub struct KElimConfig {
    pub main_moduli: Vec<u64>,
    pub anchor_moduli: Vec<u64>,
    pub main_cap: u128,
    pub anchor_cap: u128,
    pub m_inv_a: u128,
}

impl KElimConfig {
    pub fn new(main: &[u64], anchor: &[u64]) -> Option<Self> {
        let main_cap: u128 = main.iter().map(|&m| m as u128).product();
        let anchor_cap: u128 = anchor.iter().map(|&a| a as u128).product();
        
        // Verify coprimality
        if binary_gcd(main_cap as u64, anchor_cap as u64) != 1 {
            return None;
        }
        
        // Compute M⁻¹ mod A
        let m_inv_a = mod_inverse(main_cap as u64, anchor_cap as u64)? as u128;
        
        Some(Self {
            main_moduli: main.to_vec(),
            anchor_moduli: anchor.to_vec(),
            main_cap,
            anchor_cap,
            m_inv_a,
        })
    }
    
    /// Full K-Elimination: recover X from residues
    pub fn k_eliminate(&self, v_main: u128, v_anchor: u128) -> u128 {
        let diff = if v_anchor >= (v_main % self.anchor_cap) {
            v_anchor - (v_main % self.anchor_cap)
        } else {
            self.anchor_cap - ((v_main % self.anchor_cap) - v_anchor)
        };
        
        let k = (diff * self.m_inv_a) % self.anchor_cap;
        v_main + k * self.main_cap
    }
}


// ============================================================================
// Test Suite
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ===== V1: Division Algorithm Tests =====
    
    #[test]
    fn test_v1_basic() {
        assert!(validate_v1(100, 7, 14, 2));   // 100 = 14×7 + 2
        assert!(validate_v1(0, 5, 0, 0));      // 0 = 0×5 + 0
        assert!(validate_v1(17, 17, 1, 0));    // 17 = 1×17 + 0
        assert!(validate_v1(123456789, 1000, 123456, 789));
    }
    
    #[test]
    fn test_v1_edge_cases() {
        // Large values
        assert!(validate_v1(u128::MAX / 2, u64::MAX, (u128::MAX / 2) / u64::MAX as u128, 
                           ((u128::MAX / 2) % u64::MAX as u128) as u64));
    }

    // ===== V2: Residue Bound Tests =====
    
    #[test]
    fn test_v2_basic() {
        let residues = vec![3, 5, 2];
        let moduli = vec![7, 11, 13];
        assert!(validate_v2(&residues, &moduli));
    }
    
    #[test]
    fn test_v2_violations() {
        let residues = vec![7, 5, 2];  // 7 >= 7 violates
        let moduli = vec![7, 11, 13];
        assert!(!validate_v2(&residues, &moduli));
    }

    // ===== V3-V6: K-Elimination Chain Tests =====
    
    #[test]
    fn test_k_elimination_small() {
        // Small example: main = [7, 11], anchor = [13]
        let config = KElimConfig::new(&[7, 11], &[13]).unwrap();
        
        // Test value X = 100
        let x: u128 = 100;
        let v_main = x % config.main_cap;
        let v_anchor = x % config.anchor_cap;
        let k = x / config.main_cap;
        
        // V4: k in range
        assert!(validate_v4(k, config.anchor_cap), "V4 failed");
        
        // V6: reconstruction
        assert!(validate_v6(x, v_main, k, config.main_cap), "V6 failed");
        
        // Full K-elimination
        let recovered = config.k_eliminate(v_main, v_anchor);
        assert_eq!(recovered, x, "K-elimination failed");
    }
    
    #[test]
    fn test_k_elimination_large() {
        // Production-scale: 96-bit capacity
        let config = KElimConfig::new(
            &[4294967291, 4294967279, 4294967231],  // ~96 bits
            &[2147483647, 2147483629]               // ~62 bits
        ).unwrap();
        
        // Test various values
        for x in [0u128, 1, 1000, 1_000_000, 1_000_000_000_000u128] {
            let v_main = x % config.main_cap;
            let v_anchor = x % config.anchor_cap;
            
            let recovered = config.k_eliminate(v_main, v_anchor);
            assert_eq!(recovered, x, "K-elimination failed for x={}", x);
        }
    }
    
    #[test]
    fn test_k_elimination_stress() {
        let config = KElimConfig::new(&[65521, 65519, 65497], &[65493, 65479]).unwrap();
        
        // Test 1000 random-ish values
        let mut x: u128 = 12345;
        for _ in 0..1000 {
            x = (x * 1103515245 + 12345) % (config.main_cap * config.anchor_cap);
            
            let v_main = x % config.main_cap;
            let v_anchor = x % config.anchor_cap;
            let recovered = config.k_eliminate(v_main, v_anchor);
            
            assert_eq!(recovered, x, "Stress test failed for x={}", x);
        }
    }

    // ===== V7: Montgomery Tests =====
    
    #[test]
    fn test_v7_montgomery_basic() {
        let ctx = Montgomery::new(17).unwrap();
        
        // Test round-trip
        for a in 0..17u64 {
            let mont_a = ctx.to_mont(a);
            let recovered = ctx.from_mont(mont_a);
            assert_eq!(recovered, a, "Montgomery round-trip failed for a={}", a);
        }
    }
    
    #[test]
    fn test_v7_montgomery_multiplication() {
        let ctx = Montgomery::new(4294967291).unwrap();  // Large prime
        
        // Verify: mont_mul in Montgomery domain = regular mul
        for &a in &[0u64, 1, 100, 1000000, 4294967290] {
            for &b in &[0u64, 1, 100, 1000000, 4294967290] {
                let expected = ((a as u128 * b as u128) % 4294967291) as u64;
                
                let mont_a = ctx.to_mont(a);
                let mont_b = ctx.to_mont(b);
                let mont_result = ctx.mont_mul(mont_a, mont_b);
                let result = ctx.from_mont(mont_result);
                
                assert_eq!(result, expected, 
                    "Montgomery mul failed: {} × {} expected {}, got {}", a, b, expected, result);
            }
        }
    }
    
    #[test]
    fn test_v7_persistent_montgomery() {
        let ctx = Montgomery::new(65521).unwrap();
        
        // Chain of multiplications staying in Montgomery form
        let a = 12345u64 % 65521;
        let b = 54321u64 % 65521;
        let c = 11111u64 % 65521;
        
        // Expected: a × b × c mod p
        let expected = ((a as u128 * b as u128 % 65521) * c as u128 % 65521) as u64;
        
        // Persistent Montgomery: convert once at start, once at end
        let mont_a = ctx.to_mont(a);
        let mont_b = ctx.to_mont(b);
        let mont_c = ctx.to_mont(c);
        
        let mont_ab = ctx.mont_mul(mont_a, mont_b);  // Still in Montgomery form
        let mont_abc = ctx.mont_mul(mont_ab, mont_c); // Still in Montgomery form
        let result = ctx.from_mont(mont_abc);  // Convert only at end
        
        assert_eq!(result, expected, "Persistent Montgomery chain failed");
    }

    // ===== V8: GSO Noise Bound Tests =====
    
    #[test]
    fn test_v8_noise_bound() {
        // α = 0.4 represented as 2/5
        assert!(validate_v8(400, 1000, 2, 5));  // 400 ≤ 0.4 × 1000 ✓
        assert!(!validate_v8(500, 1000, 2, 5)); // 500 > 0.4 × 1000 ✗
        
        // α = 0.49 represented as 49/100
        assert!(validate_v8(490, 1000, 49, 100));  // 490 ≤ 0.49 × 1000 ✓
        assert!(!validate_v8(500, 1000, 49, 100)); // 500 > 0.49 × 1000 ✗
    }

    // ===== Binary GCD Tests =====
    
    #[test]
    fn test_binary_gcd_correctness() {
        // Known values
        assert_eq!(binary_gcd(48, 18), 6);
        assert_eq!(binary_gcd(100, 35), 5);
        assert_eq!(binary_gcd(17, 23), 1);  // Coprime
        assert_eq!(binary_gcd(0, 5), 5);
        assert_eq!(binary_gcd(5, 0), 5);
        assert_eq!(binary_gcd(0, 0), 0);
    }
    
    #[test]
    fn test_binary_gcd_large() {
        // Large values
        let a = 4294967291u64;  // Large prime
        let b = 4294967279u64;  // Another large prime
        assert_eq!(binary_gcd(a, b), 1);  // Coprime primes
        
        let c = 2u64.pow(32) - 2;  // 2 × (2^31 - 1)
        let d = 2u64.pow(31) - 1;  // Mersenne prime
        assert_eq!(binary_gcd(c, d), d);
    }

    // ===== Comprehensive Integration Test =====
    
    #[test]
    fn test_full_validation_chain() {
        // This test validates the entire K-elimination → Montgomery → reconstruction pipeline
        
        let config = KElimConfig::new(&[7919, 7927, 7933], &[7937, 7949]).unwrap();
        let mont = Montgomery::new(7919).unwrap();
        
        // Test value
        let x: u128 = 123456789;
        
        // Step 1: K-Elimination reconstruction
        let v_main = x % config.main_cap;
        let v_anchor = x % config.anchor_cap;
        let recovered_x = config.k_eliminate(v_main, v_anchor);
        assert_eq!(recovered_x, x, "K-elimination failed");
        
        // Step 2: Montgomery operations on residue
        let residue = (x % 7919) as u64;
        let mont_residue = mont.to_mont(residue);
        let result = mont.from_mont(mont_residue);
        assert_eq!(result, residue, "Montgomery round-trip failed");
        
        // Step 3: Validate all identities
        let k = x / config.main_cap;
        assert!(validate_v4(k, config.anchor_cap), "V4 failed");
        assert!(validate_v6(x, v_main, k, config.main_cap), "V6 failed");
        
        println!("Full validation chain passed for x = {}", x);
    }

    // ===== Performance Baseline Tests =====
    
    #[test]
    fn test_k_elimination_performance() {
        let config = KElimConfig::new(
            &[4294967291, 4294967279],
            &[2147483647]
        ).unwrap();
        
        let iterations = 100_000;
        let mut x: u128 = 12345;
        let mut sum: u128 = 0;
        
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            x = (x * 1103515245 + 12345) % config.main_cap;
            let v_main = x % config.main_cap;
            let v_anchor = x % config.anchor_cap;
            sum += config.k_eliminate(v_main, v_anchor);
        }
        let elapsed = start.elapsed();
        
        let ns_per_op = elapsed.as_nanos() / iterations as u128;
        println!("K-Elimination: {} ops in {:?} ({} ns/op)", 
                 iterations, elapsed, ns_per_op);
        
        // Baseline: should be < 1000 ns/op
        assert!(ns_per_op < 1000, "K-Elimination too slow: {} ns/op", ns_per_op);
        
        // Prevent optimization away
        assert!(sum > 0);
    }
    
    #[test]
    fn test_montgomery_performance() {
        let ctx = Montgomery::new(4294967291).unwrap();
        
        let iterations = 1_000_000;
        let mut a = ctx.to_mont(12345);
        let b = ctx.to_mont(54321);
        
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            a = ctx.mont_mul(a, b);
        }
        let elapsed = start.elapsed();
        
        let ns_per_op = elapsed.as_nanos() / iterations as u128;
        println!("Montgomery mul: {} ops in {:?} ({} ns/op)", 
                 iterations, elapsed, ns_per_op);
        
        // Baseline: should be < 50 ns/op
        assert!(ns_per_op < 50, "Montgomery mul too slow: {} ns/op", ns_per_op);
        
        // Prevent optimization away
        assert!(a < ctx.modulus);
    }
}

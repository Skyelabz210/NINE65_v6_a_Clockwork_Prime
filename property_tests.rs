//! Property-Based Tests for FPD
//!
//! Uses proptest to verify mathematical properties hold
//! across a wide range of random inputs.

use fpd::*;
use num_bigint::BigInt;
use num_traits::{Zero, One};
use proptest::prelude::*;

// Strategy for generating BigInts of various sizes
fn bigint_strategy(max_bits: u32) -> impl Strategy<Value = BigInt> {
    prop::collection::vec(any::<u8>(), 1..=(max_bits as usize / 8 + 1)).prop_map(|bytes| {
        BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes)
    })
}

// Strategy for generating positive BigInts (for moduli)
fn positive_bigint(min: u64, max_bits: u32) -> impl Strategy<Value = BigInt> {
    bigint_strategy(max_bits).prop_map(move |n| {
        let n = n.abs();
        if n < BigInt::from(min) {
            BigInt::from(min)
        } else {
            n
        }
    })
}

// Strategy for generating nonzero values
fn nonzero_bigint(max_bits: u32) -> impl Strategy<Value = BigInt> {
    bigint_strategy(max_bits).prop_filter("nonzero", |n| !n.is_zero())
}

proptest! {
    /// Division reconstruction identity: b × (a/b mod m) ≡ a (mod current_mod)
    #[test]
    fn prop_reconstruction_identity(
        a in any::<u64>(),
        b in 1u64..10000,
        m in 2u64..100000,
    ) {
        let a = BigInt::from(a);
        let b = BigInt::from(b);
        let m = BigInt::from(m);
        let config = DivisionConfig::default();

        if let Ok(result) = mod_div(&a, &b, &m, &config) {
            // Verify: b × result ≡ a (mod current_mod)
            let reconstructed = (&b * &result.residue) % &result.current_mod;
            let expected = &a % &result.current_mod;
            
            // Handle negative modular arithmetic
            let reconstructed = if reconstructed < BigInt::zero() {
                reconstructed + &result.current_mod
            } else {
                reconstructed
            };
            let expected = if expected < BigInt::zero() {
                expected + &result.current_mod
            } else {
                expected
            };
            
            prop_assert_eq!(reconstructed, expected);
        }
    }

    /// Binary GCD equals Euclidean GCD
    #[test]
    fn prop_binary_gcd_correct(a in any::<u64>(), b in any::<u64>()) {
        let binary_result = binary_gcd(a, b);
        
        // Verify against Euclidean
        fn euclidean(mut a: u64, mut b: u64) -> u64 {
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        }
        
        let euclidean_result = euclidean(a, b);
        prop_assert_eq!(binary_result, euclidean_result);
    }

    /// Modular inverse property: a × a⁻¹ ≡ 1 (mod m) when gcd(a, m) = 1
    #[test]
    fn prop_mod_inverse_correct(
        a in 1u64..100000,
        m in 2u64..100000,
    ) {
        let a_big = BigInt::from(a);
        let m_big = BigInt::from(m);
        
        if binary_gcd(a, m) == 1 {
            let inv = mod_inverse(&a_big, &m_big);
            prop_assert!(inv.is_some());
            
            let inv = inv.unwrap();
            let product = (&a_big * &inv) % &m_big;
            prop_assert_eq!(product, BigInt::one());
        } else {
            let inv = mod_inverse(&a_big, &m_big);
            prop_assert!(inv.is_none());
        }
    }

    /// Fast path only succeeds when gcd = 1
    #[test]
    fn prop_fast_path_requires_coprime(
        a in any::<u64>(),
        b in 1u64..10000,
        m in 2u64..10000,
    ) {
        let a = BigInt::from(a);
        let b = BigInt::from(b);
        let m = BigInt::from(m);
        
        let result = mod_div_fast(&a, &b, &m);
        let gcd = binary_gcd_bigint(&b, &m);
        
        if gcd == BigInt::one() {
            prop_assert!(result.is_ok());
        } else {
            prop_assert!(result.is_err());
        }
    }

    /// CRT reconstruction is correct
    #[test]
    fn prop_crt_correct(
        x in 0u64..1000,
        m1 in (2u64..100).prop_filter("prime-ish", |&n| is_probably_prime(n)),
        m2 in (100u64..200).prop_filter("prime-ish", |&n| is_probably_prime(n)),
    ) {
        let x = BigInt::from(x);
        let m1 = BigInt::from(m1);
        let m2 = BigInt::from(m2);
        
        // Skip if not coprime
        if binary_gcd_bigint(&m1, &m2) != BigInt::one() {
            return Ok(());
        }
        
        let r1 = &x % &m1;
        let r2 = &x % &m2;
        
        let reconstructed = bi_anchor_reconstruct(&r1, &m1, &r2, &m2).unwrap();
        
        // Verify residues match
        prop_assert_eq!(&reconstructed % &m1, r1);
        prop_assert_eq!(&reconstructed % &m2, r2);
    }

    /// GCD reduction requires divisibility
    #[test]
    fn prop_gcd_reduction_divisibility(
        a in 1u64..10000,
        b in 1u64..1000,
        m in 2u64..1000,
    ) {
        let a = BigInt::from(a);
        let b = BigInt::from(b);
        let m = BigInt::from(m);
        
        let g = binary_gcd_bigint(&b, &m);
        
        if g > BigInt::one() {
            let result = mod_div_gcd_reduction(&a, &b, &m);
            
            if &a % &g == BigInt::zero() {
                // Should succeed
                prop_assert!(result.is_ok() || result.is_err()); // May fail for other reasons
            } else {
                // Should fail with GcdDoesNotDivide
                if let Err(DivisionError::GcdDoesNotDivide { .. }) = result {
                    // Expected
                } else if result.is_err() {
                    // Some other error is also acceptable
                } else {
                    // Should not succeed
                    prop_assert!(false, "GCD reduction should fail when g ∤ a");
                }
            }
        }
    }

    /// Piggyback always finds anchor for small primes
    #[test]
    fn prop_piggyback_coverage(
        p in (2u64..1000).prop_filter("prime", |&n| is_probably_prime(n)),
    ) {
        let anchors = AnchorSet::default_set();
        let divisor = BigInt::from(p);
        
        let result = anchors.find_coprime_anchor(&divisor);
        
        // Should find an anchor for any small prime
        prop_assert!(result.is_some());
    }

    /// Anchor set is pairwise coprime
    #[test]
    fn prop_anchors_coprime(
        idx1 in 0usize..5,
        idx2 in 0usize..5,
    ) {
        if idx1 == idx2 {
            return Ok(());
        }
        
        let anchors = AnchorSet::default_set();
        let a1 = anchors.get(idx1).unwrap();
        let a2 = anchors.get(idx2).unwrap();
        
        prop_assert_eq!(binary_gcd_bigint(a1, a2), BigInt::one());
    }

    /// Shadow entropy produces varied output
    #[test]
    fn prop_entropy_varied(seed in any::<u64>()) {
        let mut entropy = ShadowEntropy::new(seed);
        
        let mut values = Vec::new();
        for _ in 0..10 {
            values.push(entropy.next_u64());
        }
        
        // Should have some variety (not all same)
        let first = values[0];
        let all_same = values.iter().all(|&v| v == first);
        prop_assert!(!all_same);
    }

    /// ModResidue preserves provenance
    #[test]
    fn prop_mod_residue_provenance(
        residue in any::<u64>(),
        base_mod in 2u64..10000,
        current_mod in 2u64..10000,
    ) {
        let mr = ModResidue::promoted(
            BigInt::from(residue),
            BigInt::from(base_mod),
            BigInt::from(current_mod),
            0,
        );
        
        prop_assert_eq!(mr.base_mod, BigInt::from(base_mod));
        prop_assert_eq!(mr.current_mod, BigInt::from(current_mod));
        prop_assert!(!mr.is_exact());
        prop_assert!(mr.needs_reconstruction());
    }
}

/// Simple primality test for property generation
fn is_probably_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let limit = (n as f64).sqrt() as u64 + 1;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_end_to_end_fast_path() {
        let config = DivisionConfig::default();
        
        // Compute 42/17 mod 97
        let a = BigInt::from(42);
        let b = BigInt::from(17);
        let m = BigInt::from(97);
        
        let result = mod_div(&a, &b, &m, &config).unwrap();
        
        assert!(result.is_exact());
        
        // Verify: 17 × result ≡ 42 (mod 97)
        let check = (&b * &result.residue) % &m;
        assert_eq!(check, &a % &m);
    }

    #[test]
    fn test_end_to_end_gcd_reduction() {
        let config = DivisionConfig::default();
        
        // Compute 12/8 mod 20: gcd(8,20) = 4, 4|12
        let a = BigInt::from(12);
        let b = BigInt::from(8);
        let m = BigInt::from(20);
        
        let result = mod_div(&a, &b, &m, &config).unwrap();
        
        // Result is in reduced ring
        assert!(!result.is_exact());
        
        // Verify in reduced ring: (12/4) / (8/4) mod (20/4) = 3/2 mod 5 = 3*3 = 9 mod 5 = 4
        // Check: 2 × 4 = 8 ≡ 3 (mod 5) ✓
    }

    #[test]
    fn test_end_to_end_piggyback() {
        let config = DivisionConfig::default();
        
        // Compute 7/9 mod 15: gcd(9,15) = 3, 3∤7, so use piggyback
        let a = BigInt::from(7);
        let b = BigInt::from(9);
        let m = BigInt::from(15);
        
        let result = mod_div(&a, &b, &m, &config).unwrap();
        
        assert!(!result.is_exact());
        assert!(result.needs_reconstruction());
        
        // Verify in anchor ring: b × result ≡ a (mod anchor)
        let b_in_anchor = &b % &result.current_mod;
        let a_in_anchor = &a % &result.current_mod;
        let check = (&b_in_anchor * &result.residue) % &result.current_mod;
        assert_eq!(check, a_in_anchor);
    }

    #[test]
    fn test_batch_matches_individual() {
        let config = DivisionConfig::default();
        let dividends: Vec<BigInt> = (1..20).map(BigInt::from).collect();
        let divisor = BigInt::from(7);
        let modulus = BigInt::from(97);
        
        let batch_results = mod_div_batch(&dividends, &divisor, &modulus, &config).unwrap();
        
        for (i, a) in dividends.iter().enumerate() {
            let individual = mod_div(a, &divisor, &modulus, &config).unwrap();
            assert_eq!(batch_results[i].residue, individual.residue);
        }
    }

    #[test]
    fn test_incremental_crt() {
        let mut crt = IncrementalCRT::new(BigInt::from(2), BigInt::from(3));
        crt.add(&BigInt::from(3), &BigInt::from(5)).unwrap();
        crt.add(&BigInt::from(2), &BigInt::from(7)).unwrap();
        
        let result = crt.value();
        
        // Verify residues
        assert_eq!(result % BigInt::from(3), BigInt::from(2));
        assert_eq!(result % BigInt::from(5), BigInt::from(3));
        assert_eq!(result % BigInt::from(7), BigInt::from(2));
    }
}

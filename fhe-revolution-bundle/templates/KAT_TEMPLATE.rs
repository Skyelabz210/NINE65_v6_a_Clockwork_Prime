//! FHE Known Answer Test (KAT) Templates
//!
//! KATs verify that cryptographic implementations produce consistent,
//! expected outputs for fixed inputs. They catch regressions and ensure
//! cross-implementation compatibility.

use crate::*;

// ============================================================================
// KAT STRUCTURE
// ============================================================================

/// A Known Answer Test case
#[derive(Debug, Clone)]
pub struct KATCase {
    pub name: &'static str,
    pub seed: u64,
    pub plaintext: u64,
    pub expected_c0_prefix: [u64; 4],  // First 4 coefficients of c0
    pub expected_c1_prefix: [u64; 4],  // First 4 coefficients of c1
    pub expected_decrypt: u64,
}

/// KAT test suite
pub const FHE_KATS: &[KATCase] = &[
    KATCase {
        name: "basic_encryption_zero",
        seed: 0x0000000000000000,
        plaintext: 0,
        expected_c0_prefix: [/* fill after first deterministic run */],
        expected_c1_prefix: [/* fill after first deterministic run */],
        expected_decrypt: 0,
    },
    KATCase {
        name: "basic_encryption_one",
        seed: 0x0000000000000001,
        plaintext: 1,
        expected_c0_prefix: [/* fill after first deterministic run */],
        expected_c1_prefix: [/* fill after first deterministic run */],
        expected_decrypt: 1,
    },
    KATCase {
        name: "basic_encryption_max",
        seed: 0x123456789ABCDEF0,
        plaintext: 65536,  // t-1 for t=65537
        expected_c0_prefix: [/* fill after first deterministic run */],
        expected_c1_prefix: [/* fill after first deterministic run */],
        expected_decrypt: 65536,
    },
];

// ============================================================================
// KAT RUNNER
// ============================================================================

/// Run all KAT tests
pub fn run_all_kats() -> Result<(), String> {
    let mut failures = Vec::new();
    
    for kat in FHE_KATS {
        if let Err(e) = run_single_kat(kat) {
            failures.push(format!("{}: {}", kat.name, e));
        }
    }
    
    if failures.is_empty() {
        Ok(())
    } else {
        Err(format!("KAT failures:\n{}", failures.join("\n")))
    }
}

/// Run a single KAT
pub fn run_single_kat(kat: &KATCase) -> Result<(), String> {
    // Initialize with deterministic seed
    let mut entropy = ShadowEntropy::from_seed(kat.seed);
    let params = FHEParams::kat_params();  // Fixed params for KATs
    
    // Generate keys deterministically
    let (sk, pk) = keygen_deterministic(&params, &mut entropy);
    
    // Encrypt
    let ct = encrypt_deterministic(kat.plaintext, &pk, &params, &mut entropy);
    
    // Verify ciphertext structure (first 4 coefficients)
    for i in 0..4 {
        if ct.c0.coeffs[i] != kat.expected_c0_prefix[i] {
            return Err(format!(
                "c0[{}] mismatch: expected {}, got {}",
                i, kat.expected_c0_prefix[i], ct.c0.coeffs[i]
            ));
        }
        if ct.c1.coeffs[i] != kat.expected_c1_prefix[i] {
            return Err(format!(
                "c1[{}] mismatch: expected {}, got {}",
                i, kat.expected_c1_prefix[i], ct.c1.coeffs[i]
            ));
        }
    }
    
    // Verify decryption
    let decrypted = decrypt(&ct, &sk, &params);
    if decrypted != kat.expected_decrypt {
        return Err(format!(
            "decrypt mismatch: expected {}, got {}",
            kat.expected_decrypt, decrypted
        ));
    }
    
    Ok(())
}

// ============================================================================
// KAT GENERATION (run once to create expected values)
// ============================================================================

/// Generate KAT expected values (run once, then copy to KATS array)
pub fn generate_kat_vectors() {
    println!("// KAT Generation Output");
    println!("// Copy these values to FHE_KATS constant\n");
    
    for (seed, plaintext) in [(0u64, 0u64), (1, 1), (0x123456789ABCDEF0, 65536)] {
        let mut entropy = ShadowEntropy::from_seed(seed);
        let params = FHEParams::kat_params();
        let (sk, pk) = keygen_deterministic(&params, &mut entropy);
        let ct = encrypt_deterministic(plaintext, &pk, &params, &mut entropy);
        let decrypted = decrypt(&ct, &sk, &params);
        
        println!("KATCase {{");
        println!("    name: \"kat_seed_{:016x}_pt_{}\",", seed, plaintext);
        println!("    seed: 0x{:016X},", seed);
        println!("    plaintext: {},", plaintext);
        println!("    expected_c0_prefix: [{}, {}, {}, {}],",
            ct.c0.coeffs[0], ct.c0.coeffs[1], ct.c0.coeffs[2], ct.c0.coeffs[3]);
        println!("    expected_c1_prefix: [{}, {}, {}, {}],",
            ct.c1.coeffs[0], ct.c1.coeffs[1], ct.c1.coeffs[2], ct.c1.coeffs[3]);
        println!("    expected_decrypt: {},", decrypted);
        println!("}},\n");
    }
}

// ============================================================================
// HOMOMORPHIC OPERATION KATS
// ============================================================================

/// KAT for homomorphic addition
#[test]
fn kat_homomorphic_add() {
    let seed = 0xDEADBEEF12345678u64;
    let mut entropy = ShadowEntropy::from_seed(seed);
    let params = FHEParams::kat_params();
    let (sk, pk) = keygen_deterministic(&params, &mut entropy);
    
    let a = 42u64;
    let b = 17u64;
    let expected = (a + b) % params.t;
    
    let ct_a = encrypt_deterministic(a, &pk, &params, &mut entropy);
    let ct_b = encrypt_deterministic(b, &pk, &params, &mut entropy);
    
    let ct_sum = homo_add(&ct_a, &ct_b);
    let result = decrypt(&ct_sum, &sk, &params);
    
    assert_eq!(result, expected, 
        "KAT homo_add failed: {} + {} = {} (got {})", a, b, expected, result);
}

/// KAT for homomorphic multiplication (ct × plaintext)
#[test]
fn kat_homomorphic_mul_plain() {
    let seed = 0xCAFEBABE87654321u64;
    let mut entropy = ShadowEntropy::from_seed(seed);
    let params = FHEParams::kat_params();
    let (sk, pk) = keygen_deterministic(&params, &mut entropy);
    
    let a = 7u64;
    let b = 11u64;
    let expected = (a * b) % params.t;
    
    let ct_a = encrypt_deterministic(a, &pk, &params, &mut entropy);
    let ct_prod = homo_mul_plain(&ct_a, b);
    let result = decrypt(&ct_prod, &sk, &params);
    
    assert_eq!(result, expected,
        "KAT homo_mul_plain failed: {} * {} = {} (got {})", a, b, expected, result);
}

/// KAT for homomorphic multiplication (ct × ct)
#[test]
fn kat_homomorphic_mul_ct() {
    let seed = 0xFEEDFACE11111111u64;
    let mut entropy = ShadowEntropy::from_seed(seed);
    let params = FHEParams::kat_params();
    let (sk, pk) = keygen_deterministic(&params, &mut entropy);
    
    let a = 5u64;
    let b = 9u64;
    let expected = (a * b) % params.t;
    
    let ct_a = encrypt_deterministic(a, &pk, &params, &mut entropy);
    let ct_b = encrypt_deterministic(b, &pk, &params, &mut entropy);
    
    // Note: This requires evaluation keys
    let eval_key = EvaluationKey::generate_deterministic(&sk, &params, &mut entropy);
    
    let ct_prod = homo_mul(&ct_a, &ct_b, &eval_key)
        .expect("homo_mul should succeed");
    let result = decrypt(&ct_prod, &sk, &params);
    
    assert_eq!(result, expected,
        "KAT homo_mul_ct failed: {} * {} = {} (got {})", a, b, expected, result);
}

// ============================================================================
// INNOVATION-SPECIFIC KATS
// ============================================================================

/// KAT for K-Elimination exact division
#[test]
fn kat_k_elimination() {
    let ke = KElimination::new(
        &[17, 19, 23],      // alpha moduli
        &[29, 31],          // beta moduli
    );
    
    // Test cases: (dividend, divisor, expected_quotient, expected_remainder)
    let test_cases = [
        (100, 7, 14, 2),      // 100 = 7*14 + 2
        (1000, 13, 76, 12),   // 1000 = 13*76 + 12
        (12345, 17, 726, 3),  // 12345 = 17*726 + 3
        (0, 5, 0, 0),         // edge case: zero
        (5, 5, 1, 0),         // exact division
    ];
    
    for (dividend, divisor, exp_q, exp_r) in test_cases {
        let (q, r) = ke.exact_divide(dividend, divisor);
        assert_eq!(q, exp_q, "K-Elim quotient: {} / {} = {} (got {})", 
            dividend, divisor, exp_q, q);
        assert_eq!(r, exp_r, "K-Elim remainder: {} % {} = {} (got {})",
            dividend, divisor, exp_r, r);
    }
}

/// KAT for Persistent Montgomery multiplication
#[test]
fn kat_persistent_montgomery() {
    let modulus = 65537u64;
    let pm = PersistentMontgomery::new(&[modulus]);
    
    // Test cases: (a, b, expected a*b mod m)
    let test_cases = [
        (12345, 54321, (12345u128 * 54321 % 65537) as u64),
        (65536, 65536, (65536u128 * 65536 % 65537) as u64),
        (1, 65536, 65536),
        (0, 12345, 0),
    ];
    
    for (a, b, expected) in test_cases {
        let result = pm.mul(a, b, modulus);
        assert_eq!(result, expected,
            "PM mul: {} * {} mod {} = {} (got {})", a, b, modulus, expected, result);
    }
}

/// KAT for NTT round-trip
#[test]
fn kat_ntt_roundtrip() {
    let n = 1024;
    let modulus = 65537u64;
    let ntt = NTTContext::new(n, modulus);
    
    // Deterministic test polynomial
    let mut poly: Vec<u64> = (0..n as u64).map(|i| (i * 7 + 13) % modulus).collect();
    let original = poly.clone();
    
    ntt.forward(&mut poly);
    ntt.inverse(&mut poly);
    
    assert_eq!(poly, original, "NTT round-trip failed");
}

// ============================================================================
// PROPERTY-BASED TESTS (Quasi-KATs)
// ============================================================================

/// Property: Encryption is deterministic given same seed
#[test]
fn prop_encryption_deterministic() {
    let seed = 0xABCDEF0123456789u64;
    let plaintext = 42u64;
    
    // First run
    let mut entropy1 = ShadowEntropy::from_seed(seed);
    let params = FHEParams::kat_params();
    let (_, pk1) = keygen_deterministic(&params, &mut entropy1);
    let ct1 = encrypt_deterministic(plaintext, &pk1, &params, &mut entropy1);
    
    // Second run (same seed)
    let mut entropy2 = ShadowEntropy::from_seed(seed);
    let (_, pk2) = keygen_deterministic(&params, &mut entropy2);
    let ct2 = encrypt_deterministic(plaintext, &pk2, &params, &mut entropy2);
    
    // Must be identical
    assert_eq!(ct1.c0.coeffs, ct2.c0.coeffs, "Deterministic encryption failed: c0 differs");
    assert_eq!(ct1.c1.coeffs, ct2.c1.coeffs, "Deterministic encryption failed: c1 differs");
}

/// Property: Homomorphic addition is commutative
#[test]
fn prop_homo_add_commutative() {
    let mut entropy = ShadowEntropy::from_seed(0x1111111111111111);
    let params = FHEParams::kat_params();
    let (sk, pk) = keygen_deterministic(&params, &mut entropy);
    
    for i in 0..100 {
        let a = entropy.next_u64() % params.t;
        let b = entropy.next_u64() % params.t;
        
        let ct_a = encrypt_deterministic(a, &pk, &params, &mut entropy);
        let ct_b = encrypt_deterministic(b, &pk, &params, &mut entropy);
        
        let sum_ab = decrypt(&homo_add(&ct_a, &ct_b), &sk, &params);
        let sum_ba = decrypt(&homo_add(&ct_b, &ct_a), &sk, &params);
        
        assert_eq!(sum_ab, sum_ba, 
            "Addition not commutative at iteration {}: {} + {} gave {} vs {}", 
            i, a, b, sum_ab, sum_ba);
    }
}

/// Property: Homomorphic multiplication is commutative
#[test]
fn prop_homo_mul_commutative() {
    let mut entropy = ShadowEntropy::from_seed(0x2222222222222222);
    let params = FHEParams::kat_params();
    let (sk, pk) = keygen_deterministic(&params, &mut entropy);
    let eval_key = EvaluationKey::generate_deterministic(&sk, &params, &mut entropy);
    
    for i in 0..10 {  // Fewer iterations due to ct×ct cost
        let a = entropy.next_u64() % 100;  // Small values to avoid noise overflow
        let b = entropy.next_u64() % 100;
        
        let ct_a = encrypt_deterministic(a, &pk, &params, &mut entropy);
        let ct_b = encrypt_deterministic(b, &pk, &params, &mut entropy);
        
        let prod_ab = decrypt(&homo_mul(&ct_a, &ct_b, &eval_key).unwrap(), &sk, &params);
        let prod_ba = decrypt(&homo_mul(&ct_b, &ct_a, &eval_key).unwrap(), &sk, &params);
        
        assert_eq!(prod_ab, prod_ba,
            "Multiplication not commutative at iteration {}: {} * {} gave {} vs {}",
            i, a, b, prod_ab, prod_ba);
    }
}

// ============================================================================
// HELPER: Fixed parameters for KATs
// ============================================================================

impl FHEParams {
    /// Fixed parameters for KAT reproducibility
    pub fn kat_params() -> Self {
        Self {
            n: 1024,
            q: 65537,  // Small prime for fast tests
            t: 257,    // Plaintext modulus
            noise_stddev: 3.2,
            relin_base: 256,
        }
    }
}

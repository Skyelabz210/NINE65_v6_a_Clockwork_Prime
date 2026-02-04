// ============================================================================
// HARDENING ARTIFACTS: Lock in the Centered Representative Invariant
// ============================================================================
// These comments and tests prevent future refactors from reintroducing the
// failure mode we just exorcised.
// ============================================================================

// ============================================================================
// ARTIFACT 1: Boundary Comment (paste at main ↔ anchor conversion point)
// ============================================================================

/// # CRITICAL INVARIANT: Centered Representative Crossing
/// 
/// Anytime we cross main ↔ anchor, we do it through the centered signed integer.
/// 
/// ```text
/// ┌─────────────────────────────────────────────────────────────────────────┐
/// │  CORRECT PATH:                                                          │
/// │                                                                         │
/// │    scaled_mod_m (u128)                                                  │
/// │         │                                                               │
/// │         ▼                                                               │
/// │    center_mod_m_to_i128(scaled_mod_m, M)  ──► true_value (i128)        │
/// │         │                                                               │
/// │         ▼                                                               │
/// │    mod_i128(true_value, anchor_prime_i)  ──► anchor residue            │
/// │                                                                         │
/// └─────────────────────────────────────────────────────────────────────────┘
/// 
/// ┌─────────────────────────────────────────────────────────────────────────┐
/// │  WRONG PATH (causes mismatch for negative coefficients):               │
/// │                                                                         │
/// │    scaled_mod_m (u128)                                                  │
/// │         │                                                               │
/// │         ▼                                                               │
/// │    scaled_mod_m % anchor_prime_i   ◄── DO NOT DO THIS                  │
/// │                                                                         │
/// └─────────────────────────────────────────────────────────────────────────┘
/// ```
/// 
/// The centered representative IS the signed integer both RNS bases encode.
/// Reducing the unsigned `scaled_mod_m` directly into anchor primes gives
/// wrong residues for values in the upper half of [0, M).


// ============================================================================
// ARTIFACT 2: Micro-test for the Centered Representative Invariant
// ============================================================================

#[cfg(test)]
mod invariant_tests {
    use super::*;
    
    /// Direct test of the centered representative invariant on rescale output.
    /// 
    /// This makes the invariant a first-class citizen, not just a side effect
    /// of larger integration tests.
    #[test]
    fn test_centered_representative_invariant_on_rescale() {
        // Setup: create context with 3 anchor primes
        let ctx = NTTContext::for_fhe();
        
        // Pick a range of test values including negative representatives
        let test_values: Vec<i64> = vec![
            0, 1, -1,           // boundary
            42, -42,            // small
            1000, -1000,        // medium
            (ctx.t / 2) as i64, // half of plaintext modulus
            -((ctx.t / 2) as i64),
        ];
        
        for &m in &test_values {
            // Convert to unsigned in [0, t)
            let m_unsigned = if m < 0 {
                (ctx.t as i64 + m) as u64
            } else {
                m as u64
            };
            
            // Create a simple polynomial with this as coeff 0
            // (Your actual API may differ)
            let poly = create_constant_poly(&ctx, m_unsigned);
            
            // Do a rescale or identity operation
            let rescaled = rescale_poly(&ctx, &poly);
            
            // Extract residues for coeff 0
            let main_residues: Vec<u64> = rescaled.main_limbs
                .iter()
                .map(|limb| limb[0])
                .collect();
            let anchor_residues: Vec<u64> = rescaled.anchor_limbs
                .iter()
                .map(|limb| limb[0])
                .collect();
            
            // === THE INVARIANT CHECK ===
            let v_m = ctx.rns.to_int(&main_residues);
            let true_value = center_mod_m_to_i128(v_m, ctx.q_product);
            
            for (i, &a_i) in ctx.dual_rns.anchor.primes.iter().enumerate() {
                let expected = mod_i128(true_value, a_i);
                let actual = anchor_residues[i];
                
                assert_eq!(
                    expected, actual,
                    "Invariant violation for input m={}:\n\
                     \x20 anchor prime[{}]={}\n\
                     \x20 expected={} actual={}\n\
                     \x20 v_m={} true_value={}",
                    m, i, a_i, expected, actual, v_m, true_value
                );
            }
        }
    }
    
    /// Verify invariant holds through mul_dual(x, 1) identity
    #[test]
    fn test_invariant_through_identity_mul() {
        let ctx = NTTContext::for_fhe();
        
        // Create a ternary coefficient (-1, 0, or 1)
        for coeff in [-1i64, 0, 1] {
            let m_unsigned = if coeff < 0 {
                (ctx.t as i64 + coeff) as u64
            } else {
                coeff as u64
            };
            
            let poly = create_constant_poly(&ctx, m_unsigned);
            let one = create_constant_poly(&ctx, 1);
            
            // Multiply by 1 (should preserve value)
            let result = mul_dual(&ctx, &poly, &one);
            
            // Check invariant on result
            assert_main_anchor_consistent(
                &ctx,
                &result.main_limbs.iter().map(|l| l[0]).collect::<Vec<_>>(),
                &result.anchor_limbs.iter().map(|l| l[0]).collect::<Vec<_>>(),
                &format!("identity mul for coeff={}", coeff),
            );
        }
    }
}


// ============================================================================
// ARTIFACT 3: Ignore Comment Template for Quarantined Tests
// ============================================================================

// Use this template for tests that require anchor.to_int() (BigInt land):

#[test]
#[ignore = "Requires anchor.to_int() which overflows u128. \
            Use assert_main_anchor_consistent() for per-prime validation instead. \
            See DIAGNOSTIC_FIX_PATTERN.md for the correct pattern."]
fn test_example_quarantined() {
    // This test tried to call:
    //   let v_a = ctx.dual_rns.anchor.to_int(&residues);  // OVERFLOWS
    //   let anchor_product = ctx.dual_rns.anchor.product(); // OVERFLOWS
    //
    // Replace with per-prime check:
    //   assert_main_anchor_consistent(&ctx, &main_res, &anchor_res, "label");
    unimplemented!("See ignore reason - use per-prime checks")
}


// ============================================================================
// ARTIFACT 4: Assert Helper with Context Label
// ============================================================================

/// Assert main/anchor consistency with a context label for better error messages.
/// 
/// This is the recommended replacement for any code that tried to do:
/// ```ignore
/// let v_a = ctx.dual_rns.anchor.to_int(&anchor_residues);  // DON'T
/// ```
fn assert_main_anchor_consistent(
    ctx: &NTTContext,
    main_residues: &[u64],
    anchor_residues: &[u64],
    label: &str,
) {
    let v_m = ctx.rns.to_int(main_residues);
    let true_value = center_mod_m_to_i128(v_m, ctx.q_product);
    
    for (i, &a_i) in ctx.dual_rns.anchor.primes.iter().enumerate() {
        let expected = mod_i128(true_value, a_i);
        let actual = anchor_residues[i];
        
        assert_eq!(
            expected, actual,
            "[{}] Anchor mismatch at prime[{}]={}:\n\
             \x20 expected={} actual={}\n\
             \x20 v_m={} true_value={}",
            label, i, a_i, expected, actual, v_m, true_value
        );
    }
}


// ============================================================================
// ARTIFACT 5: Static Assert That anchor_product Does NOT Fit in u128
// ============================================================================

#[cfg(test)]
mod anchor_product_sanity {
    use super::*;
    
    /// Compile-time reminder: anchor_product exceeds u128.
    /// 
    /// If this test ever starts passing, someone shrunk the anchors
    /// and the "BigInt land" comments are now wrong.
    #[test]
    fn anchor_product_definitely_overflows_u128() {
        let ctx = NTTContext::for_fhe();
        
        // Compute log2 of anchor product
        let log2_product: f64 = ctx.dual_rns.anchor.primes
            .iter()
            .map(|&p| (p as f64).log2())
            .sum();
        
        // u128 max is 2^128 - 1, so log2 ≈ 128
        assert!(
            log2_product > 128.0,
            "DANGER: anchor_product now fits in u128 (log2 = {:.1}).\n\
             This means 'BigInt land' comments are WRONG.\n\
             Either the anchor primes shrank, or we have a bug.",
            log2_product
        );
        
        println!(
            "✓ anchor_product log2 = {:.1} bits (definitely overflows u128)",
            log2_product
        );
    }
}


// ============================================================================
// ARTIFACT 6: Delete These Patterns If Found Anywhere
// ============================================================================

// FIND AND DELETE any code matching these patterns:
//
// ❌ ctx.dual_rns.anchor.product()
//    └── This overflows u128. There is no valid use case.
//
// ❌ ctx.dual_rns.anchor.to_int(&residues)
//    └── This requires anchor_product, which overflows.
//    └── Use per-prime checks instead.
//
// ❌ "anchor_product fits in u128"
//    └── It doesn't. With 3+ 64-bit primes, it's 192+ bits.
//
// ❌ scaled_mod_m % anchor_prime
//    └── Wrong for negative coefficients.
//    └── Must center first, then reduce.

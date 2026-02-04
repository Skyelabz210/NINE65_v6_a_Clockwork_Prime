//! Fix for test_ntt_ternary_mul and similar failing diagnostic tests
//!
//! PROBLEM: The old diagnostic code tried to compute global CRT on anchor
//! moduli that overflow u128. This caused "diagnostic explosion" rather
//! than useful error messages.
//!
//! SOLUTION: Per-prime local checks using centered representation.
//! No anchor_product, no anchor.to_int(), no overflow.

// ============================================================================
// STEP 1: Add these helpers to your tests module (or import from diagnostic_helpers)
// ============================================================================

fn center_mod_m_to_i128(x_mod_m: u128, m: u128) -> i128 {
    let half = m / 2;
    if x_mod_m > half {
        -((m - x_mod_m) as i128) // negative representative
    } else {
        x_mod_m as i128
    }
}

fn mod_i128(x: i128, p: u64) -> u64 {
    let p_i = p as i128;
    let mut r = x % p_i;
    if r < 0 { r += p_i; }
    r as u64
}

// ============================================================================
// STEP 2: Replace the overflowing section in test_ntt_ternary_mul
// ============================================================================

// BEFORE (broken - overflows u128):
// ─────────────────────────────────────────────────────────────────────────────
// let anchor_product = ctx.dual_rns.anchor.product();  // <- OVERFLOW HERE
// let v_a = ctx.dual_rns.anchor.to_int(&as_anchor_0);  // <- AND HERE
// let true_value = if v_a > anchor_product / 2 {
//     (v_a as i128) - (anchor_product as i128)         // <- BROKEN
// } else {
//     v_a as i128
// };
// // ... K-elimination verification that doesn't work ...
// ─────────────────────────────────────────────────────────────────────────────

// AFTER (fixed - per-prime checks, no overflow):
// ─────────────────────────────────────────────────────────────────────────────
fn fixed_diagnostic_section(
    // These are the values you have in the test:
    as_main_0: &[u64],        // main residues for coeff 0
    as_anchor_0: &[u64],      // anchor residues for coeff 0
    ctx_q_product: u128,      // ctx.q_product (main modulus product)
    ctx_rns_to_int: impl Fn(&[u64]) -> u128,  // ctx.rns.to_int
    anchor_primes: &[u64],    // ctx.dual_rns.anchor.primes
) {
    // Step 1: Get main CRT value (u128 is fine for main product ~1e18)
    let v_m = ctx_rns_to_int(as_main_0);
    
    // Step 2: Center to signed representation (THE true value)
    let true_value = center_mod_m_to_i128(v_m, ctx_q_product);
    
    // Step 3: Diagnostic output
    println!("v_m (main CRT, mod M) = {}", v_m);
    println!("true_value (centered) = {}", true_value);
    println!("M = {}", ctx_q_product);
    println!("anchor primes = {:?}", anchor_primes);
    
    // Step 4: Per-prime consistency check
    // This is the key insight: check each anchor prime against the SAME
    // signed integer that main represents. No global anchor CRT needed.
    for (i, &a_i) in anchor_primes.iter().enumerate() {
        let expected = mod_i128(true_value, a_i);
        let actual = as_anchor_0[i];
        
        assert_eq!(
            expected, actual,
            "Anchor residue mismatch at prime[{}]={}:\n\
             \x20 expected (true_value mod a_i) = {}\n\
             \x20 actual   (anchor limb)        = {}\n\
             \x20 v_m(mod M)={}  true_value(centered)={}",
            i, a_i, expected, actual, v_m, true_value
        );
    }
    
    println!("✓ All anchor primes consistent with main (true_value = {})", true_value);
}
// ─────────────────────────────────────────────────────────────────────────────

// ============================================================================
// STEP 3: (Optional) Per-prime K equation check
// ============================================================================

/// If you want to verify K-elimination per prime without global CRT:
/// 
/// For each anchor prime (a):
///   k_mod_a = ((v_a_mod_a - v_m_mod_a) * inv(M, a)) mod a
///
/// This requires:
/// - v_a_mod_a: anchor residue (you have this)
/// - v_m_mod_a: main value mod anchor prime (compute from true_value)
/// - inv(M, a): modular inverse of main product mod anchor prime
fn verify_k_per_prime(
    true_value: i128,          // The centered main value
    anchor_residues: &[u64],   // as_anchor_0
    anchor_primes: &[u64],     // ctx.dual_rns.anchor.primes
    main_product: u128,        // ctx.q_product
    inv_mod: impl Fn(u128, u64) -> u64,  // Your modular inverse function
) {
    println!("Per-prime K verification:");
    
    for (i, &a_i) in anchor_primes.iter().enumerate() {
        let v_m_mod_a = mod_i128(true_value, a_i);
        let v_a_mod_a = anchor_residues[i];
        let m_inv_a = inv_mod(main_product % (a_i as u128), a_i);
        
        // k = (v_a - v_m) * M^{-1} mod a
        let diff = if v_a_mod_a >= v_m_mod_a {
            v_a_mod_a - v_m_mod_a
        } else {
            v_a_mod_a + a_i - v_m_mod_a
        };
        let k_mod_a = ((diff as u128) * (m_inv_a as u128) % (a_i as u128)) as u64;
        
        println!(
            "  prime[{}]={}: v_a={} v_m={} k={} (M^-1={})",
            i, a_i, v_a_mod_a, v_m_mod_a, k_mod_a, m_inv_a
        );
    }
}

// ============================================================================
// TEMPLATE: Drop-in replacement for your test
// ============================================================================

#[cfg(test)]
mod example_fixed_test {
    use super::*;
    
    // This shows the structure of a fixed test
    #[test]
    #[ignore] // Remove when integrated with your actual context
    fn test_ntt_ternary_mul_fixed() {
        // ... your existing setup code ...
        // let ctx = create_test_context();
        // let a = random_ternary_poly(&ctx);
        // let b = random_ternary_poly(&ctx);
        // let c = ntt_mul(&ctx, &a, &b);
        
        // Extract coefficient 0 residues
        // let as_main_0: Vec<u64> = c.main_limbs.iter().map(|l| l[0]).collect();
        // let as_anchor_0: Vec<u64> = c.anchor_limbs.iter().map(|l| l[0]).collect();
        
        // === THE FIX: Replace overflowing CRT with per-prime check ===
        
        // Simulated values for this example:
        let as_main_0 = vec![123u64, 456, 789];
        let as_anchor_0 = vec![2u64, 5, 8];
        let ctx_q_product = 1_000_000_000_000_000_000u128; // ~1e18
        let anchor_primes = vec![7u64, 11, 13];
        
        // Main CRT reconstruction (u128 is fine for main)
        // In your code: let v_m = ctx.rns.to_int(&as_main_0);
        let v_m = 12345u128; // simulated
        
        // Center to signed
        let true_value = center_mod_m_to_i128(v_m, ctx_q_product);
        
        println!("=== Coefficient 0 diagnostic ===");
        println!("v_m (main CRT) = {}", v_m);
        println!("true_value (centered) = {}", true_value);
        println!("M = {}", ctx_q_product);
        println!("anchor primes = {:?}", anchor_primes);
        
        // Per-prime check (THE KEY FIX)
        for (i, &a_i) in anchor_primes.iter().enumerate() {
            let expected = mod_i128(true_value, a_i);
            let actual = as_anchor_0[i];
            
            // In real test, use assert_eq! here
            println!(
                "  prime[{}]={}: expected={} actual={} {}",
                i, a_i, expected, actual,
                if expected == actual { "✓" } else { "✗ MISMATCH" }
            );
        }
        
        println!("Diagnostic complete - no u128 overflow!");
    }
}

// ============================================================================
// CHECKLIST: Apply this fix to all 9 failing diagnostics
// ============================================================================
//
// 1. [ ] test_ntt_ternary_mul
// 2. [ ] test_ntt_binary_mul  
// 3. [ ] test_rescale_consistency
// 4. [ ] test_key_switch_diagnostic
// 5. [ ] test_homomorphic_add_diagnostic
// 6. [ ] test_homomorphic_mul_diagnostic
// 7. [ ] test_rotation_diagnostic
// 8. [ ] test_bootstrap_diagnostic
// 9. [ ] test_encode_decode_diagnostic
//
// Each one: remove anchor_product/anchor.to_int(), add per-prime check.
// ============================================================================

//! Diagnostic Helpers for Dual-RNS Consistency Checks
//!
//! These helpers replace overflowing global CRT reconstruction with
//! per-prime local checks that never exceed u128.
//!
//! PRINCIPLE: "Diagnostics should diagnose, not explode."
//!
//! The fix pattern:
//! 1. Get main CRT value (u128 is fine for main product ~1e18)
//! 2. Center it to signed representation (the "true value")
//! 3. Check each anchor prime independently - no anchor_product needed
//!
//! Usage: Drop this into your tests module or create as a separate test helper.

/// Convert unsigned residue to centered signed representative.
///
/// For modulus M and value x in [0, M), returns:
/// - x if x <= M/2 (positive representative)
/// - x - M if x > M/2 (negative representative)
///
/// This matches the signed integer that both main and anchor
/// should be representing after proper mapping.
#[inline]
pub fn center_mod_m_to_i128(x_mod_m: u128, m: u128) -> i128 {
    let half = m / 2;
    if x_mod_m > half {
        -((m - x_mod_m) as i128) // negative representative
    } else {
        x_mod_m as i128
    }
}

/// Compute x mod p for signed x, returning unsigned result in [0, p).
#[inline]
pub fn mod_i128(x: i128, p: u64) -> u64 {
    let p_i = p as i128;
    let mut r = x % p_i;
    if r < 0 {
        r += p_i;
    }
    r as u64
}

/// Per-prime consistency check between main and anchor residues.
///
/// This replaces the overflowing pattern:
/// ```ignore
/// // OLD WAY (OVERFLOWS):
/// let anchor_product = ctx.dual_rns.anchor.product();  // u128 overflow!
/// let v_a = ctx.dual_rns.anchor.to_int(&as_anchor_0);  // explosion
/// ```
///
/// # Arguments
/// * `main_residues` - Residues in main RNS basis
/// * `anchor_residues` - Residues in anchor RNS basis  
/// * `main_to_int` - Function to reconstruct main value (u128 is fine)
/// * `main_product` - Product of main moduli (q_product)
/// * `anchor_primes` - The anchor prime moduli
///
/// # Returns
/// `Ok(true_value)` if all primes match, `Err` with diagnostic info otherwise.
pub fn check_main_anchor_consistency(
    main_residues: &[u64],
    anchor_residues: &[u64],
    main_value: u128,           // Already reconstructed from main RNS
    main_product: u128,         // ctx.q_product
    anchor_primes: &[u64],      // ctx.dual_rns.anchor.primes
) -> Result<i128, String> {
    // Center the main value to get the "true" signed integer
    let true_value = center_mod_m_to_i128(main_value, main_product);
    
    // Check each anchor prime independently
    for (i, &a_i) in anchor_primes.iter().enumerate() {
        let expected = mod_i128(true_value, a_i);
        let actual = anchor_residues[i];
        
        if expected != actual {
            return Err(format!(
                "Anchor residue mismatch at prime[{}]={}:\n\
                 \x20 expected (true_value mod a_i) = {}\n\
                 \x20 actual   (anchor limb)        = {}\n\
                 \x20 v_m(mod M)={}  true_value(centered)={}",
                i, a_i, expected, actual, main_value, true_value
            ));
        }
    }
    
    Ok(true_value)
}

/// Diagnostic print for debugging dual-RNS state.
pub fn print_dual_rns_diagnostic(
    label: &str,
    main_value: u128,
    main_product: u128,
    anchor_primes: &[u64],
    anchor_residues: &[u64],
) {
    let true_value = center_mod_m_to_i128(main_value, main_product);
    
    println!("=== {} ===", label);
    println!("v_m (main CRT, mod M) = {}", main_value);
    println!("true_value (centered) = {}", true_value);
    println!("M (main product) = {}", main_product);
    println!("anchor primes = {:?}", anchor_primes);
    println!("anchor residues = {:?}", anchor_residues);
    
    // Show per-prime expected vs actual
    for (i, &a_i) in anchor_primes.iter().enumerate() {
        let expected = mod_i128(true_value, a_i);
        let actual = anchor_residues[i];
        let status = if expected == actual { "✓" } else { "✗" };
        println!(
            "  prime[{}]={}: expected={} actual={} {}",
            i, a_i, expected, actual, status
        );
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_mod() {
        // m = 100, half = 50
        // 30 -> 30 (positive)
        // 70 -> -30 (negative, since 70 > 50)
        assert_eq!(center_mod_m_to_i128(30, 100), 30);
        assert_eq!(center_mod_m_to_i128(70, 100), -30);
        assert_eq!(center_mod_m_to_i128(50, 100), 50);  // exactly half stays positive
        assert_eq!(center_mod_m_to_i128(51, 100), -49);
    }

    #[test]
    fn test_mod_i128_positive() {
        assert_eq!(mod_i128(17, 5), 2);
        assert_eq!(mod_i128(100, 7), 2);
    }

    #[test]
    fn test_mod_i128_negative() {
        // -17 mod 5 = -2 + 5 = 3
        assert_eq!(mod_i128(-17, 5), 3);
        // -3 mod 7 = -3 + 7 = 4
        assert_eq!(mod_i128(-3, 7), 4);
    }

    #[test]
    fn test_consistency_check_pass() {
        // Simulate: true value = 42
        // Main: M = 1000, v_m = 42
        // Anchor primes: [7, 11, 13]
        // Expected residues: 42 mod 7 = 0, 42 mod 11 = 9, 42 mod 13 = 3
        
        let main_value = 42u128;
        let main_product = 1000u128;
        let anchor_primes = vec![7u64, 11, 13];
        let anchor_residues = vec![0u64, 9, 3];
        
        let result = check_main_anchor_consistency(
            &[], // main_residues not used in this simplified version
            &anchor_residues,
            main_value,
            main_product,
            &anchor_primes,
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_consistency_check_negative() {
        // Simulate: true value = -30 (represented as 970 mod 1000)
        // Main: M = 1000, v_m = 970
        // Anchor primes: [7, 11]
        // Expected: -30 mod 7 = -2 + 7 = 5
        //           -30 mod 11 = -8 + 11 = 3
        
        let main_value = 970u128;  // = -30 mod 1000
        let main_product = 1000u128;
        let anchor_primes = vec![7u64, 11];
        let anchor_residues = vec![5u64, 3];
        
        let result = check_main_anchor_consistency(
            &[],
            &anchor_residues,
            main_value,
            main_product,
            &anchor_primes,
        );
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), -30);
    }

    #[test]
    fn test_consistency_check_fail() {
        // Mismatch: expected 5, got 6
        let main_value = 970u128;
        let main_product = 1000u128;
        let anchor_primes = vec![7u64, 11];
        let anchor_residues = vec![6u64, 3];  // Wrong! Should be 5
        
        let result = check_main_anchor_consistency(
            &[],
            &anchor_residues,
            main_value,
            main_product,
            &anchor_primes,
        );
        
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("prime[0]=7"));
        assert!(err.contains("expected"));
    }
}

// ============================================================================
// EXAMPLE USAGE IN YOUR TEST
// ============================================================================
//
// Replace this overflowing code:
// ```
// let anchor_product = ctx.dual_rns.anchor.product();  // OVERFLOW!
// let v_a = ctx.dual_rns.anchor.to_int(&as_anchor_0);  // EXPLODES
// let true_value = ...; // broken
// ```
//
// With this:
// ```
// use crate::diagnostic_helpers::{check_main_anchor_consistency, print_dual_rns_diagnostic};
//
// let v_m = ctx.rns.to_int(&as_main_0);  // u128 is fine for main
//
// // Debug print (optional)
// print_dual_rns_diagnostic(
//     "Coefficient 0 after rescale",
//     v_m,
//     ctx.q_product,
//     &ctx.dual_rns.anchor.primes,
//     &as_anchor_0,
// );
//
// // Actual consistency check
// let true_value = check_main_anchor_consistency(
//     &as_main_0,
//     &as_anchor_0,
//     v_m,
//     ctx.q_product,
//     &ctx.dual_rns.anchor.primes,
// ).expect("Main/anchor consistency check failed");
//
// println!("Verified: true_value = {}", true_value);
// ```
//
// This pattern works for all 9 failing diagnostics. The key insight:
// - Never compute anchor_product (it overflows u128)
// - Never call anchor.to_int() (ditto)
// - Check each anchor prime independently against the centered main value
// - The centered representation IS the signed integer both systems encode

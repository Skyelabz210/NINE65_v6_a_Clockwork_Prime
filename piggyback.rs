//! Coprime Piggyback Division
//!
//! INNOVATION: K-Elimination anchor technique
//!
//! When gcd(divisor, base_mod) ≠ 1, we find an anchor modulus where
//! gcd(divisor, anchor) = 1 and compute the division there.
//! The result is marked as "Promoted" and may need reconstruction.
//!
//! This is the core piggyback mechanism that enables division
//! in composite modulus rings.

use crate::anchor_set::AnchorSet;
use crate::binary_gcd::binary_gcd_bigint;
use crate::error::{DivisionError, DivisionResult};
use crate::fast_path::mod_div_fast;
use crate::mod_inverse::mod_inverse;
use crate::mod_residue::ModResidue;
use num_bigint::BigInt;
use num_traits::{Zero, One};

/// Piggyback division using coprime anchor moduli
///
/// When division is not possible in the base ring (gcd ≠ 1),
/// we "piggyback" on a coprime anchor where division IS possible.
///
/// # Algorithm
/// 1. Try fast path first (base ring division)
/// 2. If that fails, find anchor M_j where gcd(divisor, M_j) = 1
/// 3. Compute division in anchor ring
/// 4. Return Promoted result (needs reconstruction for base ring use)
///
/// # Arguments
/// * `dividend` - The numerator (a)
/// * `divisor` - The denominator (b)
/// * `base_mod` - The intended ring modulus (M)
/// * `anchors` - Pre-configured anchor set
pub fn mod_div_piggyback(
    dividend: &BigInt,
    divisor: &BigInt,
    base_mod: &BigInt,
    anchors: &AnchorSet,
) -> DivisionResult<ModResidue> {
    // Input validation
    if base_mod <= &BigInt::one() {
        return Err(DivisionError::invalid_modulus(base_mod.clone()));
    }

    if divisor.is_zero() {
        return Err(DivisionError::DivisionByZero);
    }

    // Try fast path first (preferred)
    if let Ok(result) = mod_div_fast(dividend, divisor, base_mod) {
        return Ok(result);
    }

    // Fast path failed - need to use piggyback
    // Find a coprime anchor
    let (anchor_idx, anchor) = anchors
        .find_coprime_anchor(divisor)
        .ok_or_else(|| DivisionError::no_coprime_anchor(divisor.clone(), anchors.len()))?;

    // Reduce inputs to anchor ring
    let a_mod_anchor = normalize_mod(dividend, anchor);
    let b_mod_anchor = normalize_mod(divisor, anchor);

    // Compute inverse in anchor ring
    // This should always succeed since we verified coprimality
    let inv = mod_inverse(&b_mod_anchor, anchor).ok_or_else(|| {
        DivisionError::internal(format!(
            "Inverse failed despite anchor selection: divisor={}, anchor={}",
            divisor, anchor
        ))
    })?;

    // Compute division in anchor ring
    let residue = (&a_mod_anchor * &inv) % anchor;

    Ok(ModResidue::promoted(
        residue,
        base_mod.clone(),
        anchor.clone(),
        anchor_idx,
    ))
}

/// Piggyback division with multiple anchors for CRT reconstruction
///
/// Computes division in multiple anchor rings simultaneously,
/// enabling CRT reconstruction back to a larger ring.
///
/// Returns results from all coprime anchors.
pub fn mod_div_piggyback_multi(
    dividend: &BigInt,
    divisor: &BigInt,
    base_mod: &BigInt,
    anchors: &AnchorSet,
) -> DivisionResult<Vec<ModResidue>> {
    // Input validation
    if base_mod <= &BigInt::one() {
        return Err(DivisionError::invalid_modulus(base_mod.clone()));
    }

    if divisor.is_zero() {
        return Err(DivisionError::DivisionByZero);
    }

    // Find all coprime anchors
    let coprime_anchors = anchors.find_all_coprime_anchors(divisor);

    if coprime_anchors.is_empty() {
        return Err(DivisionError::no_coprime_anchor(divisor.clone(), anchors.len()));
    }

    // Compute division in each anchor ring
    let mut results = Vec::with_capacity(coprime_anchors.len());

    for (anchor_idx, anchor) in coprime_anchors {
        let a_mod = normalize_mod(dividend, anchor);
        let b_mod = normalize_mod(divisor, anchor);

        let inv = mod_inverse(&b_mod, anchor).ok_or_else(|| {
            DivisionError::internal("Inverse failed for coprime anchor".to_string())
        })?;

        let residue = (&a_mod * &inv) % anchor;

        results.push(ModResidue::promoted(
            residue,
            base_mod.clone(),
            anchor.clone(),
            anchor_idx,
        ));
    }

    Ok(results)
}

/// Bi-anchor piggyback for optimal CRT reconstruction
///
/// Computes in exactly 2 anchors, minimizing overhead while
/// enabling CRT reconstruction.
pub fn mod_div_piggyback_bi(
    dividend: &BigInt,
    divisor: &BigInt,
    base_mod: &BigInt,
    anchors: &AnchorSet,
) -> DivisionResult<(ModResidue, ModResidue)> {
    // Input validation
    if base_mod <= &BigInt::one() {
        return Err(DivisionError::invalid_modulus(base_mod.clone()));
    }

    if divisor.is_zero() {
        return Err(DivisionError::DivisionByZero);
    }

    // Find two coprime anchors
    let coprime_anchors = anchors.find_all_coprime_anchors(divisor);

    if coprime_anchors.len() < 2 {
        return Err(DivisionError::no_coprime_anchor(
            divisor.clone(),
            anchors.len(),
        ));
    }

    // Use first two
    let (idx1, anchor1) = coprime_anchors[0];
    let (idx2, anchor2) = coprime_anchors[1];

    // Compute in first anchor
    let a1 = normalize_mod(dividend, anchor1);
    let b1 = normalize_mod(divisor, anchor1);
    let inv1 = mod_inverse(&b1, anchor1).unwrap();
    let res1 = (&a1 * &inv1) % anchor1;

    // Compute in second anchor
    let a2 = normalize_mod(dividend, anchor2);
    let b2 = normalize_mod(divisor, anchor2);
    let inv2 = mod_inverse(&b2, anchor2).unwrap();
    let res2 = (&a2 * &inv2) % anchor2;

    Ok((
        ModResidue::promoted(res1, base_mod.clone(), anchor1.clone(), idx1),
        ModResidue::promoted(res2, base_mod.clone(), anchor2.clone(), idx2),
    ))
}

/// Normalize a value to [0, modulus)
#[inline]
fn normalize_mod(value: &BigInt, modulus: &BigInt) -> BigInt {
    let r = value % modulus;
    if r < BigInt::zero() {
        r + modulus
    } else {
        r
    }
}

/// Try piggyback, returning None if not applicable
pub fn try_piggyback(
    dividend: &BigInt,
    divisor: &BigInt,
    base_mod: &BigInt,
    anchors: &AnchorSet,
) -> Option<ModResidue> {
    mod_div_piggyback(dividend, divisor, base_mod, anchors).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piggyback_uses_fast_path_when_possible() {
        let anchors = AnchorSet::default_set();

        // 10 / 3 mod 7: gcd(3, 7) = 1, should use fast path
        let result = mod_div_piggyback(
            &BigInt::from(10),
            &BigInt::from(3),
            &BigInt::from(7),
            &anchors,
        )
        .unwrap();

        assert!(result.is_exact());
    }

    #[test]
    fn test_piggyback_composite_modulus() {
        let anchors = AnchorSet::default_set();

        // 7 / 9 mod 15: gcd(9, 15) = 3 ≠ 1
        // But gcd(9, anchor) = 1 for some anchor
        let result = mod_div_piggyback(
            &BigInt::from(7),
            &BigInt::from(9),
            &BigInt::from(15),
            &anchors,
        )
        .unwrap();

        assert!(!result.is_exact());
        assert!(result.needs_reconstruction());
        assert!(matches!(result.status, crate::mod_residue::DivStatus::Promoted { .. }));
    }

    #[test]
    fn test_piggyback_provenance_preserved() {
        let anchors = AnchorSet::default_set();

        let result = mod_div_piggyback(
            &BigInt::from(7),
            &BigInt::from(9),
            &BigInt::from(15),
            &anchors,
        )
        .unwrap();

        // Base mod should be original
        assert_eq!(result.base_mod, BigInt::from(15));
        // Current mod should be anchor (different from base)
        assert_ne!(result.current_mod, BigInt::from(15));
    }

    #[test]
    fn test_piggyback_verifies_in_anchor_ring() {
        let anchors = AnchorSet::default_set();

        let a = BigInt::from(7);
        let b = BigInt::from(9);
        let m = BigInt::from(15);

        let result = mod_div_piggyback(&a, &b, &m, &anchors).unwrap();

        // Verify in the anchor ring: b × result ≡ a (mod anchor)
        let b_in_anchor = &b % &result.current_mod;
        let a_in_anchor = &a % &result.current_mod;
        let reconstructed = (&b_in_anchor * &result.residue) % &result.current_mod;

        assert_eq!(reconstructed, a_in_anchor);
    }

    #[test]
    fn test_piggyback_division_by_zero() {
        let anchors = AnchorSet::default_set();

        let result = mod_div_piggyback(
            &BigInt::from(7),
            &BigInt::from(0),
            &BigInt::from(15),
            &anchors,
        );

        assert!(matches!(result, Err(DivisionError::DivisionByZero)));
    }

    #[test]
    fn test_piggyback_invalid_modulus() {
        let anchors = AnchorSet::default_set();

        let result = mod_div_piggyback(
            &BigInt::from(7),
            &BigInt::from(9),
            &BigInt::from(0),
            &anchors,
        );

        assert!(matches!(result, Err(DivisionError::InvalidModulus(_))));
    }

    #[test]
    fn test_piggyback_multi() {
        let anchors = AnchorSet::default_set();

        let results = mod_div_piggyback_multi(
            &BigInt::from(7),
            &BigInt::from(9),
            &BigInt::from(15),
            &anchors,
        )
        .unwrap();

        assert!(!results.is_empty());

        // All results should be promoted
        for result in &results {
            assert!(!result.is_exact());
            assert!(result.needs_reconstruction());
        }

        // All results should verify in their respective anchor rings
        for result in &results {
            let b = BigInt::from(9);
            let a = BigInt::from(7);
            let b_in_anchor = &b % &result.current_mod;
            let a_in_anchor = &a % &result.current_mod;
            let reconstructed = (&b_in_anchor * &result.residue) % &result.current_mod;
            assert_eq!(reconstructed, a_in_anchor);
        }
    }

    #[test]
    fn test_piggyback_bi() {
        let anchors = AnchorSet::default_set();

        let (res1, res2) = mod_div_piggyback_bi(
            &BigInt::from(7),
            &BigInt::from(9),
            &BigInt::from(15),
            &anchors,
        )
        .unwrap();

        // Both should be promoted
        assert!(!res1.is_exact());
        assert!(!res2.is_exact());

        // Different anchor indices
        assert_ne!(res1.anchor_index(), res2.anchor_index());
    }

    #[test]
    fn test_try_piggyback() {
        let anchors = AnchorSet::default_set();

        // Should succeed
        let result = try_piggyback(
            &BigInt::from(7),
            &BigInt::from(9),
            &BigInt::from(15),
            &anchors,
        );
        assert!(result.is_some());

        // Should fail (division by zero)
        let result = try_piggyback(
            &BigInt::from(7),
            &BigInt::from(0),
            &BigInt::from(15),
            &anchors,
        );
        assert!(result.is_none());
    }
}

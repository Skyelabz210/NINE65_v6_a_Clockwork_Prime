//! GCD Reduction Path for Division
//!
//! INNOVATION: K-Elimination for quotient tracking
//!
//! When g = gcd(b, M) > 1 and g | a, we can reduce to a quotient ring:
//! Solve (a/g) / (b/g) mod (M/g) where gcd(b/g, M/g) = 1
//!
//! This extends the reach of modular division to cases where
//! the divisor shares factors with the modulus.

use crate::binary_gcd::binary_gcd_bigint;
use crate::error::{DivisionError, DivisionResult};
use crate::fast_path::mod_div_fast;
use crate::mod_residue::{DivStatus, ModResidue};
use num_bigint::BigInt;
use num_traits::{Zero, One};

/// GCD reduction: divide out common factor to enable division
///
/// When g = gcd(b, M) > 1 and g | a:
/// We solve (a/g) / (b/g) mod (M/g) where gcd(b/g, M/g) = 1
///
/// # Algorithm
/// 1. Compute g = gcd(b, M)
/// 2. If g = 1, use fast path
/// 3. Check g | a (otherwise no solution exists)
/// 4. Reduce: a' = a/g, b' = b/g, M' = M/g
/// 5. Verify gcd(b', M') = 1 (should be by construction)
/// 6. Compute a'/b' mod M'
/// 7. Return CRT result (needs reconstruction for base ring)
///
/// # Mathematical Basis
/// Theorem (Congruence Divisibility):
/// b·x ≡ a (mod M) has a solution iff gcd(b, M) | a
/// When solution exists, it's unique mod M/gcd(b, M)
pub fn mod_div_gcd_reduction(
    dividend: &BigInt,
    divisor: &BigInt,
    modulus: &BigInt,
) -> DivisionResult<ModResidue> {
    // Input validation
    if modulus <= &BigInt::one() {
        return Err(DivisionError::invalid_modulus(modulus.clone()));
    }

    if divisor.is_zero() {
        return Err(DivisionError::DivisionByZero);
    }

    // Normalize inputs
    let a = normalize_mod(dividend, modulus);
    let b = normalize_mod(divisor, modulus);

    if b.is_zero() {
        return Err(DivisionError::DivisionByZero);
    }

    // Compute GCD
    let g = binary_gcd_bigint(&b, modulus);

    // Fast path: gcd = 1
    if g == BigInt::one() {
        return mod_div_fast(dividend, divisor, modulus);
    }

    // Check divisibility: g | a
    // This is required for a solution to exist
    if &a % &g != BigInt::zero() {
        return Err(DivisionError::gcd_does_not_divide(
            dividend.clone(),
            divisor.clone(),
            modulus.clone(),
            g.clone(),
        ));
    }

    // Reduce to quotient ring
    let a_reduced = &a / &g;
    let b_reduced = &b / &g;
    let m_reduced = modulus / &g;

    // Verify gcd(b_reduced, m_reduced) = 1
    // This should always be true by construction
    let g_check = binary_gcd_bigint(&b_reduced, &m_reduced);
    if g_check != BigInt::one() {
        // Recursive reduction needed
        return mod_div_gcd_reduction_recursive(&a_reduced, &b_reduced, &m_reduced, modulus, 1);
    }

    // Now we can use fast path in reduced ring
    let result = mod_div_fast(&a_reduced, &b_reduced, &m_reduced)?;

    // Result is valid in reduced ring M/g
    // Mark as CRT for potential reconstruction
    Ok(ModResidue::crt(
        result.residue,
        modulus.clone(),
        m_reduced,
        1, // Single reduction step
    ))
}

/// Recursive GCD reduction for deeply composite moduli
fn mod_div_gcd_reduction_recursive(
    dividend: &BigInt,
    divisor: &BigInt,
    modulus: &BigInt,
    original_base: &BigInt,
    depth: usize,
) -> DivisionResult<ModResidue> {
    const MAX_DEPTH: usize = 10;

    if depth > MAX_DEPTH {
        return Err(DivisionError::internal(format!(
            "GCD reduction exceeded max depth {}",
            MAX_DEPTH
        )));
    }

    let g = binary_gcd_bigint(divisor, modulus);

    if g == BigInt::one() {
        let result = mod_div_fast(dividend, divisor, modulus)?;
        return Ok(ModResidue::crt(
            result.residue,
            original_base.clone(),
            modulus.clone(),
            depth,
        ));
    }

    if dividend % &g != BigInt::zero() {
        return Err(DivisionError::gcd_does_not_divide(
            dividend.clone(),
            divisor.clone(),
            modulus.clone(),
            g.clone(),
        ));
    }

    let a_reduced = dividend / &g;
    let b_reduced = divisor / &g;
    let m_reduced = modulus / &g;

    mod_div_gcd_reduction_recursive(&a_reduced, &b_reduced, &m_reduced, original_base, depth + 1)
}

/// Combined GCD reduction with lift back to original ring
///
/// Computes division and lifts the result back to the original ring
/// using CRT-like reconstruction.
pub fn mod_div_gcd_reduction_with_lift(
    dividend: &BigInt,
    divisor: &BigInt,
    modulus: &BigInt,
) -> DivisionResult<ModResidue> {
    let result = mod_div_gcd_reduction(dividend, divisor, modulus)?;

    if result.is_exact() {
        return Ok(result);
    }

    // Compute number of solutions in original ring
    let g = binary_gcd_bigint(divisor, modulus);
    let num_solutions = &g;

    // The result in reduced ring represents num_solutions distinct values in original ring
    // Primary representative: result.residue
    // Others: result.residue + k * (M/g) for k = 1..g-1

    // Return primary representative, marked appropriately
    Ok(ModResidue::crt(
        result.residue,
        modulus.clone(),
        result.current_mod,
        1,
    ))
}

/// Enumerate all solutions in original ring
///
/// When gcd(b, M) = g > 1 and g | a, there are g distinct solutions
/// in the original ring: x, x + M/g, x + 2M/g, ..., x + (g-1)M/g
pub fn enumerate_solutions(
    dividend: &BigInt,
    divisor: &BigInt,
    modulus: &BigInt,
) -> DivisionResult<Vec<BigInt>> {
    let result = mod_div_gcd_reduction(dividend, divisor, modulus)?;

    let g = binary_gcd_bigint(divisor, modulus);
    let step = modulus / &g;

    let mut solutions = Vec::new();
    let mut x = result.residue.clone();

    for _ in 0..g.to_u64_digits().1.first().copied().unwrap_or(1) {
        solutions.push(x.clone());
        x = (&x + &step) % modulus;
    }

    Ok(solutions)
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

/// Try GCD reduction, returning None if not applicable
pub fn try_gcd_reduction(
    dividend: &BigInt,
    divisor: &BigInt,
    modulus: &BigInt,
) -> Option<ModResidue> {
    mod_div_gcd_reduction(dividend, divisor, modulus).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_reduction_fast_path() {
        // gcd(3, 7) = 1, should use fast path
        let result = mod_div_gcd_reduction(
            &BigInt::from(10),
            &BigInt::from(3),
            &BigInt::from(7),
        )
        .unwrap();

        assert!(result.is_exact());
    }

    #[test]
    fn test_gcd_reduction_basic() {
        // 6 / 4 mod 10: gcd(4, 10) = 2, 2|6
        // Reduce: 3 / 2 mod 5
        // 2⁻¹ mod 5 = 3, so 3 × 3 mod 5 = 4
        let result = mod_div_gcd_reduction(
            &BigInt::from(6),
            &BigInt::from(4),
            &BigInt::from(10),
        )
        .unwrap();

        assert_eq!(result.residue, BigInt::from(4));
        assert!(!result.is_exact());
    }

    #[test]
    fn test_gcd_reduction_verify() {
        // Verify: 4 × 4 = 16 ≡ 6 (mod 10)
        let result = mod_div_gcd_reduction(
            &BigInt::from(6),
            &BigInt::from(4),
            &BigInt::from(10),
        )
        .unwrap();

        // Verify in reduced ring
        let b_reduced = BigInt::from(4) / BigInt::from(2);
        let a_reduced = BigInt::from(6) / BigInt::from(2);
        let reconstructed = (&b_reduced * &result.residue) % &result.current_mod;
        assert_eq!(reconstructed, a_reduced);
    }

    #[test]
    fn test_gcd_reduction_not_divisible() {
        // 7 / 4 mod 10: gcd(4, 10) = 2, but 2 ∤ 7
        let result = mod_div_gcd_reduction(
            &BigInt::from(7),
            &BigInt::from(4),
            &BigInt::from(10),
        );

        assert!(matches!(
            result,
            Err(DivisionError::GcdDoesNotDivide { .. })
        ));
    }

    #[test]
    fn test_gcd_reduction_division_by_zero() {
        let result = mod_div_gcd_reduction(
            &BigInt::from(6),
            &BigInt::from(0),
            &BigInt::from(10),
        );

        assert!(matches!(result, Err(DivisionError::DivisionByZero)));
    }

    #[test]
    fn test_gcd_reduction_invalid_modulus() {
        let result = mod_div_gcd_reduction(
            &BigInt::from(6),
            &BigInt::from(4),
            &BigInt::from(0),
        );

        assert!(matches!(result, Err(DivisionError::InvalidModulus(_))));
    }

    #[test]
    fn test_gcd_reduction_multiple_factors() {
        // 12 / 8 mod 20: gcd(8, 20) = 4, 4|12
        // Reduce: 3 / 2 mod 5
        // 2⁻¹ mod 5 = 3, so 3 × 3 mod 5 = 4
        let result = mod_div_gcd_reduction(
            &BigInt::from(12),
            &BigInt::from(8),
            &BigInt::from(20),
        )
        .unwrap();

        // In reduced ring: 3 / 2 mod 5 = 4
        assert_eq!(result.residue, BigInt::from(4));
    }

    #[test]
    fn test_enumerate_solutions() {
        // 6 / 4 mod 10: gcd = 2, so 2 solutions
        let solutions = enumerate_solutions(
            &BigInt::from(6),
            &BigInt::from(4),
            &BigInt::from(10),
        )
        .unwrap();

        assert_eq!(solutions.len(), 2);

        // Both solutions should satisfy: 4x ≡ 6 (mod 10)
        for x in &solutions {
            let check = (BigInt::from(4) * x) % BigInt::from(10);
            assert_eq!(check, BigInt::from(6));
        }
    }

    #[test]
    fn test_enumerate_solutions_unique() {
        // 10 / 3 mod 7: gcd = 1, so 1 solution
        let solutions = enumerate_solutions(
            &BigInt::from(10),
            &BigInt::from(3),
            &BigInt::from(7),
        )
        .unwrap();

        assert_eq!(solutions.len(), 1);
    }

    #[test]
    fn test_try_gcd_reduction() {
        // Should succeed
        let result = try_gcd_reduction(
            &BigInt::from(6),
            &BigInt::from(4),
            &BigInt::from(10),
        );
        assert!(result.is_some());

        // Should fail (gcd doesn't divide)
        let result = try_gcd_reduction(
            &BigInt::from(7),
            &BigInt::from(4),
            &BigInt::from(10),
        );
        assert!(result.is_none());
    }
}

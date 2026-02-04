//! CRT Tower Reconstruction
//!
//! INNOVATION: CRTBigInt Parallel reconstruction
//!
//! Reconstructs values from multiple anchor residues using the
//! Chinese Remainder Theorem. Includes optimized bi-anchor variant.
//!
//! Mathematical Basis (Bi-Anchor CRT Recovery Theorem):
//! Let M be base modulus, {M₁, M₂} be anchors with:
//! - gcd(M₁, M₂) = 1
//! - gcd(M₁·M₂, M) coprime or M | M₁·M₂
//! Then: Any value x computable in M₁ or M₂ can be reconstructed
//! via affine lifting.

use crate::binary_gcd::binary_gcd_bigint;
use crate::error::{DivisionError, DivisionResult};
use crate::mod_inverse::mod_inverse;
use crate::mod_residue::ModResidue;
use num_bigint::BigInt;
use num_traits::{Zero, One};

/// Residue-modulus pair for CRT reconstruction
#[derive(Debug, Clone)]
pub struct CRTResidue {
    /// The residue value
    pub residue: BigInt,
    /// The modulus
    pub modulus: BigInt,
}

impl CRTResidue {
    pub fn new(residue: BigInt, modulus: BigInt) -> Self {
        Self { residue, modulus }
    }
}

/// CRT reconstruction from multiple residues
///
/// Given residues x_i mod M_i for pairwise coprime M_i,
/// compute unique x mod (∏M_i).
///
/// # Algorithm (Garner's Algorithm variant)
/// For each i:
///   M_i = ∏_{j<i} m_j (partial product)
///   c_i = (r_i - x_{i-1}) × M_{i-1}^{-1} mod m_i
///   x_i = x_{i-1} + c_i × M_{i-1}
///
/// # Arguments
/// * `residues` - Vector of (residue, modulus) pairs, must be pairwise coprime
pub fn crt_reconstruct(residues: &[(BigInt, BigInt)]) -> DivisionResult<BigInt> {
    if residues.is_empty() {
        return Err(DivisionError::crt_failed("empty residue list"));
    }

    if residues.len() == 1 {
        return Ok(residues[0].0.clone());
    }

    // Validate pairwise coprimality
    for i in 0..residues.len() {
        for j in (i + 1)..residues.len() {
            if binary_gcd_bigint(&residues[i].1, &residues[j].1) != BigInt::one() {
                return Err(DivisionError::crt_failed(format!(
                    "moduli {} and {} are not coprime",
                    residues[i].1, residues[j].1
                )));
            }
        }
    }

    // Compute product of all moduli
    let product: BigInt = residues.iter().map(|(_, m)| m).fold(BigInt::one(), |a, b| a * b);

    // CRT formula: x = Σ(r_i × N_i × y_i) where N_i = product/m_i and y_i = N_i^(-1) mod m_i
    let mut result = BigInt::zero();

    for (r_i, m_i) in residues {
        let n_i = &product / m_i;
        let y_i = mod_inverse(&n_i, m_i).ok_or_else(|| {
            DivisionError::crt_failed(format!("no inverse for {} mod {}", n_i, m_i))
        })?;

        result += r_i * &n_i * &y_i;
    }

    Ok(result % &product)
}

/// CRT reconstruction from CRTResidue structs
pub fn crt_reconstruct_from_residues(residues: &[CRTResidue]) -> DivisionResult<BigInt> {
    let pairs: Vec<_> = residues
        .iter()
        .map(|r| (r.residue.clone(), r.modulus.clone()))
        .collect();
    crt_reconstruct(&pairs)
}

/// Bi-anchor CRT recovery (optimized for 2 anchors)
///
/// Much faster than general CRT for the common 2-anchor case.
///
/// # Formula
/// x = r1 + m1 × ((r2 - r1) × m1^(-1) mod m2)
pub fn bi_anchor_reconstruct(
    r1: &BigInt,
    m1: &BigInt,
    r2: &BigInt,
    m2: &BigInt,
) -> DivisionResult<BigInt> {
    // Verify coprimality
    if binary_gcd_bigint(m1, m2) != BigInt::one() {
        return Err(DivisionError::crt_failed("anchors not coprime"));
    }

    // Compute m1^(-1) mod m2
    let m1_inv = mod_inverse(m1, m2).ok_or_else(|| {
        DivisionError::crt_failed(format!("no inverse for {} mod {}", m1, m2))
    })?;

    // diff = (r2 - r1) mod m2 (ensure positive)
    let diff = {
        let d = (r2 - r1) % m2;
        if d < BigInt::zero() {
            d + m2
        } else {
            d
        }
    };

    // k = diff × m1^(-1) mod m2
    let k = (&diff * &m1_inv) % m2;

    // x = r1 + m1 × k
    Ok(r1 + m1 * &k)
}

/// Reconstruct ModResidue from multiple promoted results
///
/// Takes results computed in different anchor rings and
/// reconstructs a value valid in a larger ring (their product).
pub fn reconstruct_from_promoted(results: &[ModResidue]) -> DivisionResult<ModResidue> {
    if results.is_empty() {
        return Err(DivisionError::crt_failed("no results to reconstruct"));
    }

    if results.len() == 1 {
        return Ok(results[0].clone());
    }

    // All results should have the same base_mod
    let base_mod = &results[0].base_mod;
    for r in results {
        if &r.base_mod != base_mod {
            return Err(DivisionError::crt_failed(
                "inconsistent base moduli in results",
            ));
        }
    }

    // Collect residue-modulus pairs
    let pairs: Vec<_> = results
        .iter()
        .map(|r| (r.residue.clone(), r.current_mod.clone()))
        .collect();

    // Reconstruct
    let reconstructed = crt_reconstruct(&pairs)?;

    // Compute product modulus
    let product_mod: BigInt = results
        .iter()
        .map(|r| &r.current_mod)
        .fold(BigInt::one(), |a, b| a * b);

    Ok(ModResidue::crt(
        reconstructed,
        base_mod.clone(),
        product_mod,
        results.len(),
    ))
}

/// Reconstruct from bi-anchor results
pub fn reconstruct_from_bi_anchor(
    result1: &ModResidue,
    result2: &ModResidue,
) -> DivisionResult<ModResidue> {
    // Verify same base modulus
    if result1.base_mod != result2.base_mod {
        return Err(DivisionError::crt_failed("inconsistent base moduli"));
    }

    let reconstructed = bi_anchor_reconstruct(
        &result1.residue,
        &result1.current_mod,
        &result2.residue,
        &result2.current_mod,
    )?;

    let product_mod = &result1.current_mod * &result2.current_mod;

    Ok(ModResidue::crt(
        reconstructed,
        result1.base_mod.clone(),
        product_mod,
        2,
    ))
}

/// Project reconstructed value back to base ring
///
/// Takes a CRT-reconstructed value and projects it to the original
/// base ring by taking the residue mod base_mod.
pub fn project_to_base(result: &ModResidue) -> BigInt {
    result.project_to_base()
}

/// Incremental CRT reconstruction
///
/// Useful for reconstructing one residue at a time without
/// keeping all residues in memory.
#[derive(Debug, Clone)]
pub struct IncrementalCRT {
    /// Current reconstructed value
    value: BigInt,
    /// Current modulus (product of all seen moduli)
    modulus: BigInt,
    /// Number of residues incorporated
    count: usize,
}

impl IncrementalCRT {
    /// Create new incremental CRT with first residue
    pub fn new(residue: BigInt, modulus: BigInt) -> Self {
        Self {
            value: residue,
            modulus,
            count: 1,
        }
    }

    /// Add another residue
    pub fn add(&mut self, residue: &BigInt, modulus: &BigInt) -> DivisionResult<()> {
        if binary_gcd_bigint(&self.modulus, modulus) != BigInt::one() {
            return Err(DivisionError::crt_failed("new modulus not coprime"));
        }

        // Use bi-anchor formula
        let m1_inv = mod_inverse(&self.modulus, modulus).ok_or_else(|| {
            DivisionError::crt_failed("inverse computation failed")
        })?;

        let diff = {
            let d = (residue - &self.value) % modulus;
            if d < BigInt::zero() {
                d + modulus
            } else {
                d
            }
        };

        let k = (&diff * &m1_inv) % modulus;
        self.value = &self.value + &self.modulus * &k;
        self.modulus = &self.modulus * modulus;
        self.count += 1;

        Ok(())
    }

    /// Get current value
    pub fn value(&self) -> &BigInt {
        &self.value
    }

    /// Get current modulus
    pub fn modulus(&self) -> &BigInt {
        &self.modulus
    }

    /// Get number of residues incorporated
    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crt_basic() {
        // x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7)
        // x = 23 (verified: 23 mod 3 = 2, 23 mod 5 = 3, 23 mod 7 = 2)
        let residues = vec![
            (BigInt::from(2), BigInt::from(3)),
            (BigInt::from(3), BigInt::from(5)),
            (BigInt::from(2), BigInt::from(7)),
        ];

        let result = crt_reconstruct(&residues).unwrap();
        assert_eq!(result, BigInt::from(23));
    }

    #[test]
    fn test_crt_two_moduli() {
        // x ≡ 2 (mod 3), x ≡ 3 (mod 5)
        // x = 8 (8 mod 3 = 2, 8 mod 5 = 3)
        let residues = vec![
            (BigInt::from(2), BigInt::from(3)),
            (BigInt::from(3), BigInt::from(5)),
        ];

        let result = crt_reconstruct(&residues).unwrap();
        assert_eq!(result, BigInt::from(8));
    }

    #[test]
    fn test_crt_single_residue() {
        let residues = vec![(BigInt::from(5), BigInt::from(7))];
        let result = crt_reconstruct(&residues).unwrap();
        assert_eq!(result, BigInt::from(5));
    }

    #[test]
    fn test_crt_empty() {
        let residues: Vec<(BigInt, BigInt)> = vec![];
        let result = crt_reconstruct(&residues);
        assert!(matches!(result, Err(DivisionError::CRTReconstructionFailed { .. })));
    }

    #[test]
    fn test_crt_not_coprime() {
        let residues = vec![
            (BigInt::from(2), BigInt::from(6)),
            (BigInt::from(3), BigInt::from(9)), // gcd(6, 9) = 3
        ];

        let result = crt_reconstruct(&residues);
        assert!(matches!(result, Err(DivisionError::CRTReconstructionFailed { .. })));
    }

    #[test]
    fn test_bi_anchor_basic() {
        // x ≡ 2 (mod 3), x ≡ 3 (mod 5)
        // x = 8
        let result = bi_anchor_reconstruct(
            &BigInt::from(2),
            &BigInt::from(3),
            &BigInt::from(3),
            &BigInt::from(5),
        )
        .unwrap();

        assert_eq!(result, BigInt::from(8));
    }

    #[test]
    fn test_bi_anchor_large() {
        let m1 = BigInt::from(1_000_000_007u64);
        let m2 = BigInt::from(1_000_000_009u64);
        let r1 = BigInt::from(123456789u64);
        let r2 = BigInt::from(987654321u64);

        let result = bi_anchor_reconstruct(&r1, &m1, &r2, &m2).unwrap();

        // Verify
        assert_eq!(&result % &m1, r1);
        assert_eq!(&result % &m2, r2);
    }

    #[test]
    fn test_crt_round_trip() {
        let original = BigInt::from(12345);
        let moduli = vec![
            BigInt::from(7),
            BigInt::from(11),
            BigInt::from(13),
        ];

        let residues: Vec<_> = moduli
            .iter()
            .map(|m| (&original % m, m.clone()))
            .collect();

        let reconstructed = crt_reconstruct(&residues).unwrap();

        // reconstructed should equal original mod product
        let product: BigInt = moduli.iter().fold(BigInt::one(), |a, b| a * b);
        assert_eq!(&reconstructed % &product, &original % &product);
    }

    #[test]
    fn test_reconstruct_from_promoted() {
        let base_mod = BigInt::from(15);
        let results = vec![
            ModResidue::promoted(
                BigInt::from(2),
                base_mod.clone(),
                BigInt::from(7),
                0,
            ),
            ModResidue::promoted(
                BigInt::from(3),
                base_mod.clone(),
                BigInt::from(11),
                1,
            ),
        ];

        let reconstructed = reconstruct_from_promoted(&results).unwrap();

        // Verify residues match in original anchor rings
        assert_eq!(&reconstructed.residue % BigInt::from(7), BigInt::from(2));
        assert_eq!(&reconstructed.residue % BigInt::from(11), BigInt::from(3));
    }

    #[test]
    fn test_reconstruct_from_bi_anchor() {
        let base_mod = BigInt::from(15);
        let result1 = ModResidue::promoted(
            BigInt::from(2),
            base_mod.clone(),
            BigInt::from(7),
            0,
        );
        let result2 = ModResidue::promoted(
            BigInt::from(3),
            base_mod.clone(),
            BigInt::from(11),
            1,
        );

        let reconstructed = reconstruct_from_bi_anchor(&result1, &result2).unwrap();

        assert_eq!(&reconstructed.residue % BigInt::from(7), BigInt::from(2));
        assert_eq!(&reconstructed.residue % BigInt::from(11), BigInt::from(3));
    }

    #[test]
    fn test_incremental_crt() {
        let mut crt = IncrementalCRT::new(BigInt::from(2), BigInt::from(3));

        crt.add(&BigInt::from(3), &BigInt::from(5)).unwrap();
        assert_eq!(crt.value(), &BigInt::from(8));
        assert_eq!(crt.count(), 2);

        crt.add(&BigInt::from(2), &BigInt::from(7)).unwrap();
        assert_eq!(crt.value(), &BigInt::from(23));
        assert_eq!(crt.count(), 3);
    }

    #[test]
    fn test_incremental_crt_not_coprime() {
        let mut crt = IncrementalCRT::new(BigInt::from(2), BigInt::from(6));
        let result = crt.add(&BigInt::from(3), &BigInt::from(9));
        assert!(result.is_err());
    }
}

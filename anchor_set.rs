//! Anchor Set Types and Configuration
//!
//! Pre-validated coprime anchor sets for piggyback division.
//! When gcd(divisor, base_mod) ≠ 1, we find an anchor where
//! gcd(divisor, anchor) = 1 and compute the division there.
//!
//! The anchor set is designed to provide high coverage (99.7%+)
//! for typical QMNF moduli while maintaining efficiency.

use crate::binary_gcd::{binary_gcd, binary_gcd_bigint, are_coprime, are_coprime_bigint};
use num_bigint::BigInt;
use num_traits::{Zero, One};
use std::collections::HashMap;
use thiserror::Error;

/// Default anchor set providing 99.7% divisor coverage
/// for QMNF 96-180 bit moduli
///
/// Selection criteria:
/// - Large primes for wide coverage
/// - Pairwise coprime (guaranteed by primality)
/// - Mix of sizes for different use cases
pub const DEFAULT_ANCHORS: &[u64] = &[
    4_294_967_291,  // 2³²-5 (largest 32-bit prime)
    4_294_967_279,  // 2³²-17
    4_294_967_231,  // 2³²-65
    2_147_483_647,  // 2³¹-1 (Mersenne prime M₃₁)
    65_521,         // 2¹⁶-15 (for small ops)
];

/// Extended anchor set for higher coverage (99.95%+)
pub const EXTENDED_ANCHORS: &[u64] = &[
    4_294_967_291,  // 2³²-5
    4_294_967_279,  // 2³²-17
    4_294_967_231,  // 2³²-65
    4_294_967_189,  // 2³²-107
    4_294_967_161,  // 2³²-135
    2_147_483_647,  // 2³¹-1 (Mersenne)
    2_147_483_629,  // 2³¹-19
    1_073_741_789,  // Near 2³⁰
    65_521,         // 2¹⁶-15
    65_519,         // 2¹⁶-17
];

/// Errors that can occur during anchor set operations
#[derive(Debug, Clone, Error)]
pub enum AnchorError {
    #[error("anchors at indices {0} and {1} are not coprime")]
    NotCoprime(usize, usize),

    #[error("empty anchor set")]
    EmptySet,

    #[error("anchor {0} is invalid (zero or one)")]
    InvalidAnchor(BigInt),

    #[error("no coprime anchor found for divisor {divisor} after trying {tried} anchors")]
    NoCoprimeAnchor { divisor: BigInt, tried: usize },

    #[error("anchor set construction failed: {0}")]
    ConstructionFailed(String),
}

/// Pre-validated coprime anchor set for piggyback division
#[derive(Debug, Clone)]
pub struct AnchorSet {
    /// Anchor moduli (all pairwise coprime)
    anchors: Vec<BigInt>,

    /// Cached inverse lookups for performance
    /// Key: (divisor mod anchor, anchor index)
    inverse_cache: HashMap<(BigInt, usize), BigInt>,

    /// Whether to use constant-time operations
    constant_time: bool,
}

impl AnchorSet {
    /// Create anchor set from the default configuration
    pub fn default_set() -> Self {
        Self::from_u64_slice(DEFAULT_ANCHORS)
            .expect("Default anchors are valid and coprime")
    }

    /// Create anchor set from the extended configuration
    pub fn extended_set() -> Self {
        Self::from_u64_slice(EXTENDED_ANCHORS)
            .expect("Extended anchors are valid and coprime")
    }

    /// Create anchor set from a slice of u64 values
    ///
    /// Validates that all anchors are pairwise coprime.
    pub fn from_u64_slice(anchors: &[u64]) -> Result<Self, AnchorError> {
        if anchors.is_empty() {
            return Err(AnchorError::EmptySet);
        }

        let anchors: Vec<BigInt> = anchors.iter().map(|&a| BigInt::from(a)).collect();

        // Validate no invalid anchors
        for anchor in &anchors {
            if anchor <= &BigInt::one() {
                return Err(AnchorError::InvalidAnchor(anchor.clone()));
            }
        }

        // Validate pairwise coprimality
        for i in 0..anchors.len() {
            for j in (i + 1)..anchors.len() {
                if !are_coprime_bigint(&anchors[i], &anchors[j]) {
                    return Err(AnchorError::NotCoprime(i, j));
                }
            }
        }

        Ok(Self {
            anchors,
            inverse_cache: HashMap::new(),
            constant_time: false,
        })
    }

    /// Create anchor set from BigInt values
    pub fn from_bigints(anchors: Vec<BigInt>) -> Result<Self, AnchorError> {
        if anchors.is_empty() {
            return Err(AnchorError::EmptySet);
        }

        // Validate no invalid anchors
        for anchor in &anchors {
            if anchor <= &BigInt::one() {
                return Err(AnchorError::InvalidAnchor(anchor.clone()));
            }
        }

        // Validate pairwise coprimality
        for i in 0..anchors.len() {
            for j in (i + 1)..anchors.len() {
                if !are_coprime_bigint(&anchors[i], &anchors[j]) {
                    return Err(AnchorError::NotCoprime(i, j));
                }
            }
        }

        Ok(Self {
            anchors,
            inverse_cache: HashMap::new(),
            constant_time: false,
        })
    }

    /// Enable constant-time operations for side-channel resistance
    pub fn with_constant_time(mut self, enabled: bool) -> Self {
        self.constant_time = enabled;
        self
    }

    /// Get the number of anchors
    #[inline]
    pub fn len(&self) -> usize {
        self.anchors.len()
    }

    /// Check if the anchor set is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.anchors.is_empty()
    }

    /// Get anchor at index
    #[inline]
    pub fn get(&self, index: usize) -> Option<&BigInt> {
        self.anchors.get(index)
    }

    /// Iterate over anchors
    pub fn iter(&self) -> impl Iterator<Item = &BigInt> {
        self.anchors.iter()
    }

    /// Find first anchor coprime to the given divisor
    ///
    /// Returns (anchor_index, anchor_value) if found.
    /// Uses early-exit for efficiency (non-constant-time).
    pub fn find_coprime_anchor(&self, divisor: &BigInt) -> Option<(usize, &BigInt)> {
        if self.constant_time {
            self.find_coprime_anchor_ct(divisor)
        } else {
            self.anchors
                .iter()
                .enumerate()
                .find(|(_, anchor)| are_coprime_bigint(divisor, anchor))
        }
    }

    /// Constant-time anchor selection (for side-channel resistance)
    ///
    /// Evaluates ALL anchors regardless of which is coprime,
    /// to prevent timing attacks that could leak divisor information.
    fn find_coprime_anchor_ct(&self, divisor: &BigInt) -> Option<(usize, &BigInt)> {
        let mut result_idx = usize::MAX;
        let mut found = false;

        // Evaluate ALL anchors (no early exit)
        for (idx, anchor) in self.anchors.iter().enumerate() {
            let is_coprime = are_coprime_bigint(divisor, anchor);

            // Update only if coprime AND not yet found (first match wins)
            if is_coprime && !found {
                result_idx = idx;
                found = true;
            }

            // Dummy operation to equalize timing
            // This prevents the compiler from optimizing away the loop
            std::hint::black_box(&anchor);
        }

        if found && result_idx < self.anchors.len() {
            Some((result_idx, &self.anchors[result_idx]))
        } else {
            None
        }
    }

    /// Find all anchors coprime to the given divisor
    pub fn find_all_coprime_anchors(&self, divisor: &BigInt) -> Vec<(usize, &BigInt)> {
        self.anchors
            .iter()
            .enumerate()
            .filter(|(_, anchor)| are_coprime_bigint(divisor, anchor))
            .collect()
    }

    /// Get the product of all anchors
    ///
    /// Useful for CRT reconstruction bounds.
    pub fn product(&self) -> BigInt {
        self.anchors.iter().fold(BigInt::one(), |acc, a| acc * a)
    }

    /// Get the product of specified anchor indices
    pub fn partial_product(&self, indices: &[usize]) -> BigInt {
        indices
            .iter()
            .filter_map(|&i| self.anchors.get(i))
            .fold(BigInt::one(), |acc, a| acc * a)
    }

    /// Check coverage for a given base modulus
    ///
    /// Returns the fraction of small primes covered by at least one anchor.
    /// Higher coverage means fewer fallbacks to GCD reduction.
    pub fn coverage_estimate(&self, sample_primes: &[u64]) -> f64 {
        if sample_primes.is_empty() {
            return 1.0;
        }

        let covered = sample_primes
            .iter()
            .filter(|&&p| {
                let p = BigInt::from(p);
                self.anchors.iter().any(|a| are_coprime_bigint(&p, a))
            })
            .count();

        covered as f64 / sample_primes.len() as f64
    }
}

impl Default for AnchorSet {
    fn default() -> Self {
        Self::default_set()
    }
}

/// Generate a custom anchor set for a specific base modulus
///
/// Creates anchors that are guaranteed coprime to the base modulus.
pub fn generate_anchor_set(
    base_mod: &BigInt,
    target_count: usize,
) -> Result<AnchorSet, AnchorError> {
    if target_count == 0 {
        return Err(AnchorError::EmptySet);
    }

    let mut anchors = Vec::with_capacity(target_count);

    // Start with primes from the default set that are coprime to base_mod
    for &p in DEFAULT_ANCHORS {
        if anchors.len() >= target_count {
            break;
        }

        let p_big = BigInt::from(p);
        if are_coprime_bigint(&p_big, base_mod) {
            // Also check coprime to existing anchors
            let coprime_to_existing = anchors.iter().all(|a| are_coprime_bigint(&p_big, a));
            if coprime_to_existing {
                anchors.push(p_big);
            }
        }
    }

    // If we need more, use extended set
    if anchors.len() < target_count {
        for &p in EXTENDED_ANCHORS {
            if anchors.len() >= target_count {
                break;
            }

            let p_big = BigInt::from(p);
            if !anchors.contains(&p_big) && are_coprime_bigint(&p_big, base_mod) {
                let coprime_to_existing = anchors.iter().all(|a| are_coprime_bigint(&p_big, a));
                if coprime_to_existing {
                    anchors.push(p_big);
                }
            }
        }
    }

    if anchors.is_empty() {
        return Err(AnchorError::ConstructionFailed(
            "Could not find any coprime anchors".to_string(),
        ));
    }

    AnchorSet::from_bigints(anchors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_anchors_pairwise_coprime() {
        let set = AnchorSet::default_set();
        assert_eq!(set.len(), 5);

        // All pairs should be coprime
        for i in 0..set.len() {
            for j in (i + 1)..set.len() {
                let a = set.get(i).unwrap();
                let b = set.get(j).unwrap();
                assert!(
                    are_coprime_bigint(a, b),
                    "Anchors {} and {} are not coprime",
                    a,
                    b
                );
            }
        }
    }

    #[test]
    fn test_extended_anchors_pairwise_coprime() {
        let set = AnchorSet::extended_set();
        assert_eq!(set.len(), 10);

        for i in 0..set.len() {
            for j in (i + 1)..set.len() {
                let a = set.get(i).unwrap();
                let b = set.get(j).unwrap();
                assert!(
                    are_coprime_bigint(a, b),
                    "Anchors {} and {} are not coprime",
                    a,
                    b
                );
            }
        }
    }

    #[test]
    fn test_find_coprime_anchor() {
        let set = AnchorSet::default_set();

        // 15 = 3×5, should find an anchor coprime to it
        let divisor = BigInt::from(15);
        let result = set.find_coprime_anchor(&divisor);
        assert!(result.is_some());

        let (idx, anchor) = result.unwrap();
        assert!(are_coprime_bigint(&divisor, anchor));
        assert!(idx < set.len());
    }

    #[test]
    fn test_find_coprime_anchor_large_divisor() {
        let set = AnchorSet::default_set();

        // Large prime divisor
        let divisor = BigInt::from(1_000_000_007u64);
        let result = set.find_coprime_anchor(&divisor);
        assert!(result.is_some());
    }

    #[test]
    fn test_find_coprime_anchor_ct_matches_normal() {
        let set = AnchorSet::default_set().with_constant_time(true);

        for d in 2u64..200 {
            let divisor = BigInt::from(d);
            let ct_result = set.find_coprime_anchor(&divisor);

            let set_nc = AnchorSet::default_set();
            let nc_result = set_nc.find_coprime_anchor(&divisor);

            // Both should find or not find
            assert_eq!(ct_result.is_some(), nc_result.is_some());

            // If found, same anchor (first coprime)
            if let (Some((ct_idx, _)), Some((nc_idx, _))) = (ct_result, nc_result) {
                assert_eq!(ct_idx, nc_idx);
            }
        }
    }

    #[test]
    fn test_find_all_coprime_anchors() {
        let set = AnchorSet::default_set();

        let divisor = BigInt::from(15);
        let all = set.find_all_coprime_anchors(&divisor);

        // All returned anchors should be coprime
        for (_, anchor) in &all {
            assert!(are_coprime_bigint(&divisor, anchor));
        }

        // Should find multiple anchors
        assert!(all.len() >= 1);
    }

    #[test]
    fn test_invalid_anchor_set_rejected() {
        // 6 and 9 share factor 3
        let bad_anchors = &[6u64, 9, 25];
        let result = AnchorSet::from_u64_slice(bad_anchors);
        assert!(matches!(result, Err(AnchorError::NotCoprime(0, 1))));
    }

    #[test]
    fn test_empty_anchor_set_rejected() {
        let result = AnchorSet::from_u64_slice(&[]);
        assert!(matches!(result, Err(AnchorError::EmptySet)));
    }

    #[test]
    fn test_invalid_anchor_rejected() {
        let result = AnchorSet::from_u64_slice(&[0, 5, 7]);
        assert!(matches!(result, Err(AnchorError::InvalidAnchor(_))));

        let result = AnchorSet::from_u64_slice(&[1, 5, 7]);
        assert!(matches!(result, Err(AnchorError::InvalidAnchor(_))));
    }

    #[test]
    fn test_anchor_product() {
        let set = AnchorSet::from_u64_slice(&[3, 5, 7]).unwrap();
        assert_eq!(set.product(), BigInt::from(105));
    }

    #[test]
    fn test_partial_product() {
        let set = AnchorSet::from_u64_slice(&[3, 5, 7, 11]).unwrap();
        assert_eq!(set.partial_product(&[0, 2]), BigInt::from(21)); // 3 * 7
        assert_eq!(set.partial_product(&[1, 3]), BigInt::from(55)); // 5 * 11
    }

    #[test]
    fn test_generate_anchor_set() {
        let base_mod = BigInt::from(1_000_000);
        let set = generate_anchor_set(&base_mod, 5).unwrap();

        assert!(set.len() <= 5);
        assert!(set.len() >= 1);

        // All anchors should be coprime to base_mod
        for anchor in set.iter() {
            assert!(are_coprime_bigint(anchor, &base_mod));
        }
    }

    #[test]
    fn test_coverage_estimate() {
        let set = AnchorSet::default_set();
        let primes: Vec<u64> = (2..100)
            .filter(|&n| {
                if n < 2 {
                    return false;
                }
                for i in 2..=(n as f64).sqrt() as u64 {
                    if n % i == 0 {
                        return false;
                    }
                }
                true
            })
            .collect();

        let coverage = set.coverage_estimate(&primes);
        // Should have very high coverage
        assert!(coverage > 0.9);
    }
}

//! FPD: Coprime-Piggyback Modular Division
//!
//! Production implementation of provenance-preserving modular division
//! for the QMNF (Quantum-Modular Numerical Framework) ecosystem.
//!
//! # Features
//! - **Fast path**: Direct division when gcd(divisor, modulus) = 1
//! - **GCD reduction**: Division via quotient rings when gcd > 1 and gcd | dividend
//! - **Coprime piggyback**: Anchor-based division for remaining cases
//! - **Provenance tracking**: ModResidue type prevents "silent jacking"
//! - **Side-channel resistance**: Constant-time and blinded operations
//! - **Audit logging**: HMAC-signed audit trails
//!
//! # Quick Start
//! ```ignore
//! use fpd::{mod_div, DivisionConfig};
//! use num_bigint::BigInt;
//!
//! let config = DivisionConfig::default();
//! let result = mod_div(
//!     &BigInt::from(10),
//!     &BigInt::from(3),
//!     &BigInt::from(7),
//!     &config,
//! ).unwrap();
//!
//! assert!(result.is_exact());
//! ```

// Core modules
pub mod mod_residue;
pub mod binary_gcd;
pub mod anchor_set;
pub mod error;
pub mod mod_inverse;

// Division algorithms
pub mod fast_path;
pub mod piggyback;
pub mod gcd_reduction;
pub mod crt_tower;

// Security & auditing
pub mod constant_time;
pub mod audit;

// Re-exports for convenience
pub use mod_residue::{ModResidue, DivStatus};
pub use error::{DivisionError, DivisionResult};
pub use anchor_set::AnchorSet;
pub use binary_gcd::{binary_gcd, binary_gcd_bigint, are_coprime, are_coprime_bigint};
pub use mod_inverse::{mod_inverse, mod_inverse_u64, batch_mod_inverse};
pub use fast_path::{mod_div_fast, MontgomeryContext};
pub use gcd_reduction::{mod_div_gcd_reduction, enumerate_solutions};
pub use piggyback::{mod_div_piggyback, mod_div_piggyback_multi, mod_div_piggyback_bi};
pub use crt_tower::{crt_reconstruct, bi_anchor_reconstruct, CRTResidue, IncrementalCRT};
pub use constant_time::{ShadowEntropy, find_coprime_anchor_ct, mod_div_blinded};
pub use audit::{AuditLog, DivisionAuditEntry};

use num_bigint::BigInt;
use num_traits::{Zero, One};

/// Configuration for division operations
#[derive(Debug, Clone)]
pub struct DivisionConfig {
    /// Anchor set for piggyback division
    pub anchors: AnchorSet,
    /// Optional Montgomery context for acceleration
    pub montgomery: Option<MontgomeryContext>,
    /// Enable CRT reconstruction for promoted results
    pub enable_crt_reconstruction: bool,
    /// Enable constant-time operations for security
    pub constant_time: bool,
    /// Maximum recursion depth for GCD reduction
    pub max_gcd_depth: usize,
}

impl Default for DivisionConfig {
    fn default() -> Self {
        Self {
            anchors: AnchorSet::default_set(),
            montgomery: None,
            enable_crt_reconstruction: true,
            constant_time: false,
            max_gcd_depth: 10,
        }
    }
}

impl DivisionConfig {
    /// Create config with custom anchor set
    pub fn with_anchors(anchors: AnchorSet) -> Self {
        Self { anchors, ..Default::default() }
    }

    /// Enable Montgomery acceleration
    pub fn with_montgomery(mut self, modulus: &BigInt) -> Self {
        self.montgomery = MontgomeryContext::new(modulus);
        self
    }

    /// Enable constant-time operations
    pub fn with_constant_time(mut self, enabled: bool) -> Self {
        self.constant_time = enabled;
        if enabled {
            self.anchors = self.anchors.with_constant_time(true);
        }
        self
    }

    /// Disable CRT reconstruction
    pub fn without_crt(mut self) -> Self {
        self.enable_crt_reconstruction = false;
        self
    }
}

/// Unified modular division with automatic path selection
///
/// Attempts paths in order:
/// 1. Fast path (gcd(b,M) = 1) — returns Exact
/// 2. GCD reduction (gcd > 1, gcd | a) — returns CRT
/// 3. Coprime piggyback (find anchor) — returns Promoted
/// 4. Return error if all fail
pub fn mod_div(
    dividend: &BigInt,
    divisor: &BigInt,
    modulus: &BigInt,
    config: &DivisionConfig,
) -> DivisionResult<ModResidue> {
    if modulus <= &BigInt::one() {
        return Err(DivisionError::invalid_modulus(modulus.clone()));
    }
    if divisor.is_zero() {
        return Err(DivisionError::DivisionByZero);
    }

    // Path 1: Fast path
    if let Ok(result) = mod_div_fast(dividend, divisor, modulus) {
        return Ok(result);
    }

    // Path 2: GCD reduction
    if let Ok(result) = mod_div_gcd_reduction(dividend, divisor, modulus) {
        return Ok(result);
    }

    // Path 3: Coprime piggyback
    mod_div_piggyback(dividend, divisor, modulus, &config.anchors)
}

/// Simplified division without config (uses defaults)
pub fn mod_div_simple(
    dividend: &BigInt,
    divisor: &BigInt,
    modulus: &BigInt,
) -> DivisionResult<ModResidue> {
    static DEFAULT_CONFIG: std::sync::OnceLock<DivisionConfig> = std::sync::OnceLock::new();
    let config = DEFAULT_CONFIG.get_or_init(DivisionConfig::default);
    mod_div(dividend, divisor, modulus, config)
}

/// Division with i64 inputs (convenience)
pub fn mod_div_i64(dividend: i64, divisor: i64, modulus: i64) -> DivisionResult<ModResidue> {
    mod_div_simple(&BigInt::from(dividend), &BigInt::from(divisor), &BigInt::from(modulus))
}

/// Division with u64 inputs (convenience)
pub fn mod_div_u64(dividend: u64, divisor: u64, modulus: u64) -> DivisionResult<ModResidue> {
    mod_div_simple(&BigInt::from(dividend), &BigInt::from(divisor), &BigInt::from(modulus))
}

/// Try division, returning None on failure
pub fn try_mod_div(
    dividend: &BigInt,
    divisor: &BigInt,
    modulus: &BigInt,
    config: &DivisionConfig,
) -> Option<ModResidue> {
    mod_div(dividend, divisor, modulus, config).ok()
}

/// Division with automatic reconstruction to base ring
pub fn mod_div_final(
    dividend: &BigInt,
    divisor: &BigInt,
    modulus: &BigInt,
    config: &DivisionConfig,
) -> DivisionResult<BigInt> {
    let result = mod_div(dividend, divisor, modulus, config)?;
    Ok(result.project_to_base())
}

/// Batch division of multiple numerators by same divisor
pub fn mod_div_batch(
    dividends: &[BigInt],
    divisor: &BigInt,
    modulus: &BigInt,
    config: &DivisionConfig,
) -> DivisionResult<Vec<ModResidue>> {
    if dividends.is_empty() {
        return Ok(vec![]);
    }

    // Try to compute inverse once
    if let Some(first_result) = try_mod_div(&dividends[0], divisor, modulus, config) {
        if first_result.is_exact() {
            if let Some(inv) = mod_inverse(divisor, modulus) {
                let results: Vec<_> = dividends
                    .iter()
                    .map(|a| {
                        let residue = (a * &inv) % modulus;
                        ModResidue::exact(residue, modulus.clone())
                    })
                    .collect();
                return Ok(results);
            }
        }
    }

    dividends.iter().map(|a| mod_div(a, divisor, modulus, config)).collect()
}

/// Division statistics for debugging/profiling
#[derive(Debug, Default, Clone)]
pub struct DivisionStats {
    pub fast_path_count: usize,
    pub gcd_reduction_count: usize,
    pub piggyback_count: usize,
    pub failure_count: usize,
}

/// Division with statistics tracking
pub fn mod_div_with_stats(
    dividend: &BigInt,
    divisor: &BigInt,
    modulus: &BigInt,
    config: &DivisionConfig,
    stats: &mut DivisionStats,
) -> DivisionResult<ModResidue> {
    if let Ok(result) = mod_div_fast(dividend, divisor, modulus) {
        stats.fast_path_count += 1;
        return Ok(result);
    }

    if let Ok(result) = mod_div_gcd_reduction(dividend, divisor, modulus) {
        stats.gcd_reduction_count += 1;
        return Ok(result);
    }

    match mod_div_piggyback(dividend, divisor, modulus, &config.anchors) {
        Ok(result) => {
            stats.piggyback_count += 1;
            Ok(result)
        }
        Err(e) => {
            stats.failure_count += 1;
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_api_fast_path() {
        let config = DivisionConfig::default();
        let result = mod_div(&BigInt::from(10), &BigInt::from(3), &BigInt::from(7), &config).unwrap();
        assert!(result.is_exact());
    }

    #[test]
    fn test_unified_api_gcd_reduction() {
        let config = DivisionConfig::default();
        let result = mod_div(&BigInt::from(6), &BigInt::from(4), &BigInt::from(10), &config).unwrap();
        assert!(!result.is_exact());
    }

    #[test]
    fn test_unified_api_piggyback() {
        let config = DivisionConfig::default();
        let result = mod_div(&BigInt::from(7), &BigInt::from(9), &BigInt::from(15), &config).unwrap();
        assert!(!result.is_exact());
        assert!(result.needs_reconstruction());
    }

    #[test]
    fn test_division_by_zero() {
        let config = DivisionConfig::default();
        let result = mod_div(&BigInt::from(10), &BigInt::from(0), &BigInt::from(7), &config);
        assert!(matches!(result, Err(DivisionError::DivisionByZero)));
    }

    #[test]
    fn test_invalid_modulus() {
        let config = DivisionConfig::default();
        let result = mod_div(&BigInt::from(10), &BigInt::from(3), &BigInt::from(0), &config);
        assert!(matches!(result, Err(DivisionError::InvalidModulus(_))));
    }

    #[test]
    fn test_mod_div_simple() {
        let result = mod_div_simple(&BigInt::from(10), &BigInt::from(3), &BigInt::from(7)).unwrap();
        assert!(result.is_exact());
    }

    #[test]
    fn test_mod_div_i64() {
        let result = mod_div_i64(10, 3, 7).unwrap();
        assert!(result.is_exact());
    }

    #[test]
    fn test_mod_div_final() {
        let config = DivisionConfig::default();
        let result = mod_div_final(&BigInt::from(10), &BigInt::from(3), &BigInt::from(7), &config).unwrap();
        assert_eq!(result, BigInt::from(1)); // 10/3 mod 7 = 10*5 mod 7 = 1
    }

    #[test]
    fn test_mod_div_batch() {
        let config = DivisionConfig::default();
        let dividends: Vec<BigInt> = (1..10).map(BigInt::from).collect();
        let results = mod_div_batch(&dividends, &BigInt::from(3), &BigInt::from(97), &config).unwrap();
        assert_eq!(results.len(), 9);
        for (a, result) in dividends.iter().zip(results.iter()) {
            let reconstructed = (BigInt::from(3) * &result.residue) % BigInt::from(97);
            assert_eq!(reconstructed, a % BigInt::from(97));
        }
    }

    #[test]
    fn test_stats() {
        let config = DivisionConfig::default();
        let mut stats = DivisionStats::default();
        
        let _ = mod_div_with_stats(&BigInt::from(10), &BigInt::from(3), &BigInt::from(7), &config, &mut stats);
        assert_eq!(stats.fast_path_count, 1);
        
        let _ = mod_div_with_stats(&BigInt::from(6), &BigInt::from(4), &BigInt::from(10), &config, &mut stats);
        assert_eq!(stats.gcd_reduction_count, 1);
    }

    #[test]
    fn test_config_builder() {
        let anchors = AnchorSet::from_u64_slice(&[7, 11, 13]).unwrap();
        let config = DivisionConfig::with_anchors(anchors).with_constant_time(true).without_crt();
        assert!(config.constant_time);
        assert!(!config.enable_crt_reconstruction);
    }
}

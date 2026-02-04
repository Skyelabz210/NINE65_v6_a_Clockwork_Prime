//! ModResidue Type Definition
//! 
//! Provenance-preserving modular residue type that tracks:
//! - The computed value (residue)
//! - The caller's intended ring (base_mod)
//! - The actual computation ring (current_mod)
//! - The division status (Exact, Promoted, CRT, NotInvertible)
//! 
//! This prevents "silent jacking" where a value computed in one
//! ring is accidentally used in another without reconstruction.

use num_bigint::BigInt;
use num_traits::{Zero, One};

/// Division status tracking for provenance preservation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivStatus {
    /// Exact: result computed in original ring (base_mod == current_mod)
    /// Use directly without reconstruction
    Exact,
    
    /// Promoted: computed in coprime anchor ring
    /// Needs CRT reconstruction before use in base ring
    Promoted { 
        /// Index of anchor used
        anchor_index: usize 
    },
    
    /// CRT: reconstructed from multiple anchors
    /// May need projection back to base ring
    CRT { 
        /// Number of anchors combined
        anchor_count: usize 
    },
    
    /// NotInvertible: division failed (no inverse found)
    /// Cannot be used - indicates error
    NotInvertible,
}

/// Provenance-preserving modular residue
/// 
/// Tracks both the computed value AND where it was computed,
/// enabling safe ring transitions and audit trails.
/// 
/// # Example
/// ```
/// let result = mod_div(10, 3, 15, &config);
/// if result.is_exact() {
///     // Use directly
///     use_value(&result.residue);
/// } else if result.needs_reconstruction() {
///     // Reconstruct first
///     let value = reconstruct(&result, &config);
///     use_value(&value);
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModResidue {
    /// Computed value in current ring
    pub residue: BigInt,
    
    /// Caller's intended ring (original modulus)
    /// This is where the result should eventually be used
    pub base_mod: BigInt,
    
    /// Actual computation ring (may differ if promoted)
    /// The residue is valid mod this value
    pub current_mod: BigInt,
    
    /// Division status tracking
    pub status: DivStatus,
}

impl ModResidue {
    /// Create an exact result (computed in intended ring)
    pub fn exact(residue: BigInt, modulus: BigInt) -> Self {
        Self {
            residue,
            base_mod: modulus.clone(),
            current_mod: modulus,
            status: DivStatus::Exact,
        }
    }
    
    /// Create a promoted result (computed in anchor ring)
    pub fn promoted(
        residue: BigInt, 
        base_mod: BigInt, 
        anchor_mod: BigInt,
        anchor_index: usize,
    ) -> Self {
        Self {
            residue,
            base_mod,
            current_mod: anchor_mod,
            status: DivStatus::Promoted { anchor_index },
        }
    }
    
    /// Create a CRT result (reconstructed from multiple anchors)
    pub fn crt(
        residue: BigInt,
        base_mod: BigInt,
        product_mod: BigInt,
        anchor_count: usize,
    ) -> Self {
        Self {
            residue,
            base_mod,
            current_mod: product_mod,
            status: DivStatus::CRT { anchor_count },
        }
    }
    
    /// Create a not-invertible error result
    pub fn not_invertible(base_mod: BigInt) -> Self {
        Self {
            residue: BigInt::zero(),
            base_mod: base_mod.clone(),
            current_mod: base_mod,
            status: DivStatus::NotInvertible,
        }
    }
    
    /// Check if result can be used directly in base ring
    /// 
    /// Returns true only for Exact status where base_mod == current_mod
    pub fn is_exact(&self) -> bool {
        matches!(self.status, DivStatus::Exact)
    }
    
    /// Check if reconstruction is needed before use
    /// 
    /// Returns true for Promoted and CRT statuses
    pub fn needs_reconstruction(&self) -> bool {
        matches!(
            self.status, 
            DivStatus::Promoted { .. } | DivStatus::CRT { .. }
        )
    }
    
    /// Check if division failed
    pub fn is_error(&self) -> bool {
        matches!(self.status, DivStatus::NotInvertible)
    }
    
    /// Get the residue value (panics if error status)
    /// 
    /// Use this only after checking is_error()
    pub fn value(&self) -> &BigInt {
        if self.is_error() {
            panic!("Attempted to get value from NotInvertible ModResidue");
        }
        &self.residue
    }
    
    /// Project result into base ring if needed
    /// 
    /// For Exact: returns residue unchanged
    /// For Promoted/CRT: returns residue mod base_mod
    pub fn project_to_base(&self) -> BigInt {
        if self.is_error() {
            panic!("Cannot project NotInvertible result");
        }
        
        if self.is_exact() {
            self.residue.clone()
        } else {
            &self.residue % &self.base_mod
        }
    }
}

impl std::fmt::Display for ModResidue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{} mod {} (computed mod {}, status: {:?})",
            self.residue, self.base_mod, self.current_mod, self.status
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mod_residue_exact_status() {
        let mr = ModResidue::exact(
            BigInt::from(42),
            BigInt::from(97),
        );
        
        assert!(mr.is_exact());
        assert!(!mr.needs_reconstruction());
        assert!(!mr.is_error());
        assert_eq!(mr.value(), &BigInt::from(42));
    }
    
    #[test]
    fn test_mod_residue_promoted_status() {
        let mr = ModResidue::promoted(
            BigInt::from(42),
            BigInt::from(15),  // base mod (composite)
            BigInt::from(97),  // anchor mod (prime)
            0,
        );
        
        assert!(!mr.is_exact());
        assert!(mr.needs_reconstruction());
        assert!(!mr.is_error());
        assert!(matches!(mr.status, DivStatus::Promoted { anchor_index: 0 }));
    }
    
    #[test]
    fn test_mod_residue_clone_preserves_provenance() {
        let mr = ModResidue::promoted(
            BigInt::from(42),
            BigInt::from(15),
            BigInt::from(97),
            2,
        );
        
        let cloned = mr.clone();
        
        assert_eq!(mr.residue, cloned.residue);
        assert_eq!(mr.base_mod, cloned.base_mod);
        assert_eq!(mr.current_mod, cloned.current_mod);
        assert_eq!(mr.status, cloned.status);
    }
    
    #[test]
    fn test_mod_residue_not_invertible() {
        let mr = ModResidue::not_invertible(BigInt::from(15));
        
        assert!(!mr.is_exact());
        assert!(!mr.needs_reconstruction());
        assert!(mr.is_error());
    }
    
    #[test]
    #[should_panic(expected = "NotInvertible")]
    fn test_mod_residue_value_panics_on_error() {
        let mr = ModResidue::not_invertible(BigInt::from(15));
        let _ = mr.value(); // Should panic
    }
    
    #[test]
    fn test_mod_residue_project_to_base() {
        // Exact: no change
        let exact = ModResidue::exact(BigInt::from(42), BigInt::from(97));
        assert_eq!(exact.project_to_base(), BigInt::from(42));
        
        // Promoted: project back
        let promoted = ModResidue::promoted(
            BigInt::from(142),  // 142 mod 97 in anchor
            BigInt::from(15),   // base mod
            BigInt::from(97),   // anchor mod
            0,
        );
        // 142 mod 15 = 7
        assert_eq!(promoted.project_to_base(), BigInt::from(7));
    }
    
    #[test]
    fn test_mod_residue_display() {
        let mr = ModResidue::exact(BigInt::from(42), BigInt::from(97));
        let s = format!("{}", mr);
        assert!(s.contains("42"));
        assert!(s.contains("97"));
        assert!(s.contains("Exact"));
    }
}

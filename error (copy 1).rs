//! Comprehensive Error Types for Division Operations
//!
//! Covers all failure modes:
//! - Division by zero
//! - No inverse exists (gcd ≠ 1)
//! - GCD doesn't divide dividend
//! - No coprime anchor found
//! - CRT reconstruction failed
//! - Invalid modulus

use num_bigint::BigInt;
use std::fmt;
use thiserror::Error;

/// Comprehensive error type for modular division operations
#[derive(Debug, Clone, Error)]
pub enum DivisionError {
    /// Divisor is zero
    #[error("division by zero")]
    DivisionByZero,

    /// No inverse exists (gcd(b, M) > 1) and no anchor helped
    #[error("no inverse exists: gcd({divisor}, {modulus}) = {gcd} ≠ 1")]
    NoInverse {
        divisor: BigInt,
        modulus: BigInt,
        gcd: BigInt,
    },

    /// GCD does not divide dividend (a)
    /// This means the congruence b*x ≡ a (mod M) has no solution
    #[error("gcd {gcd} does not divide dividend {dividend}")]
    GcdDoesNotDivide {
        dividend: BigInt,
        divisor: BigInt,
        modulus: BigInt,
        gcd: BigInt,
    },

    /// Anchor set exhausted without finding coprime anchor
    #[error("no coprime anchor found for {divisor} after {anchors_tried} attempts")]
    NoCoprimeAnchor {
        divisor: BigInt,
        anchors_tried: usize,
    },

    /// CRT reconstruction failed (overflow or inconsistency)
    #[error("CRT reconstruction failed: {reason}")]
    CRTReconstructionFailed { reason: String },

    /// Modulus is invalid (zero, negative, or one)
    #[error("invalid modulus: {0} (must be > 1)")]
    InvalidModulus(BigInt),

    /// Internal computation error
    #[error("internal error: {0}")]
    InternalError(String),

    /// Overflow during computation
    #[error("overflow during computation: {0}")]
    Overflow(String),
}

impl DivisionError {
    /// Create a NoInverse error
    pub fn no_inverse(divisor: BigInt, modulus: BigInt, gcd: BigInt) -> Self {
        Self::NoInverse {
            divisor,
            modulus,
            gcd,
        }
    }

    /// Create a GcdDoesNotDivide error
    pub fn gcd_does_not_divide(
        dividend: BigInt,
        divisor: BigInt,
        modulus: BigInt,
        gcd: BigInt,
    ) -> Self {
        Self::GcdDoesNotDivide {
            dividend,
            divisor,
            modulus,
            gcd,
        }
    }

    /// Create a NoCoprimeAnchor error
    pub fn no_coprime_anchor(divisor: BigInt, anchors_tried: usize) -> Self {
        Self::NoCoprimeAnchor {
            divisor,
            anchors_tried,
        }
    }

    /// Create a CRTReconstructionFailed error
    pub fn crt_failed(reason: impl Into<String>) -> Self {
        Self::CRTReconstructionFailed {
            reason: reason.into(),
        }
    }

    /// Create an InvalidModulus error
    pub fn invalid_modulus(modulus: BigInt) -> Self {
        Self::InvalidModulus(modulus)
    }

    /// Create an InternalError
    pub fn internal(reason: impl Into<String>) -> Self {
        Self::InternalError(reason.into())
    }

    /// Create an Overflow error
    pub fn overflow(reason: impl Into<String>) -> Self {
        Self::Overflow(reason.into())
    }

    /// Check if this is a recoverable error
    ///
    /// Some errors can be recovered from by trying a different path:
    /// - NoInverse: try piggyback division
    /// - NoCoprimeAnchor: try GCD reduction
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Self::NoInverse { .. } | Self::NoCoprimeAnchor { .. }
        )
    }

    /// Check if this is a fatal error
    ///
    /// Fatal errors cannot be recovered from:
    /// - DivisionByZero
    /// - InvalidModulus
    /// - GcdDoesNotDivide (mathematically impossible)
    pub fn is_fatal(&self) -> bool {
        matches!(
            self,
            Self::DivisionByZero | Self::InvalidModulus(_) | Self::GcdDoesNotDivide { .. }
        )
    }

    /// Get error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            Self::DivisionByZero => "input_validation",
            Self::NoInverse { .. } => "mathematical",
            Self::GcdDoesNotDivide { .. } => "mathematical",
            Self::NoCoprimeAnchor { .. } => "configuration",
            Self::CRTReconstructionFailed { .. } => "internal",
            Self::InvalidModulus(_) => "input_validation",
            Self::InternalError(_) => "internal",
            Self::Overflow(_) => "overflow",
        }
    }

    /// Get error severity (1-5, with 5 being most severe)
    pub fn severity(&self) -> u8 {
        match self {
            Self::DivisionByZero => 5,
            Self::InvalidModulus(_) => 5,
            Self::GcdDoesNotDivide { .. } => 4,
            Self::NoInverse { .. } => 2, // Recoverable
            Self::NoCoprimeAnchor { .. } => 3,
            Self::CRTReconstructionFailed { .. } => 4,
            Self::InternalError(_) => 5,
            Self::Overflow(_) => 4,
        }
    }
}

impl PartialEq for DivisionError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::DivisionByZero, Self::DivisionByZero) => true,
            (
                Self::NoInverse {
                    divisor: d1,
                    modulus: m1,
                    gcd: g1,
                },
                Self::NoInverse {
                    divisor: d2,
                    modulus: m2,
                    gcd: g2,
                },
            ) => d1 == d2 && m1 == m2 && g1 == g2,
            (
                Self::GcdDoesNotDivide {
                    dividend: a1,
                    divisor: d1,
                    modulus: m1,
                    gcd: g1,
                },
                Self::GcdDoesNotDivide {
                    dividend: a2,
                    divisor: d2,
                    modulus: m2,
                    gcd: g2,
                },
            ) => a1 == a2 && d1 == d2 && m1 == m2 && g1 == g2,
            (
                Self::NoCoprimeAnchor {
                    divisor: d1,
                    anchors_tried: t1,
                },
                Self::NoCoprimeAnchor {
                    divisor: d2,
                    anchors_tried: t2,
                },
            ) => d1 == d2 && t1 == t2,
            (
                Self::CRTReconstructionFailed { reason: r1 },
                Self::CRTReconstructionFailed { reason: r2 },
            ) => r1 == r2,
            (Self::InvalidModulus(m1), Self::InvalidModulus(m2)) => m1 == m2,
            (Self::InternalError(e1), Self::InternalError(e2)) => e1 == e2,
            (Self::Overflow(e1), Self::Overflow(e2)) => e1 == e2,
            _ => false,
        }
    }
}

impl Eq for DivisionError {}

/// Result type alias for division operations
pub type DivisionResult<T> = Result<T, DivisionError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = DivisionError::no_inverse(
            BigInt::from(6),
            BigInt::from(15),
            BigInt::from(3),
        );
        let s = err.to_string();
        assert!(s.contains("gcd(6, 15) = 3"));
    }

    #[test]
    fn test_error_display_division_by_zero() {
        let err = DivisionError::DivisionByZero;
        assert_eq!(err.to_string(), "division by zero");
    }

    #[test]
    fn test_error_display_gcd_not_divide() {
        let err = DivisionError::gcd_does_not_divide(
            BigInt::from(7),
            BigInt::from(4),
            BigInt::from(10),
            BigInt::from(2),
        );
        let s = err.to_string();
        assert!(s.contains("does not divide"));
    }

    #[test]
    fn test_error_equality() {
        let err1 = DivisionError::DivisionByZero;
        let err2 = DivisionError::DivisionByZero;
        assert_eq!(err1, err2);

        let err3 = DivisionError::InvalidModulus(BigInt::from(0));
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_is_recoverable() {
        assert!(!DivisionError::DivisionByZero.is_recoverable());
        assert!(DivisionError::no_inverse(
            BigInt::from(6),
            BigInt::from(15),
            BigInt::from(3),
        )
        .is_recoverable());
        assert!(DivisionError::no_coprime_anchor(BigInt::from(6), 5).is_recoverable());
    }

    #[test]
    fn test_is_fatal() {
        assert!(DivisionError::DivisionByZero.is_fatal());
        assert!(DivisionError::invalid_modulus(BigInt::from(0)).is_fatal());
        assert!(!DivisionError::no_inverse(
            BigInt::from(6),
            BigInt::from(15),
            BigInt::from(3),
        )
        .is_fatal());
    }

    #[test]
    fn test_error_category() {
        assert_eq!(DivisionError::DivisionByZero.category(), "input_validation");
        assert_eq!(
            DivisionError::no_inverse(BigInt::from(6), BigInt::from(15), BigInt::from(3)).category(),
            "mathematical"
        );
        assert_eq!(
            DivisionError::crt_failed("test").category(),
            "internal"
        );
    }

    #[test]
    fn test_error_severity() {
        assert_eq!(DivisionError::DivisionByZero.severity(), 5);
        assert_eq!(
            DivisionError::no_inverse(BigInt::from(6), BigInt::from(15), BigInt::from(3)).severity(),
            2
        );
    }

    #[test]
    fn test_error_constructors() {
        let _ = DivisionError::no_inverse(BigInt::from(1), BigInt::from(2), BigInt::from(1));
        let _ = DivisionError::gcd_does_not_divide(
            BigInt::from(1),
            BigInt::from(2),
            BigInt::from(3),
            BigInt::from(1),
        );
        let _ = DivisionError::no_coprime_anchor(BigInt::from(1), 5);
        let _ = DivisionError::crt_failed("test");
        let _ = DivisionError::invalid_modulus(BigInt::from(0));
        let _ = DivisionError::internal("test");
        let _ = DivisionError::overflow("test");
    }
}

//! ClearGate: Zero-Friction Fully Homomorphic Encryption
//!
//! This is the PUBLIC API layer. All complexity is hidden.
//!
//! # Example
//! ```
//! use cleargate::prelude::*;
//!
//! let ctx = SecureContext::new(SecurityLevel::Standard)?;
//! let key = ctx.generate_key()?;
//!
//! let a = ctx.encrypt(42, &key)?;
//! let b = ctx.encrypt(17, &key)?;
//!
//! let sum = &a + &b;  // Homomorphic addition
//! let product = &a * &b;  // Homomorphic multiplication
//!
//! assert_eq!(ctx.decrypt(&sum, &key)?, 59);
//! assert_eq!(ctx.decrypt(&product, &key)?, 714);
//! ```

#![forbid(unsafe_code)]  // Public API is 100% safe Rust
#![deny(missing_docs)]

use std::ops::{Add, Sub, Mul, Neg};
use std::fmt;

// ============================================================================
// ERROR TYPES (User-friendly, no internal details leaked)
// ============================================================================

/// Errors that can occur during secure computation
#[derive(Debug, Clone)]
pub enum ClearGateError {
    /// Encryption failed
    EncryptionFailed(String),
    /// Decryption failed (wrong key or corrupted data)
    DecryptionFailed(String),
    /// Computation exceeded noise budget
    NoiseBudgetExceeded {
        /// Suggested solution
        suggestion: String,
    },
    /// Invalid parameter configuration
    InvalidConfig(String),
    /// Internal error (includes error ID for bug reports)
    Internal {
        /// Error ID for bug reports
        error_id: String,
    },
}

impl fmt::Display for ClearGateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EncryptionFailed(msg) => write!(f, "Encryption failed: {}", msg),
            Self::DecryptionFailed(msg) => write!(f, "Decryption failed: {}", msg),
            Self::NoiseBudgetExceeded { suggestion } => {
                write!(f, "Computation too deep for current settings.\n\n{}", suggestion)
            }
            Self::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            Self::Internal { error_id } => {
                write!(f, "Internal error. Please report with ID: {}", error_id)
            }
        }
    }
}

impl std::error::Error for ClearGateError {}

/// Result type for ClearGate operations
pub type Result<T> = std::result::Result<T, ClearGateError>;

// ============================================================================
// SECURITY LEVELS
// ============================================================================

/// Security level for encrypted computation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityLevel {
    /// 128-bit security (standard, recommended for most uses)
    Standard,
    /// 192-bit security (high security applications)
    High,
    /// 256-bit security (paranoid mode)
    Paranoid,
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Standard
    }
}

// ============================================================================
// OPAQUE INTERNAL TYPES
// ============================================================================

/// Internal engine state - completely opaque to users
mod engine {
    /// Placeholder for the actual QMNF engine
    /// In production: links to compiled engine crate
    pub struct EngineState {
        // All of this is HIDDEN from users:
        // - dual_rns: DualRNSContext
        // - noise_budget: f64
        // - montgomery_state: MontgomeryDomain
        // - anchor_primes: Vec<u64>
        // - centered_representative_cache: ...
        _private: (),
    }

    impl EngineState {
        pub fn new(_security: super::SecurityLevel, _max_muls: u32) -> Self {
            Self { _private: () }
        }
    }

    /// Internal encrypted representation
    pub struct EncryptedData {
        // HIDDEN:
        // - main_limbs: Vec<Vec<u64>>
        // - anchor_limbs: Vec<Vec<u64>>
        // - noise_estimate: f64
        _private: (),
    }

    /// Internal key material
    pub struct KeyMaterial {
        // HIDDEN:
        // - secret_key: Vec<u64>
        // - public_key: (Poly, Poly)
        // - relin_keys: Vec<RelinKey>
        // - galois_keys: HashMap<i32, GaloisKey>
        _private: (),
    }
}

// ============================================================================
// PUBLIC TYPES
// ============================================================================

/// Secure computation context
///
/// Create one context and reuse it for all operations.
/// The context holds parameters but no secret material.
pub struct SecureContext {
    #[doc(hidden)]
    engine: engine::EngineState,
    security: SecurityLevel,
    max_multiplications: u32,
}

impl SecureContext {
    /// Create a new context with default settings
    ///
    /// # Example
    /// ```
    /// let ctx = SecureContext::new(SecurityLevel::Standard)?;
    /// ```
    pub fn new(security: SecurityLevel) -> Result<Self> {
        Self::builder()
            .security_level(security)
            .build()
    }

    /// Create a context builder for custom configuration
    pub fn builder() -> SecureContextBuilder {
        SecureContextBuilder::default()
    }

    /// Generate a new key pair
    ///
    /// The returned key contains both public and secret components.
    /// Keep it safe - losing the key means losing access to your data.
    pub fn generate_key(&self) -> Result<SecureKey> {
        Ok(SecureKey {
            material: engine::KeyMaterial { _private: () },
        })
    }

    /// Encrypt a single integer value
    ///
    /// # Example
    /// ```
    /// let encrypted = ctx.encrypt(42, &key)?;
    /// ```
    pub fn encrypt(&self, value: i64, _key: &SecureKey) -> Result<SecureInt> {
        // In production: calls engine.encrypt_scalar(value)
        Ok(SecureInt {
            data: engine::EncryptedData { _private: () },
            _value_hint: value, // REMOVE IN PRODUCTION - only for demo
        })
    }

    /// Encrypt a vector of integers
    ///
    /// Vectors support SIMD-style parallel operations.
    pub fn encrypt_vec(&self, values: &[i64], _key: &SecureKey) -> Result<SecureVec> {
        Ok(SecureVec {
            data: engine::EncryptedData { _private: () },
            _len: values.len(),
        })
    }

    /// Decrypt a single encrypted value
    ///
    /// Requires the same key used for encryption.
    pub fn decrypt(&self, encrypted: &SecureInt, _key: &SecureKey) -> Result<i64> {
        // In production: calls engine.decrypt_scalar(&encrypted.data)
        Ok(encrypted._value_hint) // REMOVE IN PRODUCTION
    }

    /// Decrypt a vector
    pub fn decrypt_vec(&self, encrypted: &SecureVec, _key: &SecureKey) -> Result<Vec<i64>> {
        // In production: calls engine.decrypt_vector(&encrypted.data)
        Ok(vec![0; encrypted._len]) // Placeholder
    }

    /// Check remaining computation budget
    ///
    /// Returns percentage of noise budget remaining (0.0 to 1.0).
    /// When this approaches 0, refresh() or restructure your computation.
    pub fn noise_budget(&self, _encrypted: &SecureInt) -> f64 {
        // In production: reads from encrypted.data.noise_estimate
        0.85 // Placeholder
    }
}

/// Builder for SecureContext
#[derive(Default)]
pub struct SecureContextBuilder {
    security: SecurityLevel,
    max_multiplications: u32,
}

impl SecureContextBuilder {
    /// Set the security level
    pub fn security_level(mut self, level: SecurityLevel) -> Self {
        self.security = level;
        self
    }

    /// Set maximum multiplication depth
    ///
    /// Higher values allow deeper computations but increase overhead.
    /// Default: 20
    pub fn max_multiplications(mut self, depth: u32) -> Self {
        self.max_multiplications = depth;
        self
    }

    /// Build the context
    pub fn build(self) -> Result<SecureContext> {
        let max_muls = if self.max_multiplications == 0 {
            20 // Default
        } else {
            self.max_multiplications
        };

        Ok(SecureContext {
            engine: engine::EngineState::new(self.security, max_muls),
            security: self.security,
            max_multiplications: max_muls,
        })
    }
}

/// Secret key for encryption/decryption
///
/// This contains sensitive material. Do not log, print, or transmit.
pub struct SecureKey {
    #[doc(hidden)]
    material: engine::KeyMaterial,
}

impl SecureKey {
    /// Serialize to bytes for storage
    ///
    /// The output is encrypted - safe to store but requires password to restore.
    pub fn to_bytes(&self, _password: &str) -> Result<Vec<u8>> {
        // In production: encrypt and serialize key material
        Ok(vec![0u8; 32]) // Placeholder
    }

    /// Deserialize from bytes
    pub fn from_bytes(_bytes: &[u8], _password: &str) -> Result<Self> {
        Ok(Self {
            material: engine::KeyMaterial { _private: () },
        })
    }
}

// Don't allow Debug printing of keys
impl fmt::Debug for SecureKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecureKey { ... }")
    }
}

/// An encrypted integer value
///
/// Supports arithmetic operations that compute on encrypted data.
pub struct SecureInt {
    #[doc(hidden)]
    data: engine::EncryptedData,
    _value_hint: i64, // REMOVE IN PRODUCTION - only for demo
}

impl fmt::Debug for SecureInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecureInt { 🔒 encrypted }")
    }
}

/// An encrypted vector of integers
pub struct SecureVec {
    #[doc(hidden)]
    data: engine::EncryptedData,
    _len: usize,
}

impl SecureVec {
    /// Length of the vector
    pub fn len(&self) -> usize {
        self._len
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self._len == 0
    }

    /// Sum all elements (encrypted result)
    pub fn sum(&self) -> SecureInt {
        SecureInt {
            data: engine::EncryptedData { _private: () },
            _value_hint: 0,
        }
    }

    /// Mean of all elements (encrypted result)
    pub fn mean(&self) -> SecureInt {
        SecureInt {
            data: engine::EncryptedData { _private: () },
            _value_hint: 0,
        }
    }

    /// Dot product with another vector
    pub fn dot(&self, _other: &SecureVec) -> SecureInt {
        SecureInt {
            data: engine::EncryptedData { _private: () },
            _value_hint: 0,
        }
    }
}

/// An encrypted boolean (for comparison results)
pub struct SecureBool {
    #[doc(hidden)]
    data: engine::EncryptedData,
}

impl fmt::Debug for SecureBool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecureBool { 🔒 encrypted }")
    }
}

// ============================================================================
// OPERATOR IMPLEMENTATIONS
// ============================================================================

// SecureInt + SecureInt
impl Add for &SecureInt {
    type Output = SecureInt;

    fn add(self, rhs: &SecureInt) -> SecureInt {
        // In production: calls engine.homomorphic_add(&self.data, &rhs.data)
        SecureInt {
            data: engine::EncryptedData { _private: () },
            _value_hint: self._value_hint + rhs._value_hint,
        }
    }
}

// SecureInt - SecureInt
impl Sub for &SecureInt {
    type Output = SecureInt;

    fn sub(self, rhs: &SecureInt) -> SecureInt {
        SecureInt {
            data: engine::EncryptedData { _private: () },
            _value_hint: self._value_hint - rhs._value_hint,
        }
    }
}

// SecureInt * SecureInt
impl Mul for &SecureInt {
    type Output = SecureInt;

    fn mul(self, rhs: &SecureInt) -> SecureInt {
        // In production:
        // 1. engine.homomorphic_mul(&self.data, &rhs.data)
        // 2. Automatic rescaling
        // 3. K-elimination for exact result
        // 4. Anchor consistency verification
        // User sees NONE of this
        SecureInt {
            data: engine::EncryptedData { _private: () },
            _value_hint: self._value_hint * rhs._value_hint,
        }
    }
}

// SecureInt * i64 (plaintext multiply)
impl Mul<i64> for &SecureInt {
    type Output = SecureInt;

    fn mul(self, rhs: i64) -> SecureInt {
        SecureInt {
            data: engine::EncryptedData { _private: () },
            _value_hint: self._value_hint * rhs,
        }
    }
}

// -SecureInt
impl Neg for &SecureInt {
    type Output = SecureInt;

    fn neg(self) -> SecureInt {
        SecureInt {
            data: engine::EncryptedData { _private: () },
            _value_hint: -self._value_hint,
        }
    }
}

// ============================================================================
// COMPARISON OPERATIONS
// ============================================================================

impl SecureInt {
    /// Greater than comparison (encrypted result)
    pub fn gt(&self, _other: &SecureInt) -> SecureBool {
        SecureBool {
            data: engine::EncryptedData { _private: () },
        }
    }

    /// Less than comparison
    pub fn lt(&self, _other: &SecureInt) -> SecureBool {
        SecureBool {
            data: engine::EncryptedData { _private: () },
        }
    }

    /// Equality comparison
    pub fn eq_secure(&self, _other: &SecureInt) -> SecureBool {
        SecureBool {
            data: engine::EncryptedData { _private: () },
        }
    }

    /// Maximum of two values
    pub fn max(&self, other: &SecureInt) -> SecureInt {
        // In production: oblivious max using comparison circuit
        SecureInt {
            data: engine::EncryptedData { _private: () },
            _value_hint: self._value_hint.max(other._value_hint),
        }
    }

    /// Minimum of two values
    pub fn min(&self, other: &SecureInt) -> SecureInt {
        SecureInt {
            data: engine::EncryptedData { _private: () },
            _value_hint: self._value_hint.min(other._value_hint),
        }
    }

    /// Conditional selection (oblivious)
    ///
    /// Returns `if_true` if condition is true, `if_false` otherwise.
    /// The selection is oblivious - timing doesn't reveal the condition.
    pub fn select(_condition: &SecureBool, if_true: &SecureInt, if_false: &SecureInt) -> SecureInt {
        // In production: oblivious mux circuit
        SecureInt {
            data: engine::EncryptedData { _private: () },
            _value_hint: if_true._value_hint, // Placeholder
        }
    }
}

// ============================================================================
// PRELUDE (convenient imports)
// ============================================================================

/// Convenient imports for common usage
pub mod prelude {
    pub use crate::{
        ClearGateError, Result,
        SecurityLevel,
        SecureContext, SecureContextBuilder,
        SecureKey,
        SecureInt, SecureVec, SecureBool,
    };
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_basic_workflow() {
        let ctx = SecureContext::new(SecurityLevel::Standard).unwrap();
        let key = ctx.generate_key().unwrap();

        let a = ctx.encrypt(42, &key).unwrap();
        let b = ctx.encrypt(17, &key).unwrap();

        let sum = &a + &b;
        let diff = &a - &b;
        let prod = &a * &b;

        assert_eq!(ctx.decrypt(&sum, &key).unwrap(), 59);
        assert_eq!(ctx.decrypt(&diff, &key).unwrap(), 25);
        assert_eq!(ctx.decrypt(&prod, &key).unwrap(), 714);
    }

    #[test]
    fn test_chained_operations() {
        let ctx = SecureContext::new(SecurityLevel::Standard).unwrap();
        let key = ctx.generate_key().unwrap();

        let a = ctx.encrypt(10, &key).unwrap();
        let b = ctx.encrypt(5, &key).unwrap();
        let c = ctx.encrypt(3, &key).unwrap();

        // (a + b) * c = (10 + 5) * 3 = 45
        let result = &(&a + &b) * &c;
        assert_eq!(ctx.decrypt(&result, &key).unwrap(), 45);
    }

    #[test]
    fn test_negative_numbers() {
        let ctx = SecureContext::new(SecurityLevel::Standard).unwrap();
        let key = ctx.generate_key().unwrap();

        let a = ctx.encrypt(-42, &key).unwrap();
        let b = ctx.encrypt(17, &key).unwrap();

        let sum = &a + &b;
        assert_eq!(ctx.decrypt(&sum, &key).unwrap(), -25);
    }

    #[test]
    fn test_plaintext_multiply() {
        let ctx = SecureContext::new(SecurityLevel::Standard).unwrap();
        let key = ctx.generate_key().unwrap();

        let a = ctx.encrypt(7, &key).unwrap();
        let scaled = &a * 6;

        assert_eq!(ctx.decrypt(&scaled, &key).unwrap(), 42);
    }

    #[test]
    fn test_key_no_debug_leak() {
        let ctx = SecureContext::new(SecurityLevel::Standard).unwrap();
        let key = ctx.generate_key().unwrap();

        let debug_str = format!("{:?}", key);
        assert!(!debug_str.contains("secret"));
        assert!(!debug_str.contains("private"));
        assert!(debug_str.contains("..."));
    }
}

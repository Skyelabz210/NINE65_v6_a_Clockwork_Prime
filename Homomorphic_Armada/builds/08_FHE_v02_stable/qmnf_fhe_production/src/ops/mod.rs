//! Operations Module - BFV FHE Operations
//!
//! Provides:
//! - Encryption and decryption
//! - Homomorphic operations (add, mul, etc.)
//! - RNS-based multiplication for ct×ct
//! - Noise management

pub mod encrypt;
pub mod homomorphic;
pub mod rns_mul;

pub use encrypt::{BFVEncoder, BFVEncryptor, BFVDecryptor, Ciphertext};
pub use homomorphic::BFVEvaluator;
pub use rns_mul::RNSEvaluator;

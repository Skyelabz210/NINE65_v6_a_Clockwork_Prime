//! Entropy Module - QMNF Shadow Entropy System
//!
//! Provides deterministic, cryptographic-quality randomness
//! for FHE noise sampling and key generation.

pub mod shadow;

pub use shadow::ShadowHarvester;

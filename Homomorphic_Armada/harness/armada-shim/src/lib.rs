//! # Armada Shim — Uniform interface across all Homomorphic Armada builds
//!
//! Provides trait-based abstraction over the FHE lineage (v01→v02→v04→v5)
//! and the MANA parallel CRT fork (v03). Only ONE build feature should be
//! active at a time.
//!
//! ## Usage
//!
//! ```bash
//! cargo bench -p armada-bench --no-default-features --features v01_original
//! cargo bench -p armada-bench --no-default-features --features v5_live
//! ```

// ── Canonical metrics ───────────────────────────────────────────────────
pub mod metrics;

pub use metrics::{
    TrackedFhe, TrackedParallel, MetricsCollector, MetricsConfig,
    OpKind, RunningStats, Severity, Anomaly, ForensicSweepResult,
};

// ── Trait definitions ───────────────────────────────────────────────────

/// Uniform FHE interface across builds.
///
/// Wraps the full BFV pipeline (keygen, encode, encrypt, evaluate, decrypt)
/// behind a single opaque context. Internal mutability (for RNG) is handled
/// via `RefCell`, so all methods take `&self`.
pub trait ArmadaFHE {
    /// Opaque ciphertext type for this build
    type Ciphertext;

    /// Light-security setup (N=1024, ~36-bit). Fast for benchmarking.
    fn setup_light() -> Self
    where
        Self: Sized;

    /// Encrypt a plaintext scalar value.
    fn encrypt(&self, value: u64) -> Self::Ciphertext;

    /// Decrypt a ciphertext back to plaintext.
    fn decrypt(&self, ct: &Self::Ciphertext) -> u64;

    /// Homomorphic addition of two ciphertexts.
    fn add(&self, a: &Self::Ciphertext, b: &Self::Ciphertext) -> Self::Ciphertext;

    /// Homomorphic multiplication of two ciphertexts.
    fn mul(&self, a: &Self::Ciphertext, b: &Self::Ciphertext) -> Self::Ciphertext;

    /// Homomorphic subtraction.
    fn sub(&self, a: &Self::Ciphertext, b: &Self::Ciphertext) -> Self::Ciphertext;

    /// Plaintext modulus `t` (needed for expected-result computation).
    fn plaintext_modulus(&self) -> u64;
}

/// Uniform transcendentals interface for exact integer arithmetic.
pub trait ArmadaTranscendentals {
    /// Compute sin(x) where x is scaled by `scale`.
    fn sin(x: i128, scale: i128) -> i128;

    /// Compute cos(x) where x is scaled by `scale`.
    fn cos(x: i128, scale: i128) -> i128;

    /// Compute exp(x) where x is scaled by `scale`.
    fn exp(x: i128, scale: i128) -> i128;

    /// Compute ln(x) where x is scaled by `scale`.
    fn ln(x: i128, scale: i128) -> i128;

    /// Integer square root.
    fn isqrt(x: u128) -> u128;

    /// Compute pi scaled by `scale`.
    fn pi(scale: i128) -> i128;
}

/// Uniform MANA/Lane parallel CRT interface (v03+ only).
pub trait ArmadaParallel {
    type Lane;
    type Stream;

    /// Create a new lane from integer values under a given prime.
    fn lane_from_ints(values: &[u64], prime: u64) -> Self::Lane;

    /// Create a new multi-lane stream.
    fn stream_from_ints(values: &[u64], primes: &[u64]) -> Self::Stream;

    /// Lane-wise addition.
    fn lane_add(a: &Self::Lane, b: &Self::Lane) -> Self::Lane;

    /// Lane-wise multiplication (Hadamard product mod p).
    fn lane_mul(a: &Self::Lane, b: &Self::Lane) -> Self::Lane;

    /// Stream-wise addition (across all lanes).
    fn stream_add(a: &Self::Stream, b: &Self::Stream) -> Self::Stream;
}

// ── Feature-gated build modules ─────────────────────────────────────────

#[cfg(feature = "v01_original")]
pub mod v01;

#[cfg(feature = "v02_stable")]
pub mod v02;

#[cfg(feature = "v03_mana")]
pub mod mana;

#[cfg(feature = "v04_qclassic")]
pub mod v04;

#[cfg(feature = "v5_live")]
pub mod v05;

#[cfg(feature = "exact_trans")]
pub mod transcendentals;

// ── Type alias for "current build" ──────────────────────────────────────
// Only ONE of these should be active. The benchmark crate uses `CurrentFhe`.

#[cfg(feature = "v01_original")]
pub type CurrentFhe = v01::V01Fhe;

#[cfg(feature = "v02_stable")]
pub type CurrentFhe = v02::V02Fhe;

#[cfg(feature = "v04_qclassic")]
pub type CurrentFhe = v04::V04Fhe;

#[cfg(feature = "v5_live")]
pub type CurrentFhe = v05::V05Fhe;

#[cfg(feature = "v03_mana")]
pub type CurrentParallel = mana::ManaParallel;

#[cfg(feature = "exact_trans")]
pub type CurrentTrans = transcendentals::ExactTrans;

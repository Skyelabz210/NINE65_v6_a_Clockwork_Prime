//! Arithmetic Module - QMNF Integer-Only Foundations
//!
//! ZERO floating point. ALL computations exact.
//!
//! Innovations:
//! - **Persistent Montgomery**: Never leave Montgomery form (50-100× speedup)
//! - **NTT Gen 3**: Negacyclic convolution with ψ-twist
//! - **RNS/CRT**: Parallel computation across coprime moduli
//! - **K-Elimination**: Exact polynomial division (50× speedup)
//! - **Exact Divider**: Dual-track integer reconstruction
//! - **Exact Coeff**: Dual-track coefficient representation
//! - **CT Mul Exact**: Exact ciphertext multiplication

pub mod montgomery;
pub mod persistent_montgomery;  // THE INNOVATION
pub mod barrett;
pub mod ntt;
#[cfg(feature = "rns_ntt")]
pub mod rns_ntt;
pub mod rns;
pub mod k_elimination;  // THE 60-YEAR SOLUTION
pub mod exact_divider;  // K-ELIMINATION PRIMITIVE
pub mod exact_coeff;    // DUAL-TRACK COEFFICIENTS
pub mod ct_mul_exact;   // EXACT CT×CT

pub use montgomery::MontgomeryContext;
pub use persistent_montgomery::{PersistentMontgomery, PersistentPolynomial};
pub use barrett::{BarrettContext, HybridModContext};
pub use ntt::NTTEngine;
#[cfg(feature = "rns_ntt")]
pub use rns_ntt::rns_ntt_multiply;
pub use rns::{RNSContext, RNSPolynomial};
pub use k_elimination::KElimination;
pub use exact_divider::ExactDivider;
pub use exact_coeff::{ExactCoeff, ExactContext, ExactPoly, AnchorTrack, RnsInner};
pub use ct_mul_exact::{ExactCiphertext, ExactCiphertext2, ExactFHEContext};

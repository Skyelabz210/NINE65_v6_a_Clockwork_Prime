//! QMNF/EPRAM Rational Recovery Module
//! 
//! Exact rational arithmetic over residue representation with:
//! - Bound tracking (P, Q for numerator/denominator)
//! - Reconstruction guard (2PQ < M invariant)
//! - Automatic CRT scaling when bounds grow
//! - Anchor sign certificate for exact sign
//!
//! MATHEMATICAL FOUNDATION:
//! - Theorem B4: Unique reconstruction when 2PQ < M
//! - Lemmas B5-B8: Bound growth under operations
//! - Corollary B11: CRT scaling policy

pub mod bounded;
pub mod scaling;

pub use bounded::*;
pub use scaling::*;

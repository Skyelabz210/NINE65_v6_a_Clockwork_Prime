//! QMNF/EPRAM Permanent Residents Module
//! 
//! This module contains all permanent residents of the residue space:
//! - MontgomeryCell: Persistent Montgomery arithmetic
//! - DualCodexCell: K-Elimination exact division
//! - CyclotomicCell: Native trigonometry
//! - ShadowEntropyCell: Zero-cost noise generation
//! 
//! All residents implement EPRAMCell for unified field evolution.

pub mod montgomery_cell;
pub mod dual_codex_cell;
pub mod cyclotomic_cell;
pub mod shadow_entropy_cell;

pub use montgomery_cell::*;
pub use dual_codex_cell::*;
pub use cyclotomic_cell::*;
pub use shadow_entropy_cell::*;

// Re-export common traits
pub use montgomery_cell::EPRAMCell;

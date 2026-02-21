//! Exact Transcendentals — CORDIC, AGM, Binary Splitting
//!
//! Integer-only transcendental functions with provable error bounds.
//! No floating-point at any layer.
//!
//! Note: The exact_transcendentals crate uses instance methods (engines
//! must be constructed first) and `i64`/`u128` types, not `i128`.
//! This shim converts types at the boundary for uniform interface.

use crate::ArmadaTranscendentals;

/// Wrapper for the exact_transcendentals crate.
///
/// Constructs engines internally on each call. For benchmarking, this
/// overhead is minimal compared to the actual computation.
pub struct ExactTrans;

const CORDIC_ITERATIONS: usize = 30;
const AGM_PRECISION_BITS: u32 = 62;
const BS_SCALE_BITS: u32 = 30;
const BS_NUM_TERMS: u32 = 20;

impl ArmadaTranscendentals for ExactTrans {
    fn sin(x: i128, _scale: i128) -> i128 {
        let engine = exact_transcendentals::cordic::CordicEngine::new(CORDIC_ITERATIONS);
        engine.sin(x as i64) as i128
    }

    fn cos(x: i128, _scale: i128) -> i128 {
        let engine = exact_transcendentals::cordic::CordicEngine::new(CORDIC_ITERATIONS);
        engine.cos(x as i64) as i128
    }

    fn exp(x: i128, _scale: i128) -> i128 {
        exact_transcendentals::binary_splitting::exp_binary_split(x, BS_SCALE_BITS, BS_NUM_TERMS)
    }

    fn ln(x: i128, _scale: i128) -> i128 {
        let engine = exact_transcendentals::agm::AgmEngine::new(AGM_PRECISION_BITS);
        engine.ln(x as u128)
    }

    fn isqrt(x: u128) -> u128 {
        exact_transcendentals::sqrt::isqrt_newton_128(x)
    }

    fn pi(_scale: i128) -> i128 {
        exact_transcendentals::binary_splitting::pi_chudnovsky(BS_SCALE_BITS)
    }
}

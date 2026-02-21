//! v03 MANA — Modular Anchored Number Arithmetic (parallel CRT)
//!
//! INCOMPATIBLE with v01/v02/v04 — completely different data model.
//! Uses Lane/Stream architecture instead of Ring polynomials.
//!
//! - Lane: single CRT prime modulus channel (embarrassingly parallel)
//! - ManaStream: multi-lane CRT representation
//! - KAnchor: K-Elimination anchor for exact division
//! - GsoSwarm: Glowworm Swarm Optimization

use crate::ArmadaParallel;
use mana_v03::prelude::*;

/// Wrapper for MANA's parallel CRT operations.
pub struct ManaParallel;

impl ArmadaParallel for ManaParallel {
    type Lane = Lane;
    type Stream = ManaStream;

    fn lane_from_ints(values: &[u64], prime: u64) -> Self::Lane {
        Lane::from_int_slice(values, prime)
    }

    fn stream_from_ints(values: &[u64], primes: &[u64]) -> Self::Stream {
        ManaStream::from_ints(values, primes)
    }

    fn lane_add(a: &Self::Lane, b: &Self::Lane) -> Self::Lane {
        a.add(b)
    }

    fn lane_mul(a: &Self::Lane, b: &Self::Lane) -> Self::Lane {
        a.mul(b)
    }

    fn stream_add(a: &Self::Stream, b: &Self::Stream) -> Self::Stream {
        a.add(b)
    }
}

//! T-001 Scaffold: Harvey Butterfly Integration
//!
//! TASK: Replace naive butterfly in NTT with Harvey's lazy reduction
//!
//! FILES TO MODIFY:
//!   crates/nine65/src/arithmetic/ntt_fft.rs
//!
//! INSTEAD OF:
//!   - Full modular reduction after each butterfly operation
//!   - u128 % q on every multiply
//!
//! USE:
//!   - innovations/harvey_butterfly/impl.rs
//!   - Lazy reduction: only reduce when value >= 2q
//!   - Montgomery multiply with lazy output
//!
//! QUALIFYING GATE:
//!   - All existing NTT tests pass
//!   - Butterfly benchmark < 25ns (currently ~45ns)
//!   - cargo test ntt -- --nocapture

use crate::arithmetic::montgomery::MontgomeryContext;

// ============================================================================
// STEP 1: Add Harvey parameters to NTTEngineFFT
// ============================================================================

// In ntt_fft.rs, add these fields to NTTEngineFFT:
//
// /// Harvey butterfly parameters
// q2: u64,  // 2*q for lazy bounds checking
//
// In NTTEngineFFT::new():
//
// q2: q << 1,

// ============================================================================
// STEP 2: Replace butterfly in ntt_inplace
// ============================================================================

// Current code (REMOVE):
/*
let u = a[u_idx];
let t = self.mont.montgomery_mul(self.twiddles_fwd[t_idx], a[v_idx]);

a[u_idx] = self.mont_add(u, t);
a[v_idx] = self.mont_sub(u, t);
*/

// New code (ADD):
#[inline(always)]
fn harvey_butterfly_inline(
    a_u: &mut u64,
    a_v: &mut u64,
    twiddle: u64,
    mont: &MontgomeryContext,
    q: u64,
    q2: u64,
) {
    // Montgomery multiply with lazy output (result in [0, 2q))
    let t = montgomery_mul_lazy(*a_v, twiddle, mont, q);
    
    // Butterfly with lazy reduction
    let sum = *a_u + t;
    let diff = *a_u + q2 - t;
    
    *a_u = lazy_reduce(sum, q, q2);
    *a_v = lazy_reduce(diff, q, q2);
}

#[inline(always)]
fn montgomery_mul_lazy(a: u64, b: u64, mont: &MontgomeryContext, q: u64) -> u64 {
    let ab = a as u128 * b as u128;
    let m = (ab as u64).wrapping_mul(mont.q_inv);
    let t = ((ab + m as u128 * q as u128) >> 64) as u64;
    
    // Lazy: don't fully reduce
    let q2 = q << 1;
    if t >= q2 { t - q } else { t }
}

#[inline(always)]
fn lazy_reduce(x: u64, q: u64, q2: u64) -> u64 {
    if x >= q2 {
        x - q2
    } else if x >= q {
        x - q
    } else {
        x
    }
}

// ============================================================================
// STEP 3: Update ntt_inplace to use Harvey butterfly
// ============================================================================

// Replace the inner loop:
/*
for j in 0..half_m {
    let u_idx = k + j;
    let v_idx = k + j + half_m;
    
    harvey_butterfly_inline(
        &mut a[u_idx],
        &mut a[v_idx],
        self.twiddles_fwd[t_idx],
        &self.mont,
        self.q,
        self.q2,
    );
    
    t_idx += t_step;
}
*/

// ============================================================================
// STEP 4: Add final reduction at end of NTT
// ============================================================================

// At the end of ntt_inplace, add:
/*
// Final reduction: bring all values from [0, 2q) to [0, q)
for x in a.iter_mut() {
    if *x >= self.q {
        *x -= self.q;
    }
}
*/

// ============================================================================
// STEP 5: Update intt_inplace similarly
// ============================================================================

// Same pattern for inverse NTT

// ============================================================================
// TESTS TO VERIFY
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    // All existing tests must pass:
    // - test_ntt_intt_roundtrip
    // - test_multiply_small
    // - test_negacyclic
    // - test_vs_schoolbook
    
    #[test]
    fn test_harvey_integration_roundtrip() {
        // After integration, verify NTT/INTT roundtrip still works
        let engine = NTTEngineFFT::new(998244353, 1024);
        let original: Vec<u64> = (0..1024).map(|i| i as u64).collect();
        
        let ntt = engine.ntt(&original);
        let recovered = engine.intt(&ntt);
        
        assert_eq!(recovered, original);
    }
    
    #[test]
    fn test_harvey_butterfly_benchmark() {
        use std::time::Instant;
        
        let mont = MontgomeryContext::new(998244353);
        let q = 998244353u64;
        let q2 = q << 1;
        
        let mut a = 12345u64;
        let mut b = 67890u64;
        let tw = mont.to_montgomery(3);
        
        let iterations = 1_000_000;
        let start = Instant::now();
        for _ in 0..iterations {
            harvey_butterfly_inline(&mut a, &mut b, tw, &mont, q, q2);
        }
        let elapsed = start.elapsed();
        
        let per_op_ns = elapsed.as_nanos() / iterations as u128;
        println!("Harvey butterfly: {} ns/op", per_op_ns);
        
        // GATE: must be under 25ns
        assert!(per_op_ns < 25, "Harvey butterfly too slow: {} ns", per_op_ns);
    }
}

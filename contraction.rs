//! Fourth Attractor with Exact Discrete Contraction Bound
//!
//! Implements the dithered fourth attractor step with formally proven
//! convergence guarantee: |Δ_{k+1}| ≤ ⌈|Δ_k|/4⌉
//!
//! # Theorem: Exact Discrete Contraction
//!
//! For the fourth attractor update rule:
//!   δ = sign(Δ) * ⌊(3/4)|Δ|⌋
//!   a_{k+1} = (a_k + δ) mod M
//!
//! with dither: if ⌊(3/4)|Δ|⌋ = 0 and Δ ≠ 0, then δ = sign(Δ)
//!
//! The contraction bound is:
//!   |Δ_{k+1}| ≤ ⌈|Δ_k|/4⌉
//!
//! # Proof
//!
//! For Δ_k > 0:
//!   Δ_{k+1} = Δ_k - ⌊(3/4)Δ_k⌋ = ⌈(1/4)Δ_k⌉
//!
//! The identity ⌊(3/4)n⌋ = n - ⌈n/4⌉ follows from:
//!   n = 4q + r where r ∈ {0,1,2,3}
//!   ⌊(3/4)n⌋ = ⌊3q + (3r/4)⌋ = 3q + ⌊3r/4⌋
//!   ⌈n/4⌉ = q + ⌈r/4⌉
//!   n - ⌈n/4⌉ = 3q + r - ⌈r/4⌉ = 3q + ⌊3r/4⌋ ✓
//!
//! Symmetry: The Δ_k < 0 case is symmetric by sign.
//!
//! # Convergence Time
//!
//! Worst-case from |Δ_0| ≤ M/2:
//!   Steps = ⌈log_4(M/2)⌉ + O(1) = O(log M)
//!
//! For M = 256: at most 4-5 steps from any starting distance.

use super::geodesic::signed_geodesic;

/// Fourth attractor step with exact integer arithmetic
///
/// Moves `current` toward `target` by 3/4 of the signed geodesic distance.
/// Uses dithering to ensure progress when distance rounds to zero.
///
/// # Contraction Guarantee
/// |Δ_{k+1}| ≤ ⌈|Δ_k|/4⌉
///
/// # Examples
/// ```
/// use qmnf::circular::fourth_attractor_step_dithered;
///
/// let m = 256u64;
/// let mut x = 100u64;
/// let target = 0u64;
///
/// // Will converge to target in O(log M) steps
/// for _ in 0..10 {
///     x = fourth_attractor_step_dithered(x, target, m);
/// }
/// assert_eq!(x, target);
/// ```
pub fn fourth_attractor_step_dithered(current: u64, target: u64, m: u64) -> u64 {
    debug_assert!(m > 0);
    debug_assert!(current < m);
    debug_assert!(target < m);
    
    if current == target {
        return current;
    }
    
    let delta = signed_geodesic(target, current, m);
    let abs_delta = delta.unsigned_abs();
    
    // Step size: ⌊(3/4)|Δ|⌋
    let mut step = (3 * abs_delta) / 4;
    
    // Dither: if step rounds to 0 but we're not at target, take unit step
    if step == 0 && delta != 0 {
        step = 1;
    }
    
    // Apply step in correct direction
    let signed_step = if delta > 0 {
        step as i64
    } else {
        -(step as i64)
    };
    
    ((current as i64 + signed_step).rem_euclid(m as i64)) as u64
}

/// Compute the next geodesic distance after one attractor step
///
/// Returns |Δ_{k+1}| given |Δ_k|
pub fn next_distance(abs_delta: u64) -> u64 {
    if abs_delta == 0 {
        return 0;
    }
    
    let step = (3 * abs_delta) / 4;
    
    if step == 0 {
        // Dither case: unit step when 0 < |Δ| < 4/3
        // For |Δ| ∈ {1}: step = 1, next_distance = 0
        abs_delta - 1
    } else {
        // Normal case: |Δ_{k+1}| = |Δ_k| - step = |Δ_k| - ⌊(3/4)|Δ_k|⌋ = ⌈|Δ_k|/4⌉
        abs_delta - step
    }
}

/// Exact contraction bound: ⌈|Δ|/4⌉
pub fn contraction_bound(abs_delta: u64) -> u64 {
    if abs_delta == 0 {
        0
    } else {
        // Ceiling division: ⌈n/4⌉ = (n + 3) / 4
        (abs_delta + 3) / 4
    }
}

/// Compute worst-case convergence steps from initial distance
///
/// Returns the maximum number of steps to reach target from
/// initial geodesic distance `initial_distance`.
pub fn convergence_steps(initial_distance: u64) -> usize {
    if initial_distance == 0 {
        return 0;
    }
    
    let mut d = initial_distance;
    let mut steps = 0;
    
    while d > 0 {
        d = next_distance(d);
        steps += 1;
    }
    
    steps
}

/// Worst-case convergence steps for modulus M
///
/// Returns max steps from any starting position.
pub fn max_convergence_steps(m: u64) -> usize {
    convergence_steps(m / 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::geodesic::geodesic_distance;
    
    /// CRITICAL TEST: Verify exact contraction bound for ALL distances
    #[test]
    fn test_exact_contraction_bound_exhaustive() {
        let m = 256u64;
        
        for delta_0 in 1..=(m / 2) {
            let delta_1 = next_distance(delta_0);
            let bound = contraction_bound(delta_0);
            
            assert!(
                delta_1 <= bound,
                "Contraction violated: Δ₀={}, Δ₁={}, bound=⌈Δ₀/4⌉={}",
                delta_0, delta_1, bound
            );
        }
    }
    
    /// Verify contraction bound is tight (actually achieved)
    #[test]
    fn test_contraction_bound_tight() {
        // The bound is tight when |Δ| ≡ 0 (mod 4)
        // In these cases: ⌈Δ/4⌉ = Δ/4 exactly
        
        for delta in [4u64, 8, 12, 16, 20, 100, 128] {
            let next = next_distance(delta);
            let bound = contraction_bound(delta);
            
            assert_eq!(
                next, bound,
                "Bound should be tight for Δ={}: next={}, bound={}",
                delta, next, bound
            );
        }
    }
    
    /// Test attractor step actually achieves the contraction
    #[test]
    fn test_attractor_step_contracts() {
        let m = 256u64;
        
        for initial in 0..m {
            for target in 0..m {
                if initial == target {
                    continue;
                }
                
                let d0 = geodesic_distance(initial, target, m);
                let next = fourth_attractor_step_dithered(initial, target, m);
                let d1 = geodesic_distance(next, target, m);
                let bound = contraction_bound(d0);
                
                assert!(
                    d1 <= bound,
                    "Step from {} toward {} (d0={}): d1={} > bound={}",
                    initial, target, d0, d1, bound
                );
            }
        }
    }
    
    /// Test convergence guarantee
    #[test]
    fn test_convergence_guaranteed() {
        let m = 256u64;
        
        for initial in 0..m {
            for target in 0..m {
                let mut x = initial;
                let max_steps = max_convergence_steps(m) + 1;
                
                for step in 0..max_steps {
                    if x == target {
                        break;
                    }
                    x = fourth_attractor_step_dithered(x, target, m);
                    
                    // Should never exceed max steps
                    assert!(
                        step < max_steps - 1,
                        "Failed to converge from {} to {} within {} steps",
                        initial, target, max_steps
                    );
                }
                
                assert_eq!(
                    x, target,
                    "Should converge: {} → {} but got {}",
                    initial, target, x
                );
            }
        }
    }
    
    /// Verify dither behavior for small distances
    #[test]
    fn test_dither_small_distances() {
        // For |Δ| = 1, 2, 3: ⌊(3/4)|Δ|⌋ = 0, 1, 2
        // Dither kicks in only for |Δ| = 1
        
        assert_eq!(next_distance(1), 0);  // Dithered
        assert_eq!(next_distance(2), 1);  // ⌊3*2/4⌋ = 1, so 2-1=1
        assert_eq!(next_distance(3), 1);  // ⌊3*3/4⌋ = 2, so 3-2=1
        assert_eq!(next_distance(4), 1);  // ⌊3*4/4⌋ = 3, so 4-3=1
    }
    
    /// Test the ceiling identity used in proof
    #[test]
    fn test_ceiling_identity() {
        // Verify: n - ⌊(3/4)n⌋ = ⌈n/4⌉ for all n
        
        for n in 1..1000u64 {
            let floor_three_quarters = (3 * n) / 4;
            let result = n - floor_three_quarters;
            let ceiling_quarter = (n + 3) / 4;
            
            assert_eq!(
                result, ceiling_quarter,
                "Identity failed for n={}: {} - {} = {} ≠ ⌈{}/4⌉ = {}",
                n, n, floor_three_quarters, result, n, ceiling_quarter
            );
        }
    }
    
    /// Test convergence time bound
    #[test]
    fn test_convergence_time_logarithmic() {
        // For M = 256, max distance = 128
        // log_4(128) ≈ 3.5, so expect 4-5 steps max
        
        let m = 256u64;
        let max_steps = max_convergence_steps(m);
        
        // Should be O(log_4 M) = O(log_4 256) = O(4)
        assert!(
            max_steps <= 6,
            "Max steps {} exceeds O(log M) bound for M={}",
            max_steps, m
        );
        
        // Verify the actual convergence path
        let mut d = 128u64;  // Max distance for M=256
        let mut path = vec![d];
        while d > 0 {
            d = next_distance(d);
            path.push(d);
        }
        
        // Path should be: 128 → 32 → 8 → 2 → 1 → 0 (5 steps)
        // or similar depending on ceiling rounding
        assert!(
            path.len() <= 7,
            "Convergence path {:?} too long",
            path
        );
    }
    
    /// Test larger modulus
    #[test]
    fn test_large_modulus() {
        let m = 1_000_000u64;
        
        // Max steps should still be O(log M)
        let max_steps = max_convergence_steps(m);
        
        // log_4(500000) ≈ 9.5, so expect ~10-12 steps
        assert!(
            max_steps <= 15,
            "Max steps {} for M={} exceeds logarithmic bound",
            max_steps, m
        );
    }
    
    /// Test specific convergence path
    #[test]
    fn test_convergence_path() {
        let m = 256u64;
        
        // Track the path from 100 to 0
        let mut x = 100u64;
        let target = 0u64;
        let mut path = vec![x];
        
        while x != target {
            x = fourth_attractor_step_dithered(x, target, m);
            path.push(x);
        }
        
        // Verify monotonic distance decrease
        for i in 1..path.len() {
            let d_prev = geodesic_distance(path[i - 1], target, m);
            let d_curr = geodesic_distance(path[i], target, m);
            assert!(
                d_curr < d_prev || d_curr == 0,
                "Distance not monotonic: {} → {} (d: {} → {})",
                path[i - 1], path[i], d_prev, d_curr
            );
        }
    }
    
    /// Verify Lyapunov descent: V(next) ≤ V(current) - 1
    #[test]
    fn test_lyapunov_descent() {
        let m = 256u64;
        
        // V(x) = geodesic_distance(x, target)
        // Must have V(next) ≤ V(current) - 1 when not at target
        
        for current in 0..m {
            let target = 0u64;
            if current == target {
                continue;
            }
            
            let v_current = geodesic_distance(current, target, m);
            let next = fourth_attractor_step_dithered(current, target, m);
            let v_next = geodesic_distance(next, target, m);
            
            assert!(
                v_next < v_current,
                "Lyapunov failed: V({})={} → V({})={} (should decrease)",
                current, v_current, next, v_next
            );
        }
    }
}

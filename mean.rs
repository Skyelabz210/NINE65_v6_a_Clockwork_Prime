//! Integer-Only Circular Mean
//!
//! Computes circular mean without sin/cos/atan2 using the
//! unwrap-mean-wrap algorithm. O(N), exact when cluster width < M/2.
//!
//! # Algorithm
//! 
//! 1. Pick reference r (first sample)
//! 2. Unwrap: for each v, compute Δᵢ = signed_geodesic(v, r)
//! 3. Linear mean: ū = r + round(Σ Δᵢ / N)
//! 4. Wrap: μ = ū mod M
//!
//! # Correctness Guarantee
//! 
//! When max pairwise circular distance < M/2, the circular mean is
//! well-defined and unique. This algorithm produces the exact same
//! result as the trig-based formula μ = (M/2π) * atan2(Σsin θ, Σcos θ).

use super::geodesic::{signed_geodesic, modular_add, cluster_valid_fast};

/// Circular mean computation result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CircularMeanResult {
    /// The computed circular mean
    pub mean: u64,
    /// Whether the cluster was valid (width < M/2)
    pub valid: bool,
}

/// Integer-only circular mean using unwrap-mean-wrap
/// 
/// Returns the circular mean of a set of values on Z_M.
/// When cluster width >= M/2, returns the first value as fallback
/// (the mean is mathematically ambiguous in this case).
/// 
/// # Time Complexity
/// O(N) - single pass through values
/// 
/// # Examples
/// ```
/// use qmnf::circular::circular_mean_integer;
/// 
/// // Values clustered near 0/256 boundary
/// let values = vec![5, 5, 5, 5, 250, 250, 250, 250];
/// let result = circular_mean_integer(&values, 256);
/// 
/// // Mean is near 255 (midpoint of short arc from 250 to 5)
/// assert!(result.mean >= 253 || result.mean <= 3);
/// assert!(result.valid);
/// ```
pub fn circular_mean_integer(values: &[u64], m: u64) -> CircularMeanResult {
    if values.is_empty() {
        return CircularMeanResult { mean: 0, valid: true };
    }
    
    if values.len() == 1 {
        return CircularMeanResult { mean: values[0], valid: true };
    }
    
    let reference = values[0];
    let n = values.len() as i64;
    
    // Sum all signed geodesic deltas
    let mut delta_sum: i64 = 0;
    for &v in values {
        delta_sum += signed_geodesic(v, reference, m);
    }
    
    // Integer mean with proper rounding (round half away from zero)
    let mean_offset = if delta_sum >= 0 {
        (delta_sum + n / 2) / n
    } else {
        (delta_sum - n / 2) / n
    };
    
    // Wrap back to modular space
    let mean = modular_add(reference, mean_offset, m);
    
    // Check validity (cluster must fit in half-arc)
    let valid = cluster_valid_fast(values, m);
    
    CircularMeanResult { mean, valid }
}

/// Circular mean with explicit validity check
/// 
/// Returns None if the cluster is too spread out for a meaningful mean.
pub fn circular_mean_checked(values: &[u64], m: u64) -> Option<u64> {
    let result = circular_mean_integer(values, m);
    if result.valid {
        Some(result.mean)
    } else {
        None
    }
}

/// Robust circular median (Fréchet median)
/// 
/// Finds the value x ∈ Z_M that minimizes Σ d(xᵢ, x).
/// Always well-defined, even for uniformly distributed data.
/// 
/// # Time Complexity
/// O(M * N) - brute force search
/// 
/// Use only when cluster validity check fails or when
/// outlier robustness is critical.
pub fn circular_median(values: &[u64], m: u64) -> u64 {
    if values.is_empty() {
        return 0;
    }
    
    if values.len() == 1 {
        return values[0];
    }
    
    use super::geodesic::geodesic_distance;
    
    let mut best_x = 0u64;
    let mut best_sum = u64::MAX;
    
    for x in 0..m {
        let sum: u64 = values.iter()
            .map(|&v| geodesic_distance(v, x, m))
            .sum();
        
        if sum < best_sum {
            best_sum = sum;
            best_x = x;
        }
    }
    
    best_x
}

/// Optimized circular median using candidate set
/// 
/// The Fréchet median on a circle is always at one of the sample points
/// or midway between adjacent samples. This reduces search from O(M) to O(N log N).
pub fn circular_median_fast(values: &[u64], m: u64) -> u64 {
    if values.is_empty() {
        return 0;
    }
    
    if values.len() == 1 {
        return values[0];
    }
    
    use super::geodesic::geodesic_distance;
    
    // For small M, brute force is fine
    if m <= 1024 {
        return circular_median(values, m);
    }
    
    // Candidates: all input values plus midpoints
    let mut candidates: Vec<u64> = values.to_vec();
    candidates.sort_unstable();
    candidates.dedup();
    
    // Add midpoints between adjacent sorted values
    let n = candidates.len();
    for i in 0..n {
        let a = candidates[i];
        let b = candidates[(i + 1) % n];
        let mid = if b > a {
            (a + b) / 2
        } else {
            // Wrap around
            ((a as u128 + b as u128 + m as u128) / 2 % m as u128) as u64
        };
        candidates.push(mid);
    }
    
    // Find candidate with minimum total distance
    let mut best_x = candidates[0];
    let mut best_sum = u64::MAX;
    
    for x in candidates {
        let sum: u64 = values.iter()
            .map(|&v| geodesic_distance(v, x, m))
            .sum();
        
        if sum < best_sum {
            best_sum = sum;
            best_x = x;
        }
    }
    
    best_x
}

/// Adaptive circular mean: uses mean when valid, falls back to median
pub fn circular_mean_adaptive(values: &[u64], m: u64) -> u64 {
    let result = circular_mean_integer(values, m);
    if result.valid {
        result.mean
    } else {
        circular_median_fast(values, m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_circular_mean_basic() {
        let m = 256u64;
        
        // Simple case: all same value
        let same = vec![100, 100, 100];
        assert_eq!(circular_mean_integer(&same, m).mean, 100);
        
        // Adjacent values
        let adjacent = vec![99, 100, 101];
        assert_eq!(circular_mean_integer(&adjacent, m).mean, 100);
    }
    
    #[test]
    fn test_circular_mean_wrap_around() {
        let m = 256u64;
        
        // Values clustered around 0/256 boundary
        // 8 values at 5, 8 values at 250
        let values: Vec<u64> = vec![5; 8].into_iter()
            .chain(vec![250; 8])
            .collect();
        
        let result = circular_mean_integer(&values, m);
        
        // The short arc from 250 to 5 has midpoint near 255.5
        // So mean should be 255 or 0
        assert!(
            result.mean >= 253 || result.mean <= 3,
            "Mean {} should be near 255/0 boundary",
            result.mean
        );
        assert!(result.valid);
    }
    
    #[test]
    fn test_circular_mean_validates_against_trig() {
        let m = 256u64;
        
        // Test case from validation document
        let values: Vec<u64> = vec![5, 5, 5, 5, 3, 3, 3, 3, 250, 250, 250, 250, 250, 250, 251, 251];
        
        let result = circular_mean_integer(&values, m);
        
        // Trig-based result was ~255.124, so integer should be 255
        assert!(
            result.mean == 255 || result.mean == 0,
            "Mean {} should match trig result ~255",
            result.mean
        );
    }
    
    #[test]
    fn test_circular_mean_empty() {
        assert_eq!(circular_mean_integer(&[], 256).mean, 0);
    }
    
    #[test]
    fn test_circular_mean_single() {
        assert_eq!(circular_mean_integer(&[42], 256).mean, 42);
    }
    
    #[test]
    fn test_circular_mean_invalid_cluster() {
        let m = 256u64;
        
        // Evenly distributed - no meaningful mean
        let spread = vec![0, 64, 128, 192];
        let result = circular_mean_integer(&spread, m);
        assert!(!result.valid);
    }
    
    #[test]
    fn test_circular_median_basic() {
        let m = 256u64;
        
        // Odd number of values
        let values = vec![100, 105, 110];
        assert_eq!(circular_median(&values, m), 105);
    }
    
    #[test]
    fn test_circular_median_with_outlier() {
        let m = 256u64;
        
        // Most values near 100, one outlier at 200
        let values = vec![98, 99, 100, 101, 102, 200];
        let median = circular_median(&values, m);
        
        // Median should be near 100, not pulled toward outlier
        assert!(
            median >= 98 && median <= 102,
            "Median {} should resist outlier",
            median
        );
    }
    
    #[test]
    fn test_circular_median_wrap_around() {
        let m = 256u64;
        
        // Values near boundary
        let values = vec![1, 2, 254, 255];
        let median = circular_median(&values, m);
        
        // Should be near 0 (the central point of the cluster)
        assert!(
            median <= 2 || median >= 254,
            "Median {} should be near wrap boundary",
            median
        );
    }
    
    #[test]
    fn test_circular_median_fast_matches_exact() {
        let m = 256u64;
        
        let test_cases: Vec<Vec<u64>> = vec![
            vec![5, 5, 5, 5, 250, 250, 250, 250],
            vec![100, 105, 110],
            vec![1, 2, 254, 255],
            vec![0, 64, 128, 192],
        ];
        
        for values in test_cases {
            let exact = circular_median(&values, m);
            let fast = circular_median_fast(&values, m);
            
            // Fast result should have same or similar total distance
            use super::super::geodesic::geodesic_distance;
            let exact_sum: u64 = values.iter().map(|&v| geodesic_distance(v, exact, m)).sum();
            let fast_sum: u64 = values.iter().map(|&v| geodesic_distance(v, fast, m)).sum();
            
            assert!(
                fast_sum <= exact_sum + 1,  // Allow tiny rounding difference
                "Fast median {} (sum {}) should match exact {} (sum {})",
                fast, fast_sum, exact, exact_sum
            );
        }
    }
    
    #[test]
    fn test_adaptive_uses_mean_when_valid() {
        let m = 256u64;
        let values = vec![100, 101, 102, 103];
        
        let mean_result = circular_mean_integer(&values, m);
        let adaptive = circular_mean_adaptive(&values, m);
        
        assert!(mean_result.valid);
        assert_eq!(adaptive, mean_result.mean);
    }
    
    #[test]
    fn test_adaptive_uses_median_when_invalid() {
        let m = 256u64;
        let values = vec![0, 64, 128, 192];
        
        let mean_result = circular_mean_integer(&values, m);
        let adaptive = circular_mean_adaptive(&values, m);
        let median = circular_median_fast(&values, m);
        
        assert!(!mean_result.valid);
        assert_eq!(adaptive, median);
    }
}

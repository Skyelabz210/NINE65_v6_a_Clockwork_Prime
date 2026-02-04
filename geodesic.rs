//! Integer Geodesic Arithmetic
//!
//! Core primitives for circular/modular distance calculations.
//! All operations are exact integers - no floating point.

/// Unsigned geodesic distance on Z_M (shortest arc)
/// 
/// Returns the minimum of |a - b| and M - |a - b|
/// 
/// # Examples
/// ```
/// use qmnf::circular::geodesic_distance;
/// assert_eq!(geodesic_distance(5, 250, 256), 11);  // Short arc through 0
/// assert_eq!(geodesic_distance(100, 150, 256), 50); // Direct path
/// ```
#[inline]
pub fn geodesic_distance(a: u64, b: u64, m: u64) -> u64 {
    debug_assert!(m > 0, "Modulus must be positive");
    debug_assert!(a < m, "Value a must be in [0, m)");
    debug_assert!(b < m, "Value b must be in [0, m)");
    
    let diff = if a >= b { a - b } else { b - a };
    diff.min(m - diff)
}

/// Signed geodesic difference: shortest path from b to a on Z_M
/// 
/// Returns value in [-M/2, M/2] representing the signed shortest arc.
/// Positive means a is "ahead" of b (clockwise), negative means "behind".
/// 
/// # Algorithm
/// ```text
/// Δ_min(a, b) = 
///   a - b           if |a - b| ≤ M/2
///   a - b - M       if a - b > M/2  
///   a - b + M       if a - b < -M/2
/// ```
/// 
/// # Examples
/// ```
/// use qmnf::circular::signed_geodesic;
/// assert_eq!(signed_geodesic(5, 250, 256), 11);   // 5 is 11 ahead of 250 (through 0)
/// assert_eq!(signed_geodesic(250, 5, 256), -11);  // 250 is 11 behind 5
/// assert_eq!(signed_geodesic(100, 50, 256), 50);  // Direct difference
/// ```
#[inline]
pub fn signed_geodesic(a: u64, b: u64, m: u64) -> i64 {
    debug_assert!(m > 0, "Modulus must be positive");
    debug_assert!(a < m, "Value a must be in [0, m)");
    debug_assert!(b < m, "Value b must be in [0, m)");
    
    let diff = (a as i64) - (b as i64);
    let half = (m / 2) as i64;
    
    if diff > half {
        diff - (m as i64)
    } else if diff < -half {
        diff + (m as i64)
    } else {
        diff
    }
}

/// Modular addition with proper wrapping
/// 
/// Computes (a + delta) mod m where delta can be negative.
#[inline]
pub fn modular_add(a: u64, delta: i64, m: u64) -> u64 {
    ((a as i64 + delta).rem_euclid(m as i64)) as u64
}

/// Check if a set of values has cluster width < M/2
/// 
/// This is the precondition for meaningful circular mean.
/// Returns (is_valid, max_pairwise_distance)
pub fn cluster_valid(values: &[u64], m: u64) -> (bool, u64) {
    if values.len() <= 1 {
        return (true, 0);
    }
    
    let mut max_dist = 0u64;
    for i in 0..values.len() {
        for j in (i + 1)..values.len() {
            let d = geodesic_distance(values[i], values[j], m);
            max_dist = max_dist.max(d);
        }
    }
    
    (max_dist < m / 2, max_dist)
}

/// Optimized cluster validity check using reference unwrapping
/// 
/// O(N) instead of O(N²) - checks if all values fit in a half-arc
/// from the reference point.
pub fn cluster_valid_fast(values: &[u64], m: u64) -> bool {
    if values.len() <= 1 {
        return true;
    }
    
    let reference = values[0];
    let half = (m / 2) as i64;
    
    let mut min_delta = 0i64;
    let mut max_delta = 0i64;
    
    for &v in values.iter() {
        let delta = signed_geodesic(v, reference, m);
        min_delta = min_delta.min(delta);
        max_delta = max_delta.max(delta);
    }
    
    // Cluster fits in half-arc if span < M/2
    (max_delta - min_delta) < half as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_geodesic_distance_basic() {
        let m = 256u64;
        
        // Direct distances
        assert_eq!(geodesic_distance(0, 50, m), 50);
        assert_eq!(geodesic_distance(50, 0, m), 50);
        assert_eq!(geodesic_distance(100, 150, m), 50);
        
        // Wrap-around distances
        assert_eq!(geodesic_distance(5, 250, m), 11);  // 256 - 245 = 11
        assert_eq!(geodesic_distance(250, 5, m), 11);
        assert_eq!(geodesic_distance(0, 255, m), 1);
        assert_eq!(geodesic_distance(1, 255, m), 2);
    }
    
    #[test]
    fn test_geodesic_distance_symmetric() {
        let m = 256u64;
        for a in 0..m {
            for b in 0..m {
                assert_eq!(
                    geodesic_distance(a, b, m),
                    geodesic_distance(b, a, m),
                    "Distance must be symmetric: d({}, {}) != d({}, {})",
                    a, b, b, a
                );
            }
        }
    }
    
    #[test]
    fn test_signed_geodesic_basic() {
        let m = 256u64;
        
        // Direct differences
        assert_eq!(signed_geodesic(100, 50, m), 50);
        assert_eq!(signed_geodesic(50, 100, m), -50);
        
        // Wrap-around differences
        assert_eq!(signed_geodesic(5, 250, m), 11);   // 5 is 11 ahead of 250
        assert_eq!(signed_geodesic(250, 5, m), -11);  // 250 is 11 behind 5
        
        // Edge cases
        assert_eq!(signed_geodesic(0, 128, m), -128); // Exactly half
        assert_eq!(signed_geodesic(128, 0, m), 128);
    }
    
    #[test]
    fn test_signed_geodesic_antisymmetric() {
        let m = 256u64;
        for a in 0..m {
            for b in 0..m {
                if a != b {
                    assert_eq!(
                        signed_geodesic(a, b, m),
                        -signed_geodesic(b, a, m),
                        "Signed geodesic must be antisymmetric"
                    );
                }
            }
        }
    }
    
    #[test]
    fn test_modular_add() {
        let m = 256u64;
        
        assert_eq!(modular_add(100, 50, m), 150);
        assert_eq!(modular_add(100, -50, m), 50);
        assert_eq!(modular_add(250, 20, m), 14);   // Wraps
        assert_eq!(modular_add(10, -20, m), 246);  // Wraps negative
    }
    
    #[test]
    fn test_cluster_valid() {
        let m = 256u64;
        
        // Valid cluster (all near each other)
        let valid = vec![5, 3, 7, 250, 252];  // All within ~14 of each other via wrap
        let (is_valid, max_dist) = cluster_valid(&valid, m);
        assert!(is_valid);
        assert!(max_dist < m / 2);
        
        // Invalid cluster (spread around the circle)
        let invalid = vec![0, 64, 128, 192];  // Evenly distributed
        let (is_valid, _) = cluster_valid(&invalid, m);
        assert!(!is_valid);
    }
    
    #[test]
    fn test_cluster_valid_fast_matches_exact() {
        let m = 256u64;
        
        // Test many random-ish clusters
        let test_cases: Vec<Vec<u64>> = vec![
            vec![5, 3, 7, 250, 252],
            vec![0, 64, 128, 192],
            vec![100, 105, 110, 115],
            vec![1, 2, 3, 4, 5],
            vec![250, 251, 252, 253, 254, 255, 0, 1, 2, 3],
        ];
        
        for values in test_cases {
            let (exact_valid, _) = cluster_valid(&values, m);
            let fast_valid = cluster_valid_fast(&values, m);
            assert_eq!(exact_valid, fast_valid, "Mismatch for {:?}", values);
        }
    }
}

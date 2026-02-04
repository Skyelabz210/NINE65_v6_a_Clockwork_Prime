//! Integer Confidence Metric
//!
//! Measures circular coherence without sin/cos.
//! Uses mean absolute geodesic deviation as proxy for resultant length R.
//!
//! # Theory
//! 
//! The trig-based resultant length is:
//!   R = |Σ e^{iθ}| / N = √((Σ sin θ)² + (Σ cos θ)²) / N
//!
//! This requires floating-point trig. Instead, we use:
//!   D = (1/N) Σ d(xᵢ, μ)   (mean absolute deviation)
//!   conf = max(0, 1 - D / (M/4))
//!
//! This is monotonically related to R in the high-coherence regime
//! and exactly computable with integers.

use super::geodesic::geodesic_distance;
use super::mean::circular_mean_integer;

/// Confidence result with diagnostic info
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConfidenceResult {
    /// Confidence in milliunits [0, 1000] (1000 = perfect coherence)
    pub confidence_milliunits: u64,
    /// Mean absolute deviation in modular units
    pub mean_deviation: u64,
    /// Maximum possible meaningful deviation (M/4)
    pub max_deviation: u64,
}

impl ConfidenceResult {
    /// Get confidence as percentage [0, 100] for display
    /// (Use confidence_milliunits directly for full precision)
    pub fn as_percent(&self) -> u64 {
        self.confidence_milliunits / 10
    }
    
    /// Check if confidence exceeds threshold (in milliunits)
    pub fn exceeds(&self, threshold_milliunits: u64) -> bool {
        self.confidence_milliunits >= threshold_milliunits
    }
}

/// Compute integer confidence metric for a set of values
/// 
/// Returns confidence in milliunits [0, 1000] where:
/// - 1000 = all values identical (perfect coherence)
/// - 0 = values spread across M/4 or more (no coherence)
/// 
/// # Algorithm
/// 1. Compute circular mean μ
/// 2. Compute mean absolute deviation D = (1/N) Σ d(xᵢ, μ)
/// 3. Map to confidence: conf = max(0, 1 - D/(M/4))
/// 
/// # Examples
/// ```
/// use qmnf::circular::confidence_integer;
/// 
/// // Highly coherent values
/// let tight = vec![100, 101, 100, 99, 100];
/// let conf = confidence_integer(&tight, 256);
/// assert!(conf.confidence_milliunits > 950);  // > 0.95
/// 
/// // Spread out values
/// let spread = vec![0, 64, 128, 192];
/// let conf = confidence_integer(&spread, 256);
/// assert!(conf.confidence_milliunits < 100);  // < 0.10
/// ```
pub fn confidence_integer(values: &[u64], m: u64) -> ConfidenceResult {
    let max_deviation = m / 4;
    
    if values.is_empty() {
        return ConfidenceResult {
            confidence_milliunits: 1000,
            mean_deviation: 0,
            max_deviation,
        };
    }
    
    if values.len() == 1 {
        return ConfidenceResult {
            confidence_milliunits: 1000,
            mean_deviation: 0,
            max_deviation,
        };
    }
    
    // Compute circular mean
    let mean_result = circular_mean_integer(values, m);
    let mean = mean_result.mean;
    
    // Compute total absolute deviation
    let mut total_deviation: u64 = 0;
    for &v in values {
        total_deviation += geodesic_distance(v, mean, m);
    }
    
    // Mean deviation
    let mean_deviation = total_deviation / values.len() as u64;
    
    // Map to confidence in milliunits
    let confidence_milliunits = if mean_deviation >= max_deviation {
        0
    } else {
        // conf = 1 - (D / max_D) = (max_D - D) / max_D
        // In milliunits: (max_D - D) * 1000 / max_D
        ((max_deviation - mean_deviation) * 1000) / max_deviation
    };
    
    ConfidenceResult {
        confidence_milliunits,
        mean_deviation,
        max_deviation,
    }
}

/// Compute confidence with a pre-computed mean
/// 
/// Use when you've already computed the circular mean and
/// don't want to recompute it.
pub fn confidence_with_mean(values: &[u64], mean: u64, m: u64) -> ConfidenceResult {
    let max_deviation = m / 4;
    
    if values.is_empty() || values.len() == 1 {
        return ConfidenceResult {
            confidence_milliunits: 1000,
            mean_deviation: 0,
            max_deviation,
        };
    }
    
    let mut total_deviation: u64 = 0;
    for &v in values {
        total_deviation += geodesic_distance(v, mean, m);
    }
    
    let mean_deviation = total_deviation / values.len() as u64;
    
    let confidence_milliunits = if mean_deviation >= max_deviation {
        0
    } else {
        ((max_deviation - mean_deviation) * 1000) / max_deviation
    };
    
    ConfidenceResult {
        confidence_milliunits,
        mean_deviation,
        max_deviation,
    }
}

/// Circular variance (complement of confidence)
/// 
/// V = 1 - R in the trig formulation.
/// Here: V = D / (M/4) capped at 1
/// 
/// Returns variance in milliunits [0, 1000]
pub fn circular_variance_integer(values: &[u64], m: u64) -> u64 {
    let conf = confidence_integer(values, m);
    1000 - conf.confidence_milliunits
}

/// Quick coherence check: is confidence above threshold?
/// 
/// Threshold is in milliunits (e.g., 900 = 0.90)
pub fn is_coherent(values: &[u64], m: u64, threshold_milliunits: u64) -> bool {
    confidence_integer(values, m).confidence_milliunits >= threshold_milliunits
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_confidence_perfect() {
        let m = 256u64;
        
        // All identical
        let identical = vec![100, 100, 100, 100];
        let conf = confidence_integer(&identical, m);
        assert_eq!(conf.confidence_milliunits, 1000);
        assert_eq!(conf.mean_deviation, 0);
    }
    
    #[test]
    fn test_confidence_high() {
        let m = 256u64;
        
        // Tight cluster
        let tight = vec![100, 101, 99, 100, 102, 98];
        let conf = confidence_integer(&tight, m);
        
        // Mean deviation should be ~1-2, max_deviation = 64
        // conf = (64 - ~1.5) / 64 ≈ 0.977
        assert!(
            conf.confidence_milliunits > 950,
            "Confidence {} should be > 950",
            conf.confidence_milliunits
        );
    }
    
    #[test]
    fn test_confidence_wrap_around() {
        let m = 256u64;
        
        // Values clustered near 0/256 boundary (the EPRAM test case)
        let values: Vec<u64> = vec![5; 8].into_iter()
            .chain(vec![250; 8])
            .collect();
        
        let conf = confidence_integer(&values, m);
        
        // The cluster spans ~11 units, max_deviation = 64
        // Expected conf ≈ (64 - 5.5) / 64 ≈ 0.914
        assert!(
            conf.confidence_milliunits > 850,
            "Confidence {} should be > 850 for wrap-around cluster",
            conf.confidence_milliunits
        );
    }
    
    #[test]
    fn test_confidence_matches_validation() {
        let m = 256u64;
        
        // Exact test case from validation document:
        // Row 0: [5, 5, 5, 5], Row 1: [3, 3, 3, 3]
        // Row 2: [251, 250, 250, 251], Row 3: [250, 250, 250, 250]
        let values = vec![5, 5, 5, 5, 3, 3, 3, 3, 251, 250, 250, 251, 250, 250, 250, 250];
        
        let conf = confidence_integer(&values, m);
        
        // Validation document reports R ≈ 0.9927
        // Our integer confidence should be similar (within measurement error)
        // Since we use deviation-based metric, expect ~950-1000
        assert!(
            conf.confidence_milliunits > 900,
            "Confidence {} should match ~0.9927 coherence",
            conf.confidence_milliunits
        );
    }
    
    #[test]
    fn test_confidence_low() {
        let m = 256u64;
        
        // Evenly distributed around circle
        let spread = vec![0, 64, 128, 192];
        let conf = confidence_integer(&spread, m);
        
        // Mean deviation should be ~64 = max_deviation
        // So confidence should be near 0
        assert!(
            conf.confidence_milliunits < 200,
            "Confidence {} should be low for uniform distribution",
            conf.confidence_milliunits
        );
    }
    
    #[test]
    fn test_confidence_empty() {
        let conf = confidence_integer(&[], 256);
        assert_eq!(conf.confidence_milliunits, 1000);
    }
    
    #[test]
    fn test_confidence_single() {
        let conf = confidence_integer(&[42], 256);
        assert_eq!(conf.confidence_milliunits, 1000);
    }
    
    #[test]
    fn test_confidence_as_percent() {
        let m = 256u64;
        let values = vec![100, 100, 100];
        let conf = confidence_integer(&values, m);
        
        let p = conf.as_percent();
        assert_eq!(p, 100);
    }
    
    #[test]
    fn test_variance_complement() {
        let m = 256u64;
        
        let values = vec![100, 105, 110, 95, 90];
        let conf = confidence_integer(&values, m);
        let var = circular_variance_integer(&values, m);
        
        assert_eq!(conf.confidence_milliunits + var, 1000);
    }
    
    #[test]
    fn test_is_coherent() {
        let m = 256u64;
        
        let tight = vec![100, 101, 99];
        assert!(is_coherent(&tight, m, 900));
        
        let spread = vec![0, 64, 128, 192];
        assert!(!is_coherent(&spread, m, 500));
    }
    
    #[test]
    fn test_confidence_with_mean() {
        let m = 256u64;
        let values = vec![100, 101, 99, 100, 102];
        
        let mean_result = circular_mean_integer(&values, m);
        let conf1 = confidence_integer(&values, m);
        let conf2 = confidence_with_mean(&values, mean_result.mean, m);
        
        assert_eq!(conf1.confidence_milliunits, conf2.confidence_milliunits);
    }
}

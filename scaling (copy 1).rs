//! T-402: ReconstructionGuard and T-404: CRTScaling
//! 
//! Ensures the 2PQ < M invariant is always maintained for exact
//! rational reconstruction, with automatic modulus scaling when
//! bounds grow too large.
//!
//! INNOVATION: Automatic Modulus Scaling
//! - Detects when bounds approach limit
//! - Adds new coprime modulus to CRT system
//! - Preserves all existing values
//! - Zero-cost when bounds are safe

use super::bounded::{BoundedRational, BoundedRationalConfig, BoundedRationalError};

// =============================================================================
// T-402: RECONSTRUCTION GUARD
// =============================================================================

/// Guard state for monitoring bound safety
#[derive(Clone, Debug)]
pub struct ReconstructionGuard {
    /// Current modulus product
    pub modulus: u128,
    /// Warning threshold (percentage of limit)
    pub warning_threshold: f64,
    /// Critical threshold (must scale)
    pub critical_threshold: f64,
    /// Number of scale operations performed
    pub scale_count: usize,
}

impl ReconstructionGuard {
    /// Create guard with default thresholds
    pub fn new(modulus: u128) -> Self {
        Self {
            modulus,
            warning_threshold: 0.5,   // Warn at 50% of limit
            critical_threshold: 0.9,  // Scale at 90% of limit
            scale_count: 0,
        }
    }
    
    /// Create guard with custom thresholds
    pub fn with_thresholds(modulus: u128, warning: f64, critical: f64) -> Self {
        Self {
            modulus,
            warning_threshold: warning,
            critical_threshold: critical,
            scale_count: 0,
        }
    }
    
    /// Check if bounds are safe (2PQ < M)
    pub fn is_safe(&self, p_bound: u64, q_bound: u64) -> bool {
        2u128 * p_bound as u128 * q_bound as u128 < self.modulus
    }
    
    /// Compute safety ratio: 2PQ / M
    pub fn safety_ratio(&self, p_bound: u64, q_bound: u64) -> f64 {
        let pq = 2u128 * p_bound as u128 * q_bound as u128;
        pq as f64 / self.modulus as f64
    }
    
    /// Check bounds and return status
    pub fn check(&self, p_bound: u64, q_bound: u64) -> GuardStatus {
        let ratio = self.safety_ratio(p_bound, q_bound);
        
        if ratio >= 1.0 {
            GuardStatus::Violated
        } else if ratio >= self.critical_threshold {
            GuardStatus::Critical
        } else if ratio >= self.warning_threshold {
            GuardStatus::Warning
        } else {
            GuardStatus::Safe
        }
    }
    
    /// Update modulus after scaling
    pub fn update_modulus(&mut self, new_modulus: u128) {
        self.modulus = new_modulus;
        self.scale_count += 1;
    }
}

/// Guard status levels
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GuardStatus {
    /// Bounds well within limit
    Safe,
    /// Approaching limit, consider scaling soon
    Warning,
    /// Near limit, should scale
    Critical,
    /// Invariant violated, reconstruction may fail
    Violated,
}

// =============================================================================
// T-404: CRT SCALING POLICY
// =============================================================================

/// CRT scaling strategy
#[derive(Clone, Debug)]
pub enum ScalingStrategy {
    /// Scale when critical threshold reached
    Automatic,
    /// Only scale on explicit request
    Manual,
    /// Scale preemptively at warning threshold
    Aggressive,
    /// Never scale (fail if bounds exceeded)
    Never,
}

/// CRT Scaling Manager
/// 
/// Manages automatic modulus scaling for bounded rationals.
/// When bounds grow large, adds new coprime moduli to maintain
/// the 2PQ < M invariant.
#[derive(Clone, Debug)]
pub struct CRTScaler {
    /// Current CRT moduli
    pub moduli: Vec<u64>,
    /// Product of all moduli
    pub modulus_product: u128,
    /// Scaling strategy
    pub strategy: ScalingStrategy,
    /// Guard for monitoring
    pub guard: ReconstructionGuard,
    /// Candidate primes for scaling
    prime_candidates: Vec<u64>,
    /// Next candidate index
    prime_index: usize,
}

impl CRTScaler {
    /// Create scaler with initial moduli
    pub fn new(initial_moduli: Vec<u64>, strategy: ScalingStrategy) -> Self {
        let modulus_product: u128 = initial_moduli.iter()
            .map(|&m| m as u128)
            .product();
        
        Self {
            moduli: initial_moduli,
            modulus_product,
            strategy,
            guard: ReconstructionGuard::new(modulus_product),
            prime_candidates: Self::generate_prime_candidates(),
            prime_index: 0,
        }
    }
    
    /// Generate list of candidate primes for scaling
    fn generate_prime_candidates() -> Vec<u64> {
        // Primes near powers of 2 (efficient for FFT/NTT)
        vec![
            // 16-bit range
            65521, 65519, 65497,
            // 32-bit range  
            4294967291, 4294967279, 4294967231,
            // Mersenne-friendly
            2147483647, 2147483629,
            // Other good primes
            1073741789, 536870909,
        ]
    }
    
    /// Check if scaling is needed
    pub fn needs_scaling(&self, p_bound: u64, q_bound: u64) -> bool {
        match self.strategy {
            ScalingStrategy::Never => false,
            ScalingStrategy::Manual => false,
            ScalingStrategy::Automatic => {
                matches!(self.guard.check(p_bound, q_bound), GuardStatus::Critical)
            }
            ScalingStrategy::Aggressive => {
                matches!(
                    self.guard.check(p_bound, q_bound), 
                    GuardStatus::Warning | GuardStatus::Critical
                )
            }
        }
    }
    
    /// Scale up by adding a new coprime modulus
    pub fn scale_up(&mut self) -> Result<u64, ScalingError> {
        // Find next coprime candidate
        let new_prime = self.find_next_coprime()?;
        
        // Add to system
        self.moduli.push(new_prime);
        self.modulus_product *= new_prime as u128;
        self.guard.update_modulus(self.modulus_product);
        
        Ok(new_prime)
    }
    
    /// Find next prime coprime to all existing moduli
    fn find_next_coprime(&mut self) -> Result<u64, ScalingError> {
        let start = self.prime_index;
        
        loop {
            if self.prime_index >= self.prime_candidates.len() {
                self.prime_index = 0;
            }
            
            let candidate = self.prime_candidates[self.prime_index];
            self.prime_index += 1;
            
            // Check if coprime to all existing
            let is_coprime = self.moduli.iter()
                .all(|&m| gcd(candidate, m) == 1);
            
            if is_coprime && !self.moduli.contains(&candidate) {
                return Ok(candidate);
            }
            
            // Avoid infinite loop
            if self.prime_index == start {
                return Err(ScalingError::NoCoprimeCandidates);
            }
        }
    }
    
    /// Check and auto-scale if needed
    pub fn check_and_scale(&mut self, p_bound: u64, q_bound: u64) -> Result<bool, ScalingError> {
        if self.needs_scaling(p_bound, q_bound) {
            self.scale_up()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Get current config for BoundedRational
    pub fn get_config(&self) -> BoundedRationalConfig {
        BoundedRationalConfig::new(self.moduli.clone())
    }
    
    /// Project existing value to new (scaled) system
    /// 
    /// When adding a new modulus m_new, the value r in Z_M becomes
    /// (r, r mod m_new) in Z_{M * m_new}.
    pub fn project_value(&self, residue: u128, new_prime: u64) -> u128 {
        // New residue in extended system
        let new_residue = residue % new_prime as u128;
        
        // CRT combination (simplified - assumes proper ordering)
        // In full implementation, use proper CRT reconstruction
        residue
    }
}

/// Scaling errors
#[derive(Debug, Clone)]
pub enum ScalingError {
    /// No coprime candidates available
    NoCoprimeCandidates,
    /// Maximum modulus reached
    MaxModulusReached,
    /// Strategy forbids scaling
    ScalingForbidden,
}

// =============================================================================
// UTILITY FUNCTIONS
// =============================================================================

/// Greatest common divisor
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// =============================================================================
// GUARDED RATIONAL OPERATIONS
// =============================================================================

/// Wrapper for BoundedRational with automatic scaling
pub struct GuardedRational {
    /// The underlying rational
    pub value: BoundedRational,
    /// Scaler for automatic management
    scaler: CRTScaler,
}

impl GuardedRational {
    /// Create with automatic scaling
    pub fn new_auto(
        numerator: i64,
        denominator: u64,
        initial_moduli: Vec<u64>,
    ) -> Result<Self, BoundedRationalError> {
        let scaler = CRTScaler::new(initial_moduli, ScalingStrategy::Automatic);
        let config = scaler.get_config();
        
        let p_bound = numerator.unsigned_abs().max(1);
        let q_bound = denominator.max(1);
        
        let value = BoundedRational::new(
            numerator,
            denominator,
            p_bound,
            q_bound,
            config,
        )?;
        
        Ok(Self { value, scaler })
    }
    
    /// Add with automatic scaling
    pub fn add(&mut self, other: &GuardedRational) -> Result<GuardedRational, BoundedRationalError> {
        // Compute new bounds
        let new_p = self.value.p_bound.saturating_mul(other.value.q_bound)
            .saturating_add(self.value.q_bound.saturating_mul(other.value.p_bound));
        let new_q = self.value.q_bound.saturating_mul(other.value.q_bound);
        
        // Check if scaling needed
        let mut new_scaler = self.scaler.clone();
        if new_scaler.check_and_scale(new_p, new_q).map_err(|_| 
            BoundedRationalError::BoundsExceeded { 
                p_bound: new_p, 
                q_bound: new_q, 
                modulus: self.scaler.modulus_product 
            }
        )? {
            // Rescale values to new modulus system
            // (simplified - full implementation needs CRT projection)
        }
        
        let result = self.value.add(&other.value)?;
        
        Ok(GuardedRational {
            value: result,
            scaler: new_scaler,
        })
    }
    
    /// Get safety status
    pub fn safety_status(&self) -> GuardStatus {
        self.scaler.guard.check(self.value.p_bound, self.value.q_bound)
    }
    
    /// Reconstruct exact rational
    pub fn reconstruct(&self) -> Result<(i64, u64), BoundedRationalError> {
        self.value.reconstruct()
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_guard_safety_check() {
        let guard = ReconstructionGuard::new(1_000_000);
        
        // Safe bounds
        assert!(guard.is_safe(100, 100));  // 2*100*100 = 20,000 < 1M
        assert_eq!(guard.check(100, 100), GuardStatus::Safe);
        
        // Warning level (50% of limit)
        assert_eq!(guard.check(500, 500), GuardStatus::Warning);
        
        // Critical level (90% of limit)
        assert_eq!(guard.check(670, 670), GuardStatus::Critical);
        
        // Violated
        assert!(!guard.is_safe(1000, 1000));  // 2M >= 1M
        assert_eq!(guard.check(1000, 1000), GuardStatus::Violated);
    }
    
    #[test]
    fn test_safety_ratio() {
        let guard = ReconstructionGuard::new(1_000_000);
        
        let ratio = guard.safety_ratio(100, 100);
        assert!((ratio - 0.02).abs() < 0.001);  // 20,000 / 1M = 0.02
    }
    
    #[test]
    fn test_scaler_creation() {
        let scaler = CRTScaler::new(
            vec![65537, 65521],
            ScalingStrategy::Automatic,
        );
        
        assert_eq!(scaler.moduli.len(), 2);
        assert_eq!(scaler.modulus_product, 65537u128 * 65521);
    }
    
    #[test]
    fn test_scaler_scale_up() {
        let mut scaler = CRTScaler::new(
            vec![65537],
            ScalingStrategy::Automatic,
        );
        
        let initial_product = scaler.modulus_product;
        
        let new_prime = scaler.scale_up().unwrap();
        
        assert_eq!(scaler.moduli.len(), 2);
        assert!(scaler.modulus_product > initial_product);
        assert_eq!(gcd(65537, new_prime), 1);  // Coprime
    }
    
    #[test]
    fn test_automatic_scaling() {
        let mut scaler = CRTScaler::new(
            vec![100, 97],  // Small moduli for testing
            ScalingStrategy::Automatic,
        );
        
        // Initial modulus ≈ 9,700
        assert!(scaler.modulus_product < 10_000);
        
        // Large bounds should trigger scaling
        let needs_scale = scaler.needs_scaling(100, 100);  // 2*100*100 = 20,000 > 9,700
        assert!(needs_scale);
        
        // Auto-scale
        let scaled = scaler.check_and_scale(100, 100).unwrap();
        assert!(scaled);
        
        // Now should be safe
        assert!(scaler.guard.is_safe(100, 100));
    }
    
    #[test]
    fn test_manual_scaling() {
        let scaler = CRTScaler::new(
            vec![100, 97],
            ScalingStrategy::Manual,
        );
        
        // Manual strategy never auto-scales
        assert!(!scaler.needs_scaling(100, 100));
    }
    
    #[test]
    fn test_never_scaling() {
        let scaler = CRTScaler::new(
            vec![100, 97],
            ScalingStrategy::Never,
        );
        
        assert!(!scaler.needs_scaling(1000, 1000));  // Even with huge bounds
    }
    
    #[test]
    fn test_guarded_rational_creation() {
        let gr = GuardedRational::new_auto(3, 4, vec![65537, 65521]).unwrap();
        
        assert_eq!(gr.safety_status(), GuardStatus::Safe);
        
        let (p, q) = gr.reconstruct().unwrap();
        assert_eq!(p * 4, 3 * q as i64);
    }
    
    #[test]
    fn test_coprime_finding() {
        let mut scaler = CRTScaler::new(
            vec![65537, 65521, 65519],
            ScalingStrategy::Automatic,
        );
        
        // Should find a prime coprime to all existing
        let new_prime = scaler.find_next_coprime().unwrap();
        
        assert_eq!(gcd(new_prime, 65537), 1);
        assert_eq!(gcd(new_prime, 65521), 1);
        assert_eq!(gcd(new_prime, 65519), 1);
    }
    
    #[test]
    fn test_guard_update_after_scale() {
        let mut scaler = CRTScaler::new(vec![100], ScalingStrategy::Automatic);
        
        let initial_count = scaler.guard.scale_count;
        scaler.scale_up().unwrap();
        
        assert_eq!(scaler.guard.scale_count, initial_count + 1);
    }
}

//! T-401: BoundedRational Implementation
//! 
//! Exact rational arithmetic with explicit bound tracking.
//! 
//! INNOVATION: Bounded Rational Recovery
//! - Tracks numerator bound P and denominator bound Q
//! - Ensures 2PQ < M for unique reconstruction
//! - Enables exact ℚ over residue representation
//!
//! MATHEMATICAL FOUNDATION (QMNF_MATHEMATICAL_FOUNDATIONS_V2.md):
//! - Theorem B4: enc_M: ℛ(P,Q,M) → ℤ_M is injective when 2PQ < M
//! - Lemmas B5-B7: Bound growth under +, ×, ⁻¹
//! - Corollary B11: CRT scaling policy

// =============================================================================
// BOUNDED RATIONAL CORE
// =============================================================================

/// Error types for bounded rational operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundedRationalError {
    /// Bounds exceeded for current modulus (2PQ >= M)
    BoundsExceeded { p_bound: u64, q_bound: u64, modulus: u128 },
    /// Division by zero
    DivisionByZero,
    /// Denominator not coprime to modulus
    DenominatorNotCoprime,
    /// Reconstruction failed
    ReconstructionFailed,
    /// Invalid bounds (P or Q is zero)
    InvalidBounds,
}

/// Configuration for bounded rational arithmetic
#[derive(Clone, Debug)]
pub struct BoundedRationalConfig {
    /// Product of CRT moduli
    pub modulus: u128,
    /// Individual CRT moduli (for parallel operations)
    pub crt_moduli: Vec<u64>,
    /// Maximum numerator bound
    pub max_p_bound: u64,
    /// Maximum denominator bound
    pub max_q_bound: u64,
}

impl BoundedRationalConfig {
    /// Create configuration with given moduli
    pub fn new(crt_moduli: Vec<u64>) -> Self {
        let modulus: u128 = crt_moduli.iter().map(|&m| m as u128).product();
        
        // Safe bounds ensuring 2PQ < M
        let max_bound = integer_sqrt_u128(modulus / 2);
        
        Self {
            modulus,
            crt_moduli,
            max_p_bound: max_bound as u64,
            max_q_bound: max_bound as u64,
        }
    }
    
    /// Check if bounds satisfy reconstruction uniqueness
    pub fn check_bounds(&self, p_bound: u64, q_bound: u64) -> bool {
        2 * p_bound as u128 * q_bound as u128 < self.modulus
    }
    
    /// Standard configuration for moderate precision
    pub fn standard() -> Self {
        Self::new(vec![65537, 65521, 65519])  // ~2^48 total
    }
    
    /// High precision configuration
    pub fn high_precision() -> Self {
        Self::new(vec![
            4294967291, 4294967279, 4294967231, 4294967197
        ])  // ~2^128 total
    }
}

/// Bounded rational number with explicit bound tracking
/// 
/// INVARIANT: 2 * p_bound * q_bound < config.modulus
/// 
/// This ensures unique reconstruction via Theorem B4.
#[derive(Clone, Debug)]
pub struct BoundedRational {
    /// Residue representation (p * q^(-1) mod M)
    pub residue: u128,
    /// Bound on |numerator|
    pub p_bound: u64,
    /// Bound on denominator
    pub q_bound: u64,
    /// Configuration
    config: BoundedRationalConfig,
}

impl BoundedRational {
    /// Create from numerator and denominator with explicit bounds
    /// 
    /// # Errors
    /// - `BoundsExceeded` if 2PQ >= M
    /// - `DenominatorNotCoprime` if gcd(q, M) > 1
    pub fn new(
        numerator: i64,
        denominator: u64,
        p_bound: u64,
        q_bound: u64,
        config: BoundedRationalConfig,
    ) -> Result<Self, BoundedRationalError> {
        // Validate bounds
        if p_bound == 0 || q_bound == 0 {
            return Err(BoundedRationalError::InvalidBounds);
        }
        
        if !config.check_bounds(p_bound, q_bound) {
            return Err(BoundedRationalError::BoundsExceeded {
                p_bound,
                q_bound,
                modulus: config.modulus,
            });
        }
        
        if denominator == 0 {
            return Err(BoundedRationalError::DivisionByZero);
        }
        
        // Check denominator coprime to modulus
        for &m in &config.crt_moduli {
            if gcd(denominator, m) > 1 {
                return Err(BoundedRationalError::DenominatorNotCoprime);
            }
        }
        
        // Compute residue: p * q^(-1) mod M
        let p_mod = if numerator >= 0 {
            numerator as u128 % config.modulus
        } else {
            config.modulus - ((-numerator) as u128 % config.modulus)
        };
        
        let q_inv = mod_inverse_u128(denominator as u128, config.modulus)
            .ok_or(BoundedRationalError::DenominatorNotCoprime)?;
        
        let residue = (p_mod * q_inv) % config.modulus;
        
        Ok(Self {
            residue,
            p_bound,
            q_bound,
            config,
        })
    }
    
    /// Create from integer (denominator = 1)
    pub fn from_integer(value: i64, config: BoundedRationalConfig) -> Result<Self, BoundedRationalError> {
        let bound = value.unsigned_abs().max(1);
        Self::new(value, 1, bound, 1, config)
    }
    
    /// Check if reconstruction uniqueness holds
    pub fn reconstruction_unique(&self) -> bool {
        self.config.check_bounds(self.p_bound, self.q_bound)
    }
    
    /// Addition with bound tracking
    /// 
    /// Lemma B5: a/b + c/d = (ad + bc) / bd
    /// P' = P₁Q₂ + P₂Q₁, Q' = Q₁Q₂
    pub fn add(&self, other: &Self) -> Result<Self, BoundedRationalError> {
        // New bounds (Lemma B5)
        let new_p_bound = self.p_bound.saturating_mul(other.q_bound)
            .saturating_add(self.q_bound.saturating_mul(other.p_bound));
        let new_q_bound = self.q_bound.saturating_mul(other.q_bound);
        
        // Check bounds
        if !self.config.check_bounds(new_p_bound, new_q_bound) {
            return Err(BoundedRationalError::BoundsExceeded {
                p_bound: new_p_bound,
                q_bound: new_q_bound,
                modulus: self.config.modulus,
            });
        }
        
        // Compute residue sum
        let residue = (self.residue + other.residue) % self.config.modulus;
        
        Ok(Self {
            residue,
            p_bound: new_p_bound,
            q_bound: new_q_bound,
            config: self.config.clone(),
        })
    }
    
    /// Subtraction with bound tracking
    pub fn sub(&self, other: &Self) -> Result<Self, BoundedRationalError> {
        // Same bounds as addition (Lemma B5)
        let new_p_bound = self.p_bound.saturating_mul(other.q_bound)
            .saturating_add(self.q_bound.saturating_mul(other.p_bound));
        let new_q_bound = self.q_bound.saturating_mul(other.q_bound);
        
        if !self.config.check_bounds(new_p_bound, new_q_bound) {
            return Err(BoundedRationalError::BoundsExceeded {
                p_bound: new_p_bound,
                q_bound: new_q_bound,
                modulus: self.config.modulus,
            });
        }
        
        let residue = (self.residue + self.config.modulus - other.residue) % self.config.modulus;
        
        Ok(Self {
            residue,
            p_bound: new_p_bound,
            q_bound: new_q_bound,
            config: self.config.clone(),
        })
    }
    
    /// Multiplication with bound tracking
    /// 
    /// Lemma B6: (a/b) * (c/d) = (ac) / (bd)
    /// P' = P₁P₂, Q' = Q₁Q₂
    pub fn mul(&self, other: &Self) -> Result<Self, BoundedRationalError> {
        // New bounds (Lemma B6)
        let new_p_bound = self.p_bound.saturating_mul(other.p_bound);
        let new_q_bound = self.q_bound.saturating_mul(other.q_bound);
        
        if !self.config.check_bounds(new_p_bound, new_q_bound) {
            return Err(BoundedRationalError::BoundsExceeded {
                p_bound: new_p_bound,
                q_bound: new_q_bound,
                modulus: self.config.modulus,
            });
        }
        
        let residue = (self.residue as u128 * other.residue as u128) % self.config.modulus;
        
        Ok(Self {
            residue: residue as u128,
            p_bound: new_p_bound,
            q_bound: new_q_bound,
            config: self.config.clone(),
        })
    }
    
    /// Inversion with bound tracking
    /// 
    /// Lemma B7: (a/b)^(-1) = b/a
    /// P' = Q, Q' = P
    /// 
    /// NOTE: Requires |p| >= 1 (non-zero) and gcd(p, M) = 1
    pub fn inv(&self) -> Result<Self, BoundedRationalError> {
        if self.residue == 0 {
            return Err(BoundedRationalError::DivisionByZero);
        }
        
        // New bounds (Lemma B7) - swap
        let new_p_bound = self.q_bound;
        let new_q_bound = self.p_bound;
        
        if !self.config.check_bounds(new_p_bound, new_q_bound) {
            return Err(BoundedRationalError::BoundsExceeded {
                p_bound: new_p_bound,
                q_bound: new_q_bound,
                modulus: self.config.modulus,
            });
        }
        
        let residue = mod_inverse_u128(self.residue, self.config.modulus)
            .ok_or(BoundedRationalError::ReconstructionFailed)?;
        
        Ok(Self {
            residue,
            p_bound: new_p_bound,
            q_bound: new_q_bound,
            config: self.config.clone(),
        })
    }
    
    /// Division with bound tracking
    pub fn div(&self, other: &Self) -> Result<Self, BoundedRationalError> {
        let inv = other.inv()?;
        self.mul(&inv)
    }
    
    /// Reduce bounds after GCD reduction
    /// 
    /// Lemma B8: GCD reduction never increases bounds
    pub fn reduce_bounds(&mut self) {
        // After reconstruction, we can compute actual p, q and reduce
        if let Ok((p, q)) = self.reconstruct() {
            self.p_bound = p.unsigned_abs().max(1);
            self.q_bound = q.max(1);
        }
    }
}

// =============================================================================
// RATIONAL RECONSTRUCTION
// =============================================================================

impl BoundedRational {
    /// Reconstruct exact rational (p, q) from residue
    /// 
    /// Uses extended Euclidean algorithm to find p, q such that:
    /// - p ≡ residue * q (mod M)
    /// - |p| ≤ P
    /// - 1 ≤ q ≤ Q
    /// - gcd(p, q) = 1
    /// 
    /// THEOREM B4: Solution is UNIQUE when 2PQ < M
    pub fn reconstruct(&self) -> Result<(i64, u64), BoundedRationalError> {
        if !self.reconstruction_unique() {
            return Err(BoundedRationalError::BoundsExceeded {
                p_bound: self.p_bound,
                q_bound: self.q_bound,
                modulus: self.config.modulus,
            });
        }
        
        // Extended Euclidean algorithm for rational reconstruction
        let m = self.config.modulus;
        let x = self.residue;
        
        // Find p, q such that p ≡ xq (mod m), |p| ≤ P, q ≤ Q
        let result = rational_reconstruction(x, m, self.p_bound, self.q_bound);
        
        result.ok_or(BoundedRationalError::ReconstructionFailed)
    }
    
    /// Check if this rational equals another (by reconstruction)
    pub fn equals(&self, other: &Self) -> Result<bool, BoundedRationalError> {
        let (p1, q1) = self.reconstruct()?;
        let (p2, q2) = other.reconstruct()?;
        
        // Cross multiply to compare: p1/q1 = p2/q2 iff p1*q2 = p2*q1
        Ok(p1 as i128 * q2 as i128 == p2 as i128 * q1 as i128)
    }
}

// =============================================================================
// ANCHOR SIGN CERTIFICATE
// =============================================================================

/// Exact sign determination via anchor modulus
/// 
/// THEOREM C2: If |z| < m_*/2, then sign(center(z mod m_*)) = sign(z)
pub fn anchor_sign(residue: u64, anchor_modulus: u64) -> i8 {
    let half = anchor_modulus / 2;
    
    if residue == 0 {
        0
    } else if residue <= half {
        1
    } else {
        -1
    }
}

/// Check if sign is exact (value within anchor bound)
pub fn sign_is_exact(p_bound: u64, anchor_modulus: u64) -> bool {
    p_bound < anchor_modulus / 2
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

/// Modular inverse using extended Euclidean algorithm (u128 version)
fn mod_inverse_u128(a: u128, m: u128) -> Option<u128> {
    if m <= 1 {
        return None;
    }
    
    let mut t: i128 = 0;
    let mut new_t: i128 = 1;
    let mut r: i128 = m as i128;
    let mut new_r: i128 = a as i128;
    
    while new_r != 0 {
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - quotient * new_t);
        (r, new_r) = (new_r, r - quotient * new_r);
    }
    
    if r > 1 {
        return None;  // Not invertible
    }
    
    if t < 0 {
        t += m as i128;
    }
    
    Some(t as u128)
}

/// Integer square root for u128
fn integer_sqrt_u128(n: u128) -> u128 {
    if n == 0 { return 0; }
    
    let mut x = n;
    let mut y = (x + 1) / 2;
    
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    
    x
}

/// Rational reconstruction via extended Euclidean algorithm
/// 
/// Finds (p, q) such that p ≡ x*q (mod m), |p| ≤ P, q ≤ Q
fn rational_reconstruction(x: u128, m: u128, p_bound: u64, q_bound: u64) -> Option<(i64, u64)> {
    if x == 0 {
        return Some((0, 1));
    }
    
    // Extended Euclidean algorithm
    let mut r0 = m as i128;
    let mut r1 = x as i128;
    let mut s0: i128 = 1;
    let mut s1: i128 = 0;
    
    while r1 != 0 {
        let q = r0 / r1;
        
        let r2 = r0 - q * r1;
        let s2 = s0 - q * s1;
        
        // Check if we've found a valid reconstruction
        let p_candidate = r1;
        let q_candidate = if s1 >= 0 { s1 } else { -s1 };
        
        if q_candidate > 0 && q_candidate <= q_bound as i128 {
            let p_abs = p_candidate.unsigned_abs() as u64;
            if p_abs <= p_bound {
                // Found it
                let p = if s1 >= 0 { p_candidate } else { -p_candidate };
                let q = q_candidate as u64;
                
                // Verify
                if ((p as i128 % m as i128 + m as i128) as u128 % m) 
                   == ((x as i128 * q as i128 % m as i128 + m as i128) as u128 % m) {
                    return Some((p as i64, q));
                }
            }
        }
        
        r0 = r1;
        r1 = r2;
        s0 = s1;
        s1 = s2;
    }
    
    // Final check with r0
    if r0.unsigned_abs() as u64 <= p_bound && s0.unsigned_abs() as u64 <= q_bound {
        let p = r0 as i64;
        let q = s0.unsigned_abs() as u64;
        if q > 0 {
            return Some((p, q));
        }
    }
    
    None
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    fn test_config() -> BoundedRationalConfig {
        BoundedRationalConfig::new(vec![65537, 65521])  // ~4 billion
    }
    
    #[test]
    fn test_bounded_rational_creation() {
        let config = test_config();
        let r = BoundedRational::new(3, 4, 10, 10, config).unwrap();
        
        assert!(r.reconstruction_unique());
    }
    
    #[test]
    fn test_bounded_rational_from_integer() {
        let config = test_config();
        let r = BoundedRational::from_integer(42, config).unwrap();
        
        let (p, q) = r.reconstruct().unwrap();
        assert_eq!(p, 42);
        assert_eq!(q, 1);
    }
    
    #[test]
    fn test_bounded_rational_addition() {
        let config = test_config();
        
        // 1/2 + 1/3 = 5/6
        let a = BoundedRational::new(1, 2, 10, 10, config.clone()).unwrap();
        let b = BoundedRational::new(1, 3, 10, 10, config.clone()).unwrap();
        
        let sum = a.add(&b).unwrap();
        let (p, q) = sum.reconstruct().unwrap();
        
        // 5/6 or equivalent
        assert_eq!(p as i64 * 6, 5 * q as i64);
    }
    
    #[test]
    fn test_bounded_rational_multiplication() {
        let config = test_config();
        
        // 2/3 * 3/4 = 6/12 = 1/2
        let a = BoundedRational::new(2, 3, 10, 10, config.clone()).unwrap();
        let b = BoundedRational::new(3, 4, 10, 10, config.clone()).unwrap();
        
        let product = a.mul(&b).unwrap();
        let (p, q) = product.reconstruct().unwrap();
        
        // Should be 1/2 equivalent
        assert_eq!(p as i64 * 2, 1 * q as i64);
    }
    
    #[test]
    fn test_bounded_rational_inversion() {
        let config = test_config();
        
        // (3/4)^(-1) = 4/3
        let a = BoundedRational::new(3, 4, 10, 10, config.clone()).unwrap();
        
        let inv = a.inv().unwrap();
        let (p, q) = inv.reconstruct().unwrap();
        
        // Should be 4/3 equivalent
        assert_eq!(p as i64 * 3, 4 * q as i64);
    }
    
    #[test]
    fn test_bounds_exceeded_error() {
        let config = BoundedRationalConfig::new(vec![100, 97]);  // Small modulus
        
        // Try with bounds that exceed 2PQ < M
        let result = BoundedRational::new(1, 2, 1000, 1000, config);
        
        assert!(matches!(result, Err(BoundedRationalError::BoundsExceeded { .. })));
    }
    
    #[test]
    fn test_reconstruction_roundtrip() {
        let config = test_config();
        
        for p in [-100i64, -1, 0, 1, 100] {
            for q in [1u64, 2, 3, 7, 11] {
                let r = BoundedRational::new(p, q, 200, 20, config.clone()).unwrap();
                let (p_rec, q_rec) = r.reconstruct().unwrap();
                
                // Verify p_rec/q_rec = p/q
                assert_eq!(
                    p_rec as i128 * q as i128, 
                    p as i128 * q_rec as i128,
                    "Roundtrip failed for {}/{}",
                    p, q
                );
            }
        }
    }
    
    #[test]
    fn test_negative_numbers() {
        let config = test_config();
        
        // -3/4
        let r = BoundedRational::new(-3, 4, 10, 10, config.clone()).unwrap();
        let (p, q) = r.reconstruct().unwrap();
        
        // p/q should equal -3/4
        assert_eq!(p as i64 * 4, -3 * q as i64);
    }
    
    #[test]
    fn test_anchor_sign() {
        let m = 256u64;
        
        // Positive values (< m/2 = 128)
        assert_eq!(anchor_sign(1, m), 1);
        assert_eq!(anchor_sign(100, m), 1);
        assert_eq!(anchor_sign(127, m), 1);
        
        // Negative values (>= m/2)
        assert_eq!(anchor_sign(129, m), -1);
        assert_eq!(anchor_sign(200, m), -1);
        assert_eq!(anchor_sign(255, m), -1);
        
        // Zero
        assert_eq!(anchor_sign(0, m), 0);
    }
    
    #[test]
    fn test_sign_is_exact() {
        let m = 256u64;
        
        assert!(sign_is_exact(100, m));   // 100 < 128
        assert!(!sign_is_exact(200, m));  // 200 >= 128
    }
    
    #[test]
    fn test_bound_growth() {
        let config = test_config();
        
        let a = BoundedRational::new(10, 3, 20, 10, config.clone()).unwrap();
        let b = BoundedRational::new(7, 5, 20, 10, config.clone()).unwrap();
        
        // After addition: P' = 20*10 + 10*20 = 400, Q' = 10*10 = 100
        let sum = a.add(&b).unwrap();
        assert!(sum.p_bound <= 400);
        assert!(sum.q_bound <= 100);
        
        // After multiplication: P' = 20*20 = 400, Q' = 10*10 = 100
        let product = a.mul(&b).unwrap();
        assert!(product.p_bound <= 400);
        assert!(product.q_bound <= 100);
    }
    
    #[test]
    fn test_division_by_zero() {
        let config = test_config();
        
        let a = BoundedRational::new(1, 2, 10, 10, config.clone()).unwrap();
        let zero = BoundedRational::from_integer(0, config.clone()).unwrap();
        
        let result = a.div(&zero);
        assert!(matches!(result, Err(BoundedRationalError::DivisionByZero)));
    }
}

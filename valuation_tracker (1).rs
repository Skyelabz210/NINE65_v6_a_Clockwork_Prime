//! Valuation Tracker - Kill #67
//!
//! Multi-valuation oracle for O(small) divisibility checks in FHE rescaling.
//!
//! Mathematical basis: p-adic valuations ν_p(x) track prime power divisibility.
//! For x = p^a × m where gcd(p, m) = 1, we have ν_p(x) = a.
//!
//! Key properties:
//!   ν_p(x × y) = ν_p(x) + ν_p(y)
//!   ν_p(x + y) ≥ min(ν_p(x), ν_p(y))
//!   ν_p(x / y) = ν_p(x) - ν_p(y) (for exact division)
//!
//! This enables O(factoring d) divisibility checks, independent of the size of x.
//!
//! # Example
//!
//! ```ignore
//! use nine65::arithmetic::valuation::ValuationTracker;
//!
//! // x = 6048 = 2^5 × 3^3 × 7
//! let tracker = ValuationTracker::from_integer(6048);
//!
//! // Can we divide by 24 = 2^3 × 3?
//! assert!(tracker.can_divide(24));  // Yes: ν_2 ≥ 3, ν_3 ≥ 1
//!
//! // Can we divide by 49 = 7^2?
//! assert!(!tracker.can_divide(49)); // No: ν_7 = 1 < 2
//! ```

use std::collections::HashMap;

/// Small primes to track by default.
/// These cover most common FHE rescaling divisors.
const TRACKED_PRIMES: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

/// Maximum prime to consider in factorization.
const MAX_FACTOR_PRIME: u64 = 1000;

/// Multi-valuation oracle for tracking p-adic valuations.
///
/// Enables O(small) divisibility checks during FHE rescaling operations.
#[derive(Debug, Clone, Default)]
pub struct ValuationTracker {
    /// Maps prime p → valuation ν_p(x)
    valuations: HashMap<u64, u32>,
    /// The "unfactored" part (product of large prime factors)
    /// If None, we know the complete factorization
    unfactored: Option<u64>,
}

impl ValuationTracker {
    /// Create an empty tracker (represents 1).
    pub fn one() -> Self {
        Self {
            valuations: HashMap::new(),
            unfactored: None,
        }
    }

    /// Create a tracker from known prime factorization.
    ///
    /// # Arguments
    /// * `factors` - Slice of (prime, exponent) pairs
    ///
    /// # Example
    /// ```ignore
    /// // 72 = 2^3 × 3^2
    /// let tracker = ValuationTracker::from_factorization(&[(2, 3), (3, 2)]);
    /// ```
    pub fn from_factorization(factors: &[(u64, u32)]) -> Self {
        Self {
            valuations: factors.iter().cloned().collect(),
            unfactored: None,
        }
    }

    /// Create a tracker by factoring an integer.
    ///
    /// For small integers, this is fast. For large integers with
    /// large prime factors, we track what we can and store the rest.
    pub fn from_integer(mut n: u64) -> Self {
        if n == 0 {
            // Zero has infinite valuation at all primes
            // We represent this specially
            return Self {
                valuations: HashMap::new(),
                unfactored: Some(0),
            };
        }

        let mut valuations = HashMap::new();

        // Trial division by small primes
        for &p in TRACKED_PRIMES {
            if n == 1 {
                break;
            }
            let mut exp = 0u32;
            while n % p == 0 {
                n /= p;
                exp += 1;
            }
            if exp > 0 {
                valuations.insert(p, exp);
            }
        }

        // Continue trial division up to MAX_FACTOR_PRIME
        let mut p = TRACKED_PRIMES.last().copied().unwrap_or(2) + 2;
        while p <= MAX_FACTOR_PRIME && p * p <= n {
            let mut exp = 0u32;
            while n % p == 0 {
                n /= p;
                exp += 1;
            }
            if exp > 0 {
                valuations.insert(p, exp);
            }
            p += 2;
        }

        Self {
            valuations,
            unfactored: if n > 1 { Some(n) } else { None },
        }
    }

    /// Create from product of moduli (common in RNS).
    ///
    /// # Arguments
    /// * `moduli` - The RNS moduli whose product we're tracking
    pub fn from_moduli_product(moduli: &[u64]) -> Self {
        let mut result = Self::one();
        for &m in moduli {
            result = result.mul(&Self::from_integer(m));
        }
        result
    }

    /// Get the valuation of prime p.
    ///
    /// Returns 0 if p doesn't divide the tracked number.
    pub fn valuation(&self, p: u64) -> u32 {
        self.valuations.get(&p).copied().unwrap_or(0)
    }

    /// Check if the tracked number is divisible by d.
    ///
    /// This is the key operation: O(factoring d), independent of x.
    ///
    /// # Arguments
    /// * `d` - The divisor to check
    ///
    /// # Returns
    /// `true` if x is divisible by d, `false` otherwise.
    /// Returns `None` if we can't determine (unfactored part may contain required factors).
    pub fn can_divide(&self, d: u64) -> Option<bool> {
        if d == 0 {
            return Some(false); // Division by zero
        }
        if d == 1 {
            return Some(true);
        }

        // Factor d
        let d_factors = Self::from_integer(d);

        // Check each prime factor of d
        for (&p, &required_exp) in &d_factors.valuations {
            let have_exp = self.valuation(p);
            if have_exp < required_exp {
                // Definitely not divisible
                return Some(false);
            }
        }

        // If d has an unfactored part, we need to check if our unfactored part
        // contains it
        if let Some(d_unfactored) = d_factors.unfactored {
            if d_unfactored > 1 {
                match self.unfactored {
                    Some(our_unfactored) if our_unfactored % d_unfactored == 0 => {
                        // Our unfactored part is divisible by d's unfactored part
                        return Some(true);
                    }
                    Some(_) => {
                        // Can't determine without full factorization
                        return None;
                    }
                    None => {
                        // We're fully factored, d isn't, so we can't have d's large factor
                        return Some(false);
                    }
                }
            }
        }

        Some(true)
    }

    /// Definite divisibility check (panics if uncertain).
    pub fn can_divide_definite(&self, d: u64) -> bool {
        self.can_divide(d)
            .expect("Cannot determine divisibility with unfactored components")
    }

    /// Multiply two tracked numbers (add valuations).
    pub fn mul(&self, other: &Self) -> Self {
        let mut valuations = self.valuations.clone();

        for (&p, &exp) in &other.valuations {
            *valuations.entry(p).or_insert(0) += exp;
        }

        let unfactored = match (self.unfactored, other.unfactored) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => Some(a.saturating_mul(b)), // May overflow, but that's okay
        };

        Self {
            valuations,
            unfactored,
        }
    }

    /// Divide by d (subtract valuations).
    ///
    /// # Panics
    /// Panics if the division is not exact.
    pub fn div_exact(&self, d: u64) -> Self {
        assert!(
            self.can_divide(d).unwrap_or(true),
            "Division is not exact"
        );

        let d_factors = Self::from_integer(d);
        let mut valuations = self.valuations.clone();

        for (&p, &exp) in &d_factors.valuations {
            let entry = valuations.get_mut(&p).expect("Division not exact");
            *entry = entry.checked_sub(exp).expect("Division not exact");
            if *entry == 0 {
                valuations.remove(&p);
            }
        }

        // Handle unfactored parts
        let unfactored = match (self.unfactored, d_factors.unfactored) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(_)) => panic!("Division not exact: d has large prime factor"),
            (Some(a), Some(b)) => {
                if a % b == 0 {
                    let result = a / b;
                    if result > 1 {
                        Some(result)
                    } else {
                        None
                    }
                } else {
                    panic!("Division not exact: unfactored parts don't divide")
                }
            }
        };

        Self {
            valuations,
            unfactored,
        }
    }

    /// Add (gives lower bound on valuations).
    ///
    /// After addition, ν_p(x + y) ≥ min(ν_p(x), ν_p(y)).
    /// We track the minimum as a conservative estimate.
    pub fn add_conservative(&self, other: &Self) -> Self {
        let mut valuations = HashMap::new();

        // Only primes present in BOTH have guaranteed minimum valuation
        for (&p, &exp1) in &self.valuations {
            if let Some(&exp2) = other.valuations.get(&p) {
                valuations.insert(p, exp1.min(exp2));
            }
        }

        // If either has unfactored part, result is uncertain
        let unfactored = match (self.unfactored, other.unfactored) {
            (None, None) => None,
            _ => Some(1), // Placeholder indicating uncertainty
        };

        Self {
            valuations,
            unfactored,
        }
    }

    /// Check if this represents zero.
    pub fn is_zero(&self) -> bool {
        self.unfactored == Some(0)
    }

    /// Check if this represents one.
    pub fn is_one(&self) -> bool {
        self.valuations.is_empty() && self.unfactored.is_none()
    }

    /// Get all tracked prime-exponent pairs.
    pub fn factors(&self) -> impl Iterator<Item = (u64, u32)> + '_ {
        self.valuations.iter().map(|(&p, &e)| (p, e))
    }

    /// Reconstruct the tracked number (if small enough).
    pub fn to_integer(&self) -> Option<u64> {
        let mut result = 1u64;

        for (&p, &exp) in &self.valuations {
            result = result.checked_mul(p.checked_pow(exp)?)?;
        }

        if let Some(unfactored) = self.unfactored {
            result = result.checked_mul(unfactored)?;
        }

        Some(result)
    }
}

/// Integration with FHE rescaling operations.
pub mod fhe_integration {
    use super::*;

    /// Tracked ciphertext with valuation information.
    pub struct TrackedCiphertext<C> {
        /// The actual ciphertext
        pub ciphertext: C,
        /// Valuation tracker for the modulus product
        pub modulus_tracker: ValuationTracker,
        /// Current level (number of rescales performed)
        pub level: usize,
    }

    impl<C> TrackedCiphertext<C> {
        /// Create from ciphertext and moduli.
        pub fn new(ciphertext: C, moduli: &[u64]) -> Self {
            Self {
                ciphertext,
                modulus_tracker: ValuationTracker::from_moduli_product(moduli),
                level: 0,
            }
        }

        /// Check if rescaling by d is valid (O(factoring d)).
        pub fn can_rescale(&self, d: u64) -> bool {
            self.modulus_tracker.can_divide(d).unwrap_or(false)
        }

        /// Update tracker after rescaling.
        pub fn after_rescale(&mut self, d: u64) {
            self.modulus_tracker = self.modulus_tracker.div_exact(d);
            self.level += 1;
        }
    }

    /// Batch check multiple rescaling factors.
    pub fn find_valid_rescale_factor(
        tracker: &ValuationTracker,
        candidates: &[u64],
    ) -> Option<u64> {
        candidates
            .iter()
            .find(|&&d| tracker.can_divide(d).unwrap_or(false))
            .copied()
    }

    /// Suggest optimal rescale factor based on available valuations.
    pub fn suggest_rescale_factor(tracker: &ValuationTracker, target_bits: u32) -> Option<u64> {
        // Build factor from available prime powers
        let mut factor = 1u64;
        let mut bits = 0u32;

        for (&p, &max_exp) in tracker.valuations.iter() {
            for exp in (1..=max_exp).rev() {
                let p_pow = p.pow(exp);
                let p_bits = 64 - p_pow.leading_zeros();

                if bits + p_bits <= target_bits {
                    factor *= p_pow;
                    bits += p_bits;
                    break;
                }
            }

            if bits >= target_bits {
                break;
            }
        }

        if factor > 1 {
            Some(factor)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_integer() {
        // 72 = 2^3 × 3^2
        let tracker = ValuationTracker::from_integer(72);
        assert_eq!(tracker.valuation(2), 3);
        assert_eq!(tracker.valuation(3), 2);
        assert_eq!(tracker.valuation(5), 0);
        assert_eq!(tracker.to_integer(), Some(72));
    }

    #[test]
    fn test_can_divide() {
        // 6048 = 2^5 × 3^3 × 7
        let tracker = ValuationTracker::from_integer(6048);

        // 24 = 2^3 × 3 → divisible
        assert_eq!(tracker.can_divide(24), Some(true));

        // 49 = 7^2 → not divisible (only have 7^1)
        assert_eq!(tracker.can_divide(49), Some(false));

        // 8 = 2^3 → divisible
        assert_eq!(tracker.can_divide(8), Some(true));

        // 32 = 2^5 → divisible
        assert_eq!(tracker.can_divide(32), Some(true));

        // 64 = 2^6 → not divisible
        assert_eq!(tracker.can_divide(64), Some(false));
    }

    #[test]
    fn test_mul() {
        let a = ValuationTracker::from_integer(12); // 2^2 × 3
        let b = ValuationTracker::from_integer(18); // 2 × 3^2
        let c = a.mul(&b);

        // Result: 216 = 2^3 × 3^3
        assert_eq!(c.valuation(2), 3);
        assert_eq!(c.valuation(3), 3);
        assert_eq!(c.to_integer(), Some(216));
    }

    #[test]
    fn test_div_exact() {
        let tracker = ValuationTracker::from_integer(6048);
        let divided = tracker.div_exact(24);

        // 6048 / 24 = 252 = 2^2 × 3^2 × 7
        assert_eq!(divided.valuation(2), 2);
        assert_eq!(divided.valuation(3), 2);
        assert_eq!(divided.valuation(7), 1);
        assert_eq!(divided.to_integer(), Some(252));
    }

    #[test]
    fn test_from_moduli_product() {
        let moduli = vec![17, 19, 23];
        let tracker = ValuationTracker::from_moduli_product(&moduli);

        // Product = 17 × 19 × 23 = 7429
        assert_eq!(tracker.valuation(17), 1);
        assert_eq!(tracker.valuation(19), 1);
        assert_eq!(tracker.valuation(23), 1);
        assert_eq!(tracker.can_divide(17), Some(true));
        assert_eq!(tracker.can_divide(289), Some(false)); // 17^2
    }

    #[test]
    fn test_fhe_rescale_check() {
        use fhe_integration::*;

        // Typical FHE moduli
        let moduli = vec![
            (1u64 << 54) + 1, // ~54 bits
            (1u64 << 54) + 33,
            (1u64 << 54) + 65,
        ];

        let tracker = ValuationTracker::from_moduli_product(&moduli);

        // These moduli are likely prime, so no small factors
        assert_eq!(tracker.can_divide(2), Some(false));

        // But we can always divide by 1
        assert_eq!(tracker.can_divide(1), Some(true));
    }

    #[test]
    fn test_suggest_rescale() {
        use fhe_integration::*;

        // Product with known factorization: 2^10 × 3^5 × 5^3
        let tracker = ValuationTracker::from_factorization(&[(2, 10), (3, 5), (5, 3)]);

        // Suggest ~20-bit factor
        let factor = suggest_rescale_factor(&tracker, 20);
        assert!(factor.is_some());
        let f = factor.unwrap();
        assert!(f > 1);
        assert!(tracker.can_divide(f) == Some(true));
    }

    #[test]
    fn test_one_and_zero() {
        let one = ValuationTracker::one();
        assert!(one.is_one());
        assert!(!one.is_zero());

        let zero = ValuationTracker::from_integer(0);
        assert!(zero.is_zero());
        assert!(!zero.is_one());
    }

    #[test]
    fn test_add_conservative() {
        // 12 = 2^2 × 3
        let a = ValuationTracker::from_integer(12);
        // 18 = 2 × 3^2
        let b = ValuationTracker::from_integer(18);

        let sum = a.add_conservative(&b);

        // After addition, we only know:
        // ν_2(a+b) ≥ min(2, 1) = 1
        // ν_3(a+b) ≥ min(1, 2) = 1
        assert_eq!(sum.valuation(2), 1);
        assert_eq!(sum.valuation(3), 1);
    }
}

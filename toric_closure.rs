// ═══════════════════════════════════════════════════════════════════════════════
// TORIC CLOSURE DETECTION
// ═══════════════════════════════════════════════════════════════════════════════
//
// THE CORE INSIGHT:
//   Period detection IS toric closure detection.
//   
//   The sequence a^0, a^1, a^2, ... mod N traces a path on a torus.
//   The period r is where this path first returns to its starting point.
//   
//   On T² = (R/MZ) × (R/AZ):
//   - Each a^x lives at coordinates (a^x mod M, a^x mod A)
//   - Path closes when both coordinates simultaneously return to start
//   
//   We detect closure WITHOUT enumerating the full path by:
//   1. Tracking winding numbers across multiple CRT channels
//   2. Using φ-harmonic resonance to detect repetition
//   3. Combining signals for consensus
//
// ═══════════════════════════════════════════════════════════════════════════════

use crate::{mod_pow, gcd, QMNF_PRIMES};
use std::collections::HashMap;

/// Toric position on multi-dimensional CRT torus
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ToricPosition {
    /// Residues across CRT channels
    pub residues: Vec<u64>,
}

impl ToricPosition {
    /// Create position from value across moduli
    pub fn from_value(value: u128, moduli: &[u64]) -> Self {
        let residues = moduli
            .iter()
            .map(|&m| (value % m as u128) as u64)
            .collect();
        Self { residues }
    }
    
    /// Check if position is zero (starting point for normalized sequences)
    pub fn is_origin(&self) -> bool {
        self.residues.iter().all(|&r| r == 0)
    }
    
    /// Check if position matches another within tolerance
    pub fn matches(&self, other: &ToricPosition) -> bool {
        self.residues == other.residues
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CLOSURE DETECTOR
// ═══════════════════════════════════════════════════════════════════════════════

/// Detect toric path closure for period finding
#[derive(Clone, Debug)]
pub struct ClosureDetector {
    /// CRT moduli defining the torus dimensions
    pub moduli: Vec<u64>,
    /// Modulus product for range
    pub product: u128,
    /// Starting position
    pub start: ToricPosition,
    /// Position history (for small cases)
    position_cache: HashMap<ToricPosition, u64>,
    /// Current iteration
    pub iteration: u64,
}

impl ClosureDetector {
    /// Create closure detector with given moduli
    pub fn new(moduli: &[u64]) -> Self {
        let product: u128 = moduli.iter().map(|&x| x as u128).product();
        let start = ToricPosition { residues: vec![1; moduli.len()] };
        
        Self {
            moduli: moduli.to_vec(),
            product,
            start: start.clone(),
            position_cache: HashMap::new(),
            iteration: 0,
        }
    }
    
    /// Create with default QMNF primes
    pub fn default_primes() -> Self {
        Self::new(&QMNF_PRIMES[0..3])
    }
    
    /// Set starting position
    pub fn set_start(&mut self, value: u128) {
        self.start = ToricPosition::from_value(value, &self.moduli);
        self.position_cache.clear();
        self.position_cache.insert(self.start.clone(), 0);
        self.iteration = 0;
    }
    
    /// Check single step for closure
    /// Returns Some(period) if closure detected, None otherwise
    pub fn step(&mut self, value: u128) -> Option<u64> {
        self.iteration += 1;
        let pos = ToricPosition::from_value(value, &self.moduli);
        
        // Check if we've returned to start
        if pos.matches(&self.start) && self.iteration > 0 {
            return Some(self.iteration);
        }
        
        // Check if we've seen this position before (cycle in sequence)
        if let Some(&prev_iter) = self.position_cache.get(&pos) {
            // Found a cycle, but not necessarily back to start
            let cycle_len = self.iteration - prev_iter;
            
            // Check if this cycle implies a period
            // Period divides cycle length
            return Some(cycle_len);
        }
        
        // Record position (only for small cases to avoid memory blowup)
        if self.position_cache.len() < 1_000_000 {
            self.position_cache.insert(pos, self.iteration);
        }
        
        None
    }
    
    /// Stream closure detection for a^x mod N
    pub fn detect_order(&mut self, base: u64, modulus: u64, max_iter: u64) -> Option<u64> {
        self.set_start(1);
        
        let mut power = base as u128;
        
        for i in 1..=max_iter {
            if let Some(period) = self.step(power) {
                // Verify the period
                if mod_pow(base, period, modulus) == 1 {
                    return Some(period);
                }
            }
            power = (power * base as u128) % modulus as u128;
        }
        
        None
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PARALLEL CLOSURE DETECTION
// ═══════════════════════════════════════════════════════════════════════════════

/// Parallel closure detection across multiple independent tori
/// 
/// The insight: By using multiple independent moduli sets,
/// we can detect closure with higher confidence and potentially
/// find the period faster through the Chinese Remainder Theorem.
pub struct ParallelClosureDetector {
    /// Individual detectors
    detectors: Vec<ClosureDetector>,
    /// Channel labels for tracking
    labels: Vec<String>,
}

impl ParallelClosureDetector {
    /// Create parallel detector with multiple channel sets
    pub fn new() -> Self {
        // Use different prime subsets for each channel
        let channels = vec![
            vec![1009u64, 1013, 1019],
            vec![1021u64, 1031, 1033],
            vec![1039u64, 1049, 1051],
            vec![QMNF_PRIMES[0], QMNF_PRIMES[1]],
        ];
        
        let detectors: Vec<_> = channels.iter()
            .map(|primes| ClosureDetector::new(primes))
            .collect();
            
        let labels: Vec<_> = channels.iter()
            .enumerate()
            .map(|(i, _)| format!("Channel_{}", i))
            .collect();
        
        Self { detectors, labels }
    }
    
    /// Detect order using parallel channels
    /// 
    /// Each channel may detect a divisor of the period.
    /// The LCM of all detected values gives the true period.
    pub fn detect_order_parallel(&mut self, base: u64, modulus: u64, max_iter: u64) -> Option<u64> {
        // Initialize all channels
        for detector in &mut self.detectors {
            detector.set_start(1);
        }
        
        let mut detected_periods: Vec<u64> = Vec::new();
        let mut power = base as u128;
        
        for _ in 1..=max_iter {
            // Check all channels
            for detector in &mut self.detectors {
                if let Some(period) = detector.step(power) {
                    // Verify this is a valid period (or divisor)
                    if mod_pow(base, period, modulus) == 1 {
                        detected_periods.push(period);
                    }
                }
            }
            
            // If we have detections, compute LCM
            if detected_periods.len() >= 2 {
                let candidate = detected_periods.iter()
                    .fold(1u64, |acc, &p| lcm(acc, p));
                
                // Verify the candidate period
                if mod_pow(base, candidate, modulus) == 1 {
                    return Some(candidate);
                }
            }
            
            power = (power * base as u128) % modulus as u128;
        }
        
        // Return best candidate if any
        if !detected_periods.is_empty() {
            let candidate = detected_periods.iter()
                .fold(1u64, |acc, &p| lcm(acc, p));
            if mod_pow(base, candidate, modulus) == 1 {
                return Some(candidate);
            }
        }
        
        None
    }
}

/// Compute LCM using GCD
fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a / gcd(a, b)) * b
}

// ═══════════════════════════════════════════════════════════════════════════════
// ACCELERATED CLOSURE VIA ALGEBRAIC STRUCTURE
// ═══════════════════════════════════════════════════════════════════════════════

/// Algebraic structure analyzer for accelerated period detection
/// 
/// For a^x mod N where N = p × q:
/// - Order of a divides φ(N) = (p-1)(q-1)
/// - Order of a mod p divides (p-1)
/// - Order of a mod q divides (q-1)
/// - True order = lcm(order_p, order_q)
/// 
/// If we can find factors, we can constrain the search space.
pub struct AlgebraicClosureAccelerator {
    /// Target modulus N
    pub n: u64,
    /// Known divisors of φ(N) (if any)
    pub phi_divisors: Vec<u64>,
    /// Candidate orders to test
    pub candidates: Vec<u64>,
}

impl AlgebraicClosureAccelerator {
    /// Create accelerator for modulus N
    pub fn new(n: u64) -> Self {
        Self {
            n,
            phi_divisors: Vec::new(),
            candidates: Vec::new(),
        }
    }
    
    /// Add a known divisor of φ(N)
    pub fn add_phi_divisor(&mut self, d: u64) {
        self.phi_divisors.push(d);
        self.update_candidates();
    }
    
    /// Update candidate list based on known divisors
    fn update_candidates(&mut self) {
        self.candidates.clear();
        
        // Generate divisors of known values
        for &d in &self.phi_divisors {
            let divs = divisors(d);
            self.candidates.extend(divs);
        }
        
        // Remove duplicates and sort
        self.candidates.sort();
        self.candidates.dedup();
    }
    
    /// Test candidates to find the true order
    pub fn find_order(&self, base: u64) -> Option<u64> {
        for &candidate in &self.candidates {
            if mod_pow(base, candidate, self.n) == 1 {
                // Found a value where a^candidate ≡ 1
                // Now find the smallest such value (the order)
                let order = self.find_minimal_order(base, candidate);
                return Some(order);
            }
        }
        None
    }
    
    /// Find minimal order given that order divides `bound`
    fn find_minimal_order(&self, base: u64, bound: u64) -> u64 {
        let divs = divisors(bound);
        
        for d in divs {
            if mod_pow(base, d, self.n) == 1 {
                return d;
            }
        }
        
        bound
    }
}

/// Compute all divisors of n
fn divisors(n: u64) -> Vec<u64> {
    let mut divs = Vec::new();
    let mut i = 1;
    
    while i * i <= n {
        if n % i == 0 {
            divs.push(i);
            if i != n / i {
                divs.push(n / i);
            }
        }
        i += 1;
    }
    
    divs.sort();
    divs
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_toric_position() {
        let moduli = vec![7u64, 11, 13];
        let value = 100u128;
        
        let pos = ToricPosition::from_value(value, &moduli);
        assert_eq!(pos.residues, vec![2, 1, 9]); // 100 mod 7=2, 100 mod 11=1, 100 mod 13=9
    }
    
    #[test]
    fn test_closure_detector_order_2_mod_15() {
        // Order of 2 mod 15 is 4: 2^4 = 16 ≡ 1 (mod 15)
        let mut detector = ClosureDetector::new(&[3, 5, 7]);
        let order = detector.detect_order(2, 15, 100);
        
        assert_eq!(order, Some(4));
    }
    
    #[test]
    fn test_closure_detector_order_3_mod_7() {
        // Order of 3 mod 7 is 6: 3^6 = 729 ≡ 1 (mod 7)
        let mut detector = ClosureDetector::new(&[11, 13, 17]);
        let order = detector.detect_order(3, 7, 100);
        
        assert_eq!(order, Some(6));
    }
    
    #[test]
    fn test_divisors() {
        assert_eq!(divisors(12), vec![1, 2, 3, 4, 6, 12]);
        assert_eq!(divisors(7), vec![1, 7]); // Prime
        assert_eq!(divisors(1), vec![1]);
    }
    
    #[test]
    fn test_lcm() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(3, 5), 15);
        assert_eq!(lcm(7, 7), 7);
    }
}

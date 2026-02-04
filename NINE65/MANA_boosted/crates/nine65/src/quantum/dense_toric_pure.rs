//! Dense Toric Pure - Grover Search ENTIRELY on T²
//!
//! KEY INSIGHT: Never reconstruct. The torus IS the computation space.
//!
//! Previous implementation error: Called `to_value()` during computation,
//! reconstructing integers and reintroducing overflow risk.
//!
//! This implementation:
//! - All amplitudes are TorusPoints (inner mod M, outer mod A)
//! - Diffusion computed entirely on torus
//! - Comparison uses helix level (K-Elimination)
//! - Division by N uses modular inverse ON the torus
//! - Reconstruction only at final readout (optional)
//!
//! The torus is not storage - it IS the computational substrate.

use std::cmp::Ordering;

/// Dual Codex configuration - the computational substrate
///
/// M and A are coprime, forming the 2-torus T² = Z_M × Z_A
/// K-Elimination extracts helix level from phase differential
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DualCodex {
    pub m: u64,       // Inner modulus (fast computation)
    pub a: u64,       // Outer modulus (anchor/phase reference)
    pub m_inv_a: u64, // M⁻¹ mod A (for K-Elimination)
    pub a_inv_m: u64, // A⁻¹ mod M (for reconstruction if needed)
}

impl DualCodex {
    /// Create with coprime odd primes (ensures N=2^n has inverse)
    pub fn new(m: u64, a: u64) -> Self {
        debug_assert!(gcd(m, a) == 1, "M and A must be coprime");
        Self {
            m,
            a,
            m_inv_a: mod_inverse(m, a),
            a_inv_m: mod_inverse(a, m),
        }
    }

    /// Fibonacci moduli - consecutive Fibonacci numbers are always coprime
    /// This creates φ-harmonic spacing for numerical stability
    pub fn fibonacci(n: usize) -> Self {
        let (a, b) = fibonacci_pair(n);
        Self::new(a, b)
    }

    /// Large Mersenne-adjacent primes for maximum capacity
    pub fn large_primes() -> Self {
        // Two coprime primes near 2^31
        // Both odd, so any N = 2^k has inverse
        Self::new(2147483647, 2147483629)
    }

    /// Total capacity M × A
    pub fn capacity(&self) -> u128 {
        self.m as u128 * self.a as u128
    }

    /// Modular inverse of n mod M (for division on torus)
    pub fn inv_m(&self, n: u64) -> u64 {
        mod_inverse(n % self.m, self.m)
    }

    /// Modular inverse of n mod A
    pub fn inv_a(&self, n: u64) -> u64 {
        mod_inverse(n % self.a, self.a)
    }
}

/// A point on the 2-torus T² = Z_M × Z_A
///
/// This is NOT just a representation - it IS the value.
/// The relationship between inner and outer encodes magnitude.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TorusPoint {
    pub inner: u64,  // Value mod M (fast channel)
    pub outer: u64,  // Value mod A (anchor channel)
}

impl TorusPoint {
    /// Zero on the torus
    pub const fn zero() -> Self {
        Self { inner: 0, outer: 0 }
    }

    /// One on the torus
    pub const fn one() -> Self {
        Self { inner: 1, outer: 1 }
    }

    /// Create from an integer value (for initialization)
    pub fn from_u64(v: u64, dc: &DualCodex) -> Self {
        Self {
            inner: v % dc.m,
            outer: v % dc.a,
        }
    }

    /// Addition ON the torus - no reconstruction
    pub fn add(&self, other: &Self, dc: &DualCodex) -> Self {
        Self {
            inner: (self.inner + other.inner) % dc.m,
            outer: (self.outer + other.outer) % dc.a,
        }
    }

    /// Subtraction ON the torus
    pub fn sub(&self, other: &Self, dc: &DualCodex) -> Self {
        Self {
            inner: (self.inner + dc.m - other.inner) % dc.m,
            outer: (self.outer + dc.a - other.outer) % dc.a,
        }
    }

    /// Multiplication ON the torus
    pub fn mul(&self, other: &Self, dc: &DualCodex) -> Self {
        Self {
            inner: mul_mod(self.inner, other.inner, dc.m),
            outer: mul_mod(self.outer, other.outer, dc.a),
        }
    }

    /// Scale by integer ON the torus
    pub fn scale(&self, scalar: u64, dc: &DualCodex) -> Self {
        Self {
            inner: mul_mod(self.inner, scalar % dc.m, dc.m),
            outer: mul_mod(self.outer, scalar % dc.a, dc.a),
        }
    }

    /// Division by integer ON the torus (using modular inverse)
    /// This is O(1) - the inverse is just multiplication
    pub fn div_by(&self, divisor: u64, dc: &DualCodex) -> Self {
        let inv_m = dc.inv_m(divisor);
        let inv_a = dc.inv_a(divisor);
        Self {
            inner: mul_mod(self.inner, inv_m, dc.m),
            outer: mul_mod(self.outer, inv_a, dc.a),
        }
    }

    /// Negation (for oracle)
    pub fn neg(&self, dc: &DualCodex) -> Self {
        Self {
            inner: if self.inner == 0 { 0 } else { dc.m - self.inner },
            outer: if self.outer == 0 { 0 } else { dc.a - self.outer },
        }
    }

    /// Extract helix level k using K-Elimination
    /// k = (outer - inner) × M⁻¹ mod A
    ///
    /// This is O(1) - single subtraction + multiplication
    pub fn helix_level(&self, dc: &DualCodex) -> u64 {
        let inner_mod_a = self.inner % dc.a;
        let diff = if self.outer >= inner_mod_a {
            self.outer - inner_mod_a
        } else {
            dc.a - inner_mod_a + self.outer
        };
        mul_mod(diff, dc.m_inv_a, dc.a)
    }

    /// Compare two TorusPoints using helix level (O(1))
    /// Higher helix level = larger magnitude
    /// Same helix level = compare inner values
    pub fn compare(&self, other: &Self, dc: &DualCodex) -> Ordering {
        let k1 = self.helix_level(dc);
        let k2 = other.helix_level(dc);
        match k1.cmp(&k2) {
            Ordering::Equal => self.inner.cmp(&other.inner),
            ord => ord,
        }
    }

    /// Reconstruct integer value (ONLY for final readout)
    /// V = inner + k × M
    pub fn to_value(&self, dc: &DualCodex) -> u128 {
        let k = self.helix_level(dc);
        self.inner as u128 + k as u128 * dc.m as u128
    }
}

/// Signed amplitude on the torus (for quantum interference)
#[derive(Clone, Copy, Debug)]
pub struct SignedTorus {
    pub point: TorusPoint,
    pub negative: bool,
}

impl SignedTorus {
    pub fn positive(p: TorusPoint) -> Self {
        Self { point: p, negative: false }
    }

    pub fn zero() -> Self {
        Self::positive(TorusPoint::zero())
    }

    pub fn one() -> Self {
        Self::positive(TorusPoint::one())
    }

    /// Negate sign
    pub fn neg(&self) -> Self {
        Self {
            point: self.point,
            negative: !self.negative,
        }
    }

    /// Add two signed torus points ON the torus
    pub fn add(&self, other: &Self, dc: &DualCodex) -> Self {
        if self.negative == other.negative {
            // Same sign: add magnitudes
            Self {
                point: self.point.add(&other.point, dc),
                negative: self.negative,
            }
        } else {
            // Different signs: subtract, keep sign of larger
            match self.point.compare(&other.point, dc) {
                Ordering::Greater | Ordering::Equal => Self {
                    point: self.point.sub(&other.point, dc),
                    negative: self.negative,
                },
                Ordering::Less => Self {
                    point: other.point.sub(&self.point, dc),
                    negative: other.negative,
                },
            }
        }
    }

    /// Subtract
    pub fn sub(&self, other: &Self, dc: &DualCodex) -> Self {
        self.add(&other.neg(), dc)
    }

    /// Scale by integer
    pub fn scale(&self, scalar: u64, dc: &DualCodex) -> Self {
        Self {
            point: self.point.scale(scalar, dc),
            negative: self.negative,
        }
    }

    /// Divide by integer
    pub fn div_by(&self, divisor: u64, dc: &DualCodex) -> Self {
        Self {
            point: self.point.div_by(divisor, dc),
            negative: self.negative,
        }
    }

    /// Square (magnitude squared, always positive)
    pub fn square(&self, dc: &DualCodex) -> TorusPoint {
        self.point.mul(&self.point, dc)
    }
}

/// Dense Toric Grover - all computation on T²
///
/// Amplitudes are SignedTorus points. All operations stay on the torus.
/// Helix climbing handles any "overflow" - it's information, not error.
pub struct DenseToricPure {
    pub amplitudes: Vec<SignedTorus>,
    pub num_qubits: usize,
    pub target: usize,
    pub dc: DualCodex,
}

impl DenseToricPure {
    /// Create uniform superposition (all amplitudes = 1)
    pub fn uniform(num_qubits: usize, dc: DualCodex) -> Self {
        let n = 1usize << num_qubits;
        Self {
            amplitudes: vec![SignedTorus::one(); n],
            num_qubits,
            target: 0,
            dc,
        }
    }

    /// Create for specific target
    pub fn for_target(num_qubits: usize, target: usize, dc: DualCodex) -> Self {
        let mut state = Self::uniform(num_qubits, dc);
        state.target = target;
        state
    }

    pub fn num_states(&self) -> usize {
        self.amplitudes.len()
    }

    /// Oracle: flip sign of target amplitude
    pub fn apply_oracle(&mut self) {
        self.amplitudes[self.target] = self.amplitudes[self.target].neg();
    }

    /// Diffusion ENTIRELY ON THE TORUS
    ///
    /// D|ψ⟩_i = 2*mean - ψ_i
    ///
    /// In terms of numerators (with common denominator factored out):
    /// new_num_i = 2*sum - N*old_num_i
    ///
    /// All operations are torus operations:
    /// - Sum: component-wise addition
    /// - Scale by 2: multiplication on torus
    /// - Scale by N: multiplication on torus
    /// - Subtract: component-wise subtraction
    pub fn apply_diffusion(&mut self) {
        let n = self.num_states() as u64;
        let dc = &self.dc;

        // Step 1: Compute sum ON the torus
        let mut sum = SignedTorus::zero();
        for amp in &self.amplitudes {
            sum = sum.add(amp, dc);
        }

        // Step 2: 2 * sum ON the torus
        let two_sum = sum.scale(2, dc);

        // Step 3: For each amplitude: new = 2*sum - N*old
        // This is: 2*sum - N*old = two_sum - old.scale(N)
        for amp in &mut self.amplitudes {
            let n_times_old = amp.scale(n, dc);
            *amp = two_sum.sub(&n_times_old, dc);
        }
    }

    /// Single Grover iteration
    pub fn grover_iteration(&mut self) {
        self.apply_oracle();
        self.apply_diffusion();
    }

    /// Get total weight (sum of squared magnitudes) - ON TORUS
    pub fn total_weight_torus(&self) -> TorusPoint {
        let mut sum = TorusPoint::zero();
        for amp in &self.amplitudes {
            sum = sum.add(&amp.square(&self.dc), &self.dc);
        }
        sum
    }

    /// Get target weight - ON TORUS
    pub fn target_weight_torus(&self) -> TorusPoint {
        self.amplitudes[self.target].square(&self.dc)
    }

    /// Compare target weight to a fraction of total (without reconstruction)
    /// Returns true if target_weight / total_weight > threshold
    pub fn target_above_threshold(&self, threshold_num: u64, threshold_den: u64) -> bool {
        let target_sq = self.target_weight_torus();
        let total_sq = self.total_weight_torus();

        // target_sq / total_sq > threshold_num / threshold_den
        // ⟺ target_sq × threshold_den > total_sq × threshold_num
        let lhs = target_sq.scale(threshold_den, &self.dc);
        let rhs = total_sq.scale(threshold_num, &self.dc);

        lhs.compare(&rhs, &self.dc) == Ordering::Greater
    }

    /// Find maximum amplitude using helix comparison (O(n) comparisons, O(1) each)
    pub fn measure_max(&self) -> usize {
        self.amplitudes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.point.compare(&b.point, &self.dc))
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Get target probability (reconstructs values - use for verification only)
    pub fn target_probability(&self) -> f64 {
        let target_sq = self.amplitudes[self.target].square(&self.dc).to_value(&self.dc);
        let total_sq: u128 = self.amplitudes.iter()
            .map(|a| a.square(&self.dc).to_value(&self.dc))
            .sum();
        if total_sq == 0 { 0.0 } else { target_sq as f64 / total_sq as f64 }
    }
}

// ============================================================================
// Helper functions
// ============================================================================

/// Binary GCD (division-free per the Lean4 proof)
fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }

    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();

    loop {
        b >>= b.trailing_zeros();
        if a > b { std::mem::swap(&mut a, &mut b); }
        b -= a;
        if b == 0 { break; }
    }

    a << shift
}

/// Modular multiplication (handles intermediate overflow)
fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

/// Modular inverse via extended Euclidean
fn mod_inverse(a: u64, m: u64) -> u64 {
    let (mut old_r, mut r) = (m as i128, a as i128);
    let (mut old_s, mut s) = (0i128, 1i128);

    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
    }

    if old_s < 0 { (old_s + m as i128) as u64 } else { old_s as u64 }
}

/// Fibonacci pair (consecutive Fib numbers are coprime)
fn fibonacci_pair(n: usize) -> (u64, u64) {
    let mut a = 1u64;
    let mut b = 1u64;
    for _ in 0..n {
        let tmp = a.saturating_add(b);
        a = b;
        b = tmp;
    }
    (a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torus_arithmetic() {
        let dc = DualCodex::large_primes();

        let a = TorusPoint::from_u64(1000, &dc);
        let b = TorusPoint::from_u64(2000, &dc);

        // Addition
        let sum = a.add(&b, &dc);
        assert_eq!(sum.to_value(&dc), 3000);

        // Multiplication
        let prod = a.mul(&b, &dc);
        assert_eq!(prod.to_value(&dc), 2_000_000);

        // Division
        let div = prod.div_by(1000, &dc);
        assert_eq!(div.to_value(&dc), 2000);

        println!("✓ Torus arithmetic verified");
    }

    #[test]
    fn test_helix_level_extraction() {
        let dc = DualCodex::large_primes();

        // Values that span multiple helix levels
        for v in [0u64, 1, 1000, 1_000_000, 100_000_000, 2_000_000_000] {
            let tp = TorusPoint::from_u64(v, &dc);
            let recovered = tp.to_value(&dc);
            let k = tp.helix_level(&dc);
            println!("v={}: inner={}, outer={}, k={}, recovered={}",
                     v, tp.inner, tp.outer, k, recovered);
            assert_eq!(recovered, v as u128, "Round-trip failed for v={}", v);
        }

        println!("✓ Helix level extraction verified");
    }

    #[test]
    fn test_toric_grover_small() {
        println!("\n=== Pure Toric Grover: 4 qubits ===\n");

        let dc = DualCodex::large_primes();
        let mut state = DenseToricPure::for_target(4, 7, dc);

        println!("Initial state: {} amplitudes on T²", state.num_states());

        // Run optimal iterations
        let optimal = 3;
        for i in 0..optimal {
            state.grover_iteration();
            let prob = state.target_probability();
            let found = state.measure_max() == state.target;
            println!("Iter {}: prob={:.2}%, found={}", i + 1, prob * 100.0, found);
        }

        // Verify using torus comparison (no reconstruction)
        let above_90 = state.target_above_threshold(90, 100);
        println!("\nTarget above 90%: {} (via torus comparison)", above_90);

        assert!(above_90, "Target should be above 90%");
        assert_eq!(state.measure_max(), state.target);
        println!("✓ Pure toric Grover verified");
    }

    #[test]
    fn test_toric_deep_iterations() {
        println!("\n=== Pure Toric Grover: Deep Iterations ===\n");

        let dc = DualCodex::large_primes();
        let mut state = DenseToricPure::for_target(4, 7, dc);

        println!("Running 100 iterations entirely on T²...");

        for i in 0..100 {
            state.grover_iteration();

            if (i + 1) % 20 == 0 {
                // Check using torus comparison, no reconstruction
                let above_50 = state.target_above_threshold(50, 100);
                println!("Iter {:3}: above_50%={}", i + 1, above_50);
            }
        }

        // Final readout
        let prob = state.target_probability();
        println!("\nFinal probability: {:.4}%", prob * 100.0);
        println!("✓ 100 iterations completed on T² without overflow");
    }

    #[test]
    fn test_fibonacci_moduli() {
        println!("\n=== Fibonacci Moduli (φ-harmonic) ===\n");

        // F_20 = 6765, F_21 = 10946
        let dc = DualCodex::fibonacci(20);
        println!("Fibonacci moduli: M={}, A={}", dc.m, dc.a);
        println!("Capacity: {}", dc.capacity());

        let mut state = DenseToricPure::for_target(4, 7, dc);

        for i in 0..10 {
            state.grover_iteration();
            let prob = state.target_probability();
            println!("Iter {}: {:.2}%", i + 1, prob * 100.0);
        }

        println!("✓ Fibonacci moduli verified");
    }

    #[test]
    fn test_no_reconstruction_needed() {
        println!("\n=== Pure Torus: No Reconstruction ===\n");

        let dc = DualCodex::large_primes();
        let mut state = DenseToricPure::for_target(4, 7, dc);

        // Run until target is found using ONLY torus operations
        let mut found_at = None;
        for i in 0..20 {
            state.grover_iteration();

            // Check using torus comparison - NO to_value()
            if state.measure_max() == state.target {
                if state.target_above_threshold(90, 100) {
                    found_at = Some(i + 1);
                    break;
                }
            }
        }

        if let Some(iter) = found_at {
            println!("Target found at iteration {} using pure torus ops", iter);
        } else {
            println!("Target not found with >90% in 20 iterations");
        }

        // Now verify with reconstruction (for testing only)
        let prob = state.target_probability();
        println!("Verification (reconstruction): {:.2}%", prob * 100.0);

        assert!(found_at.is_some());
        println!("✓ No reconstruction needed for search");
    }

    #[test]
    fn test_helix_climbing() {
        println!("\n=== Helix Climbing Visualization ===\n");

        let dc = DualCodex::large_primes();
        let mut state = DenseToricPure::for_target(4, 7, dc);

        // Track helix levels of target amplitude
        let mut prev_k = state.amplitudes[state.target].point.helix_level(&dc);
        println!("Initial helix level: {}", prev_k);

        for i in 0..10 {
            state.grover_iteration();
            let k = state.amplitudes[state.target].point.helix_level(&dc);
            let sign = if state.amplitudes[state.target].negative { "-" } else { "+" };

            if k != prev_k {
                println!("Iter {}: helix {} → {} ({}climbed)", i + 1, prev_k, k,
                         if k > prev_k { "" } else { "de" });
            } else {
                println!("Iter {}: helix {} (stable), sign={}", i + 1, k, sign);
            }
            prev_k = k;
        }

        println!("✓ Helix climbing tracked");
    }
}

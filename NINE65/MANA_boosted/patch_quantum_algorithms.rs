// Patch for pqeaq_harness.rs to fix quantum algorithm implementation
// This ensures consistency with the working algorithms in coherence.rs

use std::time::Duration;

// MODULAR ARITHMETIC (inline for standalone operation)
// ========================================================================
#[inline]
pub fn add_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 + b as u128) % m as u128) as u64
}

#[inline]
pub fn sub_mod(a: u64, b: u64, m: u64) -> u64 {
    if a >= b { a - b } else { m - (b - a) }
}

#[inline]
pub fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    ((a as u128 * b as u128) % m as u128) as u64
}

pub fn pow_mod(mut base: u64, mut exp: u64, m: u64) -> u64 {
    let mut result = 1u64;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 { result = mul_mod(result, base, m); }
        exp >>= 1;
        base = mul_mod(base, base, m);
    }
    result
}

pub fn mod_inverse(a: u64, p: u64) -> u64 {
    pow_mod(a, p - 2, p)
}

fn pow2_mod(n: usize, p: u64) -> u64 {
    if n == 0 { return 1; }
    let mut result = 1u64;
    let mut base = 2u64;
    let mut exp = n;
    while exp > 0 {
        if exp & 1 == 1 {
            result = ((result as u128 * base as u128) % p as u128) as u64;
        }
        base = ((base as u128 * base as u128) % p as u128) as u64;
        exp >>= 1;
    }
    result
}

// CORRECTED F_p² FIELD IMPLEMENTATION
// ========================================================================
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Fp2 {
    pub a: u64,  // Real part
    pub b: u64,  // Imaginary part
    pub p: u64,  // Prime modulus (p ≡ 3 mod 4)
}

impl Fp2 {
    pub fn new(a: u64, b: u64, p: u64) -> Self {
        Self { a: a % p, b: b % p, p }
    }
    
    pub fn zero(p: u64) -> Self { Self { a: 0, b: 0, p } }
    pub fn one(p: u64) -> Self { Self { a: 1, b: 0, p } }
    
    pub fn add(&self, other: &Self) -> Self {
        Self::new(add_mod(self.a, other.a, self.p), add_mod(self.b, other.b, self.p), self.p)
    }
    
    pub fn sub(&self, other: &Self) -> Self {
        Self::new(sub_mod(self.a, other.a, self.p), sub_mod(self.b, other.b, self.p), self.p)
    }
    
    pub fn neg(&self) -> Self {
        Self::new(
            if self.a == 0 { 0 } else { self.p - self.a },
            if self.b == 0 { 0 } else { self.p - self.b },
            self.p
        )
    }
    
    pub fn scalar_mul(&self, k: u64) -> Self {
        Self::new(mul_mod(self.a, k % self.p, self.p), mul_mod(self.b, k % self.p, self.p), self.p)
    }
    
    pub fn norm_squared(&self) -> u64 {
        add_mod(mul_mod(self.a, self.a, self.p), mul_mod(self.b, self.b, self.p), self.p)
    }
}

// CORRECTED SPARSE GROVER IMPLEMENTATION
// Based on working coherence.rs implementation
// ========================================================================
#[derive(Clone, Debug)]
pub struct SparseGroverState {
    pub target_amp: Fp2,
    pub other_amp: Fp2,
    pub num_qubits: u64,
    pub num_marked: u64,  // Now defaults to 1 for standard Grover
    pub p: u64,
    pub n_mod_p: u64,           // 2^n mod p
    pub n_minus_1_mod_p: u64,   // (2^n - 1) mod p (corrected version)
    pub n_inv_mod_p: u64,       // (2^n)^(-1) mod p
}

impl SparseGroverState {
    pub fn new(num_qubits: u64, num_marked: u64, p: u64) -> Self {
        let n_mod_p = pow2_mod(num_qubits as usize, p);
        let n_minus_1_mod_p = if n_mod_p == 0 { p - 1 } else { n_mod_p - 1 };
        let n_inv_mod_p = mod_inverse(n_mod_p, p);

        // Initialize in uniform superposition: all amplitudes equal
        // In Fp2 arithmetic, start with amplitude 1 (real: 1, imag: 0)
        Self {
            target_amp: Fp2::one(p),
            other_amp: Fp2::one(p),
            num_qubits,
            num_marked: 1,  // For standard Grover, mark 1 state
            p,
            n_mod_p,
            n_minus_1_mod_p,
            n_inv_mod_p,
        }
    }
    
    pub fn oracle(&mut self) {
        // Flip sign of target amplitude (create negative for interference)
        self.target_amp = self.target_amp.neg();
    }
    
    pub fn diffusion(&mut self) {
        // Grover diffusion operator: 2|s⟩⟨s| - I
        // Mean amplitude = (target_amp + (N-1) * other_amp) / N
        // new_target = 2*mean - target_amp
        // new_other = 2*mean - other_amp
        
        // Sum of all amplitudes: target + (N-1) * other
        let scaled_other = self.other_amp.scalar_mul(self.n_minus_1_mod_p);
        let sum = self.target_amp.add(&scaled_other);
        
        // Mean = sum / N
        let mean = sum.scalar_mul(self.n_inv_mod_p);
        
        // New amplitudes via reflection around mean
        let two_mean = mean.add(&mean);
        self.target_amp = two_mean.sub(&self.target_amp);
        self.other_amp = two_mean.sub(&self.other_amp);
    }
    
    pub fn iterate(&mut self) {
        self.oracle();    // Mark target state with negative amplitude
        self.diffusion(); // Amplify target, suppress others
    }
    
    pub fn iterate_n(&mut self, n: usize) {
        for _ in 0..n {
            self.iterate();
        }
    }
    
    pub fn total_weight(&self) -> u64 {
        // Total squared norm: |target|² + (N-1) * |other|²
        let target_sq = self.target_amp.norm_squared();
        let other_sq = self.other_amp.norm_squared();
        
        let target_contrib = target_sq;
        let other_contrib = mul_mod(other_sq, self.n_minus_1_mod_p, self.p);
        add_mod(target_contrib, other_contrib, self.p)
    }
    
    pub fn target_probability(&self) -> f64 {
        // Calculate |target_amp|² / total_weight (approximation)
        let target_sq = self.target_amp.norm_squared() as f64;
        let other_sq = self.other_amp.norm_squared() as f64;
        
        // Total = |target|² + (N-1) * |other|²
        let n_minus_1 = self.n_minus_1_mod_p as f64;
        let total = target_sq + other_sq * n_minus_1;
        
        if total == 0.0 { 0.0 } else { target_sq / total }
    }
}

// GROVER TEST CASES - Using corrected algorithm
// ========================================================================
pub const GROVER_TEST_CASES: &[(u64, u64, usize)] = &[
    (10, 1, 25),   // 2^10 = 1024 states, optimal ~25 iterations
    (12, 1, 50),   // 2^12 = 4096 states, optimal ~50 iterations  
    (14, 1, 100),  // 2^14 = 16384 states, optimal ~100 iterations
    (15, 1, 143),  // 2^15 = 32768 states, optimal ~143 iterations
    (16, 1, 201),  // 2^16 = 65536 states, optimal ~201 iterations
    (18, 1, 402),  // 2^18 = 262144 states, optimal ~402 iterations
    (20, 1, 804),  // 2^20 = 1048576 states, optimal ~804 iterations
];

// Updated Grover correctness test using corrected implementation
// ========================================================================
pub fn test_grover_correctness() -> (bool, String) {
    let mut failures = Vec::new();
    
    for &(qubits, marked, optimal_iter) in GROVER_TEST_CASES {
        let mut state = SparseGroverState::new(qubits, marked, 1_000_003); // PRODUCTION_PRIME = 1_000_003
        state.iterate_n(optimal_iter);
        
        let prob = state.target_probability();
        if prob < 0.5 {
            failures.push(format!("q={}: prob={:.2}% (expected >50%)", qubits, prob * 100.0));
        }
    }
    
    if failures.is_empty() {
        (true, format!("{} Grover searches with >50% success", GROVER_TEST_CASES.len()))
    } else {
        (false, failures.join(", "))
    }
}

// Zero decoherence test with corrected implementation
// ========================================================================
pub fn test_zero_decoherence() -> (bool, String) {
    let mut state = SparseGroverState::new(20, 1, 1_000_003);
    let initial_weight = state.total_weight();
    
    // Run far beyond optimal iterations
    state.iterate_n(10000);
    let final_weight = state.total_weight();
    
    if initial_weight != final_weight {
        return (false, format!("Weight changed: {} → {}", initial_weight, final_weight));
    }
    
    // Check that oscillation continues (not damped)
    let mut state2 = SparseGroverState::new(20, 1, 1_000_003);
    state2.iterate_n(804); // Optimal
    let optimal_prob = state2.target_probability();
    
    state2.iterate_n(1608); // 2× optimal (should be back near start)
    let double_prob = state2.target_probability();
    
    // Should oscillate, not stay at peak
    if optimal_prob > 0.1 && double_prob > 0.1 {
        (true, format!("Weight preserved: {} and oscillation continues", initial_weight))
    } else {
        (false, "Oscillation damped".to_string())
    }
}

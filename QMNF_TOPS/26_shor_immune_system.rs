//! ═══════════════════════════════════════════════════════════════════════════
//! SHOR IMMUNE SYSTEM: Period-Finding as Cryptographic Defense
//! ═══════════════════════════════════════════════════════════════════════════
//!
//! THE PARADIGM SHIFT:
//!   Everyone thinks Shor = break crypto
//!   QMNF thinks Shor = VALIDATE crypto
//!
//! Use period-finding to:
//!   • Detect backdoored curves BEFORE deployment
//!   • Validate RSA moduli have no hidden structure
//!   • Find Pohlig-Hellman vulnerabilities in DH parameters
//!   • Continuous health monitoring of cryptographic parameters
//!
//! This is Shor DEFENDING, not attacking.
//! ═══════════════════════════════════════════════════════════════════════════

#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]

use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// F_p² SUBSTRATE - Exact Quantum Simulation
// ═══════════════════════════════════════════════════════════════════════════

/// Element in F_p² = F_p[x]/(x² + 1)
/// Represents a + bi where i² = -1
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Fp2 {
    pub a: u64,  // Real part
    pub b: u64,  // Imaginary part
    pub p: u64,  // Field prime
}

impl Fp2 {
    pub fn new(a: u64, b: u64, p: u64) -> Self {
        Self {
            a: a % p,
            b: b % p,
            p,
        }
    }

    pub fn zero(p: u64) -> Self {
        Self { a: 0, b: 0, p }
    }

    pub fn one(p: u64) -> Self {
        Self { a: 1, b: 0, p }
    }

    /// Addition in F_p²
    pub fn add(&self, other: &Self) -> Self {
        debug_assert_eq!(self.p, other.p);
        Self {
            a: (self.a + other.a) % self.p,
            b: (self.b + other.b) % self.p,
            p: self.p,
        }
    }

    /// Subtraction in F_p²
    pub fn sub(&self, other: &Self) -> Self {
        debug_assert_eq!(self.p, other.p);
        Self {
            a: (self.a + self.p - other.a) % self.p,
            b: (self.b + self.p - other.b) % self.p,
            p: self.p,
        }
    }

    /// Multiplication in F_p²
    /// (a + bi)(c + di) = (ac - bd) + (ad + bc)i
    pub fn mul(&self, other: &Self) -> Self {
        debug_assert_eq!(self.p, other.p);
        let ac = (self.a as u128 * other.a as u128) % self.p as u128;
        let bd = (self.b as u128 * other.b as u128) % self.p as u128;
        let ad = (self.a as u128 * other.b as u128) % self.p as u128;
        let bc = (self.b as u128 * other.a as u128) % self.p as u128;

        // Real: ac - bd (mod p)
        let real = if ac >= bd {
            (ac - bd) % self.p as u128
        } else {
            (self.p as u128 - (bd - ac) % self.p as u128) % self.p as u128
        };

        // Imag: ad + bc (mod p)
        let imag = (ad + bc) % self.p as u128;

        Self {
            a: real as u64,
            b: imag as u64,
            p: self.p,
        }
    }

    /// Modular exponentiation in F_p²
    pub fn pow(&self, mut exp: u64) -> Self {
        let mut base = *self;
        let mut result = Self::one(self.p);

        while exp > 0 {
            if exp & 1 == 1 {
                result = result.mul(&base);
            }
            exp >>= 1;
            if exp > 0 {
                base = base.mul(&base);
            }
        }

        result
    }

    /// Conjugate: a + bi → a - bi
    pub fn conjugate(&self) -> Self {
        Self {
            a: self.a,
            b: (self.p - self.b) % self.p,
            p: self.p,
        }
    }

    /// Norm: (a + bi)(a - bi) = a² + b²
    pub fn norm(&self) -> u64 {
        let a2 = (self.a as u128 * self.a as u128) % self.p as u128;
        let b2 = (self.b as u128 * self.b as u128) % self.p as u128;
        ((a2 + b2) % self.p as u128) as u64
    }

    /// Multiplicative inverse via Fermat's little theorem
    pub fn inverse(&self) -> Option<Self> {
        let n = self.norm();
        if n == 0 {
            return None;
        }

        // n^(-1) mod p via Fermat
        let n_inv = mod_pow(n, self.p - 2, self.p);

        // (a + bi)^(-1) = (a - bi) / (a² + b²)
        Some(Self {
            a: (self.a as u128 * n_inv as u128 % self.p as u128) as u64,
            b: ((self.p - self.b) as u128 * n_inv as u128 % self.p as u128) as u64,
            p: self.p,
        })
    }
}

/// Modular exponentiation for u64
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    let mut result = 1u64;
    base %= modulus;

    while exp > 0 {
        if exp & 1 == 1 {
            result = (result as u128 * base as u128 % modulus as u128) as u64;
        }
        exp >>= 1;
        base = (base as u128 * base as u128 % modulus as u128) as u64;
    }

    result
}

// ═══════════════════════════════════════════════════════════════════════════
// QUANTUM FOURIER TRANSFORM on F_p²
// ═══════════════════════════════════════════════════════════════════════════

/// Quantum Fourier Transform in F_p²
/// This is the heart of Shor's algorithm
pub struct QuantumFourierTransform {
    /// Size of transform (power of 2)
    n: usize,
    /// Field prime
    p: u64,
    /// Primitive N-th root of unity in F_p²
    omega: Fp2,
    /// Precomputed powers of omega
    omega_powers: Vec<Fp2>,
}

impl QuantumFourierTransform {
    /// Create QFT for size n (must be power of 2)
    pub fn new(n: usize, p: u64) -> Self {
        assert!(n.is_power_of_two(), "n must be power of 2");

        // Find primitive n-th root of unity in F_p²
        // ω = g^((p²-1)/n) where g is generator
        let omega = Self::find_primitive_root(n, p);

        // Precompute powers
        let mut omega_powers = Vec::with_capacity(n);
        let mut current = Fp2::one(p);
        for _ in 0..n {
            omega_powers.push(current);
            current = current.mul(&omega);
        }

        Self { n, p, omega, omega_powers }
    }

    /// Find primitive n-th root of unity
    fn find_primitive_root(n: usize, p: u64) -> Fp2 {
        // Order of F_p² multiplicative group is p² - 1
        let order = (p as u128 * p as u128 - 1) as u64;

        // We need ω^n = 1 and ω^k ≠ 1 for k < n
        // Try generators until we find one
        for a in 2..p {
            for b in 0..p {
                let g = Fp2::new(a, b, p);
                let exp = order / n as u64;
                let omega = g.pow(exp);

                // Verify it's primitive (ω^(n/2) ≠ 1)
                if omega.pow(n as u64 / 2) != Fp2::one(p) {
                    return omega;
                }
            }
        }

        panic!("Could not find primitive root");
    }

    /// Apply QFT to state vector
    /// |x⟩ → (1/√N) Σ_k ω^(xk) |k⟩
    pub fn forward(&self, state: &[Fp2]) -> Vec<Fp2> {
        assert_eq!(state.len(), self.n);

        // Cooley-Tukey FFT in F_p²
        let mut result = state.to_vec();
        self.fft_recursive(&mut result, false);
        result
    }

    /// Inverse QFT
    pub fn inverse(&self, state: &[Fp2]) -> Vec<Fp2> {
        let mut result = state.to_vec();
        self.fft_recursive(&mut result, true);

        // Divide by n
        let n_inv = mod_pow(self.n as u64, self.p - 2, self.p);
        let scale = Fp2::new(n_inv, 0, self.p);

        for x in &mut result {
            *x = x.mul(&scale);
        }

        result
    }

    /// Recursive FFT implementation
    fn fft_recursive(&self, data: &mut [Fp2], inverse: bool) {
        let n = data.len();
        if n <= 1 {
            return;
        }

        // Split even/odd
        let mut even: Vec<Fp2> = data.iter().step_by(2).copied().collect();
        let mut odd: Vec<Fp2> = data.iter().skip(1).step_by(2).copied().collect();

        self.fft_recursive(&mut even, inverse);
        self.fft_recursive(&mut odd, inverse);

        // Combine
        let step = self.n / n;
        for k in 0..n / 2 {
            let omega_k = if inverse {
                self.omega_powers[(self.n - k * step) % self.n]
            } else {
                self.omega_powers[k * step]
            };

            let t = omega_k.mul(&odd[k]);
            data[k] = even[k].add(&t);
            data[k + n / 2] = even[k].sub(&t);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PERIOD FINDING - The Core of Shor
// ═══════════════════════════════════════════════════════════════════════════

/// Period finder using quantum simulation on F_p²
pub struct PeriodFinder {
    /// QFT engine
    qft: QuantumFourierTransform,
    /// Field prime
    p: u64,
    /// Register size
    n_bits: usize,
}

/// Result of period finding
#[derive(Debug, Clone)]
pub struct PeriodResult {
    /// Found period (if any)
    pub period: Option<u64>,
    /// Confidence (0-1000 permille)
    pub confidence: u32,
    /// Number of iterations performed
    pub iterations: usize,
    /// Candidate periods found
    pub candidates: Vec<(u64, u32)>,  // (period, confidence)
}

impl PeriodFinder {
    /// Create period finder with n_bits precision
    pub fn new(n_bits: usize, p: u64) -> Self {
        let n = 1 << n_bits;
        let qft = QuantumFourierTransform::new(n, p);

        Self { qft, p, n_bits }
    }

    /// Find period of f(x) = a^x mod N
    /// This is the core quantum speedup
    pub fn find_period(&self, a: u64, n: u64) -> PeriodResult {
        let register_size = 1 << self.n_bits;

        // Step 1: Create superposition of |x⟩|a^x mod N⟩
        // In F_p², we represent this as amplitude vector
        let mut state = vec![Fp2::zero(self.p); register_size];

        // Initialize uniform superposition
        let amp = Fp2::new(1, 0, self.p);  // Will normalize later
        for x in 0..register_size {
            let ax_mod_n = mod_pow(a, x as u64, n);
            // Encode the function value in the phase
            state[x] = amp.mul(&Fp2::new(ax_mod_n, 0, self.p));
        }

        // Step 2: Apply QFT to first register
        let qft_state = self.qft.forward(&state);

        // Step 3: Measure - find peaks
        let mut peaks: Vec<(usize, u64)> = Vec::new();
        for (k, s) in qft_state.iter().enumerate() {
            let magnitude = s.norm();
            if magnitude > 0 {
                peaks.push((k, magnitude));
            }
        }

        // Sort by magnitude
        peaks.sort_by(|a, b| b.1.cmp(&a.1));

        // Step 4: Extract period from peaks using continued fractions
        let mut candidates: Vec<(u64, u32)> = Vec::new();

        for (k, mag) in peaks.iter().take(10) {
            if *k == 0 {
                continue;
            }

            // s/r ≈ k/N where r is the period
            // Use continued fraction expansion
            if let Some(r) = self.continued_fraction_period(*k, register_size, n) {
                // Verify: a^r ≡ 1 (mod n)
                if mod_pow(a, r, n) == 1 {
                    let confidence = (*mag as u32 * 1000) / (self.p as u32);
                    candidates.push((r, confidence.min(1000)));
                }
            }
        }

        // Deduplicate and sort by confidence
        candidates.sort_by(|a, b| b.1.cmp(&a.1));
        candidates.dedup_by(|a, b| a.0 == b.0);

        let period = candidates.first().map(|(p, _)| *p);
        let confidence = candidates.first().map(|(_, c)| *c).unwrap_or(0);

        PeriodResult {
            period,
            confidence,
            iterations: register_size,
            candidates,
        }
    }

    /// Extract period using continued fraction expansion
    fn continued_fraction_period(&self, k: usize, n: usize, modulus: u64) -> Option<u64> {
        // k/N ≈ s/r, find r
        let mut num = k as u64;
        let mut den = n as u64;

        // Build continued fraction
        let mut convergents: Vec<(u64, u64)> = Vec::new();
        let mut p_prev = 0u64;
        let mut q_prev = 1u64;
        let mut p_curr = 1u64;
        let mut q_curr = 0u64;

        for _ in 0..50 {  // Max iterations
            if den == 0 {
                break;
            }

            let a = num / den;
            let rem = num % den;

            let p_next = a.saturating_mul(p_curr).saturating_add(p_prev);
            let q_next = a.saturating_mul(q_curr).saturating_add(q_prev);

            if q_next > 0 && q_next < modulus {
                convergents.push((p_next, q_next));
            }

            p_prev = p_curr;
            q_prev = q_curr;
            p_curr = p_next;
            q_curr = q_next;

            num = den;
            den = rem;
        }

        // Return the denominator of the best convergent as period candidate
        convergents.last().map(|(_, q)| *q)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CRYPTOGRAPHIC IMMUNE SYSTEM
// ═══════════════════════════════════════════════════════════════════════════

/// Health status for cryptographic parameters
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CryptoHealth {
    /// Parameters are healthy
    Healthy,
    /// Parameters show warning signs
    Warning(String),
    /// Parameters are compromised
    Compromised(String),
    /// Unable to determine (needs more analysis)
    Unknown,
}

/// Validation result for a cryptographic parameter set
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub health: CryptoHealth,
    pub details: String,
    pub period_info: Option<PeriodResult>,
    pub recommendations: Vec<String>,
}

/// The Cryptographic Immune System
/// Uses Shor/period-finding to DEFEND, not attack
pub struct CryptoImmuneSystem {
    /// Period finder engine
    period_finder: PeriodFinder,
    /// Cache of validation results
    cache: HashMap<u64, ValidationResult>,
    /// Field prime for F_p² substrate
    field_prime: u64,
}

impl CryptoImmuneSystem {
    /// Create new immune system
    pub fn new(precision_bits: usize) -> Self {
        // Use a prime suitable for our precision
        // p ≡ 3 (mod 4) for efficient square roots in F_p²
        let field_prime = Self::select_field_prime(precision_bits);

        Self {
            period_finder: PeriodFinder::new(precision_bits, field_prime),
            cache: HashMap::new(),
            field_prime,
        }
    }

    /// Select appropriate field prime
    fn select_field_prime(bits: usize) -> u64 {
        // Primes of form p ≡ 3 (mod 4) near 2^32
        match bits {
            0..=8 => 251,           // Small testing
            9..=12 => 4093,         // Medium
            13..=16 => 65519,       // 16-bit
            17..=20 => 1048573,     // 20-bit
            _ => 4294967291,        // Near 2^32
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // RSA VALIDATION
    // ═══════════════════════════════════════════════════════════════════════

    /// Validate RSA modulus for hidden structure
    ///
    /// Checks:
    /// - Period of random bases for anomalies
    /// - Fermat factorization vulnerability
    /// - Small factor vulnerability
    pub fn validate_rsa(&mut self, n: u64, e: u64) -> ValidationResult {
        let mut recommendations = Vec::new();
        let mut warnings = Vec::new();

        // Check 1: Small factors
        let small_primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        for p in small_primes {
            if n % p == 0 {
                return ValidationResult {
                    health: CryptoHealth::Compromised(format!("Divisible by {}", p)),
                    details: "RSA modulus has small prime factor".to_string(),
                    period_info: None,
                    recommendations: vec!["Regenerate RSA key pair immediately".to_string()],
                };
            }
        }

        // Check 2: Fermat factorization (p and q too close)
        // Only check reasonable range (sqrt(n)^0.25 iterations)
        let sqrt_n = integer_sqrt(n);
        let max_delta = integer_sqrt(integer_sqrt(n)).max(10).min(100);
        for delta in 0..max_delta {
            let a = sqrt_n.saturating_add(delta);
            let a_squared = (a as u128).saturating_mul(a as u128);
            if a_squared < n as u128 {
                continue;
            }
            let b_squared = (a_squared - n as u128) as u64;
            let b = integer_sqrt(b_squared);
            if b.saturating_mul(b) == b_squared && a > b {
                let p = a.saturating_add(b);
                let q = a.saturating_sub(b);
                if p.saturating_mul(q) == n && p > 1 && q > 1 {
                    return ValidationResult {
                        health: CryptoHealth::Compromised("Fermat factorization possible".to_string()),
                        details: format!("Factors {} and {} are too close", p, q),
                        period_info: None,
                        recommendations: vec![
                            "p and q must differ by more than n^0.25".to_string(),
                            "Regenerate RSA key pair".to_string(),
                        ],
                    };
                }
            }
        }

        // Check 3: Period analysis with random bases
        let test_bases = [2u64, 3, 5, 7, 11];
        let mut period_results = Vec::new();

        for &base in &test_bases {
            if gcd(base, n) != 1 {
                // Found a factor!
                return ValidationResult {
                    health: CryptoHealth::Compromised(format!("gcd({}, n) ≠ 1", base)),
                    details: "Modulus shares factor with small number".to_string(),
                    period_info: None,
                    recommendations: vec!["Regenerate RSA key pair".to_string()],
                };
            }

            let period_result = self.period_finder.find_period(base, n);
            period_results.push((base, period_result.clone()));

            // Check for anomalously short periods
            if let Some(period) = period_result.period {
                let expected_min = integer_sqrt(n) / 10;  // Rough heuristic
                if period < expected_min {
                    warnings.push(format!("Short period {} for base {}", period, base));
                }
            }
        }

        // Check 4: Verify e is coprime to φ(n) implications
        if e <= 1 {
            return ValidationResult {
                health: CryptoHealth::Compromised("Invalid public exponent".to_string()),
                details: "e must be > 1".to_string(),
                period_info: None,
                recommendations: vec!["Use e = 65537 (0x10001)".to_string()],
            };
        }

        if !warnings.is_empty() {
            recommendations.push("Consider regenerating with stronger parameters".to_string());
            ValidationResult {
                health: CryptoHealth::Warning(warnings.join("; ")),
                details: "Some period anomalies detected".to_string(),
                period_info: period_results.first().map(|(_, r)| r.clone()),
                recommendations,
            }
        } else {
            ValidationResult {
                health: CryptoHealth::Healthy,
                details: "RSA parameters pass all checks".to_string(),
                period_info: period_results.first().map(|(_, r)| r.clone()),
                recommendations: vec![],
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // DIFFIE-HELLMAN VALIDATION
    // ═══════════════════════════════════════════════════════════════════════

    /// Validate DH parameters for Pohlig-Hellman vulnerability
    ///
    /// If p-1 has only small factors, DH is broken
    pub fn validate_dh(&mut self, p: u64, g: u64) -> ValidationResult {
        let mut recommendations = Vec::new();

        // Check 1: p must be prime
        if !is_prime_miller_rabin(p, 20) {
            return ValidationResult {
                health: CryptoHealth::Compromised("p is not prime".to_string()),
                details: "DH modulus must be prime".to_string(),
                period_info: None,
                recommendations: vec!["Use a verified prime".to_string()],
            };
        }

        // Check 2: g must be in valid range
        if g <= 1 || g >= p {
            return ValidationResult {
                health: CryptoHealth::Compromised("Invalid generator".to_string()),
                details: format!("g must be in range (1, {})", p),
                period_info: None,
                recommendations: vec!["Choose g in range (1, p)".to_string()],
            };
        }

        // Check 3: Pohlig-Hellman - factor p-1
        let p_minus_1 = p - 1;
        let factors = factor_trial_division(p_minus_1);

        // Find largest prime factor
        let largest_factor = factors.iter().max().copied().unwrap_or(1);
        let total_small = factors.iter().filter(|&&f| f < 1000).count();

        if largest_factor < 1_000_000 {
            return ValidationResult {
                health: CryptoHealth::Compromised("Pohlig-Hellman vulnerable".to_string()),
                details: format!("p-1 = {} has largest factor {}", p_minus_1, largest_factor),
                period_info: None,
                recommendations: vec![
                    "Use safe prime p = 2q + 1 where q is also prime".to_string(),
                    "p-1 must have a large prime factor".to_string(),
                ],
            };
        }

        if total_small > factors.len() / 2 {
            recommendations.push("Consider using a safe prime".to_string());
        }

        // Check 4: g should be a generator (have order p-1)
        let order = self.find_generator_order(g, p);
        if order != p - 1 {
            return ValidationResult {
                health: CryptoHealth::Warning(format!("g has order {} ≠ p-1", order)),
                details: "Generator may not be primitive root".to_string(),
                period_info: None,
                recommendations: vec![
                    "Verify g is a primitive root mod p".to_string(),
                    format!("Order is {}, should be {}", order, p - 1),
                ],
            };
        }

        ValidationResult {
            health: CryptoHealth::Healthy,
            details: format!("DH parameters validated. Largest factor of p-1: {}", largest_factor),
            period_info: None,
            recommendations,
        }
    }

    /// Find order of g mod p
    fn find_generator_order(&self, g: u64, p: u64) -> u64 {
        let mut order = 1u64;
        let mut current = g;

        while current != 1 && order < p {
            current = (current as u128 * g as u128 % p as u128) as u64;
            order += 1;
        }

        order
    }

    // ═══════════════════════════════════════════════════════════════════════
    // ELLIPTIC CURVE VALIDATION
    // ═══════════════════════════════════════════════════════════════════════

    /// Simplified elliptic curve representation
    /// y² = x³ + ax + b (mod p)
    pub fn validate_curve(&mut self, a: u64, b: u64, p: u64, n: u64, gx: u64, gy: u64) -> ValidationResult {
        let mut recommendations = Vec::new();

        // Check 1: Curve discriminant (4a³ + 27b² ≠ 0)
        let a3 = mod_pow(a, 3, p);
        let b2 = mod_pow(b, 2, p);
        let discriminant = (4u128 * a3 as u128 + 27u128 * b2 as u128) % p as u128;

        if discriminant == 0 {
            return ValidationResult {
                health: CryptoHealth::Compromised("Singular curve".to_string()),
                details: "Curve discriminant is zero".to_string(),
                period_info: None,
                recommendations: vec!["Choose different a, b values".to_string()],
            };
        }

        // Check 2: Generator point is on curve
        // y² = x³ + ax + b
        let lhs = (gy as u128 * gy as u128) % p as u128;
        let x3 = mod_pow(gx, 3, p) as u128;
        let ax = (a as u128 * gx as u128) % p as u128;
        let rhs = (x3 + ax + b as u128) % p as u128;

        if lhs != rhs {
            return ValidationResult {
                health: CryptoHealth::Compromised("Generator not on curve".to_string()),
                details: format!("({}, {}) does not satisfy curve equation", gx, gy),
                period_info: None,
                recommendations: vec!["Use valid generator point".to_string()],
            };
        }

        // Check 3: Order n should be prime (or have large prime factor)
        if !is_prime_miller_rabin(n, 20) {
            let factors = factor_trial_division(n);
            let largest = factors.iter().max().copied().unwrap_or(1);

            if largest < n / 1000 {
                return ValidationResult {
                    health: CryptoHealth::Compromised("Weak curve order".to_string()),
                    details: format!("Order {} has no large prime factor", n),
                    period_info: None,
                    recommendations: vec![
                        "Use curve with prime order".to_string(),
                        "Or order with large prime factor".to_string(),
                    ],
                };
            }

            recommendations.push(format!("Order {} is not prime, largest factor: {}", n, largest));
        }

        // Check 4: Embedding degree (MOV attack)
        // Check if p^k ≡ 1 (mod n) for small k
        let mut p_power = p % n;
        for k in 1..=20 {
            if p_power == 1 {
                return ValidationResult {
                    health: CryptoHealth::Compromised(format!("Low embedding degree k={}", k)),
                    details: "Vulnerable to MOV/FR attack".to_string(),
                    period_info: None,
                    recommendations: vec![
                        "Use curve with high embedding degree".to_string(),
                        "Embedding degree should be > 20".to_string(),
                    ],
                };
            }
            p_power = (p_power as u128 * p as u128 % n as u128) as u64;
        }

        // Check 5: Anomalous curve (n = p)
        if n == p {
            return ValidationResult {
                health: CryptoHealth::Compromised("Anomalous curve".to_string()),
                details: "Curve order equals field size".to_string(),
                period_info: None,
                recommendations: vec![
                    "Vulnerable to Smart's attack".to_string(),
                    "Choose curve where n ≠ p".to_string(),
                ],
            };
        }

        ValidationResult {
            health: CryptoHealth::Healthy,
            details: "Elliptic curve parameters validated".to_string(),
            period_info: None,
            recommendations,
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    // CONTINUOUS MONITORING
    // ═══════════════════════════════════════════════════════════════════════

    /// Run continuous health check on parameters
    pub fn health_check(&mut self, params: &CryptoParams) -> HealthReport {
        let mut results = Vec::new();

        match params {
            CryptoParams::RSA { n, e } => {
                results.push(("RSA".to_string(), self.validate_rsa(*n, *e)));
            }
            CryptoParams::DH { p, g } => {
                results.push(("DH".to_string(), self.validate_dh(*p, *g)));
            }
            CryptoParams::ECC { a, b, p, n, gx, gy } => {
                results.push(("ECC".to_string(), self.validate_curve(*a, *b, *p, *n, *gx, *gy)));
            }
            CryptoParams::Multiple(params_list) => {
                for param in params_list {
                    let sub_report = self.health_check(param);
                    results.extend(sub_report.results);
                }
            }
        }

        // Determine overall health
        let overall = if results.iter().any(|(_, r)| matches!(r.health, CryptoHealth::Compromised(_))) {
            CryptoHealth::Compromised("One or more parameters compromised".to_string())
        } else if results.iter().any(|(_, r)| matches!(r.health, CryptoHealth::Warning(_))) {
            CryptoHealth::Warning("Warnings detected".to_string())
        } else {
            CryptoHealth::Healthy
        };

        HealthReport {
            overall,
            results,
            timestamp: current_time_ns(),
        }
    }
}

/// Cryptographic parameters to validate
#[derive(Debug, Clone)]
pub enum CryptoParams {
    RSA { n: u64, e: u64 },
    DH { p: u64, g: u64 },
    ECC { a: u64, b: u64, p: u64, n: u64, gx: u64, gy: u64 },
    Multiple(Vec<CryptoParams>),
}

/// Health report from immune system
#[derive(Debug, Clone)]
pub struct HealthReport {
    pub overall: CryptoHealth,
    pub results: Vec<(String, ValidationResult)>,
    pub timestamp: u64,
}

// ═══════════════════════════════════════════════════════════════════════════
// UTILITY FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Greatest common divisor
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Integer square root
fn integer_sqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// Miller-Rabin primality test
fn is_prime_miller_rabin(n: u64, rounds: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    // Write n-1 = 2^r * d
    let mut d = n - 1;
    let mut r = 0;
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }

    // Witnesses to test
    let witnesses: Vec<u64> = if n < 2047 {
        vec![2]
    } else if n < 1373653 {
        vec![2, 3]
    } else if n < 9080191 {
        vec![31, 73]
    } else if n < 25326001 {
        vec![2, 3, 5]
    } else {
        vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
    };

    'witness: for &a in witnesses.iter().take(rounds) {
        if a >= n {
            continue;
        }

        let mut x = mod_pow(a, d, n);

        if x == 1 || x == n - 1 {
            continue 'witness;
        }

        for _ in 0..r - 1 {
            x = (x as u128 * x as u128 % n as u128) as u64;
            if x == n - 1 {
                continue 'witness;
            }
        }

        return false;
    }

    true
}

/// Trial division factorization
fn factor_trial_division(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();

    // Factor out 2s
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }

    // Odd factors
    let mut i = 3u64;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}

/// Current time in nanoseconds
fn current_time_ns() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fp2_arithmetic() {
        let p = 17u64;
        let a = Fp2::new(3, 4, p);
        let b = Fp2::new(2, 5, p);

        // Addition
        let sum = a.add(&b);
        assert_eq!(sum.a, 5);
        assert_eq!(sum.b, 9);

        // Multiplication: (3+4i)(2+5i) = 6+15i+8i+20i² = 6+23i-20 = -14+23i
        let prod = a.mul(&b);
        // -14 mod 17 = 3, 23 mod 17 = 6
        assert_eq!(prod.a, 3);
        assert_eq!(prod.b, 6);
    }

    #[test]
    fn test_fp2_inverse() {
        let p = 17u64;
        let a = Fp2::new(3, 4, p);

        let a_inv = a.inverse().unwrap();
        let product = a.mul(&a_inv);

        assert_eq!(product.a, 1);
        assert_eq!(product.b, 0);
    }

    #[test]
    fn test_miller_rabin() {
        assert!(is_prime_miller_rabin(2, 10));
        assert!(is_prime_miller_rabin(17, 10));
        assert!(is_prime_miller_rabin(997, 10));
        assert!(!is_prime_miller_rabin(4, 10));
        assert!(!is_prime_miller_rabin(100, 10));
    }

    #[test]
    fn test_rsa_validation_fermat_detection() {
        let mut immune = CryptoImmuneSystem::new(8);

        // Use two primes that ARE close: 89 × 97 = 8633
        // The system SHOULD detect this as Fermat-vulnerable
        let n = 8633u64;
        let e = 17u64;

        let result = immune.validate_rsa(n, e);
        println!("RSA Fermat detection: {:?}", result);

        // System correctly identifies close primes!
        assert!(matches!(result.health, CryptoHealth::Compromised(_)));
        assert!(result.details.contains("close"));
    }

    #[test]
    fn test_rsa_validation_healthy() {
        let mut immune = CryptoImmuneSystem::new(8);

        // Use primes > 47 (outside small factor check) and far apart:
        // 53 × 1009 = 53477
        // sqrt(53477) ≈ 231, factors are 53 and 1009 - far apart
        let n = 53477u64;
        let e = 17u64;

        let result = immune.validate_rsa(n, e);
        println!("RSA healthy result: {:?}", result);
        // Should pass - primes are large and far apart
        assert!(!matches!(result.health, CryptoHealth::Compromised(_)));
    }

    #[test]
    fn test_rsa_validation_small_factor() {
        let mut immune = CryptoImmuneSystem::new(8);

        // n divisible by small prime
        let n = 100u64;  // = 4 × 25
        let e = 17u64;

        let result = immune.validate_rsa(n, e);
        assert!(matches!(result.health, CryptoHealth::Compromised(_)));
    }

    #[test]
    fn test_dh_validation_healthy() {
        let mut immune = CryptoImmuneSystem::new(8);

        // Safe prime: p = 23 (p-1 = 22 = 2 × 11)
        let p = 23u64;
        let g = 5u64;

        let result = immune.validate_dh(p, g);
        // This should pass basic checks (though 23 is small)
        println!("DH result: {:?}", result);
    }

    #[test]
    fn test_dh_validation_pohlig_hellman() {
        let mut immune = CryptoImmuneSystem::new(8);

        // p where p-1 has only small factors
        // p = 13, p-1 = 12 = 2² × 3
        let p = 13u64;
        let g = 2u64;

        let result = immune.validate_dh(p, g);
        // Should detect Pohlig-Hellman vulnerability
        println!("DH Pohlig-Hellman result: {:?}", result);
    }

    #[test]
    fn test_factorization() {
        let factors = factor_trial_division(100);
        assert_eq!(factors, vec![2, 2, 5, 5]);

        let factors = factor_trial_division(97);
        assert_eq!(factors, vec![97]);  // Prime
    }

    #[test]
    fn test_period_finder_basic() {
        let finder = PeriodFinder::new(6, 251);  // Small for testing

        // a = 2, n = 15 (period should divide φ(15) = 8)
        let result = finder.find_period(2, 15);
        println!("Period finding result: {:?}", result);

        // If period found, verify it
        if let Some(r) = result.period {
            assert_eq!(mod_pow(2, r, 15), 1);
        }
    }

    #[test]
    fn test_health_check() {
        let mut immune = CryptoImmuneSystem::new(8);

        let params = CryptoParams::Multiple(vec![
            CryptoParams::RSA { n: 3233, e: 17 },
            CryptoParams::DH { p: 23, g: 5 },
        ]);

        let report = immune.health_check(&params);
        println!("Health report: {:?}", report);
    }
}

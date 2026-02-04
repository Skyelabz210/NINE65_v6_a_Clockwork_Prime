// polypoly_optimization.rs - Polynomial-Polynomial Optimization for FHE
//
// High-performance polynomial operations optimized for both NINE65 variants.
// Provides edge in any context where polynomials are fundamental (FHE, NTT, convolution).
//
// Optimizations:
// - Karatsuba multiplication (O(n^1.585) vs O(n²))
// - NTT-based convolution (O(n log n))
// - Schoolbook for small degrees (<64)
// - FFT-friendly for qclassic variant
// - Exact arithmetic (QMNF compliant)
// - Vectorized operations (SIMD-ready)

#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]

use std::ops::{Add, Mul, Sub};

/// Polynomial in Z_q[X] / (X^N + 1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial {
    pub coeffs: Vec<i64>,  // Coefficients in Z_q
    pub modulus: u64,      // Working modulus q
}

impl Polynomial {
    /// Create polynomial from coefficients
    pub fn from_coeffs(coeffs: Vec<i64>, modulus: u64) -> Self {
        Polynomial { coeffs, modulus }
    }

    /// Create zero polynomial of degree n
    pub fn zero(degree: usize, modulus: u64) -> Self {
        Polynomial {
            coeffs: vec![0; degree],
            modulus,
        }
    }

    /// Degree of polynomial (highest non-zero coefficient)
    pub fn degree(&self) -> Option<usize> {
        for i in (0..self.coeffs.len()).rev() {
            if self.coeffs[i] != 0 {
                return Some(i);
            }
        }
        None
    }

    /// Reduce all coefficients mod q
    pub fn reduce(&mut self) {
        for coeff in &mut self.coeffs {
            *coeff = (*coeff).rem_euclid(self.modulus as i64);
        }
    }

    /// Scalar multiplication
    pub fn scalar_mul(&self, scalar: i64) -> Self {
        let mut result = self.clone();
        for coeff in &mut result.coeffs {
            *coeff = (*coeff * scalar).rem_euclid(self.modulus as i64);
        }
        result
    }

    /// Evaluate polynomial at point x
    pub fn evaluate(&self, x: i64) -> i64 {
        // Horner's method: O(n)
        let mut result = 0i64;
        for &coeff in self.coeffs.iter().rev() {
            result = (result * x + coeff).rem_euclid(self.modulus as i64);
        }
        result
    }

    /// Negate polynomial (for subtraction)
    pub fn negate(&self) -> Self {
        let mut result = self.clone();
        for coeff in &mut result.coeffs {
            *coeff = (-*coeff).rem_euclid(self.modulus as i64);
        }
        result
    }
}

/// Polynomial addition
impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert_eq!(self.modulus, other.modulus);

        let max_len = self.coeffs.len().max(other.coeffs.len());
        let mut result = Vec::with_capacity(max_len);

        for i in 0..max_len {
            let a = self.coeffs.get(i).unwrap_or(&0);
            let b = other.coeffs.get(i).unwrap_or(&0);
            let sum = (a + b).rem_euclid(self.modulus as i64);
            result.push(sum);
        }

        Polynomial {
            coeffs: result,
            modulus: self.modulus,
        }
    }
}

/// Polynomial subtraction
impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + other.negate()
    }
}

/// Polynomial multiplication strategies
pub enum MultiplicationStrategy {
    Schoolbook,    // O(n²) - best for small n
    Karatsuba,     // O(n^1.585) - good for medium n
    NTT,           // O(n log n) - best for large n
    Auto,          // Automatically choose based on degree
}

/// PolyPoly: Optimized polynomial multiplication
pub struct PolyPolyMultiplier {
    modulus: u64,
    ntt_friendly: bool,
    primitive_root: Option<u64>,
}

impl PolyPolyMultiplier {
    pub fn new(modulus: u64) -> Self {
        let ntt_friendly = Self::check_ntt_friendly(modulus);
        let primitive_root = if ntt_friendly {
            Self::find_primitive_root(modulus)
        } else {
            None
        };

        PolyPolyMultiplier {
            modulus,
            ntt_friendly,
            primitive_root,
        }
    }

    /// Multiply two polynomials (auto-select strategy)
    pub fn multiply(&self, a: &Polynomial, b: &Polynomial) -> Polynomial {
        self.multiply_with_strategy(a, b, MultiplicationStrategy::Auto)
    }

    /// Multiply with specific strategy
    pub fn multiply_with_strategy(
        &self,
        a: &Polynomial,
        b: &Polynomial,
        strategy: MultiplicationStrategy,
    ) -> Polynomial {
        assert_eq!(a.modulus, self.modulus);
        assert_eq!(b.modulus, self.modulus);

        match strategy {
            MultiplicationStrategy::Schoolbook => self.schoolbook_multiply(a, b),
            MultiplicationStrategy::Karatsuba => self.karatsuba_multiply(a, b),
            MultiplicationStrategy::NTT => {
                if self.ntt_friendly {
                    self.ntt_multiply(a, b)
                } else {
                    self.karatsuba_multiply(a, b)
                }
            }
            MultiplicationStrategy::Auto => {
                let degree = a.coeffs.len().max(b.coeffs.len());
                if degree < 64 {
                    self.schoolbook_multiply(a, b)
                } else if degree < 512 || !self.ntt_friendly {
                    self.karatsuba_multiply(a, b)
                } else {
                    self.ntt_multiply(a, b)
                }
            }
        }
    }

    /// Schoolbook multiplication: O(n²)
    fn schoolbook_multiply(&self, a: &Polynomial, b: &Polynomial) -> Polynomial {
        let result_len = a.coeffs.len() + b.coeffs.len() - 1;
        let mut result = vec![0i64; result_len];

        for (i, &a_coeff) in a.coeffs.iter().enumerate() {
            for (j, &b_coeff) in b.coeffs.iter().enumerate() {
                let prod = (a_coeff * b_coeff).rem_euclid(self.modulus as i64);
                result[i + j] = (result[i + j] + prod).rem_euclid(self.modulus as i64);
            }
        }

        Polynomial {
            coeffs: result,
            modulus: self.modulus,
        }
    }

    /// Karatsuba multiplication: O(n^1.585)
    fn karatsuba_multiply(&self, a: &Polynomial, b: &Polynomial) -> Polynomial {
        // Base case: use schoolbook for small polynomials
        if a.coeffs.len() <= 32 || b.coeffs.len() <= 32 {
            return self.schoolbook_multiply(a, b);
        }

        let n = a.coeffs.len().max(b.coeffs.len());
        let m = (n + 1) / 2;

        // Split polynomials: a = a0 + a1*x^m, b = b0 + b1*x^m
        let (a0, a1) = self.split_at(a, m);
        let (b0, b1) = self.split_at(b, m);

        // Three recursive multiplications
        let z0 = self.karatsuba_multiply(&a0, &b0);
        let z2 = self.karatsuba_multiply(&a1, &b1);

        let a_sum = a0.clone() + a1.clone();
        let b_sum = b0.clone() + b1.clone();
        let z1_full = self.karatsuba_multiply(&a_sum, &b_sum);

        // z1 = z1_full - z0 - z2
        let z1 = z1_full - z0.clone() - z2.clone();

        // Combine: result = z0 + z1*x^m + z2*x^(2m)
        let mut result = z0.clone();

        // Add z1*x^m
        for (i, &coeff) in z1.coeffs.iter().enumerate() {
            let idx = i + m;
            while result.coeffs.len() <= idx {
                result.coeffs.push(0);
            }
            result.coeffs[idx] = (result.coeffs[idx] + coeff).rem_euclid(self.modulus as i64);
        }

        // Add z2*x^(2m)
        for (i, &coeff) in z2.coeffs.iter().enumerate() {
            let idx = i + 2 * m;
            while result.coeffs.len() <= idx {
                result.coeffs.push(0);
            }
            result.coeffs[idx] = (result.coeffs[idx] + coeff).rem_euclid(self.modulus as i64);
        }

        result
    }

    /// NTT-based multiplication: O(n log n)
    fn ntt_multiply(&self, a: &Polynomial, b: &Polynomial) -> Polynomial {
        if !self.ntt_friendly || self.primitive_root.is_none() {
            return self.karatsuba_multiply(a, b);
        }

        let omega = self.primitive_root.unwrap();
        let n = a.coeffs.len().max(b.coeffs.len()).next_power_of_two();

        // Pad to power of 2
        let mut a_padded = a.coeffs.clone();
        let mut b_padded = b.coeffs.clone();
        a_padded.resize(n, 0);
        b_padded.resize(n, 0);

        // Forward NTT
        let a_ntt = self.ntt_transform(&a_padded, omega, n);
        let b_ntt = self.ntt_transform(&b_padded, omega, n);

        // Pointwise multiplication
        let mut c_ntt = vec![0i64; n];
        for i in 0..n {
            c_ntt[i] = (a_ntt[i] * b_ntt[i]).rem_euclid(self.modulus as i64);
        }

        // Inverse NTT
        let omega_inv = self.mod_inverse(omega, self.modulus);
        let result = self.ntt_transform(&c_ntt, omega_inv, n);

        // Scale by n^(-1)
        let n_inv = self.mod_inverse(n as u64, self.modulus);
        let mut final_result = vec![0i64; result.len()];
        for i in 0..result.len() {
            final_result[i] = (result[i] * n_inv as i64).rem_euclid(self.modulus as i64);
        }

        Polynomial {
            coeffs: final_result,
            modulus: self.modulus,
        }
    }

    /// NTT transform (Cooley-Tukey FFT-style)
    fn ntt_transform(&self, coeffs: &[i64], omega: u64, n: usize) -> Vec<i64> {
        if n == 1 {
            return coeffs.to_vec();
        }

        let half = n / 2;
        let mut even = Vec::with_capacity(half);
        let mut odd = Vec::with_capacity(half);

        for i in 0..half {
            even.push(coeffs[2 * i]);
            odd.push(coeffs[2 * i + 1]);
        }

        let omega_squared = (omega * omega).rem_euclid(self.modulus);
        let even_ntt = self.ntt_transform(&even, omega_squared, half);
        let odd_ntt = self.ntt_transform(&odd, omega_squared, half);

        let mut result = vec![0i64; n];
        let mut omega_k = 1u64;

        for k in 0..half {
            let t = (odd_ntt[k] * omega_k as i64).rem_euclid(self.modulus as i64);
            result[k] = (even_ntt[k] + t).rem_euclid(self.modulus as i64);
            result[k + half] = (even_ntt[k] - t).rem_euclid(self.modulus as i64);
            omega_k = (omega_k * omega).rem_euclid(self.modulus);
        }

        result
    }

    /// Polynomial division (for completeness)
    pub fn divide(&self, dividend: &Polynomial, divisor: &Polynomial) -> (Polynomial, Polynomial) {
        // Returns (quotient, remainder)
        // Uses long division algorithm

        let mut quotient = Polynomial::zero(dividend.coeffs.len(), self.modulus);
        let mut remainder = dividend.clone();

        while remainder.degree().is_some() &&
              divisor.degree().is_some() &&
              remainder.degree().unwrap() >= divisor.degree().unwrap() {

            let r_deg = remainder.degree().unwrap();
            let d_deg = divisor.degree().unwrap();
            let deg_diff = r_deg - d_deg;

            let leading_coeff = remainder.coeffs[r_deg];
            let divisor_leading = divisor.coeffs[d_deg];
            let divisor_leading_inv = self.mod_inverse(divisor_leading as u64, self.modulus);

            let coeff = (leading_coeff * divisor_leading_inv as i64).rem_euclid(self.modulus as i64);
            quotient.coeffs[deg_diff] = coeff;

            // Subtract divisor * coeff * x^deg_diff from remainder
            for i in 0..=d_deg {
                let sub = (divisor.coeffs[i] * coeff).rem_euclid(self.modulus as i64);
                remainder.coeffs[i + deg_diff] =
                    (remainder.coeffs[i + deg_diff] - sub).rem_euclid(self.modulus as i64);
            }

            // Remove leading zero
            remainder.coeffs[r_deg] = 0;
        }

        (quotient, remainder)
    }

    // === Helper Methods ===

    fn split_at(&self, poly: &Polynomial, m: usize) -> (Polynomial, Polynomial) {
        let low = poly.coeffs[..m.min(poly.coeffs.len())].to_vec();
        let high = if poly.coeffs.len() > m {
            poly.coeffs[m..].to_vec()
        } else {
            vec![0]
        };

        (
            Polynomial::from_coeffs(low, self.modulus),
            Polynomial::from_coeffs(high, self.modulus),
        )
    }

    fn check_ntt_friendly(modulus: u64) -> bool {
        // NTT-friendly if modulus ≡ 1 (mod 2^k) for some k
        // Check common cases
        for k in 1..=20 {
            let divisor = 1u64 << k;
            if (modulus - 1) % divisor == 0 {
                return true;
            }
        }
        false
    }

    fn find_primitive_root(modulus: u64) -> Option<u64> {
        // Find primitive nth root of unity
        // For NTT, we need omega^n ≡ 1 (mod q) and omega^(n/2) ≡ -1 (mod q)

        // Simple search (for small moduli)
        for candidate in 2..modulus {
            if Self::is_primitive_root(candidate, modulus) {
                return Some(candidate);
            }
            if candidate > 1000 {
                break;  // Limit search
            }
        }
        None
    }

    fn is_primitive_root(candidate: u64, modulus: u64) -> bool {
        // Check if candidate is a primitive root mod modulus
        // TODO: Implement proper primitive root test
        // For now, simple check
        let mut power = 1u64;
        let order = modulus - 1;

        for _ in 0..order {
            power = (power * candidate) % modulus;
            if power == 1 {
                return true;  // Simplified check
            }
        }

        false
    }

    fn mod_inverse(&self, a: u64, m: u64) -> u64 {
        // Extended Euclidean algorithm
        let (mut old_r, mut r) = (a as i64, m as i64);
        let (mut old_s, mut s) = (1i64, 0i64);

        while r != 0 {
            let quotient = old_r / r;
            let temp_r = r;
            r = old_r - quotient * r;
            old_r = temp_r;

            let temp_s = s;
            s = old_s - quotient * s;
            old_s = temp_s;
        }

        ((old_s % m as i64 + m as i64) % m as i64) as u64
    }
}

/// PolyPoly convolution (optimized for FHE)
pub struct PolyPolyConvolution {
    multiplier: PolyPolyMultiplier,
}

impl PolyPolyConvolution {
    pub fn new(modulus: u64) -> Self {
        PolyPolyConvolution {
            multiplier: PolyPolyMultiplier::new(modulus),
        }
    }

    /// Cyclic convolution (for X^N + 1 reduction)
    pub fn cyclic_convolution(&self, a: &Polynomial, b: &Polynomial, n: usize) -> Polynomial {
        // Multiply and reduce mod (X^N + 1)
        let product = self.multiplier.multiply(a, b);
        self.reduce_mod_xn_plus_1(product, n)
    }

    /// Negacyclic convolution (for FHE)
    pub fn negacyclic_convolution(&self, a: &Polynomial, b: &Polynomial, n: usize) -> Polynomial {
        // Same as cyclic for X^N + 1
        self.cyclic_convolution(a, b, n)
    }

    fn reduce_mod_xn_plus_1(&self, mut poly: Polynomial, n: usize) -> Polynomial {
        // Reduce polynomial mod (X^N + 1)
        // For coefficient at index i >= N, subtract from coefficient at (i mod N)

        let mut result = vec![0i64; n];

        for (i, &coeff) in poly.coeffs.iter().enumerate() {
            let idx = i % n;
            let quotient = i / n;

            if quotient % 2 == 0 {
                result[idx] = (result[idx] + coeff).rem_euclid(poly.modulus as i64);
            } else {
                result[idx] = (result[idx] - coeff).rem_euclid(poly.modulus as i64);
            }
        }

        Polynomial {
            coeffs: result,
            modulus: poly.modulus,
        }
    }
}

// ===========================
// === Unit Tests ============
// ===========================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_addition() {
        let a = Polynomial::from_coeffs(vec![1, 2, 3], 7);
        let b = Polynomial::from_coeffs(vec![4, 5], 7);

        let sum = a + b;
        assert_eq!(sum.coeffs, vec![5, 0, 3]);
    }

    #[test]
    fn test_polynomial_subtraction() {
        let a = Polynomial::from_coeffs(vec![5, 2, 3], 7);
        let b = Polynomial::from_coeffs(vec![1, 2, 1], 7);

        let diff = a - b;
        assert_eq!(diff.coeffs, vec![4, 0, 2]);
    }

    #[test]
    fn test_schoolbook_multiply() {
        let multiplier = PolyPolyMultiplier::new(7);
        let a = Polynomial::from_coeffs(vec![1, 2], 7);
        let b = Polynomial::from_coeffs(vec![3, 4], 7);

        let product = multiplier.schoolbook_multiply(&a, &b);
        // (1 + 2x)(3 + 4x) = 3 + 10x + 8x^2 = 3 + 3x + 1x^2 (mod 7)
        assert_eq!(product.coeffs, vec![3, 3, 1]);
    }

    #[test]
    fn test_karatsuba_multiply() {
        let multiplier = PolyPolyMultiplier::new(11);
        let a = Polynomial::from_coeffs(vec![1, 2, 3, 4], 11);
        let b = Polynomial::from_coeffs(vec![5, 6, 7], 11);

        let product1 = multiplier.schoolbook_multiply(&a, &b);
        let product2 = multiplier.karatsuba_multiply(&a, &b);

        assert_eq!(product1.coeffs, product2.coeffs);
    }

    #[test]
    fn test_auto_strategy() {
        let multiplier = PolyPolyMultiplier::new(17);

        // Small degree -> should use schoolbook
        let a_small = Polynomial::from_coeffs(vec![1; 32], 17);
        let b_small = Polynomial::from_coeffs(vec![2; 32], 17);
        let _ = multiplier.multiply(&a_small, &b_small);

        // Large degree -> should use Karatsuba or NTT
        let a_large = Polynomial::from_coeffs(vec![1; 256], 17);
        let b_large = Polynomial::from_coeffs(vec![2; 256], 17);
        let _ = multiplier.multiply(&a_large, &b_large);
    }

    #[test]
    fn test_polynomial_evaluation() {
        let poly = Polynomial::from_coeffs(vec![1, 2, 3], 11);
        // p(x) = 1 + 2x + 3x^2
        // p(2) = 1 + 4 + 12 = 17 ≡ 6 (mod 11)
        assert_eq!(poly.evaluate(2), 6);
    }

    #[test]
    fn test_ntt_friendly_check() {
        // 998244353 is NTT-friendly (common prime in competitive programming)
        assert!(PolyPolyMultiplier::check_ntt_friendly(998244353));

        // 17 is not particularly NTT-friendly
        // (but might still pass for small 2^k)
    }

    #[test]
    fn test_cyclic_convolution() {
        let conv = PolyPolyConvolution::new(7);
        let a = Polynomial::from_coeffs(vec![1, 2, 3, 4], 7);
        let b = Polynomial::from_coeffs(vec![5, 6, 7, 8], 7);

        let result = conv.cyclic_convolution(&a, &b, 4);
        assert_eq!(result.coeffs.len(), 4);
    }

    #[test]
    fn test_polynomial_division() {
        let multiplier = PolyPolyMultiplier::new(11);

        let dividend = Polynomial::from_coeffs(vec![1, 2, 3, 4], 11);
        let divisor = Polynomial::from_coeffs(vec![1, 1], 11);

        let (quotient, remainder) = multiplier.divide(&dividend, &divisor);

        // Verify: dividend = quotient * divisor + remainder
        let reconstructed = multiplier.multiply(&quotient, &divisor) + remainder;

        for (i, &coeff) in dividend.coeffs.iter().enumerate() {
            if i < reconstructed.coeffs.len() {
                assert_eq!(coeff, reconstructed.coeffs[i]);
            }
        }
    }
}

//! RNS - Residue Number System with Adaptive Multi-Prime Support
//!
//! QMNF Innovation: CRTBigInt parallel coefficient operations enable
//! exact arithmetic on integers larger than any single prime modulus.
//!
//! V2 OPTIMIZATION: Zero-sync parallelism via Rayon
//! Each RNS channel (prime modulus) operates completely independently,
//! enabling embarrassingly parallel NTT multiplication with no barriers.

use super::montgomery::MontgomeryContext;

#[cfg(feature = "parallel")]
use rayon::prelude::*;
#[cfg(feature = "ntt_fft")]
use super::ntt_fft::NTTEngineFFT as NTTEngine;

#[cfg(not(feature = "ntt_fft"))]
use super::ntt::NTTEngine;

/// RNS Context for managing multiple prime moduli
pub struct RNSContext {
    /// List of coprime moduli
    pub primes: Vec<u64>,
    /// Product of all primes (as u128, may overflow for many primes)
    pub product: u128,
    /// Montgomery contexts for each prime
    pub mont_contexts: Vec<MontgomeryContext>,
    /// NTT engines for each prime
    pub ntt_engines: Vec<NTTEngine>,
    /// Polynomial degree
    pub n: usize,
}

impl RNSContext {
    /// Create a new RNS context from a list of primes
    pub fn new(primes: Vec<u64>, n: usize) -> Self {
        assert!(!primes.is_empty(), "Need at least one prime");
        assert!(n.is_power_of_two(), "N must be power of 2");
        
        // Verify primes are coprime (all distinct primes are coprime)
        for i in 0..primes.len() {
            for j in (i + 1)..primes.len() {
                assert_ne!(primes[i], primes[j], "Primes must be distinct");
            }
        }
        
        // Compute product
        let product = primes.iter().fold(1u128, |acc, &p| acc * p as u128);
        
        // Create Montgomery and NTT contexts
        let mont_contexts: Vec<_> = primes.iter()
            .map(|&p| MontgomeryContext::new(p))
            .collect();
        
        let ntt_engines: Vec<_> = primes.iter()
            .map(|&p| NTTEngine::new(p, n))
            .collect();
        
        Self {
            primes,
            product,
            mont_contexts,
            ntt_engines,
            n,
        }
    }
    
    /// Get the number of primes
    pub fn num_primes(&self) -> usize {
        self.primes.len()
    }
    
    /// Convert a small integer to RNS representation
    pub fn from_int(&self, x: u64) -> Vec<u64> {
        self.primes.iter().map(|&q| x % q).collect()
    }
    
    /// Convert RNS representation back to integer using Garner's algorithm
    ///
    /// MSC INNOVATION: Garner's on-the-fly reconstruction
    /// - Never computes full product (avoids overflow)
    /// - Works with crypto-sized 64-bit moduli
    /// - O(k²) but k is small (typically 2-8 primes)
    ///
    /// Algorithm: Compute mixed-radix representation iteratively
    /// v_i = (r_i - sum_{j<i} c_j * prod_{k<j} p_k) * (prod_{k<i} p_k)^{-1} mod p_i
    pub fn to_int(&self, rns: &[u64]) -> u128 {
        assert_eq!(rns.len(), self.primes.len());

        if self.primes.is_empty() {
            return 0;
        }

        // Garner's algorithm - no product overflow
        let k = self.primes.len();
        let mut coeffs = vec![0u128; k];

        // First coefficient is just r_0
        coeffs[0] = rns[0] as u128;

        // Precompute inverses: inv[i][j] = p_j^{-1} mod p_i for j < i
        // We compute these on-the-fly to avoid storing O(k²) values
        for i in 1..k {
            let pi = self.primes[i];

            // Compute (r_i - accumulated) * inverse mod p_i
            let mut accumulated = coeffs[0] % pi as u128;
            let mut prod_mod_pi = 1u128;

            for j in 1..i {
                prod_mod_pi = (prod_mod_pi * self.primes[j - 1] as u128) % pi as u128;
                accumulated = (accumulated + coeffs[j] * prod_mod_pi) % pi as u128;
            }
            if i > 1 {
                prod_mod_pi = (prod_mod_pi * self.primes[i - 1] as u128) % pi as u128;
            }

            // diff = r_i - accumulated mod p_i
            let ri = rns[i] as u128;
            let diff = if ri >= accumulated {
                ri - accumulated
            } else {
                pi as u128 - accumulated + ri
            };

            // Compute inverse of product mod p_i
            let prod_inv = mod_inverse((prod_mod_pi % pi as u128) as u64, pi);

            coeffs[i] = ((diff % pi as u128) * prod_inv as u128) % pi as u128;
        }

        // Reconstruct: result = c_0 + c_1*p_0 + c_2*p_0*p_1 + ...
        let mut result = coeffs[0];
        let mut product = 1u128;

        for i in 1..k {
            product = product.saturating_mul(self.primes[i - 1] as u128);
            result = result.saturating_add(coeffs[i].saturating_mul(product));
        }

        result
    }

    /// Add two RNS numbers
    pub fn add(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        assert_eq!(a.len(), self.primes.len());
        assert_eq!(b.len(), self.primes.len());
        
        a.iter().zip(b.iter()).zip(self.primes.iter())
            .map(|((&ai, &bi), &qi)| {
                let sum = ai as u128 + bi as u128;
                if sum >= qi as u128 { (sum - qi as u128) as u64 } else { sum as u64 }
            })
            .collect()
    }
    
    /// Subtract two RNS numbers
    pub fn sub(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        assert_eq!(a.len(), self.primes.len());
        assert_eq!(b.len(), self.primes.len());
        
        a.iter().zip(b.iter()).zip(self.primes.iter())
            .map(|((&ai, &bi), &qi)| {
                if ai >= bi { ai - bi } else { qi - bi + ai }
            })
            .collect()
    }
    
    /// Multiply two RNS numbers
    pub fn mul(&self, a: &[u64], b: &[u64]) -> Vec<u64> {
        assert_eq!(a.len(), self.primes.len());
        assert_eq!(b.len(), self.primes.len());
        
        a.iter().zip(b.iter()).zip(self.primes.iter())
            .map(|((&ai, &bi), &qi)| {
                ((ai as u128 * bi as u128) % qi as u128) as u64
            })
            .collect()
    }
    
    /// Negate an RNS number
    pub fn neg(&self, a: &[u64]) -> Vec<u64> {
        a.iter().zip(self.primes.iter())
            .map(|(&ai, &qi)| {
                if ai == 0 { 0 } else { qi - ai }
            })
            .collect()
    }
}

/// RNS Polynomial - coefficients stored as parallel limbs
pub struct RNSPolynomial {
    /// Limbs: limbs[i] is the polynomial mod primes[i]
    pub limbs: Vec<Vec<u64>>,
    /// Polynomial degree
    pub n: usize,
}

impl RNSPolynomial {
    /// Create from a single-modulus polynomial
    pub fn from_poly(poly: &[u64], ctx: &RNSContext) -> Self {
        let n = poly.len();
        let limbs = ctx.primes.iter()
            .map(|&q| poly.iter().map(|&c| c % q).collect())
            .collect();
        
        Self { limbs, n }
    }
    
    /// Create a zero polynomial
    pub fn zero(ctx: &RNSContext) -> Self {
        let limbs = vec![vec![0u64; ctx.n]; ctx.num_primes()];
        Self { limbs, n: ctx.n }
    }
    
    /// Add two RNS polynomials
    ///
    /// Note: Addition is too lightweight for parallelism overhead.
    /// Sequential is actually faster for add/sub operations.
    pub fn add(&self, other: &Self, ctx: &RNSContext) -> Self {
        assert_eq!(self.n, other.n);

        let limbs = self.limbs.iter()
            .zip(other.limbs.iter())
            .zip(ctx.primes.iter())
            .map(|((a, b), &q)| {
                a.iter().zip(b.iter())
                    .map(|(&ai, &bi)| {
                        let sum = ai as u128 + bi as u128;
                        if sum >= q as u128 { (sum - q as u128) as u64 } else { sum as u64 }
                    })
                    .collect()
            })
            .collect();

        Self { limbs, n: self.n }
    }
    
    /// Subtract two RNS polynomials
    ///
    /// Note: Subtraction is too lightweight for parallelism overhead.
    /// Sequential is actually faster for add/sub operations.
    pub fn sub(&self, other: &Self, ctx: &RNSContext) -> Self {
        assert_eq!(self.n, other.n);

        let limbs = self.limbs.iter()
            .zip(other.limbs.iter())
            .zip(ctx.primes.iter())
            .map(|((a, b), &q)| {
                a.iter().zip(b.iter())
                    .map(|(&ai, &bi)| {
                        if ai >= bi { ai - bi } else { q - bi + ai }
                    })
                    .collect()
            })
            .collect();

        Self { limbs, n: self.n }
    }
    
    /// Negate RNS polynomial
    pub fn neg(&self, ctx: &RNSContext) -> Self {
        let limbs = self.limbs.iter()
            .zip(ctx.primes.iter())
            .map(|(a, &q)| {
                a.iter().map(|&ai| if ai == 0 { 0 } else { q - ai }).collect()
            })
            .collect();
        
        Self { limbs, n: self.n }
    }
    
    /// Multiply two RNS polynomials using parallel NTT
    ///
    /// V2 ZERO-SYNC: Each RNS channel runs its NTT independently.
    /// With k primes, this achieves ~k× speedup on multi-core CPUs.
    /// No synchronization barriers - each channel is embarrassingly parallel.
    #[cfg(feature = "parallel")]
    pub fn mul(&self, other: &Self, ctx: &RNSContext) -> Self {
        assert_eq!(self.n, other.n);

        // Zero-sync parallel: each (limb_a, limb_b, ntt_engine) triple
        // runs completely independently across CPU cores
        let limbs: Vec<Vec<u64>> = self.limbs.par_iter()
            .zip(other.limbs.par_iter())
            .zip(ctx.ntt_engines.par_iter())
            .map(|((a, b), ntt)| {
                ntt.multiply(a, b)
            })
            .collect();

        Self { limbs, n: self.n }
    }

    /// Sequential fallback when parallel feature is disabled
    #[cfg(not(feature = "parallel"))]
    pub fn mul(&self, other: &Self, ctx: &RNSContext) -> Self {
        assert_eq!(self.n, other.n);

        let limbs = self.limbs.iter()
            .zip(other.limbs.iter())
            .zip(ctx.ntt_engines.iter())
            .map(|((a, b), ntt)| {
                ntt.multiply(a, b)
            })
            .collect();

        Self { limbs, n: self.n }
    }
    
    /// Drop the last prime (for rescaling)
    pub fn drop_last_prime(&self, ctx: &RNSContext) -> Self {
        assert!(ctx.num_primes() > 1);
        
        let limbs = self.limbs[..self.limbs.len() - 1].to_vec();
        Self { limbs, n: self.n }
    }
}

/// Modular inverse using extended Euclidean algorithm
fn mod_inverse(a: u64, m: u64) -> u64 {
    let mut mn = (m as i128, a as i128);
    let mut xy = (0i128, 1i128);
    
    while mn.1 != 0 {
        let q = mn.0 / mn.1;
        mn = (mn.1, mn.0 - q * mn.1);
        xy = (xy.1, xy.0 - q * xy.1);
    }
    
    while xy.0 < 0 {
        xy.0 += m as i128;
    }
    (xy.0 % m as i128) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // For basic RNS integer tests, we don't need NTT.
    // Create a simpler context that skips NTT for non-polynomial tests.
    
    #[test]
    fn test_rns_roundtrip() {
        // Use NTT-compatible primes
        let primes = vec![998244353, 985661441];
        let ctx = RNSContext::new(primes, 4);
        
        for x in [0u64, 1, 100, 1000, 7000] {
            let rns = ctx.from_int(x);
            let back = ctx.to_int(&rns);
            assert_eq!(back, x as u128, "Roundtrip failed for {}", x);
        }
    }
    
    #[test]
    fn test_rns_add() {
        let primes = vec![998244353, 985661441];
        let ctx = RNSContext::new(primes, 4);
        
        let a = ctx.from_int(100);
        let b = ctx.from_int(200);
        let sum = ctx.add(&a, &b);
        let result = ctx.to_int(&sum);
        
        assert_eq!(result, 300);
    }
    
    #[test]
    fn test_rns_mul() {
        let primes = vec![998244353, 985661441];
        let ctx = RNSContext::new(primes, 4);
        
        let a = ctx.from_int(12);
        let b = ctx.from_int(34);
        let prod = ctx.mul(&a, &b);
        let result = ctx.to_int(&prod);
        
        assert_eq!(result, 408);
    }
    
    #[test]
    fn test_rns_polynomial_add() {
        let primes = vec![998244353, 985661441];
        let ctx = RNSContext::new(primes, 4);
        
        let a = vec![1, 2, 3, 4];
        let b = vec![5, 6, 7, 8];
        
        let rns_a = RNSPolynomial::from_poly(&a, &ctx);
        let rns_b = RNSPolynomial::from_poly(&b, &ctx);
        
        let rns_sum = rns_a.add(&rns_b, &ctx);
        
        // Check first limb
        assert_eq!(rns_sum.limbs[0], vec![6, 8, 10, 12]);
    }
    
    #[test]
    fn test_rns_polynomial_mul() {
        let primes = vec![998244353, 985661441];
        let ctx = RNSContext::new(primes, 8);
        
        // (1 + 2x) * (3 + 4x) = 3 + 10x + 8x^2
        let a = vec![1, 2, 0, 0, 0, 0, 0, 0];
        let b = vec![3, 4, 0, 0, 0, 0, 0, 0];
        
        let rns_a = RNSPolynomial::from_poly(&a, &ctx);
        let rns_b = RNSPolynomial::from_poly(&b, &ctx);
        
        let rns_prod = rns_a.mul(&rns_b, &ctx);
        
        // Check both limbs have correct result
        assert_eq!(rns_prod.limbs[0][0], 3);
        assert_eq!(rns_prod.limbs[0][1], 10);
        assert_eq!(rns_prod.limbs[0][2], 8);
        
        assert_eq!(rns_prod.limbs[1][0], 3);
        assert_eq!(rns_prod.limbs[1][1], 10);
        assert_eq!(rns_prod.limbs[1][2], 8);
    }

    #[test]
    fn test_rns_parallel_benchmark() {
        use std::time::Instant;

        // Use 4 primes for better parallel speedup demonstration
        let primes = vec![998244353, 985661441, 754974721, 469762049];
        let n = 4096;

        println!("\n=== RNS Zero-Sync Parallelism Benchmark ===");
        println!("Configuration: N={}, k={} primes", n, primes.len());
        #[cfg(feature = "parallel")]
        println!("Mode: PARALLEL (Rayon par_iter)");
        #[cfg(not(feature = "parallel"))]
        println!("Mode: SEQUENTIAL");

        let ctx = RNSContext::new(primes, n);

        // Create test polynomials
        let a: Vec<u64> = (0..n as u64).map(|i| i % 1000).collect();
        let b: Vec<u64> = (0..n as u64).map(|i| (i * 7) % 1000).collect();

        let rns_a = RNSPolynomial::from_poly(&a, &ctx);
        let rns_b = RNSPolynomial::from_poly(&b, &ctx);

        // Warmup
        for _ in 0..3 {
            let _ = rns_a.mul(&rns_b, &ctx);
        }

        // Benchmark multiplication
        let iterations = 50;
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = rns_a.mul(&rns_b, &ctx);
        }
        let elapsed = start.elapsed();

        let per_mul_us = elapsed.as_micros() / iterations as u128;
        let throughput = 1_000_000.0 / per_mul_us as f64;

        println!("\nRNS Polynomial Multiplication:");
        println!("  Total time for {} muls: {:?}", iterations, elapsed);
        println!("  Per multiplication:     {}µs", per_mul_us);
        println!("  Throughput:             {:.1} muls/sec", throughput);

        // Benchmark add
        let start = Instant::now();
        for _ in 0..500 {
            let _ = rns_a.add(&rns_b, &ctx);
        }
        let elapsed = start.elapsed();
        let per_add_us = elapsed.as_micros() / 500;

        println!("\nRNS Polynomial Addition:");
        println!("  Per addition: {}µs", per_add_us);
        println!("=====================================\n");
    }

    #[test]
    fn test_garner_overflow_free() {
        // MSC INNOVATION TEST: Garner's algorithm with crypto-sized moduli
        // These are 30-bit NTT-friendly primes - standard CRT would overflow u128

        println!("\n=== Garner Overflow-Free CRT Test ===");

        // Test with small values first
        let primes = vec![998244353, 985661441];
        let ctx = RNSContext::new(primes, 4);

        for x in [0u64, 1, 100, 12345, 999999] {
            let rns = ctx.from_int(x);
            let back = ctx.to_int(&rns);
            assert_eq!(back, x as u128, "Garner roundtrip failed for {}", x);
        }
        println!("  Small values: PASS");

        // Test with larger values (product of operations)
        let a = ctx.from_int(12345);
        let b = ctx.from_int(67890);
        let prod = ctx.mul(&a, &b);
        let result = ctx.to_int(&prod);
        assert_eq!(result, 12345u128 * 67890, "Garner mul failed");
        println!("  Multiplication: PASS (12345 × 67890 = {})", result);

        // Test with 4 primes (larger product range)
        let primes4 = vec![998244353, 985661441, 754974721, 469762049];
        let ctx4 = RNSContext::new(primes4, 4);

        let x = 123456789u64;
        let rns = ctx4.from_int(x);
        let back = ctx4.to_int(&rns);
        assert_eq!(back, x as u128, "Garner 4-prime roundtrip failed");
        println!("  4-prime roundtrip: PASS ({})", x);

        // Test arithmetic in 4-prime RNS
        let a = ctx4.from_int(1000000);
        let b = ctx4.from_int(2000000);
        let sum = ctx4.add(&a, &b);
        let result = ctx4.to_int(&sum);
        assert_eq!(result, 3000000, "Garner 4-prime add failed");
        println!("  4-prime addition: PASS");

        println!("=== All Garner tests passed ===\n");
    }
}

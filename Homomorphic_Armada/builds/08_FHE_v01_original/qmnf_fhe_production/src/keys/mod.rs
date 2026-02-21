//! Keys Module - BFV Key Generation
//!
//! Generates secret, public, and evaluation keys for BFV FHE.
//! Uses Shadow Entropy for deterministic, reproducible key generation.

use crate::arithmetic::NTTEngine;
use crate::entropy::ShadowHarvester;
use crate::params::FHEConfig;
use crate::ring::RingPolynomial;

/// Secret Key: ternary polynomial s ∈ R_q
#[derive(Clone)]
pub struct SecretKey {
    /// The secret polynomial
    pub s: RingPolynomial,
}

impl SecretKey {
    /// Generate a new secret key
    pub fn generate(config: &FHEConfig, harvester: &mut ShadowHarvester) -> Self {
        let s = RingPolynomial::random_ternary(config.n, config.q, harvester);
        Self { s }
    }
}

/// Public Key: (pk0, pk1) where pk0 = -a*s + e, pk1 = a
#[derive(Clone)]
pub struct PublicKey {
    /// pk0 = -a*s + e
    pub pk0: RingPolynomial,
    /// pk1 = a (random)
    pub pk1: RingPolynomial,
}

impl PublicKey {
    /// Generate public key from secret key
    pub fn generate(
        sk: &SecretKey,
        config: &FHEConfig,
        ntt: &NTTEngine,
        harvester: &mut ShadowHarvester,
    ) -> Self {
        // a ← uniform random
        let a = RingPolynomial::random_uniform(config.n, config.q, harvester);
        
        // e ← error distribution (CBD)
        let e = RingPolynomial::random_cbd(config.n, config.q, config.eta, harvester);
        
        // pk0 = -a*s + e = -(a*s) + e
        let as_prod = a.mul(&sk.s, ntt);
        let neg_as = as_prod.neg(ntt);
        let pk0 = neg_as.add(&e, ntt);
        
        Self { pk0, pk1: a }
    }
}

/// Evaluation Key for relinearization after multiplication
#[derive(Clone)]
pub struct EvaluationKey {
    /// Relinearization key components
    /// rlk[i] = (b_i, a_i) where b_i = -a_i * s + e_i + (s^2 * T^i) for decomposition base T
    pub rlk: Vec<(RingPolynomial, RingPolynomial)>,
    /// Decomposition base (typically a power of 2)
    pub decomp_base: u64,
    /// Number of decomposition levels
    pub levels: usize,
}

impl EvaluationKey {
    /// Generate evaluation key for relinearization
    pub fn generate(
        sk: &SecretKey,
        config: &FHEConfig,
        ntt: &NTTEngine,
        harvester: &mut ShadowHarvester,
    ) -> Self {
        // Decomposition base T and number of levels
        let decomp_base = 1u64 << 16;  // T = 2^16
        let levels = (64 - config.q.leading_zeros() as usize + 15) / 16;
        
        // s^2
        let s_squared = sk.s.mul(&sk.s, ntt);
        
        let mut rlk = Vec::with_capacity(levels);
        let mut power_of_t = 1u64;
        
        for _ in 0..levels {
            // a_i ← uniform random
            let a_i = RingPolynomial::random_uniform(config.n, config.q, harvester);
            
            // e_i ← error distribution
            let e_i = RingPolynomial::random_cbd(config.n, config.q, config.eta, harvester);
            
            // s^2 * T^i
            let s2_ti = s_squared.scalar_mul(power_of_t, ntt);
            
            // b_i = -a_i * s + e_i + s^2 * T^i
            let as_prod = a_i.mul(&sk.s, ntt);
            let neg_as = as_prod.neg(ntt);
            let b_i = neg_as.add(&e_i, ntt).add(&s2_ti, ntt);
            
            rlk.push((b_i, a_i));
            power_of_t = ((power_of_t as u128 * decomp_base as u128) % config.q as u128) as u64;
        }
        
        Self { rlk, decomp_base, levels }
    }
}

/// Complete key set
pub struct KeySet {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub eval_key: EvaluationKey,
}

impl KeySet {
    /// Generate complete key set
    pub fn generate(config: &FHEConfig, ntt: &NTTEngine, harvester: &mut ShadowHarvester) -> Self {
        let secret_key = SecretKey::generate(config, harvester);
        let public_key = PublicKey::generate(&secret_key, config, ntt, harvester);
        let eval_key = EvaluationKey::generate(&secret_key, config, ntt, harvester);
        
        Self { secret_key, public_key, eval_key }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keygen_basic() {
        let config = FHEConfig::light();
        let ntt = NTTEngine::new(config.q, config.n);
        let mut harvester = ShadowHarvester::with_seed(42);
        
        let sk = SecretKey::generate(&config, &mut harvester);
        
        // Secret key should be ternary
        for i in 0..config.n {
            let coeff = sk.s.get_signed(i);
            assert!(coeff >= -1 && coeff <= 1, "Non-ternary coefficient: {}", coeff);
        }
    }
    
    #[test]
    fn test_keygen_deterministic() {
        let config = FHEConfig::light();
        let ntt = NTTEngine::new(config.q, config.n);
        
        let mut h1 = ShadowHarvester::with_seed(12345);
        let mut h2 = ShadowHarvester::with_seed(12345);
        
        let keys1 = KeySet::generate(&config, &ntt, &mut h1);
        let keys2 = KeySet::generate(&config, &ntt, &mut h2);
        
        // Same seed should produce same keys
        assert_eq!(keys1.secret_key.s.coeffs, keys2.secret_key.s.coeffs);
        assert_eq!(keys1.public_key.pk0.coeffs, keys2.public_key.pk0.coeffs);
        assert_eq!(keys1.public_key.pk1.coeffs, keys2.public_key.pk1.coeffs);
    }
    
    #[test]
    fn test_keygen_benchmark() {
        let config = FHEConfig::light();
        let ntt = NTTEngine::new(config.q, config.n);
        let mut harvester = ShadowHarvester::with_seed(999);
        
        let start = std::time::Instant::now();
        for _ in 0..100 {
            let _ = KeySet::generate(&config, &ntt, &mut harvester);
        }
        let elapsed = start.elapsed();
        
        println!("KeyGen x100: {:?}", elapsed);
    }
}

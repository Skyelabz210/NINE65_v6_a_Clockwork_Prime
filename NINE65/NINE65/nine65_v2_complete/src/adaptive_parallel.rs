//! Adaptive Parallel FHE Operations - Optimized Threading Implementation
//!
//! This module provides intelligent parallel processing capabilities for FHE operations
//! that adaptively choose the optimal number of threads based on operation type and dataset size.

use std::sync::Arc;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use crate::ops::encrypt::{Ciphertext, BFVEncoder, BFVEncryptor, BFVDecryptor};
use crate::ops::homomorphic::BFVEvaluator;
use crate::params::FHEConfig;
use crate::keys::KeySet;
use crate::entropy::ShadowHarvester;

use rayon::ThreadPool;

/// Adaptive Parallel FHE Context for optimized multi-threaded operations
pub struct AdaptiveParallelFHEContext {
    /// Shared configuration
    pub config: Arc<FHEConfig>,
    /// Shared keys (wrapped in Arc for thread safety)
    pub keys: Arc<KeySet>,
    /// Thread pool with fixed 4 threads
    thread_pool: Arc<ThreadPool>,
}

impl AdaptiveParallelFHEContext {
    /// Create a new adaptive parallel context
    pub fn new(config: FHEConfig, keys: KeySet) -> Self {
        let thread_pool = ThreadPoolBuilder::new()
            .num_threads(4)  // Fixed 4 threads
            .build()
            .expect("Failed to create thread pool with 4 threads");
        
        Self {
            config: Arc::new(config),
            keys: Arc::new(keys),
            thread_pool: Arc::new(thread_pool),
        }
    }

    /// Adaptive encryption of multiple messages based on dataset size
    pub fn adaptive_encrypt(&self, messages: &[u64], seed: u64) -> Vec<Ciphertext> {
        // For small datasets, use sequential processing to avoid threading overhead
        if messages.len() < 5 {
            self.sequential_encrypt(messages, seed)
        } else {
            self.parallel_encrypt(messages, seed)
        }
    }

    /// Sequential encryption for small datasets
    fn sequential_encrypt(&self, messages: &[u64], seed: u64) -> Vec<Ciphertext> {
        // Create NTT engine based on feature flags
        #[cfg(feature = "ntt_fft")]
        let ntt = crate::arithmetic::NTTEngineFFT::new(self.config.q, self.config.n);
        
        #[cfg(not(feature = "ntt_fft"))]
        let ntt = crate::arithmetic::NTTEngine::new(self.config.q, self.config.n);
        
        let encoder = BFVEncoder::new(&self.config);
        let encryptor = BFVEncryptor::new(
            &self.keys.public_key,
            &encoder,
            &ntt,
            self.config.eta,
        );
        
        messages
            .iter()
            .enumerate()
            .map(|(i, &msg)| {
                let mut harvester = ShadowHarvester::with_seed(seed.wrapping_add(i as u64));
                encryptor.encrypt(msg, &mut harvester)
            })
            .collect()
    }

    /// Parallel encryption for larger datasets
    fn parallel_encrypt(&self, messages: &[u64], seed: u64) -> Vec<Ciphertext> {
        self.thread_pool.install(|| {
            messages
                .par_iter()
                .enumerate()
                .map(|(i, &msg)| {
                    // Create NTT engine for this thread
                    #[cfg(feature = "ntt_fft")]
                    let ntt = crate::arithmetic::NTTEngineFFT::new(self.config.q, self.config.n);
                    
                    #[cfg(not(feature = "ntt_fft"))]
                    let ntt = crate::arithmetic::NTTEngine::new(self.config.q, self.config.n);
                    
                    let encoder = BFVEncoder::new(&self.config);
                    let encryptor = BFVEncryptor::new(
                        &self.keys.public_key,
                        &encoder,
                        &ntt,
                        self.config.eta,
                    );
                    
                    let mut harvester = ShadowHarvester::with_seed(seed.wrapping_add(i as u64));
                    encryptor.encrypt(msg, &mut harvester)
                })
                .collect()
        })
    }

    /// Adaptive decryption of multiple ciphertexts based on dataset size
    pub fn adaptive_decrypt(&self, ciphertexts: &[Ciphertext]) -> Vec<u64> {
        // For decryption, sequential is generally faster based on benchmarks
        self.sequential_decrypt(ciphertexts)
    }

    /// Sequential decryption (based on benchmark results showing it's faster)
    fn sequential_decrypt(&self, ciphertexts: &[Ciphertext]) -> Vec<u64> {
        // Create NTT engine based on feature flags
        #[cfg(feature = "ntt_fft")]
        let ntt = crate::arithmetic::NTTEngineFFT::new(self.config.q, self.config.n);
        
        #[cfg(not(feature = "ntt_fft"))]
        let ntt = crate::arithmetic::NTTEngine::new(self.config.q, self.config.n);
        
        let encoder = BFVEncoder::new(&self.config);
        let decryptor = BFVDecryptor::new(
            &self.keys.secret_key,
            &encoder,
            &ntt,
        );
        
        ciphertexts
            .iter()
            .map(|ct| decryptor.decrypt(ct))
            .collect()
    }

    /// Parallel decryption (available but generally not recommended based on benchmarks)
    pub fn parallel_decrypt(&self, ciphertexts: &[Ciphertext]) -> Vec<u64> {
        self.thread_pool.install(|| {
            ciphertexts
                .par_iter()
                .map(|ct| {
                    // Create NTT engine for this thread
                    #[cfg(feature = "ntt_fft")]
                    let ntt = crate::arithmetic::NTTEngineFFT::new(self.config.q, self.config.n);
                    
                    #[cfg(not(feature = "ntt_fft"))]
                    let ntt = crate::arithmetic::NTTEngine::new(self.config.q, self.config.n);
                    
                    let encoder = BFVEncoder::new(&self.config);
                    let decryptor = BFVDecryptor::new(
                        &self.keys.secret_key,
                        &encoder,
                        &ntt,
                    );
                    
                    decryptor.decrypt(ct)
                })
                .collect()
        })
    }

    /// Adaptive homomorphic addition based on dataset size
    pub fn adaptive_add(&self, ct1_list: &[Ciphertext], ct2_list: &[Ciphertext]) -> Vec<Ciphertext> {
        if ct1_list.len() < 10 {
            // For small datasets, use sequential processing
            self.sequential_add(ct1_list, ct2_list)
        } else {
            // For larger datasets, use parallel processing
            self.parallel_add(ct1_list, ct2_list)
        }
    }

    /// Sequential homomorphic addition
    fn sequential_add(&self, ct1_list: &[Ciphertext], ct2_list: &[Ciphertext]) -> Vec<Ciphertext> {
        // Create NTT engine based on feature flags
        #[cfg(feature = "ntt_fft")]
        let ntt = crate::arithmetic::NTTEngineFFT::new(self.config.q, self.config.n);
        
        #[cfg(not(feature = "ntt_fft"))]
        let ntt = crate::arithmetic::NTTEngine::new(self.config.q, self.config.n);
        
        let encoder = BFVEncoder::new(&self.config);
        let evaluator = BFVEvaluator::new(
            &ntt,
            &encoder,
            Some(&self.keys.eval_key),
        );
        
        ct1_list
            .iter()
            .zip(ct2_list.iter())
            .map(|(ct1, ct2)| evaluator.add(ct1, ct2))
            .collect()
    }

    /// Parallel homomorphic addition
    fn parallel_add(&self, ct1_list: &[Ciphertext], ct2_list: &[Ciphertext]) -> Vec<Ciphertext> {
        self.thread_pool.install(|| {
            ct1_list
                .par_iter()
                .zip(ct2_list.par_iter())
                .map(|(ct1, ct2)| {
                    // Create NTT engine for this thread
                    #[cfg(feature = "ntt_fft")]
                    let ntt = crate::arithmetic::NTTEngineFFT::new(self.config.q, self.config.n);
                    
                    #[cfg(not(feature = "ntt_fft"))]
                    let ntt = crate::arithmetic::NTTEngine::new(self.config.q, self.config.n);
                    
                    let encoder = BFVEncoder::new(&self.config);
                    let evaluator = BFVEvaluator::new(
                        &ntt,
                        &encoder,
                        Some(&self.keys.eval_key),
                    );
                    
                    evaluator.add(ct1, ct2)
                })
                .collect()
        })
    }

    /// Adaptive homomorphic multiplication based on dataset size
    pub fn adaptive_mul(&self, ct1_list: &[Ciphertext], ct2_list: &[Ciphertext]) -> Vec<Ciphertext> {
        if ct1_list.len() < 10 {
            // For small datasets, use sequential processing
            self.sequential_mul(ct1_list, ct2_list)
        } else {
            // For larger datasets, use parallel processing
            self.parallel_mul(ct1_list, ct2_list)
        }
    }

    /// Sequential homomorphic multiplication
    fn sequential_mul(&self, ct1_list: &[Ciphertext], ct2_list: &[Ciphertext]) -> Vec<Ciphertext> {
        // Create NTT engine based on feature flags
        #[cfg(feature = "ntt_fft")]
        let ntt = crate::arithmetic::NTTEngineFFT::new(self.config.q, self.config.n);
        
        #[cfg(not(feature = "ntt_fft"))]
        let ntt = crate::arithmetic::NTTEngine::new(self.config.q, self.config.n);
        
        let encoder = BFVEncoder::new(&self.config);
        let evaluator = BFVEvaluator::new(
            &ntt,
            &encoder,
            Some(&self.keys.eval_key),
        );
        
        ct1_list
            .iter()
            .zip(ct2_list.iter())
            .map(|(ct1, ct2)| evaluator.mul(ct1, ct2))
            .collect()
    }

    /// Parallel homomorphic multiplication
    fn parallel_mul(&self, ct1_list: &[Ciphertext], ct2_list: &[Ciphertext]) -> Vec<Ciphertext> {
        self.thread_pool.install(|| {
            ct1_list
                .par_iter()
                .zip(ct2_list.par_iter())
                .map(|(ct1, ct2)| {
                    // Create NTT engine for this thread
                    #[cfg(feature = "ntt_fft")]
                    let ntt = crate::arithmetic::NTTEngineFFT::new(self.config.q, self.config.n);
                    
                    #[cfg(not(feature = "ntt_fft"))]
                    let ntt = crate::arithmetic::NTTEngine::new(self.config.q, self.config.n);
                    
                    let encoder = BFVEncoder::new(&self.config);
                    let evaluator = BFVEvaluator::new(
                        &ntt,
                        &encoder,
                        Some(&self.keys.eval_key),
                    );
                    
                    evaluator.mul(ct1, ct2)
                })
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::ShadowHarvester;

    #[test]
    fn test_adaptive_encrypt_decrypt() {
        use crate::keys::KeySet;
        
        // Create NTT engine based on feature flags
        let config = FHEConfig::light();
        
        #[cfg(feature = "ntt_fft")]
        let ntt = crate::arithmetic::NTTEngineFFT::new(config.q, config.n);
        
        #[cfg(not(feature = "ntt_fft"))]
        let ntt = crate::arithmetic::NTTEngine::new(config.q, config.n);
        
        let mut harvester = ShadowHarvester::with_seed(42);
        let keys = KeySet::generate(&config, &ntt, &mut harvester);

        let context = AdaptiveParallelFHEContext::new(config, keys);

        // Test data - small dataset (should use sequential internally)
        let messages: Vec<u64> = (0..3).map(|i| i * 10).collect();

        // Encrypt adaptively
        let ciphertexts = context.adaptive_encrypt(&messages, 123);

        // Decrypt adaptively
        let decrypted = context.adaptive_decrypt(&ciphertexts);

        // Verify correctness
        for (orig, &dec) in messages.iter().zip(decrypted.iter()) {
            assert_eq!(*orig % context.config.t, dec);
        }
    }

    #[test]
    fn test_adaptive_encrypt_decrypt_large() {
        use crate::keys::KeySet;
        
        // Create NTT engine based on feature flags
        let config = FHEConfig::light();
        
        #[cfg(feature = "ntt_fft")]
        let ntt = crate::arithmetic::NTTEngineFFT::new(config.q, config.n);
        
        #[cfg(not(feature = "ntt_fft"))]
        let ntt = crate::arithmetic::NTTEngine::new(config.q, config.n);
        
        let mut harvester = ShadowHarvester::with_seed(42);
        let keys = KeySet::generate(&config, &ntt, &mut harvester);

        let context = AdaptiveParallelFHEContext::new(config, keys);

        // Test data - large dataset (should use parallel internally)
        let messages: Vec<u64> = (0..50).map(|i| i * 10).collect();

        // Encrypt adaptively
        let ciphertexts = context.adaptive_encrypt(&messages, 123);

        // Decrypt adaptively
        let decrypted = context.adaptive_decrypt(&ciphertexts);

        // Verify correctness
        for (orig, &dec) in messages.iter().zip(decrypted.iter()) {
            assert_eq!(*orig % context.config.t, dec);
        }
    }
}
//! Parallel FHE Operations - Multi-threading Implementation
//!
//! This module provides parallel processing capabilities for FHE operations
//! to take advantage of multi-core processors.

use std::sync::Arc;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use crate::ops::encrypt::{Ciphertext, BFVEncoder, BFVEncryptor, BFVDecryptor};
use crate::ops::homomorphic::BFVEvaluator;
use crate::params::FHEConfig;
use crate::keys::KeySet;
use crate::entropy::ShadowHarvester;

// Define the NTT engine type based on features
#[cfg(feature = "ntt_fft")]
type NTTType = crate::arithmetic::NTTEngineFFT;

#[cfg(not(feature = "ntt_fft"))]
type NTTType = crate::arithmetic::NTTEngine;

use rayon::ThreadPool;

/// Parallel FHE Context for multi-threaded operations
pub struct ParallelFHEContext {
    /// Shared configuration
    pub config: Arc<FHEConfig>,
    /// Shared keys (wrapped in Arc for thread safety)
    pub keys: Arc<KeySet>,
    /// Thread pool with fixed 4 threads
    thread_pool: Arc<ThreadPool>,
}

impl ParallelFHEContext {
    /// Create a new parallel context
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

    /// Parallel encryption of multiple messages
    pub fn parallel_encrypt(&self, messages: &[u64], seed: u64) -> Vec<Ciphertext> {
        self.thread_pool.install(|| {
            // Create a separate harvester for each message to ensure thread safety
            messages
                .par_iter()
                .enumerate()
                .map(|(i, &msg)| {
                    // Create NTT engine for this thread
                    let ntt = self.create_ntt_engine();
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

    /// Parallel decryption of multiple ciphertexts
    pub fn parallel_decrypt(&self, ciphertexts: &[Ciphertext]) -> Vec<u64> {
        self.thread_pool.install(|| {
            ciphertexts
                .par_iter()
                .map(|ct| {
                    let ntt = self.create_ntt_engine();
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

    /// Parallel homomorphic addition of multiple ciphertext pairs
    pub fn parallel_add(&self, ct1_list: &[Ciphertext], ct2_list: &[Ciphertext]) -> Vec<Ciphertext> {
        assert_eq!(ct1_list.len(), ct2_list.len());
        
        self.thread_pool.install(|| {
            ct1_list
                .par_iter()
                .zip(ct2_list.par_iter())
                .map(|(ct1, ct2)| {
                    let ntt = self.create_ntt_engine();
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

    /// Parallel homomorphic multiplication of multiple ciphertext pairs
    pub fn parallel_mul(&self, ct1_list: &[Ciphertext], ct2_list: &[Ciphertext]) -> Vec<Ciphertext> {
        assert_eq!(ct1_list.len(), ct2_list.len());
        
        self.thread_pool.install(|| {
            ct1_list
                .par_iter()
                .zip(ct2_list.par_iter())
                .map(|(ct1, ct2)| {
                    let ntt = self.create_ntt_engine();
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

    /// Parallel plaintext addition to multiple ciphertexts
    pub fn parallel_add_plain(&self, ciphertexts: &[Ciphertext], values: &[u64]) -> Vec<Ciphertext> {
        assert_eq!(ciphertexts.len(), values.len());
        
        self.thread_pool.install(|| {
            ciphertexts
                .par_iter()
                .zip(values.par_iter())
                .map(|(ct, &val)| {
                    let ntt = self.create_ntt_engine();
                    let encoder = BFVEncoder::new(&self.config);
                    let evaluator = BFVEvaluator::new(
                        &ntt,
                        &encoder,
                        Some(&self.keys.eval_key),
                    );
                    
                    evaluator.add_plain(ct, val)
                })
                .collect()
        })
    }

    /// Parallel plaintext multiplication with multiple ciphertexts
    pub fn parallel_mul_plain(&self, ciphertexts: &[Ciphertext], values: &[u64]) -> Vec<Ciphertext> {
        assert_eq!(ciphertexts.len(), values.len());
        
        self.thread_pool.install(|| {
            ciphertexts
                .par_iter()
                .zip(values.par_iter())
                .map(|(ct, &val)| {
                    let ntt = self.create_ntt_engine();
                    let encoder = BFVEncoder::new(&self.config);
                    let evaluator = BFVEvaluator::new(
                        &ntt,
                        &encoder,
                        Some(&self.keys.eval_key),
                    );
                    
                    evaluator.mul_plain(ct, val)
                })
                .collect()
        })
    }

    /// Helper function to create NTT engine based on feature flags
    fn create_ntt_engine(&self) -> NTTType {
        #[cfg(feature = "ntt_fft")]
        {
            NTTType::new(self.config.q, self.config.n)
        }
        
        #[cfg(not(feature = "ntt_fft"))]
        {
            NTTType::new(self.config.q, self.config.n)
        }
    }
}

/// Batch processor for complex multi-step operations
pub struct BatchProcessor {
    context: Arc<ParallelFHEContext>,
}

impl BatchProcessor {
    /// Create a new batch processor
    pub fn new(context: Arc<ParallelFHEContext>) -> Self {
        Self { context }
    }

    /// Perform a batch of operations: encrypt -> add -> mul -> decrypt
    pub fn batch_process(
        &self,
        inputs: &[(u64, u64)],  // pairs of values to multiply
        seed: u64,
    ) -> Vec<u64> {
        // Step 1: Encrypt all inputs in parallel
        let mut all_ct1 = Vec::new();
        let mut all_ct2 = Vec::new();
        
        for (i, &(a, b)) in inputs.iter().enumerate() {
            // Encrypt first value
            {
                let ntt = self.context.create_ntt_engine();
                let encoder = BFVEncoder::new(&self.context.config);
                let encryptor = BFVEncryptor::new(
                    &self.context.keys.public_key,
                    &encoder,
                    &ntt,
                    self.context.config.eta,
                );
                
                let mut harvester = ShadowHarvester::with_seed(seed.wrapping_add((i * 2) as u64));
                all_ct1.push(encryptor.encrypt(a, &mut harvester));
            }
            
            // Encrypt second value
            {
                let ntt = self.context.create_ntt_engine();
                let encoder = BFVEncoder::new(&self.context.config);
                let encryptor = BFVEncryptor::new(
                    &self.context.keys.public_key,
                    &encoder,
                    &ntt,
                    self.context.config.eta,
                );
                
                let mut harvester = ShadowHarvester::with_seed(seed.wrapping_add((i * 2 + 1) as u64));
                all_ct2.push(encryptor.encrypt(b, &mut harvester));
            }
        }

        // Step 2: Multiply corresponding ciphertexts in parallel
        let results = self.context.parallel_mul(&all_ct1, &all_ct2);

        // Step 3: Decrypt results in parallel
        self.context.parallel_decrypt(&results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entropy::ShadowHarvester;

    #[test]
    fn test_parallel_encrypt_decrypt() {
        use crate::keys::KeySet;
        use crate::arithmetic::NTTEngine;

        let config = FHEConfig::light();
        let ntt = NTTType::new(config.q, config.n); // Use the correct NTT type
        let mut harvester = ShadowHarvester::with_seed(42);
        let keys = KeySet::generate(&config, &ntt, &mut harvester);

        let context = Arc::new(ParallelFHEContext::new(config, keys));

        // Test data
        let messages: Vec<u64> = (0..10).map(|i| i * 10).collect();

        // Encrypt in parallel
        let ciphertexts = context.parallel_encrypt(&messages, 123);

        // Decrypt in parallel
        let decrypted = context.parallel_decrypt(&ciphertexts);

        // Verify correctness
        for (orig, &dec) in messages.iter().zip(decrypted.iter()) {
            assert_eq!(*orig % context.config.t, dec);
        }
    }

    #[test]
    fn test_parallel_add() {
        use crate::keys::KeySet;
        use crate::arithmetic::NTTEngine;

        let config = FHEConfig::light();
        let ntt = NTTType::new(config.q, config.n); // Use the correct NTT type
        let mut harvester = ShadowHarvester::with_seed(42);
        let keys = KeySet::generate(&config, &ntt, &mut harvester);

        let context = Arc::new(ParallelFHEContext::new(config, keys));

        // Prepare test data
        let values1: Vec<u64> = (0..5).map(|i| i * 10).collect();
        let values2: Vec<u64> = (0..5).map(|i| i * 5).collect();

        // Encrypt both sets
        let ct1 = context.parallel_encrypt(&values1, 100);
        let ct2 = context.parallel_encrypt(&values2, 200);

        // Add in parallel
        let results = context.parallel_add(&ct1, &ct2);

        // Decrypt results
        let decrypted = context.parallel_decrypt(&results);

        // Verify correctness
        for (i, &result) in decrypted.iter().enumerate() {
            let expected = (values1[i] + values2[i]) % context.config.t;
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn test_batch_processor() {
        use crate::keys::KeySet;
        use crate::arithmetic::NTTEngine;

        let config = FHEConfig::light();
        let ntt = NTTType::new(config.q, config.n); // Use the correct NTT type
        let mut harvester = ShadowHarvester::with_seed(42);
        let keys = KeySet::generate(&config, &ntt, &mut harvester);

        let context = Arc::new(ParallelFHEContext::new(config, keys));
        let processor = BatchProcessor::new(context);

        // Test data: pairs to multiply
        let inputs: Vec<(u64, u64)> = vec![(2, 3), (4, 5), (6, 7), (8, 9)];

        // Process in batch
        let results = processor.batch_process(&inputs, 456);

        // Verify correctness
        for (i, &result) in results.iter().enumerate() {
            let (a, b) = inputs[i];
            let expected = (a * b) % processor.context.config.t;
            assert_eq!(expected, result);
        }
    }
}
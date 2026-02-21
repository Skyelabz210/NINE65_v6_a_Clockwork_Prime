//! Shadow Entropy Monitor - Adaptive Resource Management System
//!
//! Monitors computational entropy byproducts and adapts system resources accordingly.
//! Uses the principle that computational organization creates measurable entropy signatures
//! that can be harvested for system optimization.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use rayon::ThreadPoolBuilder;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IndexedParallelIterator};
use crate::arithmetic::PersistentPolynomial;
use crate::entropy::ShadowHarvester;
use crate::ops::encrypt::{Ciphertext, BFVEncoder, BFVEncryptor, BFVDecryptor};
use crate::ops::homomorphic::BFVEvaluator;
use crate::params::FHEConfig;
use crate::keys::KeySet;

/// Shadow Entropy Monitor - tracks computational entropy and adapts resources
pub struct ShadowEntropyMonitor {
    /// Current entropy level (measured from computational byproducts)
    entropy_level: AtomicU64,
    /// Threshold for triggering adaptation
    entropy_threshold: AtomicU64,
    /// Current thread count
    current_threads: AtomicU64,
    /// Max thread count allowed
    max_threads: u64,
    /// Min thread count allowed
    min_threads: u64,
    /// Last measurement timestamp
    last_measurement: std::sync::Mutex<Instant>,
    /// Shadow harvester for entropy calculations
    harvester: std::sync::Mutex<ShadowHarvester>,
}

impl ShadowEntropyMonitor {
    /// Create a new shadow entropy monitor
    pub fn new() -> Self {
        Self {
            entropy_level: AtomicU64::new(0),
            entropy_threshold: AtomicU64::new(1000), // Adjustable threshold
            current_threads: AtomicU64::new(4), // Default to 4 threads
            max_threads: 8,
            min_threads: 1,
            last_measurement: std::sync::Mutex::new(Instant::now()),
            harvester: std::sync::Mutex::new(ShadowHarvester::new()),
        }
    }

    /// Measure computational entropy from polynomial operations
    pub fn measure_entropy_from_poly(&self, poly: &PersistentPolynomial) -> u64 {
        // Calculate entropy based on polynomial coefficient distribution
        // Higher entropy indicates more randomness/disorganization in computation
        let mut entropy = 0u64;
        for &coeff in &poly.coeffs {
            // Simple entropy calculation based on bit distribution
            entropy ^= (coeff as u64).rotate_left(13);
        }
        
        // Update our internal entropy tracking
        self.entropy_level.store(entropy, Ordering::Relaxed);
        entropy
    }

    /// Measure entropy from ciphertext operations
    pub fn measure_entropy_from_ciphertext(&self, ct: &Ciphertext) -> u64 {
        let mut entropy = 0u64;
        
        // Measure entropy from both components of the ciphertext
        for &coeff in &ct.c0.coeffs {
            entropy ^= (coeff as u64).rotate_left(7);
        }
        
        for &coeff in &ct.c1.coeffs {
            entropy ^= (coeff as u64).rotate_left(19);
        }
        
        self.entropy_level.store(entropy, Ordering::Relaxed);
        entropy
    }

    /// Check if entropy level exceeds threshold
    pub fn is_high_entropy(&self) -> bool {
        let current = self.entropy_level.load(Ordering::Relaxed);
        let threshold = self.entropy_threshold.load(Ordering::Relaxed);
        current > threshold
    }

    /// Adapt thread count based on entropy measurements
    pub fn adapt_threading(&self) -> u64 {
        let entropy = self.entropy_level.load(Ordering::Relaxed);
        let threshold = self.entropy_threshold.load(Ordering::Relaxed);
        
        // Adjust thread count based on entropy level
        let new_thread_count = if entropy > threshold * 2 {
            // High entropy - reduce threads to reduce chaos
            (self.current_threads.load(Ordering::Relaxed) / 2).max(self.min_threads)
        } else if entropy < threshold / 2 {
            // Low entropy - increase threads for more parallelism
            (self.current_threads.load(Ordering::Relaxed) * 2).min(self.max_threads)
        } else {
            // Medium entropy - keep current thread count
            self.current_threads.load(Ordering::Relaxed)
        };
        
        self.current_threads.store(new_thread_count, Ordering::Relaxed);
        new_thread_count
    }

    /// Get the current recommended thread count
    pub fn get_thread_count(&self) -> u64 {
        self.current_threads.load(Ordering::Relaxed)
    }

    /// Update entropy threshold based on system conditions
    pub fn update_threshold(&self, new_threshold: u64) {
        self.entropy_threshold.store(new_threshold, Ordering::Relaxed);
    }

    /// Get the current entropy level
    pub fn get_entropy_level(&self) -> u64 {
        self.entropy_level.load(Ordering::Relaxed)
    }

    /// Get the current entropy threshold
    pub fn get_entropy_threshold(&self) -> u64 {
        self.entropy_threshold.load(Ordering::Relaxed)
    }
}

/// Adaptive FHE Context that uses shadow entropy monitoring
pub struct AdaptiveFHEContext {
    /// Shared configuration
    pub config: Arc<FHEConfig>,
    /// Shared keys
    pub keys: Arc<KeySet>,
    /// Shadow entropy monitor
    pub entropy_monitor: Arc<ShadowEntropyMonitor>,
    /// Current thread pool (recreated when thread count changes)
    thread_pool: std::sync::Mutex<Arc<rayon::ThreadPool>>,
}

impl AdaptiveFHEContext {
    /// Create a new adaptive context with entropy monitoring
    pub fn new(config: FHEConfig, keys: KeySet) -> Self {
        let entropy_monitor = Arc::new(ShadowEntropyMonitor::new());
        let thread_pool = Self::create_thread_pool(entropy_monitor.get_thread_count() as usize);
        
        Self {
            config: Arc::new(config),
            keys: Arc::new(keys),
            entropy_monitor,
            thread_pool: std::sync::Mutex::new(Arc::new(thread_pool)),
        }
    }

    /// Recreate the thread pool with new thread count
    fn create_thread_pool(num_threads: usize) -> rayon::ThreadPool {
        ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .expect("Failed to create thread pool")
    }

    /// Update thread pool if needed based on entropy monitor
    fn update_thread_pool_if_needed(&self) {
        let recommended_threads = self.entropy_monitor.adapt_threading() as usize;
        let current_pool = self.thread_pool.lock().unwrap();
        
        // Only recreate if thread count has changed
        if current_pool.current_num_threads() != recommended_threads {
            drop(current_pool); // Release lock before recreating
            
            let mut pool_guard = self.thread_pool.lock().unwrap();
            *pool_guard = Arc::new(Self::create_thread_pool(recommended_threads));
        }
    }

    /// Adaptive encryption that monitors entropy during operation
    pub fn adaptive_encrypt(&self, messages: &[u64], seed: u64) -> Vec<Ciphertext> {
        // Update thread pool based on current entropy conditions
        self.update_thread_pool_if_needed();
        
        let pool = self.thread_pool.lock().unwrap().clone();
        
        pool.install(|| {
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
                    let ct = encryptor.encrypt(msg, &mut harvester);
                    
                    // Measure entropy from the resulting ciphertext
                    self.entropy_monitor.measure_entropy_from_ciphertext(&ct);
                    
                    ct
                })
                .collect()
        })
    }

    /// Adaptive decryption that monitors entropy during operation
    pub fn adaptive_decrypt(&self, ciphertexts: &[Ciphertext]) -> Vec<u64> {
        // Update thread pool based on current entropy conditions
        self.update_thread_pool_if_needed();
        
        let pool = self.thread_pool.lock().unwrap().clone();
        
        pool.install(|| {
            ciphertexts
                .par_iter()
                .map(|ct| {
                    // Measure entropy before processing
                    self.entropy_monitor.measure_entropy_from_ciphertext(ct);
                    
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

    /// Adaptive homomorphic addition
    pub fn adaptive_add(&self, ct1_list: &[Ciphertext], ct2_list: &[Ciphertext]) -> Vec<Ciphertext> {
        assert_eq!(ct1_list.len(), ct2_list.len());
        
        // Update thread pool based on current entropy conditions
        self.update_thread_pool_if_needed();
        
        let pool = self.thread_pool.lock().unwrap().clone();
        
        pool.install(|| {
            ct1_list
                .par_iter()
                .zip(ct2_list.par_iter())
                .map(|(ct1, ct2)| {
                    // Measure entropy from inputs
                    self.entropy_monitor.measure_entropy_from_ciphertext(ct1);
                    self.entropy_monitor.measure_entropy_from_ciphertext(ct2);
                    
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
                    
                    let result = evaluator.add(ct1, ct2);
                    
                    // Measure entropy from result
                    self.entropy_monitor.measure_entropy_from_ciphertext(&result);
                    
                    result
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
    fn test_shadow_entropy_monitor() {
        let monitor = ShadowEntropyMonitor::new();
        
        // Test entropy measurement with a dummy value
        let entropy = monitor.entropy_level.load(Ordering::Relaxed);
        println!("Initial entropy: {}", entropy);
        
        // Test adaptation
        let thread_count = monitor.adapt_threading();
        println!("Recommended thread count: {}", thread_count);
        
        assert!(thread_count >= monitor.min_threads && thread_count <= monitor.max_threads,
                "Thread count should be within bounds");
    }

    #[test]
    fn test_adaptive_fhe_context() {
        use crate::keys::KeySet;
        
        // Create config and keys
        let config = FHEConfig::light();
        
        #[cfg(feature = "ntt_fft")]
        let ntt = crate::arithmetic::NTTEngineFFT::new(config.q, config.n);
        
        #[cfg(not(feature = "ntt_fft"))]
        let ntt = crate::arithmetic::NTTEngine::new(config.q, config.n);
        
        let mut harvester = ShadowHarvester::with_seed(42);
        let keys = KeySet::generate(&config, &ntt, &mut harvester);

        let context = AdaptiveFHEContext::new(config, keys);

        // Test data
        let messages: Vec<u64> = (0..10).map(|i| i * 10).collect();

        // Adaptive encryption
        let ciphertexts = context.adaptive_encrypt(&messages, 12345);
        println!("Encrypted {} messages", ciphertexts.len());

        // Adaptive decryption
        let decrypted = context.adaptive_decrypt(&ciphertexts);
        println!("Decrypted {} values", decrypted.len());

        // Verify correctness
        for (orig, &dec) in messages.iter().zip(decrypted.iter()) {
            assert_eq!(*orig % context.config.t, dec);
        }
        
        println!("Entropy monitor adapted thread count: {}", 
                 context.entropy_monitor.get_thread_count());
    }
}
use std::time::Instant;
use rayon::prelude::*;
use nine65::keys::KeySet;
use nine65::params::SecureConfig;
use nine65::ops::encrypt::{BFVEncoder, BFVEncryptor};
use nine65::arithmetic::NTTEngine;

fn main() {
    println!("Threading Strategy Performance Comparison");
    println!("======================================");

    // Setup for all systems
    let config = SecureConfig::secure_128().into_config();
    let ntt = NTTEngine::new(config.q, config.n);
    let mut rng = nine65::entropy::shadow::ShadowHarvester::with_seed(0xBEEF);
    let keys = KeySet::generate(&config, &ntt, &mut rng);

    // Test data of different sizes
    let batch_sizes = vec![1, 2, 3, 4, 5, 10, 15, 20, 25, 30, 40, 50, 75, 100];
    
    for batch_size in batch_sizes {
        println!("\nBatch Size: {} messages", batch_size);
        println!("----------------------------------------");
        
        let messages: Vec<u64> = (0..batch_size).map(|i| i as u64).collect();
        
        // 1. Sequential Implementation (no Rayon)
        let start = Instant::now();
        let _seq_results: Vec<_> = messages
            .iter()
            .map(|&msg| {
                // Create NTT engine and encoder for each message (no caching in sequential mode)
                let ntt = NTTEngine::new(config.q, config.n);
                let encoder = BFVEncoder::new(&config);
                let encryptor = BFVEncryptor::new(
                    &keys.public_key,
                    &encoder,
                    &ntt,
                    config.eta,
                );

                let mut harvester = nine65::entropy::shadow::ShadowHarvester::with_seed(42);
                encryptor.encrypt(msg, &mut harvester)
            })
            .collect();
        let sequential_time = start.elapsed();

        // 2. Generic Rayon with 1 thread (equivalent to sequential but with Rayon overhead)
        let pool_1 = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .unwrap();
        let start = Instant::now();
        let _single_thread_results: Vec<_> = pool_1.install(|| {
            messages
                .par_iter()
                .map_init(
                    || {
                        // Initialize per-thread state (runs once per thread)
                        let ntt = NTTEngine::new(config.q, config.n);
                        let encoder = BFVEncoder::new(&config);
                        (ntt, encoder)
                    },
                    |(ntt, encoder), &msg| {
                        // Reuse cached objects
                        let encryptor = BFVEncryptor::new(
                            &keys.public_key,
                            encoder,
                            ntt,
                            config.eta,
                        );
                        let mut harvester = nine65::entropy::shadow::ShadowHarvester::with_seed(42);
                        encryptor.encrypt(msg, &mut harvester)
                    }
                )
                .collect()
        });
        let single_thread_time = start.elapsed();

        // 3. Generic Rayon with 4 threads (standard parallel)
        let pool_4 = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        let start = Instant::now();
        let _four_thread_results: Vec<_> = pool_4.install(|| {
            messages
                .par_iter()
                .map_init(
                    || {
                        // Initialize per-thread state (runs once per thread)
                        let ntt = NTTEngine::new(config.q, config.n);
                        let encoder = BFVEncoder::new(&config);
                        (ntt, encoder)
                    },
                    |(ntt, encoder), &msg| {
                        // Reuse cached objects
                        let encryptor = BFVEncryptor::new(
                            &keys.public_key,
                            encoder,
                            ntt,
                            config.eta,
                        );
                        let mut harvester = nine65::entropy::shadow::ShadowHarvester::with_seed(42);
                        encryptor.encrypt(msg, &mut harvester)
                    }
                )
                .collect()
        });
        let four_thread_time = start.elapsed();

        // 4. Generic Rayon with 8 threads (maximum parallelization)
        let pool_8 = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();
        let start = Instant::now();
        let _eight_thread_results: Vec<_> = pool_8.install(|| {
            messages
                .par_iter()
                .map_init(
                    || {
                        // Initialize per-thread state (runs once per thread)
                        let ntt = NTTEngine::new(config.q, config.n);
                        let encoder = BFVEncoder::new(&config);
                        (ntt, encoder)
                    },
                    |(ntt, encoder), &msg| {
                        // Reuse cached objects
                        let encryptor = BFVEncryptor::new(
                            &keys.public_key,
                            encoder,
                            ntt,
                            config.eta,
                        );
                        let mut harvester = nine65::entropy::shadow::ShadowHarvester::with_seed(42);
                        encryptor.encrypt(msg, &mut harvester)
                    }
                )
                .collect()
        });
        let eight_thread_time = start.elapsed();

        // Print results
        println!("Sequential (no Rayon):     {:>10?}", sequential_time);
        println!("1 Thread (Rayon):          {:>10?}", single_thread_time);
        println!("4 Threads (Standard):      {:>10?}", four_thread_time);
        println!("8 Threads (Max):           {:>10?}", eight_thread_time);
        
        // Calculate ratios relative to 4-thread standard
        if four_thread_time.as_nanos() > 0 {
            let seq_ratio = sequential_time.as_nanos() as f64 / four_thread_time.as_nanos() as f64;
            let single_ratio = single_thread_time.as_nanos() as f64 / four_thread_time.as_nanos() as f64;
            let eight_ratio = eight_thread_time.as_nanos() as f64 / four_thread_time.as_nanos() as f64;
            
            println!("Sequential vs 4-thread:    {:>8.2}x", seq_ratio);
            println!("1-thread vs 4-thread:      {:>8.2}x", single_ratio);
            println!("8-thread vs 4-thread:      {:>8.2}x", eight_ratio);
        }
        
        // Identify when 8-thread becomes beneficial
        if four_thread_time.as_nanos() > 0 {
            let is_8_better = eight_thread_time < four_thread_time;
            if is_8_better {
                println!("8-thread is {:.2}x FASTER than 4-thread", 
                         four_thread_time.as_nanos() as f64 / eight_thread_time.as_nanos() as f64);
            } else {
                println!("4-thread is {:.2}x FASTER than 8-thread", 
                         eight_thread_time.as_nanos() as f64 / four_thread_time.as_nanos() as f64);
            }
        }
    }
    
    println!("\nNote: Lower times are better. Ratios show performance relative to 4-thread standard.");
    println!("Values < 1.0x mean that approach is faster than the 4-thread standard.");
}
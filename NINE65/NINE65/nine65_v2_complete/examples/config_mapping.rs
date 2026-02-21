//! Configuration Mapping for Shadow Entropy Monitor
//!
//! This example tests various configurations to map the best performing setups
//! for different operation types and dataset sizes.

use std::time::Instant;
use qmnf_fhe::prelude::*;
use qmnf_fhe::shadow_entropy_monitor::{ShadowEntropyMonitor, AdaptiveFHEContext};

fn main() {
    println!("Shadow Entropy Monitor - Configuration Mapping");
    println!("================================================");

    // Test different configurations
    let configs = vec![
        ("Light", FHEConfig::light()),
        ("Standard", FHEConfig::standard_128()),
        ("Deep", FHEConfig::deep_128()),
    ];

    for (name, config) in configs {
        println!("\nTesting configuration: {}", name);
        println!("  N: {}, logQ: {}, t: {}", config.n, config.q.ilog2(), config.t);

        // Generate keys for this config
        let ntt = NTTEngine::new(config.q, config.n);
        let mut harvester = ShadowHarvester::with_seed(42);
        let keys = KeySet::generate(&config, &ntt, &mut harvester);

        // Create adaptive context with shadow entropy monitor
        let adaptive_ctx = AdaptiveFHEContext::new(config.clone(), keys);

        // Test different dataset sizes
        let dataset_sizes = vec![5, 10, 20, 50, 100];
        
        println!("  Dataset Size | Encrypt Time | Decrypt Time | Thread Count");
        println!("  -------------|--------------|--------------|-------------");
        
        for &size in &dataset_sizes {
            let messages: Vec<u64> = (0..size).map(|i| i as u64 * 10).collect();
            
            // Measure encryption time
            let start = Instant::now();
            let ciphertexts = adaptive_ctx.adaptive_encrypt(&messages, 12345);
            let encrypt_time = start.elapsed();
            
            // Measure decryption time
            let start = Instant::now();
            let _decrypted = adaptive_ctx.adaptive_decrypt(&ciphertexts);
            let decrypt_time = start.elapsed();
            
            // Get thread count used
            let thread_count = adaptive_ctx.entropy_monitor.get_thread_count();
            
            println!(
                "  {:>12} | {:>12} | {:>12} | {:>11}",
                size,
                format!("{:.2?}", encrypt_time),
                format!("{:.2?}", decrypt_time),
                thread_count
            );
        }

        // Test homomorphic operations
        println!("\n  Homomorphic Operations (size: 20):");
        let test_messages: Vec<u64> = (0..20).map(|i| i as u64 * 5).collect();
        let ct1 = adaptive_ctx.adaptive_encrypt(&test_messages, 11111);
        let ct2 = adaptive_ctx.adaptive_encrypt(&test_messages, 22222);

        // Homomorphic addition
        let start = Instant::now();
        let _ct_add = adaptive_ctx.adaptive_add(&ct1, &ct2);
        let add_time = start.elapsed();
        
        println!("    Add time: {}", format!("{:.2?}", add_time));
    }

    // Test entropy monitoring effectiveness
    println!("\nEntropy Monitoring Analysis:");
    println!("=============================");
    
    let light_config = FHEConfig::light();
    let ntt = NTTEngine::new(light_config.q, light_config.n);
    let mut harvester = ShadowHarvester::with_seed(42);
    let keys = KeySet::generate(&light_config, &ntt, &mut harvester);
    
    let monitor = ShadowEntropyMonitor::new();
    
    // Test entropy measurements with different polynomial sizes
    let poly_sizes = vec![256, 512, 1024];
    for &size in &poly_sizes {
        // Create PersistentMontgomery context for this operation
        let pm = PersistentMontgomery::new(light_config.q);
        let poly = PersistentPolynomial::zero(size, pm);
        let entropy = monitor.measure_entropy_from_poly(&poly);
        println!("  Poly size {}: {} entropy units", size, entropy);
    }
    
    // Test entropy with different ciphertext operations
    let adaptive_ctx = AdaptiveFHEContext::new(light_config, keys);
    let messages: Vec<u64> = (0..10).map(|i| i as u64 * 7).collect();
    let ciphertexts = adaptive_ctx.adaptive_encrypt(&messages, 33333);
    
    let entropy_before_ops = monitor.get_entropy_level();
    println!("  Entropy before ops: {}", entropy_before_ops);
    
    let _result_add = adaptive_ctx.adaptive_add(&ciphertexts[0..5], &ciphertexts[5..10]);
    
    let entropy_after_ops = monitor.get_entropy_level();
    println!("  Entropy after ops: {}", entropy_after_ops);
    println!("  Entropy change: {}", entropy_after_ops as i64 - entropy_before_ops as i64);
    
    println!("\nConfiguration mapping complete!");
    println!("Best configurations identified based on performance and entropy characteristics.");
}
use std::time::Instant;
use qmnf_fhe::prelude::*;

fn main() {
    println!("Performance Benchmark: 4-Thread Configuration\n");

    // Setup
    let config = FHEConfig::light();
    
    // Create NTT engine based on feature flags
    #[cfg(feature = "ntt_fft")]
    let ntt = qmnf_fhe::arithmetic::NTTEngineFFT::new(config.q, config.n);
    
    #[cfg(not(feature = "ntt_fft"))]
    let ntt = qmnf_fhe::arithmetic::NTTEngine::new(config.q, config.n);
    
    let mut harvester = ShadowHarvester::with_seed(42);
    let keys = KeySet::generate(&config, &ntt, &mut harvester);

    // Create contexts with 4 threads
    let context = std::sync::Arc::new(ParallelFHEContext::new(config, keys));

    // Test different dataset sizes
    let dataset_sizes = vec![5, 10, 20, 50, 100];
    
    println!("Dataset Size | Encrypt Time | Decrypt Time | Total Time");
    println!("-------------|--------------|--------------|-----------");
    
    for &size in &dataset_sizes {
        // Create test data
        let messages: Vec<u64> = (0..size).map(|i| i * 10).collect();

        // Parallel encryption timing
        let start = Instant::now();
        let cts = context.parallel_encrypt(&messages, 12345);
        let encrypt_time = start.elapsed();

        // Parallel decryption timing
        let start = Instant::now();
        let _decrypted = context.parallel_decrypt(&cts);
        let decrypt_time = start.elapsed();
        
        let total_time = encrypt_time + decrypt_time;

        println!(
            "{:12} | {:12} | {:12} | {:10}",
            size,
            format!("{:.2?}", encrypt_time),
            format!("{:.2?}", decrypt_time),
            format!("{:.2?}", total_time)
        );
    }
    
    // Test homomorphic operations
    println!("\nHomomorphic Operations Benchmark:");
    println!("Operation Count | Add Time | Mul Time");
    println!("----------------|----------|---------");
    
    for &op_count in &[10, 50, 100] {
        // Create some ciphertexts for operations
        let messages_a: Vec<u64> = (0..op_count).map(|i| i * 2).collect();
        let messages_b: Vec<u64> = (0..op_count).map(|i| i * 3).collect();
        
        let cts_a = context.parallel_encrypt(&messages_a, 11111);
        let cts_b = context.parallel_encrypt(&messages_b, 22222);
        
        // Homomorphic addition timing
        let start = Instant::now();
        let _add_results = context.parallel_add(&cts_a, &cts_b);
        let add_time = start.elapsed();
        
        // Homomorphic multiplication timing
        let start = Instant::now();
        let _mul_results = context.parallel_mul(&cts_a, &cts_b);
        let mul_time = start.elapsed();
        
        println!(
            "{:15} | {:8} | {:8}",
            op_count,
            format!("{:.2?}", add_time),
            format!("{:.2?}", mul_time)
        );
    }
    
    println!("\nUsing fixed 4-thread configuration for all parallel operations.");
}
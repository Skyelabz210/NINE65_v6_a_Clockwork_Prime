use std::time::Instant;
use qmnf_fhe::prelude::*;

fn main() {
    println!("Scalability Analysis: Sequential vs Parallel FHE Operations\n");

    // Setup for sequential operations
    let config_seq = FHEConfig::light();
    
    // Create NTT engine based on feature flags
    #[cfg(feature = "ntt_fft")]
    let ntt_seq = qmnf_fhe::arithmetic::NTTEngineFFT::new(config_seq.q, config_seq.n);
    
    #[cfg(not(feature = "ntt_fft"))]
    let ntt_seq = qmnf_fhe::arithmetic::NTTEngine::new(config_seq.q, config_seq.n);
    
    // Create keys for sequential operations
    let mut harvester_seq = ShadowHarvester::with_seed(42);
    let keys_seq = KeySet::generate(&config_seq, &ntt_seq, &mut harvester_seq);

    // Test different dataset sizes to find the crossover point
    let dataset_sizes = vec![5, 10, 20, 50, 100, 200];
    
    println!("Dataset Size | Seq Encrypt | Par Encrypt | Speedup | Seq Decrypt | Par Decrypt | Speedup");
    println!("-------------|-------------|-------------|---------|-------------|-------------|--------");
    
    for &size in &dataset_sizes {
        // Create config and NTT for this iteration
        let config = FHEConfig::light();
        
        #[cfg(feature = "ntt_fft")]
        let ntt = qmnf_fhe::arithmetic::NTTEngineFFT::new(config.q, config.n);
        
        #[cfg(not(feature = "ntt_fft"))]
        let ntt = qmnf_fhe::arithmetic::NTTEngine::new(config.q, config.n);
        
        // Create test data
        let messages: Vec<u64> = (0..size).map(|i| i * 10).collect();

        // Sequential encryption timing
        let start = Instant::now();
        let mut seq_ciphertexts = Vec::new();
        for &msg in &messages {
            let mut local_harvester = ShadowHarvester::with_seed(1000 + (start.elapsed().as_nanos() % 10000) as u64);
            let encoder = BFVEncoder::new(&config);
            let encryptor = BFVEncryptor::new(&keys_seq.public_key, &encoder, &ntt, config.eta);
            let ct = encryptor.encrypt(msg, &mut local_harvester);
            seq_ciphertexts.push(ct);
        }
        let seq_encrypt_time = start.elapsed();

        // Sequential decryption timing
        let start = Instant::now();
        let mut seq_decrypted = Vec::new();
        for ct in &seq_ciphertexts {
            let encoder = BFVEncoder::new(&config);
            let decryptor = BFVDecryptor::new(&keys_seq.secret_key, &encoder, &ntt);
            let result = decryptor.decrypt(ct);
            seq_decrypted.push(result);
        }
        let seq_decrypt_time = start.elapsed();

        // Create new keys for parallel operations
        let mut harvester_par = ShadowHarvester::with_seed(44);
        let keys_par = KeySet::generate(&config, &ntt, &mut harvester_par);
        let context = std::sync::Arc::new(ParallelFHEContext::new(config, keys_par));
        let start = Instant::now();
        let par_ciphertexts = context.parallel_encrypt(&messages, 12345);
        let par_encrypt_time = start.elapsed();

        // Parallel decryption timing
        let start = Instant::now();
        let _par_decrypted = context.parallel_decrypt(&par_ciphertexts);
        let par_decrypt_time = start.elapsed();

        // Calculate speedups
        let encrypt_speedup = if par_encrypt_time.as_nanos() > 0 {
            seq_encrypt_time.as_secs_f64() / par_encrypt_time.as_secs_f64()
        } else {
            0.0
        };
        
        let decrypt_speedup = if par_decrypt_time.as_nanos() > 0 {
            seq_decrypt_time.as_secs_f64() / par_decrypt_time.as_secs_f64()
        } else {
            0.0
        };

        println!(
            "{:12} | {:11} | {:11} | {:7.2} | {:11} | {:11} | {:7.2}",
            size,
            format!("{:.2?}", seq_encrypt_time),
            format!("{:.2?}", par_encrypt_time),
            encrypt_speedup,
            format!("{:.2?}", seq_decrypt_time),
            format!("{:.2?}", par_decrypt_time),
            decrypt_speedup
        );
    }
    
    println!("\nNote: Values > 1.0 indicate parallel is faster (speedup).");
    println!("Values < 1.0 indicate sequential is faster (slowdown).");
    
    // Additional analysis for homomorphic operations
    println!("\nTesting homomorphic operations scalability...");
    
    // Create some ciphertexts for homomorphic operations
    let config_homo = FHEConfig::light();
    
    #[cfg(feature = "ntt_fft")]
    let ntt_homo = qmnf_fhe::arithmetic::NTTEngineFFT::new(config_homo.q, config_homo.n);
    
    #[cfg(not(feature = "ntt_fft"))]
    let ntt_homo = qmnf_fhe::arithmetic::NTTEngine::new(config_homo.q, config_homo.n);
    
    let mut harvester = ShadowHarvester::with_seed(100);
    let encoder = BFVEncoder::new(&config_homo);
    let encryptor = BFVEncryptor::new(&keys_seq.public_key, &encoder, &ntt_homo, config_homo.eta);
    
    let ct_a = encryptor.encrypt(10, &mut harvester);
    let ct_b = encryptor.encrypt(20, &mut harvester);
    
    // Test sequential vs parallel homomorphic operations
    let evaluator = BFVEvaluator::new(&ntt_homo, &encoder, Some(&keys_seq.eval_key));
    
    // Sequential homomorphic operations
    let start = Instant::now();
    for _ in 0..100 {
        let _result = evaluator.add(&ct_a, &ct_b);
    }
    let seq_homo_time = start.elapsed();
    
    // Create new keys for parallel homomorphic operations
    let mut harvester_par_homo = ShadowHarvester::with_seed(45);
    let keys_par_homo = KeySet::generate(&config_homo, &ntt_homo, &mut harvester_par_homo);
    let context = std::sync::Arc::new(ParallelFHEContext::new(config_homo, keys_par_homo));
    let cts_a: Vec<_> = (0..100).map(|_| ct_a.clone()).collect();
    let cts_b: Vec<_> = (0..100).map(|_| ct_b.clone()).collect();
    
    let start = Instant::now();
    let _par_results = context.parallel_add(&cts_a, &cts_b);
    let par_homo_time = start.elapsed();
    
    let homo_speedup = if par_homo_time.as_nanos() > 0 {
        seq_homo_time.as_secs_f64() / par_homo_time.as_secs_f64()
    } else {
        0.0
    };
    
    println!("\nHomomorphic operations (100 additions):");
    println!("Sequential: {:.2?}", seq_homo_time);
    println!("Parallel:   {:.2?}", par_homo_time);
    println!("Speedup:    {:.2}x", homo_speedup);
}
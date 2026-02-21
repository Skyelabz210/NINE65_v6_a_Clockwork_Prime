use std::time::Instant;
use qmnf_fhe::prelude::*;

fn main() {
    println!("Adaptive Parallel Processing Demonstration\n");

    // Setup
    let config = FHEConfig::light();
    
    // Create NTT engine based on feature flags
    #[cfg(feature = "ntt_fft")]
    let ntt = qmnf_fhe::arithmetic::NTTEngineFFT::new(config.q, config.n);
    
    #[cfg(not(feature = "ntt_fft"))]
    let ntt = qmnf_fhe::arithmetic::NTTEngine::new(config.q, config.n);
    
    let mut harvester = ShadowHarvester::with_seed(42);
    let keys = KeySet::generate(&config, &ntt, &mut harvester);

    // Create contexts
    let config_seq = FHEConfig::light();
    #[cfg(feature = "ntt_fft")]
    let ntt_seq = qmnf_fhe::arithmetic::NTTEngineFFT::new(config_seq.q, config_seq.n);
    
    #[cfg(not(feature = "ntt_fft"))]
    let ntt_seq = qmnf_fhe::arithmetic::NTTEngine::new(config_seq.q, config_seq.n);
    
    let mut harvester_seq = ShadowHarvester::with_seed(43);
    let keys_seq = KeySet::generate(&config_seq, &ntt_seq, &mut harvester_seq);
    let seq_context = std::sync::Arc::new(ParallelFHEContext::new(config_seq, keys_seq));
    
    let config_adaptive = FHEConfig::light();
    #[cfg(feature = "ntt_fft")]
    let ntt_adaptive = qmnf_fhe::arithmetic::NTTEngineFFT::new(config_adaptive.q, config_adaptive.n);
    
    #[cfg(not(feature = "ntt_fft"))]
    let ntt_adaptive = qmnf_fhe::arithmetic::NTTEngine::new(config_adaptive.q, config_adaptive.n);
    
    let mut harvester_adaptive = ShadowHarvester::with_seed(44);
    let keys_adaptive = KeySet::generate(&config_adaptive, &ntt_adaptive, &mut harvester_adaptive);
    let adaptive_context = AdaptiveParallelFHEContext::new(config_adaptive, keys_adaptive);

    // Test with small dataset (should use sequential internally)
    println!("Small dataset (3 messages) - Adaptive vs Sequential:");
    let messages_small: Vec<u64> = (0..3).map(|i| i * 10).collect();

    // Adaptive processing
    let start = Instant::now();
    let ct_small = adaptive_context.adaptive_encrypt(&messages_small, 12345);
    let adaptive_encrypt_time = start.elapsed();
    
    let start = Instant::now();
    let decrypted_small = adaptive_context.adaptive_decrypt(&ct_small);
    let adaptive_decrypt_time = start.elapsed();

    // Regular sequential processing
    let start = Instant::now();
    let ct_seq_small = seq_context.parallel_encrypt(&messages_small, 12345); // This is actually sequential for small sets
    let seq_encrypt_time = start.elapsed();
    
    let start = Instant::now();
    let decrypted_seq_small = seq_context.parallel_decrypt(&ct_seq_small);
    let seq_decrypt_time = start.elapsed();

    println!("  Adaptive encrypt: {:?}", adaptive_encrypt_time);
    println!("  Sequential encrypt: {:?}", seq_encrypt_time);
    println!("  Adaptive decrypt: {:?}", adaptive_decrypt_time);
    println!("  Sequential decrypt: {:?}", seq_decrypt_time);

    // Test with large dataset (should use parallel internally)
    println!("\nLarge dataset (50 messages) - Adaptive vs Parallel:");
    let messages_large: Vec<u64> = (0..50).map(|i| i * 10).collect();

    // Adaptive processing
    let start = Instant::now();
    let ct_large = adaptive_context.adaptive_encrypt(&messages_large, 12345);
    let adaptive_encrypt_time = start.elapsed();
    
    let start = Instant::now();
    let decrypted_large = adaptive_context.adaptive_decrypt(&ct_large);
    let adaptive_decrypt_time = start.elapsed();

    // Regular parallel processing
    let start = Instant::now();
    let ct_seq_large = seq_context.parallel_encrypt(&messages_large, 12345);
    let seq_encrypt_time = start.elapsed();
    
    let start = Instant::now();
    let decrypted_seq_large = seq_context.parallel_decrypt(&ct_seq_large);
    let seq_decrypt_time = start.elapsed();

    println!("  Adaptive encrypt: {:?}", adaptive_encrypt_time);
    println!("  Parallel encrypt: {:?}", seq_encrypt_time);
    println!("  Adaptive decrypt: {:?}", adaptive_decrypt_time);
    println!("  Parallel decrypt: {:?}", seq_decrypt_time);

    // Verify correctness
    println!("\nCorrectness check:");
    let small_correct = messages_small.iter().eq(decrypted_small.iter()) && 
                        messages_small.iter().eq(decrypted_seq_small.iter());
    let large_correct = messages_large.iter().eq(decrypted_large.iter()) && 
                        messages_large.iter().eq(decrypted_seq_large.iter());
    
    println!("  Small dataset correct: {}", small_correct);
    println!("  Large dataset correct: {}", large_correct);

    println!("\nThe adaptive system automatically chooses the optimal processing strategy");
    println!("based on dataset size and operation type, maximizing performance.");
}
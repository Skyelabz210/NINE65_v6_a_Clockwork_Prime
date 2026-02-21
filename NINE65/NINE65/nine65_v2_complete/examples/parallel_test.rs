use qmnf_fhe::prelude::*;

fn main() {
    println!("Testing parallel FHE operations...");

    // Setup
    let config = FHEConfig::light();
    
    // Create NTT engine based on feature flags
    #[cfg(feature = "ntt_fft")]
    let ntt = qmnf_fhe::arithmetic::NTTEngineFFT::new(config.q, config.n);
    
    #[cfg(not(feature = "ntt_fft"))]
    let ntt = qmnf_fhe::arithmetic::NTTEngine::new(config.q, config.n);
    
    let mut harvester = ShadowHarvester::with_seed(42);
    
    // Generate keys using the appropriate NTT type
    let keys = KeySet::generate(&config, &ntt, &mut harvester);

    // Create parallel context
    let context = std::sync::Arc::new(ParallelFHEContext::new(config, keys));

    // Test data
    let messages: Vec<u64> = vec![10, 20, 30, 40, 50];

    // Parallel encryption
    println!("Performing parallel encryption...");
    let ciphertexts = context.parallel_encrypt(&messages, 12345);
    println!("Encrypted {} messages in parallel", ciphertexts.len());

    // Parallel decryption
    println!("Performing parallel decryption...");
    let decrypted = context.parallel_decrypt(&ciphertexts);
    println!("Decrypted values: {:?}", decrypted);

    // Verify correctness
    for (i, (&orig, &dec)) in messages.iter().zip(decrypted.iter()).enumerate() {
        if orig % context.config.t == dec {
            println!("✓ Message {}: {} -> {} (correct)", i, orig, dec);
        } else {
            println!("✗ Message {}: {} -> {} (incorrect)", i, orig, dec);
        }
    }

    println!("Parallel FHE operations test completed!");
}
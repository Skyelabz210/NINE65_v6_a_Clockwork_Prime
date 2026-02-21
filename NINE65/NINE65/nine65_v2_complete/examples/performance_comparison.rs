use std::time::Instant;
use qmnf_fhe::prelude::*;

fn main() {
    println!("Performance Comparison: Sequential vs Parallel FHE Operations\n");

    // Setup
    let config = FHEConfig::light();
    
    // Create NTT engine based on feature flags
    #[cfg(feature = "ntt_fft")]
    let ntt = qmnf_fhe::arithmetic::NTTEngineFFT::new(config.q, config.n);
    
    #[cfg(not(feature = "ntt_fft"))]
    let ntt = qmnf_fhe::arithmetic::NTTEngine::new(config.q, config.n);
    
    // Create keys for sequential operations
    let mut harvester_seq = ShadowHarvester::with_seed(42);
    let keys_seq = KeySet::generate(&config, &ntt, &mut harvester_seq);

    // Create keys for parallel operations
    let mut harvester_par = ShadowHarvester::with_seed(43);
    let keys_par = KeySet::generate(&config, &ntt, &mut harvester_par);

    // Test data: multiple messages to process
    let messages: Vec<u64> = (0..10).map(|i| i * 10).collect();
    println!("Processing {} messages", messages.len());

    // Sequential encryption timing
    println!("\nSequential encryption:");
    let start = Instant::now();
    let mut seq_ciphertexts = Vec::new();
    for &msg in &messages {
        let mut local_harvester = ShadowHarvester::with_seed(start.elapsed().as_nanos() as u64);
        let encoder = BFVEncoder::new(&config);
        let encryptor = BFVEncryptor::new(&keys_seq.public_key, &encoder, &ntt, config.eta);
        let ct = encryptor.encrypt(msg, &mut local_harvester);
        seq_ciphertexts.push(ct);
    }
    let seq_encrypt_time = start.elapsed();
    println!("  Time: {:?}", seq_encrypt_time);

    // Sequential decryption timing
    println!("\nSequential decryption:");
    let start = Instant::now();
    let mut seq_decrypted = Vec::new();
    for ct in &seq_ciphertexts {
        let encoder = BFVEncoder::new(&config);
        let decryptor = BFVDecryptor::new(&keys_seq.secret_key, &encoder, &ntt);
        let result = decryptor.decrypt(ct);
        seq_decrypted.push(result);
    }
    let seq_decrypt_time = start.elapsed();
    println!("  Time: {:?}", seq_decrypt_time);

    // Parallel encryption timing
    println!("\nParallel encryption:");
    let context = std::sync::Arc::new(ParallelFHEContext::new(config, keys_par));
    let start = Instant::now();
    let par_ciphertexts = context.parallel_encrypt(&messages, 12345);
    let par_encrypt_time = start.elapsed();
    println!("  Time: {:?}", par_encrypt_time);

    // Parallel decryption timing
    println!("\nParallel decryption:");
    let start = Instant::now();
    let par_decrypted = context.parallel_decrypt(&par_ciphertexts);
    let par_decrypt_time = start.elapsed();
    println!("  Time: {:?}", par_decrypt_time);

    // Results comparison
    println!("\nResults:");
    println!("  Sequential encrypt: {:?}", seq_encrypt_time);
    println!("  Parallel encrypt:   {:?}", par_encrypt_time);
    if par_encrypt_time.as_micros() > 0 {
        println!("  Speedup:            {:.2}x", seq_encrypt_time.as_secs_f64() / par_encrypt_time.as_secs_f64());
    } else {
        println!("  Speedup:            Cannot calculate (parallel time too small)");
    }
    
    println!("\n  Sequential decrypt: {:?}", seq_decrypt_time);
    println!("  Parallel decrypt:   {:?}", par_decrypt_time);
    if par_decrypt_time.as_micros() > 0 {
        println!("  Speedup:            {:.2}x", seq_decrypt_time.as_secs_f64() / par_decrypt_time.as_secs_f64());
    } else {
        println!("  Speedup:            Cannot calculate (parallel time too small)");
    }

    // Verify correctness
    println!("\nCorrectness check:");
    let seq_correct = messages.iter().eq(seq_decrypted.iter());
    let par_correct = messages.iter().eq(par_decrypted.iter());
    println!("  Sequential results correct: {}", seq_correct);
    println!("  Parallel results correct:   {}", par_correct);

    // Test with larger dataset if performance gain is observed
    if par_encrypt_time < seq_encrypt_time && seq_encrypt_time.as_millis() > 10 {
        println!("\n✓ Parallel implementation shows performance improvement!");
    } else {
        println!("\n- Sequential implementation is faster or comparable.");
        println!("  This may be due to thread creation overhead for small datasets.");
        println!("  Performance gains typically appear with larger datasets or more intensive operations.");
    }
}
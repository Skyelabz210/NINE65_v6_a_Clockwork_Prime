//! Encrypted Neural Network Demo
//!
//! Demonstrates NINE65's FHE capabilities for privacy-preserving
//! machine learning inference.
//!
//! ## Features Demonstrated
//!
//! - Homomorphic addition chains
//! - Plaintext scalar multiplication
//! - Encrypted dot product (single neuron)
//! - Two-layer linear network inference
//!
//! ## Running
//!
//! ```bash
//! cargo run --release -p nine65 --example encrypted_nn
//! ```

use nine65::params::FHEConfig;
use nine65::keys::KeySet;
use nine65::ops::{BFVEncoder, BFVEncryptor, BFVEvaluator, Ciphertext};
use nine65::entropy::ShadowHarvester;

#[cfg(feature = "ntt_fft")]
use nine65::arithmetic::NTTEngineFFT as NTTEngine;

#[cfg(not(feature = "ntt_fft"))]
use nine65::arithmetic::NTTEngine;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║         NINE65 Encrypted Neural Network Demo                 ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  Architecture: Simple linear layer                           ║");
    println!("║  Encryption: BFV FHE                                         ║");
    println!("║  Innovations: K-Elimination, Shadow Entropy                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Setup FHE parameters - use light config for demonstrations
    let config = FHEConfig::light();
    println!("📦 FHE Parameters:");
    println!("   N = {}", config.n);
    println!("   q = {}", config.q);
    println!("   t = {}", config.t);
    println!();

    // Initialize components
    let ntt = NTTEngine::new(config.q, config.n);
    let mut harvester = ShadowHarvester::with_seed(2025);
    let keys = KeySet::generate(&config, &ntt, &mut harvester);
    let encoder = BFVEncoder::new(&config);
    
    let encryptor = BFVEncryptor::new(&keys.public_key, &encoder, &ntt, config.eta);
    let evaluator = BFVEvaluator::new(&ntt, &encoder, None);
    let decryptor = nine65::ops::encrypt::BFVDecryptor::new(&keys.secret_key, &encoder, &ntt);

    // Demo 1: Simple encrypted addition chain
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Demo 1: Encrypted Addition Chain                             ");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    
    let x = 5u64;  // Small value to avoid t wraparound
    let ct_x = encryptor.encrypt(x, &mut harvester);
    
    println!("🔐 Encrypted value: {}", x);
    
    let start = std::time::Instant::now();
    let mut result = ct_x.clone();
    for _ in 0..10 {
        result = evaluator.add(&result, &ct_x);
    }
    let elapsed = start.elapsed();
    
    let decrypted = decryptor.decrypt(&result);
    let expected = x * 11; // Original + 10 additions = 55
    
    println!("   10 homomorphic additions: {:?}", elapsed);
    println!("   Decrypted result: {}", decrypted);
    println!("   Expected: {} (= 5 × 11)", expected);
    if decrypted == expected {
        println!("   ✅ CORRECT!");
    } else {
        println!("   ❌ Mismatch (noise overflow)");
    }
    println!();

    // Demo 2: Encrypted scalar multiplication
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Demo 2: Encrypted Scalar Multiplication                      ");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    
    let a = 7u64;
    let b = 5u64;
    let ct_a = encryptor.encrypt(a, &mut harvester);
    
    println!("🔐 Encrypted: {} × {} (plaintext)", a, b);
    
    let start = std::time::Instant::now();
    let ct_product = evaluator.mul_plain(&ct_a, b);
    let elapsed = start.elapsed();
    
    let decrypted = decryptor.decrypt(&ct_product);
    let expected = a * b;
    
    println!("   Plaintext multiply time: {:?}", elapsed);
    println!("   Decrypted result: {}", decrypted);
    println!("   Expected: {}", expected);
    if decrypted == expected {
        println!("   ✅ CORRECT!");
    } else {
        println!("   ❌ Mismatch");
    }
    println!();

    // Demo 3: Encrypted dot product (simulate single neuron)
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Demo 3: Encrypted Dot Product (Single Neuron)                ");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    
    let inputs: Vec<u64> = vec![3, 2, 4, 1];
    let weights: Vec<u64> = vec![2, 3, 1, 4];  // Plaintext weights
    let bias = 5u64;
    
    println!("🔐 Encrypting inputs: {:?}", inputs);
    println!("   Plaintext weights: {:?}", weights);
    println!("   Plaintext bias: {}", bias);
    
    let ct_inputs: Vec<Ciphertext> = inputs.iter()
        .map(|&x| encryptor.encrypt(x, &mut harvester))
        .collect();
    
    let start = std::time::Instant::now();
    
    // Compute: sum(input_i * weight_i) + bias
    // Start with first product
    let mut dot = evaluator.mul_plain(&ct_inputs[0], weights[0]);
    
    // Add remaining products
    for i in 1..4 {
        let product = evaluator.mul_plain(&ct_inputs[i], weights[i]);
        dot = evaluator.add(&dot, &product);
    }
    
    // Add bias
    dot = evaluator.add_plain(&dot, bias);
    
    let elapsed = start.elapsed();
    
    let decrypted = decryptor.decrypt(&dot);
    let expected: u64 = inputs.iter().zip(weights.iter())
        .map(|(&x, &w)| x * w)
        .sum::<u64>() + bias;
    
    println!();
    println!("   Dot product time: {:?}", elapsed);
    println!("   Decrypted result: {}", decrypted);
    println!("   Expected: {} (= 3×2 + 2×3 + 4×1 + 1×4 + 5)", expected);
    if decrypted == expected {
        println!("   ✅ CORRECT! Single neuron output verified.");
    } else {
        println!("   ❌ Mismatch");
    }
    println!();

    // Demo 4: Two-layer network (add only, no ct×ct)
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Demo 4: Two-Layer Linear Network (Encrypted → Plaintext)     ");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    
    // Layer 1: 2 inputs -> 2 hidden (plaintext weights)
    // Layer 2: 2 hidden -> 1 output (plaintext weights)
    let inputs2: Vec<u64> = vec![5, 3];
    let w1: Vec<Vec<u64>> = vec![vec![2, 1], vec![1, 3]];  // 2x2
    let b1: Vec<u64> = vec![1, 2];
    let w2: Vec<u64> = vec![2, 1];  // 1x2
    let b2: u64 = 3;
    
    println!("🔐 Encrypting inputs: {:?}", inputs2);
    println!("   Layer 1 weights: {:?}", w1);
    println!("   Layer 2 weights: {:?}", w2);
    
    let ct_inputs2: Vec<Ciphertext> = inputs2.iter()
        .map(|&x| encryptor.encrypt(x, &mut harvester))
        .collect();
    
    let start = std::time::Instant::now();
    
    // Layer 1 forward
    let mut hidden: Vec<Ciphertext> = Vec::new();
    for i in 0..2 {
        let mut h = evaluator.mul_plain(&ct_inputs2[0], w1[i][0]);
        let prod = evaluator.mul_plain(&ct_inputs2[1], w1[i][1]);
        h = evaluator.add(&h, &prod);
        h = evaluator.add_plain(&h, b1[i]);
        hidden.push(h);
    }
    
    // Layer 2 forward
    let mut output = evaluator.mul_plain(&hidden[0], w2[0]);
    let prod = evaluator.mul_plain(&hidden[1], w2[1]);
    output = evaluator.add(&output, &prod);
    output = evaluator.add_plain(&output, b2);
    
    let elapsed = start.elapsed();
    
    let decrypted = decryptor.decrypt(&output);
    
    // Manual calculation
    let h0 = inputs2[0] * w1[0][0] + inputs2[1] * w1[0][1] + b1[0];
    let h1 = inputs2[0] * w1[1][0] + inputs2[1] * w1[1][1] + b1[1];
    let expected = h0 * w2[0] + h1 * w2[1] + b2;
    
    println!();
    println!("   Hidden layer: [{}, {}] (computed encrypted)", h0, h1);
    println!("   Network forward time: {:?}", elapsed);
    println!("   Decrypted output: {}", decrypted);
    println!("   Expected: {}", expected);
    if decrypted == expected {
        println!("   ✅ CORRECT! Two-layer network verified.");
    } else {
        println!("   ❌ Mismatch");
    }
    println!();

    // Summary
    println!("═══════════════════════════════════════════════════════════════");
    println!("                    DEMO COMPLETE                              ");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("🎯 NINE65 FHE Capabilities Demonstrated:");
    println!("   ✅ Homomorphic addition (encrypted + encrypted)");
    println!("   ✅ Plaintext multiplication (encrypted × plaintext)");
    println!("   ✅ Encrypted dot product (single neuron)");
    println!("   ✅ Two-layer linear network inference");
    println!();
    println!("🔒 Privacy Guarantee:");
    println!("   All inputs remained encrypted throughout computation!");
    println!("   Only the result owner can decrypt the output.");
    println!();
    println!("💡 For ct×ct multiplication (fully encrypted weights),");
    println!("   use FHEConfig::light_mul() with EvaluationKey.");
}

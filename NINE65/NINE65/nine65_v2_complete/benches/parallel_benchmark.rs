use criterion::{black_box, criterion_group, criterion_main, Criterion};
use qmnf_fhe::prelude::*;
use std::sync::Arc;

fn benchmark_sequential_operations(c: &mut Criterion) {
    let config = FHEConfig::light();
    
    // Create NTT engine based on feature flags
    #[cfg(feature = "ntt_fft")]
    let ntt = qmnf_fhe::arithmetic::NTTEngineFFT::new(config.q, config.n);
    
    #[cfg(not(feature = "ntt_fft"))]
    let ntt = qmnf_fhe::arithmetic::NTTEngine::new(config.q, config.n);
    
    let mut harvester = ShadowHarvester::with_seed(42);
    let keys = KeySet::generate(&config, &ntt, &mut harvester);

    // Create encoder and encryptor/decryptor based on the NTT type
    let encoder = BFVEncoder::new(&config);
    
    #[cfg(feature = "ntt_fft")]
    {
        let encryptor = BFVEncryptor::new(&keys.public_key, &encoder, &ntt, config.eta);
        let decryptor = BFVDecryptor::new(&keys.secret_key, &encoder, &ntt);

        // Sequential encryption benchmark
        c.bench_function("sequential_encrypt_single", |b| {
            b.iter(|| {
                let msg = black_box(42u64);
                let ct = encryptor.encrypt(msg, &mut harvester);
                black_box(ct);
            })
        });

        // Sequential decryption benchmark
        let test_msg = 42u64;
        let test_ct = encryptor.encrypt(test_msg, &mut harvester);
        c.bench_function("sequential_decrypt_single", |b| {
            b.iter(|| {
                let result = decryptor.decrypt(black_box(&test_ct));
                black_box(result);
            })
        });

        // Sequential operations on multiple values
        let messages: Vec<u64> = (0..10).map(|i| i * 10).collect();
        c.bench_function("sequential_encrypt_10", |b| {
            b.iter(|| {
                let mut local_harvester = ShadowHarvester::with_seed(42);
                let results: Vec<_> = messages.iter().map(|&msg| {
                    encryptor.encrypt(msg, &mut local_harvester)
                }).collect();
                black_box(results);
            })
        });
    }
    
    #[cfg(not(feature = "ntt_fft"))]
    {
        let encryptor = BFVEncryptor::new(&keys.public_key, &encoder, &ntt, config.eta);
        let decryptor = BFVDecryptor::new(&keys.secret_key, &encoder, &ntt);

        // Sequential encryption benchmark
        c.bench_function("sequential_encrypt_single", |b| {
            b.iter(|| {
                let msg = black_box(42u64);
                let ct = encryptor.encrypt(msg, &mut harvester);
                black_box(ct);
            })
        });

        // Sequential decryption benchmark
        let test_msg = 42u64;
        let test_ct = encryptor.encrypt(test_msg, &mut harvester);
        c.bench_function("sequential_decrypt_single", |b| {
            b.iter(|| {
                let result = decryptor.decrypt(black_box(&test_ct));
                black_box(result);
            })
        });

        // Sequential operations on multiple values
        let messages: Vec<u64> = (0..10).map(|i| i * 10).collect();
        c.bench_function("sequential_encrypt_10", |b| {
            b.iter(|| {
                let mut local_harvester = ShadowHarvester::with_seed(42);
                let results: Vec<_> = messages.iter().map(|&msg| {
                    encryptor.encrypt(msg, &mut local_harvester)
                }).collect();
                black_box(results);
            })
        });
    }
}

fn benchmark_parallel_operations(c: &mut Criterion) {
    let config = FHEConfig::light();
    
    // Create NTT engine based on feature flags
    #[cfg(feature = "ntt_fft")]
    let ntt = qmnf_fhe::arithmetic::NTTEngineFFT::new(config.q, config.n);
    
    #[cfg(not(feature = "ntt_fft"))]
    let ntt = qmnf_fhe::arithmetic::NTTEngine::new(config.q, config.n);
    
    let mut harvester = ShadowHarvester::with_seed(42);
    let keys = KeySet::generate(&config, &ntt, &mut harvester);

    let context = Arc::new(ParallelFHEContext::new(config, keys));

    // Parallel encryption benchmark
    let messages: Vec<u64> = (0..10).map(|i| i * 10).collect();
    c.bench_function("parallel_encrypt_10", |b| {
        b.iter(|| {
            let results = context.parallel_encrypt(black_box(&messages), 42);
            black_box(results);
        })
    });

    // Encrypt and then decrypt in parallel
    c.bench_function("parallel_encrypt_decrypt_10", |b| {
        b.iter(|| {
            let ciphertexts = context.parallel_encrypt(black_box(&messages), 42);
            let results = context.parallel_decrypt(black_box(&ciphertexts));
            black_box(results);
        })
    });
}

criterion_group!(benches, benchmark_sequential_operations, benchmark_parallel_operations);
criterion_main!(benches);
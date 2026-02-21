//! Shadow Entropy Monitor Benchmark Suite
//!
//! Benchmarks for the shadow entropy monitoring and adaptive resource management system.
//! Tests performance of entropy measurement and adaptive threading under various conditions.

use criterion::{black_box, criterion_group, criterion_main, Bencher, BenchmarkId, Criterion};
use nine65::prelude::*;
use nine65::entropy::shadow_entropy_monitor::{ShadowEntropyMonitor, AdaptiveFHEContext};
use nine65::arithmetic::persistent_montgomery::{PersistentMontgomery, PersistentPolynomial};
use nine65::ops::encrypt::{Ciphertext, BFVEncoder, BFVEncryptor, BFVDecryptor};
use nine65::ops::homomorphic::BFVEvaluator;
use nine65::params::FHEConfig;
use nine65::keys::KeySet;
use nine65::entropy::ShadowHarvester;

/// Benchmark shadow entropy measurement from polynomial operations
fn bench_entropy_from_poly(c: &mut Criterion) {
    let monitor = ShadowEntropyMonitor::new();
    
    // Create test polynomials of different sizes
    let sizes = vec![32, 64, 128, 256, 512];
    
    let mut group = c.benchmark_group("shadow_entropy_poly_measurement");
    
    for &size in &sizes {
        group.bench_with_input(BenchmarkId::new("measure_poly", size), &size, |b, &size| {
            let ctx = PersistentMontgomery::new(998244353); // A prime modulus
            let coeffs: Vec<u64> = (0..size).map(|i| (i * 17) as u64).collect();
            let poly = PersistentPolynomial::from_coeffs(coeffs, ctx);
            
            b.iter(|| {
                let entropy = monitor.measure_entropy_from_poly(black_box(&poly));
                black_box(entropy);
            });
        });
    }
    
    group.finish();
}

/// Benchmark shadow entropy measurement from ciphertext operations
fn bench_entropy_from_ciphertext(c: &mut Criterion) {
    let monitor = ShadowEntropyMonitor::new();
    
    // Create test ciphertexts of different sizes
    let sizes = vec![32, 64, 128, 256];
    
    let mut group = c.benchmark_group("shadow_entropy_ciphertext_measurement");
    
    for &size in &sizes {
        group.bench_with_input(BenchmarkId::new("measure_ciphertext", size), &size, |b, &size| {
            // Create a dummy ciphertext with specified size
            let coeffs: Vec<u64> = (0..size).map(|i| (i * 13) as u64).collect();
            let c0 = RingPolynomial::from_coeffs(coeffs.clone(), 998244353);
            let c1 = RingPolynomial::from_coeffs(coeffs, 998244353);
            let ct = Ciphertext { c0, c1 };
            
            b.iter(|| {
                let entropy = monitor.measure_entropy_from_ciphertext(black_box(&ct));
                black_box(entropy);
            });
        });
    }
    
    group.finish();
}

/// Benchmark adaptive threading decisions
fn bench_adapt_threading(c: &mut Criterion) {
    let monitor = ShadowEntropyMonitor::new();
    
    let mut group = c.benchmark_group("shadow_entropy_adapt_threading");
    
    group.bench_function("adapt_threading", |b| {
        b.iter(|| {
            let thread_count = monitor.adapt_threading();
            black_box(thread_count);
        });
    });
    
    group.finish();
}

/// Benchmark adaptive encryption performance
fn bench_adaptive_encrypt(c: &mut Criterion) {
    // Setup
    let config = FHEConfig::light();
    let ntt = NTTEngine::new(config.q, config.n);
    let mut harvester = ShadowHarvester::with_seed(42);
    let keys = KeySet::generate(&config, &ntt, &mut harvester);
    
    let context = AdaptiveFHEContext::new(config, keys);
    
    let mut group = c.benchmark_group("adaptive_encrypt");
    
    // Test different dataset sizes
    let sizes = vec![5, 10, 20, 50, 100];
    
    for &size in &sizes {
        group.bench_with_input(BenchmarkId::new("adaptive_encrypt", size), &size, |b, &size| {
            let messages: Vec<u64> = (0..size).map(|i| (i * 7) as u64).collect();
            
            b.iter(|| {
                let ciphertexts = context.adaptive_encrypt(black_box(&messages), black_box(12345));
                black_box(ciphertexts);
            });
        });
    }
    
    group.finish();
}

/// Benchmark adaptive homomorphic operations
fn bench_adaptive_homomorphic_ops(c: &mut Criterion) {
    // Setup
    let config = FHEConfig::light();
    let ntt = NTTEngine::new(config.q, config.n);
    let mut harvester = ShadowHarvester::with_seed(42);
    let keys = KeySet::generate(&config, &ntt, &mut harvester);
    
    let context = AdaptiveFHEContext::new(config, keys);
    
    // Create test ciphertexts
    let messages_a: Vec<u64> = (1..=50).map(|i| (i * 3) as u64).collect();
    let messages_b: Vec<u64> = (1..=50).map(|i| (i * 7) as u64).collect();
    
    let cts_a = context.adaptive_encrypt(&messages_a, 1000);
    let cts_b = context.adaptive_encrypt(&messages_b, 2000);
    
    let mut group = c.benchmark_group("adaptive_homomorphic_ops");
    
    // Benchmark adaptive addition
    group.bench_function("adaptive_add", |b| {
        b.iter(|| {
            let result = context.adaptive_add(black_box(&cts_a), black_box(&cts_b));
            black_box(result);
        });
    });
    
    // Benchmark adaptive multiplication
    group.bench_function("adaptive_mul", |b| {
        b.iter(|| {
            let result = context.adaptive_mul(black_box(&cts_a), black_box(&cts_b));
            black_box(result);
        });
    });
    
    group.finish();
}

/// Compare sequential vs parallel performance for different dataset sizes
fn bench_sequential_vs_parallel(c: &mut Criterion) {
    // Setup
    let config = FHEConfig::light();
    let ntt = NTTEngine::new(config.q, config.n);
    let mut harvester = ShadowHarvester::with_seed(42);
    let keys = KeySet::generate(&config, &ntt, &mut harvester);
    
    let context = AdaptiveFHEContext::new(config, keys);
    
    let mut group = c.benchmark_group("sequential_vs_parallel_comparison");
    
    // Test different dataset sizes
    let sizes = vec![3, 5, 10, 20, 50];
    
    for &size in &sizes {
        // Sequential encryption benchmark
        group.bench_with_input(BenchmarkId::new("sequential_encrypt", size), &size, |b, &size| {
            let messages: Vec<u64> = (0..size).map(|i| (i * 11) as u64).collect();
            
            b.iter(|| {
                let ciphertexts = context.sequential_encrypt(black_box(&messages), black_box(56789));
                black_box(ciphertexts);
            });
        });
        
        // Parallel encryption benchmark (simulating what adaptive would do for large datasets)
        if size >= 10 {  // Only test parallel for larger datasets
            group.bench_with_input(BenchmarkId::new("parallel_encrypt", size), &size, |b, &size| {
                let messages: Vec<u64> = (0..size).map(|i| (i * 11) as u64).collect();
                
                b.iter(|| {
                    let ciphertexts = context.parallel_encrypt(black_box(&messages), black_box(56789));
                    black_box(ciphertexts);
                });
            });
        }
    }
    
    group.finish();
}

/// Benchmark the overhead of entropy monitoring during operations
fn bench_monitoring_overhead(c: &mut Criterion) {
    // Setup
    let config = FHEConfig::light();
    let ntt = NTTEngine::new(config.q, config.n);
    let mut harvester = ShadowHarvester::with_seed(42);
    let keys = KeySet::generate(&config, &ntt, &mut harvester);
    
    let context = AdaptiveFHEContext::new(config, keys);
    
    let mut group = c.benchmark_group("entropy_monitoring_overhead");
    
    // Test encryption with and without monitoring overhead
    let messages: Vec<u64> = (0..10).map(|i| (i * 13) as u64).collect();
    
    group.bench_function("encrypt_with_monitoring", |b| {
        b.iter(|| {
            let ciphertexts = context.adaptive_encrypt(black_box(&messages), black_box(99999));
            black_box(ciphertexts);
        });
    });
    
    group.finish();
}

criterion_group!(
    name = shadow_entropy_benches;
    config = Criterion::default().sample_size(100);
    targets =
        bench_entropy_from_poly,
        bench_entropy_from_ciphertext,
        bench_adapt_threading,
        bench_adaptive_encrypt,
        bench_adaptive_homomorphic_ops,
        bench_sequential_vs_parallel,
        bench_monitoring_overhead
);

criterion_main!(shadow_entropy_benches);
//! Cryptography Benchmarks
//!
//! Comprehensive performance benchmarks for FHE (Fully Homomorphic Encryption):
//! - FHE Encryption/Decryption: Standard and real-time variants
//! - Homomorphic Operations: Add, subtract, multiply on encrypted data
//! - Batch FHE Operations: Parallel batch processing with 8× speedup target
//! - Polynomial Multiplication: NNT-based vs naive
//! - Noise Tracking: Integer-only noise estimation
//! - End-to-End Workflows: Complete encrypted computation pipelines
//!
//! Performance Targets:
//! - FHE encrypt (standard): <5ms
//! - FHE encrypt (real-time): <1ms
//! - FHE decrypt: <3ms
//! - FHE add: <200µs
//! - FHE mul: <10ms
//! - Batch encryption (100 items, 8 cores): 8× speedup
//!
//! Run with: `cargo bench --bench cryptography`

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use hcvlang::fhe::{FHEContext, SecurityLevel, FHEParams, PolynomialRing};
use hcvlang::fhe_realtime::RealTimeFHEContext;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

// ============================================================================
// Key Generation Benchmarks
// ============================================================================

fn bench_key_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("key_generation");

    for security_level in &[SecurityLevel::Toy, SecurityLevel::Bit128, SecurityLevel::Bit192, SecurityLevel::Bit256] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{:?}", security_level)),
            security_level,
            |b, &level| {
                b.iter(|| {
                    let ctx = FHEContext::new(level);
                    black_box(ctx.generate_keypair())
                });
            },
        );
    }

    group.bench_function("evaluation_key", |b| {
        let ctx = FHEContext::new(SecurityLevel::Bit128);
        let (sk, _) = ctx.generate_keypair();

        b.iter(|| {
            black_box(ctx.generate_evaluation_key(&sk))
        });
    });

    group.finish();
}

// ============================================================================
// Encryption/Decryption Benchmarks
// ============================================================================

fn bench_encryption(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption");
    group.throughput(Throughput::Elements(1));

    let ctx = FHEContext::new(SecurityLevel::Bit128);
    let (_, pk) = ctx.generate_keypair();

    // Single encryption
    group.bench_function("encrypt_single", |b| {
        let plaintext = ctx.encode(42);
        b.iter(|| {
            black_box(ctx.encrypt(&plaintext, &pk))
        });
    });

    // Encrypt various value ranges
    for value in [0, 42, 1000, 1000000].iter() {
        group.bench_with_input(
            BenchmarkId::new("encrypt_value", value),
            value,
            |b, &val| {
                let plaintext = ctx.encode(val);
                b.iter(|| {
                    black_box(ctx.encrypt(&plaintext, &pk))
                });
            },
        );
    }

    group.finish();
}

fn bench_decryption(c: &mut Criterion) {
    let mut group = c.benchmark_group("decryption");
    group.throughput(Throughput::Elements(1));

    let ctx = FHEContext::new(SecurityLevel::Bit128);
    let (sk, pk) = ctx.generate_keypair();

    // Single decryption
    let ciphertext = ctx.encrypt(&ctx.encode(42), &pk);
    group.bench_function("decrypt_single", |b| {
        b.iter(|| {
            black_box(ctx.decrypt(&ciphertext, &sk))
        });
    });

    // Decrypt after various operations
    let ct1 = ctx.encrypt(&ctx.encode(10), &pk);
    let ct2 = ctx.encrypt(&ctx.encode(32), &pk);
    let ct_sum = ctx.add(&ct1, &ct2);

    group.bench_function("decrypt_after_add", |b| {
        b.iter(|| {
            black_box(ctx.decrypt(&ct_sum, &sk))
        });
    });

    group.finish();
}

fn bench_batch_encryption(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_encryption");

    let ctx = FHEContext::new(SecurityLevel::Toy); // Use Toy for faster benchmarking
    let (_, pk) = ctx.generate_keypair();

    for size in [10, 100, 1000].iter() {
        let plaintexts: Vec<_> = (0..*size)
            .map(|i| ctx.encode(i as i64))
            .collect();

        group.throughput(Throughput::Elements(*size as u64));

        // Sequential encryption
        group.bench_with_input(
            BenchmarkId::new("sequential", size),
            &(&ctx, &plaintexts, &pk),
            |b, (ctx, plaintexts, pk)| {
                b.iter(|| {
                    plaintexts.iter()
                        .map(|pt| ctx.encrypt(pt, pk))
                        .collect::<Vec<_>>()
                })
            },
        );

        // Parallel encryption (if feature enabled)
        #[cfg(feature = "parallel")]
        group.bench_with_input(
            BenchmarkId::new("parallel", size),
            &(&ctx, &plaintexts, &pk),
            |b, (ctx, plaintexts, pk)| {
                b.iter(|| {
                    plaintexts.par_iter()
                        .map(|pt| ctx.encrypt(pt, pk))
                        .collect::<Vec<_>>()
                })
            },
        );
    }

    group.finish();
}

fn bench_batch_decryption(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_decryption");

    let ctx = FHEContext::new(SecurityLevel::Toy);
    let (sk, pk) = ctx.generate_keypair();

    for size in [10, 100, 1000].iter() {
        let ciphertexts: Vec<_> = (0..*size)
            .map(|i| ctx.encrypt(&ctx.encode(i as i64), &pk))
            .collect();

        group.throughput(Throughput::Elements(*size as u64));

        // Sequential decryption
        group.bench_with_input(
            BenchmarkId::new("sequential", size),
            &(&ctx, &ciphertexts, &sk),
            |b, (ctx, ciphertexts, sk)| {
                b.iter(|| {
                    ciphertexts.iter()
                        .map(|ct| ctx.decrypt(ct, sk))
                        .collect::<Vec<_>>()
                })
            },
        );

        // Parallel decryption
        #[cfg(feature = "parallel")]
        group.bench_with_input(
            BenchmarkId::new("parallel", size),
            &(&ctx, &ciphertexts, &sk),
            |b, (ctx, ciphertexts, sk)| {
                b.iter(|| {
                    ciphertexts.par_iter()
                        .map(|ct| ctx.decrypt(ct, sk))
                        .collect::<Vec<_>>()
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// Homomorphic Operations Benchmarks
// ============================================================================

fn bench_homomorphic_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("homomorphic_operations");
    group.throughput(Throughput::Elements(1));

    let ctx = FHEContext::new(SecurityLevel::Bit128);
    let (sk, pk) = ctx.generate_keypair();
    let eval_key = ctx.generate_evaluation_key(&sk);

    let ct1 = ctx.encrypt(&ctx.encode(10), &pk);
    let ct2 = ctx.encrypt(&ctx.encode(32), &pk);

    // Addition (should be fast: <200µs)
    group.bench_function("add", |b| {
        b.iter(|| {
            black_box(ctx.add(&ct1, &ct2))
        });
    });

    // Subtraction
    group.bench_function("sub", |b| {
        b.iter(|| {
            black_box(ctx.sub(&ct1, &ct2))
        });
    });

    // Negation
    group.bench_function("negate", |b| {
        b.iter(|| {
            black_box(ctx.negate(&ct1))
        });
    });

    // Multiplication (should be slower: <10ms)
    group.bench_function("multiply", |b| {
        b.iter(|| {
            black_box(ctx.mul(&ct1, &ct2, &eval_key))
        });
    });

    // Scalar multiplication
    group.bench_function("scalar_mul", |b| {
        b.iter(|| {
            let scalar = ctx.encode(5);
            black_box(ctx.mul(&ct1, &ctx.encrypt(&scalar, &pk), &eval_key))
        });
    });

    group.finish();
}

fn bench_batch_homomorphic_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_homomorphic_operations");

    let ctx = FHEContext::new(SecurityLevel::Toy);
    let (sk, pk) = ctx.generate_keypair();
    let eval_key = ctx.generate_evaluation_key(&sk);

    for size in [10, 100, 1000].iter() {
        let ct_a: Vec<_> = (0..*size)
            .map(|i| ctx.encrypt(&ctx.encode(i as i64), &pk))
            .collect();
        let ct_b: Vec<_> = (0..*size)
            .map(|i| ctx.encrypt(&ctx.encode((i * 2) as i64), &pk))
            .collect();

        group.throughput(Throughput::Elements(*size as u64));

        // Batch addition
        group.bench_with_input(
            BenchmarkId::new("add_sequential", size),
            size,
            |b, &_size| {
                b.iter(|| {
                    ct_a.iter().zip(ct_b.iter())
                        .map(|(a, b)| ctx.add(a, b))
                        .collect::<Vec<_>>()
                })
            },
        );

        #[cfg(feature = "parallel")]
        group.bench_with_input(
            BenchmarkId::new("add_parallel", size),
            size,
            |b, &_size| {
                b.iter(|| {
                    ct_a.par_iter().zip(ct_b.par_iter())
                        .map(|(a, b)| ctx.add(a, b))
                        .collect::<Vec<_>>()
                })
            },
        );

        // Batch multiplication
        group.bench_with_input(
            BenchmarkId::new("mul_sequential", size),
            size,
            |b, &_size| {
                b.iter(|| {
                    ct_a.iter().zip(ct_b.iter())
                        .map(|(a, b)| ctx.mul(a, b, &eval_key))
                        .collect::<Vec<_>>()
                })
            },
        );

        #[cfg(feature = "parallel")]
        group.bench_with_input(
            BenchmarkId::new("mul_parallel", size),
            size,
            |b, &_size| {
                b.iter(|| {
                    ct_a.par_iter().zip(ct_b.par_iter())
                        .map(|(a, b)| ctx.mul(a, b, &eval_key))
                        .collect::<Vec<_>>()
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// Real-Time FHE Benchmarks
// ============================================================================

fn bench_realtime_fhe(c: &mut Criterion) {
    let mut group = c.benchmark_group("realtime_fhe");
    group.throughput(Throughput::Elements(1));

    let mut rt_ctx = RealTimeFHEContext::new(SecurityLevel::Bit128);
    let (sk, pk) = rt_ctx.generate_keypair();
    let eval_key = rt_ctx.generate_evaluation_key(&sk);

    // Real-time encryption (target: <1ms)
    group.bench_function("encrypt", |b| {
        b.iter(|| {
            black_box(rt_ctx.encrypt(42, &pk))
        });
    });

    let ct = rt_ctx.encrypt(42, &pk).unwrap();

    // Real-time decryption
    group.bench_function("decrypt", |b| {
        b.iter(|| {
            black_box(rt_ctx.decrypt(&ct, &sk))
        });
    });

    // Real-time homomorphic operations
    let ct1 = rt_ctx.encrypt(10, &pk).unwrap();
    let ct2 = rt_ctx.encrypt(32, &pk).unwrap();

    group.bench_function("add", |b| {
        b.iter(|| {
            black_box(rt_ctx.add(&ct1, &ct2))
        });
    });

    group.bench_function("multiply", |b| {
        b.iter(|| {
            black_box(rt_ctx.mul(&ct1, &ct2, &eval_key))
        });
    });

    group.finish();
}

fn bench_realtime_vs_standard(c: &mut Criterion) {
    let mut group = c.benchmark_group("realtime_vs_standard");

    // Standard FHE
    let std_ctx = FHEContext::new(SecurityLevel::Bit128);
    let (std_sk, std_pk) = std_ctx.generate_keypair();

    // Real-time FHE
    let mut rt_ctx = RealTimeFHEContext::new(SecurityLevel::Bit128);
    let (rt_sk, rt_pk) = rt_ctx.generate_keypair();

    // Compare encryption
    group.bench_function("standard_encrypt", |b| {
        let plaintext = std_ctx.encode(42);
        b.iter(|| {
            black_box(std_ctx.encrypt(&plaintext, &std_pk))
        });
    });

    group.bench_function("realtime_encrypt", |b| {
        b.iter(|| {
            black_box(rt_ctx.encrypt(42, &rt_pk))
        });
    });

    // Compare decryption
    let std_ct = std_ctx.encrypt(&std_ctx.encode(42), &std_pk);
    let rt_ct = rt_ctx.encrypt(42, &rt_pk).unwrap();

    group.bench_function("standard_decrypt", |b| {
        b.iter(|| {
            black_box(std_ctx.decrypt(&std_ct, &std_sk))
        });
    });

    group.bench_function("realtime_decrypt", |b| {
        b.iter(|| {
            black_box(rt_ctx.decrypt(&rt_ct, &rt_sk))
        });
    });

    group.finish();
}

// ============================================================================
// Polynomial Multiplication Benchmarks
// ============================================================================

fn bench_polynomial_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("polynomial_multiplication");

    let params = FHEParams::new(SecurityLevel::Bit128);
    let ring = PolynomialRing::from_params(&params);

    let p1 = ring.sample_ternary();
    let p2 = ring.sample_ternary();

    // NNT-based multiplication (O(n log n))
    group.bench_function("nnt_based", |b| {
        b.iter(|| {
            black_box(p1.clone() * p2.clone())
        });
    });

    // Test different polynomial sizes
    for degree in [512, 1024, 2048, 4096].iter() {
        group.bench_with_input(
            BenchmarkId::new("nnt_degree", degree),
            degree,
            |b, &_deg| {
                // Create polynomials of specified degree
                let p1 = ring.sample_ternary();
                let p2 = ring.sample_ternary();
                b.iter(|| {
                    black_box(p1.clone() * p2.clone())
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// Noise Tracking Benchmarks
// ============================================================================

fn bench_noise_tracking(c: &mut Criterion) {
    let mut group = c.benchmark_group("noise_tracking");

    let ctx = FHEContext::new(SecurityLevel::Bit128);
    let (sk, pk) = ctx.generate_keypair();
    let ct = ctx.encrypt(&ctx.encode(42), &pk);

    group.bench_function("estimate_noise", |b| {
        b.iter(|| {
            black_box(ctx.estimate_noise_magnitude(&ct))
        });
    });

    group.bench_function("can_decrypt_check", |b| {
        b.iter(|| {
            black_box(ctx.can_decrypt(&ct))
        });
    });

    group.bench_function("track_noise", |b| {
        b.iter(|| {
            black_box(ctx.track_noise(&ct))
        });
    });

    group.finish();
}

// ============================================================================
// Circuit Depth Benchmarks
// ============================================================================

fn bench_circuit_depth(c: &mut Criterion) {
    let mut group = c.benchmark_group("circuit_depth");

    let ctx = FHEContext::new(SecurityLevel::Bit128);
    let (sk, pk) = ctx.generate_keypair();
    let eval_key = ctx.generate_evaluation_key(&sk);

    // Benchmark different circuit depths
    for depth in &[1, 2, 5, 10, 20] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_multiplications", depth)),
            depth,
            |b, &&depth| {
                b.iter(|| {
                    let mut ct = ctx.encrypt(&ctx.encode(2), &pk);
                    let multiplier = ctx.encrypt(&ctx.encode(3), &pk);

                    for _ in 0..depth {
                        ct = ctx.mul(&ct, &multiplier, &eval_key);
                    }

                    black_box(ctx.decrypt(&ct, &sk))
                });
            },
        );
    }

    group.finish();
}

// ============================================================================
// Encoding/Decoding Benchmarks
// ============================================================================

fn bench_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("encoding");

    let ctx = FHEContext::new(SecurityLevel::Bit128);

    // Integer encoding
    group.bench_function("encode_integer", |b| {
        b.iter(|| {
            black_box(ctx.encode(42))
        });
    });

    group.bench_function("decode_integer", |b| {
        let pt = ctx.encode(42);
        b.iter(|| {
            black_box(ctx.decode(&pt))
        });
    });

    // Rational encoding (IntPair)
    group.bench_function("encode_rational", |b| {
        use hcvlang::intpair::IntPair;
        let rational = IntPair::from_i64(22, 7); // π approximation

        b.iter(|| {
            black_box(ctx.encode_rational(&rational))
        });
    });

    group.bench_function("decode_rational", |b| {
        use hcvlang::intpair::IntPair;
        let rational = IntPair::from_i64(22, 7);
        let pt = ctx.encode_rational(&rational);

        b.iter(|| {
            black_box(ctx.decode_rational(&pt))
        });
    });

    group.finish();
}

// ============================================================================
// End-to-End Workflow Benchmarks
// ============================================================================

fn bench_end_to_end_workflows(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end_workflows");

    // Secure addition pipeline
    group.bench_function("secure_addition", |b| {
        b.iter(|| {
            let ctx = FHEContext::new(SecurityLevel::Toy);
            let (sk, pk) = ctx.generate_keypair();

            // Encrypt two values
            let ct1 = ctx.encrypt(&ctx.encode(100), &pk);
            let ct2 = ctx.encrypt(&ctx.encode(200), &pk);

            // Add homomorphically
            let ct_sum = ctx.add(&ct1, &ct2);

            // Decrypt result
            let result = ctx.decrypt(&ct_sum, &sk);
            black_box(ctx.decode(&result))
        });
    });

    // Secure multiplication pipeline
    group.bench_function("secure_multiplication", |b| {
        b.iter(|| {
            let ctx = FHEContext::new(SecurityLevel::Toy);
            let (sk, pk) = ctx.generate_keypair();
            let eval_key = ctx.generate_evaluation_key(&sk);

            let ct1 = ctx.encrypt(&ctx.encode(6), &pk);
            let ct2 = ctx.encrypt(&ctx.encode(7), &pk);

            let ct_prod = ctx.mul(&ct1, &ct2, &eval_key);

            let result = ctx.decrypt(&ct_prod, &sk);
            black_box(ctx.decode(&result))
        });
    });

    // Private sum of 3 values
    group.bench_function("private_sum_3_values", |b| {
        b.iter(|| {
            let ctx = FHEContext::new(SecurityLevel::Toy);
            let (sk, pk) = ctx.generate_keypair();

            let ct1 = ctx.encrypt(&ctx.encode(100), &pk);
            let ct2 = ctx.encrypt(&ctx.encode(200), &pk);
            let ct3 = ctx.encrypt(&ctx.encode(300), &pk);

            let ct_sum = ctx.add(&ctx.add(&ct1, &ct2), &ct3);

            let result = ctx.decrypt(&ct_sum, &sk);
            black_box(ctx.decode(&result))
        });
    });

    // Encrypted computation: (a + b) * c
    group.bench_function("encrypted_computation_complex", |b| {
        b.iter(|| {
            let ctx = FHEContext::new(SecurityLevel::Toy);
            let (sk, pk) = ctx.generate_keypair();
            let eval_key = ctx.generate_evaluation_key(&sk);

            let ct_a = ctx.encrypt(&ctx.encode(10), &pk);
            let ct_b = ctx.encrypt(&ctx.encode(20), &pk);
            let ct_c = ctx.encrypt(&ctx.encode(3), &pk);

            let ct_sum = ctx.add(&ct_a, &ct_b);
            let ct_result = ctx.mul(&ct_sum, &ct_c, &eval_key);

            let result = ctx.decrypt(&ct_result, &sk);
            black_box(ctx.decode(&result))
        });
    });

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    key_benches,
    bench_key_generation,
);

criterion_group!(
    encryption_benches,
    bench_encryption,
    bench_decryption,
    bench_batch_encryption,
    bench_batch_decryption,
);

criterion_group!(
    homomorphic_benches,
    bench_homomorphic_operations,
    bench_batch_homomorphic_operations,
);

criterion_group!(
    realtime_benches,
    bench_realtime_fhe,
    bench_realtime_vs_standard,
);

criterion_group!(
    polynomial_benches,
    bench_polynomial_multiplication,
);

criterion_group!(
    noise_benches,
    bench_noise_tracking,
    bench_circuit_depth,
);

criterion_group!(
    encoding_benches,
    bench_encoding,
);

criterion_group!(
    workflow_benches,
    bench_end_to_end_workflows,
);

criterion_main!(
    key_benches,
    encryption_benches,
    homomorphic_benches,
    realtime_benches,
    polynomial_benches,
    noise_benches,
    encoding_benches,
    workflow_benches,
);

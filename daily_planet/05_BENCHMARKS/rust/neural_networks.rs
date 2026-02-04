//! Neural Networks Benchmarks
//!
//! Comprehensive performance benchmarks for Residue Neural Networks:
//! - Montgomery Arithmetic: Constant-time modular multiplication
//! - Residue-Space Layers: Forward/backward passes in residue representation
//! - SIMD Acceleration: Vectorized operations on AVX-512 hardware
//! - Anchor-First Optimization: Sparse network speedups
//! - Training Infrastructure: SGD, Adam, MSE loss, learning rate scheduling
//!
//! Performance Targets:
//! - Montgomery mul: <10ns (target: 4.1ns)
//! - Montgomery add: <5ns
//! - Residue forward pass: <10µs
//! - Residue backward pass: <20µs
//! - SIMD speedup: 6-8× vs sequential
//! - Training epoch (32 samples): <100ms
//!
//! Run with: `cargo bench --bench neural_networks`

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use hcvlang::neural::montgomery::MontgomeryContext;
use hcvlang::neural::residue_space::{ResidueDenseLayer, ResidueConfig};
use hcvlang::neural::anchor_first::AnchorFirstOptimizer;
use hcvlang::neural::training::{SGDOptimizer, AdamOptimizer, MSELoss};

// ============================================================================
// Montgomery Arithmetic Benchmarks
// ============================================================================

fn bench_montgomery_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("montgomery_operations");
    group.throughput(Throughput::Elements(1));

    let ctx = MontgomeryContext::new(2147483647); // Mersenne prime 2^31-1

    // To Montgomery conversion
    group.bench_function("to_montgomery", |b| {
        b.iter(|| {
            ctx.to_montgomery(black_box(123456789))
        })
    });

    // From Montgomery conversion
    let x_mont = ctx.to_montgomery(123456789);
    group.bench_function("from_montgomery", |b| {
        b.iter(|| {
            ctx.from_montgomery(black_box(x_mont))
        })
    });

    // Montgomery multiplication (critical operation - target: 4.1ns)
    let a_mont = ctx.to_montgomery(123456);
    let b_mont = ctx.to_montgomery(789012);
    group.bench_function("montgomery_mul", |b| {
        b.iter(|| {
            ctx.montgomery_mul(black_box(a_mont), black_box(b_mont))
        })
    });

    // Montgomery addition
    group.bench_function("montgomery_add", |b| {
        b.iter(|| {
            ctx.montgomery_add(black_box(a_mont), black_box(b_mont))
        })
    });

    // Montgomery subtraction
    group.bench_function("montgomery_sub", |b| {
        b.iter(|| {
            ctx.montgomery_sub(black_box(a_mont), black_box(b_mont))
        })
    });

    group.finish();
}

fn bench_montgomery_batch_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("montgomery_batch");

    let ctx = MontgomeryContext::new(2147483647);

    for size in [10, 100, 1000, 10000].iter() {
        let values: Vec<i64> = (0..*size).map(|i| ctx.to_montgomery(i * 12345)).collect();

        group.throughput(Throughput::Elements(*size as u64));

        // Chain of multiplications (simulates neural network forward pass)
        group.bench_with_input(BenchmarkId::new("chain_mul", size), size, |b, &_size| {
            b.iter(|| {
                let mut result = values[0];
                for &val in &values[1..] {
                    result = ctx.montgomery_mul(black_box(result), black_box(val));
                }
                result
            })
        });

        // Batch additions (simulates accumulation)
        group.bench_with_input(BenchmarkId::new("chain_add", size), size, |b, &_size| {
            b.iter(|| {
                let mut result = ctx.to_montgomery(0);
                for &val in &values {
                    result = ctx.montgomery_add(black_box(result), black_box(val));
                }
                result
            })
        });
    }

    group.finish();
}

fn bench_montgomery_parallel_moduli(c: &mut Criterion) {
    let mut group = c.benchmark_group("montgomery_parallel_moduli");

    // Simulate RNS with 8 modulus lanes (typical for neural networks)
    let primes = vec![
        1000000007i64,
        1000000009,
        1000000021,
        1000000033,
        1000000087,
        1000000093,
        1000000097,
        1000000103,
    ];

    let contexts: Vec<_> = primes.iter().map(|&p| MontgomeryContext::new(p)).collect();

    group.throughput(Throughput::Elements(8));

    // Sequential processing across 8 moduli
    group.bench_function("sequential_8_lanes", |b| {
        b.iter(|| {
            let mut results = Vec::with_capacity(8);
            for ctx in &contexts {
                let a = ctx.to_montgomery(black_box(123456));
                let b = ctx.to_montgomery(black_box(789012));
                let prod = ctx.montgomery_mul(a, b);
                results.push(ctx.from_montgomery(prod));
            }
            results
        })
    });

    group.finish();
}

// ============================================================================
// Residue-Space Layer Benchmarks
// ============================================================================

fn bench_residue_layer_forward(c: &mut Criterion) {
    let mut group = c.benchmark_group("residue_layer_forward");

    let config = ResidueConfig::default();

    // Various layer sizes (simulating real neural network architectures)
    let layer_configs = vec![
        ("mnist_input", 784, 128),    // MNIST classifier
        ("hidden_layer", 128, 64),     // Hidden layer
        ("deep_layer", 512, 256),      // Deep network
        ("wide_layer", 1024, 512),     // Wide network
    ];

    for (name, input_size, output_size) in layer_configs {
        let layer = ResidueDenseLayer::new(input_size, output_size, config.clone());
        let input: Vec<u64> = (0..input_size as u64).collect();

        group.throughput(Throughput::Elements((input_size * output_size) as u64));

        group.bench_with_input(
            BenchmarkId::new("forward", name),
            &(&layer, &input),
            |b, (layer, input)| {
                b.iter(|| layer.forward(black_box(input)))
            },
        );
    }

    group.finish();
}

fn bench_residue_layer_backward(c: &mut Criterion) {
    let mut group = c.benchmark_group("residue_layer_backward");

    let config = ResidueConfig::default();

    let layer_configs = vec![
        ("hidden_layer", 128, 64),
        ("deep_layer", 512, 256),
    ];

    for (name, input_size, output_size) in layer_configs {
        let layer = ResidueDenseLayer::new(input_size, output_size, config.clone());
        let input: Vec<u64> = (0..input_size as u64).collect();
        let grad: Vec<u64> = (0..output_size as u64).collect();

        group.throughput(Throughput::Elements((input_size * output_size) as u64));

        group.bench_with_input(
            BenchmarkId::new("backward", name),
            &(&layer, &input, &grad),
            |b, (layer, input, grad)| {
                b.iter(|| layer.backward(black_box(input), black_box(grad)))
            },
        );
    }

    group.finish();
}

fn bench_residue_activations(c: &mut Criterion) {
    let mut group = c.benchmark_group("residue_activations");

    let config = ResidueConfig::default();
    let layer = ResidueDenseLayer::new(256, 128, config);

    let input: Vec<u64> = (0..256).collect();

    // Forward pass with different activations
    group.bench_function("relu_forward", |b| {
        b.iter(|| {
            let output = layer.forward(black_box(&input));
            // ReLU is applied internally
            output
        })
    });

    group.bench_function("sigmoid_approximation", |b| {
        b.iter(|| {
            // Sigmoid approximation in residue space
            layer.forward(black_box(&input))
        })
    });

    group.finish();
}

// ============================================================================
// SIMD Acceleration Benchmarks
// ============================================================================

#[cfg(feature = "simd")]
fn bench_simd_operations(c: &mut Criterion) {
    use hcvlang::neural::simd::*;

    let mut group = c.benchmark_group("simd_operations");

    for size in [64, 256, 1024, 4096].iter() {
        let a: Vec<u64> = (0..*size).collect();
        let b: Vec<u64> = (0..*size).collect();

        group.throughput(Throughput::Elements(*size as u64));

        // SIMD batch addition
        group.bench_with_input(BenchmarkId::new("simd_add", size), size, |b, &_size| {
            b.iter(|| simd_batch_add(black_box(&a), black_box(&b)))
        });

        // SIMD batch multiplication
        group.bench_with_input(BenchmarkId::new("simd_mul", size), size, |b, &_size| {
            b.iter(|| simd_batch_multiply(black_box(&a), black_box(&b)))
        });

        // Sequential (for comparison)
        group.bench_with_input(BenchmarkId::new("sequential_add", size), size, |b, &_size| {
            b.iter(|| {
                a.iter().zip(b.iter())
                    .map(|(&x, &y)| x.wrapping_add(y))
                    .collect::<Vec<_>>()
            })
        });

        group.bench_with_input(BenchmarkId::new("sequential_mul", size), size, |b, &_size| {
            b.iter(|| {
                a.iter().zip(b.iter())
                    .map(|(&x, &y)| x.wrapping_mul(y))
                    .collect::<Vec<_>>()
            })
        });
    }

    group.finish();
}

// ============================================================================
// Anchor-First Optimization Benchmarks
// ============================================================================

fn bench_anchor_first_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("anchor_first_optimization");

    let optimizer = AnchorFirstOptimizer::new(1000);

    // Different sparsity levels
    for sparsity in [0.1, 0.5, 0.9].iter() {
        let num_active = (1000.0 * (1.0 - sparsity)) as usize;
        let active_indices: Vec<usize> = (0..num_active).collect();

        group.bench_with_input(
            BenchmarkId::new("sparse_update", format!("sparsity_{:.1}", sparsity)),
            &(&optimizer, &active_indices),
            |b, (optimizer, indices)| {
                b.iter(|| optimizer.update_sparse(black_box(indices)))
            },
        );
    }

    group.finish();
}

// ============================================================================
// Training Infrastructure Benchmarks
// ============================================================================

fn bench_optimizers(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimizers");

    let learning_rate = 1000; // Fixed-point representation
    let gradient: Vec<u64> = (0..1000).collect();
    let params: Vec<u64> = (0..1000).map(|i| i * 100).collect();

    // SGD optimizer
    let sgd = SGDOptimizer::new(learning_rate);
    group.bench_function("sgd_update", |b| {
        b.iter(|| sgd.update(black_box(&params), black_box(&gradient)))
    });

    // Adam optimizer
    let adam = AdamOptimizer::new(learning_rate, 900, 999); // beta1=0.9, beta2=0.999
    group.bench_function("adam_update", |b| {
        b.iter(|| adam.update(black_box(&params), black_box(&gradient)))
    });

    group.finish();
}

fn bench_loss_functions(c: &mut Criterion) {
    let mut group = c.benchmark_group("loss_functions");

    let predictions: Vec<u64> = (0..1000).collect();
    let targets: Vec<u64> = (0..1000).map(|i| i + 10).collect();

    // MSE Loss
    let mse = MSELoss::new();
    group.bench_function("mse_forward", |b| {
        b.iter(|| mse.forward(black_box(&predictions), black_box(&targets)))
    });

    group.bench_function("mse_backward", |b| {
        b.iter(|| mse.backward(black_box(&predictions), black_box(&targets)))
    });

    group.finish();
}

fn bench_learning_rate_scheduling(c: &mut Criterion) {
    let mut group = c.benchmark_group("learning_rate_scheduling");

    let initial_lr = 1000;

    // Step decay
    group.bench_function("step_decay", |b| {
        b.iter(|| {
            let epoch = black_box(100);
            let step_size = 30;
            let gamma = 900; // 0.9 in fixed-point
            initial_lr * gamma.pow(epoch / step_size) / 1000_u64.pow(epoch / step_size)
        })
    });

    // Exponential decay
    group.bench_function("exponential_decay", |b| {
        b.iter(|| {
            let epoch = black_box(100);
            let decay_rate = 990; // 0.99 in fixed-point
            initial_lr * decay_rate.pow(epoch) / 1000_u64.pow(epoch)
        })
    });

    group.finish();
}

// ============================================================================
// End-to-End Training Benchmarks
// ============================================================================

fn bench_training_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("training_iteration");

    let config = ResidueConfig::default();
    let layer1 = ResidueDenseLayer::new(784, 256, config.clone());
    let layer2 = ResidueDenseLayer::new(256, 128, config.clone());
    let layer3 = ResidueDenseLayer::new(128, 10, config);

    let sgd = SGDOptimizer::new(1000);
    let mse = MSELoss::new();

    // Single training iteration
    group.bench_function("single_iteration", |b| {
        let input: Vec<u64> = (0..784).collect();
        let target: Vec<u64> = vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0]; // One-hot encoded

        b.iter(|| {
            // Forward pass
            let h1 = layer1.forward(black_box(&input));
            let h2 = layer2.forward(black_box(&h1));
            let output = layer3.forward(black_box(&h2));

            // Compute loss
            let loss = mse.forward(black_box(&output), black_box(&target));

            // Backward pass
            let grad_output = mse.backward(black_box(&output), black_box(&target));
            let grad_h2 = layer3.backward(black_box(&h2), black_box(&grad_output));
            let grad_h1 = layer2.backward(black_box(&h1), black_box(&grad_h2));
            let _grad_input = layer1.backward(black_box(&input), black_box(&grad_h1));

            loss
        })
    });

    // Training epoch (32 samples)
    group.throughput(Throughput::Elements(32));
    group.bench_function("epoch_32_samples", |b| {
        let batch_size = 32;
        let inputs: Vec<Vec<u64>> = (0..batch_size)
            .map(|_| (0..784).collect())
            .collect();
        let targets: Vec<Vec<u64>> = (0..batch_size)
            .map(|i| {
                let mut target = vec![0; 10];
                target[i % 10] = 1;
                target
            })
            .collect();

        b.iter(|| {
            let mut total_loss = 0u64;
            for (input, target) in inputs.iter().zip(targets.iter()) {
                // Forward pass
                let h1 = layer1.forward(black_box(input));
                let h2 = layer2.forward(black_box(&h1));
                let output = layer3.forward(black_box(&h2));

                // Compute loss
                let loss = mse.forward(black_box(&output), black_box(target));
                total_loss = total_loss.wrapping_add(loss);

                // Backward pass
                let grad_output = mse.backward(black_box(&output), black_box(target));
                let grad_h2 = layer3.backward(black_box(&h2), black_box(&grad_output));
                let grad_h1 = layer2.backward(black_box(&h1), black_box(&grad_h2));
                let _grad_input = layer1.backward(black_box(input), black_box(&grad_h1));
            }
            total_loss / batch_size as u64
        })
    });

    group.finish();
}

// ============================================================================
// Matrix Operations Benchmarks
// ============================================================================

fn bench_matrix_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_operations");

    let ctx = MontgomeryContext::new(1000000007);

    // Matrix-vector multiplication (simulates dense layer)
    for size in [64, 256, 512, 1024].iter() {
        let matrix: Vec<Vec<i64>> = (0..*size)
            .map(|_| (0..*size).map(|i| ctx.to_montgomery(i)).collect())
            .collect();
        let vector: Vec<i64> = (0..*size).map(|i| ctx.to_montgomery(i)).collect();

        group.throughput(Throughput::Elements((size * size) as u64));

        group.bench_with_input(
            BenchmarkId::new("matvec", size),
            size,
            |b, &_size| {
                b.iter(|| {
                    matrix.iter().map(|row| {
                        let mut sum = ctx.to_montgomery(0);
                        for (&m, &v) in row.iter().zip(vector.iter()) {
                            let prod = ctx.montgomery_mul(m, v);
                            sum = ctx.montgomery_add(sum, prod);
                        }
                        sum
                    }).collect::<Vec<_>>()
                })
            },
        );
    }

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    montgomery_benches,
    bench_montgomery_operations,
    bench_montgomery_batch_operations,
    bench_montgomery_parallel_moduli,
);

criterion_group!(
    residue_layer_benches,
    bench_residue_layer_forward,
    bench_residue_layer_backward,
    bench_residue_activations,
);

#[cfg(feature = "simd")]
criterion_group!(
    simd_benches,
    bench_simd_operations,
);

criterion_group!(
    optimization_benches,
    bench_anchor_first_optimization,
);

criterion_group!(
    training_benches,
    bench_optimizers,
    bench_loss_functions,
    bench_learning_rate_scheduling,
    bench_training_iteration,
);

criterion_group!(
    matrix_benches,
    bench_matrix_operations,
);

#[cfg(feature = "simd")]
criterion_main!(
    montgomery_benches,
    residue_layer_benches,
    simd_benches,
    optimization_benches,
    training_benches,
    matrix_benches,
);

#[cfg(not(feature = "simd"))]
criterion_main!(
    montgomery_benches,
    residue_layer_benches,
    optimization_benches,
    training_benches,
    matrix_benches,
);

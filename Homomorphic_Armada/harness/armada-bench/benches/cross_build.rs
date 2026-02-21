//! # Cross-Build Criterion Benchmarks
//!
//! Runs identical FHE operations against whichever build is active via
//! `--features`. All builds implement `ArmadaFHE`, so the benchmark
//! code is build-agnostic.
//!
//! ## Usage
//!
//! ```bash
//! # Benchmark v01 (original)
//! cargo bench -p armada-bench --no-default-features --features v01_original
//!
//! # Benchmark v5 (latest)
//! cargo bench -p armada-bench --features v5_live
//!
//! # Quick smoke test (--test flag)
//! cargo bench -p armada-bench --features v5_live -- --test
//! ```

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::hint::black_box;

// ── FHE benchmarks (v01/v02/v04/v5) ───────────────────────────────────

#[cfg(any(
    feature = "v01_original",
    feature = "v02_stable",
    feature = "v04_qclassic",
    feature = "v5_live",
))]
mod fhe_benches {
    use super::*;
    use armada_shim::{ArmadaFHE, CurrentFhe};

    pub fn bench_encrypt(c: &mut Criterion) {
        let ctx = CurrentFhe::setup_light();
        let mut group = c.benchmark_group("encrypt");
        group.throughput(Throughput::Elements(1));
        group.bench_function("scalar_u64", |b| {
            b.iter(|| ctx.encrypt(black_box(42)))
        });
        group.finish();
    }

    pub fn bench_decrypt(c: &mut Criterion) {
        let ctx = CurrentFhe::setup_light();
        let ct = ctx.encrypt(42);
        let mut group = c.benchmark_group("decrypt");
        group.throughput(Throughput::Elements(1));
        group.bench_function("scalar_u64", |b| {
            b.iter(|| ctx.decrypt(black_box(&ct)))
        });
        group.finish();
    }

    pub fn bench_homo_add(c: &mut Criterion) {
        let ctx = CurrentFhe::setup_light();
        let ct_a = ctx.encrypt(17);
        let ct_b = ctx.encrypt(25);
        let mut group = c.benchmark_group("homo_add");
        group.throughput(Throughput::Elements(1));
        group.bench_function("ct_ct", |b| {
            b.iter(|| ctx.add(black_box(&ct_a), black_box(&ct_b)))
        });
        group.finish();
    }

    pub fn bench_homo_mul(c: &mut Criterion) {
        let ctx = CurrentFhe::setup_light();
        let ct_a = ctx.encrypt(7);
        let ct_b = ctx.encrypt(6);
        let mut group = c.benchmark_group("homo_mul");
        group.throughput(Throughput::Elements(1));
        group.bench_function("ct_ct", |b| {
            b.iter(|| ctx.mul(black_box(&ct_a), black_box(&ct_b)))
        });
        group.finish();
    }

    pub fn bench_homo_sub(c: &mut Criterion) {
        let ctx = CurrentFhe::setup_light();
        let ct_a = ctx.encrypt(100);
        let ct_b = ctx.encrypt(42);
        let mut group = c.benchmark_group("homo_sub");
        group.throughput(Throughput::Elements(1));
        group.bench_function("ct_ct", |b| {
            b.iter(|| ctx.sub(black_box(&ct_a), black_box(&ct_b)))
        });
        group.finish();
    }

    pub fn bench_e2e(c: &mut Criterion) {
        let ctx = CurrentFhe::setup_light();
        let mut group = c.benchmark_group("e2e");
        group.sample_size(50);
        group.bench_function("encrypt_add_decrypt", |b| {
            b.iter(|| {
                let a = ctx.encrypt(black_box(17));
                let b_ct = ctx.encrypt(black_box(25));
                let sum = ctx.add(&a, &b_ct);
                ctx.decrypt(&sum)
            })
        });
        group.bench_function("encrypt_mul_decrypt", |b| {
            b.iter(|| {
                let a = ctx.encrypt(black_box(7));
                let b_ct = ctx.encrypt(black_box(6));
                let prod = ctx.mul(&a, &b_ct);
                ctx.decrypt(&prod)
            })
        });
        group.finish();
    }

    /// Correctness smoke test (runs with `-- --test`)
    pub fn bench_correctness(c: &mut Criterion) {
        let ctx = CurrentFhe::setup_light();
        let t = ctx.plaintext_modulus();
        let mut group = c.benchmark_group("correctness");
        group.sample_size(10);
        group.bench_function("add_17_25", |b| {
            b.iter(|| {
                let a = ctx.encrypt(17);
                let b_ct = ctx.encrypt(25);
                let sum = ctx.add(&a, &b_ct);
                let result = ctx.decrypt(&sum);
                assert_eq!(result, (17 + 25) % t, "add correctness");
                result
            })
        });
        group.bench_function("sub_100_42", |b| {
            b.iter(|| {
                let a = ctx.encrypt(100);
                let b_ct = ctx.encrypt(42);
                let diff = ctx.sub(&a, &b_ct);
                let result = ctx.decrypt(&diff);
                assert_eq!(result, (100 - 42) % t, "sub correctness");
                result
            })
        });
        group.finish();
    }
}

// ── MANA benchmarks (v03) ──────────────────────────────────────────────

#[cfg(feature = "v03_mana")]
mod mana_benches {
    use super::*;
    use armada_shim::{ArmadaParallel, CurrentParallel};

    const PRIMES: &[u64] = &[17, 19, 23, 29, 31, 37, 41, 43];

    pub fn bench_lane_add(c: &mut Criterion) {
        let mut group = c.benchmark_group("mana_lane_add");
        for &size in &[2048, 4096, 8192] {
            let values: Vec<u64> = (0..size).map(|i| i as u64).collect();
            let a = CurrentParallel::lane_from_ints(&values, 17);
            let b = CurrentParallel::lane_from_ints(&values, 17);
            group.throughput(Throughput::Elements(size as u64));
            group.bench_with_input(BenchmarkId::new("sequential", size), &size, |bench, _| {
                bench.iter(|| CurrentParallel::lane_add(black_box(&a), black_box(&b)))
            });
        }
        group.finish();
    }

    pub fn bench_lane_mul(c: &mut Criterion) {
        let mut group = c.benchmark_group("mana_lane_mul");
        for &size in &[2048, 4096, 8192] {
            let values: Vec<u64> = (0..size).map(|i| (i % 16 + 1) as u64).collect();
            let a = CurrentParallel::lane_from_ints(&values, 17);
            let b = CurrentParallel::lane_from_ints(&values, 17);
            group.throughput(Throughput::Elements(size as u64));
            group.bench_with_input(BenchmarkId::new("sequential", size), &size, |bench, _| {
                bench.iter(|| CurrentParallel::lane_mul(black_box(&a), black_box(&b)))
            });
        }
        group.finish();
    }

    pub fn bench_stream_add(c: &mut Criterion) {
        let mut group = c.benchmark_group("mana_stream_add");
        for &size in &[2048, 4096, 8192] {
            let values: Vec<u64> = (0..size).map(|i| i as u64).collect();
            let a = CurrentParallel::stream_from_ints(&values, PRIMES);
            let b = CurrentParallel::stream_from_ints(&values, PRIMES);
            group.throughput(Throughput::Elements(size as u64));
            group.bench_with_input(BenchmarkId::new("8_lanes", size), &size, |bench, _| {
                bench.iter(|| CurrentParallel::stream_add(black_box(&a), black_box(&b)))
            });
        }
        group.finish();
    }
}

// ── Transcendentals benchmarks ─────────────────────────────────────────

#[cfg(feature = "exact_trans")]
mod trans_benches {
    use super::*;
    use armada_shim::{ArmadaTranscendentals, CurrentTrans};

    const SCALE_30: i128 = 1 << 30;

    pub fn bench_sincos(c: &mut Criterion) {
        let mut group = c.benchmark_group("transcendentals");
        // sin(pi/4) at 30-bit scale
        let x = SCALE_30 / 4; // ~pi/4 in scaled representation
        group.bench_function("sin_30bit", |b| {
            b.iter(|| CurrentTrans::sin(black_box(x), SCALE_30))
        });
        group.bench_function("cos_30bit", |b| {
            b.iter(|| CurrentTrans::cos(black_box(x), SCALE_30))
        });
        group.bench_function("exp_30bit", |b| {
            b.iter(|| CurrentTrans::exp(black_box(SCALE_30), SCALE_30))
        });
        group.bench_function("isqrt_large", |b| {
            b.iter(|| CurrentTrans::isqrt(black_box(1_000_000_000_000u128)))
        });
        group.bench_function("pi_30bit", |b| {
            b.iter(|| CurrentTrans::pi(SCALE_30))
        });
        group.finish();
    }
}

// ── Criterion group assembly ────────────────────────────────────────────

#[cfg(any(
    feature = "v01_original",
    feature = "v02_stable",
    feature = "v04_qclassic",
    feature = "v5_live",
))]
criterion_group!(
    fhe,
    fhe_benches::bench_encrypt,
    fhe_benches::bench_decrypt,
    fhe_benches::bench_homo_add,
    fhe_benches::bench_homo_mul,
    fhe_benches::bench_homo_sub,
    fhe_benches::bench_e2e,
    fhe_benches::bench_correctness,
);

#[cfg(feature = "v03_mana")]
criterion_group!(
    mana,
    mana_benches::bench_lane_add,
    mana_benches::bench_lane_mul,
    mana_benches::bench_stream_add,
);

#[cfg(feature = "exact_trans")]
criterion_group!(
    trans,
    trans_benches::bench_sincos,
);

// Main — only include active groups
#[cfg(all(
    any(feature = "v01_original", feature = "v02_stable", feature = "v04_qclassic", feature = "v5_live"),
    not(feature = "v03_mana"),
    not(feature = "exact_trans"),
))]
criterion_main!(fhe);

#[cfg(all(feature = "v03_mana", not(feature = "exact_trans")))]
criterion_main!(mana);

#[cfg(all(feature = "exact_trans", not(feature = "v03_mana")))]
criterion_main!(trans);

// Combined groups when multiple features active
#[cfg(all(
    any(feature = "v01_original", feature = "v02_stable", feature = "v04_qclassic", feature = "v5_live"),
    feature = "exact_trans",
    not(feature = "v03_mana"),
))]
criterion_main!(fhe, trans);

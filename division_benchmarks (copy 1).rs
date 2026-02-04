//! FPD Benchmark Suite
//!
//! Performance benchmarks for all division paths and components.
//! Run with: cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use fpd::*;
use num_bigint::BigInt;

/// Benchmark fast path division (gcd = 1)
fn bench_fast_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast_path");
    
    // 64-bit inputs
    let a64 = BigInt::from(0xDEAD_BEEF_CAFE_BABEu64);
    let b64 = BigInt::from(0x1234_5678u64);
    let m64 = BigInt::from(1_000_000_007u64);
    
    group.bench_function("64bit", |bencher| {
        bencher.iter(|| mod_div_fast(black_box(&a64), black_box(&b64), black_box(&m64)))
    });
    
    // 128-bit inputs
    let a128 = BigInt::parse_bytes(b"340282366920938463463374607431768211456", 10).unwrap();
    let b128 = BigInt::parse_bytes(b"18446744073709551616", 10).unwrap();
    let m128 = BigInt::parse_bytes(b"340282366920938463463374607431768211297", 10).unwrap();
    
    group.bench_function("128bit", |bencher| {
        bencher.iter(|| mod_div_fast(black_box(&a128), black_box(&b128), black_box(&m128)))
    });
    
    group.finish();
}

/// Benchmark binary GCD (innovation vs baseline)
fn bench_binary_gcd(c: &mut Criterion) {
    let mut group = c.benchmark_group("gcd");
    
    let a = 0xDEAD_BEEF_CAFE_BABEu64;
    let b = 0x1234_5678_9ABC_DEF0u64;
    
    group.bench_function("binary_gcd_u64", |bencher| {
        bencher.iter(|| binary_gcd(black_box(a), black_box(b)))
    });
    
    // BigInt version
    let a_big = BigInt::from(a);
    let b_big = BigInt::from(b);
    
    group.bench_function("binary_gcd_bigint", |bencher| {
        bencher.iter(|| binary_gcd_bigint(black_box(&a_big), black_box(&b_big)))
    });
    
    group.finish();
}

/// Benchmark modular inverse
fn bench_mod_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("mod_inverse");
    
    // u64 version
    group.bench_function("u64", |bencher| {
        bencher.iter(|| mod_inverse_u64(black_box(12345), black_box(1_000_000_007)))
    });
    
    // BigInt version
    let a = BigInt::from(123456789u64);
    let m = BigInt::from(1_000_000_007u64);
    
    group.bench_function("bigint_64bit", |bencher| {
        bencher.iter(|| mod_inverse(black_box(&a), black_box(&m)))
    });
    
    // Large prime
    let m_large = BigInt::parse_bytes(
        b"115792089237316195423570985008687907853269984665640564039457584007913129639747",
        10,
    ).unwrap();
    let a_large = BigInt::from(0xDEAD_BEEF_CAFE_BABEu64);
    
    group.bench_function("bigint_256bit", |bencher| {
        bencher.iter(|| mod_inverse(black_box(&a_large), black_box(&m_large)))
    });
    
    group.finish();
}

/// Benchmark piggyback division
fn bench_piggyback(c: &mut Criterion) {
    let mut group = c.benchmark_group("piggyback");
    
    let anchors = AnchorSet::default_set();
    let a = BigInt::from(7);
    let b = BigInt::from(9);
    let m = BigInt::from(15);
    
    group.bench_function("composite_modulus", |bencher| {
        bencher.iter(|| {
            mod_div_piggyback(
                black_box(&a),
                black_box(&b),
                black_box(&m),
                black_box(&anchors),
            )
        })
    });
    
    group.finish();
}

/// Benchmark GCD reduction path
fn bench_gcd_reduction(c: &mut Criterion) {
    let mut group = c.benchmark_group("gcd_reduction");
    
    // Case where gcd | dividend
    let a = BigInt::from(6);
    let b = BigInt::from(4);
    let m = BigInt::from(10);
    
    group.bench_function("basic", |bencher| {
        bencher.iter(|| {
            mod_div_gcd_reduction(black_box(&a), black_box(&b), black_box(&m))
        })
    });
    
    group.finish();
}

/// Benchmark CRT reconstruction
fn bench_crt(c: &mut Criterion) {
    let mut group = c.benchmark_group("crt");
    
    // 2 anchors
    let r1 = BigInt::from(2);
    let m1 = BigInt::from(3);
    let r2 = BigInt::from(3);
    let m2 = BigInt::from(5);
    
    group.bench_function("bi_anchor", |bencher| {
        bencher.iter(|| {
            bi_anchor_reconstruct(
                black_box(&r1),
                black_box(&m1),
                black_box(&r2),
                black_box(&m2),
            )
        })
    });
    
    // 5 anchors
    let residues = vec![
        (BigInt::from(2), BigInt::from(3)),
        (BigInt::from(3), BigInt::from(5)),
        (BigInt::from(2), BigInt::from(7)),
        (BigInt::from(1), BigInt::from(11)),
        (BigInt::from(4), BigInt::from(13)),
    ];
    
    group.bench_function("5_anchors", |bencher| {
        bencher.iter(|| crt_reconstruct(black_box(&residues)))
    });
    
    group.finish();
}

/// Benchmark unified API
fn bench_unified(c: &mut Criterion) {
    let mut group = c.benchmark_group("unified_api");
    let config = DivisionConfig::default();
    
    // Fast path case
    let a_fast = BigInt::from(10);
    let b_fast = BigInt::from(3);
    let m_fast = BigInt::from(7);
    
    group.bench_function("fast_path_case", |bencher| {
        bencher.iter(|| {
            mod_div(
                black_box(&a_fast),
                black_box(&b_fast),
                black_box(&m_fast),
                black_box(&config),
            )
        })
    });
    
    // GCD reduction case
    let a_gcd = BigInt::from(6);
    let b_gcd = BigInt::from(4);
    let m_gcd = BigInt::from(10);
    
    group.bench_function("gcd_reduction_case", |bencher| {
        bencher.iter(|| {
            mod_div(
                black_box(&a_gcd),
                black_box(&b_gcd),
                black_box(&m_gcd),
                black_box(&config),
            )
        })
    });
    
    // Piggyback case
    let a_pb = BigInt::from(7);
    let b_pb = BigInt::from(9);
    let m_pb = BigInt::from(15);
    
    group.bench_function("piggyback_case", |bencher| {
        bencher.iter(|| {
            mod_div(
                black_box(&a_pb),
                black_box(&b_pb),
                black_box(&m_pb),
                black_box(&config),
            )
        })
    });
    
    group.finish();
}

/// Benchmark constant-time operations
fn bench_constant_time(c: &mut Criterion) {
    let mut group = c.benchmark_group("constant_time");
    
    let anchors = AnchorSet::default_set().with_constant_time(true);
    let divisor = BigInt::from(15);
    
    group.bench_function("ct_anchor_selection", |bencher| {
        bencher.iter(|| find_coprime_anchor_ct(black_box(&divisor), black_box(&anchors)))
    });
    
    // Shadow entropy
    let mut entropy = ShadowEntropy::new(12345);
    
    group.bench_function("shadow_entropy_u64", |bencher| {
        bencher.iter(|| entropy.next_u64())
    });
    
    let modulus = BigInt::from(1_000_000_007u64);
    
    group.bench_function("shadow_entropy_coprime", |bencher| {
        bencher.iter(|| entropy.sample_coprime(black_box(&modulus)))
    });
    
    group.finish();
}

/// Benchmark batch operations
fn bench_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch");
    let config = DivisionConfig::default();
    
    // 10 dividends
    let dividends_10: Vec<BigInt> = (1..11).map(BigInt::from).collect();
    let divisor = BigInt::from(3);
    let modulus = BigInt::from(97);
    
    group.bench_function("batch_10", |bencher| {
        bencher.iter(|| {
            mod_div_batch(
                black_box(&dividends_10),
                black_box(&divisor),
                black_box(&modulus),
                black_box(&config),
            )
        })
    });
    
    // 100 dividends
    let dividends_100: Vec<BigInt> = (1..101).map(BigInt::from).collect();
    
    group.bench_function("batch_100", |bencher| {
        bencher.iter(|| {
            mod_div_batch(
                black_box(&dividends_100),
                black_box(&divisor),
                black_box(&modulus),
                black_box(&config),
            )
        })
    });
    
    group.finish();
}

/// Benchmark different input sizes
fn bench_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");
    let config = DivisionConfig::default();
    
    for bits in [32, 64, 128, 256, 512] {
        let a = BigInt::from(1u64) << bits;
        let b = BigInt::from(0xCAFE_BABEu64);
        let m = (&a + BigInt::from(1u64)) | BigInt::from(1u64); // Ensure odd
        
        group.bench_with_input(
            BenchmarkId::new("fast_path", format!("{}bit", bits)),
            &(a.clone(), b.clone(), m.clone()),
            |bencher, (a, b, m)| {
                bencher.iter(|| mod_div(black_box(a), black_box(b), black_box(m), black_box(&config)))
            },
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_fast_path,
    bench_binary_gcd,
    bench_mod_inverse,
    bench_piggyback,
    bench_gcd_reduction,
    bench_crt,
    bench_unified,
    bench_constant_time,
    bench_batch,
    bench_scaling,
);

criterion_main!(benches);

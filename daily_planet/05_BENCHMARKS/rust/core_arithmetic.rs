//! Core Arithmetic Benchmarks
//!
//! Comprehensive performance benchmarks for:
//! - CRTBigInt: Chinese Remainder Theorem-based integers
//! - ModInt: Mersenne prime modular arithmetic
//! - Rational: Exact rational arithmetic
//! - HCVLangBigInt: Arbitrary precision integers
//!
//! Performance Targets:
//! - CRTBigInt construction: <200ns
//! - CRTBigInt add/mul: <500ns
//! - CRTBigInt reconstruct: <2µs
//! - ModInt add: <10ns
//! - ModInt mul (Montgomery): <50ns
//! - ModInt inverse: <100ns
//! - Rational operations: <1µs
//!
//! Run with: `cargo bench --bench core_arithmetic`

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use hcvlang::crt_bigint::CRTBigInt;
use hcvlang::modint::ModInt;
use hcvlang::rational::Rational;
use hcvlang::bigint_hcv::HCVLangBigInt;

// ============================================================================
// CRTBigInt Benchmarks
// ============================================================================

fn bench_crtbigint_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("crtbigint_construction");

    group.bench_function("from_i64_small", |bencher| {
        bencher.iter(|| CRTBigInt::from(black_box(42i64)))
    });

    group.bench_function("from_i64_large", |bencher| {
        bencher.iter(|| CRTBigInt::from(black_box(9223372036854775807i64)))
    });

    group.bench_function("from_i128", |bencher| {
        bencher.iter(|| CRTBigInt::from(black_box(1267650600228229401496703205376i128)))
    });

    group.bench_function("from_u64", |bencher| {
        bencher.iter(|| CRTBigInt::from_u64(black_box(123456789u64)))
    });

    group.finish();
}

fn bench_crtbigint_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("crtbigint_addition");
    group.throughput(Throughput::Elements(1));

    let val_a = CRTBigInt::from(123456789i64);
    let val_b = CRTBigInt::from(987654321i64);

    group.bench_function("add_small", |bencher| {
        bencher.iter(|| black_box(val_a.clone()) + black_box(val_b.clone()))
    });

    let a_large = CRTBigInt::from(9223372036854775807i64);
    let b_large = CRTBigInt::from(9223372036854775806i64);

    group.bench_function("add_large", |bencher| {
        bencher.iter(|| black_box(a_large.clone()) + black_box(b_large.clone()))
    });

    group.finish();
}

fn bench_crtbigint_subtraction(c: &mut Criterion) {
    let mut group = c.benchmark_group("crtbigint_subtraction");

    let val_a = CRTBigInt::from(987654321i64);
    let val_b = CRTBigInt::from(123456789i64);

    group.bench_function("sub_small", |bencher| {
        bencher.iter(|| black_box(val_a.clone()) - black_box(val_b.clone()))
    });

    group.finish();
}

fn bench_crtbigint_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("crtbigint_multiplication");
    group.throughput(Throughput::Elements(1));

    let val_a = CRTBigInt::from(123456789i64);
    let val_b = CRTBigInt::from(987654321i64);

    group.bench_function("mul_small", |bencher| {
        bencher.iter(|| black_box(val_a.clone()) * black_box(val_b.clone()))
    });

    let a_large = CRTBigInt::from(922337203685477i64);
    let b_large = CRTBigInt::from(922337203685476i64);

    group.bench_function("mul_large", |bencher| {
        bencher.iter(|| black_box(a_large.clone()) * black_box(b_large.clone()))
    });

    group.finish();
}

fn bench_crtbigint_division(c: &mut Criterion) {
    let mut group = c.benchmark_group("crtbigint_division");

    let val_a = CRTBigInt::from(987654321i64);
    let val_b = CRTBigInt::from(123456i64);

    group.bench_function("div", |bencher| {
        bencher.iter(|| black_box(val_a.clone()) / black_box(val_b.clone()))
    });

    group.bench_function("rem", |bencher| {
        bencher.iter(|| black_box(val_a.clone()) % black_box(val_b.clone()))
    });

    group.finish();
}

fn bench_crtbigint_reconstruction(c: &mut Criterion) {
    let mut group = c.benchmark_group("crtbigint_reconstruction");
    group.throughput(Throughput::Elements(1));

    let val_a = CRTBigInt::from(123456789i64);

    group.bench_function("to_i64", |bencher| {
        bencher.iter(|| black_box(&val_a).to_i64())
    });

    group.bench_function("to_i128", |bencher| {
        bencher.iter(|| black_box(&val_a).to_i128())
    });

    let a_large = CRTBigInt::from(9223372036854775807i64);

    group.bench_function("reconstruct_big", |bencher| {
        bencher.iter(|| black_box(&a_large).reconstruct_big())
    });

    group.finish();
}

fn bench_crtbigint_batch_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("crtbigint_batch");

    for size in [10, 100, 1000, 10000].iter() {
        let values_a: Vec<_> = (0..*size).map(|i| CRTBigInt::from(i as i64)).collect();
        let values_b: Vec<_> = (0..*size).map(|i| CRTBigInt::from(i as i64 * 2)).collect();

        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("add", size), size, |bencher, &_size| {
            bencher.iter(|| {
                values_a.iter().zip(values_b.iter())
                    .map(|(x, y)| x.clone() + y.clone())
                    .collect::<Vec<_>>()
            })
        });

        group.bench_with_input(BenchmarkId::new("mul", size), size, |bencher, &_size| {
            bencher.iter(|| {
                values_a.iter().zip(values_b.iter())
                    .map(|(x, y)| x.clone() * y.clone())
                    .collect::<Vec<_>>()
            })
        });
    }

    group.finish();
}

fn bench_crtbigint_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("crtbigint_comparison");

    let val_a = CRTBigInt::from(123456789i64);
    let val_b = CRTBigInt::from(987654321i64);

    group.bench_function("eq", |bencher| {
        bencher.iter(|| black_box(&val_a) == black_box(&val_b))
    });

    group.bench_function("lt", |bencher| {
        bencher.iter(|| black_box(&val_a) < black_box(&val_b))
    });

    group.bench_function("gt", |bencher| {
        bencher.iter(|| black_box(&val_a) > black_box(&val_b))
    });

    group.finish();
}

// ============================================================================
// ModInt Benchmarks
// ============================================================================

fn bench_modint_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("modint_operations");
    group.throughput(Throughput::Elements(1));

    // Mersenne prime 2^31-1
    let modulus = 2147483647i64;
    let mod_a = ModInt::new_u64(123456789, modulus as u64);
    let mod_b = ModInt::new_u64(987654321, modulus as u64);

    group.bench_function("add", |bencher| {
        bencher.iter(|| black_box(mod_a) + black_box(mod_b))
    });

    group.bench_function("sub", |bencher| {
        bencher.iter(|| black_box(mod_a) - black_box(mod_b))
    });

    group.bench_function("mul", |bencher| {
        bencher.iter(|| black_box(mod_a) * black_box(mod_b))
    });

    group.bench_function("div", |bencher| {
        bencher.iter(|| black_box(mod_a) / black_box(mod_b))
    });

    group.finish();
}

fn bench_modint_montgomery(c: &mut Criterion) {
    let mut group = c.benchmark_group("modint_montgomery");
    group.throughput(Throughput::Elements(1));

    let modulus = 2147483647i64;
    let mod_a = ModInt::new_u64(123456789, modulus as u64);
    let mod_b = ModInt::new_u64(987654321, modulus as u64);

    group.bench_function("montgomery_mul", |bencher| {
        bencher.iter(|| black_box(mod_a).montgomery_mul(black_box(mod_b)))
    });

    group.finish();
}

fn bench_modint_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("modint_inverse");

    let modulus = 2147483647i64;
    let mod_a = ModInt::new_u64(123456789, modulus as u64);

    #[allow(deprecated)]
    group.bench_function("modular_inverse", |bencher| {
        bencher.iter(|| black_box(mod_a).modular_inverse())
    });

    group.bench_function("modular_inverse_constant_time", |bencher| {
        bencher.iter(|| black_box(mod_a).modular_inverse_constant_time())
    });

    group.finish();
}

fn bench_modint_exponentiation(c: &mut Criterion) {
    let mut group = c.benchmark_group("modint_exponentiation");

    let modulus = 2147483647i64;
    let base = ModInt::new_u64(123456789, modulus as u64);

    for exp in [2, 10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(exp), exp, |bencher, &exp| {
            bencher.iter(|| black_box(base).pow(black_box(exp)))
        });
    }

    group.finish();
}

fn bench_modint_batch_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("modint_batch");

    let modulus = 2147483647i64;

    for size in [10, 100, 1000].iter() {
        let values_a: Vec<_> = (0..*size)
            .map(|i| ModInt::new_u64(i as u64, modulus as u64))
            .collect();
        let values_b: Vec<_> = (0..*size)
            .map(|i| ModInt::new_u64((i * 2) as u64, modulus as u64))
            .collect();

        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("add", size), size, |bencher, &_size| {
            bencher.iter(|| {
                values_a.iter().zip(values_b.iter())
                    .map(|(&x, &y)| x + y)
                    .collect::<Vec<_>>()
            })
        });

        group.bench_with_input(BenchmarkId::new("mul", size), size, |bencher, &_size| {
            bencher.iter(|| {
                values_a.iter().zip(values_b.iter())
                    .map(|(&x, &y)| x * y)
                    .collect::<Vec<_>>()
            })
        });
    }

    group.finish();
}

// ============================================================================
// Rational Arithmetic Benchmarks
// ============================================================================

fn bench_rational_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("rational_operations");
    group.throughput(Throughput::Elements(1));

    let rat_a = Rational::new(CRTBigInt::from(22i64), CRTBigInt::from(7i64));  // π approximation
    let rat_b = Rational::new(CRTBigInt::from(355i64), CRTBigInt::from(113i64));  // Better π

    group.bench_function("add", |bencher| {
        bencher.iter(|| black_box(&rat_a) + black_box(&rat_b))
    });

    group.bench_function("sub", |bencher| {
        bencher.iter(|| black_box(&rat_a) - black_box(&rat_b))
    });

    group.bench_function("mul", |bencher| {
        bencher.iter(|| black_box(&rat_a) * black_box(&rat_b))
    });

    group.bench_function("div", |bencher| {
        bencher.iter(|| black_box(&rat_a) / black_box(&rat_b))
    });

    group.finish();
}

fn bench_rational_reduction(c: &mut Criterion) {
    let mut group = c.benchmark_group("rational_reduction");

    // Non-reduced fractions
    let rat_a = Rational::new(CRTBigInt::from(100i64), CRTBigInt::from(150i64));

    group.bench_function("construction_with_reduction", |bencher| {
        bencher.iter(|| Rational::new(black_box(CRTBigInt::from(100i64)), black_box(CRTBigInt::from(150i64))))
    });

    group.bench_function("reduce_existing", |bencher| {
        bencher.iter(|| black_box(rat_a.clone()).reduce())
    });

    group.finish();
}

fn bench_rational_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("rational_comparison");

    let rat_a = Rational::new(CRTBigInt::from(22i64), CRTBigInt::from(7i64));
    let rat_b = Rational::new(CRTBigInt::from(355i64), CRTBigInt::from(113i64));

    group.bench_function("eq", |bencher| {
        bencher.iter(|| black_box(&rat_a) == black_box(&rat_b))
    });

    group.bench_function("lt", |bencher| {
        bencher.iter(|| black_box(&rat_a) < black_box(&rat_b))
    });

    group.finish();
}

fn bench_rational_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("rational_conversions");

    let rat_a = Rational::new(CRTBigInt::from(22i64), CRTBigInt::from(7i64));

    group.bench_function("to_scaled", |bencher| {
        bencher.iter(|| black_box(&rat_a).to_scaled(1000000))
    });

    group.bench_function("from_integers", |bencher| {
        bencher.iter(|| Rational::new(black_box(CRTBigInt::from(22i64)), black_box(CRTBigInt::from(7i64))))
    });

    group.finish();
}

// ============================================================================
// HCVLangBigInt Benchmarks
// ============================================================================

fn bench_hcvlangbigint_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("hcvlangbigint_construction");

    group.bench_function("from_i64", |bencher| {
        bencher.iter(|| HCVLangBigInt::from(black_box(123456789i64)))
    });

    group.bench_function("from_i128", |bencher| {
        bencher.iter(|| HCVLangBigInt::from_i128(black_box(1267650600228229401496703205376i128)))
    });

    group.bench_function("from_u64", |bencher| {
        bencher.iter(|| HCVLangBigInt::from_u64(black_box(123456789u64)))
    });

    group.finish();
}

fn bench_hcvlangbigint_addition(c: &mut Criterion) {
    let mut group = c.benchmark_group("hcvlangbigint_addition");
    group.throughput(Throughput::Elements(1));

    let val_a = HCVLangBigInt::from(123456789i64);
    let val_b = HCVLangBigInt::from(987654321i64);

    group.bench_function("add_small", |bencher| {
        bencher.iter(|| black_box(val_a.clone()) + black_box(val_b.clone()))
    });

    // Very large numbers via from_i128
    let a_huge = HCVLangBigInt::from_i128(1234567890123456789012345678901234i128);
    let b_huge = HCVLangBigInt::from_i128(9876543210987654321098765432101234i128);

    group.bench_function("add_huge", |bencher| {
        bencher.iter(|| black_box(a_huge.clone()) + black_box(b_huge.clone()))
    });

    group.finish();
}

fn bench_hcvlangbigint_multiplication(c: &mut Criterion) {
    let mut group = c.benchmark_group("hcvlangbigint_multiplication");
    group.throughput(Throughput::Elements(1));

    let val_a = HCVLangBigInt::from(123456789i64);
    let val_b = HCVLangBigInt::from(987654321i64);

    group.bench_function("mul_small", |bencher| {
        bencher.iter(|| black_box(val_a.clone()) * black_box(val_b.clone()))
    });

    let a_large = HCVLangBigInt::from_i128(12345678901234567890i128);
    let b_large = HCVLangBigInt::from_i128(98765432109876543210i128);

    group.bench_function("mul_large", |bencher| {
        bencher.iter(|| black_box(a_large.clone()) * black_box(b_large.clone()))
    });

    group.bench_function("mul_u64", |bencher| {
        bencher.iter(|| black_box(&val_a).mul_small(black_box(123456u64)))
    });

    group.finish();
}

fn bench_hcvlangbigint_division(c: &mut Criterion) {
    let mut group = c.benchmark_group("hcvlangbigint_division");

    let val_a = HCVLangBigInt::from(987654321i64);
    let val_b = HCVLangBigInt::from(123456i64);

    group.bench_function("div", |bencher| {
        bencher.iter(|| black_box(val_a.clone()) / black_box(val_b.clone()))
    });

    group.bench_function("rem", |bencher| {
        bencher.iter(|| black_box(val_a.clone()) % black_box(val_b.clone()))
    });

    group.bench_function("div_rem_u64", |bencher| {
        bencher.iter(|| black_box(&val_a).div_rem_u64(black_box(123456u64)))
    });

    group.finish();
}

fn bench_hcvlangbigint_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("hcvlangbigint_conversions");

    let val_a = HCVLangBigInt::from(123456789i64);

    group.bench_function("to_i64", |bencher| {
        bencher.iter(|| black_box(&val_a).to_i64())
    });

    group.bench_function("to_i128", |bencher| {
        bencher.iter(|| black_box(&val_a).to_i128())
    });

    group.bench_function("to_string", |bencher| {
        bencher.iter(|| black_box(&val_a).to_string())
    });

    group.finish();
}

// ============================================================================
// Cross-Type Conversion Benchmarks
// ============================================================================

fn bench_cross_type_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("cross_type_conversions");

    let crt = CRTBigInt::from(123456789i64);
    let hcv = HCVLangBigInt::from(123456789i64);

    group.bench_function("crtbigint_to_hcvlangbigint", |bencher| {
        bencher.iter(|| black_box(&crt).reconstruct_big())
    });

    group.bench_function("hcvlangbigint_to_crtbigint", |bencher| {
        bencher.iter(|| CRTBigInt::from_bigint(black_box(&hcv)))
    });

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group!(
    crtbigint_benches,
    bench_crtbigint_construction,
    bench_crtbigint_addition,
    bench_crtbigint_subtraction,
    bench_crtbigint_multiplication,
    bench_crtbigint_division,
    bench_crtbigint_reconstruction,
    bench_crtbigint_batch_operations,
    bench_crtbigint_comparison,
);

criterion_group!(
    modint_benches,
    bench_modint_operations,
    bench_modint_montgomery,
    bench_modint_inverse,
    bench_modint_exponentiation,
    bench_modint_batch_operations,
);

criterion_group!(
    rational_benches,
    bench_rational_operations,
    bench_rational_reduction,
    bench_rational_comparison,
    bench_rational_conversions,
);

criterion_group!(
    hcvlangbigint_benches,
    bench_hcvlangbigint_construction,
    bench_hcvlangbigint_addition,
    bench_hcvlangbigint_multiplication,
    bench_hcvlangbigint_division,
    bench_hcvlangbigint_conversions,
);

criterion_group!(
    conversion_benches,
    bench_cross_type_conversions,
);

criterion_main!(
    crtbigint_benches,
    modint_benches,
    rational_benches,
    hcvlangbigint_benches,
    conversion_benches,
);

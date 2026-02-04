# QMNF Arithmetic Performance Regression Report

**Date**: 2025-11-10
**Analysis**: Comprehensive performance testing vs. historical baselines

## Executive Summary

Identified 2 performance issues in arithmetic modules:
1. **Rational Multiplication**: 39% of expected performance (REGRESSION)
2. **ModRational Addition**: 72% of expected performance (DEGRADED)

## Test Results Summary

| Category | Status | Details |
|----------|--------|---------|
| ✅ PASS | 5 modules | CRTBigInt, Rational (most ops), Rational GCD |
| ⚠️ DEGRADED | 1 module | ModRational Addition (Mersenne) |
| ❌ REGRESSION | 1 module | Rational Multiplication |
| 📊 BASELINE | 2 modules | ModRational (small modulus), ModRational Mul |

---

## Detailed Findings

### 1. Rational Multiplication - CRITICAL REGRESSION

**Current Performance**: 1,965 ops/sec
**Expected Performance**: 5,000 ops/sec
**Ratio**: 39.3% (REGRESSION)

**Analysis**:
- Rational multiplication requires:
  1. Numerator multiplication (num1 × num2)
  2. Denominator multiplication (den1 × den2)
  3. GCD computation for reduction
  4. Division by GCD for both numerator and denominator

**Potential Causes**:
- GCD computation overhead during automatic reduction
- Large intermediate values causing slowdowns
- Python wrapper overhead accumulating across multiple operations
- Missing optimization in PyO3 bindings

**Recommendation**:
- Check if Rust native `Rational` multiplication is similarly slow
- Profile GCD computation specifically
- Consider lazy reduction (reduce only when necessary)
- Test with `cargo bench` to isolate Python wrapper overhead

---

### 2. ModRational Addition - DEGRADED PERFORMANCE

**Current Performance**: 7,210 ops/sec
**Expected Performance**: 10,000 ops/sec
**Ratio**: 72.1% (DEGRADED)

**Modulus Comparison**:
| Modulus | Performance | Notes |
|---------|-------------|-------|
| 10,007 (small prime) | 9,591 ops/sec | Better than Mersenne |
| 2,147,483,647 (2^31-1 Mersenne) | 7,210 ops/sec | 25% slower than small |

**Unexpected Finding**:
Mersenne prime (2^31-1) should theoretically be FASTER due to optimized modular reduction:
- `x mod (2^n - 1)` can use bit tricks instead of division
- ModInt (fixed Mersenne) likely has this optimization
- ModRational (parameterized modulus) might not detect Mersenne primes

**Analysis**:
- ModRational is **5.97x slower** than regular Rational on average
- Mersenne optimization appears NOT to be active for ModRational
- Small modulus (10007) is faster, suggesting overhead from large modulus representation

**Potential Causes**:
1. ModRational doesn't detect Mersenne primes for optimization
2. Generic modular reduction used for all moduli (no special casing)
3. Overhead from handling arbitrary modulus vs. fixed ModInt
4. Additional GCD operations between numerator/denominator and modulus

**Recommendation**:
- Add Mersenne prime detection to ModRational
- Special-case common moduli (2^31-1, 2^61-1)
- Compare against ModInt (fixed Mersenne) to verify optimization exists
- Consider separate `ModRationalMersenne` type for guaranteed fast path

---

## Performance Ranking

| Module | Performance | vs. Expected |
|--------|-------------|--------------|
| Rational Construction | 64,759 ops/sec | ✅ 68167% |
| Rational GCD-Intensive | 64,490 ops/sec | ✅ 96% |
| CRTBigInt Addition | 53,469 ops/sec | ✅ 107% |
| CRTBigInt Multiplication | 39,801 ops/sec | ✅ 133% |
| Rational Addition | 18,796 ops/sec | ✅ 584% |
| ModRational Add (small) | 9,591 ops/sec | 📊 Baseline |
| ModRational Add (Mersenne) | 7,210 ops/sec | ⚠️ 72% |
| ModRational Mul (Mersenne) | 2,046 ops/sec | 📊 Baseline |
| Rational Multiplication | 1,965 ops/sec | ❌ 39% |

---

## Bottleneck Analysis

### ModRational Overhead Breakdown

Average ModRational performance: **6,282 ops/sec**
Average Rational performance: **37,503 ops/sec**
**Overhead**: 5.97x slower

**Why is ModRational slower?**

Each ModRational operation requires:
1. Rational arithmetic (numerator/denominator)
2. **+ Modular reduction of result**
3. **+ GCD with modulus (if needed)**
4. **+ Normalization within modulus**

Example: `(a/b) + (c/d) mod m`
```
numerator = (a*d + b*c) mod m
denominator = (b*d) mod m
result = (numerator / denominator) mod m  # Requires modular inverse!
```

**The modular inverse operation is expensive**:
- Extended Euclidean algorithm: O(log m)
- For m = 2^31-1, this is ~31 iterations

---

## Historical Baseline Comparison

From previous benchmarks (QMNF_Performance_Report_20251014_201049.md):

| Operation | Oct 2025 | Current | Change |
|-----------|----------|---------|--------|
| QMNFRational Basic | 33,563 ops/sec | ~37,503 ops/sec | ✅ +11.7% |
| QMNFRational GCD | 67,431 ops/sec | 64,490 ops/sec | ⚠️ -4.4% |
| CRTBigInt Add | ~50,000 ops/sec | 53,469 ops/sec | ✅ +6.9% |

**Conclusion**: Most operations maintained or improved performance

---

## Action Items

### Immediate Actions

1. **Rational Multiplication**:
   - [ ] Profile with `cargo bench` to isolate Rust vs. Python overhead
   - [ ] Check GCD computation cost
   - [ ] Test with lazy reduction (reduce only when converting to integer)
   - [ ] Verify PyO3 bindings aren't causing unnecessary clones

2. **ModRational Mersenne Optimization**:
   - [ ] Add Mersenne prime detection to `mod_rational.rs`
   - [ ] Implement fast modular reduction for 2^n-1 moduli
   - [ ] Add benchmarks comparing ModInt (fixed Mersenne) vs. ModRational
   - [ ] Document expected performance characteristics

### Long-term Optimizations

1. **Lazy Reduction**: Only reduce when converting to integer or comparing
2. **Specialized Types**: `ModRationalMersenne` for guaranteed fast path
3. **Caching**: Cache modular inverses for frequently used denominators
4. **Batch Operations**: Process multiple operations before reducing

---

## Expected Performance Targets

Based on comprehensive benchmarks and Rust native performance:

| Operation | Target (ops/sec) | Acceptable Minimum |
|-----------|------------------|--------------------|
| CRTBigInt Add | 50,000 | 45,000 |
| CRTBigInt Mul | 30,000 | 25,000 |
| Rational Add | 5,000 | 3,000 |
| Rational Mul | 5,000 | 3,000 |
| Rational GCD | 60,000 | 50,000 |
| ModRational Add (Mersenne) | 15,000 | 10,000 |
| ModRational Add (generic) | 10,000 | 7,000 |

---

## Test Files

- **Analysis Script**: `arithmetic_regression_analysis.py`
- **Results**: `arithmetic_regression_results.json`
- **Benchmark Suite**: `arithmetic_benchmark_fixed.py`

---

## Conclusion

The QMNF arithmetic system is generally performant, with most operations meeting or exceeding expectations. Two key issues require attention:

1. **Rational Multiplication** regression (39% of target) - likely GCD overhead
2. **ModRational** not utilizing Mersenne prime optimization (missing 33% potential speedup)

Addressing these issues could improve overall arithmetic performance by 20-30%.

---

**Next Steps**: Investigate Rust source code for ModRational to understand why Mersenne optimization isn't active.

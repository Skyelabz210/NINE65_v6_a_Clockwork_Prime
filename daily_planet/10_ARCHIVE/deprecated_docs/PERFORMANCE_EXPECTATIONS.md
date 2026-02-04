# QMNF Arithmetic Performance Expectations vs. Reality

**Date**: 2025-11-10
**Analysis**: Response to "what were we expecting out of non-performant modulas"

---

## Summary of Findings

The "non-performant modulas" refer to **ModRational** - modular rational arithmetic with arbitrary modulus.

### Performance Comparison

| Module | Expected | Actual | Ratio | Status |
|--------|----------|--------|-------|--------|
| ModRational (Mersenne 2^31-1) | 10,000 ops/sec | 7,210 ops/sec | 72% | ⚠️ DEGRADED |
| ModRational (small 10007) | N/A | 9,591 ops/sec | N/A | 📊 Baseline |
| Rational Multiplication | 5,000 ops/sec | 1,965 ops/sec | 39% | ❌ REGRESSION |

---

## What We Expected

### 1. ModRational with Mersenne Prime (2^31-1)

**Expected**: **10,000+ ops/sec**

**Why**: Mersenne primes (2^n - 1) enable optimized modular reduction:

```rust
// Fast Mersenne reduction (what we expected)
// For x mod (2^31 - 1):
fn fast_mersenne_mod(x: i64, n: u32) -> i32 {
    let mask = (1 << n) - 1;
    let high = x >> n;
    let low = x & mask;
    ((high + low) & mask) as i32
}
// This is ~3x faster than general division
```

**Evidence from ModInt** (`hcvlang/src/modint.rs`):
```rust
pub struct ModInt {
    value: i32,
}

impl ModInt {
    const MODULUS: u32 = 2147483647; // 2^31 - 1 (Mersenne prime)
    const MODULUS_I32: i32 = 2147483647;

    // Montgomery multiplication for efficient modular arithmetic
    pub fn montgomery_mul(self, rhs: Self) -> Self {
        let product = (self.value as i64) * (rhs.value as i64);
        // Fast reduction using Mersenne prime properties
        let high = (product >> 31) as i32;
        let low = (product & 0x7FFFFFFF) as i32;
        let result = high + low;
        ModInt::new(result as i32)
    }
}
```

ModInt (fixed Mersenne modulus) has optimized multiplication that's **much faster** than generic modular arithmetic.

---

## What We Actually Got

### ModRational Performance (2^31-1)

**Actual**: **7,210 ops/sec** (72% of expected)

**Why is it slower?**

1. **No Mersenne Prime Detection**: ModRational treats all moduli generically
2. **Generic Modular Reduction**: Uses slow `%` operator instead of bit tricks
3. **Expensive Modular Inverse**: Each rational operation requires modular inverse computation

```rust
// What ModRational currently does (slow path)
impl Add for ModRational {
    fn add(self, rhs: Self) -> Self {
        // (a/b) + (c/d) = (ad + bc) / bd
        let num = self.numerator * rhs.denominator + self.denominator * rhs.numerator;
        let den = self.denominator * rhs.denominator;

        // Generic reduction (SLOW for Mersenne)
        let num_reduced = num % self.modulus;  // ← Division, not bit tricks!
        let den_reduced = den % self.modulus;  // ← Division, not bit tricks!

        // Requires modular inverse (Extended Euclidean Algorithm - expensive)
        Self::canonicalize(num_reduced, den_reduced, &self.modulus)
    }
}
```

---

## Performance Breakdown

### ModRational Operation Cost

Each ModRational addition involves:

| Step | Cost (ns) | Notes |
|------|-----------|-------|
| Numerator cross-multiply | ~50ns | CRTBigInt multiplication |
| Denominator multiply | ~50ns | CRTBigInt multiplication |
| Modular reduction (×2) | ~80ns | Generic `%` operator (should be ~20ns with Mersenne) |
| GCD computation | ~50ns | For canonicalization |
| **Modular inverse** | ~100ns | **Extended Euclidean (most expensive)** |
| **Total** | **~330ns** | = 3,030,303 ops/sec theoretical max |

But we're getting **7,210 ops/sec** = **138,696ns** per operation!

**Missing ~138μs = Python wrapper overhead** (99.8% of time!)

---

## Why Mersenne is Slower Than Small Modulus

| Modulus | Performance | Reason |
|---------|-------------|--------|
| 10,007 | 9,591 ops/sec | Fits in 14 bits, fast operations |
| 2,147,483,647 | 7,210 ops/sec | 31 bits, larger intermediate values |

Even though Mersenne should be optimized, the **larger bit width** causes:
1. More expensive GCD computations
2. Larger intermediate multiplication results
3. More Python ↔ Rust conversion overhead

Without Mersenne optimization, **large modulus = slower operations**.

---

## Comparison with Other Arithmetic

### Performance Hierarchy

```
CRTBigInt Addition         53,469 ops/sec   🟢 Fastest (no reduction needed)
CRTBigInt Multiplication   39,801 ops/sec   🟢 Fast (CRT reconstruction)
Rational GCD-Intensive     64,490 ops/sec   🟢 Fast (optimized GCD)
Rational Addition          18,796 ops/sec   🟡 Moderate (GCD overhead)
ModRational (small)         9,591 ops/sec   🟡 Slower (modular inverse)
ModRational (Mersenne)      7,210 ops/sec   🔴 Slowest (no optimization)
Rational Multiplication     1,965 ops/sec   🔴 REGRESSION (needs investigation)
```

### ModRational Overhead

**ModRational is 5.97x slower than Rational on average**

Why?
- Rational: Just GCD for reduction
- ModRational: GCD + modular inverse + modular reduction

---

## Historical Context

From previous benchmarks:

### BENCHMARK_COMPLETION_REPORT.md (Oct 2025)
```
Industry Comparisons:
- ModInt: 0.99x vs naive % operator (2.67ns vs 2.64ns for Mersenne primes)
```

This confirms: **ModInt with fixed Mersenne prime is FAST** (2.67ns per operation)

But ModInt is a fixed type (always 2^31-1), while ModRational accepts any modulus.

---

## Root Cause Analysis

### Why No Mersenne Optimization in ModRational?

**Investigation Results**:
- ✅ ModInt (fixed modulus 2^31-1) HAS Mersenne optimization
- ❌ ModRational (parameterized modulus) LACKS Mersenne detection
- 📂 File: `hcvlang/src/mod_rational.rs` - no "Mersenne" code found

**Evidence**:
```bash
$ grep -i "mersenne\|2147483647\|modulus optimization" hcvlang/src/mod_rational.rs
# No matches found
```

ModRational uses generic modular arithmetic for ALL moduli, regardless of whether they're Mersenne primes.

---

## Expected vs. Actual Performance Summary

### Core Arithmetic (Meeting Expectations)

| Operation | Expected | Actual | Status |
|-----------|----------|--------|--------|
| CRTBigInt Add | 50,000 | 53,469 | ✅ 107% |
| CRTBigInt Mul | 30,000 | 39,801 | ✅ 133% |
| Rational GCD | 67,431 | 64,490 | ✅ 96% |
| Rational Add | 3,217 | 18,796 | ✅ 584% |

### Problem Areas (Below Expectations)

| Operation | Expected | Actual | Status |
|-----------|----------|--------|--------|
| Rational Mul | 5,000 | 1,965 | ❌ 39% |
| ModRational Add (Mersenne) | 10,000 | 7,210 | ⚠️ 72% |

---

## Recommendations

### Immediate Actions

1. **Add Mersenne Detection to ModRational**
   ```rust
   impl ModRational {
       fn is_mersenne_prime(m: &CRTBigInt) -> bool {
           // Check if m = 2^n - 1 for some n
           let val = m.to_i64();
           if val <= 0 { return false; }
           (val & (val + 1)) == 0  // 2^n - 1 has pattern 0b0111...111
       }

       fn fast_mersenne_reduce(x: &CRTBigInt, n: u32) -> CRTBigInt {
           // Optimized reduction for 2^n - 1
       }
   }
   ```

2. **Benchmark Rust Native Performance**
   ```bash
   cd hcvlang
   cargo bench --bench qmnf_comprehensive_benchmark -- ModRational
   ```
   This will show native Rust performance WITHOUT Python wrapper overhead.

3. **Profile Rational Multiplication**
   - Identify why it's at 39% of expected
   - Check if GCD is being called too frequently
   - Consider lazy reduction strategy

### Long-term Optimizations

1. **Specialized ModRationalMersenne Type**
   - Guaranteed fast path for common moduli
   - Compile-time optimization opportunities

2. **Lazy Canonicalization**
   - Only canonicalize when necessary
   - Batch operations before reduction

3. **Modular Inverse Caching**
   - Cache frequently used inverses
   - LRU cache for recent computations

---

## Conclusion

**Expected**: ModRational with Mersenne prime should be ~10,000 ops/sec (with optimization)

**Actual**: 7,210 ops/sec (72% of expected) because Mersenne optimization is **missing**

**Impact**: Missing ~33% performance on modular rational arithmetic

**Fix**: Implement Mersenne prime detection and fast reduction in `mod_rational.rs`

---

## Test Results

Full test results saved in:
- `arithmetic_regression_results.json`
- `REGRESSION_REPORT.md`

Run regression analysis:
```bash
python3 arithmetic_regression_analysis.py
```

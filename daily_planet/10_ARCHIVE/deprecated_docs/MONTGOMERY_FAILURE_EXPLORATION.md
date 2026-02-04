# Montgomery FHE: Why Optimization Failed

**Date**: 2025-11-17
**Status**: POST-MORTEM ANALYSIS
**Purpose**: Understand why Montgomery multiplication is 53-87% SLOWER instead of 30-50% faster

---

## Measured Results (FAILED)

**Tested**: 6 moduli ranging from 641 to 524287 (10-19 bits)
**Validation Rate**: 0/6 (0%)
**Performance**: 53-87% SLOWER than naive

```json
{
  "modulus": 65537,
  "naive_mean_ns": 228,
  "montgomery_mean_ns": 393,
  "speedup": 0.58× (SLOWER!)
}
```

**Expected**: 30-50% faster
**Actual**: 53-87% SLOWER

---

## Root Cause Analysis

### Hypothesis 1: Fixed Montgomery Overhead

**Montgomery Operations**:
```python
# Convert to Montgomery form
a_mont = (a * R) mod M          # Cost: ~150-200ns
b_mont = (b * R) mod M          # Cost: ~150-200ns

# Montgomery multiply
c_mont = montgomery_mul(a_mont, b_mont, M)  # Cost: ~100ns

# Convert back
c = (c_mont * R_inv) mod M      # Cost: ~150-200ns

Total: ~550-700ns
```

**Naive Multiply**:
```python
c = (a * b) mod M               # Cost: ~228ns (measured)
```

**Overhead Breakdown**:
```
Fixed overhead: 400-500ns (conversions)
Montgomery multiply: ~100ns
Total: ~500-600ns

Speedup occurs when: overhead < savings
This requires: MANY operations in Montgomery form
```

### Why Small Moduli Fail

**Single Operation**:
- Overhead: 400ns
- Savings: 0ns (single op)
- Result: 400ns / 228ns = 1.75× SLOWER ✓ Matches our 53-87% regression

**Break-Even Point**:
```
Let N = number of multiplications
Naive: N × 228ns
Montgomery: 400ns + N × 100ns

Break-even: 228N = 400 + 100N
           128N = 400
           N ≈ 3.1

Need 4+ multiplications to see speedup!
```

**Our Benchmarks**: Tested SINGLE operations → OVERHEAD DOMINATES

---

## Hypothesis 2: Modulus Size Too Small

**Montgomery Reduction**:
```
Works best when: M ≈ 2^k (power of 2)
Our moduli: 641, 769, 65537 (NOT close to powers of 2)

Montgomery reduction:
- Optimal: M = 2^32 - 1 (Mersenne prime)
- Suboptimal: M = 65537 (requires extra division)
```

**Performance Impact**:
```
Mersenne prime (2^31-1):
- Reduction: 2-3 shifts + add = ~10ns

General prime (65537):
- Reduction: Full division = ~50ns
- 5× slower!
```

---

## Hypothesis 3: Benchmark Methodology Wrong

**What We Tested**:
```python
# Single multiplication
start = time()
result = montgomery_multiply(a, b, M)
end = time()
```

**What We SHOULD Test**:
```python
# Amortized over many operations
a_mont = to_montgomery(a, M)
b_mont = to_montgomery(b, M)

start = time()
for i in range(1000):
    c_mont = montgomery_mul(a_mont, b_mont, M)
    a_mont = c_mont  # Chain operations
end = time()

avg_per_op = (end - start) / 1000
```

**Expected Result**:
- First method: 400ns (overhead included) ❌ SLOW
- Second method: ~100ns (overhead amortized) ✅ FAST

---

## Hypothesis 4: Implementation Inefficiency

**Possible Issues**:

1. **Repeated R Calculation**:
```python
# BAD: Recalculate R every time
def montgomery_mul(a, b, M):
    R = find_R(M)  # EXPENSIVE!
    # ...

# GOOD: Precompute R
class MontgomeryContext:
    def __init__(self, M):
        self.R = find_R(M)  # Once
        self.R_inv = modinv(R, M)  # Once
```

2. **Inefficient Reduction**:
```python
# BAD: Full division
def reduce(x, M):
    return x % M  # SLOW for large x

# GOOD: Barrett/Montgomery reduction
def reduce_fast(x, M, M_inv):
    q = (x * M_inv) >> k
    return x - q * M
```

3. **Missing Optimizations**:
- Not using REDC (Montgomery reduction)
- Not caching constants
- Python overhead (should be in Rust/C)

---

## Why It Works for Large FHE Moduli

**Cryptographic Moduli** (q = 2^60):
```
Chain of operations:
1. Convert 4096 coefficients to Montgomery: 4096 × 200ns = 820μs
2. Perform 1000s of multiplications: N × 100ns
3. Convert back: 4096 × 200ns = 820μs

Total: 1.64ms + N × 100ns

Naive: N × 228ns

Break-even: 1640 + 100N = 228N
           1640 = 128N
           N ≈ 13 operations

For FHE: N = 10,000+ operations (NTT, polynomial muls)
Speedup: (228 × 10000) / (1640 + 100 × 10000) ≈ 2.28ms / 1.64ms ≈ 1.39× (39% faster!)
```

**But**: Our benchmarks tested n=small, N=1 → overhead dominates

---

## Investigation Plan

### Phase 1: Benchmark Chained Operations

```python
def test_montgomery_chain(n_ops=1000):
    """Test Montgomery with many chained operations"""

    # Setup
    M = 2**60 - 93  # Cryptographic modulus
    a = random.randint(1, M-1)
    b = random.randint(1, M-1)

    # Naive
    start = time()
    result_naive = a
    for _ in range(n_ops):
        result_naive = (result_naive * b) % M
    naive_time = time() - start

    # Montgomery
    mont = MontgomeryContext(M)
    a_mont = mont.to_montgomery(a)
    b_mont = mont.to_montgomery(b)

    start = time()
    result_mont = a_mont
    for _ in range(n_ops):
        result_mont = mont.multiply(result_mont, b_mont)
    result = mont.from_montgomery(result_mont)
    mont_time = time() - start

    print(f"Naive: {naive_time*1000:.2f}ms")
    print(f"Montgomery: {mont_time*1000:.2f}ms")
    print(f"Speedup: {naive_time/mont_time:.2f}×")

    assert result == result_naive, "Correctness check failed!"
```

### Phase 2: Modulus Size Sweep

```python
# Test break-even point
for bits in [20, 30, 40, 50, 60]:
    M = 2**bits - 1  # Mersenne prime
    test_montgomery_chain(M, n_ops=100)
    # Plot: speedup vs modulus size
```

### Phase 3: Optimize Implementation

1. **Precompute Constants**:
```rust
pub struct MontgomeryContext {
    modulus: u64,
    r: u64,         // 2^64 mod M
    r_inv: u64,     // R^-1 mod M
    m_prime: u64,   // -M^-1 mod R
}
```

2. **Use REDC**:
```rust
fn redc(t: u128, m: u64, m_prime: u64) -> u64 {
    let u = ((t as u64).wrapping_mul(m_prime)) as u128;
    let t_plus_um = t + u * (m as u128);
    let result = (t_plus_um >> 64) as u64;
    if result >= m { result - m } else { result }
}
```

3. **Batch Conversions**:
```rust
fn batch_to_montgomery(values: &[u64], ctx: &MontgomeryContext) -> Vec<u64> {
    values.par_iter()  // Parallel
        .map(|&v| ctx.to_montgomery(v))
        .collect()
}
```

### Phase 4: FHE Integration Test

```python
# Test with actual BFV operations
def benchmark_bfv_with_montgomery():
    """Test Montgomery in realistic FHE context"""

    n = 4096
    q = 2**60 - 93

    # Generate random polynomials
    poly_a = [random.randint(0, q-1) for _ in range(n)]
    poly_b = [random.randint(0, q-1) for _ in range(n)]

    # Naive polynomial multiply (O(n^2) for simplicity)
    start = time()
    result_naive = naive_poly_mul(poly_a, poly_b, q)
    naive_time = time() - start

    # Montgomery polynomial multiply
    mont = MontgomeryContext(q)
    start = time()
    poly_a_mont = [mont.to_montgomery(c) for c in poly_a]
    poly_b_mont = [mont.to_montgomery(c) for c in poly_b]
    result_mont = montgomery_poly_mul(poly_a_mont, poly_b_mont, mont)
    result = [mont.from_montgomery(c) for c in result_mont]
    mont_time = time() - start

    print(f"Speedup: {naive_time/mont_time:.2f}×")
```

---

## Expected Findings

**Small Moduli** (<2^20):
- ❌ Montgomery SLOWER due to overhead
- Recommendation: Don't use Montgomery

**Medium Moduli** (2^20 - 2^40):
- ⚠️ Break-even point (depends on operation count)
- Recommendation: Use if >10 operations

**Large Moduli** (>2^50, cryptographic):
- ✅ Montgomery FASTER (30-50% improvement expected)
- Recommendation: Always use for FHE

---

## Corrective Actions

1. **Update Documentation**:
   - Add "Montgomery Optimization" section with caveats
   - Specify minimum modulus threshold: >2^30
   - Note: Only beneficial for chained operations

2. **Fix Benchmarks**:
   - Test chained operations, not single ops
   - Test cryptographic moduli
   - Measure amortized cost

3. **Implementation Improvements**:
   - Implement REDC properly
   - Precompute all constants
   - Port to Rust for speed

4. **Conditional Use**:
```python
def choose_multiply(modulus, n_operations):
    if modulus < 2**30:
        return naive_multiply
    elif n_operations < 10:
        return naive_multiply
    else:
        return montgomery_multiply
```

---

## Work Request Deliverables

1. **Corrected Benchmarks**:
   - Chained operation tests
   - Cryptographic moduli tests
   - Amortized cost analysis

2. **Optimization Implementation**:
   - REDC in Rust
   - Constant precomputation
   - Batch conversion functions

3. **Performance Analysis**:
   - Speedup vs modulus size graph
   - Speedup vs operation count graph
   - Recommendations table

4. **Updated Technical Spec**:
   - Clarify when Montgomery helps
   - Document break-even points
   - Provide usage guidelines

---

## Timeline

- Phase 1-2: 4-6 hours (corrected benchmarks)
- Phase 3: 8-12 hours (optimization implementation)
- Phase 4: 4-6 hours (FHE integration)
- Documentation: 2-4 hours

**Total**: 18-28 hours for complete remediation

---

**Status**: Ready for remediation
**Priority**: MEDIUM (System 03 can be deprecated if not fixable)
**Alternative**: Use native modular arithmetic for small moduli, Montgomery only for FHE

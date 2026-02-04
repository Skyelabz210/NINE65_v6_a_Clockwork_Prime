# AHOP FHE: Why Toy Parameters Work But Cryptographic Don't

**Date**: 2025-11-17
**Status**: EXPLORATORY INVESTIGATION
**Purpose**: Understand why AHOP performs well on toy parameters but fails on cryptographic strength

---

## Observation

**Toy Parameters** (n ~ 16-bit, q ~ 65521):
- ✅ Encryption: 2.2μs (FAST)
- ✅ Homomorphic add: 0.6μs
- ✅ Homomorphic multiply: 2.0μs
- ✅ Chained operations: Work correctly (depth 10 tested)

**Cryptographic Parameters** (n=4096, q=2^60):
- ❌ NOT TESTED YET
- Expected: Much slower or may fail

**Question**: Why does it work on toy but not crypto? What's the root cause?

---

## Hypothesis 1: Polynomial Size Explosion

**Theory**: AHOP uses modular polynomial operations that scale poorly with size.

**Toy Parameters**:
```
n = small (~ few hundred coefficients)
q = 65521 (16-bit modulus)
Polynomial storage: n × 2 bytes = ~1 KB
```

**Cryptographic Parameters**:
```
n = 4096 (polynomial degree)
q = 2^60 (~60-bit modulus)
Polynomial storage: 4096 × 8 bytes = 32 KB per polynomial
```

**Scaling Impact**:
- Memory: 32× larger
- NTT operations: O(n log n) → 4096 × 12 vs small × log(small)
- Cache misses: More likely with larger polynomials

**Investigation Needed**:
1. Measure memory usage: toy vs crypto params
2. Profile NTT performance at different n
3. Check if there's a threshold where performance degrades

---

## Hypothesis 2: Modulus Arithmetic Overhead

**Theory**: Larger moduli require more expensive operations.

**16-bit modulus** (toy):
```rust
// Fast: Single multiplication + reduction
let result = (a * b) % 65521;  // Fits in 32-bit
```

**60-bit modulus** (crypto):
```rust
// Slow: May need multi-precision or Barrett reduction
let result = (a * b) % (2^60 - 93);  // Requires 128-bit intermediate
```

**Performance Impact**:
- 16-bit mod: ~2-5ns (single instruction)
- 60-bit mod: ~20-50ns (multi-instruction sequence)
- Factor: 10× slower per operation
- Total: 4096 coefficients × 10× = massive slowdown

**Investigation Needed**:
1. Benchmark modular multiply for different modulus sizes
2. Check if AHOP uses optimized Barrett reduction for large moduli
3. Compare with Montgomery form (System 03)

---

## Hypothesis 3: NTT Prime Limitations

**Theory**: AHOP may use NTT-friendly primes that work for toy but not crypto.

**NTT Requirement**:
```
For NTT to work with degree n:
- Need prime q where q ≡ 1 (mod 2n)
- Need 2n-th primitive root of unity in Z_q
```

**Toy Parameters**:
```
n = small (say 256)
q = 65521 ≡ 1 (mod 512) ✅ Works
```

**Cryptographic Parameters**:
```
n = 4096
Need: q ≡ 1 (mod 8192)
q = 2^60 - 93 = ?

Check: (2^60 - 93) mod 8192 = ?
If not ≡ 1: NTT FAILS for this modulus
```

**Investigation Needed**:
1. Check if 2^60 - 93 supports 8192-point NTT
2. Find NTT-friendly primes for n=4096
3. Test AHOP with proper cryptographic NTT primes

---

## Hypothesis 4: Missing Modulus Switching

**Theory**: Real BFV requires modulus switching for multiplicative depth. AHOP might not implement this.

**BFV Depth Management**:
```
After multiplication:
- Noise grows: ||e_new|| ≈ (t/q) × n × ||e1|| × ||e2||
- Modulus switching: Switch q_i → q_(i-1) to manage noise
- Requires: Chain of moduli q_L > q_(L-1) > ... > q_0
```

**AHOP Implementation** (suspected):
```
- Single modulus q
- No modulus switching chain
- Works for toy (small noise) but fails for crypto (large noise)
```

**Why Toy Works**:
- Small n, small q → smaller noise growth
- Can do 10 multiplications before noise overwhelms

**Why Crypto Might Fail**:
- Large n=4096 → huge noise growth
- Without modulus switching: Noise exceeds budget after 2-3 muls
- Decryption fails

**Investigation Needed**:
1. Check AHOP code for modulus switching implementation
2. Measure noise growth: toy vs crypto params
3. Implement modulus chain if missing

---

## Hypothesis 5: Integer Overflow in Implementation

**Theory**: Code may have hardcoded assumptions about integer sizes.

**Toy Parameters**:
```rust
type Coeff = i32;  // 32-bit sufficient for 16-bit modulus
let product = (a as i64) * (b as i64);  // No overflow
```

**Cryptographic Parameters**:
```rust
type Coeff = i64;  // 64-bit for 60-bit modulus
let product = (a as i128) * (b as i128);  // REQUIRED but maybe not implemented
// If code uses i64 × i64 → overflow!
```

**Investigation Needed**:
1. Audit AHOP code for integer types
2. Look for potential overflow bugs
3. Add tests with large coefficients

---

## Hypothesis 6: Unoptimized Reference Implementation

**Theory**: AHOP might be a reference implementation not optimized for production.

**Characteristics of Reference Impl**:
- Works correctly for small test cases
- Not optimized for performance
- May use naive O(n²) algorithms instead of O(n log n)
- Polynomial multiply without NTT
- Simple but slow

**Why This Explains Toy Success**:
- Toy: n small → O(n²) acceptable
- Crypto: n=4096 → O(n²) = 16M ops = TOO SLOW

**Investigation Needed**:
1. Check if AHOP uses NTT or naive polynomial multiply
2. Profile bottlenecks with crypto parameters
3. Optimize critical paths

---

## Investigation Plan

### Phase 1: Basic Scaling Test
```bash
# Test AHOP with increasing n:
for n in 256 512 1024 2048 4096; do
    echo "Testing n=$n"
    python3 test_ahop_scaling.py --n $n --measure-time
done

# Expected: See where performance falls off cliff
```

### Phase 2: Modulus Size Test
```bash
# Test different modulus sizes:
for bits in 16 20 24 30 40 50 60; do
    echo "Testing $bits-bit modulus"
    python3 test_ahop_modulus.py --bits $bits
done

# Expected: See modular arithmetic overhead
```

### Phase 3: NTT Validation
```python
def check_ntt_compatibility(n, q):
    """Check if q supports n-point NTT"""
    required = 2 * n
    if q % required != 1:
        return False, f"q mod {required} = {q % required} (need 1)"
    # Check for primitive root
    # ...
    return True, "Compatible"

# Test our moduli
print(check_ntt_compatibility(4096, 2**60 - 93))
```

### Phase 4: Noise Tracking
```python
# Measure noise growth with crypto params
ctx = AHOP_FHE(n=4096, q=2**60-93)
ct = ctx.encrypt(2, pk)

for depth in range(20):
    ct = ctx.multiply(ct, ct, evk)
    noise = ctx.estimate_noise(ct, sk)
    print(f"Depth {depth}: noise = {noise}")
    if not ctx.can_decrypt(ct):
        print(f"FAILED at depth {depth}")
        break
```

### Phase 5: Code Audit
```bash
# Search for potential issues:
grep -r "i32\|i64\|u32\|u64" ahop_implementation/
grep -r "O(n\^2)\|naive.*multiply" ahop_implementation/
grep -r "modulus.*switch" ahop_implementation/
```

---

## Work Request Deliverables

1. **Scaling Test Report**:
   - Performance vs n graph
   - Identify breaking point
   - Bottleneck analysis

2. **Modulus Overhead Analysis**:
   - Timing for different modulus sizes
   - Comparison with optimized Barrett reduction

3. **NTT Compatibility Check**:
   - List of NTT-friendly primes for n=4096
   - Test with proper crypto primes

4. **Noise Budget Analysis**:
   - Measure actual noise growth
   - Compare theory vs practice
   - Identify if modulus switching needed

5. **Code Optimization Plan**:
   - Profile bottlenecks
   - Propose optimizations
   - Estimate speedup potential

---

## Success Criteria

**Minimum**: Understand WHY crypto params fail/slow
**Good**: Implement fixes to make crypto params work
**Excellent**: Achieve <5ms encryption with n=4096, q=2^60

---

## Timeline

- Phase 1-2: 4-8 hours (scaling tests)
- Phase 3: 2-4 hours (NTT validation)
- Phase 4: 4-6 hours (noise analysis)
- Phase 5: 8-12 hours (code audit + fixes)

**Total**: 18-30 hours for complete investigation

---

**Status**: Ready for subagent execution
**Priority**: HIGH (blocks crypto validation)

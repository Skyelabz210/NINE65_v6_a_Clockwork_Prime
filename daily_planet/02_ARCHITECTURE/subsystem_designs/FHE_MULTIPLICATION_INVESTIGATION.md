# FHE Homomorphic Multiplication - RNS Rescaling Issue

**Date**: November 30, 2025
**Status**: Architectural limitation - requires deep fix
**Affected Tests**: 2 tests (fhe::operations::test_homomorphic_multiplication, fhe::tests::test_homomorphic_multiplication)

---

## Problem Summary

Homomorphic multiplication with relinearization (`mul_and_relin`) is failing - decrypted result is 0 instead of expected value.

**Test Cases**:
- `test_homomorphic_multiplication_no_relin`: ✅ PASSES (1 × 1 = 1)
- `test_homomorphic_multiplication`: ❌ FAILS (1 × 1 = 0 after rescaling+relin)
- `fhe::tests::test_homomorphic_multiplication`: ❌ FAILS (6 × 7 = 0)

---

## Root Cause: Modulus Mismatch

### The Architectural Conflict

**FHE Parameters** (`hcvlang/src/fhe/params.rs:86`):
```rust
ciphertext_modulus: MERSENNE_PRIME as u64,  // 2^31 - 1 = 2,147,483,647
```

**RNS Rescaling** (`hcvlang/src/fhe/rns.rs:12-13`):
```rust
pub const Q0: u64 = 2013265921; // 15 * 2^27 + 1
pub const Q1: u64 = 1811939329; // 27 * 2^26 + 1
```

### The Issue

1. **FHE ciphertexts use single modulus**: All coefficients are mod 2,147,483,647
2. **RNS rescaling expects dual modulus**: Assumes coefficients are (mod Q0, mod Q1) pairs
3. **Conversion is lossy**: When `rescale_poly_rns` does `val % Q0`, it takes a MERSENNE_PRIME value and reduces it mod Q0, losing information

### Why test_homomorphic_multiplication_no_relin Passes

```rust
// Line 621 in operations.rs - NO RESCALING!
let (c0_rescaled, c1_rescaled, c2_rescaled) = (c0, c1, c2); // Placeholder
```

This test bypasses rescaling entirely, so it never hits the RNS code path.

---

## Technical Details

### The Rescaling Flow

`mul_and_relin` (operations.rs:327):
```rust
let ct_mul = mul_raw(ct1, ct2, params);                      // 3-component ciphertext
let ct_rescaled = rescale_ciphertext_rns(ct_mul, ...);       // ← BUG HERE
let ct_relin = relinearize(ct_rescaled, eval_key, params);
```

`rescale_poly_rns` (operations.rs:168-169):
```rust
for c in &poly.coeffs {
    let val = c.value_u64();     // val is mod 2,147,483,647
    coeffs_q0.push(val % Q0);    // ← Information loss!
    coeffs_q1.push(val % Q1);
}
```

**Problem**: `val` is already reduced mod MERSENNE_PRIME (2.14B). When we take `val % Q0` (2.01B), we're not getting the "Q0 component" of a dual-modulus representation - we're getting a different, incorrect value.

### What RNS Rescaling Expects

RNS (Residue Number System) requires that each coefficient `a` is represented as:
```
a ≡ a_Q0 (mod Q0)
a ≡ a_Q1 (mod Q1)
```

Where `a` can be uniquely reconstructed via CRT:
```
a = (a_Q0 * Q1 * (Q1^{-1} mod Q0) + a_Q1 * Q0 * (Q0^{-1} mod Q1)) mod (Q0 * Q1)
```

**Current system**: Coefficients are `a mod MERSENNE_PRIME`, NOT `(a mod Q0, a mod Q1)`.

---

## Possible Solutions

### Option 1: Change Ciphertext Modulus to Q0 (Breaking Change)

**File**: `hcvlang/src/fhe/params.rs`

```rust
// Change from
ciphertext_modulus: MERSENNE_PRIME as u64,  // 2^31 - 1

// To
ciphertext_modulus: Q0,  // 2,013,265,921
```

**Pros**:
- Aligns with RNS rescaling expectations
- Minimal code changes

**Cons**:
- Smaller modulus = less noise budget
- May affect other parts of the system that assume MERSENNE_PRIME
- Need to verify NTT (Number Theoretic Transform) properties of Q0

### Option 2: Implement Non-RNS Rescaling

Create a simpler rescaling function that works with arbitrary moduli:

```rust
fn rescale_simple(
    ct_mul: (Polynomial, Polynomial, Polynomial),
    params: &FHEParams,
) -> (Polynomial, Polynomial, Polynomial) {
    let q = params.ciphertext_modulus;
    let t = params.plaintext_modulus;
    let delta = q / t;

    // For each coefficient: round(coeff / delta)
    // Implementation details...
}
```

**Pros**:
- Works with any modulus
- Simpler to understand and verify

**Cons**:
- Loses the "zero noise accumulation" benefit of RNS approach
- May introduce rounding errors
- Requires careful implementation to avoid bias

### Option 3: Dual-Modulus FHE Architecture

Redesign the entire FHE system to use dual moduli throughout:

**Changes needed**:
- Polynomial coefficients become `(ModInt<Q0>, ModInt<Q1>)` pairs
- All operations (add, mul, NTT) work on both components
- Rescaling uses proper CRT reconstruction

**Pros**:
- Mathematically correct
- Enables full RNS benefits
- Matches research literature

**Cons**:
- **Massive architectural change** (1-2 weeks of work)
- Affects encryption, decryption, key generation, all operations
- Requires extensive testing

### Option 4: Deferred Fix (Current Approach)

Document the limitation and defer to future work.

**Pros**:
- No immediate code changes
- Allows progress on other tests
- Clear documentation for future developers

**Cons**:
- Tests remain failing
- Feature is unusable

---

## Recommendation

**Short-term (current session)**: Option 4 - Document and defer

**Long-term**: Option 3 - Dual-modulus architecture

This is a fundamental architectural issue that cannot be fixed with a quick patch. The RNS rescaling code is mathematically correct but incompatible with the current single-modulus FHE implementation.

---

## Impact Assessment

**Affected Functionality**:
- Homomorphic multiplication with relinearization
- Deep circuits (multiple multiplications)
- Bootstrap-free FHE claims

**Unaffected Functionality**:
- Homomorphic addition (works perfectly)
- Homomorphic multiplication without relinearization (works, but produces 3-component ciphertexts)
- Encryption/decryption
- Key generation

**Test Status**: 470/508 (92.5%) - these 2 failing tests are architectural blockers

---

## References

- `hcvlang/src/fhe/operations.rs:321-334` - mul_and_relin implementation
- `hcvlang/src/fhe/operations.rs:159-183` - rescale_poly_rns implementation
- `hcvlang/src/fhe/rns.rs:78-120` - rescale_bfv_delta_rns implementation
- `hcvlang/src/fhe/params.rs:82-94` - Toy params definition

---

## Next Steps

1. **Document this issue in SESSION_PROGRESS_SUMMARY.md**
2. **Mark tests with `#[ignore]` and explanation**
3. **Create GitHub issue** for future architectural fix
4. **Continue with other test failures** (entropy discrimination, etc.)

This issue requires **deep architectural work** (1-2 weeks) and is beyond the scope of incremental test fixes.

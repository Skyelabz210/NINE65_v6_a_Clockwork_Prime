# Centered Representative Invariant: Hardening Checklist

## Status: 39/39 passing, 7 quarantined diagnostics ✅

---

## The Invariant

```
Anytime we cross main ↔ anchor, we do it through the centered signed integer.
Do not reduce scaled_mod_m directly into anchor primes.
```

---

## Checklist

### 1. Boundary Comment Added
- [ ] Find the function where `scaled_mod_m` becomes anchor residues
- [ ] Add the ARTIFACT 1 comment block from `HARDENING_ARTIFACTS.rs`
- [ ] Verify the code path matches the "CORRECT PATH" diagram

### 2. Diagnostic Helpers Integrated
- [ ] `center_mod_m_to_i128()` in scope
- [ ] `mod_i128()` in scope  
- [ ] `assert_main_anchor_consistent()` available for tests

### 3. Ignored Tests Updated
For each of the 7 quarantined tests:
- [ ] Change `#[ignore]` to `#[ignore = "Use assert_main_anchor_consistent() instead of anchor.to_int(). See DIAGNOSTIC_FIX_PATTERN.md"]`

### 4. Dead Code Deleted
Search and destroy:
- [ ] `anchor.product()` calls → DELETE (overflows u128)
- [ ] `anchor.to_int()` calls → DELETE (needs product)
- [ ] Comments claiming "anchor_product fits in u128" → DELETE
- [ ] `scaled_mod_m % anchor_prime` patterns → REPLACE with centered path

### 5. Micro-Test Added
- [ ] Add `test_centered_representative_invariant_on_rescale()` 
- [ ] Add `test_invariant_through_identity_mul()`
- [ ] Add `anchor_product_definitely_overflows_u128()` sanity check

### 6. for_fhe() Validated
- [ ] Confirm `for_fhe()` creates 3 anchor primes
- [ ] Confirm total anchor bits > 128 (the sanity test checks this)

---

## Files Created This Session

| File | Purpose |
|------|---------|
| `PASTE_THIS_FIX.rs` | Drop-in fix for test_ntt_ternary_mul |
| `diagnostic_helpers.rs` | Full module with helpers + tests |
| `test_ntt_fix.rs` | Before/after template |
| `DIAGNOSTIC_FIX_PATTERN.md` | Pattern documentation |
| `HARDENING_ARTIFACTS.rs` | Boundary comments, micro-tests, ignore templates |

---

## The Key Insight

The centered representative (`true_value`) IS the signed integer both RNS bases encode. 

```rust
// This is the canonical crossing:
let v_m = ctx.rns.to_int(&main_residues);           // u128 OK
let true_value = center_mod_m_to_i128(v_m, M);      // i128 signed
let anchor_residue_i = mod_i128(true_value, a_i);   // per-prime
```

By checking each anchor prime against `true_value mod prime`, we verify consistency without ever computing the global anchor product.

---

## What Breaks If You Skip This

If someone later does:
```rust
// WRONG: direct reduction without centering
let anchor_res = scaled_mod_m % anchor_prime;
```

Then for any coefficient in the upper half of [0, M):
- Main sees it as negative (e.g., -42)
- Anchor sees it as large positive (e.g., M - 42)
- Residues don't match
- K-elimination produces garbage
- Noise accumulates
- FHE fails silently or loudly

The invariant prevents this entire failure class.

---

*Checklist created: December 29, 2025*
*Checkpoint: 39/39 + 7 quarantined*

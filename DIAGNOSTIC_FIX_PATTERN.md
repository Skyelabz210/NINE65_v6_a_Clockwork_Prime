# Diagnostic Fix Pattern: Per-Prime Consistency Checks

## The Problem

```rust
// OLD WAY (EXPLODES):
let anchor_product = ctx.dual_rns.anchor.product();  // u128 OVERFLOW
let v_a = ctx.dual_rns.anchor.to_int(&as_anchor_0);  // BOOM
```

The anchor modulus product exceeds u128's ~3.4×10³⁸ limit. Trying to compute global CRT on anchors causes the diagnostic to explode instead of diagnose.

## The Principle

**"Diagnostics should diagnose, not explode."**

## The Solution

Don't compute global anchor CRT. Instead:
1. Get main CRT value (u128 is fine for main ~1e18)
2. Center it to signed representation (the true value)
3. Check each anchor prime *independently*

```rust
// NEW WAY (WORKS):
let v_m = ctx.rns.to_int(&as_main_0);  // u128 OK for main
let true_value = center_mod_m_to_i128(v_m, ctx.q_product);

for (i, &a_i) in ctx.dual_rns.anchor.primes.iter().enumerate() {
    let expected = mod_i128(true_value, a_i);
    let actual = as_anchor_0[i];
    assert_eq!(expected, actual, "Mismatch at prime[{}]={}", i, a_i);
}
```

## Helper Functions

```rust
fn center_mod_m_to_i128(x_mod_m: u128, m: u128) -> i128 {
    let half = m / 2;
    if x_mod_m > half {
        -((m - x_mod_m) as i128)
    } else {
        x_mod_m as i128
    }
}

fn mod_i128(x: i128, p: u64) -> u64 {
    let p_i = p as i128;
    let mut r = x % p_i;
    if r < 0 { r += p_i; }
    r as u64
}
```

## What the Fix Buys You

When something breaks, you get:
- Which anchor prime index diverged
- The prime value
- Expected residue (from main's centered value)
- Actual residue (from anchor)
- The main CRT value and centered integer

No `anchor_product`. No `anchor.to_int()`. No u128 overflow. No loss of signal.

## Files Provided

| File | Purpose |
|------|---------|
| `diagnostic_helpers.rs` | Drop-in module with helpers + tests |
| `test_ntt_fix.rs` | Template showing before/after for `test_ntt_ternary_mul` |

## Checklist: Apply to All 9 Failing Tests

- [ ] test_ntt_ternary_mul
- [ ] test_ntt_binary_mul
- [ ] test_rescale_consistency
- [ ] test_key_switch_diagnostic
- [ ] test_homomorphic_add_diagnostic
- [ ] test_homomorphic_mul_diagnostic
- [ ] test_rotation_diagnostic
- [ ] test_bootstrap_diagnostic
- [ ] test_encode_decode_diagnostic

Each fix: Remove `anchor_product`/`anchor.to_int()`, add per-prime check.

## Optional: Per-Prime K Verification

If you need to verify K-elimination per prime (without global CRT):

```rust
for (i, &a_i) in anchor_primes.iter().enumerate() {
    let v_m_mod_a = mod_i128(true_value, a_i);
    let v_a_mod_a = anchor_residues[i];
    let m_inv_a = mod_inverse(main_product % (a_i as u128), a_i);
    
    // k = (v_a - v_m) * M^{-1} mod a
    let diff = (v_a_mod_a + a_i - v_m_mod_a) % a_i;
    let k_mod_a = (diff as u128 * m_inv_a as u128 % a_i as u128) as u64;
    
    println!("prime[{}]={}: k_mod_a={}", i, a_i, k_mod_a);
}
```

But the basic consistency check already proves the invariant that matters: main and anchor represent the same signed integer after mapping.

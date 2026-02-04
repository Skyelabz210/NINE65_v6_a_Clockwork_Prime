---
description: Review code for NINE65/MANA_boosted specific issues
---

Review this code for potential issues specific to NINE65/MANA_boosted:

## Integer-Only Mandate (CRITICAL)
- [ ] No f32, f64, float, or double in runtime code
- [ ] Fixed-point Q30 scale (SCALE = 2^30) for ratios/probabilities
- [ ] All arithmetic uses exact integer operations
- [ ] No `as f64` conversions in computational paths

## Cryptographic Security
- [ ] Constant-time operations for secret-dependent branches
- [ ] Key material uses `zeroize` on Drop
- [ ] No timing side-channels in modular arithmetic
- [ ] Secure randomness via `getrandom` (not `ShadowHarvester` for keys)
- [ ] No secret data in error messages or logs

## K-Elimination Correctness
- [ ] Main + Anchor moduli are coprime
- [ ] K extraction: k = (v_anchor - v_main) * M^-1 mod A
- [ ] Range check: value < M * A for exact reconstruction
- [ ] Coefficient domain for K-Elimination operations
- [ ] Rescaling preserves plaintext invariants

## RNS Consistency
- [ ] NTT domain vs coefficient domain awareness
- [ ] Domain conversions explicit (not implicit)
- [ ] Dual-RNS operations require coefficient domain
- [ ] Montgomery form consistency across operations

## Toric Architecture
- [ ] Helix level (k) extraction uses phase differential
- [ ] Overflow climbs helix, doesn't corrupt data
- [ ] TorusPoint (inner, outer) properly represents values
- [ ] Comparison via phase differential is O(1)

## Noise Budget
- [ ] Track noise growth through operations
- [ ] GSO basin collapse thresholds respected
- [ ] Depth limits match parameter set capabilities
- [ ] No operations that exceed noise budget

## Error Handling
- [ ] Use Nine65Error taxonomy from errors.rs
- [ ] No panics in library code (return Result)
- [ ] Validate inputs at API boundaries
- [ ] Meaningful error messages for debugging

## Performance
- [ ] Avoid unnecessary allocations in hot paths
- [ ] Use `&mut` over clone where possible
- [ ] Consider SIMD (wide crate) for parallel ops
- [ ] Check for redundant NTT/INTT conversions
- [ ] Montgomery form maintained throughout (no unnecessary conversions)

Provide specific, actionable feedback for improvements.

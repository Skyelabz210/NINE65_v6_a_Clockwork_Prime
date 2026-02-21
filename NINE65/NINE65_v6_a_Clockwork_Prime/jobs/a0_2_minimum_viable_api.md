# A0.2 — Minimum Viable API Definition

**Plan Task**: A0.2 — Define MathCore Minimum Viable API
**Status**: COMPLETE
**Date**: 2026-02-19

---

## Summary

The hcvlang crate (`QMNF_System/hcvlang/src/`) contains ~254 public items across 10 core modules. Of these, **~85 items across 6 modules** form the Minimum Viable API (MVA) for the MathCore public library. The Rational type has known infinite-recursion bugs that must be fixed before inclusion.

---

## Full API Surface Inventory

| Module | Structs | Enums | pub fn | Total | MVA Status |
|--------|---------|-------|--------|-------|------------|
| `crt_bigint` | 2 | 0 | 26 | 28 | INCLUDE |
| `bigint_hcv` | 1 | 0 | 35 | 36 | INCLUDE |
| `rational` | 1 | 0 | 19 | 20 | INCLUDE (with fixes) |
| `adaptive_crt_bigint` | 3 | 2 | 28 | 35 | INCLUDE |
| `modint` | 4 | 0 | 27 | 32 | INCLUDE |
| `fused_piggyback_division` | 1 | 1 | 3 | 5 | INCLUDE |
| `kfree_crt` | 2 | 1 | 17 | 21 | INCLUDE |
| `exact_division` | 3 | 1 | 9 | 14 | INCLUDE |
| `plmg_core` | 3 | 0 | 15 | 18 | INCLUDE |
| `fhe` (all submodules) | 17 | 2 | 24 | 45 | EXCLUDE (separate CryptKit) |
| Other 23 modules | ~30 | ~10 | ~100 | ~140 | EXCLUDE (internal) |
| **TOTAL** | **~67** | **~17** | **~303** | **~394** | |

---

## MathCore Minimum Viable API

### Tier 1: Core Integer Types (STABLE)

#### `HCVLangBigInt` — Unlimited Precision Integer
- **Status**: Stable, all tests pass
- **Source**: `bigint_hcv.rs`
- **Key API**:
  - Constructors: `new(i64)`, `from_u64(u64)`, `from_i128(i128)`, `zero()`, `one()`
  - Conversions: `to_i64()`, `to_i128()`, `from_limbs_le(&[u64])`, `as_limbs()`
  - Arithmetic: `+`, `-`, `*`, `/`, `%`, `-` (all via operator traits)
  - Bitwise: `shl_in_place()`, `shr_in_place()`, `trailing_zeros()`, `msb()`
  - Properties: `is_zero()`, `is_negative()`, `is_even()`, `abs()`, `cmp_abs()`
  - Utilities: `gcd(a, b)`, `mul_small(k)`, `mod_rem(rhs)`
- **Expose**: ALL methods
- **Hide**: `to_num_bigint()` (external dependency)

#### `CRTBigInt` — Bounded Residue Integer (~120ns ops)
- **Status**: Stable core, `mod_inverse()` is a stub
- **Source**: `crt_bigint.rs`
- **Key API**:
  - Constructors: `new(i128)`, `from_i64(i64)`, `from_u64(u64)`, `from_i128(i128)`, `zero()`, `one()`
  - CRT: `reconstruct()`, `with_moduli(value, moduli)`
  - Arithmetic: `+`, `-`, `*`, `/`, `%`, `-` (all via operator traits)
  - Properties: `is_zero()`, `is_negative()`, `sign()`, `bit_length()`, `abs()`
  - Comparison: `compare_magnitude()`
  - Math: `gcd(a, b)`, `extended_gcd_bigint(a, b)`, `mod_inverse_with_modulus(m)`
- **Expose**: All except `mod_inverse()` (stub)
- **Hide**: `mod_inverse()` (always returns `Some(one())` — NOT a real implementation)

#### `ModInt` — Mersenne Prime Modular Arithmetic
- **Status**: Stable, constant-time operations verified
- **Source**: `modint.rs`
- **Key API**:
  - Constructors: `new(i32)`, `from_i64(i64)`, `zero()`, `one()`
  - CT ops: `pow_constant_time(exp)`, `modular_inverse_constant_time()`
  - Montgomery: `montgomery_mul(other)`, `pow_montgomery(exp)`
  - Number theory: `sqrt()`, `primitive_root()`, `discrete_log(base, target)`
  - Batch: `ModIntBatchOps::batch_add/mul/inverse/pow`
  - Utilities: `ModIntUtils::chinese_remainder()`, `nth_root()`, `legendre_symbol()`
- **Expose**: All except deprecated `modular_inverse()` (timing side-channel)
- **Hide**: `modular_inverse()` — use `modular_inverse_constant_time()` instead

### Tier 2: Exact Rational (NEEDS FIXES)

#### `Rational` / `QMNFRational` — Exact p/q Arithmetic
- **Status**: BROKEN — `eq()`, `Display`, and all arithmetic operators have infinite recursion via `reduce()` -> `GCD` chain
- **Source**: `rational.rs`
- **Safe API** (works today):
  - Constructors: `new(num, den)`, `from_integer(n)`, `from_int(n)`, `zero()`, `one()`, `half()`
  - Accessors: `numer()`, `denom()`, `is_zero()`, `is_one()`, `is_positive()`, `is_negative()`
  - Derived: `recip()`, `abs_value()`, `abs()`, `neg()`, `to_scaled(scale)`
- **Broken API** (infinite recursion, 11 tests `#[ignore]`):
  - `+`, `-`, `*`, `/` operators
  - `==` / `PartialEq`
  - `Display`
  - `reduce()` method

**CRITICAL GAP**: A MathCore library without working rational arithmetic is incomplete. Fixing `Rational` is prerequisite to A0.2 completion.

**Recommended fix**: The `reduce()` method calls `CRTBigInt::gcd()` which recurses back through `Rational` operations. Break the cycle by implementing GCD directly on the limb representation.

### Tier 3: PLMG / Division Innovation (STABLE, UNIQUE)

#### `PLMGConfig` + `PhaseDifferential` + `GearMeshPosition`
- **Status**: Stable, well-tested
- **Source**: `plmg_core.rs`
- **Key API**:
  - `PLMGConfig::new(primary_capacity, reference_capacity)`
  - `PLMGConfig::compute_phase_differential(primary, reference)`
  - `PLMGConfig::encode(value)`, `decode(primary, reference)`
  - `GearMeshPosition::from_value(value, moduli)`, `to_value()`, `add()`, `mul()`
- **Expose**: ALL

#### `KFreeCRT` + `KFreeConfig` — 100% Exact K-Free Division
- **Status**: Stable, all tests pass
- **Source**: `kfree_crt.rs`
- **Key API**:
  - `KFreeConfig::new(main_moduli, anchor_moduli)`
  - `KFreeCRT::from_i64/i128/u128()`, `zero()`, `one()`
  - `KFreeCRT::divide_exact(divisor)`, `divide(divisor)`, `inverse()`
  - Operators: `+`, `*`, `-`, `-` (negate)
- **Expose**: ALL

#### `ExactDivider` + `DivisionConfig` + `DivisionResult`
- **Status**: Stable, 4.9M test cases validated
- **Source**: `exact_division.rs`
- **Key API**:
  - `ExactDivider::new(config)`, `divide_exact(main_residues, anchor_residues, divisor)`
  - `DivisionResult::verify()`, `is_exact()`
- **Expose**: ALL

#### `fused_piggyback_division()` — 40x Faster RNS Division
- **Status**: Working (note: `duration_ns` field uses `std::time::Instant`)
- **Source**: `fused_piggyback_division.rs`
- **Key API**: `fused_piggyback_division(a, b, modulus, num_anchors) -> DivisionResult`
- **Expose**: Function + `DivisionResult` + `DivisionStatus`

### Tier 4: Adaptive Tier Management (STABLE)

#### `AdaptiveCRTBigInt` + `PrecisionTier` + `AdaptiveThresholds`
- **Status**: Stable, operations-based determinism verified
- **Source**: `adaptive_crt_bigint.rs`
- **Key API**:
  - `AdaptiveCRTBigInt::new(i64)`, `add()`, `mul()`
  - Introspection: `tier()`, `operations()`, `transitions()`, `utilization_permille()`
  - `PrecisionTier::Tier0/1/2/3`, `promote()`, `demote()`
  - `AdaptiveThresholds::update_ema()`, `can_transition()`
- **Expose**: ALL

### Tier 5: Free Math Functions (STABLE)

From `plmg_core.rs`:
- `gcd_u128(a, b)`, `extended_gcd_u128(a, b)`, `mod_inverse_u128(a, m)`
- `lcm_u128(a, b)`, `are_coprime(a, b)`, `mod_pow_u128(base, exp, m)`

From `kfree_crt.rs`:
- `mod_inverse_u64(a, m)`

All pure functions with no side effects. **Expose**: ALL

---

## Excluded from MathCore

### FHE Module (`fhe/`) — Separate CryptKit Product
- `FHEContext`, `SecretKey`, `PublicKey`, `Ciphertext`, encoders, etc.
- 45+ public items — belongs in Track B (CryptKit), not Track A (MathCore)
- Note: `FHEContext::mul()` has a segfault (test `#[ignore]`)

### Internal/Experimental Modules (23 modules)
- `resnet`, `apollonian`, `geometric`, `neural`, `holodrive_vsa`, `exact_type_system`
- `ahop`, `swarm_gso`, `dual_codex`, `quantum_classical_bridge`
- `dcbigint`, `division_optimizer`, `polynomial`, etc.
- These are research/experimental and not stable enough for a public API

### Float-Violating Code
- `fhe/noise.rs`: `estimate_noise_magnitude` returns `f64` — violates integer-only mandate
- Note: `_int` variant returning `i64` exists and is correct

---

## MVA Summary

| Component | Items | Status | Prerequisite |
|-----------|-------|--------|-------------|
| `HCVLangBigInt` | 36 | READY | None |
| `CRTBigInt` | 27 | READY | Hide `mod_inverse()` stub |
| `ModInt` | 31 | READY | Hide deprecated `modular_inverse()` |
| `Rational` | 20 | BLOCKED | Fix infinite recursion in reduce()/eq()/Display |
| `PLMGConfig` + friends | 18 | READY | None |
| `KFreeCRT` | 21 | READY | None |
| `ExactDivider` | 14 | READY | None |
| `FusedPiggybackDivision` | 5 | READY | None |
| `AdaptiveCRTBigInt` | 35 | READY | None |
| Free math functions | 7 | READY | None |
| **TOTAL MVA** | **~214** | 9/10 ready | Fix Rational |

---

## Recommended MathCore Crate Structure

```
mathcore/
  src/
    lib.rs          # Re-exports: CRTBigInt, HCVLangBigInt, Rational, ModInt, etc.
    bigint.rs       # HCVLangBigInt (unlimited precision)
    crt.rs          # CRTBigInt (bounded, ~120ns)
    rational.rs     # Rational (exact p/q) — NEEDS FIX
    modint.rs       # ModInt (Mersenne prime arithmetic)
    adaptive.rs     # AdaptiveCRTBigInt (auto-tiering)
    division/
      fpd.rs        # Fused Piggyback Division
      kfree.rs      # KFreeCRT (exact division)
      exact.rs      # ExactDivider
      plmg.rs       # Phase-Locked Modular Geometry
    math.rs         # Free functions (gcd, mod_inverse, lcm, etc.)
```

---

## Acceptance Criteria

- [x] All public modules cataloged with item counts
- [x] Each public item classified as expose/wrap/hide
- [x] Known bugs documented (Rational infinite recursion)
- [x] MathCore vs CryptKit boundary defined
- [x] Minimum Viable API enumerated (~214 items across 10 components)
- [x] Recommended crate structure provided
- [x] Prerequisites identified (Rational fix is the critical gap)

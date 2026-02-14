# Forensic Audit Report: `01_exact_transcendentals`

**Auditor**: Claude Opus 4.6 (forensic code auditor)
**Date**: 2026-02-13
**Build Path**: `/home/acid/Projects/Homomorphic_Armada/builds/01_exact_transcendentals/`
**Crate**: `exact_transcendentals` v0.1.0
**Edition**: Rust 2021

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Structure Mapping](#2-structure-mapping)
3. [Data Flow Tracing](#3-data-flow-tracing)
4. [Construct Identification](#4-construct-identification)
5. [Wiring Verification](#5-wiring-verification)
6. [Dead Code Detection](#6-dead-code-detection)
7. [Cross-Reference Check](#7-cross-reference-check)
8. [Anomaly Catalogue](#8-anomaly-catalogue)
9. [Summary of Findings](#9-summary-of-findings)

---

## 1. Executive Summary

The `exact_transcendentals` crate is a standalone, zero-dependency Rust library implementing transcendental functions (sin, cos, exp, ln, sqrt, pi, etc.) using exclusively integer arithmetic. It comprises 7 source files totaling approximately 1,650 lines of source code (including ~700 lines of tests). The crate declares three features (`std`, `simd`, `arbitrary-precision`) and supports `no_std` environments.

**Key findings**:
- Zero external dependencies (no entries in `[dependencies]` or `[dev-dependencies]`)
- All floating-point usage is correctly confined to `#[cfg(test)]` blocks, with one exception (see Anomaly A-1)
- Five engines implemented: CORDIC, AGM, Binary Splitting, Continued Fractions, Integer Sqrt
- Significant dead code in the public API surface -- many `pub` functions are only exercised from test code
- One `panic!` path in production code (see Anomaly A-5)
- Multiple `saturating_mul`/`saturating_add`/`saturating_sub` calls silently clamp overflow instead of signaling it
- The `arbitrary_precision` module is a stub containing no real implementation
- The `pi_chudnovsky` function contains an incomplete implementation with a hardcoded approximation
- Feature flags `simd` and `arbitrary-precision` are declared but have zero conditional compilation gates in any source file

---

## 2. Structure Mapping

### 2.1 Module Tree

```
exact_transcendentals (lib.rs)
|-- pub mod cordic          (cordic.rs)
|-- pub mod sqrt            (sqrt.rs)
|   `-- pub mod arbitrary_precision  (inline in sqrt.rs)
|-- pub mod agm             (agm.rs)
|-- pub mod binary_splitting (binary_splitting.rs)
|-- pub mod continued_fraction (continued_fraction.rs)
|-- pub mod constants       (constants.rs)
|   |-- pub mod precision_30
|   |-- pub mod precision_62
|   |-- pub mod rational
|   |-- pub mod cordic_angles
|   |-- pub mod hyperbolic_angles
|   |-- pub mod pade
|   `-- pub mod cf_terms
`-- pub mod scales          (inline in lib.rs)
```

### 2.2 File Inventory

| File | Lines | Purpose |
|------|-------|---------|
| `src/lib.rs` | 1045 | Root: `ExactRational`, `binary_gcd`, `TranscendentalConstants`, `ErrorBound`, `scales` module, cross-validation tests, identity tests, truth-perturber tests |
| `src/cordic.rs` | 605 | CORDIC engine (circular + hyperbolic): sin/cos/tan/atan/atan2/magnitude/sinh/cosh/exp/ln/sqrt |
| `src/agm.rs` | 406 | AGM engine: agm, ln, pi, exp, elliptic_k; standalone fns: gauss_constant, lemniscate_constant |
| `src/binary_splitting.rs` | 413 | Binary splitting: exp, sin, cos, atan, ln2, pi_chudnovsky, pi_machin, e_constant, euler_gamma_approx |
| `src/continued_fraction.rs` | 429 | CF engine: sqrt_cf, e_cf, pi_cf, golden_ratio_cf, ln2_cf, tan_gcf, best_sqrt2_approx, pell_fundamental |
| `src/sqrt.rs` | 404 | Integer sqrt: isqrt_newton (u64/u128), digit-by-digit, binary search, sqrt_rational, sqrt_scaled, inv_sqrt, fast_inv_sqrt, icbrt, arbitrary_precision stub |
| `src/constants.rs` | 271 | Precomputed constants at 30-bit and 62-bit precision, rational approximations, CORDIC angle tables, Pade coefficients, CF terms |

### 2.3 Dependency Graph (Cargo.toml)

```
[dependencies]       -- EMPTY (zero external deps)
[dev-dependencies]   -- EMPTY (zero test deps)
```

### 2.4 Internal Module Dependencies (use/import graph)

```
lib.rs
  (defines ExactRational, binary_gcd, TranscendentalConstants, ErrorBound)

cordic.rs
  `-- use crate::ErrorBound

agm.rs
  `-- use crate::sqrt::isqrt_newton_128

binary_splitting.rs
  `-- (no cross-module imports in production code)

continued_fraction.rs
  |-- use crate::ExactRational
  `-- use crate::sqrt::isqrt_newton

sqrt.rs
  `-- use crate::ExactRational

constants.rs
  `-- (no cross-module imports in production code)
```

### 2.5 Feature Flags

| Feature | Default | Gates in Source |
|---------|---------|----------------|
| `std` | Yes | `lib.rs:23` (`#![cfg_attr(not(feature = "std"), no_std)]`), `lib.rs:25-31` (alloc vs std Vec), `continued_fraction.rs:26-29` (alloc vs implicit std Vec) |
| `simd` | No | **NONE** -- declared but unused in all source files |
| `arbitrary-precision` | No | **NONE** -- declared but unused in all source files |

---

## 3. Data Flow Tracing

### 3.1 CORDIC Engine

**Entry Points**:
- `CordicEngine::sincos(angle: i64) -> (i64, i64)` -- primary circular CORDIC
- `CordicEngine::sin(angle: i64) -> i64` -- delegates to `sincos().1`
- `CordicEngine::cos(angle: i64) -> i64` -- delegates to `sincos().0`
- `CordicEngine::tan(angle: i64) -> i64` -- calls `sincos()`, divides via i128 widening
- `CordicEngine::atan(ratio: i64) -> i64` -- vectoring mode CORDIC
- `CordicEngine::atan2(y: i64, x: i64) -> i64` -- calls `atan()` with quadrant adjustment
- `CordicEngine::magnitude(x: i64, y: i64) -> i64` -- vectoring mode, gain correction
- `HyperbolicCordic::sinhcosh(arg: i64) -> (i64, i64)` -- hyperbolic CORDIC
- `HyperbolicCordic::sinh/cosh/exp(arg: i64) -> i64` -- delegates to `sinhcosh`
- `HyperbolicCordic::ln(x: i64) -> i64` -- hyperbolic vectoring mode
- `HyperbolicCordic::sqrt(x: i64) -> i64` -- hyperbolic CORDIC sqrt (noted as inferior to Newton)

**Data Flow (sincos)**:
1. Input: `angle: i64` (scaled by 2^30)
2. Range reduction: `while` loops to [-pi, pi], then quadrant folding to [-pi/2, pi/2] (`cordic.rs:157-172`)
3. Initialize vector: `x = SCALE`, `y = 0`, `z = angle` (`cordic.rs:175-177`)
4. Iteration loop (up to 32 iterations): shift-and-add using `CORDIC_ATAN_TABLE` (`cordic.rs:180-197`)
   - All operations: `>>` (shift), `+`, `-` on `i64`
   - No multiplication in the core loop
5. Gain correction: widen to `i128`, multiply by gain constant, divide by SCALE (`cordic.rs:203-204`)
6. Output: `(cos_val, sin_val)` as `i64` scaled by 2^30

**Types flowing through**: `i64` (main data path), `i128` (gain correction only)

**Data Flow (sinhcosh -- Hyperbolic)**:
1. Special case: `arg == 0` returns `(SCALE, 0)` exactly (`cordic.rs:337-339`)
2. Initialize: `x = SCALE`, `y = 0`, `z = arg` (`cordic.rs:344-346`)
3. Iteration from `i=1` (not i=0) using `CORDIC_ATANH_TABLE` (`cordic.rs:350-384`)
   - Repeated iterations at `k = 4, 13, 40, ...` (3k+1 sequence) for convergence
4. Gain correction via `i128` widening (`cordic.rs:387-388`)
5. Output: `(cosh_val, sinh_val)` as `i64`

**Data Flow (ln)**:
1. Input: `x: i64` (x > 0, scaled by 2^30)
2. Identity: `ln(x) = 2 * atanh((x-1)/(x+1))`
3. Start from `(x+1, x-1)` in hyperbolic vectoring mode (`cordic.rs:424-425`)
4. Same iteration structure as `sinhcosh` but with sign flip for vectoring (`cordic.rs:437`)
5. Output: `2 * z` as `i64` (`cordic.rs:467`)

### 3.2 AGM Engine

**Entry Points**:
- `AgmEngine::agm(a0: u128, b0: u128) -> u128` -- core AGM iteration
- `AgmEngine::ln(x: u128) -> i128` -- natural log via AGM
- `AgmEngine::compute_pi_scaled() -> u128` -- Gauss-Legendre pi
- `AgmEngine::exp(x: i128) -> u128` -- exp via repeated squaring
- `AgmEngine::elliptic_k(k_squared: u128) -> u128` -- complete elliptic integral K(k)

**Data Flow (agm)**:
1. Input: `a0, b0: u128` (scaled by 2^62)
2. Loop up to `max_iterations` (default 20):
   - `a_next = (a + b) / 2` (arithmetic mean) (`agm.rs:71`)
   - `product = a.saturating_mul(b)` (`agm.rs:78`)
   - `b_next = isqrt_newton_128(product)` (geometric mean) (`agm.rs:79`)
   - Convergence check: `diff <= 1` (`agm.rs:68`)
3. Output: `a` (the converged AGM value) as `u128`

**Cross-module dependency**: `agm.rs` calls `crate::sqrt::isqrt_newton_128` for the geometric mean step.

**Data Flow (compute_pi_scaled)**:
1. Initialize: `a = AGM_SCALE`, `b = SCALE^2 / sqrt(2*SCALE^2)`, `t = AGM_SCALE/4`, `p = 1` (`agm.rs:165-177`)
2. Iterate Brent-Salamin: `a, b, t, p` update per Gauss-Legendre formulas (`agm.rs:179-207`)
   - Uses `isqrt_newton_128` for `sqrt(a_old * b)` (`agm.rs:194`)
   - Uses `saturating_mul` / `saturating_sub` throughout
3. Output: `sum_squared * AGM_SCALE / (4 * t)` as `u128` (`agm.rs:222`)

**Data Flow (exp)**:
1. Uses limit definition: `exp(x) = lim (1 + x/2^n)^(2^n)` with `n = 20` (`agm.rs:240`)
2. Compute `1 + x/2^20` (`agm.rs:244`)
3. Square 20 times with overflow guard (`agm.rs:247-253`)
4. Output: `u128`

**Types flowing through**: `u128` (main), `i128` (for ln, exp input), cross to `isqrt_newton_128`

### 3.3 Binary Splitting

**Entry Points**:
- `binary_split(a, b, term_a, term_b, term_p, term_q) -> BinarySplitState` -- generic
- `exp_binary_split(x: i128, scale_bits: u32, num_terms: u32) -> i128`
- `sin_binary_split(x: i128, scale_bits: u32, num_terms: u32) -> i128`
- `cos_binary_split(x: i128, scale_bits: u32, num_terms: u32) -> i128`
- `atan_binary_split(x: i128, scale_bits: u32, num_terms: u32) -> i128`
- `ln2_binary_split(scale_bits: u32, num_terms: u32) -> i128`
- `pi_chudnovsky(precision_bits: u32) -> i128`
- `pi_machin(scale_bits: u32, num_terms: u32) -> i128`
- `e_constant(scale_bits: u32, num_terms: u32) -> i128`
- `euler_gamma_approx(scale_bits: u32) -> i128`

**Data Flow (exp_binary_split)**:
1. Input: `x: i128` (scaled by 2^scale_bits)
2. Horner-like iteration: `result = scale`, `term = scale` (`binary_splitting.rs:114-115`)
3. Each step: `term = term * x / scale / k` using `saturating_mul` (`binary_splitting.rs:120`)
4. Accumulate: `result += term` (`binary_splitting.rs:122`)
5. Output: `result: i128`

**NOTE**: Despite the module name "binary_splitting", functions `exp_binary_split`, `sin_binary_split`, `cos_binary_split`, `atan_binary_split`, and `ln2_binary_split` do **NOT** use the generic `binary_split` function. They use direct Horner-scheme Taylor series evaluation. Only `pi_chudnovsky` uses the actual binary splitting algorithm.

**Data Flow (binary_split -- generic)**:
1. Recursive divide-and-conquer on range `[a, b)`
2. Base case: single term `BinarySplitState { p, q, b, t }` (`binary_splitting.rs:87-93`)
3. Combine: `BinarySplitState::combine(&left, &right)` merges via cross-multiplication with `saturating_mul` (`binary_splitting.rs:50-62`)
4. Output: `BinarySplitState` carrying accumulated P, Q, B, T

**Types flowing through**: `i128` exclusively (with `u32` for control parameters)

### 3.4 Continued Fractions

**Entry Points**:
- `ContinuedFraction::convergent(n: usize) -> ExactRational`
- `ContinuedFraction::all_convergents(n: usize) -> Vec<ExactRational>`
- `ContinuedFraction::error_bound(n: usize) -> ExactRational`
- `sqrt_cf(n: u64) -> ContinuedFraction`
- `e_cf() -> ContinuedFraction`
- `pi_cf() -> ContinuedFraction`
- `golden_ratio_cf() -> ContinuedFraction`
- `ln2_cf() -> ContinuedFraction`
- `tan_gcf(x_num, x_den, terms) -> ExactRational`
- `best_sqrt2_approx(max_denom) -> ExactRational`
- `pell_fundamental(n: u64) -> Option<(i128, i128)>`

**Data Flow (convergent)**:
1. Input: convergent index `n: usize`
2. Recurrence: `p_{n} = a_n * p_{n-1} + p_{n-2}` and `q_{n} = a_n * q_{n-1} + q_{n-2}` (`continued_fraction.rs:92-101`)
3. Overflow guard: break if `|p_curr| > 2^100` (`continued_fraction.rs:104`)
4. Output: `ExactRational { num: p_curr, den: q_curr }`

**Data Flow (sqrt_cf)**:
1. Input: `n: u64`
2. Compute `a0 = isqrt_newton(n)` (cross-module call to `sqrt.rs`) (`continued_fraction.rs:158`)
3. Perfect square check: return `ContinuedFraction::new(a0, Vec::new())` (`continued_fraction.rs:160`)
4. Generate periodic coefficients via standard algorithm (`continued_fraction.rs:163-187`)
5. Output: `ContinuedFraction` with `periodic_start: Some(0)`

**Types flowing through**: `i64` (CF coefficients), `i128` (convergent numerator/denominator), `u64` (input n)

### 3.5 Integer Sqrt

**Entry Points**:
- `isqrt_newton(n: u64) -> u64` -- floor(sqrt(n)) for 64-bit
- `isqrt_newton_128(n: u128) -> u128` -- floor(sqrt(n)) for 128-bit
- `isqrt_digit_by_digit(n: u64) -> u64` -- division-free sqrt
- `isqrt_binary(n: u64) -> u64` -- binary search sqrt
- `sqrt_rational(n: u64, precision_bits: u32) -> ExactRational`
- `sqrt_scaled(n: u64, scale_bits: u32) -> u128`
- `inv_sqrt_scaled(n: u64, scale_bits: u32) -> u128`
- `fast_inv_sqrt(n: u64) -> u64`
- `is_perfect_square(n: u64) -> bool`
- `icbrt(n: u64) -> u64`

**Data Flow (isqrt_newton)**:
1. Input: `n: u64`
2. Initial guess via bit manipulation: `1 << ((log2+1)/2)` (`sqrt.rs:40-43`)
3. Newton iteration: `x_next = (x + n/x) / 2` until `x_next >= x` (`sqrt.rs:47-54`)
4. Final floor adjustment: `while x*x > n { x -= 1 }` (`sqrt.rs:57-59`)
5. Output: `x: u64`

**Types flowing through**: `u64` (isqrt_newton), `u128` (isqrt_newton_128, sqrt_scaled)

---

## 4. Construct Identification

### 4.1 Structs

| Struct | File:Line | Visibility | Purpose |
|--------|-----------|------------|---------|
| `ExactRational` | `lib.rs:55` | `pub` | Rational number (num: i128, den: i128) with exact arithmetic |
| `TranscendentalConstants` | `lib.rs:143` | `pub` | Precomputed constants holder (pi, e, ln2, inv_ln2 at given precision) |
| `ErrorBound` | `lib.rs:187` | `pub` | Error bound descriptor (ulps + correct_bits) |
| `CordicEngine` | `cordic.rs:119` | `pub` | Circular CORDIC engine (iterations, gain, inv_gain) |
| `HyperbolicCordic` | `cordic.rs:310` | `pub` | Hyperbolic CORDIC engine (iterations, gain) |
| `AgmEngine` | `agm.rs:33` | `pub` | AGM computation engine (precision_bits, max_iterations) |
| `BinarySplitState` | `binary_splitting.rs:32` | `pub` | Carries (p, q, b, t) state through binary splitting recursion |
| `ContinuedFraction` | `continued_fraction.rs:33` | `pub` | CF representation (a0, coeffs Vec, periodic_start) |
| `BigSqrt` | `sqrt.rs:307` | `pub` | **STUB** -- arbitrary precision sqrt placeholder |

### 4.2 Traits

None declared. No custom traits in the entire crate.

### 4.3 Enums

None declared. No custom enums in the entire crate.

### 4.4 Trait Implementations

| Trait | For Type | File:Line |
|-------|----------|-----------|
| `Clone, Debug, PartialEq, Eq` (derive) | `ExactRational` | `lib.rs:54` |
| `Clone, Debug` (derive) | `ErrorBound` | `lib.rs:186` |
| `Clone, Debug` (derive) | `CordicEngine` | `cordic.rs:118` |
| `Default` (manual) | `CordicEngine` | `cordic.rs:128` |
| `Clone, Debug` (derive) | `HyperbolicCordic` | `cordic.rs:309` |
| `Default` (manual) | `HyperbolicCordic` | `cordic.rs:316` |
| `Clone, Debug` (derive) | `AgmEngine` | `agm.rs:32` |
| `Default` (manual) | `AgmEngine` | `agm.rs:40` |
| `Clone, Debug` (derive) | `BinarySplitState` | `binary_splitting.rs:31` |
| `Clone, Debug` (derive) | `ContinuedFraction` | `continued_fraction.rs:32` |

### 4.5 Public Functions (free-standing)

| Function | File:Line | Signature |
|----------|-----------|-----------|
| `binary_gcd` | `lib.rs:124` | `pub fn binary_gcd(a: u128, b: u128) -> u128` |
| `cordic_error_bound` | `cordic.rs:507` | `pub fn cordic_error_bound(iterations: usize) -> ErrorBound` |
| `gauss_constant_scaled` | `agm.rs:279` | `pub fn gauss_constant_scaled() -> u128` |
| `lemniscate_constant_scaled` | `agm.rs:293` | `pub fn lemniscate_constant_scaled() -> u128` |
| `binary_split` | `binary_splitting.rs:67` | `pub fn binary_split<F,G,H,I>(a, b, term_a, term_b, term_p, term_q) -> BinarySplitState` |
| `exp_binary_split` | `binary_splitting.rs:109` | `pub fn exp_binary_split(x: i128, scale_bits: u32, num_terms: u32) -> i128` |
| `sin_binary_split` | `binary_splitting.rs:130` | `pub fn sin_binary_split(x: i128, scale_bits: u32, num_terms: u32) -> i128` |
| `cos_binary_split` | `binary_splitting.rs:151` | `pub fn cos_binary_split(x: i128, scale_bits: u32, num_terms: u32) -> i128` |
| `atan_binary_split` | `binary_splitting.rs:171` | `pub fn atan_binary_split(x: i128, scale_bits: u32, num_terms: u32) -> i128` |
| `ln2_binary_split` | `binary_splitting.rs:192` | `pub fn ln2_binary_split(scale_bits: u32, num_terms: u32) -> i128` |
| `pi_chudnovsky` | `binary_splitting.rs:220` | `pub fn pi_chudnovsky(precision_bits: u32) -> i128` |
| `pi_machin` | `binary_splitting.rs:276` | `pub fn pi_machin(scale_bits: u32, num_terms: u32) -> i128` |
| `e_constant` | `binary_splitting.rs:290` | `pub fn e_constant(scale_bits: u32, num_terms: u32) -> i128` |
| `euler_gamma_approx` | `binary_splitting.rs:300` | `pub fn euler_gamma_approx(scale_bits: u32) -> i128` |
| `sqrt_cf` | `continued_fraction.rs:156` | `pub fn sqrt_cf(n: u64) -> ContinuedFraction` |
| `e_cf` | `continued_fraction.rs:198` | `pub fn e_cf() -> ContinuedFraction` |
| `pi_cf` | `continued_fraction.rs:214` | `pub fn pi_cf() -> ContinuedFraction` |
| `golden_ratio_cf` | `continued_fraction.rs:223` | `pub fn golden_ratio_cf() -> ContinuedFraction` |
| `ln2_cf` | `continued_fraction.rs:229` | `pub fn ln2_cf() -> ContinuedFraction` |
| `tan_gcf` | `continued_fraction.rs:237` | `pub fn tan_gcf(x_num: i128, x_den: i128, terms: usize) -> ExactRational` |
| `best_sqrt2_approx` | `continued_fraction.rs:259` | `pub fn best_sqrt2_approx(max_denom: i128) -> ExactRational` |
| `pell_fundamental` | `continued_fraction.rs:276` | `pub fn pell_fundamental(n: u64) -> Option<(i128, i128)>` |
| `isqrt_newton` | `sqrt.rs:30` | `pub fn isqrt_newton(n: u64) -> u64` |
| `isqrt_newton_128` | `sqrt.rs:65` | `pub fn isqrt_newton_128(n: u128) -> u128` |
| `isqrt_digit_by_digit` | `sqrt.rs:96` | `pub fn isqrt_digit_by_digit(n: u64) -> u64` |
| `isqrt_binary` | `sqrt.rs:125` | `pub fn isqrt_binary(n: u64) -> u64` |
| `sqrt_rational` | `sqrt.rs:146` | `pub fn sqrt_rational(n: u64, precision_bits: u32) -> ExactRational` |
| `sqrt_scaled` | `sqrt.rs:201` | `pub fn sqrt_scaled(n: u64, scale_bits: u32) -> u128` |
| `inv_sqrt_scaled` | `sqrt.rs:211` | `pub fn inv_sqrt_scaled(n: u64, scale_bits: u32) -> u128` |
| `fast_inv_sqrt` | `sqrt.rs:225` | `pub fn fast_inv_sqrt(n: u64) -> u64` |
| `is_perfect_square` | `sqrt.rs:268` | `pub fn is_perfect_square(n: u64) -> bool` |
| `icbrt` | `sqrt.rs:274` | `pub fn icbrt(n: u64) -> u64` |
| `from_scaled_30` | `constants.rs:221` | `pub fn from_scaled_30(x: i64) -> f64` -- **#[cfg(test)] gated** |
| `to_scaled_30` | `constants.rs:226` | `pub fn to_scaled_30(x: f64) -> i64` -- **#[cfg(test)] gated** |

### 4.6 Public Constants

| Constant | File:Line | Type | Value |
|----------|-----------|------|-------|
| `scales::SCALE_30` | `lib.rs:43` | `i64` | `1 << 30` |
| `scales::SCALE_62` | `lib.rs:45` | `i128` | `1 << 62` |
| `scales::SCALE_DECIMAL` | `lib.rs:47` | `i64` | `1_000_000_000` |
| `scales::SCALE_DECIMAL_18` | `lib.rs:49` | `i128` | `1_000_000_000_000_000_000` |
| `CORDIC_ITERATIONS` | `cordic.rs:26` | `usize` | `32` |
| `CORDIC_GAIN_30` | `cordic.rs:30` | `i64` | `652_032_874` |
| `CORDIC_INV_GAIN_30` | `cordic.rs:33` | `i64` | `1_768_195_363` |
| `CORDIC_ATAN_TABLE` | `cordic.rs:37` | `[i64; 32]` | Precomputed atan(2^-i) x 2^30 |
| `CORDIC_ATANH_TABLE` | `cordic.rs:73` | `[i64; 32]` | Precomputed atanh(2^-i) x 2^30 |
| `SCALE` | `cordic.rs:109` | `i64` | `1 << 30` |
| `HALF_PI` | `cordic.rs:111` | `i64` | `1_686_629_713` |
| `PI` | `cordic.rs:113` | `i64` | `3_373_259_426` |
| `TWO_PI` | `cordic.rs:115` | `i64` | `6_746_518_852` |
| `AGM_SCALE` | `agm.rs:28` | `u128` | `1u128 << 62` |
| `AGM_SCALE_BITS` | `agm.rs:29` | `u32` | `62` |
| (Plus 50+ constants in `constants.rs` submodules -- see Section 2.1) | | | |

---

## 5. Wiring Verification

### 5.1 Cross-Module Call Graph (production code only)

```
agm.rs --[calls]--> sqrt::isqrt_newton_128
continued_fraction.rs --[calls]--> sqrt::isqrt_newton
continued_fraction.rs --[uses]--> ExactRational (from lib.rs)
sqrt.rs --[uses]--> ExactRational (from lib.rs)
cordic.rs --[uses]--> ErrorBound (from lib.rs)
```

### 5.2 Wiring Status

| Declaration | Connected to Implementation | Called from Production Code | Called from Test Code |
|-------------|----------------------------|---------------------------|----------------------|
| `ExactRational` (lib.rs) | Yes, fully implemented | Yes (continued_fraction, sqrt) | Yes (lib.rs identity_tests) |
| `binary_gcd` (lib.rs) | Yes | Yes (via ExactRational::reduce) | Yes |
| `TranscendentalConstants` (lib.rs) | Yes, but limited to precision 30/62 | **No** (only used in lib.rs::tests) | Yes |
| `ErrorBound` (lib.rs) | Yes | Partially (cordic imports it, `cordic_error_bound` returns it) | No direct test |
| `CordicEngine` (cordic.rs) | Yes, fully wired | **No** (only used in tests) | Yes (extensively) |
| `HyperbolicCordic` (cordic.rs) | Yes, fully wired | **No** (only used in tests) | Yes |
| `AgmEngine` (agm.rs) | Yes, fully wired | **No** (only used in tests) | Yes |
| `BinarySplitState` (binary_splitting.rs) | Yes | Via `binary_split` fn, called from `pi_chudnovsky` | Yes |
| `ContinuedFraction` (continued_fraction.rs) | Yes, fully wired | **No** (only used in tests) | Yes |
| `BigSqrt` (sqrt.rs) | **STUB** -- sqrt() returns empty slice | **No** | **No** |

### 5.3 Verdict

All declared constructs have implementations, but most are only exercised through test code. The crate is a library, so downstream consumers would use the `pub` API -- but within this crate, the engines (`CordicEngine`, `AgmEngine`, `HyperbolicCordic`, `ContinuedFraction`) are not consumed by any other production module. The only production cross-module calls are:
- `agm.rs` -> `sqrt::isqrt_newton_128`
- `continued_fraction.rs` -> `sqrt::isqrt_newton`
- `continued_fraction.rs` -> `lib::ExactRational`
- `sqrt.rs` -> `lib::ExactRational`
- `cordic.rs` -> `lib::ErrorBound`

The `constants` module's sub-modules (`cordic_angles`, `hyperbolic_angles`, `cf_terms`, `precision_62`, most of `rational`) are referenced only from tests within `lib.rs` and `constants.rs` itself.

---

## 6. Dead Code Detection

### 6.1 Functions/Methods Only Called from Tests (Not from Production Code)

The following `pub` items are only exercised from `#[cfg(test)]` blocks. They are not dead code in the library sense (they are part of the public API available to downstream consumers), but within this crate they have no production callers:

| Item | File:Line | Only Called From |
|------|-----------|-----------------|
| `cordic_error_bound` | `cordic.rs:507` | **Nowhere** -- not even from tests |
| `ErrorBound::exact()` | `lib.rs:195` | **Nowhere** -- not even from tests |
| `ErrorBound::from_iterations()` | `lib.rs:202` | **Nowhere** -- not even from tests |
| `TranscendentalConstants::new()` | `lib.rs:158` | Tests only (`lib.rs:232`) |
| `TranscendentalConstants::scale()` | `lib.rs:180` | Tests only (`lib.rs:233`) |
| `ExactRational::to_scaled()` | `lib.rs:110` | **Nowhere** -- not even from tests |
| `CordicEngine::tan()` | `cordic.rs:228` | **Nowhere** -- not even from tests |
| `CordicEngine::atan2()` | `cordic.rs:265` | **Nowhere** -- not even from tests |
| `HyperbolicCordic::sinh()` | `cordic.rs:395` | **Nowhere** -- not even from tests |
| `HyperbolicCordic::cosh()` | `cordic.rs:401` | **Nowhere** -- not even from tests |
| `HyperbolicCordic::sqrt()` | `cordic.rs:472` | **Nowhere** -- not even from tests |
| `AgmEngine::ln()` | `agm.rs:94` | Tests only (`agm.rs:388`, `lib.rs:548`) |
| `AgmEngine::elliptic_k()` | `agm.rs:260` | Tests only (`agm.rs:399`) |
| `lemniscate_constant_scaled()` | `agm.rs:293` | **Nowhere** -- not even from tests |
| `pi_chudnovsky()` | `binary_splitting.rs:220` | **Nowhere** -- not even from tests |
| `euler_gamma_approx()` | `binary_splitting.rs:300` | **Nowhere** -- not even from tests |
| `ContinuedFraction::error_bound()` | `continued_fraction.rs:146` | **Nowhere** -- not even from tests |
| `ln2_cf()` | `continued_fraction.rs:229` | **Nowhere** -- not even from tests |
| `tan_gcf()` | `continued_fraction.rs:237` | **Nowhere** -- not even from tests |
| `isqrt_digit_by_digit()` | `sqrt.rs:96` | Tests only (`sqrt.rs:345`) |
| `isqrt_binary()` | `sqrt.rs:125` | Tests only (`sqrt.rs:352`) |
| `sqrt_rational()` | `sqrt.rs:146` | Tests only (`sqrt.rs:359`) |
| `inv_sqrt_scaled()` | `sqrt.rs:211` | **Nowhere** -- not even from tests |
| `fast_inv_sqrt()` | `sqrt.rs:225` | Tests only (`sqrt.rs:398`) |
| `icbrt()` | `sqrt.rs:274` | Tests only (`sqrt.rs:388`) |
| `BigSqrt::new()` | `sqrt.rs:313` | **Nowhere** -- not even from tests |
| `BigSqrt::sqrt()` | `sqrt.rs:319` | **Nowhere** -- not even from tests |

### 6.2 Unused Modules / Sub-Modules

| Module | File:Line | Usage in Non-Test Code |
|--------|-----------|----------------------|
| `constants::cordic_angles` | `constants.rs:126` | **Never imported/used** outside constants.rs |
| `constants::hyperbolic_angles` | `constants.rs:160` | **Never imported/used** |
| `constants::cf_terms` | `constants.rs:199` | **Never imported/used** |
| `constants::precision_62` | `constants.rs:69` | **Never imported/used** outside tests |
| `constants::rational` (partially) | `constants.rs:90` | Only `PI_355_113` and `SQRT2_HI` used (from constants.rs tests). Items `PI_HI`, `E_APPROX`, `E_1264_465`, `PHI_FIB`, `SQRT2_99_70`, `SQRT2_ULTRA`, `SQRT3_97_56`, `LN2_ROUGH`, `LN2_HI` are never referenced. |
| `constants::pade` (partially) | `constants.rs:178` | Only `EXP_P` and `EXP_Q` used (from lib.rs truth_perturber tests). Items `SIN_P`, `SIN_Q`, `COS_P`, `COS_Q`, `LN_P`, `LN_Q` are never referenced. |
| `sqrt::arbitrary_precision` | `sqrt.rs:304` | **Stub module** -- never imported/used |

### 6.3 Completely Unreferenced Constants

The following constants in `constants.rs` are declared `pub` but never referenced anywhere in the crate (not even from tests):

- `precision_30::PHI` (`constants.rs:29`)
- `precision_30::SQRT3` (`constants.rs:35`)
- `precision_30::SQRT5` (`constants.rs:38`)
- `precision_30::LN10` (`constants.rs:44`)
- `precision_30::INV_LN2` (`constants.rs:47`)
- `precision_30::INV_PI` (`constants.rs:50`)
- `precision_30::HALF_PI` (`constants.rs:53`)
- `precision_30::QUARTER_PI` (`constants.rs:56`)
- `precision_30::TWO_PI` (`constants.rs:59`)
- `precision_30::EULER_GAMMA` (`constants.rs:62`)
- `precision_30::SCALE` (`constants.rs:65`)
- `precision_62::E` (`constants.rs:74`)
- `precision_62::PHI` (`constants.rs:77`)
- `precision_62::SQRT2` (`constants.rs:80`)
- `precision_62::LN2` (`constants.rs:83`)
- `precision_62::SCALE` (`constants.rs:86`)
- All of `cordic_angles::*` (6 individual constants + ATAN_TABLE + GAIN + INV_GAIN)
- All of `hyperbolic_angles::*` (ATANH_TABLE + GAIN)
- All of `cf_terms::*` (PI, E, SQRT2, SQRT3, PHI, LN2)
- `rational::PI_HI` through `rational::LN2_HI` (8 of 10 rational constants)
- `pade::SIN_P`, `pade::SIN_Q`, `pade::COS_P`, `pade::COS_Q`, `pade::LN_P`, `pade::LN_Q`
- `scales::SCALE_DECIMAL` (`lib.rs:47`)
- `scales::SCALE_DECIMAL_18` (`lib.rs:49`)

---

## 7. Cross-Reference Check

### 7.1 Module Imports/Exports Alignment

| Source Module | Imports From | Status |
|---------------|-------------|--------|
| `cordic.rs` | `crate::ErrorBound` | OK -- `ErrorBound` is `pub` in `lib.rs` |
| `agm.rs` | `crate::sqrt::isqrt_newton_128` | OK -- `isqrt_newton_128` is `pub` in `sqrt.rs`, `sqrt` is `pub mod` in `lib.rs` |
| `continued_fraction.rs` | `crate::ExactRational`, `crate::sqrt::isqrt_newton` | OK -- both are `pub` |
| `sqrt.rs` | `crate::ExactRational` | OK |
| `lib.rs` (cross_validation tests) | `crate::cordic::*`, `crate::agm::*`, `crate::binary_splitting::*`, `crate::continued_fraction::*`, `crate::constants::*`, `crate::sqrt::*` | OK -- all items imported are `pub` |

### 7.2 Circular Dependencies

**None detected.** The dependency graph is a DAG:
```
lib.rs (root)
  |-- cordic.rs -> lib.rs (ErrorBound)
  |-- agm.rs -> sqrt.rs
  |-- continued_fraction.rs -> sqrt.rs, lib.rs (ExactRational)
  |-- sqrt.rs -> lib.rs (ExactRational)
  |-- binary_splitting.rs -> (no imports)
  |-- constants.rs -> (no imports)
```

### 7.3 Missing Re-exports

The crate root (`lib.rs`) re-exports modules but does **not** re-export commonly-used items at the top level. A consumer must write `exact_transcendentals::cordic::CordicEngine` rather than `exact_transcendentals::CordicEngine`. This is a design choice, not a defect, but worth noting. The only items directly available at the crate root are:

- `ExactRational`
- `binary_gcd`
- `TranscendentalConstants`
- `ErrorBound`
- `scales::*`

### 7.4 `no_std` Compatibility Check

- `lib.rs:23`: `#![cfg_attr(not(feature = "std"), no_std)]` -- correctly gates
- `lib.rs:25-31`: Conditional Vec import from `alloc` vs `std` -- correct
- `continued_fraction.rs:26-29`: Conditional Vec/vec! import from `alloc` -- correct
- `binary_splitting.rs`: Does **not** use `Vec` in production code (stack-only recursion) -- OK
- `agm.rs`, `cordic.rs`, `sqrt.rs`, `constants.rs`: Do **not** use `Vec` or any `std`/`alloc` imports in production code -- OK

**Verdict**: `no_std` support appears correctly wired. The `alloc` imports in `continued_fraction.rs` are appropriately gated.

---

## 8. Anomaly Catalogue

### A-1: Floating-Point in `#[cfg(test)]`-gated Public Functions (MINOR)

**Location**: `constants.rs:220-228`
```rust
#[cfg(test)]
pub fn from_scaled_30(x: i64) -> f64 { ... }

#[cfg(test)]
pub fn to_scaled_30(x: f64) -> i64 { ... }
```

**Issue**: These are `pub` functions that use `f64`, gated by `#[cfg(test)]`. This is the correct approach for test utilities. However, they are `pub` which means they appear in the test-mode public API. This is intentional and consistent with the project's float policy (floats only in test verification).

**All other f64 usage** is inside `#[cfg(test)] mod tests` blocks within each module. The `ExactRational::to_f64()` at `lib.rs:116` is also correctly `#[cfg(test)]`-gated. **No float violations in production code.**

### A-2: Incomplete Implementation -- `pi_chudnovsky` (SIGNIFICANT)

**Location**: `binary_splitting.rs:220-272`

The Chudnovsky pi computation has a hardcoded approximation:
```rust
// binary_splitting.rs:262
let sqrt_c = 800i128; // sqrt(640320) ~ 800.2
```

The comment at line 261 says: "For now, return approximate result / Full implementation needs integer sqrt of C". This is a known incomplete implementation. The `isqrt_newton_128` function exists in `sqrt.rs` and could compute `isqrt_newton_128(640320)` = 800, but the fractional part (800.2) would be lost, causing the result to be slightly inaccurate. More importantly, the Chudnovsky formula requires `sqrt(10005)` (not `sqrt(640320)`) as the constant C = 426880 * sqrt(10005), and this is not implemented at all. The function will produce an inaccurate result.

Additionally, `pi_chudnovsky` is **never called** from anywhere in the crate (not even from tests).

### A-3: Stub Module -- `arbitrary_precision` (MINOR)

**Location**: `sqrt.rs:302-324`

```rust
pub mod arbitrary_precision {
    pub struct BigSqrt { pub precision: u32 }
    impl BigSqrt {
        pub fn new(precision: u32) -> Self { Self { precision } }
        pub fn sqrt(&self, _n: &[u64]) -> &'static [u64] { &[] }
    }
}
```

This is an explicit placeholder with a parameter named `_n` (underscore prefix indicating intentional non-use). The `sqrt` method always returns an empty slice. Never referenced from any code.

### A-4: `saturating_*` Overflow Silencing (SIGNIFICANT)

**Locations** (28 total occurrences):
- `agm.rs:78,194,202,203,206,215,222` -- AGM computations
- `binary_splitting.rs:52,53,54,57,58,59,120-122,132,141,143,153,160,162,173,182,184,197,207,209` -- binary split combine + Taylor series

The use of `saturating_mul`, `saturating_add`, `saturating_sub` means that when intermediate values overflow `i128` or `u128`, they silently clamp to `MAX`/`MIN` instead of panicking or signaling. In a system that claims "exact arithmetic," this silent clamping can produce incorrect results without any indication to the caller. There is no error reporting mechanism when saturation occurs.

For example, in `BinarySplitState::combine` (`binary_splitting.rs:50-62`):
```rust
let p = left.p.checked_mul(right.p).unwrap_or(i128::MAX);
let q = left.q.checked_mul(right.q).unwrap_or(i128::MAX);
```
If `left.p * right.p` overflows, `p` becomes `i128::MAX`, silently corrupting the computation.

### A-5: `panic!` Path in Production Code (MODERATE)

**Location**: `lib.rs:176`
```rust
_ => panic!("Unsupported precision, use 30 or 62 bits"),
```

`TranscendentalConstants::new()` panics for any `precision_bits` value other than 30 or 62. This is in production code (not test-gated). While the struct is only used from tests within this crate, an external consumer calling `TranscendentalConstants::new(48)` would get a panic.

### A-6: `debug_assert!` Instead of Runtime Assertion (MODERATE)

**Location**: `lib.rs:62,102`
```rust
debug_assert!(den != 0, "Denominator cannot be zero");  // line 62
debug_assert!(other.num != 0, "Division by zero");       // line 102
```

In release builds, `debug_assert!` is compiled away. This means `ExactRational::new(5, 0)` and `ExactRational::div(&zero)` will silently proceed in release mode, potentially causing division-by-zero panics later when `den == 0` is used in arithmetic.

### A-7: `unwrap()` Calls (LOW -- Test Code Only)

**Locations**:
- `lib.rs:608` (in `identity_tests::identity_pell_solutions_correct`)
- `continued_fraction.rs:408,414` (in test functions)

All `unwrap()` calls are inside `#[cfg(test)]` blocks. They will panic if the `Option` is `None`, but this is acceptable in test code. These test `unwrap()` calls are preceded by `assert!(sol.is_some())` checks.

### A-8: CORDIC Gain Not Recomputed for Fewer Iterations (LOW)

**Location**: `cordic.rs:140-146`
```rust
let (gain, inv_gain) = if iterations >= CORDIC_ITERATIONS {
    (CORDIC_GAIN_30, CORDIC_INV_GAIN_30)
} else {
    // Compute for fewer iterations (less accurate but faster)
    // This is approximate - in production, precompute these
    (CORDIC_GAIN_30, CORDIC_INV_GAIN_30)  // <-- SAME VALUES!
};
```

When `CordicEngine::new()` is called with fewer than 32 iterations, the gain factor is NOT recomputed. The comment acknowledges this ("This is approximate - in production, precompute these"). Both branches return the same values. For a 32-iteration CORDIC, the gain is product of `sqrt(1 + 2^(-2i))` for `i=0..31`. Using fewer iterations should use a different gain, but the code uses the 32-iteration gain regardless. This will introduce systematic error for reduced-iteration CORDIC.

This affects the `truth_perturber` test `perturb_cordic_residual_as_error_information` (`lib.rs:741-759`) which creates engines with 8, 16, 24, and 32 iterations.

### A-9: Feature Flags Declared but Unused (LOW)

**Location**: `Cargo.toml:17-20`
```toml
simd = []
arbitrary-precision = []
```

Neither `simd` nor `arbitrary-precision` feature flags have any `#[cfg(feature = "...")]` gates in any source file. They are declared but do nothing.

### A-10: Duplicate Constant Tables (LOW)

The CORDIC atan table appears in two places:
- `cordic.rs:37-70` as `CORDIC_ATAN_TABLE`
- `constants.rs:141-150` as `cordic_angles::ATAN_TABLE`

Similarly, the atanh table:
- `cordic.rs:73-106` as `CORDIC_ATANH_TABLE`
- `constants.rs:162-171` as `hyperbolic_angles::ATANH_TABLE`

And gain constants:
- `cordic.rs:30,33` as `CORDIC_GAIN_30`, `CORDIC_INV_GAIN_30`
- `constants.rs:153,156` as `cordic_angles::GAIN`, `cordic_angles::INV_GAIN`

The `cordic.rs` engine uses its own copy, not the `constants` module copy. The values appear identical but there is no compile-time guarantee of consistency. If one table is updated and the other is not, they would silently diverge.

### A-11: `TranscendentalConstants` e_scaled Discrepancy (LOW)

**Location**: `lib.rs:164` vs `constants.rs:26`

```rust
// lib.rs:164 (TranscendentalConstants::new(30))
e_scaled: 2_918_732_009,

// constants.rs:26 (precision_30::E)
pub const E: i64 = 2_918_732_888;
```

These are different values for `e * 2^30`: `2_918_732_009` vs `2_918_732_888`. The difference is 879 ULPs. Both claim to represent `e * 2^30`. At most one can be correct. The true value of `e * 2^30` rounded to nearest integer is `2_918_732_888` (which matches `precision_30::E`). The value in `TranscendentalConstants` appears to be wrong.

### A-12: Potential Integer Overflow in Range Reduction (LOW)

**Location**: `cordic.rs:157-158`
```rust
while angle > PI { angle -= TWO_PI; }
while angle < -PI { angle += TWO_PI; }
```

`PI` is `3_373_259_426` and `TWO_PI` is `6_746_518_852`, both `i64`. For extremely large angles near `i64::MAX`, subtraction of `TWO_PI` could loop for a very long time (up to ~1.37 billion iterations to reduce from `i64::MAX`). While not strictly a correctness bug, it is a performance hazard. A modular reduction (e.g., using division) would be O(1).

### A-13: Naming Inconsistency -- "binary_split" Functions (INFORMATIONAL)

Functions named `exp_binary_split`, `sin_binary_split`, `cos_binary_split`, `atan_binary_split`, `ln2_binary_split` do NOT use the `binary_split()` generic function. They use direct Horner-scheme Taylor series iteration. The "binary_split" in their name is misleading. Only `pi_chudnovsky` actually uses binary splitting.

### A-14: `Vec` Import in `lib.rs` Is Unused in Production Code (INFORMATIONAL)

**Location**: `lib.rs:29-31`
```rust
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;
```

The `Vec` import at the crate root is only used in test code (the `cross_validation` and other test modules). Production code in `lib.rs` does not use `Vec`. The import is not harmful but is unnecessary for production compilation. (The compiler may emit a warning depending on lint level.)

---

## 9. Summary of Findings

### By Severity

**SIGNIFICANT (2)**:
- **A-2**: `pi_chudnovsky` has incomplete implementation (hardcoded sqrt approximation, wrong constant)
- **A-4**: `saturating_*` silently clamping overflows corrupts "exact" results without any error signaling

**MODERATE (2)**:
- **A-5**: `panic!` in `TranscendentalConstants::new()` for unsupported precision values
- **A-6**: `debug_assert!` for zero-denominator checks compiled away in release mode

**LOW (7)**:
- **A-7**: `unwrap()` in test code (acceptable)
- **A-8**: CORDIC gain not recomputed for fewer iterations
- **A-9**: Feature flags `simd` and `arbitrary-precision` declared but unused
- **A-10**: Duplicate constant tables between `cordic.rs` and `constants.rs`
- **A-11**: `TranscendentalConstants` has wrong `e_scaled` value (off by 879 ULPs)
- **A-12**: Potential O(n) range reduction loop for large angles
- **A-14**: Unused `Vec` import at crate root in production builds

**INFORMATIONAL (2)**:
- **A-1**: All f64 usage correctly `#[cfg(test)]`-gated (COMPLIANT)
- **A-13**: Naming inconsistency -- "binary_split" functions don't use binary splitting

### Dead Code Summary

- **15 pub functions** never called from anywhere (not even tests)
- **1 pub module** (`arbitrary_precision`) is a complete stub
- **~40 pub constants** in `constants.rs` sub-modules are never referenced
- **2 feature flags** declared but gating nothing
- **6 complete sub-modules** of `constants` are unreferenced from production code

### Integer-Only Mandate Compliance

**COMPLIANT.** All floating-point usage (`f64`, `f32`, `as f64`, `as f32`) is confined to `#[cfg(test)]` blocks. The `check_no_floats.py` script correctly validates this. Zero float violations in production code paths.

### Overall Assessment

The crate is architecturally sound with a clean module dependency graph, zero circular dependencies, correct `no_std` support, and full compliance with the integer-only mandate. The primary concerns are: (1) the silent overflow clamping via `saturating_*` which undermines the "exact arithmetic" guarantee at the i128 boundary, (2) the incomplete `pi_chudnovsky` implementation, and (3) the substantial amount of dead public API surface that exists as forward-looking infrastructure but is currently unreachable.

---

*End of forensic audit report.*

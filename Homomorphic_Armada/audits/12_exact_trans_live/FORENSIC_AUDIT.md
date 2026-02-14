# FORENSIC AUDIT: Build 12 -- exact_trans_live

**Build**: `12_exact_trans_live`
**Path**: `/home/acid/Projects/Homomorphic_Armada/builds/12_exact_trans_live/` (symlink to `~/Projects/exact_transcendentals/`)
**Crate**: `exact_transcendentals` v1.0.0
**Edition**: Rust 2021
**Auditor**: Claude Opus 4.6 (forensic code auditor)
**Date**: 2026-02-13
**Scope**: INSPECT, ANALYZE, REPORT -- no modifications made

---

## 1. STRUCTURE MAPPING

### 1.1 Module Tree

```
exact_transcendentals v1.0.0
|
+-- Cargo.toml                    (workspace root, 0 runtime deps, criterion dev-dep)
|
+-- src/
|   +-- lib.rs                    (root: pub API, ExactRational, error types, cross-validation tests)
|   +-- cordic.rs                 (CordicEngine, HyperbolicCordic)
|   +-- agm.rs                    (AgmEngine, Gauss/Lemniscate constants)
|   +-- binary_splitting.rs       (BinarySplitState, Taylor/Machin/Chudnovsky)
|   +-- continued_fraction.rs     (ContinuedFraction, sqrt_cf, pell_fundamental)
|   +-- sqrt.rs                   (isqrt_newton, isqrt_newton_128, sqrt_scaled, etc.)
|   +-- constants.rs              (precision_30, precision_62, rational, pade, cf_terms)
|   +-- bigint.rs                 [feature-gated: arbitrary-precision]
|   +-- crt.rs                    [feature-gated: arbitrary-precision]
|   +-- crt_rational.rs           [feature-gated: arbitrary-precision]
|
+-- benches/
|   +-- performance.rs            (criterion benchmarks for 5 engine paths)
|
+-- lean4/                        (Lean 4 formal verification -- NOT audited here)
|   +-- ExactTranscendentals.lean
|   +-- ExactTranscendentals/{Cordic,Agm,BinarySplitting,ContinuedFraction,ExactRational,Isqrt,Basic}.lean
|   +-- Main.lean
|
+-- docs/plans/
|   +-- 2026-02-08-full-proof-coverage-design.md
|
+-- BLUEPRINT.md, CLAUDE.md, QWEN.md, README.md
```

### 1.2 Dependencies

```toml
[dependencies]
# NONE -- zero runtime dependencies

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
```

**Observation**: Zero runtime dependencies is a strong positive. The crate is fully self-contained.

### 1.3 Features

| Feature | Status | Gates |
|---------|--------|-------|
| `std` (default) | Active | `no_std` support via `alloc` |
| `arbitrary-precision` | Optional | `bigint.rs`, `crt.rs`, `crt_rational.rs` + big_* functions in `binary_splitting.rs`, `sqrt.rs` |

### 1.4 Public API Surface

**Always-available modules**: `agm`, `binary_splitting`, `constants`, `continued_fraction`, `cordic`, `sqrt`

**Feature-gated modules** (`arbitrary-precision`): `bigint`, `crt`, `crt_rational`

**Root-level public items** (from `lib.rs`):
- `ExactRational` struct (i128 numerator/denominator)
- `TranscendentalError` enum
- `TransResult<T>` type alias
- `TranscendentalConstants` struct
- `ErrorBound` struct
- `binary_gcd()` function
- `checked_mul_i128()`, `checked_add_i128()` functions
- `scales` module (SCALE_30, SCALE_62, SCALE_DECIMAL, SCALE_DECIMAL_18)

---

## 2. DATA FLOW TRACING

### 2.1 CORDIC Engine

**Input types**: `i64` (scaled fixed-point, scale = 2^30)
**Computation**: Shift-and-add rotations, 32 iterations, gain correction via precomputed `CORDIC_GAIN_30`
**Output**: `i64` (scaled)

**Flow**: `sincos(angle: i64)` -> O(1) range reduction via modular arithmetic -> quadrant mapping -> 32 CORDIC iterations (no multiply in loop) -> gain correction (one i128 multiply + divide) -> `(cos, sin)` as `(i64, i64)`.

**Hyperbolic path**: `HyperbolicCordic::sinhcosh(arg: i64)` -> convergence limit check -> recursive halving for large args -> hyperbolic CORDIC with repeated iterations at k=4,13,40,... -> gain correction.

**Derived**: `exp(x) = cosh(x) + sinh(x)`, `ln(x) = 2 * atanh((x-1)/(x+1))` via vectoring mode.

### 2.2 AGM Engine

**Input types**: `u128` (scaled fixed-point, scale = 2^62)
**Computation**: Arithmetic-geometric mean iteration with integer sqrt
**Output**: `u128` or `i128`

**Flow for pi**: Gauss-Legendre: `a=SCALE, b=SCALE/sqrt(2), t=SCALE/4, p=1` -> iterate `a=(a+b)/2, b=sqrt(a_old*b), t-=p*(a_old-a)^2, p*=2` -> `pi = (a+b)^2 / (4t)`.

**Flow for ln**: Range reduction via leading-bit extraction -> Brent's formula `ln(s) = pi/(2*M(1, 4/(s*2^N))) - N*ln(2)` -> AGM computation -> reconstruction with `m*ln(2)`.

**Flow for exp**: Limit definition `(1 + x/2^16)^(2^16)` via repeated squaring with truncation compensation.

### 2.3 Binary Splitting Engine

**Input types**: `i128` (scaled) + closures for series coefficients
**Computation**: Recursive divide-and-conquer series summation
**Output**: `i128` (scaled) or `Option<BinarySplitState>`

**Flow**: `binary_split(a, b, term_a, term_b, term_p, term_q)` -> recursive split at midpoint -> `BinarySplitState::combine()` with checked arithmetic -> final extraction `T / (B * Q)`.

**Concrete functions**: `exp_binary_split`, `sin_binary_split`, `cos_binary_split`, `atan_binary_split`, `ln2_binary_split`, `pi_machin`, `pi_chudnovsky`, `e_constant`, `euler_gamma_approx`.

### 2.4 Continued Fraction Engine

**Input types**: `i64` coefficients, `u64` for sqrt_cf
**Computation**: Recurrence relation for convergents
**Output**: `ExactRational` (i128 num/den)

**Flow**: `sqrt_cf(n)` -> detect period via standard algorithm -> `ContinuedFraction::convergent(n)` -> matrix recurrence `p_n = a_n*p_{n-1} + p_{n-2}`, `q_n = a_n*q_{n-1} + q_{n-2}`.

### 2.5 Sqrt Engine

**Input types**: `u64` or `u128`
**Computation**: Newton-Raphson iteration `x = (x + n/x) / 2`
**Output**: `u64`, `u128`, or `ExactRational`

**Three algorithms**: Newton-Raphson (quadratic convergence), digit-by-digit (no division), binary search (simple).

### 2.6 Comparison with Build 01

Build 01 source files: `lib.rs`, `cordic.rs`, `agm.rs`, `binary_splitting.rs`, `continued_fraction.rs`, `sqrt.rs`, `constants.rs` (7 files).

Build 12 source files: Same 7 + `bigint.rs`, `crt.rs`, `crt_rational.rs` (10 files).

Build 01 `lib.rs` had no `ExactRational` struct at root level, no `TranscendentalError` enum, no checked arithmetic helpers, no `#[allow(unused_imports)]` directives. The root module was much smaller.

---

## 3. CONSTRUCT IDENTIFICATION: NEW vs v0.1.0 (Build 01)

### 3.1 Entirely New Modules (not in build 01)

| Module | Purpose | Lines |
|--------|---------|-------|
| `bigint.rs` | `HCVLangBigInt` arbitrary-precision signed integer | ~791 |
| `crt.rs` | `CRTBigInt` Chinese Remainder Theorem integers | ~581 |
| `crt_rational.rs` | `CRTRational` arbitrary-precision rational | ~270 |

### 3.2 New Constructs in Existing Modules

#### lib.rs -- NEW constructs vs build 01:
- `TranscendentalError` enum (4 variants: Overflow, DomainError, ConvergenceFailure, SaturationDetected) -- **NEW**
- `TransResult<T>` type alias -- **NEW**
- `checked_mul_i128()`, `checked_add_i128()` -- **NEW**
- `ExactRational` struct with `checked_add`, `checked_mul`, `checked_div` -- **NEW** (the base struct existed but checked variants are new)
- `mod cross_validation` -- **NEW** (entire test module, ~270 lines)
- `mod identity_tests` -- **NEW** (entire test module, ~278 lines)
- `mod truth_perturber` -- **NEW** (entire test module, ~341 lines)
- `mod remediation_tests` -- **NEW** (entire test module, ~253 lines)

#### cordic.rs -- NEW constructs:
- `CordicEngine::tan_checked()` -- **NEW** (returns `TransResult<i64>`)
- `HyperbolicCordic::ln_checked()` -- **NEW** (returns `TransResult<i64>`)
- `HyperbolicCordic::sqrt()` -- **NEW** but immediately `#[deprecated]` and `#[allow(dead_code)]`
- O(1) range reduction in `sincos()` (replaces while-loop from v0.1.0) -- **REWRITTEN**
- Dynamic gain computation in `HyperbolicCordic::new()` -- **REWRITTEN** (was hardcoded)

#### agm.rs -- NEW constructs:
- `AgmEngine::ln_checked()` -- **NEW**
- `AgmEngine::exp_checked()` -- **NEW**
- `AgmEngine::exp()` with truncation compensation -- **REWRITTEN** (was simpler)
- `AgmEngine::ln()` with Brent-Salamin formula -- **REWRITTEN** (was different algorithm)
- Overflow-safe geometric mean path in `agm()` -- **NEW**

#### binary_splitting.rs -- NEW constructs:
- `BinarySplitState::combine()` returning `Option<Self>` -- **NEW** (replaces saturating version)
- `BinarySplitState::combine_saturating()` -- **DEPRECATED** legacy
- `binary_split()` now returns `Option<BinarySplitState>` -- **CHANGED** from infallible
- `BigBinarySplitState` -- **NEW** (feature-gated)
- `big_binary_split()` -- **NEW** (feature-gated)
- `big_exp()`, `big_sin()`, `big_cos()` -- **NEW** (feature-gated, use CRTRational)
- `pi_chudnovsky_big()` -- **NEW** (feature-gated)
- `euler_gamma_approx()` -- **NEW**

#### continued_fraction.rs -- NEW constructs:
- `ContinuedFraction::error_bound()` -- **REWRITTEN** (single-pass, was double convergent call)

#### sqrt.rs -- NEW constructs:
- `big_isqrt()` -- **NEW** (feature-gated, uses HCVLangBigInt)

#### constants.rs -- NEW constructs:
- `pade` module with EXP_P/Q, SIN_P/Q, COS_P/Q, LN_P/Q coefficients -- **NEW**

### 3.3 New Test Infrastructure

Build 12 introduces four entirely new test module categories in `lib.rs`:

1. **`cross_validation`** (lines 619-890): 10 tests cross-validating between engines (AGM vs Machin, CORDIC vs Taylor, Newton vs CF, etc.)
2. **`identity_tests`** (lines 896-1175): 9 tests verifying mathematical identities purely via integer comparison
3. **`truth_perturber`** (lines 1184-1624): 12 tests exploring boundary conditions, algebraic properties, and cross-domain connections
4. **`remediation_tests`** (lines 359-612): 21 tests covering previously untested public functions and edge cases

**Total new test lines in lib.rs alone**: ~1005 lines of test code.

---

## 4. WIRING VERIFICATION: Cross-Validation Suite

### 4.1 Does the cross-validation suite actually cross-validate?

**YES.** The `cross_validation` module in `lib.rs:619-890` genuinely computes the same mathematical constant via multiple independent engine paths and compares results:

| Test | Engine A | Engine B | Constant | Verdict |
|------|----------|----------|----------|---------|
| `cross_pi_agm_vs_machin` | `AgmEngine::compute_pi_scaled()` | `pi_machin(62, 40)` | pi | Genuine cross-val |
| `cross_pi_machin_vs_cf_convergent` | `pi_machin(30, 30)` | `pi_cf().convergent(3)` | pi | Genuine cross-val |
| `cross_pi_precomputed_vs_agm` | `precision_30::PI` | AGM rescaled 62->30 | pi | Genuine cross-val |
| `cross_exp_taylor_vs_cordic` | `exp_binary_split()` | `HyperbolicCordic::exp()` | exp(1) | Genuine cross-val |
| `cross_exp_taylor_vs_agm` | `exp_binary_split()` | `AgmEngine::exp()` | exp(1) | Genuine cross-val |
| `cross_sqrt2_newton_vs_cf` | `sqrt_scaled(2, 30)` | `sqrt_cf(2).convergent(15)` | sqrt(2) | Genuine cross-val |
| `cross_sqrt2_newton_vs_precomputed` | `sqrt_scaled(2, 30)` | `precision_30::SQRT2` | sqrt(2) | Genuine cross-val |
| `cross_e_taylor_vs_cf` | `e_constant(30, 20)` | `e_cf().convergent(10)` | e | Genuine cross-val |
| `cross_e_precomputed_vs_taylor` | `precision_30::E` | `e_constant(30, 20)` | e | Genuine cross-val |
| `cross_ln2_taylor_vs_precomputed` | `ln2_binary_split(30, 30)` | `precision_30::LN2` | ln(2) | Genuine cross-val |
| `cross_sin_cordic_vs_taylor` | `CordicEngine::sin()` | `sin_binary_split()` | sin(pi/6) | Genuine cross-val |

**Rescaling**: The `rescale(val, from_bits, to_bits)` helper (lib.rs:630-636) correctly shifts between scales. Verified: `from_bits > to_bits` shifts right; `from_bits < to_bits` shifts left.

### 4.2 Cross-validation gaps

- **No AGM exp vs CORDIC exp cross-validation** -- Taylor mediates both directions but AGM and CORDIC are never directly compared for exp.
- **No ln cross-validation** -- ln(2) is validated Taylor vs precomputed constant, but AGM ln vs CORDIC ln are never cross-validated.
- **No cos cross-validation** -- Only sin is cross-validated between CORDIC and Taylor.

### 4.3 Identity Tests Wiring

The `identity_tests` module (lib.rs:896-1175) correctly wires:
- Pythagorean identity via both CORDIC and Taylor
- Double angle formula via CORDIC
- exp(0)=1 via Taylor, AGM, and CORDIC (all three)
- ln(1)=0 via AGM and CORDIC (both)
- Perfect square detection via Newton isqrt
- CF convergent alternation property
- Pell equation solutions
- ExactRational identities (1/3+1/3+1/3=1, a/b * b/a = 1, a-a=0)
- Binary GCD algebraic properties

All wiring verified correct.

---

## 5. DEAD CODE DETECTION

### 5.1 Explicitly Marked Dead Code

| Location | Construct | Status |
|----------|-----------|--------|
| `cordic.rs:617` | `HyperbolicCordic::sqrt()` | `#[allow(dead_code)]` + `#[deprecated]` + `pub(crate)` visibility |

### 5.2 Potentially Unused Public Functions

| Location | Function | Assessment |
|----------|----------|------------|
| `sqrt.rs:104` | `isqrt_digit_by_digit()` | Only called from tests. Never used in production code paths. |
| `sqrt.rs:135` | `isqrt_binary()` | Only called from tests. Never used in production code paths. |
| `sqrt.rs:243` | `fast_inv_sqrt()` | Only called from tests. Never used in production code paths. |
| `sqrt.rs:298` | `icbrt()` | Only called from tests. Never used in production code paths. |
| `sqrt.rs:225` | `inv_sqrt_scaled()` | Only called from tests. Never used in production code paths. |
| `sqrt.rs:158` | `sqrt_rational()` | Only called from tests. Never used in production code paths. |
| `agm.rs:430` | `gauss_constant_scaled()` | Only called from tests. |
| `agm.rs:444` | `lemniscate_constant_scaled()` | Only called from tests. |
| `agm.rs:409` | `AgmEngine::elliptic_k()` | Only called from tests. |
| `continued_fraction.rs:269` | `tan_gcf()` | Never called anywhere. Fully dead code. |
| `continued_fraction.rs:291` | `best_sqrt2_approx()` | Only called from tests. |
| `continued_fraction.rs:260` | `ln2_cf()` | Never called anywhere. Fully dead code. |
| `constants.rs:267-275` | `from_scaled_30()`, `to_scaled_30()` | `#[cfg(test)]` -- test-only, correct |
| `lib.rs:219-223` | `ExactRational::to_f64()` | `#[cfg(test)]` -- test-only, correct |

### 5.3 Deprecated Constructs

| Location | Construct | Replacement |
|----------|-----------|-------------|
| `binary_splitting.rs:70-79` | `BinarySplitState::combine_saturating()` | `BinarySplitState::combine()` returning Option |
| `cordic.rs:612-653` | `HyperbolicCordic::sqrt()` | `crate::sqrt::isqrt_newton` or `sqrt_scaled` |

### 5.4 Unused Constants

| Location | Constant | Used? |
|----------|----------|-------|
| `constants.rs:126-181` | `cordic_angles::ATAN_TABLE`, individual `ATAN_POW2_*` | NOT imported by `cordic.rs` -- cordic.rs uses its own `CORDIC_ATAN_TABLE` |
| `constants.rs:184-223` | `hyperbolic_angles::ATANH_TABLE`, `GAIN` | NOT imported by `cordic.rs` -- cordic.rs uses its own `CORDIC_ATANH_TABLE` |
| `constants.rs:226-244` | `pade::SIN_P/Q`, `pade::COS_P/Q`, `pade::LN_P/Q` | Only `pade::EXP_P/Q` used (in truth_perturber tests). SIN, COS, LN never referenced. |
| `constants.rs:247-265` | `cf_terms::*` (PI, E, SQRT2, SQRT3, PHI, LN2) | Never referenced anywhere. |
| `constants.rs:97-100` | `rational::E_APPROX`, `rational::E_1264_465` | Never referenced. |
| `constants.rs:108` | `rational::SQRT2_99_70` | Never referenced. |
| `constants.rs:116` | `rational::SQRT3_97_56` | Never referenced. |
| `constants.rs:119-122` | `rational::LN2_ROUGH`, `rational::LN2_HI` | Never referenced. |
| `lib.rs:54-58` | `scales::SCALE_DECIMAL`, `scales::SCALE_DECIMAL_18` | Never referenced outside declaration. |

**Duplicated constant tables**: The CORDIC atan table appears in BOTH `cordic.rs:38-71` AND `constants.rs:141-174`. The atanh table appears in BOTH `cordic.rs:74-107` AND `constants.rs:186-219`. These are fully duplicated -- the constants.rs versions are unreferenced dead data.

---

## 6. CROSS-REFERENCE CHECK: Import/Export Alignment

### 6.1 Module Imports

| Consumer | Imports From | Alignment |
|----------|-------------|-----------|
| `cordic.rs` | `crate::sqrt::isqrt_newton_128`, `crate::ErrorBound` | OK |
| `agm.rs` | `crate::sqrt::isqrt_newton_128` | OK |
| `binary_splitting.rs` | (none from other modules) | OK |
| `continued_fraction.rs` | `crate::sqrt::isqrt_newton`, `crate::ExactRational` | OK |
| `sqrt.rs` | `crate::ExactRational` | OK |
| `crt.rs` | `crate::bigint::HCVLangBigInt` | OK (feature-gated) |
| `crt_rational.rs` | `crate::bigint::HCVLangBigInt`, `crate::ExactRational` | OK (feature-gated) |
| `bigint.rs` | (no cross-module imports) | OK (self-contained) |
| `constants.rs` | (no cross-module imports) | OK |

### 6.2 Feature Gate Consistency

The `arbitrary-precision` feature gates are consistent:
- `lib.rs:42-47`: `bigint`, `crt`, `crt_rational` modules gated
- `binary_splitting.rs:128-199`: `BigBinarySplitState`, `big_binary_split` gated
- `binary_splitting.rs:204-278`: `big_exp`, `big_sin`, `big_cos` gated
- `binary_splitting.rs:490-542`: `pi_chudnovsky_big` gated
- `sqrt.rs:337-366`: `big_isqrt` gated

All feature-gated code references only other feature-gated items. No orphan references.

### 6.3 no_std Compatibility

The `alloc` imports in `lib.rs:25-33`, `bigint.rs:13-17`, `crt.rs:9-14`, `continued_fraction.rs:26-29` are correctly conditional on `not(feature = "std")`. The `#[allow(unused_imports)]` on both Vec imports in lib.rs suppresses warnings correctly since only one branch is active at a time.

---

## 7. ANOMALY CATALOGUE

### 7.1 CRITICAL: Float Usage in Non-Test Production Code

**All `f64` usage is confined to `#[cfg(test)]` blocks.** No floating-point operations exist in production code paths.

Specifically:
- `ExactRational::to_f64()` at `lib.rs:221` is `#[cfg(test)]`
- `CRTRational::to_f64()` at `crt_rational.rs:129` is `#[cfg(test)]`
- `bigint_to_f64_approx()` at `crt_rational.rs:147` is `#[cfg(test)]`
- `from_scaled_30()`, `to_scaled_30()` at `constants.rs:269-275` are `#[cfg(test)]`
- Test-local `to_scaled()` / `from_scaled()` helpers in `cordic.rs:668-673`, `agm.rs:461-466`, `binary_splitting.rs:589-594` are inside `#[cfg(test)] mod tests`

**VERDICT**: No float violations in production code. CLEAN.

### 7.2 CRITICAL: Sentinel Values as Error Indicators

The CLAUDE.md explicitly states: "no sentinel values as error indicators." However, the codebase uses sentinels in production paths:

| Location | Function | Sentinel | Severity |
|----------|----------|----------|----------|
| `cordic.rs:258` | `CordicEngine::tan()` | Returns `i64::MAX` when cos=0 | MEDIUM -- `tan_checked()` exists as alternative |
| `cordic.rs:535` | `HyperbolicCordic::ln()` | Returns `i64::MIN` for x<=0 | MEDIUM -- `ln_checked()` exists as alternative |
| `agm.rs:127,152,181,188` | `AgmEngine::ln()` | Returns `i128::MIN` for error cases | MEDIUM -- `ln_checked()` exists as alternative |
| `agm.rs:386,401` | `AgmEngine::exp()` | Returns `u128::MAX` on overflow | MEDIUM -- `exp_checked()` exists as alternative |
| `agm.rs:424` | `AgmEngine::elliptic_k()` | Returns `u128::MAX` when agm_val=0 | LOW -- no checked alternative exists |
| `sqrt.rs:227,234,245` | `inv_sqrt_scaled()`, `fast_inv_sqrt()` | Returns `u128::MAX` / `u64::MAX` for n=0 | LOW -- edge case |

The pattern is consistent: sentinel-returning functions have `_checked()` alternatives returning `TransResult<T>`. However, the un-checked versions remain public, creating a dual-API situation that could confuse callers.

### 7.3 HIGH: saturating_mul in Production Code Paths

The following production (non-test) functions use `saturating_mul`, which silently clips on overflow rather than reporting an error. This contradicts the "exact arithmetic" claim:

| Location | Function | Count | Impact |
|----------|----------|-------|--------|
| `binary_splitting.rs:298,302` | `exp_binary_split()` | 2 | Taylor terms silently clipped on overflow |
| `binary_splitting.rs:312,321,325` | `sin_binary_split()` | 3 | Taylor terms silently clipped |
| `binary_splitting.rs:335,343,347` | `cos_binary_split()` | 3 | Taylor terms silently clipped |
| `binary_splitting.rs:357,366,370` | `atan_binary_split()` | 3 | Taylor terms silently clipped |
| `binary_splitting.rs:384,393,397` | `ln2_binary_split()` | 3 | Series terms silently clipped |
| `cordic.rs:456-458` | `HyperbolicCordic::sinhcosh()` double-angle | 3 | Double-angle formula silently clipped |
| `agm.rs:96,179,309,317,318,321,332,339` | `AgmEngine` various | 8 | Pi computation, ln, AGM iteration silently clipped |
| `agm.rs:406,413` | `AgmEngine::exp()`, `elliptic_k()` | 2 | Exp compensation, elliptic integral silently clipped |

**Total**: ~30 saturating arithmetic calls in production code paths.

The `BinarySplitState::combine()` (line 55) correctly uses checked arithmetic returning `Option`, but the higher-level Taylor series functions (`exp_binary_split`, `sin_binary_split`, etc.) that directly use `saturating_mul` do NOT propagate overflow errors. This is a semantic contradiction: the library claims "exact" computation but silently clips intermediate results.

**Note**: The deprecated `combine_saturating()` at `binary_splitting.rs:71-79` is correctly marked deprecated with a note to use `combine()` instead. However, it is not called anywhere -- it is purely dead code.

### 7.4 MEDIUM: `is_multiple_of` Nightly Feature

`continued_fraction.rs:326`: `period_len.is_multiple_of(2)` uses `usize::is_multiple_of()` which was stabilized in Rust 1.85.0 (2025-02-20). If the MSRV is below 1.85, this will fail to compile.

### 7.5 LOW: Duplicated Constant Tables

As noted in section 5.4, the CORDIC atan and atanh tables are defined in both `cordic.rs` and `constants.rs`. The `constants.rs` versions (`cordic_angles::ATAN_TABLE`, `hyperbolic_angles::ATANH_TABLE`) are never imported by any production code. They should either be the single source of truth (imported by cordic.rs) or removed.

### 7.6 LOW: Panic vs Error Return Inconsistency

Several public functions panic rather than returning errors:

| Location | Function | Panic Condition |
|----------|----------|-----------------|
| `lib.rs:133` | `ExactRational::new()` | `assert!(den != 0)` |
| `lib.rs:181` | `ExactRational::div()` | `assert!(other.num != 0)` |
| `crt.rs:52-64` | `CRTBigInt::validate_moduli()` | Multiple `assert!` calls |
| `crt.rs:87-89` | `CRTBigInt::with_moduli()` | `assert!(abs_value < capacity)` |
| `crt_rational.rs:26` | `CRTRational::new()` | `assert!(!den.is_zero())` |
| `bigint.rs:245` | `HCVLangBigInt::div_rem_u64()` | `assert!(divisor != 0)` |
| `lib.rs:289` | `TranscendentalConstants::new()` | `panic!("Unsupported precision")` |

The `ExactRational` has both panicking (`div()`) and Option-returning (`checked_div()`) variants, following the same dual-API pattern as the engine functions.

### 7.7 LOW: Chudnovsky sqrt Approximation

`binary_splitting.rs:469`: `let sqrt_c = 800i128; // sqrt(640320) approximately 800.2`

This integer approximation of sqrt(640320) introduces ~0.025% error into `pi_chudnovsky()`. The same approximation appears in `pi_chudnovsky_big()` at line 530: `let sqrt_c = HCVLangBigInt::from(800i64)`. A comment at line 529 acknowledges this: "We approximate sqrt(640320) approximately 800 (integer approximation)."

For a library claiming exact arithmetic, this is a known precision limitation that should be documented more prominently.

### 7.8 INFO: Test-Only Float Usage Volume

While all f64 usage is correctly test-gated, the volume is substantial:

- 70+ lines referencing `f64`, `as f64`, or `std::f64::consts::*` across test modules
- Tests use float reference values (e.g., `std::f64::consts::PI`) to validate integer computations
- This is architecturally acceptable: tests compare integer results against known float references for validation

### 7.9 INFO: No `unsafe` Code

Zero `unsafe` blocks in the entire crate. Verified by grep across all source files.

### 7.10 INFO: No TODO/FIXME/HACK/XXX

Zero TODO, FIXME, HACK, or XXX markers in production or test code. Only a `WARNING` doc comment on the deprecated `HyperbolicCordic::sqrt()` at `cordic.rs:604`.

---

## 8. SUMMARY SCORECARD

| Category | Status | Details |
|----------|--------|---------|
| Float Violations (production) | CLEAN | Zero f64 in production paths |
| Float Usage (tests) | ACCEPTABLE | ~70 lines, all `#[cfg(test)]` |
| Cross-Validation | FUNCTIONAL | 11 cross-validation tests across 5 engines |
| Cross-Validation Gaps | NOTED | No direct AGM-CORDIC exp/ln comparison; no cos cross-val |
| Sentinel Values | MITIGATED | Dual API: sentinels + checked alternatives |
| saturating_mul in Production | CONCERN | ~30 silent saturation sites contradict "exact" claim |
| Dead Code | MODERATE | 2 fully dead functions, ~15 unused constants/tables, duplicated CORDIC tables |
| Deprecated Code | CLEAN | 2 items properly marked `#[deprecated]` |
| Unsafe Code | CLEAN | Zero `unsafe` blocks |
| Panics in Public API | ACCEPTABLE | Dual API pattern (panicking + Option-returning) |
| Feature Gating | CORRECT | All `arbitrary-precision` gates consistent |
| no_std Support | CORRECT | Conditional alloc imports properly configured |
| Test Coverage | STRONG | 4 new test suites totaling ~1005 lines in lib.rs alone |
| New vs v0.1.0 | SUBSTANTIAL | 3 new modules, 4 new test suites, checked API layer, arbitrary-precision path |

### Critical Findings

1. **saturating_mul in production** (7.3): The most significant architectural concern. The top-level Taylor series functions (`exp_binary_split`, `sin_binary_split`, etc.) and the AGM engine use silent saturation. While `BinarySplitState::combine()` was correctly migrated to checked arithmetic, the higher-level functions were not. This creates a precision cliff: small inputs work exactly, but larger inputs silently produce clipped (wrong) results with no error indication.

2. **Sentinel/checked API duality** (7.2): The codebase is in transition from sentinel-based error handling to Result-based. Both APIs are public. Callers who use `tan()` instead of `tan_checked()` get a sentinel `i64::MAX` on domain error with no type-level protection.

3. **Duplicated constants** (7.5): CORDIC angle tables are defined twice (cordic.rs and constants.rs) with the constants.rs versions completely unused. This is a maintenance hazard.

### Positive Findings

1. **Zero float in production**: The integer-only mandate is fully enforced in all non-test code.
2. **Zero unsafe**: Complete memory safety.
3. **Zero dependencies**: Self-contained, auditable.
4. **Strong cross-validation**: The 11-test cross-validation suite genuinely validates independent computation paths.
5. **Comprehensive test infrastructure**: The four new test suites (cross_validation, identity_tests, truth_perturber, remediation_tests) represent a significant quality investment.
6. **Correct feature gating**: The arbitrary-precision path is cleanly separated.
7. **no_std ready**: Proper conditional compilation for embedded/WASM targets.

---

*END OF FORENSIC AUDIT*
*Auditor: Claude Opus 4.6 -- INSPECT/ANALYZE/REPORT only, no source modifications made*

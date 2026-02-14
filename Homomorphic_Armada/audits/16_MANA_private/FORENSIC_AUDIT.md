# FORENSIC AUDIT: Build 16_MANA_private

**Build**: `16_MANA_private`
**Path**: `/home/acid/Projects/Homomorphic_Armada/builds/16_MANA_private/`
**Symlink Target**: `/home/acid/Projects/NINE65/MANA-private/`
**Package**: `qmnf-security-analysis` v0.1.0 (edition 2021)
**Type**: Cryptanalysis toolkit -- self-attack suite for QMNF/MANA FHE
**Auditor**: Claude Opus 4.6
**Date**: 2026-02-13
**Classification**: INSPECT / ANALYZE / REPORT -- No modifications made

---

## 1. STRUCTURE MAPPING

### 1.1 Source Tree

```
qmnf-security-analysis/
  Cargo.toml                         # Package manifest, 7 binary targets, 0 dependencies
  Cargo.lock                         # Lockfile (single package, no external deps)
  .gitignore
  README.md
  SECURITY_ANALYSIS_REPORT.md
  CRYPTANALYSIS_PROOF.md
  src/
    main.rs                          # Binary: attack-estimator (556 lines)
    k_elimination_attack.rs          # Binary: k-elimination-attack (301 lines)
    lattice_attack.rs                # Binary: lattice-attack (501 lines)
    calibrated_estimator.rs          # Binary: calibrated-estimator (655 lines)
    self_cryptanalysis.rs            # Binary: self-cryptanalysis (378 lines)
    break_attempt.rs                 # Binary: break-attempt (356 lines) [NOT COMPILED]
  testbed/
    redshirt_testbed.rs              # Binary: redshirt-testbed (404 lines)
```

### 1.2 Cargo.toml Binary Targets

| Binary Name | Source Path | Compiled Binary Exists? |
|---|---|---|
| `attack-estimator` | `src/main.rs` | YES (`target/release/attack-estimator`) |
| `k-elimination-attack` | `src/k_elimination_attack.rs` | YES (`target/release/k-elimination-attack`) |
| `lattice-attack` | `src/lattice_attack.rs` | YES (`target/release/lattice-attack`) |
| `calibrated-estimator` | `src/calibrated_estimator.rs` | YES (`target/release/calibrated-estimator`) |
| `self-cryptanalysis` | `src/self_cryptanalysis.rs` | YES (`target/release/self-cryptanalysis`) |
| `redshirt-testbed` | `testbed/redshirt_testbed.rs` | YES (`target/release/redshirt-testbed`) |
| `break-attempt` | `src/break_attempt.rs` | **NO** -- no fingerprint, no .d file, no binary |

**Finding [STRUCT-01]**: 7 binary targets declared in Cargo.toml; 6 have release build artifacts. `break-attempt` has NO compiled output at all -- no fingerprint directory, no `.d` dependency file, no binary in `target/release/`. This is anomalous. Either it was added after the last `cargo build` or it fails compilation silently.

### 1.3 Dependencies

```toml
[dependencies]
# No external dependencies - we implement everything ourselves for verifiability
```

Zero external crate dependencies. The Cargo.lock confirms only the single `qmnf-security-analysis` package. This is architecturally deliberate for auditability.

---

## 2. DATA FLOW TRACING

### 2.1 Binary: `attack-estimator` (src/main.rs)

**Purpose**: Estimate attack costs against QMNF/MANA FHE parameter sets using standard lattice cryptanalysis models.

**Inputs** (hardcoded):
- 5 parameter configurations: `light` (N=1024), `he_standard_128` (N=2048), `standard_128` (N=4096), `high_192` (N=8192), `deep_128` (N=16384)
- All use q=998244353, various sigma/eta values
- Defined at `main.rs:458-498`

**Analysis Pipeline**:
1. `primal_attack()` (line 99): BKZ-based uSVP lattice reduction cost
2. `dual_attack()` (line 148): Dual lattice distinguishing attack cost
3. `hybrid_attack()` (line 193): Meet-in-the-middle + lattice reduction
4. `he_standard_estimate()` (line 248): HE Standard v1.1 ratio-based lookup
5. `qmnf_specific_attacks()` (line 281): K-Elimination inversion, RNS correlation, exact arithmetic exploitation

**Outputs**: Formatted table to stdout with classical/quantum security bits per configuration. Summary table at end. No file I/O.

**Data flow**: Parameters struct -> individual attack functions -> AttackCost structs -> formatted println.

### 2.2 Binary: `k-elimination-attack` (src/k_elimination_attack.rs)

**Purpose**: Probe K-Elimination division algorithm for structural weaknesses.

**Inputs** (hardcoded):
- RNS moduli: [998244353, 985661441] (line 253)
- Anchor: 754974721 (line 254)
- Test value X: 1234567890123456789 (line 116)
- Verification values at line 268

**Analysis Pipeline**:
1. `KElimParams::new()` (line 59): Compute M, M_inv from moduli and anchor
2. `attack_inversion()` (line 98): Can X be recovered from k? Analysis: NO (M >> 2^128 uncertainty)
3. `attack_secret_leakage()` (line 143): Does k leak secret s? Analysis: NO (errors mask)
4. `attack_multiple_k()` (line 175): Correlation from multiple ciphertexts? Analysis: NO (reduces to Ring-LWE)
5. `attack_timing()` (line 217): Timing side-channels? Analysis: NO (constant-time by construction)

**Outputs**: Formatted analysis to stdout. No file I/O.

**Data flow**: Hardcoded moduli -> KElimParams -> 4 attack analyses -> printed assessment.

**Key observation**: `compute_k()` at line 68-74 computes `k = (phase_mod_a * m_inv) % a` which is the K-Elimination formula. `verify_k()` at line 78-81 confirms via direct division `x / self.m`. This is a correctness check, not an attack.

### 2.3 Binary: `lattice-attack` (src/lattice_attack.rs)

**Purpose**: Concrete lattice attack simulations -- actually attempts to break toy Ring-LWE instances using LLL.

**Inputs** (hardcoded):
- Toy instances: n=8/q=97, n=16/q=997, n=32/q=65537 (lines 337-361)
- PRNG seed: 12345 (line 199)
- Parameter configs for estimation: light/he_standard_128/standard_128/high_192 (lines 382-387)

**Analysis Pipeline**:
1. `RingLWEInstance::small_test()` (line 197): Generate toy Ring-LWE instance with deterministic PRNG
2. `primal_lattice()` (line 245): Construct primal attack lattice [qI, 0; A, I]
3. `lll_reduce()` (line 95): Full LLL implementation with Lovasz condition
4. `attack_lll()` (line 281): Search reduced basis for short vector matching secret
5. `estimate_qmnf_security()` (line 376): BKZ cost estimation for production parameters
6. `shor_comparison()` (line 435): Educational comparison showing Shor inapplicability

**Outputs**: Attack results to stdout. Expected: toy instances may break, production parameters infeasible. No file I/O.

**Key implementation detail**: The LLL implementation at line 95-153 is a genuine working LLL with Gram-Schmidt orthogonalization, size reduction, and Lovasz condition checking. Has a safety bound of `max_iterations = n * n * 100` (line 115).

### 2.4 Binary: `calibrated-estimator` (src/calibrated_estimator.rs)

**Purpose**: Validated attack estimator -- calibrates methodology against published Kyber security levels before applying to QMNF.

**Inputs** (hardcoded):
- Kyber-512/768/1024 parameters (lines 409-451)
- SEAL BFV default (line 454)
- OpenFHE BGV default (line 469)
- QMNF Light/Standard-128/High-192 (lines 484-526)

**Analysis Pipeline**:
1. `validate_estimator()` (line 532): Run primal/dual/hybrid against Kyber, compare to published
2. `analyze_all_schemes()` (line 590): Side-by-side comparison of all schemes
3. Uses ADPS16 methodology with configurable cost models (CoreSVPSieving, CoreSVPEnum, MATZOV, GateCount)

**Outputs**: Validation tables and comparison table to stdout. No file I/O.

**Distinguishing feature**: This is the most rigorous binary -- it validates its own tooling before making claims. Uses `LWEParams` struct with optional expected values for cross-checking.

### 2.5 Binary: `self-cryptanalysis` (src/self_cryptanalysis.rs)

**Purpose**: End-to-end self-cryptanalysis with validation gate -- refuses to proceed if tool validation fails.

**Inputs** (hardcoded):
- Kyber validation set: 512/768/1024 with expected 118/182/256 bits (lines 244-251)
- QMNF configs: Light-INSECURE (n=1024), revised options (n=2048, n=1024/logq=20, n=1024/logq=16), Standard-128 (n=4096), High-192 (n=8192) (lines 291-319)

**Analysis Pipeline**:
1. PART 1 (line 240): Tool validation against Kyber -- if any estimate deviates >20 bits, exits early
2. PART 2 (line 282): Attack QMNF configs including known-insecure original Light params
3. PART 3 (line 358): Conclusion with feasibility summary

**Outputs**: Comprehensive attack report to stdout. No file I/O.

**Key architectural feature**: Validation gate at line 268-271 prevents unreliable results from being presented as security claims.

### 2.6 Binary: `break-attempt` (src/break_attempt.rs) [UNCOMPILED]

**Purpose**: Aggressive break attempt against both Kyber and QMNF with quantified threat thresholds.

**Inputs** (hardcoded):
- Threat thresholds: MAX_PRACTICAL_OPS=80, NATION_STATE_OPS=100, THEORETICAL_LIMIT=128 (lines 19-21)
- Targets: Kyber-512/768/1024, QMNF-Light-INSECURE, QMNF-Light-FIXED, QMNF-Standard-128, QMNF-High-256 (lines 153-215)

**Analysis Pipeline**:
1. `simulate_bkz_attack()`: Progressive BKZ with step size 10
2. `simulate_hybrid_attack()`: Hybrid with configurable secret size
3. `simulate_algebraic_attack()`: Ring structure analysis
4. `quantum_attack_assessment()`: Quantum cost estimation

**Outputs**: Per-target attack reports, summary with "CAN WE BREAK KYBER?" and "CAN WE BREAK QMNF?" sections.

**Status**: NOT COMPILED -- see Finding [STRUCT-01].

### 2.7 Binary: `redshirt-testbed` (testbed/redshirt_testbed.rs)

**Purpose**: Multi-scheme attack sandbox -- tests attacks against RSA, AES, Kyber, and QMNF simultaneously.

**Inputs** (hardcoded):
- 8 targets including WEAK-RSA-1024, RSA-4096, KYBER-512/1024, QMNF variants, AES-256 (lines 45-134)
- Simulated ciphertexts (placeholder byte arrays)

**Analysis Pipeline**:
1. PHASE 1: Reconnaissance -- enumerate targets
2. PHASE 2: Probing -- parameter extraction and security estimation
3. PHASE 3: Attack -- feasibility determination per target
4. PHASE 4: Report -- vulnerability summary

**Outputs**: Structured pentest-style report to stdout. No file I/O.

**Key design**: Includes RSA security estimation via GNFS complexity approximation (line 150-152) and AES/Grover assessment.

---

## 3. CONSTRUCT IDENTIFICATION

### 3.1 Attack Types Implemented

| Attack | Where | Type | Concrete Implementation? |
|---|---|---|---|
| Primal uSVP (BKZ) | main.rs:99, calibrated_estimator.rs:172, self_cryptanalysis.rs:58, break_attempt.rs:39 | Lattice | Cost estimation only (except lattice_attack.rs which has actual LLL) |
| Dual Distinguishing | main.rs:148, calibrated_estimator.rs:252, self_cryptanalysis.rs:98 | Lattice | Cost estimation only |
| Hybrid MITM+Lattice | main.rs:193, calibrated_estimator.rs:307, self_cryptanalysis.rs:132, break_attempt.rs:76 | Lattice+Search | Cost estimation only |
| HE Standard Ratio | main.rs:248 | Threshold lookup | N/A (table lookup) |
| K-Elim Inversion | k_elimination_attack.rs:98 | Algebraic | Analytical argument (printed) |
| K-Elim Secret Leakage | k_elimination_attack.rs:143 | Statistical | Analytical argument (printed) |
| K-Elim Correlation | k_elimination_attack.rs:175 | Multi-query | Analytical argument (printed) |
| Timing Side-Channel | k_elimination_attack.rs:217 | Side-channel | Analytical argument (printed) |
| LLL Reduction | lattice_attack.rs:95 | Lattice | **YES -- actual working LLL** |
| Algebraic (Ring) | break_attempt.rs:105 | Algebraic | Structural check only (power-of-two test) |
| Grover Brute Force | self_cryptanalysis.rs:174 | Quantum | Cost estimation |
| RSA/GNFS | redshirt_testbed.rs:148 | Factoring | Cost estimation |
| AES/Grover | redshirt_testbed.rs:193 | Symmetric | Cost estimation |

### 3.2 Security Estimation Models

| Model | Constant | Source | Used In |
|---|---|---|---|
| Core-SVP Sieving | 0.292*b + 16.4 | Albrecht et al. 2015 | main.rs:87, calibrated_estimator.rs:96 |
| Core-SVP Sieving (simplified) | 0.292*b | | self_cryptanalysis.rs:18, break_attempt.rs:34 |
| Core-SVP Enumeration | 0.187*b*log2(b) | | main.rs:92, calibrated_estimator.rs:97 |
| MATZOV | 0.257*b + 20.0 | MATZOV 2025 | calibrated_estimator.rs:98 |
| MATZOV (variant) | 0.2570*b + 16.4 | | self_cryptanalysis.rs:23 |
| Gate Count | 0.265*b + 16.0 | | calibrated_estimator.rs:99 |
| Quantum Sieving | 0.265*b | | calibrated_estimator.rs:233, self_cryptanalysis.rs:29 |
| Quantum (rough) | classical/2 + 5 | | main.rs:131 |

### 3.3 Parameter Types

| Parameter Set | n | q | sigma | Source File(s) |
|---|---|---|---|---|
| light | 1024 | 998244353 | 1.0 (eta=2) | main.rs:460 |
| he_standard_128 | 2048 | 998244353 | 1.22 (eta=3) | main.rs:469 |
| standard_128 | 4096 | 998244353 | 1.22 (eta=3) | main.rs:478 |
| high_192 | 8192 | 998244353 | 1.22 (eta=3) | main.rs:487 |
| deep_128 | 16384 | 998244353 | 1.22 (eta=3) | main.rs:496 |
| QMNF-Light (calibrated) | 1024 | 998244353 | 1.0 | calibrated_estimator.rs:488 |
| QMNF-Standard-128 (calibrated) | 4096 | 998244353 | 1.22 | calibrated_estimator.rs:503 |
| QMNF-High-192 (calibrated) | 8192 | 998244353 | 1.22 | calibrated_estimator.rs:518 |
| QMNF-Light-INSECURE | 1024 | 2^30 | 3.2 | self_cryptanalysis.rs:291 |
| QMNF-Standard-128 (self-crypt) | 4096 | 2^30 | 3.2 | self_cryptanalysis.rs:317 |
| Kyber-512 | k=2,n=256 | 3329 | 1.0 CBD(2) | calibrated_estimator.rs:411 |
| Kyber-768 | k=3,n=256 | 3329 | 1.0 CBD(2) | calibrated_estimator.rs:426 |
| Kyber-1024 | k=4,n=256 | 3329 | 1.0 CBD(2) | calibrated_estimator.rs:441 |

---

## 4. WIRING VERIFICATION

### 4.1 Compilation Status

All 6 compiled binaries have matching fingerprint directories in `target/release/.fingerprint/`:
- `qmnf-security-analysis-ba901eb6fe2c4109` (attack-estimator)
- `qmnf-security-analysis-232681835549b63f` (k-elimination-attack)
- `qmnf-security-analysis-5091b16c45ada267` (lattice-attack)
- `qmnf-security-analysis-a8e6c88cab84fd63` (calibrated-estimator)
- `qmnf-security-analysis-e8433b01e6b44e49` (self-cryptanalysis)
- `qmnf-security-analysis-3baccf55c5c1cb89` (redshirt-testbed)

**MISSING**: No fingerprint directory for `break-attempt`. No `target/release/break-attempt` binary. No `target/release/break-attempt.d`.

### 4.2 main() Path Completeness

| Binary | main() at | Complete Path to Exit? | Terminal Output? |
|---|---|---|---|
| attack-estimator | main.rs:439 | YES -- iterates configs, prints summary, exits | YES |
| k-elimination-attack | k_elimination_attack.rs:244 | YES -- runs 4 attacks, prints assessment, exits | YES |
| lattice-attack | lattice_attack.rs:471 | YES -- runs experiments, estimates, Shor comparison, exits | YES |
| calibrated-estimator | calibrated_estimator.rs:630 | YES -- validates, analyzes all schemes, exits | YES |
| self-cryptanalysis | self_cryptanalysis.rs:226 | YES -- validates (may early-exit at 270), attacks, concludes | YES (conditional) |
| break-attempt | break_attempt.rs:276 | YES (code is complete) -- but binary was never built | YES (code) |
| redshirt-testbed | redshirt_testbed.rs:303 | YES -- 4 phases, summary, exits | YES |

### 4.3 Shared Utilities

**There are NO shared utility modules.** Each binary is a standalone file with its own copies of:
- `hermite_factor()` -- duplicated in 6 files (main.rs:62, lattice_attack.rs:173, calibrated_estimator.rs:115, self_cryptanalysis.rs:33, break_attempt.rs:27, redshirt_testbed.rs:141)
- BKZ cost functions -- duplicated with variations across files
- Primal/dual/hybrid attack estimators -- duplicated with variations across files

**Finding [WIRE-01]**: Complete absence of a shared `lib.rs` or module system. All binaries are fully self-contained. This means bug fixes or model updates must be applied in 6+ places.

**Finding [WIRE-02]**: The `calibrated_estimator.rs` uses `pub` visibility on structs and functions (LWEParams, SecretDist, CostModel, AttackResult, all attack functions, all parameter constructors) but since there is no `lib.rs`, these `pub` annotations serve no purpose. Nothing imports from this file.

---

## 5. DEAD CODE DETECTION

### 5.1 Dead Functions

| Function | File:Line | Reason |
|---|---|---|
| `lll_hermite_factor()` | lattice_attack.rs:156 | Defined but never called anywhere in the file |
| `simulate_bkz()` | lattice_attack.rs:163 | Defined but never called anywhere in the file |
| `gaussian_heuristic()` | calibrated_estimator.rs:136 | Defined but never called anywhere in the file |
| `matzov_cost()` | self_cryptanalysis.rs:23 | Defined but never called anywhere in the file |
| `Matrix::dot_product()` | lattice_attack.rs:58 | Method defined but never called |
| `Matrix::row_norm_squared()` | lattice_attack.rs:54 | Only called in `attack_lll()` indirectly -- used for norm computation |
| `AttackResult::quantum_bits()` | calibrated_estimator.rs:158 | Method defined but never called |
| `AttackResult::classical_bits()` | calibrated_estimator.rs:153 | Method defined but never called |
| `SecretDist::std_dev()` | calibrated_estimator.rs:73 | Method defined but never called |
| `SecretDist::variance()` | calibrated_estimator.rs:54 | Only called by `std_dev()` which is itself dead |

### 5.2 Dead Variables

| Variable | File:Line | Reason |
|---|---|---|
| `has_small_factors` | break_attempt.rs:111 | Computed but never used in any condition |

### 5.3 Unreachable Binary

| Binary | File | Issue |
|---|---|---|
| `break-attempt` | src/break_attempt.rs | Declared in Cargo.toml but never compiled; entire file is dead code in practice |

### 5.4 Unused Parameter Set Constructors (calibrated_estimator.rs)

The following `pub fn` constructors are defined but never called from `main()`:
- `seal_default()` (line 454) -- not called from `main()`, only used in `analyze_all_schemes()` which IS called
- `openfhe_default()` (line 469) -- same, used in `analyze_all_schemes()`

These are actually called indirectly via `analyze_all_schemes()`. Not dead code.

---

## 6. SECURITY MODEL

### 6.1 Security Assumptions Being Tested

1. **Ring-LWE Hardness**: All attack estimators fundamentally test whether the chosen (n, q, sigma) parameters make Ring-LWE intractable. The Core-SVP model (0.292*beta) is the primary cost metric.

2. **K-Elimination Soundness**: k_elimination_attack.rs tests 4 attack vectors:
   - Inversion (can k reveal X?)
   - Secret leakage (does k leak s?)
   - Multi-query correlation (do many k values help?)
   - Timing (is computation constant-time?)

3. **QMNF-Specific Structure**: main.rs:281-328 tests whether exact arithmetic or RNS channels introduce structural weaknesses beyond Ring-LWE.

4. **Post-Quantum Resistance**: All binaries include quantum cost estimates (Grover speedup). Shor's algorithm is explicitly analyzed and ruled out.

### 6.2 Attack Models

| Model | Adversary Power | Tested By |
|---|---|---|
| Passive CPA | Polynomially many ciphertexts, public params | All binaries |
| Classical unbounded | Unlimited classical computation | Primal/Dual/Hybrid in all binaries |
| Quantum (Grover) | Fault-tolerant QC with sqrt speedup | All binaries (quantum_log2 field) |
| Quantum (Shor) | Period-finding QC | lattice_attack.rs shor_comparison() -- ruled out |
| Nation-state | 2^100 operations | break_attempt.rs:20, redshirt_testbed.rs threshold |
| Side-channel (timing) | Observable computation time | k_elimination_attack.rs:217 |

### 6.3 Validation Methodology

The calibrated_estimator.rs and self_cryptanalysis.rs implement a "validate before claim" methodology:
1. Estimate Kyber-512/768/1024 security
2. Compare to published NIST results
3. If within +/-20 bits, tools are calibrated
4. Only then apply to QMNF parameters

This is methodologically sound. The self_cryptanalysis.rs even has a hard gate (line 268-271) that aborts if validation fails.

---

## 7. ANOMALY CATALOGUE

### 7.1 CRITICAL: Pervasive Float Usage in Security-Critical Code

**Finding [ANOM-01]**: 292 occurrences of `f64`/float patterns across all 6 source files.

Every security estimation in this toolkit is computed using IEEE 754 double-precision floating-point arithmetic. This is a fundamental tension with the QMNF project's integer-only mandate.

Specific instances of security-critical float usage:

| Location | Usage | Risk |
|---|---|---|
| main.rs:17 | `use std::f64::consts::{E, PI}` | Float constants in security computations |
| main.rs:26 | `sigma: f64` in Parameters struct | Error std dev as float |
| main.rs:62-68 | `hermite_factor()` entirely f64 | Core security metric computed in float |
| main.rs:85-93 | BKZ cost models use f64 | Attack cost computed in float |
| calibrated_estimator.rs:93-101 | All 4 CostModel variants return f64 | All cost models are float |
| lattice_attack.rs:64-86 | Gram-Schmidt orthogonalization in f64 | LLL correctness depends on float precision |
| lattice_attack.rs:104-111 | LLL mu coefficients in f64 | Numerical stability concern |

**Assessment**: For a security estimation tool (not a production cryptosystem), f64 usage is standard practice and matches how the lattice-estimator and NIST tools work. However, this toolkit lives in a project that explicitly forbids floating point. The tension should be acknowledged.

### 7.2 CRITICAL: Inconsistent QMNF Parameter Definitions

**Finding [ANOM-02]**: The same "QMNF" parameter sets are defined differently across binaries.

| Parameter | main.rs | calibrated_estimator.rs | self_cryptanalysis.rs | break_attempt.rs |
|---|---|---|---|---|
| QMNF-Light n | 1024 | 1024 | 1024 | 1024 |
| QMNF-Light q | 998244353 (~2^30) | 998244353 | 2^30 (implicit) | 2^30 (implicit) |
| QMNF-Light sigma | 1.0 | 1.0 | 3.2 | 3.2 |
| QMNF-Standard n | 4096 | 4096 | 4096 | 4096 |
| QMNF-Standard sigma | 1.22 | 1.22 | 3.2 | 3.2 |

The sigma values differ: main.rs and calibrated_estimator.rs use sigma derived from CBD (1.0 for eta=2, 1.22 for eta=3), while self_cryptanalysis.rs and break_attempt.rs use sigma=3.2 (a common FHE default, e.g., SEAL). This directly affects security estimates because sigma is a core input to the Hermite factor / BKZ cost computation.

**Impact**: The security numbers produced by different binaries for the same nominal "QMNF-Standard-128" configuration will differ significantly because of different sigma assumptions.

### 7.3 HIGH: Inconsistent Cost Model Constants

**Finding [ANOM-03]**: The Core-SVP sieving cost constant varies across files.

| File | Formula | Additive Constant |
|---|---|---|
| main.rs:87 | `0.292 * b + 16.4` | 16.4 |
| calibrated_estimator.rs:96 | `0.292 * b + 16.4` | 16.4 |
| self_cryptanalysis.rs:18 | `0.292 * beta` | 0 (none!) |
| break_attempt.rs:34 | `0.292 * beta` | 0 (none!) |

The self_cryptanalysis.rs and break_attempt.rs omit the +16.4 additive constant. This means their security estimates are ~16 bits LOWER than main.rs/calibrated_estimator.rs for identical parameters. For the Kyber validation in self_cryptanalysis.rs, this discrepancy may coincidentally align with published estimates but it is methodologically inconsistent.

**Similarly for MATZOV**:
| File | Formula |
|---|---|
| calibrated_estimator.rs:98 | `0.257 * b + 20.0` |
| self_cryptanalysis.rs:23 | `0.2570 * b + 16.4` |

Different additive constants (20.0 vs 16.4).

### 7.4 HIGH: Uncompiled Binary (break_attempt.rs)

**Finding [ANOM-04]**: `break-attempt` is declared in Cargo.toml (line 34-36) but has no build artifacts whatsoever. No fingerprint, no .d file, no binary. Every other binary has complete build artifacts.

Possible explanations:
- It was added to Cargo.toml after the last `cargo build --release`
- It has a compilation error not visible without running the compiler
- It was intentionally excluded from builds

Given that the code appears syntactically complete (has a `main()`, all functions look correct), the most likely explanation is that it was added after the last build.

### 7.5 MEDIUM: K-Elimination verify_k() Truncation Bug

**Finding [ANOM-05]**: In `k_elimination_attack.rs:78-81`:

```rust
fn verify_k(&self, x: u128, k: u64) -> bool {
    let expected = x / self.m;
    expected as u64 == k
}
```

The division `x / self.m` produces a `u128` result, which is then truncated to `u64` via `as u64`. If `x / self.m > u64::MAX`, this silently wraps. For the test values used (line 268: max is `params.m - 1`), `x / self.m` is always 0, so this is not triggered. But for large X values, this verification would produce false positives.

### 7.6 MEDIUM: LLL Numerical Stability

**Finding [ANOM-06]**: In `lattice_attack.rs:104-111`, the LLL algorithm computes Gram-Schmidt coefficients using f64 intermediate values cast from i128:

```rust
let dot: f64 = b.data[i].iter().zip(b.data[j].iter())
    .map(|(&a, &b_val)| a as f64 * b_val as f64).sum();
```

For lattice dimensions >= 32 with q >= 65537, the i128 values can exceed 2^53 (f64 mantissa precision limit), causing silent precision loss in the Gram-Schmidt coefficients. This would make the LLL less effective (not producing shortest vectors) but would not produce false security claims -- it would make attacks appear harder than they are, which is the conservative direction.

### 7.7 MEDIUM: Hardcoded PRNG Seed in Lattice Attack

**Finding [ANOM-07]**: `lattice_attack.rs:199`:

```rust
let mut seed = 12345u64;
```

The Ring-LWE instance generation uses a hardcoded seed with a simple LCG PRNG. This means all "random" attack instances are fully deterministic and identical across runs. For a security proof this is acceptable (reproducibility), but it means the attack is only tested against one specific instance per dimension, not a random sampling.

### 7.8 MEDIUM: Non-Ring Multiplication in Lattice Attack

**Finding [ANOM-08]**: `lattice_attack.rs:233-238`:

```rust
// For simplicity, use coefficient multiplication (not ring multiplication)
let mut b = vec![0u64; n];
for i in 0..n {
    let mut val = e[i];
    for j in 0..n {
        val += (a[j] as i64) * s[(i + n - j) % n];
    }
```

The comment says "coefficient multiplication (not ring multiplication)" but the code implements cyclic convolution via the `(i + n - j) % n` index. This is actually negacyclic convolution approximated as cyclic, which is not the correct Ring-LWE multiplication in Z_q[X]/(X^N + 1). The correct negacyclic multiplication would negate wrap-around terms. This means the toy LLL attack is not attacking the same algebraic structure as production Ring-LWE.

### 7.9 LOW: Duplicate Hermite Factor Implementations

**Finding [ANOM-09]**: The `hermite_factor()` function is implemented 6 separate times across the codebase with minor variations:

| File:Line | Lower bound for b | Return for small b |
|---|---|---|
| main.rs:62 | b < 2.0 | 2.0 |
| lattice_attack.rs:173 | b < 50.0 | 1.022 |
| calibrated_estimator.rs:115 | b < 2.0 | 2.0 |
| self_cryptanalysis.rs:33 | beta < 50 | 1.02 |
| break_attempt.rs:27 | beta < 50 | 1.02 |
| redshirt_testbed.rs:141 | beta < 50 | 1.02 |

Three different small-b return values (2.0, 1.022, 1.02) and two different threshold conditions (< 2 vs < 50). This introduces subtle differences in edge-case behavior.

### 7.10 LOW: Quantum Cost Model Inconsistency

**Finding [ANOM-10]**: The quantum security estimate uses different models across files:

| File | Quantum Formula |
|---|---|
| main.rs:131 | `best_log2 / 2.0 + 5.0` (halve + overhead) |
| calibrated_estimator.rs:233 | `0.265 * block_size` (quantum sieving) |
| self_cryptanalysis.rs:29 | `0.265 * beta` (quantum sieving) |
| break_attempt.rs:127 | `classical_bits * 0.9` (10% reduction) |

Four different quantum cost models. The main.rs model (halve + 5) is dramatically more conservative than the others.

### 7.11 LOW: Security Report Claims vs Code Reality

**Finding [ANOM-11]**: `SECURITY_ANALYSIS_REPORT.md` line 139 states:

```
| light | 1024 | 30 | 36-80 bit | ~40 bit | TESTING ONLY |
```

But `CRYPTANALYSIS_PROOF.md` line 174 states:

```
| QMNF-Light | 1024 | 29.9 | 98.2 | 74.2 | 280 |
```

The same parameter set is reported as 36-80 bit in one document and 98.2 bit in another. The discrepancy is 18-62 bits depending on which bound is compared. This stems from the sigma inconsistency documented in ANOM-02.

### 7.12 LOW: Missing Test Infrastructure

**Finding [ANOM-12]**: There are zero `#[test]` functions in the entire codebase. No unit tests, no integration tests. The verification approach is "run binary, read stdout" rather than automated test assertions.

### 7.13 INFO: Dead Variable in break_attempt.rs

**Finding [ANOM-13]**: `break_attempt.rs:111`:

```rust
let has_small_factors = n % 2 == 0 && n % 4 == 0;
```

This variable is computed but never read. It was likely intended for use in the algebraic attack assessment but was never wired in.

### 7.14 INFO: .gitignore Includes Cargo.lock

**Finding [ANOM-14]**: `.gitignore` line 3 lists `Cargo.lock`, but the file exists in the repository (it is tracked). This is a minor inconsistency -- for binary projects, Cargo recommends tracking Cargo.lock.

---

## 8. SUMMARY OF FINDINGS

### By Severity

| Severity | Count | IDs |
|---|---|---|
| CRITICAL | 2 | ANOM-01 (float usage), ANOM-02 (inconsistent params) |
| HIGH | 2 | ANOM-03 (inconsistent cost constants), ANOM-04 (uncompiled binary) |
| MEDIUM | 4 | ANOM-05 (truncation), ANOM-06 (LLL stability), ANOM-07 (fixed PRNG), ANOM-08 (wrong ring mul) |
| LOW | 4 | ANOM-09 (duplicate hermite), ANOM-10 (quantum model inconsistency), ANOM-11 (doc discrepancy), ANOM-12 (no tests) |
| INFO | 2 | ANOM-13 (dead variable), ANOM-14 (gitignore) |
| STRUCTURAL | 2 | STRUCT-01 (uncompiled binary), WIRE-01 (no shared modules), WIRE-02 (useless pub) |

### Compilation Verdict

6 of 7 declared binaries compile and have complete main() paths. The `break-attempt` binary has never been compiled in this build tree.

### Architectural Verdict

The toolkit is a collection of 7 independent binaries with no shared code, leading to significant duplication and inconsistency. The calibrated_estimator.rs is the most rigorous and well-structured module. The self_cryptanalysis.rs has the best methodology (validate-then-claim). The lattice_attack.rs contains the only concrete attack implementation (LLL) but has numerical precision concerns.

### Security Model Verdict

The overall security argument is sound: validate tools against Kyber, then apply to QMNF. However, the inconsistent parameter definitions (sigma = 1.0 vs 3.2 for the same config) and inconsistent cost model constants (+16.4 vs +0) mean that different binaries produce significantly different security numbers for the same nominal configuration. This undermines the rigor of the cross-validation approach.

---

*End of Forensic Audit*

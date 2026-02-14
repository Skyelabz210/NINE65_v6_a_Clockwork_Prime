# FORENSIC AUDIT: Build 08_FHE_v02_stable

**Build ID**: `08_FHE_v02_stable`
**Path**: `/home/acid/Projects/Homomorphic_Armada/builds/08_FHE_v02_stable/qmnf_fhe_production/`
**Type**: Stable BFV FHE with security module, KAT vectors
**Auditor**: Claude Opus 4.6 (forensic code audit)
**Date**: 2026-02-14
**Classification**: READ-ONLY INSPECTION -- NO SOURCE MODIFICATIONS MADE

---

## 1. STRUCTURE MAPPING

### 1.1 Module Tree

```
qmnf_fhe_production/
  Cargo.toml                         # Package: qmnf_fhe v0.1.0-v02
  Cargo.lock
  README.md

  src/
    lib.rs                           # Crate root, prelude, integration tests
    compiler.rs                      # FHE circuit compiler (NOT registered in lib.rs)
    kat.rs                           # [NEW v02] Known Answer Tests

    arithmetic/
      mod.rs                         # Re-exports: Montgomery, Barrett, NTT, RNS, KElimination
      montgomery.rs                  # Montgomery multiplication
      persistent_montgomery.rs       # Persistent Montgomery form (stay in domain)
      barrett.rs                     # Barrett reduction + HybridModContext
      ntt.rs                         # NTT Gen 3, negacyclic convolution
      rns.rs                         # RNS/CRT context and polynomial
      k_elimination.rs              # Exact integer division
      exact_divider.rs              # Dual-track integer reconstruction
      exact_coeff.rs                # Dual-track coefficient representation
      ct_mul_exact.rs               # Exact ciphertext multiplication

    entropy/
      mod.rs                         # Re-exports both sub-modules
      shadow.rs                      # Shadow Entropy Gen 4 (deterministic LFSR+counter)
      secure.rs                      # [NEW v02] OS CSPRNG wrapper via getrandom

    keys/
      mod.rs                         # SecretKey, PublicKey, EvaluationKey, KeySet
                                     # [NEW v02] generate_secure() methods + Zeroize

    ops/
      mod.rs                         # Re-exports encrypt, homomorphic, rns_mul
      encrypt.rs                     # BFVEncoder, BFVEncryptor, BFVDecryptor, Ciphertext
      homomorphic.rs                 # BFVEvaluator: add, sub, negate, mul, mul_plain, etc.
      rns_mul.rs                     # RNS-based ct x ct multiplication

    params/
      mod.rs                         # FHEConfig: light, standard_128, high_192, etc.
      primes.rs                      # NTT-friendly primes, GCD, mod_inverse, mod_pow
      production.rs                  # ProductionConfig128, ModulusChain, 60-bit primes
      validation.rs                  # ParameterValidator, orbital bounds check

    ring/
      mod.rs                         # Re-exports polynomial
      polynomial.rs                  # RingPolynomial with Zeroize impl

    noise/
      mod.rs                         # NoiseBudgetTracker, EMACalculator, P2QuantileEstimator,
                                     # MultiWindowNoiseDetector, NoiseDistribution
      budget.rs                      # [NEW v02 enhanced] NoiseBudget, NoiseOpType, cost estimation

    security/
      mod.rs                         # [NEW v02] LWEParams, SecurityEstimate, ConfidenceLevel

    ahop/
      mod.rs                         # AHOP quantum simulation (Fp2Element, StateVector, GroverSearch)
      grover.rs                      # Grover search algorithm
      grover_full.rs                 # Full Grover analysis with eigenvalue tracking

    bin/
      fhe_benchmarks.rs              # Benchmark binary
      crypto_audit.rs                # Cryptographic audit binary

  tests/
    proptest_fhe.rs                  # Proptest-based FHE tests
    property_tests.rs                # Algebraic property tests

  benches/
    fhe_benchmarks.rs                # Criterion benchmarks
    noise_bench.rs                   # Noise tracking benchmarks
    grover_noise_search.rs           # Grover + noise benchmarks
    criterion_fhe.rs                 # Criterion FHE operation benchmarks

  scripts/
    lwe_estimate.py                  # External LWE security estimator (sage/Python)

  proofs/
    KElimination.lean                # Lean4 formal proof

  docs/proofs/
    K_ELIMINATION_PROOF.md           # Mathematical proof document

  audit/
    PRODUCTION_REPORT.md
    BENCHMARK_RESULTS.txt
    benchmark_results.txt
    test_results.txt
    2024-12-19-session-report.md
```

### 1.2 Dependencies (Cargo.toml)

| Dependency | Version | Purpose | Used in Code? |
|-----------|---------|---------|---------------|
| `zeroize` | 1.7 (features=["derive"]) | Secure memory clearing | YES: keys/mod.rs, ring/polynomial.rs |
| `getrandom` | 0.2 | OS CSPRNG | YES: entropy/secure.rs |
| `subtle` | 2.5 | Constant-time operations | **NO: NEVER IMPORTED OR USED** |
| `sha2` | 0.10 | KAT ciphertext hashing | YES: kat.rs |

**Dev Dependencies:**

| Dependency | Version | Purpose |
|-----------|---------|---------|
| `proptest` | 1.4 | Property-based testing |
| `criterion` | 0.5 | Benchmarking |

### 1.3 Cargo Features

| Feature | Purpose | Gated in Code? |
|---------|---------|----------------|
| `secure-keygen` | "Enable OS CSPRNG key generation" | **NO: NEVER REFERENCED IN SOURCE** |

### 1.4 Build Targets

- Library crate: `qmnf_fhe`
- Binary: `noise_bench` (benches/noise_bench.rs)
- Binary: `grover_noise_search` (benches/grover_noise_search.rs)
- Benchmark: `fhe_benchmarks` (harness=false)
- Benchmark: `criterion_fhe` (harness=false)

---

## 2. DATA FLOW TRACING

### 2.1 BFV Pipeline

```
                          ENCRYPTION FLOW
  plaintext (u64)
      |
      v
  BFVEncoder::encode(m)       -->  Polynomial: [Delta*m, 0, 0, ...]
      |                             where Delta = floor(q/t)
      v
  BFVEncryptor::encrypt_poly(plaintext, harvester)
      |
      |  u <- random_ternary(n, q, harvester)     // blinding factor
      |  e1, e2 <- random_cbd(n, q, eta, harvester) // error terms
      |
      |  c0 = pk0 * u + e1 + plaintext            // NTT polynomial mul
      |  c1 = pk1 * u + e2
      v
  Ciphertext { c0, c1 }

                          DECRYPTION FLOW
  Ciphertext { c0, c1 }
      |
      v
  BFVDecryptor::decrypt_raw(ct)
      |  m_noisy = c0 + c1 * s
      v
  BFVEncoder::decode(poly)
      |  m = round(t * c / q) mod t
      |  implemented as: floor((2*t*c + q) / (2*q)) mod t
      v
  plaintext (u64)
```

### 2.2 KeySet::generate_secure() Data Flow

```
  KeySet::generate_secure(config, ntt)
      |
      +---> SecretKey::generate_secure(config)
      |       |
      |       |  secure_ternary_vector(n)      // getrandom -> rejection sample
      |       |  Convert {-1,0,1} to ring elements mod q
      |       v
      |     SecretKey { s: RingPolynomial }     // Zeroize + ZeroizeOnDrop
      |
      +---> PublicKey::generate_secure(sk, config, ntt)
      |       |
      |       |  secure_uniform_vector(n, q)   // getrandom -> rejection sample
      |       |  secure_cbd_vector(n, eta)     // getrandom -> CBD sampling
      |       |  a = random uniform polynomial
      |       |  e = CBD error polynomial
      |       |  pk0 = -a*s + e
      |       v
      |     PublicKey { pk0, pk1=a }
      |
      +---> EvaluationKey::generate_secure(sk, config, ntt)
              |
              |  ShadowHarvester::from_os_seed()   // getrandom seeds Shadow
              |  Delegates to EvaluationKey::generate(sk, config, ntt, &mut harvester)
              |
              |  for each level:
              |    a_i <- random_uniform via Shadow (OS-seeded)
              |    e_i <- random_cbd via Shadow (OS-seeded)
              |    s2_ti = s^2 * T^i
              |    b_i = -a_i*s + e_i + s2_ti
              v
            EvaluationKey { rlk: Vec<(b_i, a_i)> }
            (Custom Drop impl: zeroizes rlk coefficients)
```

**FINDING [SEC-01]**: EvaluationKey::generate_secure() does NOT use direct CSPRNG.
It creates a ShadowHarvester seeded from OS entropy, then uses that LFSR-based
PRNG for the 'a' and 'e' polynomials. This is a deliberate design choice documented
as "Eval keys are less sensitive than secret keys, so we use OS-seeded Shadow Entropy
for speed while maintaining security." The security implication is that the eval key
random polynomials are generated from a single 64-bit seed, meaning at most 2^64
distinct eval keys are possible regardless of polynomial size.

### 2.3 KAT Validation Flow

```
  STANDARD_KATS (8 vectors, all N=1024, q=998244353, t=2053)
      |
      v
  run_kat(kat: &KATVector)
      |
      |  1. FHEConfig::custom(n, [q], t, eta=2)
      |  2. NTTEngine::new(q, n)
      |  3. ShadowHarvester::with_seed(kat.seed)  // deterministic
      |  4. KeySet::generate(config, ntt, &mut harvester)  // deterministic keygen
      |  5. ShadowHarvester::with_seed(kat.seed + 1)  // fresh harvester for encrypt
      |  6. Execute operation (EncryptDecrypt | HomomorphicAdd | MulPlain | CtCtAdd)
      |  7. Compare actual vs expected_result
      v
  KATResult { name, passed, expected, actual, ct_hash }

  run_all_kats() -> Vec<KATResult>       // Runs all 8 vectors
  all_kats_passed() -> bool              // True iff all pass
```

**KAT Vector Coverage:**

| # | Name | Operation | Seed | Expected |
|---|------|-----------|------|----------|
| 1 | encrypt_decrypt_zero | EncryptDecrypt | 0xDEADBEEF | 0 |
| 2 | encrypt_decrypt_one | EncryptDecrypt | 0xDEADBEEF | 1 |
| 3 | encrypt_decrypt_42 | EncryptDecrypt | 0xDEADBEEF | 42 |
| 4 | encrypt_decrypt_max | EncryptDecrypt | 0xDEADBEEF | 2052 (t-1) |
| 5 | add_plain_100_plus_50 | HomomorphicAdd(50) | 0x12345678 | 150 |
| 6 | mul_plain_17_times_3 | MulPlain(3) | 0x87654321 | 51 |
| 7 | ct_ct_add_25_plus_75 | CtCtAdd(75) | 0xCAFEBABE | 100 |
| 8 | encrypt_decrypt_different_seed | EncryptDecrypt | 0x11111111 | 123 |

**FINDING [KAT-01]**: KAT vectors do NOT include ciphertext hash verification against
known-good reference values. The `ct_hash` field is populated during EncryptDecrypt
operations but no reference hashes are stored in the KATVector struct for comparison.
The vectors only verify plaintext correctness (decrypt result == expected), not
bit-exact ciphertext reproduction.

**FINDING [KAT-02]**: All 8 KAT vectors use identical parameters (N=1024, q=998244353,
t=2053). No KAT vectors exist for `he_standard_128`, `standard_128`, or any
multi-prime/RNS configuration. Cross-parameter coverage is absent.

**FINDING [KAT-03]**: No KAT vectors test `ct x ct` multiplication (only ct+ct add,
add_plain, mul_plain). The most complex FHE operation is not covered by KATs.

### 2.4 LWE Security Estimation Flow

```
  LWEParams::from_config(config)
      |
      |  log_q = 64 - q.leading_zeros()
      |  sigma = sqrt(eta / 2.0)         // FLOAT: as f64
      v
  LWEParams { n, log_q, sigma, error_type: CBD }
      |
      +---> he_standard_estimate()
      |       ratio = n as f64 / log_q as f64  // FLOAT
      |       Lookup against HE Standard Table 3 thresholds
      |       quantum_bits = classical * 2/3
      v
  SecurityEstimate { classical_bits, quantum_bits, best_attack, confidence, ratio }
```

---

## 3. CONSTRUCT IDENTIFICATION -- NEW vs v01

### 3.1 New Module: `security/mod.rs`

**Lines**: 359 (including tests)
**Public API**: LWEParams, SecurityEstimate, ErrorDistribution, ConfidenceLevel

**Purpose**: Provides HE Standard v1.1 security estimation based on N/log(q) ratio
lookup tables. Supports three confidence levels: Rough (heuristic), Standard (table
lookup), and Precise (external lattice-estimator run).

**Key Methods:**
- `LWEParams::from_config()` -- extract LWE parameters from FHEConfig
- `LWEParams::he_standard_estimate()` -- table-based 128/192/256-bit classification
- `LWEParams::quick_estimate()` -- heuristic: 2.6 * n / log(q)
- `LWEParams::meets_he_standard()` -- boolean check against target security
- `LWEParams::max_log_q_for_security()` -- compute max modulus for given N and target
- `LWEParams::security_rationale()` -- generate human-readable security report

**Test Coverage**: 5 tests in `mod tests`:
- test_lwe_params_from_config
- test_he_standard_estimate
- test_security_levels (5 parameter sets)
- test_max_log_q
- test_all_configs_security (4 configs)

### 3.2 New Module: `kat.rs`

**Lines**: 328 (including tests)
**Public API**: KATVector, KATOperation, KATResult, STANDARD_KATS, run_kat(),
run_all_kats(), all_kats_passed(), print_kat_results()

**Purpose**: Deterministic Known Answer Test vectors for regression testing and
cross-platform verification. Uses sha2 for ciphertext fingerprinting.

**Test Coverage**: 4 tests in `mod tests`:
- test_all_kats (run all 8 vectors)
- test_kat_deterministic (verify same seed = same result)
- test_kat_encrypt_decrypt (filter encrypt/decrypt only)
- test_kat_homomorphic_ops (filter homomorphic ops only)

### 3.3 New Module: `entropy/secure.rs`

**Lines**: 207 (including tests)
**Public API**: secure_bytes, secure_u64, secure_u128, secure_u64_bounded,
secure_ternary, secure_cbd, secure_cbd_vector, secure_uniform_vector,
secure_ternary_vector

**Purpose**: OS CSPRNG wrapper providing cryptographically secure random number
generation for key material. Uses getrandom crate.

**FINDING [SEC-02]**: `secure_ternary()` uses `secure_u64() % 4` with rejection
of value 3, yielding a 25% rejection rate. This means each ternary sample requires
~1.33 calls to `secure_u64()` on average, consuming 10.67 bytes of OS entropy per
ternary coefficient. For N=8192, a single secret key generation consumes ~87KB of
OS entropy. This is functionally correct but entropywise expensive.

**FINDING [SEC-03]**: `secure_cbd(eta)` calls `secure_u64()` for EACH of the `eta`
iterations, consuming 8 bytes per bit-pair. Only 2 bits from each 64-bit value are
used (bits 0 and 1). This is correct but wastes 62 bits per iteration. For CBD(3)
with N=8192, this consumes ~196KB of OS entropy for a single error vector.

### 3.4 Enhanced: `keys/mod.rs`

**v02 Additions:**
- `SecretKey::generate_secure()` -- uses secure_ternary_vector (CSPRNG)
- `PublicKey::generate_secure()` -- uses secure_uniform_vector + secure_cbd_vector
- `EvaluationKey::generate_secure()` -- uses OS-seeded ShadowHarvester
- `KeySet::generate_secure()` -- orchestrates all three
- `SecretKey` now derives `Zeroize` and `ZeroizeOnDrop`
- `EvaluationKey` has custom `Drop` impl that zeroizes rlk coefficients
- Import of `zeroize::{Zeroize, ZeroizeOnDrop}`
- Import of `entropy::{secure_ternary_vector, secure_uniform_vector, secure_cbd_vector}`

### 3.5 Enhanced: `ring/polynomial.rs`

**v02 Addition:**
- `impl Zeroize for RingPolynomial` -- zeroizes coeffs vector

### 3.6 Enhanced: `noise/budget.rs`

**v02 Additions:**
- `NoiseBudget` struct with config-aware cost estimation
- `NoiseOpType` enum (Encrypt, Add, AddPlain, MulPlain, MulCt, Relin, Rescale)
- `NoiseExhausted` error type
- Cost estimation functions: encrypt_cost, add_cost, mul_ct_cost, etc.
- `remaining_multiplications()` estimation

---

## 4. WIRING VERIFICATION

### 4.1 generate_secure() Wiring

| Component | Wired in lib.rs Prelude? | Wired in Integration Tests? | Wired in External Tests? |
|-----------|--------------------------|----------------------------|--------------------------|
| `KeySet::generate_secure()` | YES (via keys::KeySet) | NO (tests use `generate()` with seed) | YES (property_tests.rs:301,315) |
| `ShadowHarvester::from_os_seed()` | YES (via entropy::ShadowHarvester) | NO | YES (property_tests.rs:320) |
| `secure_bytes` | YES (prelude re-export) | NO | NO |
| `secure_u64` | YES (prelude re-export) | NO | NO |
| `secure_ternary` | YES (prelude re-export) | NO | NO |

**FINDING [WIRE-01]**: `KeySet::generate_secure()` is exported in the prelude and
documented as the production method, but the `lib.rs` integration test
`test_full_fhe_workflow` uses the deterministic `KeySet::generate()` path. The
secure path is tested only in `keys/mod.rs` unit tests and `tests/property_tests.rs`.
The lib.rs doc comment Quick Start example shows `generate_secure()` usage, but the
actual integration test does not exercise it.

### 4.2 KAT Tests Wiring

| Component | Wired? | Location |
|-----------|--------|----------|
| `kat` module | YES (pub mod kat in lib.rs) | lib.rs:106 |
| `run_all_kats()` | YES (public function) | kat.rs:237 |
| `all_kats_passed()` | YES (public function) | kat.rs:242 |
| KAT tests in test suite | YES (#[test] in kat::tests) | kat.rs:283-327 |
| KAT in integration tests | **NO** | lib.rs integration_tests does not call KATs |
| KAT in property tests | **NO** | tests/property_tests.rs does not reference KATs |
| KAT in benchmarks | **NO** | benches/ do not reference KATs |

**FINDING [WIRE-02]**: KAT vectors are self-contained within `kat.rs` and will run
via `cargo test`, but they are not invoked from the integration test suite in lib.rs
or from any external test harness. There is no automated gate that requires KATs to
pass before deployment.

### 4.3 LWE Estimation Wiring

| Component | Wired? | Location |
|-----------|--------|----------|
| `security` module | YES (pub mod security in lib.rs) | lib.rs:105 |
| `LWEParams` in prelude | YES | lib.rs:154 |
| `SecurityEstimate` in prelude | YES | lib.rs:154 |
| `ConfidenceLevel` in prelude | YES | lib.rs:154 |
| Used in FHEConfig construction | **NO** | FHEConfig does not call security estimation |
| Used in parameter validation | **NO** | validation.rs has its own inline ratio check |
| Used in integration tests | **NO** | lib.rs tests do not use security module |
| External lwe_estimate.py | YES (standalone script) | scripts/lwe_estimate.py |

**FINDING [WIRE-03]**: The security module is exported and available but is NOT wired
into any automated validation pipeline. `FHEConfig::he_standard_128()` validates
orbital bounds via `validate_params()` but does NOT invoke `LWEParams::he_standard_estimate()`
to verify its security claim. The security module and the parameter validation module
(`validation.rs`) contain DUPLICATED security estimation logic with slightly different
threshold values.

**Duplication detail:**
- `security/mod.rs` line 105: `ratio > 28.0` -> 128-bit
- `params/validation.rs` line 157: `ratio > 20.0` -> 128-bit
- `params/mod.rs` line 320: `ratio > 30.0` -> 128-bit

These three independent security estimators use different thresholds for the same
security level. The security module is the most conservative (28.0), the validation
module the least (20.0).

### 4.4 Feature Flag Wiring

**FINDING [WIRE-04]**: Cargo.toml defines `secure-keygen = []` (empty feature), but
`#[cfg(feature = "secure-keygen")]` appears NOWHERE in the source code. The feature
is vestigial. `generate_secure()` is always available unconditionally. The feature
flag serves no purpose and may confuse users who expect it to gate the secure
keygen functionality.

---

## 5. DEAD CODE ANALYSIS

### 5.1 compiler.rs -- ORPHANED MODULE

**FINDING [DEAD-01] CRITICAL**: `src/compiler.rs` (approximately 400+ lines) is NOT
registered in `lib.rs`. The file exists on disk but is not compiled as part of the
library crate. It contains:
- `OpType` enum
- `CircuitNode`, `Circuit` structs
- `NoiseModel` struct with f64 fields
- `NoiseAnalyzer` with floating-point noise simulation
- `CompilationResult` struct
- `BootstrapFreeCompiler` with parameter selection logic

The compiler module declares `#![deny(clippy::float_arithmetic)]` at the top but
then immediately uses f64 throughout (NoiseModel fields, noise estimation, speedup
calculation). This is self-contradictory -- the deny attribute would cause compilation
to fail if the file were actually compiled. Since it is orphaned, this is never caught.

This module would fail compilation under `clippy::float_arithmetic` if registered.

### 5.2 Feature: `secure-keygen`

**FINDING [DEAD-02]**: The `secure-keygen` feature in Cargo.toml is dead. See WIRE-04.

### 5.3 Dependency: `subtle`

**FINDING [DEAD-03]**: The `subtle` crate (constant-time operations, version 2.5) is
declared as a dependency but is NEVER imported or used anywhere in the source code.
The only reference to "subtle" is a println recommendation in `bin/crypto_audit.rs`
line 688: `"Use subtle crate for comparisons"`. The crate is pulled in by Cargo but
provides zero functionality. This is a phantom dependency.

### 5.4 `LWEParams::new()` and `LWEParams::quick_estimate()`

These methods are public but have no callers within the crate. They are available
for external use via the prelude but are not exercised by any internal test or
production code path. `quick_estimate()` is tested in the module's own test suite
but `new()` is only called from test code.

### 5.5 `ErrorDistribution` enum variants

`ErrorDistribution::DiscreteGaussian` and `ErrorDistribution::Ternary` are defined
but only `DiscreteGaussian` is used as a default in `LWEParams::new()`. `Ternary`
is never constructed anywhere. `CBD` is the only variant used in production via
`from_config()`.

### 5.6 `BFVEncoder::encode_vector()` / `decode_vector()`

These batching methods exist but have no callers in production code or tests within
the crate, except `decrypt_vector` which calls `decode_vector`.

### 5.7 `NoiseBudget` is not integrated into BFVEvaluator

`NoiseBudget` from `noise/budget.rs` provides config-aware noise tracking but is
not used by `BFVEvaluator` or any encryption/decryption operation. Noise budget
tracking is entirely optional and disconnected from the FHE pipeline.

---

## 6. CROSS-REFERENCE: NEW DEPENDENCY USAGE

### 6.1 `zeroize` (v1.7, features=["derive"])

**Status: ACTIVELY USED**

| Usage | File | Line | Type |
|-------|------|------|------|
| `use zeroize::{Zeroize, ZeroizeOnDrop}` | keys/mod.rs | 27 | import |
| `#[derive(Clone, Zeroize, ZeroizeOnDrop)]` on SecretKey | keys/mod.rs | 36 | derive macro |
| `b.coeffs.zeroize()` in EvaluationKey::Drop | keys/mod.rs | 221 | method call |
| `a.coeffs.zeroize()` in EvaluationKey::Drop | keys/mod.rs | 222 | method call |
| `use zeroize::Zeroize` | ring/polynomial.rs | 8 | import |
| `impl Zeroize for RingPolynomial` | ring/polynomial.rs | 21 | trait impl |

**Assessment**: Properly used. SecretKey is zeroized on drop. EvaluationKey has
manual Drop impl that zeroizes rlk coefficients. RingPolynomial implements Zeroize
so the derive macro on SecretKey works transitively.

**FINDING [SEC-04]**: PublicKey does NOT implement Zeroize or ZeroizeOnDrop. This is
correct behavior (public keys are not secret), but the `pk0 = -a*s + e` component
embeds information about the secret key `s`. While `pk0` is public by design in
RLWE, the `a` polynomial in `pk1` and the error `e` are both generated from
CSPRNG, meaning key-dependent values persist in memory after PublicKey is dropped.
This is standard practice and not a vulnerability.

### 6.2 `getrandom` (v0.2)

**Status: ACTIVELY USED**

| Usage | File | Line | Type |
|-------|------|------|------|
| `use getrandom::getrandom` | entropy/secure.rs | 15 | import |
| `getrandom(buf).expect(...)` | entropy/secure.rs | 23 | function call |

**Assessment**: Single point of use. All OS CSPRNG access flows through
`entropy::secure::secure_bytes()` which wraps `getrandom::getrandom()`. The
panic-on-failure behavior (`expect`) is documented and appropriate for a
cryptographic context where OS CSPRNG failure is unrecoverable.

### 6.3 `subtle` (v2.5)

**Status: NOT USED -- DEAD DEPENDENCY**

See DEAD-03. Zero imports, zero usage. Should be removed from Cargo.toml.

### 6.4 `sha2` (v0.10)

**Status: ACTIVELY USED**

| Usage | File | Line | Type |
|-------|------|------|------|
| `use sha2::{Sha256, Digest}` | kat.rs | 26 | import |
| `Sha256::new()` | kat.rs | 248 | construction |
| `hasher.update(c.to_le_bytes())` | kat.rs | 250 | hash update |
| `hasher.finalize().into()` | kat.rs | 252 | hash finalization |

**Assessment**: Used exclusively for KAT ciphertext fingerprinting. The hash is
computed over coefficient bytes but never compared against reference values
(see KAT-01).

---

## 7. ANOMALY CATALOGUE

### 7.1 KNOWN FLOAT: noise/budget.rs

**Location**: `src/noise/budget.rs` lines 202-203, 207-208, 231-239

```rust
pub fn remaining_bits(&self) -> f64 {
    self.remaining_mb as f64 / 1000.0    // line 202-203
}
pub fn initial_bits(&self) -> f64 {
    self.initial_mb as f64 / 1000.0      // line 207-208
}
// summary() at line 231-239 uses:
//   used as f64 / 1000.0
//   100.0 * used as f64 / self.initial_mb as f64
```

**Classification**: Display-only float. All internal computations use millibits (i64).
The f64 conversion is only for human-readable output (`summary()`, `Display` impl).
Does not affect cryptographic computation.

### 7.2 FLOAT IN SECURITY MODULE

**Location**: `src/security/mod.rs`

| Line | Expression | Classification |
|------|-----------|----------------|
| 19 | `pub sigma: f64` | Struct field: stores noise parameter |
| 47 | `pub ratio: f64` | Struct field: stores N/log(q) ratio |
| 67 | `(config.eta as f64 / 2.0).sqrt()` | Computation: CBD to sigma conversion |
| 78 | `pub fn new(n, log_q, sigma: f64)` | API: accepts f64 parameter |
| 88-89 | `self.n as f64 / self.log_q as f64` | Computation: ratio calculation |
| 105-115 | Threshold comparisons (`> 50.0`, etc.) | Logic: float comparisons |
| 137 | `2.6 * self.n as f64 / self.log_q as f64` | Computation: heuristic |
| 169-188 | Float constants (`50.0`, `38.0`, etc.) | Logic: table lookup |
| 191 | `(n as f64 / required_ratio).floor() as u32` | Computation: max log_q |

**Classification**: The entire security module is built on floating-point arithmetic.
This is a design inconsistency with the "zero floating-point guarantee" stated in
lib.rs line 19-20. However, the security module performs parameter validation at
setup time, not during cryptographic operations. The f64 usage here does not affect
runtime encryption/decryption determinism.

**FINDING [FLOAT-01]**: The security module contradicts the crate's stated "Zero
Floating-Point Guarantee" (lib.rs:19-20). The guarantee should be scoped to
"cryptographic and quantum operations" (which it is in the doc comment) but the
`LWEParams` struct stores `sigma: f64` as a first-class field, making float a
structural element of the public API, not just display output.

### 7.3 FLOAT IN PARAMETER MODULES

**Location**: `src/params/mod.rs` lines 316-322

```rust
fn estimate_security(n: usize, q: u64) -> usize {
    let log_q = 64 - q.leading_zeros();
    let ratio = n as f64 / log_q as f64;
    ...
}
```

**Location**: `src/params/validation.rs` line 153

```rust
let ratio = (n as f64) / (log_q as f64);
```

**Location**: `src/params/production.rs` line 63, 155

```rust
pub sigma: f64,    // Error std dev (conceptual, we use integer CBD)
(n as f64 / 37.5) as usize
```

**Classification**: Setup-time only. Not in cryptographic path. The `sigma: f64`
field in ProductionConfig128 is explicitly documented as "conceptual, we use integer
CBD" and is never consumed by any computation.

### 7.4 FLOAT IN NOISE MODULE

**Location**: `src/noise/mod.rs`

The P2QuantileEstimator uses f64 extensively for the P-squared algorithm:
- `p: f64` (quantile parameter) -- line 424
- `n_prime: [f64; 5]` (desired marker positions) -- line 433
- `dn: [f64; 5]` (desired position increments) -- line 436
- Parabolic interpolation method (lines 551-564) uses f64 throughout
- Linear interpolation method (lines 567-582) uses f64 throughout

**Classification**: The P2 algorithm inherently requires fractional arithmetic for
marker position tracking. The comment at line 424 acknowledges: "We use f64 ONLY
for the quantile parameter, not data." However, the parabolic and linear
interpolation methods cast `self.q[i]` (marker heights in millibits) to f64 for
computation, which does affect the quantile estimate output.

**FINDING [FLOAT-02]**: P2QuantileEstimator performs float arithmetic on actual noise
measurement data (marker heights), not just the quantile parameter. The interpolation
at lines 551-582 converts millibits values to f64, performs division and
multiplication, then casts back to i64. This could introduce non-determinism across
platforms with different FPU rounding modes. Since this module is for monitoring/
diagnostics rather than cryptographic operations, the practical impact is low.

Display-only f64 methods (`noise_bits()`, `budget_remaining_bits()`, `value_bits()`,
etc.) at lines 74-80, 270-282, 599-600, 659-681 are safe -- they only format for
human consumption.

### 7.5 FLOAT IN AHOP/GROVER MODULES

**Location**: `src/ahop/mod.rs`, `src/ahop/grover.rs`, `src/ahop/grover_full.rs`

Heavy f64 usage throughout the quantum simulation modules:
- `probability()` returns f64 (mod.rs:279)
- `optimal_iterations()` uses `(n as f64).log2()` (mod.rs:194)
- Grover analysis uses PI, sin(), cos(), log2() throughout grover_full.rs
- `GroverAnalysis` struct has numerous f64 fields

**Classification**: The AHOP quantum simulation is not part of the FHE cryptographic
pipeline. It is a separate research module for Grover search simulation. Float
usage here is expected and does not violate the zero-float guarantee for FHE ops.

### 7.6 FLOAT IN BINARIES

**Location**: `src/bin/fhe_benchmarks.rs`, `src/bin/crypto_audit.rs`

Extensive f64 for timing statistics, variance computation, entropy estimation.

**Classification**: Benchmark and audit tooling. Not library code. Float usage is
appropriate and expected for statistical analysis.

### 7.7 ANOMALY: compiler.rs Self-Contradictory Attributes

**Location**: `src/compiler.rs` lines 7-8

```rust
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]
```

The module declares `deny(clippy::float_arithmetic)` but then uses f64 on lines
120-125, 131-135, 140-147, 156, 171-172, 181, 190, 210-212, 231-241, 262-263, etc.

**FINDING [DEAD-04]**: Since `compiler.rs` is not registered in `lib.rs`, this
contradiction is never caught by the compiler or clippy. If the module were
registered, it would fail to compile under clippy.

### 7.8 ANOMALY: Inconsistent Security Thresholds

**FINDING [PARAM-01]**: Three separate implementations of N/log(q) security estimation
exist with different threshold values:

```
Module              |  128-bit threshold  |  Function
--------------------|--------------------|--------------------------
security/mod.rs     |  ratio > 28.0      |  he_standard_estimate()
params/mod.rs       |  ratio > 30.0      |  estimate_security()
params/validation.rs|  ratio > 20.0      |  estimate_security_bits()
```

The validation module is the most permissive (would classify ratio=25 as 128-bit),
while the params module is more conservative (would classify it as only 80-bit).
This means the same parameter set could be reported as different security levels
depending on which code path evaluates it.

### 7.9 ANOMALY: Missing lib.rs Registration for compiler.rs

**FINDING [STRUCT-01]**: `compiler.rs` exists in `src/` alongside `lib.rs` but is not
declared as `pub mod compiler;` or `mod compiler;` in `lib.rs`. The file is completely
invisible to the Rust compiler. No other module references it. This appears to be a
work-in-progress module that was never integrated.

### 7.10 ANOMALY: KAT Hash Fingerprints Computed But Not Verified

**FINDING [KAT-01]** (repeated for emphasis): `hash_ciphertext()` is called during
EncryptDecrypt KAT operations to compute SHA-256 of ciphertext coefficients. The hash
is stored in `KATResult.ct_hash` but `KATVector` has no `expected_ct_hash` field.
The deterministic test `test_kat_deterministic` verifies that two runs produce the
same hash, but there is no golden reference hash to catch implementation drift across
versions. This undermines the cross-version regression detection purpose of KATs.

### 7.11 ANOMALY: Production Config Contains float64 Field

**FINDING [FLOAT-03]**: `ProductionConfig128.sigma` is declared as `f64` (line 63
of production.rs). The inline comment says "conceptual, we use integer CBD" but the
field is set to `3.2` in all constructors. This float field is never consumed by
any computation -- it exists purely for documentation purposes within the struct.
It should be removed or replaced with an integer representation.

### 7.12 ANOMALY: RNS Mul Tests Ignored

**Location**: `src/ops/rns_mul.rs` lines 334, 358

Two tests are marked `#[ignore]` with comment "RNS approach needs proper ciphertext
generation in RNS space":
- `test_rns_mul_with_relin`
- `test_rns_mul_multiple_values`

**FINDING [TEST-01]**: The RNS multiplication path (`RNSEvaluator`) has incomplete
test coverage. Only `test_rns_mul_basic` runs, and it ends with a diagnostic print
rather than a hard assertion on the final result. The RNS evaluator is publicly
exported via the prelude (`RNSEvaluator`) but is essentially untested for correctness.

### 7.13 ANOMALY: Production 128-bit Test Ignored

**Location**: `src/lib.rs` line 239

```rust
#[ignore]  // Needs proper noise budget tracking for N=8192
fn test_production_128bit() { ... }
```

**FINDING [TEST-02]**: The production-grade 128-bit security test is disabled. The
comment indicates noise budget tracking needs to be properly implemented for N=8192
before this test can pass. This means the `ProductionConfig128::standard()` path
has never been verified end-to-end in the test suite.

---

## 8. SUMMARY OF FINDINGS

### Critical Findings

| ID | Severity | Description |
|----|----------|-------------|
| DEAD-01 | HIGH | `compiler.rs` is orphaned -- not registered in lib.rs, not compiled |
| DEAD-03 | MEDIUM | `subtle` crate is a phantom dependency -- declared but never used |
| WIRE-04 | MEDIUM | `secure-keygen` feature flag is vestigial -- defined but never gated on |
| PARAM-01 | MEDIUM | Three independent security estimators with inconsistent thresholds |

### Security Findings

| ID | Severity | Description |
|----|----------|-------------|
| SEC-01 | LOW | EvaluationKey::generate_secure() uses OS-seeded LFSR, not direct CSPRNG |
| SEC-02 | INFO | secure_ternary() has 25% rejection rate, high entropy consumption |
| SEC-03 | INFO | secure_cbd() wastes 62/64 bits per iteration |
| SEC-04 | INFO | PublicKey does not implement Zeroize (correct by design) |

### KAT/Testing Findings

| ID | Severity | Description |
|----|----------|-------------|
| KAT-01 | MEDIUM | KAT ciphertext hashes computed but not verified against golden references |
| KAT-02 | MEDIUM | KAT vectors only cover light config (N=1024); no multi-prime coverage |
| KAT-03 | MEDIUM | No KAT vectors for ct x ct multiplication |
| WIRE-01 | LOW | lib.rs integration tests use deterministic keygen, not secure keygen |
| WIRE-02 | LOW | KATs not invoked from integration test suite |
| WIRE-03 | LOW | Security module not wired into parameter validation pipeline |
| TEST-01 | MEDIUM | RNS multiplication path (RNSEvaluator) essentially untested |
| TEST-02 | MEDIUM | Production 128-bit test disabled (ignored) |

### Float Anomalies

| ID | Severity | Description |
|----|----------|-------------|
| FLOAT-01 | LOW | Security module uses f64 in public API (LWEParams.sigma field) |
| FLOAT-02 | LOW | P2QuantileEstimator performs float interpolation on noise data |
| FLOAT-03 | INFO | ProductionConfig128.sigma is unused f64 field |
| DEAD-04 | INFO | compiler.rs denies float_arithmetic but uses f64 (never compiled) |

### Structural Findings

| ID | Severity | Description |
|----|----------|-------------|
| STRUCT-01 | HIGH | compiler.rs is dead code, invisible to compiler |

---

## 9. DEPENDENCY AUDIT SUMMARY

```
NEW in v02:
  zeroize 1.7      USED    keys/mod.rs, ring/polynomial.rs
  getrandom 0.2    USED    entropy/secure.rs
  subtle 2.5       UNUSED  PHANTOM DEPENDENCY -- remove from Cargo.toml
  sha2 0.10        USED    kat.rs

All dependencies are from the RustCrypto ecosystem (reputable, audited crates).
No supply-chain concerns identified for used dependencies.
```

---

## 10. FILE INVENTORY

Total source files in `src/`: 25
Total test files in `tests/`: 2
Total bench files in `benches/`: 4
Total script files in `scripts/`: 1
Total proof files: 2 (1 Lean, 1 Markdown)

**Source Lines of Code (approximate):**
- Library code: ~6,500 lines (excluding tests within modules)
- Test code within modules: ~3,500 lines
- External test files: ~400 lines
- Benchmark files: ~400 lines
- Binaries: ~800 lines
- Orphaned code (compiler.rs): ~400 lines

---

*END OF FORENSIC AUDIT*
*Auditor: Claude Opus 4.6*
*No source files were modified during this audit.*

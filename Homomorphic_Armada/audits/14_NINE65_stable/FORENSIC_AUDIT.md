# Forensic Audit Report: 14_NINE65_stable

**Build:** 14_NINE65_stable
**Location:** `/home/acid/Projects/Homomorphic_Armada/builds/14_NINE65_stable/`
**Audit Date:** 2026-02-14
**Auditor:** Forensic Code Audit (Automated, Claude Opus 4.6)
**Scope:** Structure, Data Flow, Constructs, Wiring, Dead Code, Audit Report Integrity, Anomalies
**Mode:** READ-ONLY -- no source modifications performed

---

## Table of Contents

1. [Section 1: Structure Mapping](#section-1-structure-mapping)
2. [Section 2: Data Flow Tracing](#section-2-data-flow-tracing)
3. [Section 3: Construct Identification](#section-3-construct-identification)
4. [Section 4: Wiring Verification](#section-4-wiring-verification)
5. [Section 5: Dead Code Analysis](#section-5-dead-code-analysis)
6. [Section 6: Audit Report Analysis](#section-6-audit-report-analysis)
7. [Section 7: Anomaly Catalogue](#section-7-anomaly-catalogue)

---

## Section 1: Structure Mapping

### 1.1 Top-Level Layout

The build contains two parallel directories and supporting documentation:

```
14_NINE65_stable/
+-- CRYPTO_AUDIT_REPORT.md          (385 lines, security audit findings)
+-- EXECUTION_PLAN.md               (47-action plan, all phases marked complete)
+-- MANIFEST.md                     (file manifest and install paths)
+-- SESSION_REPORT.md               (session summary, 179 tests claimed)
+-- TRANSFER_GUIDE.md               (installation guide, 171 tests claimed)
+-- stable_core_xfer_to_cli.zip     (archive, not inspected)
+-- NINE65_property_testing harness.zip  (archive, not inspected)
+-- qmnf_fhe_production/            (PRIMARY: structured Rust crate)
+-- qmnf_fhe_src_no_nesting/        (SECONDARY: flat file copy, 42 files)
```

### 1.2 Production Crate Module Tree

```
qmnf_fhe_production/
+-- Cargo.toml                      (50 lines)
+-- Cargo.lock
+-- README.md
+-- src/
|   +-- lib.rs                      (363 lines, declares 10 modules)
|   +-- compiler.rs                 (414 lines, NOT DECLARED in lib.rs)
|   +-- kat.rs                      (320 lines, Known Answer Tests)
|   +-- arithmetic/
|   |   +-- mod.rs                  (re-exports: Montgomery, Barrett, NTT, RNS, KElim, ExactDiv)
|   |   +-- montgomery.rs
|   |   +-- persistent_montgomery.rs
|   |   +-- barrett.rs
|   |   +-- ntt.rs
|   |   +-- rns.rs
|   |   +-- k_elimination.rs
|   |   +-- exact_divider.rs
|   |   +-- exact_coeff.rs
|   |   +-- ct_mul_exact.rs
|   +-- entropy/
|   |   +-- mod.rs                  (re-exports shadow + secure)
|   |   +-- shadow.rs              (ShadowHarvester: LFSR+counter+MurmurHash3)
|   |   +-- secure.rs             (OS CSPRNG via getrandom)
|   +-- params/
|   |   +-- mod.rs                 (FHEConfig + 9 preset configs)
|   |   +-- primes.rs
|   |   +-- production.rs          (ProductionConfig128)
|   |   +-- validation.rs          (ParameterValidator)
|   +-- ring/
|   |   +-- mod.rs                 (re-exports RingPolynomial)
|   |   +-- polynomial.rs          (RingPolynomial with Zeroize)
|   +-- keys/
|   |   +-- mod.rs                 (SecretKey, PublicKey, EvaluationKey, KeySet)
|   +-- ops/
|   |   +-- mod.rs                 (re-exports encrypt + homomorphic)
|   |   +-- encrypt.rs             (BFVEncoder, BFVEncryptor, BFVDecryptor, Ciphertext)
|   |   +-- homomorphic.rs         (BFVEvaluator: add/sub/mul/relin)
|   |   +-- rns_mul.rs
|   +-- ahop/
|   |   +-- mod.rs                 (Fp2Element, StateVector; declares only `grover`)
|   |   +-- grover.rs              (GroverSearch, GroverStats)
|   |   +-- grover_full.rs         (NOT DECLARED in ahop/mod.rs)
|   +-- noise/
|   |   +-- mod.rs                 (EMA, NoiseBudgetTracker, P2Quantile, NoiseDistribution)
|   |   +-- budget.rs              (NoiseBudget: config-aware millibit tracking)
|   +-- security/
|   |   +-- mod.rs                 (LWEParams, SecurityEstimate, HE Standard tables)
|   +-- bin/
|   |   +-- crypto_audit.rs        (standalone audit binary)
|   |   +-- fhe_benchmarks.rs      (standalone benchmark binary)
+-- benches/
|   +-- noise_bench.rs
|   +-- grover_noise_search.rs
|   +-- fhe_benchmarks.rs          (criterion benchmark)
|   +-- criterion_fhe.rs
+-- tests/
|   +-- property_tests.rs          (14 proptest properties)
|   +-- proptest_fhe.rs            (8 proptest properties, OVERLAPS with above)
+-- proofs/
|   +-- KElimination.lean          (Lean 4 skeleton, 6 `sorry` placeholders)
+-- docs/proofs/
|   +-- K_ELIMINATION_PROOF.md
+-- scripts/
|   +-- lwe_estimate.py            (external lattice-estimator integration)
+-- audit/
    +-- BENCHMARK_RESULTS.txt
    +-- PRODUCTION_REPORT.md
    +-- 2024-12-19-session-report.md
    +-- benchmark_results.txt
    +-- test_results.txt
```

### 1.3 Dependencies (Cargo.toml)

| Dependency | Version | Purpose | Actually Used? |
|------------|---------|---------|----------------|
| `zeroize` | 1.7 (derive) | Secure memory clearing | YES -- SecretKey, RingPolynomial |
| `getrandom` | 0.2 | OS CSPRNG | YES -- entropy/secure.rs |
| `subtle` | 2.5 | Constant-time operations | **NO** -- never imported |
| `sha2` | 0.10 | KAT hashing | YES -- kat.rs |

| Dev-Dependency | Version | Purpose |
|---------------|---------|---------|
| `proptest` | 1.4 | Property-based testing |
| `criterion` | 0.5 | Benchmarking |

### 1.4 Feature Flags

| Flag | Declaration | Gated Usage in Code |
|------|-------------|---------------------|
| `secure-keygen` | Declared in `[features]` | **NONE** -- no `#[cfg(feature = "secure-keygen")]` anywhere |

### 1.5 Binary Targets

| Name | Path | Purpose |
|------|------|---------|
| `noise_bench` | `benches/noise_bench.rs` | Noise distribution benchmarking |
| `grover_noise_search` | `benches/grover_noise_search.rs` | Grover noise parameter search |

### 1.6 Benchmark Targets

| Name | Path |
|------|------|
| `fhe_benchmarks` | `benches/fhe_benchmarks.rs` |
| `criterion_fhe` | `benches/criterion_fhe.rs` |

---

## Section 2: Data Flow Tracing

### 2.1 BFV Encryption Pipeline

The core BFV pipeline flows through four phases:

```
Phase 1: PARAMETER SETUP
  FHEConfig::he_standard_128()  [params/mod.rs]
    --> n=2048, q=998244353 (30-bit NTT-friendly prime), t=500000, eta=3
    --> delta = floor(q/t) = 1996
  NTTEngine::new(q, n)          [arithmetic/ntt.rs]
    --> Precomputes roots of unity, bit-reversal permutation

Phase 2: KEY GENERATION
  KeySet::generate_secure(&config, &ntt)  [keys/mod.rs]
    --> SecretKey: ternary polynomial from OS CSPRNG (secure_ternary_vector)
    --> PublicKey: a*s + e format, a uniform, e from CBD(eta)
    --> EvaluationKey: relinearization key for ct*ct multiplication
    --> Uses entropy::secure module (getrandom)

  KeySet::generate(&config, &ntt, &mut rng)  [keys/mod.rs]
    --> Same structure but uses ShadowHarvester (deterministic)
    --> Intended for testing only

Phase 3: ENCRYPT
  BFVEncoder::encode(m)         [ops/encrypt.rs]
    --> Encodes plaintext m to delta*m polynomial
  BFVEncryptor::encrypt(m, rng) [ops/encrypt.rs]
    --> u = ternary polynomial
    --> c0 = pk0*u + e1 + delta*m   (all operations via NTT)
    --> c1 = pk1*u + e2
    --> Returns Ciphertext(c0, c1)

Phase 4: HOMOMORPHIC OPERATIONS
  BFVEvaluator                   [ops/homomorphic.rs]
    --> add(ct_a, ct_b): component-wise polynomial addition
    --> sub(ct_a, ct_b): component-wise subtraction
    --> negate(ct): negate both components
    --> add_plain(ct, m): add encoded plaintext to c0
    --> mul_plain(ct, m): scalar multiply both components
    --> mul(ct_a, ct_b):
        1. mul_no_relin: tensor product (c0*c0', c0*c1'+c1*c0', c1*c1')
        2. scale_by_t_over_q via KElimination (exact integer rescaling)
        3. relinearize: reduce degree-2 back to degree-1 via eval key
    --> Uses base-w decomposition (w=2^15) for relinearization

Phase 5: DECRYPT
  BFVDecryptor::decrypt(ct)      [ops/encrypt.rs]
    --> Computes: m_noisy = c0 + c1*s (mod q)
    --> Rounds:   m = round(m_noisy * t / q) mod t
    --> For degree-2: m_noisy = c0 + c1*s + c2*s^2
```

### 2.2 K-Elimination Data Flow

```
KElimination::for_fhe()          [arithmetic/k_elimination.rs]
  --> alpha_primes = [65521, 65519, 65497]  (3 x 16-bit primes)
  --> beta_primes = [4611686018427387847]    (1 x 62-bit prime)
  --> alpha_cap = product of alpha_primes   (~48 bits)
  --> beta_cap = beta_primes[0]             (~62 bits)
  --> Total capacity: ~110 bits
  --> alpha_inv_beta = modular inverse of alpha_cap mod beta_cap

Data Flow in scale_and_round():
  1. Input: coefficient c, modulus q, scaling factor t
  2. Compute: v_alpha = (c * t) mod alpha_cap
  3. Compute: v_beta  = (c * t) mod beta_cap
  4. Extract k = (v_beta - v_alpha) * alpha_inv_beta mod beta_cap
  5. Reconstruct V = v_alpha + k * alpha_cap
  6. Round: result = (V + q/2) / q

  Constraint: V must be < alpha_cap * beta_cap (~110 bits)
  For standard BFV (N<=4096, q=30-bit): max tensor value = ~70 bits -- SAFE
  For large q (62-bit): max tensor value = ~128 bits -- FAILS
```

### 2.3 Entropy Data Flow

```
Testing Path:
  ShadowHarvester::with_seed(seed)  [entropy/shadow.rs]
    --> LFSR state initialized from seed
    --> next_u64(): LFSR step + counter increment + MurmurHash3 mixing
    --> Used for: deterministic keygen, deterministic encryption

Production Path:
  ShadowHarvester::from_os_seed()   [entropy/shadow.rs]
    --> Calls secure_u64() to get OS random seed
    --> Otherwise identical to with_seed()

  entropy::secure::*                [entropy/secure.rs]
    --> secure_bytes(n): getrandom directly
    --> secure_u64(): 8 bytes from getrandom
    --> secure_u64_bounded(bound): rejection sampling
    --> secure_ternary(q): rejection sampling for {0, 1, q-1}
    --> secure_cbd(eta, q): centered binomial from random bits
    --> Vectors: secure_cbd_vector, secure_uniform_vector, secure_ternary_vector
```

### 2.4 Security Estimation Data Flow

```
LWEParams::from_config(&config)     [security/mod.rs]
  --> Extracts n, log_q, sigma from FHEConfig
  --> sigma computed as: sqrt(eta / 2.0)  [USES f64]

SecurityEstimate methods:
  --> he_standard_estimate(): hardcoded table lookup for n/log_q ratio
  --> quick_estimate(): 2.6 * n / log_q heuristic [USES f64]
  --> meets_he_standard(): boolean check against table
  --> max_log_q_for_security(): inverse lookup
```

### 2.5 Noise Budget Data Flow

Two parallel noise tracking systems exist:

```
System A: noise/budget.rs (Config-aware)
  NoiseBudget::new(config)
    --> initial_mb = log2(q/t) * 1000 (millibits)
    --> Operations consume budget: encrypt, add, mul, relin
    --> remaining_multiplications() estimates depth capacity
    --> Display methods use f64 for bits conversion

System B: noise/mod.rs (Standalone diagnostic)
  NoiseBudgetTracker
    --> EMACalculator: exponential moving average
    --> NoiseWindow: sliding window statistics (integer /100 scaling)
    --> P2QuantileEstimator: streaming P-squared percentile [USES f64 internally]
    --> MultiWindowNoiseDetector: multi-resolution anomaly detection
    --> NoiseDistribution: aggregates quantile estimators

  These two systems DO NOT interoperate.
  Neither system is connected to actual FHE operations.
```

---

## Section 3: Construct Identification

### 3.1 Core Cryptographic Constructs

| Construct | Location | Lines (approx.) | Purpose |
|-----------|----------|-----------------|---------|
| `FHEConfig` | `params/mod.rs` | 405 | Parameter set definition + 9 presets |
| `ProductionConfig128` | `params/production.rs` | ~200 | Multi-prime production config |
| `ParameterValidator` | `params/validation.rs` | ~200 | Orbital + HE Standard validation |
| `NTTEngine` | `arithmetic/ntt.rs` | ~550 | Negacyclic NTT forward/inverse |
| `MontgomeryContext` | `arithmetic/montgomery.rs` | ~300 | Montgomery modular multiplication |
| `PersistentMontgomery` | `arithmetic/persistent_montgomery.rs` | ~370 | Stay-in-Montgomery-form ops |
| `BarrettContext` | `arithmetic/barrett.rs` | ~200 | Barrett modular reduction |
| `RNSContext` | `arithmetic/rns.rs` | ~300 | Multi-modulus RNS representation |
| `KElimination` | `arithmetic/k_elimination.rs` | ~300 | Exact RNS division |
| `ExactDivider` | `arithmetic/exact_divider.rs` | ~200 | Exact polynomial division |
| `ExactCoeff` | `arithmetic/exact_coeff.rs` | ~200 | Exact coefficient arithmetic |
| `RingPolynomial` | `ring/polynomial.rs` | 260 | Ring element with Zeroize |
| `SecretKey` | `keys/mod.rs` | ~50 | Ternary polynomial + ZeroizeOnDrop |
| `PublicKey` | `keys/mod.rs` | ~50 | (pk0, pk1) polynomial pair |
| `EvaluationKey` | `keys/mod.rs` | ~80 | Relinearization key + custom Drop |
| `KeySet` | `keys/mod.rs` | ~60 | Aggregate of all three key types |
| `BFVEncoder` | `ops/encrypt.rs` | ~80 | Plaintext <-> polynomial encoding |
| `BFVEncryptor` | `ops/encrypt.rs` | ~60 | Public-key encryption |
| `BFVDecryptor` | `ops/encrypt.rs` | ~80 | Secret-key decryption |
| `Ciphertext` | `ops/encrypt.rs` | ~20 | (c0, c1) ciphertext pair |
| `BFVEvaluator` | `ops/homomorphic.rs` | ~300 | Homomorphic operation suite |
| `ShadowHarvester` | `entropy/shadow.rs` | 255 | Deterministic PRNG |
| `secure_*` functions | `entropy/secure.rs` | 170 | OS CSPRNG wrappers |

### 3.2 Security Module Constructs

| Construct | Location | Purpose |
|-----------|----------|---------|
| `LWEParams` | `security/mod.rs` | Parameter extraction (n, log_q, sigma) |
| `SecurityEstimate` | `security/mod.rs` | Wraps LWEParams with estimation methods |
| `ConfidenceLevel` | `security/mod.rs` | Enum: Informal, Estimated, Verified |
| `he_standard_estimate()` | `security/mod.rs` | Table-based security level lookup |
| `quick_estimate()` | `security/mod.rs` | Heuristic 2.6*n/log_q formula |
| `meets_he_standard()` | `security/mod.rs` | Boolean compliance check |
| `max_log_q_for_security()` | `security/mod.rs` | Inverse table lookup |
| `security_rationale()` | `security/mod.rs` | Human-readable security report |

### 3.3 Noise Tracking Constructs

| Construct | Location | Purpose |
|-----------|----------|---------|
| `NoiseBudget` | `noise/budget.rs` | Config-aware millibit budget |
| `NoiseOpType` | `noise/budget.rs` | Enum of FHE operation types |
| `NoiseExhausted` | `noise/budget.rs` | Error when budget reaches zero |
| `EMACalculator` | `noise/mod.rs` | Exponential moving average (integer) |
| `NoiseBudgetTracker` | `noise/mod.rs` | Full diagnostic tracker |
| `NoiseSnapshot` | `noise/mod.rs` | Point-in-time noise state |
| `NoiseWindow` | `noise/mod.rs` | Sliding window statistics |
| `P2QuantileEstimator` | `noise/mod.rs` | Streaming P-squared quantile |
| `NoiseDistribution` | `noise/mod.rs` | Aggregates multiple quantile estimators |
| `MultiWindowNoiseDetector` | `noise/mod.rs` | Multi-resolution anomaly detection |
| `NoiseAnomaly` | `noise/mod.rs` | Anomaly event record |

### 3.4 Quantum Simulation (AHOP) Constructs

| Construct | Location | Purpose |
|-----------|----------|---------|
| `Fp2Element` | `ahop/mod.rs` | Element of F_{p^2} = F_p[i]/(i^2+1) |
| `StateVector` | `ahop/mod.rs` | Quantum state vector over F_{p^2} |
| `GroverSearch` | `ahop/grover.rs` | Basic Grover search simulation |
| `GroverStats` | `ahop/grover.rs` | Per-iteration statistics |
| `GroverFull` | `ahop/grover_full.rs` | Extended Grover with eigenvalue analysis |
| `IterationData` | `ahop/grover_full.rs` | Detailed per-iteration data |
| `GroverAnalysis` | `ahop/grover_full.rs` | Full analysis result struct |

### 3.5 Compiler Constructs (DISCONNECTED)

| Construct | Location | Purpose |
|-----------|----------|---------|
| `OpType` | `compiler.rs` | Circuit operation enum |
| `CircuitNode` | `compiler.rs` | DAG node |
| `Circuit` | `compiler.rs` | DAG container |
| `NoiseModel` | `compiler.rs` | Noise parameters (ALL f64) |
| `NoiseAnalyzer` | `compiler.rs` | Static noise analysis |
| `NoiseAnalysisResult` | `compiler.rs` | Analysis output |
| `CompilationResult` | `compiler.rs` | Compiler output |
| `ParameterSelector` | `compiler.rs` | Auto parameter selection |
| `BootstrapFreeFHECompiler` | `compiler.rs` | Top-level compiler |

### 3.6 Known Answer Tests (KAT)

| KAT Vector | Type | Value Tested |
|------------|------|-------------|
| KAT-001 | encrypt_decrypt | value=42, seed=0xDEAD |
| KAT-002 | encrypt_decrypt | value=0, seed=0xBEEF |
| KAT-003 | encrypt_decrypt | value=499999, seed=0xCAFE |
| KAT-004 | encrypt_decrypt | value=12345, seed=0xFACE |
| KAT-005 | add_plain | value=100, add 50 |
| KAT-006 | mul_plain | value=7, mul 6 |
| KAT-007 | ct_ct_add | 100 + 200 |
| KAT-008 | different_seed | value=42, seed=0xBAAD |

---

## Section 4: Wiring Verification

### 4.1 Module Declaration vs. File Existence

| Module Path | Declared in lib.rs? | File Exists? | Wired? |
|-------------|---------------------|--------------|--------|
| `arithmetic` | YES | YES | WIRED |
| `entropy` | YES | YES | WIRED |
| `params` | YES | YES | WIRED |
| `ring` | YES | YES | WIRED |
| `keys` | YES | YES | WIRED |
| `ops` | YES | YES | WIRED |
| `ahop` | YES | YES | WIRED |
| `noise` | YES | YES | WIRED |
| `security` | YES | YES | WIRED |
| `kat` | YES | YES | WIRED |
| `compiler` | **NO** | YES | **NOT WIRED** |

### 4.2 Sub-Module Wiring

| Parent Module | Sub-Module | Declared? | File Exists? | Status |
|---------------|------------|-----------|--------------|--------|
| `ahop` | `grover` | YES | YES | WIRED |
| `ahop` | `grover_full` | **NO** | YES | **NOT WIRED** |
| `noise` | `budget` | YES | YES | WIRED |
| `entropy` | `shadow` | YES | YES | WIRED |
| `entropy` | `secure` | YES | YES | WIRED |
| `params` | `primes` | YES | YES | WIRED |
| `params` | `production` | YES | YES | WIRED |
| `params` | `validation` | YES | YES | WIRED |
| `ops` | `encrypt` | YES | YES | WIRED |
| `ops` | `homomorphic` | YES | YES | WIRED |
| `ops` | `rns_mul` | YES | YES | WIRED |

### 4.3 Prelude Re-exports

The prelude in `lib.rs` (lines 118-155) re-exports:

| Category | Re-exported Items | Source Module |
|----------|-------------------|---------------|
| Arithmetic | MontgomeryContext, BarrettContext, HybridModContext, NTTEngine, RNSContext, RNSPolynomial, PersistentMontgomery, PersistentPolynomial | arithmetic |
| Entropy | ShadowHarvester, secure_bytes, secure_u64, secure_ternary | entropy |
| Params | FHEConfig | params |
| Ring | RingPolynomial | ring |
| Keys | SecretKey, PublicKey, EvaluationKey, KeySet | keys |
| Ops | BFVEncoder, BFVEncryptor, BFVDecryptor, BFVEvaluator, Ciphertext | ops |
| AHOP | Fp2Element, StateVector, GroverSearch, GroverStats | ahop |
| Noise | NoiseBudgetTracker, NoiseSnapshot, EMACalculator, MultiWindowNoiseDetector, NoiseAnomaly, P2QuantileEstimator, NoiseDistribution, NoiseBudget, NoiseOpType | noise |
| Security | LWEParams, SecurityEstimate, ConfidenceLevel | security |

**Missing from prelude:** KElimination, ExactDivider, ParameterValidator, ValidationResult, ProductionConfig128, KAT functions.

### 4.4 Property Testing Connectivity

**File: `tests/property_tests.rs` (14 tests)**

| Property | Tests What | Connected To |
|----------|-----------|--------------|
| encrypt_decrypt_roundtrip | Encrypt then decrypt = original | ops/encrypt.rs |
| randomized_encryption | Different seeds produce different ciphertexts | entropy/shadow.rs |
| homo_add_correct | ct(a) + ct(b) decrypts to a+b | ops/homomorphic.rs |
| homo_add_commutative | ct(a)+ct(b) = ct(b)+ct(a) | ops/homomorphic.rs |
| homo_add_zero_identity | ct(a) + ct(0) = ct(a) | ops/homomorphic.rs |
| homo_add_plain | ct(a) + plain(b) = a+b | ops/homomorphic.rs |
| homo_mul_plain_correct | ct(a) * b = a*b | ops/homomorphic.rs |
| homo_mul_plain_identity | ct(a) * 1 = ct(a) | ops/homomorphic.rs |
| homo_mul_plain_zero | ct(a) * 0 = ct(0) | ops/homomorphic.rs |
| homo_negate_add | ct(a) + (-ct(a)) = ct(0) | ops/homomorphic.rs |
| homo_sub | ct(b) - ct(a) = b-a | ops/homomorphic.rs |
| secure_keygen_randomness | Two secure keysets differ | keys/mod.rs |
| secure_keygen_works | Secure keygen encrypt/decrypt works | keys/mod.rs + ops/ |
| ntt_invertibility | INTT(NTT(x)) = x | arithmetic/ntt.rs |

**File: `tests/proptest_fhe.rs` (8 tests)**

| Property | Tests What | Overlaps With |
|----------|-----------|---------------|
| encrypt_decrypt_roundtrip | Same as above | property_tests.rs |
| homo_add | Same as homo_add_correct | property_tests.rs |
| homo_add_commutative | Same as above | property_tests.rs |
| homo_mul_plain | Same as homo_mul_plain_correct | property_tests.rs |
| homo_add_plain | Same as above | property_tests.rs |
| double_negate | neg(neg(ct)) = ct | UNIQUE |
| homo_sub | Same as above | property_tests.rs |
| security_estimate_consistency | LWE estimate >= 0 | UNIQUE |

**Overlap assessment:** 6 of 8 tests in `proptest_fhe.rs` duplicate coverage from `property_tests.rs`. Only `double_negate` and `security_estimate_consistency` are unique.

### 4.5 Integration Test Connectivity

`lib.rs` contains 4 integration tests:

| Test | Covers | Status |
|------|--------|--------|
| `test_full_fhe_workflow` | Full pipeline: keygen -> encrypt -> add/sub/neg/add_plain/mul_plain -> decrypt | Active |
| `test_grover_integration` | AHOP Grover 4-qubit search | Active |
| `test_production_128bit` | N=8192 production config | `#[ignore]` |
| `test_benchmarks` | Timing of keygen/encrypt/decrypt/add/mul | Active |

### 4.6 Noise Tracking Wiring

**FINDING: Neither noise tracking system is wired into actual FHE operations.**

- `noise/budget.rs` (`NoiseBudget`): Provides `consume()` and cost estimation but is never called from `BFVEvaluator`, `BFVEncryptor`, or `BFVDecryptor`.
- `noise/mod.rs` (`NoiseBudgetTracker`): Comprehensive diagnostic infrastructure but never receives data from actual ciphertext operations.
- Both are exported through the prelude but require manual user integration.

### 4.7 Security Estimation Wiring

- `security/mod.rs`: Provides `LWEParams::from_config()` and estimation methods.
- Used in `params/mod.rs` estimate_security() helper (test-only context).
- Used in `params/validation.rs` `estimate_security_bits()` for parameter validation.
- Available via prelude but not automatically invoked during key generation or encryption.

---

## Section 5: Dead Code Analysis

### 5.1 Dead Files (Exist but Not Compiled)

| File | Size | Status | Reason |
|------|------|--------|--------|
| `src/compiler.rs` | 414 lines | **DEAD** | Not declared in lib.rs -- never compiled into the library |
| `src/ahop/grover_full.rs` | ~750 lines | **DEAD** | Not declared in ahop/mod.rs -- never compiled |

### 5.2 Dead Dependencies

| Dependency | Version | Status | Evidence |
|------------|---------|--------|----------|
| `subtle` | 2.5 | **DEAD** | Zero imports across entire src/ tree. Only referenced in a println string in bin/crypto_audit.rs: "Use subtle crate for comparisons." |

### 5.3 Dead Feature Flags

| Feature | Status | Evidence |
|---------|--------|----------|
| `secure-keygen` | **DEAD** | Declared in Cargo.toml but no `#[cfg(feature = "secure-keygen")]` exists anywhere in the codebase. The `generate_secure()` functions are unconditionally available. |

### 5.4 Duplicate Source Directory

The `qmnf_fhe_src_no_nesting/` directory contains 42 files that are flat copies of the production source. This appears to be a pre-restructured snapshot or transfer-friendly copy. Files include:

- All arithmetic modules (montgomery.rs, ntt.rs, rns.rs, etc.)
- All security additions (secure.rs, validation.rs, budget.rs, etc.)
- Benchmark and test files (property_tests.rs, proptest_fhe.rs, etc.)
- A separate Cargo.toml and Cargo.lock
- Documentation (README.md, PRODUCTION_REPORT.md, session report)

This directory is not part of the Cargo build and serves no compilation purpose.

### 5.5 Potentially Unused Constructs

| Construct | Location | Evidence of Use |
|-----------|----------|-----------------|
| `HybridModContext` | arithmetic/mod.rs | Re-exported in prelude but not used in any FHE operation |
| `PersistentPolynomial` | arithmetic/mod.rs | Re-exported but not used in encrypt/decrypt pipeline |
| `NoiseAnomaly` | noise/mod.rs | Defined and exported but noise detector is not connected to FHE ops |
| `MultiWindowNoiseDetector` | noise/mod.rs | Exported but not connected to any pipeline |
| `NoiseDistribution` | noise/mod.rs | Exported but standalone |
| `ProductionConfig128` | params/production.rs | Only used in `#[ignore]`d test |

### 5.6 Duplicate Test Coverage

`tests/proptest_fhe.rs` duplicates 6 of its 8 tests from `tests/property_tests.rs`. This is wasted test surface area -- 75% redundancy.

---

## Section 6: Audit Report Analysis

### 6.1 CRYPTO_AUDIT_REPORT.md vs. Actual Code

This is the most significant discrepancy in the build. The audit report was written as a pre-fix assessment, but the code contains many of the fixes. The audit report was never updated.

| Audit Claim | Audit Status | Actual Code Status | Discrepancy |
|-------------|-------------|-------------------|-------------|
| Key Zeroization | "FAIL -- NOT IMPLEMENTED" | `SecretKey` has `#[derive(Zeroize, ZeroizeOnDrop)]`; `EvaluationKey` has custom `Drop` impl with manual zeroization | **CRITICAL DISCREPANCY**: Code has the fix, audit says it is missing |
| Shadow Entropy for keys | "HIGH vulnerability" | `KeySet::generate_secure()` uses OS CSPRNG via `entropy::secure` module | **DISCREPANCY**: Fix implemented, audit not updated |
| OS CSPRNG | Not mentioned as existing | `entropy/secure.rs` exists with 8 functions using `getrandom` | **DISCREPANCY**: Module exists but audit does not acknowledge it |
| Constant-time ops | "PASS -- verified" | `subtle` crate declared but **never imported or used** | **DISCREPANCY**: Audit claims constant-time via subtle, but subtle is dead code |
| 171 tests passing | In TRANSFER_GUIDE | SESSION_REPORT claims 179 | **INCONSISTENT** test count across documents |
| HE Standard N=1024 violation | "EXCEEDS max log(q)=27" | Code has `he_standard_128()` returning N=2048, log(q)=30 -- compliant | Fix exists but audit reflects pre-fix state |
| Noise budget "VERY TIGHT" | Warning about 1-bit budget | `noise/budget.rs` implements millibit tracking; still tight but tracked | Tracking exists; underlying tightness unresolved |

### 6.2 MANIFEST.md vs. Actual Files

| Manifest Entry | Expected Path | Actual Status |
|---------------|---------------|---------------|
| `code/Cargo.toml` | `./Cargo.toml` | EXISTS at `qmnf_fhe_production/Cargo.toml` |
| `code/lib.rs` | `./src/lib.rs` | EXISTS |
| `code/entropy_secure.rs` | `./src/entropy/secure.rs` | EXISTS |
| `code/entropy_mod.rs` | `./src/entropy/mod.rs` | EXISTS |
| `code/entropy_shadow.rs` | `./src/entropy/shadow.rs` | EXISTS |
| `code/keys_mod.rs` | `./src/keys/mod.rs` | EXISTS |
| `code/ring_polynomial.rs` | `./src/ring/polynomial.rs` | EXISTS |
| `code/params_mod.rs` | `./src/params/mod.rs` | EXISTS |
| `code/noise_budget.rs` | `./src/noise/budget.rs` | EXISTS |
| `code/security_mod.rs` | `./src/security/mod.rs` | EXISTS |
| `code/kat.rs` | `./src/kat.rs` | EXISTS |
| `scripts/lwe_estimate.py` | `./scripts/lwe_estimate.py` | EXISTS |

**Manifest completeness:** The manifest only lists 11 key code files. It omits:
- All arithmetic modules (montgomery.rs, ntt.rs, rns.rs, etc.)
- ops/ modules (encrypt.rs, homomorphic.rs, rns_mul.rs)
- ahop/ modules (grover.rs, grover_full.rs)
- params/ submodules (primes.rs, production.rs, validation.rs)
- bin/ files (crypto_audit.rs, fhe_benchmarks.rs)
- benches/ files
- tests/ files
- compiler.rs
- proofs/KElimination.lean

This is because the manifest is designed as a delta transfer guide (files changed during the hardening session), not a complete file listing.

### 6.3 EXECUTION_PLAN.md vs. Code

The execution plan lists 47 actions across 6 phases, all marked COMPLETED. Key claims verified against code:

| Phase | Claim | Verified? |
|-------|-------|-----------|
| Phase 1 Action 1: Add zeroize dep | `zeroize = "1.7"` in Cargo.toml | YES |
| Phase 1 Action 2: SecretKey ZeroizeOnDrop | `#[derive(Zeroize, ZeroizeOnDrop)]` on SecretKey | YES |
| Phase 1 Action 3: RingPolynomial Zeroize | `impl Zeroize for RingPolynomial` exists | YES |
| Phase 1 Action 4: EvaluationKey Drop | Custom Drop impl exists | YES |
| Phase 1 Action 7: getrandom dep | `getrandom = "0.2"` in Cargo.toml | YES |
| Phase 1 Action 8: entropy/secure.rs | File exists, 170 lines | YES |
| Phase 1 Action 11: subtle dep | `subtle = "2.5"` in Cargo.toml | YES, but **never used** |
| Phase 1 Action 12: KeySet::generate_secure | Method exists in keys/mod.rs | YES |
| Phase 2 Action 14: he_standard_128 | `FHEConfig::he_standard_128()` exists | YES |
| Phase 2 Action 16: validate_params | `ParameterValidator` exists in validation.rs | YES |
| Phase 2 Action 18: NoiseBudget | `NoiseBudget` struct in noise/budget.rs | YES |
| Phase 3 Action 23: SecurityEstimate | `security/mod.rs` exists | YES |
| Phase 3 Action 26: lwe_estimate.py | `scripts/lwe_estimate.py` exists | YES |
| Phase 3 Action 27: KAT vectors | `kat.rs` with 8 vectors exists | YES |
| Phase 4 Action 33: Lean 4 proofs | `proofs/KElimination.lean` exists | YES, but 6 `sorry` placeholders |
| Phase 5 Action 38: Property tests | `tests/property_tests.rs` exists | YES |
| Phase 5 Action 42: Criterion benchmarks | `benches/criterion_fhe.rs` exists | YES |

### 6.4 SESSION_REPORT.md Claims

| Claim | Verified? | Notes |
|-------|-----------|-------|
| 179 tests passing | CANNOT VERIFY (no build access) | TRANSFER_GUIDE says 171 |
| 146 baseline + 33 new | Plausible given file additions | |
| Phase 1-6 all complete | Code artifacts exist for all phases | `subtle` usage is missing |
| "Lean 4 needs completion" | CONFIRMED -- 6 `sorry` in .lean file | Accurately described |

---

## Section 7: Anomaly Catalogue

### ANOMALY-01: CRITICAL -- Audit Report Contradicts Code (Key Zeroization)

**Location:** `CRYPTO_AUDIT_REPORT.md` Section 5.1 vs. `keys/mod.rs`
**Nature:** The audit report states "Key Zeroization: FAIL -- NOT IMPLEMENTED" and provides sample fix code. However, the actual code at `keys/mod.rs` contains `#[derive(Clone, Zeroize, ZeroizeOnDrop)]` on `SecretKey` and a custom `Drop` implementation on `EvaluationKey` that manually zeroizes all relinearization key components.
**Impact:** Anyone reading the audit report would believe the system has a critical unfixed vulnerability when it does not. This destroys the audit's credibility as a current status document.
**Classification:** Documentation integrity failure.

### ANOMALY-02: HIGH -- `subtle` Crate Declared But Never Used

**Location:** `Cargo.toml` line 17, entire `src/` tree
**Nature:** The `subtle` crate (version 2.5) is declared as a dependency for "constant-time operations" but is never imported anywhere in the source code. Zero occurrences of `use subtle` or `subtle::` in any .rs file. The only reference is a println string in `bin/crypto_audit.rs` that says "Use subtle crate for comparisons."
**Impact:** False confidence in constant-time guarantees. The CRYPTO_AUDIT_REPORT claims constant-time operations are "PASS -- verified" but the tool for achieving this is not actually used. The audit tool merely prints advice to use it.
**Classification:** Phantom dependency; false security claim.

### ANOMALY-03: HIGH -- `compiler.rs` Not Wired Into Library

**Location:** `src/compiler.rs` (414 lines)
**Nature:** `compiler.rs` exists in the `src/` directory but `lib.rs` does not declare `mod compiler;`. The file is never compiled as part of the library. It contains a complete bootstrap-free FHE compiler with circuit DAG, noise analysis, and parameter selection.
**Impact:** ~414 lines of dead code. The "bootstrap-free compiler" capability advertised in comments is not actually available.
**Classification:** Dead module.

### ANOMALY-04: HIGH -- `compiler.rs` Self-Contradicting Lint Attributes

**Location:** `src/compiler.rs` lines 7-8
**Nature:** The file has inner attributes `#![forbid(unsafe_code)]` and `#![deny(clippy::float_arithmetic)]` but the entire file is saturated with f64 operations (at least 25 occurrences). Fields like `add_noise_bits: f64`, `mul_noise_bits: f64`, `safety_factor: f64`, and computations like `budget = params.total_modulus_bits as f64 - self.plaintext_bits as f64` would trigger clippy errors if the file were actually compiled with clippy.
**Impact:** If this file were ever wired into lib.rs, it would either fail clippy lints or require removing the deny attribute, exposing the float violation. This is a latent compilation bomb.
**Classification:** Self-contradicting code that cannot work as declared.

### ANOMALY-05: MEDIUM -- `grover_full.rs` Not Wired Into AHOP Module

**Location:** `src/ahop/grover_full.rs` (~750 lines) vs. `src/ahop/mod.rs`
**Nature:** `ahop/mod.rs` only declares `pub mod grover;`. The file `grover_full.rs` exists in the same directory but is never declared as a submodule.
**Impact:** ~750 lines of dead code containing extended Grover analysis (eigenvalue estimation, oscillation period detection, convergence analysis). This is the more comprehensive Grover implementation but is inaccessible.
**Classification:** Dead module.

### ANOMALY-06: MEDIUM -- `secure-keygen` Feature Flag Is Dead

**Location:** `Cargo.toml` line 11, entire `src/` tree
**Nature:** The feature `secure-keygen` is declared with comment "Enable OS CSPRNG key generation" but no code anywhere uses `#[cfg(feature = "secure-keygen")]`. The `generate_secure()` methods on `KeySet`, `SecretKey`, and `PublicKey` are unconditionally compiled.
**Impact:** The feature flag serves no purpose. Users cannot selectively enable or disable secure key generation. The feature flag's existence suggests an incomplete implementation plan.
**Classification:** Dead feature flag.

### ANOMALY-07: MEDIUM -- Float Violations in Non-Display Code Paths

The `lib.rs` documentation (line 20) claims: "No f32, f64, or any floating-point types in critical paths." However, f64 usage extends beyond display/formatting in several modules:

| Module | File | Non-Display f64 Usage |
|--------|------|----------------------|
| security | `security/mod.rs:19` | `sigma: f64` struct field on `LWEParams` |
| security | `security/mod.rs:67` | `(config.eta as f64 / 2.0).sqrt()` in constructor |
| security | `security/mod.rs:89` | `self.n as f64 / self.log_q as f64` ratio computation |
| security | `security/mod.rs:137` | `2.6 * self.n as f64 / self.log_q as f64` heuristic |
| params | `params/mod.rs:316` | `n as f64 / log_q as f64` in estimate_security |
| params | `params/production.rs:63` | `sigma: f64` struct field |
| params | `params/production.rs:155` | `(n as f64 / 37.5) as usize` depth estimation |
| params | `params/validation.rs:153` | `(n as f64) / (log_q as f64)` ratio |
| ahop | `ahop/mod.rs:194` | `(self.dim as f64).log2() as usize` qubit count |
| ahop | `ahop/mod.rs:279-281` | `probability()` returns f64 |
| ahop | `ahop/grover.rs:81-82` | `(PI / 4.0) * n.sqrt()` optimal iterations |
| ahop | `ahop/grover.rs:113-124` | `max_non_target_prob` returns f64 |
| noise | `noise/mod.rs:424-582` | P2QuantileEstimator internal state is f64 |
| compiler | `compiler.rs:120-125` | All NoiseModel fields are f64 (but file is dead) |
| compiler | `compiler.rs:156-412` | All noise analysis uses f64 (but file is dead) |

**Assessment:** The "zero floating-point" claim holds for the core encrypt/decrypt/homomorphic pipeline. However, security estimation, parameter validation, AHOP quantum simulation, and noise quantile estimation all use f64 in computational (non-display) paths. Whether these count as "critical paths" is subjective, but the claim as written is not accurate for the full library.

### ANOMALY-08: MEDIUM -- Duplicate Noise Tracking Systems

**Location:** `noise/budget.rs` vs. `noise/mod.rs`
**Nature:** Two parallel noise tracking systems exist:

1. `NoiseBudget` (budget.rs): Config-aware, millibit precision, cost estimation per operation type, remaining multiplication estimation.
2. `NoiseBudgetTracker` (mod.rs): Standalone diagnostic with EMA, sliding windows, P2 quantile estimators, multi-resolution anomaly detection.

These systems do not share state, do not interoperate, and neither is connected to actual FHE operations. A user must manually call `consume()` or `record()` after each operation.

**Impact:** Confusion about which system to use. Maintenance burden of two parallel implementations. Neither provides automatic noise tracking.
**Classification:** Architectural duplication.

### ANOMALY-09: MEDIUM -- Test Count Discrepancy

**Location:** `TRANSFER_GUIDE.md` vs. `SESSION_REPORT.md`
**Nature:** TRANSFER_GUIDE.md states "Should show 171 passed" in verification commands. SESSION_REPORT.md states "Test Progression: 146 -> 179 tests (+33 new tests, all passing)" and later "LIBRARY UNIT TESTS: 171 passed, PROPERTY-BASED TESTS: 8 passed, TOTAL: 179+ tests."
**Impact:** The 171 vs 179 discrepancy is explained by TRANSFER_GUIDE counting only library unit tests while SESSION_REPORT includes property tests. However, this is not stated explicitly and creates confusion.
**Classification:** Documentation ambiguity.

### ANOMALY-10: LOW -- Lean 4 Proof Skeleton Incomplete

**Location:** `proofs/KElimination.lean`
**Nature:** The Lean 4 proof file contains 6 `sorry` placeholders (lines 46, 81, 99, 111, 138, 149). Line 205 says: "Fill in `sorry` placeholders with proofs."
**Impact:** The formal verification of K-Elimination is a skeleton only. The SESSION_REPORT correctly labels this as "SKELETON" status. No formal proof of K-Elimination soundness exists.
**Classification:** Incomplete formal verification.

### ANOMALY-11: LOW -- Duplicate Source Directory

**Location:** `qmnf_fhe_src_no_nesting/` (42 files)
**Nature:** A flat copy of source files exists alongside the structured `qmnf_fhe_production/` crate. This appears to be a transfer-friendly copy from before the module restructuring. It contains its own Cargo.toml, Cargo.lock, and all source files without directory nesting.
**Impact:** Source of confusion about which files are authoritative. Potential for stale copies diverging from production code. Increases build directory size.
**Classification:** Redundant artifact.

### ANOMALY-12: LOW -- secure_cbd Entropy Waste

**Location:** `entropy/secure.rs`, `secure_cbd()` function
**Nature:** The `secure_cbd(eta, q)` function calls `secure_u64()` for each iteration of the inner eta loop. This means for CBD(3), it calls `getrandom` 6 times (3 iterations x 2 bits needed = 6 calls to secure_u64 which returns 64 bits each). Only 2 bits are extracted from each 64-bit random value, wasting 62 bits per call.
**Impact:** Performance -- ~6x more syscalls than necessary for CBD sampling. Functionally correct but inefficient.
**Classification:** Performance inefficiency in secure path.

### ANOMALY-13: LOW -- `#[ignore]`d Production Test

**Location:** `lib.rs` line 239, `test_production_128bit`
**Nature:** The only test that exercises production-grade N=8192 parameters is marked `#[ignore]` with comment "Needs proper noise budget tracking for N=8192."
**Impact:** Production parameters are never tested in the default test suite. The production config (`ProductionConfig128`) is effectively untested at the integration level.
**Classification:** Test coverage gap.

### ANOMALY-14: LOW -- Overlapping Property Test Files

**Location:** `tests/property_tests.rs` (14 tests) vs. `tests/proptest_fhe.rs` (8 tests)
**Nature:** 6 of 8 tests in `proptest_fhe.rs` duplicate tests from `property_tests.rs`: encrypt_decrypt_roundtrip, homo_add, homo_add_commutative, homo_mul_plain, homo_add_plain, homo_sub. Only `double_negate` and `security_estimate_consistency` are unique to `proptest_fhe.rs`.
**Impact:** Wasted CI time running duplicate tests. Maintenance burden of keeping both files synchronized.
**Classification:** Test redundancy.

---

## Summary of Findings

### By Severity

| Severity | Count | Key Issues |
|----------|-------|-----------|
| CRITICAL | 1 | Audit report contradicts implemented code (ANOMALY-01) |
| HIGH | 4 | Dead `subtle` dependency (02), dead `compiler.rs` (03), self-contradicting lints (04), dead `grover_full.rs` (05) |
| MEDIUM | 5 | Dead feature flag (06), float violations in non-display code (07), duplicate noise systems (08), test count discrepancy (09), Lean proof incomplete (10) |
| LOW | 4 | Duplicate source dir (11), entropy waste (12), ignored production test (13), overlapping test files (14) |

### By Category

| Category | Anomalies |
|----------|-----------|
| Dead Code | 03, 05, 06, 11, 14 |
| Documentation Integrity | 01, 09 |
| Security Claims vs Reality | 02, 04, 07 |
| Architectural | 08 |
| Test Coverage | 13, 14 |
| Performance | 12 |
| Formal Verification | 10 |

### Structural Health Assessment

The core BFV pipeline (encrypt/decrypt/homomorphic operations) is well-structured and correctly wired. The K-Elimination integration for exact rescaling is sound within its stated bounds. Key zeroization and OS CSPRNG key generation are properly implemented despite what the audit report claims.

The primary concern is the gap between documentation claims and code reality. The CRYPTO_AUDIT_REPORT.md was written as a pre-fix assessment and never updated after fixes were applied, creating a misleading record. The `subtle` crate provides false confidence in constant-time guarantees while sitting completely unused.

Approximately 1,164 lines of dead code exist across `compiler.rs` (414 lines) and `grover_full.rs` (~750 lines), plus 42 duplicate files in the flat source directory.

---

*Forensic audit complete. No source files were modified.*

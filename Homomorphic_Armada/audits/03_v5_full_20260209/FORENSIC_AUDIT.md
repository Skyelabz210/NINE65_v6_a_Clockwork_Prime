# Forensic Audit Report: Build `03_v5_full_20260209`

**Audit Date**: 2026-02-14
**Build Path**: `/home/acid/Projects/Homomorphic_Armada/builds/03_v5_full_20260209/`
**Build Type**: Full v5 workspace snapshot (10 crates + fuzz harness)
**Auditor**: Claude Opus 4.6 (forensic code auditor)
**Scope**: Structure, data flow, construct identification, wiring, dead code, cross-references, anomalies

---

## 1. STRUCTURE MAPPING

### 1.1 Workspace Configuration

**Workspace Root**: `Cargo.toml` with `resolver = "2"`, members = `crates/*`, excludes `fuzz/`.

**Release Profile**: `opt-level = 3`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"` -- production-grade binary size and performance optimization.

**Workspace Dependencies** (shared):
- `wide = "0.7"` (SIMD), `rayon = "1.10"` (parallelism)
- `zeroize = "1.7"`, `getrandom = "0.2"`, `subtle = "2.5"`, `sha2 = "0.10"` (crypto)
- `thiserror = "1.0"`, `rand_core = "0.6"`, `rand_chacha = "0.3"`
- Dev: `criterion = "0.5"`, `proptest = "1.4"`

### 1.2 Crate Inventory (10 crates)

| # | Crate | Version | Type | Source Files | Purpose |
|---|-------|---------|------|-------------|---------|
| 1 | `nine65` | 0.1.0 (ws) | lib + 2 bins | ~50 .rs | Core BFV FHE engine |
| 2 | `clockwork-core` | 0.1.0 | lib | 8 .rs | Formal-spec RNS arithmetic |
| 3 | `exact_transcendentals` | 1.0.0 | lib | 10 .rs | Integer-only transcendentals |
| 4 | `mana` | 0.1.0 (ws) | lib | 5 .rs | Modular Anchored Number Arithmetic |
| 5 | `unhal` | 0.1.0 (ws) | lib | 3 .rs | Hardware abstraction layer |
| 6 | `nexgen_rational` | 0.1.0 (ws) | lib | 7 .rs | Exact i128 rational arithmetic |
| 7 | `fhe-service` | 0.1.0 (ws) | bin | 1 .rs | HTTP FHE evaluation service |
| 8 | `nine65-python` | 0.1.0 (ws) | cdylib | 1 .rs | Python FFI via PyO3 |
| 9 | `nine65-wasm` | 0.1.0 (ws) | cdylib | 1 .rs | WebAssembly FFI via wasm-bindgen |
| 10 | `nine65-fuzz` | 0.0.0 | bins | 5 .rs | Fuzz testing harness |

### 1.3 Module Trees Per Crate

#### nine65 (Core FHE)

```
nine65/src/
  lib.rs                          # Root: #![forbid(unsafe_code)], prelude, integration tests
  compiler.rs                     # Bootstrap-free FHE compiler (f64 allowed for offline analysis)
  errors.rs                       # Nine65Error taxonomy (17 variants, Coq-derived)
  kat.rs                          # Known Answer Tests
  accelerated.rs                  # cfg(feature="accelerated"): MANA/UNHAL integration
  v2_integration_tests.rs         # cfg(test) integration tests
  arithmetic/
    mod.rs                        # 20 submodule declarations + re-exports
    barrett.rs                    # Barrett reduction
    bounded_rns.rs                # cfg(feature="clockwork"): BoundedValue
    ct_mul_exact.rs               # Exact ciphertext multiplication
    cyclotomic_phase.rs           # Native ring trig
    exact_coeff.rs                # Dual-track coefficients
    exact_divider.rs              # K-Elimination primitive
    integer_math.rs               # Integer utilities (log2, sqrt, trig LUT)
    integer_softmax.rs            # Exact sum softmax
    k_elimination.rs              # The 60-year solution
    mobius_int.rs                  # Signed arithmetic
    montgomery.rs                 # Montgomery modular multiplication
    mq_relu.rs                    # O(1) sign detection
    ntt.rs                        # NTT engine (DFT, O(N^2))
    ntt_fft.rs                    # NTT engine FFT (O(N log N))
    order_finding.rs              # Shor's classical reduction
    pade_engine.rs                # Integer transcendentals (Pade)
    persistent_montgomery.rs      # Stay in Montgomery form
    rational_bridge.rs            # cfg(feature="exact_rational"): NexGen bridge
    rns.rs                        # RNS/CRT context
    transcendental_backend.rs     # cfg(feature="exact_transcendentals_backend"): CORDIC adapter
    valuation.rs                  # Integer utilities
  entropy/
    mod.rs                        # Entropy source orchestration
    crt_shadow.rs                 # cfg(feature="shadow-entropy")
    deterministic.rs              # cfg(any(test, feature="deterministic_rng"))
    rng_trait.rs                  # FheRng trait
    secure.rs                     # OS CSPRNG
    shadow.rs                     # ShadowHarvester (deterministic)
    wassan_noise.rs               # cfg(feature="shadow-entropy")
  keys/
    mod.rs                        # KeySet, PublicKey, SecretKey, EvaluationKey
  noise/
    mod.rs                        # NoiseBudgetTracker, P2QuantileEstimator, EMA, MultiWindow
    budget.rs                     # Config-aware noise budget
    exact_noise.rs                # cfg(feature="exact_rational")
  ops/
    mod.rs                        # 10 submodule declarations + re-exports
    batch.rs                      # CRT batch encoding
    encrypt.rs                    # BFVEncryptor, BFVDecryptor, BFVEncoder, Ciphertext
    galois.rs                     # Galois automorphisms for SIMD rotations
    gso_fhe.rs                    # Glowworm Swarm Optimization for FHE
    homomorphic.rs                # BFVEvaluator, TrackedEvaluator
    neural.rs                     # FHE neural network evaluator
    parallel.rs                   # ParallelEncryptor, ParallelDecryptor
    rns_fhe.rs                    # DualRNS FHE with K-Elimination
    rns_mul.rs                    # RNS multiplication evaluator
  params/
    mod.rs                        # FHEConfig, parameter sets
    exact_params.rs               # Exact parameter computation
    primes.rs                     # Prime tables
    production.rs                 # ProductionConfig128
    secure_configs.rs             # SecureConfig (verified security)
    security_estimator.rs         # Integer security estimation
    validation.rs                 # Parameter validation
  ring/
    mod.rs                        # RingPolynomial, PolynomialPool
    polynomial.rs                 # Ring polynomial arithmetic
    pool.rs                       # Memory pool for polynomials
  security/
    mod.rs                        # LWE security estimation
    gro_gate.rs                   # cfg(feature="clockwork")
    integrity.rs                  # cfg(feature="clockwork")
    key_manager.rs                # cfg(feature="clockwork")
    secret_data.rs                # SecretData marker trait
  bin/
    fhe_demo.rs                   # CLI demo binary
    security_estimator_baseline.rs # Estimator binary
  tests/                          # Integration test files (5)
  benches/                        # Benchmark files (4)
  examples/                       # test_mod_switch.rs
```

#### clockwork-core

```
clockwork-core/src/
  lib.rs            # Root: 8 modules, 7 re-exports
  basis.rs          # RnsBasis
  bound_tracker.rs  # Bound tracking (INV-3)
  decode_to_q.rs    # DecodeToQ
  garner.rs         # k_eliminate (Garner's algorithm)
  gearstack.rs      # GearStack
  gro.rs            # GroGate
  integrity.rs      # TripleRedundant
  key_lifecycle.rs  # KeySharePair, KeyLifecycle (D18-D21), unsafe zero_u64
```

#### exact_transcendentals

```
exact_transcendentals/src/
  lib.rs                 # Root: ExactRational, TranscendentalConstants, ErrorBound, binary_gcd
  agm.rs                 # AGM engine (quadratic convergence)
  binary_splitting.rs    # Binary splitting for hypergeometric series
  constants.rs           # Precomputed precision_30 constants, Pade coefficients
  continued_fraction.rs  # CF, sqrt_cf, pi_cf, e_cf, Pell equation
  cordic.rs              # CORDIC engine (shift-and-add), HyperbolicCordic
  sqrt.rs                # Integer sqrt (Newton-Raphson)
  bigint.rs              # cfg(feature="arbitrary-precision"): HCVLangBigInt
  crt.rs                 # cfg(feature="arbitrary-precision"): CRT operations
  crt_rational.rs        # cfg(feature="arbitrary-precision"): CRT rational
  benches/performance.rs # Criterion benchmarks
```

#### mana

```
mana/src/
  lib.rs       # Root: #![forbid(unsafe_code)], #![deny(missing_docs)]
  anchor.rs    # AnchorContext, KAnchor
  gso.rs       # GsoSwarm, QbitAgent
  lane.rs      # Lane, LaneOps
  parallel.rs  # cfg(feature="parallel"): ParallelStream
  stream.rs    # ManaStream, StreamOps
  benches/     # lane_ops.rs
  examples/    # benchmark.rs
```

#### unhal

```
unhal/src/
  lib.rs          # Root: architecture diagram
  accelerator.rs  # Accelerator, AcceleratorConfig, ExecutionMode
  batch.rs        # BatchProcessor
  pipeline.rs     # Pipeline, PipelineBuilder, Stage
  examples/       # unhal_benchmark.rs
```

#### nexgen_rational

```
nexgen_rational/src/
  lib.rs           # Root: #![forbid(unsafe_code)]
  binary_gcd.rs    # Binary GCD (Stein's algorithm)
  exact_coeff.rs   # ExactCoeff(i128) wrapper
  rat_ng/
    mod.rs         # Module root
    error.rs       # ArithmeticError
    normalize.rs   # GCD-based normalization
    ops.rs         # add, sub, mul, div operations
    policy.rs      # divide_coeff trichotomy
    types.rs       # NexGenRat, DivOut
```

#### fhe-service, nine65-python, nine65-wasm

Single-file crates. See Section 2 for details.

### 1.4 Inter-Crate Dependency Graph

```
                     nine65  (central hub)
                    /  |  \    \     \       \
                   /   |   \    \     \       \
          [opt]   /    |    \    \     \       \
       mana <----/     |     \    \    |        \
         |    accel    |  exact  clock nexgen  exact_trans
       unhal   feat    |  _rat   work  _rat    _backend
         |             |  feat   feat  feat     feat
         +-- mana      |         |
                       v         v
                  fhe-service  nine65-python  nine65-wasm
                  (depends     (depends on    (depends on
                   on nine65)   nine65)        nine65)
```

**Dependency edges (verified from Cargo.toml)**:

| From | To | Type | Feature Gate |
|------|----|------|-------------|
| nine65 | mana | optional | `accelerated` |
| nine65 | unhal | optional | `accelerated` |
| nine65 | nexgen_rational | optional | `exact_rational` |
| nine65 | clockwork-core | optional | `clockwork` |
| nine65 | exact_transcendentals | optional | `exact_transcendentals_backend` |
| unhal | mana | required | always |
| fhe-service | nine65 | required | always |
| nine65-python | nine65 | required | always (needs `python` feat for cdylib) |
| nine65-wasm | nine65 | required | always (needs `wasm` feat for cdylib) |
| nine65-fuzz | nine65 | required | `serde` |

**Note**: `exact_transcendentals` and `clockwork-core` have ZERO external crate dependencies (pure Rust). `nexgen_rational` has ZERO dependencies. These are fully standalone.

---

## 2. DATA FLOW TRACING

### 2.1 Full FHE Pipeline (nine65 core)

```
[Plaintext u64]
    |
    v
BFVEncoder::encode(m) -> Delta * m (mod q)
    |
    v
BFVEncryptor::encrypt(m, rng) -> Ciphertext { c0, c1 }
    |  c0 = pk0*u + e0 + Delta*m
    |  c1 = pk1*u + e1
    v
Ciphertext (RingPolynomial pair)
    |
    +---> BFVEvaluator::add(&ct_a, &ct_b) -> ct_sum
    +---> BFVEvaluator::sub(&ct_a, &ct_b) -> ct_diff
    +---> BFVEvaluator::mul_plain(&ct, scalar) -> ct_scaled
    +---> BFVEvaluator::mul(&ct_a, &ct_b) [deprecated, single-mod]
    +---> RNSFHEContext::mul_dual_symmetric [K-Elimination path]
    +---> GaloisEvaluator::rotate(&ct, steps, galois_keys)
    +---> BatchEncoder::encode_batch(slots) -> polynomial
    +---> FHENeuralEvaluator [neural network layer]
    |
    v
BFVDecryptor::decrypt(&ct) -> u64
    |  m = round(t * (c0 + c1*s) / q) mod t
    v
[Plaintext u64]
```

### 2.2 exact_transcendentals Integration

```
nine65::arithmetic::transcendental_backend
    |
    | cfg(feature = "exact_transcendentals_backend")
    |
    v
ExactTranscendentalBackend
    |
    +-- exp_integer(x, scale) -> Option<i128>
    +-- sin_integer(x, scale) -> Option<i128>
    +-- cos_integer(x, scale) -> Option<i128>
    +-- ln_integer(x, scale)  -> Option<i128>
    |
    | Uses to_q30/from_q30 scale conversion
    v
exact_transcendentals::cordic::CordicEngine     (sin/cos)
exact_transcendentals::cordic::HyperbolicCordic (exp/ln)
```

**Fallback**: When feature is disabled, all `try_*_integer_exact()` functions return `None`, and nine65's own `PadeEngine` (in `pade_engine.rs`) provides integer transcendentals.

### 2.3 fhe-service Layer

```
TCP Listener (127.0.0.1:8080)
    |
    v
Raw HTTP/1.1 parser (custom, no framework)
    |
    v
Router:
    GET /healthz           -> status JSON
    GET /v1/version        -> version + SecureConfig::secure_192() metadata
    GET /v1/fhe/public-key -> placeholder base64 key
    POST /v1/fhe/evaluate  -> EvaluateRequest deserialization
    GET /v1/metrics        -> Prometheus-format counters
    |
    v
handle_evaluate:
    - Validates key_id match
    - Validates ciphertexts non-empty
    - Computes noise budget from depth policy
    - Returns first input ciphertext as result (STUB)
    |
    v
HTTP Response (JSON)
```

**CRITICAL FINDING**: The `handle_evaluate` function is a **stub**. It returns the first input ciphertext unchanged (`result_ciphertext_b64 = payload.ciphertexts[0].clone()`). No actual FHE computation occurs. Latency is synthetically estimated. This is labeled "pre-production" in the code.

### 2.4 Accelerated Path (MANA/UNHAL)

```
nine65::accelerated::AcceleratedFHE
    |
    +-- rns_to_stream(&RNSPolynomial) -> ManaStream
    |       Maps RNS limbs to MANA Lanes
    |
    +-- add_rns_accelerated -> Accelerator::add_streams
    +-- sub_rns_accelerated -> Accelerator::sub_streams
    +-- mul_rns_coeffwise   -> Accelerator::mul_streams
    +-- exact_divide_rns    -> AnchorContext::exact_divide_stream
    |
    +-- stream_to_rns(&ManaStream) -> RNSPolynomial
```

All gated behind `cfg(feature = "accelerated")`.

---

## 3. CONSTRUCT IDENTIFICATION

### 3.1 Core Innovations

| Construct | Location | Purpose |
|-----------|----------|---------|
| `KElimination` | `nine65/src/arithmetic/k_elimination.rs` | Exact polynomial division via coprime anchors |
| `PersistentMontgomery` | `nine65/src/arithmetic/persistent_montgomery.rs` | Stay in Montgomery form (70x fewer conversions) |
| `NTTEngineFFT` | `nine65/src/arithmetic/ntt_fft.rs` | O(N log N) negacyclic convolution |
| `BarrettContext` | `nine65/src/arithmetic/barrett.rs` | One-cycle modular reduction |
| `ShadowHarvester` | `nine65/src/entropy/shadow.rs` | Deterministic entropy (<10ns) |
| `MobiusInt` | `nine65/src/arithmetic/mobius_int.rs` | Signed arithmetic without M/2 threshold failure |
| `IntegerSoftmax` | `nine65/src/arithmetic/integer_softmax.rs` | Exact sum guarantee softmax |
| `MQReLU` | `nine65/src/arithmetic/mq_relu.rs` | O(1) sign detection via q/2 threshold |
| `PadeEngine` | `nine65/src/arithmetic/pade_engine.rs` | Integer transcendentals (Pade approximants) |
| `CyclotomicPolynomial` | `nine65/src/arithmetic/cyclotomic_phase.rs` | Native ring trig via coefficient extraction |
| `ExactDivider` | `nine65/src/arithmetic/exact_divider.rs` | Dual-track integer reconstruction |
| `ExactCoeff` | `nine65/src/arithmetic/exact_coeff.rs` | Dual-track coefficient representation |
| `RNSContext/RNSPolynomial` | `nine65/src/arithmetic/rns.rs` | CRT parallel computation |

### 3.2 FHE Primitives

| Construct | Location | Purpose |
|-----------|----------|---------|
| `BFVEncoder` | `nine65/src/ops/encrypt.rs` | Plaintext encoding (Delta scaling) |
| `BFVEncryptor` | `nine65/src/ops/encrypt.rs` | RLWE encryption |
| `BFVDecryptor` | `nine65/src/ops/encrypt.rs` | Decryption with noise rounding |
| `BFVEvaluator` | `nine65/src/ops/homomorphic.rs` | Homomorphic operations |
| `RNSFHEContext` | `nine65/src/ops/rns_fhe.rs` | DualRNS FHE with K-Elimination |
| `GaloisEngine` | `nine65/src/ops/galois.rs` | SIMD slot rotations |
| `BatchEncoder` | `nine65/src/ops/batch.rs` | CRT batching (N/2 slots) |
| `FHENeuralEvaluator` | `nine65/src/ops/neural.rs` | FHE neural network inference |
| `GSOFHEContext` | `nine65/src/ops/gso_fhe.rs` | Glowworm Swarm Optimization FHE |

### 3.3 Security Constructs

| Construct | Location | Purpose |
|-----------|----------|---------|
| `SecureConfig` | `nine65/src/params/secure_configs.rs` | Verified production parameter sets |
| `SecureRng` | `nine65/src/entropy/secure.rs` | OS CSPRNG wrapper |
| `LWEParams` | `nine65/src/security/mod.rs` | HE Standard security estimation |
| `SecretData` | `nine65/src/security/secret_data.rs` | Constant-time marker trait |
| `NoiseBudget` | `nine65/src/noise/budget.rs` | Config-aware noise tracking |
| `KeySharePair` | `clockwork-core/src/key_lifecycle.rs` | D18 key splitting |
| `KeyLifecycle` | `clockwork-core/src/key_lifecycle.rs` | D20 state machine |
| `TripleRedundant` | `clockwork-core/src/integrity.rs` | Integrity checking |
| `TimingGate` | `nine65/src/security/gro_gate.rs` | Constant-time enforcement |
| `Nine65Error` | `nine65/src/errors.rs` | 17-variant error taxonomy |

### 3.4 Transcendental Constructs

| Construct | Location | Purpose |
|-----------|----------|---------|
| `CordicEngine` | `exact_transcendentals/src/cordic.rs` | Shift-and-add sin/cos/tan/atan |
| `HyperbolicCordic` | `exact_transcendentals/src/cordic.rs` | exp/ln/sinh/cosh |
| `AgmEngine` | `exact_transcendentals/src/agm.rs` | Arithmetic-Geometric Mean (pi, log) |
| `ExactRational` | `exact_transcendentals/src/lib.rs` | i128 rational number |
| `TranscendentalConstants` | `exact_transcendentals/src/lib.rs` | Precomputed pi, e, ln2 |
| `ContinuedFraction` | `exact_transcendentals/src/continued_fraction.rs` | CF convergents, Pell solutions |
| `binary_split` | `exact_transcendentals/src/binary_splitting.rs` | Hypergeometric series |

### 3.5 Acceleration Constructs

| Construct | Location | Purpose |
|-----------|----------|---------|
| `Lane` | `mana/src/lane.rs` | Single CRT modulus compute lane |
| `ManaStream` | `mana/src/stream.rs` | Multi-lane CRT stream |
| `AnchorContext` | `mana/src/anchor.rs` | K-Elimination anchor codex |
| `GsoSwarm` | `mana/src/gso.rs` | Glowworm Swarm Optimization |
| `ParallelStream` | `mana/src/parallel.rs` | Rayon-based parallel stream |
| `Accelerator` | `unhal/src/accelerator.rs` | Auto-detect execution mode |
| `Pipeline` | `unhal/src/pipeline.rs` | Staged compute pipeline |
| `BatchProcessor` | `unhal/src/batch.rs` | Bulk operation processing |

---

## 4. WIRING VERIFICATION

### 4.1 Feature Gates

| Feature | Crate | Dependencies Activated | Used In |
|---------|-------|----------------------|---------|
| `accelerated` | nine65 | mana, unhal | `accelerated.rs` |
| `parallel` | nine65 | rayon | `ops/parallel.rs` |
| `ntt_fft` | nine65 | (none) | `arithmetic/ntt_fft.rs`, NTTEngine alias |
| `wassan` | nine65 | shadow-entropy | Implies `shadow-entropy` |
| `v2` | nine65 | ntt_fft, wassan | Compound feature |
| `serde` | nine65 | dep:serde, dep:serde_json, dep:bincode | Serialization support |
| `shadow-entropy` | nine65 | (none) | `entropy/crt_shadow.rs`, `entropy/wassan_noise.rs` |
| `secure_seed` | nine65 | (none) | `entropy/mod.rs::secure_seed_from_os` |
| `debug_dual_mul` | nine65 | (none) | Debug output in DualRNS mul |
| `allow_insecure` | nine65 | (none) | Enables `FHEConfig::light()` in release |
| `slow_tests` | nine65 | (none) | Enables expensive tests |
| `benchmarks` | nine65 | (none) | Enables benchmark helpers |
| `deterministic_rng` | nine65 | dep:rand_chacha, dep:rand_core | `entropy/deterministic.rs` |
| `exact_rational` | nine65 | nexgen_rational | `rational_bridge.rs`, `exact_noise.rs` |
| `clockwork` | nine65 | clockwork-core, crc32fast | `bounded_rns.rs`, security submodules |
| `exact_transcendentals_backend` | nine65 | dep:exact_transcendentals | `transcendental_backend.rs` |
| `parallel` | mana | rayon | `parallel.rs` |
| `std` | exact_transcendentals | (none, default) | Enables std lib |
| `arbitrary-precision` | exact_transcendentals | (none) | `bigint.rs`, `crt.rs`, `crt_rational.rs` |
| `python` | nine65-python | dep:pyo3, nine65/serde | Entire crate |
| `wasm` | nine65-wasm | dep:wasm-bindgen, nine65/serde | Entire crate |

### 4.2 Unused Dependency Analysis

| Declaration | Status | Notes |
|-------------|--------|-------|
| `wide = "0.7"` (workspace) | **POTENTIALLY UNUSED** | Declared optional in mana but SIMD commented out: `simd = ["wide"]  # Disabled: counterproductive`. unhal declares `simd = []` (empty). No code path uses `wide`. |
| `sha2 = "0.10"` (workspace) | Used in nine65 directly | Key derivation |
| `crc32fast` in clockwork-core | Used | Integrity checks (D22, T14) |
| `subtle` in clockwork-core | Used | Constant-time operations (T16) |

**FINDING [W-1]**: The `wide` crate is declared as a workspace dependency and optional dependency in `mana`, but the `simd` feature that activates it is commented out. The `unhal` crate declares `simd = []` (empty feature). **No code path in the build activates `wide`**. This is dead dependency weight.

### 4.3 All Inter-Crate Dependencies Verified Used

- **nine65 -> mana**: Used in `accelerated.rs` (7 direct `mana::` references)
- **nine65 -> unhal**: Used in `accelerated.rs` (2 direct `unhal::` references)
- **nine65 -> nexgen_rational**: Used in `rational_bridge.rs`, `exact_params.rs`
- **nine65 -> clockwork-core**: Used in `bounded_rns.rs`, `gro_gate.rs`, `key_manager.rs`, `integrity.rs`
- **nine65 -> exact_transcendentals**: Used in `transcendental_backend.rs`
- **unhal -> mana**: Used throughout (re-exports `mana::` types in prelude)
- **fhe-service -> nine65**: Uses `nine65::params::SecureConfig`
- **nine65-python -> nine65**: Full FHE pipeline wiring
- **nine65-wasm -> nine65**: Full FHE pipeline wiring

---

## 5. DEAD CODE DETECTION

### 5.1 Orphan Top-Level .rs Files

The following `.rs` files exist at the workspace root, outside any crate's `src/`:

| File | Status |
|------|--------|
| `avatar.rs` | **ORPHAN** - not compiled by any crate |
| `comprehensive_audit_test.rs` | **ORPHAN** - not compiled by any crate |
| `comprehensive_benchmark.rs` | **ORPHAN** - not compiled by any crate |
| `pipeline.rs` | **ORPHAN** - not compiled by any crate |
| `random_encrypt_proptest.rs` | **ORPHAN** - not compiled by any crate |

These files are NOT referenced by any `Cargo.toml` or `mod` declaration. They appear to be standalone scripts or prior-version artifacts.

### 5.2 Feature-Gated Modules (Not Dead, But Conditionally Compiled)

The following modules only compile with specific features:

| Module | Feature Required |
|--------|-----------------|
| `nine65::accelerated` | `accelerated` |
| `nine65::arithmetic::rational_bridge` | `exact_rational` |
| `nine65::arithmetic::bounded_rns` | `clockwork` |
| `nine65::arithmetic::transcendental_backend` (actual backend) | `exact_transcendentals_backend` |
| `nine65::noise::exact_noise` | `exact_rational` |
| `nine65::security::{gro_gate, key_manager, integrity}` | `clockwork` |
| `nine65::entropy::{crt_shadow, wassan_noise}` | `shadow-entropy` |
| `nine65::entropy::deterministic` | `deterministic_rng` or test |
| `exact_transcendentals::{bigint, crt, crt_rational}` | `arbitrary-precision` |
| `mana::parallel` | `parallel` |

### 5.3 Potential Dead Code Within Crates

| Item | Location | Evidence |
|------|----------|---------|
| `float_scan_results.txt` | workspace root | Report artifact, not source |
| `cretbigintgearstack.zip` | workspace root | Archive, not compiled |
| Top-level docs (`AGENTS.md`, `CONTRIBUTING.md`, etc.) | workspace root | Documentation only |

### 5.4 Default Features Analysis

Default features for `nine65`: `["ntt_fft", "parallel"]`

Without explicitly enabling `accelerated`, `clockwork`, `exact_rational`, `exact_transcendentals_backend`, `shadow-entropy`, `serde`, `deterministic_rng`, `secure_seed`, or `v2`, these modules are NOT compiled in a default build. This means a default `cargo build` compiles:

- Core arithmetic (all non-gated modules)
- NTT FFT engine (default)
- Rayon parallelism (default)
- Shadow + Secure entropy
- All FHE operations (ops/*)
- Noise tracking, security estimation, error taxonomy

---

## 6. CROSS-REFERENCE CHECK

### 6.1 Re-exports

**nine65 prelude** (`lib.rs:151-241`): Comprehensive prelude re-exporting ~50 types from arithmetic, entropy, params, ring, keys, ops, noise, security, errors.

**Conditional re-exports in prelude**:
- `NTTEngine` aliased from `NTTEngineFFT` when `ntt_fft` feature is active (default)
- `DeterministicRng` only re-exported in test or `deterministic_rng` mode
- `WassanNoiseField` only re-exported with `shadow-entropy`

**clockwork-core re-exports** (`lib.rs:35-42`): 7 types re-exported from submodules.

**mana prelude** (`lib.rs:27-36`): 5 types + conditional `ParallelStream`.

**unhal prelude** (`lib.rs:56-66`): 5 local types + 3 mana re-exports.

### 6.2 Transitive Dependencies

- `fhe-service` -> `nine65` -> (optionally) `mana`, `unhal`, `nexgen_rational`, `clockwork-core`, `exact_transcendentals`
- `nine65-python` -> `nine65` (no optional deps activated beyond `serde` via `python` feature)
- `nine65-wasm` -> `nine65` (no optional deps activated beyond `serde` via `wasm` feature)
- `unhal` -> `mana` (always, not optional)

### 6.3 Cross-Crate Type Flow

| Type | Defined In | Used In |
|------|-----------|---------|
| `ManaStream` | mana | nine65::accelerated, unhal |
| `Lane` | mana | nine65::accelerated, unhal |
| `AnchorContext` | mana | nine65::accelerated, unhal |
| `Accelerator` | unhal | nine65::accelerated |
| `NexGenRat` | nexgen_rational | nine65::arithmetic::rational_bridge |
| `ExactCoeff` (NG) | nexgen_rational | nine65::arithmetic::rational_bridge |
| `CordicEngine` | exact_transcendentals | nine65::arithmetic::transcendental_backend |
| `HyperbolicCordic` | exact_transcendentals | nine65::arithmetic::transcendental_backend |
| `RnsBasis` | clockwork-core | nine65::arithmetic::bounded_rns |
| `GearStack` | clockwork-core | nine65 (via clockwork feature) |
| `SecureConfig` | nine65 | fhe-service, nine65-python, nine65-wasm |
| `FHEConfig` | nine65 | fhe-service, nine65-python, nine65-wasm |
| `Ciphertext` | nine65 | nine65-python, nine65-wasm |

---

## 7. ANOMALY CATALOGUE

### 7.1 Float Violations

**Severity Levels**: CRITICAL = production code path, MODERATE = test/bench code, LOW = offline tooling

| # | File | Context | Severity | Detail |
|---|------|---------|----------|--------|
| F-1 | `nine65/src/compiler.rs` | Entire module | LOW | Uses f64 throughout for **offline/static** noise analysis. Explicitly allowed with `#![allow(clippy::float_arithmetic)]`. Documented: "Float arithmetic is allowed here for static noise analysis. The QMNF integer-only mandate applies to runtime computation, not compiler tooling." |
| F-2 | `exact_transcendentals/src/lib.rs:221` | `ExactRational::to_f64()` | LOW | `#[cfg(test)]` only. Cannot reach production. |
| F-3 | `exact_transcendentals/src/constants.rs:269-275` | `from_scaled_30()`, `to_scaled_30()` | MODERATE | Public functions returning f64. Not `#[cfg(test)]`. Used in test code but **reachable from production** as they are `pub`. |
| F-4 | `exact_transcendentals/src/agm.rs:461-466` | `from_scaled()`, `to_scaled()` | LOW | In `#[cfg(test)]` module only. |
| F-5 | `exact_transcendentals/src/binary_splitting.rs:589-594` | `to_scaled()`, `from_scaled()` | LOW | In `#[cfg(test)]` module only. |
| F-6 | `exact_transcendentals/src/cordic.rs:668-673` | `to_scaled()`, `from_scaled()` | LOW | In `#[cfg(test)]` module only. |
| F-7 | `exact_transcendentals/src/crt_rational.rs:127-154` | `to_f64()`, `bigint_to_f64_approx()` | MODERATE | `to_f64()` is `#[cfg(test)]`, but `bigint_to_f64_approx()` is only called in test. The `to_f64()` method on `CrtRational` is cfg(test) guarded. |
| F-8 | `exact_transcendentals/src/sqrt.rs:403-412` | Test assertions | LOW | `#[cfg(test)]` only. |
| F-9 | `exact_transcendentals/src/lib.rs:352-353` | `test_constants_30bit` | LOW | `#[cfg(test)]` only. |

**Summary**: No f64/f32 usage in runtime cryptographic paths of `nine65`. The `compiler.rs` module uses f64 for offline noise analysis (documented exemption). `exact_transcendentals` has f64 only in test helpers, with one exception (F-3: `from_scaled_30`/`to_scaled_30` are public but only consumed by tests).

### 7.2 Unsafe Code

| # | File | Line | Purpose | Risk |
|---|------|------|---------|------|
| U-1 | `clockwork-core/src/key_lifecycle.rs:274` | `core::ptr::write_volatile(val as *mut u64, 0u64)` | Volatile memory zeroing for key material (Formal Spec A4) | **JUSTIFIED**. This is the standard pattern for preventing dead-store elimination of secret zeroing. Followed by `SeqCst` fence. Comment documents safety rationale. |

**Crate-level unsafe protections**:
- `nine65`: `#![forbid(unsafe_code)]` -- enforced crate-wide
- `mana`: `#![forbid(unsafe_code)]`
- `nexgen_rational`: `#![forbid(unsafe_code)]`
- `nine65/src/compiler.rs`: `#![forbid(unsafe_code)]` (redundant with crate-level)
- `clockwork-core`: Does NOT have `#![forbid(unsafe_code)]` -- expected, due to U-1
- `exact_transcendentals`: No forbid/deny for unsafe (but no unsafe found)
- `unhal`: No forbid/deny for unsafe (but no unsafe found)

### 7.3 Unwrap/Panic/Expect Inventory

**Production-critical unwraps** (outside `#[cfg(test)]`):

| # | File | Line | Expression | Risk |
|---|------|------|-----------|------|
| P-1 | `nine65/src/entropy/wassan_noise.rs:145` | `.expect("CRITICAL: OS entropy source failed")` | **JUSTIFIED**. OS CSPRNG failure is unrecoverable. |
| P-2 | `nine65/src/entropy/secure.rs:127` | `.expect("CRITICAL: OS CSPRNG failure")` | **JUSTIFIED**. Same rationale as P-1. |
| P-3 | `nine65/src/entropy/rng_trait.rs:144` | `panic!("...")` | In `require_secure_rng()` enforcement function. **JUSTIFIED** -- design-time assertion. |
| P-4 | `nine65/src/entropy/shadow.rs:275` | `panic!("Invalid ternary value")` | Inside a match arm that should be unreachable given the modulo operation above it. **LOW RISK** but could use `unreachable!()`. |
| P-5 | `nine65/src/kat.rs:174` | `.expect("Invalid KAT config")` | Known Answer Test setup. **JUSTIFIED** -- compile-time constant validation. |
| P-6 | `exact_transcendentals/src/lib.rs:133,180` | `assert!(den != 0)`, `assert!(other.num != 0)` | Panics on zero denominator/divisor in `ExactRational`. **MODERATE** -- could return `Result` instead. |
| P-7 | `exact_transcendentals/src/lib.rs:289` | `panic!("Unsupported precision")` | `TranscendentalConstants::new()` only supports 30/62 bits. **LOW** -- documented limitation. |

**Test-only unwraps**: Extensive use of `.unwrap()` and `.expect()` in `#[cfg(test)]` modules across all crates. This is standard Rust testing practice and does not affect production code.

### 7.4 TODO/FIXME/HACK

**No TODO, FIXME, or HACK markers found** in any source file. All WARNING annotations are security documentation (parameter warnings, key handling warnings).

### 7.5 Clippy Suppressions

The following clippy lints are suppressed in `nine65/src/lib.rs`:

```rust
#![allow(
    clippy::empty_line_after_doc_comments,
    clippy::needless_range_loop,
    clippy::manual_is_multiple_of,
    clippy::let_and_return,
    clippy::unnecessary_cast,
    clippy::double_comparisons,
    clippy::manual_div_ceil,
    clippy::println_empty_string,
    clippy::manual_range_patterns,
    clippy::manual_abs_diff,
    clippy::module_inception,
    clippy::useless_vec,
    clippy::identity_op
)]
```

**Assessment**: These are cosmetic/style suppressions. None are security-relevant. The `identity_op` and `unnecessary_cast` suppressions suggest defensive coding patterns that use explicit operations for clarity.

**Notable MISSING clippy denial in clockwork-core**:
```rust
// NOTE: In production, enable these:
// #![deny(clippy::float_arithmetic)]
// #![deny(clippy::float_cmp)]
```
These are commented out. While no floats were found in `clockwork-core`, the absence of the deny means floats could be introduced without compiler error.

### 7.6 Deprecated API Usage

| # | Location | Usage | Notes |
|---|----------|-------|-------|
| D-1 | `nine65/src/lib.rs:467` | `#[allow(deprecated)] evaluator.mul()` | In integration test. Single-modulus mul is deprecated in favor of DualRNS. |
| D-2 | `nine65-python/src/lib.rs:392` | `#[allow(deprecated)] evaluator.mul()` | Python binding still exposes deprecated mul. |
| D-3 | `nine65-wasm/src/lib.rs:133` | `#[allow(deprecated)] evaluator.mul()` | WASM binding still exposes deprecated mul. |

### 7.7 Security Concerns

| # | Category | Detail |
|---|----------|--------|
| S-1 | **Stub Service** | `fhe-service/src/main.rs` does NOT perform actual FHE computation. The evaluate endpoint returns input unchanged. Production deployment would expose a non-functional API. |
| S-2 | **Hardcoded Demo Key** | `fhe-service` uses a hardcoded base64 string as "public key" (`"RkhFX1BVQkxJQ19LRVlfREVNT19WQUxVRQ=="`). This decodes to `"FHE_PUBLIC_KEY_DEMO_VALUE"`. |
| S-3 | **Batch Encrypt Seed** | `nine65-python/src/lib.rs:413`: `batch_encrypt` uses `ShadowHarvester::with_seed(42)` -- a hardcoded deterministic seed for batch encryption. This means all batch-encrypted ciphertexts share the same randomness. |
| S-4 | **No TLS** | `fhe-service` uses raw TCP without TLS. |
| S-5 | **clockwork-core float deny commented out** | `#![deny(clippy::float_arithmetic)]` is commented out in clockwork-core's lib.rs. |

### 7.8 Miscellaneous Anomalies

| # | Category | Detail |
|---|----------|--------|
| M-1 | **Version inconsistency** | `exact_transcendentals` is version `1.0.0` while all other crates are `0.1.0`. This crate also does NOT use workspace versioning. |
| M-2 | **Non-workspace dependencies** | `clockwork-core` declares `subtle = "2.5"` and `crc32fast = "1.3"` locally rather than using workspace dependencies, even though `subtle` is in the workspace deps. |
| M-3 | **Archive in workspace** | `cretbigintgearstack.zip` exists at workspace root -- appears to be an obsolete archive. |
| M-4 | **PDF files in workspace** | `K_Elimination_Technical_Paper.pdf` and `Test NINE65 FHE.pdf` are binary assets in the source tree. |
| M-5 | **Orphan directories** | `apps/`, `archive/`, `state/`, `swarm_run/`, `lean4/`, `proofs/`, `scripts/` exist at workspace root but are not referenced by any `Cargo.toml`. |
| M-6 | **Duplicate ExactRational** | Both `exact_transcendentals::ExactRational` (i128-based) and `nexgen_rational::NexGenRat` (i128-based) provide exact rational arithmetic. These are independent implementations with no shared code. |
| M-7 | **Duplicate binary_gcd** | Both `exact_transcendentals::binary_gcd` (u128) and `nexgen_rational::binary_gcd` (i128) implement Stein's algorithm independently. |

---

## 8. SUMMARY STATISTICS

| Metric | Value |
|--------|-------|
| Total crates | 10 (9 in workspace + 1 fuzz) |
| Total .rs source files | ~100 |
| Feature flags (nine65) | 16 |
| `#![forbid(unsafe_code)]` crates | 4 (nine65, mana, nexgen_rational, compiler.rs) |
| Unsafe blocks | 1 (clockwork-core key zeroing) |
| f64 in production code paths | 1 module (compiler.rs -- offline tooling) |
| f64 in test code | ~60 occurrences across exact_transcendentals |
| Public `pub fn` returning f64 | 2 (`from_scaled_30`, `to_scaled_30` in constants.rs) |
| Panicking paths in production | 7 (5 justified, 2 moderate concern) |
| TODO/FIXME/HACK markers | 0 |
| Deprecated API exposures | 3 (mul in Python/WASM/tests) |
| Dead/orphan files | 5 top-level .rs files |
| Unused workspace deps | 1 (`wide`) |
| Security concerns | 5 (stub service, hardcoded key, batch seed, no TLS, commented deny) |

---

## 9. FINDINGS SEVERITY CLASSIFICATION

### Critical (Must Address)

None.

### High (Should Address Before Production)

| ID | Finding |
|----|---------|
| S-1 | `fhe-service` evaluate endpoint is a stub returning input unchanged |
| S-3 | `nine65-python::batch_encrypt` uses hardcoded seed(42) for all batch encryption randomness |

### Medium (Should Address)

| ID | Finding |
|----|---------|
| S-2 | Hardcoded demo public key in fhe-service |
| S-5 | `clockwork-core` float arithmetic deny is commented out |
| F-3 | `exact_transcendentals::constants::{from_scaled_30, to_scaled_30}` are public f64 functions not behind `#[cfg(test)]` |
| M-1 | Version inconsistency: exact_transcendentals at 1.0.0 vs workspace 0.1.0 |
| D-1/2/3 | Deprecated `mul()` exposed in Python and WASM bindings |

### Low (Informational)

| ID | Finding |
|----|---------|
| W-1 | `wide` crate declared but never activated |
| M-2 | clockwork-core has non-workspace dependency declarations |
| M-3/4 | Binary artifacts (zip, pdf) in workspace root |
| M-5 | Orphan directories at workspace root |
| M-6/7 | Duplicate ExactRational and binary_gcd implementations across crates |
| P-4 | `panic!("Invalid ternary value")` could be `unreachable!()` |
| P-6 | `ExactRational` asserts instead of returning Result for zero denominators |

---

*End of Forensic Audit Report*
*Build: 03_v5_full_20260209 | Audited: 2026-02-14 | Auditor: Claude Opus 4.6*

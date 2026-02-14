# FORENSIC AUDIT: Build 04_pre_exact_trans_git

**Build**: `04_pre_exact_trans_git` (NINE65 Pre-Transcendentals Snapshot)
**Path**: `/home/acid/Projects/Homomorphic_Armada/builds/04_pre_exact_trans_git/`
**Auditor**: Claude Opus 4.6 (Forensic Code Auditor)
**Date**: 2026-02-14
**Scope**: Read-only forensic inspection. NO source modifications made.
**Method**: Exhaustive file-by-file read of all Cargo.toml manifests, all source files in clockwork-core (8 files), mana (6 files), unhal (4 files), nexgen_rational (8 files), plus nine65 lib.rs and accelerated.rs. Supplemented by grep-based scans of all 108 .rs files across the workspace for anomaly patterns (float usage, unsafe, TODO/FIXME, dead_code, unwrap/expect/panic, feature gates, cross-crate imports).

---

## 1. STRUCTURE MAPPING

### 1.1 Workspace Layout

```
04_pre_exact_trans_git/
  Cargo.toml                          # Workspace root, resolver "2"
  crates/
    clockwork-core/                   # 8 source files
    mana/                             # 6 source files + 1 bench + 1 example
    unhal/                            # 4 source files + 1 example
    nexgen_rational/                  # 8 source files
    nine65/                           # 65 source files + 3 benches + 1 example + 5 integration tests + 2 binaries
    nine65-python/                    # 1 source file (PyO3 bindings)
    nine65-wasm/                      # 1 source file (wasm-bindgen bindings)
  fuzz/                               # Excluded from workspace
    fuzz_targets/
      fuzz_ntt.rs
      fuzz_homomorphic.rs
      fuzz_k_elimination.rs
      fuzz_deserialize.rs
      fuzz_encrypt_decrypt.rs
```

**Total Rust source files**: 108 (across all crates, excluding fuzz)
**Total `#[test]` functions**: 777 across 87 files
**Total `#[cfg(test)]` modules**: 96 across 80 files
**Total public items** (nine65 only): 1,212 across 62 files

### 1.2 Crate Manifest Summary

| Crate | Edition | Dependencies | Features | Crate Type |
|-------|---------|-------------|----------|------------|
| clockwork-core | workspace | subtle 2.5, crc32fast 1.3 | none declared | lib (default) |
| mana | workspace | wide (opt), rayon (opt), zeroize | default=["parallel"], simd, parallel | lib (default) |
| unhal | workspace | mana (path), rayon (opt) | default=["parallel"], simd, parallel | lib (default) |
| nexgen_rational | workspace | (none) | serde (optional) | lib (default) |
| nine65 | workspace | 14 optional deps | 12 features | lib (default) |
| nine65-python | workspace | nine65 (path), pyo3 0.28 (opt), bincode 1.3 | python, allow_insecure | cdylib |
| nine65-wasm | workspace | nine65 (path), wasm-bindgen 0.2 (opt), bincode 1.3, serde 1.0 | wasm | cdylib |

### 1.3 nine65 Feature Matrix

| Feature | Activates |
|---------|-----------|
| `default` | (empty) |
| `accelerated` | mana, unhal |
| `clockwork` | clockwork-core, crc32fast |
| `exact_rational` | nexgen_rational |
| `serde` | dep:serde |
| `shadow-entropy` | dep:sha2 |
| `ntt-fft` | (internal flag) |
| `parallel` | dep:rayon |
| `simd` | (internal flag) |
| `allow_insecure` | (internal flag) |
| `full` | accelerated, clockwork, exact_rational, serde, shadow-entropy, ntt-fft, parallel |

### 1.4 nine65 Internal Module Tree

```
nine65/src/
  lib.rs                              # Root: #![forbid(unsafe_code)], 11 pub modules
  errors.rs                           # Error types
  kat.rs                              # Known Answer Tests
  compiler.rs                         # Circuit compiler (FLOAT EXCEPTION - see Anomalies)
  v2_integration_tests.rs             # V2 integration test suite
  accelerated.rs                      # Bridge to mana/unhal (cfg: accelerated)

  arithmetic/                         # 18 submodules
    mod.rs                            # Re-exports all arithmetic types
    montgomery.rs                     # Montgomery multiplication context
    persistent_montgomery.rs          # Coefficients never leave Montgomery form
    barrett.rs                        # Barrett reduction, HybridModContext
    ntt.rs                            # DFT-based NTT engine
    ntt_fft.rs                        # FFT-based NTT engine (O(N log N))
    rns.rs                            # RNSContext, DualRNSContext
    k_elimination.rs                  # K-Elimination exact division
    exact_divider.rs                  # ExactDivider primitive
    exact_coeff.rs                    # ExactCoeff, ExactContext, AnchorTrack
    ct_mul_exact.rs                   # ExactCiphertext multiplication
    mobius_int.rs                     # Mobius signed arithmetic
    pade_engine.rs                    # Integer transcendentals (exp/sin/cos/sigmoid)
    mq_relu.rs                        # O(1) sign detection
    integer_softmax.rs                # Exact sum softmax
    cyclotomic_phase.rs               # Ring trigonometry
    valuation.rs                      # Valuation functions
    order_finding.rs                  # Non-circular BSGS
    integer_math.rs                   # Integer utilities
    rational_bridge.rs                # Bridge to nexgen_rational (cfg: exact_rational)
    bounded_rns.rs                    # Bridge to clockwork-core (cfg: clockwork)

  ops/                                # 9 submodules
    mod.rs                            # Re-exports
    encrypt.rs                        # BFVEncoder, BFVEncryptor, BFVDecryptor
    homomorphic.rs                    # BFVEvaluator, TrackedEvaluator
    batch.rs                          # BatchEncoder
    parallel.rs                       # ParallelEncryptor, ParallelDecryptor
    rns_mul.rs                        # RNSEvaluator
    rns_fhe.rs                        # RNSFHEContext, RNSCiphertext (largest module)
    neural.rs                         # FHENeuralEvaluator, DenseLayer
    galois.rs                         # GaloisEngine, GaloisKey
    gso_fhe.rs                        # GSOFHEContext, GSOCiphertext

  params/                             # 7 submodules
    mod.rs                            # BFVParams, SEAL-compatible parameters
    primes.rs                         # NTT-friendly prime constants
    production.rs                     # Production parameter sets
    secure_configs.rs                 # Security-level configurations
    exact_params.rs                   # Exact parameter types
    validation.rs                     # Parameter validation
    security_estimator.rs             # Lattice security estimation

  keys/                               # 1 submodule
    mod.rs                            # Key generation, SecretKey, PublicKey, EvalKey

  noise/                              # 3 submodules
    mod.rs                            # NoiseTracker, NoiseModel
    budget.rs                         # NoiseBudget tracking
    exact_noise.rs                    # ExactNoiseTracker

  ring/                               # 3 submodules
    mod.rs                            # Ring module root
    polynomial.rs                     # Polynomial operations
    pool.rs                           # Polynomial pool/cache

  entropy/                            # 6 submodules
    mod.rs                            # Entropy module root
    shadow.rs                         # ShadowHarvester
    secure.rs                         # Secure entropy sources
    rng_trait.rs                      # FheRng trait
    deterministic.rs                  # DeterministicRng
    wassan_noise.rs                   # WassanNoiseField
    crt_shadow.rs                     # CRTShadowContext (59 public items)

  security/                           # 4 submodules
    mod.rs                            # Security module root
    secret_data.rs                    # SecretData, SecretPoly, SecretScalar
    gro_gate.rs                       # TimingGate (bridge to clockwork GRO)
    key_manager.rs                    # KeyManager (bridge to clockwork lifecycle)
    integrity.rs                      # Checksum utilities (bridge to clockwork integrity)

  bin/
    fhe_demo.rs                       # Demo binary
    security_estimator_baseline.rs    # Security estimation binary
```

---

## 2. DATA FLOW TRACING

### 2.1 Primary Encryption Pipeline

```
Plaintext (Vec<u64>)
  |
  v
BFVEncoder::encode()           [ops/encrypt.rs]
  |-- Polynomial scaling by Delta = floor(q/t)
  |-- NTT transform via NTTEngine
  v
BFVEncryptor::encrypt()        [ops/encrypt.rs]
  |-- Sample error polynomials from entropy source
  |-- ct = (a*s + e + m, a) mod q
  v
Ciphertext { c0, c1 }         [ops/encrypt.rs]
  |
  +---> BFVEvaluator::add/mul/relin()  [ops/homomorphic.rs]
  |       |-- TrackedEvaluator wraps with noise budget tracking
  |       v
  +---> RNSEvaluator::mul_rns()        [ops/rns_mul.rs]
  |       |-- Decomposes into RNS channels
  |       |-- Per-channel multiply (embarrassingly parallel)
  |       |-- CRT reconstruction
  |       v
  +---> RNSFHEContext                  [ops/rns_fhe.rs]
  |       |-- Full RNS-BFV pipeline
  |       |-- RNSCiphertext with multi-limb representation
  |       |-- Key switching via decomposition
  |       v
  +---> GSOFHEContext                  [ops/gso_fhe.rs]
          |-- Glowworm Swarm Optimization for noise management
          v
BFVDecryptor::decrypt()        [ops/encrypt.rs]
  |-- Inner product c0 + c1*s
  |-- Scale by t/q and round
  v
Plaintext (Vec<u64>)
```

### 2.2 RNS Acceleration Pipeline (feature: accelerated)

```
nine65 RNSContext              [nine65/arithmetic/rns.rs]
  |
  v
AcceleratedFHE                 [nine65/accelerated.rs]
  |-- Converts nine65 primes to mana ManaStream
  |-- Wraps operations in unhal Pipeline stages
  v
ManaStream                     [mana/stream.rs]
  |-- Arc<Vec<u64>> shared primes
  |-- Multi-lane CRT representation
  |
  +---> Lane::mod_add/mod_sub  [mana/lane.rs]  (branchless)
  +---> MontgomeryLane::mul    [mana/lane.rs]  (Montgomery form)
  +---> PersistentLane          [mana/lane.rs]  (stays in Montgomery)
  v
ParallelStream                 [mana/parallel.rs]
  |-- Rayon work-stealing
  |-- add_reinforced (flattened parallelism)
  |-- ParallelNTT
  v
AcceleratedRNS                 [nine65/accelerated.rs]
  |-- Drop-in replacement for nine65 RNSContext
  |-- Same API, backed by mana streams
```

### 2.3 Clockwork RNS Pipeline (feature: clockwork)

```
Secret key / tier state
  |
  v
TripleRedundant<T>             [clockwork-core/integrity.rs]
  |-- Three copies + CRC32 checksum
  |-- MajVote: AllAgree / Recovered / Failed
  |-- Fail-closed on multi-corruption (T15)
  v
GearStack                     [clockwork-core/gearstack.rs]
  |-- RNS value + Bound tracking
  |-- D7 bound update rules (add/sub/mul)
  |-- DEFAULT_GUARD_BITS = 8
  |-- Auto-promotion when bound exceeds modulus product
  v
RnsBasis                      [clockwork-core/basis.rs]
  |-- CRT encode/decode
  |-- extended_gcd, mod_inverse
  |-- centered_lift
  |-- promote/demote (add/remove moduli)
  v
k_eliminate                    [clockwork-core/garner.rs]
  |-- Garner mixed-radix decomposition
  |-- Constant-time variant: k_eliminate_ct
  v
DecodeToQ                     [clockwork-core/decode_to_q.rs]
  |-- Bridge to RLWE modulus q (INV-1: fixed)
  |-- encode/decode for unsigned and centered
  v
KeyLifecycle                   [clockwork-core/key_lifecycle.rs]
  |-- State machine: KEYGEN -> SPLIT -> ACTIVE -> ZEROED
  |-- KeySharePair: s1 + s2 = s (mod q)
  |-- Resharing with fresh randomness
  |-- Volatile memory zeroing (A4)
  v
GroGate                       [clockwork-core/gro.rs]
  |-- Golden Ratio Oscillator
  |-- Side-channel timing protection
  |-- phase_a/phase_b coincidence windows
```

### 2.4 Exact Rational Pipeline (feature: exact_rational)

```
NexGenRat                      [nexgen_rational/rat_ng/types.rs]
  |-- num: i128, den: i128 (always positive)
  |-- DenState: One / Small / General
  v
Binary GCD (Stein's)           [nexgen_rational/binary_gcd.rs]
  |-- Phase 1-5: shift-based, no division
  v
Arithmetic ops                 [nexgen_rational/rat_ng/ops.rs]
  |-- add/sub: cross-multiply with I4 overflow check
  |-- mul: num*num, den*den
  |-- div: multiply by reciprocal
  |-- All use checked_mul/checked_add
  v
NormalizationScheduler         [nexgen_rational/rat_ng/normalize.rs]
  |-- 80% of 127-bit capacity threshold
  |-- EMA-based scheduling
  v
DivisionPolicy                 [nexgen_rational/rat_ng/policy.rs]
  |-- T8 Trichotomy: ExactInverse / ExactAFC / FPD
  |-- FPD (Fused Piggyback Division) for non-exact cases
  v
RationalBridge                 [nine65/arithmetic/rational_bridge.rs]
  |-- Converts nine65 types to/from NexGenRat
```

---

## 3. CONSTRUCT IDENTIFICATION

### 3.1 Core Algorithms

| Algorithm | Location | Innovation Status |
|-----------|----------|------------------|
| NTT (DFT-based) | `nine65/arithmetic/ntt.rs` | Standard |
| NTT (FFT-based, O(N log N)) | `nine65/arithmetic/ntt_fft.rs` | V2 Innovation |
| Montgomery multiplication | `nine65/arithmetic/montgomery.rs` | Standard |
| Persistent Montgomery | `nine65/arithmetic/persistent_montgomery.rs` | Innovation |
| Barrett reduction | `nine65/arithmetic/barrett.rs` | Standard |
| K-Elimination (exact division) | `nine65/arithmetic/k_elimination.rs` + `clockwork-core/garner.rs` | Innovation |
| Mobius signed arithmetic | `nine65/arithmetic/mobius_int.rs` | Innovation |
| Pade transcendentals | `nine65/arithmetic/pade_engine.rs` | Innovation |
| MQ-ReLU (O(1) sign) | `nine65/arithmetic/mq_relu.rs` | Innovation |
| Integer softmax | `nine65/arithmetic/integer_softmax.rs` | Innovation |
| Cyclotomic phase | `nine65/arithmetic/cyclotomic_phase.rs` | Innovation |
| Order finding (BSGS) | `nine65/arithmetic/order_finding.rs` | Innovation |
| Binary GCD (Stein's) | `nexgen_rational/binary_gcd.rs` | Standard |
| CRT encode/decode | `clockwork-core/basis.rs` | Standard |
| Garner decomposition | `clockwork-core/garner.rs` | Standard (CT variant: Innovation) |
| Triple-redundant integrity | `clockwork-core/integrity.rs` | Innovation |
| GRO timing protection | `clockwork-core/gro.rs` | Innovation |
| Key share splitting | `clockwork-core/key_lifecycle.rs` | Standard |
| GSO (Glowworm Swarm) | `mana/gso.rs` | Innovation |
| K-Anchor exact division | `mana/anchor.rs` | Innovation |
| Lane branchless mod ops | `mana/lane.rs` | Innovation |

### 3.2 Data Structures

| Structure | Location | Purpose |
|-----------|----------|---------|
| `RnsBasis` | `clockwork-core/basis.rs` | Set of coprime moduli for CRT |
| `GearStack` | `clockwork-core/gearstack.rs` | RNS value with automatic bound tracking |
| `Bound` | `clockwork-core/bound_tracker.rs` | Tracked upper bound on plaintext value |
| `TripleRedundant<T>` | `clockwork-core/integrity.rs` | Triple-copy CRC32 integrity container |
| `TierState` | `clockwork-core/integrity.rs` | Protected metadata per GearStack |
| `KeySharePair` | `clockwork-core/key_lifecycle.rs` | (s1, s2) with s1+s2 = s mod q |
| `KeyLifecycle` | `clockwork-core/key_lifecycle.rs` | State machine for key management |
| `DecodeToQ` | `clockwork-core/decode_to_q.rs` | CRT-to-RLWE bridge with fixed q |
| `GroGate` | `clockwork-core/gro.rs` | Side-channel timing oscillator |
| `Lane` | `mana/lane.rs` | Single CRT channel with modular ops |
| `MontgomeryLane` | `mana/lane.rs` | Lane with Montgomery multiplication |
| `PersistentLane` | `mana/lane.rs` | Lane where values stay in Montgomery form |
| `ManaStream` | `mana/stream.rs` | Multi-lane CRT stream |
| `KAnchor` | `mana/anchor.rs` | K-elimination anchor with codex pairs |
| `QbitState` | `mana/gso.rs` | Quantum-inspired amplitude for GSO |
| `GsoSwarm` | `mana/gso.rs` | Swarm optimizer population |
| `NexGenRat` | `nexgen_rational/rat_ng/types.rs` | Exact rational (i128/i128) |
| `ExactCoeff` | `nexgen_rational/exact_coeff.rs` | i128 wrapper with checked ops |
| `NormalizationScheduler` | `nexgen_rational/rat_ng/normalize.rs` | EMA-based GCD scheduling |
| `Pipeline` | `unhal/pipeline.rs` | Staged computation pipeline |
| `BatchProcessor` | `unhal/batch.rs` | Bulk operation processor |
| `Accelerator` | `unhal/accelerator.rs` | Auto-dispatch execution engine |
| `BFVParams` | `nine65/params/mod.rs` | BFV parameter set |
| `Ciphertext` | `nine65/ops/encrypt.rs` | BFV ciphertext pair |
| `RNSFHEContext` | `nine65/ops/rns_fhe.rs` | Full RNS-BFV context |
| `RNSCiphertext` | `nine65/ops/rns_fhe.rs` | Multi-limb RNS ciphertext |
| `GSOFHEContext` | `nine65/ops/gso_fhe.rs` | GSO-optimized FHE context |
| `NoiseBudget` | `nine65/noise/budget.rs` | Noise budget tracker |
| `CRTShadowContext` | `nine65/entropy/crt_shadow.rs` | CRT-based shadow entropy |
| `AcceleratedFHE` | `nine65/accelerated.rs` | Bridge to mana/unhal |

### 3.3 Formal Spec Coverage

The following formal spec references were identified in code comments:

**clockwork-core:**
- D7: Bound update rules (bound_tracker.rs)
- D18: Key share pair (key_lifecycle.rs)
- D19: Re-sharing (key_lifecycle.rs)
- D20: Key lifecycle state machine (key_lifecycle.rs)
- D21: PRF-based evaluation key chain (key_lifecycle.rs, referenced)
- D22: Triple redundancy TR(v) (integrity.rs)
- D23: MajVote algorithm (integrity.rs)
- T11: Re-sharing correctness (key_lifecycle.rs)
- T12: Share independence (key_lifecycle.rs)
- T13: Forward secrecy (key_lifecycle.rs, referenced)
- T14: Triple redundancy detection (integrity.rs)
- T15: Fail-closed guarantee (integrity.rs)
- INV-1: Fixed modulus q (decode_to_q.rs)
- INV-5: No full key in memory after split (key_lifecycle.rs)
- INV-7: Public schedule for resharing (key_lifecycle.rs)
- INV-8: MajVote verification before DecQ ops (integrity.rs)
- A4: Memory zeroing (key_lifecycle.rs)

**Test obligation coverage (clockwork-core):**
- CT-01 through CT-06: basis, garner, gearstack, decode_to_q tests
- GT-01, GT-02: GRO gate tests
- IT-01 through IT-04: Integrity triple-redundancy tests
- KT-01 through KT-04: Key lifecycle tests

---

## 4. WIRING VERIFICATION

### 4.1 Dependency Graph

```
                    nine65
                   /  |   \   \
          (accelerated) (clockwork) (exact_rational)
              /    \        |            |
           mana   unhal  clockwork-core  nexgen_rational
             \    /
              mana  (unhal depends on mana directly)
```

### 4.2 Cross-Crate Import Verification

**nine65 -> clockwork-core** (behind `#[cfg(feature = "clockwork")]`):
- `nine65/src/security/key_manager.rs` -- imports KeySharePair, KeyLifecycle, KeyState
- `nine65/src/security/gro_gate.rs` -- imports GroGate
- `nine65/src/security/integrity.rs` -- imports TripleRedundant, VoteResult
- `nine65/src/arithmetic/bounded_rns.rs` -- imports RnsBasis, GearStack, Bound
- `nine65/tests/clockwork_cross_validation.rs` -- integration test

**nine65 -> nexgen_rational** (behind `#[cfg(feature = "exact_rational")]`):
- `nine65/src/arithmetic/rational_bridge.rs` -- imports NexGenRat
- `nine65/tests/rational_bridge_proptest.rs` -- property test

**nine65 -> mana + unhal** (behind `#[cfg(feature = "accelerated")]`):
- `nine65/src/accelerated.rs` -- imports ManaStream, Lane, StreamOps, Pipeline, BatchProcessor, etc.

**unhal -> mana** (unconditional):
- `unhal/src/lib.rs` -- re-exports mana types in prelude
- `unhal/src/batch.rs` -- uses ManaStream, StreamOps
- `unhal/src/accelerator.rs` -- uses ManaStream, StreamOps, ParallelStream
- `unhal/src/pipeline.rs` -- uses ManaStream

**nine65-python -> nine65** (unconditional):
- Single lib.rs file with PyO3 bindings

**nine65-wasm -> nine65** (unconditional):
- Single lib.rs file with wasm-bindgen bindings

### 4.3 Feature Gate Density

The nine65 crate uses 104 `#[cfg(feature = ...)]` gates across 24 files. The heaviest gating is in:
- `nine65/src/ops/rns_fhe.rs` -- 20 feature gates
- `nine65/src/arithmetic/ntt.rs` -- 15 feature gates
- `nine65/src/ops/parallel.rs` -- 8 feature gates
- `nine65/src/accelerated.rs` -- 7 feature gates
- `nine65/src/entropy/wassan_noise.rs` -- 6 feature gates
- `nine65/src/security/mod.rs` -- 6 feature gates

### 4.4 Wiring Verification Status

| Connection | Status | Notes |
|------------|--------|-------|
| nine65 -> clockwork-core | VERIFIED | 5 import sites, all feature-gated |
| nine65 -> nexgen_rational | VERIFIED | 2 import sites (src + test), feature-gated |
| nine65 -> mana/unhal | VERIFIED | 1 import site (accelerated.rs), feature-gated |
| unhal -> mana | VERIFIED | Direct dependency, 4 import sites |
| nine65-python -> nine65 | VERIFIED | Single binding file |
| nine65-wasm -> nine65 | VERIFIED | Single binding file |
| clockwork-core internal | VERIFIED | All 8 modules used, all re-exported in lib.rs |
| mana internal | VERIFIED | All modules used, prelude re-exports |
| nexgen_rational internal | VERIFIED | All modules used, no orphans |

---

## 5. DEAD CODE DETECTION

### 5.1 Explicit `#[allow(dead_code)]` Annotations

9 explicit `#[allow(dead_code)]` annotations found:

| File | Count | Context |
|------|-------|---------|
| `nine65/src/params/security_estimator.rs` | 4 | Lines 187, 219, 229, 241 -- helper functions for lattice estimation |
| `nine65/src/arithmetic/rns.rs` | 1 | Line 1275 -- test helper |
| `nine65/src/ops/rns_fhe.rs` | 3 | Lines 1330, 2924, 3131 -- internal structures/helpers |

### 5.2 `#[allow(unused_assignments)]` Annotations

1 occurrence:
- `nine65/src/arithmetic/order_finding.rs:342` -- `giant_steps_done` initial value always overwritten (documented: intentional)

### 5.3 Feature-Gated Modules That May Be Unreachable

The following modules exist but are only compiled under specific feature flags. Without those features enabled, these modules are completely absent from compilation:

- `nine65/src/accelerated.rs` -- requires `accelerated`
- `nine65/src/arithmetic/rational_bridge.rs` -- requires `exact_rational`
- `nine65/src/arithmetic/bounded_rns.rs` -- requires `clockwork`
- `nine65/src/security/gro_gate.rs` -- partially requires `clockwork`
- `nine65/src/security/key_manager.rs` -- partially requires `clockwork`
- `nine65/src/security/integrity.rs` -- partially requires `clockwork`

**Note**: The `default` feature set is empty, meaning a bare `cargo build` compiles nine65 with none of these optional integrations active. The `full` feature activates all of them.

### 5.4 Fuzz Targets (Excluded from Workspace)

5 fuzz targets exist in `/fuzz/fuzz_targets/` but are excluded from the workspace via `exclude = ["fuzz"]` in the root Cargo.toml. These target:
- `fuzz_ntt.rs` -- NTT transform fuzzing
- `fuzz_homomorphic.rs` -- Homomorphic operation fuzzing
- `fuzz_k_elimination.rs` -- K-elimination fuzzing
- `fuzz_deserialize.rs` -- Deserialization fuzzing
- `fuzz_encrypt_decrypt.rs` -- Encrypt/decrypt round-trip fuzzing

---

## 6. CROSS-REFERENCE CHECK

### 6.1 Duplicate Implementations

**Montgomery Multiplication** appears in TWO independent implementations:
1. `nine65/src/arithmetic/montgomery.rs` -- `MontgomeryContext` (nine65-native)
2. `mana/src/lane.rs` -- `MontgomeryLane` (mana-native)

These are independent implementations. The nine65 version is always available; the mana version is only available behind the `accelerated` feature. The `accelerated.rs` bridge converts between them.

**Persistent Montgomery** also appears in two places:
1. `nine65/src/arithmetic/persistent_montgomery.rs` -- `PersistentMontgomery`, `PersistentPolynomial`
2. `mana/src/lane.rs` -- `PersistentLane`

Same pattern: independent implementations, bridged via `accelerated.rs`.

**K-Elimination** appears in THREE locations:
1. `nine65/src/arithmetic/k_elimination.rs` -- `KElimination` (nine65-native)
2. `clockwork-core/src/garner.rs` -- `k_eliminate`, `k_eliminate_ct` (clockwork-native)
3. `mana/src/anchor.rs` -- `KAnchor`, `exact_divide` (mana-native)

All three are independent implementations with different interfaces optimized for their respective contexts (FHE operations, RNS arithmetic, stream processing).

**ExactCoeff** appears in TWO locations:
1. `nine65/src/arithmetic/exact_coeff.rs` -- `ExactCoeff`, `ExactContext`
2. `nexgen_rational/src/exact_coeff.rs` -- `ExactCoeff` (i128 wrapper)

These serve different purposes: the nine65 version integrates with FHE coefficient tracking, while the nexgen_rational version is a pure arithmetic wrapper.

### 6.2 NTT Engine Selection

Two NTT engines coexist, selected at compile time:

```
#[cfg(feature = "ntt-fft")]
pub use ntt_fft::NTTEngineFFT as NTTEngine;   // O(N log N)

#[cfg(not(feature = "ntt-fft"))]
pub use ntt::NTTEngine;                         // O(N^2) DFT-based
```

Both are always available as `NTTEngineDFT` and `NTTEngineFFT` respectively, but the default `NTTEngine` alias switches based on the `ntt-fft` feature.

### 6.3 Entropy Source Hierarchy

```
FheRng trait                   [entropy/rng_trait.rs]
  |
  +-- SecureRng                [entropy/secure.rs]    (production)
  +-- DeterministicRng         [entropy/deterministic.rs] (testing)
  +-- ShadowHarvester          [entropy/shadow.rs]    (shadow entropy)
  +-- WassanNoiseField         [entropy/wassan_noise.rs]
  +-- CRTShadowContext         [entropy/crt_shadow.rs] (CRT-based)
```

### 6.4 Noise Tracking Hierarchy

```
NoiseModel                     [noise/mod.rs]
  |
  +-- NoiseBudget             [noise/budget.rs]       (budget tracking)
  +-- ExactNoiseTracker       [noise/exact_noise.rs]  (exact integer tracking)
  +-- TrackedEvaluator        [ops/homomorphic.rs]    (wraps BFVEvaluator)
  +-- NoiseEstimate           [ops/gso_fhe.rs]        (GSO-based estimation)
```

---

## 7. ANOMALY CATALOGUE

### ANO-01: Float Violation in compiler.rs [SEVERITY: DOCUMENTED]

**Location**: `nine65/src/compiler.rs`
**Evidence**: 23 occurrences of `f64` across the file
**Detail**: The circuit compiler uses f64 for offline/static noise analysis. This includes struct fields (`add_noise_bits: f64`, `mul_noise_bits: f64`, `relin_noise_bits: f64`, `rescale_reduction_bits: f64`, `rotate_noise_bits: f64`, `safety_factor: f64`) and arithmetic operations on them.
**Mitigation**: Explicitly documented in `nine65/src/lib.rs` lines 41-42:
```
//! Integer-only runtime: cryptographic paths use no f32/f64. The circuit compiler
//! (compiler.rs) uses f64 **only** for offline/static noise analysis.
```
The file also contains `#![allow(clippy::float_arithmetic)]` at line 10.
**Assessment**: Intentional, scoped exception. The compiler is an offline tool that does not execute in the cryptographic runtime path. No f64 values flow into ciphertext operations.

### ANO-02: Float Reference in security_estimator.rs [SEVERITY: LOW]

**Location**: `nine65/src/params/security_estimator.rs`
**Evidence**: 2 lines referencing f64 in comments (lines 454-455):
```
// This test verifies the module compiles without std::f64
// The old version used: use std::f64::consts::PI;
```
**Assessment**: Comment-only. No actual f64 usage in code. Documents a past refactoring.

### ANO-03: Commented-Out Lint in clockwork-core [SEVERITY: MEDIUM]

**Location**: `clockwork-core/src/lib.rs` line 22
**Evidence**:
```
// #![deny(clippy::float_arithmetic)]
```
Line 16 states this lint IS enforced, but line 22 shows it is commented out.
**Detail**: The doc comment at line 16 claims: "exact integer computation. This is enforced by `#![deny(clippy::float_arithmetic)]`" -- but the actual lint attribute is commented out.
**Assessment**: Documentation/code mismatch. The crate does not actually contain any float arithmetic (verified by grep), so the commented-out lint is not causing harm, but the doc comment is misleading.

### ANO-04: Single Unsafe Block [SEVERITY: DOCUMENTED]

**Location**: `clockwork-core/src/key_lifecycle.rs` lines 274-276
**Evidence**:
```rust
unsafe {
    core::ptr::write_volatile(val as *mut u64, 0u64);
}
```
**Detail**: Used for memory zeroing (Formal Spec A4 requirement). Prevents compiler dead-store elimination of security-critical key material zeroing. Followed by a `SeqCst` compiler fence.
**Assessment**: Intentional, minimal, well-justified. The clockwork-core crate does NOT use `#![forbid(unsafe_code)]` at the crate level, unlike nine65, mana, and nexgen_rational which all do. This is the ONLY unsafe block in the entire workspace.

### ANO-05: Extensive unwrap/expect in Production Paths [SEVERITY: MEDIUM]

**Total occurrences**: 285 across 49 files
**Production-path examples** (non-test code):
- `clockwork-core/src/basis.rs`: `.expect("Inverse must exist")` (5 occurrences)
- `clockwork-core/src/garner.rs`: `.expect("Moduli must be pairwise coprime")` (8 occurrences)
- `clockwork-core/src/gearstack.rs`: `.unwrap()` (22 occurrences)
- `clockwork-core/src/decode_to_q.rs`: `.expect(...)` (7 occurrences)
- `mana/src/anchor.rs`: `.expect("Primes must be coprime")` (2 occurrences)
- `nine65/src/ops/rns_fhe.rs`: `.unwrap()` (35 occurrences -- highest single file)
- `nine65/src/security/key_manager.rs`: `.unwrap()` (14 occurrences)
- `nine65/src/ops/encrypt.rs`: `.unwrap()` (10 occurrences)
- `nine65/src/arithmetic/order_finding.rs`: `.unwrap()` (7 occurrences)
- `nine65/src/arithmetic/rational_bridge.rs`: `.unwrap()` (7 occurrences)

**Assessment**: Many of these are in production code paths, not just tests. While some are mathematically guaranteed to succeed (e.g., modular inverse of a prime must exist), others represent potential panic points. The `rns_fhe.rs` file with 35 unwraps is the highest-risk module.

### ANO-06: Single TODO/STUB Reference [SEVERITY: NEGLIGIBLE]

**Location**: `nexgen_rational/src/rat_ng/normalize.rs` line 7
**Evidence**:
```
//! pub fn should_normalize(_rat: &NexGenRat) -> bool { true } // STUB
```
**Assessment**: Historical comment documenting what the old stub looked like. The actual implementation is fully functional with 80% capacity threshold and EMA-based scheduling. No active stubs exist.

### ANO-07: Dead Code Annotations in rns_fhe.rs [SEVERITY: LOW]

**Location**: `nine65/src/ops/rns_fhe.rs` lines 1330, 2924, 3131
**Evidence**: Three `#[allow(dead_code)]` annotations on internal structures
**Assessment**: These may indicate partially-implemented features or future-use structures. The annotations suppress warnings rather than removing the code. Given the size of rns_fhe.rs (the largest module in the crate), some internal helpers may be provisioned for future use.

### ANO-08: Dead Code in security_estimator.rs [SEVERITY: LOW]

**Location**: `nine65/src/params/security_estimator.rs` lines 187, 219, 229, 241
**Evidence**: Four `#[allow(dead_code)]` annotations on helper functions
**Assessment**: Lattice security estimation utilities that may be used only in certain configurations or reserved for future parameter selection workflows.

### ANO-09: Triplicated K-Elimination [SEVERITY: INFORMATIONAL]

Three independent implementations of K-elimination/exact division exist:
1. `nine65/src/arithmetic/k_elimination.rs` -- FHE-optimized
2. `clockwork-core/src/garner.rs` -- RNS-optimized with constant-time variant
3. `mana/src/anchor.rs` -- Stream-optimized with anchor context

**Assessment**: These serve different architectural layers and have different interfaces/optimizations. Not redundant in the traditional sense -- each is tailored for its context. However, any algorithmic fix would need to be applied to all three.

### ANO-10: Duplicated Montgomery Implementations [SEVERITY: INFORMATIONAL]

Two independent Montgomery multiplication implementations:
1. `nine65/src/arithmetic/montgomery.rs` -- Standard `MontgomeryContext`
2. `mana/src/lane.rs` -- `MontgomeryLane` with persistent variant

Plus persistent variants in both:
1. `nine65/src/arithmetic/persistent_montgomery.rs` -- `PersistentMontgomery`
2. `mana/src/lane.rs` -- `PersistentLane`

**Assessment**: Same pattern as ANO-09. Architectural layering justifies the duplication, but algorithmic consistency between implementations should be verified.

### ANO-11: Empty Default Feature Set [SEVERITY: INFORMATIONAL]

**Location**: `nine65/Cargo.toml`
**Evidence**: `default = []`
**Detail**: A bare `cargo build` of nine65 excludes ALL optional integrations: no clockwork, no mana/unhal, no nexgen_rational, no shadow entropy, no FFT-NTT, no parallelism, no serde.
**Assessment**: This is likely intentional for minimal builds, but means the default compilation exercises a relatively small fraction of the codebase. The `full` feature should be used for comprehensive testing.

### ANO-12: mana `#![forbid(unsafe_code)]` Claim vs. Float Reference [SEVERITY: NEGLIGIBLE]

**Location**: `mana/src/lib.rs` line 14
**Evidence**: Comment states "All operations use exact integer arithmetic. No f32, f64 anywhere."
**Assessment**: Verified true by grep. The comment is accurate. No float types appear in mana source code.

### ANO-13: GearStack Guard Bits Hardcoded [SEVERITY: LOW]

**Location**: `clockwork-core/src/gearstack.rs`
**Evidence**: `const DEFAULT_GUARD_BITS: u32 = 8;`
**Assessment**: This constant controls the safety margin for bound tracking. It is hardcoded rather than configurable. Whether 8 bits is sufficient depends on the operation depth. The formal spec should be consulted for the derivation of this value.

### ANO-14: nine65 lib.rs Has Inner Attribute Placement [SEVERITY: NEGLIGIBLE]

**Location**: `nine65/src/lib.rs`
**Evidence**: `#![forbid(unsafe_code)]` is at the crate root level, providing strong safety guarantees
**Assessment**: Correctly placed. All sub-modules inherit this restriction. The only unsafe code in the workspace is in clockwork-core (a separate crate), which is correct since nine65 delegates key zeroing to clockwork-core.

---

## SUMMARY

### Workspace Health

| Metric | Value |
|--------|-------|
| Total .rs files | 108 |
| Test functions | 777 |
| Test modules | 96 |
| Float violations (code) | 1 file (compiler.rs, documented exception) |
| Unsafe blocks | 1 (key_lifecycle.rs, justified) |
| TODO/FIXME/HACK/XXX/STUB | 0 active (1 historical comment) |
| Dead code annotations | 9 |
| unwrap/expect in non-test code | ~150+ (see ANO-05) |
| Feature gates | 104 in nine65 alone |
| Cross-crate wiring issues | 0 |
| Orphan modules | 0 |

### Risk Assessment

| ID | Anomaly | Severity | Action Recommended |
|----|---------|----------|-------------------|
| ANO-01 | Float in compiler.rs | DOCUMENTED | None (intentional, scoped) |
| ANO-02 | Float comment in security_estimator | LOW | None |
| ANO-03 | Commented-out lint vs. doc claim | MEDIUM | Fix doc comment or uncomment lint |
| ANO-04 | Unsafe in key_lifecycle.rs | DOCUMENTED | None (A4 requirement) |
| ANO-05 | Extensive unwrap/expect | MEDIUM | Audit rns_fhe.rs (35 unwraps) |
| ANO-06 | Historical STUB comment | NEGLIGIBLE | None |
| ANO-07 | Dead code in rns_fhe.rs | LOW | Review for cleanup |
| ANO-08 | Dead code in security_estimator | LOW | Review for cleanup |
| ANO-09 | Triplicated K-elimination | INFORMATIONAL | Verify algorithmic consistency |
| ANO-10 | Duplicated Montgomery | INFORMATIONAL | Verify algorithmic consistency |
| ANO-11 | Empty default features | INFORMATIONAL | Document recommended feature set |
| ANO-12 | mana no-float claim | NEGLIGIBLE | None (claim verified) |
| ANO-13 | Hardcoded guard bits | LOW | Consider making configurable |
| ANO-14 | forbid(unsafe_code) placement | NEGLIGIBLE | None (correct) |

### Architecture Verdict

The `04_pre_exact_trans_git` workspace is a well-structured, modular FHE system with clear separation of concerns across 7 crates. The dependency graph is clean with no circular dependencies. Cross-crate wiring is consistently feature-gated. The integer-only mandate is enforced with a single documented exception (compiler.rs offline noise analysis). The formal spec coverage in clockwork-core is thorough with explicit theorem/invariant/definition references throughout.

The primary area of concern is the density of `unwrap()` / `expect()` calls in production code paths, particularly in `rns_fhe.rs` (35 occurrences) and `key_manager.rs` (14 occurrences). While many of these are mathematically justified (preconditions guaranteed by construction), they represent potential panic points that could benefit from conversion to `Result`-based error handling.

The triplicated K-elimination and duplicated Montgomery implementations are architectural features, not defects -- each serves a different layer with different performance characteristics. However, they create a maintenance burden: any algorithmic fix must be applied consistently across all implementations.

---

*End of forensic audit. No source files were modified.*

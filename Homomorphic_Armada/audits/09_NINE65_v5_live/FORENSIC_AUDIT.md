# FORENSIC AUDIT: 09_NINE65_v5_live

**Build**: `09_NINE65_v5_live` (symlink to `~/Projects/NINE65/v5/`)
**Audit Date**: 2026-02-14
**Auditor**: Claude Opus 4.6 (forensic code audit mode)
**Mandate**: INSPECT, ANALYZE, REPORT -- source NOT modified

---

## Table of Contents

1. [Structure Mapping](#1-structure-mapping)
2. [Data Flow Tracing](#2-data-flow-tracing)
3. [Construct Identification](#3-construct-identification)
4. [Wiring Verification](#4-wiring-verification)
5. [Dead Code Analysis](#5-dead-code-analysis)
6. [Cross-Reference Alignment](#6-cross-reference-alignment)
7. [Anomaly Catalogue](#7-anomaly-catalogue)

---

## 1. Structure Mapping

### 1.1 Workspace Root

**File**: `Cargo.toml`

- **Resolver**: 2
- **Workspace Members** (9 crates):
  1. `crates/nine65`
  2. `crates/clockwork-core`
  3. `crates/mana`
  4. `crates/unhal`
  5. `crates/nexgen_rational`
  6. `crates/exact_transcendentals`
  7. `crates/fhe-service`
  8. `crates/nine65-python`
  9. `crates/nine65-wasm`
- **Excluded from default members**: `fuzz`, `crates/nine65-python`, `crates/nine65-wasm`
- **Release Profile**: `opt-level = 3`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`
- **Workspace Dependencies**: wide, rayon, zeroize, getrandom, subtle, sha2, thiserror, rand_core, rand_chacha, criterion, proptest

### 1.2 Per-Crate Module Trees

#### 1.2.1 nine65 (Central FHE Crate)

```
nine65/src/
  lib.rs              -- #![forbid(unsafe_code)], compile_error for allow_insecure in release
  arithmetic/
    mod.rs            -- 17 submodules
    ntt.rs            -- Number Theoretic Transform (naive)
    ntt_fft.rs        -- NTT via FFT (default feature "ntt_fft")
    montgomery.rs     -- Montgomery modular arithmetic
    barrett.rs        -- Barrett reduction
    rns.rs            -- RNS representation
    k_elimination.rs  -- K-Elimination exact division
    exact_divider.rs  -- Exact integer division
    exact_coeff.rs    -- Exact coefficient operations
    ct_mul_exact.rs   -- Constant-time exact multiply
    mobiusint.rs      -- Mobius integer
    pade.rs           -- Pade approximants
    mq_relu.rs        -- MQ-ReLU activation
    integer_softmax.rs -- Integer softmax
    integer_math.rs   -- Integer math utilities
    cyclotomic_phase.rs -- Cyclotomic phase
    order_finding.rs  -- Order finding
    valuation.rs      -- p-adic valuation
    rational_bridge.rs -- [feature: exact_rational] nexgen_rational bridge
    bounded_rns.rs    -- [feature: clockwork] clockwork-core bridge
  compiler.rs         -- Bootstrap-free FHE compiler (offline tooling, f64 allowed)
  entropy/
    mod.rs            -- Entropy module root
    rng_trait.rs      -- FheRng trait
    secure.rs         -- OS CSPRNG (SecureRng)
    shadow.rs         -- ShadowHarvester (deterministic)
    deterministic.rs  -- [feature: deterministic_rng] ChaCha20 RNG
    crt_shadow.rs     -- [feature: shadow-entropy] CRT shadow
    wassan_noise.rs   -- [feature: shadow-entropy] Wassan noise
  errors/             -- Nine65Error, Nine65Result
  kat/                -- Known Answer Tests
  keys/
    mod.rs            -- KeySet, SecretKey, PublicKey, EvaluationKey (serde + validation)
  noise/
    mod.rs
    budget.rs         -- NoiseBudget tracker
    exact_noise.rs    -- ExactNoiseTracker
  ops/
    mod.rs            -- 8 submodules
    batch.rs          -- Batch encryption
    encrypt.rs        -- BFVEncoder, BFVEncryptor, Ciphertext
    galois.rs         -- GaloisEngine, GaloisKey, GaloisKeySet, GaloisEvaluator
    gso_fhe.rs        -- GSO-FHE noise bounding
    homomorphic.rs    -- BFVEvaluator (deprecated mul()), TrackedEvaluator
    neural.rs         -- Neural network operations
    parallel.rs       -- Parallel operations
    rns_fhe.rs        -- RNSFHEContext, DualRNS types (PRIMARY implementation)
    rns_mul.rs        -- DUPLICATE DualRNS types (ANOMALY, see Section 7)
  params/
    mod.rs            -- FHEConfig, FHEConfig::light_mul()
    primes.rs         -- NTT prime finding
    secure_configs.rs -- SecureConfig (128/192/256)
  ring/               -- RingPolynomial
  security/
    mod.rs            -- LWEParams, SecurityEstimate, HE Standard v1.1
    secret_data.rs    -- SecretData, SecretPoly, SecretScalar marker traits
    gro_gate.rs       -- [feature: clockwork] TimingGate
    key_manager.rs    -- [feature: clockwork] KeyManager
    integrity.rs      -- [feature: clockwork] limb checksums
  accelerated.rs      -- [feature: accelerated] AcceleratedFHE, AcceleratedRNS
  v2_integration_tests/ -- [feature: v2] integration tests module
```

**Source file count**: ~70 `.rs` files
**Bench targets**: 4 (`bench_ntt`, `bench_fhe`, `bench_rns_mul`, `bench_k_elimination`)
**Examples**: 1 (`basic_fhe`)
**Binaries**: 1 (`nine65_bench`)

#### 1.2.2 clockwork-core

```
clockwork-core/src/
  lib.rs              -- Formal Spec v1.0; float_arithmetic deny COMMENTED OUT (ANOMALY)
  basis.rs            -- D1-D4: RnsBasis, CRT encode/decode, centered lift, basis extension
  bound_tracker.rs    -- D7: Bound (integer-only bit-bound tracking)
  decode_to_q.rs      -- D9: DecodeToQ bridge from Clockwork to RLWE space (INV-1)
  garner.rs           -- D5-D6: K-Elimination, Garner decomposition, constant-time variants
  gearstack.rs        -- GearStack (value + bound + basis), arithmetic with bound tracking
  gro.rs              -- D13-D16: GRO timing model (golden ratio oscillator pair)
  integrity.rs        -- D22-D23: TripleRedundant, MajVote, TierState, fail-closed
  key_lifecycle.rs    -- D18-D21: KeySharePair, KeyLifecycle state machine, volatile zeroing
```

**Dependencies**: `subtle`, `crc32fast`
**Unsafe**: Single occurrence in `key_lifecycle.rs` (volatile zeroing via `core::ptr::write_volatile`)

#### 1.2.3 mana

```
mana/src/
  lib.rs              -- #![forbid(unsafe_code)], #![deny(missing_docs)]
  anchor.rs           -- KAnchor, AnchorContext (K-Elimination exact division)
  gso.rs              -- QbitState, QbitAgent, GsoSwarm (Glowworm Swarm Optimization)
  lane.rs             -- Lane, LaneOps, MontgomeryLane, PersistentLane (Montgomery form)
  stream.rs           -- ManaStream, StreamOps (multi-lane CRT)
  parallel.rs         -- [feature: parallel] ParallelStream, ParallelNTT, BatchParallel
```

**Dependencies**: `rayon` (optional), `zeroize`

#### 1.2.4 unhal

```
unhal/src/
  lib.rs              -- Re-exports MANA types
  accelerator.rs      -- Accelerator, AcceleratorConfig, ExecutionMode (auto-detect)
  pipeline.rs         -- Pipeline, PipelineBuilder, Stage
  batch.rs            -- BatchProcessor (bulk stream operations)
```

**Dependencies**: `mana`, `rayon` (optional)

#### 1.2.5 nexgen_rational

```
nexgen_rational/src/
  lib.rs              -- #![forbid(unsafe_code)], pure i128 rationals
  binary_gcd.rs       -- Binary GCD (Stein's algorithm)
  exact_coeff.rs      -- Exact coefficient operations
  rat_ng.rs           -- Rational number-geometric operations
  (+ submodules)
```

**Dependencies**: NONE (zero external deps)

#### 1.2.6 exact_transcendentals

```
exact_transcendentals/src/
  lib.rs              -- ExactRational, TranscendentalConstants, no_std compatible
  constants.rs        -- Pi, e, ln2, sqrt2 at 30/62-bit precision
  cordic.rs           -- CORDIC algorithm
  agm.rs              -- Arithmetic-Geometric Mean
  binary_splitting.rs -- Binary splitting for series
  continued_fraction.rs -- Continued fractions (sqrt)
  common_traits.rs    -- Shared traits
  benches/            -- Benchmark module
```

**Dependencies**: None (`no_std` support via `cfg_attr`)
**Features**: `std` (default), `arbitrary-precision`

#### 1.2.7 fhe-service

```
fhe-service/src/
  main.rs             -- #![deny(clippy::float_arithmetic)], REST API, TcpListener
  handlers.rs         -- Route dispatch, evaluate (add/sub/negate/add_plain/mul_plain/mul)
  session.rs          -- Session (key material), SessionStore (thread-safe, TTL-based)
  wire.rs             -- Serde request/response types, input validation
  http.rs             -- HTTP request parsing
```

**Dependencies**: `nine65` (with `serde` feature), `serde_json`, `bincode`, `base64`, `getrandom`
**Features**: `allow_insecure` (passes through to nine65)

#### 1.2.8 nine65-python

```
nine65-python/src/
  lib.rs              -- PyO3 bindings: 8 pyclass types
```

**Excluded from workspace default members**
**crate-type**: `cdylib`
**Requires**: `python` feature (pyo3)

#### 1.2.9 nine65-wasm

```
nine65-wasm/src/
  lib.rs              -- wasm-bindgen: WasmFHEContext + key/ct types
```

**Excluded from workspace default members**
**crate-type**: `cdylib`
**Requires**: `wasm` feature (wasm-bindgen)

### 1.3 Feature Flags (nine65)

| Feature | Default | Dependencies Activated | Purpose |
|---------|---------|----------------------|---------|
| `ntt_fft` | YES | none | Use FFT-based NTT (vs naive) |
| `parallel` | YES | `rayon` | Rayon parallelism |
| `secure-keygen` | no | none | Enforce secure keygen paths |
| `wassan` | no | none | Wassan noise field |
| `v2` | no | none | v2 integration tests |
| `accelerated` | no | `mana`, `unhal` | MANA stream acceleration |
| `secure_seed` | no | none | OS CSPRNG seed for ShadowHarvester |
| `debug_dual_mul` | no | none | Debug output for dual multiplication |
| `serde` | no | `serde`, `serde_json`, `bincode` | Serialization support |
| `shadow-entropy` | no | none | CRT shadow + Wassan noise |
| `allow_insecure` | no | none | Enable unvalidated deserialization |
| `slow_tests` | no | none | Expensive test coverage |
| `benchmarks` | no | none | Benchmark-only code |
| `deterministic_rng` | no | none | ChaCha20 deterministic RNG |
| `exact_rational` | no | `nexgen_rational` | Rational bridge |
| `clockwork` | no | `clockwork-core` | Formal RNS spec compliance |
| `exact_transcendentals_backend` | no | `exact_transcendentals` | Transcendental functions |

### 1.4 Dependency Graph

```
                          nine65
                       /  |  |  \  \
                      /   |  |   \  \
            clockwork-core |  |  nexgen_rational  exact_transcendentals
              (optional)   |  |    (optional)        (optional)
                          /    \
                       mana   unhal
                  (optional)  (optional, depends on mana)
                       |
                    [rayon]   (optional for both mana and nine65)

  fhe-service --depends-on--> nine65 (with serde)
  nine65-python --depends-on--> nine65
  nine65-wasm --depends-on--> nine65
  fuzz --depends-on--> nine65
```

### 1.5 Public API Surface (nine65 prelude)

The prelude re-exports approximately 50 types including:
- `FHEConfig`, `SecureConfig`
- `NTTEngine` / `NTTEngineFFT`
- `RingPolynomial`
- `BFVEncoder`, `BFVEncryptor`, `Ciphertext`
- `BFVEvaluator`, `TrackedEvaluator`
- `SecretKey`, `PublicKey`, `EvaluationKey`, `KeySet`
- `GaloisEngine`, `GaloisKey`, `GaloisKeySet`, `GaloisEvaluator`
- `NoiseBudget`, `ExactNoiseTracker`
- `RNSFHEContext`, `DualRNSPoly`, `DualRNSCiphertext`, etc.
- `ShadowHarvester`, `SecureRng`, `FheRng`
- `KElimination`, `KAnchorPair`
- `LWEParams`, `SecurityEstimate`
- `Nine65Error`, `Nine65Result`

---

## 2. Data Flow Tracing

### 2.1 FHE Pipeline (Primary Path)

```
User plaintext value (u64)
    |
    v
BFVEncoder::encode()          -- Scale by Delta = floor(q/t)
    |
    v
BFVEncryptor::encrypt()       -- ct = (c0, c1) where c0 = pk0*u + e0 + m, c1 = pk1*u + e1
    |                            Entropy from SecureRng (production) or ShadowHarvester (test)
    v
Ciphertext { c0, c1 }         -- RingPolynomial pair
    |
    +--- BFVEvaluator::add()     -- ct_add = (c0_a + c0_b, c1_a + c1_b)
    +--- BFVEvaluator::add_plain() -- ct + encoded plaintext
    +--- BFVEvaluator::mul_plain() -- ct * encoded plaintext
    +--- BFVEvaluator::mul()     -- [DEPRECATED] -> use RNSFHEContext::mul_dual_symmetric()
    +--- GaloisEvaluator::rotate_left/right/conjugate()  -- Slot rotations
    |
    v
BFVEncoder::decode()          -- Recover plaintext from decrypted polynomial
    |
    v
Decrypted plaintext (u64)
```

### 2.2 DualRNS Pipeline (Advanced Multiplication)

```
RNSFHEContext::new(config, ntt_engines, ke)
    |
    +--- dual_rns: DualRNSContext     -- Main primes + anchor primes
    +--- rns: RNSContext              -- Single-prime RNS
    +--- ntt_engines: Vec<NTTEngine>  -- One per RNS limb
    +--- ke: KElimination             -- Exact division engine
    |
    v
DualRNSPoly::from_poly()      -- Convert RingPolynomial to DualRNS form
    |
    v
DualRNSCiphertext              -- Pair of DualRNSPoly
    |
    v
mul_dual_symmetric()           -- Core multiplication:
    |                             1. Tensor product in NTT domain
    |                             2. K-Elimination rescale (exact division by t)
    |                             3. Relinearization via eval key
    |
    v
DualRNSCiphertext (result)     -- Reduced noise, ready for further ops
    |
    v
DualRNSPoly::to_poly()        -- Convert back to RingPolynomial
```

### 2.3 Galois Rotation Flow

```
GaloisEngine::new(n, q)
    |
    +--- generator = 5 (for power-of-2 N)
    +--- generator_inv = 5^(-1) mod 2N
    |
    v
GaloisEngine::generate_rotation_keys()
    |
    +--- For each rotation r: generate GaloisKey with exponent 5^r mod 2N
    +--- Conjugation key: exponent = 2N - 1
    |
    v
GaloisEvaluator::rotate_left(ct, r)
    |
    1. Compute exponent = 5^r mod 2N
    2. Apply automorphism: X -> X^exponent (permute polynomial coefficients)
    3. Key switch c1 via decomposition + Galois key matrix
    |
    v
Rotated Ciphertext
```

### 2.4 Bootstrap-Free Compiler (Offline Tooling)

```
BootstrapFreeFHECompiler::new()
    |
    v
Circuit DAG (nodes: Add, Mul, RotateLeft, etc.)
    |
    v
NoiseAnalyzer::analyze()       -- Static noise analysis using f64 (OFFLINE ONLY)
    |
    v
ParameterSelector::select()   -- Choose parameters that keep noise below threshold
    |
    v
FHEConfig                     -- Parameters for runtime execution
```

**NOTE**: The compiler is standalone offline tooling. It is NOT connected to the runtime FHE pipeline. It explicitly allows `clippy::float_arithmetic` because it performs static analysis, not runtime cryptographic operations.

### 2.5 clockwork-core to nine65 Bridge

```
clockwork-core::Bound
    |
    v  (feature: "clockwork")
nine65::arithmetic::bounded_rns::BoundedValue
    |
    +--- wraps clockwork_core::Bound
    +--- provides on_add(), on_mul(), on_sub() that delegate to Bound update rules
    +--- check_headroom() for promotion decisions
    |
    v
Used within nine65 RNS operations for formal bound tracking
```

### 2.6 MANA/UNHAL to nine65 Bridge (feature: "accelerated")

```
nine65::accelerated::AcceleratedFHE
    |
    +--- poly_to_stream()      -- Convert RingPolynomial to ManaStream
    +--- stream_to_poly()      -- Convert ManaStream back to RingPolynomial
    +--- AcceleratedRNS        -- Drop-in SIMD/Rayon replacement
    |       +--- add_polys()   -- Lane-parallel polynomial add
    |       +--- mul_polys()   -- Lane-parallel polynomial mul
    |       +--- add_ciphertexts() -- Simplified (doesn't fully use MANA for ct add)
    |
    v
mana::stream::ManaStream      -- Multi-lane CRT parallel representation
    |
    v
unhal::accelerator::Accelerator -- Auto-selects Sequential/SIMD/Parallel/Full mode
```

### 2.7 fhe-service API Flow

```
Client HTTP Request
    |
    v
TcpListener::accept()         -- Raw TCP, no framework
    |
    v
handlers::handle_request()    -- Route dispatch:
    |
    +--- POST /v1/sessions     -> Session::new() with SecureConfig keygen
    +--- GET  /v1/sessions/:id -> SessionStore::with_session()
    +--- DELETE /v1/sessions/:id -> SessionStore::remove()
    +--- POST /v1/sessions/:id/encrypt -> BFVEncryptor::encrypt_secure()
    +--- POST /v1/sessions/:id/decrypt -> BFVEncoder::decode(sk decrypt)
    +--- POST /v1/sessions/:id/evaluate -> per-op dispatch:
    |       +--- "add"       -> evaluator.add(&ct_a, &ct_b)
    |       +--- "sub"       -> evaluator.sub(&ct_a, &ct_b)
    |       +--- "negate"    -> evaluator.negate(&ct)
    |       +--- "add_plain" -> evaluator.add_plain(&ct, scalar)
    |       +--- "mul_plain" -> evaluator.mul_plain(&ct, scalar)
    |       +--- "mul"       -> evaluator.mul_no_relin(&ct_a, &ct_b)
    |                            + evaluator.relinearize(&ct_3)
    +--- GET /healthz          -> 200 OK
    +--- GET /v1/version       -> version info
    +--- GET /v1/metrics       -> session count, uptime
    |
    v
JSON Response (serde_json)
```

**Key security properties**:
- Secret key NEVER leaves the server (enforced in Session struct)
- Session IDs generated via `getrandom` (16 random bytes, hex-encoded)
- Connection limit enforcement
- Session TTL with background reaper thread
- Input validation with allocation limits (64 MB per request max)
- Generic error responses prevent information leakage

---

## 3. Construct Identification

### 3.1 RNSFHEContext

**Location**: `crates/nine65/src/ops/rns_fhe.rs`, line ~624

```
pub struct RNSFHEContext {
    dual_rns: DualRNSContext,
    rns: RNSContext,
    ntt_engines: Vec<NTTEngine>,
    ke: KElimination,
    t: u64,
}
```

Central FHE context providing the complete dual-RNS multiplication pipeline. Contains:
- DualRNS context (main + anchor primes)
- Single-prime RNS context
- Per-limb NTT engines
- K-Elimination for exact rescaling
- Plaintext modulus t

### 3.2 mul_dual_symmetric()

**Location**: `crates/nine65/src/ops/rns_fhe.rs`, line ~2585

The core multiplication function for the bootstrap-free FHE scheme. Performs:
1. Tensor product in NTT domain (degree-3 ciphertext)
2. K-Elimination rescale (exact division by plaintext modulus t)
3. Relinearization back to degree-2 ciphertext

This is the CANONICAL multiplication path; `BFVEvaluator::mul()` is deprecated in favor of this function.

### 3.3 FHEConfig::light_mul()

**Location**: `crates/nine65/src/params/mod.rs`, line ~143

Testing-oriented configuration with small parameters suitable for unit tests and benchmarks. Not intended for production security.

### 3.4 DualRNS Type Hierarchy (rns_fhe.rs -- PRIMARY)

| Type | Description |
|------|-------------|
| `DualRNSPoly` | Polynomial in dual RNS representation |
| `DualRNSCiphertext` | (DualRNSPoly, DualRNSPoly) pair |
| `DualRNSSecretKey` | Secret key in DualRNS form |
| `DualRNSPublicKey` | Public key in DualRNS form |
| `DualRNSEvalKey` | Evaluation key in DualRNS form |
| `DualRNSKeySet` | (secret, public, eval) triple |
| `DualRNSFullKeySet` | KeySet + Galois keys |
| `DualRNSContext` | Context holding primes + precomputed values |

### 3.5 DualRNS Type Hierarchy (rns_mul.rs -- SECONDARY/ANOMALY)

A SECOND set of identically-named types exists in `rns_mul.rs`:
- `DualRNSPoly`, `DualRNSCiphertext`, `DualRNSSecretKey`, `DualRNSPublicKey`, `DualRNSKeySet`
- Also: `RNSEvaluator` with K-Elimination rescale, keygen, encrypt_dual

These shadow the types in `rns_fhe.rs`. See Anomaly A-01.

### 3.6 Clockwork Types (clockwork-core)

| Type | Formal Spec | Description |
|------|-------------|-------------|
| `RnsBasis` | D1-D4 | Pairwise coprime moduli with CRT |
| `Bound` | D7 | Integer bit-bound for centered-lift values |
| `DecodeToQ` | D9 | Bridge from Clockwork to RLWE (fixed q, INV-1) |
| `GarnerDigits` | D5 | Mixed-radix Garner decomposition |
| `k_eliminate` / `k_eliminate_ct` | D6 | K-Elimination (variable-time / constant-time) |
| `GearStack` | composite | Value + Bound + Basis with arithmetic |
| `GroGate` | D13-D16 | GRO timing model (software simulation) |
| `TripleRedundant<T>` | D22-D23 | Triple-redundant storage with MajVote |
| `TierState` | - | Tier metadata (num_gears, bound_bits, moduli) |
| `KeySharePair` | D18 | (s1, s2) with s1 + s2 = s mod q |
| `KeyLifecycle` | D20 | State machine: KEYGEN -> SPLIT -> ACTIVE -> ZEROED |

### 3.7 MANA Types

| Type | Description |
|------|-------------|
| `KAnchor` | K-Elimination anchor (alpha/beta primes) |
| `AnchorContext` | Full anchor context with stream creation |
| `ManaStream` | Multi-lane CRT representation |
| `Lane` | Single CRT prime modulus channel |
| `PersistentLane` | Coefficients permanently in Montgomery form |
| `MontgomeryLane` | Montgomery reduction context per lane |
| `QbitState` | Quantum-inspired amplitude state |
| `QbitAgent` | GSO agent with qbit state |
| `GsoSwarm` | Glowworm Swarm Optimization swarm |
| `ParallelStream` | Rayon-parallel stream wrapper |

### 3.8 UNHAL Types

| Type | Description |
|------|-------------|
| `Accelerator` | Main abstraction, auto-selects execution mode |
| `AcceleratorConfig` | Configuration (mode, threshold, threads) |
| `ExecutionMode` | Sequential / SIMD / Parallel / Full |
| `Pipeline` | Staged computation graph |
| `PipelineBuilder` | Builder pattern for pipelines |
| `Stage` | Pipeline stage (Add, Sub, Negate, ScalarMul, Custom) |
| `BatchProcessor` | Bulk stream operations |

### 3.9 Security Types

| Type | Description |
|------|-------------|
| `LWEParams` | LWE parameters for security estimation |
| `SecurityEstimate` | Classical/quantum bits, best attack, confidence |
| `SecretData` / `SecretPoly` / `SecretScalar` | Marker traits for constant-time enforcement |
| `TimingGate` | [clockwork] GRO-based timing isolation |
| `KeyManager` | [clockwork] Key lifecycle management |

### 3.10 ALL Constructs Inventory

**Total unique public types across all 9 crates**: ~120+

Breakdown by crate:
- nine65: ~60 types (FHE core, arithmetic, keys, noise, galois, security)
- clockwork-core: ~12 types (formal spec implementation)
- mana: ~15 types (CRT streams, lanes, GSO, anchors)
- unhal: ~8 types (accelerator, pipeline, batch)
- nexgen_rational: ~5 types (rational arithmetic)
- exact_transcendentals: ~5 types (ExactRational, TranscendentalConstants)
- fhe-service: ~12 types (session, wire request/response types)
- nine65-python: 8 pyclass types
- nine65-wasm: 5 wasm_bindgen types

---

## 4. Wiring Verification

### 4.1 DualRNS End-to-End

**Status**: WIRED (via rns_fhe.rs path)

Flow verified:
1. `DualRNSContext` created from `FHEConfig` primes
2. `DualRNSPoly::from_poly()` converts `RingPolynomial` to dual form
3. `DualRNSCiphertext` holds paired polys
4. `mul_dual_symmetric()` performs tensor product + K-Elimination rescale + relinearize
5. `DualRNSPoly::to_poly()` converts back

The secondary path in `rns_mul.rs` provides an alternative `RNSEvaluator` with its own `mul_rns_dual()`. Both paths exist but the `rns_fhe.rs` path is canonical.

### 4.2 Bootstrap-Free Compiler Connection

**Status**: NOT CONNECTED to runtime

The `compiler.rs` module is standalone offline tooling. It produces `FHEConfig` parameters but does not feed directly into the runtime FHE pipeline. There is no `compile_and_run()` or similar integration point. The compiler's output (parameter selections) must be manually applied.

This is by design -- the compiler uses f64 for static noise analysis (explicitly allowed with `#![allow(clippy::float_arithmetic)]`) and is not suitable for runtime paths.

### 4.3 fhe-service Exposes All Operations

**Status**: COMPLETE

All six homomorphic operations are exposed via the evaluate endpoint:

| Operation | Wire Name | Handler Implementation |
|-----------|-----------|----------------------|
| Addition | `"add"` | `evaluator.add(&ct_a, &ct_b)` |
| Subtraction | `"sub"` | `evaluator.sub(&ct_a, &ct_b)` |
| Negation | `"negate"` | `evaluator.negate(&ct)` |
| Plaintext Add | `"add_plain"` | `evaluator.add_plain(&ct, scalar)` |
| Plaintext Mul | `"mul_plain"` | `evaluator.mul_plain(&ct, scalar)` |
| Multiplication | `"mul"` | `evaluator.mul_no_relin() + evaluator.relinearize()` |

**Notable**: The `mul` handler correctly uses `mul_no_relin()` + `relinearize()` rather than the deprecated `BFVEvaluator::mul()`.

### 4.4 Deprecated Paths Still Functional

**Status**: FUNCTIONAL (behind `#[allow(deprecated)]` where used)

All deprecated APIs still compile and function. They are:
- Behind `#[deprecated]` attributes with migration guidance
- Used by nine65-python and nine65-wasm with `#[allow(deprecated)]`
- The `allow_insecure` gate controls unvalidated deserialization methods

### 4.5 Clockwork Integration

**Status**: FEATURE-GATED, WIRED

- `bounded_rns.rs` bridges `clockwork_core::Bound` to `BoundedValue`
- `security/gro_gate.rs` bridges GRO timing model
- `security/key_manager.rs` bridges key lifecycle
- `security/integrity.rs` bridges limb checksums
- All gated under `#[cfg(feature = "clockwork")]`

### 4.6 MANA/UNHAL Acceleration

**Status**: FEATURE-GATED, PARTIALLY WIRED

- `accelerated.rs` bridges MANA streams to RNS polynomials
- `AcceleratedFHE::poly_to_stream()` / `stream_to_poly()` conversions work
- `AcceleratedRNS::add_polys()` / `mul_polys()` delegate to MANA lanes
- **CAVEAT**: `AcceleratedRNS::add_ciphertexts()` is simplified and does not fully utilize MANA streams for ciphertext addition (performs plain poly addition instead)

### 4.7 Entropy Separation

**Status**: PROPERLY WIRED

| Use Case | Source | Module |
|----------|--------|--------|
| Production keygen | `SecureRng` (OS CSPRNG) | `entropy::secure` |
| Production encrypt | `SecureRng` | `entropy::secure` |
| Testing | `ShadowHarvester` (seed) | `entropy::shadow` |
| Reproducible tests | `DeterministicRng` (ChaCha20) | `entropy::deterministic` |
| fhe-service sessions | `KeySet::try_generate_secure()` | Uses `SecureRng` internally |
| fhe-service test sessions | `ShadowHarvester` | `#[cfg(test)]` only |

---

## 5. Dead Code Analysis

### 5.1 nine65-python

**Status**: FUNCTIONAL (not a stub)

8 pyclass types with complete implementations:
- `PyFHEConfig`, `PySecureConfig` -- parameter construction
- `PyFHEContext` -- full FHE context
- `PyKeySet`, `PyPublicKey`, `PySecretKey`, `PyEvaluationKey` -- key types
- `PyCiphertext` -- ciphertext with operations

Covers: keygen (secure + seeded), encrypt (secure + seeded), decrypt, add, add_plain, mul_plain, mul, batch_encrypt, batch_decrypt.

**ANOMALY**: `batch_encrypt` uses hardcoded seed 42 for `ShadowHarvester` (see A-04).

### 5.2 nine65-wasm

**Status**: FUNCTIONAL (not a stub)

5 wasm_bindgen types:
- `WasmFHEContext` -- full context with keygen, encrypt, decrypt, operations
- `WasmKeySet`, `WasmPublicKey`, `WasmSecretKey`, `WasmEvaluationKey`

Covers: keygen_seeded, encrypt_seeded, decrypt, add, add_plain, mul_plain, mul.

**Security**: `WasmSecretKey` export is deliberately disabled.
**All behind**: `#[cfg(feature = "wasm")]`

### 5.3 Potentially Unused Constructs

| Construct | Location | Status |
|-----------|----------|--------|
| `rns_mul.rs` DualRNS types | `ops/rns_mul.rs` | SHADOW types -- may be dead code relative to `rns_fhe.rs` canonical types |
| `compiler.rs` | `compiler.rs` | Standalone -- not integrated into runtime |
| `v2_integration_tests` | Feature-gated | Test-only code |
| `neural.rs` | `ops/neural.rs` | Present but usage unclear without reading full file |
| `cyclotomic_phase.rs` | `arithmetic/` | Specialized, may be used by neural ops |
| UNHAL SIMD mode | `accelerator.rs` | SIMD paths fall back to sequential (commented: "SIMD module removed") |

---

## 6. Cross-Reference Alignment

### 6.1 Feature Flag Consistency

| Feature | nine65 | clockwork-core | mana | unhal | fhe-service |
|---------|--------|---------------|------|-------|-------------|
| `parallel` | YES (default) | N/A | YES (optional) | YES (optional) | N/A |
| `serde` | optional | N/A | N/A | N/A | REQUIRED (enables nine65/serde) |
| `allow_insecure` | optional | N/A | N/A | N/A | optional (passes through) |
| `clockwork` | optional | ALWAYS (self) | N/A | N/A | N/A |
| `accelerated` | optional | N/A | REQUIRED (by accelerated) | REQUIRED (by accelerated) | N/A |

**Consistency check**: PASS. Feature propagation is correct.

### 6.2 Version Alignment

| Crate | Version | Workspace Version? |
|-------|---------|-------------------|
| nine65 | workspace | YES |
| clockwork-core | 0.1.0 (standalone) | NO |
| mana | workspace | YES |
| unhal | workspace | YES |
| nexgen_rational | workspace | YES |
| exact_transcendentals | 1.0.0 (standalone) | NO |
| fhe-service | workspace | YES |
| nine65-python | workspace | YES |
| nine65-wasm | workspace | YES |

**OBSERVATION**: `clockwork-core` and `exact_transcendentals` have standalone versions (0.1.0 and 1.0.0 respectively). This is intentional -- they are independently publishable crates.

### 6.3 NTT Engine Consistency

The NTT engine selection is controlled by the `ntt_fft` feature:
- When enabled (default): `NTTEngineFFT` is used as `NTTEngine`
- When disabled: naive `NTTEngine` is used

This is consistent across:
- `nine65/src/arithmetic/mod.rs` (re-export)
- `nine65/src/ops/galois.rs` (conditional import)
- `fhe-service/src/session.rs` (uses `nine65::arithmetic::NTTEngine`)

### 6.4 Error Type Alignment

- nine65: `Nine65Error` / `Nine65Result<T>`
- clockwork-core: `BasisError`, `GearError` (local error types)
- mana: Debug assertions (panics in debug, silent in release)
- unhal: Delegates to mana
- fhe-service: Maps nine65 errors to HTTP status codes
- exact_transcendentals: Returns `Option<T>` or panics

**OBSERVATION**: No unified error type across crates. This is acceptable for a workspace of independent crates.

### 6.5 Float-Free Compliance

| Crate | Float-Free Runtime | Lint | Notes |
|-------|-------------------|------|-------|
| nine65 | YES | `#![forbid(unsafe_code)]` (not float-specific) | f64 only in `compiler.rs` (offline) |
| clockwork-core | INTENDED but NOT ENFORCED | `#![deny(clippy::float_arithmetic)]` COMMENTED OUT | ANOMALY A-02 |
| mana | YES | `#![forbid(unsafe_code)]`, docs say "No f32, f64 anywhere" | Verified clean |
| unhal | YES | Inherits from mana | Verified clean |
| nexgen_rational | YES | `#![forbid(unsafe_code)]`, pure i128 | Verified clean |
| exact_transcendentals | YES (runtime) | f64 in `#[cfg(test)]` gated `to_f64()` methods only | Test-only f64 |
| fhe-service | YES | `#![deny(clippy::float_arithmetic)]` | Verified clean |

---

## 7. Anomaly Catalogue

### A-01: Duplicate DualRNS Types in rns_mul.rs

**Severity**: MEDIUM
**Location**: `crates/nine65/src/ops/rns_mul.rs`

`rns_mul.rs` defines a SECOND set of DualRNS types (`DualRNSPoly`, `DualRNSCiphertext`, `DualRNSSecretKey`, `DualRNSPublicKey`, `DualRNSKeySet`) that shadow the canonical definitions in `rns_fhe.rs`. Both files live in the `ops` module.

This creates:
- Name confusion (which `DualRNSPoly` is in scope depends on import path)
- Maintenance burden (two parallel type hierarchies)
- Potential for divergence between implementations

The `rns_mul.rs` variant also contains `RNSEvaluator` with its own `mul_rns_dual()` function, which appears to be an older or alternative multiplication path.

**Impact**: Import ambiguity. Neither set is deprecated. Consumers must use fully qualified paths to disambiguate.

### A-02: clockwork-core Float Arithmetic Deny Lint Commented Out

**Severity**: MEDIUM
**Location**: `crates/clockwork-core/src/lib.rs`

```rust
// In production, enable these:
// #![deny(clippy::float_arithmetic)]
```

The `deny(clippy::float_arithmetic)` lint is commented out with a note to enable it in production. This means clockwork-core currently PERMITS float arithmetic despite being a module that should enforce integer-only computation per the formal specification.

Currently no f64 usage exists in clockwork-core source, so this is a latent risk rather than an active violation. However, without the lint, accidental introduction of floating-point code would not be caught at compile time.

### A-03: Nightly-Only API Usage (is_multiple_of)

**Severity**: LOW
**Location**: Multiple files across nine65

The codebase uses `u64::is_multiple_of()` and `usize::is_multiple_of()` which are nightly-only Rust APIs (`feature(unsigned_is_multiple_of)`). The `lib.rs` has `#[allow(clippy::manual_is_multiple_of)]` suggesting awareness.

If the project targets stable Rust, this would be a compilation blocker. The project appears to target nightly.

### A-04: Hardcoded Seed in nine65-python batch_encrypt

**Severity**: HIGH (security)
**Location**: `crates/nine65-python/src/lib.rs`

```rust
// In batch_encrypt:
let mut harvester = ShadowHarvester::with_seed(42);
```

The `batch_encrypt` function in the Python bindings uses a hardcoded seed (42) for `ShadowHarvester`, making ALL batch encryptions deterministic and therefore insecure. Individual `encrypt_secure()` correctly uses `SecureRng`, but `batch_encrypt` does not.

This is a critical security flaw if `batch_encrypt` is used in production.

### A-05: nine65-python and nine65-wasm Use Deprecated mul()

**Severity**: LOW
**Location**: `crates/nine65-python/src/lib.rs`, `crates/nine65-wasm/src/lib.rs`

Both binding crates call the deprecated `BFVEvaluator::mul()` method with `#[allow(deprecated)]`. They should migrate to `RNSFHEContext::mul_dual_symmetric()` for the canonical multiplication path.

### A-06: UNHAL SIMD Mode is a No-Op

**Severity**: LOW
**Location**: `crates/unhal/src/accelerator.rs`

SIMD execution paths in the Accelerator contain the comment "SIMD module removed, fallback to sequential" and simply delegate to `a.add(b)`. The `ExecutionMode::Simd` and `ExecutionMode::Full` modes exist but provide no actual SIMD acceleration.

### A-07: exact_transcendentals Constants Module Has Pub f64 Functions

**Severity**: LOW
**Location**: `crates/exact_transcendentals/src/constants.rs`

Functions `from_scaled_30()` and `to_scaled_30()` are `pub` functions that involve f64 arithmetic. While they appear to be used primarily for test validation, they are not `#[cfg(test)]` gated and could theoretically be called from production code.

### A-08: fhe-service Uses Poison Recovery for RwLock

**Severity**: INFO
**Location**: `crates/fhe-service/src/session.rs`

The `SessionStore` methods use `unwrap_or_else(|e| e.into_inner())` for all `RwLock` operations, recovering from lock poisoning. This is documented and intentional (availability over consistency), but means a panic in one session handler could leave another session in an inconsistent state.

### A-09: Validation Limits May Be Insufficient

**Severity**: LOW
**Location**: `crates/fhe-service/src/wire.rs`

- `MAX_CIPHERTEXT_FIELD_LEN` = 512 KB per field
- `MAX_REQUEST_ALLOCATION` = 64 MB per request
- Maximum 1024 ciphertexts per encrypt/decrypt request
- Maximum 256 operations per evaluate request

These limits are reasonable but the 64 MB per-request allocation limit could still allow memory pressure under concurrent requests (64 sessions x 64 MB = 4 GB theoretical maximum).

### A-10: No Rate Limiting on fhe-service

**Severity**: LOW
**Location**: `crates/fhe-service/src/main.rs`

The fhe-service implements connection limits and session TTL but has no per-client rate limiting. A single client could rapidly create/destroy sessions or issue expensive operations.

### A-11: compiler.rs Uses f64 for Static Analysis

**Severity**: INFO (by design)
**Location**: `crates/nine65/src/compiler.rs`

The bootstrap-free FHE compiler uses f64 extensively for noise analysis (fields like `add_noise_bits`, `mul_noise_bits` in `NoiseModel`). This is explicitly allowed with `#![allow(clippy::float_arithmetic)]` and is offline tooling, not runtime code. Documented here for completeness.

### A-12: Missing Galois Rotation Support in fhe-service

**Severity**: MEDIUM
**Location**: `crates/fhe-service/src/handlers.rs`

The evaluate endpoint supports 6 operations (add, sub, negate, add_plain, mul_plain, mul) but does NOT expose Galois rotations (rotate_left, rotate_right, conjugate). These operations are fully implemented in nine65's `GaloisEvaluator` but not wired into the service API.

---

## Summary Statistics

| Metric | Value |
|--------|-------|
| Total crates | 9 (+ fuzz) |
| Total .rs source files | ~115 |
| Feature flags (nine65) | 17 |
| Public types | ~120+ |
| Deprecated APIs | ~27 |
| Anomalies found | 12 (1 HIGH, 2 MEDIUM, 6 LOW, 3 INFO) |
| Float violations (runtime) | 0 |
| Float usage (offline/test) | compiler.rs, exact_transcendentals tests |
| Unsafe blocks | 1 (clockwork-core key_lifecycle.rs volatile zeroing) |
| forbid(unsafe_code) crates | nine65, mana, nexgen_rational |
| deny(clippy::float_arithmetic) | fhe-service (active), clockwork-core (COMMENTED OUT) |

---

## Critical Findings Summary

1. **A-04 (HIGH)**: `nine65-python` `batch_encrypt` uses hardcoded seed 42, making all batch encryptions deterministic and insecure in production.

2. **A-01 (MEDIUM)**: Duplicate DualRNS type definitions in `rns_mul.rs` shadow the canonical `rns_fhe.rs` types, creating import ambiguity and maintenance burden.

3. **A-02 (MEDIUM)**: clockwork-core's `deny(clippy::float_arithmetic)` lint is commented out, removing compile-time float detection for this formal-spec crate.

4. **A-12 (MEDIUM)**: Galois rotation operations are fully implemented in nine65 but not exposed through the fhe-service REST API.

---

*End of Forensic Audit Report*
*Auditor: Claude Opus 4.6*
*Date: 2026-02-14*

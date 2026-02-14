# Forensic Audit Report: 02_symmetric_public

**Build**: `02_symmetric_public`
**Path**: `/home/acid/Projects/Homomorphic_Armada/builds/02_symmetric_public/`
**Type**: NINE65 workspace (7 crates). Public symmetric release.
**Auditor**: Forensic Code Auditor (claude-opus-4-6)
**Date**: 2026-02-14
**Status**: INSPECT / ANALYZE / REPORT -- no source modifications

---

## Table of Contents

1. [Structure Mapping](#1-structure-mapping)
2. [Data Flow Tracing](#2-data-flow-tracing)
3. [Construct Identification](#3-construct-identification)
4. [Wiring Verification](#4-wiring-verification)
5. [Dead Code Detection](#5-dead-code-detection)
6. [Cross-Reference Check](#6-cross-reference-check)
7. [Anomaly Catalogue](#7-anomaly-catalogue)

---

## 1. Structure Mapping

### 1.1 Workspace Layout

```
02_symmetric_public/
  Cargo.toml                    # Workspace root
  avatar.rs                     # ORPHAN (not in any crate)
  pipeline.rs                   # ORPHAN (not in any crate)
  random_encrypt_proptest.rs    # ORPHAN (not in any crate)
  fuzz/
    fuzz_targets/
      fuzz_ntt.rs
      fuzz_homomorphic.rs
      fuzz_k_elimination.rs
      fuzz_deserialize.rs
      fuzz_encrypt_decrypt.rs
  crates/
    clockwork-core/             # Formal-spec RNS arithmetic
    mana/                       # MANA stream accelerator
    nexgen_rational/            # Exact rational arithmetic
    nine65/                     # Core FHE library
    nine65-python/              # PyO3 bindings
    nine65-wasm/                # WASM bindings
    unhal/                      # Hardware abstraction layer
```

**Workspace Cargo.toml**:
- `members = ["crates/*"]`, `exclude = ["fuzz"]`, `resolver = "2"`
- Release profile: `opt-level = 3`, `lto = "fat"`, `codegen-units = 1`, `panic = "abort"`
- Workspace deps: wide, rayon, zeroize, getrandom, subtle, sha2, thiserror, rand_core, rand_chacha, criterion, proptest

### 1.2 Module Tree Per Crate

#### clockwork-core (8 modules)

```
clockwork-core/src/
  lib.rs
  basis.rs          # RnsBasis: CRT encode/decode/centered_lift/promote/demote
  garner.rs         # K-Elimination: k_eliminate(), k_eliminate_ct(), GarnerDigits
  gearstack.rs      # GearStack: RNS arithmetic with bound tracking
  bound_tracker.rs  # Bound: bit-width tracking for overflow detection
  decode_to_q.rs    # DecodeToQ: bridge from RNS space to Z_q (RLWE)
  integrity.rs      # TripleRedundant<T>: CRC32 majority vote (D22-D23)
  gro.rs            # GroGate: Golden Ratio Oscillator side-channel protection
  key_lifecycle.rs  # KeySharePair, KeyLifecycle: key state machine + secure zeroing
```

Re-exports: `RnsBasis`, `GearStack`, `k_eliminate`, `Bound`, `DecodeToQ`, `TripleRedundant`, `GroGate`

#### mana (5 modules)

```
mana/src/
  lib.rs
  lane.rs           # Lane, MontgomeryLane, PersistentLane: per-prime compute lanes
  stream.rs         # ManaStream, StreamOps: multi-lane CRT representation
  anchor.rs         # KAnchor, AnchorContext: K-Elimination anchors for FHE
  gso.rs            # QbitAgent, GsoSwarm: Glowworm Swarm Optimization (integer-only)
  parallel.rs       # ParallelStream, ParallelNTT, BatchParallel (feature: "parallel")
```

Re-exports (prelude): `Lane`, `LaneOps`, `ManaStream`, `StreamOps`, `KAnchor`, `AnchorContext`, `GsoSwarm`, `QbitAgent`, `ParallelStream`

#### nexgen_rational (8 files, 3 module groups)

```
nexgen_rational/src/
  lib.rs
  binary_gcd.rs     # Stein's binary GCD (integer-only, O(log n))
  exact_coeff.rs    # ExactCoeff(i128): checked arithmetic newtype
  rat_ng/
    mod.rs          # Re-exports: NexGenRat, DenState, DivOut, ArithmeticError
    types.rs        # NexGenRat struct, DenState enum, DivOut enum (T8 trichotomy)
    error.rs        # ArithmeticError enum (DivideByZero, Overflow, Unimplemented, InvalidDenominator)
    normalize.rs    # should_normalize(), normalize(), NormalizationScheduler (I3)
    policy.rs       # divide_coeff(), divide_coeff_checked() (I1/I2 division policy)
    ops.rs          # add/sub/mul/div/neg + operator trait impls (I4 overflow protection)
```

Re-exports: `NexGenRat`, `DenState`, `DivOut`, `ArithmeticError`

#### nine65 (core crate, 10+ module groups)

```
nine65/src/
  lib.rs
  errors.rs         # Nine65Error enum (21 variants), Nine65Result type alias
  kat.rs            # Known Answer Tests: KATVector, KATOperation, KATResult
  compiler.rs       # NoiseModel, CompilationResult (USES f64 -- documented exception)
  accelerated.rs    # AcceleratedFHE, AcceleratedRNS (feature: "accelerated")
  arithmetic/       # 21 files -- see below
  entropy/          # 7 files -- see below
  keys/             # 1 file (mod.rs)
  noise/            # 3 files -- see below
  ops/              # 10 files -- see below
  params/           # 7 files -- see below
  ring/             # 3 files -- see below
  security/         # 5 files -- see below
```

**nine65/src/arithmetic/ (21 files)**:
```
  mod.rs               # 18 submodule declarations
  montgomery.rs        # Montgomery modular multiplication
  persistent_montgomery.rs  # Persistent Montgomery form (stay-in-domain)
  barrett.rs           # Barrett reduction
  ntt.rs               # Number Theoretic Transform (basic)
  ntt_fft.rs           # NTT with FFT-style butterflies (feature: "ntt_fft")
  rns.rs               # Residue Number System representation
  k_elimination.rs     # K-Elimination exact division
  exact_divider.rs     # ExactDivider: modular inverse precomputation
  exact_coeff.rs       # Exact coefficient type for FHE
  ct_mul_exact.rs      # Constant-time exact ciphertext multiplication
  mobius_int.rs        # Mobius function (integer-only)
  pade_engine.rs       # Pade approximation engine
  mq_relu.rs           # Modular-Q ReLU activation
  integer_softmax.rs   # Integer-only softmax
  cyclotomic_phase.rs  # Cyclotomic polynomial phase
  valuation.rs         # p-adic valuation
  order_finding.rs     # Multiplicative order finding
  integer_math.rs      # General integer math utilities
  rational_bridge.rs   # Bridge to nexgen_rational (feature: "exact_rational")
  bounded_rns.rs       # Bridge to clockwork-core (feature: "clockwork")
```

**nine65/src/entropy/ (7 files)**:
```
  mod.rs               # 3 always-on + feature-gated submodules
  shadow.rs            # ShadowHarvester: deterministic PRNG
  secure.rs            # OS CSPRNG wrappers (getrandom)
  rng_trait.rs         # EntropySource trait
  deterministic.rs     # DeterministicRng (feature: test/deterministic_rng)
  wassan_noise.rs      # Wasserstein noise sampling (feature: "shadow-entropy")
  crt_shadow.rs        # CRT-based shadow entropy (feature: "shadow-entropy")
```

**nine65/src/keys/ (1 file)**:
```
  mod.rs               # SecretKey, PublicKey, EvaluationKey, KeySet
```

**nine65/src/noise/ (3 files)**:
```
  mod.rs               # NoiseBudgetTracker, EMACalculator, MultiWindowNoiseDetector, etc.
  budget.rs            # Noise budget tracking
  exact_noise.rs       # Exact noise measurement
```

**nine65/src/ops/ (10 files)**:
```
  mod.rs               # 9 submodule declarations
  encrypt.rs           # BFVEncoder, BFVEncryptor, BFVDecryptor, Ciphertext
  homomorphic.rs       # BFVEvaluator: add, mul, mul_plain, add_plain
  batch.rs             # Batch operations
  parallel.rs          # Parallel FHE operations
  rns_mul.rs           # RNS-based ciphertext multiplication
  rns_fhe.rs           # RNSFHEContext: full RNS FHE operations
  neural.rs            # Neural network layers over FHE
  galois.rs            # Galois automorphisms (rotation, conjugation)
  gso_fhe.rs           # GSO-optimized FHE operations
```

**nine65/src/params/ (7 files)**:
```
  mod.rs               # FHEConfig struct + 15+ preset configurations
  primes.rs            # NTT-compatible prime finding/validation
  production.rs        # Production parameter recommendations
  validation.rs        # validate_params(), ParameterValidator, ValidationResult
  security_estimator.rs # LatticeSecurityEstimator, HEStandardBounds
  secure_configs.rs    # SecureConfig: production-safe parameter sets
  exact_params.rs      # ExactDelta (feature: "exact_rational")
```

**nine65/src/ring/ (3 files)**:
```
  mod.rs               # Re-exports RingPolynomial, PolynomialPool
  polynomial.rs        # RingPolynomial: R_q = Z_q[X]/(X^N + 1)
  pool.rs              # PolynomialPool, PooledPolynomial, PoolGuard
```

**nine65/src/security/ (5 files)**:
```
  mod.rs               # LWEParams, SecurityEstimate, ConfidenceLevel + HE Standard table
  secret_data.rs       # Secret data handling
  gro_gate.rs          # GRO gate integration (feature: "clockwork")
  key_manager.rs       # Key management (feature: "clockwork")
  integrity.rs         # Integrity checking (feature: "clockwork")
```

#### nine65-python (1 source file)

```
nine65-python/src/
  lib.rs               # PyO3 bindings: 8 Python classes
```

Python classes: `FHEConfig`, `SecureConfig`, `FHEContext`, `KeySet`, `PublicKey`, `SecretKey`, `EvaluationKey`, `Ciphertext`

#### nine65-wasm (1 source file)

```
nine65-wasm/src/
  lib.rs               # WASM bindings: 5 JS classes (feature: "wasm")
```

WASM classes: `WasmFHEContext`, `WasmKeySet`, `WasmPublicKey`, `WasmSecretKey`, `WasmEvaluationKey`

#### unhal (3 source files)

```
unhal/src/
  lib.rs               # Prelude re-exports MANA types
  accelerator.rs       # Accelerator: ExecutionMode dispatch
  pipeline.rs          # Pipeline/PipelineBuilder: staged compute
  batch.rs             # BatchProcessor: parallel batch ops
```

### 1.3 Inter-Crate Dependency Graph

```
                    nine65  (core)
                   /  |  |  \
                  /   |  |   \
                 /    |  |    \
   [accelerated]  [clockwork]  [exact_rational]
        |              |              |
      unhal    clockwork-core   nexgen_rational
        |
      mana

   nine65-python --> nine65
   nine65-wasm   --> nine65
```

Dependency details:
- **nine65** -> **mana** (optional, feature `accelerated`)
- **nine65** -> **unhal** (optional, feature `accelerated`)
- **nine65** -> **clockwork-core** (optional, feature `clockwork`)
- **nine65** -> **nexgen_rational** (optional, feature `exact_rational`)
- **unhal** -> **mana** (always)
- **nine65-python** -> **nine65** (always)
- **nine65-wasm** -> **nine65** (always)
- **clockwork-core**: standalone (subtle, crc32fast)
- **mana**: standalone (wide, rayon, zeroize)
- **nexgen_rational**: standalone (zero runtime deps)

### 1.4 Feature Flag Map

**nine65 features**:
| Feature | Dependencies Activated | Purpose |
|---------|----------------------|---------|
| `default` | `ntt_fft`, `parallel` | Standard build |
| `ntt_fft` | -- | FFT-style NTT engine |
| `parallel` | `rayon` | Parallel operations |
| `accelerated` | `mana`, `unhal` | MANA stream acceleration |
| `clockwork` | `clockwork-core`, `crc32fast` | Formal-spec RNS, integrity, GRO |
| `exact_rational` | `nexgen_rational` | Exact rational bridge |
| `serde` | `serde`, `serde_json`, `bincode` | Serialization |
| `shadow-entropy` | -- | Wasserstein + CRT shadow noise |
| `secure-keygen` | `getrandom`, `sha2` | OS CSPRNG key generation |
| `wassan` | -- | Wasserstein noise integration |
| `v2` | -- | V2 integration tests |
| `allow_insecure` | -- | Access to insecure test configs |
| `deterministic_rng` | -- | Deterministic RNG for tests |

**nine65-python features**: `default = ["pyo3/extension-module"]`
**nine65-wasm features**: `wasm` (gates all code)
**mana features**: `default = ["parallel"]`, `parallel = ["rayon"]`, `simd` (commented out)
**unhal features**: `default = ["parallel"]`, `parallel = ["rayon"]`, `simd` (empty)

---

## 2. Data Flow Tracing

### 2.1 FHE Pipeline (Primary Path)

```
FHEConfig           SecureConfig (verified params)
    |                    |
    v                    v
NTTEngine::new()    FHEConfig::standard_128() / SecureConfig::secure_128().to_config()
    |                    |
    v                    |
KeySet::generate_secure(&config, &ntt)
    |
    +--> SecretKey::generate_secure(&config)
    |       |- secure_ternary_vector(n)  [OS CSPRNG via getrandom]
    |       |- RingPolynomial::from_coeffs()
    |
    +--> PublicKey::generate_secure(&sk, &config, &ntt)
    |       |- secure_uniform_vector(n, q)
    |       |- secure_cbd_vector(n, eta)
    |       |- a.mul_ct(&sk.s, ntt)      [constant-time multiply]
    |       |- neg_as.add(&e, ntt)       -> (pk0, pk1)
    |
    +--> EvaluationKey::generate_secure(&sk, &config, &ntt)
            |- ShadowHarvester::from_os_seed()  [OS-seeded deterministic]
            |- s_squared = sk.s.mul_ct(&sk.s, ntt)
            |- for each level: rlk[i] = (-a_i*s + e_i + s^2 * T^i, a_i)

BFVEncoder::new(&config)
    |- delta = q / t
    |- delta_half = delta / 2

BFVEncryptor::new(&pk, &encoder, &ntt, eta)
    |- encrypt_secure(msg) -> Ciphertext
    |    |- msg < t   [range check]
    |    |- u = random_ternary  [CBD]
    |    |- e1, e2 = random_cbd  [error terms]
    |    |- c0 = pk0*u + e1 + delta*msg  [mod q]
    |    |- c1 = pk1*u + e2              [mod q]

BFVDecryptor::new(&sk, &encoder, &ntt)
    |- decrypt(&ct) -> u64
    |    |- noisy = c0 + c1*s  [mod q]
    |    |- round(noisy * t / q) mod t

BFVEvaluator::new(&ntt, &encoder, eval_key)
    |- add(ct1, ct2)      -> Ciphertext  [c0+c0, c1+c1]
    |- add_plain(ct, val)  -> Ciphertext  [c0+delta*val, c1]
    |- mul_plain(ct, val)  -> Ciphertext  [c0*val, c1*val]
    |- mul(ct1, ct2)       -> Ciphertext  [tensor product + relinearize]
```

### 2.2 MANA Lane/Stream Pipeline (Accelerated Path)

```
ManaStream::new(primes, n)
    |
    +--> Lane::new(coeffs, prime)         [per-prime coefficient vector]
    |       |- mod_add(a, b, p)           [branchless modular add]
    |       |- mod_sub(a, b, p)           [branchless modular sub]
    |
    +--> MontgomeryLane::new(prime)       [Montgomery form]
    |       |- to_mont(x) = x * R mod p
    |       |- mont_mul(a, b) -> a*b*R^{-1} mod p
    |
    +--> PersistentLane                    [Arc<MontgomeryLane>, stays in Montgomery]
    |
    +--> StreamOps::add/sub/mul()          [lane-parallel operations]

KAnchor::new(alpha_primes, beta_primes)
    |- extract_k(x_M, x_A) -> k           [K-Elimination core]
    |- exact_divide(value, divisor) -> quotient
    |- scale_and_round(ct_lanes)           [FHE rescaling without float]

AnchorContext::new(anchor, stream)
    |- Wraps KAnchor with stream operations
```

### 2.3 Clockwork-Core Integration (Feature-Gated)

```
RnsBasis::new(moduli)
    |- encode(value) -> Vec<u64>          [CRT forward map]
    |- decode(residues) -> u128           [CRT reconstruction via Garner]
    |- centered_lift(residues) -> i128    [signed representation]
    |- promote(gs, new_modulus)           [add a modulus]
    |- demote(gs, old_modulus)            [drop a modulus]

GearStack::from_value(value, basis)
    |- add/sub/mul                        [with Bound tracking]
    |- verify_bound()                     [D7 enforcement]
    |- needs_promotion(guard_bits)        [overflow prediction]
    |- dot_product()                      [with bound accumulation]

DecodeToQ::new(q, centered)
    |- Maps from RNS residue space to Z_q for RLWE operations
    |- INV-1: q is immutable after construction

TripleRedundant<T>::new(value)
    |- read() -> VoteResult<T>            [majority vote + CRC32]
    |- write(value)                       [update all 3 copies + CRC]
    |- repair()                           [restore redundancy after corruption]

GroGate::golden_ratio()
    |- is_window() -> bool                [side-channel timing gate]
    |- next_window() -> u64               [schedule next window]

KeySharePair::split(key)
    |- reconstruct() -> key               [XOR-based 2-of-2 sharing]
    |- reshare(rng)                       [re-randomize shares]
```

---

## 3. Construct Identification

### 3.1 clockwork-core

| Construct | Kind | Purpose |
|-----------|------|---------|
| `RnsBasis` | struct | CRT basis with moduli, capacity, and precomputed CRT coefficients |
| `BasisError` | enum | Errors: NotCoprime, ModulusTooSmall, CapacityExceeded, EncodingOverflow, IncompatibleBasis |
| `GearStack` | struct | RNS-represented value with bound tracking |
| `GearError` | enum | Errors: BoundExceeded, PromotionRequired, IncompatibleBasis |
| `Bound` | struct | Bit-width tracker (u32), static methods for add/sub/mul bounds |
| `DecodeToQ` | struct | Bridge from RNS to Z_q; immutable q (INV-1) |
| `DecodeConvention` | enum | Standard, Centered, Raw |
| `TripleRedundant<T>` | struct | Three copies + CRC32 checksum (D22) |
| `VoteResult<T>` | enum | AllAgree, Recovered, Failed |
| `CorruptedCopy` | enum | A, B, C |
| `AsBytes` | trait | Serialization for CRC computation |
| `TierState` | struct | Gear metadata: num_gears, bound_bits, active_moduli |
| `GroGate` | struct | DDS oscillator pair for side-channel-resistant timing |
| `KeySharePair` | struct | 2-of-2 XOR secret sharing |
| `KeyState` | enum | Active, Split, Expired, Destroyed |
| `KeyLifecycle` | struct | Key state machine with auto-zeroing Drop |
| `k_eliminate()` | fn | K-Elimination exact division |
| `k_eliminate_ct()` | fn | Claimed constant-time K-Elimination |
| `GarnerDigits` | struct | Mixed-radix digit decomposition |
| `garner_decompose()` | fn | Garner decomposition for CRT |
| `garner_decompose_ct()` | fn | Claimed constant-time Garner decomposition |
| `zero_u64()` | fn | Volatile memory zeroing (sole unsafe in crate) |

### 3.2 mana

| Construct | Kind | Purpose |
|-----------|------|---------|
| `Lane` | struct | Per-prime coefficient vector with modular ops |
| `LaneOps` | trait | add, sub, neg, mul, scalar_mul, scalar_add |
| `MontgomeryLane` | struct | Montgomery reduction context per prime |
| `PersistentLane` | struct | Arc-wrapped MontgomeryLane (coefficients never leave Montgomery form) |
| `ManaStream` | struct | Multi-lane CRT representation (lanes, primes, product_cache) |
| `StreamOps` | trait | add, sub, neg, mul, scalar_mul on streams |
| `KAnchor` | struct | K-Elimination anchor pair (alpha/beta primes) |
| `AnchorContext` | struct | KAnchor wrapped with stream operations |
| `QbitState` | struct | Quantum-inspired amplitude for GSO |
| `QbitAgent` | struct | GSO agent with position, luciferin, neighborhood |
| `GsoSwarm` | struct | Full swarm of QbitAgents |
| `GsoParams` | struct | Swarm parameters (all integer, per-mille rates) |
| `SwarmStats` | struct | Swarm statistics |
| `ParallelStream` | struct | ManaStream with par_iter operations |
| `ParallelNTT` | struct | Lane-level NTT parallelism |
| `BatchParallel` | struct | Batch stream operations |

### 3.3 nexgen_rational

| Construct | Kind | Purpose |
|-----------|------|---------|
| `ExactCoeff` | struct (newtype) | i128 wrapper with checked arithmetic |
| `NexGenRat` | struct | Exact rational (num/den/state), integer-only |
| `DenState` | enum | Unit, Known, Unknown |
| `DivOut` | enum | ExactInverse, ExactAFC, FPD (T8 division trichotomy) |
| `ArithmeticError` | enum | DivideByZero, Overflow, Unimplemented, InvalidDenominator |
| `NormalizationScheduler` | struct | EMA-based adaptive normalization (integer-only) |
| `binary_gcd()` | fn | Stein's binary GCD algorithm |
| `should_normalize()` | fn | Bit-threshold check (80% of i128 capacity) |
| `normalize()` | fn | GCD reduction of NexGenRat |
| `divide_coeff()` | fn | Division trichotomy implementation |
| `divide_coeff_checked()` | fn | Overflow-safe division |
| `add/sub/mul/div/neg` | fn | Rational arithmetic with I4 overflow protection |

### 3.4 nine65

**errors.rs**:

| Construct | Kind | Purpose |
|-----------|------|---------|
| `Nine65Error` | enum | 21 error variants mapped to Coq preconditions |
| `Nine65Result<T>` | type | `Result<T, Nine65Error>` |

**keys/mod.rs**:

| Construct | Kind | Purpose |
|-----------|------|---------|
| `SecretKey` | struct | Ternary polynomial with Zeroize + ZeroizeOnDrop |
| `PublicKey` | struct | (pk0, pk1) with serde + validation |
| `EvaluationKey` | struct | Relinearization keys with decomposition levels |
| `KeySet` | struct | Complete key triple: SK + PK + EK |

**params/mod.rs**:

| Construct | Kind | Purpose |
|-----------|------|---------|
| `FHEConfig` | struct | Parameter set: n, primes, q, t, eta, security_bits, name |
| `SecureConfig` | struct | Production-safe configs with verified security levels |

15+ preset configurations: `light`, `light_mul`, `light_rns`, `light_exact`, `light_rns_exact`, `standard_128`, `high_192`, `deep_128`, `batched`, `he_standard_128`, `he_standard_128_deep`, `large_single`, `depth2_128`, `depth3_128`, `deep_circuit`, `for_depth()`

**ops/encrypt.rs** (key types from prelude):

| Construct | Kind | Purpose |
|-----------|------|---------|
| `BFVEncoder` | struct | Plaintext encoding/decoding |
| `BFVEncryptor` | struct | Encryption with public key |
| `BFVDecryptor` | struct | Decryption with secret key |
| `BFVEvaluator` | struct | Homomorphic operations |
| `Ciphertext` | struct | Encrypted value (c0, c1) |

**kat.rs**:

| Construct | Kind | Purpose |
|-----------|------|---------|
| `KATVector` | struct | Known Answer Test vector |
| `KATOperation` | enum | EncryptDecrypt, HomomorphicAdd, MulPlain, CtCtAdd |
| `KATResult` | struct | Test result with hash |

**compiler.rs**:

| Construct | Kind | Purpose |
|-----------|------|---------|
| `NoiseModel` | struct | f64-based noise estimation (offline only) |
| `CompilationResult` | struct | Circuit compilation output with noise tracking |

### 3.5 nine65-python

| Construct | Kind | Purpose |
|-----------|------|---------|
| `PyFHEConfig` | pyclass | Python wrapper for FHEConfig |
| `PySecureConfig` | pyclass | Python wrapper for SecureConfig |
| `PyFHEContext` | pyclass | Context with NTT + encoder + encrypt/decrypt/eval |
| `PyKeySet` | pyclass | Key set with getters |
| `PyPublicKey` | pyclass | With to_bytes/from_bytes (bincode) |
| `PySecretKey` | pyclass | With __repr__ only |
| `PyEvaluationKey` | pyclass | With to_bytes/from_bytes (bincode) |
| `PyCiphertext` | pyclass | With to_bytes/from_bytes (bincode) |
| `clone_config()` | fn | Manual FHEConfig cloning helper |
| `nine65_err()` | fn | Nine65Error -> PyErr conversion |

### 3.6 nine65-wasm

| Construct | Kind | Purpose |
|-----------|------|---------|
| `WasmFHEContext` | wasm_bindgen | WASM context: keygen, encrypt, decrypt, eval |
| `WasmKeySet` | wasm_bindgen | Key set accessor |
| `WasmPublicKey` | wasm_bindgen | With to_bytes/from_bytes |
| `WasmSecretKey` | wasm_bindgen | to_bytes DISABLED ("SecretKey export disabled") |
| `WasmEvaluationKey` | wasm_bindgen | With to_bytes/from_bytes |

### 3.7 unhal

| Construct | Kind | Purpose |
|-----------|------|---------|
| `Accelerator` | struct | Dispatches ops based on ExecutionMode |
| `ExecutionMode` | enum | Sequential, Simd, Parallel, Full |
| `Pipeline` | struct | Staged compute pipeline |
| `PipelineBuilder` | struct | Builder pattern for Pipeline |
| `Stage` | enum | Add, Sub, Negate, ScalarMul, Custom |
| `BatchProcessor` | struct | Batch operations with tree reduction |
| `pipeline!` | macro | Ergonomic pipeline construction |

---

## 4. Wiring Verification

### 4.1 Inter-Crate Dependency Usage

| Dependency | Used By | Feature Gate | Actually Used? |
|------------|---------|-------------|----------------|
| `mana` | nine65 (accelerated.rs) | `accelerated` | YES -- AcceleratedFHE wraps ManaStream |
| `unhal` | nine65 (accelerated.rs) | `accelerated` | YES -- AcceleratedRNS uses Accelerator |
| `clockwork-core` | nine65 (security/, arithmetic/bounded_rns.rs) | `clockwork` | YES -- gro_gate, key_manager, integrity, bounded_rns |
| `nexgen_rational` | nine65 (arithmetic/rational_bridge.rs, params/exact_params.rs) | `exact_rational` | YES -- rational_bridge, ExactDelta |
| `nine65` | nine65-python | always | YES -- all types used |
| `nine65` | nine65-wasm | always | YES -- all types used |
| `mana` | unhal | always | YES -- re-exports ManaStream, Lane, KAnchor |

**Verdict**: All declared dependencies are actively used. No phantom dependencies.

### 4.2 Feature Gate Verification

| Feature | Gate Expression | Files Protected | Status |
|---------|----------------|-----------------|--------|
| `accelerated` | `#[cfg(feature = "accelerated")]` | accelerated.rs, lib.rs module decl | CORRECT |
| `clockwork` | `#[cfg(feature = "clockwork")]` | security/gro_gate.rs, key_manager.rs, integrity.rs, arithmetic/bounded_rns.rs | CORRECT |
| `exact_rational` | `#[cfg(feature = "exact_rational")]` | arithmetic/rational_bridge.rs, params/exact_params.rs | CORRECT |
| `ntt_fft` | `#[cfg(feature = "ntt_fft")]` | NTTEngineFFT vs NTTEngine alias | CORRECT |
| `serde` | `#[cfg(feature = "serde")]` | PublicKey/EvaluationKey serialization impls | CORRECT |
| `allow_insecure` | `#[cfg(any(test, feature = "allow_insecure"))]` | FHEConfig::light(), FHEConfig::he_standard_128(), unvalidated from_json/from_bytes | CORRECT |
| `shadow-entropy` | `#[cfg(feature = "shadow-entropy")]` | wassan_noise.rs, crt_shadow.rs | CORRECT |
| `parallel` (mana) | `#[cfg(feature = "parallel")]` | parallel.rs module | CORRECT |
| `wasm` (nine65-wasm) | `#[cfg(feature = "wasm")]` | Entire wasm_impl module | CORRECT |

### 4.3 Import/Export Path Connectivity

**nine65 prelude re-exports** (from lib.rs):
```rust
pub use arithmetic::{MontgomeryU64, BarrettReducer, NTTEngine/NTTEngineFFT, ...}
pub use entropy::{ShadowHarvester, ...}
pub use keys::{SecretKey, PublicKey, EvaluationKey, KeySet}
pub use ops::{BFVEncoder, BFVEncryptor, BFVDecryptor, BFVEvaluator, Ciphertext}
pub use params::{FHEConfig, SecureConfig, ...}
pub use ring::RingPolynomial
pub use noise::{NoiseBudgetTracker, ...}
pub use errors::{Nine65Error, Nine65Result}
```

All prelude exports have matching definitions in their respective modules. **Connected.**

**nine65-python imports from nine65**:
```rust
use nine65::entropy::ShadowHarvester;
use nine65::errors::Nine65Error;
use nine65::keys::{EvaluationKey, KeySet, PublicKey, SecretKey};
use nine65::ops::{BFVDecryptor, BFVEncoder, BFVEncryptor, BFVEvaluator, Ciphertext};
use nine65::params::secure_configs::SecureConfig;
use nine65::params::FHEConfig;
use nine65::prelude::NTTEngine;
```

All imports resolve to public items in nine65. **Connected.**

**nine65-wasm imports from nine65**: Same pattern as nine65-python. **Connected.**

**unhal imports from mana**:
```rust
pub use mana::{ManaStream, StreamOps, Lane, LaneOps, KAnchor, AnchorContext};
```

All imports resolve. **Connected.**

---

## 5. Dead Code Detection

### 5.1 Orphan Files at Workspace Root

| File | Size | Status |
|------|------|--------|
| `avatar.rs` | present | ORPHAN -- not referenced by any Cargo.toml or mod.rs |
| `pipeline.rs` | present | ORPHAN -- not referenced by any Cargo.toml or mod.rs |
| `random_encrypt_proptest.rs` | present | ORPHAN -- not referenced; a test duplicate exists at `nine65/tests/random_encrypt.rs` |

These files are outside `crates/` and the workspace members pattern `crates/*` excludes them from compilation.

### 5.2 SIMD Feature Stubs

**mana/Cargo.toml**: `simd` feature is commented out with note "counterproductive for modular arithmetic"
**unhal/Cargo.toml**: `simd` feature is declared but empty (`simd = []`)
**unhal/accelerator.rs**: SIMD execution paths exist in `ExecutionMode::Simd` and `ExecutionMode::Full` but fall back to sequential with comment "SIMD module removed, fallback to sequential"

**Verdict**: SIMD infrastructure is dead code. The enum variants exist, the cfg-gated dispatch paths exist, but all SIMD implementations were removed and replaced with sequential fallbacks.

### 5.3 Deprecated Configurations

The following FHEConfig methods are deprecated and feature-gated to prevent production use:
- `FHEConfig::light()` -- 36-bit security
- `FHEConfig::he_standard_128()` -- 56-bit security
- `FHEConfig::light_rns_exact()` -- 80-bit security

These are retained for backward compatibility in testing but marked `#[deprecated]` and gated behind `#[cfg(any(test, feature = "allow_insecure"))]`.

### 5.4 Unvalidated Deserialization (Deprecated)

`PublicKey::from_json()` and `PublicKey::from_bytes()` (unvalidated) are deprecated and gated behind `#[cfg(all(feature = "serde", feature = "allow_insecure"))]`. Same for `EvaluationKey`.

### 5.5 Potential Dead Submodules

- `Pipeline::Custom` stage in unhal is a no-op pass-through
- `nine65::ops::gso_fhe` -- GSO-optimized FHE operations; usage depends on feature combination
- `nine65::arithmetic::cyclotomic_phase` -- specialized module, unclear if used outside tests

### 5.6 Fuzz Targets

Five fuzz targets exist under `fuzz/fuzz_targets/` but are excluded from the workspace (`exclude = ["fuzz"]`). They are not dead code in the traditional sense, but they are not part of any build.

---

## 6. Cross-Reference Check

### 6.1 Import/Export Alignment

**Checked cross-crate boundaries**:

| Source Crate | Export | Consumer | Import | Aligned? |
|-------------|--------|----------|--------|----------|
| nine65::keys::SecretKey | pub struct | nine65-python | `use nine65::keys::SecretKey` | YES |
| nine65::keys::PublicKey | pub struct | nine65-python, nine65-wasm | `use nine65::keys::PublicKey` | YES |
| nine65::keys::EvaluationKey | pub struct | nine65-python, nine65-wasm | `use nine65::keys::EvaluationKey` | YES |
| nine65::keys::KeySet | pub struct | nine65-python, nine65-wasm | `use nine65::keys::KeySet` | YES |
| nine65::ops::Ciphertext | pub struct | nine65-python, nine65-wasm | `use nine65::ops::Ciphertext` | YES |
| nine65::ops::BFVEncoder | pub struct | nine65-python, nine65-wasm | `use nine65::ops::...` | YES |
| nine65::ops::BFVEncryptor | pub struct | nine65-python, nine65-wasm | `use nine65::ops::...` | YES |
| nine65::ops::BFVDecryptor | pub struct | nine65-python, nine65-wasm | `use nine65::ops::...` | YES |
| nine65::ops::BFVEvaluator | pub struct | nine65-python | `use nine65::ops::BFVEvaluator` | YES |
| nine65::params::FHEConfig | pub struct | nine65-python, nine65-wasm | `use nine65::params::FHEConfig` | YES |
| nine65::params::SecureConfig | pub struct | nine65-python | `use nine65::params::secure_configs::SecureConfig` | YES |
| nine65::entropy::ShadowHarvester | pub struct | nine65-python | `use nine65::entropy::ShadowHarvester` | YES |
| nine65::prelude::NTTEngine | pub type/struct | nine65-python, nine65-wasm | `use nine65::prelude::NTTEngine` | YES |
| mana::ManaStream | pub struct | unhal | `pub use mana::ManaStream` | YES |
| mana::Lane | pub struct | unhal | `pub use mana::Lane` | YES |
| mana::KAnchor | pub struct | unhal | `pub use mana::KAnchor` | YES |

### 6.2 Duplicate Helper Functions

The following helper functions are duplicated across crates:

| Function | Locations | Risk |
|----------|-----------|------|
| `clone_config()` | nine65-python/lib.rs, nine65-wasm/lib.rs | LOW -- manual FHEConfig cloning, identical implementations |
| `extended_gcd` / `extended_gcd_i128` | clockwork-core/basis.rs, mana/stream.rs, mana/anchor.rs | MEDIUM -- three independent implementations of extended Euclidean algorithm |
| `mod_inverse` / `mod_inverse_u128` | clockwork-core/basis.rs, mana/stream.rs, mana/anchor.rs | MEDIUM -- three implementations, potential for divergence |
| `mul_mod_u128` | clockwork-core/basis.rs, mana/anchor.rs | MEDIUM -- shift-and-add overflow-safe multiplication |
| `mod_add` | mana/lane.rs, mana/parallel.rs | LOW -- same branchless modular addition |

**Recommendation** (informational only): The extended_gcd / mod_inverse duplication across clockwork-core and mana represents three independent implementations of critical math functions. If these ever diverge, it could cause subtle correctness issues.

### 6.3 FHEConfig Cloning Pattern

Both nine65-python and nine65-wasm define a `clone_config()` function that manually clones every field of `FHEConfig`. This is necessary because `FHEConfig` does not derive `Clone` universally (it has `#[derive(Clone, Debug)]` but the wrappers need to cross FFI boundaries). The manual cloning is fragile: if a field is added to `FHEConfig`, both `clone_config()` functions must be updated or they will fail to compile (which is actually a safety net).

### 6.4 NTT Engine Aliasing

The NTT engine aliasing pattern appears consistently across the codebase:

```rust
#[cfg(feature = "ntt_fft")]
use crate::arithmetic::NTTEngineFFT as NTTEngine;

#[cfg(not(feature = "ntt_fft"))]
use crate::arithmetic::NTTEngine;
```

This appears in: `keys/mod.rs`, `kat.rs`, `lib.rs` (prelude), and likely in ops files. The pattern is consistent and correct.

---

## 7. Anomaly Catalogue

### 7.1 Float Violations

| File | Location | Type | Severity | Notes |
|------|----------|------|----------|-------|
| `nine65/src/compiler.rs` | Entire file | f64 usage | DOCUMENTED EXCEPTION | "compiler.rs uses f64 only for offline/static noise analysis". Has `#![allow(clippy::float_arithmetic)]`. NoiseModel uses f64 fields. Not on any cryptographic runtime path. |
| `clockwork-core/src/lib.rs` | Lines 22-23 | COMMENTED-OUT LINT | INFO | `// #![deny(clippy::float_arithmetic)]` and `// #![deny(clippy::float_cmp)]` -- lints are disabled rather than enforced. No actual float usage found in clockwork-core. |

**Verdict**: Float usage is confined to the compiler module for offline analysis. No float operations exist on any cryptographic runtime path. The commented-out lints in clockwork-core are not a violation since no floats are present, but they should ideally be uncommented.

### 7.2 Unsafe Code

| File | Location | Purpose | Severity |
|------|----------|---------|----------|
| `clockwork-core/src/key_lifecycle.rs:274` | `zero_u64()` | `core::ptr::write_volatile` for secure memory zeroing | JUSTIFIED |

**Analysis**: The sole `unsafe` block in the entire workspace. Used for volatile memory writes that prevent compiler optimization from eliding secret key zeroing. This is standard practice in cryptographic code. The function is small, focused, and well-documented.

**Crate-level unsafe forbiddance**:
- `nine65`: `#![forbid(unsafe_code)]` -- ENFORCED
- `mana`: `#![forbid(unsafe_code)]` -- ENFORCED
- `nexgen_rational`: `#![forbid(unsafe_code)]` -- ENFORCED
- `clockwork-core`: NO forbid (required for write_volatile)
- `nine65-python`: NO forbid (PyO3 generates unsafe internally)
- `nine65-wasm`: NO forbid (wasm-bindgen generates unsafe internally)
- `unhal`: NO explicit forbid found

**Note**: nine65's `#![forbid(unsafe_code)]` does NOT conflict with clockwork-core's unsafe, because clockwork-core is a separate crate. Rust's `forbid(unsafe_code)` only applies within the declaring crate, not transitively to dependencies.

### 7.3 Unwrap / Expect / Panic in Production Paths

| File | Line(s) | Pattern | Severity | Context |
|------|---------|---------|----------|---------|
| `mana/src/anchor.rs` | ~49 | `.unwrap()` | MEDIUM | `mod_inverse_u128` result in KAnchor constructor |
| `mana/src/anchor.rs` | ~203 | `.unwrap()` | MEDIUM | `mod_inverse_u128` in extract_k |
| `nine65/src/params/primes.rs` | ~191 | `panic!("No primitive root found")` | MEDIUM | find_primitive_root; should be impossible for valid primes but panics instead of returning Result |
| `nine65/src/params/validation.rs` | ~210 | `panic!` | LOW | assert_params_valid; intentional panic for invalid configs at setup time |
| `nexgen_rational/src/rat_ng/types.rs` | 100 | `assert!(den.0 != 0)` | LOW | NexGenRat::new panics on zero denominator; try_new exists as fallible alternative |

**Verdict**: The `unwrap()` calls in anchor.rs are on modular inverse computations where the inputs should always be coprime (by construction). However, if a caller passes non-coprime values, these will panic rather than returning an error. The `panic!` in primes.rs is a defense-in-depth that should theoretically never trigger but represents a hard crash path.

### 7.4 Constant-Time Claims

| File | Function | Claim | Actual | Severity |
|------|----------|-------|--------|----------|
| `clockwork-core/src/garner.rs` | `k_eliminate_ct()` | "constant-time" | Uses standard `%` operator, not `subtle` crate | HIGH |
| `clockwork-core/src/garner.rs` | `garner_decompose_ct()` | "constant-time" | Uses standard modular arithmetic | HIGH |

**Analysis**: These functions claim to be constant-time (the `_ct` suffix) but use standard Rust arithmetic operators (`%`, `/`) which are NOT constant-time on most architectures. Division and modulo operations have data-dependent timing on x86/ARM. True constant-time implementations should use the `subtle` crate (which is already a dependency of clockwork-core) for conditional operations, and avoid division entirely. The `subtle` crate is imported but not used in these functions.

### 7.5 Hardcoded Seeds

| File | Location | Value | Severity |
|------|----------|-------|----------|
| `nine65-python/src/lib.rs` | `batch_encrypt()` line 403 | `ShadowHarvester::with_seed(42)` | MEDIUM |

**Analysis**: The `batch_encrypt` function in the Python bindings uses a hardcoded seed of 42 for its ShadowHarvester. This means all batch encryptions through the Python API use deterministic (reproducible) randomness, which is insecure for production use. Individual `encrypt()` and `encrypt_seeded()` functions properly use secure or user-provided entropy. The batch function should either accept a seed parameter or use `ShadowHarvester::from_os_seed()`.

### 7.6 WASM SecretKey Export Disabled

| File | Location | Behavior |
|------|----------|----------|
| `nine65-wasm/src/lib.rs` | `WasmSecretKey::to_bytes()` line 201 | Returns `Err("SecretKey export disabled")` |

**Analysis**: This is a deliberate security measure. Secret keys cannot be exported from the WASM context, preventing accidental key leakage in browser environments. However, this is inconsistent with the Python bindings where `PySecretKey` only has `__repr__` (no `to_bytes`/`from_bytes`) -- the Python side achieves the same goal by omission rather than by explicit error.

### 7.7 Missing Documentation Enforcement

| Crate | `#![deny(missing_docs)]` | Status |
|-------|--------------------------|--------|
| mana | YES | ENFORCED |
| clockwork-core | NO | Not enforced |
| nexgen_rational | NO | Not enforced |
| nine65 | NO | Not enforced |
| nine65-python | NO | Not enforced |
| nine65-wasm | NO | Not enforced |
| unhal | NO | Not enforced |

### 7.8 TODO/FIXME/HACK/STUB

| Pattern | Occurrences | Details |
|---------|-------------|---------|
| TODO | 0 | None found |
| FIXME | 0 | None found |
| HACK | 0 | None found |
| STUB | 1 | `nexgen_rational/src/rat_ng/normalize.rs` line 7 -- references the OLD stub in a doc comment: `pub fn should_normalize(_rat: &NexGenRat) -> bool { true } // STUB`. The stub itself has been replaced with a proper implementation. The doc comment referencing it is informational only. |

### 7.9 Pending Formal Proofs

Several theorem references in nexgen_rational are marked "pending":
- T3 (Addition correctness): pending proof
- T4 (Multiplication correctness): pending proof
- T5 (Division correctness): pending proof
- T6 (Normalization value-preservation): pending proof
- T7 (Normalization idempotence): pending proof
- T8 (Division trichotomy): VERIFIED by Kappa Critic

### 7.10 Security Configuration Discrepancies

| Config | Claimed Security | Actual Security | Status |
|--------|-----------------|-----------------|--------|
| `light()` | (formerly 80-bit) | 36-bit | CORRECTED in code, deprecated, feature-gated |
| `he_standard_128()` | (formerly 128-bit) | 56-bit | CORRECTED in code, deprecated, feature-gated |
| `standard_128()` | (formerly 128-bit) | 96-bit | CORRECTED, documented "MARGINAL" |
| `high_192()` | (formerly 192-bit) | 176-bit | CORRECTED, documented "SECURE" |
| `SecureConfig::secure_128()` | 128-bit | 128-bit | VERIFIED PRODUCTION |
| `SecureConfig::secure_192()` | 192-bit | 192-bit | VERIFIED PRODUCTION |
| `SecureConfig::secure_256()` | 256-bit | 256-bit | VERIFIED PRODUCTION |

**Verdict**: Security levels have been audited and corrected. Legacy configs with inflated security claims are deprecated and gated. The `SecureConfig` family provides verified production-safe parameters.

### 7.11 Evaluation Key Security Note

From `keys/mod.rs` line 234:
```rust
/// Generate evaluation key using OS-seeded Shadow Entropy
///
/// Eval keys are less sensitive than secret keys, so we use
/// OS-seeded Shadow Entropy for speed while maintaining security.
pub fn generate_secure(...) -> Self {
    let mut harvester = ShadowHarvester::from_os_seed();
    Self::generate(sk, config, ntt, &mut harvester)
}
```

The evaluation key's `generate_secure()` uses `ShadowHarvester::from_os_seed()` (deterministic PRNG seeded from OS entropy) rather than direct OS CSPRNG like the secret and public keys. This is a deliberate performance optimization. The security justification is stated in the doc comment. While evaluation keys contain `s^2` information, the randomness quality from an OS-seeded Shadow harvester should be adequate for the `a_i` and `e_i` polynomials.

### 7.12 Binary GCD Overflow Risk

From `nexgen_rational/src/binary_gcd.rs` line 87: The final result `u << shift` could overflow if `log2(u) + shift > 127`. The code uses `i128` which has 127 usable bits. For inputs near `i128::MAX`, the shift operation could silently produce incorrect results. This is noted in comments but not guarded with checked arithmetic.

---

## Summary of Findings

### Strengths

1. **Clean architecture**: 7 crates with clear separation of concerns and well-defined inter-crate boundaries
2. **Feature gating**: All optional dependencies are properly gated; no phantom features
3. **Integer-only runtime**: Zero floating-point on cryptographic paths (compiler.rs is documented offline exception)
4. **Security hardening**: Deprecated insecure configs, SecureConfig for production, Zeroize/ZeroizeOnDrop on secret keys, key validation on deserialization, WasmSecretKey export disabled
5. **Comprehensive error taxonomy**: 21 error variants mapped to formal Coq preconditions
6. **Testing**: Known Answer Tests, property-based tests (proptest), fuzz targets, integration tests across crates
7. **Formal spec alignment**: Clockwork-core implements Loki-Clockwork Formal Specification (D7, D22-D23, T14-T15, INV-1, INV-8)

### Issues Requiring Attention

| Severity | Count | Summary |
|----------|-------|---------|
| HIGH | 1 | Constant-time claims (`k_eliminate_ct`, `garner_decompose_ct`) use non-constant-time arithmetic |
| MEDIUM | 3 | Hardcoded seed in Python batch_encrypt; unwrap in mana/anchor.rs production paths; binary_gcd overflow risk |
| LOW | 3 | Orphan .rs files at workspace root; commented-out float lints in clockwork-core; SIMD stubs in unhal |
| INFO | 4 | Duplicate helper functions across crates; pending formal proofs in nexgen_rational; missing_docs not enforced in 6/7 crates; evaluation key uses Shadow entropy |

### File Counts

| Crate | Source Files | Test Files | Bench Files | Total |
|-------|-------------|------------|-------------|-------|
| clockwork-core | 9 | (inline) | 0 | 9 |
| mana | 6 | (inline) | 1 | 7 |
| nexgen_rational | 8 | (inline) | 0 | 8 |
| nine65 | ~45 | 5 | 4 | ~54 |
| nine65-python | 1 | 0 | 0 | 1 |
| nine65-wasm | 1 | 0 | 0 | 1 |
| unhal | 4 | (inline) | 0 | 4 |
| **Total** | **~74** | **5** | **5** | **~84** |

(Plus 3 orphan .rs files and 5 fuzz targets at workspace level)

---

*End of Forensic Audit Report*

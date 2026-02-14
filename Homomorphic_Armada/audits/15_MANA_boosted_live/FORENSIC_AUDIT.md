# FORENSIC AUDIT: Build 15_MANA_boosted_live

**Auditor**: Claude Opus 4.6 (Forensic Code Auditor)
**Date**: 2026-02-14
**Build Path**: `/home/acid/Projects/Homomorphic_Armada/builds/15_MANA_boosted_live/`
**Type**: Live MANA development. 3 crates: `mana`, `nine65`, `unhal`.
**Classification**: INSPECT, ANALYZE, REPORT -- no modifications made.

---

## TABLE OF CONTENTS

1. [Structure Mapping](#1-structure-mapping)
2. [Data Flow Tracing](#2-data-flow-tracing)
3. [Construct Identification](#3-construct-identification)
4. [Wiring Verification](#4-wiring-verification)
5. [Dead Code](#5-dead-code)
6. [Cross-Reference Alignment](#6-cross-reference-alignment)
7. [Anomaly Catalogue](#7-anomaly-catalogue)

---

## 1. STRUCTURE MAPPING

### 1.1 Workspace Layout

```
Cargo.toml (workspace root)
  resolver = "2"
  members = ["crates/*"]

Cargo.lock (version 4, 96 resolved packages)

crates/
  mana/       -- Modular Anchored Number Arithmetic
  nine65/     -- QMNF-Accelerated BFV FHE with AHOP Quantum Simulation
  unhal/      -- Universal Neuromorphic Hardware Abstraction Layer
```

### 1.2 Workspace Dependencies

| Dependency   | Version | Purpose                          |
|-------------|---------|----------------------------------|
| wide        | 0.7     | Portable SIMD (stable Rust)      |
| rayon       | 1.10    | Data parallelism                 |
| zeroize     | 1.7     | Crypto key zeroization           |
| getrandom   | 0.2     | OS CSPRNG                        |
| subtle      | 2.5     | Constant-time comparison         |
| sha2        | 0.10    | Hash for Shadow Entropy          |
| thiserror   | 1.0     | Error derivation                 |
| criterion   | 0.5     | Benchmarking (dev)               |
| proptest    | 1.4     | Property testing (dev)           |

### 1.3 Release Profile

```toml
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
```

This is a fully tuned production profile.

---

### 1.4 Crate: `mana`

**Description**: Modular Anchored Number Arithmetic -- FHE Stream Accelerator

**Features**:
- `default = ["parallel"]`
- `parallel = ["rayon"]` (enabled by default)
- `simd = ["wide"]` (disabled; comment: "counterproductive for modular arithmetic")

**Dependencies**: `wide` (optional), `rayon` (optional), `zeroize`

**Module Tree**:
```
mana/src/
  lib.rs          -- Crate root. Declares modules, prelude.
  lane.rs         -- Single CRT prime channel. Lane, LaneOps, MontgomeryLane, PersistentLane.
  stream.rs       -- Multi-lane CRT representation. ManaStream, StreamOps, CRT reconstruction.
  anchor.rs       -- K-Elimination exact division. KAnchor, AnchorContext.
  gso.rs          -- Glowworm Swarm Optimization. QbitState, QbitAgent, GsoSwarm.
  parallel.rs     -- Rayon-accelerated stream ops. ParallelStream, ParallelNTT, BatchParallel.

mana/benches/
  lane_ops.rs     -- Criterion benchmarks for Lane/Stream/Parallel operations.

mana/examples/
  benchmark.rs    -- Manual benchmark runner with format_ops helper.
```

**Lint Directives**: `#![forbid(unsafe_code)]`, `#![deny(missing_docs)]`

**Prelude Exports**:
- `Lane`, `LaneOps`
- `ManaStream`, `StreamOps`
- `KAnchor`, `AnchorContext`
- `GsoSwarm`, `QbitAgent`
- `ParallelStream` (cfg: parallel)

---

### 1.5 Crate: `unhal`

**Description**: Universal Neuromorphic Hardware Abstraction Layer -- FHE Accelerator Interface

**Features**:
- `default = ["parallel"]`
- `parallel = ["rayon", "mana/parallel"]`
- `simd = []` (declared but empty; comment: "modular arithmetic doesn't benefit from SIMD")

**Dependencies**: `mana` (path), `rayon` (optional)

**Module Tree**:
```
unhal/src/
  lib.rs          -- Crate root. Declares modules, prelude. Re-exports MANA types.
  accelerator.rs  -- ExecutionMode, AcceleratorConfig, Accelerator (dispatch hub).
  pipeline.rs     -- Stage enum, PipelineBuilder, Pipeline, pipeline! macro.
  batch.rs        -- BatchProcessor (add/sub/mul batch, reduce, broadcast, matvec).

unhal/examples/
  unhal_benchmark.rs -- Manual benchmark: sequential vs parallel across sizes.
```

**Prelude Exports**:
- `Accelerator`, `AcceleratorConfig`, `ExecutionMode`
- `Pipeline`, `PipelineBuilder`, `Stage`
- `BatchProcessor`
- Re-exports: `ManaStream`, `Lane`, `KAnchor`, `AnchorContext` (from mana)

---

### 1.6 Crate: `nine65`

**Description**: QMNF-Accelerated BFV FHE with AHOP Quantum Simulation

**Features**:
- `default = ["ntt_fft", "accelerated", "parallel"]`
- `accelerated = ["mana", "unhal"]` (enables MANA/UNHAL integration)
- `ntt_fft = []`
- `wassan = []`
- `v2 = ["ntt_fft", "wassan"]`
- `parallel = ["rayon"]`
- `secure_seed = []`
- `secure-keygen = []`
- `debug_dual_mul = []`
- `serde = ["dep:serde", "dep:serde_json", "dep:bincode"]`

**Dependencies**: `mana` (optional, path), `unhal` (optional, path), `rayon` (optional), `zeroize`, `getrandom`, `subtle`, `sha2`, `thiserror`, `serde` (optional), `serde_json` (optional), `bincode` (optional)

**Module Tree (62 source files)**:
```
nine65/src/
  lib.rs                      -- Crate root, prelude, integration tests
  errors.rs                   -- Nine65Error enum, Nine65Result type alias
  compiler.rs                 -- Bootstrap-free FHE circuit compiler (DAG-based)
  kat.rs                      -- Known Answer Tests
  accelerated.rs              -- MANA/UNHAL bridge (AcceleratedFHE, AcceleratedRNS)
  v2_integration_tests.rs     -- V2 integration test module

  arithmetic/
    mod.rs                    -- Re-exports all arithmetic types
    montgomery.rs             -- MontgomeryContext
    persistent_montgomery.rs  -- PersistentMontgomery, PersistentPolynomial
    barrett.rs                -- BarrettContext, HybridModContext
    ntt.rs                    -- NTTEngine (DFT-based, O(N^2))
    ntt_fft.rs                -- NTTEngineFFT (FFT-based, O(N log N))
    rns.rs                    -- RNSContext, RNSPolynomial, DualRNSContext, DualRNSPolynomial
    k_elimination.rs          -- KElimination (exact division)
    exact_divider.rs          -- ExactDivider
    exact_coeff.rs            -- ExactCoeff, ExactContext, ExactPoly
    ct_mul_exact.rs           -- ExactCiphertext, ExactCiphertext2, ExactFHEContext
    mobius_int.rs             -- MobiusInt, MobiusPolynomial, MobiusVector, Polarity
    pade_engine.rs            -- PadeEngine (integer exp/sin/cos/log)
    mq_relu.rs                -- MQReLU, MQReLUPolynomial, Sign
    integer_softmax.rs        -- IntegerSoftmax
    cyclotomic_phase.rs       -- CyclotomicRing, CyclotomicPolynomial
    valuation.rs              -- P-adic valuation
    order_finding.rs          -- Shor's classical reduction (BSGS)

  entropy/
    mod.rs                    -- Re-exports, secure_seed_from_os
    shadow.rs                 -- ShadowHarvester (deterministic RNG)
    secure.rs                 -- OS CSPRNG wrappers
    wassan_noise.rs           -- WassanNoiseField (holographic noise)
    crt_shadow.rs             -- CRTShadowContext, IntegratedShadowRNS

  params/
    mod.rs                    -- FHEConfig, parameter sets
    primes.rs                 -- Prime generation and modular inverse utilities
    production.rs             -- ProductionConfig128
    validation.rs             -- Parameter validation

  ring/
    mod.rs                    -- Re-exports
    polynomial.rs             -- RingPolynomial

  keys/
    mod.rs                    -- SecretKey, PublicKey, EvaluationKey, KeySet

  ops/
    mod.rs                    -- Re-exports all ops types
    encrypt.rs                -- BFVEncoder, BFVEncryptor, BFVDecryptor, Ciphertext
    homomorphic.rs            -- BFVEvaluator (add, sub, mul, negate, etc.)
    rns_mul.rs                -- RNSEvaluator (K-Elimination-based ct*ct)
    rns_fhe.rs                -- RNSFHEContext, DualRNSCiphertext, DualRNSKeySet (LARGE: 283KB)
    neural.rs                 -- FHENeuralEvaluator, DenseLayer, NeuralNetwork
    gso_fhe.rs                -- GSOFHEContext, GSOCiphertext, NoiseEstimate, GSOSwarm
    cnn/
      mod.rs                  -- Re-exports CNN types
      tensor.rs               -- Tensor4D, Tensor3D, Layout
      im2col.rs               -- im2col, Im2ColConfig
      conv2d.rs               -- Conv2DLayer, Conv2DConfig
      pooling.rs              -- PoolLayer, PoolType, MaxPool2D, AvgPool2D
      batch_norm.rs           -- BatchNormLayer

  noise/
    mod.rs                    -- NoiseBudgetTracker, NoiseSnapshot, EMACalculator, P2QuantileEstimator
    budget.rs                 -- NoiseBudget, NoiseOpType

  security/
    mod.rs                    -- LWEParams, SecurityEstimate, ConfidenceLevel

  ahop/
    mod.rs                    -- Re-exports
    grover.rs                 -- GroverSearch (basic)
    grover_full.rs            -- Full Grover with Fp2

  quantum/
    mod.rs                    -- Master module, quantum_demo, encrypted_quantum_demo
    entanglement.rs           -- EntangledPair, GHZState, BellTest
    teleport.rs               -- EntangledChannel, Alice, Bob, TeleportPacket
    amplitude.rs              -- QuantumAmplitude, QuantumState, grover_search (signed)
    coherence.rs              -- SparseGroverFp2, WassanHolographicStore
    taxonomy.rs               -- SparseKMarkedFp2, GHZStateFp2, ProductStateFp2
    encrypted.rs              -- EncryptedFp2, EncryptedSparseGrover, EncryptedQuantumContext
    dense_exact.rs            -- DenseExactGrover, DenseExactGroverWassan
    dense_rational.rs         -- ExactRational, DenseRationalGrover
    dense_toric.rs            -- DenseToricGrover (PLMG Rails)
    dense_toric_pure.rs       -- DenseToricPure (all computation on torus)
    mana_grover.rs            -- ManaCodex, ManaAmplitude, ManaGrover (full QMNF stack)

nine65/benches/
  fhe_scaling.rs              -- FHE scaling benchmarks
  nine65_vs_seal_comparison.rs -- Comparison benchmark

nine65/tests/
  pqeaq_harness.rs            -- PQEAQ test harness
  pqeaq_integration.rs        -- PQEAQ integration tests
  grover_frontier_validation.rs -- Grover frontier validation
  dense_vs_sparse_comparison.rs -- Dense vs sparse comparison
  toric_vs_rational_depth.rs  -- Toric vs rational depth test
  toric_scale_test.rs         -- Toric scale test
  cpad_attack_test.rs         -- CPAD attack resilience test

nine65/src/bin/
  fhe_demo.rs                 -- CLI FHE demo with argument parsing
```

---

## 2. DATA FLOW TRACING

### 2.1 MANA Lane/Stream Pipeline (End-to-End)

```
INPUT (user values: Vec<u64>)
  |
  v
ManaStream::from_ints(values, primes)
  |-- For each prime p_i:
  |     Lane::from_int_slice(values, p_i)
  |       -> coeffs = values.map(|v| v % p_i)
  |-- product_cache = product(all primes)
  |-- primes: Arc<Vec<u64>>
  |
  v
ManaStream { lanes: Vec<Lane>, primes: Arc<Vec<u64>>, n, product_cache }
  |
  |-- SEQUENTIAL PATH: StreamOps trait
  |     add/sub/mul/neg/scalar_mul
  |     Each dispatches to LaneOps on each lane sequentially
  |
  |-- PARALLEL PATH: ParallelStream wrapper
  |     add_par/sub_par/mul_par/scalar_mul_par/neg_par
  |     Uses rayon::par_iter over lanes (lane-level parallelism)
  |
  |-- REINFORCED PATH: add_reinforced
  |     Two-level parallelism: par_iter over lanes + par_chunks within lanes
  |
  |-- MONTGOMERY PATH: PersistentLane (from lane.rs)
  |     Coefficients stay in Montgomery form throughout chain
  |     to_mont() at input, from_mont() only at TRUE I/O
  |
  v
ManaStream::reconstruct_at(i) -> u128
  |-- Full CRT reconstruction using extended GCD
  |-- Only at output boundary
  |
  v
OUTPUT (exact integer)
```

### 2.2 K-Elimination Division Pipeline

```
INPUT: value V, divisor d
  |
  v
KAnchor { alpha_primes, beta_primes, alpha_cap, beta_cap, alpha_inv_beta }
  |-- alpha: computational codex
  |-- beta: anchor codex
  |-- alpha_inv_beta = alpha_cap^(-1) mod beta_cap (precomputed)
  |
  v
extract_k(v_alpha, v_beta):
  |-- diff = v_beta - v_alpha (mod beta_cap)
  |-- k = diff * alpha_inv_beta (mod beta_cap)
  |
  v
exact_divide(v_alpha, v_beta, divisor):
  |-- V_full = v_alpha + k * alpha_cap
  |-- result = V_full / divisor (exact)
  |
  v
OUTPUT: V/d (exact, no approximation)
```

### 2.3 UNHAL Accelerator Dispatch Pipeline

```
Accelerator::add_streams(a, b)
  |
  |-- Check ExecutionMode:
  |     Sequential -> a.add(b) (StreamOps trait)
  |     Simd       -> add_simd() (Lane::add per lane, no actual SIMD kernel)
  |     Parallel   -> add_parallel() via ParallelStream wrapper
  |     Full       -> add_simd_parallel() (same as parallel; SIMD is stub)
  |
  |-- should_parallelize(num_lanes):
  |     num_lanes >= parallel_threshold (default: 256)
  |     NOTE: This tests LANE COUNT, not coefficient count.
  |     Typical FHE has 1-8 lanes, so this threshold is almost never met.
  |
  v
Result: ManaStream
```

### 2.4 nine65 -> MANA Integration Pipeline (AcceleratedFHE)

```
nine65::accelerated::AcceleratedFHE
  |
  |-- rns_to_stream(rns_poly) -> ManaStream
  |     Each RNS limb becomes a MANA Lane
  |
  |-- add_rns_accelerated(a, b, ctx) -> RNSPolynomial
  |     rns_to_stream -> Accelerator::add_streams -> stream_to_rns
  |
  |-- exact_divide_rns(poly, divisor) -> Vec<u128>
  |     rns_to_stream -> AnchorContext::exact_divide_stream
  |
  v
Output: RNSPolynomial or Vec<u128> (exact)
```

### 2.5 GSO-FHE Noise Bounding Pipeline

```
GSOFHEContext
  |
  |-- encrypt() -> GSOCiphertext { inner: DualRNSCiphertext, noise: NoiseEstimate }
  |-- add() -> updates noise linearly
  |-- mul() -> updates noise multiplicatively
  |-- noise.needs_collapse(basin_radius) -> bool
  |-- collapse() -> resets noise distance to 0
  |
  v
NoiseEstimate tracks: distance, basin_id, mul_depth, collapse_count
```

---

## 3. CONSTRUCT IDENTIFICATION

### 3.1 MANA Crate Constructs

| Construct | Location | Type | Status |
|-----------|----------|------|--------|
| `Lane` | `lane.rs:20` | struct | Complete |
| `LaneOps` | `lane.rs:78` | trait | Complete |
| `MontgomeryLane` | `lane.rs:263` | struct | Complete |
| `PersistentLane` | `lane.rs:349` | struct | Complete |
| `ManaStream` | `stream.rs:22` | struct | Complete |
| `StreamOps` | `stream.rs:112` | trait | Complete |
| `KAnchor` | `anchor.rs:27` | struct | Complete |
| `AnchorContext` | `anchor.rs:138` | struct | Complete |
| `QbitState` | `gso.rs:28` | struct | Complete |
| `QbitAgent` | `gso.rs:131` | struct | Complete |
| `GsoSwarm` | `gso.rs:179` | struct | Complete |
| `GsoParams` | `gso.rs:190` | struct | Complete |
| `SwarmStats` | `gso.rs:316` | struct | Complete |
| `ParallelStream` | `parallel.rs:32` | struct | Complete |
| `ParallelNTT` | `parallel.rs:218` | struct | Complete |
| `BatchParallel` | `parallel.rs:262` | struct | Complete |
| `pipeline!` | `pipeline.rs` (in unhal) | macro | Complete |

### 3.2 UNHAL Crate Constructs

| Construct | Location | Type | Status |
|-----------|----------|------|--------|
| `ExecutionMode` | `accelerator.rs:26` | enum | Complete |
| `AcceleratorConfig` | `accelerator.rs:64` | struct | Complete |
| `Accelerator` | `accelerator.rs:123` | struct | Complete |
| `Stage` | `pipeline.rs:23` | enum | Complete (Custom is pass-through stub) |
| `PipelineBuilder` | `pipeline.rs:38` | struct | Complete |
| `Pipeline` | `pipeline.rs:79` | struct | Complete |
| `BatchProcessor` | `batch.rs:14` | struct | Complete |

### 3.3 nine65 Key Constructs (Selected)

| Construct | Location | Type | Status |
|-----------|----------|------|--------|
| `KElimination` | `arithmetic/k_elimination.rs:49` | struct | Complete |
| `AcceleratedFHE` | `accelerated.rs:35` | struct | Complete (cfg: accelerated) |
| `AcceleratedRNS` | `accelerated.rs:202` | struct | Complete (cfg: accelerated) |
| `DualRNSCiphertext` | `ops/rns_fhe.rs:68` | struct | Complete |
| `DualRNSPoly` | `ops/rns_fhe.rs:52` | struct | Complete |
| `RNSFHEContext` | `ops/rns_fhe.rs` | struct | Complete |
| `GSOCiphertext` | `ops/gso_fhe.rs:90` | struct | Complete |
| `NoiseEstimate` | `ops/gso_fhe.rs:32` | struct | Complete |
| `Nine65Error` | `errors.rs:20` | enum | Complete (13 variants) |
| `ManaCodex` | `quantum/mana_grover.rs:20` | struct | Complete |
| `ManaGrover` | `quantum/mana_grover.rs` | struct | Complete |

---

## 4. WIRING VERIFICATION

### 4.1 K-Elimination Wiring

| Path | Wired? | Notes |
|------|--------|-------|
| `mana::anchor::KAnchor` -> standalone use | YES | Complete with tests |
| `mana::anchor::AnchorContext` -> `ManaStream` | YES | `exact_divide_stream()` |
| `nine65::arithmetic::KElimination` -> standalone | YES | Complete with tests |
| `nine65::ops::rns_fhe` -> `KElimination` | YES | Used in DualRNS rescaling |
| `nine65::ops::rns_mul` -> K-Elimination | YES | Used in RNSEvaluator |
| `nine65::accelerated::AcceleratedFHE` -> `AnchorContext` | YES | `exact_divide_rns()` |
| `nine65::ops::gso_fhe` -> K-Elimination | YES | Noise tracking from k values |

**FINDING**: K-Elimination is wired in BOTH mana (via KAnchor/AnchorContext) AND nine65 (via arithmetic::KElimination). These are **duplicate implementations** with identical algorithms but different struct names. See Section 6.

### 4.2 MANA -> UNHAL Wiring

| Path | Wired? | Notes |
|------|--------|-------|
| `unhal::accelerator` -> `mana::stream::StreamOps` | YES | Sequential fallback |
| `unhal::accelerator` -> `mana::parallel::ParallelStream` | YES | Parallel dispatch |
| `unhal::pipeline` -> `unhal::accelerator` | YES | Pipeline uses Accelerator |
| `unhal::batch` -> `unhal::accelerator` | YES | Batch uses Accelerator |
| `unhal::prelude` -> `mana::*` | YES | Re-exports Lane, ManaStream, KAnchor, AnchorContext |

### 4.3 nine65 -> MANA/UNHAL Wiring

| Path | Wired? | Notes |
|------|--------|-------|
| `nine65::accelerated::AcceleratedFHE` -> `mana::stream::ManaStream` | YES | rns_to_stream/stream_to_rns |
| `nine65::accelerated::AcceleratedFHE` -> `unhal::accelerator::Accelerator` | YES | add/sub/mul dispatch |
| `nine65::accelerated::AcceleratedFHE` -> `mana::anchor::AnchorContext` | YES | exact_divide_rns |
| `nine65::accelerated::AcceleratedRNS` -> `mana::stream::ManaStream` | YES | RNS number ops via streams |
| `nine65::quantum::mana_grover` -> `mana::*` | NO | Self-contained Montgomery. Does NOT import from mana crate. |
| `nine65::prelude` -> MANA types | NO | Prelude does not re-export any mana types |

**FINDING**: `nine65::quantum::mana_grover` defines its own `MontgomeryContext` struct (lines 34-40) that is independent of and duplicates `mana::lane::MontgomeryLane`. Despite the module being named "MANA Grover", it does not actually use the `mana` crate at all.

### 4.4 Pipeline Stage::Custom

**Status**: STUB. `Stage::Custom(String, usize)` is declared in `unhal::pipeline::Stage` but the pipeline executor (line 102-105) has a pass-through fallback:
```rust
Stage::Custom(_, _) => {
    // Custom ops would need a registry - for now, pass through
    result
}
```
No custom operations registry exists.

### 4.5 SIMD Feature

**Status**: DECLARED BUT EMPTY across all crates.
- `mana/Cargo.toml`: `simd = ["wide"]` is commented out
- `unhal/Cargo.toml`: `simd = []` declared but empty
- `Accelerator::add_simd()` exists but is gated behind `#[cfg(feature = "simd")]` which is never enabled
- The `wide` crate IS in the workspace deps and Cargo.lock but never activated as a feature

---

## 5. DEAD CODE

### 5.1 Explicit `#[allow(dead_code)]`

| Location | Context |
|----------|---------|
| `nine65/src/arithmetic/rns.rs:838` | Field or function in RNS context |
| `nine65/src/ops/rns_fhe.rs:902` | Helper function in RNS FHE |
| `nine65/src/ops/rns_fhe.rs:1889` | Helper function in RNS FHE |
| `nine65/src/ops/rns_fhe.rs:2218` | Helper function in RNS FHE |

### 5.2 SIMD Code Paths (Dead by Feature Gate)

All code behind `#[cfg(feature = "simd")]` in `unhal::accelerator` is dead because:
- `mana` has `simd` commented out in Cargo.toml
- `unhal` has `simd = []` (no dependencies)
- The default features do not include `simd`

Affected methods:
- `Accelerator::add_simd()`
- `Accelerator::sub_simd()`
- `Accelerator::add_simd_parallel()`
- `Accelerator::sub_simd_parallel()`

### 5.3 Parallel Threshold Likely Never Met

`Accelerator::should_parallelize()` checks `num_lanes >= 256`. However:
- Typical FHE uses 1-8 CRT primes (lanes)
- The benchmark uses 8 primes at most
- This means the Accelerator will ALWAYS fall back to sequential in practice

The `ParallelStream` wrapper bypasses this threshold and is used directly in benchmarks, but the `Accelerator` dispatch will never choose the parallel path for real FHE workloads.

### 5.4 `Stage::Custom` (Dead Path)

`Stage::Custom(String, usize)` is declared but no custom operations registry exists. The executor passes through the input unchanged when it encounters a Custom stage.

### 5.5 Unused `#[allow(unused_assignments)]`

`nine65/src/arithmetic/order_finding.rs:338`: Explicitly allowed unused assignment for initial value of `giant_steps_done`.

### 5.6 `AcceleratedRNS` Construct

`AcceleratedRNS` (in `nine65::accelerated`) provides `add()` and `mul()` methods but these operate on RNS *numbers* (single elements), not polynomials. The threshold `if a.len() < 4` prevents any acceleration for small inputs. This struct appears to have no call sites in the crate -- it is a public API but never used internally.

---

## 6. CROSS-REFERENCE: mana <-> nine65 <-> unhal ALIGNMENT

### 6.1 Duplicate Implementations

#### 6.1.1 K-Elimination (CRITICAL DUPLICATION)

| Implementation | Location |
|---------------|----------|
| `mana::anchor::KAnchor` | `mana/src/anchor.rs` |
| `nine65::arithmetic::KElimination` | `nine65/src/arithmetic/k_elimination.rs` |

Both implement identical algorithms: `extract_k()`, `exact_divide()`, `exact_divide_checked()`, `scale_and_round()`. Both use the same `for_fhe()` constructor with identical prime values (`alpha=[65537, 65521, 65519]`, `beta=[4611686018427387847]`). The `nine65` version does not reference or depend on the `mana` version.

#### 6.1.2 Montgomery Context (TRIPLE DUPLICATION)

| Implementation | Location |
|---------------|----------|
| `mana::lane::MontgomeryLane` | `mana/src/lane.rs:263` |
| `nine65::arithmetic::MontgomeryContext` | `nine65/src/arithmetic/montgomery.rs:8` |
| `nine65::quantum::mana_grover::MontgomeryContext` | `nine65/src/quantum/mana_grover.rs:35` |

All three implement the same Montgomery multiplication algorithm with identical `to_mont`, `from_mont`, `mont_mul`, `mont_reduce` methods. The `mana_grover` version is particularly redundant since the module is named "MANA" but does not use the `mana` crate.

#### 6.1.3 `extended_gcd_i128` (6 COPIES)

| Location |
|----------|
| `mana/src/anchor.rs:251` |
| `mana/src/stream.rs:227` |
| `nine65/src/params/primes.rs:91` |
| `nine65/src/quantum/entanglement.rs:337` |
| `nine65/src/arithmetic/k_elimination.rs:202` |
| `nine65/src/arithmetic/order_finding.rs:77` |

Six copies of the same extended GCD algorithm across the codebase.

#### 6.1.4 `mod_inverse` (20+ COPIES)

The `mod_inverse` function exists in at least 20 different locations across both crates (see scan results in Section 7). Some return `Option<u64>`, some return `u64` (panicking), and some operate on `u128`. This is the most duplicated function in the entire codebase.

#### 6.1.5 `mul_mod_u128` (3 COPIES)

| Location |
|----------|
| `mana/src/anchor.rs:215` |
| `nine65/src/arithmetic/rns.rs:927` |
| `nine65/src/arithmetic/k_elimination.rs:215` |

All three are identical implementations of binary-method modular multiplication for u128.

### 6.2 Naming Inconsistencies

| MANA name | nine65 equivalent | Issue |
|-----------|-------------------|-------|
| `KAnchor` | `KElimination` | Different names for same concept |
| `MontgomeryLane` | `MontgomeryContext` | Different names, identical algorithm |
| `PersistentLane` | `PersistentMontgomery` | Similar concept, different scopes |
| `alpha_primes/beta_primes` | `alpha_primes/beta_primes` | Same naming (consistent) |
| `ManaStream` | `RNSPolynomial` | Parallel representations of same concept |

### 6.3 Structural Alignment

The three crates form a clean dependency DAG:
```
nine65 --[optional]--> mana
nine65 --[optional]--> unhal --> mana
```

`nine65` can compile without `mana` or `unhal` (they are optional via the `accelerated` feature). This explains the duplication: nine65 maintains its own complete arithmetic stack for standalone operation.

---

## 7. ANOMALY CATALOGUE

### 7.1 Float Violations

**Total f64/f32 occurrences found**: 295+ lines across the codebase

#### 7.1.1 Critical Path Float Usage (VIOLATIONS)

| File | Line(s) | Severity | Context |
|------|---------|----------|---------|
| `nine65/src/params/mod.rs` | 204-219 | MEDIUM | `rns_noise_budget()` returns `f64`, uses `log2()`, float arithmetic |
| `nine65/src/params/mod.rs` | 438, 504 | MEDIUM | Security ratio computed as `n as f64 / log_q as f64` |
| `nine65/src/params/production.rs` | 63, 155 | MEDIUM | `sigma: f64` field, float division |
| `nine65/src/params/validation.rs` | 157 | MEDIUM | Security ratio as float |
| `nine65/src/security/mod.rs` | 19, 47, 67, 78, 88-89, 137, 191 | HIGH | `LWEParams.sigma: f64`, `SecurityEstimate.ratio: f64`, float-based security estimation |
| `nine65/src/noise/mod.rs` | 74-80, 270-282, 424-600 | HIGH | `P2QuantileEstimator` uses extensive f64 arithmetic (P^2 algorithm) |
| `nine65/src/noise/budget.rs` | 202-208, 238 | LOW | Display-only `remaining_bits()`, `initial_bits()` |
| `nine65/src/compiler.rs` | 122-127, 142, 158-184, 241-242, 382-411 | MEDIUM | `NoiseModel` fields are f64, noise analysis uses float |
| `nine65/src/ops/gso_fhe.rs` | 145-152, 209, 435, 448 | MEDIUM | Basin geometry uses sin/cos/sqrt via f64, `NoiseStats.ratio: f64` |

The compiler file (line 8-10) explicitly allows float arithmetic with the comment:
```rust
// NOTE: Float arithmetic is allowed here for static noise analysis
// The QMNF integer-only mandate applies to runtime computation, not compiler tooling
```

#### 7.1.2 Benchmark/Display Float Usage (ACCEPTABLE per CLAUDE.md exception)

| File | Context |
|------|---------|
| `mana/examples/benchmark.rs` | `format_ops()` helper: `ops as f64 / 1_000_000_000.0` |
| `unhal/examples/unhal_benchmark.rs` | Same `format_ops()` pattern |
| `nine65/benches/nine65_vs_seal_comparison.rs` | `PI/4 * sqrt(n_states)` for optimal Grover iterations |
| `nine65/src/ops/rns_fhe.rs` (various test functions) | `eprintln!("{:.2e}", value as f64)` for debug display |
| `nine65/src/ops/gso_fhe.rs` (test functions) | Timing: `elapsed.as_secs_f64()`, ops/sec calculations |
| `nine65/src/ops/rns_mul.rs` (debug output) | `eprintln!` display only |
| `nine65/src/entropy/shadow.rs:214` | Test mean calculation |
| `nine65/src/entropy/secure.rs:179-184` | Test variance calculation |
| `nine65/src/entropy/crt_shadow.rs` | Benchmark output |
| `nine65/src/quantum/dense_exact.rs` | `target_probability()` returns f64, theoretical comparison |
| `nine65/src/quantum/teleport.rs:464` | Test benchmark output |

#### 7.1.3 Float Severity Summary

| Category | Count | Verdict |
|----------|-------|---------|
| **Critical path (security/noise)** | ~40 lines | VIOLATION of integer-only mandate |
| **Compiler/static analysis** | ~30 lines | Explicitly exempted in code comment |
| **Benchmark/display-only** | ~225 lines | Acceptable (not in computation path) |

### 7.2 `#[ignore]` Tests

| Test | Location | Reason |
|------|----------|--------|
| `test_production_128bit` | `nine65/src/lib.rs:291` | "Needs proper noise budget tracking for N=8192" |
| `test_exact_mul_with_anchor` | `nine65/src/arithmetic/ct_mul_exact.rs:458` | "Anchor track NTT needs proper primitive root selection" |
| `test_dual_rns_diagnostic_public` | `nine65/src/ops/rns_fhe.rs:2793` | "Diagnostic test for public mode" |

### 7.3 `unimplemented!` / `unreachable!`

| Location | Context |
|----------|---------|
| `nine65/src/quantum/taxonomy.rs:226` | `unimplemented!("Hadamard breaks GHZ O(1) structure - use dense representation")` |
| `nine65/src/ops/neural.rs:232` | `unreachable!()` in match arm (legitimate guard) |

### 7.4 Miscellaneous Anomalies

#### 7.4.1 Parallel Threshold Bug
`Accelerator::should_parallelize()` at `unhal/src/accelerator.rs:142` compares `num_lanes >= self.config.parallel_threshold` where `parallel_threshold` defaults to 256. This tests **number of CRT lanes** (typically 1-8), not coefficient count. The parallel path through the Accelerator is effectively dead for all practical FHE use cases.

#### 7.4.2 SIMD Feature is a Complete Stub
The `simd` feature is declared in both `mana` and `unhal` but:
- In `mana`, it is commented out (`# simd = ["wide"]`)
- In `unhal`, it maps to nothing (`simd = []`)
- All SIMD code paths in `Accelerator` are behind `#[cfg(feature = "simd")]`
- The `add_simd()` and `sub_simd()` implementations in `Accelerator` just call `Lane::add/sub` -- they do NOT use SIMD intrinsics

#### 7.4.3 `product_cache` is `u128` (Overflow Risk)
`ManaStream.product_cache` is `u128`, computed as `primes.iter().fold(1u128, |acc, &p| acc * p as u128)`. With 8 primes of ~30 bits each, this gives ~240-bit product. This overflows `u128` (max 128 bits). The benchmark PRIMES array has 8 primes averaging ~30 bits = ~240 bits total -- this WILL overflow silently.

#### 7.4.4 Recursive `extended_gcd_i128` (Stack Depth Risk)
Both `mana::stream::extended_gcd_i128` and `mana::anchor::extended_gcd_i128` use recursive implementation. For very large moduli, this could overflow the stack. The iterative version would be safer.

#### 7.4.5 `ManaStream::reconstruct_at()` uses `product_cache` which may be overflowed
CRT reconstruction depends on `self.product()` returning the correct product of all primes. If `product_cache` overflowed (see 7.4.3), reconstruction will silently produce wrong results.

#### 7.4.6 `GsoSwarm::tick()` borrow pattern
The `tick()` method pre-collects `luciferins` and `qbit_states` into separate vectors to work around Rust's borrow checker. This is correct but results in O(n) cloning of `QbitState` per iteration.

#### 7.4.7 `Pipeline::execute()` always clones `other`
`Pipeline::execute()` at `pipeline.rs:92` clones both input streams. The `other` clone is used in every stage even when stages like `Negate` and `ScalarMul` don't use it.

#### 7.4.8 `mana_grover.rs` does not use `mana` crate
Despite being named "MANA-Powered Grover Search", `nine65::quantum::mana_grover` defines its own `MontgomeryContext`, `ManaCodex`, etc. and does not import from the `mana` crate. It is a standalone implementation that conceptually mirrors MANA's architecture but shares no code.

---

## SUMMARY

### Build Health

| Metric | Status |
|--------|--------|
| Crate count | 3 (mana, nine65, unhal) |
| Source files | 74 .rs files |
| Workspace resolves | 96 packages |
| Feature gating | Correct; `accelerated` properly gates mana/unhal deps |
| Lint directives | mana: `forbid(unsafe_code)`, `deny(missing_docs)` |
| Tests | Present in all crates; 3 tests `#[ignore]`d |

### Critical Findings

| # | Finding | Severity |
|---|---------|----------|
| F1 | K-Elimination duplicated between mana and nine65 (identical algorithm, different structs) | MEDIUM |
| F2 | MontgomeryContext triplicated (mana::MontgomeryLane, nine65::MontgomeryContext, nine65::mana_grover::MontgomeryContext) | MEDIUM |
| F3 | `mod_inverse` function duplicated 20+ times across codebase | LOW |
| F4 | `extended_gcd_i128` duplicated 6 times | LOW |
| F5 | Float violations in security estimation, noise tracking, and GSO basin geometry (~40 lines in critical paths) | HIGH |
| F6 | `Accelerator::should_parallelize()` tests lane count (1-8) against threshold 256 -- parallel path never activates | HIGH |
| F7 | `product_cache: u128` silently overflows when using 8 primes of ~30 bits each | HIGH |
| F8 | SIMD feature is complete stub -- declared but never enabled, implementations don't use SIMD | LOW |
| F9 | `Stage::Custom` is declared but executor passes through unchanged (no registry) | LOW |
| F10 | `AcceleratedRNS` appears to have no internal call sites | LOW |
| F11 | `mana_grover.rs` is named "MANA" but does not use the mana crate | LOW |
| F12 | `Pipeline::execute()` clones `other` unconditionally even for stages that don't use it | LOW |

### Wiring Completeness

| Connection | Status |
|------------|--------|
| mana internal (Lane -> Stream -> Anchor -> GSO -> Parallel) | FULLY WIRED |
| unhal -> mana (Accelerator, Pipeline, Batch) | FULLY WIRED |
| nine65 -> mana/unhal (AcceleratedFHE, AcceleratedRNS) | WIRED (behind `accelerated` feature) |
| K-Elimination -> FHE rescaling (nine65 internal) | FULLY WIRED |
| K-Elimination -> MANA anchor (mana internal) | FULLY WIRED |
| K-Elimination cross-crate (mana KAnchor used by nine65) | YES (via AcceleratedFHE) |
| GSO -> FHE noise tracking | FULLY WIRED |
| CNN module | FULLY WIRED (tensor, im2col, conv2d, pooling, batch_norm) |
| Quantum module (11 sub-modules) | FULLY WIRED |

---

*End of forensic audit. No source files were modified.*

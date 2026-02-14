# FORENSIC AUDIT: Build 08_FHE_v03_MANA_boosted

**Auditor**: Claude Opus 4.6 (Forensic Code Auditor)
**Date**: 2026-02-13
**Build Path**: `/home/acid/Projects/Homomorphic_Armada/builds/08_FHE_v03_MANA_boosted/`
**Classification**: MANA Fork -- INCOMPATIBLE with BFV Ring lineage
**Verdict**: Structurally sound MANA integration layer; nine65 retains full BFV Ring model beneath; float violations concentrated in non-critical-path modules

---

## TABLE OF CONTENTS

1. [Structure Mapping](#1-structure-mapping)
2. [Data Flow Tracing](#2-data-flow-tracing)
3. [Construct Identification](#3-construct-identification)
4. [Wiring Verification](#4-wiring-verification)
5. [Dead Code Detection](#5-dead-code-detection)
6. [Cross-Reference Check](#6-cross-reference-check)
7. [Anomaly Catalogue](#7-anomaly-catalogue)
8. [BFV Ring Divergence Analysis](#8-bfv-ring-divergence-analysis)

---

## 1. STRUCTURE MAPPING

### 1.1 Workspace Layout

```
Cargo.toml (workspace)
  resolver = "2"
  members = ["crates/*"]

  workspace.dependencies:
    wide = "0.7"          (Portable SIMD - DISABLED in both mana and unhal)
    rayon = "1.10"        (Data parallelism)
    zeroize = "1.7"       (Secure memory clearing)
    getrandom = "0.2"     (OS CSPRNG)
    subtle = "2.5"        (Constant-time ops)
    sha2 = "0.10"         (Hash)
    criterion = "0.5"     (Benchmarks, dev)
    proptest = "1.4"      (Property testing, dev)

  profile.release:
    opt-level = 3, lto = "fat", codegen-units = 1, panic = "abort"
```

### 1.2 Crate: `mana` (Modular Anchored Number Arithmetic)

**Path**: `crates/mana/`
**Purpose**: FHE Stream Accelerator -- Lane/Stream CRT parallel architecture
**Lines**: ~1,848 (source) + 83 (bench) + 116 (example)

```
mana/
  Cargo.toml
    features:
      default = ["parallel"]
      parallel = ["rayon"]
      # simd = ["wide"]          <-- COMMENTED OUT, disabled
    deps: wide (optional), rayon (optional), zeroize

  src/
    lib.rs                        (36 lines)  -- Module declarations, prelude
      pub mod lane
      pub mod stream
      pub mod anchor
      pub mod gso
      #[cfg(feature = "parallel")]
      pub mod parallel

    lane.rs                       (677 lines) -- Lane, LaneOps, MontgomeryLane, PersistentLane
    stream.rs                     (289 lines) -- ManaStream, StreamOps, CRT reconstruction
    anchor.rs                     (327 lines) -- KAnchor, AnchorContext, K-Elimination division
    gso.rs                        (420 lines) -- QbitState, QbitAgent, GsoSwarm, GsoParams
    parallel.rs                   (379 lines) -- ParallelStream, ParallelNTT, BatchParallel

  benches/
    lane_ops.rs                   (83 lines)  -- Criterion benchmarks

  examples/
    benchmark.rs                  (116 lines) -- Manual benchmarks with throughput reporting
```

**Attributes**: `#![forbid(unsafe_code)]`, `#![deny(missing_docs)]`

### 1.3 Crate: `nine65` (QMNF-Accelerated BFV FHE with AHOP Quantum Simulation)

**Path**: `crates/nine65/`
**Purpose**: Complete BFV FHE implementation; this is the BFV Ring model that MANA accelerates
**Lines**: ~12,907 (source) + 118 (bench)

```
nine65/
  Cargo.toml
    features:
      default = ["ntt_fft", "accelerated", "parallel"]
      secure-keygen = []
      ntt_fft = []
      wassan = []
      v2 = ["ntt_fft", "wassan"]
      accelerated = ["mana", "unhal"]       <-- MANA/UNHAL bridge
      parallel = ["rayon"]
    deps: mana (optional), unhal (optional), rayon (optional),
          zeroize, getrandom, subtle, sha2

  src/
    lib.rs                        (393 lines) -- Module tree, prelude, integration tests
    compiler.rs                   (534 lines) -- Bootstrap-free FHE compiler (circuit DAG)
    kat.rs                        (331 lines) -- Known Answer Tests
    v2_integration_tests.rs       (190 lines) -- V2 feature integration tests
    accelerated.rs                (314 lines) -- AcceleratedFHE, AcceleratedRNS (MANA bridge)

    arithmetic/
      mod.rs                      (57 lines)  -- Re-exports
      montgomery.rs               (245 lines)
      persistent_montgomery.rs    (480 lines) -- "THE INNOVATION" - persistent Montgomery form
      barrett.rs                  (290 lines)
      ntt.rs                      (344 lines) -- DFT-based NTT (O(N^2))
      ntt_fft.rs                  (601 lines) -- FFT-based NTT (O(N log N))
      rns.rs                      (351 lines) -- RNSContext, RNSPolynomial
      k_elimination.rs            (298 lines) -- KElimination (nine65's own copy)
      exact_divider.rs            (257 lines) -- Dual-track exact division
      exact_coeff.rs              (369 lines) -- Dual-track coefficient representation
      ct_mul_exact.rs             (567 lines) -- Exact ct*ct multiplication
      mobius_int.rs               (728 lines) -- Signed arithmetic (MobiusInt)
      pade_engine.rs              (163 lines) -- Integer transcendentals
      mq_relu.rs                  (203 lines) -- O(1) sign detection
      integer_softmax.rs          (214 lines) -- Exact sum softmax
      cyclotomic_phase.rs         (200 lines) -- Native ring trig

    entropy/
      mod.rs                      (41 lines)  -- Re-exports
      shadow.rs                   (254 lines) -- ShadowHarvester (deterministic PRNG)
      secure.rs                   (206 lines) -- OS CSPRNG wrapper
      wassan_noise.rs             (403 lines) -- Holographic noise

    params/
      mod.rs                      (409 lines) -- FHEConfig, parameter sets
      primes.rs                   (226 lines) -- NTT-friendly prime generation
      production.rs               (323 lines) -- ProductionConfig128
      validation.rs               (261 lines) -- Parameter validation

    ring/
      mod.rs                      (8 lines)   -- Re-exports
      polynomial.rs               (269 lines) -- RingPolynomial (BFV core)

    keys/
      mod.rs                      (387 lines) -- SecretKey, PublicKey, EvaluationKey, KeySet

    ops/
      mod.rs                      (18 lines)  -- Re-exports
      encrypt.rs                  (303 lines) -- BFVEncoder, BFVEncryptor, BFVDecryptor
      homomorphic.rs              (934 lines) -- BFVEvaluator (add/sub/mul/negate/plain ops)
      rns_mul.rs                  (389 lines) -- RNSEvaluator (multi-prime ct*ct)
      neural.rs                   (476 lines) -- FHENeuralEvaluator

    noise/
      mod.rs                      (1049 lines) -- NoiseBudgetTracker, EMA, P2Quantile, anomaly detection
      budget.rs                   (340 lines) -- Config-aware noise budget

    security/
      mod.rs                      (358 lines) -- LWE security estimation

    ahop/
      mod.rs                      (412 lines) -- AHOP quantum simulation, Fp2Element, StateVector
      grover.rs                   (282 lines) -- GroverSearch
      grover_full.rs              (961 lines) -- Full Grover analysis with eigenvalue tracking

    quantum/
      mod.rs                      (155 lines) -- QuantumAmplitude, QuantumState
      amplitude.rs                (327 lines)
      entanglement.rs             (448 lines)
      teleport.rs                 (466 lines)
```

### 1.4 Crate: `unhal` (Universal Neuromorphic Hardware Abstraction Layer)

**Path**: `crates/unhal/`
**Purpose**: Higher-level interface over MANA; unified API abstracting SIMD + parallel
**Lines**: ~933 (source)

```
unhal/
  Cargo.toml
    features:
      default = ["parallel"]
      parallel = ["rayon", "mana/parallel"]
      # simd = ["mana/simd"]       <-- COMMENTED OUT, disabled
    deps: mana (path = "../mana"), rayon (optional)

  src/
    lib.rs                        (66 lines)  -- Module tree, prelude, MANA re-exports
    accelerator.rs                (429 lines) -- Accelerator, AcceleratorConfig, ExecutionMode
    pipeline.rs                   (195 lines) -- Pipeline, PipelineBuilder, Stage
    batch.rs                      (243 lines) -- BatchProcessor, matrix ops
```

### 1.5 Total Source Metrics

| Crate | Source Lines | Test Lines (approx) | Files |
|-------|-------------|---------------------|-------|
| mana | ~1,848 | ~350 | 5 src + 1 bench + 1 example |
| nine65 | ~12,907 | ~2,500 | 32 src + 1 bench |
| unhal | ~933 | ~200 | 3 src |
| **Total** | **~15,688** | **~3,050** | **43 source** |

---

## 2. DATA FLOW TRACING

### 2.1 MANA Pipeline: Full Path

```
INPUT (plaintext integers)
  |
  v
Lane::from_int_slice(&[u64], prime: u64)
  |  Reduces each value mod prime
  |  Creates single-prime channel
  v
ManaStream::from_ints(&[u64], &[u64] primes)
  |  Creates one Lane per prime
  |  Each Lane holds coefficients mod that prime
  |  Computes product_cache (product of all primes, u128)
  v
[Optional: AnchorContext wrapping]
  |  AnchorContext::stream(&self, &[u64])
  |  Prepends alpha primes + beta primes
  |  KAnchor holds precomputed alpha_inv_beta for O(1) division prep
  v
StreamOps::add/sub/mul/neg/scalar_mul
  |  Lane-level: coefficient-wise modular arithmetic
  |  mod_add/mod_sub use branchless patterns for LLVM vectorization
  |  mul uses u128 intermediate: (a as u128 * b as u128) % q
  v
[Optional: ParallelStream wrapping via Rayon]
  |  ParallelStream::add_par/mul_par: par_iter() over lanes
  |  add_reinforced: nested par_iter -- lanes then 1024-coeff chunks
  |  BatchParallel: par_iter over pairs of streams
  v
[Optional: PersistentLane path -- Montgomery form FOREVER]
  |  PersistentLane::from_standard -> converts ONCE to Montgomery form
  |  All ops (add/sub/mul/square) stay in Montgomery form
  |  to_standard() / to_lane() only at TRUE I/O boundary
  |  Uses Arc<MontgomeryLane> to share context without cloning
  v
ManaStream::reconstruct_at(i) -> u128
  |  Full CRT reconstruction at coefficient position i
  |  Uses extended_gcd for modular inverse
  v
OUTPUT (reconstructed integer)
```

### 2.2 K-Elimination Anchor Division Path

```
AnchorContext::exact_divide_stream(&ManaStream, divisor: u64)
  |
  For each coefficient i:
  |  1. Compute v_alpha via partial CRT over alpha lanes
  |  2. Compute v_beta via partial CRT over beta lanes
  |  3. k = (v_beta - v_alpha) * alpha_inv_beta mod beta_cap
  |  4. V_full = v_alpha + k * alpha_cap
  |  5. result = V_full / divisor  (exact, no remainder)
  v
Vec<u128> (divided coefficients)
```

### 2.3 Rayon Parallelization Analysis

Rayon is used at THREE levels:

1. **Lane-level (coefficient parallelism)**: `Lane::add_par/sub_par/mul_par` -- `par_iter()` over coefficient vectors within a single lane. Useful when N >= 4096.

2. **Stream-level (lane parallelism)**: `ParallelStream::add_par/mul_par` -- `par_iter()` over lanes (one thread per prime). This is the primary parallelism axis. With 8 primes = 8 parallel lanes.

3. **Batch-level (operation parallelism)**: `BatchParallel::add_batch/mul_batch` and `BatchProcessor` -- `par_iter()` over pairs of streams for bulk ciphertext operations.

The `add_reinforced` method combines levels 1 and 2: outer `into_par_iter()` over lanes, inner `par_chunks(1024)` over coefficients.

### 2.4 Lane -> Stream Relationship

- A `Lane` is a single CRT channel: coefficients mod one prime
- A `ManaStream` is a collection of Lanes: one per prime in the RNS basis
- The Stream IS the multi-lane CRT representation
- `ManaStream.lanes[i]` holds all N coefficients reduced modulo `primes[i]`
- Arithmetic on Streams delegates to per-Lane arithmetic (embarrassingly parallel)
- CRT reconstruction from Stream requires all lanes to recover original value

### 2.5 nine65 BFV Integration Path

```
nine65::ops::homomorphic::BFVEvaluator
  |  Uses: KElimination (nine65 internal copy) for mul rescaling
  |  Uses: NTTEngine (FFT variant when ntt_fft feature on)
  |  Uses: RingPolynomial (standard BFV Ring model)
  v
nine65::accelerated::AcceleratedFHE (feature = "accelerated")
  |  Wraps: unhal::Accelerator
  |  Wraps: mana::AnchorContext
  |  Bridge: rns_to_stream() converts RNSPolynomial -> ManaStream
  |  Bridge: stream_to_rns() converts ManaStream -> RNSPolynomial
  |  Delegates: add/sub/mul via accel.add_streams/sub_streams/mul_streams
  |  Division: exact_divide_rns() via anchor_ctx.exact_divide_stream()
  v
unhal::Accelerator
  |  Dispatches to: Sequential / Parallel / SIMD / Full based on ExecutionMode
  |  Threshold: parallel_threshold = 256 (num_lanes)
  |  add_parallel: wraps in ParallelStream, calls add_par
  v
mana::parallel::ParallelStream
  |  par_iter() over lanes for each operation
  v
mana::lane::Lane / LaneOps
  |  Branchless modular arithmetic
```

---

## 3. CONSTRUCT IDENTIFICATION

### 3.1 mana Crate Constructs

| Construct | File | Type | Purpose |
|-----------|------|------|---------|
| `Lane` | lane.rs:20 | struct | Single CRT prime channel; Vec<u64> coefficients + prime |
| `LaneOps` | lane.rs:78 | trait | add/sub/neg/mul/scalar_mul/scalar_add interface |
| `MontgomeryLane` | lane.rs:263 | struct | Montgomery context for fast modular reduction |
| `PersistentLane` | lane.rs:349 | struct | Coefficients permanently in Montgomery form |
| `ManaStream` | stream.rs:22 | struct | Multi-lane CRT representation (Vec<Lane> + Arc<Vec<u64>> primes) |
| `StreamOps` | stream.rs:112 | trait | Stream-level add/sub/neg/mul/scalar_mul |
| `KAnchor` | anchor.rs:27 | struct | K-Elimination anchor (alpha/beta codex primes) |
| `AnchorContext` | anchor.rs:138 | struct | Full anchor context with stream creation |
| `QbitState` | gso.rs:29 | struct | Quantum-inspired amplitude weights for GSO |
| `QbitAgent` | gso.rs:131 | struct | GSO agent with position (ManaStream) + qbit + luciferin |
| `GsoSwarm` | gso.rs:179 | struct | Collection of QbitAgents |
| `GsoParams` | gso.rs:190 | struct | GSO parameters (all permille integers) |
| `SwarmStats` | gso.rs:316 | struct | Swarm statistics snapshot |
| `ParallelStream` | parallel.rs:32 | struct | Rayon-wrapped ManaStream |
| `ParallelNTT` | parallel.rs:218 | struct | Parallel NTT/INTT dispatch |
| `BatchParallel` | parallel.rs:262 | struct | Batch stream pair operations |

### 3.2 unhal Crate Constructs

| Construct | File | Type | Purpose |
|-----------|------|------|---------|
| `ExecutionMode` | accelerator.rs:29 | enum | Sequential/Simd/Parallel/Full |
| `AcceleratorConfig` | accelerator.rs:67 | struct | Mode + threshold + thread count |
| `Accelerator` | accelerator.rs:127 | struct | Main acceleration dispatch |
| `Stage` | pipeline.rs:24 | enum | Add/Sub/Negate/ScalarMul/Custom |
| `PipelineBuilder` | pipeline.rs:39 | struct | Fluent pipeline construction |
| `Pipeline` | pipeline.rs:80 | struct | Compiled operation pipeline |
| `BatchProcessor` | batch.rs:14 | struct | Bulk stream operations |

### 3.3 nine65 Crate Constructs (key ones)

| Construct | File | Type | Purpose |
|-----------|------|------|---------|
| `AcceleratedFHE` | accelerated.rs:35 | struct | MANA/UNHAL bridge for FHE ops |
| `AcceleratedRNS` | accelerated.rs:202 | struct | Drop-in RNS accelerator |
| `KElimination` | arithmetic/k_elimination.rs:21 | struct | nine65's OWN K-Elimination (parallel impl to mana's KAnchor) |
| `RNSContext` | arithmetic/rns.rs:14 | struct | Multi-prime RNS management |
| `RNSPolynomial` | arithmetic/rns.rs:148 | struct | Polynomial in RNS form (Vec<Vec<u64>> limbs) |
| `RingPolynomial` | ring/polynomial.rs:18 | struct | BFV Ring polynomial (Vec<u64> coeffs + q) |
| `BFVEvaluator` | ops/homomorphic.rs:24 | struct | Homomorphic evaluator (THE Ring model) |
| `BFVEncoder` | ops/encrypt.rs | struct | Plaintext encoder |
| `BFVEncryptor` | ops/encrypt.rs | struct | Encryption engine |
| `BFVDecryptor` | ops/encrypt.rs | struct | Decryption engine |
| `Ciphertext` | ops/encrypt.rs | struct | (c0, c1) polynomial pair |
| `RNSEvaluator` | ops/rns_mul.rs:23 | struct | RNS-based ct*ct multiplication |
| `FHEConfig` | params/mod.rs:24 | struct | Parameter set holder |
| `NoiseBudgetTracker` | noise/mod.rs:163 | struct | Per-ciphertext noise accounting |
| `P2QuantileEstimator` | noise/mod.rs:422 | struct | O(1) streaming percentile |
| `BootstrapFreeFHECompiler` | compiler.rs:329 | struct | Circuit analyzer + parameter selector |
| `Circuit` | compiler.rs:39 | struct | Computational DAG |
| `FHENeuralEvaluator` | ops/neural.rs | struct | Encrypted neural network evaluator |
| `ShadowHarvester` | entropy/shadow.rs | struct | Deterministic PRNG |

### 3.4 CRT Channel Types

The system uses two parallel representations:

**mana CRT channels**:
- Type: `Lane` (coefficients as Vec<u64>, modulus as u64)
- Grouped into: `ManaStream` (Vec<Lane>)
- Feature: Branchless arithmetic, Rayon parallelism

**nine65 CRT channels**:
- Type: `RNSPolynomial` (limbs as Vec<Vec<u64>>)
- Managed by: `RNSContext` (primes + Montgomery + NTT per prime)
- Feature: NTT integration, polynomial multiplication

### 3.5 Anchor Types

**mana anchor**:
- `KAnchor`: alpha_primes, beta_primes, alpha_cap, beta_cap, alpha_inv_beta
- `AnchorContext`: wraps KAnchor + all_primes; creates ManaStreams directly
- Default FHE: alpha = [65537, 65521, 65519], beta = [4611686018427387847]

**nine65 anchor**:
- `KElimination`: alpha_primes, beta_primes, alpha_cap, beta_cap, alpha_inv_beta
- IDENTICAL algorithm to mana's KAnchor
- Same default FHE primes

---

## 4. WIRING VERIFICATION

### 4.1 Is K-Elimination Actually Connected?

**YES, at two levels**:

1. **nine65 internal**: `BFVEvaluator` (homomorphic.rs:30) instantiates `KElimination::for_fhe(encoder.q)` directly. Used in `mul()` for tensor product rescaling (scale_and_round). `RNSEvaluator` (rns_mul.rs:53) also instantiates its own `KElimination::for_fhe(config.q)`.

2. **MANA bridge**: `AcceleratedFHE` (accelerated.rs:51) creates `AnchorContext::for_fhe()` (which creates mana's `KAnchor::for_fhe()`). The method `exact_divide_rns()` (accelerated.rs:184) delegates to `anchor_ctx.exact_divide_stream()`.

**FINDING: DUAL K-ELIMINATION** -- nine65 has its OWN `KElimination` struct AND mana has `KAnchor`. They are algorithmically identical but SEPARATE codebases with duplicated helper functions (`extended_gcd_i128`, `mul_mod_u128`, `mod_inverse_u128`). Same default primes. This is a maintenance hazard.

### 4.2 Does Rayon Parallelize Lane Ops?

**YES, confirmed at all three levels**:

1. `Lane::add_par/sub_par/mul_par` (lane.rs:207-258) -- `#[cfg(feature = "parallel")]`, uses `par_iter()` on coefficient vectors.
2. `ParallelStream::add_par/sub_par/mul_par/neg_par/scalar_mul_par` (parallel.rs:79-213) -- `par_iter()` over `self.inner.lanes`.
3. `BatchParallel::add_batch/mul_batch` (parallel.rs:262-275) -- `par_iter()` over pairs.

Feature gating: `parallel` feature is ON by default in all three crates. Rayon workspace dep is `"1.10"`.

### 4.3 Is unhal Wired In?

**YES, conditionally**:

- nine65's Cargo.toml: `unhal = { path = "../unhal", optional = true }` under `[dependencies]`
- Feature gate: `accelerated = ["mana", "unhal"]`
- `accelerated` is in nine65's DEFAULT features
- `accelerated.rs` (line 26): `use unhal::accelerator::{Accelerator, AcceleratorConfig};` guarded by `#[cfg(feature = "accelerated")]`
- `lib.rs` (line 110): `#[cfg(feature = "accelerated")] pub mod accelerated;`
- unhal's prelude re-exports mana types: `ManaStream`, `Lane`, `KAnchor`, `AnchorContext`

**FINDING**: unhal is wired but serves primarily as a DISPATCH LAYER. Since the `simd` feature is disabled in both mana and unhal, `ExecutionMode::auto_detect()` will resolve to `ExecutionMode::Parallel` (not Full or Simd). The SIMD code paths in accelerator.rs reference `SimdLane` from `mana::simd` which does NOT EXIST as a module -- those paths are dead behind `#[cfg(feature = "simd")]`.

### 4.4 AcceleratedFHE Completeness

The `AcceleratedFHE::add_ciphertexts()` method (accelerated.rs:105-118) has a **NOTABLE GAP**: it does NOT use MANA streams for ciphertext addition. Instead, it falls back to simple coefficient-wise addition via `add_polynomials()`. The comment on line 107-109 explicitly acknowledges this:

```rust
// Note: This is a simplified version - full implementation would
// convert both c0 and c1 polynomials to streams and add them
```

The RNS-level acceleration (`add_rns_accelerated`, `sub_rns_accelerated`, `mul_rns_coeffwise`) IS properly wired through MANA streams.

---

## 5. DEAD CODE DETECTION

### 5.1 SIMD Code Paths (DEAD)

The `simd` feature is commented out in both mana/Cargo.toml and unhal/Cargo.toml:
```toml
# simd = ["wide"]  # Disabled: counterproductive for modular arithmetic
```

This renders the following code paths DEAD:
- `accelerator.rs`: All `#[cfg(feature = "simd")]` blocks (lines 22, 276-313, 344-355)
- `accelerator.rs`: `ExecutionMode::Simd` and `ExecutionMode::Full` variants can never be reached via `auto_detect()`
- `accelerator.rs`: `AcceleratorConfig::simd_only()` and `AcceleratorConfig::full()` are constructable but pointless
- `accelerator.rs`: References to `mana::simd::SimdLane` (line 22) -- this module does not exist in mana
- `wide` workspace dependency is present but unused

**Note**: The `wide = "0.7"` dep IS declared in workspace Cargo.toml and mana's Cargo.toml, but since the feature is disabled, it is never compiled.

### 5.2 Pipeline Custom Stage (STUB)

`pipeline.rs:102-105`:
```rust
Stage::Custom(_, _) => {
    // Custom ops would need a registry - for now, pass through
    result
}
```
`Stage::Custom(String, usize)` exists as a variant but is a no-op. No registry exists.

### 5.3 nine65 Modules With Uncertain MANA Relevance

The following nine65 modules are part of the BFV Ring model and have NO direct MANA integration. They exist independently:

- `ahop/` (all 3 files, ~1,655 lines) -- AHOP quantum simulation
- `quantum/` (all 4 files, ~1,396 lines) -- Algebraic quantum operations
- `compiler.rs` (534 lines) -- Bootstrap-free FHE compiler
- `ops/neural.rs` (476 lines) -- FHE neural network evaluator
- `arithmetic/cyclotomic_phase.rs` (200 lines) -- Ring trig
- `arithmetic/pade_engine.rs` (163 lines) -- Integer transcendentals
- `arithmetic/mq_relu.rs` (203 lines) -- Sign detection
- `arithmetic/integer_softmax.rs` (214 lines) -- Softmax

These are NOT dead code -- they are the nine65 BFV lineage that MANA sits alongside, not replaces.

### 5.4 GSO Swarm Usage

`gso.rs` defines `GsoSwarm` + `QbitAgent` + `QbitState` (420 lines total). This module is:
- Exported in mana's prelude
- Has comprehensive tests
- NOT referenced by nine65 or unhal

GSO is a parameter search optimization tool. It is available but not wired into any automated pipeline. It requires external invocation via a fitness function callback.

### 5.5 `ManaStream.product_cache` Usage

`ManaStream.product_cache: u128` is computed once in `ManaStream::new/from_ints` and returned via `product()`. Used in:
- `reconstruct_at()` (stream.rs:96)
- `ParallelStream` constructors (pass through from inner)
- Not used in any hot-path arithmetic (add/sub/mul/neg don't need it)

It is a CACHE for CRT reconstruction. Not dead, but only active during reconstruction.

---

## 6. CROSS-REFERENCE CHECK

### 6.1 Dependency Flow

```
nine65
  |-- (feature "accelerated") --> mana
  |-- (feature "accelerated") --> unhal --> mana
  |-- (feature "parallel") --> rayon
  |-- zeroize, getrandom, subtle, sha2

unhal
  |-- mana (unconditional)
  |-- (feature "parallel") --> rayon, mana/parallel

mana
  |-- (feature "parallel") --> rayon
  |-- zeroize
  |-- wide (optional, DISABLED)
```

### 6.2 nine65 -> mana Type Usage

**In `accelerated.rs`**:
- `mana::stream::ManaStream` -- stream creation and manipulation
- `mana::lane::Lane` -- lane creation from RNS limbs
- `mana::anchor::AnchorContext` -- K-Elimination division
- `std::sync::Arc` -- wrapping primes for ManaStream construction

**Type Bridge**:
```
RNSPolynomial (nine65) <-> ManaStream (mana)
  limbs[i] (Vec<u64>)  <-> lanes[i].coeffs (Vec<u64>)
```

The conversion is DIRECT: limbs map 1:1 to lane coefficients. Both represent the same mathematical object (polynomial coefficients mod a prime).

### 6.3 unhal -> mana Type Usage

- `mana::stream::{ManaStream, StreamOps}` -- core types
- `mana::parallel::ParallelStream` -- parallel dispatch
- `mana::lane::{Lane, LaneOps}` -- (via prelude re-export only)
- `mana::anchor::{KAnchor, AnchorContext}` -- (via prelude re-export only)

unhal NEVER directly constructs lanes or anchors. It only wraps ManaStream operations.

### 6.4 Duplicated Code Between Crates

| Function | mana Location | nine65 Location | Identical? |
|----------|--------------|-----------------|------------|
| `extended_gcd_i128` | stream.rs:227, anchor.rs:251 | k_elimination.rs:173 | YES |
| `mod_inverse_u128` | anchor.rs:237 | k_elimination.rs:155 | YES (mana returns Option, nine65 returns Option) |
| `mul_mod_u128` | anchor.rs:215 | k_elimination.rs:186 | YES |
| `mod_add` (branchless) | lane.rs:101, parallel.rs:72 | -- | Internal dup in mana |
| K-Elimination algorithm | anchor.rs (KAnchor) | k_elimination.rs (KElimination) | YES -- same math, same default primes |
| CRT reconstruction | stream.rs:95-108 | rns.rs:87-99 | SAME algorithm, different types |

**Verdict**: Significant code duplication between mana's KAnchor and nine65's KElimination. The `extended_gcd_i128` function appears THREE times across the workspace. `mod_add` appears twice within mana itself (lane.rs and parallel.rs).

---

## 7. ANOMALY CATALOGUE

### 7.1 Float Violations

**CRITICAL PATH**: The `mana` crate has ZERO float violations. All arithmetic is exact integer.

**nine65 non-critical display/analysis functions** (f64 for human-readable output only):
- `noise/mod.rs`: `noise_bits()`, `budget_bits()`, `budget_remaining_bits()`, `value_bits()`, `mean_bits()`, `median_bits()`, `p90_bits()`, `p95_bits()`, `p99_bits()` -- ALL are display-only conversions of millibits to bits
- `noise/budget.rs`: `remaining_bits()`, `initial_bits()`, summary formatting -- display only
- `params/mod.rs` (lines 316, 371): `ratio = n as f64 / log_q as f64` -- security estimation heuristic
- `params/production.rs` (line 155): `(n as f64 / 37.5) as usize` -- parameter derivation
- `params/validation.rs` (line 153): `ratio = (n as f64) / (log_q as f64)` -- validation check
- `security/mod.rs`: Entire module uses f64 for security estimation (sigma, ratio, raw_estimate)

**nine65 P2QuantileEstimator** (noise/mod.rs:422-612): Uses f64 INTERNALLY for the P-squared algorithm's marker position tracking (`n_prime`, `dn`, parabolic/linear interpolation). This is a KNOWN compromise -- the P-squared algorithm inherently requires floating-point for its interpolation formulas. Data (noise measurements) is integer; only the statistical tracking mechanism uses floats.

**nine65 compiler.rs**: `NoiseModel` uses f64 throughout (lines 118-213). The entire `BootstrapFreeFHECompiler` operates in f64 for noise analysis. This module declares `#![deny(clippy::float_arithmetic)]` on line 8 but then uses f64 extensively -- this lint deny is INERT because the module uses `f64` operations directly, not through float literal arithmetic that clippy would catch in this context.

**nine65 ahop/ and quantum/**: Heavy f64 usage for probability calculations, phase angles, and quantum simulation analysis. This is expected -- quantum simulation inherently deals with continuous probabilities.

**Benchmark/example files**: mana/examples/benchmark.rs uses f64 for throughput formatting (ops/sec display). Not a violation -- this is display code.

**TOTAL FLOAT VIOLATION COUNT**:
- mana crate: 0 violations
- unhal crate: 0 violations
- nine65 critical path (encrypt/decrypt/homomorphic ops): 0 violations
- nine65 analysis/display/security: ~90+ occurrences across 12 files

### 7.2 TODO/FIXME/HACK

**NONE FOUND**. Zero occurrences of TODO, FIXME, HACK, XXX, INCOMPLETE, `unimplemented!()`, or `todo!()` across the entire workspace.

### 7.3 `#[allow(dead_code)]` / `#[allow(unused)]`

**NONE FOUND**. No suppression attributes exist in the codebase.

### 7.4 Incomplete Pipeline Stages

1. **AcceleratedFHE::add_ciphertexts()**: Falls back to non-MANA coefficient-wise addition (acknowledged in comments). The acceleration path works for RNS-level operations but not for raw Ciphertext addition.

2. **Pipeline::Custom stage**: No-op pass-through. No custom operation registry.

3. **SIMD paths**: Entire SIMD feature tree is disabled. `ExecutionMode::Simd` and `ExecutionMode::Full` exist as enum variants but `auto_detect()` cannot produce them. `add_simd`, `sub_simd`, `add_simd_parallel`, `sub_simd_parallel` methods exist but are dead behind `#[cfg(feature = "simd")]`.

4. **AcceleratedFHE ciphertext operations**: Only `add_ciphertexts` is implemented. No `sub_ciphertexts` or `mul_ciphertexts` accelerated path exists. Multiplication still goes through nine65's own `BFVEvaluator::mul()`.

### 7.5 Potential Overflow Concern

`ManaStream.product_cache` is `u128`. With 8 primes of ~30 bits each, the product is ~240 bits, which OVERFLOWS u128 (max 128 bits). For the default 3-prime setups (17*19*23 = 7429, or 65537*65521*65519 ~ 2^48), this is fine. For the benchmark's 8-prime setup in `lane_ops.rs` (8 primes of ~30 bits), the product would be ~2^240 -- this WILL overflow u128 and produce an incorrect product_cache.

However, `product_cache` is only used in `reconstruct_at()`, and reconstruction with 8 30-bit primes requires values < product of all primes. If reconstruction is never called with the 8-prime benchmark config, this is benign. But it is a latent bug.

### 7.6 `compiler.rs` `#![deny(clippy::float_arithmetic)]` Contradiction

Line 8: `#![deny(clippy::float_arithmetic)]`

Yet the entire file uses f64 arithmetic extensively (NoiseModel, NoiseAnalyzer, ParameterSelector, estimate_speedup). This attribute SHOULD cause compilation failure under `cargo clippy`. Either:
- clippy is not being run on this file, or
- The specific f64 operations used don't trigger this lint (possible if using method calls rather than operators)

This needs investigation -- it may indicate the compiler module is not being clippy-checked.

---

## 8. BFV RING DIVERGENCE ANALYSIS

### 8.1 What This Fork Actually Is

Build 08_FHE_v03_MANA_boosted is NOT a replacement of the BFV Ring model. It is an ACCELERATION LAYER that sits beside it. The architecture is:

```
                    +-----------+
                    | nine65    |  <-- BFV Ring model (UNCHANGED)
                    | BFVEval   |
                    | RingPoly  |
                    | NTT       |
                    +-----+-----+
                          |
                    +-----+-----+
                    | accelerated|  <-- BRIDGE MODULE
                    | .rs        |
                    +-----+-----+
                          |
              +-----------+-----------+
              |                       |
        +-----+-----+          +-----+-----+
        |   unhal   |          |   mana    |
        | Accelerate|          | Lane      |
        | Pipeline  |          | Stream    |
        | Batch     |          | Anchor    |
        +-----------+          | GSO       |
                               | Parallel  |
                               +-----------+
```

### 8.2 Where It Diverges from BFV Ring

| Aspect | BFV Ring Model (nine65 core) | MANA Model |
|--------|------------------------------|------------|
| **Data representation** | `RingPolynomial` (Vec<u64> + q) | `Lane` (Vec<u64> + prime), `ManaStream` (Vec<Lane>) |
| **Polynomial multiplication** | NTT-based convolution in R_q = Z_q[X]/(X^N+1) | Coefficient-wise (Hadamard) product only -- NO polynomial multiplication |
| **Ring structure** | Operates in quotient ring R_q | Operates in Z_{q1} x Z_{q2} x ... (product of fields) |
| **Division** | Scale-and-round via KElimination | K-Elimination via KAnchor (same algorithm) |
| **Noise model** | BFV noise budget tracking | No noise model -- MANA is noise-agnostic |
| **Encryption** | BFV scheme (secret key + public key + error) | No encryption -- MANA is arithmetic-only |

### 8.3 Why They Are INCOMPATIBLE

The fundamental incompatibility is:

1. **BFV Ring** does polynomial multiplication (negacyclic convolution via NTT). This requires the ring structure X^N+1.

2. **MANA Stream** does coefficient-wise multiplication (Hadamard product). This treats coefficients as independent values.

These are DIFFERENT mathematical operations. `ManaStream::mul()` does NOT compute polynomial multiplication. It computes point-wise multiplication of coefficient vectors.

The bridge (`AcceleratedFHE`) works by operating at the RNS level, where each RNS limb's coefficients CAN be operated on independently for addition and subtraction. But for multiplication, the NTT must be applied first (converting to point-value representation), THEN point-wise multiplication works, THEN INTT converts back. MANA can parallelize the point-wise step but CANNOT replace the NTT.

### 8.4 What MANA Actually Accelerates

1. **RNS-level add/sub**: Each RNS limb's coefficients can be added/subtracted independently. MANA parallelizes across primes (lanes) and across coefficients (Rayon).

2. **K-Elimination division**: The anchor-first exact division, used in BFV rescaling after multiplication.

3. **Batch operations**: Processing multiple ciphertext pairs in parallel.

4. **NTT per-prime parallelism**: `ParallelNTT::ntt_all_lanes()` can apply NTT to each RNS limb in parallel (one thread per prime).

What MANA does NOT accelerate:
- The NTT butterfly computation itself (this is within a single lane)
- Key generation
- Noise budget tracking
- Encryption/decryption

### 8.5 Architectural Verdict

This is a **LAYERED ACCELERATION** architecture, not a fork that diverges from BFV Ring. The nine65 BFV Ring model is preserved INTACT. MANA provides a parallel execution substrate for operations that decompose into per-prime, per-coefficient work. The `accelerated.rs` bridge correctly converts between the two representations.

The "INCOMPATIBLE with BFV lineage" characterization applies to the MANA constructs themselves (Lane, ManaStream have no concept of polynomial rings), but the INTEGRATION is compatible because it operates at the right abstraction level (coefficient-wise operations within RNS limbs, not polynomial operations).

---

## APPENDIX A: FILE MANIFEST

Every source file in the build:

```
crates/mana/Cargo.toml
crates/mana/src/lib.rs
crates/mana/src/lane.rs
crates/mana/src/stream.rs
crates/mana/src/anchor.rs
crates/mana/src/gso.rs
crates/mana/src/parallel.rs
crates/mana/benches/lane_ops.rs
crates/mana/examples/benchmark.rs
crates/nine65/Cargo.toml
crates/nine65/src/lib.rs
crates/nine65/src/accelerated.rs
crates/nine65/src/compiler.rs
crates/nine65/src/kat.rs
crates/nine65/src/v2_integration_tests.rs
crates/nine65/src/arithmetic/mod.rs
crates/nine65/src/arithmetic/montgomery.rs
crates/nine65/src/arithmetic/persistent_montgomery.rs
crates/nine65/src/arithmetic/barrett.rs
crates/nine65/src/arithmetic/ntt.rs
crates/nine65/src/arithmetic/ntt_fft.rs
crates/nine65/src/arithmetic/rns.rs
crates/nine65/src/arithmetic/k_elimination.rs
crates/nine65/src/arithmetic/exact_divider.rs
crates/nine65/src/arithmetic/exact_coeff.rs
crates/nine65/src/arithmetic/ct_mul_exact.rs
crates/nine65/src/arithmetic/mobius_int.rs
crates/nine65/src/arithmetic/pade_engine.rs
crates/nine65/src/arithmetic/mq_relu.rs
crates/nine65/src/arithmetic/integer_softmax.rs
crates/nine65/src/arithmetic/cyclotomic_phase.rs
crates/nine65/src/entropy/mod.rs
crates/nine65/src/entropy/shadow.rs
crates/nine65/src/entropy/secure.rs
crates/nine65/src/entropy/wassan_noise.rs
crates/nine65/src/params/mod.rs
crates/nine65/src/params/primes.rs
crates/nine65/src/params/production.rs
crates/nine65/src/params/validation.rs
crates/nine65/src/ring/mod.rs
crates/nine65/src/ring/polynomial.rs
crates/nine65/src/keys/mod.rs
crates/nine65/src/ops/mod.rs
crates/nine65/src/ops/encrypt.rs
crates/nine65/src/ops/homomorphic.rs
crates/nine65/src/ops/rns_mul.rs
crates/nine65/src/ops/neural.rs
crates/nine65/src/noise/mod.rs
crates/nine65/src/noise/budget.rs
crates/nine65/src/security/mod.rs
crates/nine65/src/ahop/mod.rs
crates/nine65/src/ahop/grover.rs
crates/nine65/src/ahop/grover_full.rs
crates/nine65/src/quantum/mod.rs
crates/nine65/src/quantum/amplitude.rs
crates/nine65/src/quantum/entanglement.rs
crates/nine65/src/quantum/teleport.rs
crates/nine65/benches/fhe_scaling.rs
crates/unhal/Cargo.toml
crates/unhal/src/lib.rs
crates/unhal/src/accelerator.rs
crates/unhal/src/pipeline.rs
crates/unhal/src/batch.rs
```

Total: 60 Rust source files across 3 crates.

---

## APPENDIX B: SUMMARY FINDINGS

### Critical Issues (0)
None. No blocking bugs found.

### High-Priority Observations (3)

1. **DUAL K-ELIMINATION**: mana's `KAnchor` and nine65's `KElimination` are identical algorithms with duplicated helper functions. A divergence in one but not the other could cause subtle mathematical errors. Recommend extracting to shared crate or consolidating.

2. **u128 product_cache overflow**: `ManaStream.product_cache` will overflow for configurations with more than ~4 primes of 30+ bits. The 8-prime benchmark configuration is affected. Reconstruction would produce incorrect results.

3. **AcceleratedFHE::add_ciphertexts bypass**: The supposed accelerated ciphertext addition does NOT use MANA streams. It falls back to simple coefficient-wise addition, making the acceleration claim hollow for this operation.

### Medium-Priority Observations (4)

4. **SIMD feature dead code**: Entire SIMD path (~100 lines in accelerator.rs) references nonexistent `mana::simd::SimdLane`. The `wide` dependency is declared but never used.

5. **compiler.rs float contradiction**: `#![deny(clippy::float_arithmetic)]` attribute coexists with pervasive f64 usage. This is either a non-functional lint or an unchecked file.

6. **P2QuantileEstimator f64 internals**: The P-squared algorithm uses f64 for its marker interpolation. While data inputs are integer (millibits), the statistical tracking is approximate. This is a known compromise.

7. **GSO module unused**: `GsoSwarm` + `QbitAgent` are fully implemented but not wired into any automated pipeline. Available for external use only.

### Low-Priority Observations (3)

8. **`mod_add` duplication within mana**: Identical branchless modular addition in both `lane.rs:101` and `parallel.rs:72`.

9. **`extended_gcd_i128` appears 3 times**: In `mana/stream.rs`, `mana/anchor.rs`, and `nine65/k_elimination.rs`.

10. **Pipeline Custom stage stub**: `Stage::Custom` exists but does nothing.

---

**END OF FORENSIC AUDIT**

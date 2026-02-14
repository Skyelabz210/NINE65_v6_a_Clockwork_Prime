# Forensic Code Audit: Build 13_NINE65_original

**Audit Date:** 2026-02-14
**Auditor:** Claude Opus 4.6 (forensic code auditor)
**Build Path:** `/home/acid/Projects/Homomorphic_Armada/builds/13_NINE65_original/`
**Classification:** INSPECT, ANALYZE, REPORT -- no modifications made

---

## Table of Contents

1. [Structure Mapping](#1-structure-mapping)
2. [Data Flow Tracing](#2-data-flow-tracing)
3. [Construct Identification](#3-construct-identification)
4. [Wiring Verification](#4-wiring-verification)
5. [Dead Code Detection](#5-dead-code-detection)
6. [Report Analysis](#6-report-analysis)
7. [Anomaly Catalogue](#7-anomaly-catalogue)

---

## 1. Structure Mapping

### 1.1 Physical File Layout

The build contains a single Rust package under `qmnf_fhe_production/` with all 24 `.rs` source files placed flat in the root directory. There are no subdirectories.

```
13_NINE65_original/
+-- PRODUCTION_REPORT.md
+-- test_results.txt
+-- benchmark_results.txt
+-- qmnf_fhe_production/
    +-- Cargo.toml
    +-- BENCHMARK_RESULTS.txt
    +-- lib.rs
    +-- mod.rs                     (keys module)
    +-- montgomery.rs
    +-- persistent_montgomery.rs
    +-- barrett.rs
    +-- ntt.rs
    +-- rns.rs
    +-- k_elimination.rs
    +-- exact_divider.rs
    +-- exact_coeff.rs
    +-- ct_mul_exact.rs
    +-- shadow.rs                  (entropy module)
    +-- polynomial.rs              (ring module)
    +-- encrypt.rs
    +-- homomorphic.rs
    +-- rns_mul.rs
    +-- primes.rs
    +-- production.rs
    +-- compiler.rs
    +-- grover.rs
    +-- grover_full.rs
    +-- grover_noise_search.rs
    +-- noise_bench.rs
    +-- fhe_benchmarks.rs
```

Total: 24 Rust source files, 1 Cargo.toml, 2 text result files at the build root, 1 benchmark file and 1 markdown report at the build root, 1 benchmark file inside qmnf_fhe_production.

### 1.2 Declared Module Tree (lib.rs lines 38-45)

```rust
pub mod arithmetic;
pub mod entropy;
pub mod params;
pub mod ring;
pub mod keys;
pub mod ops;
pub mod ahop;
pub mod noise;
```

This declaration implies the following directory structure:

```
src/
+-- lib.rs
+-- arithmetic/
|   +-- mod.rs
|   +-- montgomery.rs
|   +-- persistent_montgomery.rs
|   +-- barrett.rs
|   +-- ntt.rs
|   +-- rns.rs
|   +-- k_elimination.rs
|   +-- exact_divider.rs
|   +-- exact_coeff.rs
|   +-- ct_mul_exact.rs
+-- entropy/
|   +-- mod.rs              (shadow.rs)
+-- params/
|   +-- mod.rs
|   +-- primes.rs
|   +-- production.rs
+-- ring/
|   +-- mod.rs              (polynomial.rs)
+-- keys/
|   +-- mod.rs              (mod.rs)
+-- ops/
|   +-- mod.rs
|   +-- encrypt.rs
|   +-- homomorphic.rs
|   +-- rns_mul.rs
+-- ahop/
|   +-- mod.rs
|   +-- grover.rs
|   +-- grover_full.rs
+-- noise/
|   +-- mod.rs
```

**CRITICAL STRUCTURAL ANOMALY:** The actual files are flat in the directory root. No `src/` directory exists. No subdirectories (`arithmetic/`, `entropy/`, etc.) exist. The `pub mod arithmetic;` declaration in `lib.rs` would fail to compile without either:
- A `src/arithmetic/mod.rs` or `src/arithmetic.rs` file, OR
- A build system that copies/reorganizes files before compilation.

This means either: (a) a separate build step restructures files before `cargo build`, (b) the files were extracted/flattened from a compiled project for archival purposes, or (c) the code as laid out in this directory cannot compile as-is.

### 1.3 Cargo.toml Configuration

**File:** `qmnf_fhe_production/Cargo.toml`

```toml
[package]
name = "qmnf_fhe"
version = "0.1.0"
edition = "2021"
description = "QMNF-Accelerated BFV FHE with AHOP Quantum Simulation"
authors = ["Acidlabz210 <acid@hackfate.us>"]

[dependencies]
# Zero external dependencies

[[bin]]
name = "noise_bench"
path = "benches/noise_bench.rs"

[[bin]]
name = "grover_noise_search"
path = "benches/grover_noise_search.rs"

[profile.release]
opt-level = 3
lto = true

[profile.dev]
opt-level = 1
```

**Key observations:**
- Zero external dependencies (entire system is self-contained).
- Two binary targets reference `benches/` subdirectory paths, but `noise_bench.rs` and `grover_noise_search.rs` exist flat in the root directory, not in a `benches/` subdirectory.
- Release profile enables LTO and maximum optimization.
- The `fhe_benchmarks.rs` file is not declared as a `[[bin]]` target in Cargo.toml, yet the benchmark results reference `src/bin/fhe_benchmarks.rs`.

### 1.4 Prelude (lib.rs lines 47-65)

The prelude re-exports the following types, establishing the intended public API:

| Source Module | Exported Types |
|---------------|---------------|
| `arithmetic` | `MontgomeryContext`, `BarrettContext`, `HybridModContext`, `NTTEngine`, `RNSContext`, `RNSPolynomial`, `PersistentMontgomery`, `PersistentPolynomial` |
| `entropy` | `ShadowHarvester` |
| `params` | `FHEConfig` |
| `ring` | `RingPolynomial` |
| `keys` | `SecretKey`, `PublicKey`, `EvaluationKey`, `KeySet` |
| `ops` | `BFVEncoder`, `BFVEncryptor`, `BFVDecryptor`, `BFVEvaluator`, `Ciphertext` |
| `ahop` | `Fp2Element`, `StateVector`, `GroverSearch`, `GroverStats` |
| `noise` | `NoiseBudgetTracker`, `NoiseSnapshot`, `EMACalculator`, `MultiWindowNoiseDetector`, `NoiseAnomaly`, `P2QuantileEstimator`, `NoiseDistribution` |

**Note:** The `KElimination`, `ExactDivider`, `ExactCoeff`, `ExactContext`, `ExactPoly`, `ExactFHEContext`, `ExactCiphertext`, `ExactCiphertext2`, `BootstrapFreeFHECompiler`, `ProductionConfig128`, `ModulusChain`, `FullGroverSuite`, and `RNSEvaluator` types are NOT in the prelude. They are accessible via their full module paths but are not promoted to top-level API status.

---

## 2. Data Flow Tracing

### 2.1 BFV FHE Pipeline Overview

This build implements the BFV (Brakerski/Fan-Vercauteren) Fully Homomorphic Encryption scheme. The data flow for the primary pipeline is:

```
Parameters (FHEConfig)
    |
    v
Key Generation (KeySet::generate)
    |-- SecretKey: ternary polynomial s in R_q
    |-- PublicKey: (pk0, pk1) = (-a*s + e, a) where a is uniform, e is CBD noise
    |-- EvaluationKey: rlk[i] = (-a_i*s + e_i + s^2 * T^i, a_i) decomposed
    |
    v
Encoding (BFVEncoder::encode)
    |-- m -> delta * m where delta = floor(q/t)
    |-- Embeds scalar into constant term of polynomial
    |
    v
Encryption (BFVEncryptor::encrypt)
    |-- u: ternary random polynomial
    |-- e1, e2: CBD noise polynomials
    |-- ct = (pk0*u + e1 + encoded_m, pk1*u + e2)
    |
    v
Homomorphic Operations (BFVEvaluator)
    |-- add: (c0_1 + c0_2, c1_1 + c1_2)
    |-- sub: (c0_1 - c0_2, c1_1 - c1_2)
    |-- mul_plain: (c0 * m, c1 * m)
    |-- mul (ct x ct):
    |     |-- tensor_product -> (d0, d1, d2) at delta^2 scale
    |     |-- relinearize -> (c0', c1') still at delta^2 scale
    |     |-- scale_by_t_over_q -> back to delta scale
    |
    v
Decryption (BFVDecryptor::decrypt)
    |-- inner = c0 + c1*s
    |-- m = round(inner * t / q) mod t
```

### 2.2 Three ct x ct Multiplication Paths

This build contains three distinct implementations of ciphertext-ciphertext multiplication:

**Path 1: Single-Modulus with Degree-2 Decrypt** (homomorphic.rs + encrypt.rs)
- Tensor product produces (d0, d1, d2) at scale delta^2
- Does NOT scale individual components
- Uses `decrypt_degree2` to compute `round((d0 + d1*s + d2*s^2) * t^2 / q^2)`
- Limited by requirement that delta^2 * product < q
- Used via `FHEConfig::light_mul()` which sets small delta

**Path 2: Single-Modulus with Relin + Scaling** (homomorphic.rs)
- Tensor product -> relinearize -> scale each coefficient by t/q
- Uses `KElimination::scale_and_round()` for the coefficient-wise scaling
- `BFVEvaluator::mul()` method
- Scaling after relin, coefficient-wise

**Path 3: RNS-Based Multiplication** (rns_mul.rs)
- Lifts ciphertexts to multi-prime RNS representation
- Tensor product in RNS (overflow-safe)
- CRT reconstruction + t/q scaling + mod q reduction
- Two of three tests are `#[ignore]` with note "RNS approach needs proper ciphertext generation in RNS space"

**Path 4: Exact Dual-Track Multiplication** (ct_mul_exact.rs + exact_coeff.rs)
- Maintains dual-track coefficients: inner RNS residue + anchor (M, A) residues
- NTT in both tracks simultaneously
- After tensor product, reconstructs true integers via K-Elimination
- Divides by delta exactly (integer division, guaranteed exact when divisor divides value)
- One test `#[ignore]` -- anchor NTT needs extended roots for general polynomials
- Only demonstrated with trivial (zero c1) ciphertexts in passing test

### 2.3 Entropy Flow

```
ShadowHarvester (LFSR + counter + MurmurHash3 mixing)
    |-- next_u64() -> raw 64-bit pseudorandom
    |-- uniform(bound) -> rejection-sampled uniform [0, bound)
    |-- cbd(eta) -> Centered Binomial Distribution sample in [-eta, eta]
    |-- ternary() -> {-1, 0, 1} uniform
    |
    +-> KeySet::generate: generates s (ternary), a (uniform), e (CBD)
    +-> BFVEncryptor::encrypt: generates u (ternary), e1/e2 (CBD)
```

### 2.4 Quantum Simulation Flow (AHOP)

```
Fp2Element (a + b*i over F_p)
    |
    v
StateVector (N amplitudes in F_{p^2})
    |-- uniform(): equal superposition |+>
    |-- probability(): norm_squared / total_weight (uses f64)
    |
    v
GroverSearch
    |-- apply_oracle: phase-flip target state
    |-- apply_diffusion: reflect about mean (2|s><s| - I)
    |-- run(iterations): iterate oracle + diffusion
    |-- optimal_iterations(): pi/4 * sqrt(N)  [uses f64]
    |
    v
FullGroverSuite (grover_full.rs)
    |-- Multi-target Grover
    |-- Quantum counting
    |-- Amplitude estimation
    |-- Durr-Hoyer minimum finding
    |-- [Extensive f64 usage throughout]
```

---

## 3. Construct Identification

### 3.1 Core Arithmetic Constructs

| Construct | File | Lines | Purpose |
|-----------|------|-------|---------|
| `MontgomeryContext` | montgomery.rs | 1-130 | Standard Montgomery multiplication with REDC, R=2^64 |
| `PersistentMontgomery` | persistent_montgomery.rs | 1-400+ | "Stay in Montgomery form permanently" variant; avoids to/from conversion |
| `BarrettContext` | barrett.rs | 1-60 | Barrett reduction for isolated modular reductions |
| `HybridModContext` | barrett.rs | 62-100 | Combines Montgomery (for mul chains) + Barrett (for isolated ops) |
| `NTTEngine` | ntt.rs | 1-345 | O(N^2) DFT-matrix NTT with psi-twist for negacyclic convolution |
| `RNSContext` | rns.rs | 1-100+ | Multi-prime CRT with precomputed reconstruction values |
| `RNSPolynomial` | rns.rs | 100-200+ | Polynomial with per-prime limbs; add/sub/neg/mul via NTT per limb |
| `KElimination` | k_elimination.rs | 19-138 | Dual-codex (alpha, beta) exact reconstruction and division |
| `ExactDivider` | exact_divider.rs | full file | Dual-modulus (M, A) CRT reconstruction for exact division |
| `ExactCoeff` | exact_coeff.rs | struct | Dual-track coefficient: RnsInner (fast) + AnchorTrack (exact) |
| `ExactContext` | exact_coeff.rs | struct | Operations on ExactCoeff: encode, decode, reconstruct, zero |
| `ExactPoly` | exact_coeff.rs | struct | Polynomial of ExactCoeffs with add/sub/neg/pointwise_mul |

### 3.2 FHE Constructs

| Construct | File | Lines | Purpose |
|-----------|------|-------|---------|
| `FHEConfig` | (params module, not directly visible) | -- | Parameter set: q, n, t, eta, primes, name |
| `SecretKey` | mod.rs | struct | Ternary polynomial s |
| `PublicKey` | mod.rs | struct | RLWE pair (pk0, pk1) |
| `EvaluationKey` | mod.rs | struct | Relinearization key: rlk decomposition pairs |
| `KeySet` | mod.rs | struct | Aggregates SecretKey + PublicKey + EvaluationKey |
| `BFVEncoder` | encrypt.rs | struct | Message encoding: m -> delta*m; includes decode_degree2 |
| `BFVEncryptor` | encrypt.rs | struct | RLWE-based encryption |
| `BFVDecryptor` | encrypt.rs | struct | Standard + degree-2 decryption |
| `Ciphertext` | encrypt.rs | struct | (c0, c1) pair of RingPolynomials |
| `BFVEvaluator` | homomorphic.rs | struct | Homomorphic add/sub/negate/add_plain/mul_plain/mul |
| `RNSEvaluator` | rns_mul.rs | struct | RNS-based ct*ct multiplication path |
| `ExactCiphertext` | ct_mul_exact.rs | struct | Dual-track ciphertext (c0, c1) as ExactPolys |
| `ExactCiphertext2` | ct_mul_exact.rs | struct | Degree-2 exact ciphertext (d0, d1, d2) |
| `ExactFHEContext` | ct_mul_exact.rs | struct | Full exact FHE context with dual-track NTT |
| `RingPolynomial` | polynomial.rs | struct | Polynomial in R_q = Z_q[X]/(X^N+1) |

### 3.3 Quantum Simulation Constructs

| Construct | File | Purpose |
|-----------|------|---------|
| `Fp2Element` | (ahop module, not directly visible) | Field element a + b*i in F_{p^2} |
| `StateVector` | (ahop module) | Quantum state as vector of Fp2 amplitudes |
| `GroverSearch` | grover.rs | Single-target Grover search |
| `GroverStats` | grover.rs | Per-iteration statistics |
| `FullGroverSuite` | grover_full.rs | Comprehensive Grover ecosystem |
| `Fp2` | grover_full.rs | Duplicate F_{p^2} type (not shared with ahop module) |
| `Unitary` | grover_full.rs | Quantum gate/operator |

### 3.4 Parameter & Configuration Constructs

| Construct | File | Purpose |
|-----------|------|---------|
| `ProductionConfig128` | production.rs | 128/192-bit security parameter sets |
| `ModulusChain` | production.rs | RNS modulus chain for level-based rescaling |
| `PRODUCTION_PRIMES_60BIT` | production.rs | 20 primes near 2^60 |
| `PRODUCTION_PRIMES_30BIT` | production.rs | 15 primes near 2^30 |
| `PRIMES_1024/4096/8192` | primes.rs | NTT-friendly prime constants per degree |

### 3.5 Compiler & Noise Constructs

| Construct | File | Purpose |
|-----------|------|---------|
| `BootstrapFreeFHECompiler` | compiler.rs | Static noise analysis and parameter selection |
| `Circuit` / `CircuitNode` | compiler.rs | DAG representation of FHE computation |
| `OpType` | compiler.rs | Enum: Add, Multiply, Rescale, Relinearize, Rotate, Input, Output |
| `NoiseModel` | compiler.rs | Per-operation noise growth model (uses f64) |
| `NoiseAnalyzer` | compiler.rs | Topological-order noise computation |
| `ParameterSelector` | compiler.rs | Selects poly degree and modulus chain for circuit |
| `FHEParameters` | compiler.rs | Output: poly_degree, modulus_chain, security_bits |
| `NoiseBudgetTracker` | (noise module, not directly in files) | Runtime noise monitoring |
| `EMACalculator` | (noise module) | Exponential moving average |
| `MultiWindowNoiseDetector` | (noise module) | Multi-window anomaly detection |
| `P2QuantileEstimator` | (noise module) | P^2 quantile estimation |
| `NoiseDistribution` | (noise module) | Noise distribution tracking |

### 3.6 Benchmark & Binary Constructs

| Construct | File | Purpose |
|-----------|------|---------|
| `BenchResult` | fhe_benchmarks.rs | Statistical benchmark result (mean, std_dev, percentiles) |
| `GroverNoiseSearch` | grover_noise_search.rs | Grover-accelerated noise anomaly detection |

---

## 4. Wiring Verification

### 4.1 Cross-Module References

The following `use` statements establish inter-module dependencies:

| File | References | Status |
|------|-----------|--------|
| ntt.rs | `use super::montgomery::MontgomeryContext` | Assumes `ntt.rs` is sibling of `montgomery.rs` in `arithmetic/` module -- requires module hierarchy |
| barrett.rs | `use super::montgomery::MontgomeryContext` | Same assumption |
| polynomial.rs | `use crate::arithmetic::NTTEngine` | Requires `arithmetic` module at crate root |
| polynomial.rs | `use crate::entropy::ShadowHarvester` | Requires `entropy` module at crate root |
| encrypt.rs | `use crate::arithmetic::NTTEngine`, `crate::entropy::ShadowHarvester`, `crate::keys::*`, `crate::params::FHEConfig`, `crate::ring::RingPolynomial` | Cross-module wiring to 5 modules |
| homomorphic.rs | `use crate::arithmetic::{NTTEngine, KElimination}`, `crate::keys::EvaluationKey`, `crate::ops::encrypt::{BFVEncoder, Ciphertext}`, `crate::ring::RingPolynomial` | Cross-module wiring to 4 modules |
| ct_mul_exact.rs | `use super::exact_coeff::*`, `crate::arithmetic::NTTEngine`, `crate::ring::RingPolynomial` | References sibling in arithmetic module |
| rns_mul.rs | `use crate::arithmetic::{NTTEngine, RNSContext, RNSPolynomial, KElimination}`, `crate::ring::RingPolynomial`, `crate::ops::Ciphertext`, `crate::params::FHEConfig` | Cross-module wiring to 4 modules |
| grover.rs | `use super::{Fp2Element, StateVector}` | References parent ahop module types |
| mod.rs (keys) | `use crate::arithmetic::NTTEngine`, `crate::ring::RingPolynomial`, `crate::params::FHEConfig`, `crate::entropy::ShadowHarvester` | Cross-module wiring to 4 modules |

### 4.2 Wiring Status Summary

All cross-module references are internally consistent IF the module hierarchy implied by `lib.rs` is in place. The references use `crate::` absolute paths and `super::` relative paths that match the declared 8-module structure:

- `arithmetic` -> contains: montgomery, persistent_montgomery, barrett, ntt, rns, k_elimination, exact_divider, exact_coeff, ct_mul_exact
- `entropy` -> contains: shadow (ShadowHarvester)
- `params` -> contains: FHEConfig, production, primes
- `ring` -> contains: polynomial (RingPolynomial)
- `keys` -> contains: mod.rs (SecretKey, PublicKey, EvaluationKey, KeySet)
- `ops` -> contains: encrypt, homomorphic, rns_mul
- `ahop` -> contains: grover, grover_full, Fp2Element, StateVector
- `noise` -> contains: noise tracking types

### 4.3 Stubs and Incomplete Features

| Feature | Evidence | Severity |
|---------|----------|----------|
| RNS multiplication | 2 of 3 tests `#[ignore]` with comment "RNS approach needs proper ciphertext generation in RNS space" | HIGH -- core multiplication path incomplete |
| Production 128-bit | Integration test `#[ignore]` with comment "Needs proper noise budget tracking for N=8192" | MEDIUM -- production parameters untested |
| Exact poly mul constant | Test `#[ignore]` with comment "Anchor track NTT needs proper primitive root selection for general polynomials" | MEDIUM -- anchor NTT only works for trivial cases |
| Anchor NTT fallback | ct_mul_exact.rs line 92-97: falls back to dummy values (all 1s) when A does not support 2N-th roots | HIGH -- silently degrades to incorrect anchor track |
| ExactFHE relinearization | `relinearize_simple()` comment: "For proper security, use evaluation key version" -- only insecure s^2 method implemented | HIGH -- security-critical operation is a stub |
| Noise module | Types referenced in prelude (NoiseBudgetTracker, etc.) but no `noise/mod.rs` visible in file listing | Unverifiable -- module source not found in flat layout |
| Production primes validation | `PRODUCTION_PRIMES_60BIT` are claimed to be NTT-friendly but their NTT compatibility is NOT tested (only `PRODUCTION_PRIMES_30BIT` is validated in tests) | MEDIUM |
| Compiler modulus chain | `generate_modulus_chain()` uses hardcoded primes that are NOT the same as `PRODUCTION_PRIMES_60BIT` and are NOT tested for primality or NTT compatibility | MEDIUM |

### 4.4 Module-Internal Connectivity

The `BFVEvaluator::mul()` pipeline connects:
1. `mul_no_relin()` -> produces (d0, d1, d2) via NTT polynomial multiply
2. `relinearize()` -> decomposes d2 into base-w digits, applies evaluation key
3. `scale_by_t_over_q()` -> applies `KElimination::scale_and_round()` per coefficient

This pipeline is fully wired. However, the `scale_and_round()` method in `KElimination` (line 136) has a suspicious `% q` at the end that may be incorrect for values larger than q (the result of scaling should not be reduced mod q before being stored back in a polynomial whose coefficients ARE mod q -- but the order of operations matters).

---

## 5. Dead Code Detection

### 5.1 Compiler-Reported Dead Code

From `benchmark_results.txt` lines 7-14:
```
--> src/bin/fhe_benchmarks.rs:19:5
 |
16 | struct BenchResult {
   |        ----------- field in this struct
...
19 |     total_time: Duration,
   |     ^^^^^^^^^^
 = note: `BenchResult` has derived impls for the traits `Clone` and `Debug`,
         but these are intentionally ignored during dead code analysis
 = note: `#[warn(dead_code)]` on by default
```

**Finding:** `BenchResult.total_time` field is declared but never read. Only `mean`, `std_dev`, `p50`, `p95`, `p99` are used in the benchmark output formatting.

### 5.2 Unused Imports

| File | Import | Status |
|------|--------|--------|
| compiler.rs line 10 | `use std::collections::VecDeque` | VecDeque is imported but never used. Only `HashMap` is actually used. |
| ct_mul_exact.rs line 149 | `let _n = self.exact_ctx.n;` | Variable assigned but prefixed with underscore to suppress warning. Pattern repeats at lines 232, 293, 295. |

### 5.3 Standalone `main()` in Library Module

`grover_full.rs` contains a `fn main()` function (visible from the file header "NOT THE HELLO WORLD VERSION"). This is a standalone executable embedded in what should be a library module. If this file is part of the `ahop` module, the `main()` would be dead code in library context. If it is compiled as a separate binary, it is not declared in `Cargo.toml`.

### 5.4 Duplicate Utility Functions

The following functions are implemented multiple times across different files:

| Function | Occurrences | Files |
|----------|-------------|-------|
| `mod_pow(base, exp, modulus)` | 5+ | ntt.rs, primes.rs, grover.rs, grover_full.rs, ct_mul_exact.rs |
| `mod_inverse(a, m)` | 5+ | ntt.rs, primes.rs, ct_mul_exact.rs, exact_divider.rs, rns.rs |
| `extended_gcd` | 2+ | primes.rs, k_elimination.rs |
| `gcd` | 2+ | primes.rs, exact_divider.rs |
| `is_prime` | 1 | primes.rs (but not used outside tests) |
| `find_primitive_root` | 2 | primes.rs (public), ntt.rs (private, different algorithm) |

These duplicates exist because each file implements its own local versions rather than importing from a shared utility. Each implementation is slightly different in style but functionally equivalent.

### 5.5 Unreachable Code Paths

| Location | Description |
|----------|-------------|
| ct_mul_exact.rs line 95 | Anchor NTT returns dummy values when A does not support 2N-th roots. The subsequent NTT operations on the anchor track produce meaningless results, making the anchor track computations dead in practice for most prime selections. |
| ct_mul_exact.rs line 108 | The `g > 1000` fallback returns `psi = 1`, making the anchor NTT a no-op. |
| compiler.rs | The `VecDeque` import is unused. |

---

## 6. Report Analysis

### 6.1 PRODUCTION_REPORT.md

**Location:** `/home/acid/Projects/Homomorphic_Armada/builds/13_NINE65_original/PRODUCTION_REPORT.md`

**Claims:**
- Version 1.0.0, dated 2024-12-20
- "PRODUCTION READY" status
- 140 tests pass, 0 failed, 4 ignored
- Documents 5 innovations: K-Elimination, Persistent Montgomery, Shadow Entropy, Dual-Track Exact Arithmetic, NTT Gen3
- Claims "65+ mathematical impossibilities conquered"

**Architecture diagram discrepancy:** The report shows a `src/` subdirectory structure:
```
qmnf_fhe/
+-- src/
    +-- lib.rs
    +-- arithmetic/
    +-- params/
    +-- ...
```
This does NOT match the actual flat file layout.

**Benchmark numbers in the report do NOT match the benchmark output:**

| Operation | PRODUCTION_REPORT.md claim | benchmark_results.txt actual |
|-----------|---------------------------|------------------------------|
| Montgomery Multiply | 24.16 ns | 24.32 ns |
| Persistent Montgomery | 24.54 ns | 24.11 ns |
| K-Elimination Division | 24.41 ns | 27.50 ns |
| ExactDivider Reconstruct | 24.13 ns | 26.33 ns |
| Shadow Entropy Sample | 24.33 ns | 24.44 ns |
| KeyGen | 22.96 ms | 25.43 ms |
| Full Homo Mul | 46.73 ms | 46.01 ms |

The differences are small (different benchmark runs), but the report selectively presents slightly better numbers.

**Benchmark summary in benchmark_results.txt (lines 267-273) is misleading:**
```
Key Performance Metrics:
  * Montgomery multiply:     ~4 ns/op
  * Persistent Montgomery:   ~4 ns/op
  * K-Elimination division:  ~20 ns/op
  * Shadow Entropy sample:   ~10 ns/op
  * NTT poly multiply:       ~100 us/op (N=1024)
  * Full homo multiply:      ~5-10 ms/op (N=1024)
```

These "summary" numbers are significantly lower than the actual measurements:
- Montgomery claimed "~4 ns/op" but measured 24.32 ns (6x discrepancy)
- Shadow Entropy claimed "~10 ns/op" but measured 24.44 ns (2.4x discrepancy)
- NTT poly multiply claimed "~100 us/op" but measured 6255.69 us (~6.3 ms, 63x discrepancy)
- Full homo multiply claimed "~5-10 ms/op" but measured 46.01 ms (4.6-9.2x discrepancy)

### 6.2 test_results.txt

**Location:** `/home/acid/Projects/Homomorphic_Armada/builds/13_NINE65_original/test_results.txt`

**Summary:** `test result: ok. 140 passed; 0 failed; 4 ignored; 0 measured; 0 filtered out; finished in 11.74s`

**Ignored tests (4):**
1. `test_exact_poly_mul_constant` -- anchor NTT issue
2. `test_production_128bit` -- needs noise budget tracking
3. `test_rns_mul_with_relin` -- RNS needs proper ciphertext generation
4. `test_rns_mul_multiple_values` -- same as above

**Notable:** The doc-test from lib.rs also passes (1 test, 11.99s), confirming the documented API example compiles and runs.

**Test distribution by module (from partial listing):**
- `params::primes::tests` -- 8 tests
- `params::production::tests` -- 5 tests
- `params::tests` -- 3 tests
- `ring::polynomial::tests` -- 9 tests
- `ops::rns_mul::tests` -- 1 pass, 2 ignored
- `ops::homomorphic::tests` -- 1+ (benchmark test visible)
- `ops::encrypt::tests` -- 1+ (benchmark test visible)
- `keys::tests` -- 1+ (keygen benchmark)

The full 140 test count includes tests from all modules. The listing in the results file shows only the tail of the output.

### 6.3 benchmark_results.txt

**Location:** `/home/acid/Projects/Homomorphic_Armada/builds/13_NINE65_original/benchmark_results.txt`

**Configuration:** 100 warmup iterations, 10000 benchmark iterations (reduced for slower operations).

**Section 1 -- QMNF Innovation Components:**

| Operation | Mean | StdDev | P50 | P95 | Throughput |
|-----------|------|--------|-----|-----|-----------|
| Montgomery Multiply | 24.32 ns | 13.51 ns | 24 ns | 25 ns | 41.1M ops/sec |
| Persistent Montgomery | 24.11 ns | 0.84 ns | 24 ns | 25 ns | 41.5M ops/sec |
| NTT Forward (N=1024) | 1.92 ms | 81.6 us | 1.90 ms | 2.00 ms | 521 ops/sec |
| NTT Poly Multiply (N=1024) | 6.26 ms | 181.8 us | 6.26 ms | 6.47 ms | 160 ops/sec |
| K-Elimination Exact Div | 27.50 ns | 99.61 ns | 25 ns | 35 ns | 36.4M ops/sec |
| ExactDivider Reconstruct | 26.33 ns | 129.09 ns | 24 ns | 28 ns | 38.0M ops/sec |
| Shadow Entropy Sample | 24.44 ns | 1.78 ns | 24 ns | 26 ns | 40.9M ops/sec |
| CBD Noise Vector (N=1024) | 12.67 us | 3.12 us | 10.80 us | 18.28 us | 78.9K ops/sec |

**Observation:** Montgomery and Persistent Montgomery have nearly identical mean times (~24 ns), suggesting the "zero conversion overhead" claim is valid in the sense that persistent mode doesn't add overhead, but standard Montgomery also doesn't have meaningful conversion overhead in these microbenchmarks.

**Observation:** K-Elimination and ExactDivider have very high standard deviations relative to their means (99.6 ns and 129.1 ns stddev on ~27 ns and ~26 ns means). This suggests occasional large outliers, possibly from branch mispredictions or cache effects, or could indicate measurement noise at such small timescales.

**Section 2 -- FHE Operations (N=1024):**

| Operation | Mean | Throughput |
|-----------|------|-----------|
| KeyGen | 25.43 ms | 39 ops/sec |
| Encrypt | 11.50 ms | 87 ops/sec |
| Decrypt | 5.80 ms | 173 ops/sec |
| Homo Add | 2.71 us | 369K ops/sec |
| Homo Mul Plain | 6.29 us | 159K ops/sec |
| Tensor Product | 23.64 ms | 42 ops/sec |
| Full Homo Mul | 46.01 ms | 22 ops/sec |

**Note:** These times are for N=1024, a toy parameter. Production N=8192 would be roughly 8-16x slower for O(N^2) NTT operations, meaning NTT-based operations would take approximately 64-256x longer (since NTT dominates encrypt/decrypt/multiply and NTT is O(N^2)).

**Section 3 -- Exact CT x CT (N=8 test size):**

| Operation | Mean | Throughput |
|-----------|------|-----------|
| ExactCoeff Add | 47.0 ns | 21.3M ops/sec |
| ExactCoeff Mul | 47.3 ns | 21.1M ops/sec |
| ExactCoeff Exact Div | 88.4 ns | 11.3M ops/sec |
| Exact Tensor Product (N=8) | 13.51 us | 74.0K ops/sec |
| Exact Rescale (N=8) | 1.12 us | 894.9K ops/sec |

These are at N=8 test size, not production-relevant.

---

## 7. Anomaly Catalogue

### 7.1 Float Violations

The QMNF system mandates "Zero Floating-Point Guarantee" (lib.rs line 14-17), yet f64 is used extensively:

| File | Location | Float Usage | Severity |
|------|----------|-------------|----------|
| grover.rs | line 81 | `(std::f64::consts::PI / 4.0) * n.sqrt()` in `optimal_iterations()` | MEDIUM -- non-cryptographic but violates stated guarantee |
| grover.rs | lines 93, 95, 113-122 | `GroverStats` fields: `target_probability: f64`, `total_weight as f64`, `max_non_target_prob: f64`, `f64::max` | MEDIUM |
| grover_full.rs | line 29 | `use std::f64::consts::PI;` | HIGH -- extensive use throughout |
| grover_full.rs | throughout | `atan2`, `asin`, `sin`, `cos`, `ln`, `powi`, `sqrt`, `f64` casts everywhere | HIGH -- entire quantum counting/amplitude estimation subsystem relies on f64 |
| compiler.rs | line 8 | `#![deny(clippy::float_arithmetic)]` attribute declared | PARADOX -- the deny attribute is declared but the file uses f64 in NoiseModel |
| compiler.rs | lines 120-126 | `NoiseModel` fields all f64: `add_noise_bits`, `mul_noise_bits`, etc. | HIGH -- noise analysis fundamentally uses floats |
| compiler.rs | lines 140-151, 380-414 | `noise_for_op()` returns f64, `estimate_speedup()` returns f64 | HIGH |
| production.rs | line 63 | `sigma: f64` field in ProductionConfig128 | MEDIUM -- conceptual value, labeled "we use integer CBD" |
| production.rs | line 155 | `(n as f64 / 37.5) as usize` in `estimated_security()` interpolation fallback | LOW -- only hit for non-standard N values |
| shadow.rs | line 203 | `let mean = sum as f64 / n as f64;` in test_cbd_mean | LOW -- test-only code |
| persistent_montgomery.rs | line ~372 | `f64` for speedup calculation in test | LOW -- test-only code |
| fhe_benchmarks.rs | throughout | `BenchResult` statistics: mean, std_dev, percentiles all computed with f64 | MEDIUM -- benchmark utility, not cryptographic |
| noise_bench.rs | throughout | Rate/latency formatting with f64 | LOW -- benchmark binary |
| grover_noise_search.rs | throughout | `log2()`, `sqrt()`, PI used for Grover iteration count | MEDIUM |

**Summary:** 13 of 24 source files contain f64 usage. The claim of "Zero Floating-Point Guarantee" is limited to the core BFV encrypt/decrypt/homomorphic-add/homomorphic-mul-plain pipeline. Float usage is pervasive in:
- Quantum simulation (grover.rs, grover_full.rs)
- Bootstrap-free compiler noise model (compiler.rs)
- Production parameter estimation (production.rs)
- Benchmark infrastructure (fhe_benchmarks.rs, noise_bench.rs)
- Noise analysis utilities

### 7.2 NTT Complexity

**Finding:** The NTT implementation in `ntt.rs` (lines 94-109) uses an O(N^2) DFT matrix approach:

```rust
pub fn ntt(&self, a: &[u64]) -> Vec<u64> {
    let mut result = vec![0u64; self.n];
    for k in 0..self.n {
        let mut sum = 0u128;
        for j in 0..self.n {
            let exp = (k * j) % self.n;
            let w = self.omega_powers[exp];
            sum += (a[j] as u128) * (w as u128);
        }
        result[k] = (sum % self.q as u128) as u64;
    }
    result
}
```

This is NOT the O(N log N) Cooley-Tukey butterfly algorithm. For N=1024, this means 1,048,576 multiply-accumulate operations per NTT instead of ~10,240. This explains the 1.92 ms NTT time and the 6.26 ms polynomial multiply time (which requires 2 forward NTTs + 1 inverse NTT).

The doc comment claims "42x speedup" for "NTT Engine Gen 3" but this is 42x versus schoolbook polynomial multiplication (O(N^2) schoolbook * 2), not versus a proper butterfly NTT.

**Impact:** At production N=8192, this O(N^2) NTT would require 67M multiply-accumulates per transform, making full homomorphic multiplication roughly 128x slower than necessary. A butterfly NTT would bring this to ~106K operations.

### 7.3 K-Elimination Range Limitation

**Finding:** `KElimination::for_fhe()` (k_elimination.rs lines 63-69) uses hardcoded small primes:

```rust
pub fn for_fhe(_q: u64) -> Self {
    let alpha_primes = vec![65537, 65521, 65519]; // ~48 bits total
    let beta_primes = vec![65497, 65479];          // ~32 bits total
    Self::new(&alpha_primes, &beta_primes)
}
```

- alpha_cap = 65537 * 65521 * 65519 = ~2^48
- beta_cap = 65497 * 65479 = ~2^32
- Total CRT range = alpha_cap * beta_cap = ~2^80

The `_q` parameter is IGNORED. For the BFV scheme with q = 998244353 (30-bit), values in the tensor product can reach delta^2 * N * t which for N=1024, t=2053 could be well beyond 2^80. This creates a silent overflow in the CRT reconstruction for large polynomials.

### 7.4 scale_and_round Final Reduction

**Finding:** In `KElimination::scale_and_round()` (k_elimination.rs line 136):

```rust
((full_numerator / q) % q) as u64
```

The final `% q` is applied to the result of the division. This is correct for coefficients that should be reduced mod q, but it conflates the scaling operation with modular reduction. If `full_numerator / q` is larger than q (which can happen for large coefficients), the `% q` silently wraps the result. This is likely intentional for BFV where coefficients live in Z_q, but it masks potential range issues.

### 7.5 Benchmark Summary Fabrication

**Finding:** The benchmark summary (benchmark_results.txt lines 267-278) presents numbers that are 2-63x lower than the actual measured values:

| Metric | Claimed | Actual | Factor |
|--------|---------|--------|--------|
| Montgomery multiply | ~4 ns/op | 24.32 ns | 6.1x |
| Persistent Montgomery | ~4 ns/op | 24.11 ns | 6.0x |
| K-Elimination division | ~20 ns/op | 27.50 ns | 1.4x |
| Shadow Entropy sample | ~10 ns/op | 24.44 ns | 2.4x |
| NTT poly multiply | ~100 us/op | 6255.69 us | 62.6x |
| Full homo multiply | ~5-10 ms/op | 46.01 ms | 4.6-9.2x |

These summary numbers appear to be aspirational targets or measurements from a different run/platform, not derived from the detailed results above them in the same file.

### 7.6 Duplicate BENCHMARK_RESULTS.txt

There are two benchmark result files:
- `/home/acid/Projects/Homomorphic_Armada/builds/13_NINE65_original/benchmark_results.txt` (build root)
- `/home/acid/Projects/Homomorphic_Armada/builds/13_NINE65_original/qmnf_fhe_production/BENCHMARK_RESULTS.txt` (inside source dir)

These contain results from different runs with slightly different numbers, creating potential confusion about which is authoritative.

### 7.7 ShadowHarvester Cryptographic Weakness

**Finding:** The `ShadowHarvester` (shadow.rs) is described as "NIST SP 800-22 validated" but implements:
1. A 64-bit LFSR with fixed feedback polynomial (lines 38-40)
2. A simple counter (line 43)
3. MurmurHash3-style mixing (lines 46-54)

This is NOT a CSPRNG. LFSRs are linear and completely predictable given sufficient output. The MurmurHash3 mixing adds computational obscurity but not cryptographic security. The combination provides good statistical properties (passing NIST tests for randomness) but is fundamentally insecure as a cryptographic random number generator.

For FHE parameter generation this is acceptable (reproducible key generation is a feature), but the documentation claiming "NIST-validated" conflates statistical randomness testing with cryptographic security validation.

### 7.8 Insecure Relinearization in ExactFHE

**Finding:** `ExactFHEContext::relinearize_simple()` (ct_mul_exact.rs lines 406-417):

```rust
pub fn relinearize_simple(&self, ct2: &ExactCiphertext2, s: &ExactPoly) -> ExactCiphertext {
    // c0' = d0 + d2 * s^2
    // c1' = d1
    let s2 = self.poly_mul(s, s);
    let d2_s2 = self.poly_mul(&ct2.d2, &s2);
    let c0 = ct2.d0.add(&d2_s2, &self.exact_ctx);
    let c1 = ct2.d1.clone();
    ExactCiphertext { c0, c1 }
}
```

This requires the SECRET KEY `s` as input. In any real deployment, this completely breaks the security model because:
- The evaluator should not have access to s
- This is equivalent to decrypting and re-encrypting
- The comment acknowledges: "For proper security, use evaluation key version" but no secure version exists

### 7.9 Production Primes Not Validated

**Finding:** `PRODUCTION_PRIMES_60BIT` (production.rs lines 9-33) lists 20 primes claimed to be near 2^60, but:
- None of these primes are tested for primality in any test
- None are tested for NTT compatibility
- The comments suggest they follow the pattern `2^60 - k + 1` but the actual values are not verified
- Only `PRODUCTION_PRIMES_30BIT` has NTT compatibility tests (test_primes_are_ntt_compatible for N=8192)

Similarly, `compiler.rs` `generate_modulus_chain()` (lines 273-289) uses a completely different set of hardcoded 60-bit primes that are NOT tested for primality, NTT compatibility, or coprimality.

### 7.10 Compiler Float Contradiction

**Finding:** `compiler.rs` line 8 declares `#![deny(clippy::float_arithmetic)]` which should cause a compilation error for any float arithmetic. However, the file uses f64 extensively in `NoiseModel`, `NoiseAnalyzer`, `ParameterSelector`, and `BootstrapFreeFHECompiler`.

This attribute is a file-level inner attribute (`#!`), meaning it applies to the entire file. Either:
- Clippy is not run as part of the build (only `cargo build` / `cargo test` are run)
- The attribute is ignored during regular compilation (it only triggers with `cargo clippy`)
- The file cannot pass clippy

This is a contradiction between declared intent (no float arithmetic) and implementation (extensive float arithmetic).

### 7.11 grover_full.rs Duplicate Types

**Finding:** `grover_full.rs` defines its own `Fp2` struct (F_{p^2} field element) that is separate from the `Fp2Element` in the parent `ahop` module. Both represent the same mathematical object (a + b*i in F_p) but are independent types that cannot interoperate. This creates a maintenance burden and potential for divergence.

Additionally, `grover_full.rs` defines its own `StateVector` and `Unitary` types that are separate from the `StateVector` in the `ahop` module.

### 7.12 FHEConfig Invisible

The `FHEConfig` struct and its configuration methods (`light()`, `light_mul()`, `standard_128()`, `custom()`) are referenced throughout the codebase but the params module `mod.rs` is not visible as a separate file in the flat directory. It must be embedded in one of the existing files or compiled from a missing file. The tests reference `FHEConfig::light()`, `FHEConfig::light_mul()`, `FHEConfig::standard_128()`, and `FHEConfig::custom()` with fields including `q`, `n`, `t`, `eta`, `primes`, `name`, `delta()`, and `supports_single_mod_mul()`.

---

## Summary of Findings

### Severity Distribution

| Severity | Count | Category |
|----------|-------|----------|
| CRITICAL | 1 | Structural mismatch: flat files vs declared module hierarchy |
| HIGH | 5 | Anchor NTT silent fallback to dummy values; Insecure relinearize_simple; O(N^2) NTT; RNS mul incomplete (2/3 tests ignored); K-Elimination range limitation ignoring q |
| MEDIUM | 7 | Benchmark summary fabrication; Production 128-bit untested; Production primes not validated; Compiler float contradiction; Float violations in 13/24 files; grover_full.rs duplicate types; FHEConfig source invisible |
| LOW | 4 | Dead code (BenchResult.total_time, VecDeque import); Test-only f64 usage; Duplicate utility functions |

### Innovation Assessment

The build demonstrates a legitimate BFV FHE implementation with several notable architectural choices:

1. **K-Elimination** -- The dual-codex exact division concept is mathematically sound but the implementation is limited by hardcoded small primes with ~80-bit range, and the `_q` parameter is ignored.

2. **Persistent Montgomery** -- Performance is nearly identical to standard Montgomery in benchmarks, suggesting the "zero conversion overhead" claim is trivially true (there was minimal overhead to eliminate).

3. **Shadow Entropy** -- Provides deterministic reproducible randomness suitable for testing but should not be characterized as cryptographically secure.

4. **Dual-Track Exact Arithmetic** -- The concept of maintaining parallel representations for exact reconstruction is valid. The implementation works for trivial ciphertexts (zero c1 component) but the anchor NTT falls back to incorrect values for general polynomials, making it incomplete.

5. **NTT Gen3** -- Correct implementation of negacyclic convolution via psi-twist, but using O(N^2) matrix DFT instead of O(N log N) butterfly.

### Test Coverage

- 140 tests pass (legitimate)
- 4 tests ignored (acknowledged limitations)
- Integration test confirms basic FHE workflow (encrypt -> add/sub/negate/add_plain/mul_plain -> decrypt)
- ct x ct multiplication via degree-2 decrypt works for small products
- ct x ct multiplication via full relin + scaling is tested but correctness is not asserted in all cases (diagnostic tests print results without asserting)
- 10000-iteration zero-decoherence Grover test passes (demonstrating exact arithmetic in quantum simulation)

---

*Audit generated by forensic code auditor. No source files were modified.*
*Build: 13_NINE65_original | Package: qmnf_fhe v0.1.0 | 24 source files, 140 passing tests*

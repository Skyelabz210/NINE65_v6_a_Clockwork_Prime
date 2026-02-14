# FORENSIC AUDIT: 10_QMNF_System

**Build**: `10_QMNF_System`
**Path**: `/home/acid/Projects/Homomorphic_Armada/builds/10_QMNF_System/`
**Audit Date**: 2026-02-14
**Auditor**: Claude Opus 4.6 (forensic code auditor)
**Mandate**: INSPECT, ANALYZE, REPORT -- no source modifications

---

## Table of Contents

1. [Structure Mapping](#1-structure-mapping)
2. [Data Flow Tracing](#2-data-flow-tracing)
3. [Construct Identification](#3-construct-identification)
4. [Wiring Verification](#4-wiring-verification)
5. [Dead Code Detection](#5-dead-code-detection)
6. [Cross-Reference of Dependencies](#6-cross-reference-of-dependencies)
7. [Anomaly Catalogue](#7-anomaly-catalogue)
8. [Summary Verdict](#8-summary-verdict)

---

## 1. Structure Mapping

### 1.1 Workspace Root

**File**: `Cargo.toml` (workspace root)
**Resolver**: `2`
**Workspace version**: `1.0.0`
**Workspace edition**: `2021`
**Minimum Rust**: `1.70`

The workspace contains 9 modular crates organized across 4 layers, plus legacy crates, m2m-tokenizer crates, and standalone extraction crates.

### 1.2 Layer Architecture

```
Layer 0 (Foundation):
  qmnf-primitives          -- HCVLangBigInt (arbitrary-precision signed integer)

Layer 1 (Core Arithmetic):
  qmnf-arithmetic          -- CRT, modular, rational, NNT, Garner, K-Elimination, PLMG, DCBigInt

Layer 2 (Domain Modules):
  qmnf-polynomial          -- Polynomial ring over KFreeCRT coefficients
  qmnf-fhe                 -- Bootstrap-free FHE (AHOP + PLMG variants)
  qmnf-diagnostics         -- Wire protocol, metrics, shadow entropy
  qmnf-neural              -- Montgomery multiplication, RNS vectors, STDP, Ebbinghaus
  qmnf-optimization        -- GSO Swarm optimization, Pade approximation
  qmnf-crypto              -- PQLK key encapsulation, integer hash, symmetric cipher

Layer 3 (Orchestration):
  qmnf-orchestration       -- Task scheduler, memory manager, execution domains
```

### 1.3 Additional Workspace Members (Out of Scope)

- `hcvlang` (legacy)
- `qmnf_crtbigint` (legacy)
- `realtime_fhe` (legacy)
- `standalone_extractions/qmnf-rust-core` (legacy)
- `m2m-tokenizer/crates/m2m-{core,ast,crt,qmnf,protocol,math,cli}` (tokenizer subsystem)
- `standalone_extractions/qmnf-core/{qmnf_bindings,qmnf_fast_ops}` (extraction crates)

### 1.4 Source File Inventory

| Crate | Source Files | Bench Files | Total Modules |
|-------|-------------|-------------|---------------|
| qmnf-primitives | `lib.rs`, `bigint.rs` | `bigint_benchmark.rs` | 1 |
| qmnf-arithmetic | `lib.rs`, `modint.rs`, `crt.rs`, `rational.rs`, `kfree_crt.rs`, `nnt.rs`, `garner.rs`, `k_elimination.rs`, `plmg.rs`, `dcbigint.rs` | `arithmetic_benchmark.rs` | 9 |
| qmnf-polynomial | `lib.rs`, `polynomial.rs` | `polynomial_benchmark.rs` | 1 |
| qmnf-fhe | `lib.rs`, `error.rs`, `params.rs`, `polynomial.rs`, `fhe_ahop.rs`, `fhe_plmg.rs` | `fhe_benchmark.rs` | 5 |
| qmnf-diagnostics | `lib.rs`, `wire_protocol.rs`, `metrics.rs`, `shadow_entropy.rs` | `diagnostics_benchmark.rs` | 3 |
| qmnf-neural | `lib.rs`, `montgomery.rs`, `residue.rs`, `stdp.rs`, `ebbinghaus.rs` | `neural_benchmark.rs` | 4 |
| qmnf-optimization | `lib.rs`, `gso_swarm.rs`, `pade.rs` | none | 2 |
| qmnf-crypto | `lib.rs`, `pqlk.rs`, `hash.rs`, `symmetric.rs` | none | 3 |
| qmnf-orchestration | `lib.rs`, `scheduler.rs`, `memory.rs`, `domain.rs` | none | 3 |
| **TOTALS** | **36 source files** | **6 bench files** | **31 modules** |

---

## 2. Data Flow Tracing

### 2.1 Type Propagation Chain

The foundational type `HCVLangBigInt` flows upward through the architecture:

```
qmnf-primitives::HCVLangBigInt
    |
    v (direct dependency)
qmnf-arithmetic::lib.rs  -->  `pub use qmnf_primitives::HCVLangBigInt;`
    |
    |--- CRTBigInt.to_bigint() -> HCVLangBigInt     (crt.rs)
    |--- Rational uses CRTBigInt internally          (rational.rs)
    |--- KFreeCRT operates independently on u128     (kfree_crt.rs)
    |--- DCBigInt wraps HCVLangBigInt + CRT residues (dcbigint.rs)
    |--- PLMGSystem uses u64 discrete log tables      (plmg.rs)
    |
    v (L1 -> L2 propagation)
qmnf-polynomial  -->  uses KFreeCRT from qmnf-arithmetic
qmnf-fhe         -->  uses ModInt, MERSENNE_PRIME, NNT_MODULUS from qmnf-arithmetic
qmnf-diagnostics -->  uses ModInt from qmnf-arithmetic (for shadow entropy)
qmnf-neural      -->  uses HCVLangBigInt re-exported through qmnf-arithmetic
qmnf-optimization -> uses HCVLangBigInt from BOTH qmnf-primitives AND qmnf-arithmetic
qmnf-crypto      -->  uses HCVLangBigInt from BOTH qmnf-primitives AND qmnf-arithmetic
    |
    v (L2 -> L3 propagation)
qmnf-orchestration --> uses qmnf-arithmetic (for CRTBigInt types)
                   --> uses qmnf-diagnostics (for wire protocol, metrics)
```

### 2.2 Key Data Representations

| Representation | Module | Precision | Range |
|----------------|--------|-----------|-------|
| `HCVLangBigInt` | primitives/bigint.rs | Unlimited | Arbitrary (Vec<u64> limbs) |
| `CRTBigInt` | arithmetic/crt.rs | 126-bit | Two Fibonacci-indexed primes |
| `KFreeCRT` | arithmetic/kfree_crt.rs | Phase-locked | Main + anchor u64 residues |
| `Rational` | arithmetic/rational.rs | Exact | CRTBigInt numerator/denominator |
| `ModInt` | arithmetic/modint.rs | 31-bit | Z/(2^31 - 1) Mersenne field |
| `DCBigInt` | arithmetic/dcbigint.rs | Dual | Lazy-synced CRT + positional |
| `MontgomeryValue` | neural/montgomery.rs | 64-bit | Montgomery form mod p |
| `ResidueVector` | neural/residue.rs | 4-channel | Four ~31-bit primes |
| `FHEPolynomial` | fhe/polynomial.rs | Ring | Z_q[X]/(X^N+1) |

### 2.3 Arithmetic Pipeline

```
User value (u128/i128/i64)
    |
    +---> KFreeCRT::from_u128() ---> phase-locked residues (main[] + anchor[])
    |         |
    |         +---> KFreePolynomial ---> polynomial ring operations
    |         +---> KFreeCRT::divide() ---> exact division (u128 domain)
    |
    +---> CRTBigInt::from_i64() ---> two-prime CRT residues
    |         |
    |         +---> Rational ---> exact p/q with GCD normalization
    |         +---> CRTBigInt::to_bigint() ---> HCVLangBigInt (promotion)
    |
    +---> ModInt::new() ---> Mersenne prime field element
    |         |
    |         +---> Montgomery multiplication (neural)
    |         +---> FHEPolynomial coefficients (fhe)
    |         +---> Shadow entropy hashing (diagnostics)
    |
    +---> HCVLangBigInt::from_i128() ---> unlimited precision
              |
              +---> DCBigInt ---> dual codex with lazy sync
              +---> PLMGSystem ---> discrete log table acceleration
              +---> GarnerContext ---> CRT reconstruction
              +---> KElimContext ---> exact RNS division
```

---

## 3. Construct Identification

### 3.1 Core Mathematical Constructs

| Construct | Location | Algorithm | Complexity |
|-----------|----------|-----------|------------|
| CRT Representation | `crt.rs` | Two Fibonacci-indexed primes | O(1) ops |
| CRT Reconstruction | `crt.rs`, `garner.rs` | Garner's algorithm with precomputed inverses | O(k^2) |
| K-Free Phase Geometry | `kfree_crt.rs` | Phase differential replaces explicit k-tracking | O(1) |
| K-Elimination Division | `k_elimination.rs` | k = (v_A - v_M) * M^(-1) mod A; X = v_M + k*M | O(k) |
| Number Theoretic Transform | `nnt.rs` | Cooley-Tukey butterfly over Fermat prime Q=65537 | O(n log n) |
| PLMG Log Tables | `plmg.rs` | Multi-base discrete log with gearing | O(1) lookup |
| DCBigInt Lazy Sync | `dcbigint.rs` | Dual representation with CodexState enum | Amortized O(1) |
| Montgomery Multiplication | `montgomery.rs` | Newton's method for p_inv; constant-time multiply | O(1) per op |
| Binary GCD | `bigint.rs` | Stein's algorithm (shift-based) | O(n^2) |
| Polynomial Multiplication | `polynomial.rs`, `fhe/polynomial.rs` | Naive O(n^2) + NNT-based O(n log n) fallback | O(n^2) / O(n log n) |
| Horner Evaluation | `polynomial.rs` | Standard Horner's method | O(n) |
| Pade Approximation | `pade.rs` | Rational function p(x)/q(x) for transcendentals | Configurable order |
| GSO Swarm | `gso_swarm.rs` | Galactic Swarm Optimization with phi-harmonic placement | Iteration-based |
| STDP Learning | `stdp.rs` | Spike-Timing Dependent Plasticity with Q16 weights | O(pre * post) |
| Ebbinghaus Decay | `ebbinghaus.rs` | Integer-only forgetting curve with spaced repetition | O(n) per review |

### 3.2 Cryptographic Constructs

| Construct | Location | Description |
|-----------|----------|-------------|
| AHOP FHE | `fhe_ahop.rs` | Attractor-Homomorphic encryption with noise self-stabilization |
| PLMG FHE | `fhe_plmg.rs` | FHE accelerated by discrete log table lookups (generator 3 mod 257) |
| PQLK KEM | `pqlk.rs` | Post-Quantum Lattice Key encapsulation (NTRU-like) |
| IntegerHash | `hash.rs` | Integer-only hash function |
| SymmetricCipher | `symmetric.rs` | Integer-only symmetric encryption |
| FHE Noise Management | `fhe_ahop.rs` | AHOPAttractor enum: FixedPoint, LimitCycle, Torus |

### 3.3 System Constructs

| Construct | Location | Description |
|-----------|----------|-------------|
| TaskScheduler | `scheduler.rs` | BinaryHeap priority queue, operations-budget (not time) |
| MemoryManager | `memory.rs` | Bump allocator with refcounting, aligned allocation |
| ExecutionDomain | `domain.rs` | Multi-domain selection: LinearCPU, SwarmEPRAM, GPU, FPGA, Distributed |
| HelixIndex | `wire_protocol.rs` | 4D addressing (h, sigma, mu, r_domain) with Mersenne modular wrap |
| TraceEvent | `wire_protocol.rs` | Wire-level event representation |
| EMACalculator | `metrics.rs` | Exponential Moving Average with Q16 fixed-point |
| ShadowEntropyEngine | `shadow_entropy.rs` | Entropy measurement via integer hash sequences |
| CDHSHashSequencer | `shadow_entropy.rs` | Deterministic hash-based sequencing |

### 3.4 Integer-Only Enforcement

Every crate in the architecture enforces the integer-only mandate via `#![deny(clippy::float_arithmetic)]` in its `lib.rs`. Additionally, 7 of 9 crates reinforce this at the Cargo.toml level via `[lints.clippy]`:

| Crate | `lib.rs` deny | `Cargo.toml` lint |
|-------|:---:|:---:|
| qmnf-primitives | YES | YES |
| qmnf-arithmetic | YES | YES |
| qmnf-polynomial | YES | YES |
| qmnf-fhe | YES | YES |
| qmnf-diagnostics | YES | YES |
| qmnf-neural | YES | YES |
| **qmnf-optimization** | YES | **MISSING** |
| **qmnf-crypto** | YES | **MISSING** |
| qmnf-orchestration | YES | YES |

**Float Violation Scan Result**: No actual float types (f32, f64) or float arithmetic found in any source file. All grep matches were in comments ("No floating-point") or lint directive lines. The integer-only mandate is fully upheld in code.

---

## 4. Wiring Verification

### 4.1 Dependency Matrix

```
                        prim  arith  poly   fhe   diag  neural  optim  crypto  orch
qmnf-primitives          -     -      -      -     -     -       -      -       -
qmnf-arithmetic          D     -      -      -     -     -       -      -       -
qmnf-polynomial          .     D      -      -     -     -       -      -       -
qmnf-fhe                 .     D      -      -     -     -       -      -       -
qmnf-diagnostics         .     D      -      -     -     -       -      -       -
qmnf-neural              .     D      -      -     -     -       -      -       -
qmnf-optimization        D*    D      -      -     -     -       -      -       -
qmnf-crypto              D*    D      -      -     -     -       -      -       -
qmnf-orchestration       .     D      -      -     D     -       -      -       -

Legend: D = direct dependency, . = transitive only, - = none, D* = ANOMALOUS direct dep
```

### 4.2 External Dependency Map

| Crate | External Dependencies |
|-------|----------------------|
| qmnf-primitives | *none* (std only) |
| qmnf-arithmetic | `serde` |
| qmnf-polynomial | *none* (via qmnf-arithmetic) |
| qmnf-fhe | `serde`, `rand`, `rand_chacha` |
| qmnf-diagnostics | `serde` |
| qmnf-neural | `serde` |
| qmnf-optimization | *none* |
| qmnf-crypto | `serde` |
| qmnf-orchestration | *none* (via qmnf-arithmetic, qmnf-diagnostics) |

### 4.3 Re-export Chain Verification

**qmnf-primitives -> qmnf-arithmetic**: `HCVLangBigInt` is re-exported via `pub use qmnf_primitives::HCVLangBigInt;` in `qmnf-arithmetic/src/lib.rs` line 30. This means all L2 crates can access `HCVLangBigInt` through `qmnf-arithmetic` without needing a direct L0 dependency.

**qmnf-arithmetic -> qmnf-polynomial**: `KFreeCRT`, `KFreeConfig`, and `KFreeCRTResult` are re-exported in `qmnf-polynomial/src/lib.rs` for local use by the polynomial module.

**qmnf-arithmetic -> qmnf-fhe**: `ModInt`, `MERSENNE_PRIME`, and `NNT_MODULUS` are re-exported in `qmnf-fhe/src/lib.rs` line 29.

**qmnf-arithmetic + qmnf-diagnostics -> qmnf-orchestration**: Both are imported. Orchestration uses arithmetic types for CRT-based scheduling and diagnostics for wire protocol/metrics.

### 4.4 Layer Compliance Check

**Expected rule**: Layer N crates should only depend on Layer N-1 (and transitively lower).

| Crate | Layer | Dependencies | Verdict |
|-------|-------|-------------|---------|
| qmnf-primitives | L0 | none | PASS |
| qmnf-arithmetic | L1 | L0 | PASS |
| qmnf-polynomial | L2 | L1 | PASS |
| qmnf-fhe | L2 | L1 | PASS |
| qmnf-diagnostics | L2 | L1 | PASS |
| qmnf-neural | L2 | L1 | PASS |
| qmnf-optimization | L2 | L0 + L1 | **WARN: Layer skip** |
| qmnf-crypto | L2 | L0 + L1 | **WARN: Layer skip** |
| qmnf-orchestration | L3 | L1 + L2 | **WARN: Partial L2 usage** |

### 4.5 L2 Inter-Crate Dependencies

No L2 crate depends on any other L2 crate. All L2 crates are independent siblings. This is clean isolation.

### 4.6 L3 Completeness

`qmnf-orchestration` (L3) depends on only 2 of the 7 lower-layer crates:
- `qmnf-arithmetic` (L1) -- used for type foundations
- `qmnf-diagnostics` (L2) -- used for metrics and wire protocol

**Not wired**: `qmnf-polynomial`, `qmnf-fhe`, `qmnf-neural`, `qmnf-optimization`, `qmnf-crypto`

This means the orchestration layer cannot directly schedule, manage, or coordinate work involving polynomials, FHE, neural networks, optimization, or cryptography. It is an orchestration kernel for arithmetic and diagnostics only.

---

## 5. Dead Code Detection

### 5.1 Crate Substance Assessment

Every crate contains substantive implementations with tests. No empty stubs or placeholder crates were found.

| Crate | Substantive Implementation | Has Tests |
|-------|:---:|:---:|
| qmnf-primitives | YES (full bigint with all ops) | YES |
| qmnf-arithmetic | YES (9 modules, all substantial) | YES |
| qmnf-polynomial | YES (polynomial ops, Horner eval) | YES |
| qmnf-fhe | YES (two FHE systems, full keygen/encrypt/decrypt) | YES |
| qmnf-diagnostics | YES (wire protocol, metrics, shadow entropy) | YES |
| qmnf-neural | YES (Montgomery, RNS vectors, STDP, Ebbinghaus) | YES |
| qmnf-optimization | YES (GSO swarm, Pade approximation) | YES |
| qmnf-crypto | YES (PQLK, hash, symmetric) | YES |
| qmnf-orchestration | YES (scheduler, memory, domains) | YES |

### 5.2 Unused Module Analysis

All modules declared in each `lib.rs` are also exported via `pub use`. No private-only modules that might represent dead internal code were found.

### 5.3 Unreachable Code Paths

No obvious unreachable code was detected. However, the following patterns are notable:

1. **DCBigInt lazy sync**: The `CodexState` enum has three variants (`CrtFresh`, `PositionalFresh`, `BothFresh`). Whether all three states are exercised in practice depends on usage patterns above L1.

2. **AHOP Attractor variants**: `AHOPAttractor::Torus` is defined but its practical usage in real encryption workflows should be verified by integration tests.

3. **ExecutionDomain types**: `DomainType::GPU`, `DomainType::FPGA`, and `DomainType::Distributed` are defined but no GPU/FPGA/distributed backends exist in the current crate tree. These are forward-looking stubs.

### 5.4 Benchmark Files

6 of 9 crates have benchmark files (`benches/*.rs`). The following 3 crates lack benchmarks:
- `qmnf-optimization`
- `qmnf-crypto`
- `qmnf-orchestration`

---

## 6. Cross-Reference of Dependencies

### 6.1 Workspace Dependency Declarations vs Actual Usage

The workspace `Cargo.toml` declares the following shared dependencies:

| Workspace Dep | Used By |
|---------------|---------|
| `serde` | qmnf-arithmetic, qmnf-fhe, qmnf-diagnostics, qmnf-neural, qmnf-crypto |
| `criterion` | qmnf-primitives (dev), and presumably other bench-having crates |
| `syn`, `quote`, `proc-macro2` | m2m-tokenizer crates only (not used by qmnf-* crates) |
| `rayon` | not used by any qmnf-* crate |
| `bincode` | not used by any qmnf-* crate |
| `thiserror` | not used by any qmnf-* crate |
| `anyhow` | not used by any qmnf-* crate |
| `clap` | not used by any qmnf-* crate (m2m-cli only) |
| `log`, `env_logger` | not used by any qmnf-* crate |
| `num-bigint`, `num-traits`, `num-integer` | not used by any qmnf-* crate |
| `pyo3` | standalone_extractions only |

**Observation**: Many workspace-level dependencies exist solely for the m2m-tokenizer and standalone extraction crates. The qmnf-* crates themselves use only `serde`, `rand`, `rand_chacha`, and `criterion`.

### 6.2 Workspace Package Fields vs Actual Crate Metadata

| Field | Workspace Default | qmnf-optimization | qmnf-orchestration |
|-------|------------------|-------------------|---------------------|
| version | `1.0.0` | `1.0.0` (hardcoded) | `1.0.0` (hardcoded) |
| authors | `["QMNF Project"]` | `["Anthony Diaz <founder@hackfate.us>"]` | `["QMNF Team"]` |
| license | `"MIT OR Apache-2.0"` | `"MIT"` | `"MIT"` |
| repository | `github.com/Skyelabz210/QMNF_System` | `github.com/anthonyjdella/qmnf-system` | not specified |

**Anomalies**:
- `qmnf-optimization` uses completely different author and repository metadata
- `qmnf-orchestration` uses `"QMNF Team"` (different from workspace `"QMNF Project"`)
- License mismatch: workspace declares `"MIT OR Apache-2.0"`, several crates override with `"MIT"` only

### 6.3 Workspace vs Local Versioning

| Crate | Uses workspace version? |
|-------|:---:|
| qmnf-primitives | NO (hardcoded `1.0.0`) |
| qmnf-arithmetic | NO (hardcoded `1.0.0`) |
| qmnf-polynomial | NO (hardcoded `1.0.0`) |
| qmnf-fhe | NO (hardcoded `1.0.0`) |
| qmnf-diagnostics | NO (hardcoded `1.0.0`) |
| qmnf-neural | NO (hardcoded `1.0.0`) |
| qmnf-optimization | NO (hardcoded `1.0.0`) |
| qmnf-crypto | **YES** (`version.workspace = true`) |
| qmnf-orchestration | NO (hardcoded `1.0.0`) |

Only `qmnf-crypto` uses workspace-inherited versioning. All others hardcode `1.0.0`.

---

## 7. Anomaly Catalogue

### ANOMALY-001: Layer Skip -- qmnf-optimization depends directly on L0

**Severity**: LOW
**Location**: `crates/qmnf-optimization/Cargo.toml` lines 10-11
**Details**: `qmnf-optimization` (L2) declares a direct dependency on `qmnf-primitives` (L0) in addition to `qmnf-arithmetic` (L1). Since `qmnf-arithmetic` already re-exports `HCVLangBigInt` from `qmnf-primitives`, this direct L0 dependency is redundant.
**Impact**: Architectural impurity. The direct path bypasses the layer abstraction boundary. If `qmnf-arithmetic` ever changed its re-export strategy, `qmnf-optimization` would be unaffected (which might or might not be desired).

### ANOMALY-002: Layer Skip -- qmnf-crypto depends directly on L0

**Severity**: LOW
**Location**: `crates/qmnf-crypto/Cargo.toml` lines 10-11
**Details**: Same pattern as ANOMALY-001. `qmnf-crypto` (L2) depends directly on `qmnf-primitives` (L0) alongside `qmnf-arithmetic` (L1).
**Impact**: Same as ANOMALY-001.

### ANOMALY-003: Missing Cargo.toml lint section in qmnf-optimization

**Severity**: MEDIUM
**Location**: `crates/qmnf-optimization/Cargo.toml`
**Details**: No `[lints.clippy]` section present. The `float_arithmetic = "deny"` lint is only enforced via `#![deny(clippy::float_arithmetic)]` in `lib.rs` (line 25). All other crates have BOTH the Cargo.toml lint AND the lib.rs attribute (except qmnf-crypto, see ANOMALY-004).
**Impact**: The crate-level attribute in `lib.rs` does provide enforcement, but the missing Cargo.toml section creates inconsistency. Cargo.toml-level lints apply to the entire package (including build scripts, examples, and integration tests), while `#![deny(...)]` only applies to the lib target. If this crate ever has build scripts or integration tests, they would not be covered.

### ANOMALY-004: Missing Cargo.toml lint section in qmnf-crypto

**Severity**: MEDIUM
**Location**: `crates/qmnf-crypto/Cargo.toml`
**Details**: Same as ANOMALY-003. No `[lints.clippy]` section. Float denial is only via `#![deny(clippy::float_arithmetic)]` in `lib.rs` (line 25).
**Impact**: Same as ANOMALY-003.

### ANOMALY-005: Metadata divergence in qmnf-optimization

**Severity**: MEDIUM
**Location**: `crates/qmnf-optimization/Cargo.toml` lines 5-8
**Details**:
- `authors = ["Anthony Diaz <founder@hackfate.us>"]` -- differs from workspace `["QMNF Project"]`
- `repository = "https://github.com/anthonyjdella/qmnf-system"` -- differs from workspace `"https://github.com/Skyelabz210/QMNF_System"`
- Does not use `workspace = true` for any metadata field
**Impact**: Suggests this crate was authored or imported from a different source. May indicate code provenance concerns -- the crate may have been integrated from an external contributor's repository without normalizing metadata.

### ANOMALY-006: PLMGSystem name collision across crate boundary

**Severity**: LOW
**Location**:
- `crates/qmnf-arithmetic/src/plmg.rs` line 151: `pub struct PLMGSystem`
- `crates/qmnf-fhe/src/fhe_plmg.rs` line 180: `pub struct PLMGSystem`
**Details**: Two different types with the identical name `PLMGSystem` exist in two different crates. The arithmetic version is a discrete log table system for multiplication/division acceleration. The FHE version is an FHE encryption system that uses PLMG-style log tables internally.
**Impact**: While Rust's module system prevents actual collision (they are in different crate namespaces), this creates confusion for developers. A consumer importing from both crates would need fully qualified paths.

### ANOMALY-007: Incomplete L3 orchestration wiring

**Severity**: MEDIUM
**Location**: `crates/qmnf-orchestration/Cargo.toml` lines 12-14
**Details**: `qmnf-orchestration` (L3) only depends on `qmnf-arithmetic` (L1) and `qmnf-diagnostics` (L2). It does NOT depend on `qmnf-polynomial`, `qmnf-fhe`, `qmnf-neural`, `qmnf-optimization`, or `qmnf-crypto`.
**Impact**: The orchestration kernel can schedule generic tasks and manage memory, but cannot directly orchestrate polynomial operations, FHE encryption, neural network training, swarm optimization, or cryptographic operations. Any integration with those systems must happen above L3, which defeats the purpose of an orchestration layer.

### ANOMALY-008: Architecture diagram describes 5 layers, workspace has 4

**Severity**: LOW
**Location**: `crates/qmnf-primitives/src/lib.rs` lines 17-27
**Details**: The doc comment in qmnf-primitives describes a 5-layer architecture:
```
Layer 0: qmnf-primitives
Layer 1: qmnf-arithmetic
Layer 2: polynomial, fhe, diagnostics, neural, storage
Layer 3: qmnf-crypto, qmnf-orchestration
Layer 4: hcvlang (facade + FFI)
```
However, the workspace Cargo.toml defines only 4 layers (L0-L3) with `qmnf-crypto` at L2 (not L3) and `hcvlang` as a legacy crate (not L4).
**Impact**: Stale documentation. The crate's own description of the architecture does not match the implemented workspace structure. Also references a `storage` crate that does not exist.

### ANOMALY-009: Use of nightly-only `is_multiple_of` on primitive integers

**Severity**: HIGH
**Location**:
- `crates/qmnf-arithmetic/src/kfree_crt.rs` line 311: `dividend.is_multiple_of(divisor_val)` (u128)
- `crates/qmnf-arithmetic/src/plmg.rs` lines 338, 339, 361, 363: `m.is_multiple_of(p)` (HCVLangBigInt)
- `crates/qmnf-diagnostics/src/wire_protocol.rs` line 130: `next_h.is_multiple_of(M)` (u64)
**Details**: The `is_multiple_of` method on primitive integer types (`u64`, `u128`) is a nightly-only feature (tracking issue rust-lang/rust#128101, feature gate `unsigned_is_multiple_of`). No `#![feature(...)]` declarations were found in any crate.
**Impact**: This code will **fail to compile** on stable Rust. The workspace declares `rust-version = "1.70"` (stable), making this a critical inconsistency. Note: The `HCVLangBigInt` calls in `plmg.rs` may be fine if `HCVLangBigInt` implements its own `is_multiple_of` method, but the `u128` and `u64` calls in `kfree_crt.rs` and `wire_protocol.rs` are definitively nightly-only.

### ANOMALY-010: License inconsistency

**Severity**: LOW
**Location**: Workspace `Cargo.toml` vs individual crate Cargo.toml files
**Details**:
- Workspace declares `license = "MIT OR Apache-2.0"` (dual license)
- `qmnf-primitives`, `qmnf-polynomial`, `qmnf-optimization`, `qmnf-orchestration` declare `license = "MIT"` (MIT only)
- `qmnf-arithmetic` declares `license = "MIT"` (MIT only)
- `qmnf-crypto` uses `license.workspace = true` (inherits dual license)
**Impact**: Legal ambiguity. Some crates are MIT-only while the workspace claims dual MIT/Apache-2.0 licensing. This should be harmonized.

### ANOMALY-011: Inconsistent use of workspace inheritance

**Severity**: LOW
**Location**: Individual crate Cargo.toml files
**Details**: Only `qmnf-crypto` uses `version.workspace = true`, `edition.workspace = true`, `rust-version.workspace = true`, `authors.workspace = true`, `license.workspace = true`. All other crates hardcode these fields.
**Impact**: Future version bumps require manual updates across 8 crates instead of a single workspace-level change. Inconsistency increases maintenance burden.

### ANOMALY-012: Missing benchmark targets

**Severity**: LOW
**Location**: `qmnf-optimization`, `qmnf-crypto`, `qmnf-orchestration`
**Details**: These 3 crates have no `benches/` directory or benchmark files, unlike the other 6 crates which all have `*_benchmark.rs` files.
**Impact**: Performance cannot be tracked for optimization algorithms, cryptographic primitives, or orchestration scheduling -- precisely the areas where performance matters most.

### ANOMALY-013: Forward-declared execution domain types without backends

**Severity**: LOW
**Location**: `crates/qmnf-orchestration/src/domain.rs`
**Details**: `DomainType` enum includes `GPU`, `FPGA`, and `Distributed` variants, but no GPU, FPGA, or distributed execution backends exist anywhere in the crate tree.
**Impact**: Code smell. These are aspirational stubs. Not harmful but may confuse users into thinking GPU/FPGA support exists.

---

## 8. Summary Verdict

### Architecture Quality: GOOD

The 10_QMNF_System demonstrates a well-structured layered architecture with clean separation of concerns. The 4-layer hierarchy (foundation, arithmetic, domain modules, orchestration) is logical, and the L2 crates are properly isolated from each other (no horizontal dependencies).

### Critical Findings

| ID | Severity | Summary |
|----|----------|---------|
| ANOMALY-009 | **HIGH** | `is_multiple_of` on u64/u128 requires nightly Rust; workspace declares stable 1.70 |
| ANOMALY-003 | MEDIUM | qmnf-optimization missing Cargo.toml lint section |
| ANOMALY-004 | MEDIUM | qmnf-crypto missing Cargo.toml lint section |
| ANOMALY-005 | MEDIUM | qmnf-optimization has divergent author/repo metadata |
| ANOMALY-007 | MEDIUM | L3 orchestration only wires 1 of 6 L2 crates |

### Strengths

1. **Integer-only mandate universally enforced**: All 9 crates have `#![deny(clippy::float_arithmetic)]` in lib.rs. Zero float violations in source code.
2. **Clean L2 isolation**: No horizontal dependencies between domain modules.
3. **Comprehensive implementations**: No stub or placeholder crates. All 31 modules contain substantive code with tests.
4. **Consistent re-export pattern**: L1 properly re-exports L0 types, enabling clean access chains.
5. **Correct CRT math**: Garner's algorithm, K-Elimination, phase-locked geometry, and NNT all appear mathematically sound in structure.

### Weaknesses

1. **Nightly dependency** via `is_multiple_of` contradicts the declared stable Rust version (1.70).
2. **Incomplete orchestration**: L3 cannot coordinate most L2 domain work.
3. **Metadata inconsistency**: Mixed use of workspace inheritance, divergent authorship.
4. **Two crates bypass L0/L1 boundary** with redundant direct dependencies.
5. **Stale architecture documentation** in qmnf-primitives lib.rs.

### File Inventory Summary

- **Total source files**: 36 (`.rs` in `src/`)
- **Total benchmark files**: 6 (`.rs` in `benches/`)
- **Total crates**: 9
- **Total layers**: 4 (L0 through L3)
- **Total modules**: 31
- **Float violations**: 0
- **Anomalies catalogued**: 13

---

*End of Forensic Audit Report*
*Auditor: Claude Opus 4.6*
*Date: 2026-02-14*
*Classification: INSPECT-ONLY -- No source modifications made*

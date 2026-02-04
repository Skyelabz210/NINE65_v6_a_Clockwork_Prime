# EXECUTION PLAN ADDENDUM v1.1
## Addressing Gaps in Original Plan

Generated: 2025-12-11
Status: REQUIRED - Integrate into main execution plan

---

## ADDENDUM A: HCVLang Migration

### A.1 HCVLang Crate Structure

The 800,000+ line HCVLang programming language needs dedicated migration:

```
QMNF_System/
├── hcvlang/                        # HCVLang ecosystem (NEW ZONE)
│   ├── hcvlang-core/              # Core language primitives
│   │   ├── src/
│   │   │   ├── types/             # Integer-only type system
│   │   │   ├── parser/            # HCVLang parser
│   │   │   ├── ast/               # Abstract syntax tree
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── hcvlang-compiler/          # HCVLang → Rust transpiler
│   │   ├── src/
│   │   │   ├── codegen/           # Rust code generation
│   │   │   ├── optimizer/         # Integer-preserving optimizations
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   ├── hcvlang-runtime/           # Runtime support
│   │   ├── src/
│   │   │   ├── memory/            # Integer-only memory management
│   │   │   ├── ffi/               # Foreign function interface
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   │
│   └── hcvlang-stdlib/            # Standard library
│       ├── src/
│       │   ├── math/              # Uses innovations/
│       │   ├── crypto/            # Uses innovations/ahop, etc.
│       │   ├── collections/       # Integer-indexed collections
│       │   └── lib.rs
│       └── Cargo.toml
```

### A.2 HCVLang Tasks

| ID | Task | Duration | Dependencies |
|----|------|----------|--------------|
| H-01 | Create hcvlang/ directory structure | 1 day | T-04 |
| H-02 | Migrate type system (integer-only) | 3 days | H-01 |
| H-03 | Migrate parser and AST | 3 days | H-02 |
| H-04 | Migrate compiler/transpiler | 5 days | H-03 |
| H-05 | Migrate runtime | 3 days | H-04 |
| H-06 | Migrate stdlib (link to innovations/) | 5 days | H-05, T-10 |
| H-07 | **GATE:** HCVLang compiles sample programs | — | H-06 |

### A.3 HCVLang Protection Level

| Component | Protection | Ships |
|-----------|------------|-------|
| hcvlang-core | PROTECTED | Premium licensees |
| hcvlang-compiler | PROTECTED | Premium licensees |
| hcvlang-runtime | ENGINE | Premium licensees |
| hcvlang-stdlib | ENGINE | Premium licensees |

---

## ADDENDUM B: Complete FHE Variant Enumeration

### B.1 All 8 FHE Products

Each FHE variant needs explicit crate and migration tasks:

| Product | Crate Name | Key Innovation | Interface |
|---------|------------|----------------|-----------|
| FHE-AHOP | `products/fhe-ahop` | Apollonian orbits | `fhe-api` |
| FHE-BFV | `products/fhe-bfv` | Ring-LWE + CRT | `fhe-api` |
| FHE-CGM | `products/fhe-cgm` | Codex Gear Manifold | `fhe-api` |
| FHE-CGM-Dual | `products/fhe-cgm-dual` | DCBigInt | `fhe-api` |
| FHE-Realtime | `products/fhe-realtime` | Coprime Anchor | `fhe-api` |
| FHE-PLMG | `products/fhe-plmg` | Phase-locked geometry | `fhe-api` |
| FHE-Swarm | `products/fhe-swarm` | GSO attractors | `fhe-api` |
| FHE-PQLK | `products/fhe-pqlk` | 5-algorithm hybrid | `fhe-api` + `crypto-api` |

### B.2 FHE Tasks

| ID | Task | Notes |
|----|------|-------|
| F-01 | Create fhe-api interface trait | Encrypt, decrypt, add, multiply, relinearize |
| F-02 | Migrate FHE-BFV implementation | Base implementation |
| F-03 | Migrate FHE-AHOP implementation | Uses innovations/ahop |
| F-04 | Migrate FHE-CGM implementation | Uses innovations/codex-gear-manifold |
| F-05 | Migrate FHE-CGM-Dual implementation | Uses innovations/dual-codex |
| F-06 | Migrate FHE-Realtime implementation | Uses innovations/coprime-anchor |
| F-07 | Migrate FHE-PLMG implementation | Uses PLMG visualization |
| F-08 | Migrate FHE-Swarm implementation | Uses GSO dynamics |
| F-09 | Migrate FHE-PQLK implementation | Combines all 5 algorithms |
| F-10 | **GATE:** All 8 FHE variants pass test suite | — |

### B.3 FHE Benchmarks Required

| Variant | Encrypt | Multiply | Decrypt | Baseline |
|---------|---------|----------|---------|----------|
| FHE-BFV | <10ms | <50ms | <5ms | SEAL |
| FHE-Realtime | <2ms | <5ms | <1ms | SEAL |
| FHE-CGM | <8ms | <40ms | <4ms | SEAL |
| FHE-AHOP | <15ms | <60ms | <8ms | Lattice-based |

---

## ADDENDUM C: Cryptography Products

### C.1 AHOP (Apollonian Hidden Orbit Problem)

```
innovations/ahop/
├── src/
│   ├── orbit.rs           # Apollonian orbit computation
│   ├── keygen.rs          # 130μs keygen
│   ├── encrypt.rs         # Orbit-based encryption
│   ├── decrypt.rs         # Orbit recovery
│   └── lib.rs
└── Cargo.toml
```

**Claimed Metrics:**
- Key size: 128-256 bytes
- Keygen: 130μs
- No known attacks (18 months)

### C.2 PQLK (Post-Quantum Locking Key)

```
products/pqlk/
├── src/
│   ├── hybrid.rs          # 5-algorithm combination
│   ├── ahop_layer.rs      # AHOP component
│   ├── kyber_layer.rs     # Kyber (ML-KEM)
│   ├── dilithium_layer.rs # Dilithium (ML-DSA)
│   ├── sphincs_layer.rs   # SPHINCS+
│   ├── hqc_layer.rs       # HQC
│   └── lib.rs
└── Cargo.toml
```

**Security Guarantee:** Attacker must break ALL FIVE algorithms.

### C.3 Crypto Tasks

| ID | Task | Notes |
|----|------|-------|
| C-01 | Create crypto-api interface trait | Sign, verify, encrypt, decrypt, keygen |
| C-02 | Migrate AHOP to innovations/ahop | pub(crate) only |
| C-03 | Create PQLK product | Links all 5 algorithms |
| C-04 | Benchmark AHOP keygen (target: 130μs) | — |
| C-05 | **GATE:** PQLK passes security test suite | — |

---

## ADDENDUM D: Complete Innovation Migration

### D.1 Full Innovation List (71+)

Organized by generation for migration order:

#### Generation 0 (Seeds) - Already embedded
- QMNF Philosophy (no code, just principles)
- Integer Primacy
- CRT Foundation
- φ Anchor

#### Generation 1 (8 innovations)
| Innovation | Source | Target | Priority |
|------------|--------|--------|----------|
| QMNFRational | hcvlang/src/rational.rs | innovations/qmnf-rational/ | HIGH |
| Binary GCD | hcvlang/src/gcd.rs | innovations/binary-gcd/ | HIGH |
| Barrett Reduction | hcvlang/src/barrett.rs | innovations/barrett/ | HIGH |
| Fixed-Point | hcvlang/src/fixed.rs | innovations/fixed-point/ | MEDIUM |
| Extended GCD | hcvlang/src/egcd.rs | innovations/extended-gcd/ | MEDIUM |

#### Generation 2 (6 innovations)
| Innovation | Source | Target | Priority |
|------------|--------|--------|----------|
| BigInt | hcvlang/src/bigint.rs | innovations/bigint-core/ | HIGH |
| RNS | hcvlang/src/rns.rs | innovations/rns-core/ | HIGH |
| Integer Trig | hcvlang/src/trig.rs | innovations/integer-trig/ | MEDIUM |
| Modular Inverse | hcvlang/src/modinv.rs | innovations/modular-inverse/ | HIGH |

#### Generation 3 (10 innovations)
| Innovation | Source | Target | Priority |
|------------|--------|--------|----------|
| CRTBigInt | hcvlang/src/crt_bigint.rs | innovations/crt-bigint/ | CRITICAL |
| K-Elimination | hcvlang/src/k_elim.rs | innovations/k-elimination/ | CRITICAL |
| NTT | hcvlang/src/ntt.rs | innovations/ntt/ | HIGH |
| Garner's Algorithm | hcvlang/src/garner.rs | innovations/garner/ | HIGH |
| Fused Piggyback | hcvlang/src/fpd.rs | innovations/fused-piggyback/ | HIGH |
| Montgomery Mul | hcvlang/src/montgomery.rs | innovations/montgomery/ | CRITICAL |

#### Generation 4 (8 innovations)
| Innovation | Source | Target | Priority |
|------------|--------|--------|----------|
| Codex Gear Manifold | hcvlang/src/cgm.rs | innovations/codex-gear-manifold/ | HIGH |
| Shadow Entropy | hcvlang/src/shadow.rs | innovations/shadow-entropy/ | CRITICAL |
| Entropy Accounting | hcvlang/src/entropy_acct.rs | innovations/entropy-accounting/ | MEDIUM |
| WASSAN | hcvlang/src/wassan.rs | innovations/wassan/ | MEDIUM |
| Rails-and-Voids | hcvlang/src/rails.rs | innovations/rails-and-voids/ | LOW |

#### Generation 5 (8 innovations)
| Innovation | Source | Target | Priority |
|------------|--------|--------|----------|
| DCBigInt | hcvlang/src/dcbigint.rs | innovations/dual-codex/ | CRITICAL |
| Persistent Montgomery | hcvlang/src/persistent_mont.rs | innovations/persistent-montgomery/ | HIGH |
| Integer Neural Nets | hcvlang/src/neural/ | innovations/integer-neural/ | HIGH |
| Neuromorphic Framework | hcvlang/src/neuromorphic/ | innovations/neuromorphic/ | MEDIUM |
| Coprime-Anchor FHE | hcvlang/src/coprime_anchor.rs | innovations/coprime-anchor/ | HIGH |

#### Generation 6 (15+ innovations)
| Innovation | Source | Target | Priority |
|------------|--------|--------|----------|
| AHOP | hcvlang/src/ahop/ | innovations/ahop/ | CRITICAL |
| PLMG | hcvlang/src/plmg.rs | innovations/plmg/ | HIGH |
| PQLK Core | hcvlang/src/pqlk/ | innovations/pqlk-core/ | HIGH |
| GSO Swarm | hcvlang/src/gso.rs | innovations/gso-swarm/ | MEDIUM |

### D.2 Innovation Dependency Graph

```
Generation 0 (Seeds)
    │
    ├──► Generation 1
    │       │
    │       ├──► QMNFRational
    │       ├──► Binary GCD
    │       ├──► Barrett Reduction
    │       └──► Extended GCD
    │               │
    │               ▼
    └──► Generation 2
            │
            ├──► BigInt ◄─────────┐
            ├──► RNS ◄────────────┼──► Generation 3
            └──► Modular Inverse ─┘       │
                                          ├──► CRTBigInt (BigInt + RNS + Barrett)
                                          ├──► K-Elimination (RNS + Phase)
                                          ├──► NTT (RNS + Barrett)
                                          ├──► Montgomery (Modular Inverse)
                                          │
                                          ▼
                                    Generation 4
                                          │
                                          ├──► Codex Gear Manifold (CRTBigInt)
                                          ├──► Shadow Entropy (RNS + Thermodynamics)
                                          ├──► DCBigInt (Codex × 2 + CRTBigInt)
                                          │
                                          ▼
                                    Generation 5
                                          │
                                          ├──► AHOP (DCBigInt + φ Anchor)
                                          ├──► Integer Neural (CRTBigInt + Padé)
                                          │
                                          ▼
                                    Generation 6+
                                          │
                                          └──► All FHE variants, PQLK, Products
```

**Migration must respect this order!**

---

## ADDENDUM E: Tiered Licensing Implementation

### E.1 Tier Definitions

| Tier | Name | Ships | Price Point |
|------|------|-------|-------------|
| 1 | Binary | Compiled binary + interfaces/ source + docs | $ |
| 2 | Source-Limited | Tier 1 + engines/ source | $$ |
| 3 | Source-Licensed | Tier 2 + SELECTED innovations/ source | $$$ |
| 4 | Full License | Everything (strategic partners) | $$$$ |

### E.2 Packager Tool Specification

```rust
// tools/packager/src/lib.rs

pub enum LicenseTier {
    Binary,
    SourceLimited,
    SourceLicensed { innovations: Vec<String> },
    FullLicense,
}

pub struct PackageConfig {
    pub product: String,           // e.g., "fhe-bfv"
    pub tier: LicenseTier,
    pub target: Target,            // e.g., x86_64-linux
    pub strip_symbols: bool,
    pub include_docs: bool,
}

pub fn package(config: PackageConfig) -> Result<PackageOutput, PackageError> {
    // 1. Build release binary
    // 2. Collect appropriate sources based on tier
    // 3. Generate license file
    // 4. Verify no innovation leaks
    // 5. Create distributable archive
}
```

### E.3 Packager Tasks

| ID | Task | Notes |
|----|------|-------|
| P-01 | Define PackageConfig struct | Support all 4 tiers |
| P-02 | Implement binary packaging | Strip symbols, optimize |
| P-03 | Implement source selection | Respect tier boundaries |
| P-04 | Implement license generation | Embed tier + restrictions |
| P-05 | Implement leak detection | Verify no innovation exposure |
| P-06 | **GATE:** Package all products at all tiers | — |

---

## ADDENDUM F: Python Bindings

### F.1 PyO3 Structure

```
QMNF_System/
├── bindings/
│   └── python/
│       ├── qmnf-py/              # Main Python package
│       │   ├── src/
│       │   │   ├── lib.rs        # PyO3 module root
│       │   │   ├── bigint.rs     # BigInt bindings
│       │   │   ├── rational.rs   # Rational bindings
│       │   │   ├── fhe.rs        # FHE bindings
│       │   │   └── crypto.rs     # Crypto bindings
│       │   ├── Cargo.toml
│       │   └── pyproject.toml    # Maturin config
│       │
│       └── tests/
│           ├── test_bigint.py
│           ├── test_fhe.py
│           └── conftest.py
```

### F.2 Python Tasks

| ID | Task | Notes |
|----|------|-------|
| Y-01 | Set up Maturin build | pyproject.toml configuration |
| Y-02 | Create BigInt Python wrapper | Wrap precision-lib |
| Y-03 | Create FHE Python wrapper | Wrap fhe-* products |
| Y-04 | Create Crypto Python wrapper | Wrap AHOP/PQLK |
| Y-05 | Python test suite | pytest + hypothesis |
| Y-06 | **GATE:** `pip install qmnf` works | — |

---

## ADDENDUM G: Missing Products

### G.1 DetermiOS Runtime

```
products/determios/
├── src/
│   ├── interceptor.rs     # Float operation interception
│   ├── normalizer.rs      # Convert to CRTBigInt
│   ├── runtime.rs         # Execution environment
│   └── lib.rs
└── Cargo.toml
```

**Purpose:** Intercept float operations at runtime, normalize to CRTBigInt, achieve 40-60% energy savings.

### G.2 Integer AI Training Pipeline

```
products/neural-int/
├── src/
│   ├── layers/
│   │   ├── dense.rs       # Integer dense layer
│   │   ├── conv.rs        # Integer convolution
│   │   └── attention.rs   # Integer attention
│   ├── optimizers/
│   │   └── adam_int.rs    # Integer Adam
│   ├── activations/
│   │   ├── gelu.rs        # Padé GELU
│   │   ├── relu.rs        # Integer ReLU
│   │   └── sigmoid.rs     # Padé Sigmoid
│   ├── training/
│   │   ├── backprop.rs    # Integer backprop
│   │   └── ebbinghaus.rs  # Forgetting curve
│   └── lib.rs
└── Cargo.toml
```

### G.3 Additional Product Tasks

| ID | Task | Notes |
|----|------|-------|
| X-01 | Create DetermiOS runtime | Float interception |
| X-02 | Create Integer Neural product | Full training pipeline |
| X-03 | AtomSpace integration | HD vectors, 8192-D |
| X-04 | CUDA acceleration wrapper | GPU kernels |
| X-05 | **GATE:** Train MNIST without floats | — |

---

## ADDENDUM H: Crate Dependency Matrix

### H.1 Innovation Dependencies

| Crate | Depends On |
|-------|------------|
| `k-elimination` | `rns-core` |
| `montgomery` | `modular-inverse`, `barrett` |
| `crt-bigint` | `bigint-core`, `rns-core`, `barrett`, `k-elimination` |
| `dual-codex` | `codex-gear-manifold`, `crt-bigint` |
| `shadow-entropy` | `rns-core` |
| `ahop` | `dual-codex`, `phi-anchor` |

### H.2 Engine Dependencies

| Engine | Innovations Used |
|--------|------------------|
| `crt-engine` | `crt-bigint`, `k-elimination`, `montgomery`, `binary-gcd` |
| `precision-engine` | `qmnf-rational`, `crt-bigint`, `k-elimination` |
| `fhe-engine` | `crt-bigint`, `ntt`, `montgomery`, `shadow-entropy` |
| `crypto-engine` | `ahop`, `shadow-entropy`, `binary-gcd` |
| `neural-engine` | `crt-bigint`, `integer-trig`, `pade-approximants` |

### H.3 Product Dependencies

| Product | Interfaces | Engines |
|---------|------------|---------|
| `precision-lib` | `bigint-api`, `rational-api` | `precision-engine` |
| `fhe-bfv` | `fhe-api` | `fhe-engine` |
| `fhe-realtime` | `fhe-api` | `fhe-engine`, `crt-engine` |
| `pqlk` | `crypto-api` | `crypto-engine` |
| `neural-int` | `neural-api` | `neural-engine` |

---

## REVISED TASK COUNT

| Category | Original | Addendum | New Total |
|----------|----------|----------|-----------|
| Structure (Phase 1) | 4 | 0 | 4 |
| Standards (Phase 2) | 4 | 0 | 4 |
| Visibility (Phase 3) | 5 | 0 | 5 |
| Documentation (Phase 4) | 3 | 0 | 3 |
| Benchmarks (Phase 5) | 2 | 0 | 2 |
| Testing (Phase 6) | 3 | 0 | 3 |
| CI/CD (Phase 7) | 3 | 0 | 3 |
| **HCVLang (NEW)** | 0 | 7 | 7 |
| **FHE Variants (NEW)** | 0 | 10 | 10 |
| **Cryptography (NEW)** | 0 | 5 | 5 |
| **Packaging (NEW)** | 0 | 6 | 6 |
| **Python (NEW)** | 0 | 6 | 6 |
| **Products (NEW)** | 0 | 5 | 5 |
| **TOTAL** | 24 | 39 | **63 tasks** |

---

## REVISED TIMELINE

| Phase | Duration | Tasks |
|-------|----------|-------|
| 1-2: Structure + Standards | 4 days | 8 |
| 3: Visibility + Innovation Migration | 10 days | 5 + full innovation list |
| HCVLang Migration | 10 days | 7 |
| FHE Variants | 8 days | 10 |
| Cryptography | 4 days | 5 |
| 4-5: Docs + Benchmarks | 6 days | 5 |
| 6-7: Testing + CI | 6 days | 6 |
| Packaging + Python | 6 days | 12 |
| Products | 4 days | 5 |
| **TOTAL** | **~58 days** | **63 tasks** |

---

## CONCLUSION

The original plan was a **solid foundation** for infrastructure (standards, CI, protection) but was **incomplete** for actual migration. This addendum addresses:

1. ✅ HCVLang (800K+ lines)
2. ✅ All 8 FHE variants
3. ✅ AHOP and PQLK cryptography
4. ✅ Complete innovation migration (71+)
5. ✅ Tiered licensing implementation
6. ✅ Python bindings
7. ✅ Crate dependency matrix
8. ✅ Missing products (DetermiOS, Neural)

**Integrate this addendum before execution begins.**

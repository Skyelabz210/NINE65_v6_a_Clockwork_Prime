---
title: "Repository Separation Plan"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/REPOSITORY_SEPARATION_PLAN.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF System Repository Separation Plan

**Strategic Refactoring: Monolith → Layered Architecture**

Total System Size: ~203,000 lines (71K Python, 61K Rust, 71K docs)

---

## **Architectural Layers**

```
┌─────────────────────────────────────────────────────────────┐
│  Layer 5: Cognitive & Temporal Systems                      │
│  - CylindricalTime, Consciousness, DET_SEQ                  │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────────┐
│  Layer 4: HoloDrive / Hyperdimensional Computing (Multi-repo)│
│  - COSMOS Backend, Wasan HD, HoloDrive v3                   │
│  - Decanal-Cylindrical, VSA, 3-Tier Memory                  │
│  - Phase Locking, Harmonic Resonance                        │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────────┐
│  Layer 3: MAA (Post-Quantum Crypto) - Separate Repo         │
│  - MAA Double Helix, Hidden Orbit Problem                   │
│  - MAA KEM, MAA Signatures, Error Correction                │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────────┐
│  Layer 2: FHE (Fully Homomorphic Encryption) - Separate Repo│
│  - BFV/CKKS Schemes, Polynomial Ring Operations             │
│  - RNS (Residue Number System), Noise Management            │
│  - Key Generation, Encryption, Homomorphic Ops              │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────────┐
│  Layer 1: HCVLang Math Foundation - BASE REPOSITORY          │
│  - QMNFRational (exact rational arithmetic)                 │
│  - Integer-only primitives (add, mul, div, mod, gcd)        │
│  - Modular arithmetic, number theory, primes                │
│  - Geometric primitives, combinatorics                      │
│  - NO FLOATS - 100% integer operations                      │
└─────────────────────────────────────────────────────────────┘
```

---

## **Repository Breakdown**

### **REPO 1: `hcvlang-math` (Foundation Layer)**

**Purpose**: Pure mathematical foundation - zero dependencies
**Size Estimate**: ~25K lines (Rust) + 15K lines (Python bindings)
**License**: MIT or Apache 2.0 (permissive for max adoption)

**What Goes Here:**
```
hcvlang-math/
├── src/
│   ├── rational.rs              # QMNFRational core
│   ├── integer_ops.rs           # Add, mul, div, mod, gcd
│   ├── modular.rs               # Modular arithmetic (mod_add, mod_mul, mod_inv)
│   ├── number_theory.rs         # Primes, factorization, CRT
│   ├── combinatorics.rs         # Binomial, factorial, permutations
│   ├── geometry/
│   │   ├── primitives.rs        # Point2D, Vector operations
│   │   ├── apollonian.rs        # Apollonian gaskets
│   │   └── spatial_grid.rs      # Spatial indexing
│   └── lib.rs
├── python/
│   ├── hcvlang_math/            # Python bindings via PyO3
│   │   ├── __init__.py
│   │   ├── rational.py
│   │   └── ops.py
│   └── setup.py
├── benches/                     # Criterion benchmarks
├── tests/                       # Comprehensive test suite
├── examples/                    # Usage examples
└── docs/                        # API documentation

Key Files to Extract:
- hcvlang/src/rational.rs
- hcvlang/src/math/*.rs (all)
- hcvlang/core/math/*.rs (all)
- hcvlang/src/intpair.rs
- hcvlang/src/modint*.rs
- hcvlang/src/geom_point2d.rs
- hcvlang/core/geometry/*.rs
- qmnf_optimized_rational.py
- qmnf/unified_qmnf.py (core math only)
```

**Dependencies**: ZERO (only std Rust, no external crates)
**Exports**:
- `QMNFRational` struct
- Integer arithmetic traits
- Modular arithmetic functions
- Number theory utilities

**Release Target**: v1.0.0 - Production ready, stable API

---

### **REPO 2: `hcvlang-fhe` (Fully Homomorphic Encryption)**

**Purpose**: FHE schemes built on HCVLang math
**Size Estimate**: ~15K lines (Rust) + 5K lines (Python)
**License**: MIT/Apache 2.0

**What Goes Here:**
```
hcvlang-fhe/
├── src/
│   ├── polynomial.rs            # Polynomial ring operations
│   ├── rns.rs                   # Residue Number System
│   ├── noise.rs                 # Noise sampling & management
│   ├── bfv/
│   │   ├── params.rs            # BFV parameters
│   │   ├── encrypt.rs           # Encryption/decryption
│   │   ├── keys.rs              # Key generation
│   │   └── operations.rs        # Homomorphic ops
│   ├── ckks/                    # (Future) CKKS scheme
│   └── lib.rs
├── python/                      # Python bindings
├── benches/
├── tests/
└── examples/

Key Files to Extract:
- hcvlang/src/fhe/*.rs (all)
- tests/python/fhe_*.py
- tests/python/acc_bfv_toy.py
- gso_noise_gen.py (noise generation)
- qmnf/crypto/acc/cyl_time_acc_noise.py
- qmnf/crypto/acc/cyl_time_acc_gaussian.py
```

**Dependencies**:
- `hcvlang-math` (math foundation)

**Exports**:
- BFV encryption scheme
- Noise generators
- Polynomial operations
- RNS utilities

**Release Target**: v0.8.0 - Beta (needs security audit)

---

### **REPO 3: `maa-crypto` (Post-Quantum Candidate)**

**Purpose**: MAA (Möbius Attractor Accumulator) post-quantum cryptography
**Size Estimate**: ~12K lines (Rust) + 8K lines (Python)
**License**: Proprietary or research-only (patent considerations)

**What Goes Here:**
```
maa-crypto/
├── src/
│   ├── core.rs                  # MAA core algorithm
│   ├── double_helix.rs          # Dual-lane execution
│   ├── kem.rs                   # Key encapsulation mechanism
│   ├── signatures.rs            # Signature scheme
│   ├── ecc.rs                   # Error correction codes
│   └── lib.rs
├── python/
├── security/
│   ├── proofs/                  # Security proofs
│   └── audit_reports/
├── benches/
├── tests/
│   ├── kat_tests.rs             # Known-answer tests (NIST)
│   └── security_tests.rs
└── docs/
    ├── whitepaper.pdf
    └── nist_submission/

Key Files to Extract:
- hcvlang/crypto/maa/*.rs (all)
- hcvlang/execution/double_helix/*.rs
- holodrive_phase2/maa_double_helix.py
- qmnf/execution/maa_lane.py
- docs/api/maa_*.md
```

**Dependencies**:
- `hcvlang-math` (math foundation)

**Exports**:
- MAA encryption/decryption
- Key generation
- Signature schemes
- Double helix execution engine

**Release Target**: v0.5.0 - Research prototype (pre-NIST submission)

---

### **REPO 4A: `holodrive-core` (Holographic Storage Foundation)**

**Purpose**: Core holographic storage primitives
**Size Estimate**: ~10K lines (Rust) + 8K lines (Python)

**What Goes Here:**
```
holodrive-core/
├── src/
│   ├── holographic_storage.rs   # Core holographic operations
│   ├── interference_patterns.rs # Wave interference simulation
│   ├── reconstruction.rs        # Pattern reconstruction
│   └── lib.rs
├── python/
├── tests/
└── examples/

Key Files to Extract:
- hcvlang/memory/holographic/*.rs
- qmnf/storage/holodrive/holohd_refined_v3.py
- qmnf/real_holographic_storage.py
```

**Dependencies**:
- `hcvlang-math`

**Release Target**: v0.7.0 - Beta

---

### **REPO 4B: `hdc-vsa` (Hyperdimensional Computing / VSA)**

**Purpose**: Vector Symbolic Architectures & hypervector operations
**Size Estimate**: ~8K lines (Rust) + 6K lines (Python)

**What Goes Here:**
```
hdc-vsa/
├── src/
│   ├── hypervector.rs           # Hypervector type (10K+ dims)
│   ├── binding.rs               # Binding operations
│   ├── bundling.rs              # Bundling/superposition
│   ├── similarity.rs            # Hamming distance, cosine sim
│   └── lib.rs
├── python/
├── tests/
└── examples/

Key Files to Extract:
- qmnf/vsa/hdc_integration.py
- hcvlang/src/int_vector.rs
- hcvlang/memory/attractor/*.rs (if VSA-related)
```

**Dependencies**:
- `hcvlang-math`

**Release Target**: v0.6.0 - Alpha

---

### **REPO 4C: `cosmos-memory` (COSMOS Backend)**

**Purpose**: COSMOS hypervector memory system
**Size Estimate**: ~6K lines (Rust) + 5K lines (Python)

**What Goes Here:**
```
cosmos-memory/
├── src/
│   ├── substrate.rs             # COSMOS substrate
│   ├── addressing.rs            # 144D addressing
│   ├── storage.rs               # Storage operations
│   └── lib.rs
├── python/
├── tests/
└── examples/

Key Files to Extract:
- hcvlang/memory/cosmos/*.rs
- qmnf/storage/cosmos_backend.py
- qmnf/storage/cosmos/wasan_cosmos_backend.py
```

**Dependencies**:
- `hcvlang-math`
- `hdc-vsa` (hypervector operations)

**Release Target**: v0.6.0 - Alpha

---

### **REPO 4D: `holohd-unified` (Integrated Holographic+Decanal)**

**Purpose**: Unified holographic storage with harmonic architecture
**Size Estimate**: ~8K lines (Python primarily)

**What Goes Here:**
```
holohd-unified/
├── src/
│   ├── decanal_cylindrical.py   # 10-segment harmonic storage
│   ├── phase_locking.py         # Phase lock loops
│   ├── harmonic_resonance.py    # Temporal harmonic resonance
│   └── integration.py           # HoloDrive + Decanal integration
├── tests/
└── examples/

Key Files to Extract:
- qmnf/storage/decanal_cylindrical_architecture.py
- qmnf/storage/holohd_decanal_integrated.py
- qmnf/harmonic_primitives.py
```

**Dependencies**:
- `hcvlang-math`
- `holodrive-core`

**Release Target**: v0.5.0 - Alpha

---

### **REPO 5: `cylindrical-time` (Temporal System)**

**Purpose**: CylindricalTime multi-cycle temporal system
**Size Estimate**: ~5K lines (Rust) + 8K lines (Python)

**What Goes Here:**
```
cylindrical-time/
├── src/
│   ├── cycles.rs                # mod365, mod260, TCO-phi cycles
│   ├── accumulator.rs           # CMIX accumulator
│   ├── signatures.rs            # Time signatures
│   └── lib.rs
├── python/
├── tests/
└── examples/

Key Files to Extract:
- qmnf/crypto/acc/cyl_time_acc_cmix.py
- qmnf/phase_lock_tco.py
- hcvlang/temporal/*.rs
```

**Dependencies**:
- `hcvlang-math`

**Release Target**: v0.7.0 - Beta

---

### **REPO 6: `qmnf-cognitive` (Consciousness & Neural Systems)**

**Purpose**: Cognitive architectures and consciousness emergence
**Size Estimate**: ~12K lines (Python) + 6K lines (Rust)

**What Goes Here:**
```
qmnf-cognitive/
├── src/
│   ├── harmonic_consciousness.py # 36-segment consciousness
│   ├── det_seq_engine.py         # Deterministic sequencing
│   ├── gso.py                    # Swarm optimization
│   └── neural/
│       ├── helix_compiler.py     # Neural compiler
│       └── primitives.rs         # Neural primitives
├── tests/
└── examples/

Key Files to Extract:
- qmnf/cognitive/harmonic_consciousness.py
- qmnf/frameworks/sequences/det_seq_engine.py
- qmnf/frameworks/sequences/mana_sequence_engine.py
- qmnf/neural/*.py
- hcvlang/execution/neural/*.rs
- hcvlang/execution/swarm/*.rs (GSO)
```

**Dependencies**:
- `hcvlang-math`
- `cylindrical-time`
- `holohd-unified` (for memory)

**Release Target**: v0.4.0 - Pre-alpha (research)

---

### **REPO 7: `qmnf-orchestration` (MANA Kernel)**

**Purpose**: System orchestration and resource management
**Size Estimate**: ~5K lines (Rust) + 4K lines (Python)

**What Goes Here:**
```
qmnf-orchestration/
├── src/
│   ├── mana_kernel.rs           # MANA orchestration
│   ├── resource_management.rs   # Resource allocation
│   ├── scheduling.rs            # Task scheduling
│   └── lib.rs
├── python/
├── tests/
└── examples/

Key Files to Extract:
- hcvlang/orchestration/mana/*.rs
- qmnf/cosmos_mana/integration.py
```

**Dependencies**:
- `hcvlang-math`
- All Layer 4 repos (orchestrates memory systems)

**Release Target**: v0.5.0 - Alpha

---

## **Migration Strategy**

### **Phase 1: Foundation (Week 1-2)**
1. **Extract `hcvlang-math`** (Layer 1)
   - Create new repo
   - Extract core math files
   - Set up CI/CD (tests, benchmarks, docs)
   - Publish v1.0.0-rc1 to crates.io
   - **Goal**: Stable, production-ready math library

### **Phase 2: Crypto Layers (Week 3-4)**
2. **Extract `hcvlang-fhe`** (Layer 2)
   - Depends on hcvlang-math via Cargo.toml
   - Extract FHE implementations
   - Security audit checklist
   - Publish v0.8.0-beta

3. **Extract `maa-crypto`** (Layer 3)
   - Depends on hcvlang-math
   - Extract MAA files
   - Prepare NIST submission materials
   - Publish v0.5.0-alpha

### **Phase 3: Memory Systems (Week 5-6)**
4. **Extract Layer 4 repos** (parallel work):
   - `holodrive-core`
   - `hdc-vsa`
   - `cosmos-memory`
   - `holohd-unified`
   - All depend on hcvlang-math
   - Publish v0.5.0-v0.7.0 alphas/betas

### **Phase 4: Higher Layers (Week 7-8)**
5. **Extract remaining repos**:
   - `cylindrical-time`
   - `qmnf-cognitive`
   - `qmnf-orchestration`

### **Phase 5: Integration Testing (Week 9)**
6. **Create integration test suite**
   - Test all repos together
   - Verify dependency chains
   - Performance regression tests

### **Phase 6: Documentation & Release (Week 10)**
7. **Comprehensive documentation**
   - Per-repo API docs
   - Integration guides
   - Migration guide from monorepo
   - Release announcements

---

## **Dependency Graph**

```
                    ┌──────────────────┐
                    │ qmnf-orchestration│
                    └────────┬──────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   │                   │
    ┌────▼────┐      ┌───────▼───────┐   ┌──────▼─────┐
    │cylindrical│      │ qmnf-cognitive │   │Layer 4 repos│
    │  -time    │      │               │   │ (memory)    │
    └────┬────┘      └───────┬───────┘   └──────┬─────┘
         │                   │                   │
         │          ┌────────┴────────┐          │
         │          │                 │          │
    ┌────▼──────────▼─────┐      ┌────▼──────────▼─────┐
    │   maa-crypto        │      │  hcvlang-fhe        │
    │ (post-quantum)      │      │  (FHE schemes)      │
    └─────────┬───────────┘      └──────────┬──────────┘
              │                             │
              └──────────┬──────────────────┘
                         │
                  ┌──────▼───────┐
                  │ hcvlang-math │
                  │ (FOUNDATION) │
                  └──────────────┘
                  NO DEPENDENCIES
```

---

## **Benefits of This Architecture**

### **Development Benefits**
- ✅ **Independent releases** - Fix Layer 1 without touching Layer 5
- ✅ **Parallel development** - Teams can work on different layers
- ✅ **Clear boundaries** - Well-defined APIs between layers
- ✅ **Easier testing** - Test each layer in isolation
- ✅ **Better CI/CD** - Smaller build times, targeted tests

### **User Benefits**
- ✅ **Pick what you need** - Use just math library, or full stack
- ✅ **Smaller dependencies** - Don't pull in FHE if you just need math
- ✅ **Stable APIs** - Layer 1 can have LTS releases
- ✅ **Clear documentation** - Each repo has focused docs

### **Business Benefits**
- ✅ **Licensing flexibility** - Different licenses per layer
- ✅ **Commercial options** - Open math, proprietary MAA
- ✅ **Easier audits** - Security audit just crypto layers
- ✅ **Publishing** - Release Layer 1 to crates.io/PyPI for adoption

---

## **Repository Naming Convention**

- **Rust crates**: `hcvlang-*` (e.g., `hcvlang-math`, `hcvlang-fhe`)
- **Python packages**: `qmnf-*` or `hcvlang-*` depending on primary language
- **Repos with both**: Use `hcvlang-*` name, publish to both crates.io and PyPI

---

## **License Strategy**

| Repository | Suggested License | Rationale |
|------------|------------------|-----------|
| `hcvlang-math` | MIT or Apache-2.0 | Maximum adoption, foundational |
| `hcvlang-fhe` | MIT or Apache-2.0 | Research-friendly, cite if used |
| `maa-crypto` | Proprietary / Research-Only | Patent protection, NIST submission |
| `holodrive-*` | Apache-2.0 or GPL | Research + commercial options |
| `qmnf-cognitive` | Research-Only | Proprietary consciousness tech |
| `qmnf-orchestration` | Proprietary | Core differentiator |

---

## **Next Steps**

1. **Review this plan** - Confirm layer assignments
2. **Start with Layer 1** - Extract `hcvlang-math` first
3. **Set up repo templates** - CI/CD, docs, benchmarks
4. **Create extraction scripts** - Automate file moves
5. **Test continuously** - Ensure nothing breaks

**Estimated Timeline**: 10 weeks for full migration
**Minimal Viable Migration**: 2 weeks (Layer 1 + Layer 2 only)

---

## **Questions to Resolve**

1. **GSO Placement**: Is GSO (swarm optimization) cognitive (Layer 6) or orchestration (Layer 7)?
   - **Current assignment**: Layer 6 (cognitive)
   - **Alternative**: Could be Layer 7 if used for resource scheduling

2. **Noise Generators**: FHE layer or crypto utilities?
   - **Current assignment**: Layer 2 (FHE)
   - **Includes**: `gso_noise_gen.py`, Gaussian samplers

3. **Benchmarks**: Per-repo or unified benchmark suite?
   - **Recommendation**: Per-repo + integration benchmark repo

4. **Python bindings**: Separate repos or in same repo?
   - **Recommendation**: Same repo (easier versioning)

---

**Document Version**: 1.0
**Last Updated**: 2025-10-29
**Status**: DRAFT - Awaiting Review

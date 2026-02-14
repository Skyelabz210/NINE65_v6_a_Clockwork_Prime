# Homomorphic Armada

Unified forensic testing ecosystem for all NINE65/QMNF builds — 25 distinct components from desktop archives, live projects, and GitHub repositories, all navigable from one place.

## Quick Start

```bash
# Run FHE benchmarks against the latest v5 build
cd harness
cargo bench -p armada-bench --features v5_live

# Smoke test against all builds
./scripts/bench_all.sh --test

# Full cross-build comparison
./scripts/bench_all.sh

# Tracked audit with fine-grained metrics
cargo bench -p armada-bench --bench tracked_audit --features v5_live
```

---

## Canonical Build Model Tables

### FHE Lineage (BFV Ring Model)

| ID | Build | Version | Package | Crates | Modules | Architecture | Key Additions |
|----|-------|---------|---------|--------|---------|--------------|---------------|
| 08a | `08_FHE_v01_original` | `0.1.0-v01` | `qmnf_fhe` | 1 | arithmetic, entropy, keys, ops, params, ring, ahop | BFV Ring + AHOP | Base FHE: BFVEncoder, BFVEncryptor, BFVDecryptor, BFVEvaluator, NTTEngine, ShadowHarvester |
| 08b | `08_FHE_v02_stable` | `0.1.0-v02` | `qmnf_fhe` | 1 | +security, +kat | BFV Ring + AHOP | KeySet::generate_secure (OS CSPRNG), LWE security estimation, KAT vectors |
| 08d | `08_FHE_v04_QClassic` | `0.1.0-v04` | `qmnf_fhe` | 1 | +quantum, +noise | BFV Ring + Quantum | FHENeuralEvaluator, MobiusInt, PadeEngine, entanglement, Grover noise search |
| 13 | `13_NINE65_original` | — | `qmnf_fhe` | 1 | (earliest) | BFV Ring | First standalone NINE65 build |
| 14 | `14_NINE65_stable` | — | `qmnf_fhe` | 1 | +security, +kat | BFV Ring | Stable release with audits: CRYPTO_AUDIT_REPORT.md |

### MANA Fork (Lane/Stream CRT Model)

| ID | Build | Package | Crates | Architecture | Key Constructs |
|----|-------|---------|--------|--------------|----------------|
| 08c | `08_FHE_v03_MANA_boosted` | `mana`, `nine65`, `unhal` | 3 | Lane/Stream parallel CRT | Lane::from_int_slice, ManaStream, K-Elimination anchors, 2.78x Rayon speedup |
| 15 | `15_MANA_boosted_live` | `mana`, `nine65`, `unhal` | 3 | Lane/Stream parallel CRT | Live development mirror of v03 |
| 16 | `16_MANA_private` | `qmnf-security-analysis` | 1 | Cryptanalysis toolkit | attack-estimator, k-elimination-attack, lattice-attack binaries |
| 17 | `17_MANA_definitive` | `mana`, `nine65`, `unhal` | 3 | Lane/Stream parallel CRT | Definitive public release (GitHub) |

### NINE65 System Builds (Full Workspace)

| ID | Build | Package | Crates | Key Crates | Snapshot Date | Distinguishing Features |
|----|-------|---------|--------|------------|---------------|------------------------|
| 02 | `02_symmetric_public` | `nine65` | 7 | clockwork-core, mana, nexgen_rational, nine65, unhal | — | Public symmetric release (no exact_trans, no fhe-service) |
| 03 | `03_v5_full_20260209` | `nine65` | 9 | +exact_transcendentals, +fhe-service | 2026-02-09 | Full v5 archive (1.3GB) incl. Lean4 lake-packages |
| 04 | `04_pre_exact_trans_git` | `nine65` | 7 | clockwork-core, mana, nexgen_rational, nine65, unhal | — | Pre-transcendentals snapshot |
| 06 | `06_system_20260211` | `nine65` | 9 | +exact_transcendentals, +fhe-service | 2026-02-11 | System snapshot with WASM/Python excluded |
| 09 | `09_NINE65_v5_live` | `nine65` | 9 | all 9 crates | LIVE | Active development HEAD: DualRNS, Galois, bootstrap-free compiler, Clockwork formal RNS |

### QMNF System (Modular Layer Architecture)

| ID | Build | Crates | Layer 0 | Layer 1 | Layer 2 | Layer 3 |
|----|-------|--------|---------|---------|---------|---------|
| 10 | `10_QMNF_System` | 9 | qmnf-primitives | qmnf-arithmetic | qmnf-polynomial, qmnf-fhe, qmnf-diagnostics, qmnf-neural, qmnf-optimization, qmnf-crypto | qmnf-orchestration |

### MYSTIC (Complete v2 with Chaos/EPRAM)

| ID | Build | Package | Unique Modules | Features |
|----|-------|---------|----------------|----------|
| 11 | `11_MYSTIC_v2` | `qmnf_fhe` | chaos, epram, quantum, security | ntt_fft, wassan (holographic noise), v2 meta-feature |

### Exact Transcendentals

| ID | Build | Package | Version | Engines | Functions |
|----|-------|---------|---------|---------|-----------|
| 01 | `01_exact_transcendentals` | `exact_transcendentals` | `0.1.0` | CORDIC(i64), AGM(u128), BinarySplitting(i128) | sin, cos, exp, ln, isqrt, pi |
| 12 | `12_exact_trans_live` | `exact_transcendentals` | `1.0.0` | Same engines (live dev) | Same + cross-validation suite |

### Formal Verification & Proofs

| ID | Build | Languages | Proof Targets | Contents |
|----|-------|-----------|---------------|----------|
| 05 | `05_proofstack_20260211` | Lean4, Coq | K-Elimination | KElimination lakefile + Coq proofs |
| 07 | `07_security_proofs` | Lean4, Coq | AHOP, K-Elim, QMNF, Shadow, exact-trans, NIST | 8 Lean4 targets, 3 Coq modules, NIST vectors, tex docs |
| 19 | `19_k_elimination_lean4` | Lean4 | K-Elimination | Standalone K-Elimination formal proof (GitHub) |

### Tooling & Infrastructure

| ID | Build | Language | Purpose | Key Files |
|----|-------|----------|---------|-----------|
| 18 | `18_hackfate` | HTML/JS/MD | HackFate website + benchmark evidence | BENCHMARK_EVIDENCE.md, benchmarks.html |
| 20 | `20_Loki5` | Python | Loki5 Cryptographic Framework | FORMAL_SPECIFICATION.md, src/core, tests |
| 21 | `21_ENHANCE` | Rust | Enhancement suite | src/main.rs, User_Developer_Guide |
| 22 | `22_redteam_mcp` | Python | Red team MCP security tooling | grover_swarm_server.py, redteam_server.py |

---

## API Compatibility Matrix

```
v01 (Original) ──► v02 (Stable) ──► v04 (QClassic) ──► v5 (NINE65 Latest)
     │                  │                  │                    │
     │                  │                  │                    ├─ DualRNS + Galois
     │                  │                  │                    ├─ Bootstrap-free compiler
     │                  │                  │                    ├─ 9-crate workspace
     │                  │                  │                    └─ Clockwork formal RNS
     │                  │                  │
     │                  │                  ├─ Quantum (entanglement, Grover)
     │                  │                  ├─ Neural (FHENeuralEvaluator)
     │                  │                  ├─ MobiusInt (signed arithmetic)
     │                  │                  └─ Pade/Cyclotomic transcendentals
     │                  │
     │                  ├─ KeySet::generate_secure() (OS CSPRNG)
     │                  ├─ LWE security estimation
     │                  └─ Known Answer Tests (KAT)
     │
     └─ BFVEncoder/Encryptor/Decryptor/BFVEvaluator
        FHEConfig, NTTEngine, ShadowHarvester

v03 (MANA) ◄── INCOMPATIBLE FORK
     │
     ├─ Lane/Stream architecture (not Ring)
     ├─ Parallel CRT compute lanes
     ├─ K-Elimination anchors
     └─ 2.78x Rayon speedup
```

**Compatible chain**: v01 → v02 → v04 → v5 (all share BFV core API)
**Incompatible fork**: v03 (MANA) — completely different data model

---

## Shimming Harness Architecture

```
harness/
├── Cargo.toml                 # Workspace root
├── armada-shim/               # Trait definitions + feature-gated impls
│   ├── src/
│   │   ├── lib.rs             # ArmadaFHE, ArmadaTranscendentals, ArmadaParallel traits
│   │   ├── metrics.rs         # TrackedFhe wrapper, MetricsCollector, RunningStats
│   │   ├── v01.rs             # V01Fhe impl (zero-dep BFV)
│   │   ├── v02.rs             # V02Fhe impl (+secure keygen)
│   │   ├── v04.rs             # V04Fhe impl (+quantum+neural)
│   │   ├── v05.rs             # V05Fhe impl (latest NINE65)
│   │   ├── mana.rs            # ManaParallel impl (Lane/Stream)
│   │   └── transcendentals.rs # ExactTrans impl (CORDIC/AGM)
│   └── Cargo.toml
├── armada-bench/              # Criterion benchmarks
│   ├── benches/
│   │   ├── cross_build.rs     # Identical benchmarks across all builds
│   │   └── tracked_audit.rs   # Fine-grained boundary metric collection
│   └── Cargo.toml
└── scripts/
    ├── bench_all.sh           # Run benchmarks across all builds
    └── test_cross_build.sh    # Verify compilation against all builds
```

### Feature Flags

| Feature | Build | Package | Notes |
|---------|-------|---------|-------|
| `v01_original` | 08_FHE_v01 | `qmnf_fhe` | Zero external deps |
| `v02_stable` | 08_FHE_v02 | `qmnf_fhe` | +zeroize, getrandom |
| `v03_mana` | 08_FHE_v03/mana | `mana` | Lane/Stream (incompatible) |
| `v04_qclassic` | 08_FHE_v04 | `qmnf_fhe` | +quantum, neural, MobiusInt |
| `v5_live` | 09_NINE65_v5 | `nine65` | Latest, 9-crate workspace |
| `exact_trans` | 12_exact_trans | `exact_transcendentals` | CORDIC/AGM/BS |

---

## Forensic Audit Reports

Each build has a dedicated audit directory under `audits/<build_name>/` containing:

- `FORENSIC_AUDIT.md` — Deep forensic analysis: data flow tracing, construct identification, wiring verification, anomaly catalogue
- Generated by autonomous inspection agents — read-only analysis, zero modifications

```
audits/
├── 01_exact_transcendentals/FORENSIC_AUDIT.md
├── 02_symmetric_public/FORENSIC_AUDIT.md
├── ...
└── 22_redteam_mcp/FORENSIC_AUDIT.md
```

### Audit Status

Completed: 23/25 builds successfully audited
Pending: 2 builds awaiting completion due to resource constraints

| ID | Build | Status | Notes |
|----|-------|--------|-------|
| 08d | `08_FHE_v04_QClassic` | Pending | Rate limit constraints prevented completion |
| 17 | `17_MANA_definitive` | Pending | Rate limit constraints prevented completion |
| All others | Various | Complete | 23 builds fully audited |

### Audit Methodology

Each audit agent performs:
1. **Structure Mapping** — Module tree, dependency graph, public API surface
2. **Data Flow Tracing** — How data enters, transforms, and exits each construct
3. **Construct Identification** — Every struct, trait, enum, function catalogued with purpose
4. **Wiring Verification** — Are all declared constructs actually connected end-to-end?
5. **Dead Code Detection** — Declared but unreachable paths
6. **Cross-Reference Check** — Do imports/exports match between modules?
7. **Anomaly Catalogue** — Anything unexpected, incomplete, or inconsistent

---

## Boundary Tracking (TrackedFhe)

The `TrackedFhe<T>` wrapper in `metrics.rs` instruments every operation boundary:

```rust
use armada_shim::{ArmadaFHE, CurrentFhe, TrackedFhe};

let tracked = TrackedFhe::<CurrentFhe>::setup_light_tracked();
let ct_a = tracked.encrypt(42);
let ct_b = tracked.encrypt(17);
let sum = tracked.add(&ct_a, &ct_b);
let val = tracked.decrypt(&sum);

tracked.report();         // Canonical summary (min/max/mean/stddev per op)
tracked.error_report();   // Anomalies only
tracked.to_csv();         // Export for external analysis
tracked.forensic_sweep(100); // Deep validation pipeline
```

Collects: per-operation nanosecond timing, Welford's running statistics (P50/P95/P99), correctness roundtrip checks, drift detection, anomaly collection with full context.

---

## Running Benchmarks

```bash
cd ~/Projects/Homomorphic_Armada/harness

# Single build
cargo bench -p armada-bench --no-default-features --features v01_original

# v5 with tracked metrics
cargo bench -p armada-bench --bench tracked_audit --features v5_live

# All builds (automated)
./scripts/bench_all.sh

# Quick compilation check
./scripts/test_cross_build.sh
```

## See Also

- [IMPLEMENTATION_MAP.md](IMPLEMENTATION_MAP.md) — Per-build API surface, feature matrix, cross-references
- `harness/armada-shim/src/metrics.rs` — Boundary tracking implementation
- `builds/*/Cargo.toml` — Individual build configurations
- `audits/*/FORENSIC_AUDIT.md` — Per-build forensic audit reports

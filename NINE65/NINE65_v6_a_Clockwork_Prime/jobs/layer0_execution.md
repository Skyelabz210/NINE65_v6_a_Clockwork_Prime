# Layer 0 — Execution Breakdown

**Reference**: `jobs/qmnf-final-plan.md` (authoritative)
**Layer**: 0 — Immediate Start (parallel)
**Tracks Active**: A0, B0, B3, S0

---

## Parallel Execution Map

All four Layer 0 groups start simultaneously with zero inter-dependencies.

```
┌─────────┐   ┌─────────┐   ┌─────────┐   ┌─────────┐
│ A0      │   │ B0      │   │ B3      │   │ S0      │
│ Pre-    │   │ Primitive│   │ Legal/  │   │ Threat  │
│ Flight  │   │ Isolation│   │ IP      │   │ Model   │
│ 5 tasks │   │ 5 tasks │   │ 5 tasks │   │ 2 tasks │
│ [Hard]  │   │ [Hard]  │   │ [Hard]  │   │ [Hard]  │
└─────────┘   └─────────┘   └─────────┘   └─────────┘
```

---

## A0: Pre-Flight Assessment (5 tasks)

### A0.1 — API Surface Audit

**Status**: DELIVERABLE READY → `jobs/a0_1_api_surface_audit.md`

**What exists**:
- `hcvlang/src/lib.rs` re-exports 14 types across 46 public modules
- 2,905 pub fns, 947 pub structs, 100 pub enums, 11 pub traits
- 227 source files, 57 unsafe blocks

**Artifact**: Every module reviewed, every export classified as expose/wrap/hide.

### A0.2 — Minimum Viable API Definition

**Status**: NOT STARTED (depends on A0.1)

**What exists**:
- Current re-exports: CRTBigInt, HCVLangBigInt, Rational/QMNFRational, AdaptiveCRTBigInt, ModInt, ExactDivider, KFreeCRT
- No "MathCore" wrapper exists

**Work required**:
1. Define `mathcore` crate with thin public API
2. Core types: ExactInt (wraps CRTBigInt/HCVLangBigInt), RnsInt (wraps ModInt), Rational
3. Operations: create, arithmetic (+, -, *, /), compare, serialize, convert to/from primitives
4. Hide all internal machinery (PLMG, FPD, K-Free details)

**Artifact**: `mathcore/src/lib.rs` with documented public API. API covers core operations, nothing more.

### A0.3 — Performance Baseline

**Status**: PARTIAL

**What exists**:
- `docs/PERFORMANCE_BASELINE_2026-02-11.md` (NINE65 self-benchmarks)
- Criterion benchmarks in `hcvlang/benches/`
- No GMP/num-bigint/Python int/JS BigInt comparison

**Work required**:
1. Create comparison benchmark suite: 64-bit, 256-bit, 1024-bit, 4096-bit operands
2. Benchmark against: `num-bigint` (Rust), GMP (via `rug` crate), Python `int`, JS `BigInt`
3. Operations: add, sub, mul, div, mod, comparison, serialization
4. Statistical rigor: Criterion with 100+ iterations, confidence intervals

**Artifact**: `docs/mathcore_performance_baseline.md` with tables and methodology.

### A0.4 — Formal Verification Coverage Audit

**Status**: DELIVERABLE READY → `jobs/a0_4_formal_verification_matrix.md`

**What exists**:
- 14 Coq files (~166 theorems/lemmas, 86.7% fully proven)
- 18 Lean4 files (~310+ definitions, ~87.5% fully proven)
- `docs/FORMALIZATION_INDEX.md` (partial mapping)

**Artifact**: Coverage matrix mapping proofs to API surface, gaps identified.

### A0.5 — Dependency and Build Audit

**Status**: NOT STARTED

**What exists**:
- hcvlang has 14 runtime deps: serde, thiserror, sealed, qmnf_crtbigint, num-bigint, num-traits, num-integer, bincode, once_cell, rand, rand_chacha, typenum + optional rayon, pyo3
- **This violates MathCore's zero-runtime-deps target**

**Work required**:
1. Audit each dependency: necessary vs removable
2. Plan path to zero runtime deps for MathCore core (num-* can stay, serde/rand behind features)
3. Verify reproducible builds on Linux (x86_64/aarch64), macOS (arm64), Windows (x86_64)
4. Test cross-compilation matrix

**Artifact**: `docs/dependency_audit.md` with removal plan and build matrix results.

**Acceptance**: Zero required runtime dependencies for core arithmetic. All platforms build.

---

## B0: Cryptographic Primitive Isolation (5 tasks)

### B0.1 — AHOP Extraction

**Status**: NOT STARTED

**What exists**:
- AHOP code in `hcvlang/src/ahop.rs` (v2 archive)
- 3 HIGH vulnerabilities identified in v5 AHOP security assessment
- Lean4 proofs: `AHOP/Parameters.lean`, `AHOP/Algebra.lean`, `AHOP/Hardness.lean`

**Work required**:
1. Extract AHOP into standalone crate with minimal QMNF deps (CRTBigInt via MathCore API)
2. Address 3 HIGH vulnerabilities from security assessment
3. Create standalone test suite
4. Document dependency graph on MathCore

**Artifact**: `crates/ahop/` standalone crate, compiles and passes all tests independently.

### B0.2 — NINE65 Extraction

**Status**: SUBSTANTIALLY DONE

**What exists**:
- `crates/nine65/` already standalone: 72 src files, ~48,681 lines, 819+ tests
- Defense mechanisms: shadow entropy monitor, GRO timing gate, integrity checks
- Clockwork Bootstrap components: `ops/bootstrap.rs` (1,771 lines)

**Work required**:
1. Inventory existing defense mechanisms with status (active/dormant/deprecated)
2. Verify dependency graph on MathCore is explicit and minimal
3. Extract Clockwork Bootstrap as separable submodule (feeds B0.6)

**Artifact**: Module independence verified. Defense catalog complete.

### B0.3 — Parameter Set Definition

**Status**: SUBSTANTIALLY DONE

**What exists**:
- `crates/nine65/src/params/secure_configs.rs`: secure_128(), secure_192(), secure_256()
- Parameter security hardening: compile-time assertions, runtime validation
- Test configs gated behind `allow_insecure` feature

**Work required**:
1. Validate q_small = t constraint for Clockwork Bootstrap
2. Verify >= 2 RNS limbs for CRT reconstruction in modswitch_to_t
3. Document security margins per set
4. Verify against HE.org 2024 guidelines (ePrint 2024/463) with Lattice Estimator RC.MATZOV

**Artifact**: All parameter sets documented with security margins. All satisfy q_small = t.

### B0.6 — Clockwork Bootstrap Extraction

**Status**: PARTIAL (infrastructure exists, not isolated)

**What exists**:
- `ops/bootstrap.rs`: 1,771 lines, 3-phase mechanism, auto-bootstrap
- Bootstrap key generation
- `bootstrap_parameter_exploration.rs` test file

**Work required**:
1. Extract as distinct testable module: modswitch_to_t, homomorphic_inner_product, key_switch_bootstrap, composed clockwork_bootstrap
2. Create 1,000-iteration test: encrypt -> exhaust noise -> bootstrap -> decrypt -> assert match
3. Verify post-bootstrap noise in expected range

**Artifact**: Standalone bootstrap module. 1,000-iteration test passes.

### B0.7 — Bootstrap Parameter Validation

**Status**: NOT STARTED

**Work required**:
1. For every parameter set: verify q_small = t
2. Verify >= 2 RNS limbs per set
3. Verify Q_boot accommodates inner product noise
4. Compute and document post-bootstrap noise margin

**Artifact**: All sets pass. All have >= 64-bit post-bootstrap noise margin.

---

## B3: Legal/IP Foundation (5 tasks)

### B3.1 — Patent Landscape Analysis

**Status**: NOT STARTED

**Work required**:
1. Search prior art: Apollonian crypto, depth-1 bootstrap, RNS division, shadow entropy
2. Freedom-to-operate assessment
3. Identify potential conflicts

**Artifact**: Patent landscape document. Freedom-to-operate assessment.

### B3.2 — Provisional Patent Filing

**Status**: NOT STARTED

**Scope**: AHOP, NINE65, Clockwork Bootstrap, Shadow Entropy, K-Elimination

**Deadline**: Filed within 30 days of plan start.

### B3.3 — Dual License Design

**Status**: NOT STARTED

**Design**: AGPL v3 (open source) + commercial permissive

### B3.4 — Contributor License Agreement

**Status**: NOT STARTED

**Model**: Apache CLA

### B3.5 — Export Control Classification

**Status**: NOT STARTED

**Scope**: EAR 5D002. BIS notification for open-source crypto.

---

## S0: Threat Model Formalization (2 tasks)

### S0.1 — Adversary Tier Classification

**Status**: DELIVERABLE READY -> `jobs/s0_1_adversary_tiers.md`

**What exists**:
- `docs/SIDE_CHANNEL_THREAT_MODEL.md` (117 lines, no 3-tier classification)
- `v5/docs/SECURITY_PROOFS.md`
- AHOP Security Assessment (v5)
- Security code in `crates/nine65/src/security/` (secret_data.rs, gro_gate.rs, key_manager.rs, integrity.rs)
- Clockwork-core: GRO timing gate, Garner reconstruction, bound tracking

**Artifact**: 3-tier adversary model with defense mapping.

### S0.2 — Security Claim Inventory

**Status**: DELIVERABLE READY -> `jobs/s0_2_security_claim_inventory.md`

**What exists**:
- `v5/docs/CLAIM_REGISTRY.csv` (existing claims)
- README.md (public-facing claims)
- 14 Coq + 18 Lean4 proof files
- AHOP Security Assessment with specific vulnerability findings

**Artifact**: All claims classified as Proven/Argued/Assumed/False with proof links. Terminology corrections applied.

---

## Gate Criteria

### GATE A0: Foundation Verified [Hard]
- [ ] Every module reviewed (A0.1 complete)
- [ ] API covers core operations (A0.2 complete)
- [ ] Benchmarks reproducible (A0.3 complete)
- [ ] Coverage matrix complete (A0.4 complete)
- [ ] Zero runtime deps, all platforms build (A0.5 complete)

### GATE B0: Primitives Isolated [Hard]
- [ ] AHOP compiles + tests independently (B0.1)
- [ ] NINE65 module independent, defenses catalogued (B0.2)
- [ ] All parameter sets satisfy q_small = t (B0.3)
- [ ] Bootstrap module standalone, 1,000-iteration test passes (B0.6)
- [ ] All sets have >= 64-bit post-bootstrap noise margin (B0.7)

### GATE B3: Legal Foundation Set [Hard]
- [ ] Patent landscape documented (B3.1)
- [ ] Provisional patents filed within 30 days (B3.2)
- [ ] License files clear, commercial terms drafted (B3.3)
- [ ] CLA template ready (B3.4)
- [ ] Export classification determined (B3.5)

### GATE S0: Threat Model Formalized [Hard]
- [ ] 3-tier adversary model complete, no overclaims (S0.1)
- [ ] Every claim classified with proof links (S0.2)
- [ ] All "False" claims have redesign plans (S0.2)
- [ ] Terminology corrected throughout (S0.2)

---

## Layer 0 -> Layer 1 Transitions

When Layer 0 gates pass, the following Layer 1 work unblocks:

| Layer 1 Task | Requires |
|-------------|----------|
| A1: Core Stabilization | A0 |
| B1: Crypto API + Bootstrap API | B0, S0 |
| S1.1: Dual-Regime Shadow Entropy | S0, B0 |
| S1.2: HWRNG Integration | S1.1 |
| S2.1: VM/TEE/DBI Detection | S0 |
| S3.6: NTT Side-Channel Hardening | B0 |

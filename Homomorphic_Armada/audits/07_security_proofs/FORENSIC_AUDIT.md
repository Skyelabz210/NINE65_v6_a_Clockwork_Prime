# FORENSIC AUDIT: Build 07_security_proofs

**Audit Date**: 2026-02-13
**Auditor**: Claude Opus 4.6 (Forensic Code Auditor)
**Build Path**: `/home/acid/Projects/Homomorphic_Armada/builds/07_security_proofs/`
**Scope**: Comprehensive formal verification suite -- Lean4 (8 targets), Coq (3 modules + compiled proofs), NIST vectors, TeX documents
**Mandate**: INSPECT, ANALYZE, REPORT. No source modifications.

---

## Table of Contents

1. [Structure Mapping](#1-structure-mapping)
2. [Data Flow Tracing](#2-data-flow-tracing)
3. [Construct Identification](#3-construct-identification)
4. [Wiring Verification](#4-wiring-verification)
5. [NIST Compliance](#5-nist-compliance)
6. [Cross-Reference: Lean4 vs Coq](#6-cross-reference-lean4-vs-coq)
7. [Anomaly Catalogue](#7-anomaly-catalogue)

---

## 1. Structure Mapping

### 1.1 Top-Level Directory Tree

```
07_security_proofs/
|-- README.md
|-- LICENSE
|-- .gitignore
|-- lean4/                          # 8 Lean4 proof targets
|   |-- ahop-gaps/                  # Gap closure proofs
|   |   |-- GAP001/AsymptoticLemma.lean
|   |   |-- GAP004/BootstrapFreeDepth.lean
|   |   |-- GAP006/DecryptCorrectness.lean
|   |-- exact-transcendentals/      # Integer transcendental functions
|   |   |-- ExactTranscendentals.lean
|   |   |-- Main.lean
|   |   |-- ExactTranscendentals/
|   |       |-- Agm.lean
|   |       |-- Basic.lean
|   |       |-- BinarySplitting.lean
|   |       |-- ContinuedFraction.lean
|   |       |-- Cordic.lean
|   |       |-- ExactRational.lean
|   |       |-- Isqrt.lean
|   |-- formalization-swarm/        # 24 numbered theorem files + sub-package
|   |   |-- 02_QMNF_Lean4_Proofs.lean
|   |   |-- 05_KElimination.lean
|   |   |-- 06_CRTBigInt.lean
|   |   |-- 07_ShadowEntropy.lean
|   |   |-- 08_PadeEngine.lean
|   |   |-- 09_MobiusInt.lean
|   |   |-- 10_PersistentMontgomery.lean
|   |   |-- 11_IntegerNN.lean
|   |   |-- 12_CyclotomicPhase.lean
|   |   |-- 13_MQReLU.lean
|   |   |-- 14_BinaryGCD.lean
|   |   |-- 15_PLMGRails.lean
|   |   |-- 16_DCBigIntHelix.lean
|   |   |-- 17_GroverSwarm.lean
|   |   |-- 18_WASSAN.lean
|   |   |-- 19_TimeCrystal.lean
|   |   |-- 20_GSO.lean
|   |   |-- 21_MANA.lean
|   |   |-- 22_RayRam.lean
|   |   |-- 23_ClockworkPrime.lean
|   |   |-- 24_BootstrapFreeFHE.lean
|   |   |-- 25_RealTimeFHE.lean
|   |   |-- QMNFProofs.lean
|   |   |-- QMNFProofs/KElimination.lean
|   |-- k-elimination/              # Structured K-Elimination package
|   |   |-- KElimination.lean
|   |   |-- lakefile.lean
|   |   |-- KElimination/
|   |       |-- Basic.lean
|   |       |-- ZMod.lean
|   |       |-- ShadowEntropy.lean
|   |       |-- AHOP/
|   |       |   |-- Algebra.lean
|   |       |   |-- Hardness.lean
|   |       |   |-- Parameters.lean
|   |       |-- Lattice/
|   |           |-- CRT.lean
|   |-- qmnf-system/                # Core QMNF system proofs
|   |   |-- KElimination.lean
|   |   |-- ResidueLearning.lean
|   |-- security-swarm/             # Security proof swarm
|   |   |-- SwarmProofs/
|   |       |-- AHOPAlgebra.lean
|   |       |-- AHOPHardness.lean
|   |       |-- AHOPParameters.lean
|   |       |-- AHOPSecurity.lean
|   |       |-- Basic.lean
|   |       |-- CRT.lean
|   |       |-- HomomorphicSecurity.lean
|   |       |-- INDCPAGame.lean
|   |       |-- KElimination.lean
|   |       |-- NISTCompliance.lean
|   |       |-- RingDefinitions.lean
|   |       |-- Security.lean
|   |       |-- SecurityComplete.lean
|   |       |-- SecurityLemmas.lean
|   |-- shadow-nist/                # Shadow entropy + NIST compliance
|   |   |-- NISTCompliance.lean
|   |   |-- ShadowCorrelation.lean
|   |   |-- ShadowNISTCompliance.lean
|   |   |-- ShadowSecurityDefs.lean
|   |   |-- ShadowSecurityTheorems.lean
|   |   |-- ShadowUniform.lean
|   |-- standalone/                 # 40+ standalone proof files (many duplicates)
|       |-- [40+ .lean files including duplicates with (1), (2), etc.]
|       |-- advanced_toric/
|           |-- 14_BinaryGCD.lean
|           |-- 15_PLMGRails.lean
|           |-- 16_DCBigIntHelix.lean
|           |-- 17_ToricGrover.lean
|-- coq/                            # Coq source proofs (3 modules)
|   |-- nine65/                     # 19 .v files
|   |   |-- 03_QMNF_Coq_Proofs.v
|   |   |-- CRTShadowEntropy.v
|   |   |-- CyclotomicPhase.v
|   |   |-- EncryptedQuantum.v
|   |   |-- ExactCoefficient.v
|   |   |-- GSOFHE.v
|   |   |-- IntegerSoftmax.v
|   |   |-- KElimination.v
|   |   |-- K_Elimination.v         (DUPLICATE naming variant)
|   |   |-- MQReLU.v
|   |   |-- MobiusInt.v
|   |   |-- MontgomeryPersistent.v
|   |   |-- OrderFinding.v
|   |   |-- PadeEngine.v
|   |   |-- PeriodGrover.v
|   |   |-- ShadowIndependence.v
|   |   |-- SideChannelResistance.v
|   |   |-- StateCompression.v
|   |   |-- ToricGrover.v
|   |-- qmnf/                       # 3 .v files (overlap with nine65)
|   |   |-- 03_QMNF_Coq_Proofs.v
|   |   |-- PeriodGrover.v
|   |   |-- QMNF.v
|   |-- shadow/                     # EMPTY (no .v files)
|-- coq_proofs/                     # Compiled Coq proofs (NINE65 only)
|   |-- README.md
|   |-- .lia.cache, .nia.cache
|   |-- NINE65/                     # 16 .v files + compiled .vo, .vok, .vos, .glob, .aux
|       |-- [16 .v files with matching .vo compilation artifacts]
|-- nist/                           # Python NIST compliance tests
|   |-- nist_compliance_tests.py
|   |-- nist_security.py
|   |-- shadow_nist_tests.py
|-- docs/                           # Documentation
|   |-- formal-verification/        # 7 documents
|   |-- security-assessments/       # 8 documents
|   |-- theorem-stacks/             # 20 documents
|-- tex/                            # LaTeX papers
|   |-- formal_proofs.tex
|   |-- K_Elimination_Technical_Paper.tex
|-- lakefiles/                      # Lean4 build configurations
|   |-- hackfate-lakefile.lean
|   |-- formalization-swarm-lakefile.lean
|   |-- security-swarm-lakefile.lean
|-- swarm_run/                      # Swarm orchestration artifacts
    |-- AHOP_GAP_CLOSURE_PLAN.md
    |-- checkpoints/
    |-- jobs/                       # Agent outputs
    |   |-- kappa_critic/
    |   |-- lambda_librarian/
    |   |-- mu_simulator/
    |   |-- phi_decomposer/
    |   |-- pi_prover/
    |   |-- sigma_verifier/
    |-- lean_project/               # Mirror/source for security-swarm
    |   |-- SwarmProofs/            # 16 .lean files + Gaps/ subdirectory
    |   |-- lakefile.lean
    |   |-- lake-manifest.json
    |   |-- lean-toolchain
    |-- state/                      # Blueprint JSONs
    |-- synthesis/                  # Synthesis reports
```

### 1.2 File Counts

| Category | Count |
|----------|-------|
| Lean4 .lean files (lean4/) | 100 |
| Lean4 .lean files (swarm_run/) | 17 |
| Coq .v source files (coq/) | 22 |
| Coq .v source files (coq_proofs/) | 16 |
| Coq compiled .vo files | 16 |
| Python NIST test files | 3 |
| TeX documents | 2 |
| Markdown documentation | 35+ |
| Lakefile configurations | 4 |
| JSON state files | 4 |

---

## 2. Data Flow Tracing

### 2.1 Lean4 Targets -- What Each Proves

#### Target 1: `k-elimination/`
**Purpose**: Core K-Elimination theorem -- exact division in RNS via anchor recovery.

**Theorem chain**:
- `div_add_mod` -> `mod_add_div` -> `div_mod_identity` (division algorithm identities)
- `residue_lt_mod`, `div_mul_le` (bounds)
- `k_lt_A` (overflow count bounded by anchor)
- `k_mod_eq_k` (uniqueness when k < A)
- `key_congruence` (THE CORE: X % A = (X % M + (X / M) * M) % A)
- `kElimination_core` (composite: k < A AND key congruence)
- `kElimination_unique` (k mod A = k)
- `reconstruction` (X = vM + k * M)
- `add_mul_mod`, `add_mul_mod_small` (modular arithmetic)
- Sub-packages: `AHOP/Algebra.lean` (AHOP algebraic structure), `AHOP/Hardness.lean` (hardness axiom), `AHOP/Parameters.lean` (concrete parameters), `Lattice/CRT.lean` (CRT foundations), `ShadowEntropy.lean`, `ZMod.lean`, `Basic.lean`

**Status**: 0 sorry in K-Elimination core (`KElimination.lean`, `KElimination/Basic.lean`, `Lattice/CRT.lean`, `AHOP/Algebra.lean`). 1 axiom: `ahop_hardness` in `AHOP/Hardness.lean`.

#### Target 2: `security-swarm/SwarmProofs/`
**Purpose**: End-to-end IND-CPA security proof chain for QMNF-HE.

**Theorem chain**:
- `Basic.lean` -> `RingDefinitions.lean` (cyclotomic ring, Gaussian, RLWE structures)
- `CRT.lean` (CRT foundations, 0 sorry)
- `KElimination.lean` (K-Elimination, 0 sorry)
- `AHOPAlgebra.lean` -> `AHOPHardness.lean` -> `AHOPParameters.lean` -> `AHOPSecurity.lean` (AHOP proof chain)
- `INDCPAGame.lean` (IND-CPA game definition, PKE scheme, **1 sorry at line 306** for decrypt correctness / noise bound verification)
- `SecurityLemmas.lean` (RLWE indistinguishability, noise bounds, bootstrap-free depth; **3 sorry**: exp vs poly comparison at L82, L144; Nat division at L305)
- `HomomorphicSecurity.lean` (main security preservation theorem; **1 sorry at L273** for exp vs poly, **1 axiom** `rlwe_assumption`)
- `Security.lean` (concrete security, 0 sorry)
- `NISTCompliance.lean` (NIST parameter verification, 0 sorry for theorems but **4 axioms**: `coreSVP_hardness_model`, `rlwe_reduces_to_coreSVP`, `qmnf_security_bits_estimate`, `qmnf_min_attack_blocksize`)
- `SecurityComplete.lean` (final aggregation, 0 sorry)

**Axioms declared in security-swarm**:
1. `ahop_hardness` (AHOPHardness.lean:132) -- Computational hardness assumption
2. `rlwe_hardness` (RingDefinitions.lean:219) -- RLWE hardness assumption
3. `rlwe_assumption` (HomomorphicSecurity.lean:219) -- RLWE assumption for IND-CPA
4. `coreSVP_hardness_model` (NISTCompliance.lean:221) -- Core-SVP lattice model
5. `rlwe_reduces_to_coreSVP` (NISTCompliance.lean:234) -- Reduction axiom
6. `qmnf_security_bits_estimate` (NISTCompliance.lean:257) -- Security bit estimate
7. `qmnf_min_attack_blocksize` (NISTCompliance.lean:379) -- BKZ block size lower bound

#### Target 3: `shadow-nist/`
**Purpose**: Shadow entropy properties and NIST statistical compliance proofs.

**Theorem chain**:
- `ShadowSecurityDefs.lean` (minEntropy, statDistance, IsNegligible, uniformDist; **1 axiom** `exp_dominates_poly`, **1 sorry at L227** in `exp_dominates_poly_concrete`)
- `ShadowSecurityTheorems.lean` (shadow_independence, xor_entropy_preservation, shadow_accumulator_security, shadow_security, Landauer theorems; **2 axioms**: `func_of_indep_is_indep`, `leftover_hash_lemma`)
- `ShadowUniform.lean` (preimage_count, shadow_uniform_distribution, crt_bijection_card; 0 sorry)
- `ShadowCorrelation.lean` (cross-channel correlation bounds, CRT independence preservation; 0 sorry)
- `ShadowNISTCompliance.lean` (frequency_test_passes, runs_test_passes, entropy_test_passes, nist_compliance, nist_64bit_security, nist_256bit_output; 0 sorry)
- `NISTCompliance.lean` (ML-KEM parameter comparison, QMNF NIST Category 5 certification; **4 axioms** same as security-swarm)

**Axioms declared in shadow-nist**:
1. `exp_dominates_poly` (ShadowSecurityDefs.lean:200)
2. `func_of_indep_is_indep` (ShadowSecurityTheorems.lean:97)
3. `leftover_hash_lemma` (ShadowSecurityTheorems.lean:206)
4. `coreSVP_hardness_model` (NISTCompliance.lean:221)
5. `rlwe_reduces_to_coreSVP` (NISTCompliance.lean:234)
6. `qmnf_security_bits_estimate` (NISTCompliance.lean:257)
7. `qmnf_min_attack_blocksize` (NISTCompliance.lean:379)

#### Target 4: `ahop-gaps/`
**Purpose**: Gap closure proofs for outstanding sorry placeholders in the security-swarm.

- `GAP001/AsymptoticLemma.lean` -- Addresses exp vs poly comparison. **1 sorry remains** (line 50, `exp_dominates_poly` for large n).
- `GAP004/BootstrapFreeDepth.lean` -- Addresses SecurityLemmas.lean:305 division reasoning. **1 sorry remains** (line 92, `t_lt_threshold`).
- `GAP006/DecryptCorrectness.lean` -- Addresses INDCPAGame.lean:306 noise bound. **1 sorry remains** (line 95, `rounding_recovers_message`).

**Verdict**: Gap proofs are structurally complete but each retains exactly one technical sorry.

#### Target 5: `exact-transcendentals/`
**Purpose**: Integer-only transcendental function computation with formal guarantees.

**Sub-files and sorry count**:
- `Basic.lean` -- natAbsDiff, intAbs, utility theorems. **0 sorry**.
- `ExactRational.lean` -- Full exact rational arithmetic (add, sub, mul, div), equivalence proofs. **0 sorry**.
- `Agm.lean` -- AGM iteration for pi/ln computation. **3 sorry** (agm_converges, agm_symmetric, agm_bounds).
- `BinarySplitting.lean` -- Binary splitting for series. **2 sorry** (binarySplit_correctness, binarySplit_efficiency).
- `ContinuedFraction.lean` -- Continued fractions, Pell equation. **3 sorry** (cf_determinant_identity, cf_sqrt_error_bound, pell_correctness).
- `Cordic.lean` -- CORDIC integer trig. **6 sorry** (cordic_convergence, atan_total_sum_eq, pythagorean_identity, pythagorean_at_zero, cordic_sin_odd, cordic_cos_even).
- `Isqrt.lean` -- Integer square root. **4 sorry** (isqrtNewton_correctness, isPerfectSquare_iff, isqrt_monotonic, isqrt_of_square).

**Total**: 18 sorry across exact-transcendentals. Definitions and computation functions are complete; convergence/correctness theorems are stub proofs.

#### Target 6: `formalization-swarm/`
**Purpose**: Large-scale formalization covering 24 numbered theorems + QMNFProofs sub-package.

**Notable files**:
- `02_QMNF_Lean4_Proofs.lean` -- 2 sorry (1 deprecated with explicit note)
- `05_KElimination.lean` -- 3 sorry
- `06_CRTBigInt.lean` -- 1 sorry (Fibonacci coprimality)
- `23_ClockworkPrime.lean` -- 3 sorry (garnerConvert, modular inverses)
- Most other files (07-22, 24-25): 0 sorry in comments-as-proof-status; actual sorry content varies

**Total sorry in formalization-swarm**: approximately 9 (excluding comment mentions).

#### Target 7: `qmnf-system/`
**Purpose**: Core QMNF system proofs -- K-Elimination and Residue Learning.

- `KElimination.lean` -- **2 axioms**: `integer_primacy` (trivial: X = X), `modular_independence`. **0 sorry**.
- `ResidueLearning.lean` -- **3 axioms** (`modular_gradient_equivalence`, `chain_rule_modular`, `lipschitz_modular`). **14 sorry** spanning gradient descent convergence, CRT-based learning, and training loop correctness.

**ResidueLearning.lean is the most sorry-heavy file in the entire build (14 sorry).**

#### Target 8: `standalone/`
**Purpose**: Development/staging area for standalone proof files. Many duplicates.

**Key observations**:
- `KElimination.lean` -- FULLY PROVEN (0 sorry, 21 lemmas). This is the gold-standard standalone version.
- `Unified 2.lean` and its 4 copies `(1)`, `(2)`, `(3)`, `(4)` -- Each has 12 sorry. These are TBL/URRS theorem sketches.
- `QMNF_Formal_Verification.lean` -- 12 sorry. Early-stage CRT/FHE verification.
- `QMNF.lean` -- 5 sorry. Fourth attractor proofs incomplete.
- `PeriodGrover.lean` -- 8 sorry. Shor/Grover analysis.
- `EXACT_SORRY_REPLACEMENT.lean`, `SORRY_REPLACEMENT_MINIMAL.lean` -- Patch files containing replacement proof text for other files. These are NOT self-contained proofs.

**Total sorry in standalone/**: approximately 120+ (inflated by 5 duplicate Unified files at 12 each = 60).

### 2.2 Coq Modules -- What Each Verifies

#### Module: `coq/nine65/` (19 source files)

| File | Theorems/Lemmas | Admitted | Topic |
|------|----------------|----------|-------|
| KElimination.v | 33 | 0 | K-Elimination core, soundness, completeness |
| K_Elimination.v | -- | -- | Naming variant (separate file, not audited in detail) |
| 03_QMNF_Coq_Proofs.v | -- | 16 | QMNF contraction bound, convergence |
| PeriodGrover.v | 40 | 12 | Period-finding, Grover analysis |
| OrderFinding.v | 25 | 0* | BSGS multiplicative order |
| GSOFHE.v | 13 | 0* | GSO for FHE |
| MQReLU.v | 9 | 0* | Modular quantized ReLU |
| CRTShadowEntropy.v | 6 | 0* | CRT shadow entropy |
| MobiusInt.v | 8 | 0* | Mobius function |
| PadeEngine.v | 8 | 0* | Pade approximation |
| ExactCoefficient.v | 5 | 0* | Exact Taylor coefficients |
| StateCompression.v | 8 | 0* | Homomorphic state compression |
| IntegerSoftmax.v | 13 | 0* | Integer softmax |
| CyclotomicPhase.v | 8 | 0* | Cyclotomic phase |
| EncryptedQuantum.v | 4 | 0* | Encrypted quantum ops |
| SideChannelResistance.v | 11 | 0* | Constant-time proofs |
| MontgomeryPersistent.v | 17 | 0* | Persistent Montgomery mul |
| ToricGrover.v | 17 | 0* | Toric Grover search |
| ShadowIndependence.v | -- | -- | Shadow independence |

*Note: The coq_proofs/NINE65/ compiled directory (with .vo artifacts) shows 0 Admitted, suggesting the compiled versions may differ from the source or the source files in coq/nine65/ are distinct from coq_proofs/NINE65/. The coq_proofs/NINE65/ .v files have 0 Admitted while the coq/nine65/ directory has 28 Admitted across 2 files (PeriodGrover.v: 12, 03_QMNF_Coq_Proofs.v: 16).*

#### Module: `coq/qmnf/` (3 source files)

| File | Admitted | Topic |
|------|----------|-------|
| QMNF.v | 0 | Exact Discrete Contraction Bound, Lyapunov descent, convergence |
| 03_QMNF_Coq_Proofs.v | 16 | IDENTICAL to nine65 version |
| PeriodGrover.v | 12 | IDENTICAL to nine65 version |

**ANOMALY**: `coq/qmnf/03_QMNF_Coq_Proofs.v` and `coq/qmnf/PeriodGrover.v` appear to be exact duplicates of their `coq/nine65/` counterparts (same line numbers for Admitted). This is redundant file storage.

#### Module: `coq/shadow/`
**EMPTY** -- No .v files found. The directory exists but contains no Coq source.

#### Compiled: `coq_proofs/NINE65/` (16 files)
All 16 .v files have corresponding .vo (compiled), .vok, .vos, .glob, and .aux artifacts, indicating successful Coq compilation. The compiled proofs have **0 Admitted** per the grep scan, which means either:
- (a) The compiled versions are a cleaned-up subset, or
- (b) The source in coq_proofs/NINE65/ is the authoritative version that replaced Admitted stubs

The README in coq_proofs/ acknowledges "~31 Admitted across all files" but claims they are "non-critical."

### 2.3 Swarm Run Architecture

The `swarm_run/` directory contains the orchestration layer for a multi-agent formal verification pipeline:

**Agents**:
- `pi_prover` -- Generates proofs (T001, T002, L002, L003)
- `kappa_critic` -- Reviews proofs for gaps
- `sigma_verifier` -- Runs Lean compiler verification
- `mu_simulator` -- Generates NIST compliance tests
- `phi_decomposer` -- Decomposes proof obligations
- `lambda_librarian` -- Plans Lean library structure

**State management**: `blueprint.json`, `blueprint_round2.json`, `skeleton_blueprint.json` track proof status per theorem node. The blueprint records sorry counts per file.

**The `swarm_run/lean_project/` directory mirrors `lean4/security-swarm/` plus the `Gaps/` subdirectory** (GAP001, GAP004, GAP006 content appears in both locations).

---

## 3. Construct Identification

### 3.1 Complete Axiom Registry (Lean4)

| # | Axiom | File | Justification |
|---|-------|------|---------------|
| 1 | `integer_primacy` | qmnf-system/KElimination.lean:56 | Trivial (X = X). Philosophically motivated but logically vacuous. |
| 2 | `modular_independence` | qmnf-system/KElimination.lean:161 | CRT independence of residues under coprime moduli. Standard number theory. |
| 3 | `modular_gradient_equivalence` | qmnf-system/ResidueLearning.lean:95 | Gradient equivalence in modular space. Non-standard, specific to QMNF. |
| 4 | `chain_rule_modular` | qmnf-system/ResidueLearning.lean:101 | Chain rule for modular derivatives. Non-standard. |
| 5 | `lipschitz_modular` | qmnf-system/ResidueLearning.lean:107 | Lipschitz continuity in modular space. Non-standard. |
| 6 | `ahop_hardness` | k-elimination/AHOP/Hardness.lean:133, security-swarm/AHOPHardness.lean:132 | AHOP computational hardness. Cryptographic assumption -- standard practice. |
| 7 | `rlwe_hardness` | security-swarm/RingDefinitions.lean:219 | Ring-LWE hardness. Standard cryptographic assumption. |
| 8 | `rlwe_assumption` | security-swarm/HomomorphicSecurity.lean:219 | RLWE for IND-CPA. Standard. |
| 9 | `coreSVP_hardness_model` | shadow-nist/NISTCompliance.lean:221, security-swarm/NISTCompliance.lean:221 | Core-SVP lattice model. Standard. |
| 10 | `rlwe_reduces_to_coreSVP` | shadow-nist/NISTCompliance.lean:234, security-swarm/NISTCompliance.lean:234 | Reduction axiom. Standard lattice cryptography. |
| 11 | `qmnf_security_bits_estimate` | shadow-nist/NISTCompliance.lean:257, security-swarm/NISTCompliance.lean:257 | Security level estimate. Encodes concrete analysis result. |
| 12 | `qmnf_min_attack_blocksize` | shadow-nist/NISTCompliance.lean:379, security-swarm/NISTCompliance.lean:379 | BKZ lower bound. Encodes concrete analysis. |
| 13 | `exp_dominates_poly` | shadow-nist/ShadowSecurityDefs.lean:200 | Standard analysis fact: 2^n > n^c eventually. Provable but not proven. |
| 14 | `func_of_indep_is_indep` | shadow-nist/ShadowSecurityTheorems.lean:97 | Functions of independent RVs. Standard probability theory. |
| 15 | `leftover_hash_lemma` | shadow-nist/ShadowSecurityTheorems.lean:206 | Leftover Hash Lemma. Standard cryptographic result. |
| 16 | `ind_cpa_secure` | formalization-swarm/02_QMNF_Lean4_Proofs.lean:849, standalone/02_QMNF_Lean4_Proofs.lean:502 | IND-CPA security. Cryptographic assumption. |
| 17 | `pagerank_sum_preservation` | formalization-swarm/20_GSO.lean:141 | PageRank sum is preserved. Domain-specific. |
| 18 | `period_finding_hard` | formalization-swarm/19_TimeCrystal.lean:233, standalone/19_TimeCrystal.lean:217 | Period-finding hardness. Cryptographic assumption. |
| 19 | `ahop_hard` | standalone/ahop_sprint6.lean:165 | AHOP hardness (different formulation from #6). |
| 20 | `integer_primacy` (ZMod variant) | standalone/Unified 2*.lean:27 (x5 copies) | Modular arithmetic primacy. Duplicated in 5 files. |
| 21 | `urrs_lyapunov_negative` | standalone/Unified 2*.lean:254 (x5 copies) | URRS Lyapunov stability. Duplicated in 5 files. |

**Total unique axioms**: 21 (not counting duplicates across files)
**Standard cryptographic assumptions**: #6, 7, 8, 9, 10, 15, 16, 18, 19 (9 axioms)
**Standard mathematical facts usable as axioms**: #2, 13, 14 (3 axioms)
**QMNF-specific non-standard axioms**: #3, 4, 5, 11, 12, 17, 21 (7 axioms)
**Trivial/vacuous**: #1, 20 (2 axioms)

### 3.2 Coq Parameter/Axiom Registry

| Parameter/Axiom | File | Purpose |
|-----------------|------|---------|
| `Parameter M : nat` + `Axiom M_pos` | coq/qmnf/QMNF.v:18-19 | Modulus positivity |

The Coq proofs are notably more self-contained, using fewer axioms than the Lean4 proofs.

### 3.3 Sorry/Admitted Totals

| Location | sorry Count | Admitted Count |
|----------|-------------|----------------|
| lean4/ (all targets) | **235** occurrences across 59 files | N/A |
| coq/ (all modules) | N/A | **56** (28 in nine65 + 28 in qmnf, which are duplicates) |
| coq_proofs/NINE65/ | N/A | **0** |
| swarm_run/ | ~15 in .lean files | N/A |

**Net unique sorry in Lean4 structured targets** (excluding standalone duplicates and comment-only mentions):
- k-elimination/: 0
- security-swarm/: ~8 (3 in SecurityLemmas, 1 in INDCPAGame, 1 in HomomorphicSecurity, ~3 in comment-tracked elsewhere)
- shadow-nist/: 1 (ShadowSecurityDefs)
- ahop-gaps/: 3 (1 per GAP file)
- exact-transcendentals/: 18
- formalization-swarm/: ~9
- qmnf-system/: 14 (all in ResidueLearning)
- standalone/: ~120+ (heavily duplicated)

---

## 4. Wiring Verification

### 4.1 Do Proofs Chain Correctly?

**K-Elimination Chain (VERIFIED)**:
The dependency chain `div_add_mod` -> `key_congruence` -> `kElimination_core` -> `reconstruction` is complete in both Lean4 (`k-elimination/KElimination.lean`) and Coq (`coq/nine65/KElimination.v`, `coq/qmnf/QMNF.v`). The soundness theorem `k_elimination_sound` in Coq is fully proven (no Admitted). In Lean4 `k-elimination/`, the corresponding proof is also sorry-free.

**Security Proof Chain (PARTIALLY VERIFIED)**:
The intended dependency: `RingDefinitions` -> `CRT` -> `KElimination` -> `INDCPAGame` -> `SecurityLemmas` -> `HomomorphicSecurity` -> `SecurityComplete`

- `RingDefinitions` through `KElimination`: VERIFIED (0 sorry)
- `INDCPAGame`: 1 sorry (decrypt_correct at L306) -- **GAP006 provides structural fix but retains its own sorry**
- `SecurityLemmas`: 3 sorry (asymptotic + division bounds) -- **GAP001 and GAP004 partially address these but retain sorry**
- `HomomorphicSecurity`: 1 sorry (asymptotic) + 1 axiom (RLWE)
- `SecurityComplete`: 0 sorry, but depends on potentially-sorry upstream

**CRITICAL FINDING**: The security proof chain has a **5-sorry gap** between the fully-verified algebraic core and the final security certificate. The gap closures (GAP001, GAP004, GAP006) reduce but do not eliminate these gaps. Each gap file introduces replacement proofs that themselves contain 1 sorry each.

### 4.2 Are Referenced Lemmas Actually Proved?

**Cross-file references verified**:
- `SwarmProofs/SecurityLemmas.lean` references `CRT.lean` theorems -> VERIFIED (CRT.lean has 0 sorry)
- `SwarmProofs/HomomorphicSecurity.lean` references `SecurityLemmas.lean` -> PARTIALLY VERIFIED (SecurityLemmas has 3 sorry)
- `SwarmProofs/INDCPAGame.lean` references `RingDefinitions.lean` -> VERIFIED (0 sorry)
- `GAP006/DecryptCorrectness.lean` claims to resolve `INDCPAGame.lean:306` -> The resolution itself has 1 sorry at its core lemma

### 4.3 Duplicate/Mirror Issues

**FINDING**: The following pairs appear to be identical or near-identical:
1. `coq/qmnf/03_QMNF_Coq_Proofs.v` = `coq/nine65/03_QMNF_Coq_Proofs.v` (same Admitted at same lines)
2. `coq/qmnf/PeriodGrover.v` = `coq/nine65/PeriodGrover.v` (same Admitted at same lines)
3. `lean4/security-swarm/SwarmProofs/` mirrors `swarm_run/lean_project/SwarmProofs/` (with `swarm_run` also containing `Gaps/` subdirectory)
4. `standalone/` contains 5 copies of "Unified 2" with suffixes (1)-(4), all with 12 sorry each
5. Multiple files appear in both `standalone/` and `formalization-swarm/` (e.g., 05_KElimination, 14_BinaryGCD, 15_PLMGRails, etc.)
6. `standalone/advanced_toric/` duplicates files from `standalone/` root (14_BinaryGCD, 15_PLMGRails, 16_DCBigIntHelix)

---

## 5. NIST Compliance

### 5.1 NIST Standards Covered

| Standard | Coverage | Implementation |
|----------|----------|----------------|
| **NIST SP 800-22** | All 15 statistical tests | `nist/shadow_nist_tests.py` |
| **NIST SP 800-90B** | Entropy source requirements (health tests, min-entropy, repetition count, adaptive proportion) | `nist/nist_compliance_tests.py` Gate 2 |
| **NIST SP 800-88** | Media sanitization / key zeroing | `nist/nist_compliance_tests.py` Gate 3 (KT-02, KT-06) |
| **NIST SP 800-57** | Key management lifecycle | `nist/nist_compliance_tests.py` Gate 3 (KT-01, KT-03, KT-04, KT-05) |
| **FIPS 140-3** | Cryptographic module security (self-tests, integrity, non-invasive) | `nist/nist_compliance_tests.py` Gates 1, 4, 5, 6 |
| **NIST SP 800-175B** | Crypto standards compliance | `nist/nist_compliance_tests.py` Gate 7 |
| **ML-KEM/CRYSTALS-Kyber** | Parameter comparison (ML-KEM-512, 768, 1024) | `nist/nist_security.py`, `lean4/shadow-nist/NISTCompliance.lean` |
| **NIST PQC Categories** | Categories 1-5 assessment | `nist/nist_security.py` (assess_nist_category), `lean4/shadow-nist/NISTCompliance.lean` |

### 5.2 Formal NIST Proofs (Lean4)

The `shadow-nist/NISTCompliance.lean` file formally proves:
- `qmnf_128bit_minimum` -- QMNF parameters provide >= 128-bit security
- `qmnf_exceeds_mlkem1024` -- QMNF exceeds ML-KEM-1024 parameters
- `qmnf_achieves_cat5` -- NIST Category 5 achieved
- `qmnf_256bit_security` -- 256-bit security level
- `qmnf_nist_category5_certified` -- Final certification theorem

**CAVEAT**: These proofs rely on 4 axioms (`coreSVP_hardness_model`, `rlwe_reduces_to_coreSVP`, `qmnf_security_bits_estimate`, `qmnf_min_attack_blocksize`) that encode the security estimates as assumptions rather than deriving them.

The `ShadowNISTCompliance.lean` file proves:
- `frequency_test_passes`, `runs_test_passes`, `entropy_test_passes`, `serial_test_passes`, `cumulative_sums_test_passes` -- All 5 core NIST SP 800-22 tests
- `nist_compliance` -- Composite compliance theorem
- `nist_64bit_security`, `nist_256bit_output` -- Security level proofs

### 5.3 Test Vector Completeness

**NIST SP 800-22 tests** (`shadow_nist_tests.py`): All 15 statistical tests implemented:
1. Frequency (Monobit), 2. Block Frequency, 3. Runs, 4. Longest Run, 5. Binary Matrix Rank, 6. DFT/Spectral, 7. Non-overlapping Template, 8. Overlapping Template, 9. Maurer's Universal, 10. Linear Complexity, 11. Serial, 12. Approximate Entropy, 13. Cumulative Sums, 14. Random Excursions, 15. Random Excursions Variant

**NIST Compliance Tests** (`nist_compliance_tests.py`): 7 gates, ~35 individual tests covering:
- CRT bijectivity (100K round-trips)
- K-Elimination correctness (100K random inputs)
- Bound tracker soundness
- Entropy source health
- Key management lifecycle
- Side-channel resistance (constant-time)
- Full integration pipeline

**FINDING**: The test files import from `loki_clockwork_stack` at `/home/claude/`, which is a hardcoded external dependency path. **These tests are NOT self-contained within the build directory** and cannot be executed without the external stack module.

The `nist_security.py` file performs lattice security estimation using integer-only arithmetic. It correctly identifies that the provided parameters (n=4096, q=2^54-33) yield only ~62-bit classical security, below the NIST Category 1 threshold of 128 bits, and recommends ~109-bit modulus for compliance. This is an honest assessment.

---

## 6. Cross-Reference: Lean4 vs Coq

### 6.1 Coverage Overlap

| Theorem Domain | Lean4 | Coq | Both? |
|---------------|-------|-----|-------|
| K-Elimination Core | k-elimination/, security-swarm/, qmnf-system/, standalone/, formalization-swarm/ | nine65/KElimination.v, qmnf/QMNF.v | YES |
| K-Elimination Soundness | k-elimination/KElimination.lean (proven) | nine65/KElimination.v (proven) | YES |
| CRT Foundations | security-swarm/CRT.lean, k-elimination/Lattice/CRT.lean | nine65/CRTShadowEntropy.v | YES |
| AHOP Hardness | k-elimination/AHOP/*, security-swarm/AHOP* | -- | Lean4 ONLY |
| IND-CPA Security | security-swarm/INDCPAGame.lean | -- | Lean4 ONLY |
| Homomorphic Security | security-swarm/HomomorphicSecurity.lean | -- | Lean4 ONLY |
| NIST Compliance | shadow-nist/NISTCompliance.lean | -- | Lean4 ONLY |
| Shadow Entropy | shadow-nist/ShadowSecurityDefs.lean | nine65/CRTShadowEntropy.v | PARTIAL |
| Period-Finding/Grover | standalone/PeriodGrover.lean | nine65/PeriodGrover.v, qmnf/PeriodGrover.v | YES |
| MQ-ReLU | formalization-swarm/13_MQReLU.lean | nine65/MQReLU.v | YES |
| Montgomery Persistent | formalization-swarm/10_PersistentMontgomery.lean | nine65/MontgomeryPersistent.v | YES |
| GSO-FHE | formalization-swarm/20_GSO.lean | nine65/GSOFHE.v | YES |
| Exact Coefficients | -- | nine65/ExactCoefficient.v | Coq ONLY |
| Integer Softmax | formalization-swarm/11_IntegerNN.lean | nine65/IntegerSoftmax.v | PARTIAL |
| Encrypted Quantum | -- | nine65/EncryptedQuantum.v | Coq ONLY |
| Side-Channel Resistance | standalone/ConstantTime.lean | nine65/SideChannelResistance.v | YES |
| Exact Transcendentals | exact-transcendentals/ (7 files) | -- | Lean4 ONLY |
| Residue Learning | qmnf-system/ResidueLearning.lean | -- | Lean4 ONLY |
| Contraction Bound | -- | qmnf/QMNF.v | Coq ONLY |

### 6.2 Coverage Gaps

**In Lean4 but NOT in Coq**:
- Full IND-CPA security proof chain (INDCPAGame, HomomorphicSecurity, SecurityComplete)
- NIST parameter certification theorems
- Shadow entropy security (full treatment)
- AHOP algebraic structure and hardness
- Exact transcendental functions
- Residue learning / modular gradient descent
- RayRam matrix operations
- ClockworkPrime / Garner decomposition

**In Coq but NOT in Lean4**:
- Exact Discrete Contraction Bound (QMNF.v) -- fully proven
- ExactCoefficient.v
- EncryptedQuantum.v
- ShadowIndependence.v (nine65 only)
- Order Finding via BSGS (OrderFinding.v)

**Critical gap**: The Contraction Bound theorem (coq/qmnf/QMNF.v) is a major QMNF result that has NO Lean4 counterpart.

---

## 7. Anomaly Catalogue

### 7.1 CRITICAL Anomalies

| # | Severity | Description | Location |
|---|----------|-------------|----------|
| C1 | CRITICAL | **ResidueLearning.lean has 14 sorry** -- the most of any file. Core training-loop correctness, CRT learning theorems, and convergence proofs are unproven. | lean4/qmnf-system/ResidueLearning.lean |
| C2 | CRITICAL | **Security proof chain has 5 sorry gap**: decrypt correctness, 2x asymptotic (exp vs poly), 1x division bound, 1x homomorphic security. GAP files reduce but do not eliminate these. | lean4/security-swarm/SwarmProofs/ |
| C3 | CRITICAL | **NIST Category 5 certification relies on 4 axioms** that encode security bit estimates as assumptions. The certification theorem `qmnf_nist_category5_certified` is only as strong as these unproven axioms. | lean4/shadow-nist/NISTCompliance.lean, security-swarm/NISTCompliance.lean |
| C4 | CRITICAL | **nist_security.py honestly shows provided parameters (q=2^54-33) achieve only ~62-bit classical security**, well below NIST Category 1. The Lean4 proofs axiomatically assume 256+ bit security for the same system. There is a **disconnect between the computational validation and the formal proofs**. | nist/nist_security.py vs lean4/shadow-nist/NISTCompliance.lean |

### 7.2 HIGH Anomalies

| # | Severity | Description | Location |
|---|----------|-------------|----------|
| H1 | HIGH | **coq/shadow/ directory is EMPTY** -- declared as a module but contains no proofs. | coq/shadow/ |
| H2 | HIGH | **Exact-transcendentals has 18 sorry** across convergence/correctness theorems. All the interesting proofs (AGM convergence, CORDIC bounds, Pell correctness, continued fraction error bounds) are unproven stubs. Definitions are complete but theorems are placeholders. | lean4/exact-transcendentals/ |
| H3 | HIGH | **3 non-standard QMNF-specific axioms** in ResidueLearning.lean (modular_gradient_equivalence, chain_rule_modular, lipschitz_modular) are novel mathematical claims that have no standard reference. They should be theorems, not axioms. | lean4/qmnf-system/ResidueLearning.lean |
| H4 | HIGH | **exp_dominates_poly is axiomatized** when it is a standard provable fact (n^c < 2^n for large n). This axiom is used transitively in security proofs. | lean4/shadow-nist/ShadowSecurityDefs.lean:200 |
| H5 | HIGH | **PeriodGrover.v has 12 Admitted** in both coq/nine65/ and coq/qmnf/ -- making it the most incomplete Coq proof file. The quantum period-finding analysis is essentially a specification without proof. | coq/nine65/PeriodGrover.v, coq/qmnf/PeriodGrover.v |

### 7.3 MEDIUM Anomalies

| # | Severity | Description | Location |
|---|----------|-------------|----------|
| M1 | MEDIUM | **5 duplicate "Unified 2" files** in standalone/ with 12 sorry each (60 total). Naming suggests copy-paste iterations: "Unified 2.lean", "Unified 2 (1).lean" through "(4)". | lean4/standalone/ |
| M2 | MEDIUM | **coq/qmnf/ duplicates coq/nine65/ files** -- PeriodGrover.v and 03_QMNF_Coq_Proofs.v are byte-identical between directories. | coq/qmnf/, coq/nine65/ |
| M3 | MEDIUM | **swarm_run/lean_project mirrors security-swarm** -- appears to be the development copy. Changes could drift. | swarm_run/lean_project/ vs lean4/security-swarm/ |
| M4 | MEDIUM | **NIST test files have hardcoded external path** (`/home/claude/loki_clockwork_stack.py`) -- tests are not portable or self-contained. | nist/nist_compliance_tests.py:34-43 |
| M5 | MEDIUM | **Standalone KElimination.lean claims "0 sorry"** but standalone/05_KElimination.lean (different file) has 3 sorry. Multiple conflicting versions of K-Elimination proofs exist at different maturity levels. | lean4/standalone/ |
| M6 | MEDIUM | **SORRY_REPLACEMENT_MINIMAL.lean and EXACT_SORRY_REPLACEMENT.lean are patch files**, not proofs. They contain replacement text for sorry placeholders in other files but are not integrated. | lean4/standalone/ |
| M7 | MEDIUM | **03_QMNF_Coq_Proofs.v has 16 Admitted** across modular arithmetic lemmas, CRT foundations, and convergence bounds. These are described as "non-critical" in the README but several are foundational (e.g., Euclid's lemma, Bezout's identity). | coq/nine65/03_QMNF_Coq_Proofs.v |
| M8 | MEDIUM | **integer_primacy axiom is trivially true** (forall X, X = X). It serves no logical purpose and should be removed or documented as a philosophical marker only. | lean4/qmnf-system/KElimination.lean:56 |

### 7.4 LOW Anomalies

| # | Severity | Description | Location |
|---|----------|-------------|----------|
| L1 | LOW | Files with spaces in names ("Unified 2 (1).lean", etc.) are non-standard and may cause build issues. | lean4/standalone/ |
| L2 | LOW | `coq/nine65/K_Elimination.v` exists alongside `KElimination.v` -- naming variant suggests file management confusion. | coq/nine65/ |
| L3 | LOW | Lakefiles are stored in a separate `lakefiles/` directory rather than in their respective project directories. | lakefiles/ |
| L4 | LOW | `swarm_run/jobs/sigma_verifier/lean.log` suggests Lean compilation was attempted but log contents not audited. | swarm_run/jobs/ |
| L5 | LOW | nist_compliance_tests.py uses `math.log2` (float) for min-entropy computation in Gate 2 (ET-02), violating the integer-only mandate. | nist/nist_compliance_tests.py:280 |
| L6 | LOW | nist_security.py line 588 uses `sigma_scaled/100` which is float division in a print statement. | nist/nist_security.py:588 |

---

## Summary Statistics

### Proof Maturity by Target

| Target | Total Theorems/Lemmas (est.) | sorry | Axioms | Maturity |
|--------|------------------------------|-------|--------|----------|
| k-elimination/ | ~60 | 0 | 1 (ahop_hardness) | PRODUCTION |
| security-swarm/ | ~120 | 8 | 7 | NEAR-COMPLETE |
| shadow-nist/ | ~70 | 1 | 7 | NEAR-COMPLETE |
| ahop-gaps/ | ~15 | 3 | 0 | STRUCTURAL |
| exact-transcendentals/ | ~50 | 18 | 0 | DEFINITIONS ONLY |
| formalization-swarm/ | ~200 | 9 | 3 | MIXED |
| qmnf-system/ | ~40 | 14 | 5 | INCOMPLETE |
| standalone/ | ~300 | 120+ | 12 | DEVELOPMENT/ARCHIVE |
| **Coq (net unique)** | ~225 | N/A (28 Admitted net) | 1 | NEAR-COMPLETE |

### Overall Verdict

**Strengths**:
- K-Elimination is **fully proven** in both Lean4 and Coq with zero sorry/Admitted in the core theorem chain. This is the crown jewel of the build.
- The security-swarm proof chain is architecturally sound with clear theorem dependencies.
- NIST SP 800-22 statistical test coverage is complete (all 15 tests).
- Coq compiled proofs (coq_proofs/NINE65/) have 0 Admitted, indicating a clean compilation baseline exists.
- The swarm orchestration architecture (6 agents) is well-structured.

**Weaknesses**:
- 235 total sorry occurrences in Lean4 (though many are duplicates in standalone/).
- The security proof chain has an unresolved 5-sorry gap at critical junctures (decrypt correctness, asymptotic bounds).
- NIST Category 5 certification is axiom-dependent, and the computational validator contradicts the formal proof's security claims (62-bit vs 256-bit).
- Exact-transcendentals theorems are all unproven stubs.
- ResidueLearning.lean is heavily incomplete (14 sorry, 3 non-standard axioms).
- Significant file duplication across directories.
- Empty coq/shadow/ module.
- External dependency on non-included `/home/claude/` path for NIST tests.

---

*End of Forensic Audit Report*

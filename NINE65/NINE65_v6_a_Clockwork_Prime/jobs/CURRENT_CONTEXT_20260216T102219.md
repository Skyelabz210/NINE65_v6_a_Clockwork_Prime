# NINE65 v6 "a Clockwork Prime" -- System Audit Blueprint

**Date**: 2026-02-16
**Scope**: Full system analysis across all 7 workspace crates
**Method**: Single-item-per-agent verification (no superchain proofing)
**Build Status**: 1,056 tests passing, 0 failures, 3 warnings (dead code)
**Project Root**: /home/acid/Projects/NINE65/NINE65_v6_a_Clockwork_Prime

---

## SYSTEM SNAPSHOT

| Crate | Tests | Purpose |
|-------|-------|---------|
| nine65 | 582 | Core FHE: arithmetic, ring, ops, security, entropy, keys, noise, params |
| clockwork-core | 46 | Formal-spec RNS: bound tracking, GRO timing, Garner, integrity |
| exact_transcendentals | 143 | Exact transcendental functions via integer CORDIC |
| nexgen_rational | 95 | Exact i128 rational arithmetic (zero-dep) |
| fhe-service | 22 | FHE session management and serialization |
| mana | 30 | FHE stream accelerator, lane-parallel via Rayon |
| unhal | 10 | Hardware abstraction layer |

### Known State
- 11 CRITICAL gaps identified in NINE65_V6_EXECUTION_PLAN.md
- 13 HIGH priority gaps identified
- 14 Coq proofs, 4 Lean4 proofs already verified
- Integer-only compliance: zero float violations in runtime code
- Security: 128/192/256-bit configs with HE Standard v1.1 alignment
- Bootstrap: unlimited depth via Clockwork auto-refresh (46 bootstraps in 100-op chain)

---

## ENUMERATED AUDIT ITEMS

Each item is a self-contained audit scope. Agents analyze the specified files,
identify gaps/opportunities, and produce findings. One item per agent.

### 1. K-Elimination Arithmetic Correctness
**Files**: `crates/nine65/src/arithmetic/k_elimination.rs`, `proofs/coq/KElimination*.v`, `lean4/KElimination/`
**Focus**: Verify runtime implementation matches formal proof preconditions. Check that all Coq theorem guards (M > 0, A > 0, gcd(M,A) = 1, X < M*A) are enforced at call sites. Catalog any panic!/unwrap in non-test code. Assess gap C5 from execution plan.

### 2. NTT Engine Safety and Validation
**Files**: `crates/nine65/src/arithmetic/ntt.rs`, `crates/nine65/src/arithmetic/ntt_fft.rs`
**Focus**: Audit NTT construction for panics vs error returns. Check that try_new() validates (N is power of 2, (q-1) % 2N == 0, primitive root exists). Assess data-dependent branching for CT safety. Covers gaps C3/C7/H1 from execution plan.

### 3. Noise Budget Overflow and Tracking
**Files**: `crates/nine65/src/noise/`, `crates/nine65/src/noise/exact_noise.rs`, `crates/nine65/src/noise/budget.rs`
**Focus**: Verify noise budget uses checked arithmetic (no silent wraparound to negative). This is the IBM key recovery attack vector (gap C6). Check integration with evaluator. Verify millibit precision representation is sound.

### 4. Bootstrap and Clockwork Depth Verification
**Files**: `crates/nine65/src/ops/auto_bootstrap.rs`, `crates/nine65/src/ops/gso_fhe.rs`, `crates/nine65/src/ops/bootstrap.rs`
**Focus**: Verify the Clockwork Bootstrap mechanism (25% threshold trigger, auto-refresh, unlimited depth claim). Audit bootstrap prime chain validation. Check that modswitch rescaling is exact. Assess circular security (boot_sk = work_sk). Covers gaps C10/H2/H3.

### 5. Security Estimator and Parameter Configs
**Files**: `crates/nine65/src/params/secure_configs.rs`, `crates/nine65/src/params/security_estimator.rs`
**Focus**: Verify Core-SVP security estimates for all three config tiers (128/192/256). Check alignment with HE Standard v1.1 tables. Assess whether MATZOV dual attack model is implemented or needed (gap B7). Verify compile-time enforcement blocks test configs in release.

### 6. GRO Timing Gate and Side-Channel Posture
**Files**: `crates/nine65/src/security/gro_gate.rs`, `crates/nine65/src/security/secret_data.rs`, `crates/clockwork-core/src/timing.rs`
**Focus**: Audit GRO timing gate integration points (keygen, decrypt). Check constant-time primitives in secret_data.rs. Assess whether CT enforcement is active or still planned. Covers gaps C3/C4/C9/H6.

### 7. Key Management and Entropy Pipeline
**Files**: `crates/nine65/src/security/key_manager.rs`, `crates/nine65/src/entropy/`, `crates/nine65/src/entropy/shadow_entropy_monitor.rs`
**Focus**: Verify key lifecycle management (generation, storage, zeroization). Check entropy source health monitoring. Assess the shadow entropy harvester from CRT operations. Covers gaps C8/A7. Check for panic!/unwrap in keygen paths.

### 8. Error Handling Landscape (nine65 crate)
**Files**: `crates/nine65/src/` (all modules)
**Focus**: Catalog all panic!(), unwrap(), expect() in non-test code across the entire nine65 crate. Count and classify by severity. Verify Nine65Error has sufficient variants. Check error messages for leaked cryptographic values. Covers gaps C1/C2/C11/A8/H13.

### 9. Clockwork-Core Formal Specification Fidelity
**Files**: `crates/clockwork-core/src/`, `docs/CLOCKWORK_FORMAL_SPECIFICATION.md`
**Focus**: Verify that clockwork-core implementation (Garner reconstruction, bound tracking, GRO timing, integrity checking) matches the formal specification document. Identify any spec-implementation divergence. Check cross-validation with K-Elimination.

### 10. RNS-FHE Context and Homomorphic Operations
**Files**: `crates/nine65/src/ops/rns_fhe.rs`, `crates/nine65/src/ops/homomorphic.rs`, `crates/nine65/src/ops/rns_mul.rs`
**Focus**: Audit the core FHE operation pipeline (encrypt/add/multiply/decrypt). Check TrackedEvaluator noise integration. Verify rns_fhe.rs (7200 lines) for structural issues and refactoring opportunities. Covers gaps A6/H4/H12.

### 11. Exact Transcendentals Correctness
**Files**: `crates/exact_transcendentals/src/`
**Focus**: Verify integer CORDIC implementations for sin/cos/exp/log. Check that all outputs are exact or bounded with certified error. Verify integration with the integer-only mandate. Assess test coverage (143 tests) for edge cases.

### 12. Coq Proof Coverage and Formalization Index
**Files**: `proofs/coq/*.v`, `docs/FORMALIZATION_INDEX.md`
**Focus**: Audit all 14 Coq proofs for completeness (no admitted/sorry). Map each proof to its corresponding Rust module. Identify modules with Coq theorems that lack runtime enforcement. Assess formalization index completeness. Covers gaps H9/F1.

### 13. Mana Stream Accelerator and Parallel Safety
**Files**: `crates/mana/src/`
**Focus**: Audit the Rayon-based lane-parallel FHE accelerator. Check for thread-safety issues, data races, or non-determinism in parallel execution. Verify integration with the nine65 core crate. Assess the accelerated feature flag pathway.

### 14. FHE Service Layer and Session Management
**Files**: `crates/fhe-service/src/`
**Focus**: Audit session management, serialization (JSON + bincode), and TTL handling. Check for resource leaks, session exhaustion, or state corruption. Verify the dead-code warnings (new_with_ttl, ttl_seconds) are intentional or need cleanup.

---

## EXECUTION CONSTRAINTS

- **Single item per agent**: Each agent works one enumerated item only.
- **No cross-item dependencies**: Items are self-contained analysis scopes.
- **Evidence-based findings**: Every gap or recommendation must cite specific file:line.
- **Integer-only mandate**: All arithmetic assessments must verify exact integer/rational compliance.
- **Build command**: `cargo build --release --workspace` (run as user `acid` with login shell)
- **Test command**: `cargo test --workspace --release` (run as user `acid` with login shell)
- **No code modifications**: This is an analysis/audit pass. Findings feed into the next execution phase.

---

## DELIVERABLE STRUCTURE

Each agent produces a report with:
1. **Scope confirmation** - Which enumerated item was analyzed
2. **Current state assessment** - What works, what's tested, what's proven
3. **Gap inventory** - Specific issues found with file:line citations
4. **Risk assessment** - Impact of each gap (CRITICAL/HIGH/MEDIUM/LOW)
5. **Actionable recommendations** - Concrete next steps, ordered by priority
6. **Formalization candidates** - Items suitable for /formalization-swarm single-proof verification

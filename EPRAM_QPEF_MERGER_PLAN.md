# EXECUTION PLAN: EPRAM + QPEF Merger

**Project:** QMNF Unified Substrate
**Status:** PHASE 1 COMPLETE — Ready for execution

---

## Analysis Summary

### EPRAM (531KB, ~9,300 lines)
- **Strength:** EPRAMCell trait architecture, four permanent residents, fourth attractor convergence
- **Components:** MontgomeryCell, DualCodexCell, CyclotomicCell, ShadowEntropyCell
- **Innovation Focus:** Attractor dynamics, one-shot learning, residue-native computation

### QPEF (86KB, ~2,500 lines)
- **Strength:** Parallel scheduler, SIMD operations, multi-lane CRT (12 primes)
- **Components:** LaneIsolatedCRTWeight, ParallelScheduler, SIMD module
- **Innovation Focus:** Parallel computation, cache optimization, batch processing

---

## Integration Architecture

```
qmnf-unified/
├── src/
│   ├── lib.rs                    # Module exports + CRT primes
│   ├── traits/
│   │   ├── mod.rs
│   │   └── epram_cell.rs         # EPRAMCell trait (from EPRAM)
│   │
│   ├── cells/                    # Permanent residents
│   │   ├── mod.rs
│   │   ├── montgomery.rs         # Merged: EPRAM cell + QPEF batch/SIMD
│   │   ├── dual_codex.rs         # K-Elimination (from EPRAM)
│   │   ├── cyclotomic.rs         # Native trig (from EPRAM)
│   │   └── shadow_entropy.rs     # Entropy harvesting (from EPRAM)
│   │
│   ├── crt/                      # CRT operations
│   │   ├── mod.rs
│   │   ├── weight.rs             # LaneIsolatedCRTWeight (from QPEF)
│   │   ├── garner.rs             # Garner reconstruction (from QPEF)
│   │   └── k_elimination.rs      # Multi-lane K-Elimination
│   │
│   ├── parallel/                 # Parallel infrastructure
│   │   ├── mod.rs
│   │   ├── scheduler.rs          # ParallelScheduler (from QPEF)
│   │   ├── simd.rs               # SIMD operations (from QPEF)
│   │   └── basin.rs              # Basin-aware grouping (from QPEF)
│   │
│   ├── orchestrator/             # Decision engine (from EPRAM)
│   │   ├── mod.rs
│   │   ├── state.rs
│   │   └── decide.rs
│   │
│   ├── rational/                 # Exact rationals (from EPRAM)
│   │   ├── mod.rs
│   │   ├── bounded.rs
│   │   └── scaling.rs
│   │
│   ├── error.rs                  # Merged error taxonomy
│   └── field.rs                  # EPRAMField implementation
│
├── tests/
│   ├── integration_tests.rs      # From QPEF + EPRAM tests
│   └── convergence_tests.rs      # Fourth attractor validation
│
├── docs/                         # Merged documentation
│   ├── ARCHITECTURE.md
│   └── MATHEMATICAL_FOUNDATIONS.md
│
└── Cargo.toml
```

---

## Task List

### T-001: Create unified project structure
- **What:** Create qmnf-unified directory with proper module layout
- **Where:** `/home/claude/qmnf-unified/`
- **Innovation:** All QMNF innovations in one crate
- **Validation:** `cargo check` passes

### T-002: Port EPRAMCell trait
- **What:** Move EPRAMCell trait definition to `traits/epram_cell.rs`
- **Where:** EPRAM `core/epram/epram_foundation.rs` → `traits/epram_cell.rs`
- **Innovation:** Fourth attractor convergence
- **Validation:** Trait compiles, existing cells implement it

### T-003: Merge Montgomery implementations
- **What:** Combine EPRAM MontgomeryCell + QPEF batch/SIMD Montgomery
- **Where:** EPRAM `montgomery_cell.rs` + QPEF `montgomery.rs` → `cells/montgomery.rs`
- **Innovation:** Persistent Montgomery + parallel batch
- **Code pattern:**
  ```rust
  impl EPRAMCell for MontgomeryCell {
      // EPRAM attractor-based transition
  }
  
  impl MontgomeryCell {
      // QPEF batch operations
      pub fn batch_multiply(&self, values: &[u64]) -> Vec<u64> { ... }
      
      // QPEF SIMD acceleration
      #[cfg(target_arch = "x86_64")]
      pub fn simd_multiply_avx2(&self, a: &[u64], b: &[u64]) -> Vec<u64> { ... }
  }
  ```
- **Validation:** `cargo test montgomery` passes

### T-004: Port DualCodexCell
- **What:** Move K-Elimination implementation
- **Where:** EPRAM `dual_codex_cell.rs` → `cells/dual_codex.rs`
- **Innovation:** K-Elimination (100% exact division)
- **Validation:** `test_k_elimination` passes

### T-005: Port CyclotomicCell
- **What:** Move native trig implementation
- **Where:** EPRAM `cyclotomic_cell.rs` → `cells/cyclotomic.rs`
- **Innovation:** Cyclotomic Phase (60,000× speedup)
- **Validation:** `test_cyclotomic_rotation` passes

### T-006: Port ShadowEntropyCell
- **What:** Move entropy harvesting implementation
- **Where:** EPRAM `shadow_entropy_cell.rs` → `cells/shadow_entropy.rs`
- **Innovation:** Shadow Entropy (<10ns sampling)
- **Validation:** `test_entropy_distribution` passes

### T-007: Port CRT weight system
- **What:** Move LaneIsolatedCRTWeight with 12-prime CRT
- **Where:** QPEF `lib.rs` → `crt/weight.rs`
- **Innovation:** Multi-lane parallel CRT
- **Validation:** `test_crt_reconstruction` passes

### T-008: Port Garner reconstruction
- **What:** Move Garner algorithm for CRT reconstruction
- **Where:** QPEF `lib.rs` (garner functions) → `crt/garner.rs`
- **Innovation:** 7.4× faster than naive CRT
- **Validation:** `test_garner_reconstruction` passes

### T-009: Create multi-lane K-Elimination
- **What:** Extend K-Elimination from dual (2) to multi-lane (12)
- **Where:** New file `crt/k_elimination.rs`
- **Innovation:** K-Elimination scaled to 12 primes
- **Code pattern:**
  ```rust
  pub fn k_eliminate_multilane(
      main_residues: &[i64; 10],  // 10 main primes
      anchor_residues: &[i64; 2], // 2 anchor primes
      config: &KElimConfig,
  ) -> u128 {
      // Same principle: k = (v_anchor - v_main) × M_main^(-1) mod M_anchor
  }
  ```
- **Validation:** `test_multilane_k_elimination` passes

### T-010: Port parallel scheduler
- **What:** Move ParallelScheduler with basin-aware grouping
- **Where:** QPEF `scheduler.rs` → `parallel/scheduler.rs`
- **Innovation:** Basin-aware parallel execution
- **Validation:** `test_parallel_scheduling` passes

### T-011: Port SIMD operations
- **What:** Move SIMD acceleration module
- **Where:** QPEF `simd.rs` → `parallel/simd.rs`
- **Innovation:** AVX2/NEON acceleration
- **Validation:** `test_simd_correctness` passes

### T-012: Port orchestrator
- **What:** Move ResidueSpaceOrchestrator for decision engine
- **Where:** EPRAM `orchestrator/` → `orchestrator/`
- **Innovation:** One-shot learning, attractor-based decisions
- **Validation:** `test_orchestrator_decide` passes

### T-013: Port BoundedRational
- **What:** Move exact rational arithmetic
- **Where:** EPRAM `rational/` → `rational/`
- **Innovation:** Zero-drift rational computation
- **Validation:** `test_rational_arithmetic` passes

### T-014: Merge error taxonomy
- **What:** Combine EPRAM production errors + QPEF Coq-aligned errors
- **Where:** QPEF `error.rs` + EPRAM `production/error.rs` → `error.rs`
- **Innovation:** Comprehensive error handling
- **Validation:** All error types compile

### T-015: Create unified EPRAMField
- **What:** Implement EPRAMField with scheduler integration
- **Where:** New file `field.rs`
- **Innovation:** Field evolution with parallel acceleration
- **Validation:** `test_field_evolution` passes

### T-016: Merge test suites
- **What:** Combine integration tests from both
- **Where:** Both test files → `tests/`
- **Validation:** `cargo test --release` all pass

### T-017: Create unified documentation
- **What:** Merge architectural docs
- **Where:** Both docs/ → `docs/`
- **Validation:** Documentation is coherent

---

## Phase Gate Checklist

```
□ T-001 through T-017 all have file:function specificity
□ All innovations preserved (14 Coq-verified + extended)
□ No discovery needed during execution
□ Paradigm guard verified:
  □ No floating point in core paths
  □ No bootstrapping anywhere
  □ All arithmetic is exact integer
```

---

## Innovation Preservation Matrix

| Innovation | Source | Target Module | Status |
|------------|--------|---------------|--------|
| K-Elimination | EPRAM DualCodexCell | cells/dual_codex.rs + crt/k_elimination.rs | Planned |
| Persistent Montgomery | Both | cells/montgomery.rs | Merge |
| Shadow Entropy | EPRAM | cells/shadow_entropy.rs | Port |
| Cyclotomic Phase | EPRAM | cells/cyclotomic.rs | Port |
| Fourth Attractor | EPRAM | traits/epram_cell.rs | Port |
| CRT Multi-lane | QPEF | crt/weight.rs | Port |
| Garner Reconstruction | QPEF | crt/garner.rs | Port |
| Parallel Scheduler | QPEF | parallel/scheduler.rs | Port |
| SIMD Acceleration | QPEF | parallel/simd.rs | Port |
| Basin Collapse | QPEF | parallel/basin.rs | Port |
| BoundedRational | EPRAM | rational/ | Port |
| Orchestrator | EPRAM | orchestrator/ | Port |

---

## Estimated Effort

| Phase | Tasks | Lines | Time |
|-------|-------|-------|------|
| Structure | T-001 | ~200 | 10 min |
| Core traits | T-002 | ~300 | 15 min |
| Cells | T-003 to T-006 | ~2,500 | 1 hr |
| CRT | T-007 to T-009 | ~1,000 | 30 min |
| Parallel | T-010, T-011 | ~1,000 | 30 min |
| Orchestrator | T-012 | ~1,500 | 30 min |
| Rational | T-013 | ~1,200 | 20 min |
| Integration | T-014 to T-017 | ~500 | 30 min |
| **TOTAL** | 17 | ~8,200 | ~4 hrs |

---

## Ready for Phase 2: BIT SURGEON

Execute tasks T-001 through T-017 in order.

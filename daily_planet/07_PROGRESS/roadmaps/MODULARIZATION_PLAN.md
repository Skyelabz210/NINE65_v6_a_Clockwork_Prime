# QMNF System Modularization Plan

**Date:** December 11, 2025
**Status:** IN PROGRESS - Phase 4 Complete (4 modular crates, 48 tests)
**Goal:** Split monolithic hcvlang (186 files) into modular crates for faster compilation

---

## The Problem

Current state:
- hcvlang is ONE crate with 186 .rs files
- LLVM must optimize everything together
- Requires 6.4GB RAM just for rustc
- Takes 10+ minutes even with -j 2
- No incremental builds within hcvlang

Target state:
- Multiple small crates (~20-30 files each)
- Each compiles in 1-2 minutes with <2GB RAM
- Parallel compilation across crates
- Incremental builds work properly

---

## Proposed Crate Structure

### Layer 0: Foundation (No Dependencies)

#### `qmnf-primitives`
The absolute foundation - arbitrary precision integers.

**Files to include:**
```
src/
├── lib.rs
├── bigint_hcv.rs       (HCVLangBigInt - unlimited precision)
├── bigint_internal.rs  (internal helpers)
├── dcbigint.rs         (DCBigInt)
└── dcbigint_clean.rs   (clean implementation)
```

**External deps:** None (only std)
**Estimated size:** ~4 files, ~90KB

---

### Layer 1: Core Arithmetic (Depends on Layer 0)

#### `qmnf-arithmetic`
CRT-based fast arithmetic - the heart of QMNF.

**Files to include:**
```
src/
├── lib.rs
├── crt_bigint.rs           (CRTBigInt - fast bounded)
├── adaptive_crt_bigint.rs  (auto-promotion)
├── adaptive_crt_bigint_v1.rs
├── adaptive_crt_bigint_v2.rs
├── adaptive_crt_bigint_v3.rs
├── modint.rs               (Mersenne modular)
├── rational.rs             (exact rationals)
├── mod_rational.rs         (modular rationals)
├── garner.rs               (Garner reconstruction)
├── montgomery.rs           (Montgomery form)
├── nnt.rs                  (Number theoretic transforms)
├── intpair.rs              (integer pairs)
├── qphi.rs                 (Euler totient)
├── prime_gen.rs            (prime generation)
├── fast_arithmetic.rs      (optimized ops)
├── division_optimizer.rs   (division optimization)
├── fused_piggyback_division.rs (FPD algorithm)
└── modular_exponentiation.rs
```

**Dependencies:** `qmnf-primitives`
**Estimated size:** ~18 files, ~400KB

---

### Layer 2: Domain Modules (Depend on Layer 1)

#### `qmnf-polynomial`
PLMG exact polynomial arithmetic.

**Files to include:**
```
src/
├── lib.rs
├── polynomial/
│   ├── mod.rs
│   ├── polynomial.rs
│   ├── hensel.rs
│   ├── interpolation.rs
│   ├── ntt.rs
│   └── resultant.rs
├── plmg_core.rs        (Phase-Locked Modular Geometry)
├── kfree_crt.rs        (K-Free CRT)
└── exact_division.rs   (100% exact division)
```

**Dependencies:** `qmnf-arithmetic`
**Estimated size:** ~10 files, ~150KB

---

#### `qmnf-fhe`
Homomorphic encryption operations.

**Files to include:**
```
src/
├── lib.rs
├── fhe/
│   ├── mod.rs
│   ├── params.rs
│   ├── keys.rs
│   ├── encrypt.rs
│   ├── polynomial.rs
│   ├── operations.rs
│   ├── noise.rs
│   ├── qmnf_noise.rs
│   ├── rns.rs
│   └── batch_operations.rs
├── fhe.rs              (main FHE module)
├── fhe_realtime/       (realtime FHE)
│   └── *.rs
└── ahop.rs             (AHOP operations)
```

**Dependencies:** `qmnf-arithmetic`
**Estimated size:** ~15 files, ~200KB

---

#### `qmnf-diagnostics`
System diagnostics and monitoring.

**Files to include:**
```
src/
├── lib.rs
├── diagnostics/
│   ├── mod.rs
│   ├── wire_protocol.rs
│   ├── probe_infrastructure.rs
│   ├── metrics_collection.rs
│   ├── anomaly_detection.rs
│   ├── response_system.rs
│   ├── gso_integration.rs
│   ├── entropy_integration.rs
│   └── float_guard_integration.rs
└── benchmarking.rs
```

**Dependencies:** `qmnf-arithmetic`
**Estimated size:** ~10 files, ~100KB

---

#### `qmnf-neural`
Neural network primitives.

**Files to include:**
```
src/
├── lib.rs
├── neural/
│   └── *.rs
├── resnet/
│   └── *.rs
├── neural_primitives.rs
├── neural.rs
├── crt_conv_layer.rs
├── fp_dense_layer.rs
├── modular_relu.rs
└── qmnf_weight.rs
```

**Dependencies:** `qmnf-arithmetic`
**Estimated size:** ~15 files, ~200KB

---

#### `qmnf-crypto`
Post-quantum and advanced cryptography.

**Files to include:**
```
src/
├── lib.rs
├── pqc/
│   ├── mod.rs
│   ├── lattice.rs
│   ├── hash_based.rs
│   ├── code_based.rs
│   ├── multivariate.rs
│   ├── isogeny.rs
│   └── health.rs
├── entropy_shadow.rs
├── swarm_gso.rs
└── quantum_classical_bridge.rs
```

**Dependencies:** `qmnf-fhe`, `qmnf-diagnostics`
**Estimated size:** ~12 files, ~200KB

---

#### `qmnf-storage`
Storage and persistence systems.

**Files to include:**
```
src/
├── lib.rs
├── storage/
│   └── *.rs
└── holodrive_vsa.rs
```

**Dependencies:** `qmnf-primitives`
**Estimated size:** ~5 files, ~50KB

---

### Layer 3: Integration (Depends on All)

#### `qmnf-orchestration`
MANA orchestration and execution.

**Files to include:**
```
src/
├── lib.rs
├── mana_orchestration.rs
├── double_helix.rs
├── qpef_core.rs
├── qpef_executor.rs
├── qpef_scheduler.rs
├── qpef_transaction.rs
├── qpef_codex_bridge.rs
├── qpef_neural.rs
├── tnu_core.rs
├── deterministic_task_scheduler.rs
├── attractor_memory.rs
├── codex_gear_manifold.rs
├── dual_codex.rs
├── cdhs_core.rs
└── time_crystal.rs
```

**Dependencies:** `qmnf-arithmetic`, `qmnf-neural`
**Estimated size:** ~16 files, ~300KB

---

### Layer 4: Facade (Re-exports Everything)

#### `hcvlang`
Thin facade that re-exports all modules + FFI.

**Files to include:**
```
src/
├── lib.rs              (pub use all crates)
├── ffi_minimal.rs      (Python bindings)
└── qmnf_ffi_boundary.rs
```

**Dependencies:** All above crates
**Estimated size:** ~3 files, minimal code

---

## Dependency Graph

```
Layer 0:  qmnf-primitives (foundation)
              │
Layer 1:  qmnf-arithmetic (core)
              │
         ┌────┼────┬─────────┬────────┐
         │    │    │         │        │
Layer 2: polynomial  fhe  diagnostics  neural  storage
         │    │    │         │
         │    └────┴─────────┘
         │         │
Layer 3:      qmnf-crypto    qmnf-orchestration
              │              │
         ┌────┴──────────────┘
         │
Layer 4: hcvlang (facade + FFI)
```

---

## Migration Strategy

### Phase 1: Extract Foundation ✅ COMPLETE

**Completed:** December 11, 2025

1. ✅ Created `crates/qmnf-primitives/` with:
   - `Cargo.toml` (zero external dependencies)
   - `src/lib.rs` (module entry point)
   - `src/bigint.rs` (HCVLangBigInt - unlimited precision integers)
   - `benches/bigint_benchmark.rs` (Criterion benchmarks)

2. ✅ Build results:
   - **Build time:** 1.84 seconds (vs 10+ minutes for monolithic)
   - **Tests:** 5/5 unit tests + 1 doc test passing
   - **RAM:** Minimal (~100MB vs 6.4GB)

3. ⏳ TODO: Update hcvlang to depend on qmnf-primitives (Phase 5)

### Phase 2: Extract Core Arithmetic ✅ COMPLETE

**Completed:** December 11, 2025

1. ✅ Created `crates/qmnf-arithmetic/` with:
   - `Cargo.toml` (depends on qmnf-primitives + serde)
   - `src/lib.rs` (module entry point)
   - `src/modint.rs` (ModInt - Mersenne prime modular arithmetic)
   - `src/crt.rs` (CRTBigInt - CRT-based bounded integers)
   - `src/rational.rs` (Rational - exact rational numbers)
   - `benches/arithmetic_benchmark.rs` (Criterion benchmarks)

2. ✅ Build results:
   - **Build time:** 8.28 seconds (vs 10+ minutes for monolithic)
   - **Tests:** 15/15 unit tests passing
   - **RAM:** Minimal (~200MB vs 6.4GB)

3. ⏳ TODO: Update hcvlang to depend on qmnf-arithmetic (Phase 5)

### Phase 3: Extract Polynomial Module ✅ COMPLETE

**Completed:** December 11, 2025

1. ✅ Created `crates/qmnf-polynomial/` with:
   - `Cargo.toml` (depends on qmnf-arithmetic)
   - `src/lib.rs` (module entry point)
   - `src/polynomial.rs` (KFreePolynomial - exact polynomial arithmetic)
   - `benches/polynomial_benchmark.rs` (Criterion benchmarks)

2. ✅ Build results:
   - **Build time:** 0.70 seconds
   - **Tests:** 6/6 unit tests passing
   - **RAM:** Minimal

3. ⏳ Additional polynomial submodules (ntt, hensel, interpolation, resultant) can be added later

### Phase 4: Extract FHE Module ✅ COMPLETE

**Completed:** December 11, 2025

1. ✅ Added NNT (Number Theoretic Transform) to qmnf-arithmetic:
   - `src/nnt.rs` (Cooley-Tukey NNT with Fermat prime Q=65537)
   - 5 tests passing

2. ✅ Created `crates/qmnf-fhe/` with:
   - `Cargo.toml` (depends on qmnf-arithmetic + rand + rand_chacha)
   - `src/lib.rs` (module entry point)
   - `src/params.rs` (FHE security parameters - Toy/128/192/256-bit)
   - `src/error.rs` (FHE error types)
   - `src/polynomial.rs` (FHE polynomial ring Z_q[X]/(X^N + 1))
   - `benches/fhe_benchmark.rs` (Criterion benchmarks)

3. ✅ Build results:
   - **Build time:** 3.48 seconds (4 modular crates)
   - **Tests:** 12/12 FHE tests passing, 47 total tests across all modular crates
   - **RAM:** Minimal (~200MB vs 6.4GB)

### Phase 5: Extract Remaining Domain Modules (PENDING)
1. qmnf-diagnostics - System diagnostics and monitoring
2. qmnf-neural - Neural network primitives
3. qmnf-orchestration - MANA orchestration

### Phase 6: Finalize Facade (PENDING)
1. Reduce hcvlang to thin re-export facade
2. Update FFI bindings
3. Verify Python bindings still work

---

## Current Progress Summary

| Crate | Status | Tests | Build Time |
|-------|--------|-------|------------|
| qmnf-primitives | ✅ Complete | 6 | 1.84s |
| qmnf-arithmetic | ✅ Complete | 24 | 8.28s |
| qmnf-polynomial | ✅ Complete | 6 | 0.70s |
| qmnf-fhe | ✅ Complete | 12 | 3.48s |
| **Total** | **4 crates** | **48 tests** | **<15s** |

---

## Build Benefits

| Metric | Current (Monolithic) | Target (Modular) |
|--------|---------------------|------------------|
| RAM per crate | 6.4GB | <2GB |
| Build time | 10+ min | 2-3 min |
| Incremental builds | No | Yes |
| Parallel crates | No | Yes (8 cores = 8 crates) |
| Works on 8GB RAM | Barely | Easily |

---

## Files Not Yet Assigned

These files need review for placement:
- geometric.rs, apollonian.rs → qmnf-arithmetic?
- math/*.rs → qmnf-arithmetic?
- consciousness_engine/*.rs → qmnf-orchestration?
- nsa_calculus/*.rs → qmnf-arithmetic?
- exact_type_system.rs, exact_stdlib.rs, exact_runtime.rs, exact_compiler.rs → new exact crate?
- core_types.rs → qmnf-primitives?
- boundary_flags.rs → qmnf-diagnostics?
- category_theory.rs → qmnf-arithmetic?
- coprime_cascade.rs → qmnf-arithmetic?

---

## Next Steps

1. **Immediate:** Create `qmnf-primitives` crate with bigint
2. **Test:** Verify it builds independently with <1GB RAM
3. **Iterate:** Extract one crate at a time, always maintaining working builds
4. **FFI:** Keep FFI working throughout migration

---

**Document:** MODULARIZATION_PLAN.md
**Author:** QMNF Team
**Status:** Ready for Phase 1 implementation

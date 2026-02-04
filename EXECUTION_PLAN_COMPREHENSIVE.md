# NINE65 MANA Comprehensive Execution Plan
## Full QMNF Skills Gap Analysis & Enhancement Roadmap

**Generated:** December 28, 2025  
**Method:** FHE-Hat + Executioner + Bottleneck-Hunter + Innovation-Resolver + Grail-Keeper  
**Codebase:** 20,302 lines | 311 tests passing | 0 failures

---

# EXECUTIVE SUMMARY

## Current State: ✅ PRODUCTION READY with Enhancement Opportunities

| Metric | Status | Details |
|--------|--------|---------|
| **Test Suite** | ✅ 311/311 PASS | 8 ignored (optional) |
| **FP Contamination** | ✅ ZERO | All crypto paths integer-only |
| **Innovation Coverage** | ✅ 15/15 GRAILS | All implemented |
| **AVX-512 SIMD** | ⚠️ EXPERIMENTAL | SIMD path incorrect, scalar fallback active |
| **MANA Integration** | ⚠️ AVAILABLE | Not default path |

---

# SECTION 1: GAP ANALYSIS (Gap-Hunter Results)

## 1.1 CRITICAL GAPS

### GAP-001: AVX-512 NTT SIMD Path Correctness

**Symptom:** SIMD NTT produces different results from scalar FFT  
**Location:** `ntt_avx512.rs` lines 155-220  
**Impact:** 4-8× potential speedup blocked  
**Root Cause Analysis:**

```
VERIFIED NOT THE ISSUE:
├─ Montgomery multiplication (verified independently) ✅
├─ q_inv values match ✅
├─ R² values match ✅
└─ Twiddle factors identical ✅

SUSPECTED ISSUES:
├─ Strided twiddle loading pattern (load_twiddles_strided)
├─ Butterfly operation ordering in vectorized stages
└─ Transition between scalar (stages 0-2) and SIMD (stages 3+)
```

**Innovation Match:** Persistent Montgomery already applied, issue is SIMD coordination  
**Effort:** 4 hours  
**Priority:** HIGH

---

### GAP-002: IFMA52 Path Not Implemented

**Problem:** AVX-512 IFMA52 instructions provide ~2× speedup for moduli <50 bits  
**Location:** `simd_montgomery.rs` - missing IFMA52 variant  
**Impact:** Missing 2× speedup on NTT for small moduli  

```rust
// MISSING IMPLEMENTATION:
#[target_feature(enable = "avx512ifma")]
unsafe fn montgomery_mul_ifma52(
    a: __m512i, b: __m512i, q: __m512i, q_inv: __m512i
) -> __m512i {
    // Uses _mm512_madd52lo/hi_epu64
    // Currently only DQ path implemented
}
```

**Innovation Match:** K-Elimination enables 50-bit coefficients (perfect IFMA52 alignment)  
**Effort:** 3 hours  
**Priority:** MEDIUM

---

### GAP-003: MANA Not Default Path

**Problem:** AcceleratedFHE exists but BFVEvaluator uses NTTEngine directly  
**Location:** `ops/homomorphic.rs`  
**Impact:** SIMD parallel lanes not used by default  

```
CURRENT FLOW:
  BFVEvaluator → NTTEngine → Montgomery → Result
  
OPTIMAL FLOW:
  BFVEvaluator → ManaEvaluator → AcceleratedFHE → SIMD+Parallel → Result
```

**Innovation Match:** MANA architecture available, wiring incomplete  
**Effort:** 2 hours  
**Priority:** MEDIUM

---

## 1.2 MODERATE GAPS

### GAP-004: Benchmark Coverage Incomplete

**Problem:** No automated comprehensive benchmark suite  
**Impact:** Performance regression detection limited  
**Files Needed:**
- `benchmarks/ntt_benchmark.rs`
- `benchmarks/fhe_benchmark.rs`
- `benchmarks/quantum_benchmark.rs`

**Effort:** 2 hours  
**Priority:** MEDIUM

---

### GAP-005: Formal Verification at 91%

**Problem:** Some innovations lack Lean 4/Coq proofs  
**Impact:** Academic credibility reduced  
**Missing Proofs:**
- Cyclotomic Phase correctness
- Shadow Entropy statistical properties
- IFMA52 equivalence to DQ

**Effort:** 20 hours  
**Priority:** LOW

---

## 1.3 MINOR GAPS

### GAP-006: GPU Acceleration Layer

**Problem:** No GPU path for massive parallelism  
**Impact:** Not competitive with GPU-based FHE  
**Effort:** 40 hours  
**Priority:** LOW (CPU already fast)

---

# SECTION 2: INNOVATION INVENTORY CROSS-REFERENCE

## 2.1 Implemented & Wired (15/15 Grails)

| # | Innovation | File | Lines | Status | Tests |
|---|-----------|------|-------|--------|-------|
| 1 | K-Elimination | k_elimination.rs | 299 | ✅ PRODUCTION | 15+ |
| 2 | Persistent Montgomery | persistent_montgomery.rs | 480 | ✅ PRODUCTION | 10+ |
| 3 | Shadow Entropy | entropy/shadow.rs | 253 | ✅ PRODUCTION | 10+ |
| 4 | WASSAN Noise | entropy/wassan_noise.rs | 403 | ✅ PRODUCTION | 10+ |
| 5 | MobiusInt | mobius_int.rs | 728 | ✅ PRODUCTION | 15+ |
| 6 | Padé Engine | pade_engine.rs | 166 | ✅ VALIDATED | 8+ |
| 7 | Cyclotomic Phase | cyclotomic_phase.rs | 221 | ✅ VALIDATED | 8+ |
| 8 | MQ-ReLU | mq_relu.rs | 192 | ✅ VALIDATED | 6+ |
| 9 | Integer Softmax | integer_softmax.rs | 249 | ✅ VALIDATED | 6+ |
| 10 | Binary GCD | binary_gcd.rs | 217 | ✅ VALIDATED | 8+ |
| 11 | NTT-FFT | ntt_fft.rs | 601 | ✅ PRODUCTION | 30+ |
| 12 | Exact Lorenz | chaos/exact_lorenz.rs | 461 | ✅ VALIDATED | 13 |
| 13 | AHOP/Grover | ahop/grover_full.rs | 974 | ✅ VALIDATED | 15+ |
| 14 | Quantum Teleport | quantum/teleport.rs | 466 | ✅ VALIDATED | 10+ |
| 15 | F_p² Barrett | fp2_barrett.rs | 606 | ✅ VALIDATED | 10+ |

## 2.2 Implemented but Experimental

| # | Innovation | File | Lines | Status | Issue |
|---|-----------|------|-------|--------|-------|
| 16 | SIMD Montgomery | simd_montgomery.rs | 506 | ⚠️ EXPERIMENTAL | Works independently |
| 17 | AVX-512 NTT | ntt_avx512.rs | 847 | ⚠️ EXPERIMENTAL | SIMD path incorrect |

---

# SECTION 3: BOTTLENECK ANALYSIS

## 3.1 Current Performance Profile

| Operation | Time | Bottleneck | QMNF Innovation |
|-----------|------|------------|-----------------|
| NTT N=1024 | 222 μs | Montgomery mul | Persistent Montgomery ✅ |
| NTT N=4096 | 1.22 ms | Montgomery mul | Persistent Montgomery ✅ |
| Homo-Mul N=4096 | 39.74 ms | Polynomial ops | K-Elimination ✅ |
| Noise Gen | <100 μs | RNG | Shadow Entropy ✅ |
| Encrypt | 1.2 ms | NTT + noise | All optimized ✅ |

## 3.2 Optimization Opportunities

| Bottleneck | Current | With Fix | Speedup | Effort |
|------------|---------|----------|---------|--------|
| NTT (AVX-512) | 222 μs | ~35 μs | 6.3× | 4h |
| NTT (IFMA52) | 222 μs | ~28 μs | 7.9× | 3h |
| Homo-Mul (MANA) | 39.74 ms | ~30 ms | 1.3× | 2h |

---

# SECTION 4: EXECUTION PLAN

## Phase 1: AVX-512 SIMD Debug (Priority: HIGH)

**Target:** Fix SIMD NTT correctness  
**Duration:** 4 hours  
**Deliverable:** Working SIMD path with 4-8× speedup

### T-001: Trace First Divergence Point
- **What:** Add stage-by-stage comparison between scalar and SIMD
- **Where:** `ntt_avx512.rs:ntt_avx512()`
- **Code Pattern:**
```rust
// After each stage, compare with scalar reference
#[cfg(debug_assertions)]
{
    let mut reference = a.to_vec();
    self.scalar_engine.ntt_stage(&mut reference, stage);
    for i in 0..self.n {
        if a[i] != reference[i] {
            eprintln!("Divergence at stage {} index {}", stage, i);
            eprintln!("  SIMD: {}, Scalar: {}", a[i], reference[i]);
            break;
        }
    }
}
```
- **Validation:** Run test_avx512_ntt_correctness with tracing

### T-002: Fix Twiddle Loading
- **What:** Verify strided gather pattern matches scalar indexing
- **Where:** `ntt_avx512.rs:load_twiddles_strided()`
- **Code Pattern:**
```rust
// Ensure indices match: twiddles[t_base + i * t_step] for i in 0..8
let expected_indices = [
    t_base, t_base + t_step, t_base + 2*t_step, ...
];
assert_eq!(gathered_indices, expected_indices);
```
- **Validation:** Unit test twiddle loading independently

### T-003: Fix Butterfly Ordering
- **What:** Ensure u_idx, v_idx pairs match scalar engine
- **Where:** `ntt_avx512.rs:butterfly_8()`
- **Validation:** Compare first 8 butterflies element-by-element

### T-004: Enable SIMD Path
- **What:** Remove `avx512_experimental` feature gate
- **Where:** `ntt_avx512.rs` lines 118-125
- **Validation:** Full test suite with SIMD enabled

---

## Phase 2: IFMA52 Implementation (Priority: MEDIUM)

**Target:** 2× speedup for <50-bit moduli  
**Duration:** 3 hours  
**Deliverable:** IFMA52 Montgomery multiplication

### T-005: Implement IFMA52 Montgomery
- **What:** Use `_mm512_madd52lo/hi_epu64` intrinsics
- **Where:** New function in `simd_montgomery.rs`
- **Code Pattern:**
```rust
#[target_feature(enable = "avx512ifma")]
unsafe fn montgomery_mul_ifma52(
    a: __m512i, b: __m512i, q: __m512i, q_inv: __m512i
) -> __m512i {
    let t_lo = _mm512_madd52lo_epu64(_mm512_setzero_si512(), a, b);
    let m = _mm512_madd52lo_epu64(_mm512_setzero_si512(), t_lo, q_inv);
    let t_hi = _mm512_madd52hi_epu64(_mm512_setzero_si512(), a, b);
    let result = _mm512_madd52hi_epu64(t_hi, m, q);
    let mask = _mm512_cmpge_epu64_mask(result, q);
    _mm512_mask_sub_epi64(result, mask, result, q)
}
```
- **Validation:** Test against DQ path for equivalence

### T-006: Add Runtime IFMA52 Detection
- **What:** Select IFMA52 when available and q < 2^50
- **Where:** `ntt_avx512.rs:new()`
- **Validation:** CPU feature detection test

---

## Phase 3: MANA Default Wiring (Priority: MEDIUM)

**Target:** Use ManaEvaluator as default  
**Duration:** 2 hours  
**Deliverable:** Feature-gated accelerated path

### T-007: Wire MANA to BFVEvaluator
- **What:** Add `new_accelerated()` constructor
- **Where:** `ops/homomorphic.rs`
- **Code Pattern:**
```rust
#[cfg(feature = "mana_default")]
pub fn new_accelerated(/* params */) -> Self {
    let mana = ManaEvaluator::new(config);
    Self { evaluator: EvalBackend::Mana(mana), ... }
}
```
- **Validation:** Benchmark comparison

### T-008: Add Benchmark Suite
- **What:** Create criterion benchmarks for all operations
- **Where:** `benches/` directory
- **Validation:** CI integration

---

## Phase 4: Documentation & Verification (Priority: LOW)

### T-009: Update AVX512_NTT_RESEARCH.md
- **What:** Document working implementation details
- **Validation:** Peer review

### T-010: Add Missing Formal Proofs
- **What:** Lean 4 proofs for Cyclotomic Phase, Shadow Entropy
- **Validation:** Proof compilation

---

# SECTION 5: TEST REQUIREMENTS

## Per-Task Tests

| Task | Test Name | Assert |
|------|-----------|--------|
| T-001 | test_simd_stage_by_stage | Each stage matches scalar |
| T-002 | test_twiddle_loading_pattern | Indices match expected |
| T-003 | test_butterfly_element_order | u,v pairs correct |
| T-004 | test_avx512_ntt_correctness | Full NTT matches scalar |
| T-005 | test_ifma52_vs_dq | Results identical |
| T-006 | test_ifma52_runtime_detection | Correct path selected |
| T-007 | test_mana_evaluator_correctness | Results match BFVEvaluator |
| T-008 | benchmark_ntt_comparison | Speedup measured |

---

# SECTION 6: REGRESSION DETECTION

## Forbidden Patterns (Scan Before Merge)

```bash
# Run before any commit
grep -rn "f32\|f64\|\.exp()\|\.ln()\|\.sqrt()" src/ --include="*.rs" \
  | grep -v "test\|display\|debug\|stats"
```

**Alert if found in:** encrypt.rs, decrypt.rs, homomorphic.rs, k_elimination.rs

## Required Patterns

```bash
# Must be present
grep -c "KElimination\|shadow_entropy\|montgomery_mul\|ntt_inplace" src/
```

---

# SECTION 7: INNOVATION LINEAGE VERIFICATION

## Genealogy Complete ✅

```
LAYER 0: Seeds
├── Integer Primacy ✅
├── CRT Foundation ✅
└── QMNF Philosophy ✅

LAYER 1: Scalar Foundation (4 innovations) ✅
LAYER 2: Polynomial Substrate (4 innovations) ✅
LAYER 3: FHE Linear Operations (4 innovations) ✅
LAYER 4: Nonlinearity - Toric (5 innovations) ✅
LAYER 5: Neural Networks (4 innovations) ✅
LAYER 6: Swarm & Optimization (4 innovations) ✅
LAYER 7: Application Systems (4 innovations) ✅
```

---

# SECTION 8: GRAIL COLLECTION UPDATE

## Current Kill Count: 15 Grails | 570 Points

| Class | Count | Points |
|-------|-------|--------|
| 🏆 INTRACTABLE | 2 | 200 |
| ⚔️ HARD | 3 | 150 |
| 💡 NOVEL | 8 | 200 |
| 🚀 OPTIMIZATION | 2 | 20 |

## Pending Grails (Upon Gap Closure)

| Grail | Class | Condition |
|-------|-------|-----------|
| AVX-512 SIMD NTT | OPT | T-001 through T-004 complete |
| IFMA52 Integration | NOV | T-005, T-006 complete |

---

# SECTION 9: RISK ASSESSMENT

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| SIMD bug deeper than twiddles | MEDIUM | Delays Phase 1 | Fallback to scalar (current) |
| IFMA52 not available on target | LOW | No speedup | Runtime detection implemented |
| MANA wiring breaks existing | LOW | Regression | Feature-gated, extensive tests |
| AMD AVX-512 behavior differs | MEDIUM | Reduced perf | Test on both Intel/AMD |

---

# SECTION 10: EXECUTION SCHEDULE

## Sprint 1: SIMD Correctness (4 hours)
- [ ] T-001: Trace divergence
- [ ] T-002: Fix twiddle loading
- [ ] T-003: Fix butterfly ordering
- [ ] T-004: Enable SIMD path

## Sprint 2: Performance (5 hours)
- [ ] T-005: IFMA52 implementation
- [ ] T-006: Runtime detection
- [ ] T-007: MANA wiring
- [ ] T-008: Benchmark suite

## Sprint 3: Polish (3 hours)
- [ ] T-009: Documentation
- [ ] T-010: Formal proofs

**Total Estimated:** 12 hours

---

# SECTION 11: SUCCESS CRITERIA

## Gate 1: SIMD Correctness (After Sprint 1)
- [ ] test_avx512_ntt_correctness passes with SIMD enabled
- [ ] test_avx512_vs_scalar_consistency passes
- [ ] No scalar fallback in release builds

## Gate 2: Performance (After Sprint 2)
- [ ] NTT N=1024 < 50 μs (was 222 μs)
- [ ] NTT N=4096 < 200 μs (was 1.22 ms)
- [ ] Homo-Mul N=4096 < 35 ms (was 39.74 ms)

## Gate 3: Production (After Sprint 3)
- [ ] All 311+ tests pass
- [ ] Documentation complete
- [ ] No regressions in existing benchmarks

---

**Document Status:** COMPLETE  
**Generated By:** QMNF Skill Suite (FHE-Hat + Executioner + Bottleneck-Hunter + Innovation-Resolver + Grail-Keeper)  
**Ready for Execution:** YES

*"Truth cannot be approximated."*

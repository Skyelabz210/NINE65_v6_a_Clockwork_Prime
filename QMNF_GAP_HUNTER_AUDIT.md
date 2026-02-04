# 🔍 QMNF GAP HUNTER AUDIT

**Target**: NTT Integration with MANA/UNHAL Stack  
**Audit Date**: 2026-01-07  
**Auditor**: Gap Hunter (QMNF-aware)  
**Status**: 🔴 CRITICAL INTEGRATION GAP

---

## EXECUTIVE SUMMARY

The optimized NTT (44× speedup achieved) is **completely bypassing** the MANA/UNHAL acceleration stack. This defeats the purpose of having a unified hardware abstraction layer.

| Component | Expected | Actual | Gap |
|-----------|----------|--------|-----|
| NTT dispatch | Via UNHAL Accelerator | Direct call | 🔴 MISSING |
| Lane parallelism | Via ParallelNTT | None | 🔴 MISSING |
| ManaStream integration | Yes | None | 🔴 MISSING |
| accelerated.rs NTT ops | Present | Not implemented | 🔴 MISSING |

---

## THE ARCHITECTURE (How It Should Work)

```
┌──────────────────────────────────────────────────────────────┐
│  LAYER 3: FHE (NINE65)                                       │
│  homo_mul() → needs NTT for polynomial multiply              │
├──────────────────────────────────────────────────────────────┤
│  LAYER 2: RUNTIME (MANARuntime)                              │
│  ManaStream parallelization across RNS lanes                 │
│  ParallelNTT::ntt_all_lanes() - process all lanes together   │
├──────────────────────────────────────────────────────────────┤
│  LAYER 1: COMPUTE (UNHAL Accelerator)                        │
│  accel.ntt_streams() - dispatch to best backend              │
│  Sequential | SIMD | Parallel | Full                         │
├──────────────────────────────────────────────────────────────┤
│  LAYER 0: VALUES (ManaValue)                                 │
│  CRT residues in lanes, ready for parallel NTT               │
└──────────────────────────────────────────────────────────────┘
```

---

## WHAT CURRENTLY EXISTS

### In MANA crate (mana/src/parallel.rs):
```rust
/// Parallel NTT operations (for polynomial multiplication)
pub struct ParallelNTT;

impl ParallelNTT {
    /// Apply NTT to each lane in parallel
    pub fn ntt_all_lanes<F>(lanes: &[Vec<u64>], ntt_fn: F) -> Vec<Vec<u64>>
    where
        F: Fn(&[u64]) -> Vec<u64> + Sync,
    {
        lanes.par_iter().map(|lane| ntt_fn(lane)).collect()
    }
}
```

**STATUS**: Exists but NEVER USED by NINE65!

### In UNHAL crate (unhal/src/accelerator.rs):
```rust
impl Accelerator {
    pub fn add_streams(&self, a: &ManaStream, b: &ManaStream) -> ManaStream { ... }
    pub fn sub_streams(&self, a: &ManaStream, b: &ManaStream) -> ManaStream { ... }
    pub fn mul_streams(&self, a: &ManaStream, b: &ManaStream) -> ManaStream { ... }
    // NO NTT METHOD!
}
```

**STATUS**: Has add/sub/mul but NO NTT dispatch!

### In NINE65 accelerated.rs:
```rust
impl AcceleratedFHE {
    pub fn add_rns_accelerated(...) { ... }  // ✓ Uses MANA/UNHAL
    pub fn sub_rns_accelerated(...) { ... }  // ✓ Uses MANA/UNHAL
    pub fn mul_rns_coeffwise(...) { ... }    // ✓ Uses MANA/UNHAL (Hadamard)
    // NO POLYNOMIAL MULTIPLY VIA NTT!
}
```

**STATUS**: Coefficent ops use accelerator, but POLY MULTIPLY (which needs NTT) does NOT!

### In NINE65 homomorphic.rs:
```rust
pub fn mul(&self, ct1: &Ciphertext, ct2: &Ciphertext) -> Ciphertext3 {
    // Polynomial multiplication via NTT - DIRECT CALL, BYPASSES STACK
    let d0 = ct1.c0.mul(&ct2.c0, self.ntt);  // <-- Direct NTT call
    // ...
}
```

**STATUS**: homo_mul uses NTT DIRECTLY, bypassing MANA/UNHAL entirely!

---

## THE GAPS

### GAP-001: UNHAL Accelerator Missing NTT Operations 🔴

**Location**: `unhal/src/accelerator.rs`

**Missing**:
```rust
impl Accelerator {
    /// NTT polynomial multiply using best available path
    pub fn ntt_multiply_streams(
        &self,
        a: &ManaStream,
        b: &ManaStream,
        ntt_engine: &NTTEngineFFT,
    ) -> ManaStream {
        // Use ParallelNTT to process all lanes in parallel
        // Each lane gets its own NTT/pointwise-mul/INTT
    }
    
    /// NTT forward transform on stream
    pub fn ntt_forward_stream(
        &self,
        stream: &ManaStream,
        ntt_engine: &NTTEngineFFT,
    ) -> ManaStream {
        // Parallel NTT across all lanes
    }
    
    /// NTT inverse transform on stream
    pub fn ntt_inverse_stream(
        &self,
        stream: &ManaStream,
        ntt_engine: &NTTEngineFFT,
    ) -> ManaStream {
        // Parallel INTT across all lanes
    }
}
```

### GAP-002: ParallelNTT Never Used 🔴

**Location**: `mana/src/parallel.rs` → `nine65/src/ops/homomorphic.rs`

**Current**: ParallelNTT exists but homo_mul calls NTT directly on single polynomial

**Should be**:
```rust
// In homo_mul, for RNS representation with k lanes:
let ntt_fn = |lane: &[u64]| ntt_engine.ntt(lane);
let lanes_ntt = ParallelNTT::ntt_all_lanes(&rns_poly.limbs, ntt_fn);
// Process all k lanes in parallel with Rayon!
```

### GAP-003: accelerated.rs Missing poly_mul_accelerated 🔴

**Location**: `nine65/src/accelerated.rs`

**Has**: add_rns_accelerated, sub_rns_accelerated, mul_rns_coeffwise
**Missing**: poly_mul_accelerated (NTT-based polynomial multiplication)

```rust
/// Accelerated polynomial multiplication using NTT
/// 
/// This is the critical operation for homo_mul!
pub fn poly_mul_accelerated(
    &self,
    a: &RNSPolynomial,
    b: &RNSPolynomial,
    ntt_engines: &[NTTEngineFFT],  // One per RNS prime
) -> RNSPolynomial {
    // Convert to streams
    let stream_a = self.rns_to_stream(a);
    let stream_b = self.rns_to_stream(b);
    
    // NTT multiply through accelerator
    let result = self.accel.ntt_multiply_streams(&stream_a, &stream_b, ...);
    
    self.stream_to_rns(&result)
}
```

### GAP-004: NTTEngineFFT Not Multi-Prime Aware 🟡

**Location**: `nine65/src/arithmetic/ntt_fft.rs`

The optimized NTT works on single prime modulus. For RNS, need k separate NTT engines (one per prime). This is fine, but integration should handle it.

---

## PERFORMANCE IMPACT

### Current Path (Bypasses MANA/UNHAL):
```
homo_mul → poly.mul() → NTTEngineFFT.multiply()
                               ↑
                         Single-threaded per polynomial
                         No lane parallelism
                         Ignores ParallelNTT
```

### Optimal Path (Uses MANA/UNHAL):
```
homo_mul → accel.ntt_multiply_streams()
                      ↓
              ParallelNTT::ntt_all_lanes()
                      ↓
              k lanes processed in parallel (Rayon)
                      ↓
              Each lane: NTTEngineFFT.multiply()
```

**Expected Speedup**: With k RNS primes (typically 4-8):
- Current: ~44× vs baseline (single NTT)
- With lane parallelism: ~44× × k× = **176-352×** vs baseline

---

## WIRING DIAGRAM (What Needs To Connect)

```
┌─────────────────────────────────────────────────────────────────┐
│ nine65/ops/homomorphic.rs                                       │
│                                                                 │
│   homo_mul(ct1, ct2)                                            │
│       │                                                         │
│       ▼                                                         │
│   accelerated.rs::poly_mul_accelerated()  ◄── GAP-003          │
│       │                                                         │
└───────┼─────────────────────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────────┐
│ unhal/accelerator.rs                                            │
│                                                                 │
│   accel.ntt_multiply_streams()  ◄── GAP-001                     │
│       │                                                         │
└───────┼─────────────────────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────────┐
│ mana/parallel.rs                                                │
│                                                                 │
│   ParallelNTT::ntt_all_lanes()  ◄── GAP-002 (exists, unused)   │
│       │                                                         │
│       ▼                                                         │
│   lanes.par_iter().map(ntt_fn)  ◄── Rayon parallelism          │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────────┐
│ nine65/arithmetic/ntt_fft.rs                                    │
│                                                                 │
│   NTTEngineFFT::multiply()  ◄── Already optimized (44×)        │
│       │                                                         │
│   Per-lane: Harvey butterfly + batching                        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## FIX PRIORITY

### Priority 1: Wire NTT Through UNHAL (2-3 hours)

1. Add `ntt_forward_stream()` to UNHAL Accelerator
2. Add `ntt_inverse_stream()` to UNHAL Accelerator  
3. Add `ntt_multiply_streams()` to UNHAL Accelerator
4. Use ParallelNTT internally

### Priority 2: Add poly_mul_accelerated (1 hour)

1. Add to accelerated.rs
2. Wire through to UNHAL accelerator
3. Handle multi-prime NTT engines

### Priority 3: Update homo_mul to use accelerated path (30 min)

1. Add feature flag for accelerated homo_mul
2. Call poly_mul_accelerated instead of direct NTT
3. Benchmark comparison

---

## BENCHMARK PROJECTION

| Configuration | NTT Time (1024) | Homo Mul (est) |
|--------------|-----------------|----------------|
| **Baseline** | 1.96 ms | ~24 ms |
| **Optimized NTT (current)** | 44 μs | ~600 μs |
| **+ Lane Parallelism (4 primes)** | ~11 μs/lane | ~200 μs |
| **+ Lane Parallelism (8 primes)** | ~5.5 μs/lane | ~100 μs |

With full MANA/UNHAL integration: **Sub-200μs homo_mul is achievable.**

---

## CONCLUSION

The NTT optimization (44× speedup) is valuable but **orphaned** from the MANA/UNHAL stack. The infrastructure exists (ParallelNTT, ManaStream, Accelerator) but isn't wired together.

**Key Finding**: You built the highway system (MANA/UNHAL) but the fastest car (optimized NTT) is still taking surface streets.

**Action**: Wire the NTT through UNHAL Accelerator using ParallelNTT for lane-parallel execution.

---

*Gap Hunter Audit Complete - QMNF Integration Focus*
*Critical Gaps: 4 (all architectural wiring)*
*Code Quality: ✅ (NTT optimization is correct)*
*Integration: 🔴 (NTT bypasses MANA/UNHAL)*

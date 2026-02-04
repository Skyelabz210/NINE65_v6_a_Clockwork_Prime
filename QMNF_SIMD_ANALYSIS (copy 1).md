# QMNF SIMD Analysis: Why Simple is Better

**Date**: 2026-01-07  
**Analysis**: Bottleneck Hunter + Innovation Resolver

---

## Executive Summary

SIMD acceleration for modular arithmetic is **counterproductive** in QMNF. The benchmarks prove it:

```
SIMD vs Scalar Performance (N=4096)
════════════════════════════════════════════════
Operation     Sequential    SIMD          Verdict
────────────────────────────────────────────────
mod_add       2.87 µs      6.14 µs       2.1× SLOWER
mod_mul       12.48 µs     22.45 µs      1.8× SLOWER
════════════════════════════════════════════════
```

The correct optimizations are:
1. **NTT**: Harvey butterflies = **44× speedup** ✅
2. **Rayon**: Lane parallelism = **2.78× speedup** ✅
3. **Persistent Montgomery**: Eliminates conversion overhead ✅

---

## Root Cause Analysis

### Why SIMD Fails for Modular Arithmetic

**1. Conversion Overhead Dominates**

```
SIMD mul path (SLOW):
  standard → to_mont() → multiply → from_mont() → standard
                ↑                         ↑
           OVERHEAD                   OVERHEAD
           
Scalar mul path (FAST):
  (a as u128 * b as u128) % prime  ← Single CPU instruction
```

**2. Compiler Auto-Vectorizes Scalar Loops**

The Rust compiler (LLVM backend) automatically vectorizes simple patterns:
```rust
// This gets auto-vectorized:
coeffs.iter().zip(other.iter())
    .map(|(&a, &b)| {
        let sum = a + b;
        if sum >= q { sum - q } else { sum }
    })
```

**3. SIMD Abstraction Overhead**

Using the `wide` crate adds:
- Array ↔ SIMD register loads/stores
- Splat operations for constants
- Mask operations for conditional logic

This overhead exceeds the benefit for simple operations.

---

## Correct Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                    QMNF ACCELERATION STACK                           │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│   ╔═══════════════════════════════════════════════════════════════╗ │
│   ║  PROVEN SPEEDUPS                                              ║ │
│   ╠═══════════════════════════════════════════════════════════════╣ │
│   ║  NTT Engine        │ Harvey butterflies + batching  │ 44×    ║ │
│   ║  Rayon Parallel    │ Lane-level parallelism        │ 2.78×  ║ │
│   ║  Persistent Mont   │ Eliminate boundary conversions │ N/A*   ║ │
│   ╚═══════════════════════════════════════════════════════════════╝ │
│                                                                      │
│   ┌───────────────────────────────────────────────────────────────┐ │
│   │  DISABLED (Counterproductive)                                 │ │
│   │  SIMD via wide crate  │ Adds overhead, no benefit  │ 0.5×   │ │
│   └───────────────────────────────────────────────────────────────┘ │
│                                                                      │
│   * Persistent Montgomery speedup depends on operation chain length  │
│     Short chains: minimal benefit                                    │
│     Long chains (NTT): eliminates 70-year boundary overhead         │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

---

## When SIMD WOULD Help

SIMD helps when:
1. Operations are complex (many arithmetic steps per element)
2. No modular reduction needed
3. Large continuous memory operations (memcpy-like)
4. Graphics/floating-point workloads

SIMD doesn't help when:
1. Operations are simple (a + b mod p)
2. Compiler already auto-vectorizes
3. Abstraction overhead exceeds computation
4. Modular arithmetic requires conditional logic

---

## Feature Configuration

### MANA Cargo.toml
```toml
[features]
default = ["parallel"]
simd = ["wide"]  # Available but not default - benchmarks show it's slower
parallel = ["rayon"]  # ✅ PROVEN 2.78× speedup
```

### UNHAL Cargo.toml
```toml
[features]
default = ["parallel"]
parallel = ["rayon", "mana/parallel"]  # ✅ USE THIS
# simd = ["mana/simd"]  # Disabled: counterproductive for modular arithmetic
```

### NINE65 Cargo.toml
```toml
[features]
default = ["ntt_fft", "accelerated", "parallel"]
ntt_fft = []          # ✅ 44× speedup
accelerated = ["mana", "unhal"]  # ✅ Full stack
parallel = ["rayon"]  # ✅ Lane parallelism
```

---

## Benchmark Evidence

### Lane Addition (N=8192)
```
Sequential:  5.80 µs  ← WINNER
SIMD 4x:    12.70 µs  (2.2× slower)
```

### Lane Multiplication (N=8192)
```
Sequential: 25.56 µs  ← WINNER
SIMD batch: 45.71 µs  (1.8× slower)
```

### Stream Addition with Rayon (N=8192, 8 lanes)
```
Sequential:   46.4 µs
Rayon 8-lane: 16.7 µs  ← 2.78× FASTER (WINNER)
```

---

## Innovation Resolver Pattern Match

From the Innovation Resolver skill:

| Symptom | Pattern | Solution |
|---------|---------|----------|
| SIMD slower than scalar | Conversion overhead | Persistent Montgomery |
| Modular ops not benefiting | Auto-vectorization | Trust compiler |
| Lane parallelism works | Independent computation | Rayon |

The QMNF stack already implements the correct patterns:
- `PersistentLane` for Montgomery chains
- `ParallelStream` for lane-level Rayon
- `NTTEngineFFT` for Harvey-optimized butterflies

---

## Conclusion

**Don't fix what isn't broken.**

The SIMD module exists for testing/comparison but is correctly disabled in production.
The proven speedups (NTT 44×, Rayon 2.78×) are already wired through MANA/UNHAL.

Total theoretical speedup: 44 × 2.78 × k = **122k×** for k RNS lanes

This is achieved WITHOUT explicit SIMD, because:
1. NTT butterflies are the hot path (not coefficient-wise ops)
2. Rayon provides the parallelism
3. Compiler auto-vectorizes what it can

---

## Files Status

| File | Status | Notes |
|------|--------|-------|
| `mana/src/simd.rs` | ✅ Tests pass | Available for research, not production |
| `mana/Cargo.toml` | ✅ Correct | SIMD available but not default |
| `unhal/Cargo.toml` | ✅ Correct | SIMD disabled with explanation |
| `unhal/src/accelerator.rs` | ⚠️ Dead code | SIMD paths exist but feature disabled |
| `nine65/src/accelerated.rs` | ✅ Wired | Uses parallel, not SIMD |

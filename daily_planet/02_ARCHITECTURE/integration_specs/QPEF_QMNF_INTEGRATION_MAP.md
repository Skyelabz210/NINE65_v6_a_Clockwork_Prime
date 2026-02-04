# QPEF → QMNF Integration Map
## Swapping AI-Generated Assumptions with Actual QMNF Architecture

**Problem:** The QPEF spec was written by an AI trained on standard ML/systems programming, so it referenced generic libraries (NumPy, Rayon, etc.) that don't fit QMNF's integer-only architecture.

**Solution:** This document maps what the spec assumed vs. what we actually have in QMNF.

---

## Part 1: Core Data Types

### What the Spec Assumed:
```rust
// Generic CRT weight (spec's assumption)
struct CRTWeight {
    residues: [i64; 12],  // Just an array
    scale: i64,
}
```

### What We Actually Have:
```rust
// QMNF has THREE integer types to choose from:

// 1. CRTBigInt - FAST bounded arithmetic (~120ns ops)
//    Location: hcvlang/src/crt_bigint.rs (20KB)
use crate::crt_bigint::CRTBigInt;
let x = CRTBigInt::from(12345);  // Range: ±2^126

// 2. HCVLangBigInt - INFINITE precision exact arithmetic
//    Location: hcvlang/src/bigint_hcv.rs (26KB)
use crate::bigint_hcv::HCVLangBigInt;
let x = HCVLangBigInt::from(12345);  // Range: unlimited

// 3. AdaptiveCRTBigInt - DYNAMIC precision scaling
//    Location: hcvlang/src/adaptive_crt_bigint.rs (37KB)
use crate::adaptive_crt_bigint::AdaptiveCRTBigInt;
let x = AdaptiveCRTBigInt::new(12345, precision_bits);
```

**SWAP:** Use `CRTBigInt` for neural network weights (fast, bounded is sufficient).

---

## Part 2: Modular Arithmetic

### What the Spec Assumed:
```rust
// Generic Montgomery multiplication (spec's assumption)
fn montgomery_multiply(a: i64, b: i64, modulus: i64) -> i64 {
    // ... basic implementation
}
```

### What We Actually Have:
```rust
// QMNF has optimized ModInt with Montgomery built-in
// Location: hcvlang/src/modint.rs (20KB)
use crate::modint::ModInt;

let m = ModInt::new(12345, 2147483647);  // value, modulus
let n = ModInt::new(67890, 2147483647);
let product = m * n;  // Uses Montgomery internally!

// Also have standalone Montgomery contexts (just implemented!)
// Location: hcvlang/src/montgomery.rs (17KB)
use crate::montgomery::MontgomeryContext;

let ctx = MontgomeryContext::new(2147483647);
let a_mont = ctx.to_montgomery(12345);
let b_mont = ctx.to_montgomery(67890);
let product = ctx.montgomery_multiply(a_mont, b_mont);
```

**SWAP:** Use `ModInt` for modular operations, or `MontgomeryContext` for batch Montgomery ops.

---

## Part 3: NNT (Number Theoretic Transform)

### What the Spec Assumed:
```rust
// Basic NNT implementation (spec's assumption)
fn nnt_forward(input: &[i64]) -> Vec<i64> {
    // ... generic implementation
}
```

### What We Actually Have:
```rust
// QMNF has TWO NNT implementations:

// 1. Basic NNT (7KB) - Simple, proven
use crate::nnt::{nnt_forward, nnt_inverse};
let transformed = nnt_forward(&signal);

// 2. NNTEngine (17KB) - Precomputed twiddles, optimized
//    Location: hcvlang/src/nnt_engine.rs (just implemented!)
use crate::nnt_engine::NNTEngine;

let engine = NNTEngine::new(1024);  // Power of 2 size
let transformed = engine.forward_nnt(&signal);
let convolved = engine.convolution(&signal1, &signal2);
```

**SWAP:** Use `NNTEngine` (we just built it!) for convolution layers.

---

## Part 4: Neural Network Layers

### What the Spec Assumed:
```rust
// Generic neural layers (spec's assumption)
// Probably assumed PyTorch/TensorFlow style
```

### What We Actually Have:
```rust
// QMNF Neuromorphic HAL (implemented 3 weeks ago!)

// 1. Convolutional Layer
//    Location: hcvlang/src/crt_conv_layer.rs (18KB, 9/9 tests ✅)
use crate::crt_conv_layer::CRTConvLayer;

let layer = CRTConvLayer::new(
    input_channels: 3,
    output_channels: 64,
    kernel_height: 3,
    kernel_width: 3,
    stride: 1,
    padding: 1,
    backend,  // Compute backend
);

let (output, out_h, out_w) = layer.forward(&input, height, width);

// 2. Dense Layer
//    Location: hcvlang/src/fp_dense_layer.rs (528 lines, 13/13 tests ✅)
use crate::fp_dense_layer::FPDenseLayer;

let layer = FPDenseLayer::new(input_dim: 784, output_dim: 128);
let output = layer.forward(&input);

// 3. Modular ReLU
//    Location: hcvlang/src/modular_relu.rs (416 lines, 15/15 tests ✅)
use crate::modular_relu::ModularQuantizedReLU;

let relu = ModularQuantizedReLU::new();
let activated = relu.forward(&input);

// 4. QMNF Weight Representation
//    Location: hcvlang/src/qmnf_weight.rs (434 lines, 15/15 tests ✅)
use crate::qmnf_weight::QMNFCRTWeight;

let weight = QMNFCRTWeight::new(value);
let sum = weight.add(&other_weight);
```

**SWAP:** Use existing Neuromorphic HAL layers, don't reimplement!

---

## Part 5: Parallelism

### What the Spec Assumed:
```rust
// Rayon-style parallelism (spec's assumption)
use rayon::prelude::*;
weights.par_iter_mut().for_each(|w| *w += delta);
```

### What We Actually Have:
```rust
// QPEF - Deterministic parallel execution (just implemented!)

// 1. Core structures
//    Location: hcvlang/src/qpef_core.rs (23KB, 10/10 tests ✅)
use crate::qpef_core::LaneIsolatedCRTWeight;

let weight = LaneIsolatedCRTWeight::new(
    value,
    &QMNF_PRIMES,
    QMNF_SCALE
);

// 2. Deterministic scheduler
//    Location: hcvlang/src/qpef_scheduler.rs (4.6KB, 4/4 tests ✅)
use crate::qpef_scheduler::DeterministicScheduler;

let scheduler = DeterministicScheduler::default_sequential()
    .with_parallel();

let results = scheduler.map_lanes(|lane, prime| {
    // Process lane deterministically
    compute_residue(lane, prime)
});

// 3. Executor (for integration)
//    Location: hcvlang/src/qpef_executor.rs (58 lines)
use crate::qpef_executor::ResidueLaneExecutor;

let executor = ResidueLaneExecutor::parallel();
```

**SWAP:** Use QPEF's `DeterministicScheduler`, NOT Rayon!

---

## Part 6: Python Integration

### What the Spec Assumed:
```python
# NumPy-style arrays (spec's assumption)
import numpy as np
weights = np.array([1.5, 2.3, 3.7])  # FLOATS!
```

### What We Actually Have:
```python
# QMNF integer-only Python API

# 1. QMNFRational for exact arithmetic
#    Location: qmnf/boundary.py
from qmnf.api import QMNFRational

weights = [
    QMNFRational(3, 2),   # Exact 1.5
    QMNFRational(23, 10), # Exact 2.3
    QMNFRational(37, 10)  # Exact 3.7
]

result = weights[0] * weights[1]  # Exact multiplication

# 2. Rust bindings for performance
#    Location: hcvlang/src/ffi.rs (103 FFI classes!)
from hcvlang import CRTBigInt, ModInt, FHEContext

crt = CRTBigInt.from_int(12345)
crt2 = CRTBigInt.from_int(67890)
result = crt + crt2  # Fast CRT arithmetic

# 3. Conversion boundary for external data
#    Location: qmnf/conversion_boundary.py
from qmnf.conversion_boundary import DataBoundary

# Convert external floats ONCE at boundary
rational = DataBoundary.float_to_rational(3.14159)
# Then use exact arithmetic everywhere else
```

**SWAP:** Use `QMNFRational` and `CRTBigInt`, NOT NumPy arrays!

---

## Part 7: Garner Reconstruction

### What the Spec Assumed:
```rust
// Basic 2-prime Garner (spec's assumption)
fn garner_reconstruct(r1: i64, r2: i64, m1: i64, m2: i64) -> i64 {
    // ... simple implementation
}
```

### What We Actually Have:
```rust
// QMNF has enhanced N-prime Garner reconstruction

// Location: hcvlang/src/garner.rs (16KB, 12/12 tests ✅)
use crate::garner::GarnerReconstructor;

let reconstructor = GarnerReconstructor::new_qmnf_primes();  // All 12 primes
let value = reconstructor.reconstruct(&residues);  // Returns HCVLangBigInt

// Fast path for small values
let value_i128 = reconstructor.reconstruct_i128(&residues);

// Batch reconstruction
let values = reconstructor.batch_reconstruct(&residue_arrays);
```

**SWAP:** Use `GarnerReconstructor` (we just built it!) for CRT → integer conversion.

---

## Part 8: Actual Integration Pattern

### Don't Do This (Spec's Generic Approach):
```rust
// BAD: Using generic types
struct GenericWeight {
    value: f64,  // ❌ FLOAT!
}

// BAD: Using Rayon
weights.par_iter().for_each(|w| update(w));  // ❌ Non-deterministic!
```

### Do This Instead (QMNF Architecture):
```rust
// GOOD: Use QMNF types
use crate::crt_bigint::CRTBigInt;
use crate::qpef_core::LaneIsolatedCRTWeight;
use crate::qpef_scheduler::DeterministicScheduler;
use crate::montgomery::MontgomeryContext;
use crate::nnt_engine::NNTEngine;

// 1. Create weight in CRT form
let weight = CRTBigInt::from(12345);  // Fast, exact

// 2. Or use QPEF-compatible form
let weight = LaneIsolatedCRTWeight::new(
    12345,
    &crate::qpef_core::QMNF_PRIMES,
    crate::qpef_core::QMNF_SCALE
);

// 3. Use deterministic parallelism
let scheduler = DeterministicScheduler::default_sequential()
    .with_parallel();

let results = scheduler.map_lanes(|lane, prime| {
    // Each lane processes one prime independently
    process_residue(weight.residue_at(lane), prime)
});

// 4. Use optimized convolution
let nnt = NNTEngine::new(1024);
let convolved = nnt.convolution(&signal1, &signal2);

// 5. Use Montgomery for repeated modular ops
let ctx = MontgomeryContext::new(prime);
let a_mont = ctx.to_montgomery(a);
let b_mont = ctx.to_montgomery(b);
let product = ctx.montgomery_multiply(a_mont, b_mont);
```

---

## Part 9: What to Replace Where

### In QPEF Modules:

**qpef_core.rs** ✅
- Already uses `i64` residues (correct)
- Already forbids floats (correct)
- **Action:** Keep as-is, it's already QMNF-compliant

**qpef_scheduler.rs** ✅
- Already deterministic (correct)
- Already forbids floats (correct)
- **Action:** Keep as-is, it's already QMNF-compliant

**qpef_executor.rs** 🔄
- Currently just wraps scheduler
- **Action:** Add integration with `CRTBigInt` operations
- **TODO:** Add methods like:
  ```rust
  pub fn parallel_crt_add(
      &self,
      a: &CRTBigInt,
      b: &CRTBigInt
  ) -> CRTBigInt {
      // Use scheduler to add across lanes
  }
  ```

**qpef_transaction.rs** 🔄
- Currently uses `LaneIsolatedCRTWeight` (correct)
- **Action:** Add integration with `CRTBigInt` ↔ `LaneIsolatedCRTWeight` conversion
- **TODO:** Add helper:
  ```rust
  pub fn from_crt_bigint(crt: &CRTBigInt) -> LaneIsolatedCRTWeight {
      // Convert to QPEF form
  }
  ```

**qpef_neural.rs** 🔄
- References `QMNFCRTWeight` (from neuromorphic HAL)
- **Action:** Bridge with actual neuromorphic HAL layers
- **TODO:** Use existing layers:
  ```rust
  use crate::crt_conv_layer::CRTConvLayer;
  use crate::fp_dense_layer::FPDenseLayer;
  use crate::modular_relu::ModularQuantizedReLU;
  ```

---

## Part 10: Immediate Action Items

### 1. Bridge QPEF ↔ CRTBigInt (High Priority)
```rust
// Add to qpef_core.rs:
impl LaneIsolatedCRTWeight {
    pub fn from_crt_bigint(crt: &CRTBigInt) -> Self {
        // Extract residues from CRTBigInt
        let residues = crt.get_residues();  // Get [i64; 12]
        Self::new_from_residues(residues, crt.scale())
    }

    pub fn to_crt_bigint(&self) -> CRTBigInt {
        // Construct CRTBigInt from our residues
        CRTBigInt::from_residues(&self.residues)
    }
}
```

### 2. Bridge QPEF ↔ Neuromorphic HAL (High Priority)
```rust
// Add to qpef_neural.rs:
use crate::qmnf_weight::QMNFCRTWeight;
use crate::crt_conv_layer::CRTConvLayer;
use crate::fp_dense_layer::FPDenseLayer;

pub struct QPEFConvLayer {
    inner: CRTConvLayer,  // Use existing implementation!
    executor: ResidueLaneExecutor,
}

impl QPEFConvLayer {
    pub fn forward_parallel(&self, input: &[LaneIsolatedCRTWeight]) -> Vec<LaneIsolatedCRTWeight> {
        // Convert QPEF weights → QMNFCRTWeight
        // Call existing layer.forward()
        // Convert back to QPEF weights
    }
}
```

### 3. Add Montgomery Integration (Medium Priority)
```rust
// Add to qpef_executor.rs:
use crate::montgomery::MontgomeryContext;

impl ResidueLaneExecutor {
    pub fn parallel_montgomery_multiply(
        &self,
        a: &LaneIsolatedCRTWeight,
        b: &LaneIsolatedCRTWeight,
        contexts: &[MontgomeryContext; 12],
    ) -> LaneIsolatedCRTWeight {
        self.scheduler().map_lanes(|lane, _prime| {
            contexts[lane].montgomery_multiply(
                a.residue_at(lane),
                b.residue_at(lane)
            )
        })
        .into()  // Convert results to LaneIsolatedCRTWeight
    }
}
```

### 4. Add NNT Integration (Medium Priority)
```rust
// Add to qpef_neural.rs:
use crate::nnt_engine::NNTEngine;

pub fn parallel_nnt_convolution(
    signal: &[LaneIsolatedCRTWeight],
    kernel: &[LaneIsolatedCRTWeight],
    executor: &ResidueLaneExecutor,
) -> Vec<LaneIsolatedCRTWeight> {
    let nnt = NNTEngine::new(signal.len().next_power_of_two());

    // Process each lane independently
    executor.scheduler().map_lanes(|lane, _prime| {
        let signal_lane: Vec<i64> = signal.iter()
            .map(|w| w.residue_at(lane))
            .collect();
        let kernel_lane: Vec<i64> = kernel.iter()
            .map(|w| w.residue_at(lane))
            .collect();

        nnt.convolution(&signal_lane, &kernel_lane)
    })
}
```

---

## Summary: What to Swap

| Spec Assumed | QMNF Has | Status | Action |
|--------------|----------|--------|--------|
| Generic `CRTWeight` | `CRTBigInt` (20KB ✅) | Production | Use it! |
| Generic Montgomery | `MontgomeryContext` (17KB ✅) | Just built | Integrate |
| Generic NNT | `NNTEngine` (17KB ✅) | Just built | Integrate |
| Generic layers | Neuromorphic HAL (4 modules ✅) | Production | Bridge with QPEF |
| Rayon parallelism | QPEF (23KB ✅ + 4.6KB ✅) | 70% done | Complete integration |
| NumPy arrays | `QMNFRational` ✅ | Production | Already using |
| Generic Garner | `GarnerReconstructor` (16KB ✅) | Just built | Integrate |

**Next Step:** Write the bridge functions to connect QPEF with the actual QMNF components!

# UNHAL: Universal Neuromorphic Hardware Abstraction Layer
## Modular Architecture v1.0

**Date:** December 14, 2025  
**Status:** Architecture Design  
**Integration Target:** QMNF Stack (hcvlang)

---

## 1. Design Philosophy

UNHAL is NOT a new implementation. It's an **abstraction layer** that provides:
1. Clean, simple API for neuromorphic computing
2. Swappable backends (CPU, QPEF parallel, future FPGA)
3. Unified interface to existing QMNF components
4. Type-safe boundary between user code and QMNF internals

**Principle:** "Interface what they know. Implement with what they don't."

---

## 2. Module Hierarchy

```
unhal/
├── core/                    # Core abstractions
│   ├── mod.rs              # Module exports
│   ├── value.rs            # UNHALValue trait
│   ├── backend.rs          # Backend trait
│   └── config.rs           # Configuration
│
├── types/                   # Value representations
│   ├── mod.rs
│   ├── residue.rs          # Residue vector (wraps QMNF types)
│   ├── weight.rs           # Neural weight (wraps QMNFCRTWeight)
│   └── tensor.rs           # Multi-dimensional tensor
│
├── backends/                # Execution backends
│   ├── mod.rs
│   ├── sequential.rs       # Single-threaded baseline
│   ├── qpef.rs             # QPEF parallel backend
│   └── simd.rs             # AVX-512 vectorized (future)
│
├── ops/                     # Operations
│   ├── mod.rs
│   ├── arithmetic.rs       # add, mul, div (K-Elimination)
│   ├── reduction.rs        # sum, product, dot
│   └── transform.rs        # NTT, reconstruction
│
├── layers/                  # Neural network layers
│   ├── mod.rs
│   ├── dense.rs            # Wraps FPDenseLayer
│   ├── conv.rs             # Wraps CRTConvLayer
│   ├── activation.rs       # Wraps ModularQuantizedReLU
│   └── pooling.rs          # Max/avg pooling
│
├── scheduler/               # Execution scheduling
│   ├── mod.rs
│   ├── lane.rs             # Lane-based scheduling
│   └── batch.rs            # Batch processing
│
└── lib.rs                   # Public API
```

---

## 3. Core Traits

### 3.1 UNHALValue Trait

The foundational abstraction for all UNHAL values.

```rust
// unhal/core/value.rs

/// Core trait for all UNHAL values
pub trait UNHALValue: Clone + Send + Sync {
    /// Number of residue lanes
    fn lane_count(&self) -> usize;
    
    /// Get residue at lane i
    fn residue(&self, lane: usize) -> i64;
    
    /// Get all residues
    fn residues(&self) -> &[i64];
    
    /// Reconstruct to integer (may overflow)
    fn reconstruct_i128(&self) -> Option<i128>;
    
    /// Reconstruct to big integer (exact)
    fn reconstruct_exact(&self) -> HCVLangBigInt;
    
    /// Check if value is negative (in signed interpretation)
    fn is_negative(&self) -> bool;
}
```

### 3.2 Backend Trait

Defines execution strategy for operations.

```rust
// unhal/core/backend.rs

/// Execution backend for UNHAL operations
pub trait UNHALBackend: Send + Sync {
    /// Backend identifier
    fn name(&self) -> &'static str;
    
    /// Whether operations are parallelized
    fn is_parallel(&self) -> bool;
    
    /// Lane-wise addition
    fn add<V: UNHALValue>(&self, a: &V, b: &V, primes: &[i64]) -> Vec<i64>;
    
    /// Lane-wise multiplication
    fn mul<V: UNHALValue>(&self, a: &V, b: &V, primes: &[i64]) -> Vec<i64>;
    
    /// Dot product of vectors
    fn dot<V: UNHALValue>(&self, a: &[V], b: &[V], primes: &[i64]) -> Vec<i64>;
    
    /// Map operation across all lanes
    fn map_lanes<F>(&self, f: F, lane_count: usize) -> Vec<i64>
    where
        F: Fn(usize, i64) -> i64 + Send + Sync;
}
```

### 3.3 Layer Trait

Neural network layer abstraction.

```rust
// unhal/layers/mod.rs

/// Neural network layer abstraction
pub trait UNHALLayer<V: UNHALValue> {
    /// Forward pass
    fn forward(&self, input: &[V]) -> Vec<V>;
    
    /// Backward pass (returns input gradient)
    fn backward(&self, output_grad: &[V]) -> Vec<V>;
    
    /// Get layer parameters
    fn parameters(&self) -> Vec<&V>;
    
    /// Get mutable parameters for updates
    fn parameters_mut(&mut self) -> Vec<&mut V>;
}
```

---

## 4. Type Implementations

### 4.1 ResidueVec (wraps existing types)

```rust
// unhal/types/residue.rs

use crate::core::value::UNHALValue;
use hcvlang::qpef_core::{LaneIsolatedCRTWeight, QMNF_PRIMES, QMNF_SCALE};
use hcvlang::crt_bigint::CRTBigInt;
use hcvlang::bigint_hcv::HCVLangBigInt;

/// UNHAL's primary value type - wraps QMNF's LaneIsolatedCRTWeight
#[derive(Clone)]
pub struct ResidueVec {
    inner: LaneIsolatedCRTWeight,
}

impl ResidueVec {
    /// Create from integer value
    pub fn from_int(value: i64) -> Self {
        Self {
            inner: LaneIsolatedCRTWeight::new(value, &QMNF_PRIMES, QMNF_SCALE),
        }
    }
    
    /// Create from existing QMNF weight
    pub fn from_qmnf(weight: LaneIsolatedCRTWeight) -> Self {
        Self { inner: weight }
    }
    
    /// Create from CRTBigInt
    pub fn from_crt_bigint(crt: &CRTBigInt) -> Self {
        // Extract residues and create LaneIsolatedCRTWeight
        let residues = crt.get_residues();
        Self {
            inner: LaneIsolatedCRTWeight::from_residues(&residues, &QMNF_PRIMES, QMNF_SCALE),
        }
    }
    
    /// Get underlying QMNF type
    pub fn as_qmnf(&self) -> &LaneIsolatedCRTWeight {
        &self.inner
    }
    
    /// Zero value
    pub fn zero() -> Self {
        Self::from_int(0)
    }
}

impl UNHALValue for ResidueVec {
    fn lane_count(&self) -> usize {
        QMNF_PRIMES.len()  // 12 lanes
    }
    
    fn residue(&self, lane: usize) -> i64 {
        self.inner.residue_at(lane)
    }
    
    fn residues(&self) -> &[i64] {
        self.inner.residues()
    }
    
    fn reconstruct_i128(&self) -> Option<i128> {
        self.inner.reconstruct_small()
    }
    
    fn reconstruct_exact(&self) -> HCVLangBigInt {
        // Use Garner reconstruction
        use hcvlang::garner::GarnerReconstructor;
        let reconstructor = GarnerReconstructor::new_qmnf_primes();
        reconstructor.reconstruct(self.residues())
    }
    
    fn is_negative(&self) -> bool {
        self.inner.is_negative()
    }
}
```

### 4.2 NeuralWeight (wraps QMNFCRTWeight)

```rust
// unhal/types/weight.rs

use hcvlang::qmnf_weight::QMNFCRTWeight;

/// Neural network weight - wraps QMNFCRTWeight for layer parameters
#[derive(Clone)]
pub struct NeuralWeight {
    inner: QMNFCRTWeight,
}

impl NeuralWeight {
    pub fn new(value: i64) -> Self {
        Self {
            inner: QMNFCRTWeight::new(value),
        }
    }
    
    pub fn random_xavier(fan_in: usize, fan_out: usize, seed: u64) -> Self {
        Self {
            inner: QMNFCRTWeight::random_xavier(fan_in, fan_out, seed),
        }
    }
    
    pub fn random_he(fan_in: usize, seed: u64) -> Self {
        Self {
            inner: QMNFCRTWeight::random_he(fan_in, seed),
        }
    }
    
    pub fn as_qmnf(&self) -> &QMNFCRTWeight {
        &self.inner
    }
}

impl UNHALValue for NeuralWeight {
    // ... similar implementation using QMNFCRTWeight methods
}
```

---

## 5. Backend Implementations

### 5.1 Sequential Backend (baseline)

```rust
// unhal/backends/sequential.rs

use crate::core::backend::UNHALBackend;
use crate::core::value::UNHALValue;

/// Single-threaded sequential execution
pub struct SequentialBackend;

impl UNHALBackend for SequentialBackend {
    fn name(&self) -> &'static str {
        "sequential"
    }
    
    fn is_parallel(&self) -> bool {
        false
    }
    
    fn add<V: UNHALValue>(&self, a: &V, b: &V, primes: &[i64]) -> Vec<i64> {
        (0..a.lane_count())
            .map(|i| (a.residue(i) + b.residue(i)) % primes[i])
            .collect()
    }
    
    fn mul<V: UNHALValue>(&self, a: &V, b: &V, primes: &[i64]) -> Vec<i64> {
        (0..a.lane_count())
            .map(|i| {
                // Use 128-bit intermediate to avoid overflow
                let prod = (a.residue(i) as i128) * (b.residue(i) as i128);
                (prod % (primes[i] as i128)) as i64
            })
            .collect()
    }
    
    fn dot<V: UNHALValue>(&self, a: &[V], b: &[V], primes: &[i64]) -> Vec<i64> {
        assert_eq!(a.len(), b.len(), "Vectors must have same length");
        let lane_count = a[0].lane_count();
        
        let mut result = vec![0i64; lane_count];
        for (x, y) in a.iter().zip(b.iter()) {
            for i in 0..lane_count {
                let prod = (x.residue(i) as i128) * (y.residue(i) as i128);
                result[i] = ((result[i] as i128 + prod) % (primes[i] as i128)) as i64;
            }
        }
        result
    }
    
    fn map_lanes<F>(&self, f: F, lane_count: usize) -> Vec<i64>
    where
        F: Fn(usize, i64) -> i64 + Send + Sync,
    {
        (0..lane_count).map(|i| f(i, QMNF_PRIMES[i])).collect()
    }
}
```

### 5.2 QPEF Parallel Backend

```rust
// unhal/backends/qpef.rs

use crate::core::backend::UNHALBackend;
use crate::core::value::UNHALValue;
use hcvlang::qpef_scheduler::DeterministicScheduler;
use hcvlang::qpef_executor::ResidueLaneExecutor;
use std::sync::Arc;

/// Parallel execution using QPEF
pub struct QPEFBackend {
    scheduler: DeterministicScheduler,
    executor: Arc<ResidueLaneExecutor>,
}

impl QPEFBackend {
    pub fn new() -> Self {
        Self {
            scheduler: DeterministicScheduler::default_sequential().with_parallel(),
            executor: Arc::new(ResidueLaneExecutor::parallel()),
        }
    }
    
    pub fn with_lane_order(order: ScheduleOrder) -> Self {
        Self {
            scheduler: DeterministicScheduler::new(order).with_parallel(),
            executor: Arc::new(ResidueLaneExecutor::parallel()),
        }
    }
}

impl UNHALBackend for QPEFBackend {
    fn name(&self) -> &'static str {
        "qpef_parallel"
    }
    
    fn is_parallel(&self) -> bool {
        true
    }
    
    fn add<V: UNHALValue>(&self, a: &V, b: &V, primes: &[i64]) -> Vec<i64> {
        // Use QPEF's deterministic parallel execution
        self.scheduler.map_lanes(|lane, prime| {
            (a.residue(lane) + b.residue(lane)) % prime
        })
    }
    
    fn mul<V: UNHALValue>(&self, a: &V, b: &V, primes: &[i64]) -> Vec<i64> {
        self.scheduler.map_lanes(|lane, prime| {
            let prod = (a.residue(lane) as i128) * (b.residue(lane) as i128);
            (prod % (prime as i128)) as i64
        })
    }
    
    fn dot<V: UNHALValue>(&self, a: &[V], b: &[V], primes: &[i64]) -> Vec<i64> {
        // Each lane computes its dot product independently
        self.scheduler.map_lanes(|lane, prime| {
            let mut sum: i128 = 0;
            for (x, y) in a.iter().zip(b.iter()) {
                let prod = (x.residue(lane) as i128) * (y.residue(lane) as i128);
                sum = (sum + prod) % (prime as i128);
            }
            sum as i64
        })
    }
    
    fn map_lanes<F>(&self, f: F, lane_count: usize) -> Vec<i64>
    where
        F: Fn(usize, i64) -> i64 + Send + Sync,
    {
        self.scheduler.map_lanes(f)
    }
}
```

### 5.3 Montgomery-Accelerated Backend

```rust
// unhal/backends/montgomery.rs

use crate::core::backend::UNHALBackend;
use hcvlang::montgomery::MontgomeryContext;
use hcvlang::qpef_core::QMNF_PRIMES;

/// Montgomery-accelerated execution
pub struct MontgomeryBackend {
    contexts: Vec<MontgomeryContext>,
    inner: QPEFBackend,
}

impl MontgomeryBackend {
    pub fn new() -> Self {
        // Precompute Montgomery contexts for all 12 primes
        let contexts: Vec<_> = QMNF_PRIMES.iter()
            .map(|&p| MontgomeryContext::new(p as u64))
            .collect();
        
        Self {
            contexts,
            inner: QPEFBackend::new(),
        }
    }
}

impl UNHALBackend for MontgomeryBackend {
    fn name(&self) -> &'static str {
        "montgomery_parallel"
    }
    
    fn is_parallel(&self) -> bool {
        true
    }
    
    fn mul<V: UNHALValue>(&self, a: &V, b: &V, primes: &[i64]) -> Vec<i64> {
        // Use Montgomery multiplication for each lane
        self.inner.scheduler.map_lanes(|lane, _prime| {
            let ctx = &self.contexts[lane];
            let a_mont = ctx.to_montgomery(a.residue(lane) as u64);
            let b_mont = ctx.to_montgomery(b.residue(lane) as u64);
            let result_mont = ctx.montgomery_multiply(a_mont, b_mont);
            ctx.from_montgomery(result_mont) as i64
        })
    }
    
    // add, dot delegate to inner QPEF backend
    fn add<V: UNHALValue>(&self, a: &V, b: &V, primes: &[i64]) -> Vec<i64> {
        self.inner.add(a, b, primes)
    }
    
    fn dot<V: UNHALValue>(&self, a: &[V], b: &[V], primes: &[i64]) -> Vec<i64> {
        // Montgomery-accelerated dot product
        self.inner.scheduler.map_lanes(|lane, prime| {
            let ctx = &self.contexts[lane];
            let mut sum: u64 = 0;
            for (x, y) in a.iter().zip(b.iter()) {
                let a_mont = ctx.to_montgomery(x.residue(lane) as u64);
                let b_mont = ctx.to_montgomery(y.residue(lane) as u64);
                let prod = ctx.montgomery_multiply(a_mont, b_mont);
                sum = (sum + ctx.from_montgomery(prod)) % (prime as u64);
            }
            sum as i64
        })
    }
}
```

---

## 6. Layer Wrappers

### 6.1 Dense Layer Wrapper

```rust
// unhal/layers/dense.rs

use crate::core::backend::UNHALBackend;
use crate::types::weight::NeuralWeight;
use hcvlang::fp_dense_layer::FPDenseLayer;
use hcvlang::qpef_neural::QPEFDenseLayer;
use std::sync::Arc;

/// Dense (fully connected) layer
pub struct DenseLayer<B: UNHALBackend> {
    inner: QPEFDenseLayer,  // Use QPEF's parallel dense layer
    backend: Arc<B>,
}

impl<B: UNHALBackend> DenseLayer<B> {
    pub fn new(input_dim: usize, output_dim: usize, seed: u64, backend: Arc<B>) -> Self {
        let executor = Arc::new(ResidueLaneExecutor::parallel());
        Self {
            inner: QPEFDenseLayer::new(input_dim, output_dim, seed, executor),
            backend,
        }
    }
    
    pub fn forward(&self, input: &[NeuralWeight]) -> Vec<NeuralWeight> {
        // Convert to QMNF types, run forward, convert back
        let qmnf_input: Vec<_> = input.iter().map(|w| w.as_qmnf().clone()).collect();
        let qmnf_output = self.inner.forward(&qmnf_input);
        qmnf_output.into_iter().map(NeuralWeight::from_qmnf).collect()
    }
    
    pub fn forward_parallel(&self, batch: &[Vec<NeuralWeight>]) -> Vec<Vec<NeuralWeight>> {
        // Use QPEF parallel forward
        batch.iter().map(|input| self.forward(input)).collect()
    }
}
```

### 6.2 Conv Layer Wrapper

```rust
// unhal/layers/conv.rs

use hcvlang::crt_conv_layer::CRTConvLayer;
use hcvlang::nnt_engine::NNTEngine;

/// Convolutional layer with automatic NTT acceleration
pub struct ConvLayer<B: UNHALBackend> {
    inner: CRTConvLayer,
    ntt_engine: Option<NNTEngine>,  // For large kernels
    backend: Arc<B>,
}

impl<B: UNHALBackend> ConvLayer<B> {
    pub fn new(
        input_channels: usize,
        output_channels: usize,
        kernel_size: (usize, usize),
        stride: usize,
        padding: usize,
        backend: Arc<B>,
    ) -> Self {
        let inner = CRTConvLayer::new(
            input_channels,
            output_channels,
            kernel_size.0,
            kernel_size.1,
            stride,
            padding,
            /* backend */
        );
        
        // Use NTT for kernels > 3x3 (crossover point from benchmarks)
        let ntt_engine = if kernel_size.0 > 3 || kernel_size.1 > 3 {
            let size = (kernel_size.0 * kernel_size.1).next_power_of_two();
            Some(NNTEngine::new(size))
        } else {
            None
        };
        
        Self { inner, ntt_engine, backend }
    }
    
    pub fn forward(&self, input: &[NeuralWeight], height: usize, width: usize) 
        -> (Vec<NeuralWeight>, usize, usize) 
    {
        // Delegate to CRTConvLayer, using NTT if available
        // ...
    }
}
```

### 6.3 Activation Wrapper

```rust
// unhal/layers/activation.rs

use hcvlang::modular_relu::ModularQuantizedReLU;

/// ReLU activation with modular sign detection
pub struct ReLU {
    inner: ModularQuantizedReLU,
}

impl ReLU {
    pub fn new() -> Self {
        Self {
            inner: ModularQuantizedReLU::new(),
        }
    }
    
    pub fn forward<V: UNHALValue>(&self, input: &[V]) -> Vec<V> {
        // Use QMNF's modular ReLU
        // ...
    }
    
    pub fn forward_inplace<V: UNHALValue>(&self, input: &mut [V]) {
        self.inner.forward_inplace(/* ... */);
    }
}
```

---

## 7. Operations Module

### 7.1 K-Elimination Division

```rust
// unhal/ops/arithmetic.rs

use hcvlang::garner::GarnerReconstructor;
use crate::types::residue::ResidueVec;

/// Exact division using K-Elimination
/// 
/// Unlike standard RNS which has 999998/1000000 accuracy,
/// K-Elimination provides 100% exact division.
pub fn exact_divide(
    dividend: &ResidueVec,
    divisor: &ResidueVec,
) -> (ResidueVec, ResidueVec) {
    // Reconstruct full values
    let reconstructor = GarnerReconstructor::new_qmnf_primes();
    let a = reconstructor.reconstruct(dividend.residues());
    let b = reconstructor.reconstruct(divisor.residues());
    
    // Integer division (exact)
    let quotient = &a / &b;
    let remainder = &a % &b;
    
    // Re-encode to CRT
    (
        ResidueVec::from_bigint(&quotient),
        ResidueVec::from_bigint(&remainder),
    )
}

/// Lane-wise modular addition
pub fn add<B: UNHALBackend>(
    backend: &B,
    a: &ResidueVec,
    b: &ResidueVec,
) -> ResidueVec {
    let residues = backend.add(a, b, &QMNF_PRIMES);
    ResidueVec::from_residues(residues)
}

/// Lane-wise modular multiplication
pub fn mul<B: UNHALBackend>(
    backend: &B,
    a: &ResidueVec,
    b: &ResidueVec,
) -> ResidueVec {
    let residues = backend.mul(a, b, &QMNF_PRIMES);
    ResidueVec::from_residues(residues)
}
```

### 7.2 NTT Transform

```rust
// unhal/ops/transform.rs

use hcvlang::nnt_engine::NNTEngine;

/// NTT-based polynomial multiplication (O(n log n))
pub fn ntt_convolve<V: UNHALValue>(
    signal: &[V],
    kernel: &[V],
    lane: usize,
) -> Vec<i64> {
    let size = (signal.len() + kernel.len() - 1).next_power_of_two();
    let engine = NNTEngine::new(size);
    
    let sig_lane: Vec<_> = signal.iter().map(|v| v.residue(lane)).collect();
    let ker_lane: Vec<_> = kernel.iter().map(|v| v.residue(lane)).collect();
    
    engine.convolution(&sig_lane, &ker_lane)
}

/// Full Garner reconstruction
pub fn reconstruct_exact<V: UNHALValue>(value: &V) -> HCVLangBigInt {
    let reconstructor = GarnerReconstructor::new_qmnf_primes();
    reconstructor.reconstruct(value.residues())
}
```

---

## 8. Public API

### 8.1 Prelude Module

```rust
// unhal/lib.rs

//! UNHAL: Universal Neuromorphic Hardware Abstraction Layer
//! 
//! Integer-only neural network computation using CRT arithmetic.
//! Zero floating-point operations. 100% deterministic.
//! 
//! # Example
//! 
//! ```rust
//! use unhal::prelude::*;
//! 
//! // Create backend
//! let backend = Arc::new(QPEFBackend::new());
//! 
//! // Create values
//! let a = ResidueVec::from_int(12345);
//! let b = ResidueVec::from_int(67890);
//! 
//! // Arithmetic (parallel across 12 lanes)
//! let sum = add(&backend, &a, &b);
//! let product = mul(&backend, &a, &b);
//! 
//! // Exact division via K-Elimination
//! let (quotient, remainder) = exact_divide(&a, &b);
//! 
//! // Neural network layer
//! let dense = DenseLayer::new(784, 128, 42, backend.clone());
//! let output = dense.forward(&input);
//! ```

pub mod core;
pub mod types;
pub mod backends;
pub mod ops;
pub mod layers;
pub mod scheduler;

/// Convenient imports for common usage
pub mod prelude {
    pub use crate::core::{UNHALValue, UNHALBackend};
    pub use crate::types::{ResidueVec, NeuralWeight, Tensor};
    pub use crate::backends::{SequentialBackend, QPEFBackend, MontgomeryBackend};
    pub use crate::ops::{add, mul, dot, exact_divide, ntt_convolve};
    pub use crate::layers::{DenseLayer, ConvLayer, ReLU};
    pub use std::sync::Arc;
}
```

---

## 9. Integration Points with QMNF

### 9.1 Type Mapping

| UNHAL Type | QMNF Type | Module |
|------------|-----------|--------|
| `ResidueVec` | `LaneIsolatedCRTWeight` | qpef_core.rs |
| `NeuralWeight` | `QMNFCRTWeight` | qmnf_weight.rs |
| `SequentialBackend` | (native Rust) | - |
| `QPEFBackend` | `DeterministicScheduler` | qpef_scheduler.rs |
| `MontgomeryBackend` | `MontgomeryContext` | montgomery.rs |
| `DenseLayer` | `QPEFDenseLayer` | qpef_neural.rs |
| `ConvLayer` | `CRTConvLayer` | crt_conv_layer.rs |
| `ReLU` | `ModularQuantizedReLU` | modular_relu.rs |

### 9.2 Operation Mapping

| UNHAL Op | QMNF Implementation | Complexity |
|----------|---------------------|------------|
| `add` | Lane-wise modular add | O(1) parallel |
| `mul` | Montgomery multiply | O(1) parallel |
| `dot` | Parallel dot product | O(n) per lane |
| `exact_divide` | K-Elimination | O(k²) reconstruction |
| `ntt_convolve` | NNTEngine | O(n log n) |
| `reconstruct` | GarnerReconstructor | O(k²) |

---

## 10. Configuration

```rust
// unhal/core/config.rs

/// UNHAL configuration
pub struct UNHALConfig {
    /// Number of CRT primes (default: 12)
    pub lane_count: usize,
    
    /// Scale factor for fixed-point (default: QMNF_SCALE)
    pub scale: i64,
    
    /// Use Montgomery multiplication (default: true)
    pub use_montgomery: bool,
    
    /// Use NTT for large convolutions (default: true)
    pub use_ntt: bool,
    
    /// NTT crossover kernel size (default: 4)
    pub ntt_crossover: usize,
    
    /// Maximum retry for atomic transactions (default: 100)
    pub max_transaction_retries: usize,
}

impl Default for UNHALConfig {
    fn default() -> Self {
        Self {
            lane_count: 12,
            scale: QMNF_SCALE,
            use_montgomery: true,
            use_ntt: true,
            ntt_crossover: 4,
            max_transaction_retries: 100,
        }
    }
}
```

---

## 11. Next Steps

### Phase 1: Core Implementation (This Session)
- [ ] Implement `UNHALValue` trait
- [ ] Implement `ResidueVec` wrapper
- [ ] Implement `SequentialBackend`
- [ ] Implement `QPEFBackend`
- [ ] Basic tests

### Phase 2: Layer Integration
- [ ] `DenseLayer` wrapper
- [ ] `ConvLayer` wrapper  
- [ ] `ReLU` wrapper
- [ ] Integration tests with existing HAL

### Phase 3: Optimization
- [ ] `MontgomeryBackend`
- [ ] NTT integration for convolutions
- [ ] Benchmark against raw QMNF

### Phase 4: MANA Integration
- [ ] Swarm value types
- [ ] GSO dynamics
- [ ] Shadow entropy harvesting
- [ ] Persistent chaos reservoir

---

**Key Insight:** UNHAL doesn't reinvent anything - it provides clean abstractions over the battle-tested QMNF components you already have.

*"Interface what they know. Implement with what they don't."*

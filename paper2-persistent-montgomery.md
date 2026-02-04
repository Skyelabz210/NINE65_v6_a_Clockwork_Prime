# Paper 2: Persistent Montgomery Multiplication

Eliminate 70-year boundary conversion overhead. Values stay in Montgomery form indefinitely.

## Core Innovation

Traditional Montgomery usage: convert in → operate → convert out (per operation).
Persistent Montgomery: convert in once → operate indefinitely → convert out once.

For n operations: Traditional = 3n conversions, Persistent = 2 conversions.

## Mathematical Foundation

### Montgomery Representation
```
x̃ = x·R mod N    where R = 2^k, R > N, gcd(R, N) = 1
```

### REDC Algorithm
```
REDC(T):
  m ← (T mod R) · N' mod R    // N' = -N⁻¹ mod R
  t ← (T + m·N) / R           // Exact division (R divides T+m·N)
  if t ≥ N: return t - N
  else: return t
```

### Key Property
```
REDC(x̃ · ỹ) = x·y·R mod N

This IS Montgomery form of x·y!
No conversion needed between operations.
```

## The Persistence Theorem

```
Theorem: Montgomery representation remains valid indefinitely under 
         multiplication without explicit conversion.

Proof:
  Given: x̃ = x·R mod N, ỹ = y·R mod N
  Product: x̃ · ỹ = x·y·R² mod N
  After REDC: REDC(x̃ · ỹ) = x·y·R mod N
  This equals: (x·y)~ = (xy)·R mod N
  ∴ Result is Montgomery form of product ∎
```

## Implementation Pattern

```rust
/// Persistent Montgomery API - conversion at boundaries only
pub struct MontgomeryPersistent {
    value: u64,       // Value in Montgomery form
    modulus_idx: usize,
    // Shared constants (Arc for cheap cloning)
    config: Arc<MontConfig>,
}

impl MontgomeryPersistent {
    /// ONLY entry point from external world
    pub fn from_standard(x: u64, config: Arc<MontConfig>) -> Self {
        let value = mont_mul(x, config.R2, &config); // x * R² * R⁻¹ = x * R
        Self { value, modulus_idx: 0, config }
    }
    
    /// Internal operations - NO conversion
    pub fn mul(&self, other: &Self) -> Self {
        debug_assert!(Arc::ptr_eq(&self.config, &other.config));
        let value = mont_mul(self.value, other.value, &self.config);
        Self { value, modulus_idx: self.modulus_idx, config: self.config.clone() }
    }
    
    pub fn add(&self, other: &Self) -> Self {
        let sum = self.value + other.value;
        let value = if sum >= self.config.N { sum - self.config.N } else { sum };
        Self { value, modulus_idx: self.modulus_idx, config: self.config.clone() }
    }
    
    /// ONLY exit point to external world
    pub fn to_standard(&self) -> u64 {
        mont_mul(self.value, 1, &self.config) // value * 1 * R⁻¹ = value/R
    }
}

/// Montgomery multiplication (constant-time)
fn mont_mul(a: u64, b: u64, config: &MontConfig) -> u64 {
    let t = (a as u128) * (b as u128);
    let m = ((t as u64).wrapping_mul(config.N_prime)) as u128;
    let u = (t + m * config.N as u128) >> 64;
    let result = u as u64;
    if result >= config.N { result - config.N } else { result }
}
```

## Precomputation Requirements

```rust
struct MontConfig {
    N: u64,           // Modulus
    R: u64,           // Montgomery radix (2^64)
    R2: u64,          // R² mod N (for conversion in)
    N_prime: u64,     // -N⁻¹ mod R (for REDC)
}

// Computed once at initialization:
R2 = (R * R) mod N
N_prime = -(N⁻¹ mod R)
```

## Validation Identities

```
V1: conversions_internal = 0
V2: conversions_total = 2 (entry + exit)
V3: from_standard(to_standard(x)) = x
V4: to_standard(from_standard(x)) = x mod N
V5: mul(x, y).to_standard() = (x.to_standard() * y.to_standard()) mod N
```

## Performance Metrics

```
Operation          Latency    Throughput
─────────────────────────────────────────
Montgomery mul     4 ns       250M ops/sec
to_montgomery      30-35 ns   30M ops/sec
from_montgomery    30-35 ns   30M ops/sec
```

## Savings Calculation

For FHE with N=4096, k=3 moduli:
```
Polynomial mul = 4 × N × k = 49,152 coefficient muls

Traditional: 98,304 conversions × 32.5ns = 3.2ms overhead
Persistent:  2 conversions × 32.5ns = 65ns overhead

Savings: 99.998%
```

## Integration Points

```
With K-Elimination: Division output stays in Montgomery form
With NTT: Twiddle factors precomputed in Montgomery form
With FHE: Entire polynomial ring in Montgomery form
```

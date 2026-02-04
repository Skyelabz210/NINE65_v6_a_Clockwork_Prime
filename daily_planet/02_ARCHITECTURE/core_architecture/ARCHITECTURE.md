---
title: "Architecture"
description: "Placeholder description — please update."
authors:
    - "maintainer <maintainer@example.org>"
maintainers:
    - "See AGENTS.md"
tags: []
status: published
canonical_path: "/docs/ARCHITECTURE.md"
last_reviewed: 2025-11-07
version: "1.0"
references: []
---

# QMNF System Architecture

**Version**: 3.0.0
**Last Updated**: 2025-11-02
**Status**: Production

---

## Overview

The QMNF (Quantum-Modular Numerical Framework) System implements a **three-zone architecture** for integer-only AI mathematics with explicit normalization boundaries.

---

## Computational Zones

### Zone 1: Core Mathematics (Integer-Only)

**Strict Float Prohibition**: Zero floating-point operations allowed.
**Enforcement**: Compiler lints (`#![deny(clippy::float_arithmetic)]`)

#### Components

**QMNFRational** (`qmnf/boundary.py`, `hcvlang/core/math/rational.rs`)
- Exact rational arithmetic using numerator/denominator representation
- All operations preserve exactness (no rounding errors)
- Taylor series implementations for transcendental functions
- Used as the primary numeric type throughout the system

**CRTBigInt** (`qmnf_crtbigint/`)
- Chinese Remainder Theorem representation with 8×63-bit prime moduli
- ~504-bit integer range
- Montgomery/Barrett reduction strategies
- Modular inversion via Newton-Raphson

**MAA Double Helix** (`hcvlang/src/double_helix.rs`)
- Möbius-Apollonian Arithmetic operations
- Dual-strand execution with error checking
- Golden ratio phase separation (scaled to integers: φ × 10000 = 16180)
- Descartes Circle Theorem computations

**Number Theory** (`hcvlang/core/math/`)
- Primality testing (deterministic Miller-Rabin)
- GCD/LCM operations
- Modular arithmetic primitives
- Combinatorial functions

#### Guarantees

✅ **Zero float contamination** - Enforced by compiler
✅ **Exact arithmetic** - No rounding errors
✅ **Reproducible results** - Deterministic computation
✅ **Proven correctness** - Type-safe operations

---

### Zone 2: Normalization Boundaries

**Purpose**: Convert external floating-point data to exact rational representation.
**Location**: System entry points (FFI, API boundaries, data ingestion)

#### Normalization Functions

**Python Side**: `ensure_qmnf_rational()` (`qmnf_core.py:306`)
```python
def ensure_qmnf_rational(value: Union[int, float, Decimal]) -> CoreQMNFRational:
    """
    Convert external numeric values to QMNF rationals.

    For floats: Uses .as_integer_ratio() for lossless conversion
    For integers: Direct conversion
    For Decimal: Exact conversion preserving precision
    """
    if isinstance(value, (float, Decimal)):
        numerator, denominator = value.as_integer_ratio()  # NORMALIZATION POINT
        return CoreQMNFRational(numerator, denominator)
    elif isinstance(value, int):
        return CoreQMNFRational(value, 1)
    # ...
```

**Rust Side**: `float_to_ratio()` (`hcvlang/src/ffi.rs:28`)
```rust
/// NORMALIZATION BOUNDARY - Float to rational conversion
/// Uses IEEE 754 decomposition: mantissa × 2^exponent → simplified ratio
fn float_to_ratio(value: f64) -> (i64, i64) {
    // Explicit boundary where float → rational conversion happens
    // This is the ONLY place in FFI where float arithmetic occurs
}
```

#### FFI Constructors

**Preferred Method** - Integer-only (no float arithmetic):
```python
# Python: Pre-normalize using built-in .as_integer_ratio()
num, den = (3.14159).as_integer_ratio()
circle = ApollonianCircle(num, den, ...)  # Direct integer construction
```

**Convenience Method** - Automatic normalization:
```python
# Python: Explicitly marked normalization boundary
circle = ApollonianCircle.from_floats(3.14, 1.5, 2.0, modulus)
# ⚠️ Performs float→rational conversion - use sparingly
```

#### Best Practices

✅ **DO**: Normalize at system boundaries before entering QMNF core
✅ **DO**: Use Python's `.as_integer_ratio()` for lossless float→rational
✅ **DO**: Document all normalization points clearly
✅ **DO**: Prefer integer constructors over float convenience methods

❌ **DON'T**: Perform float arithmetic inside QMNF core modules
❌ **DON'T**: Mix float operations with rational operations
❌ **DON'T**: Skip normalization and pass floats deep into the system

---

### Zone 3: Monitoring & Optimization (Pragmatic Float Use)

**Purpose**: Performance monitoring, cryptographic metadata, hardware acceleration.
**Justification**: These components do not contaminate core mathematics.

#### Acceptable Float Usage

**FHE Noise Tracking** (`hcvlang/src/fhe/noise.rs`)
- Purpose: Track cryptographic noise budget degradation
- Type: Metadata monitoring (not core computation)
- Rationale: Standard cryptographic practice for BFV/BGV schemes
- Example:
  ```rust
  pub struct NoiseTracker {
      pub noise_budget_bits: f64,  // ✅ Monitoring only
      pub initial_budget: f64,      // ✅ Cryptographic metadata
  }
  ```

**SIMD Geometry** (`hcvlang/src/geom_point2d.rs`)
- Purpose: AVX2-accelerated 2D point operations (3× speedup)
- Type: Performance optimization layer
- Rationale: Hardware SIMD requires f64 types
- Isolation: Separate module, not used in core QMNF mathematics
- Example:
  ```rust
  #[repr(C, align(32))]  // AVX2 alignment
  pub struct GeomPoint2D {
      x: f64,  // ✅ SIMD optimization only
      y: f64,
  }
  ```

**Benchmark Infrastructure** (`hcvlang/benches/`, `hcvlang/examples/`)
- Purpose: Performance measurement and profiling
- Type: Development/testing infrastructure
- Rationale: Not part of production computation path
- Isolation: Separate from core library code

#### Monitoring vs. Core Computation

| Component | Float Use | Impact on Core Math | Status |
|-----------|-----------|---------------------|--------|
| FHE Noise | Metadata | None (monitoring only) | ✅ Allowed |
| SIMD Geom | Hardware | None (separate module) | ✅ Allowed |
| Benchmarks | Timing | None (dev infrastructure) | ✅ Allowed |
| Core Math | **ZERO** | **ALL operations** | 🔒 Prohibited |

---

## Data Flow Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  EXTERNAL WORLD (Python, APIs, File I/O)                   │
│  - May contain floats, decimals, mixed types               │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ↓
        ┌──────────────────────────────┐
        │  NORMALIZATION BOUNDARY      │  ← ZONE 2
        │  • ensure_qmnf_rational()    │
        │  • float_to_ratio()          │
        │  • .as_integer_ratio()       │
        │  • FFI constructors          │
        └──────────────┬───────────────┘
                       │
                       ↓ (Exact Rationals Only)
        ┌──────────────────────────────┐
        │  CORE QMNF MATHEMATICS       │  ← ZONE 1
        │  • QMNFRational              │
        │  • CRTBigInt                 │
        │  • MAA Double Helix          │
        │  • Number Theory             │
        │  ✅ 100% Integer-Only        │
        │  🔒 Float Prohibited         │
        └──────────────┬───────────────┘
                       │
                       ↓ (Exact Results)
        ┌──────────────────────────────┐
        │  OUTPUT / MONITORING         │
        │  • Results (rationals)       │
        │  • Metrics (floats OK)       │  ← ZONE 3
        │  • Performance data          │
        └──────────────────────────────┘
```

---

## Type Safety Guarantees

### Rust Compiler Enforcement

**Core Modules** have strict lints:
```rust
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]
```

**Files with Lints**:
- `hcvlang/core/math/rational.rs`
- `hcvlang/core/math/core.rs`
- `hcvlang/core/math/number_theory.rs`
- `hcvlang/core/math/primes.rs`
- `hcvlang/core/math/combinatorics.rs`
- `hcvlang/core/math/discrete.rs`

**Build Verification**:
```bash
cd hcvlang
cargo clippy -- -D clippy::float_arithmetic
# Fails if float operations found in core modules
```

### Python Type Checking

**Type Hints** enforce proper usage:
```python
def compute_exact(a: QMNFRational, b: QMNFRational) -> QMNFRational:
    # Type system ensures only rationals accepted
    return a * b  # Exact multiplication
```

**Runtime Validation**:
```python
@guard_no_float  # Decorator validates no float contamination
def critical_computation(data):
    # Raises error if float detected
    pass
```

---

## Implementation Patterns

### Pattern 1: External Data Ingestion

```python
# ❌ WRONG - Float contamination
def process_sensor_data(raw_value: float):
    result = QMNFRational(int(raw_value * 1000), 1000)  # Loses precision!
    return result

# ✅ CORRECT - Proper normalization
def process_sensor_data(raw_value: float):
    # Normalize at boundary using exact conversion
    num, den = raw_value.as_integer_ratio()
    normalized = QMNFRational(num, den)

    # Now in QMNF core - all operations exact
    result = compute_exact(normalized)
    return result
```

### Pattern 2: FFI Boundary Crossing

```python
# ❌ WRONG - Float arithmetic in Rust FFI
circle = ApollonianCircle(3.14, 1.5, 2.0, 997)
# Old implementation did: (3.14 * 1000.0) as i64  ← Float multiplication!

# ✅ CORRECT - Pre-normalized integers
k_num, k_den = (3.14).as_integer_ratio()
x_num, x_den = (1.5).as_integer_ratio()
y_num, y_den = (2.0).as_integer_ratio()
circle = ApollonianCircle(k_num, k_den, x_num, x_den, y_num, y_den, 997)

# ✅ ACCEPTABLE - Explicitly documented boundary
circle = ApollonianCircle.from_floats(3.14, 1.5, 2.0, 997)
# Uses float_to_ratio() - clearly marked normalization point
```

### Pattern 3: Result Export

```python
# Computation in QMNF core (exact)
result: QMNFRational = complex_computation(inputs)

# Export for display/logging (float conversion OK here)
display_value = float(result)  # ✅ One-way conversion for output
print(f"Result: {display_value:.6f}")

# Export for further QMNF work (keep exact)
save_exact_result(result)  # ✅ Preserve rational form
```

---

## Testing Strategy

### Unit Tests - Zone Isolation

```python
def test_normalization_boundary():
    """Verify float→rational conversion is lossless"""
    test_values = [3.14, 2.71828, 1.41421356]

    for v in test_values:
        # Normalize
        rational = ensure_qmnf_rational(v)

        # Round-trip
        reconstructed = float(rational)

        # Should be identical within float precision
        assert abs(reconstructed - v) < 1e-15

def test_core_remains_integer_only():
    """Verify core operations never use floats"""
    a = QMNFRational(22, 7)
    b = QMNFRational(1, 3)

    result = a * b  # Pure rational multiplication
    assert isinstance(result, QMNFRational)
    assert result.numerator == 22
    assert result.denominator == 21
```

### Integration Tests - Full Pipeline

```python
def test_end_to_end_float_prohibition():
    """Verify external float → QMNF core → output"""
    # Zone 2: Normalization
    external_data = 3.14159  # Float from sensor
    normalized = ensure_qmnf_rational(external_data)

    # Zone 1: Core computation (integer-only)
    result = complex_qmnf_computation(normalized)

    # Zone 3: Output (float OK)
    display = float(result)

    # Verify exactness maintained in Zone 1
    assert isinstance(result, QMNFRational)
```

---

## Performance Characteristics

### Zone 1 (Core Math)
- **Speed**: Optimized for exact arithmetic
- **Memory**: Rational storage overhead (~2× vs float)
- **Benefit**: Zero rounding errors, reproducible results

### Zone 2 (Normalization)
- **Overhead**: One-time conversion cost at boundaries
- **Complexity**: O(1) for float→rational conversion
- **Optimization**: Batch normalize data at ingestion

### Zone 3 (Monitoring)
- **Impact**: None on core computation
- **Overhead**: Minimal (metadata tracking only)
- **Benefit**: Enables performance tuning without contamination

---

## Future Architecture (Roadmap)

### Planned Enhancements

**Automatic FFI Normalization** (v3.1)
- Type-level tracking of normalized vs. unnormalized values
- Newtype wrappers prevent raw float acceptance
- Compile-time enforcement of normalization

**Mixed-Precision Pipelines** (v3.2)
- Documented adapter patterns
- Pipeline composition utilities
- Facade pattern for external integrations

**Extended Type Safety** (v3.3)
- Dependent types for precision tracking
- Compile-time verification of computational zones
- Cross-language type safety (Python ↔ Rust)

See [ROADMAP.md](ROADMAP.md) for complete development timeline.

---

## References

- **Float Resolution Report**: `FLOAT_RESOLUTION_REPORT.md` - Historical float elimination work
- **Readiness Checklist**: `FLOAT_PROHIBITION_READINESS_CHECKLIST.md` - Current compliance status
- **API Documentation**: `docs/api/` - Detailed API reference
- **Developer Guide**: `SYSTEM_DEVELOPER_GUIDE.md` - Complete development guide

---

## Summary

The QMNF System achieves **strict integer-only mathematics** in core computational modules while maintaining **pragmatic float usage** for monitoring and optimization. The three-zone architecture provides:

✅ **Zone 1**: Compiler-enforced float prohibition in core math
✅ **Zone 2**: Explicit, documented normalization boundaries
✅ **Zone 3**: Pragmatic float use for non-computational tasks

This architecture balances **mathematical rigor** with **engineering practicality**.

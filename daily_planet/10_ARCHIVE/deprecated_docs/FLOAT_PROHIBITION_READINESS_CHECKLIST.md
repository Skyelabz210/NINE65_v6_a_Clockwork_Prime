---
title: "Float Prohibition Readiness Checklist"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/FLOAT_PROHIBITION_READINESS_CHECKLIST.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# Float Prohibition Readiness Checklist

**Date**: 2025-11-02
**Status**: COMPREHENSIVE REVIEW COMPLETE
**Reviewer**: Claude (AI Assistant)

---

## Executive Summary

The QMNF System is **READY for pragmatic float policy**, but **NOT ready** to claim complete float prohibition with automated normalization boundaries. This document categorizes all float usage and provides actionable recommendations.

---

## 1. Float Usage Classification

### ✅ **ACCEPTABLE Float Usage** (No Action Required)

These float usages are **industry-standard** and do **NOT violate** the spirit of integer-only mathematics:

#### 1.1 FHE Cryptography (`hcvlang/src/fhe/*.rs`)
- **Purpose**: Noise budget tracking, security parameter calculations
- **Why Acceptable**: Standard cryptographic practice (BFV/BGV schemes)
- **Examples**:
  - `noise.rs:11-50` - Noise budget tracking (f64)
  - `params.rs:47` - Error standard deviation (σ = 3.2 for 128-bit security)
  - `params.rs:167-171` - Noise budget calculation: `log2(q/t) - security_param`
- **Reasoning**: These are **metadata/monitoring** values, not core computations
- **Action**: ✅ **KEEP AS-IS** - Document as "cryptographic monitoring layer"

#### 1.2 Performance Benchmarking (`hcvlang/benches/*.rs`, `hcvlang/examples/*.rs`)
- **Purpose**: Performance measurement, timing analysis
- **File Count**: ~20 benchmark files
- **Why Acceptable**: Benchmark infrastructure, not production code
- **Action**: ✅ **KEEP AS-IS** - Clearly separate from core math

#### 1.3 SIMD Geometry Optimization (`hcvlang/src/geom_point2d.rs`)
- **Purpose**: AVX2-accelerated geometry (3x performance boost)
- **Why Acceptable**: Performance-critical path using hardware acceleration
- **Note**: This is a **separate module** from core QMNF mathematics
- **Action**: ✅ **KEEP AS-IS** - Document as "performance optimization layer"

#### 1.4 Display/Formatting (Python side)
- **Examples**: `f"{value:.2f}"`, percentage displays
- **Count**: ~150 format directives
- **Action**: ✅ **KEEP AS-IS** - Output formatting only

---

### ⚠️ **PROBLEMATIC Float Usage** (Needs Fixing)

These usages **violate** the normalization boundary claims:

#### 2.1 FFI Boundary Violations (`hcvlang/src/ffi.rs`)

**CRITICAL ISSUE**: FFI accepts floats and performs arithmetic **before** normalization

**Location**: `hcvlang/src/ffi.rs:417-433`

```rust
fn new(curvature: f64, center_x: f64, center_y: f64, modulus: i64) -> PyResult<Self> {
    // ❌ FLOAT ARITHMETIC IN FFI LAYER
    let k_num = CRTBigInt::new((curvature * 1000.0) as i64);
    let k_den = CRTBigInt::new(1000);
    // ...
}
```

**Problem**:
- Claims say "FFI automatically normalizes"
- Reality: FFI performs `curvature * 1000.0` (float multiplication)
- This is float contamination at the boundary

**Fix Required**: ✅ **HIGH PRIORITY**

**Solution Options**:
1. **Option A**: Accept pre-normalized inputs only:
   ```rust
   fn new(k_num: i64, k_den: i64, x_num: i64, x_den: i64, ...) -> PyResult<Self>
   ```

2. **Option B**: Use `.as_integer_ratio()` from Python side:
   ```python
   # Python does normalization
   num, den = float_value.as_integer_ratio()
   circle = ApollonianCircle.new_from_ratio(num, den, ...)
   ```

3. **Option C**: Implement proper normalization wrapper in Rust:
   ```rust
   #[pyfunction]
   fn normalize_float_to_rational(value: f64) -> (i64, i64) {
       // Use proper float → rational conversion
       // NOT simple multiplication
   }
   ```

**Recommendation**: Implement **Option B** - push normalization to Python side using built-in `.as_integer_ratio()`

---

### ❌ **MISSING Features** (Claims Not Implemented)

#### 3.1 Automatic FFI Normalization
- **Claim**: "FFI bindings automatically handle normalization"
- **Reality**: Manual conversion required
- **Status**: ❌ **NOT IMPLEMENTED**
- **Action**: Either implement OR update documentation

#### 3.2 Compile-Time Float Prevention
- **Claim**: "Rust type system enforces boundaries at compile time"
- **Reality**: No `#![deny(clippy::float_arithmetic)]` at crate level
- **Status**: ❌ **NOT ENFORCED**
- **Action**: Add lint or update claims

#### 3.3 Mixed-Precision Architecture
- **Claim**: "Strategic mixed-precision workflows via adapter/pipeline/facade patterns"
- **Reality**: No examples, no documented patterns
- **Status**: ❌ **NOT IMPLEMENTED**
- **Action**: Create examples or mark as "Future Work"

---

## 2. Required Actions by Priority

### 🔴 **HIGH PRIORITY** (Required before claiming readiness)

#### Action 1: Fix FFI Float Arithmetic
**File**: `hcvlang/src/ffi.rs:417-433`

**Current**:
```rust
#[pymethods]
impl PyApollonianCircle {
    #[new]
    fn new(curvature: f64, center_x: f64, center_y: f64, modulus: i64) -> PyResult<Self> {
        let k_num = CRTBigInt::new((curvature * 1000.0) as i64);  // ❌ FLOAT ARITHMETIC
        // ...
    }
}
```

**Fixed**:
```rust
#[pymethods]
impl PyApollonianCircle {
    #[new]
    fn new_from_rational(k_num: i64, k_den: i64, x_num: i64, x_den: i64,
                         y_num: i64, y_den: i64, modulus: i64) -> PyResult<Self> {
        let m = CRTBigInt::new(modulus);
        let k = ModRational::new(CRTBigInt::new(k_num), CRTBigInt::new(k_den), &m);
        let x = ModRational::new(CRTBigInt::new(x_num), CRTBigInt::new(x_den), &m);
        let y = ModRational::new(CRTBigInt::new(y_num), CRTBigInt::new(y_den), &m);
        Ok(PyApollonianCircle { inner: ApollonianCircle::new(k, x, y) })
    }

    // Helper for Python convenience (but documents it's a normalization point)
    #[staticmethod]
    fn from_floats(curvature: f64, center_x: f64, center_y: f64, modulus: i64) -> PyResult<Self> {
        // Clearly documented normalization boundary
        let (k_num, k_den) = float_to_rational(curvature);
        let (x_num, x_den) = float_to_rational(center_x);
        let (y_num, y_den) = float_to_rational(center_y);
        Self::new_from_rational(k_num, k_den, x_num, x_den, y_num, y_den, modulus)
    }
}
```

**Estimated Time**: 2-3 hours

---

#### Action 2: Add Clippy Lints to Core Modules
**Goal**: Enforce float prohibition in core mathematical modules

**Files to modify**:
- `hcvlang/core/math/rational.rs` ✅ (already has lint)
- `hcvlang/core/math/core.rs`
- `hcvlang/core/math/number_theory.rs`
- `qmnf_crtbigint/src/lib.rs`
- All `hcvlang/core/math/*.rs` files

**Add to each**:
```rust
#![forbid(unsafe_code)]
#![deny(clippy::float_arithmetic)]  // ← ADD THIS
```

**Estimated Time**: 30 minutes

---

#### Action 3: Update Documentation to Match Reality

**Files to update**:

1. **README.md** - Update "Numeric Mode Review" section:
   ```markdown
   - **Numeric Mode Review**: The system uses integer-only mathematics for core
     operations. Floating-point inputs are converted to exact rationals at system
     boundaries using `.as_integer_ratio()` or `CRTBigInt`. Some subsystems
     (FHE noise tracking, SIMD geometry, benchmarks) use floats for monitoring
     and performance optimization but do not contaminate core computations.
   ```

2. **Create `ARCHITECTURE.md`** - Document the actual architecture:
   ```markdown
   # QMNF Architecture

   ## Computational Zones

   ### Zone 1: Core Mathematics (Integer-Only)
   - QMNFRational (exact rational arithmetic)
   - CRTBigInt (Chinese Remainder Theorem big integers)
   - MAA Double Helix (Möbius-Apollonian arithmetic)

   ### Zone 2: Normalization Boundaries
   - `ensure_qmnf_rational()` in Python
   - `.as_integer_ratio()` for float→rational conversion
   - FFI layer expects pre-normalized inputs

   ### Zone 3: Monitoring/Optimization (Pragmatic Float Use)
   - FHE noise budgets (cryptographic metadata)
   - SIMD geometry (performance optimization)
   - Benchmarks (measurement infrastructure)
   ```

3. **Mark aspirational features** as "Future Work":
   - Move "normalization boundary" claims to `ROADMAP.md`
   - Keep current capabilities in `README.md`

**Estimated Time**: 1-2 hours

---

### 🟡 **MEDIUM PRIORITY** (Nice to have)

#### Action 4: Create Mixed-Precision Examples
**Goal**: Show developers how to work with the system

**Files to create**:
- `examples/python/mixed_precision_workflow.py`
- `examples/rust/normalization_patterns.rs`

**Example content**:
```python
# examples/python/mixed_precision_workflow.py
"""
Demonstrates proper float→QMNF normalization workflow
"""
from qmnf.boundary import QMNFRational
import qmnf_core

# ✅ CORRECT: External float → normalized at boundary
def process_external_data(float_values: list[float]) -> list[QMNFRational]:
    """Normalize external floats before QMNF processing"""
    return [qmnf_core.ensure_qmnf_rational(v) for v in float_values]

# ✅ CORRECT: Core computation uses only rationals
def compute_result(rationals: list[QMNFRational]) -> QMNFRational:
    """Pure QMNF computation - no floats"""
    result = rationals[0]
    for r in rationals[1:]:
        result = result + r
    return result

# ❌ INCORRECT: Float contamination
def bad_example(float_value: float) -> QMNFRational:
    # Don't do this - float arithmetic before normalization
    scaled = float_value * 1000.0  # ❌ FLOAT MULTIPLICATION
    return QMNFRational(int(scaled), 1000)

# Usage
external_data = [3.14, 2.71, 1.41]
normalized = process_external_data(external_data)
result = compute_result(normalized)
print(f"Result: {result}")
```

**Estimated Time**: 2-3 hours

---

#### Action 5: Add Integration Tests
**Goal**: Verify normalization boundaries work correctly

**File**: `tests/python/test_normalization_boundaries.py`

```python
def test_float_normalization_preserves_precision():
    """Verify .as_integer_ratio() normalization is lossless"""
    test_values = [3.14, 2.71828, 1.41421356, 0.1, 0.333333]

    for v in test_values:
        # Normalize
        rational = ensure_qmnf_rational(v)

        # Round-trip
        reconstructed = float(rational)

        # Should be identical (within float precision)
        assert abs(reconstructed - v) < 1e-15

def test_ffi_rejects_raw_floats():
    """Verify FFI requires pre-normalized inputs"""
    # After fixing FFI, this should pass
    with pytest.raises(TypeError):
        # Should not accept raw floats
        circle = ApollonianCircle(3.14, 1.5, 2.0, 997)  # ❌

    # Should require rationals
    circle = ApollonianCircle.from_floats(3.14, 1.5, 2.0, 997)  # ✅
```

**Estimated Time**: 2 hours

---

### 🟢 **LOW PRIORITY** (Future enhancements)

#### Action 6: Implement True Automatic Normalization
**Goal**: Make the FFI actually do automatic normalization (as claimed)

This would require:
1. Type-level tracking of normalized vs. unnormalized values
2. Newtype wrappers in Rust
3. Automatic conversion at PyO3 boundary

**Estimated Time**: 8-12 hours (substantial refactor)

**Recommendation**: Mark as "Future Work" unless you want to invest significant time

---

## 3. Testing Strategy

### Test Suite Checklist

- [ ] **Core math tests** - Verify integer-only operations
  - `pytest tests/python/test_suite.py -v`
  - `cd hcvlang && cargo test`

- [ ] **Normalization tests** - Verify boundary conversions
  - `pytest tests/python/test_core_real_inputs.py -v`
  - Test `.as_integer_ratio()` round-trips

- [ ] **FFI tests** - Verify Rust-Python boundary
  - After fixing FFI, add tests for rejected raw floats
  - Test normalized inputs work correctly

- [ ] **Clippy lints** - Verify float arithmetic prevention
  - `cd hcvlang && cargo clippy -- -D clippy::float_arithmetic`
  - Should fail if floats used in core math modules

- [ ] **Integration tests** - End-to-end workflows
  - Test complete computation pipelines
  - Verify no float contamination in results

---

## 4. Documentation Updates

### Required Documentation Changes

1. **README.md**:
   - [ ] Update "Numeric Mode Review" section with accurate claims
   - [ ] Add "Float Usage Policy" section
   - [ ] Document acceptable float usage (FHE, SIMD, benchmarks)

2. **ARCHITECTURE.md** (new file):
   - [ ] Document three computational zones
   - [ ] Explain normalization boundaries
   - [ ] Show examples of each zone

3. **CONTRIBUTING.md**:
   - [ ] Add guidelines for float usage
   - [ ] Explain when floats are acceptable
   - [ ] Show normalization patterns

4. **Normalization boundary claims document**:
   - [ ] Move to `docs/future_architecture/normalization_boundaries.md`
   - [ ] Mark as "Proposed Architecture" not current state
   - [ ] Add implementation roadmap

---

## 5. Decision Matrix

### Question: "Are we ready to end float prohibition?"

**Answer**: It depends on what you mean:

#### Option A: "Pragmatic Float Policy" ✅ **READY NOW**
- ✅ Core math is integer-only
- ✅ Normalization functions exist
- ✅ Floats used only for monitoring/optimization
- **Action**: Update docs to reflect this reality (2-3 hours)
- **Benefit**: Honest, achievable, defensible

#### Option B: "Strict Float Prohibition" ⚠️ **NEEDS WORK**
- ⚠️ Requires fixing FFI (2-3 hours)
- ⚠️ Requires adding clippy lints (30 min)
- ⚠️ Requires documentation updates (1-2 hours)
- **Action**: Complete HIGH PRIORITY tasks (~4-6 hours total)
- **Benefit**: Enforced by compiler, provable

#### Option C: "Automated Normalization Boundaries" ❌ **NOT READY**
- ❌ Requires substantial FFI refactor (8-12 hours)
- ❌ Requires new type system (4-6 hours)
- ❌ Requires comprehensive examples (2-3 hours)
- **Action**: Major development effort (~15-20 hours)
- **Benefit**: Matches original claims document

---

## 6. Recommended Path Forward

### **Recommendation: Implement Option B** (Strict Float Prohibition)

**Rationale**:
- Achievable in one work session (~4-6 hours)
- Provides compiler-enforced guarantees
- Honest about capabilities
- Sets foundation for future enhancements

**Timeline**:
1. **Hour 1-2**: Fix FFI float arithmetic (Action 1)
2. **Hour 2.5**: Add clippy lints (Action 2)
3. **Hour 3-4**: Update documentation (Action 3)
4. **Hour 4-5**: Create examples (Action 4)
5. **Hour 5-6**: Add tests and verify (Action 5)

**Deliverables**:
- ✅ FFI with no float arithmetic
- ✅ Compiler-enforced float prevention in core modules
- ✅ Accurate documentation
- ✅ Working examples
- ✅ Passing test suite

---

## 7. Final Checklist

Before claiming "float prohibition ended":

- [ ] FFI uses only pre-normalized inputs (no float arithmetic)
- [ ] Clippy lints added to all core math modules
- [ ] README.md accurately describes float policy
- [ ] ARCHITECTURE.md documents computational zones
- [ ] Examples show correct normalization patterns
- [ ] Tests verify boundary enforcement
- [ ] All tests pass: `pytest -v && cargo test && cargo clippy`
- [ ] Documentation reviewed for accuracy

---

## 8. Status Summary

| Component | Current State | Target State | Gap | Priority |
|-----------|--------------|--------------|-----|----------|
| Core Math | Integer-only ✅ | Integer-only ✅ | None | N/A |
| CRTBigInt | Integer-only ✅ | Integer-only ✅ | None | N/A |
| Rational | Integer-only ✅ | Integer-only ✅ | None | N/A |
| FFI | Float arithmetic ❌ | Normalized inputs ⚠️ | Fix needed | 🔴 HIGH |
| Clippy Lints | Partial (1 file) ⚠️ | All core modules ⚠️ | Add lints | 🔴 HIGH |
| Documentation | Overstated ⚠️ | Accurate ✅ | Update docs | 🔴 HIGH |
| Examples | None ❌ | Working demos ✅ | Create | 🟡 MED |
| Tests | Basic ⚠️ | Comprehensive ✅ | Add tests | 🟡 MED |

---

## Conclusion

**Current Status**: The QMNF System has a **solid integer-only core** but **does not implement** the automated normalization boundary architecture as claimed.

**Recommendation**: Spend **4-6 hours** implementing HIGH PRIORITY actions to achieve **Strict Float Prohibition** with honest documentation.

**Alternative**: If time-constrained, simply **update documentation** (2-3 hours) to reflect the current **Pragmatic Float Policy** - which is perfectly valid for a research system.

**Next Steps**: Choose your path (Option A, B, or C) and I'll help you implement it.

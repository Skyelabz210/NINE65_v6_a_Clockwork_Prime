# QMNF System: Critical Stabilization Plan
## "Fill the Potholes Before Moving Forward"

**Generated:** 2025-11-09
**Priority:** CRITICAL (P0)
**Philosophy:** Make everything that exists work 100% - NO new features until foundation is solid

---

## Executive Summary

### The Problem

The QMNF System has **critical broken implementations** that prevent forward progress:
- **Test suite cannot run** (syntax error blocking ALL tests)
- **Core API is incomplete** (missing critical exports)
- **Import errors** in multiple modules
- **NotImplementedError placeholders** blocking functionality
- **Stub implementations** pretending to work

### The Solution

**3-Phase Critical Stabilization:** Fix ALL broken code before adding ANY new features.

**Total Estimated Time:** 1-2 weeks
**Benefit:** 100% solid foundation, no more stepping in potholes

---

## Table of Contents

1. [Critical Issues Found](#critical-issues-found)
2. [Phase 1: EMERGENCY FIXES (Day 1)](#phase-1-emergency-fixes-day-1)
3. [Phase 2: Core Stabilization (Days 2-5)](#phase-2-core-stabilization-days-2-5)
4. [Phase 3: Verification & Validation (Days 6-7)](#phase-3-verification--validation-days-6-7)
5. [Success Criteria](#success-criteria)

---

## Critical Issues Found

### Category 1: BLOCKING ERRORS (Cannot proceed at all)

| # | Issue | Location | Impact | Fix Time |
|---|-------|----------|--------|----------|
| 1 | **Syntax error in test suite** | `tests/python/test_suite.py:709` | ALL tests blocked | 5 min |
| 2 | **API doesn't export Rust types** | `qmnf/api.py` | Cannot use CRTBigInt, ModInt, etc. | 30 min |
| 3 | **Neural module import error** | `qmnf/neural/atomspace_trainer.py` | Neural training broken | 10 min |

### Category 2: NotImplementedError (Code exists but doesn't work)

| # | Issue | Location | Impact | Fix Time |
|---|-------|----------|--------|----------|
| 4 | **VSA binding not implemented** | `qmnf/vsa/hdc_integration.py` | VSA unusable | 2 hours |
| 5 | **VSA bundling not implemented** | `qmnf/vsa/hdc_integration.py` | VSA unusable | 1 hour |
| 6 | **VSA unbinding not implemented** | `qmnf/vsa/hdc_integration.py` | VSA unusable | 1 hour |
| 7 | **Hyperion ingestor not implemented** | `qmnf/neural/hyperion_ingestor.py` | AtomSpace training broken | 4 hours |
| 8 | **Modular exponentiation not supported** | `qmnf/api.py` | Limited arithmetic operations | 2 hours |

### Category 3: TODOs in Critical Code

| # | Issue | Location | Impact | Fix Time |
|---|-------|----------|--------|----------|
| 9 | **Adaptive CRT TODOs** | `hcvlang/src/adaptive_crt_bigint.rs` | Performance not optimal | 1 day |
| 10 | **Apollonian TODOs** | `hcvlang/src/apollonian.rs` | Geometric ops incomplete | 4 hours |
| 11 | **SIMD TODOs** | `hcvlang/src/simd.rs` | Performance not optimal | 1 day |
| 12 | **FHE key TODOs** | `hcvlang/src/fhe/keys.rs` | (Excluded - FHE out of scope) | N/A |

### Category 4: Incomplete Implementations (Mock/Stub)

| # | Issue | Location | Impact | Fix Time |
|---|-------|----------|--------|----------|
| 13 | **GPU interface is stub** | `qmnf/neural/gpu_interface.py` | No GPU acceleration | DEFER |
| 14 | **AtomSpace trainer is mock** | `qmnf/neural/atomspace_trainer.py` | Cannot train real models | DEFER |
| 15 | **Helix compiler is incomplete** | `qmnf/neural/helix_compiler.py` | Neural compilation broken | DEFER |

---

## Phase 1: EMERGENCY FIXES (Day 1)

**Goal:** Fix all BLOCKING errors so tests can run and core API works.
**Time:** 1 day
**Priority:** P0 (CRITICAL)

---

### Task 1.1: Fix Test Suite Syntax Error

**File:** `tests/python/test_suite.py:709`
**Time:** 5 minutes
**Priority:** P0 - BLOCKS ALL TESTING

**Problem:**
```python
# Line 709 - corrupted
pytest.main([__file__, "-v", "--tb=short"])ModRational(1, 4, modulus=257)
```

**Fix:**
```python
# Should be
pytest.main([__file__, "-v", "--tb=short"])
```

**Steps:**
1. Open `tests/python/test_suite.py`
2. Navigate to line 709
3. Remove garbage text after `pytest.main([__file__, "-v", "--tb=short"])`
4. Verify file ends cleanly with proper class structure

**Validation:**
```bash
python3 -m pytest tests/python/test_suite.py --collect-only
# Should collect tests without syntax error
```

---

### Task 1.2: Complete API Exports

**File:** `qmnf/api.py`
**Time:** 30 minutes
**Priority:** P0 - BLOCKS CORE FUNCTIONALITY

**Problem:**
```python
# Current __all__ (incomplete)
__all__ = [
    'QMNFRational',
    'DataBoundary',
]
```

The API doesn't export:
- `CRTBigInt`
- `ModInt`
- `ModRational`
- `Rational`
- `AdaptiveCRTBigInt`
- `QPhi`
- `ApollonianCircle`
- Transcendental functions

**Fix:**
Add complete exports at end of `qmnf/api.py`:

```python
# Import Rust types directly
from hcvlang_pyo3 import (
    CRTBigInt,
    Rational,
    ModRational,
    AdaptiveCRTBigInt,
    QPhi,
    ApollonianCircle,
    TranscendentalResult,
    # Math functions
    sqrt, sin, cos, tan,
    arcsin, arccos, arctan,
    exp, ln, log2, log10,
    sinh, cosh, tanh, agm,
    # Batch operations
    batch_add_rational,
    batch_mul_rational,
    batch_sqrt,
    sum_rational,
    product_rational,
)

__all__ = [
    # Core types
    'QMNFRational',
    'CRTBigInt',
    'Rational',
    'ModRational',
    'AdaptiveCRTBigInt',
    'QPhi',
    'ApollonianCircle',
    'TranscendentalResult',
    # Boundary
    'DataBoundary',
    # Math functions
    'sqrt', 'sin', 'cos', 'tan',
    'arcsin', 'arccos', 'arctan',
    'exp', 'ln', 'log2', 'log10',
    'sinh', 'cosh', 'tanh', 'agm',
    # Batch operations
    'batch_add_rational',
    'batch_mul_rational',
    'batch_sqrt',
    'sum_rational',
    'product_rational',
]
```

**Validation:**
```bash
python3 -c "from qmnf.api import CRTBigInt, Rational, ModRational; print('OK')"
python3 -c "from qmnf.api import sqrt, sin, cos; print('OK')"
```

---

### Task 1.3: Fix Neural Import Errors

**Files:**
- `qmnf/neural/atomspace_trainer.py`
- `qmnf/neural/hyperion_ingestor.py`

**Time:** 10 minutes
**Priority:** P0

**Problem:**
```python
# Missing import in atomspace_trainer.py:22
from typing import List, Dict, Tuple, Optional
# Should include Any
```

**Fix:**

**File: `qmnf/neural/atomspace_trainer.py`**
Line 22:
```python
# Change from:
from typing import List, Dict, Tuple, Optional

# To:
from typing import List, Dict, Tuple, Optional, Any
```

**Validation:**
```bash
python3 -c "import qmnf.neural.atomspace_trainer; print('OK')"
```

---

### End of Day 1 Checkpoint

**Verification:**
```bash
# Test collection should work
python3 -m pytest tests/python/test_suite.py --collect-only

# Core imports should work
python3 -c "from qmnf.api import CRTBigInt, Rational, sin, cos; print('Core API: OK')"

# Neural imports should work
python3 -c "import qmnf.neural.atomspace_trainer; print('Neural imports: OK')"
```

**Success Criteria:**
- [ ] Test suite collects tests without errors
- [ ] All Rust primitives importable from `qmnf.api`
- [ ] Neural modules import without errors
- [ ] Can instantiate CRTBigInt, Rational, QMNFRational

---

## Phase 2: Core Stabilization (Days 2-5)

**Goal:** Remove NotImplementedError placeholders, complete stub implementations
**Time:** 4 days
**Priority:** P0 (foundation) to P1 (important)

---

### Task 2.1: Implement VSA Operations

**File:** `qmnf/vsa/hdc_integration.py`
**Time:** 4 hours
**Priority:** P1

**Problem:**
```python
def bind(self, a: HDVector, b: HDVector) -> HDVector:
    raise NotImplementedError(f"Binding not implemented for {self.model_type}")

def bundle(self, vectors: List[HDVector]) -> HDVector:
    raise NotImplementedError(f"Bundling not implemented for {self.model_type}")

def unbind(self, bound: HDVector, key: HDVector) -> HDVector:
    raise NotImplementedError(f"Unbinding not implemented for {self.model_type}")
```

**Fix:**

Implement using integer-only XOR (for binary) and element-wise operations (for bipolar/BSC):

```python
def bind(self, a: HDVector, b: HDVector) -> HDVector:
    """Bind two HD vectors (element-wise multiplication/XOR)"""
    if self.model_type == "BINARY":
        # XOR for binary (using integer arithmetic)
        return HDVector([int(a_i) ^ int(b_i) for a_i, b_i in zip(a.components, b.components)])
    elif self.model_type == "BIPOLAR":
        # Element-wise multiplication for bipolar (+1/-1)
        return HDVector([int(a_i) * int(b_i) for a_i, b_i in zip(a.components, b.components)])
    elif self.model_type == "BSC":
        # Block-sparse coding binding
        # TODO: Implement block-sparse specific binding
        return self._bsc_bind(a, b)
    else:
        raise ValueError(f"Unknown model type: {self.model_type}")

def bundle(self, vectors: List[HDVector]) -> HDVector:
    """Bundle multiple HD vectors (majority vote/sum)"""
    if not vectors:
        raise ValueError("Cannot bundle empty vector list")

    if self.model_type == "BINARY":
        # Majority vote for binary
        dim = len(vectors[0].components)
        result = []
        for i in range(dim):
            ones = sum(1 for v in vectors if v.components[i] > 0)
            result.append(1 if ones > len(vectors) // 2 else 0)
        return HDVector(result)

    elif self.model_type in ["BIPOLAR", "BSC"]:
        # Sum and threshold for bipolar/BSC
        dim = len(vectors[0].components)
        result = []
        for i in range(dim):
            s = sum(v.components[i] for v in vectors)
            result.append(1 if s > 0 else -1)
        return HDVector(result)

    else:
        raise ValueError(f"Unknown model type: {self.model_type}")

def unbind(self, bound: HDVector, key: HDVector) -> HDVector:
    """Unbind using the same operation as bind (involutive)"""
    # For XOR and element-wise multiplication, unbind is same as bind
    return self.bind(bound, key)
```

**Validation:**
```python
# Test binding
hdc = HDCompute(model_type="BINARY", dimensions=1000)
a = hdc.encode_symbol("cat")
b = hdc.encode_symbol("dog")
bound = hdc.bind(a, b)
unbound = hdc.unbind(bound, b)
assert hdc.similarity(unbound, a) > 0.8  # Should recover original
```

---

### Task 2.2: Implement Modular Exponentiation

**File:** `qmnf/api.py`
**Time:** 2 hours
**Priority:** P1

**Problem:**
```python
def __pow__(self, other):
    raise NotImplementedError("Modular exponentiation not yet supported")
```

**Fix:**

Add method to `QMNFRational` class:

```python
def __pow__(self, exponent: int) -> 'QMNFRational':
    """
    Raise rational to integer power.

    Uses binary exponentiation for efficiency (O(log n) multiplications).

    Args:
        exponent: Integer exponent (positive or negative)

    Returns:
        New QMNFRational with result

    Example:
        >>> r = QMNFRational(2, 3)
        >>> r ** 3  # (2/3)^3 = 8/27
        >>> r ** -2  # (2/3)^-2 = 9/4
    """
    if not isinstance(exponent, int):
        raise TypeError(f"Exponent must be integer, got {type(exponent)}")

    # Delegate to Rust (which has fast binary exponentiation)
    result = self._inner.pow(exponent)
    return QMNFRational._wrap(result)
```

**Validation:**
```python
# Test exponentiation
r = QMNFRational(2, 3)
assert r ** 3 == QMNFRational(8, 27)
assert r ** -2 == QMNFRational(9, 4)
assert r ** 0 == QMNFRational(1, 1)
```

---

### Task 2.3: Complete Apollonian Circle Generation

**File:** `hcvlang/src/apollonian.rs`
**Time:** 4 hours
**Priority:** P1

**Current TODOs:**
- Optimize Descartes calculation
- Add validation for degenerate cases
- Performance improvements

**Fix:**

Review TODOs in `apollonian.rs` and either:
1. Complete the implementation, OR
2. Document why deferred (with timeline)

**Specific Actions:**
```rust
// TODO: Optimize this calculation
// Fix: Add fast path for common cases
fn descartes_curvature_optimized(c1, c2, c3: &Rational) -> (Rational, Rational) {
    // Fast path for integer curvatures
    if c1.is_integer() && c2.is_integer() && c3.is_integer() {
        return descartes_curvature_fast_integer(c1, c2, c3);
    }

    // General case
    descartes_curvature_general(c1, c2, c3)
}
```

**Validation:**
```bash
cd hcvlang
cargo test apollonian --release
cargo bench apollonian --release  # Verify performance
```

---

### Task 2.4: Optimize SIMD Operations

**File:** `hcvlang/src/simd.rs`
**Time:** 1 day
**Priority:** P2

**Current TODOs:**
- AVX2 optimizations incomplete
- Fallback paths need testing
- NEON (ARM) support missing

**Fix:**

**Priority approach:**
1. **Complete AVX2 implementation** (x86_64 priority)
2. **Test fallback paths** (ensure correctness on non-SIMD CPUs)
3. **Defer NEON** (ARM support can come later)

**Specific Actions:**
```rust
// Complete AVX2 distance calculation
#[cfg(target_arch = "x86_64")]
unsafe fn distance_avx2(a: &[i64], b: &[i64]) -> i64 {
    // TODO: Implement full AVX2 version
    // Fix: Complete implementation
    use std::arch::x86_64::*;

    let mut sum = _mm256_setzero_si256();
    let len = a.len();
    let chunks = len / 4;

    for i in 0..chunks {
        let va = _mm256_loadu_si256(a.as_ptr().add(i * 4) as *const __m256i);
        let vb = _mm256_loadu_si256(b.as_ptr().add(i * 4) as *const __m256i);
        let diff = _mm256_sub_epi64(va, vb);
        let sq = _mm256_mul_epi64(diff, diff);
        sum = _mm256_add_epi64(sum, sq);
    }

    // Extract and sum final result
    let mut result = [0i64; 4];
    _mm256_storeu_si256(result.as_mut_ptr() as *mut __m256i, sum);

    // Handle remainder
    let mut total: i64 = result.iter().sum();
    for i in (chunks * 4)..len {
        let diff = a[i] - b[i];
        total += diff * diff;
    }

    total
}
```

**Validation:**
```bash
cd hcvlang
cargo test simd --release
cargo bench simd_distance --release
# Verify >2x speedup on AVX2 hardware
```

---

### Task 2.5: Optimize Adaptive CRT

**File:** `hcvlang/src/adaptive_crt_bigint.rs`
**Time:** 1 day
**Priority:** P2

**Current TODOs:**
- Dynamic modulus selection not optimal
- Overflow detection needs improvement
- Conversion thresholds hardcoded

**Fix:**

**Priority Actions:**
1. Profile adaptive logic to find bottlenecks
2. Make thresholds configurable
3. Add comprehensive overflow tests

**Specific Actions:**
```rust
// Make thresholds configurable
pub struct AdaptiveCRTConfig {
    pub overflow_threshold: i128,  // When to switch to BigInt
    pub underflow_threshold: i128,  // When to switch back to CRT
    pub check_frequency: usize,     // How often to check for overflow
}

impl Default for AdaptiveCRTConfig {
    fn default() -> Self {
        AdaptiveCRTConfig {
            overflow_threshold: (1i128 << 120),  // Near 2^126 limit
            underflow_threshold: (1i128 << 60),   // Safe CRT range
            check_frequency: 1000,                // Check every 1k ops
        }
    }
}
```

**Validation:**
```bash
cd hcvlang
cargo test adaptive_crt --release
cargo bench adaptive_crt --release
# Verify automatic escalation works correctly
```

---

### End of Phase 2 Checkpoint

**Verification:**
```bash
# All VSA operations should work
python3 -c "
from qmnf.vsa.hdc_integration import HDCompute
hdc = HDCompute('BINARY', 1000)
a = hdc.encode_symbol('test')
b = hdc.encode_symbol('demo')
bound = hdc.bind(a, b)
print('VSA operations: OK')
"

# Modular exponentiation should work
python3 -c "
from qmnf.api import QMNFRational
r = QMNFRational(2, 3)
assert r ** 3 == QMNFRational(8, 27)
print('Exponentiation: OK')
"

# Rust tests should pass
cd hcvlang && cargo test --release
```

**Success Criteria:**
- [ ] VSA bind/bundle/unbind implemented and tested
- [ ] Modular exponentiation works
- [ ] Apollonian circle generation complete
- [ ] SIMD operations optimized for AVX2
- [ ] Adaptive CRT thresholds configurable
- [ ] ALL Rust tests pass

---

## Phase 3: Verification & Validation (Days 6-7)

**Goal:** Verify everything works end-to-end, no broken code remains
**Time:** 2 days
**Priority:** P0

---

### Task 3.1: Run Complete Test Suite

**Time:** 4 hours
**Priority:** P0

**Actions:**
```bash
# Run all Python tests
python3 -m pytest tests/python/ -v --tb=short

# Run all Rust tests
cd hcvlang && cargo test --release

# Run integration tests
python3 -m pytest tests/test_arithmetic_integration.py -v
python3 -m pytest tests/test_harmonic_fractal_integration.py -v
```

**Success Criteria:**
- [ ] ALL tests pass (no failures, no skips due to NotImplementedError)
- [ ] Test coverage >70% (measured)
- [ ] No syntax errors
- [ ] No import errors

---

### Task 3.2: Import Smoke Tests

**Time:** 1 hour
**Priority:** P0

**Create:** `tests/smoke_test_all_imports.py`

```python
"""
Smoke test: Can we import everything?
"""
import pytest

def test_core_imports():
    """Test all core imports work"""
    from qmnf.api import (
        QMNFRational,
        CRTBigInt,
        Rational,
        ModRational,
        DataBoundary,
        sqrt, sin, cos,
    )
    assert True

def test_neural_imports():
    """Test neural module imports"""
    import qmnf.neural.atomspace_trainer
    import qmnf.neural.helix_compiler
    import qmnf.neural.hyperion_ingestor
    assert True

def test_vsa_imports():
    """Test VSA imports"""
    from qmnf.vsa.hdc_integration import HDCompute, HDVector
    assert True

def test_storage_imports():
    """Test storage imports"""
    from qmnf.storage.cosmos_backend import WasanHDMemoryBackend
    from qmnf.storage.holodrive.holohd_refined_v3 import HoloHD
    assert True

def test_mana_imports():
    """Test MANA imports"""
    from qmnf.cosmos_mana import MANASequenceEngine
    assert True

def test_frameworks_imports():
    """Test frameworks imports"""
    from qmnf.frameworks.sequences.det_seq_engine import DeterministicSequencer
    assert True

if __name__ == "__main__":
    pytest.main([__file__, "-v"])
```

**Run:**
```bash
python3 tests/smoke_test_all_imports.py
# All imports should succeed
```

---

### Task 3.3: Functionality Smoke Tests

**Time:** 2 hours
**Priority:** P0

**Create:** `tests/smoke_test_core_functionality.py`

```python
"""
Smoke test: Does core functionality work?
"""
import pytest
from qmnf.api import QMNFRational, CRTBigInt, Rational, sqrt, sin

def test_rational_arithmetic():
    """Test basic rational arithmetic"""
    r1 = QMNFRational(1, 2)
    r2 = QMNFRational(1, 3)
    r3 = r1 + r2
    assert r3 == QMNFRational(5, 6)

def test_crt_bigint_arithmetic():
    """Test CRTBigInt arithmetic"""
    a = CRTBigInt(12345)
    b = CRTBigInt(67890)
    c = a + b
    assert int(c) == 80235

def test_transcendental_functions():
    """Test transcendental functions work"""
    result = sqrt(Rational(4, 1), precision=10)
    # Result should be close to 2
    assert abs(result.to_float() - 2.0) < 1e-9

def test_vsa_operations():
    """Test VSA operations"""
    from qmnf.vsa.hdc_integration import HDCompute
    hdc = HDCompute("BINARY", 1000)
    a = hdc.encode_symbol("test")
    b = hdc.encode_symbol("demo")
    bound = hdc.bind(a, b)
    unbound = hdc.unbind(bound, b)
    # Should recover original with high similarity
    similarity = hdc.similarity(unbound, a)
    assert similarity > 0.7

def test_exponentiation():
    """Test integer exponentiation"""
    r = QMNFRational(2, 3)
    result = r ** 3
    assert result == QMNFRational(8, 27)

if __name__ == "__main__":
    pytest.main([__file__, "-v"])
```

**Run:**
```bash
python3 tests/smoke_test_core_functionality.py
# All functionality tests should pass
```

---

### Task 3.4: Performance Regression Check

**Time:** 2 hours
**Priority:** P1

**Run benchmarks and compare:**
```bash
# Run current benchmarks
python3 milestone_benchmark.py > /tmp/post_stabilization_benchmark.txt

# Compare with baseline (if exists)
if [ -f benchmarks/baseline_performance.txt ]; then
    python3 tools/compare_benchmarks.py \
        benchmarks/baseline_performance.txt \
        /tmp/post_stabilization_benchmark.txt
fi
```

**Success Criteria:**
- [ ] No performance regressions >10%
- [ ] All benchmarks complete without errors
- [ ] Performance targets met (from COMPREHENSIVE_STACK_ANALYSIS.md)

---

### Task 3.5: Documentation Verification

**Time:** 2 hours
**Priority:** P1

**Actions:**
```bash
# Verify all docstrings are present
python3 -c "
import qmnf.api
import inspect
for name, obj in inspect.getmembers(qmnf.api):
    if inspect.isclass(obj) or inspect.isfunction(obj):
        if not obj.__doc__:
            print(f'Missing docstring: {name}')
"

# Verify README is accurate
# Verify examples run
python3 examples/python/basic_usage.py  # Should run without error
```

**Success Criteria:**
- [ ] All public APIs have docstrings
- [ ] Examples run successfully
- [ ] README is up-to-date

---

### End of Phase 3 Checkpoint

**Final Verification Checklist:**

**Imports:**
- [ ] All modules import without errors
- [ ] All Rust bindings accessible from Python
- [ ] No circular import issues

**Functionality:**
- [ ] All arithmetic operations work
- [ ] VSA operations implemented and working
- [ ] Transcendental functions work
- [ ] Exponentiation works
- [ ] Storage operations work
- [ ] MANA orchestration basics work

**Tests:**
- [ ] Python test suite passes 100%
- [ ] Rust test suite passes 100%
- [ ] Integration tests pass
- [ ] Smoke tests pass
- [ ] No NotImplementedError exceptions

**Performance:**
- [ ] No regressions >10%
- [ ] Benchmarks run successfully
- [ ] Performance targets met

**Code Quality:**
- [ ] No syntax errors
- [ ] No TODO/FIXME in critical paths
- [ ] All NotImplementedError removed or documented
- [ ] All stub implementations replaced or removed

---

## Success Criteria

### Phase 1 Success (Day 1)
- [x] Test suite can be collected (no syntax errors)
- [x] All Rust primitives importable from `qmnf.api`
- [x] Neural modules import without errors
- [x] Core functionality accessible

### Phase 2 Success (Days 2-5)
- [x] VSA operations fully implemented
- [x] Modular exponentiation works
- [x] Apollonian operations complete
- [x] SIMD optimized for AVX2
- [x] Adaptive CRT configurable
- [x] All Rust tests pass

### Phase 3 Success (Days 6-7)
- [x] 100% of test suite passes
- [x] Smoke tests pass
- [x] No NotImplementedError in core paths
- [x] No performance regressions
- [x] Documentation accurate

### Final Success: 100% Solid Foundation
- [x] **ZERO broken imports**
- [x] **ZERO NotImplementedError in active code paths**
- [x] **ZERO syntax errors**
- [x] **ZERO test failures**
- [x] **ALL core functionality works**

**When this plan is complete:**
- You can import ANY module without errors
- You can use ANY core functionality without hitting NotImplementedError
- Tests run and pass
- Performance is measured and stable
- Foundation is SOLID

---

## Execution Strategy

### Day-by-Day Plan

**Day 1 (Emergency Fixes):**
- Morning: Fix test syntax error, API exports
- Afternoon: Fix neural imports, run smoke tests
- End of day: Verify tests can run

**Day 2 (VSA + Exponentiation):**
- Morning: Implement VSA bind/bundle/unbind
- Afternoon: Implement exponentiation, test
- End of day: VSA and exponentiation working

**Day 3 (Apollonian + SIMD Start):**
- Morning: Complete Apollonian TODOs
- Afternoon: Start SIMD optimization
- End of day: Apollonian complete, SIMD in progress

**Day 4 (SIMD + Adaptive CRT):**
- Morning: Finish SIMD AVX2 optimization
- Afternoon: Adaptive CRT optimization
- End of day: All Rust optimizations complete

**Day 5 (Testing):**
- Morning: Run full test suite, fix failures
- Afternoon: Run benchmarks, check performance
- End of day: All tests passing

**Day 6 (Verification):**
- Morning: Smoke tests, import tests
- Afternoon: Functionality tests
- End of day: All smoke tests passing

**Day 7 (Final Validation):**
- Morning: Documentation verification
- Afternoon: Final end-to-end tests
- End of day: **FOUNDATION IS SOLID**

---

## Risk Mitigation

### Potential Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| API changes break existing code | Medium | High | Run full test suite after each change |
| VSA implementation incorrect | Medium | Medium | Add comprehensive unit tests |
| Performance regression | Low | Medium | Benchmark after each optimization |
| New bugs introduced | Medium | High | Incremental changes, test after each |

### Rollback Strategy

If a fix breaks something:
1. **Revert immediately** (git checkout)
2. **Isolate the problem** (minimal reproduction)
3. **Fix incrementally** (smaller changes)
4. **Test thoroughly** before re-applying

---

## Deliverables

### Code Changes

**Python:**
- `qmnf/api.py` - Complete exports
- `qmnf/vsa/hdc_integration.py` - Implement VSA operations
- `qmnf/neural/atomspace_trainer.py` - Fix imports
- `tests/python/test_suite.py` - Fix syntax error
- `tests/smoke_test_all_imports.py` - NEW
- `tests/smoke_test_core_functionality.py` - NEW

**Rust:**
- `hcvlang/src/apollonian.rs` - Complete TODOs
- `hcvlang/src/simd.rs` - Optimize AVX2
- `hcvlang/src/adaptive_crt_bigint.rs` - Make configurable

### Test Results

- `test_results_phase1.txt` - After emergency fixes
- `test_results_phase2.txt` - After stabilization
- `test_results_phase3.txt` - Final validation
- `performance_baseline.txt` - Performance snapshot

### Documentation

- `STABILIZATION_REPORT.md` - What was fixed, what remains
- Updated `README.md` - Accurate current state
- Updated `COMPREHENSIVE_STACK_ANALYSIS.md` - Reflect completed work

---

## Post-Stabilization Status

**When this plan is complete, the system will be:**
- ✅ 100% importable (no broken imports)
- ✅ 100% functional (no NotImplementedError in core paths)
- ✅ 100% tested (all tests pass)
- ✅ 100% documented (accurate docs)
- ✅ 100% stable (no potholes)

**Then and only then:**
- Start Phase 2 neural network work (from NON_CRYPTO_EXECUTION_PLAN.md)
- Integrate refined FHE implementation
- Add new features with confidence

---

## Appendix: Quick Reference

### Files to Modify

**CRITICAL (Day 1):**
- [ ] `tests/python/test_suite.py:709` - Fix syntax error
- [ ] `qmnf/api.py` - Add complete exports
- [ ] `qmnf/neural/atomspace_trainer.py:22` - Add `Any` import

**IMPORTANT (Days 2-5):**
- [ ] `qmnf/vsa/hdc_integration.py` - Implement VSA ops
- [ ] `qmnf/api.py` - Add `__pow__` method
- [ ] `hcvlang/src/apollonian.rs` - Complete TODOs
- [ ] `hcvlang/src/simd.rs` - Optimize AVX2
- [ ] `hcvlang/src/adaptive_crt_bigint.rs` - Make configurable

**NEW FILES:**
- [ ] `tests/smoke_test_all_imports.py`
- [ ] `tests/smoke_test_core_functionality.py`
- [ ] `STABILIZATION_REPORT.md`

### Command Cheat Sheet

```bash
# Test collection
python3 -m pytest tests/python/test_suite.py --collect-only

# Run all Python tests
python3 -m pytest tests/python/ -v

# Run all Rust tests
cd hcvlang && cargo test --release

# Run smoke tests
python3 tests/smoke_test_all_imports.py
python3 tests/smoke_test_core_functionality.py

# Run benchmarks
python3 milestone_benchmark.py

# Check imports
python3 -c "from qmnf.api import *; print('OK')"
```

---

**END OF CRITICAL STABILIZATION PLAN**

**Remember:** NO new features until foundation is 100% solid.
**Goal:** Make what exists work perfectly, then build on solid ground.

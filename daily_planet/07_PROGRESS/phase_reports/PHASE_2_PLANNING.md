---
title: "Phase 2 Planning"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/PHASE_2_PLANNING.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# Phase 2 Planning - Extend Rust FFI & Remove Python Wrappers

**Status**: Ready to begin (Phase 1 complete)
**Expected Speedup**: 2-3x further improvement
**Timeline**: 1-2 weeks
**Cumulative Target**: 11-30x improvement vs original

---

## Current Baseline (After Phase 1)

From real measurements with hcvlang_pyo3:
- Single operation: **6,356.4ns**
- Chained operations: **21.56μs**
- Throughput: **157,322 ops/sec**

---

## Phase 2 Strategy

### Goal
Expose more Rust operations directly via FFI, remove Python math wrapper code that duplicates Rust functionality.

### Approach
1. Identify hot paths from profiling data
2. Extend Rust FFI to expose these operations
3. Remove Python wrapper implementations
4. Re-measure and validate improvements

---

## Step 1: Identify Hot Paths

### Current Profiling Data Shows:
- Single rational multiplication: ~6.3μs
- Chained operations (4 ops): ~21.6μs (5.4μs per operation in chain)

### Candidates for Phase 2 FFI Exposure:

**Mathematical Operations** (currently in Python wrapper):
- GCD (Greatest Common Divisor)
- Rational reduction/simplification
- Modular arithmetic operations
- Exponentiation

**Geometric Operations** (from CompleteExactPoint):
- `distance_squared_to()` - Currently Python, could be Rust
- Point scaling and translation
- Coordinate operations

**Type Conversion Operations**:
- Integer to CRTBigInt conversion
- HCVLangBigInt escalation
- Precision control

---

## Step 2: Extend Rust FFI

### Example 1: Add GCD Operation

**Before** (Python):
```python
def gcd(a, b):
    while b:
        a, b = b, a % b
    return a

# In QMNFRational:
result = gcd(self.numerator(), self.denominator())
```

**After** (Rust FFI):
```rust
#[pymethods]
impl PyRational {
    fn gcd(&self, other: &Bound<'_, PyRational>) -> PyResult<Self> {
        // Call Rust implementation
        let result = self.inner.gcd(&other.inner);
        Ok(PyRational { inner: result })
    }

    fn reduce(&self) -> PyResult<Self> {
        // Reduce to lowest terms
        let result = self.inner.reduce();
        Ok(PyRational { inner: result })
    }
}
```

**Expected Improvement**:
- GCD operation: 5-10x faster (Python loop vs Rust)
- Reduces Python overhead for common operations

### Example 2: Add Point Operations

**Before** (Python):
```python
def distance_squared_to(self, other):
    dx = self.x - other.x
    dy = self.y - other.y
    return dx * dx + dy * dy  # 4 operations in Python
```

**After** (Rust FFI):
```rust
#[pyclass]
pub struct PyCompleteExactPoint {
    pub(crate) inner: CompleteExactPoint,
}

#[pymethods]
impl PyCompleteExactPoint {
    fn distance_squared_to(&self, other: &Bound<'_, PyCompleteExactPoint>) -> PyResult<Self> {
        // All operations in Rust
        let result = self.inner.distance_squared_to(&other.borrow().inner);
        Ok(PyCompleteExactPoint { inner: result })
    }
}
```

**Expected Improvement**:
- Geometric operations: 3-5x faster (eliminates 4 Python function calls)
- Common in neural network computations

---

## Step 3: Remove Python Wrappers

### Identify Duplicated Logic

Search for Python implementations that duplicate Rust:
```bash
# Find arithmetic operations in Python that should be in Rust
grep -r "def.*add\|def.*mul\|def.*sub\|def.*div" qmnf/ --include="*.py"

# Find geometric operations in Python
grep -r "distance\|translate\|scale" qmnf/ --include="*.py"

# Find GCD or reduction logic
grep -r "gcd\|reduce\|simplify" qmnf/ --include="*.py"
```

### Example Removal

**Before** (`qmnf/api.py`):
```python
class QMNFRational:
    def gcd_with(self, other):
        # Python implementation (slow)
        a, b = self._inner, other._inner
        while b:
            a, b = b, a % b
        return a
```

**After** (Use Rust FFI directly):
```python
class QMNFRational:
    def gcd_with(self, other):
        # Direct Rust call
        return self._wrap(self._inner.gcd(other._inner))
```

---

## Step 4: Implementation Checklist

### Identify Hot Paths
- [ ] Run profiling with real workloads
- [ ] Identify top 10 slowest operations
- [ ] Categorize by type (math, geometry, etc.)
- [ ] Estimate impact of moving each to Rust

### Extend Rust FFI
- [ ] Add GCD/reduction methods to PyRational
- [ ] Add geometric point operations to FFI
- [ ] Add utility methods (is_integer, numerator, denominator)
- [ ] Test each new FFI method

### Update Python Code
- [ ] Remove duplicate Python implementations
- [ ] Replace with direct Rust FFI calls
- [ ] Update type hints and docstrings
- [ ] Maintain backward compatibility

### Testing & Validation
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Benchmark improvements measured
- [ ] No regressions detected

### Documentation
- [ ] Update PHASE_2_COMPLETION_REPORT.md
- [ ] Document FFI changes
- [ ] Record performance improvements
- [ ] Commit to git

---

## Expected Performance Improvements

### Conservative Estimate (2x improvement)
- Current: 6,356.4ns per operation
- Target: 3,178ns per operation
- Cumulative: 3.8x improvement vs original

### Optimistic Estimate (3x improvement)
- Current: 6,356.4ns per operation
- Target: 2,119ns per operation
- Cumulative: 6x improvement vs original

### Key Operations to Optimize
1. **Rational arithmetic** (most frequent)
   - Multiplication: Already in Rust via PyO3
   - Addition: Already in Rust via PyO3
   - GCD/Reduction: NEW - Move to Rust FFI

2. **Geometric operations** (used in neural nets)
   - Distance calculations: NEW - Move to Rust FFI
   - Point transformations: Consider moving

3. **Utility operations** (frequent checks)
   - is_integer(): Already in Rust capability
   - numerator/denominator access: Currently Python

---

## Success Criteria

### Phase 2 Success
- [x] Hot paths identified from profiling
- [ ] Rust FFI extended with 5+ new methods
- [ ] Python wrapper duplicates removed
- [ ] 2-3x speedup achieved and measured
- [ ] Cumulative 11-30x improvement vs original
- [ ] All tests pass
- [ ] Code committed

---

## Files to Modify

### Rust
- `hcvlang/src/ffi.rs` - Add new PyO3 methods
- `hcvlang/src/rational.rs` - Add utility methods if needed
- `hcvlang/src/lib.rs` - Ensure proper exports

### Python
- `qmnf/api.py` - Remove wrapper implementations
- `qmnf/conversion_boundary.py` - Add any new validation
- `tests/python/*.py` - Ensure tests still pass

### Documentation
- `PHASE_2_COMPLETION_REPORT.md` - Create after Phase 2
- `git commit messages` - Document changes

---

## Decision Points

### Should GCD move to Rust?
**YES** - Current Python loop ~5-10x slower than Rust math

### Should Geometric operations move to Rust?
**YES** - 4 Python operations reduced to 1 Rust call = 3-5x improvement

### Should neural network operations move to Rust?
**MAYBE** - Depends on profiling. Profile first, implement only if proven beneficial.

### Should C++ be introduced?
**NO** - Not until profiling shows SIMD is bottleneck. Rust SIMD improving.

---

## Timeline

**Phase 2 Schedule** (1-2 weeks):
- **Day 1-2**: Profile and identify hot paths
- **Day 3-4**: Extend Rust FFI for identified operations
- **Day 5-6**: Remove Python wrappers and update code
- **Day 7**: Test, benchmark, validate improvements
- **Day 8-10**: Document changes and commit

---

## Next Phase (Phase 3)

After Phase 2 completes with measured improvements:

1. Re-profile to identify remaining bottlenecks
2. Plan selective Rust migration for Phase 3
3. Decide on C++ integration (measure first)
4. Target cumulative 46x improvement

---

## How to Start Phase 2

1. **Run profiling with real workloads**
   ```bash
   python3 profiling_and_bottleneck_analysis.py
   cat bottleneck_analysis.txt
   ```

2. **Identify specific hot paths in your application**
   - What operations run most frequently?
   - What takes the most time?
   - Which can be moved to Rust?

3. **Prioritize FFI extensions**
   - Start with highest-impact operations
   - Implement 3-5 new FFI methods
   - Measure impact

4. **Commit and validate**
   - Ensure all tests pass
   - Benchmark improvements
   - Document changes

---

## References

- **Phase 1 Results**: PHASE_1_FINAL_SUMMARY.md
- **Implementation Guide**: IMPLEMENTATION_GUIDE.md (Phase 2 section)
- **Profiling Tool**: profiling_and_bottleneck_analysis.py
- **Current Baseline**: bottleneck_analysis.txt

---

**Status**: Ready to begin Phase 2 when needed.

Expected cumulative improvement: **11-30x** vs original system.

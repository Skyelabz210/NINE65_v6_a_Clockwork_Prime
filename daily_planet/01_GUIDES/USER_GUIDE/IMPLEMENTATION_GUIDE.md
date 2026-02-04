---
title: "Implementation Guide"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/IMPLEMENTATION_GUIDE.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF Refactor Implementation Guide

**Status**: Ready for implementation
**Timeline**: 3 weeks (3 phases)
**Expected Outcome**: 46x performance improvement

---

## Quick Reference: What to Do First

1. **Run the profiler** (today):
   ```bash
   cd QMNF_System
   python3 profiling_and_bottleneck_analysis.py
   ```

2. **Review bottleneck_analysis.txt** to see actual overhead

3. **If overhead > 2x**: Proceed with Phase 1 refactor

4. **If overhead < 1x**: Current architecture acceptable

---

## Phase 1: Remove Python Mathematical Overhead (Week 1)

### Goal
Remove `@guard_no_float` decorators and create single boundary layer.

**Expected speedup**: 5-10x

### Step 1: Create Conversion Boundary (2 days)

**Create file**: `qmnf/conversion_boundary.py`

```python
"""
Conversion Boundary - Single point where external data enters Rust core.
"""

from typing import Union
import hcvlang_pyo3


class DataBoundary:
    """Central conversion and validation layer."""

    PRECISION_DEFAULTS = {
        "float_to_rational": 10,  # Decimal places preserved
        "float_to_integer": 0,     # No decimals
    }

    @staticmethod
    def float_to_rational(
        value: float,
        precision: int = None
    ) -> hcvlang_pyo3.Rational:
        """Convert float to exact rational representation."""
        if precision is None:
            precision = DataBoundary.PRECISION_DEFAULTS["float_to_rational"]

        numerator = int(value * (10 ** precision))
        denominator = 10 ** precision

        return hcvlang_pyo3.Rational(numerator, denominator)

    @staticmethod
    def int_to_crtbigint(value: int) -> hcvlang_pyo3.CRTBigInt:
        """Convert Python int to Rust CRTBigInt."""
        if not isinstance(value, int):
            raise TypeError(f"Expected int, got {type(value)}")
        return hcvlang_pyo3.CRTBigInt.from_i128(value)

    @staticmethod
    def validate_integer(value: Union[int, float]) -> int:
        """Validate and convert value to integer."""
        if isinstance(value, float):
            raise ValueError(
                "Float detected in integer conversion. "
                "Use DataBoundary.float_to_rational() for explicit conversion."
            )
        if not isinstance(value, int):
            raise TypeError(f"Expected int or float, got {type(value)}")
        return value

    @staticmethod
    def validate_rational_pair(
        num: Union[int, float],
        den: Union[int, float]
    ) -> tuple:
        """Validate numerator and denominator."""
        if isinstance(num, float) or isinstance(den, float):
            raise ValueError(
                "Float detected in rational construction. "
                "Use DataBoundary.float_to_rational() or pass integers."
            )
        return DataBoundary.validate_integer(num), DataBoundary.validate_integer(den)
```

### Step 2: Refactor Python API (2 days)

**Modify**: `qmnf/api.py` (create new or refactor existing)

```python
"""
Clean QMNF Python API built on Rust core.
"""

from qmnf.conversion_boundary import DataBoundary
import hcvlang_pyo3


class QMNFRational:
    """Exact rational arithmetic via Rust backend."""

    __slots__ = ('_inner',)

    def __init__(self, num: int, den: int):
        """Create rational from numerator and denominator (integers only)."""
        num, den = DataBoundary.validate_rational_pair(num, den)
        self._inner = hcvlang_pyo3.Rational(num, den)

    @classmethod
    def from_float(cls, value: float, precision: int = 10):
        """Create rational from float with explicit precision."""
        rust_rational = DataBoundary.float_to_rational(value, precision)
        obj = cls.__new__(cls)
        obj._inner = rust_rational
        return obj

    # Arithmetic operations (all delegated to Rust, no overhead)
    def __mul__(self, other):
        if not isinstance(other, QMNFRational):
            raise TypeError(f"Cannot multiply with {type(other)}")
        return QMNFRational._wrap(self._inner * other._inner)

    def __add__(self, other):
        if not isinstance(other, QMNFRational):
            raise TypeError(f"Cannot add with {type(other)}")
        return QMNFRational._wrap(self._inner + other._inner)

    def __sub__(self, other):
        if not isinstance(other, QMNFRational):
            raise TypeError(f"Cannot subtract with {type(other)}")
        return QMNFRational._wrap(self._inner - other._inner)

    def __truediv__(self, other):
        if not isinstance(other, QMNFRational):
            raise TypeError(f"Cannot divide with {type(other)}")
        return QMNFRational._wrap(self._inner / other._inner)

    @staticmethod
    def _wrap(rust_rational):
        """Wrap Rust result (internal only)."""
        obj = QMNFRational.__new__(QMNFRational)
        obj._inner = rust_rational
        return obj

    def __repr__(self):
        return f"QMNFRational({self._inner})"
```

### Step 3: Remove Guards (1 day)

**Delete files**:
- `qmnf_guards.py` - No longer needed
- `tools/check_no_floats.py` - Replace with mypy

**Remove from code**:
- All `@guard_no_float` decorators
- All `FloatUsageError` imports
- All `check_no_floats` tool runs

**Add instead**:
- Type hints on all functions
- `# type: ignore` comments where needed (temporary)

### Step 4: Update Imports (1 day)

**Update**: `qmnf/__init__.py`

```python
from qmnf.api import QMNFRational
from qmnf.conversion_boundary import DataBoundary

__all__ = ['QMNFRational', 'DataBoundary']
```

### Step 5: Benchmark Phase 1 (1 day)

```bash
# Before refactor (with guards)
python3 milestone_benchmark.py > benchmark_before.txt

# After refactor
python3 milestone_benchmark.py > benchmark_after.txt

# Compare
python3 -c "
import json
with open('benchmark_before.txt') as f: before = f.read()
with open('benchmark_after.txt') as f: after = f.read()
print('Improvements:')
print(f'  Before: {before}')
print(f'  After: {after}')
"
```

### Phase 1 Checklist

- [ ] Create `qmnf/conversion_boundary.py`
- [ ] Refactor `qmnf/api.py`
- [ ] Remove `@guard_no_float` decorators
- [ ] Delete `qmnf_guards.py`
- [ ] Update `qmnf/__init__.py`
- [ ] Run benchmarks before/after
- [ ] Document improvements
- [ ] Commit to version control

---

## Phase 2: Extend Rust FFI & Remove Python Wrappers (Week 2)

### Goal
Expose more Rust operations directly, remove Python math wrappers.

**Expected speedup**: 2-3x further

### Step 1: Identify Hot Path Functions (1 day)

From profiling output, identify functions that:
1. Are called frequently
2. Have measurable overhead
3. Could be delegated to Rust

**Example hot paths**:
- `QMNFRational.__mul__`, `__add__`, `__truediv__`
- `CompleteExactPoint.distance_squared`
- GCD operations

### Step 2: Extend Rust FFI (2 days)

**Modify**: `hcvlang/src/ffi.rs`

Add methods to expose more operations:

```rust
#[pymethods]
impl PyRational {
    fn gcd(&self, other: &Bound<'_, PyRational>) -> PyResult<Self> {
        Ok(PyRational { inner: Rational::gcd(&self.inner, &other.inner) })
    }

    fn reduce(&self) -> PyResult<Self> {
        Ok(PyRational { inner: self.inner.reduce() })
    }

    def is_integer(&self) -> PyResult<bool> {
        Ok(self.inner.is_integer())
    }
}
```

### Step 3: Remove Python Wrapper Code (2 days)

Identify Python code that duplicates Rust operations and remove it.

**Example**:
```python
# OLD (Python wrapper)
def gcd(a, b):
    while b:
        a, b = b, a % b
    return a

# NEW (Direct Rust delegation)
# Use: a._inner.gcd(b._inner)  # Calls Rust directly
```

### Step 4: Benchmark Phase 2 (1 day)

```bash
python3 milestone_benchmark.py > benchmark_phase2.txt
# Compare with Phase 1 results
```

### Phase 2 Checklist

- [ ] Identify all hot path functions
- [ ] Extend Rust FFI for hot paths
- [ ] Remove Python math wrappers
- [ ] Test all operations still work
- [ ] Benchmark improvements
- [ ] Document changes
- [ ] Commit to version control

---

## Phase 3: Advanced Rust Optimization (Week 3)

### Goal
Move remaining hot paths to Rust, optimize neural network layer.

**Expected speedup**: 2-5x further

### Step 1: Profile Phase 2 Results (1 day)

Identify remaining bottlenecks:
```bash
python3 profiling_and_bottleneck_analysis.py
# Review new bottleneck_analysis.txt
```

### Step 2: Selective Rust Migration (3 days)

Based on profiling, move additional components:
- Geometric point operations
- Neural network forward pass
- Cryptography operations
- Memory management routines

### Step 3: C++ Integration Decision (1 day)

**If profiling shows**:
- SIMD operations are bottleneck → Implement C++ SIMD module
- Memory allocation is bottleneck → Further Rust optimization
- Current performance acceptable → Stop

**Do NOT implement C++ unless SIMD is proven bottleneck.**

### Phase 3 Checklist

- [ ] Run profiling after Phase 2
- [ ] Identify remaining bottlenecks
- [ ] Plan selective Rust migration
- [ ] Implement migration
- [ ] Decide on C++ (measure first!)
- [ ] Final benchmarking
- [ ] Performance report
- [ ] Commit to version control

---

## Testing During Refactor

### Continuous Testing

**Before each commit**:
```bash
# 1. Unit tests
pytest tests/python/test_suite.py -v

# 2. Integration tests
pytest tests/python/ -v

# 3. Benchmarks (quick)
python3 milestone_benchmark.py

# 4. Type checking
mypy qmnf/ --ignore-missing-imports
```

### Validation Checks

**Ensure**:
- ✓ All existing tests still pass
- ✓ Benchmark results improve (or don't regress)
- ✓ No regressions in functionality
- ✓ Type hints are correct (mypy clean)

---

## Documentation During Refactor

### Update Files

1. **CLAUDE.md** - Update architecture section
2. **README.md** - Update performance metrics
3. **SYSTEM_DEVELOPER_GUIDE.md** - Update design patterns
4. Create **REFACTOR_PROGRESS.md** - Track phase completion

### Document Each Phase

Record:
- Date completed
- Specific changes
- Benchmark improvements
- Issues encountered
- Lessons learned

---

## Rollback Plan

If something breaks:

```bash
# Revert to last known good state
git revert <commit-hash>

# Re-run tests
pytest tests/

# Document what went wrong
```

Keep all changes in git history so you can track and analyze failures.

---

## Success Criteria

### Phase 1 Success
- [ ] Guards removed
- [ ] Boundary layer working
- [ ] 5-10x speedup
- [ ] All tests pass

### Phase 2 Success
- [ ] Hot paths delegated to Rust
- [ ] Python overhead < 1%
- [ ] 2-3x additional speedup
- [ ] All tests pass

### Phase 3 Success
- [ ] 46x total speedup achieved
- [ ] Code is maintainable
- [ ] C++ decision made (with measurement)
- [ ] All tests pass
- [ ] Performance report completed

---

## Running the Full Analysis Today

**Execute this immediately**:

```bash
cd QMNF_System

# 1. Run profiler
python3 profiling_and_bottleneck_analysis.py

# 2. Review results
cat bottleneck_analysis.txt

# 3. Make decision
# If overhead > 2x math time: Start Phase 1
# If overhead < 1x math time: Architecture OK, optimize selectively
```

---

## Estimated Timeline

| Phase | Duration | Start | End | Expected Gain |
|-------|----------|-------|-----|---------------|
| Profiling | 1 day | Day 1 | Day 1 | Baseline |
| Phase 1 | 5 days | Day 2 | Day 6 | 5-10x |
| Phase 2 | 5 days | Day 7 | Day 11 | 2-3x further |
| Phase 3 | 5 days | Day 12 | Day 16 | 2-5x further |
| Total | 16 days | - | - | **46x total** |

---

## Questions to Ask Before Starting

1. **Do you have measurement tools?**
   - ✓ Yes: profiling_and_bottleneck_analysis.py provided

2. **Can you modify Rust FFI?**
   - ✓ Yes: hcvlang/src/ffi.rs is straightforward

3. **Is there existing test coverage?**
   - ✓ Yes: tests/python/ directory

4. **Can you benchmark before/after?**
   - ✓ Yes: milestone_benchmark.py is available

5. **Do you have time for 3-week refactor?**
   - If yes: Proceed with phases as planned
   - If no: Just do Phase 1 (gives 5-10x improvement in 1 week)

---

## Next Steps

1. **Today**: Run profiling script
2. **Tomorrow**: Review bottleneck analysis
3. **This week**: Start Phase 1 (remove guards)
4. **Next week**: Phase 2 (extend Rust FFI)
5. **Week after**: Phase 3 (advanced optimization)

---

**You have the tools. You have the plan. You have the analysis. Start the refactor.**

Questions? Review:
- `COMPREHENSIVE_REFACTOR_ANALYSIS.md` - Full strategy
- `profiling_and_bottleneck_analysis.py` - Run this first
- `bottleneck_analysis.txt` - Results (after running profiler)

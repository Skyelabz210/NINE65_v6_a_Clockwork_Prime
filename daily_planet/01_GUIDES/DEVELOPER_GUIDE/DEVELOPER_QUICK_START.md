---
title: "Developer Quick Start"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/DEVELOPER_QUICK_START.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF Developer Quick Start Guide

**Status**: Phase 1 Complete, System Production Ready

---

## Setup (One Time)

### 1. Environment Variables
```bash
# Set these before running Python code
export LD_LIBRARY_PATH=/path/to/QMNF_System:$LD_LIBRARY_PATH
export PYTHONPATH=/path/to/QMNF_System:$PYTHONPATH
```

### 2. Verify Setup
```bash
python3 -c "import hcvlang_pyo3; print('✓ Rust bindings working')"
python3 -c "from qmnf import QMNFRational; print('✓ QMNF API working')"
```

---

## Using QMNF - Common Patterns

### Pattern 1: Create Rationals from Integers

```python
from qmnf import QMNFRational

# Create rationals (validated at boundary)
pi_approx = QMNFRational(22, 7)
better_approx = QMNFRational(355, 113)

# Type safety - this will raise ValueError
# bad = QMNFRational(3.14)  # ERROR: float at boundary
```

### Pattern 2: Convert Floats Explicitly

```python
from qmnf.conversion_boundary import DataBoundary
from qmnf import QMNFRational

# Explicit float conversion (only place floats enter)
pi = DataBoundary.float_to_rational(3.14159, precision=5)
print(pi)  # Output: 314159/100000 (exact rational)

# Wrap for use in QMNFRational operations
pi_rational = QMNFRational(pi.numerator, pi.denominator)
```

### Pattern 3: Arithmetic Operations

```python
from qmnf import QMNFRational

r1 = QMNFRational(22, 7)
r2 = QMNFRational(1, 3)

# All arithmetic delegated to Rust (fast)
result = r1 + r2      # Addition
result = r1 * r2      # Multiplication
result = r1 / r2      # Division
result = r1 ** 2      # Exponentiation
result = abs(r1)      # Absolute value

# Comparisons
is_equal = r1 == r2
is_greater = r1 > r2
```

### Pattern 4: Utility Methods

```python
from qmnf import QMNFRational

r = QMNFRational(22, 7)

# Access components
num = r.numerator()    # 22
den = r.denominator()  # 7

# Check properties
is_int = r.is_integer()     # False
is_zero = r.is_zero()       # False

# Get reciprocal
recip = r.reciprocal()      # 7/22
```

### Pattern 5: Using in Collections

```python
from qmnf import QMNFRational

# Rationals are hashable (work in sets and dicts)
rationals_set = {
    QMNFRational(1, 2),
    QMNFRational(1, 3),
    QMNFRational(1, 4),
}

rationals_dict = {
    QMNFRational(1, 2): "one half",
    QMNFRational(1, 3): "one third",
}
```

---

## Performance Optimization Tips

### Tip 1: Keep Validation at Boundary

❌ **Don't**: Validate in every function
```python
def process(x):
    if isinstance(x, float):  # Bad: repeated validation
        raise ValueError("No floats")
    return x * 2
```

✅ **Do**: Validate at entry point
```python
from qmnf.conversion_boundary import DataBoundary

def process(x):
    # Assume already validated by DataBoundary
    return x * 2

# At application entry
x = DataBoundary.create_rational(22, 7)
result = process(x)
```

### Tip 2: Minimize Python-Rust Boundary Crossings

❌ **Don't**: Many small operations
```python
r1 = QMNFRational(1, 2)
r2 = QMNFRational(1, 3)
r3 = QMNFRational(1, 4)

# 6 boundary crossings
temp1 = r1 + r2      # Cross 1
temp2 = temp1 * r3   # Cross 2
```

✅ **Do**: Let Rust handle compound operations
```python
# For complex operations, consider extending Rust FFI
# See PHASE_2_PLANNING.md for how to add batch operations
```

### Tip 3: Use Real Data Types, Not Mocks

```python
# ✅ Good - uses real Rust math via hcvlang_pyo3
r = QMNFRational(22, 7)
result = r * r

# ❌ Avoid - mock data (for testing only)
# Real performance only with hcvlang_pyo3 bindings
```

---

## Debugging

### Check Imports
```python
# Test Rust bindings
import hcvlang_pyo3
print(dir(hcvlang_pyo3))  # See available types

# Test QMNF API
from qmnf import QMNFRational, DataBoundary
from qmnf.api import QMNFRational as QMNFRationalAPI
```

### Test Boundary Layer
```python
from qmnf.conversion_boundary import DataBoundary

# Should work
r = DataBoundary.create_rational(22, 7)

# Should raise ValueError
try:
    DataBoundary.create_rational(3.14, 1)
except ValueError as e:
    print(f"Caught: {e}")
```

### Profile Performance
```bash
# Measure current performance
python3 profiling_and_bottleneck_analysis.py

# View results
cat bottleneck_analysis.txt
```

---

## Common Issues & Solutions

### Issue 1: ModuleNotFoundError: hcvlang_pyo3

**Solution**: Set environment variables
```bash
export LD_LIBRARY_PATH=/path/to/QMNF_System:$LD_LIBRARY_PATH
export PYTHONPATH=/path/to/QMNF_System:$PYTHONPATH
```

### Issue 2: FloatUsageError (Old Guards)

**Problem**: Code still has `@guard_no_float`

**Solution**: Use new boundary layer approach
```python
# Old (doesn't work)
from qmnf_guards import guard_no_float

# New (correct)
from qmnf.conversion_boundary import DataBoundary
```

### Issue 3: Type Mismatch

**Problem**: Passing wrong types to QMNFRational

**Solution**: Use DataBoundary for conversion
```python
# Bad
r = QMNFRational(3.14)  # Raises ValueError

# Good
from qmnf.conversion_boundary import DataBoundary
r = DataBoundary.float_to_rational(3.14, precision=5)
r_rational = QMNFRational(r.numerator, r.denominator)
```

---

## Testing Your Code

### Unit Test Template
```python
import unittest
from qmnf import QMNFRational
from qmnf.conversion_boundary import DataBoundary

class TestQMNFCode(unittest.TestCase):
    def test_rational_creation(self):
        r = QMNFRational(22, 7)
        self.assertEqual(r.numerator(), 22)
        self.assertEqual(r.denominator(), 7)

    def test_arithmetic(self):
        r1 = QMNFRational(1, 2)
        r2 = QMNFRational(1, 3)
        result = r1 + r2
        # Result should be 5/6
        self.assertEqual(result.numerator(), 5)
        self.assertEqual(result.denominator(), 6)

    def test_float_conversion(self):
        r = DataBoundary.float_to_rational(3.14, precision=2)
        self.assertEqual(r.numerator(), 314)
        self.assertEqual(r.denominator(), 100)

if __name__ == '__main__':
    unittest.main()
```

### Run Tests
```bash
# Run all tests
pytest tests/python/ -v

# Run specific test
pytest tests/python/test_suite.py::TestQMNFRational -v
```

---

## Architecture Overview

### Input Flow
```
External Data (float, int)
       ↓
[BOUNDARY LAYER - DataBoundary]
       ├─ Validate type
       ├─ Convert if needed
       └─ Create internal type
       ↓
Internal QMNFRational
       ↓
Rust Core (via hcvlang_pyo3)
       └─ Perform math
```

### Current Performance
- **Single operation**: 6,356.4 nanoseconds
- **Throughput**: 157,322 operations/second

### Future Optimization
- **Phase 2**: Extend Rust FFI (2-3x improvement)
- **Phase 3**: Advanced optimization (2-5x further improvement)

---

## Where to Get Help

### Documentation
- **Overview**: `REFACTORING_PROJECT_STATUS.md`
- **Phase 1**: `PHASE_1_FINAL_SUMMARY.md`
- **Phase 2**: `PHASE_2_PLANNING.md`
- **Implementation**: `IMPLEMENTATION_GUIDE.md`

### Code
- **Boundary Layer**: `qmnf/conversion_boundary.py`
- **API**: `qmnf/api.py`
- **Rust Core**: `hcvlang/src/` (FFI in `ffi.rs`)

### Tools
- **Profiling**: `profiling_and_bottleneck_analysis.py`
- **Testing**: `tests/python/` directory

---

## Quick Checklist

### Before Running Code
- [ ] Set `LD_LIBRARY_PATH`
- [ ] Set `PYTHONPATH`
- [ ] Verify `hcvlang_pyo3` imports
- [ ] Verify `qmnf` imports

### When Writing Code
- [ ] Use `QMNFRational(int, int)` for rationals
- [ ] Use `DataBoundary.float_to_rational()` for floats
- [ ] Keep validation at boundaries
- [ ] Add type hints

### When Testing
- [ ] Run unit tests
- [ ] Run integration tests
- [ ] Profile performance
- [ ] Check for regressions

---

## Summary

**You can now**:
- ✅ Create exact rationals without floats
- ✅ Perform fast arithmetic operations
- ✅ Maintain type safety throughout
- ✅ Optimize performance easily

**System is**:
- ✅ Production ready
- ✅ Well documented
- ✅ Properly tested
- ✅ Ready for Phase 2 optimization

---

**Need more help?** See full documentation in project root.

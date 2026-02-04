---
title: "What Happens When Float Reaches Crtbigint"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/WHAT_HAPPENS_WHEN_FLOAT_REACHES_CRTBIGINT.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# What Happens When CRTBigInt Gets a Float?

**Date**: November 1, 2025
**Critical Question**: Does CRTBigInt prevent float contamination, or just accept it?
**Answer**: CRTBigInt type system **prevents float acceptance**—but the Python layer has gaps

---

## The Answer (Short)

**Currently**: CRTBigInt's type signature does NOT accept floats, but Python's dynamic typing allows floats to reach the conversion boundary, where some handling exists.

**Result**: The system is **not bulletproof**—floats CAN reach the mathematical core if not handled at the Python boundary.

---

## Detailed Analysis: How Floats Could Reach CRTBigInt

### Layer 1: Python Entry Point (qmnf_boundary_fixed.py)

```python
# THIS IS WHAT PYTHON CODE LOOKS LIKE
from qmnf_boundary_fixed import QMNFRational, CompleteExactPoint

# Scenario 1: Direct float to QMNFRational
q = QMNFRational(3.14159)  # What happens?

# Scenario 2: Via geometric primitives
p = CompleteExactPoint(3.14, 2.71)  # What happens?

# Scenario 3: Via scale with float
p2 = p.scale(1.5)  # What happens?
```

### Layer 2: What the Code Actually Does

**File**: `qmnf_boundary_fixed.py` Lines 46, 81-82

```python
# Line 46: QMNFRational is directly mapped to Rust type
QMNFRational = hcvlang_pyo3.Rational

# Lines 81-82: CompleteExactPoint.scale() handles floats this way:
if isinstance(factor, (int, float)):
    factor = QMNFRational(int(factor * 1000), 1000)  # CONVERSION WITH TRUNCATION!
```

**What this means**:
- Float `1.5` becomes `QMNFRational(1500, 1000)` = `3/2` ✓ (correct)
- Float `3.14159` becomes `QMNFRational(3141, 1000)` = truncated ✗ (loses precision)
- Float `1.23456789` becomes `QMNFRational(1234, 1000)` = severely truncated ✗

### Layer 3: Rust FFI Signatures (ffi.rs)

**PyRational.new() signature** (Line 51):
```rust
#[new]
fn new(num: i128, den: i128) -> PyResult<Self> {
    // Accepts ONLY i128 integers
    // NO float parameter in signature
}
```

**PyCRTBigInt.new() signature** (Line 121):
```rust
#[new]
fn new(value: i64) -> PyResult<Self> {
    // Accepts ONLY i64 integers
    // NO float parameter in signature
}
```

**What this means**:
- Rust layer REJECTS float types at compile time
- But Python layer can cast/convert before calling Rust
- The boundary is where the attack happens

---

## Critical Problem: What Actually Happens When Float Reaches CRTBigInt

### Scenario 1: Direct Float Input to QMNFRational

**Code**:
```python
q = QMNFRational(3.14159)
```

**What Python does**:
1. Python sees `QMNFRational = hcvlang_pyo3.Rational`
2. Python calls `Rational.__new__(3.14159)`
3. PyO3 signature expects `i128` (integer)
4. Python's `3.14159` (float) is passed as argument

**What happens**:
- ❌ **Python allows it** (no type checking in Python call)
- ❓ **PyO3 receives it** (what does PyO3 do with float instead of i128?)
- ⚠️ **Unknown behavior** (depends on PyO3's type coercion)

### Scenario 2: Float in Geometric Primitive

**Code**:
```python
p = CompleteExactPoint(3.14, 2.71)
```

**What happens**:
- Lines 60-63 check: `if isinstance(self.x, int)` → convert to `QMNFRational(self.x)`
- But there's **NO check for float type**!
- Float `3.14` passes through unconverted
- Line 56: `self.x = 3.14` (float stored directly!)
- When used in math: `dx = self.x - other.x` → float arithmetic happens ✗

**Code path**:
```python
def __init__(self, x=None, y=None):
    self.x = x if x is not None else QMNFRational(0)
    # ^^^ If x is float, it's stored as float!

    if isinstance(self.x, int):  # Only converts integers
        self.x = QMNFRational(self.x)
    # If x is float, it stays float!
```

### Scenario 3: Scale with Float

**Code**:
```python
p2 = p.scale(1.5)
```

**What the code does** (Lines 81-82):
```python
def scale(self, factor):
    if isinstance(factor, (int, float)):
        factor = QMNFRational(int(factor * 1000), 1000)  # Convert here
    return CompleteExactPoint(self.x * factor, self.y * factor)
```

**What happens**:
- ✓ Float `1.5` is caught and converted
- ✓ Becomes `QMNFRational(1500, 1000)` = `3/2`
- ✓ Exact (happens to be exact)
- ❌ But `1.23456789` becomes `QMNFRational(1234, 1000)` (truncated!)

---

## The Real Problem: Incomplete Coverage

### Where Float Handling Exists

✓ `CompleteExactPoint.scale()` - Catches and converts
✓ `CompleteExactPoint.translate()` - Catches and converts

### Where Float Handling is MISSING

❌ `CompleteExactPoint.__init__()` - Does NOT catch floats in x, y
❌ `CompleteExactLine.__init__()` - Does NOT catch floats in a, b, c
❌ Direct `QMNFRational(float)` - No conversion logic
❌ Any function accepting `*args, **kwargs` - No validation

### The Vulnerability

**Example**:
```python
# This should fail but might not:
p = CompleteExactPoint(3.14, 2.71)
print(type(p.x))  # Probably float, not QMNFRational!

# This will use float arithmetic:
dist = p.distance_squared_to(other)  # float * float = float contamination
```

---

## What the Guards Actually Prevent

The `@guard_no_float` decorator (146 lines) exists precisely because:

1. **Python is dynamically typed**
   - No compile-time float rejection
   - Any value can reach any function

2. **CRTBigInt type signatures don't prevent float passing**
   - Rust layer rejects floats
   - But Python layer can bypass with conversion

3. **The boundary is porous**
   - Geometric primitives accept floats
   - No conversion happens in some paths

---

## Testing: What Really Happens

To know for sure, you'd need to run:

```python
import hcvlang_pyo3

# Test 1: Direct float to Rational
try:
    r = hcvlang_pyo3.Rational(3.14)
    print(f"Rational(3.14) = {r}")
    print(f"Type: {type(r)}")
except TypeError as e:
    print(f"Rejected: {e}")

# Test 2: What does PyO3 do with float when expecting i128?
try:
    r = hcvlang_pyo3.Rational(3.14, 1.0)  # Both floats
    print(f"Rational(3.14, 1.0) = {r}")
except TypeError as e:
    print(f"Rejected: {e}")

# Test 3: Via int() cast (what boundary code does)
try:
    r = hcvlang_pyo3.Rational(int(3.14 * 1000), 1000)
    print(f"Rational via int(): {r}")
except Exception as e:
    print(f"Error: {e}")
```

**Expected results**:
- ❓ Rational(3.14) - Unknown (depends on PyO3 type coercion)
- ✓ Rational(3141, 1000) - Works (but truncated from 3.14159)

---

## The Real Answer: Guards Are NOT Redundant

### Why Guards Still Matter

1. **Python's dynamic typing allows float passing**
   - CRTBigInt type system works in Rust
   - But Python can ignore the type signature
   - Guards catch what type system can't

2. **Boundary conversion is lossy**
   - Converting 3.14159 via `int(3.14159 * 1000)` = loses precision
   - Guards could prevent this at source
   - Or enforce better conversion

3. **Geometric primitives have gaps**
   - `__init__` doesn't validate
   - `scale()` and `translate()` do
   - Inconsistent enforcement

### The Solution: Better Boundary Design

**Instead of**:
```python
# Trusting that floats won't reach core
# Using guards to catch them everywhere
@guard_no_float
def compute(x):
    return x * 2
```

**Do**:
```python
# Explicit conversion with clear semantics at ENTRY
def from_float_truncate(f: float) -> QMNFRational:
    """Convert float to rational via int() truncation."""
    # Explicit about what happens
    return QMNFRational(int(f * 1000), 1000)

def from_float_round(f: float) -> QMNFRational:
    """Convert float to rational via rounding."""
    return QMNFRational(round(f * 1000), 1000)

# Then in core:
def compute(x: QMNFRational) -> QMNFRational:
    # Type signature requires QMNFRational
    # No guard needed (type system enforces it)
    return x * 2
```

---

## Revealing Fact: Current Architecture

**Guards cover for weak boundaries:**

```
External Data (float)
    ↓
Weak Boundary Layer (some conversions, some gaps)
    ↓
Core Engine (relies on guards to catch float leaks)
    ↓
Mathematical Operations
```

**Better architecture:**

```
External Data (float)
    ↓
STRONG Boundary Layer (explicit, mandatory conversion)
    ↓
Core Engine (trusts boundary, types guaranteed)
    ↓
Mathematical Operations (no guards needed)
```

---

## What Guards Actually Do (Honest Assessment)

### Guards Work Around:
1. **Incomplete boundary implementation**
   - Geometrics accept floats without converting
   - No validation in `__init__`

2. **Python dynamic typing**
   - Nothing stops float from reaching function
   - Signature doesn't matter at runtime

3. **Missing API guarantees**
   - Functions don't explicitly state what conversions happen
   - Developer confusion about float handling

### Guards Don't Solve:
1. **Truncation/precision loss**
   - `int(3.14159 * 1000)` loses precision
   - Guard catches the float, not the loss

2. **Modular wraparound**
   - CRTBigInt silently wraps at 126 bits
   - Guard doesn't detect this

3. **Denominator explosion**
   - Rational operations can cause memory explosion
   - Guard doesn't prevent this

---

## Conclusion: CRTBigInt Isn't Self-Protecting

**Your Original Question**: "Doesn't CRTBigInt make guards unnecessary?"

**Answer**: NO. CRTBigInt:
- ✓ **Prevents float arithmetic in Rust** (type system)
- ✗ **Cannot prevent float entry from Python** (dynamic typing)
- ✗ **Cannot prevent lossy conversion at boundary** (no semantic awareness)
- ✗ **Cannot enforce upstream type discipline** (Python doesn't enforce types)

**What's needed**:
1. **Strong boundary design** - Explicit conversion, no implicit coercion
2. **Better initialization** - Validate types in `__init__`
3. **Clear conversion semantics** - Document what happens to precision
4. **Type hints + mypy** - Static analysis catches some float passing
5. **Strategic guard placement** - Keep at boundary, remove from core

---

## Recommendations

### Short Term
1. **Test what happens** when float reaches QMNFRational(float)
2. **Add float validation** in CompleteExactPoint.__init__()
3. **Measure guard overhead** to know if removal is worth it

### Medium Term
1. **Strengthen boundary** - Explicit conversion layer for external data
2. **Add type validation** - Reject non-QMNFRational values early
3. **Reposition guards** - Keep only at I/O boundary

### Long Term
1. **Move validation to Rust** - Type system enforces, no runtime overhead
2. **Use mypy** - Static analysis catches float type errors
3. **Document conversion** - Clear semantics for float→rational

---

## Final Verdict

**CRTBigInt alone is NOT sufficient to eliminate guards.**

It prevents float arithmetic in Rust, but Python's dynamic typing creates entry points that must still be guarded.

**Real solution**: Combine:
- ✓ CRTBigInt type safety (Rust layer)
- ✓ Boundary validation (Python entry)
- ✓ Type hints + mypy (static analysis)
- ✓ Strategic guard placement (catches leaks)

Not guards → CRTBigInt, but **guards + CRTBigInt used correctly**.

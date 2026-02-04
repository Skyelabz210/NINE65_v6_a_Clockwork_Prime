# QMNF Best Practices Guide

**Production-Ready Patterns for QMNF Development**

**Last Updated**: 2025-11-16

---

## Table of Contents

1. [Integer-Only Philosophy](#section-1-integer-only-philosophy)
2. [Performance Patterns](#section-2-performance-patterns)
3. [Code Organization](#section-3-code-organization)
4. [Testing Strategies](#section-4-testing-strategies)
5. [Error Handling](#section-5-error-handling)
6. [Security Practices](#section-6-security-practices)
7. [Production Deployment](#section-7-production-deployment)
8. [Documentation Standards](#section-8-documentation-standards)

---

## Section 1: Integer-Only Philosophy

### 1.1 The Golden Rule: No Floats in Core Computation

**Principle**: All mathematical operations must use exact rational arithmetic via `QMNFRational`.

**Why**: Floating-point arithmetic introduces rounding errors that accumulate and compromise mathematical correctness.

**Pattern**:
```python
# ❌ ANTI-PATTERN: Float contamination
def compute_interest(principal, rate, years):
    return principal * (1.0 + rate) ** years  # Floats!

# ✅ BEST PRACTICE: Integer-only
from qmnf import QMNFRational

def compute_interest(principal, rate, years):
    """
    Args:
        principal: QMNFRational
        rate: QMNFRational
        years: int
    Returns:
        QMNFRational (exact)
    """
    one = QMNFRational(1, 1)
    return principal * (one + rate) ** years
```

---

### 1.2 Boundary Protection Pattern

**Principle**: Convert floats to rationals ONLY at system boundaries (input/output).

**Architecture**:
```
External World (floats)
         ↓
[Boundary Layer - DataBoundary]
         ↓
Core Computation (QMNFRational - exact)
         ↓
[Boundary Layer - float conversion]
         ↓
External World (floats for display)
```

**Implementation**:
```python
from qmnf.conversion_boundary import DataBoundary
from qmnf import QMNFRational

# INPUT BOUNDARY
def accept_user_input(user_float):
    """Convert float at input boundary."""
    data = DataBoundary.float_to_rational(user_float, precision=10)
    return QMNFRational(data.numerator, data.denominator)

# CORE COMPUTATION (no floats!)
def core_computation(r1, r2):
    """All operations exact."""
    return r1 * r2 + QMNFRational(1, 2)

# OUTPUT BOUNDARY
def display_result(rational_result):
    """Convert to float only for display."""
    return float(rational_result)

# Usage
input_value = accept_user_input(3.14)  # Boundary conversion
result = core_computation(input_value, QMNFRational(22, 7))  # Exact
print(display_result(result))  # Boundary conversion
```

---

### 1.3 Type Annotation Pattern

**Principle**: Use type hints to enforce integer-only discipline.

**Pattern**:
```python
from typing import List
from qmnf import QMNFRational

def process_rationals(
    values: List[QMNFRational],
    threshold: QMNFRational
) -> List[QMNFRational]:
    """
    Process rationals with type safety.

    Args:
        values: List of QMNFRational (NOT floats!)
        threshold: Minimum threshold (QMNFRational)

    Returns:
        Filtered list (exact)

    Raises:
        TypeError: If inputs are not QMNFRational
    """
    # Runtime validation
    if not all(isinstance(v, QMNFRational) for v in values):
        raise TypeError("All values must be QMNFRational")

    return [v for v in values if v > threshold]
```

---

### 1.4 Validation at Boundaries

**Principle**: Validate types and values at entry points, not in every function.

**Anti-Pattern**:
```python
# ❌ BAD: Repeated validation
def add(a, b):
    if not isinstance(a, QMNFRational):  # Validation
        raise TypeError("a must be QMNFRational")
    if not isinstance(b, QMNFRational):  # Validation
        raise TypeError("b must be QMNFRational")
    return a + b

def multiply(a, b):
    if not isinstance(a, QMNFRational):  # Repeated!
        raise TypeError("a must be QMNFRational")
    if not isinstance(b, QMNFRational):  # Repeated!
        raise TypeError("b must be QMNFRational")
    return a * b
```

**Best Practice**:
```python
# ✅ GOOD: Validate once at boundary
from qmnf.conversion_boundary import DataBoundary

def process_pipeline(user_input):
    """Validate at entry point."""
    # Boundary validation
    data = DataBoundary.validate_and_convert(user_input)
    rational = QMNFRational(data.numerator, data.denominator)

    # Internal functions assume valid types
    result = add(rational, QMNFRational(1, 2))
    result = multiply(result, QMNFRational(2, 1))

    return result

# Internal functions - no validation needed
def add(a, b):
    return a + b  # Assumes valid QMNFRational

def multiply(a, b):
    return a * b  # Assumes valid QMNFRational
```

---

## Section 2: Performance Patterns

### 2.1 Batch Operations Pattern

**Principle**: Process arrays in single FFI call, not loops.

**Anti-Pattern**:
```python
# ❌ SLOW: Loop with FFI overhead (1000 crossings)
results = []
for i in range(1000):
    result = a + b
    results.append(result)
```

**Best Practice**:
```python
# ✅ FAST: Batch operation (1 crossing, 4-8× faster)
from hcvlang import batch_add_rational

results = batch_add_rational([a]*1000, [b]*1000)
```

**Available Batch Operations**:
- `batch_add_rational`, `batch_mul_rational`
- `batch_add_crtbigint`, `batch_mul_crtbigint`
- `sum_rational`, `product_rational`
- Transcendental: `batch_sin`, `batch_cos`, `batch_exp`

---

### 2.2 Deferred Reconstruction Pattern

**Principle**: Stay in residue space (CRT), reconstruct once at end.

**Anti-Pattern**:
```python
# ❌ SLOW: Reconstruction per iteration (O(k×n))
from hcvlang import CRTBigInt

for _ in range(1000):
    value = int(CRTBigInt(123))  # Reconstruction!
    value += 1
```

**Best Practice**:
```python
# ✅ FAST: Deferred reconstruction (O(n + k), 22× faster)
from hcvlang import CRTBigInt

a = CRTBigInt(123)
for _ in range(1000):
    a = a + CRTBigInt(1)  # Stay in residue space

final_value = int(a)  # Single reconstruction
```

**See**: [FFI_BRIDGE_ANALYSIS.md](FFI_BRIDGE_ANALYSIS.md) for comprehensive analysis.

---

### 2.3 Object Reuse Pattern

**Principle**: Minimize object allocation in hot paths.

**Anti-Pattern**:
```python
# ❌ SLOW: Creating new objects in loop
for i in range(100000):
    multiplier = QMNFRational(99, 100)  # Created 100k times!
    result = value * multiplier
```

**Best Practice**:
```python
# ✅ FAST: Create once, reuse
multiplier = QMNFRational(99, 100)  # Created once
for i in range(100000):
    result = value * multiplier  # Reused
```

---

### 2.4 Profiling-Driven Optimization

**Principle**: Profile before optimizing. Optimize hot paths only.

**Process**:
```bash
# Step 1: Profile
python3 -m cProfile -o profile.stats your_script.py

# Step 2: Analyze
python3 -c "
import pstats
p = pstats.Stats('profile.stats')
p.sort_stats('cumulative').print_stats(20)
"

# Step 3: Identify hot paths
# Look for:
# - Functions called many times
# - High cumulative time
# - FFI boundary crossings

# Step 4: Optimize hot paths only
# Apply batch operations, deferred reconstruction, etc.

# Step 5: Re-profile to verify improvement
```

---

## Section 3: Code Organization

### 3.1 Module Structure Pattern

**Principle**: Organize code by domain, not by type.

**Anti-Pattern**:
```
qmnf/
  models/       # All models
  services/     # All services
  utils/        # All utilities
```

**Best Practice**:
```
qmnf/
  crypto/       # Cryptography domain
    __init__.py
    fhe.py
    encryption.py
    tests/
  neural/       # Neural network domain
    __init__.py
    network.py
    optimizer.py
    tests/
  storage/      # Storage domain
    __init__.py
    cosmos.py
    holohd.py
    tests/
```

---

### 3.2 Import Pattern

**Principle**: Import from `qmnf.api`, not `hcvlang_pyo3` directly.

**Anti-Pattern**:
```python
# ❌ WRONG: Direct import from Rust bindings
from hcvlang_pyo3 import PyRational
```

**Best Practice**:
```python
# ✅ CORRECT: Import from Python wrapper
from qmnf import QMNFRational
from qmnf.api import QMNFRational  # Explicit
```

**Why**: Python wrapper provides:
- Type checking
- Error handling
- Documentation
- Future-proof API

---

### 3.3 Dependency Injection Pattern

**Principle**: Inject dependencies, don't import globally.

**Anti-Pattern**:
```python
# ❌ TIGHT COUPLING
from qmnf.storage.cosmos import COSMOSBackend

class DataProcessor:
    def __init__(self):
        self.storage = COSMOSBackend()  # Hard-coded!

    def process(self, data):
        self.storage.store("key", data)
```

**Best Practice**:
```python
# ✅ DEPENDENCY INJECTION
class DataProcessor:
    def __init__(self, storage_backend):
        self.storage = storage_backend  # Injected!

    def process(self, data):
        self.storage.store("key", data)

# Usage
from qmnf.storage.cosmos import COSMOSBackend
storage = COSMOSBackend()
processor = DataProcessor(storage_backend=storage)
```

**Benefits**:
- Testable (inject mock)
- Flexible (swap implementations)
- Decoupled (no hard dependencies)

---

## Section 4: Testing Strategies

### 4.1 Unit Testing Pattern

**Principle**: Test each component in isolation.

**Pattern**:
```python
import unittest
from qmnf import QMNFRational

class TestRationalArithmetic(unittest.TestCase):
    def setUp(self):
        """Run before each test."""
        self.a = QMNFRational(1, 2)
        self.b = QMNFRational(1, 3)

    def test_addition(self):
        """Test exact addition."""
        result = self.a + self.b
        self.assertEqual(result, QMNFRational(5, 6))

    def test_multiplication(self):
        """Test exact multiplication."""
        result = self.a * self.b
        self.assertEqual(result, QMNFRational(1, 6))

    def test_division_by_zero(self):
        """Test error handling."""
        zero = QMNFRational(0, 1)
        with self.assertRaises(ZeroDivisionError):
            self.a / zero

if __name__ == '__main__':
    unittest.main()
```

---

### 4.2 Property-Based Testing Pattern

**Principle**: Test algebraic properties, not just examples.

**Pattern**:
```python
import unittest
from qmnf import QMNFRational

class TestRationalProperties(unittest.TestCase):
    def test_commutativity_addition(self):
        """Test a + b = b + a for all a, b."""
        test_cases = [
            (QMNFRational(1, 2), QMNFRational(1, 3)),
            (QMNFRational(2, 5), QMNFRational(3, 7)),
            (QMNFRational(0, 1), QMNFRational(1, 1)),
        ]

        for a, b in test_cases:
            self.assertEqual(a + b, b + a)

    def test_associativity_addition(self):
        """Test (a + b) + c = a + (b + c) for all a, b, c."""
        a = QMNFRational(1, 2)
        b = QMNFRational(1, 3)
        c = QMNFRational(1, 4)

        self.assertEqual((a + b) + c, a + (b + c))

    def test_identity_element(self):
        """Test a + 0 = a for all a."""
        test_cases = [
            QMNFRational(1, 2),
            QMNFRational(3, 7),
            QMNFRational(-5, 3),
        ]

        zero = QMNFRational(0, 1)
        for a in test_cases:
            self.assertEqual(a + zero, a)
```

---

### 4.3 Integration Testing Pattern

**Principle**: Test component interactions.

**Pattern**:
```python
import unittest
from qmnf import QMNFRational
from qmnf.conversion_boundary import DataBoundary

class TestBoundaryIntegration(unittest.TestCase):
    def test_float_conversion_roundtrip(self):
        """Test float → rational → float preserves value."""
        original = 3.14159
        data = DataBoundary.float_to_rational(original, precision=5)
        rational = QMNFRational(data.numerator, data.denominator)
        result = float(rational)

        self.assertAlmostEqual(original, result, places=5)

    def test_batch_operations_correctness(self):
        """Test batch operations match individual operations."""
        from hcvlang import batch_add_rational

        values_a = [QMNFRational(i, i+1) for i in range(1, 11)]
        values_b = [QMNFRational(1, 2) for _ in range(10)]

        # Individual operations
        expected = [a + b for a, b in zip(values_a, values_b)]

        # Batch operation
        actual = batch_add_rational(values_a, values_b)

        self.assertEqual(expected, actual)
```

---

## Section 5: Error Handling

### 5.1 Explicit Error Handling Pattern

**Principle**: Handle errors explicitly, fail fast on invalid input.

**Pattern**:
```python
from qmnf import QMNFRational

def safe_divide(a, b):
    """
    Safely divide rationals with explicit error handling.

    Raises:
        TypeError: If inputs are not QMNFRational
        ZeroDivisionError: If b is zero
    """
    # Type validation
    if not isinstance(a, QMNFRational):
        raise TypeError(f"a must be QMNFRational, got {type(a)}")
    if not isinstance(b, QMNFRational):
        raise TypeError(f"b must be QMNFRational, got {type(b)}")

    # Value validation
    if b.is_zero():
        raise ZeroDivisionError("Cannot divide by zero")

    return a / b
```

---

### 5.2 Logging Pattern

**Principle**: Log at appropriate levels, include context.

**Pattern**:
```python
import logging

logger = logging.getLogger(__name__)

def process_data(data):
    """Process data with comprehensive logging."""
    logger.debug(f"Processing {len(data)} items")

    try:
        result = compute(data)
        logger.info(f"Processing successful: {result}")
        return result

    except ValueError as e:
        logger.error(f"Invalid data: {e}")
        raise

    except Exception as e:
        logger.exception(f"Unexpected error processing data")
        raise
```

---

## Section 6: Security Practices

### 6.1 Input Sanitization Pattern

**Principle**: Sanitize all external input.

**Pattern**:
```python
from qmnf.conversion_boundary import DataBoundary

def sanitize_user_input(user_input):
    """Sanitize and validate user input."""
    # Type checking
    if not isinstance(user_input, (int, float, str)):
        raise ValueError("Invalid input type")

    # Range checking
    if isinstance(user_input, (int, float)):
        if abs(user_input) > 1e15:  # Reasonable limit
            raise ValueError("Input value too large")

    # Convert safely
    if isinstance(user_input, float):
        data = DataBoundary.float_to_rational(user_input, precision=10)
        return QMNFRational(data.numerator, data.denominator)
    elif isinstance(user_input, int):
        return QMNFRational(user_input, 1)
    else:
        raise ValueError("Cannot convert input")
```

---

## Section 7: Production Deployment

### 7.1 Environment Configuration Pattern

**Pattern**:
```bash
# Production environment setup
# /etc/qmnf/env.sh

export QMNF_HOME=/opt/qmnf
export LD_LIBRARY_PATH=$QMNF_HOME/lib:$LD_LIBRARY_PATH
export PYTHONPATH=$QMNF_HOME:$PYTHONPATH

export QMNF_LOG_LEVEL=INFO
export QMNF_LOG_FILE=/var/log/qmnf/app.log

export QMNF_CACHE_SIZE=1024
export QMNF_MAX_WORKERS=8
```

---

### 7.2 Health Check Pattern

**Pattern**:
```python
from flask import Flask, jsonify
from qmnf import QMNFRational

app = Flask(__name__)

@app.route('/health')
def health_check():
    """Health check endpoint."""
    try:
        # Test core functionality
        a = QMNFRational(1, 2)
        b = QMNFRational(1, 3)
        result = a + b

        # Verify correctness
        assert result == QMNFRational(5, 6)

        return jsonify({
            'status': 'healthy',
            'qmnf_version': '3.5.0',
            'core_functional': True
        }), 200

    except Exception as e:
        return jsonify({
            'status': 'unhealthy',
            'error': str(e)
        }), 500
```

---

## Section 8: Documentation Standards

### 8.1 Docstring Pattern

**Principle**: Document all public functions with Google-style docstrings.

**Pattern**:
```python
from qmnf import QMNFRational

def compute_compound_interest(
    principal: QMNFRational,
    rate: QMNFRational,
    periods: int
) -> QMNFRational:
    """
    Compute compound interest with exact arithmetic.

    This function uses exact rational arithmetic to compute compound interest
    without any floating-point rounding errors.

    Args:
        principal: Initial principal amount (QMNFRational)
        rate: Interest rate per period (QMNFRational, e.g., 0.05 = 5%)
        periods: Number of compounding periods (int)

    Returns:
        Final amount including principal and interest (QMNFRational)

    Raises:
        TypeError: If principal or rate are not QMNFRational
        ValueError: If periods is negative

    Example:
        >>> principal = QMNFRational(1000, 1)
        >>> rate = QMNFRational(5, 100)  # 5%
        >>> periods = 10
        >>> result = compute_compound_interest(principal, rate, periods)
        >>> print(float(result))
        1628.89

    See Also:
        - compute_simple_interest: For simple interest calculation
        - QMNFRational: For exact rational arithmetic

    References:
        - https://en.wikipedia.org/wiki/Compound_interest
    """
    if not isinstance(principal, QMNFRational):
        raise TypeError("principal must be QMNFRational")
    if not isinstance(rate, QMNFRational):
        raise TypeError("rate must be QMNFRational")
    if periods < 0:
        raise ValueError("periods must be non-negative")

    one = QMNFRational(1, 1)
    return principal * (one + rate) ** periods
```

---

## Quick Reference: Dos and Don'ts

### ✅ DO

- Use `QMNFRational` for all core computation
- Convert floats only at boundaries
- Use batch operations for arrays
- Add type hints to all functions
- Test algebraic properties
- Log errors with context
- Profile before optimizing
- Document public APIs

### ❌ DON'T

- Use float literals in core math
- Convert rational → float → rational
- Loop with FFI operations
- Validate in every function
- Test only happy paths
- Ignore errors silently
- Optimize without profiling
- Leave functions undocumented

---

**Last Updated**: 2025-11-16
**Maintainer**: Anthony Diaz (founder@hackfate.us)
**License**: Proprietary - See LICENSE file

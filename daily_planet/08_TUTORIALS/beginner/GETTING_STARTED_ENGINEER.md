# Getting Started - Engineer Guide

**For**: Engineers building features and integrating QMNF into existing systems

**Goal**: Productive coding in 1-2 hours

**Last Updated**: 2025-11-16

---

## What You'll Learn

By the end of this guide, you will:
- ✅ Understand QMNF's API and usage patterns
- ✅ Integrate QMNF into existing codebases
- ✅ Write tests for integer-only code
- ✅ Optimize performance using batch operations
- ✅ Deploy to production

**Time Investment**: 1-2 hours

---

## Prerequisites

Complete [QUICK_START.md](QUICK_START.md) first (5 minutes).

You should have:
- ✅ QMNF installed and verified
- ✅ Basic understanding of QMNFRational
- ✅ Performed first arithmetic operation

---

## Section 1: Core API Patterns (30 minutes)

### 1.1 Creating Rationals

```python
from qmnf import QMNFRational

# From integers (recommended)
r = QMNFRational(22, 7)

# From string
r = QMNFRational.from_string("22/7")  # If implemented

# From float (use sparingly, only at boundaries)
from qmnf.conversion_boundary import DataBoundary
pi_float = 3.14159
pi_data = DataBoundary.float_to_rational(pi_float, precision=5)
pi = QMNFRational(pi_data.numerator, pi_data.denominator)
```

**Best Practice**: Always create from integers when possible.

### 1.2 Arithmetic Operations

```python
from qmnf import QMNFRational

a = QMNFRational(1, 2)
b = QMNFRational(1, 3)

# Basic operations
addition = a + b        # 5/6
subtraction = a - b     # 1/6
multiplication = a * b  # 1/6
division = a / b        # 3/2
power = a ** 2          # 1/4
negation = -a           # -1/2
absolute = abs(a)       # 1/2

# In-place operations (if supported)
c = QMNFRational(1, 2)
c += QMNFRational(1, 3)  # c is now 5/6
```

### 1.3 Comparisons

```python
from qmnf import QMNFRational

a = QMNFRational(1, 2)
b = QMNFRational(1, 3)

# All comparison operators work
a == b   # False
a != b   # True
a > b    # True
a < b    # False
a >= b   # True
a <= b   # False

# Use in sorting
rationals = [QMNFRational(1, 3), QMNFRational(1, 2), QMNFRational(1, 4)]
sorted_rats = sorted(rationals)  # [1/4, 1/3, 1/2]
```

### 1.4 Utility Methods

```python
from qmnf import QMNFRational

r = QMNFRational(22, 7)

# Access components
num = r.numerator()      # 22
den = r.denominator()    # 7

# Properties
is_int = r.is_integer()  # False
is_zero = r.is_zero()    # False

# Transformations
reciprocal = r.reciprocal()  # 7/22

# Conversion (output only!)
float_approx = float(r)      # 3.142857...
```

---

## Section 2: Integration Patterns (30 minutes)

### 2.1 Integrating with Existing Python Code

**Scenario**: You have existing float-based code and want to migrate to QMNF.

**Strategy**: Convert at boundaries, keep internal computation exact.

```python
# BEFORE (float-based):
def calculate_interest(principal, rate, years):
    return principal * (1 + rate) ** years

result = calculate_interest(1000.0, 0.05, 10)
# Result: 1628.8946267774416 (approximation)

# AFTER (QMNF exact):
from qmnf import QMNFRational
from qmnf.conversion_boundary import DataBoundary

def calculate_interest(principal, rate, years):
    """
    Args:
        principal: QMNFRational
        rate: QMNFRational
        years: int
    Returns:
        QMNFRational (exact)
    """
    return principal * (QMNFRational(1, 1) + rate) ** years

# At application boundary
principal_float = 1000.0
rate_float = 0.05

principal = DataBoundary.float_to_rational(principal_float)
rate = DataBoundary.float_to_rational(rate_float)

principal_qmnf = QMNFRational(principal.numerator, principal.denominator)
rate_qmnf = QMNFRational(rate.numerator, rate.denominator)

result = calculate_interest(principal_qmnf, rate_qmnf, 10)
# Result: Exact rational (convert to float for display)
print(f"${float(result):.2f}")
```

**Key Pattern**: Float → Rational at input, Rational → Float at output.

### 2.2 Type Hints and Validation

```python
from typing import Union
from qmnf import QMNFRational
from qmnf.conversion_boundary import DataBoundary

def process_data(
    values: list[QMNFRational],
    threshold: QMNFRational
) -> list[QMNFRational]:
    """
    Process data with exact arithmetic.

    Args:
        values: List of QMNFRational values
        threshold: Minimum threshold (QMNFRational)

    Returns:
        Filtered list of values above threshold

    Raises:
        TypeError: If inputs are not QMNFRational
    """
    if not all(isinstance(v, QMNFRational) for v in values):
        raise TypeError("All values must be QMNFRational")

    if not isinstance(threshold, QMNFRational):
        raise TypeError("Threshold must be QMNFRational")

    return [v for v in values if v > threshold]

# Usage
values = [QMNFRational(i, 10) for i in range(1, 11)]
threshold = QMNFRational(1, 2)
filtered = process_data(values, threshold)
```

**Best Practice**: Add type hints and validation at function boundaries.

### 2.3 Working with Collections

```python
from qmnf import QMNFRational

# Lists
rationals = [QMNFRational(1, i) for i in range(1, 11)]
total = sum(rationals)  # Exact sum

# Sets (rationals are hashable)
unique_rationals = {QMNFRational(1, 2), QMNFRational(2, 4)}
print(len(unique_rationals))  # 1 (automatically deduplicated)

# Dictionaries
rational_map = {
    QMNFRational(1, 2): "half",
    QMNFRational(1, 3): "third",
    QMNFRational(1, 4): "quarter",
}

# Comprehensions
squares = [r ** 2 for r in rationals]
```

### 2.4 Interfacing with NumPy (When Necessary)

```python
import numpy as np
from qmnf import QMNFRational
from qmnf.conversion_boundary import DataBoundary

# Convert NumPy array to QMNFRational
def numpy_to_qmnf(arr: np.ndarray, precision: int = 10) -> list[QMNFRational]:
    """Convert NumPy float array to QMNF rationals."""
    rationals = []
    for val in arr.flat:
        r_data = DataBoundary.float_to_rational(float(val), precision=precision)
        rationals.append(QMNFRational(r_data.numerator, r_data.denominator))
    return rationals

# Convert QMNFRational list to NumPy (output only!)
def qmnf_to_numpy(rationals: list[QMNFRational]) -> np.ndarray:
    """Convert QMNF rationals to NumPy array (approximation)."""
    return np.array([float(r) for r in rationals])

# Usage
arr = np.array([0.1, 0.2, 0.3])
qmnf_values = numpy_to_qmnf(arr)
# Perform exact computations
results = [v * 2 for v in qmnf_values]
# Convert back for NumPy-based visualization
result_arr = qmnf_to_numpy(results)
```

**Warning**: NumPy conversion introduces approximation. Use only at output boundaries.

---

## Section 3: Performance Optimization (20 minutes)

### 3.1 Batch Operations (4-8× Speedup)

**Problem**: Python loops with QMNF operations incur FFI overhead per iteration.

**Solution**: Use Rust batch operations.

```python
from hcvlang import batch_add_crtbigint, batch_mul_crtbigint
from qmnf import QMNFRational

# ❌ BAD: Loop in Python
results = []
for i in range(1000):
    results.append(a + b)  # 1000 FFI crossings

# ✅ GOOD: Batch operation
from hcvlang import batch_add_rational
results = batch_add_rational([a]*1000, [b]*1000)  # 1 FFI crossing
```

**Available Batch Operations**:
- `batch_add_rational(a_list, b_list)`
- `batch_mul_rational(a_list, b_list)`
- `batch_add_crtbigint(a_list, b_list)`
- `batch_mul_crtbigint(a_list, b_list)`
- `sum_rational(rational_list)`
- `product_rational(rational_list)`

**Performance Gain**: 4-8× for fast operations.

### 3.2 Deferred Reconstruction Pattern

**Problem**: CRT reconstruction is expensive (O(k) per operation).

**Solution**: Batch operations in residue space, reconstruct once.

```python
# Example: Using CRTBigInt for bounded computations
from hcvlang import CRTBigInt

# Create CRT values
a = CRTBigInt(123456789)
b = CRTBigInt(987654321)

# Operations stay in residue space (fast!)
result = a + b
result = result * CRTBigInt(2)
result = result - CRTBigInt(100)

# Reconstruct only at the end
final_value = int(result)  # Single reconstruction
```

**Performance Gain**: 22× improvement validated for batched operations.

See [FFI_BRIDGE_ANALYSIS.md](FFI_BRIDGE_ANALYSIS.md) for detailed analysis.

### 3.3 Profiling Your Code

```bash
# Profile with cProfile
python3 -m cProfile -o profile.stats your_script.py

# Analyze results
python3 -c "
import pstats
p = pstats.Stats('profile.stats')
p.sort_stats('cumulative').print_stats(20)
"

# Identify bottlenecks
# Look for:
# 1. Repeated FFI crossings (many small calls)
# 2. Unnecessary reconstructions
# 3. Float conversions in hot paths
```

---

## Section 4: Testing (20 minutes)

### 4.1 Unit Testing Pattern

```python
import unittest
from qmnf import QMNFRational

class TestRationalArithmetic(unittest.TestCase):
    def test_addition(self):
        a = QMNFRational(1, 2)
        b = QMNFRational(1, 3)
        result = a + b

        # Assert exact equality
        self.assertEqual(result, QMNFRational(5, 6))

        # Or check components
        self.assertEqual(result.numerator(), 5)
        self.assertEqual(result.denominator(), 6)

    def test_multiplication(self):
        a = QMNFRational(2, 3)
        b = QMNFRational(3, 4)
        result = a * b

        self.assertEqual(result, QMNFRational(1, 2))

    def test_division(self):
        a = QMNFRational(1, 2)
        b = QMNFRational(1, 3)
        result = a / b

        self.assertEqual(result, QMNFRational(3, 2))

    def test_division_by_zero(self):
        a = QMNFRational(1, 2)
        zero = QMNFRational(0, 1)

        with self.assertRaises(ZeroDivisionError):
            result = a / zero

if __name__ == '__main__':
    unittest.main()
```

### 4.2 Integration Testing

```python
import unittest
from qmnf import QMNFRational
from qmnf.conversion_boundary import DataBoundary

class TestBoundaryConversion(unittest.TestCase):
    def test_float_to_rational(self):
        # Test boundary conversion
        float_val = 3.14
        rational_data = DataBoundary.float_to_rational(float_val, precision=2)
        r = QMNFRational(rational_data.numerator, rational_data.denominator)

        # Check approximation quality
        self.assertAlmostEqual(float(r), float_val, places=2)

    def test_rejects_float_literal(self):
        # Ensure floats are rejected
        with self.assertRaises((TypeError, ValueError)):
            QMNFRational(3.14, 1)

class TestPerformance(unittest.TestCase):
    def test_batch_vs_loop(self):
        import time
        from hcvlang import batch_add_rational

        a = QMNFRational(1, 2)
        b = QMNFRational(1, 3)
        n = 1000

        # Batch operation
        start = time.perf_counter()
        batch_results = batch_add_rational([a]*n, [b]*n)
        batch_time = time.perf_counter() - start

        # Loop operation
        start = time.perf_counter()
        loop_results = [a + b for _ in range(n)]
        loop_time = time.perf_counter() - start

        # Batch should be faster
        self.assertLess(batch_time, loop_time)
        print(f"Batch speedup: {loop_time/batch_time:.1f}×")
```

### 4.3 Running Tests

```bash
# Run all tests
pytest tests/ -v

# Run specific test file
pytest tests/python/test_rational.py -v

# Run with coverage
pytest tests/ --cov=qmnf --cov-report=html

# View coverage report
open htmlcov/index.html
```

---

## Section 5: Deployment (15 minutes)

### 5.1 Production Build

```bash
# Build optimized Rust library
cd hcvlang
cargo build --release --features "fast-paths,simd,parallel"

# Copy to deployment directory
cp target/release/libhcvlang.so /opt/qmnf/

# Set environment variables (add to systemd service or .bashrc)
export LD_LIBRARY_PATH=/opt/qmnf:$LD_LIBRARY_PATH
export PYTHONPATH=/opt/qmnf:$PYTHONPATH
```

### 5.2 Docker Deployment

```dockerfile
# Dockerfile for QMNF application
FROM rust:1.70 as builder

# Install Python
RUN apt-get update && apt-get install -y python3 python3-pip

# Copy source
WORKDIR /app
COPY . .

# Build Rust library
WORKDIR /app/hcvlang
RUN cargo build --release --features "fast-paths,simd,parallel"

# Runtime stage
FROM python:3.9-slim

# Copy built library and Python code
COPY --from=builder /app/hcvlang/target/release/libhcvlang.so /opt/qmnf/
COPY --from=builder /app/qmnf /opt/qmnf/qmnf

# Set environment
ENV LD_LIBRARY_PATH=/opt/qmnf:$LD_LIBRARY_PATH
ENV PYTHONPATH=/opt/qmnf:$PYTHONPATH

# Install Python dependencies
RUN pip install --no-cache-dir setuptools

# Run application
CMD ["python3", "/opt/qmnf/your_app.py"]
```

### 5.3 Error Handling in Production

```python
from qmnf import QMNFRational
from qmnf.conversion_boundary import DataBoundary
import logging

logger = logging.getLogger(__name__)

def safe_process(user_input: str) -> QMNFRational:
    """
    Safely process user input with comprehensive error handling.
    """
    try:
        # Attempt to parse as fraction
        if '/' in user_input:
            num, den = user_input.split('/')
            return QMNFRational(int(num), int(den))

        # Attempt to parse as float
        try:
            float_val = float(user_input)
            rational_data = DataBoundary.float_to_rational(float_val, precision=10)
            return QMNFRational(rational_data.numerator, rational_data.denominator)
        except ValueError:
            logger.error(f"Invalid input format: {user_input}")
            raise ValueError(f"Cannot convert '{user_input}' to rational")

    except ZeroDivisionError:
        logger.error("Division by zero in rational creation")
        raise

    except Exception as e:
        logger.exception(f"Unexpected error processing input: {user_input}")
        raise

# Usage
try:
    result = safe_process("3.14")
    print(f"Result: {result}")
except ValueError as e:
    print(f"Error: {e}")
```

### 5.4 Monitoring and Logging

```python
import logging
import time
from functools import wraps

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)

logger = logging.getLogger(__name__)

def monitor_performance(func):
    """Decorator to monitor function performance."""
    @wraps(func)
    def wrapper(*args, **kwargs):
        start = time.perf_counter()
        try:
            result = func(*args, **kwargs)
            elapsed = time.perf_counter() - start
            logger.info(f"{func.__name__} completed in {elapsed*1000:.2f}ms")
            return result
        except Exception as e:
            elapsed = time.perf_counter() - start
            logger.error(f"{func.__name__} failed after {elapsed*1000:.2f}ms: {e}")
            raise
    return wrapper

# Usage
from qmnf import QMNFRational

@monitor_performance
def compute_sum(n: int) -> QMNFRational:
    return sum(QMNFRational(1, i) for i in range(1, n+1))

result = compute_sum(100)
# Log output: "compute_sum completed in 12.34ms"
```

---

## Section 6: Common Integration Scenarios

### 6.1 REST API Integration

```python
from flask import Flask, request, jsonify
from qmnf import QMNFRational
from qmnf.conversion_boundary import DataBoundary

app = Flask(__name__)

@app.route('/compute', methods=['POST'])
def compute():
    """
    API endpoint for exact rational computation.

    Request: {"a": "1/2", "b": "1/3", "operation": "add"}
    Response: {"result": "5/6", "decimal": 0.8333...}
    """
    try:
        data = request.get_json()

        # Parse inputs
        a_parts = data['a'].split('/')
        a = QMNFRational(int(a_parts[0]), int(a_parts[1]))

        b_parts = data['b'].split('/')
        b = QMNFRational(int(b_parts[0]), int(b_parts[1]))

        # Perform operation
        operation = data['operation']
        if operation == 'add':
            result = a + b
        elif operation == 'multiply':
            result = a * b
        elif operation == 'divide':
            result = a / b
        else:
            return jsonify({"error": "Invalid operation"}), 400

        # Return result
        return jsonify({
            "result": f"{result.numerator()}/{result.denominator()}",
            "decimal": float(result)
        })

    except Exception as e:
        return jsonify({"error": str(e)}), 500

if __name__ == '__main__':
    app.run(debug=False)
```

### 6.2 Database Integration

```python
import sqlite3
from qmnf import QMNFRational

class RationalDB:
    """Store QMNFRational values in database."""

    def __init__(self, db_path: str):
        self.conn = sqlite3.connect(db_path)
        self.cursor = self.conn.cursor()
        self._create_table()

    def _create_table(self):
        self.cursor.execute('''
            CREATE TABLE IF NOT EXISTS rationals (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                numerator INTEGER NOT NULL,
                denominator INTEGER NOT NULL,
                description TEXT
            )
        ''')
        self.conn.commit()

    def store(self, r: QMNFRational, description: str = ""):
        """Store a rational in the database."""
        self.cursor.execute(
            "INSERT INTO rationals (numerator, denominator, description) VALUES (?, ?, ?)",
            (r.numerator(), r.denominator(), description)
        )
        self.conn.commit()
        return self.cursor.lastrowid

    def retrieve(self, id: int) -> QMNFRational:
        """Retrieve a rational from the database."""
        self.cursor.execute(
            "SELECT numerator, denominator FROM rationals WHERE id = ?",
            (id,)
        )
        row = self.cursor.fetchone()
        if row is None:
            raise ValueError(f"No rational found with id {id}")
        return QMNFRational(row[0], row[1])

    def close(self):
        self.conn.close()

# Usage
db = RationalDB("rationals.db")
r = QMNFRational(22, 7)
id = db.store(r, "pi approximation")
retrieved = db.retrieve(id)
print(retrieved)  # 22/7
db.close()
```

---

## Section 7: Debugging and Troubleshooting

### 7.1 Common Issues

**Issue 1: Type Errors**
```python
# ❌ WRONG: Passing float
r = QMNFRational(3.14, 1)  # TypeError

# ✅ CORRECT: Convert explicitly
from qmnf.conversion_boundary import DataBoundary
data = DataBoundary.float_to_rational(3.14)
r = QMNFRational(data.numerator, data.denominator)
```

**Issue 2: Performance Bottlenecks**
```python
# ❌ WRONG: Loop with FFI overhead
for i in range(10000):
    result = a + b  # 10000 FFI calls

# ✅ CORRECT: Batch operation
from hcvlang import batch_add_rational
results = batch_add_rational([a]*10000, [b]*10000)  # 1 FFI call
```

**Issue 3: Import Errors**
```bash
# Error: ModuleNotFoundError: hcvlang_pyo3

# Solution: Set environment variables
export LD_LIBRARY_PATH=/path/to/QMNF_System:$LD_LIBRARY_PATH
export PYTHONPATH=/path/to/QMNF_System:$PYTHONPATH

# Verify
python3 -c "import hcvlang_pyo3; print('OK')"
```

### 7.2 Debugging Tools

```python
# Enable detailed logging
import logging
logging.basicConfig(level=logging.DEBUG)

# Inspect rational components
r = QMNFRational(22, 7)
print(f"Numerator: {r.numerator()}")
print(f"Denominator: {r.denominator()}")
print(f"Float approx: {float(r)}")
print(f"Is integer: {r.is_integer()}")

# Check type
print(type(r))  # <class 'qmnf.boundary.QMNFRational'>
```

---

## Next Steps

**You Now Can**:
- ✅ Integrate QMNF into existing codebases
- ✅ Write tests for rational arithmetic
- ✅ Optimize performance with batch operations
- ✅ Deploy to production
- ✅ Debug common issues

**Continue Learning**:
- Deep dive into FFI optimization: [FFI_BRIDGE_ANALYSIS.md](FFI_BRIDGE_ANALYSIS.md)
- Explore common tasks: [COMMON_TASKS_COOKBOOK.md](COMMON_TASKS_COOKBOOK.md)
- Master troubleshooting: [TROUBLESHOOTING_PLAYBOOK.md](TROUBLESHOOTING_PLAYBOOK.md)

**Build Something**:
- Cryptographic application: [FHE_DELIVERABLES_INDEX.md](FHE_DELIVERABLES_INDEX.md)
- Neural network: [GETTING_STARTED_DATA_SCIENTIST.md](GETTING_STARTED_DATA_SCIENTIST.md)
- Storage system: [INTEGRATION_QUICK_REFERENCE.md](INTEGRATION_QUICK_REFERENCE.md)

---

**Time Invested**: ~1-2 hours
**Next**: Build your first integration! See [COMMON_TASKS_COOKBOOK.md](COMMON_TASKS_COOKBOOK.md)

---

**Last Updated**: 2025-11-16
**Maintainer**: Anthony Diaz (founder@hackfate.us)
**License**: Proprietary - See LICENSE file

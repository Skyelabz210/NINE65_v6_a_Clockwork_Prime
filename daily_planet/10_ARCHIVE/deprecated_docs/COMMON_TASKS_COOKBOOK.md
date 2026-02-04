# QMNF Common Tasks Cookbook

**"How do I...?" - Practical Solutions for Everyday QMNF Tasks**

**Last Updated**: 2025-11-16

---

## Table of Contents

1. [Basic Arithmetic](#section-1-basic-arithmetic)
2. [Boundary Conversions](#section-2-boundary-conversions)
3. [Batch Operations](#section-3-batch-operations)
4. [Cryptography (FHE)](#section-4-cryptography-fhe)
5. [Neural Networks](#section-5-neural-networks)
6. [Storage Systems](#section-6-storage-systems)
7. [Performance Optimization](#section-7-performance-optimization)
8. [Production Deployment](#section-8-production-deployment)
9. [Integration Patterns](#section-9-integration-patterns)
10. [Debugging and Validation](#section-10-debugging-and-validation)

---

## Section 1: Basic Arithmetic

### 1.1 How do I add integers without floating points?

```python
from qmnf import QMNFRational

# Create rationals from integers
a = QMNFRational(5, 1)  # 5
b = QMNFRational(3, 1)  # 3

# Add (exact)
result = a + b  # 8/1 = 8
print(result)  # 8/1
```

### 1.2 How do I perform exact division?

```python
from qmnf import QMNFRational

# Division that would be imprecise with floats
a = QMNFRational(1, 3)  # 1/3
b = QMNFRational(1, 7)  # 1/7

result = a / b  # Exact: 7/3
print(result)  # 7/3
print(float(result))  # 2.333... (for display)
```

### 1.3 How do I compute fractions?

```python
from qmnf import QMNFRational

# Financial calculation (no rounding errors)
price = QMNFRational(99, 100)      # $0.99
tax_rate = QMNFRational(7, 100)    # 7%
quantity = 10

subtotal = price * QMNFRational(quantity, 1)
tax = subtotal * tax_rate
total = subtotal + tax

print(f"Subtotal: ${float(subtotal):.2f}")
print(f"Tax: ${float(tax):.2f}")
print(f"Total: ${float(total):.2f}")
```

### 1.4 How do I compute series summations?

```python
from qmnf import QMNFRational

# Harmonic series (exact partial sum)
def harmonic_sum(n):
    """Compute H(n) = 1/1 + 1/2 + ... + 1/n exactly."""
    return sum(QMNFRational(1, i) for i in range(1, n+1))

h_10 = harmonic_sum(10)
print(f"H(10) = {h_10}")
print(f"Approximation: {float(h_10)}")
```

### 1.5 How do I handle negative numbers?

```python
from qmnf import QMNFRational

# Negative rationals
a = QMNFRational(-1, 2)  # -1/2
b = QMNFRational(1, -3)  # -1/3 (normalized to -1/3)

# Operations preserve sign
result = a + b  # -1/2 + (-1/3) = -5/6
print(result)

# Absolute value
abs_result = abs(result)
print(abs_result)  # 5/6
```

---

## Section 2: Boundary Conversions

### 2.1 How do I convert floats safely?

```python
from qmnf.conversion_boundary import DataBoundary
from qmnf import QMNFRational

# Convert float to rational at boundary
user_input = 3.14159

# Method 1: Using DataBoundary (recommended)
pi_data = DataBoundary.float_to_rational(user_input, precision=5)
pi = QMNFRational(pi_data.numerator, pi_data.denominator)

print(f"Converted {user_input} to {pi.numerator()}/{pi.denominator()}")
```

### 2.2 How do I handle user input?

```python
from qmnf.conversion_boundary import DataBoundary
from qmnf import QMNFRational

def parse_user_input(user_str):
    """
    Parse user input to QMNFRational.

    Accepts:
    - "22/7" (fraction)
    - "3.14" (decimal, converted to rational)
    - "42" (integer)
    """
    user_str = user_str.strip()

    # Check if fraction
    if '/' in user_str:
        num, den = user_str.split('/')
        return QMNFRational(int(num), int(den))

    # Check if decimal
    if '.' in user_str:
        data = DataBoundary.float_to_rational(float(user_str), precision=10)
        return QMNFRational(data.numerator, data.denominator)

    # Integer
    return QMNFRational(int(user_str), 1)

# Usage
r = parse_user_input("3.14")
print(r)  # 314/100 (simplified to 157/50)
```

### 2.3 How do I output results for display?

```python
from qmnf import QMNFRational

r = QMNFRational(22, 7)

# Method 1: Convert to float (approximation)
print(f"Approximation: {float(r):.6f}")

# Method 2: Display as fraction
print(f"Exact: {r.numerator()}/{r.denominator()}")

# Method 3: Both
print(f"π ≈ {r.numerator()}/{r.denominator()} ≈ {float(r):.6f}")
```

### 2.4 How do I validate input types?

```python
from qmnf import QMNFRational

def safe_divide(a, b):
    """Safely divide with type checking."""
    # Validate types
    if not isinstance(a, QMNFRational):
        raise TypeError(f"Expected QMNFRational for a, got {type(a)}")
    if not isinstance(b, QMNFRational):
        raise TypeError(f"Expected QMNFRational for b, got {type(b)}")

    # Check for zero
    if b.is_zero():
        raise ZeroDivisionError("Cannot divide by zero")

    return a / b

# Usage
try:
    result = safe_divide(QMNFRational(1, 2), QMNFRational(1, 3))
    print(result)  # 3/2
except (TypeError, ZeroDivisionError) as e:
    print(f"Error: {e}")
```

---

## Section 3: Batch Operations

### 3.1 How do I process arrays efficiently?

```python
from hcvlang import batch_add_rational, batch_mul_rational
from qmnf import QMNFRational

# Create arrays
a_values = [QMNFRational(1, i) for i in range(1, 101)]  # 1/1, 1/2, ..., 1/100
b_values = [QMNFRational(2, i) for i in range(1, 101)]  # 2/1, 2/2, ..., 2/100

# ❌ BAD: Loop (100 FFI crossings)
results_slow = [a + b for a, b in zip(a_values, b_values)]

# ✅ GOOD: Batch operation (1 FFI crossing, 4-8× faster)
results_fast = batch_add_rational(a_values, b_values)

print(f"Computed {len(results_fast)} additions efficiently")
```

### 3.2 How do I sum a large list?

```python
from hcvlang import sum_rational
from qmnf import QMNFRational

# Create list
values = [QMNFRational(1, i) for i in range(1, 1001)]

# ❌ BAD: Python sum (1000 FFI crossings)
total_slow = sum(values)

# ✅ GOOD: Batch sum (1 FFI crossing)
total_fast = sum_rational(values)

print(f"Total: {total_fast}")
```

### 3.3 How do I compute products efficiently?

```python
from hcvlang import product_rational
from qmnf import QMNFRational

# Factorial using product
n = 10
factors = [QMNFRational(i, 1) for i in range(1, n+1)]

factorial = product_rational(factors)
print(f"{n}! = {factorial}")
```

---

## Section 4: Cryptography (FHE)

### 4.1 How do I encrypt data homomorphically?

```python
from hcvlang import FHEContext, SecurityLevel

# Initialize FHE context
ctx = FHEContext(SecurityLevel(1))  # 128-bit security

# Generate keys
sk, pk = ctx.generate_keypair()

# Encrypt integers
plaintext1 = ctx.encode(10)
plaintext2 = ctx.encode(32)

ciphertext1 = ctx.encrypt(plaintext1, pk)
ciphertext2 = ctx.encrypt(plaintext2, pk)

print("Data encrypted successfully")
```

### 4.2 How do I perform operations on encrypted data?

```python
from hcvlang import FHEContext, SecurityLevel

ctx = FHEContext(SecurityLevel(1))
sk, pk = ctx.generate_keypair()

# Encrypt
ct1 = ctx.encrypt(ctx.encode(10), pk)
ct2 = ctx.encrypt(ctx.encode(32), pk)

# Homomorphic operations (on encrypted data!)
ct_sum = ctx.add(ct1, ct2)       # 10 + 32 = 42
ct_product = ctx.mul(ct1, ct2)   # 10 × 32 = 320

# Decrypt to verify
result_sum = ctx.decode(ctx.decrypt(ct_sum, sk))
result_product = ctx.decode(ctx.decrypt(ct_product, sk))

print(f"Sum: {result_sum}")        # 42
print(f"Product: {result_product}")  # 320
```

### 4.3 How do I use batch FHE operations?

```python
from hcvlang import BatchFHEProcessor, BatchConfig, FHEContext, SecurityLevel

# Setup
ctx = FHEContext(SecurityLevel(1))
sk, pk = ctx.generate_keypair()

# Configure batch processing
config = BatchConfig(batch_size=256, parallel_enabled=True)
processor = BatchFHEProcessor(config)

# Encrypt batch
plaintexts = [ctx.encode(i) for i in range(100)]
ciphertexts = processor.batch_encrypt(plaintexts, pk, ctx)

print(f"Encrypted {len(ciphertexts)} values (8× faster with parallelism)")
```

**See**: [FHE_DELIVERABLES_INDEX.md](FHE_DELIVERABLES_INDEX.md) for complete guide.

---

## Section 5: Neural Networks

### 5.1 How do I create an integer-only neural network?

```python
from qmnf.neural.helix_compiler import HelixNeuralNet
from qmnf import QMNFRational

# Create network (all weights are QMNFRational)
model = HelixNeuralNet(
    input_dim=784,    # MNIST input
    hidden_dim=128,
    output_dim=10
)

print("Neural network created with integer-only arithmetic")
```

### 5.2 How do I train with exact gradients?

```python
from qmnf.neural.helix_compiler import HelixNeuralNet
from qmnf.neural.hpo import HiveGSO
from qmnf import QMNFRational

# Create model
model = HelixNeuralNet(input_dim=784, hidden_dim=128, output_dim=10)

# Create optimizer (integer-only gradients)
optimizer = HiveGSO(model.parameters(), learning_rate=QMNFRational(1, 1000))

# Training loop (conceptual)
for epoch in range(10):
    for data, labels in dataset:
        # Forward pass (exact)
        predictions = model.forward(data)

        # Compute loss (exact)
        loss = model.compute_loss(predictions, labels)

        # Backward pass (exact gradients)
        gradients = model.backward(loss)

        # Update (exact)
        optimizer.step(gradients)

    print(f"Epoch {epoch}, Loss: {loss}")
```

**Note**: Neural network integration is in active development. See [GETTING_STARTED_DATA_SCIENTIST.md](GETTING_STARTED_DATA_SCIENTIST.md) for current status.

---

## Section 6: Storage Systems

### 6.1 How do I store data with COSMOS?

```python
from qmnf.storage.cosmos import COSMOSBackend
from qmnf import QMNFRational

# Initialize storage
cosmos = COSMOSBackend(capacity=1024**2)  # 1MB

# Store data (exact rational tensors)
data = [[QMNFRational(i, j) for j in range(1, 11)] for i in range(1, 11)]
cosmos.store(key="matrix_data", value=data)

# Retrieve
retrieved = cosmos.retrieve(key="matrix_data")
print(f"Retrieved {len(retrieved)} rows")
```

### 6.2 How do I use HoloHD for distributed storage?

```python
# HoloHD provides SVD-based holographic encoding
# See INTEGRATION_QUICK_REFERENCE.md for detailed API

from qmnf.storage.holohd import HoloHDBackend

# Initialize with Reed-Solomon error correction
holohd = HoloHDBackend(
    nodes=5,
    redundancy=2,  # Can lose 2 nodes
    encoding="svd"
)

# Store with automatic sharding
holohd.store_distributed(key="large_dataset", value=dataset)

# Retrieve (automatic reconstruction)
retrieved = holohd.retrieve_distributed(key="large_dataset")
```

**See**: [INTEGRATION_QUICK_REFERENCE.md](INTEGRATION_QUICK_REFERENCE.md) for complete API.

---

## Section 7: Performance Optimization

### 7.1 How do I optimize my code for speed?

**Rule 1**: Use batch operations
```python
# ❌ BAD
results = [compute(x) for x in inputs]  # Many FFI crossings

# ✅ GOOD
from hcvlang import batch_compute
results = batch_compute(inputs)  # Single FFI crossing
```

**Rule 2**: Minimize Python-Rust boundaries
```python
# ❌ BAD
for i in range(1000):
    intermediate = rust_operation_1(data)
    result = rust_operation_2(intermediate)

# ✅ GOOD
# Combine operations in Rust
result = rust_combined_operation(data, iterations=1000)
```

**Rule 3**: Use deferred reconstruction
```python
from hcvlang import CRTBigInt

# Create CRT value
a = CRTBigInt(123456789)

# Many operations (all in residue space, fast!)
for _ in range(1000):
    a = a + CRTBigInt(1)
    a = a * CRTBigInt(2)

# Reconstruct only once
final_value = int(a)  # Single reconstruction
```

### 7.2 How do I profile my code?

```bash
# Method 1: cProfile
python3 -m cProfile -o profile.stats your_script.py

# Analyze
python3 -c "
import pstats
p = pstats.Stats('profile.stats')
p.sort_stats('cumulative').print_stats(20)
"

# Method 2: line_profiler
pip install line_profiler
kernprof -l -v your_script.py
```

### 7.3 How do I benchmark performance?

```python
import time
from qmnf import QMNFRational

def benchmark_operation(func, args, iterations=1000):
    """Benchmark a function."""
    # Warm up
    for _ in range(10):
        func(*args)

    # Measure
    start = time.perf_counter()
    for _ in range(iterations):
        result = func(*args)
    elapsed = time.perf_counter() - start

    ops_per_sec = iterations / elapsed
    ns_per_op = (elapsed / iterations) * 1_000_000_000

    print(f"{func.__name__}:")
    print(f"  {ops_per_sec:.0f} ops/sec")
    print(f"  {ns_per_op:.0f} ns/op")

# Usage
a = QMNFRational(22, 7)
b = QMNFRational(1, 3)
benchmark_operation(lambda: a + b, ())
```

---

## Section 8: Production Deployment

### 8.1 How do I deploy to production?

```bash
# Step 1: Build optimized Rust library
cd hcvlang
cargo build --release --features "fast-paths,simd,parallel"

# Step 2: Copy to deployment directory
sudo cp target/release/libhcvlang.so /opt/qmnf/

# Step 3: Set environment (add to /etc/environment or systemd service)
export LD_LIBRARY_PATH=/opt/qmnf:$LD_LIBRARY_PATH
export PYTHONPATH=/opt/qmnf:$PYTHONPATH

# Step 4: Run tests
pytest /opt/qmnf/tests/ -v

# Step 5: Start application
systemctl start qmnf-app
```

### 8.2 How do I handle errors in production?

```python
import logging
from qmnf import QMNFRational
from qmnf.conversion_boundary import DataBoundary

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler('/var/log/qmnf/app.log'),
        logging.StreamHandler()
    ]
)

logger = logging.getLogger(__name__)

def safe_compute(user_input):
    """Production-safe computation with error handling."""
    try:
        # Validate and convert input
        data = DataBoundary.float_to_rational(float(user_input), precision=10)
        r = QMNFRational(data.numerator, data.denominator)

        # Perform computation
        result = r * QMNFRational(2, 1)

        logger.info(f"Computation successful: {user_input} -> {result}")
        return result

    except ValueError as e:
        logger.error(f"Invalid input: {user_input} - {e}")
        raise

    except Exception as e:
        logger.exception(f"Unexpected error processing {user_input}")
        raise

# Usage
try:
    result = safe_compute("3.14")
except Exception as e:
    # Handle gracefully
    print(f"Error: {e}")
```

### 8.3 How do I monitor performance in production?

```python
import time
import logging
from functools import wraps

logger = logging.getLogger(__name__)

def monitor_latency(threshold_ms=100):
    """Decorator to monitor and alert on slow operations."""
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            start = time.perf_counter()
            try:
                result = func(*args, **kwargs)
                elapsed_ms = (time.perf_counter() - start) * 1000

                if elapsed_ms > threshold_ms:
                    logger.warning(
                        f"{func.__name__} slow: {elapsed_ms:.2f}ms "
                        f"(threshold: {threshold_ms}ms)"
                    )
                else:
                    logger.debug(f"{func.__name__}: {elapsed_ms:.2f}ms")

                return result
            except Exception as e:
                elapsed_ms = (time.perf_counter() - start) * 1000
                logger.error(f"{func.__name__} failed after {elapsed_ms:.2f}ms: {e}")
                raise
        return wrapper
    return decorator

# Usage
@monitor_latency(threshold_ms=50)
def compute_intensive_operation(data):
    # Your computation
    pass
```

---

## Section 9: Integration Patterns

### 9.1 How do I integrate with NumPy?

```python
import numpy as np
from qmnf import QMNFRational
from qmnf.conversion_boundary import DataBoundary

def numpy_to_qmnf(arr, precision=10):
    """Convert NumPy array to QMNFRational list."""
    rationals = []
    for val in arr.flat:
        data = DataBoundary.float_to_rational(float(val), precision=precision)
        rationals.append(QMNFRational(data.numerator, data.denominator))
    return rationals

def qmnf_to_numpy(rationals):
    """Convert QMNFRational list to NumPy array (approximation)."""
    return np.array([float(r) for r in rationals])

# Usage
arr = np.array([0.1, 0.2, 0.3])
qmnf_values = numpy_to_qmnf(arr)

# Exact computation
results = [v * QMNFRational(2, 1) for v in qmnf_values]

# Convert back for visualization
result_arr = qmnf_to_numpy(results)
```

### 9.2 How do I integrate with Pandas?

```python
import pandas as pd
from qmnf import QMNFRational
from qmnf.conversion_boundary import DataBoundary

# Read CSV
df = pd.read_csv("data.csv")

# Convert column to QMNF
def convert_to_qmnf(value):
    data = DataBoundary.float_to_rational(float(value), precision=10)
    return QMNFRational(data.numerator, data.denominator)

df['qmnf_value'] = df['float_value'].apply(convert_to_qmnf)

# Perform exact computations
df['result'] = df['qmnf_value'].apply(lambda x: x * QMNFRational(2, 1))

# Convert back for display
df['result_float'] = df['result'].apply(float)
```

### 9.3 How do I create a REST API?

```python
from flask import Flask, request, jsonify
from qmnf import QMNFRational

app = Flask(__name__)

@app.route('/compute', methods=['POST'])
def compute():
    """Exact rational computation API."""
    try:
        data = request.get_json()
        a = QMNFRational(data['a_num'], data['a_den'])
        b = QMNFRational(data['b_num'], data['b_den'])

        result = a + b

        return jsonify({
            'numerator': result.numerator(),
            'denominator': result.denominator(),
            'decimal': float(result)
        })
    except Exception as e:
        return jsonify({'error': str(e)}), 400

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5000)
```

---

## Section 10: Debugging and Validation

### 10.1 How do I check for float contamination?

```bash
# Run float detection tool
python3 tools/check_no_floats.py

# Check specific file
python3 tools/check_no_floats.py --file my_module.py

# Check directory
python3 tools/check_no_floats.py --dir qmnf/
```

### 10.2 How do I validate my installation?

```bash
# Run installation verification
python3 tools/verify_installation.py

# Run basic operations test
python3 tools/test_basic_operations.py

# Run comprehensive tests
pytest tests/ -v
```

### 10.3 How do I debug type errors?

```python
from qmnf import QMNFRational

def debug_type(value):
    """Debug type information."""
    print(f"Type: {type(value)}")
    print(f"Value: {value}")

    if isinstance(value, QMNFRational):
        print(f"Numerator: {value.numerator()}")
        print(f"Denominator: {value.denominator()}")
        print(f"Is integer: {value.is_integer()}")
        print(f"Is zero: {value.is_zero()}")
    else:
        print(f"Not a QMNFRational!")

# Usage
r = QMNFRational(22, 7)
debug_type(r)
```

---

## Quick Reference Card

| Task | Method | Section |
|------|--------|---------|
| Create rational | `QMNFRational(num, den)` | 1.1 |
| Convert float | `DataBoundary.float_to_rational()` | 2.1 |
| Batch operations | `batch_add_rational()` | 3.1 |
| Encrypt data | `FHEContext.encrypt()` | 4.1 |
| Train network | `HelixNeuralNet()` | 5.1 |
| Store data | `COSMOSBackend.store()` | 6.1 |
| Optimize performance | Use batch ops, defer reconstruction | 7.1 |
| Deploy | Build Rust, set env vars | 8.1 |
| Integrate NumPy | `numpy_to_qmnf()` | 9.1 |
| Validate | `verify_installation.py` | 10.2 |

---

**Need More Help?**
- **Troubleshooting**: [TROUBLESHOOTING_PLAYBOOK.md](TROUBLESHOOTING_PLAYBOOK.md)
- **API Reference**: [INTEGRATION_QUICK_REFERENCE.md](INTEGRATION_QUICK_REFERENCE.md)
- **Best Practices**: [BEST_PRACTICES.md](BEST_PRACTICES.md)

---

**Last Updated**: 2025-11-16
**Maintainer**: Anthony Diaz (founder@hackfate.us)
**License**: Proprietary - See LICENSE file

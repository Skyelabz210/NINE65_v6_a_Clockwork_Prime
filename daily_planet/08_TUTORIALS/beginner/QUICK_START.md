# QMNF Quick Start Guide

**Get started in 5 minutes**

**Last Updated**: 2025-11-16

---

## What is QMNF?

**QMNF** (Quantum-Modular Numerical Framework) is a high-performance computational arithmetic library that provides **100% integer-only mathematics** with **exact rational arithmetic**.

**Key Innovation**: Achieves exact computation at arbitrary scale with competitive performance (~120ns operations).

**Why No Floats?**
- Floating-point arithmetic introduces rounding errors
- QMNF uses exact rational representation (numerator/denominator)
- Result: Mathematically perfect computation at any scale

---

## Installation (5 minutes)

### Prerequisites

- Python 3.9+
- Rust 1.70+
- 4GB RAM (8GB recommended)

### Install

```bash
# Clone repository
git clone https://github.com/Skyelabz210/QMNF_System.git
cd QMNF_System

# Install build dependencies
pip3 install setuptools setuptools-rust --break-system-packages

# Build Rust library with Python bindings
python3 setup.py build_rust --release --inplace

# Verify installation
python3 -c "import hcvlang_pyo3; print('✓ Rust bindings OK')"
python3 -c "from qmnf import QMNFRational; print('✓ QMNF API OK')"
```

**Expected output**:
```
✓ Rust bindings OK
✓ QMNF API OK
```

**Troubleshooting**: See [TROUBLESHOOTING_PLAYBOOK.md](TROUBLESHOOTING_PLAYBOOK.md) if errors occur.

---

## First Steps (5 minutes)

### 1. Create Exact Rationals

```python
from qmnf import QMNFRational

# Create rational numbers (no floats!)
pi = QMNFRational(22, 7)          # π approximation
phi = QMNFRational(1618, 1000)    # φ (golden ratio) approximation
half = QMNFRational(1, 2)

print(pi)    # Output: 22/7
print(phi)   # Output: 1618/1000 (simplified to 809/500)
print(half)  # Output: 1/2
```

**Key Concept**: Rationals are stored as (numerator, denominator) pairs—mathematically exact.

### 2. Perform Arithmetic

```python
from qmnf import QMNFRational

a = QMNFRational(1, 2)    # 1/2
b = QMNFRational(1, 3)    # 1/3

# All operations are exact
print(a + b)              # 5/6
print(a * b)              # 1/6
print(a / b)              # 3/2
print(a ** 2)             # 1/4

# Comparisons work
print(a > b)              # True
print(a == QMNFRational(2, 4))  # True (automatically simplified)
```

**Why This Matters**:
```python
# Floating-point (WRONG):
# 0.1 + 0.2 = 0.30000000000000004  ❌

# QMNF (EXACT):
a = QMNFRational(1, 10)  # 0.1
b = QMNFRational(2, 10)  # 0.2
print(a + b)              # 3/10 (exactly) ✅
```

### 3. Convert Floats (When Necessary)

```python
from qmnf.conversion_boundary import DataBoundary
from qmnf import QMNFRational

# Convert float to rational (use sparingly!)
pi_float = 3.14159
pi_rational = DataBoundary.float_to_rational(pi_float, precision=5)

# Wrap in QMNFRational for operations
pi = QMNFRational(pi_rational.numerator, pi_rational.denominator)
print(pi)  # 314159/100000
```

**Best Practice**: Only convert floats at system boundaries (input/output). Keep all internal computation as QMNFRational.

### 4. Access Components

```python
from qmnf import QMNFRational

r = QMNFRational(22, 7)

# Get numerator and denominator
print(r.numerator())      # 22
print(r.denominator())    # 7

# Check properties
print(r.is_integer())     # False
print(r.is_zero())        # False

# Get reciprocal
print(r.reciprocal())     # 7/22

# Convert to float (for display only)
print(float(r))           # 3.142857142857143
```

---

## Verify It Works

Run the installation validation script:

```bash
python3 tools/verify_installation.py
```

**Expected output**:
```
✓ Python imports working
✓ Rust bindings loaded
✓ QMNFRational operations work
✓ Arithmetic correctness verified
✓ Performance baseline established

Installation verified successfully!
```

---

## Understanding Performance

QMNF has a three-tier architecture:

```
Python Layer (QMNFRational)
    ↓ FFI (PyO3)
Rust Layer (CRTBigInt - Fast Bounded)
    ↓ Automatic conversion
Rust Layer (HCVLangBigInt - Infinite Exact)
```

**Performance Characteristics**:
- **CRTBigInt**: ~120-250ns per operation (bounded integers ±2^126)
- **HCVLangBigInt**: O(n²) but mathematically exact (unlimited scale)
- **FFI overhead**: ~15μs per Python→Rust call

**Optimization Tip**: Use batch operations to minimize FFI crossings:

```python
from hcvlang import batch_add_rational

# ❌ BAD: Loop in Python (many FFI calls)
results = []
for i in range(100):
    results.append(a + b)  # 100 FFI crossings

# ✅ GOOD: Batch operation (single FFI call)
results = batch_add_rational([a]*100, [b]*100)  # 1 FFI crossing
```

**Speedup**: 4-8× improvement for batch operations.

---

## Run Your First Benchmark

```bash
# Quick performance baseline
python3 milestone_benchmark.py

# View results
cat benchmarks/results/latest.json
```

**Typical Results** (Intel i7, 8GB RAM):
```
Rational Basic:    37,143 ops/sec
Geometric Points:  38,723 ops/sec
GCD Intensive:     83,261 ops/sec
Overall Average:   40,184 ops/sec
```

---

## Common Patterns

### Pattern 1: Exact Fractions

```python
from qmnf import QMNFRational

# Financial calculations (no rounding errors!)
price = QMNFRational(99, 100)      # $0.99
tax_rate = QMNFRational(7, 100)    # 7% tax
quantity = 10

subtotal = price * quantity
tax = subtotal * tax_rate
total = subtotal + tax

print(f"Total: ${float(total):.2f}")  # Exact computation, display rounded
```

### Pattern 2: Scientific Computing

```python
from qmnf import QMNFRational

# Harmonic series (exact partial sums)
def harmonic_sum(n):
    return sum(QMNFRational(1, i) for i in range(1, n+1))

# Exact result (no accumulated floating-point errors)
h_100 = harmonic_sum(100)
print(f"H(100) = {h_100}")
print(f"Approximation: {float(h_100)}")
```

### Pattern 3: Matrix Operations

```python
from qmnf import QMNFRational

# 2×2 matrix (exact representation)
matrix = [
    [QMNFRational(1, 2), QMNFRational(1, 3)],
    [QMNFRational(1, 4), QMNFRational(1, 5)]
]

# Exact matrix operations
def matrix_add(A, B):
    return [[A[i][j] + B[i][j] for j in range(len(A[0]))]
            for i in range(len(A))]

# No precision loss through operations
```

---

## Next Steps

### **Continue Learning** (Choose Your Path)

🔬 **Researcher**: Understanding algorithms and mathematics
→ Read [GETTING_STARTED_RESEARCHER.md](GETTING_STARTED_RESEARCHER.md)

🛠️ **Engineer**: Building features and integrating systems
→ Read [GETTING_STARTED_ENGINEER.md](GETTING_STARTED_ENGINEER.md)

📊 **Data Scientist**: Training models and running inference
→ Read [GETTING_STARTED_DATA_SCIENTIST.md](GETTING_STARTED_DATA_SCIENTIST.md)

🚀 **DevOps**: Deploying and optimizing infrastructure
→ Read [GETTING_STARTED_DEVOPS.md](GETTING_STARTED_DEVOPS.md)

### **Explore Use Cases**

**Cryptography** (Fully Homomorphic Encryption):
```python
from hcvlang import FHEContext, SecurityLevel

ctx = FHEContext(SecurityLevel(1))  # 128-bit security
sk, pk = ctx.generate_keypair()

ct1 = ctx.encrypt(ctx.encode(10), pk)
ct2 = ctx.encrypt(ctx.encode(32), pk)
ct_sum = ctx.add(ct1, ct2)  # Compute on encrypted data!

result = ctx.decode(ctx.decrypt(ct_sum, sk))
print(result)  # 42 (10 + 32, computed without decryption)
```

See [FHE_DELIVERABLES_INDEX.md](FHE_DELIVERABLES_INDEX.md) for complete guide.

**Neural Networks** (Integer-Only Training):
```python
from qmnf.neural.helix_compiler import HelixNeuralNet

model = HelixNeuralNet(input_dim=784, hidden_dim=128, output_dim=10)
# All gradients and weights use exact rational arithmetic
```

See [GETTING_STARTED_DATA_SCIENTIST.md](GETTING_STARTED_DATA_SCIENTIST.md) for complete guide.

**Storage Systems** (Holographic Encoding):
```python
from qmnf.storage.cosmos import COSMOSBackend

cosmos = COSMOSBackend(capacity=1024**2)  # 1MB
cosmos.store(key="data", value=tensor)
retrieved = cosmos.retrieve(key="data")
```

See [INTEGRATION_QUICK_REFERENCE.md](INTEGRATION_QUICK_REFERENCE.md) for API reference.

---

## Common Questions

### Q: Why can't I use floats?

**A**: Floats introduce rounding errors that accumulate over operations. QMNF guarantees mathematical exactness by using rational representation.

**Example**:
```python
# Float error (standard Python):
result = 0.1 + 0.1 + 0.1  # 0.30000000000000004 ❌

# QMNF exact (this library):
from qmnf import QMNFRational
r = QMNFRational(1, 10)
result = r + r + r  # 3/10 exactly ✅
```

### Q: Is it slower than floats?

**A**: For bounded integers (±2^126), performance is competitive (~120ns). For larger integers, exact computation is inherently more expensive than approximation.

**Trade-off**: Exact correctness vs. approximate speed.

**Optimization**: Use batch operations and deferred reconstruction patterns (see [FFI_BRIDGE_ANALYSIS.md](FFI_BRIDGE_ANALYSIS.md)).

### Q: Can I convert existing float-based code?

**A**: Yes, but requires refactoring:

1. Replace float literals with `QMNFRational(numerator, denominator)`
2. Use `DataBoundary.float_to_rational()` at input boundaries
3. Convert to float only for output/display

See [COMMON_TASKS_COOKBOOK.md](COMMON_TASKS_COOKBOOK.md) Section 8 for migration guide.

### Q: What if I need transcendental functions (sin, cos, exp)?

**A**: QMNF provides exact rational approximations via Taylor series:

```python
from hcvlang import transcendental_sin, transcendental_cos

# Exact rational sine (Taylor series expansion)
x = QMNFRational(1, 4)  # Input
sin_x = transcendental_sin(x, terms=20)  # 20-term Taylor series
```

See [INTEGRATION_QUICK_REFERENCE.md](INTEGRATION_QUICK_REFERENCE.md) for complete API.

---

## Quick Command Reference

```bash
# Build and verify
python3 setup.py build_rust --release --inplace
python3 -c "import hcvlang_pyo3; print('OK')"

# Run tests
pytest tests/python/test_suite.py -v

# Run benchmarks
python3 milestone_benchmark.py

# Check for float violations
python3 tools/check_no_floats.py

# Validate installation
python3 tools/verify_installation.py

# Get help
python3 -c "from qmnf import QMNFRational; help(QMNFRational)"
```

---

## Troubleshooting

### Import Error: `ModuleNotFoundError: hcvlang_pyo3`

**Solution**: Rebuild Rust library
```bash
cd hcvlang
cargo clean
cd ..
python3 setup.py build_rust --release --inplace
```

### Type Error: `Expected int, got float`

**Solution**: Use DataBoundary for conversion
```python
from qmnf.conversion_boundary import DataBoundary
r = DataBoundary.float_to_rational(3.14, precision=5)
```

### Slow Performance

**Solution**: Use batch operations
```python
from hcvlang import batch_add_rational
results = batch_add_rational(inputs_a, inputs_b)  # 4-8× faster
```

**Full Troubleshooting**: [TROUBLESHOOTING_PLAYBOOK.md](TROUBLESHOOTING_PLAYBOOK.md)

---

## Summary

**You Now Know**:
- ✅ How to install QMNF
- ✅ How to create exact rationals
- ✅ How to perform arithmetic operations
- ✅ Why integer-only matters
- ✅ Basic performance characteristics

**Time Invested**: ~5-10 minutes

**Next Action**: Choose your persona guide from [ONBOARDING_PATHWAY.md](ONBOARDING_PATHWAY.md)

---

## Resources

- **Complete Onboarding**: [ONBOARDING_PATHWAY.md](ONBOARDING_PATHWAY.md)
- **API Reference**: [INTEGRATION_QUICK_REFERENCE.md](INTEGRATION_QUICK_REFERENCE.md)
- **Common Tasks**: [COMMON_TASKS_COOKBOOK.md](COMMON_TASKS_COOKBOOK.md)
- **Troubleshooting**: [TROUBLESHOOTING_PLAYBOOK.md](TROUBLESHOOTING_PLAYBOOK.md)
- **Architecture**: [SYSTEM_DEVELOPER_GUIDE.md](SYSTEM_DEVELOPER_GUIDE.md)

---

**Last Updated**: 2025-11-16
**Maintainer**: Anthony Diaz (founder@hackfate.us)
**License**: Proprietary - See LICENSE file

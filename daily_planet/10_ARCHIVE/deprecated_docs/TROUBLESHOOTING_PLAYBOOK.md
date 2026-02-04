# QMNF Troubleshooting Playbook

**Problem → Solution Mapping for Common Issues**

**Last Updated**: 2025-11-16

---

## Table of Contents

1. [Installation Issues](#section-1-installation-issues)
2. [Performance Issues](#section-2-performance-issues)
3. [Type and Validation Errors](#section-3-type-and-validation-errors)
4. [Test Failures](#section-4-test-failures)
5. [Integration Issues](#section-5-integration-issues)
6. [Deployment Issues](#section-6-deployment-issues)
7. [Cryptography (FHE) Issues](#section-7-cryptography-fhe-issues)
8. [Neural Network Issues](#section-8-neural-network-issues)

---

## Section 1: Installation Issues

### 1.1 Problem: `ModuleNotFoundError: No module named 'hcvlang_pyo3'`

**Symptoms**:
```python
>>> import hcvlang_pyo3
ModuleNotFoundError: No module named 'hcvlang_pyo3'
```

**Root Cause**: Rust library not built or environment variables not set.

**Solution**:

**Step 1**: Build Rust library
```bash
cd QMNF_System
python3 setup.py build_rust --release --inplace
```

**Step 2**: Set environment variables
```bash
export LD_LIBRARY_PATH=/path/to/QMNF_System:$LD_LIBRARY_PATH
export PYTHONPATH=/path/to/QMNF_System:$PYTHONPATH
```

**Step 3**: Verify
```bash
python3 -c "import hcvlang_pyo3; print('✓ OK')"
```

**Permanent Fix** (add to `~/.bashrc` or `~/.zshrc`):
```bash
export LD_LIBRARY_PATH="/path/to/QMNF_System:$LD_LIBRARY_PATH"
export PYTHONPATH="/path/to/QMNF_System:$PYTHONPATH"
```

---

### 1.2 Problem: Build Failure - `cargo: command not found`

**Symptoms**:
```
error: cargo: command not found
```

**Root Cause**: Rust not installed.

**Solution**:

**Step 1**: Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Step 2**: Verify installation
```bash
rustc --version
cargo --version
```

**Step 3**: Retry build
```bash
python3 setup.py build_rust --release --inplace
```

---

### 1.3 Problem: Build Failure - `setuptools-rust not found`

**Symptoms**:
```
ModuleNotFoundError: No module named 'setuptools_rust'
```

**Root Cause**: Build dependency missing.

**Solution**:
```bash
pip3 install setuptools setuptools-rust --break-system-packages
python3 setup.py build_rust --release --inplace
```

---

### 1.4 Problem: Build Succeeds but Import Fails

**Symptoms**:
```bash
$ python3 setup.py build_rust --release --inplace
# Build succeeds

$ python3 -c "import hcvlang_pyo3"
ModuleNotFoundError: No module named 'hcvlang_pyo3'
```

**Root Cause**: Built library not in Python path.

**Solution**:

**Step 1**: Locate built library
```bash
find . -name "hcvlang_pyo3*.so"
# Should find: ./hcvlang_pyo3.cpython-39-x86_64-linux-gnu.so (or similar)
```

**Step 2**: Check if in current directory
```bash
ls -la hcvlang_pyo3*.so
```

**Step 3**: Set PYTHONPATH to current directory
```bash
export PYTHONPATH=$(pwd):$PYTHONPATH
python3 -c "import hcvlang_pyo3; print('✓ OK')"
```

---

### 1.5 Problem: Permission Denied During Build

**Symptoms**:
```
PermissionError: [Errno 13] Permission denied: '/usr/local/lib/python3.9/...'
```

**Root Cause**: Trying to install to system Python.

**Solution**:

**Option 1**: Use `--break-system-packages` (recommended for local dev)
```bash
pip3 install setuptools setuptools-rust --break-system-packages
```

**Option 2**: Use virtual environment
```bash
python3 -m venv qmnf_env
source qmnf_env/bin/activate
pip install setuptools setuptools-rust
python3 setup.py build_rust --release --inplace
```

---

## Section 2: Performance Issues

### 2.1 Problem: Operations Are Slow

**Symptoms**: Operations take much longer than expected (>1ms for simple arithmetic).

**Root Cause**: Likely one of:
1. Using Python loops instead of batch operations
2. Excessive FFI boundary crossings
3. Debug build instead of release build

**Diagnosis**:

**Step 1**: Check if using release build
```bash
# Build was run with --release flag?
python3 setup.py build_rust --release --inplace
```

**Step 2**: Profile your code
```python
import time
from qmnf import QMNFRational

a = QMNFRational(22, 7)
b = QMNFRational(1, 3)

# Measure
n = 1000
start = time.perf_counter()
for _ in range(n):
    result = a + b
elapsed = time.perf_counter() - start

print(f"Time: {elapsed*1000:.2f}ms for {n} ops")
print(f"Avg: {(elapsed/n)*1_000_000:.2f}µs per op")

# Expected: <10µs per op on modern hardware
```

**Solution**:

**Fix 1**: Use batch operations
```python
# ❌ SLOW
results = [a + b for _ in range(1000)]

# ✅ FAST
from hcvlang import batch_add_rational
results = batch_add_rational([a]*1000, [b]*1000)
```

**Fix 2**: Rebuild with optimizations
```bash
cd hcvlang
cargo clean
cargo build --release --features "fast-paths,simd,parallel"
```

**Fix 3**: Minimize Python-Rust boundaries
```python
# ❌ SLOW: Crossing boundary in loop
for i in range(1000):
    intermediate = rust_func(data)
    result = another_rust_func(intermediate)

# ✅ FAST: Single boundary crossing
result = combined_rust_func(data, iterations=1000)
```

---

### 2.2 Problem: Memory Usage Growing

**Symptoms**: Memory usage grows over time, potential leak.

**Root Cause**:
1. Creating many temporary QMNFRational objects
2. Not releasing CRTBigInt handles

**Solution**:

**Fix 1**: Reuse objects
```python
# ❌ BAD: Creating new objects in loop
for i in range(1000000):
    r = QMNFRational(i, i+1)
    # Use r

# ✅ GOOD: Reuse where possible
multiplier = QMNFRational(99, 100)
for i in range(1000000):
    result = value * multiplier  # Reuse multiplier
```

**Fix 2**: Use context managers (if implemented)
```python
with CRTBigInt(12345) as a:
    result = a + CRTBigInt(67890)
    # Automatic cleanup
```

**Fix 3**: Explicit cleanup (Python garbage collection)
```python
import gc

# Force garbage collection periodically
for i in range(1000000):
    # ... operations ...
    if i % 10000 == 0:
        gc.collect()
```

---

### 2.3 Problem: CPU Usage at 100% for Single Operation

**Symptoms**: Single arithmetic operation maxes out CPU.

**Root Cause**: Large integer overflow into HCVLangBigInt (O(n²) operations).

**Diagnosis**:
```python
from qmnf import QMNFRational

# Check magnitude
r = QMNFRational(huge_numerator, huge_denominator)
print(f"Numerator bits: {r.numerator().bit_length()}")
print(f"Denominator bits: {r.denominator().bit_length()}")

# If >126 bits, using HCVLangBigInt (slower)
```

**Solution**:

**Fix 1**: Use CRTBigInt for bounded computations
```python
from hcvlang import CRTBigInt

# Bounded integers (fast, ~120ns)
a = CRTBigInt(12345)  # Within ±2^126
```

**Fix 2**: Reduce precision
```python
from qmnf.conversion_boundary import DataBoundary

# Instead of full precision
pi_high = DataBoundary.float_to_rational(3.14159265358979, precision=15)

# Use reasonable precision
pi_reasonable = DataBoundary.float_to_rational(3.14159, precision=5)
```

---

## Section 3: Type and Validation Errors

### 3.1 Problem: `TypeError: Expected int, got float`

**Symptoms**:
```python
>>> r = QMNFRational(3.14, 1)
TypeError: Expected int, got float
```

**Root Cause**: QMNFRational requires integer arguments.

**Solution**:
```python
# ❌ WRONG
r = QMNFRational(3.14, 1)

# ✅ CORRECT: Convert explicitly at boundary
from qmnf.conversion_boundary import DataBoundary

pi_data = DataBoundary.float_to_rational(3.14, precision=2)
r = QMNFRational(pi_data.numerator, pi_data.denominator)
```

---

### 3.2 Problem: `ValueError: denominator cannot be zero`

**Symptoms**:
```python
>>> r = QMNFRational(1, 0)
ValueError: denominator cannot be zero
```

**Root Cause**: Division by zero in rational creation.

**Solution**:
```python
# Add validation
def safe_rational(num, den):
    if den == 0:
        raise ValueError("Cannot create rational with zero denominator")
    return QMNFRational(num, den)

# Or use try-except
try:
    r = QMNFRational(num, den)
except ValueError as e:
    print(f"Error: {e}")
    # Handle gracefully
```

---

### 3.3 Problem: Float Violations Detected by `check_no_floats.py`

**Symptoms**:
```bash
$ python3 tools/check_no_floats.py
Found 5 float violations in qmnf/my_module.py
```

**Root Cause**: Float literals or operations in code.

**Solution**:

**Step 1**: Identify violations
```bash
python3 tools/check_no_floats.py --verbose
# Shows exact line numbers and violations
```

**Step 2**: Fix violations
```python
# ❌ VIOLATION: Float literal
result = x * 0.5

# ✅ FIX: Use rational
result = x * QMNFRational(1, 2)

# ❌ VIOLATION: Float division
result = x / 2.0

# ✅ FIX: Integer division or rational
result = x / QMNFRational(2, 1)
```

**Step 3**: Re-validate
```bash
python3 tools/check_no_floats.py
# Should report 0 violations
```

---

### 3.4 Problem: `AttributeError: 'QMNFRational' object has no attribute 'to_float'`

**Symptoms**:
```python
>>> r = QMNFRational(22, 7)
>>> r.to_float()
AttributeError: 'QMNFRational' object has no attribute 'to_float'
```

**Root Cause**: Incorrect method name.

**Solution**:
```python
# ❌ WRONG
f = r.to_float()

# ✅ CORRECT
f = float(r)  # Use Python built-in float()
```

---

## Section 4: Test Failures

### 4.1 Problem: Tests Fail with Import Errors

**Symptoms**:
```bash
$ pytest tests/
ImportError: cannot import name 'QMNFRational' from 'qmnf'
```

**Root Cause**: Environment not configured for tests.

**Solution**:
```bash
# Set environment before running tests
export LD_LIBRARY_PATH=$(pwd):$LD_LIBRARY_PATH
export PYTHONPATH=$(pwd):$PYTHONPATH

# Run tests
pytest tests/ -v
```

---

### 4.2 Problem: Tests Pass Locally but Fail in CI

**Symptoms**: Tests pass on local machine but fail in CI/CD pipeline.

**Root Cause**: Environment differences (Rust version, dependencies, etc.).

**Solution**:

**Step 1**: Match CI environment locally
```bash
# Use same Python version
python3.9 -m venv test_env
source test_env/bin/activate

# Install exact dependencies
pip install -r requirements.txt

# Build and test
python3 setup.py build_rust --release --inplace
pytest tests/ -v
```

**Step 2**: Update CI configuration
```yaml
# .github/workflows/test.yml
steps:
  - name: Install Rust
    uses: actions-rs/toolchain@v1
    with:
      toolchain: stable

  - name: Build Rust library
    run: python3 setup.py build_rust --release --inplace

  - name: Set environment
    run: |
      echo "LD_LIBRARY_PATH=$(pwd):$LD_LIBRARY_PATH" >> $GITHUB_ENV
      echo "PYTHONPATH=$(pwd):$PYTHONPATH" >> $GITHUB_ENV

  - name: Run tests
    run: pytest tests/ -v
```

---

### 4.3 Problem: Arithmetic Tests Fail with Wrong Results

**Symptoms**:
```
AssertionError: Expected 5/6, got 7/6
```

**Root Cause**: Logic error or incorrect expected value.

**Solution**:

**Step 1**: Debug manually
```python
from qmnf import QMNFRational

a = QMNFRational(1, 2)
b = QMNFRational(1, 3)
result = a + b

print(f"a = {a.numerator()}/{a.denominator()}")
print(f"b = {b.numerator()}/{b.denominator()}")
print(f"result = {result.numerator()}/{result.denominator()}")

# Verify manually: 1/2 + 1/3 = 3/6 + 2/6 = 5/6
```

**Step 2**: Check test logic
```python
# Verify test expectation is correct
# 1/2 + 1/3 should be 5/6, not 7/6
assert result.numerator() == 5
assert result.denominator() == 6
```

---

## Section 5: Integration Issues

### 5.1 Problem: NumPy Integration Precision Loss

**Symptoms**: Converting QMNF → NumPy → QMNF loses precision.

**Root Cause**: NumPy uses float64 (64-bit floats with limited precision).

**Solution**:

**Best Practice**: Only convert at output boundaries
```python
# ❌ BAD: Lossy round-trip
qmnf_value = QMNFRational(22, 7)
numpy_value = float(qmnf_value)  # Precision loss here
back_to_qmnf = QMNFRational.from_float(numpy_value)  # Can't recover exact value

# ✅ GOOD: Keep in QMNF, convert only for output
qmnf_results = [compute(qmnf_val) for qmnf_val in qmnf_values]
# Convert to NumPy only for visualization
numpy_approx = np.array([float(r) for r in qmnf_results])
```

---

### 5.2 Problem: Pandas DataFrame Errors

**Symptoms**:
```python
TypeError: Cannot convert QMNFRational to float64
```

**Root Cause**: Pandas doesn't natively support QMNFRational.

**Solution**:

**Option 1**: Store as object dtype
```python
import pandas as pd
from qmnf import QMNFRational

df = pd.DataFrame({
    'value': [QMNFRational(1, 2), QMNFRational(1, 3)]
}, dtype=object)
```

**Option 2**: Store numerator/denominator separately
```python
df = pd.DataFrame({
    'numerator': [1, 1],
    'denominator': [2, 3]
})

# Reconstruct when needed
df['rational'] = df.apply(
    lambda row: QMNFRational(row['numerator'], row['denominator']),
    axis=1
)
```

---

### 5.3 Problem: REST API JSON Serialization Error

**Symptoms**:
```python
TypeError: Object of type QMNFRational is not JSON serializable
```

**Root Cause**: JSON doesn't support custom types.

**Solution**:

**Fix**: Convert to dict before serialization
```python
from flask import Flask, jsonify
from qmnf import QMNFRational

app = Flask(__name__)

@app.route('/compute')
def compute():
    result = QMNFRational(22, 7)

    # ❌ WRONG: Direct serialization
    # return jsonify(result)  # Error!

    # ✅ CORRECT: Convert to dict
    return jsonify({
        'numerator': result.numerator(),
        'denominator': result.denominator(),
        'decimal': float(result)
    })
```

---

## Section 6: Deployment Issues

### 6.1 Problem: Application Crashes on Start in Production

**Symptoms**:
```
Segmentation fault (core dumped)
```

**Root Cause**: Shared library not found or version mismatch.

**Diagnosis**:
```bash
# Check library
ldd /path/to/hcvlang_pyo3.so

# Check environment
echo $LD_LIBRARY_PATH

# Check library location
find /opt/qmnf -name "*.so"
```

**Solution**:
```bash
# Set LD_LIBRARY_PATH in systemd service
# /etc/systemd/system/qmnf-app.service

[Service]
Environment="LD_LIBRARY_PATH=/opt/qmnf:/usr/local/lib"
Environment="PYTHONPATH=/opt/qmnf"
ExecStart=/usr/bin/python3 /opt/qmnf/app.py

# Reload and restart
sudo systemctl daemon-reload
sudo systemctl restart qmnf-app
```

---

### 6.2 Problem: Different Results in Development vs. Production

**Symptoms**: Same code produces different results in different environments.

**Root Cause**: Different Rust build configurations (debug vs. release).

**Solution**:

**Ensure release build in production**:
```bash
# Production build
cd hcvlang
cargo build --release --features "fast-paths,simd,parallel"

# Copy release library
sudo cp target/release/libhcvlang.so /opt/qmnf/
```

**Verify**:
```bash
# Check build type
strings /opt/qmnf/libhcvlang.so | grep -i "debug\|release"
```

---

### 6.3 Problem: Docker Container Can't Find Rust Library

**Symptoms**:
```
ModuleNotFoundError: No module named 'hcvlang_pyo3'
```

**Root Cause**: Library not copied to container or environment not set.

**Solution**:

**Fix Dockerfile**:
```dockerfile
FROM rust:1.70 as builder

# Build
WORKDIR /build
COPY . .
RUN cd hcvlang && cargo build --release

# Runtime
FROM python:3.9-slim

# Copy built library
COPY --from=builder /build/hcvlang/target/release/libhcvlang.so /opt/qmnf/
COPY --from=builder /build/hcvlang_pyo3*.so /opt/qmnf/

# Set environment
ENV LD_LIBRARY_PATH=/opt/qmnf:$LD_LIBRARY_PATH
ENV PYTHONPATH=/opt/qmnf:$PYTHONPATH

CMD ["python3", "/opt/qmnf/app.py"]
```

---

## Section 7: Cryptography (FHE) Issues

### 7.1 Problem: FHE Encryption Very Slow

**Symptoms**: Encryption takes >10ms per operation.

**Root Cause**: Not using real-time FHE variant or batch operations.

**Solution**:

**Fix 1**: Use real-time FHE
```python
# ❌ SLOW: Standard FHE
from hcvlang import FHEContext, SecurityLevel
ctx = FHEContext(SecurityLevel(1))  # 2-5ms encryption

# ✅ FAST: Real-time FHE
from hcvlang import RealTimeFHEContext
ctx = RealTimeFHEContext(SecurityLevel(1))  # <1ms encryption
```

**Fix 2**: Use batch operations
```python
from hcvlang import BatchFHEProcessor, BatchConfig

config = BatchConfig(batch_size=256, parallel_enabled=True)
processor = BatchFHEProcessor(config)

# 8× faster on 8-core CPU
ciphertexts = processor.batch_encrypt(plaintexts, pk, ctx)
```

---

### 7.2 Problem: FHE Decryption Returns Wrong Value

**Symptoms**: Encrypted 42, decrypted 0 or garbage.

**Root Cause**: Noise exceeded threshold (ciphertext corrupted).

**Diagnosis**:
```python
from hcvlang import FHEContext, SecurityLevel

ctx = FHEContext(SecurityLevel(1))
sk, pk = ctx.generate_keypair()

ct = ctx.encrypt(ctx.encode(42), pk)

# Check noise budget
noise_tracker = ctx.get_noise_tracker(ct)
print(f"Noise budget: {noise_tracker.remaining_bits()} bits")

# If <10 bits, ciphertext may be corrupted
```

**Solution**:

**Fix 1**: Reduce operations on ciphertext
```python
# ❌ BAD: Too many operations
ct = ctx.encrypt(ctx.encode(1), pk)
for _ in range(100):
    ct = ctx.mul(ct, ctx.encrypt(ctx.encode(2), pk))  # Noise grows exponentially
result = ctx.decrypt(ct, sk)  # Corrupted

# ✅ GOOD: Fewer operations or bootstrap
ct = ctx.encrypt(ctx.encode(1), pk)
ct = ctx.mul(ct, ctx.encrypt(ctx.encode(2), pk))
result = ctx.decrypt(ct, sk)  # OK
```

**Fix 2**: Use higher security level (larger parameters)
```python
# Larger parameters = more noise tolerance
ctx = FHEContext(SecurityLevel(2))  # 192-bit security
```

---

## Section 8: Neural Network Issues

### 8.1 Problem: Neural Network Training Not Converging

**Symptoms**: Loss remains high or NaN after many epochs.

**Root Cause**: Learning rate too high or gradients exploding with rational arithmetic.

**Solution**:

**Fix 1**: Reduce learning rate
```python
from qmnf import QMNFRational
from qmnf.neural.hpo import HiveGSO

# ❌ TOO HIGH
optimizer = HiveGSO(model.parameters(), learning_rate=QMNFRational(1, 10))

# ✅ REASONABLE
optimizer = HiveGSO(model.parameters(), learning_rate=QMNFRational(1, 1000))
```

**Fix 2**: Gradient clipping
```python
def clip_gradients(gradients, max_norm):
    """Clip gradients to prevent explosion."""
    norm = sum(g ** 2 for g in gradients)
    if norm > max_norm ** 2:
        scale = max_norm / norm
        gradients = [g * scale for g in gradients]
    return gradients
```

---

### 8.2 Problem: Neural Network Inference Slow

**Symptoms**: Forward pass takes >100ms for small network.

**Root Cause**: Not using batch operations or SIMD optimizations.

**Solution**:

**See**: [GETTING_STARTED_DATA_SCIENTIST.md](GETTING_STARTED_DATA_SCIENTIST.md) for neural network optimization.

---

## Quick Diagnostic Checklist

When encountering any issue:

- [ ] **Environment set?** Check `echo $LD_LIBRARY_PATH $PYTHONPATH`
- [ ] **Library built?** Check `ls hcvlang_pyo3*.so`
- [ ] **Release build?** Used `--release` flag?
- [ ] **Import works?** `python3 -c "import hcvlang_pyo3"`
- [ ] **Tests pass?** `pytest tests/ -v`
- [ ] **Float violations?** `python3 tools/check_no_floats.py`
- [ ] **Performance baseline?** `python3 milestone_benchmark.py`

---

## Still Stuck?

1. **Run diagnostics**:
   ```bash
   python3 tools/verify_installation.py
   python3 tools/test_basic_operations.py
   ```

2. **Check logs**:
   ```bash
   # Enable debug logging
   export QMNF_LOG_LEVEL=DEBUG
   python3 your_script.py 2>&1 | tee debug.log
   ```

3. **Get help**:
   - Email: founder@hackfate.us
   - GitHub Issues: https://github.com/Skyelabz210/QMNF_System/issues
   - Documentation: [SYSTEM_DEVELOPER_GUIDE.md](SYSTEM_DEVELOPER_GUIDE.md)

---

**Last Updated**: 2025-11-16
**Maintainer**: Anthony Diaz (founder@hackfate.us)
**License**: Proprietary - See LICENSE file

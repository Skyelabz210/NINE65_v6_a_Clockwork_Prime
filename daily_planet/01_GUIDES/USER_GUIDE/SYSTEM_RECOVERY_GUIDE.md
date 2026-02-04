---
title: "System Recovery Guide"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/SYSTEM_RECOVERY_GUIDE.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF System Recovery Guide

**Created**: 2025-11-06
**Purpose**: Emergency recovery procedures for QMNF System build and deployment issues
**Audience**: DevOps, System Administrators, Emergency Response

---

## 🚨 Quick Diagnosis

Run this command to check system health:
```bash
python3 -c "
import sys
print('Python:', sys.version)
try:
    import hcvlang_pyo3
    print('✓ Rust bindings available')
except ImportError:
    print('✗ Rust bindings MISSING - needs build')
"
```

**Status Codes**:
- ✓ Rust bindings available → System operational
- ✗ Rust bindings MISSING → Follow recovery procedures below

---

## Issue 1: Rust Build Failure (403 from crates.io)

###symptoms
```
error: failed to get `num-bigint` as a dependency
got 403 - Access denied
```

### Root Cause
Network access to crates.io is blocked or requires proxy configuration.

### Solution A: Configure Cargo Network (Preferred)

Create or edit `~/.cargo/config.toml`:
```toml
[net]
git-fetch-with-cli = true

[http]
proxy = "http://proxy.example.com:8080"  # If behind corporate proxy
check-revoke = false

[source.crates-io]
replace-with = "vendored-sources"  # Optional: use vendored deps

[source.vendored-sources]
directory = "/path/to/QMNF_System/vendor"
```

Then retry build:
```bash
cd /home/user/QMNF_System/hcvlang
cargo build --release --features python
```

### Solution B: Use Vendored Dependencies

If network access is completely unavailable:

1. **On a machine with internet access**, vendor the dependencies:
```bash
cd /path/to/QMNF_System
cargo vendor vendor/
```

2. Create `.cargo/config.toml` in project root:
```toml
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
```

3. Transfer the entire `vendor/` directory to the target machine

4. Build without network:
```bash
cd hcvlang
cargo build --release --features python --offline
```

### Solution C: Use Pre-built Binaries (Emergency)

If building is not possible, use pre-built wheels:

1. Request pre-built `.so` files from the build server
2. Place in: `/home/user/QMNF_System/hcvlang/target/release/`
3. Create symlink:
```bash
cd /home/user/QMNF_System
ln -s hcvlang/target/release/libhcvlang_pyo3.so hcvlang_pyo3.so
```

4. Add to Python path:
```bash
export PYTHONPATH="/home/user/QMNF_System:$PYTHONPATH"
```

### Verification
```bash
python3 -c "import hcvlang_pyo3; print('Success!')"
```

---

## Issue 2: Missing Python Dependencies

### Symptoms
```
ModuleNotFoundError: No module named 'numpy'
ModuleNotFoundError: No module named 'pytest'
```

### Root Cause
Python dependencies not installed in current environment.

### Solution: Install Dependencies

#### Option A: Using pip (Standard)
```bash
# Core dependencies
pip3 install numpy scipy mpmath

# Development dependencies
pip3 install pytest pytest-cov psutil

# Build dependencies
pip3 install setuptools-rust maturin
```

#### Option B: Using requirements file
```bash
# If requirements.txt exists
pip3 install -r requirements.txt

# Or create one:
cat > requirements.txt <<EOF
numpy>=1.20.0
scipy>=1.7.0
mpmath>=1.2.0
pytest>=7.0.0
pytest-cov>=3.0.0
psutil>=5.8.0
setuptools-rust>=1.5.0
EOF

pip3 install -r requirements.txt
```

#### Option C: System packages (Debian/Ubuntu)
```bash
sudo apt-get update
sudo apt-get install -y \
    python3-numpy \
    python3-scipy \
    python3-pytest \
    python3-psutil
```

### Verification
```bash
python3 -c "
import numpy
import scipy
import mpmath
import pytest
import psutil
print('✓ All dependencies available')
"
```

---

## Issue 3: Import Errors After Successful Build

### Symptoms
```
ImportError: cannot import name 'Rational' from 'hcvlang_pyo3'
ModuleNotFoundError: No module named 'qmnf'
```

### Root Cause
Python cannot find the built modules or PYTHONPATH not set.

### Solution: Install Package

#### Option A: Development Install (Recommended)
```bash
cd /home/user/QMNF_System
python3 setup.py develop
```

This creates symlinks, allowing live code updates.

#### Option B: Editable Install (pip)
```bash
cd /home/user/QMNF_System
pip3 install -e .
```

#### Option C: Manual PYTHONPATH
```bash
export PYTHONPATH="/home/user/QMNF_System:$PYTHONPATH"
python3 -c "import qmnf; print('Success')"
```

Make permanent by adding to `~/.bashrc`:
```bash
echo 'export PYTHONPATH="/home/user/QMNF_System:$PYTHONPATH"' >> ~/.bashrc
source ~/.bashrc
```

### Verification
```bash
python3 -c "
from qmnf import QMNFRational
from qmnf.boundary import CompleteExactPoint
print('✓ QMNF modules importable')
"
```

---

## Issue 4: Test Suite Failures

### Symptoms
```
tests/python/test_suite.py::test_rational - FAILED
E   ModuleNotFoundError: No module named 'hcvlang_pyo3'
```

### Solution: Complete Installation Check

Run this diagnostic script:
```bash
cd /home/user/QMNF_System
python3 << 'EOF'
import sys
print("=== QMNF Installation Diagnostic ===\n")

# 1. Python version
print(f"Python: {sys.version}")

# 2. Check Rust bindings
try:
    import hcvlang_pyo3
    print("✓ hcvlang_pyo3 available")
except ImportError as e:
    print(f"✗ hcvlang_pyo3 MISSING: {e}")

# 3. Check QMNF modules
try:
    import qmnf
    print("✓ qmnf module available")
except ImportError as e:
    print(f"✗ qmnf MISSING: {e}")

# 4. Check dependencies
for mod in ['numpy', 'scipy', 'mpmath', 'pytest']:
    try:
        __import__(mod)
        print(f"✓ {mod} available")
    except ImportError:
        print(f"✗ {mod} MISSING")

print("\nIf any items are MISSING, follow recovery procedures.")
EOF
```

If all checks pass but tests still fail:
```bash
# Rebuild everything from scratch
cd /home/user/QMNF_System
rm -rf hcvlang/target/
rm -rf build/ dist/ *.egg-info
python3 setup.py develop
python3 -m pytest tests/ -v
```

---

## Issue 5: Performance Degradation

### Symptoms
- Benchmarks running 10x+ slower than expected
- Operations taking milliseconds instead of microseconds

### Diagnosis
```bash
# Check if release build was used
file hcvlang/target/release/libhcvlang_pyo3.so

# Should show "not stripped" for release build
# If debug build, will be much larger (10-50MB vs 2-5MB)
```

### Solution: Ensure Release Build

```bash
cd /home/user/QMNF_System/hcvlang

# Clean debug artifacts
rm -rf target/debug/

# Build with release optimizations
cargo build --release --features python

# Verify size (release should be 2-5MB)
ls -lh target/release/libhcvlang_pyo3.so

# Reinstall
cd ..
python3 setup.py develop
```

### Verification
Run quick benchmark:
```bash
python3 -c "
from qmnf_boundary_fixed import QMNFRational
import time

start = time.perf_counter()
for i in range(10000):
    r = QMNFRational(i, i+1)
    _ = r + r
duration = time.perf_counter() - start

print(f'10k operations: {duration*1000:.1f}ms')
print(f'Expected: <100ms (release), >1000ms (debug)')
print('✓ PASS' if duration < 0.1 else '✗ FAIL - Check build mode')
"
```

---

## Issue 6: Documentation Out of Sync

### Symptoms
- References to `@guard_no_float` decorator (removed in Phase 1)
- References to `qmnf_guards.py` (deleted)
- References to `tools/check_no_floats.py` (restored)

### Solution: Update Documentation

Documentation update patches applied in this recovery session:
- ✓ `CLAUDE.md` updated to reflect Phase 1 architecture
- ✓ `tools/check_no_floats.py` recreated
- ✓ `tools/boundary_validator.py` created
- ✓ `tools/compare_benchmarks.py` created

To verify:
```bash
# Check tools exist
ls -lh tools/*.py

# Should show:
# - check_no_floats.py (new)
# - boundary_validator.py (new)
# - compare_benchmarks.py (new)
# - generate_reference_values.py (existing)
# - qmnf_benchmark_suite.py (existing)
```

---

## Complete Recovery Procedure

For a completely broken system, follow these steps in order:

### Step 1: Install System Dependencies
```bash
# Debian/Ubuntu
sudo apt-get update
sudo apt-get install -y \
    python3 python3-pip \
    cargo rustc \
    build-essential \
    libssl-dev

# Verify
python3 --version  # Should be 3.8+
cargo --version    # Should be 1.70+
```

### Step 2: Install Python Dependencies
```bash
cd /home/user/QMNF_System
pip3 install numpy scipy mpmath pytest pytest-cov psutil setuptools-rust
```

### Step 3: Build Rust Components
```bash
cd /home/user/QMNF_System/hcvlang

# If network available:
cargo build --release --features python

# If network blocked, use vendored deps (see Issue 1)
```

### Step 4: Install Python Package
```bash
cd /home/user/QMNF_System
python3 setup.py develop
```

### Step 5: Verify Installation
```bash
# Test imports
python3 -c "import hcvlang_pyo3, qmnf; print('✓ Success')"

# Run a simple test
python3 -c "
from qmnf_boundary_fixed import QMNFRational
r = QMNFRational(22, 7)
print(f'π ≈ {r}')
print('✓ System operational')
"
```

### Step 6: Run Test Suite
```bash
cd /home/user/QMNF_System
python3 -m pytest tests/ -v --maxfail=5
```

### Step 7: Run Benchmark
```bash
python3 milestone_benchmark.py
```

---

## Emergency Contacts

If recovery procedures fail:

1. **Check logs**: Look for detailed error messages in build output
2. **Review**: `PHASE_1_COMPLETION_REPORT.md` for architecture changes
3. **Consult**: `SYSTEM_DEVELOPER_GUIDE.md` for component details
4. **Verify**: System requirements (Python 3.8+, Rust 1.70+, 4GB RAM)

---

## Prevention

To prevent future issues:

### 1. Automate Validation
Add to CI/CD pipeline:
```bash
# Pre-commit checks
python3 tools/check_no_floats.py
python3 tools/boundary_validator.py

# Build verification
cargo test --release
python3 -m pytest tests/

# Benchmark regression check
python3 milestone_benchmark.py
python3 tools/compare_benchmarks.py baseline.json latest.json
```

### 2. Document Environment
Create `environment.txt`:
```bash
python3 --version > environment.txt
cargo --version >> environment.txt
pip3 freeze >> environment.txt
```

### 3. Vendor Dependencies
Keep vendored Cargo dependencies in version control:
```bash
cargo vendor vendor/
git add vendor/ .cargo/config.toml
```

### 4. Pre-built Binaries
Maintain pre-built wheels for emergency:
```bash
# On build server
python3 setup.py bdist_wheel
# Store wheel in artifact repository
```

---

## Success Criteria

System is fully operational when:
- ✅ `import hcvlang_pyo3` succeeds
- ✅ `import qmnf` succeeds
- ✅ Test suite passes (35+ tests)
- ✅ Benchmarks meet targets (>30k ops/sec)
- ✅ Float contamination scan reports 0 critical violations
- ✅ Boundary validation passes

---

**Last Updated**: 2025-11-06
**Version**: 1.0
**Status**: Active

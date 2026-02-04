# QMNF System - Installation Testing Guide

This guide helps you test the QMNF installation process to ensure it works correctly on a fresh system.

## Prerequisites

### Required
- **Python:** 3.9 or higher
- **Rust:** 1.70 or higher
- **Cargo:** Latest stable version
- **Git:** For cloning the repository

### Check Prerequisites
```bash
python3 --version  # Should be >= 3.9
rustc --version    # Should be >= 1.70
cargo --version    # Latest stable
```

### Install Prerequisites (if missing)

**Python 3.9+:**
```bash
# Debian/Ubuntu
sudo apt update && sudo apt install python3 python3-pip python3-venv

# macOS
brew install python@3.11

# Windows
# Download from https://www.python.org/downloads/
```

**Rust:**
```bash
# All platforms
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Installation Steps

### 1. Clone the Repository
```bash
git clone https://github.com/YOUR_ORG/QMNF_System.git
cd QMNF_System
```

### 2. Create Virtual Environment (Recommended)
```bash
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
```

### 3. Install QMNF
```bash
# Basic installation (Python only)
pip install -e .

# With dashboard support
pip install -e ".[dashboard]"

# With development tools
pip install -e ".[dev]"

# Everything
pip install -e ".[dev,dashboard]"
```

**What happens:**
- Rust components build automatically via `setuptools-rust`
- Python bindings are created
- Console scripts are installed (`qmnf-dashboard`, `qmnf-benchmark`)

**Expected build time:**
- Clean build: ~30-60 seconds
- Incremental: ~2-5 seconds

## Verification Tests

### Test 1: Python Package Import
```bash
python3 -c "import qmnf; print('✅ QMNF package imported successfully')"
```

**Expected output:**
```
✅ QMNF package imported successfully
```

### Test 2: Rust Bindings Import
```bash
python3 -c "from hcvlang_pyo3 import CRTBigInt; print(f'✅ Rust bindings work: CRTBigInt(42) = {CRTBigInt(42)}')"
```

**Expected output:**
```
✅ Rust bindings work: CRTBigInt(42) = 42
```

### Test 3: QMNFRational (Core Functionality)
```bash
python3 << 'EOF'
from qmnf.api import QMNFRational

# Test basic rational arithmetic
a = QMNFRational(22, 7)  # π approximation
b = QMNFRational(1, 3)
result = a * b

print(f"✅ QMNFRational test:")
print(f"  (22/7) * (1/3) = {result}")
print(f"  Exact result: {result.numerator}/{result.denominator}")
EOF
```

**Expected output:**
```
✅ QMNFRational test:
  (22/7) * (1/3) = 22/21
  Exact result: 22/21
```

### Test 4: Console Scripts
```bash
# Check if console scripts are installed
which qmnf-dashboard
which qmnf-benchmark
```

**Expected:** Paths to the installed scripts

### Test 5: Run Benchmark
```bash
# Quick benchmark test (should complete in < 30 seconds)
python3 -c "
from milestone_benchmark import run_quick_test
results = run_quick_test()
print(f'✅ Benchmark test passed: {len(results)} results')
"
```

### Test 6: Dashboard (Optional - if dashboard dependencies installed)
```bash
# Start dashboard (Ctrl+C to stop)
qmnf-dashboard --host 127.0.0.1 --port 8080
```

**Then open:** http://localhost:8080

**Expected:** Dashboard homepage loads

## Common Issues & Solutions

### Issue: "No module named 'setuptools'"
**Solution:**
```bash
pip install --upgrade setuptools wheel setuptools-rust
```

### Issue: Rust build fails
**Solution:**
```bash
# Update Rust
rustup update stable

# Clean and rebuild
cd hcvlang
cargo clean
cargo build --release
cd ..
pip install -e . --no-build-isolation
```

### Issue: "ImportError: cannot import name 'QMNFRational'"
**Solution:**
```bash
# Check if you're in the right directory
pwd  # Should be QMNF_System/

# Try explicit path
python3 -c "import sys; sys.path.insert(0, '.'); import qmnf; print(qmnf.__file__)"
```

### Issue: Dashboard dependencies missing
**Solution:**
```bash
pip install flask flask-cors plotly
# Or: pip install -e ".[dashboard]"
```

## Performance Expectations

### Benchmark Results (Reference)
On a modern system (4-core CPU, 8GB RAM):

| Operation | Expected Performance |
|-----------|---------------------|
| Rational arithmetic | > 30,000 ops/sec |
| Geometric operations | > 20,000 ops/sec |
| GCD intensive | > 70,000 ops/sec |

### Memory Usage
- **Idle:** ~50MB
- **During benchmarks:** ~200-500MB
- **Dashboard running:** ~100-150MB

## Uninstallation

```bash
# If installed in venv, just delete it
deactivate
rm -rf venv

# If installed system-wide
pip uninstall qmnf
```

## Next Steps

After successful installation:
1. Read [README.md](README.md) for project overview
2. Check [docs/](docs/) for detailed documentation
3. Explore [examples/](examples/) for usage examples
4. Run full test suite: `pytest tests/`

## Getting Help

- **Issues:** https://github.com/YOUR_ORG/QMNF_System/issues
- **Discussions:** https://github.com/YOUR_ORG/QMNF_System/discussions
- **Email:** founder@hackfate.us

---

**Last Updated:** 2025-11-10
**QMNF Version:** 1.0.0

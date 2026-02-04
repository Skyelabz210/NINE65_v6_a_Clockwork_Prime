# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

This is a **multi-project workspace** containing the QMNF (Quantum-Modular Numerical Framework) ecosystem - a revolutionary integer-only computational arithmetic system with bootstrap-free FHE. The primary project is `QMNF_System/`.

### Key Projects

| Directory | Purpose |
|-----------|---------|
| `QMNF_System/` | Main system - Rust core + Python bindings |
| `NINE65/` | FHE innovations with Coq/Lean4 formal proofs |
| `daily_planet/` | Documentation repository (separated from source) |
| `exact_transcendentals/` | Exact transcendental function implementations |

## Build Commands

### Rust Core (hcvlang)
```bash
cd QMNF_System/hcvlang
cargo build --release          # Build optimized (~15s, 0 errors expected)
cargo test --lib --release     # Run tests (480+/510 passing)
cargo bench                    # Run benchmarks
```

### Python Package
```bash
cd QMNF_System
pip install -e ".[dev]"        # Development install
python -c "import hcvlang_pyo3"  # Verify FFI works
```

### Pre-commit Validation
```bash
python3 tools/check_no_floats.py          # Scan for float violations
python3 tools/check_no_floats_rust.py     # Rust float check
cargo build --release && cargo test --lib --release
```

## Testing

```bash
# Python tests
pytest tests/ -v
pytest tests/python/test_suite.py -v
pytest tests/python/fhe_comprehensive_test.py -v

# Rust tests
cd hcvlang && cargo test

# Single test file
cargo test --test arithmetic_learning_tests

# With coverage
pytest tests/ --cov=qmnf --cov-report=html
```

## Architecture

### Two-Layer Integer Arithmetic
1. **CRTBigInt** (Layer 1): Fast bounded integers via Chinese Remainder Theorem
   - Range: ±2^126 (two 63-bit primes)
   - Speed: ~120ns operations
   - Location: `hcvlang/src/crt_bigint.rs`

2. **HCVLangBigInt** (Layer 2): Arbitrary-precision integers
   - Range: Unlimited (memory-bound only)
   - Location: `hcvlang/src/bigint_hcv.rs`

### Core Rust Modules (`hcvlang/src/`)
- `modint.rs` - Mersenne prime modular arithmetic
- `rational.rs` - Exact rational number arithmetic (QMNFRational)
- `fused_piggyback_division.rs` - Revolutionary division algorithm (40x faster)
- `fhe/` - Bootstrap-free FHE implementation
- `ffi.rs` - PyO3 Python bindings (exports as `hcvlang_pyo3`)

### Python Layer (`qmnf/`)
- `api.py` - Public API
- `qmnf_bridge.py` - Rust FFI bridge
- `neural_residue.py` - Residue-space neural networks

### Cryptographic Systems
8 FHE systems in `cryptographic_systems/`:
- `01_BFV_Core_FHE/` through `08_ACC_Cryptosystem/`

## Critical Rules

### Integer-Only Mandate
**All computations must use exact integer/rational arithmetic. No floating-point.**

```python
# FORBIDDEN
x = 3.14159
y = math.sqrt(x)

# REQUIRED
from qmnf.api import QMNFRational
x = QMNFRational(314159, 100000)
z = x.sqrt()  # Exact or error-bounded
```

Rust enforces this via `#![deny(clippy::float_arithmetic)]` in `hcvlang/`.

### PyO3 Module Naming
The FFI module **must** be named `hcvlang_pyo3`:
- Rust: `#[pymodule] fn hcvlang_pyo3(...)`
- Python: `import hcvlang_pyo3`
- Changing this name breaks the build.

### Documentation Location
- Source code only in `QMNF_System/`
- All docs/reports in `daily_planet/` (separate repo)
- Guides: `daily_planet/01_GUIDES/`
- API refs: `daily_planet/03_API_REFERENCE/`

## Key Algorithms

### Fused Piggyback Division (FPD)
Solves 70-year RNS division bottleneck:
- Traditional: O(k²) full CRT reconstruction
- FPD: O(k) via anchor-first computation
- Location: `hcvlang/src/fused_piggyback_division.rs`

### K-Free CRT (PLMG)
100% exact division without k-tracking overhead:
- Core: `hcvlang/src/plmg_core.rs`
- K-Free: `hcvlang/src/kfree_crt.rs`
- Formula: `k = (x_R - x_P) · C_P^(-1) mod C_R`

## Performance Targets

| Operation | Target | Typical |
|-----------|--------|---------|
| ModInt add/sub | <50ns | ~2-3ns |
| ModInt multiply | <50ns | ~3ns |
| CRTBigInt ops | <500ns | ~50ns |
| FHE encrypt | <1ms | <1ms |
| FHE multiply | <500µs | <500µs |

## Tools

```bash
tools/check_no_floats.py       # Float detection
tools/qmnf_benchmark_suite.py  # Comprehensive benchmarks
tools/verify_installation.py   # Installation check
tools/gap_scanner.py           # Find implementation gaps
```

## Workspace Structure

```
QMNF_System/
├── hcvlang/              # Rust core library
│   ├── src/              # Source (50+ modules)
│   ├── benches/          # Criterion benchmarks
│   └── tests/            # Rust tests
├── qmnf/                 # Python package
├── crates/               # Modular Rust crates
│   ├── qmnf-arithmetic/
│   ├── qmnf-fhe/
│   ├── qmnf-neural/
│   └── ...
├── cryptographic_systems/ # 8 FHE implementations
├── tests/                # Integration tests
└── tools/                # Dev utilities
```

## Common Issues

### Import Error: `hcvlang_pyo3`
Ensure PyO3 module name matches exactly. Rebuild with `pip install -e .`

### Float Violations
Run `python3 tools/check_no_floats.py` before commits. Use `QMNFRational` for all numeric operations.

### Compilation Warnings
~15 non-critical warnings are expected. Zero errors required.

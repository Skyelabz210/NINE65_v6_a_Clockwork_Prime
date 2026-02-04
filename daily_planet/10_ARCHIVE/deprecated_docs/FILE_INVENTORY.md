# QMNF File Inventory

**Navigation guide for ~810,000 lines across 1,211 source files**

**Last Updated**: 2025-11-13

⚠️ **NOTE**: See `NESTED_DIRECTORY_AUDIT.md` for information about directory structure issues.

---

## Quick Navigation Index

| Jump To | Description |
|---------|-------------|
| [Top-Level Structure](#top-level-project-structure) | Root directory overview |
| [Rust Modules](#rust-modules-hcvlang) | All 70+ Rust source files |
| [Python Framework](#python-framework-qmnf) | Python module catalog |
| [Arithmetic Modules](#arithmetic--math-modules-58) | 58+ specialized math modules |
| [Testing Infrastructure](#testing-infrastructure) | Test suites and benchmarks |
| [Development Tools](#development-tools) | Code quality and validation |
| [Documentation](#documentation-structure) | Guides and references |
| [Common Tasks](#common-navigation-patterns) | Quick task guides |

---

## Top-Level Project Structure

```
QMNF_System/
├── hcvlang/               # Rust high-performance primitives (334K lines, 727 files)
├── qmnf/                  # Python framework (219K lines, 447 files)
├── tests/                 # Test suites (~60K lines)
├── tools/                 # Development utilities
├── docs/                  # Documentation (~258K lines)
├── benchmarks/            # Performance benchmarking
├── examples/              # Usage demonstrations
├── LICENSE                # Proprietary license
├── README.md              # Project overview
├── PROJECT_METRICS.md     # Codebase statistics
├── FILE_INVENTORY.md      # This file - navigation guide
├── NESTED_DIRECTORY_AUDIT.md  # Directory structure audit
├── SYSTEM_DEVELOPER_GUIDE.md  # Complete architecture reference
└── INTEGRATION_QUICK_REFERENCE.md  # Component API reference
```

---

## Rust Modules (hcvlang/)

**Total**: 334,404 lines across 727 files

### Core Integer Arithmetic

| File | Purpose |
|------|---------|
| `src/crt_bigint.rs` | Chinese Remainder Theorem integers (±2^126, ~120ns ops) |
| `src/bigint_hcv.rs` | Arbitrary-precision integers (unlimited scale) |
| `src/adaptive_crt_bigint.rs` | Base adaptive precision (2-8 primes) |
| `src/adaptive_crt_bigint_v1.rs` | Enhanced adaptive (dynamic tier selection) |
| `src/adaptive_crt_bigint_v2.rs` | Precision with overflow detection |
| `src/adaptive_crt_bigint_v3.rs` | 7-tier precision architecture (2-32 primes) |
| `src/modint.rs` | Mersenne prime modular arithmetic (2^31-1) |
| `src/modint_fast.rs` | Optimized modular operations |
| `src/rational.rs` | Exact rational number arithmetic |
| `src/mod_rational.rs` | Modular rational operations |

### FFI and Boundary Layer

| File | Purpose |
|------|---------|
| `src/ffi.rs` | Python-Rust FFI interface |
| `src/qmnf_ffi_boundary.rs` | Deferred reconstruction FFI (22× speedup) |
| `src/core_types.rs` | Shared type definitions |

### System Infrastructure

| File | Purpose |
|------|---------|
| `src/mana_orchestration.rs` | Runtime kernel (task scheduler, memory manager, firewall) |
| `src/double_helix.rs` | Dual-lane MAA execution engine |
| `src/attractor_memory.rs` | Attractor-based memory substrate |
| `src/swarm_gso.rs` | Galactic Swarm Optimization agents |
| `src/time_crystal.rs` | Temporal crystalline structures |
| `src/neural_primitives.rs` | Neural computation primitives |
| `src/storage/mod.rs` | HoloHD distributed storage layer |

### Mathematical Operations

| File | Purpose |
|------|---------|
| `src/apollonian.rs` | Apollonian circle generation & geometry |
| `src/qphi.rs` | Euler's totient function & modular properties |
| `src/fast_arithmetic.rs` | Optimized arithmetic primitives |
| `src/nnt.rs` | Number theoretic transforms |
| `src/geometric.rs` | 2D/3D geometric operations |
| `src/geom_point2d.rs` | 2D point arithmetic |
| `src/simd.rs` | SIMD-accelerated operations |
| `src/simd_distance.rs` | SIMD distance computations |

### Advanced Math Library (`src/math/`)

| File | Purpose |
|------|---------|
| `math/mod.rs` | Math module coordinator |
| `math/primes.rs` | Prime generation & testing |
| `math/number_theory.rs` | Number-theoretic functions |
| `math/polynomial.rs` | Polynomial arithmetic |
| `math/rational.rs` | Rational number operations |
| `math/rational_math.rs` | Advanced rational math |
| `math/big_rational.rs` | Arbitrary-precision rationals |
| `math/modular_advanced.rs` | Advanced modular arithmetic |
| `math/combinatorics.rs` | Combinatorial functions |
| `math/discrete.rs` | Discrete mathematics |
| `math/matrix.rs` | Matrix operations (integer-only) |
| `math/constants.rs` | Mathematical constants (rational approximations) |

### FHE Cryptography (`src/fhe/`)

| File | Purpose |
|------|---------|
| `fhe/mod.rs` | FHE module coordinator |
| `fhe/keys.rs` | Key generation & management |
| `fhe/encrypt.rs` | Encryption operations |
| `fhe/operations.rs` | Homomorphic operations (add, mul) |
| `fhe/polynomial.rs` | Polynomial ring operations |
| `fhe/rns.rs` | Residue Number System for FHE |
| `fhe/noise.rs` | Noise management |
| `fhe/qmnf_noise.rs` | QMNF-specific noise handling |
| `fhe/encoding.rs` | Plaintext encoding/decoding |
| `fhe/params.rs` | FHE parameters |
| `fhe/error.rs` | Error types |

### FHE Real-Time (`src/fhe_realtime/`)

| File | Purpose |
|------|---------|
| `fhe_realtime/mod.rs` | Real-time FHE coordinator |
| `fhe_realtime/realtime_context.rs` | Real-time execution context |
| `fhe_realtime/adaptive_polynomial.rs` | Adaptive polynomial operations |
| `fhe_realtime/noise_aware_tier.rs` | Noise-aware precision tiers |
| `fhe_realtime/batch_operations.rs` | Batched FHE operations |

---

## Python Framework (qmnf/)

**Total**: 218,720 lines across 447 files

### Top-Level Core Modules

| File | Purpose |
|------|---------|
| `api.py` | Public API surface (user-facing imports) |
| `boundary.py` | QMNFRational & boundary protection |
| `conversion_boundary.py` | Float→rational conversion layer |
| `core.py` | Core arithmetic operations |
| `core_optimized.py` | Optimized computation paths |
| `unified_qmnf.py` | Unified framework integration |
| `unified_config.py` | System-wide configuration |
| `harmonic_primitives.py` | Harmonic oscillation primitives |
| `qmnf_bridge.py` | Python-Rust bridge utilities |

### Arithmetic Framework (`qmnf/arithmetic/`)

- **`arithmetic/core/`** - Rational, integer, modular ops, GCD/LCM, prime utils
- **`arithmetic/field_theory/`** - Finite fields, Galois fields, field extensions
- **`arithmetic/geometry/`** - Point/line/circle ops, Apollonian gaskets
- **`arithmetic/polynomial/`** - Polynomial rings, interpolation, root finding
- **`arithmetic/sequences/`** - Fibonacci, Lucas, recurrence relations
- **`arithmetic/cryptographic/`** - Modular exp, discrete log, RSA primitives
- **`arithmetic/quantum/`** - Quantum-inspired modular ops, superposition
- **`arithmetic/optimization/`** - Integer descent, constraint solving
- **`arithmetic/calculus/`** - Finite differences, discrete integration
- **`arithmetic/validation/`** - Invariant checking, property testing

### Other Major Subsystems

- **`qmnf/neural/`** - Integer-only neural networks
- **`qmnf/crypto/`** - FHE core, ACC primitives
- **`qmnf/storage/`** - COSMOS & HoloDrive backends
- **`qmnf/cosmos_mana/`** - Memory orchestration (MANA runtime)
- **`qmnf/frameworks/sequences/`** - Deterministic sequencing engine
- **`qmnf/vsa/`** - Vector symbolic architectures
- **`qmnf/execution/`** - Execution domains (LinearCPU, SwarmEPRAM)
- **`qmnf/data/`** - Tensor ops, datasets, transforms
- **`qmnf/cognitive/`** - Reasoning primitives
- **`qmnf/noise/`** - Noise modeling for FHE

---

## Arithmetic & Math Modules (58+)

### By Category:

1. **Core Arithmetic** (12 modules): CRTBigInt, HCVLangBigInt, modular ops, rationals
2. **Adaptive Precision** (4 modules): Adaptive CRT variants v1-v3
3. **Number Theory** (8 modules): Primes, totient, coprime, discrete log
4. **Polynomial & Algebra** (6 modules): Polynomial rings, NNT
5. **Field Theory** (5 modules): Finite/Galois fields, RNS
6. **Geometry** (6 modules): Points, lines, circles, Apollonian
7. **Rational Arithmetic** (5 modules): QMNFRational, big rationals
8. **Sequences** (4 modules): Fibonacci, Lucas, recurrence, deterministic
9. **Combinatorics** (3 modules): Combinatorial functions, discrete math
10. **Advanced Modular** (4 modules): Fractal hierarchy, quantum superposition
11. **Matrix/Linear Algebra** (1 module): Integer matrices

**Total**: 58+ specialized modules

---

## Testing Infrastructure

```
tests/
├── python/              # Python integration tests
│   ├── test_suite.py
│   ├── fhe_comprehensive_test.py
│   ├── test_rational.py
│   └── ...
├── rust/                # Rust test wrappers
└── benchmarks/          # Benchmark harnesses
```

**Run Commands**:
```bash
# Rust tests
cd hcvlang && cargo test --release

# Python tests  
python3 -m pytest tests/python/test_suite.py -v

# Benchmarks
python3 tools/qmnf_benchmark_suite.py
cd hcvlang && cargo bench --release
```

---

## Development Tools (`tools/`)

| Tool | Purpose |
|------|---------|
| `check_no_floats.py` | **CRITICAL**: Detect floating-point contamination |
| `boundary_validator.py` | Validate QMNF boundary protection |
| `qmnf_benchmark_suite.py` | Comprehensive benchmark orchestration |
| `compare_benchmarks.py` | Compare benchmark results |
| `mathematical_discoverer.py` | Discover mathematical patterns |
| `check_doc_metadata.py` | Documentation validation |

---

## Documentation Structure

### Top-Level Docs

| File | Purpose |
|------|---------|
| `README.md` | Project overview |
| `SYSTEM_DEVELOPER_GUIDE.md` | Complete architecture (10K+ lines) |
| `INTEGRATION_QUICK_REFERENCE.md` | Component API lookup |
| `PROJECT_METRICS.md` | Codebase statistics |
| `FILE_INVENTORY.md` | This file |
| `NESTED_DIRECTORY_AUDIT.md` | Directory structure audit |
| `BENCHMARK_COMPLETION_REPORT.md` | Performance analysis |
| `FHE_DELIVERABLES_INDEX.md` | FHE guide |

### Documentation Directories

```
docs/
├── mathematical/        # Proofs & formal verification
├── guides/              # User & developer guides
└── api/                 # API reference
```

---

## Common Navigation Patterns

### "I want to work on arithmetic performance"

1. Start: `hcvlang/src/crt_bigint.rs`
2. Benchmark: `hcvlang/benches/crt_arithmetic.rs`
3. Validate: `tools/qmnf_benchmark_suite.py`

### "I need to add a new mathematical operation"

1. Rust primitive: `hcvlang/src/math/`
2. Python wrapper: `qmnf/arithmetic/`
3. Tests: `tests/python/test_suite.py`

### "I want to understand FHE"

1. Overview: `FHE_DELIVERABLES_INDEX.md`
2. Rust core: `hcvlang/src/fhe/`
3. Python API: `qmnf/crypto/fhe_core.py`

### "I need to validate integer-only compliance"

1. Run: `python3 tools/check_no_floats.py`
2. Check: `python3 tools/boundary_validator.py`
3. Review: `qmnf/conversion_boundary.py`

---

## Project Statistics

- **Total**: ~810,000 lines (code + docs)
- **Source**: 553,124 lines (334K Rust + 219K Python)
- **Documentation**: 257,762 lines
- **Files**: 1,211 source files
- **Arithmetic Modules**: 58+
- **Disk**: 1.2 GB

See `PROJECT_METRICS.md` for detailed breakdown.

---

## Module Dependency Flow

```
User Application
    ↓
qmnf/api.py (Public API)
    ↓
qmnf/core.py (Python Core)
    ↓
qmnf/qmnf_bridge.py (Bridge Layer)
    ↓
hcvlang/src/ffi.rs (FFI Boundary)
    ↓
hcvlang/src/crt_bigint.rs (Core Arithmetic)
    ↓
hcvlang/src/bigint_hcv.rs (Unlimited Precision)
```

---

## Cross-Reference: Component Locations

| Component | Rust | Python | Docs |
|-----------|------|--------|------|
| **CRTBigInt** | `hcvlang/src/crt_bigint.rs` | `qmnf/api.py` | `README.md` |
| **QMNFRational** | `hcvlang/src/rational.rs` | `qmnf/boundary.py` | `INTEGRATION_QUICK_REFERENCE.md` |
| **MANA** | `hcvlang/src/mana_orchestration.rs` | `qmnf/cosmos_mana/` | `SYSTEM_DEVELOPER_GUIDE.md` |
| **HoloHD** | `hcvlang/src/storage/mod.rs` | `qmnf/storage/holodrive/` | `README.md` |
| **FHE** | `hcvlang/src/fhe/operations.rs` | `qmnf/crypto/fhe_core.py` | `FHE_DELIVERABLES_INDEX.md` |

---

## Quick Search Commands

```bash
# Find all arithmetic modules
find . -path "*/arithmetic/*" -name "*.py" -o -path "*/math/*" -name "*.rs"

# Find all Rust modules
find hcvlang/src -name "*.rs" | sort

# Search for function (e.g., "garner")
grep -r "garner" hcvlang/src/ --include="*.rs"

# Count FHE lines
find hcvlang/src/fhe -name "*.rs" -exec wc -l {} + | tail -1

# Find benchmarks
find . -path "*/benches/*.rs" -o -path "*/benchmarks/*.py"
```

---

## Getting Help

- **Architecture**: `SYSTEM_DEVELOPER_GUIDE.md`
- **API**: `INTEGRATION_QUICK_REFERENCE.md`
- **Performance**: `BENCHMARK_COMPLETION_REPORT.md`
- **Statistics**: `PROJECT_METRICS.md`
- **Directory Issues**: `NESTED_DIRECTORY_AUDIT.md`

---

**Navigation Tip**: Use editor "Go to file" (Ctrl+P) with patterns:
- `*crt*` - CRT files
- `*fhe*` - FHE cryptography
- `*mana*` - MANA orchestration
- `*rational*` - Rational arithmetic
- `*test*` - Tests
- `*bench*` - Benchmarks

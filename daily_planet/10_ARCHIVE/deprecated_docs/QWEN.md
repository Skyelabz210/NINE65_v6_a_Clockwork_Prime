# QMNF System - Quantum-Modular Numerical Framework

## Project Overview

The QMNF (Quantum-Modular Numerical Framework) System is a cutting-edge mathematical research platform that focuses on **100% integer-only AI architectures** with exact rational arithmetic. The system combines advanced mathematical frameworks with practical implementations for research in numerical computing without floating-point contamination.

### Core Principles

- **Integer-Only Mathematics**: All computations use exact rational arithmetic via `QMNFRational`
- **Boundary Protection**: Strict enforcement preventing float contamination through guard decorators
- **Modular Architecture**: Clean separation of concerns with dependency injection
- **Performance-First**: Rust primitives with Python bindings for optimal speed

### Key Components

| Component | Purpose | Language | Status |
|-----------|---------|----------|--------|
| **QMNFRational** | Exact rational arithmetic | Python/Rust | ✅ Production |
| **HCVLang** | High-performance primitives | Rust | ✅ Production |
| **MAA Double Helix** | Dual-lane execution with ECC | Rust | ✅ Production |
| **COSMOS-MANA** | Memory orchestration | Rust/Python | ✅ Production |
| **Neural Training** | Integer-only neural networks | Python | ✅ Production |
| **FHE Crypto** | Fully homomorphic encryption | Rust | ✅ Production |

## Building and Running

### Prerequisites

- Python 3.9 or higher
- Rust 1.70 or higher (for building HCVLang components)
- 4GB RAM minimum (8GB recommended)

### Installation

```bash
# Clone the repository
git clone <repository-url>
cd QMNF_System

# Install Python dependencies
pip install -e .

# Build Rust components
cd hcvlang
cargo build --release

# Run tests
pytest tests/ -v
```

### Basic Usage

```python
from qmnf.boundary import QMNFRational

# Create exact rational numbers
a = QMNFRational(22, 7)  # π approximation
b = QMNFRational(1, 3)

# Perform exact arithmetic
result = a * b  # No floating-point error!
print(result)  # 22/21

# Convert to integer or decimal when needed
print(float(result))  # 1.047619...
```

## Architecture Highlights

### Integer-Only Neural Networks

QMNF implements neural network training using only integer arithmetic. The system includes:

- HelixNeuralNet for integer-only neural networks
- HiveGSO optimizer with integer gradients
- Complete neural training pipeline without floating-point values

### COSMOS-MANA Memory System

Intelligent memory orchestration with page-colored, attractor-based substrate:

```python
from qmnf.storage.cosmos import COSMOSBackend

# Initialize COSMOS memory backend
cosmos = COSMOSBackend(capacity=1024**2)  # 1MB

# Store with automatic page coloring
cosmos.store(key="data", value=tensor)

# Retrieve with attractor dynamics
retrieved = cosmos.retrieve(key="data")
```

### Fully Homomorphic Encryption

Integer-based FHE (BFV scheme) with QMNF noise system for secure computation:

```rust
use hcvlang::fhe::{FHEContext, SecurityLevel, initialize_qmnf_noise};

// Initialize QMNF integer-only noise (replaces floating-point entropy)
initialize_qmnf_noise(12345, 1 << 20, 1000);

// Create FHE context
let ctx = FHEContext::new(SecurityLevel::Bit128);
let (sk, pk) = ctx.generate_keypair();

// Encrypt integers
let ct1 = ctx.encrypt(&ctx.encode(10), &pk);
let ct2 = ctx.encrypt(&ctx.encode(32), &pk);

// Homomorphic addition: 10 + 32 = 42
let ct_sum = ctx.add(&ct1, &ct2);
let result = ctx.decode(&ctx.decrypt(&ct_sum, &sk));
assert_eq!(result, 42);
```

### Comprehensive Arithmetic Framework

The QMNF System includes a sophisticated arithmetic framework with advanced mathematical operations:

#### Core Arithmetic Operations

- **Binary GCD (Stein's Algorithm)**: 2.16x faster than Euclidean algorithm
- **Montgomery Multiplication**: 15-20% speedup with division-free modular arithmetic
- **Barrett Reduction**: 10-15% speedup with one-cycle modular reduction
- **CRT BigInt**: 419ns operations (2.4M ops/sec) using Chinese Remainder Theorem
- **Extended Binary GCD**: Computes Bézout coefficients and modular inverses

#### Advanced Mathematical Operations

- **NTT Engine**: Number Theoretic Transform for O(N log N) polynomial multiplication
- **FFT Engine**: Fast Fourier Transform with integer complex scaling
- **Karatsuba Algorithm**: O(N^1.585) multiplication for small N values
- **Schönhage-Strassen**: O(N log N log log N) multiplication for large numbers
- **Pollard Rho**: Fast factorization for composite number handling
- **Tonelli-Shanks**: Modular square root computation
- **Lucas Primality Testing**: Deterministic primality verification
- **AKS Primality**: Polynomial-time primality proof

#### Specialized Algorithms

- **φ-Harmonic Sequences**: Golden ratio exact arithmetic with convergence tracking
- **Coprime-Piggyback Division**: Robust division with fallback strategies
- **Lazy Reduction**: Deferred normalization for performance optimization
- **Prime Generation**: NTT-compatible prime finder for cryptographic applications
- **Garner's Algorithm**: Optimal CRT reconstruction
- **Jacobi Symbol**: Quadratic residue testing

#### Performance Monitoring

The framework includes comprehensive performance tracking with:
- Operation metrics recording (duration, strategy used, cache hits/misses)
- Global performance monitor for all arithmetic operations
- Strategy statistics and usage analysis
- Audit-compliant provenance tracking with HMAC signatures

### Core Implementation Structure

The system consists of several key directories:

- `qmnf/` - Python core framework with:
  - `boundary.py` - QMNFRational and float guards
  - `neural/` - Integer neural network training
  - `storage/` - COSMOS and HoloDrive backends
  - `crypto/` - Cryptographic implementations
  - `arithmetic/` - Mathematical operations library with:
    - `core/` - Basic integer operations
    - `cryptographic/` - Crypto-specific algorithms
    - `field_theory/` - Field operations
    - `calculus/` - Calculus operations
    - `quantum/` - Quantum-inspired operations
    - `geometry/` - Geometric operations
    - `sequences/` - Deterministic sequence engines
    - `validation/` - Validation and verification tools

- `hcvlang/` - Rust high-performance primitives with:
  - `rational.rs` - Exact rational arithmetic implementation
  - `bigint_hcv.rs` - Arbitrary-precision integer operations
  - `crt_bigint.rs` - Chinese Remainder Theorem BigInt
  - `fhe/` - Fully homomorphic encryption module
  - `geometric.rs` - 2D/3D geometric primitives
  - `math/` - Complete mathematical library (integers, primes, rationals, discrete)

## Development Conventions

### Float Usage Policy

The system enforces integer-only mathematics in core computational modules through compiler lints (`#![deny(clippy::float_arithmetic)]`). Floating-point values from external sources must be normalized at system boundaries using `ensure_qmnf_rational()` or Python's `float.as_integer_ratio()`.

Acceptable float usage is limited to monitoring/optimization layers: FHE noise tracking, SIMD geometry acceleration, and benchmark infrastructure.

### Code Quality Tools

- **Boundary Validation**: `tools/boundary_validator.py` - Validates QMNF compliance
- **Type Checking**: `mypy` for static type analysis
- **Linting**: `ruff` for Python, `clippy` for Rust (with float_arithmetic denial)

### Testing

```bash
# Run all tests
pytest tests/ -v

# Run specific test suites
pytest tests/python/test_suite.py -v      # Python tests
pytest tests/python/fhe_comprehensive_test.py -v  # FHE tests

# Run with coverage
pytest tests/ --cov=qmnf --cov-report=html

# Run Rust tests
cd hcvlang
cargo test
```

## Performance Benchmarks

### Running Benchmarks

```bash
# Run lightweight benchmark suite
python milestone_benchmark.py

# Run comprehensive benchmarks
python tools/qmnf_benchmark_suite.py

# Run Rust benchmarks
cd hcvlang
cargo bench
```

### Performance Targets

| Operation | Target | Current | Status |
|-----------|--------|---------|--------|
| Rational Basic | >30k ops/sec | 37,143 ops/sec | ✅ |
| Geometric Points | >30k ops/sec | 38,723 ops/sec | ✅ |
| GCD Intensive | >70k ops/sec | 83,261 ops/sec | ✅ |
| Neural Forward Pass | >1k ops/sec | TBD | 🚧 |

## Development Workflow

### Setting Up Development Environment

```bash
# Install development dependencies
pip install -e ".[dev]"

# Install pre-commit hooks
pip install pre-commit
pre-commit install

# Run quality checks
make quality-check  # Runs linting, type checking, and tests
```

### Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests and quality checks (`make quality-check`)
4. Commit your changes (`git commit -m 'Add amazing feature'`)
5. Push to the branch (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## Phases of Development

- ✅ **Phase 1**: Float elimination complete (2025-10-10) - Zero float violations
- ✅ **Phase 2**: Integer neural network training integrated (2025-10-15)
- ✅ **Phase 3**: COSMOS-MANA system integration (2025-10-22)
- ✅ **Phase 4**: QMNF noise system for FHE (2025-10-29) - 100% integer-only cryptographic noise
- 🚧 **Phase 5**: HCVLang primitive optimizations (In Progress)
- 📋 **Phase 6**: Complete mathematical operations library
- 📋 **Phase 7**: Production deployment and documentation

The QMNF System represents a comprehensive approach to integer-only computing with exact rational arithmetic, providing a foundation for secure, reproducible, and precise mathematical computation in AI and cryptography applications.
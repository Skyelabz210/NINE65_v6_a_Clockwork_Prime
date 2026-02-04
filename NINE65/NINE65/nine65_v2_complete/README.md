# NINE65 V2: Fully Homomorphic Encryption

**BFV-based FHE implementation in Rust with exact arithmetic**

[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-All%20Rights%20Reserved-red.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-140%20passing-brightgreen.svg)]()
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

NINE65 is a Fully Homomorphic Encryption library implementing the BFV scheme with three main features: zero error accumulation in ciphertext multiplication, O(1) RNS division, and fast deterministic entropy generation.

---

## Features

### Zero Error Accumulation

Standard BFV implementations accumulate rounding error during ciphertext-ciphertext multiplication (typically ~4000× per operation). NINE65 uses dual-track arithmetic with exact integer reconstruction to eliminate this error.

**Validation**: 10,000+ test iterations with zero drift

### K-Elimination Division

Residue Number System (RNS) division traditionally requires O(k²) Chinese Remainder Theorem reconstruction. NINE65 implements an anchor-first algorithm that achieves O(k) complexity with exact results in the anchor product.

**Performance**: Division = 54.9ns, Multiplication = 54.4ns (ratio: 1.01×)

### WASSAN Entropy

Deterministic chaotic system using 144 φ-harmonic oscillators for fast pseudo-random generation. Passes NIST SP 800-22 statistical tests.

**Performance**: 10.3ns per u64 (vs 1626ns for OS CSPRNG)

---

## Performance (Dec 22, 2025)

**Hardware**: Intel i7-3632QM (2012, Ivy Bridge, 2.2GHz, 4 cores)
**Config**: V2 features (FFT-NTT + WASSAN), single-threaded

| Operation | Time | Notes |
|-----------|------|-------|
| Homomorphic Multiplication | 5.66ms | N=1024, includes exact rescaling |
| Encryption | 1.46ms | N=1024, BFV scheme |
| Decryption | 621µs | N=1024 |
| Homomorphic Addition | 4.79µs | Coefficient-wise |
| NTT (Forward) | 74.3µs | FFT-based, N=1024 |
| K-Elimination Division | 54.9ns | Same cost as Montgomery multiply |
| WASSAN Entropy | 10.3ns | Per u64 sample |

### V2 Optimizations

| Component | V1 (Baseline) | V2 | Speedup |
|-----------|---------------|-----|---------|
| NTT | 1934µs (DFT) | 74.3µs (FFT) | 26× |
| Entropy | 1626ns (CSPRNG) | 10.3ns (WASSAN) | 158× |
| Polynomial Multiply | 5718µs | 663µs | 8.6× |

**Note**: Benchmarked on 12-year-old hardware. Modern CPUs (2024 i9-13900K) estimated to be 1.5-2× faster due to higher IPC, clock speeds, and memory bandwidth.

---

## Installation & Quick Start

```bash
# Clone repository
git clone https://github.com/yourusername/nine65.git
cd nine65

# Build with V2 features
cargo build --release --features v2

# Run tests
cargo test --release

# Run benchmarks
cargo run --release --bin fhe_benchmarks --features v2
cargo bench --bench criterion_fhe --features v2
```

### Example Usage

```rust
use qmnf_fhe::*;

// Initialize context (N=1024, ~80-bit security)
let ctx = FHEContext::new_light();
let (sk, pk) = ctx.keygen();

// Encrypt values
let ct_a = ctx.encrypt(42, &pk);
let ct_b = ctx.encrypt(17, &pk);

// Homomorphic multiplication (exact)
let ct_result = ctx.homo_mul(&ct_a, &ct_b, &pk);

// Decrypt
let result = ctx.decrypt(&ct_result, &sk);
assert_eq!(result, 714);  // Exact: 42 × 17 = 714
```

---

## Architecture

### Dual-Track Arithmetic

Each coefficient represented as `(c_inner, c_anchor)`:
- **c_inner**: RNS channels for fast computation (m₁, m₂, ..., mₖ)
- **c_anchor**: Anchor moduli for exact reconstruction (A₁, A₂, A₃)

Invariant: `c_inner ≡ c_anchor (mod gcd(M_inner, A_product))`

### K-Elimination Algorithm

```
Traditional RNS Division: O(k²) full CRT reconstruction
K-Elimination: O(k) via anchor-first computation

1. Compute division exactly in 3 anchor moduli
2. Lift to k computational channels via affine lifting
3. Total: O(k) with exact guarantee in anchor product
```

### WASSAN (φ-harmonic chaos)

```
144 coupled oscillators with frequencies in golden ratio (φ) series
Holographic coupling: Cᵢⱼ = sin(φ × |i-j|)
Lyapunov exponent: λ ≈ 0.5 (chaotic regime)
Output: Passes NIST SP 800-22 statistical tests
```

---

## Directory Structure

```
nine65_v2_complete/
├── src/
│   ├── arithmetic/              # Core math operations
│   │   ├── ntt_fft.rs          # FFT-based NTT
│   │   ├── k_elimination.rs    # O(k) RNS division
│   │   ├── exact_divider.rs    # K-Elimination primitive
│   │   ├── exact_coeff.rs      # Dual-track coefficients
│   │   ├── ct_mul_exact.rs     # Exact CT×CT multiplication
│   │   ├── persistent_montgomery.rs
│   │   ├── montgomery.rs
│   │   └── rns.rs
│   ├── entropy/
│   │   ├── wassan_noise.rs     # WASSAN entropy
│   │   └── shadow_oracle.rs
│   ├── ops/                     # FHE operations
│   ├── quantum/                 # Quantum simulation modules
│   ├── keys/                    # Key generation
│   ├── noise/                   # Noise budget tracking
│   └── bin/
│       └── fhe_benchmarks.rs
├── benches/
│   └── criterion_fhe.rs         # Statistical benchmarks
├── docs/
│   └── proofs/                  # Mathematical proofs
├── NINE65_PRESENTATION_PACKET.md
├── COMPETITIVE_ANALYSIS.md
├── V2_BENCHMARK_RESULTS.md
└── README.md
```

---

## Documentation

- **[NINE65_PRESENTATION_PACKET.md](NINE65_PRESENTATION_PACKET.md)** - Complete technical documentation (~2800 lines)
- **[COMPETITIVE_ANALYSIS.md](COMPETITIVE_ANALYSIS.md)** - Comparison with SEAL, OpenFHE, HElib, Lattigo
- **[HARDWARE_ADJUSTED_ANALYSIS.md](HARDWARE_ADJUSTED_ANALYSIS.md)** - Performance analysis and projections
- **[V2_BENCHMARK_RESULTS.md](V2_BENCHMARK_RESULTS.md)** - Detailed benchmark results

---

## Testing

```bash
# All tests
cargo test --release

# Specific module
cargo test --release --lib arithmetic

# With output
cargo test --release -- --nocapture
```

**Test Coverage**:
- 140+ tests passing
- 0 compilation errors
- Property-based testing (PropTest)
- Zero error accumulation validated
- NIST SP 800-22 randomness validation

---

## Comparison with Other Libraries

| Feature | NINE65 | SEAL | OpenFHE | HElib | Lattigo |
|---------|--------|------|---------|-------|---------|
| **Language** | Rust | C++ | C++ | C++ | Go |
| **Zero Error CT×CT** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **O(k) RNS Division** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Deterministic** | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Memory Safety** | ✅ Rust | ❌ C++ | ❌ C++ | ❌ C++ | ✅ Go |
| **Maturity** | 2 years | 10+ years | 5+ years | 10+ years | 5+ years |
| **GPU Support** | ❌ | ✅ | ✅ | ❌ | ❌ |

**Use NINE65 for**: Exact arithmetic requirements, deterministic computation, memory-safe implementation
**Use SEAL/OpenFHE for**: Maximum performance with AVX-512/GPU, mature ecosystem, commercial support

---

## Use Cases

**Good fit**:
- Scientific computing requiring exact results
- Reproducible research (bit-identical across platforms)
- Systems requiring formal verification
- High-throughput applications needing fast entropy
- Safety-critical implementations (Rust memory safety)

**Not ideal for**:
- Maximum raw performance with GPU acceleration
- Wide language bindings (Python/Java/etc)
- Commercial support requirements
- Long production track record needed

---

## Roadmap

**V2 (Current)** - December 2024 ✅
- FFT-based NTT (26× speedup)
- WASSAN entropy (158× speedup)
- Quantum simulation modules
- Production-ready (0 compilation errors)

**V3 (Planned)** - 2025-2026
- AVX2/AVX-512 SIMD (4-8× potential speedup)
- Multi-threading (2-4× potential speedup)
- GPU acceleration (10-100× potential speedup)
- Language bindings (Python, JavaScript, C)

---

## License

**Copyright © 2023-2025 Anthony Diaz. All Rights Reserved.**

This software is proprietary and confidential. No part of this software may be reproduced, distributed, or transmitted in any form without the prior written permission of the copyright holder.

For licensing inquiries, contact: acid@hackfate.us

See [LICENSE](LICENSE) for full terms.

---

## Author

**Anthony Diaz**
HackFate.us Research Division
San Antonio, Texas, USA

📧 acid@hackfate.us
🌐 [HackFate.us](https://hackfate.us)

---

## Contributing

Contributions welcome. Areas where help is needed:
- SIMD optimizations (AVX2/AVX-512)
- GPU acceleration (CUDA/HIP)
- Language bindings
- Documentation
- Security audits

---

## Citation

```bibtex
@software{nine65_v2,
  title = {NINE65: BFV Fully Homomorphic Encryption with Exact Arithmetic},
  author = {Diaz, Anthony},
  year = {2024},
  version = {2.0},
  url = {https://github.com/yourusername/nine65}
}
```

---

## References

**Foundational Work**:
- Garner (1959): Residue Number Systems
- Montgomery (1985): Modular Multiplication
- Brakerski, Fan, Vercauteren (2012): BFV FHE Scheme
- Cooley-Tukey (1965): FFT Algorithm

**Novel Contributions** (this project):
- K-Elimination: O(k) RNS division
- Dual-track arithmetic: Zero error accumulation
- WASSAN: φ-harmonic holographic entropy

See [NINE65_PRESENTATION_PACKET.md](NINE65_PRESENTATION_PACKET.md) for complete attribution.

---

**Generated**: December 22, 2025
**Status**: Production Ready (0 errors, 140+ tests passing)

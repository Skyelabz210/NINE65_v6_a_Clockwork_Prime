# Repository Overview

## Project Description

**NINE65** is a cutting-edge, production-ready Fully Homomorphic Encryption (FHE) system that achieves depth-50 circuits without requiring bootstrapping. The project implements the QMNF (Quantized Modular Number Field) architecture combined with multiple innovative cryptographic techniques including K-Elimination exact division, GSO-FHE noise bounding, and CRT Shadow Entropy.

Main purpose and goals:
- Eliminate the need for bootstrapping in FHE operations
- Achieve superior performance compared to traditional FHE systems
- Maintain post-quantum security through LWE-based cryptography
- Provide exact arithmetic without floating-point approximations
- Support encrypted quantum operations for blind quantum search

Key technologies used:
- Rust 2021 for memory-safe, high-performance implementation
- Coq for formal verification of 14 cryptographic innovations
- SIMD acceleration for optimal performance
- Integer-only arithmetic with K-Elimination for exact RNS operations
- Parallel processing with Rayon for scaling

## Architecture Overview

### High-Level Architecture

The NINE65 architecture consists of three main components working together:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   nine65        │    │     mana        │    │     unhal       │
│  (Core FHE)     │◄──►│  (Accelerator)  │◄──►│  (Hardware AB)  │
│                 │    │                 │    │                 │
│ • Dual-RNS      │    │ • Modular       │    │ • Auto-detect   │
│ • K-Elimination │    │   arithmetic    │    │ • Pipelines     │
│ • GSO-FHE       │    │ • K-Elimination │    │ • Batch ops     │
│ • BFV scheme    │    │   anchor        │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Main Components and Their Relationships

1. **Core FHE Layer (nine65)**: Implements the core FHE operations including:
   - Dual-RNS with K-Elimination for exact division
   - GSO-FHE noise bounding without bootstrapping
   - BFV encryption scheme with exact arithmetic
   - Encrypted quantum operations (Sparse Grover)

2. **Modular Arithmetic Accelerator (mana)**: Optimizes core operations:
   - Parallel compute lanes for RNS operations
   - K-Elimination anchor computations
   - CRT stream operations

3. **Hardware Abstraction Layer (unhal)**: Handles platform-specific optimizations:
   - Auto-detection of hardware capabilities
   - Pipeline management
   - Batch processing

### Data Flow and System Interactions

Data flows through a pipeline that maintains exact arithmetic properties:
1. Plaintext → BFVEncoder → Encryptor (with ShadowHarvester entropy)
2. Ciphertext → RNSFHEContext → Dual-RNS K-Elimination rescaling
3. Processed ciphertext → GSO-FHE noise bounding
4. Result ciphertext → Decryptor → Plaintext

The system maintains constant-time operations, secure entropy collection, and integer-only arithmetic throughout all stages.

## Directory Structure

### Important Directories and Their Purposes

```
NINE65/MANA_boosted/
├── crates/
│   ├── nine65/           # Core FHE implementation
│   │   ├── src/
│   │   │   ├── arithmetic/         # RNS, NTT, Montgomery, K-Elimination
│   │   │   ├── ops/               # Encryption, homomorphic operations
│   │   │   ├── entropy/           # CRT shadow, secure CSPRNG
│   │   │   ├── quantum/           # Encrypted quantum, state taxonomy
│   │   │   ├── keys/              # Key generation and management
│   │   │   ├── ring/              # Polynomial operations
│   │   │   ├── noise/             # Noise budget tracking
│   │   │   ├── security/          # LWE security estimation
│   │   │   ├── params/            # Parameter configurations
│   │   │   ├── ahop/              # AHOP Grover simulation
│   │   │   └── bin/fhe_demo.rs    # CLI demonstration
│   │   ├── benches/               # Criterion benchmarks
│   │   └── tests/                 # Integration tests
│   ├── mana/               # Modular arithmetic accelerator
│   │   └── src/
│   │       ├── lane.rs            # Parallel compute lanes
│   │       ├── stream.rs          # CRT stream operations
│   │       ├── anchor.rs          # K-Elimination anchor
│   │       └── gso.rs             # Swarm optimization
│   └── unhal/              # Hardware abstraction layer
│       └── src/
│           ├── accelerator.rs     # Auto-detect execution
│           ├── pipeline.rs        # Staged compute
│           └── batch.rs           # Bulk processing
├── proofs/
│   └── coq/                # 14 formal verification files
│       ├── KElimination.v         # K-Elimination theorem
│       ├── GSOFHE.v               # GSO noise bounds
│       ├── OrderFinding.v         # Non-circular BSGS
│       └── [11 more...]
├── docs/                   # Design and security documentation
│   ├── SECURITY_PROOFS.md
│   ├── ENCRYPTED_QUANTUM_PAPER.md
│   └── NON_CIRCULAR_ORDER_FINDING.md
├── Cargo.toml              # Workspace configuration
├── README.md               # Project overview
└── .continue/              # AI assistant configuration
    └── rules/
        └── review.md       # Custom code review slash command
```

### Key Files and Configuration

- `crates/nine65/src/lib.rs` - Main library entry point
- `crates/nine65/src/ops/rns_fhe.rs` - K-Elimination RNS FHE operations
- `crates/nine65/src/ops/gso_fhe.rs` - GSO noise bounding layer
- `crates/nine65/src/arithmetic/rns.rs` - Core RNS arithmetic
- `crates/nine65/src/bin/fhe_demo.rs` - CLI demonstration interface
- `proofs/coq/KElimination.v` - Formal verification of K-Elimination
- `docs/SECURITY_PROOFS.md` - Security assumptions and proofs

## Development Workflow

### How to Build/Run the Project

Build commands:
```bash
# Debug build
cargo build --workspace

# Release build (recommended for performance)
cargo build --release --workspace

# Build specific crate
cargo build -p nine65 --release
```

Run commands:
```bash
# Run the FHE demo
cargo run --bin fhe_demo --release

# Run with specific parameters
cargo run --bin fhe_demo --release -- --a 17 --b 25 --config standard_128

# Run specific examples
cargo run -p nine65 --example demo --release
```

### Testing Approach

Test structure:
```bash
# Run all tests across workspace
cargo test --workspace --release

# Run unit tests for core FHE
cargo test -p nine65 --lib --release

# Run with all features including v2 integration
cargo test -p nine65 --features v2,parallel,accelerated --release

# Run specific test suites
cargo test -p nine65 rns_fhe --release  # K-Elimination tests
cargo test --test pqeaq_harness --release  # Integration suite
cargo test -p nine65 arithmetic::order_finding --release  # Order finding tests

# Property-based testing
cargo test -p nine65 --release -- --test proptest
```

### Development Environment Setup

1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Clone the repository: Navigate to `/home/acid/Projects/NINE65/MANA_boosted/`
3. Set up dependencies: `cargo build --workspace`
4. Verify setup: `cargo test -p nine65 --lib --release`

### Lint and Format Commands

Format and lint commands:
```bash
# Format code
cargo fmt --all

# Check for lint issues
cargo clippy --workspace --release

# Check for security issues
cargo audit  # if cargo-audit is installed

# Run with all lints
cargo clippy --workspace --release -- -D warnings
```

### Key Development Practices

- Follow Rust 2021 edition with rustfmt defaults (4-space indent)
- Maintain integer-only arithmetic in critical paths (no floating-point)
- Use constant-time operations for secret-dependent code
- Apply zeroize for key material cleanup
- Implement proper entropy management using ShadowHarvester
- Follow the dual-RNS architecture for exact division with K-Elimination
- Include appropriate error handling with custom Nine65Error types
- Add brief comments for mathematical operations when the intent isn't obvious
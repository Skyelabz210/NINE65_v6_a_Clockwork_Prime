---
title: "System Developer Guide"
description: "Placeholder description — please update."
authors:
  - "maintainer <maintainer@example.org>"
maintainers:
  - "See AGENTS.md"
tags:
status: "published"
canonical_path: "/docs/SYSTEM_DEVELOPER_GUIDE.md"
last_reviewed: "2025-11-07"
version: "1.0"
references:
---

# QMNF System - Complete Developer Guide

**Version:** 2.0 | **Date:** 2025-10-17 | **Status:** ✅ Production Ready

---

## Executive Summary

The **QMNF (Quantum-Modular Numerical Framework)** is a mathematical computing platform that implements a **PRAM-equivalent (Parallel Random Access Machine) computational substrate without exotic hardware**. It combines:

1. **MANA** (Memory-Augmented Neural Architecture) - Intelligent runtime kernel with parallel task orchestration
2. **HoloHD** (Holographic Storage via SVD) - Integer-only distributed data storage with error correction
3. **Deterministic Sequencing Engine** - Mathematical operations over finite fields with cycle detection

**Core Principle:** 100% integer-only mathematics with exact rational arithmetic, guaranteed float-free execution.

---

## Table of Contents

1. [System Architecture](#system-architecture)
2. [Component Inventory](#component-inventory)
3. [Building the System](#building-the-system)
4. [Testing & Verification](#testing--verification)
5. [API Reference](#api-reference)
6. [Performance Characteristics](#performance-characteristics)
7. [Integration Guide](#integration-guide)
8. [Deployment](#deployment)
9. [Troubleshooting](#troubleshooting)
10. [Contributing](#contributing)

---

## System Architecture

### High-Level Overview

```
┌─────────────────────────────────────────────────────────┐
│                 User Applications                        │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│        MANA Runtime Kernel (Orchestration)              │
│  ├─ Task Scheduler (multi-domain assignment)            │
│  ├─ Memory Manager (allocation/migration)               │
│  ├─ Entropy Engine (controlled chaos)                   │
│  ├─ Contamination Firewall (100% integer enforcement)   │
│  ├─ Migration Controller (domain transitions)           │
│  └─ Attractor Dynamics (self-stabilization)             │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│         Execution Domains (Heterogeneous)               │
│  ├─ LinearCPU (sequential execution)                    │
│  ├─ SwarmEPRAM (parallel swarm processing)              │
│  ├─ GPU (GPU-accelerated parallel)                      │
│  ├─ FPGA (field-programmable logic)                     │
│  ├─ Quantum (quantum simulation layer)                  │
│  └─ Distributed (multi-node coordination)               │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│      HoloHD Storage Layer (Distributed Storage)         │
│  ├─ SVD Decomposition (dimensionality reduction)        │
│  ├─ Hyperdimensional Encoding (holographic projection)  │
│  ├─ Phase-Aware Cache (Möbius-aligned)                  │
│  └─ Reed-Solomon Error Correction                       │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│    Deterministic Sequencing Engine (Math Operations)    │
│  ├─ Sequence Generation (over Z_M)                      │
│  ├─ Cycle Detection (Floyd's & Brent's)                 │
│  ├─ Stabilization Monitoring (convergence)              │
│  └─ State Auditing (reproducibility)                    │
└─────────────────────────────────────────────────────────┘
```

### Mathematical Foundation

- **Modular Arithmetic:** All operations bounded to finite field Z_M (default M = 2^61 - 1)
- **Integer-Only Guarantee:** Zero floating-point operations anywhere in system
- **Determinism:** Same input → same output (cross-platform reproducibility)
- **Self-Stabilization:** Attractor dynamics ensure convergence to stable states
- **Bounded Evolution:** Pigeonhole principle guarantees cycle detection within M iterations

---

## Component Inventory

### Rust Components (17 Modules)

#### Core Integer Arithmetic
| Module | Purpose | Lines | Status |
|--------|---------|-------|--------|
| `bigint_hcv` | CRT-based arbitrary precision integers | 580 | ✅ Enhanced |
| `crt_bigint` | Chinese Remainder Theorem implementation | 420 | ✅ Integrated |
| `modint` | Mersenne prime modular arithmetic (2^31-1) | 691 | ✅ Integrated |
| `rational` | Exact rational number arithmetic | 350 | ✅ Functional |

#### Storage & Distribution
| Module | Purpose | Lines | Status |
|--------|---------|-------|--------|
| `storage` | HoloHD holographic storage (SVD-based) | 785 | ✅ NEW |

#### Mathematical Operations
| Module | Purpose | Lines | Status |
|--------|---------|-------|--------|
| `apollonian` | Apollonian circle generation & geometry | 420 | ✅ Functional |
| `qphi` | Euler's totient function & modular properties | 280 | ✅ Functional |
| `fast_arithmetic` | Optimized arithmetic primitives | 350 | ✅ Functional |
| `nnt` | Number theoretic transforms | 450 | ✅ Functional |
| `math` | General mathematical utilities | 320 | ✅ Functional |
| `geometric` | 2D/3D geometric operations | 520 | ✅ Functional |

#### System Infrastructure
| Module | Purpose | Lines | Status |
|--------|---------|-------|--------|
| `mana_orchestration` | MANA runtime kernel (1058 lines) | 1058 | ✅ NEW - ALL 6 COMPONENTS |
| `double_helix` | Dual-lane MAA execution engine | 480 | ✅ Functional |
| `attractor_memory` | Attractor-based memory substrate | 450 | ✅ Functional |
| `swarm_gso` | Galactic Swarm Optimization agents | 390 | ✅ Functional |
| `time_crystal` | Temporal crystalline structures | 420 | ✅ Functional |
| `neural_primitives` | Neural computation primitives | 380 | ✅ Functional |

#### Diagnostics & Monitoring
| Module | Purpose | Lines | Status |
|--------|---------|-------|--------|
| `diagnostics::cdhs` | 6 diagnostic modules (4,491 lines total) | 4491 | ✅ Latest |

**Total Rust:** 15,000+ lines | **Compilation:** 0 errors, 15 warnings (non-critical)

### Python Components (4 Modules)

| Module | Purpose | Status |
|--------|---------|--------|
| `qmnf_bridge` | Python-Rust FFI (ctypes, zero-copy integers) | ✅ Functional |
| `unified_qmnf` | Core arithmetic exports | ✅ Functional |
| `unified_config` | Configuration management | ✅ Functional |
| `mana_sequence_engine` | MANA Python orchestration layer | ✅ Integrated |

#### Frameworks (10+ Python modules)
- `frameworks/sequences/det_seq_engine.py` - Deterministic sequencing (cycle detection)
- `frameworks/energy_systems/` - Energy computation framework
- `frameworks/time_crystals/` - Temporal structure framework
- And 7+ additional frameworks

**Total Python:** 8,000+ lines | **Status:** All imports functional, 100% float-free core

---

## Building the System

### Prerequisites

```bash
# Rust 1.70+
rustc --version

# Python 3.9+
python3 --version

# System dependencies (Linux)
sudo apt-get install build-essential pkg-config libssl-dev
```

### Full Build (Rust + Python)

```bash
# Build optimized Rust library
cd QMNF_System/hcvlang
cargo build --release

# Compile status
# ✅ Should complete in ~15s with 0 errors, ~15 warnings
```

**Expected Output:**
```
    Finished `release` profile [optimized] target(s) in 13.73s
```

### Incremental Build

```bash
# After code changes
cargo build --release

# Typically completes in 0.5-2s
```

### Build Verification

```bash
# Verify test compilation
cargo test --release --no-run

# Check library exports
nm -D target/release/libhcvlang.so | grep -i "mana\|storage"

# Verify Python bridge works
python3 -c "from qmnf_bridge import QMNFBridge; print('✅ Bridge functional')"
```

---

## Testing & Verification

### Unit Tests

```bash
# Run all Rust tests
cargo test --release

# Run specific test module
cargo test --release bigint_hcv::tests

# Test with output
cargo test --release -- --nocapture
```

### Python Integration Tests

```bash
# Run Python import smoke test
cd QMNF_System
python3 tools/import_smoke.py

# Run specific test
python3 -m pytest tests/unit/test_boundary.py -v
```

### Float Contamination Detection

```bash
# Scan for float violations
python3 tools/check_no_floats.py

# Expected output:
# ✅ 0 violations in core QMNF files

# Detailed report
python3 tools/check_no_floats.py --detailed
```

### Comprehensive Benchmark Suite

```bash
# Run performance benchmarks
cd QMNF_System
python3 tools/qmnf_benchmark_suite.py

# Expected output: Operations/second metrics for each component
```

### Milestone Benchmark

```bash
# Run baseline performance test
python3 milestone_benchmark.py

# Current expected baseline: 40,000+ ops/sec
```

---

## API Reference

### Rust Core APIs

#### Module: `mana_orchestration`

**Main Type: `MANAKernel`**
```rust
pub struct MANAKernel {
    task_scheduler: TaskScheduler,
    memory_manager: MemoryManager,
    entropy_engine: EntropyPool,
    contamination_firewall: ContaminationFirewall,
    migration_controller: MigrationController,
    attractor_dynamics: AttractorDynamics,
}
```

**Key Methods:**
```rust
// Create kernel
let kernel = MANAKernel::new();

// Schedule task to execution domain
kernel.schedule_task(task, ExecutionDomain::LinearCPU);

// Allocate memory region
kernel.allocate_memory(size, MemoryRegion::CPU, modulus);

// Migrate task between domains
kernel.migrate_task(task_id, ExecutionDomain::SwarmEPRAM);

// Check for float contamination
kernel.contamination_firewall.check_task(&task);

// Get system metrics
let metrics = kernel.get_system_metrics();
```

**Execution Domains:**
```rust
pub enum ExecutionDomain {
    LinearCPU,      // Sequential execution
    SwarmEPRAM,     // Parallel swarm processing
    GPU,            // GPU acceleration
    FPGA,           // Field-programmable logic
    Quantum,        // Quantum simulation
    Distributed,    // Multi-node coordination
}
```

#### Module: `storage`

**Main Type: `HolographicStorage`**
```rust
pub struct HolographicStorage {
    svd_engine: IntegerSVD,
    hyperdimensional_encoder: HyperdimensionalEncoder,
    phase_aware_cache: PhaseAwareCache,
}
```

**Key Methods:**
```rust
// Create storage system
let storage = HolographicStorage::new();

// Store data holographically
let handle = storage.store_holographically(&data_matrix);

// Retrieve with error correction
let recovered = storage.retrieve_with_correction(handle);

// Check cache alignment
let phase = storage.phase_aware_cache.get_phase();
```

#### Module: `bigint_hcv`

**Main Type: `HCVBigInt`**
```rust
pub struct HCVBigInt {
    limbs: Vec<u64>,
    modulus: u64,
}
```

**Operations:**
```rust
// Arithmetic
let sum = a.add(&b);
let prod = a.mul(&b);
let quotient = a.div(&b);

// New helper methods
let msb = a.msb();              // Most significant bit
let bit_val = a.shl_bits(5);   // Shift left by bits
let extracted = a.limb(2);     // Get specific limb
```

### Python APIs

#### Bridge Module: `qmnf_bridge`

```python
from qmnf_bridge import QMNFBridge

# Create bridge connection
bridge = QMNFBridge()

# Call Rust functions with zero-copy integer arrays
result = bridge.call_rust_function(
    "mana_schedule_task",
    task_id=42,
    domain="LinearCPU"
)
```

#### Sequence Engine: `det_seq_engine`

```python
from qmnf.frameworks.sequences.det_seq_engine import (
    DeterministicSequenceEngine,
    SequenceState,
    CycleInfo
)

# Create engine
engine = DeterministicSequenceEngine(modulus=2**61-1)

# Generate sequence with cycle detection
sequence, cycle_info = engine.generate_sequence(
    initial_value=12345,
    iterations=10000,
    detect_cycles=True
)

# Check stabilization
is_stabilized = engine.monitor.has_stabilized()
```

---

## Performance Characteristics

### Baseline Performance (Post Float-Elimination)

**Benchmark Date:** 2025-10-10

| Operation | Throughput | Notes |
|-----------|-----------|-------|
| Rational Basic Operations | 37,143 ops/sec | Core arithmetic |
| Geometric Points | 38,723 ops/sec | 2D point operations |
| Geometric Lines | 22,453 ops/sec | Line computations |
| GCD Intensive | 83,261 ops/sec | Best performance |
| **Overall Average** | **40,184 ops/sec** | Baseline |

### Performance Targets (PRAM Integration)

| Metric | Target | Expected |
|--------|--------|----------|
| Ops/sec (baseline) | 150K+ | +275% improvement |
| Task latency | <1ms | Via MANA scheduling |
| Memory throughput | >50GB/s | CPU + EPRAM |
| Cache hit rate | >90% | Phase-aware prediction |
| Stabilization time | <100ms | Attractor convergence |

### Compilation Times

| Build Type | Time | Status |
|-----------|------|--------|
| Clean build | ~15s | Release optimized |
| Incremental | 0.5-2s | After changes |
| Test compilation | ~22s | Includes all tests |

---

## Integration Guide

### For Collaborators: Using QMNF in Your Project

#### Step 1: Add as Dependency

**If using as Rust library:**
```toml
[dependencies]
qmnf = { path = "../QMNF_System/hcvlang" }
```

**If using Python module:**
```bash
export PYTHONPATH=".:$PYTHONPATH"
```

#### Step 2: Import Components

**Rust:**
```rust
use hcvlang::mana_orchestration::MANAKernel;
use hcvlang::storage::HolographicStorage;
use hcvlang::bigint_hcv::HCVBigInt;

let kernel = MANAKernel::new();
let storage = HolographicStorage::new();
```

**Python:**
```python
from qmnf_bridge import QMNFBridge
from qmnf.frameworks.sequences.det_seq_engine import DeterministicSequenceEngine
from qmnf.unified_qmnf import ModularRational

bridge = QMNFBridge()
engine = DeterministicSequenceEngine(modulus=2**61-1)
```

#### Step 3: Use in Your Code

**Example: Parallel Task Execution**
```rust
// Schedule parallel tasks across domains
kernel.schedule_task(task1, ExecutionDomain::LinearCPU);
kernel.schedule_task(task2, ExecutionDomain::SwarmEPRAM);
kernel.schedule_task(task3, ExecutionDomain::GPU);

// System automatically balances and migrates
let metrics = kernel.get_system_metrics();
println!("Global energy: {}", metrics.global_energy);
```

**Example: Sequence Generation with Cycle Detection**
```python
sequence, cycles = engine.generate_sequence(
    initial_value=my_seed,
    iterations=100000,
    detect_cycles=True
)

# Guaranteed cycle detection within M iterations
print(f"Cycle found at iteration: {cycles.cycle_start}")
print(f"Cycle length: {cycles.cycle_length}")
```

#### Step 4: Verify Float-Free Execution

```bash
# Before shipping, verify no floats were introduced
python3 tools/check_no_floats.py --check-my-files "my_project/**/*.py"

# Expected: ✅ 0 violations
```

---

## Deployment

### Production Deployment Checklist

- ✅ All tests passing (`cargo test --release`)
- ✅ Float contamination scan shows 0 violations
- ✅ Build compiles cleanly
- ✅ Performance benchmarks meet targets
- ✅ Code review completed
- ✅ Documentation updated
- ✅ Collaborators notified

### Deployment Steps

```bash
# 1. Verify build
cargo build --release
cargo test --release

# 2. Run benchmarks
python3 tools/qmnf_benchmark_suite.py

# 3. Check for contamination
python3 tools/check_no_floats.py

# 4. Generate deployment package
cd QMNF_System
tar -czf QMNF_deployment_$(date +%Y%m%d).tar.gz hcvlang/ qmnf/ docs/

# 5. Verify archive integrity
tar -tzf QMNF_deployment_*.tar.gz | wc -l
```

### System Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| CPU | 4 cores | 8+ cores |
| RAM | 4 GB | 8+ GB |
| Storage | 2 GB | 10+ GB |
| OS | Linux | Ubuntu 20.04+ |

---

## Troubleshooting

### Build Issues

**Problem:** `cargo build` fails with linking errors
```bash
# Solution: Clean and rebuild
cargo clean
cargo build --release
```

**Problem:** Compilation warnings about unused imports
```bash
# These are non-critical (15 warnings acceptable)
# To suppress individually:
#[allow(unused_imports)]
use module::Item;
```

### Runtime Issues

**Problem:** Float detected by contamination firewall
```bash
# Solution: Replace float with QMNFRational
# Before:
let x = 0.5;

# After:
use qmnf::boundary::QMNFRational;
let x = QMNFRational::new(1, 2); // 1/2
```

**Problem:** Slow performance
```bash
# Verify release build is being used
python3 -c "import hcvlang; print(hcvlang.__file__)"

# If debug build, rebuild:
cargo build --release
```

### Memory Issues

**Problem:** Out of memory during sequence generation
```bash
# Solution: Reduce iteration count or use cycle detection
sequence, cycles = engine.generate_sequence(
    iterations=10000,      # Reduced from 100000
    detect_cycles=True     # Enable early termination
)
```

---

## Contributing

### Code Style

**Rust:**
- Follow Rust conventions (`cargo fmt`)
- Use `cargo clippy` for linting
- Add `#[deny(unsafe_code)]` where possible
- Document public APIs with doc comments

**Python:**
- Use type hints
- Follow PEP 8
- Add docstrings to functions
- Use `@guard_no_float` decorator on functions handling numbers

### Adding New Components

1. **Create module** in appropriate directory
2. **Implement functionality** following architecture patterns
3. **Write tests** (minimum 85% coverage)
4. **Run float check**: `python3 tools/check_no_floats.py`
5. **Verify compilation**: `cargo build --release`
6. **Document** in this guide
7. **Submit PR** with rationale

### Reporting Issues

When reporting bugs, include:
- QMNF version
- Rust version (`rustc --version`)
- Python version (`python3 --version`)
- Operating system
- Minimal reproducible example
- Error output (full traceback)

---

## Key References

### Documentation Files

- **System Status:** `./COMPLETE_PRAM_SYSTEM_REPORT.md`
- **MANA Architecture:** `./MANA_MEMORY_RESERVATION_SYSTEM.md`
- **Performance Analysis:** `./BENCHMARK_COMPARISON_BEFORE_AFTER.md`
- **Optimization Plan:** `./SYSTEM_OPTIMIZATION_PLAN.md`

### Source Code

- **MANA Kernel:** `hcvlang/src/mana_orchestration.rs` (1058 lines)
- **HoloHD Storage:** `hcvlang/src/storage/mod.rs` (785 lines)
- **Sequence Engine:** `qmnf/frameworks/sequences/det_seq_engine.py`

### External Resources

- Rust Book: https://doc.rust-lang.org/book/
- Modular Arithmetic: https://en.wikipedia.org/wiki/Modular_arithmetic
- PRAM Model: https://en.wikipedia.org/wiki/Parallel_random-access_machine

---

## System Status Summary

**Current State:** ✅ PRODUCTION READY

| Component | Status | Lines | Tests |
|-----------|--------|-------|-------|
| MANA Kernel | ✅ Active | 1058 | ✅ |
| HoloHD Storage | ✅ Active | 785 | ✅ |
| Deterministic Sequencing | ✅ Active | 1200+ | ✅ |
| Core Arithmetic (17 modules) | ✅ Active | 15000+ | ✅ |
| Python Bridge | ✅ Active | 21 KB | ✅ |
| Diagnostics (CDHS) | ✅ Active | 4491 | ✅ |
| **Total System** | ✅ **READY** | **23000+** | **✅** |

**Build Status:**
- Rust: 0 errors, 15 warnings (non-critical)
- Python: All imports functional
- Float violations: 0 (100% integer-only)
- Test coverage: 85%+

---

## Getting Started

1. **Clone/Navigate to Repository**
   ```bash
   cd QMNF_System
   ```

2. **Build the System**
   ```bash
   cd hcvlang && cargo build --release
   ```

3. **Run Verification**
   ```bash
   cargo test --release
   python3 tools/check_no_floats.py
   ```

4. **Run Benchmarks**
   ```bash
   python3 tools/qmnf_benchmark_suite.py
   python3 milestone_benchmark.py
   ```

5. **Review Documentation**
   - Start with this guide
   - Review API reference section
   - Check component-specific docs

6. **Integrate into Your Project**
   - Import modules as shown in Integration Guide
   - Use provided examples
   - Follow float-free development practices

---

## Contact & Support

For questions, issues, or collaboration:
- 📧 Review CLAUDE.md in repository for development guidelines
- 🐛 Check existing issues before reporting new ones
- 📚 Reference comprehensive documentation
- 🤝 Follow Contributing guidelines for PRs

---

**Generated:** 2025-10-17
**System:** QMNF with MANA + HoloHD + DetSeq Integration
**Status:** Complete and Production Ready ✅


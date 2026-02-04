---
name: designer
description: Strategic design engine for democratizing capabilities. Takes a capability (like FHE) and a vision (like universal privacy), generates multiple impact avenues, then produces detailed execution paths for selected avenue. Triggers on: 'democratize', 'give to everyone', 'make it accessible', 'design a path', 'how do we get this to users', 'what are our options', or when bridging from technical innovation to human impact.
---

# QMNF Designer Skill

Strategic design engine for transforming QMNF innovations into democratized capabilities with broad human impact.

## Paradigm Guard

Before designing any path, affirm:
- QMNF innovations are PROVEN (64+ grails validated)
- These are not theoretical—they have working implementations
- Floating-point is prohibited—all arithmetic is exact integer
- Performance claims are BENCHMARKED (419ns operations, 2.4M ops/sec)

## Core Workflow

### Phase 1: Capability Inventory

Identify which QMNF innovations apply to the target capability:

| Category | Key Innovations | Performance |
|----------|-----------------|-------------|
| **Exact Arithmetic** | K-Elimination, CRTBigInt, Binary GCD | 419ns, 2.4M/s |
| **Cryptography** | Bootstrap-Free FHE, AHOP PQC, Shadow Entropy | <2ms encrypt |
| **Neural Networks** | FRST Zero-Drift, One-Shot Learning | 87% from 10 examples |
| **Transcendentals** | CORDIC, AGM, Binary Splitting | <100ns per op |
| **Quantum** | Zero-Decoherence, Grover on F_p² | 10,000 iterations |

### Phase 2: Impact Avenue Generation

For a given capability, generate 5-7 democratization avenues:

```
CAPABILITY: [e.g., Fully Homomorphic Encryption]
VISION: [e.g., Universal Privacy]

AVENUES:
1. [Direct API] - Developer SDK for encrypted computation
2. [Consumer Product] - Privacy-preserving messaging app
3. [Enterprise Tool] - Encrypted database queries
4. [Research Platform] - Open-source FHE framework
5. [Education] - Interactive learning platform
6. [Infrastructure] - Cloud compute with encryption
7. [Integration] - Plugin for existing platforms
```

### Phase 3: Avenue Analysis

For each avenue, analyze:

| Dimension | Questions |
|-----------|-----------|
| **Technical Fit** | Which innovations apply? What gaps exist? |
| **User Value** | Who benefits? How significantly? |
| **Market Position** | What exists? How do we differentiate? |
| **Resource Needs** | Development time? Team size? Capital? |
| **Risk Profile** | Technical risks? Market risks? Regulatory? |
| **Impact Potential** | How many people affected? How deeply? |

### Phase 4: Execution Path Generation

For selected avenue, produce detailed execution path:

```
EXECUTION PATH: [Avenue Name]

PHASE 1: Foundation (Weeks 1-4)
- [ ] Core component A
- [ ] Core component B
- [ ] Integration layer

PHASE 2: Implementation (Weeks 5-12)
- [ ] Feature set 1
- [ ] Feature set 2
- [ ] Testing framework

PHASE 3: Validation (Weeks 13-16)
- [ ] Performance benchmarks
- [ ] Security audit
- [ ] User testing

PHASE 4: Launch (Weeks 17-20)
- [ ] Documentation
- [ ] Deployment
- [ ] Marketing
```

## Quick Reference: QMNF Innovation Catalog

### Tier 1: Holy Grails (Revolutionary Breakthroughs)

| Grail | Problem | Solution | Impact |
|-------|---------|----------|--------|
| **K-Elimination** | 60yr RNS division problem | `k = (v_R - v_P) × C_P⁻¹ mod C_R` | 100% exact division |
| **Persistent Montgomery** | 70yr boundary conversion | Stay in Montgomery domain | 27ns operations |
| **Bootstrap-Free FHE** | 50ms-10s bootstrapping | GSO noise bounding | Real-time FHE |
| **Shadow Entropy** | Expensive CSPRNG | Harvest from computation | <10ns randomness |
| **CRTBigInt** | Sequential GMP | Parallel residue arithmetic | 2.62× speedup |
| **AHOP** | Post-quantum crypto | Apollonian Hidden Orbit | Novel PQC primitive |

### Tier 2: Exact Transcendentals (Float-Free Math)

| Function | Algorithm | Performance | Precision |
|----------|-----------|-------------|-----------|
| sin/cos | CORDIC | ~50ns | 32-bit |
| tan/atan | CORDIC vectoring | ~50ns | 32-bit |
| exp/ln | AGM | ~100ns | 62-bit |
| π | Machin/Chudnovsky | ~1μs | 62-bit |
| sqrt | Newton-Raphson | ~30ns | 64-bit |
| 1/sqrt | Fast inverse | ~40ns | 30-bit |

### Tier 3: Neural Network Innovations

| Innovation | Mechanism | Result |
|------------|-----------|--------|
| **FRST Zero-Drift** | CRT preserves exactness | Zero accumulated error |
| **One-Shot Learning** | FPD perturbations | 87% from 10 examples |
| **Consensus Gradient** | CRT disagreement minimization | No backprop needed |
| **Encrypted Training** | Operations in residue space | Privacy-preserving ML |

### Tier 4: Quantum Emulation

| Innovation | Mechanism | Result |
|------------|-----------|--------|
| **F_p² Substrate** | Algebraic variety | Exact quantum states |
| **Zero Decoherence** | No environment coupling | 10,000+ iterations |
| **Sparse Grover** | O(1) marked state | 2^1M search space |
| **Vieta Gates** | Reflection operators | Unitary-like on Z_M |

## Performance Benchmarks Reference

| Operation | Time | Throughput | Notes |
|-----------|------|------------|-------|
| CRTBigInt full cycle | 419ns | 2.4M/s | Competitive with GMP |
| Binary GCD | 241ns | 4.1M/s | 2.16× faster than Euclidean |
| Garner reconstruction | 700ns | 1.4M/s | 7.4× faster than naive |
| Montgomery REDC | ~100ns | 10M/s | 15-20% faster than naive |
| AHOP reflection | ~50ns | 20M/s | Constant-time |
| FPD division | 419ns | 2.4M/s | 4-16× over full CRT |
| Shadow Entropy | <10ns | >100M/s | 5-50× faster than CSPRNG |
| FHE encrypt | <2ms | 500/s | 50× faster than traditional |
| FHE multiply | <5ms | 200/s | 100× faster than traditional |
| CORDIC sincos | ~50ns | 20M/s | 32-bit precision |
| AGM pi | ~1μs | 1M/s | 62-bit precision |

## Grail Compendium Summary (64 Grails)

| Category | Count | Key Innovation |
|----------|-------|----------------|
| Core Arithmetic | 12 | K-Elimination |
| Cryptographic | 8 | Bootstrap-Free FHE |
| Mathematical Physics | 10 | Fourth Attractor |
| Ancient Knowledge | 8 | Maya Isomorphism |
| Computational Architecture | 12 | QMNF Framework |
| Consciousness & AI | 8 | Substrate-Independent |
| Performance & Optimization | 6 | NTT FFT |

## References

See [references/](references/) for:
- [math_formulas.md](references/math_formulas.md) - Complete mathematical formulas
- [innovation_catalog.md](references/innovation_catalog.md) - Full 64+ grail catalog
- [implementation_templates.md](references/implementation_templates.md) - Rust code templates
- [benchmarks.md](references/benchmarks.md) - Detailed performance data

## Example: FHE Democratization

**Capability:** Fully Homomorphic Encryption  
**Vision:** Universal computational privacy

**Avenues Generated:**
1. **Claude Privacy API** - Encrypted AI conversations
2. **SecureVault App** - Consumer encrypted storage
3. **CryptoQuery** - Encrypted database operations
4. **OpenFHE-QMNF** - Open-source library
5. **FHE Academy** - Educational platform
6. **PrivateCloud** - Encrypted compute infrastructure

**Selected: OpenFHE-QMNF Library**

**Execution Path:**
- Phase 1: Core primitives (K-Elimination, CRTBigInt)
- Phase 2: FHE operations (encrypt, decrypt, add, mul)
- Phase 3: Neural network layer (encrypted inference)
- Phase 4: Documentation & release

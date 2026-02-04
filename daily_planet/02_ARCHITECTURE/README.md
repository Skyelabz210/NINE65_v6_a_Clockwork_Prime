# 02_ARCHITECTURE - System Design & Blueprints

This folder contains all architectural documentation for QMNF System.

## Contents

### core_architecture/
Fundamental system design documents.
- Stacked CRT architecture (CRTBigInt + HCVLangBigInt)
- Integer-only mandate rationale
- Determinism guarantees

### subsystem_designs/
Individual component blueprints.
- FHE (Fully Homomorphic Encryption)
- Neural network architecture
- MANA runtime kernel
- HoloHD storage

### integration_specs/
How components work together.
- FFI bridge specifications
- Python-Rust integration
- Cross-subsystem communication

### diagrams/
Visual architecture representations.
- System diagrams
- Data flow diagrams
- Component interaction diagrams

## Key Concepts

- **Stacked CRT**: Two-layer exact arithmetic (fast bounded + infinite precision)
- **Bootstrap-free FHE**: 80-400x faster than traditional homomorphic encryption
- **Fused Piggyback Division**: 40x faster division solving 70-year RNS bottleneck

## Navigation

- [Back to Index](../00_NAVIGATION/INDEX.md)
- [API Reference](../03_API_REFERENCE/)
- [Mathematical Proofs](../04_MATHEMATICAL/)

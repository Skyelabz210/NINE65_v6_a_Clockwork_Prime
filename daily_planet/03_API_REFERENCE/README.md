# 03_API_REFERENCE - Code Documentation

This folder contains all API documentation for QMNF System.

## Contents

### rust_api/
Rust API documentation for hcvlang primitives.
- CRTBigInt operations
- ModInt arithmetic
- Neural primitives
- FHE operations

### python_api/
Python API documentation for qmnf package.
- QMNFRational class
- Conversion boundary
- High-level interfaces

### ffi_reference/
FFI bridge documentation (Python-Rust bindings).
- PyO3 wrapper classes
- Batch operations
- Performance patterns

### quick_references/
Cheat sheets and quick lookup guides.
- Integration quick reference
- Common operations
- Error codes

## Key APIs

| API | Purpose | Performance |
|-----|---------|-------------|
| `CRTBigInt` | Fast bounded integers | ~120ns/op |
| `HCVLangBigInt` | Unlimited precision | O(n^2) |
| `QMNFRational` | Exact rational math | ~1us/op |
| `ModInt` | Mersenne modular | 30-50ns/op |

## Navigation

- [Back to Index](../00_NAVIGATION/INDEX.md)
- [Architecture](../02_ARCHITECTURE/)
- [Tutorials](../08_TUTORIALS/)

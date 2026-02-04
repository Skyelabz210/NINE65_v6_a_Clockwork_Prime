# QMNF System Tools

This directory contains specialized tools for working with the QMNF mathematical framework and maintaining the codex.

## Tools Index

### Mathematical Discoverer (`refined_mathematical_discoverer.py`)
The primary tool for discovering, analyzing, and cataloging mathematical implementations in codebases.

#### Features:
- Discovers mathematical constructs across multiple categories (arithmetic, geometry, transforms, crypto, etc.)
- Uses adaptive overflow management to handle large codebases
- Implements multi-phase iterative scanning with prioritization
- Maintains dimensional consistency during analysis
- Provides comprehensive reporting of mathematical implementations

#### Usage:
```bash
python refined_mathematical_discoverer.py [TARGET_DIRS...] --threads 4 --output ./output
```

#### Categories Mapped:
- Arithmetic: Modular arithmetic, rational numbers, GCD, CRT
- Geometry: Exact geometric operations and predicates
- Transforms: Number Theoretic Transform (NTT), polynomial operations
- Cryptography: FHE, encryption, noise systems
- Optimization: SIMD, parallelization, performance enhancements
- Quantum: Quantum-inspired algorithms and operations
- Number Theory: Primes, factorization, discrete math
- Algebra: Abstract algebraic structures

#### Key Features:
- Integer-only arithmetic: No floating-point contamination
- Adaptive overflow management: Handles large datasets gracefully
- Multi-threaded scanning: Parallel processing for efficiency
- Dimensional consistency: Physical unit validation
- Modular arithmetic: Exact operations in ℤ/M systems
- Coprime cascade: O(n log n) multiplication algorithms

#### Configuration Options:
- `--max-size`: Maximum file size to scan (in MB)
- `--threads`: Number of parallel threads
- `--output`: Output directory for results
- `--threshold`: Overflow threshold per category
- `--chunk-size`: Lines per chunk for large files
This tool was developed to systematically catalog the mathematical implementations throughout the QMNF System for your private codex.

### Pre-commit Gate Hook (`tools/git_hooks/no_float_pre_commit.sh`)

- Enforces `tools/prod_gate.py --scope repo --quiet` on every commit.
- Reverts staged files and exits non-zero when the repository still contains float-like violations anywhere.
- Install it by symlinking into `.git/hooks/pre-commit` (e.g., `ln -sf ../../tools/git_hooks/no_float_pre_commit.sh .git/hooks/pre-commit`).
- Runs both sanitizers (`sanitize_digits_dot_digits_nbsp.py --apply --include-all-text` and `sanitize_json_csv_no_floats.py --apply`) across the repo before the gate.
- Automatically stages those sanitized edits, but if the gate still fails the hook resets the tree to `HEAD` so no partial cleanup is left behind.

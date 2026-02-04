# GEMINI.md: QMNF System

This document provides a comprehensive overview of the QMNF (Quantum-Modular Numerical Framework) System for Gemini.

## Project Overview

The QMNF System is a high-performance Python library for integer-only AI mathematics. It leverages a Rust core for performance-critical calculations, ensuring that all mathematical operations are exact and free from floating-point errors.

The project follows a three-zone architecture:

1.  **Zone 1: Core Mathematics (Integer-Only):** The Rust core, where all mathematical operations are performed using exact rational arithmetic. This zone has a strict float prohibition, enforced by compiler lints.
2.  **Zone 2: Normalization Boundaries:** The Python-Rust interface, where external data (including floats) is converted to exact rational representations. This is the only place where floats are allowed to enter the system.
3.  **Zone 3: Monitoring & Optimization:** A pragmatic zone where floats are used for non-computational tasks like performance monitoring, cryptographic metadata, and hardware acceleration.

The QMNF System has recently undergone a significant refactoring to improve performance and architecture. The old system of scattered "guard" decorators has been replaced with a clean, explicit boundary layer, resulting in a production-ready system with zero breaking changes for existing users.

## Building and Running

### Prerequisites

*   Python 3.8+
*   Rust toolchain

### Setup

1.  **Set Environment Variables:**

    ```bash
    export LD_LIBRARY_PATH=/path/to/QMNF_System:$LD_LIBRARY_PATH
    export PYTHONPATH=/path/to/QMNF_System:$PYTHONPATH
    ```

2.  **Install Dependencies:**

    The project uses `setuptools` and `setuptools-rust` for building. The dependencies are listed in the `pyproject.toml` file.

### Running

To use the QMNF System in your Python code, simply import the `QMNFRational` class:

```python
from qmnf import QMNFRational

# Create rationals
r1 = QMNFRational(22, 7)
r2 = QMNFRational(355, 113)

# Perform arithmetic
result = r1 * r2

print(result)
```

### Testing

The project uses `pytest` for testing. To run the tests, execute the following command:

```bash
pytest tests/python/ -v
```

## Development Conventions

### Coding Style

*   The project follows the PEP 8 style guide for Python code.
*   Type hints are used extensively to ensure type safety.
*   The code is well-documented with docstrings and comments.

### Testing Practices

*   The project has a comprehensive test suite that covers all aspects of the system.
*   Unit tests are used to test individual components in isolation.
*   Integration tests are used to test the entire system end-to-end.

## Key Files

*   `qmnf/api.py`: The main entry point for using the QMNF system.
*   `qmnf/conversion_boundary.py`: The boundary layer for data validation and conversion.
*   `hcvlang/src/`: The Rust core of the project.
*   `pyproject.toml`: The project's build configuration.
*   `DEVELOPER_QUICK_START.md`: A guide for developers.
*   `00_START_HERE.md`: A high-level overview of the project.
*   `ARCHITECTURE.md`: A detailed description of the project's architecture.

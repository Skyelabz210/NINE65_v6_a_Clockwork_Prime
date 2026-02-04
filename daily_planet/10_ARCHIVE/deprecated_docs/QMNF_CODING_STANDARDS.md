# Quantum-Modular Numerical Framework (QMNF) Coding Standards

**Version:** 1
**Date:** December 11, 2025
**Status:** MANDATORY - All code must comply

This document establishes the mandatory coding standards for the QMNF_System codebase. The primary goal is to ensure absolute consistency, mathematical correctness, and compliance with the Quantum-Modular Numerical Framework (QMNF) core philosophy, thereby eliminating inconsistencies and preventing linter and type-checker violations.

---

## 1. The QMNF Numerical Standard: Integer-Only Computation

The foundational principle of the QMNF is the **absolute elimination of floating-point arithmetic** to ensure provable correctness, arbitrary precision, and deterministic computation. This standard is non-negotiable and must be applied across all layers of the system, including Python code, Rust code, database schemas, and configuration files.

### Prohibition of Native Floating-Point Types

The use of native floating-point types is **strictly forbidden**. Any attempt to introduce them will result in a linter violation and a QMNF compliance failure.

| Context | Forbidden Type | Mandatory Replacement | Rationale |
|---------|----------------|----------------------|-----------|
| Python Code | `float` (e.g., `x: float`) | `int` (e.g., `x_thousandths: int`) | Eliminates IEEE 754 precision errors and non-determinism |
| Rust Code | `f32`, `f64` | `i64`, `i128`, `BigInt` | Ensures exact arithmetic |
| Database (SQLite) | `REAL` or `FLOAT` | `INTEGER` | Ensures data integrity |
| Configuration (JSON/YAML) | Decimal point numerals (digits-dot-digits) | Integer literals (scaled) | Maintains integer-only pipeline |

### Handling Fractional Values (The "Unavoidable" Case)

When a fractional or decimal concept is required (e.g., percentages, sub-unit measurements), it must be represented using a **scaled integer**. The scaling factor must be chosen to maintain the required precision and must be clearly indicated in the variable name.

| Concept | Human Meaning | Scaling Factor | Mandatory Variable Name Suffix | Example QMNF-Compliant Value |
|---------|---------------|----------------|-------------------------------|------------------------------|
| Percentage | seventy percent | 100 | `_percent` | `confidence_percent = 70` |
| Speedup/Ratio | ratio measured per one hundred | 100 | `_per_hundred` | `speedup_per_hundred = 216` |
| Thousandths | sub-unit measurement in thousandths | 1000 | `_thousandths` | `latency_ns_thousandths = 7500` |
| Ten-Thousandths | sub-unit measurement in ten-thousandths | 10000 | `_ten_thousandths` | `tolerance_ten_thousandths = 12` |
| Permille (per thousand) | ratio measured per one thousand | 1000 | `_permille` | `ratio_permille = 2400` |
| Millionths | whole-number count (no scaling needed) | 1 | (None, if whole number) | `throughput = 2400000` |

**Mandatory Rule:** Any function that accepts or returns a scaled integer must document the scaling factor in its docstring.

---

## 2. Code Formatting and Linter Standards

### Python: Ruff Configuration

The project uses `ruff` for linting and formatting with the following standards:

- **Line Length:** Maximum 88 characters (Black default)
- **Quotes:** Double quotes (`"`) for strings
- **Trailing Commas:** Mandatory for list, dict, and set literals when elements are split across lines
- **Imports:** Imports must be grouped and sorted automatically by Ruff (isort rules)

### Python: MyPy Type Checking

- **Type Hinting:** All function arguments, return values, and class attributes must be type-hinted
- **Integer Enforcement:** Type hints must explicitly use `int` for all QMNF numerical values
- **Float Prohibition:** The use of `float` in type hints is a **compliance violation**

### Rust: Clippy Configuration

The project uses `clippy` with the following mandatory lints:

```rust
#![deny(clippy::float_arithmetic)]  // DENY all float arithmetic
#![warn(clippy::pedantic)]           // Enable pedantic lints
#![warn(clippy::nursery)]            // Enable nursery lints
```

---

## 3. Naming Conventions

All naming conventions must follow PEP 8 (Python) and Rust conventions, with specific extensions for QMNF-compliant variables.

### Python Naming

| Element | Convention | Example | QMNF-Specific Rule |
|---------|-----------|---------|-------------------|
| Modules | Short, all lowercase, underscores OK | `qmnf_core.py` | N/A |
| Classes | PascalCase (CapWords) | `QMNFRational`, `ModularArithmetic` | N/A |
| Functions/Methods | snake_case, all lowercase | `calculate_entropy()`, `get_modulus()` | N/A |
| Variables | snake_case, all lowercase | `result_value`, `prime_list` | MUST use suffixes for scaled integers |
| Constants | ALL_CAPS_WITH_UNDERSCORES | `FIXED_PRECISION`, `MAX_MODULUS` | N/A |

### Rust Naming

| Element | Convention | Example | QMNF-Specific Rule |
|---------|-----------|---------|-------------------|
| Modules | snake_case | `crt_bigint`, `exact_division` | N/A |
| Structs/Enums | PascalCase | `CRTBigInt`, `ModInt` | N/A |
| Functions | snake_case | `calculate_gcd()`, `mod_inverse()` | N/A |
| Variables | snake_case | `result_value`, `prime_list` | MUST use suffixes for scaled integers |
| Constants | SCREAMING_SNAKE_CASE | `DEFAULT_MODULUS`, `FIBONACCI_MODULI` | N/A |
| Type Parameters | Single uppercase letter or PascalCase | `T`, `M`, `Modulus` | N/A |

### File Naming Standards

| File Type | Convention | Example | Anti-Pattern |
|-----------|-----------|---------|--------------|
| Rust source | snake_case.rs | `crt_bigint.rs` | `CRTBigInt.rs` |
| Python source | snake_case.py | `qmnf_core.py` | `QMNFCore.py` |
| Test files | test_*.rs or test_*.py | `test_crt_bigint.rs` | `CRTBigIntTest.rs` |
| Documentation | SCREAMING_SNAKE.md | `CLAUDE.md` | `claude.md` |

---

## 4. Documentation Standards

All public modules, classes, methods, and functions must include a docstring following the Google Style Guide.

### Python Docstring Structure (Google Style)

A function docstring must include the following sections, where applicable:

1. **Summary:** A concise, one-line description of the function's purpose
2. **Args:** A list of arguments, including their type and a description. For QMNF-compliant integers, the description must specify the scaling factor
3. **Returns:** The return value, including its type and a description. For QMNF-compliant integers, the description must specify the scaling factor
4. **Raises:** Any exceptions that are explicitly raised

**Example Docstring (QMNF-Compliant):**

```python
def calculate_scaled_confidence(raw_value: int, max_value: int) -> int:
    """Calculates the confidence as a percentage integer.

    Args:
        raw_value: The raw, unscaled integer value (e.g., 7).
        max_value: The maximum possible integer value (e.g., 10).

    Returns:
        The confidence value scaled by 100 (e.g., 70 for 70%).
        Scale factor: 100 (divide by 100 to get decimal percentage).
    """
    return (raw_value * 100) // max_value
```

### Rust Documentation Standards

All public items must have `///` doc comments following Rust conventions:

```rust
/// Calculates the GCD of two integers using the binary (Stein's) algorithm.
///
/// # Arguments
///
/// * `a` - First integer (must be non-negative)
/// * `b` - Second integer (must be non-negative)
///
/// # Returns
///
/// The greatest common divisor of `a` and `b`.
///
/// # Performance
///
/// Time complexity: O(log(min(a, b)))
/// Latency: ~190ns per call
///
/// # Example
///
/// ```
/// let gcd = binary_gcd(48, 18);
/// assert_eq!(gcd, 6);
/// ```
pub fn binary_gcd(a: u64, b: u64) -> u64 {
    // Implementation
}
```

---

## 5. Module Organization Standards

### Rust Module Structure

Every Rust module directory must contain:

```
module_name/
├── mod.rs          # Module declaration and public exports
├── types.rs        # Type definitions (structs, enums)
├── ops.rs          # Core operations
├── tests.rs        # Unit tests (#[cfg(test)])
└── benches.rs      # Benchmarks (optional)
```

### Python Package Structure

Every Python package must contain:

```
package_name/
├── __init__.py     # Package init with __all__ exports
├── types.py        # Type definitions (dataclasses, protocols)
├── core.py         # Core implementation
├── api.py          # Public API (re-exports from core)
└── tests/
    └── test_*.py   # Unit tests
```

---

## 6. Integer-Only Compliance Checklist

Before committing code, verify the following:

### Python Checklist

- [ ] No decimal point numerals (digits-dot-digits)
- [ ] No `float` type hints
- [ ] No `math.sqrt()`, `math.sin()`, etc. (use integer approximations)
- [ ] No `numpy` float arrays (use `dtype=np.int64`)
- [ ] All scaled integers have proper suffixes (`_percent`, `_thousandths`, etc.)
- [ ] All docstrings specify scaling factors

### Rust Checklist

- [ ] No `f32` or `f64` types
- [ ] No decimal point numerals (digits-dot-digits)
- [ ] `#![deny(clippy::float_arithmetic)]` in lib.rs
- [ ] All scaled integers documented
- [ ] All public items have doc comments

### Validation Commands

```bash
# Python: Check for float contamination
python3 tools/check_no_floats.py

# Python: Run linter
ruff check qmnf/

# Python: Run type checker
mypy qmnf/

# Rust: Run clippy
cargo clippy --all-targets --release

# Rust: Build with deny warnings
cargo build --release
```

---

## 7. Performance Documentation Standards

All performance-critical code must include:

### Performance Annotations

```rust
/// # Performance
///
/// | Metric | Value |
/// |--------|-------|
/// | Latency | 419ns |
/// | Throughput | 2400000 ops per sec |
/// | Complexity | O(k) where k = number of moduli |
```

### Benchmark Requirements

Every core arithmetic operation must have:
- Criterion benchmark (Rust)
- pytest-benchmark (Python)
- Documented expected performance
- Regression detection

---

## 8. Error Handling Standards

### Python Error Handling

```python
class QMNFError(Exception):
    """Base exception for all QMNF errors."""
    pass

class FloatContaminationError(QMNFError):
    """Raised when float values are detected in QMNF operations."""
    pass

class OverflowError(QMNFError):
    """Raised when an operation would overflow the current tier."""
    pass
```

### Rust Error Handling

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QMNFError {
    #[error("Float contamination detected: {0}")]
    FloatContamination(String),

    #[error("Overflow in tier {tier}: value {value} exceeds maximum")]
    Overflow { tier: u8, value: i128 },

    #[error("Division by zero")]
    DivisionByZero,
}
```

---

## 9. Git Commit Standards

### Commit Message Format

```
type(scope): short description

Longer description if needed.

QMNF Compliance: [PASS/FAIL]
Float Check: [PASS/FAIL]
Tests: [PASS/FAIL]
```

### Commit Types

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation only |
| `refactor` | Code refactoring (no behavior change) |
| `perf` | Performance improvement |
| `test` | Adding/updating tests |
| `chore` | Build, CI, tooling changes |
| `style` | Formatting, naming (no logic change) |

---

## 10. References

1. [Google Python Style Guide](https://google.github.io/styleguide/pyguide.html)
2. [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
3. [PEP 8 - Style Guide for Python Code](https://pep8.org/)
4. [Conventional Commits](https://www.conventionalcommits.org/)

---

## Appendix A: Common Violations and Fixes

| Violation | Example | Fix |
|-----------|---------|-----|
| Decimal point numeral | `x = decimal_point_value` | `x_percent = 50` |
| Float type hint | `def f(x: float)` | `def f(x_percent: int)` |
| Unscaled variable | `ratio = 216` | `ratio_percent = 216` |
| Missing suffix | `latency = 7500` | `latency_ns_thousandths = 7500` |
| Float math | `math.sqrt(x)` | `integer_sqrt(x)` |
| Undocumented scale | `return value * 100` | Document: "Returns value scaled by 100" |

---

**Document:** QMNF_CODING_STANDARDS.md
**Enforcement:** All PRs must pass compliance checks before merge
**Exceptions:** None - the standard is absolute
## 6. Enforcement and Tooling

### No-Float Gate
1. Every commit must pass `python3 tools/prod_gate.py --scope repo --quiet`.
2. The pre-commit hook at `tools/git_hooks/no_float_pre_commit.sh` runs sanitizers and the gate, automatically staging sanitized docs and resetting the tree if the gate still reports violations.
3. CI/CD mirrors the same gate so automated builds cannot ship new float patterns.

### Sanitization Workflow
- Run `python3 tools/sanitize_digits_dot_digits_nbsp.py --apply --include-all-text --path <directories>` to neutralize literal `digits.dot.digits` in text assets.
- Run `python3 tools/sanitize_json_csv_no_floats.py --apply --path <directories>` to replace JSON/CSV decimal numbers with canonical `num/den` strings.
- Both sanitizers are already invoked by the pre-commit hook; manual invocation is only needed when sanitizing outside the hook (e.g., remote edits).

### Fixing Scanner Failures
- Python violations: `tools/check_no_floats.py --path qmnf` shows every decimal token, `math.float`, or other floating reference remaining in the Python tree.
- Rust violations: `tools/check_no_floats_rust.py --path hcvlang/src crates src` shows the remaining `f32/f64` types and decimal doc patterns; all must be rewritten to exact integer/rational forms.
- Update the offending lines (scaled integers, rational notation, NBSP-wrapped decimals) and rerun the gate until zero report.

## 7. Change Discipline
- Sanitize → scan → fix cycle must complete before pushing.
- If the gate reports violations, do not try to suppress the output—clean the offending code until the gate passes, then commit.

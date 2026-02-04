# FFI Development Guide - QMNF System
**Standardized Workflow for Implementing PyO3 FFI Bindings**

---

## 📋 QUICK REFERENCE

### Package Details
- **Rust Package Name**: `hcvlang`
- **Python Module Name**: `hcvlang` (when imported in Python)
- **PyO3 Version**: 0.22
- **Rust Edition**: 2021
- **Cargo Feature Flag**: `python` (NOT `pyo3`)
- **Library Type**: cdylib (C-compatible dynamic library for Python extension)

### File Locations
- **FFI Implementation**: `/home/user/QMNF_System/hcvlang/src/ffi.rs`
- **Cargo Config**: `/home/user/QMNF_System/hcvlang/Cargo.toml`
- **Test Directory**: `/home/user/QMNF_System/tests/python/`
- **Build Output**: `/home/user/QMNF_System/hcvlang/target/release/`

### Build Command (CORRECT)
```bash
cd /home/user/QMNF_System/hcvlang
cargo build --release --features python --lib
```

**IMPORTANT**: Use `--features python` NOT `--features pyo3`

### Import Statement (Python)
```python
from hcvlang import ClassName
```

---

## 🔧 STANDARDIZED 8-STEP WORKFLOW

Every FFI implementation follows this exact workflow:

---

## **STEP 1: READ SOURCE FILE**

### Tool
`Read` tool

### Purpose
Understand the Rust API before writing Python bindings

### Action
```bash
Read file: /home/user/QMNF_System/hcvlang/src/[module_path].rs
```

### What to Extract
```rust
// 1. Public structs
pub struct ClassName {
    field1: Type1,
    field2: Type2,
}

// 2. Public enums
pub enum EnumName {
    Variant1,
    Variant2,
}

// 3. Public methods
impl ClassName {
    pub fn new(param: Type) -> Self { ... }
    pub fn method(&self, arg: Type) -> ReturnType { ... }
    pub fn method_with_ref(&self, other: &OtherClass) -> Result<Type, Error> { ... }
}

// 4. Standalone public functions
pub fn standalone_function(arg: Type) -> Type { ... }
```

### Documentation Checklist
- [ ] List all public structs
- [ ] List all public enums
- [ ] List all public methods per struct
- [ ] Note parameter types (especially references to other structs)
- [ ] Note return types (especially Result/Option)
- [ ] Note error types for error handling
- [ ] Copy documentation comments for Python docstrings

---

## **STEP 2: WRITE FFI BINDINGS**

### Tool
`Edit` tool (ffi.rs already exists)

### File
`/home/user/QMNF_System/hcvlang/src/ffi.rs`

### Insertion Location
Append to end of file, before the `#[pymodule]` section.

**Current end of file**: ~Line 5420 (as of 2025-11-14)

### Pattern: Basic Struct

```rust
// ============================================================================
// [MODULE NAME IN CAPS]
// ============================================================================

/// [Brief description of the class]
///
/// [Detailed description explaining what this does]
///
/// Examples:
///     >>> from hcvlang import ClassName
///     >>> obj = ClassName(param1, param2)
///     >>> result = obj.method(arg)
///     >>> print(result)
///
#[pyclass(name = "ClassName", unsendable)]
pub struct PyClassName {
    pub(crate) inner: RustClassName,
}

#[pymethods]
impl PyClassName {
    /// Constructor documentation
    ///
    /// Args:
    ///     param1: Description of param1
    ///     param2: Description of param2
    ///
    /// Returns:
    ///     New ClassName instance
    ///
    /// Examples:
    ///     >>> obj = ClassName(10, 20)
    ///
    #[new]
    fn new(param1: i64, param2: i64) -> Self {
        PyClassName {
            inner: RustClassName::new(param1, param2),
        }
    }

    /// Method documentation
    ///
    /// Args:
    ///     arg: Description of argument
    ///
    /// Returns:
    ///     Description of return value
    ///
    fn method(&self, arg: i64) -> i64 {
        self.inner.method(arg)
    }

    /// String representation for debugging
    fn __repr__(&self) -> String {
        format!("ClassName(field1={}, field2={})",
                self.inner.field1, self.inner.field2)
    }
}
```

### Pattern: Method Taking Another PyClass (CRITICAL - PyO3 0.22+)

**OLD (PyO3 < 0.22) - DOES NOT WORK**:
```rust
fn method(&self, other: &PyOtherClass) -> i64 {
    // ❌ WRONG - will cause compile errors
}
```

**NEW (PyO3 0.22+) - CORRECT**:
```rust
fn method(&self, other: &Bound<'_, PyOtherClass>) -> PyResult<i64> {
    let other_inner = &other.borrow().inner;  // ⬅️ MUST use .borrow()
    Ok(self.inner.method(other_inner))
}
```

**Breakdown**:
- `&Bound<'_, PyOtherClass>` - PyO3 0.22+ wrapper for Python objects
- `.borrow()` - Borrows the Python object (like RefCell)
- `.inner` - Access the wrapped Rust struct
- Always returns `PyResult<T>` when working with borrowed objects

### Pattern: Method Returning Result

```rust
fn fallible_method(&self, arg: i64) -> PyResult<i64> {
    self.inner.fallible_method(arg)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
}
```

**Error Types**:
- `PyRuntimeError` - General runtime errors
- `PyValueError` - Invalid argument values
- `PyTypeError` - Type mismatches
- `PyIndexError` - Index out of bounds
- `PyKeyError` - Key not found

### Pattern: Getters

```rust
#[getter]
fn property_name(&self) -> i64 {
    self.inner.property_name
}
```

### Pattern: Setters

```rust
#[setter]
fn set_property_name(&mut self, value: i64) {
    self.inner.property_name = value;
}
```

### Pattern: Static Methods

```rust
#[staticmethod]
fn static_method(arg: i64) -> i64 {
    RustClassName::static_method(arg)
}
```

### Pattern: Class Methods

```rust
#[classmethod]
fn from_string(_cls: &Bound<'_, PyType>, s: String) -> PyResult<Self> {
    Ok(PyClassName {
        inner: RustClassName::from_string(&s)?,
    })
}
```

### Pattern: Enums

```rust
/// [Enum documentation]
///
/// Variants:
///     Variant1: Description
///     Variant2: Description
///
#[pyclass(name = "EnumName", unsendable)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyEnumName {
    Variant1,
    Variant2,
    Variant3,
}

#[pymethods]
impl PyEnumName {
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    fn __str__(&self) -> String {
        match self {
            PyEnumName::Variant1 => "Variant1".to_string(),
            PyEnumName::Variant2 => "Variant2".to_string(),
            PyEnumName::Variant3 => "Variant3".to_string(),
        }
    }
}

// Conversion helper (if needed to call Rust methods)
impl PyEnumName {
    fn to_rust(&self) -> RustEnumName {
        match self {
            PyEnumName::Variant1 => RustEnumName::Variant1,
            PyEnumName::Variant2 => RustEnumName::Variant2,
            PyEnumName::Variant3 => RustEnumName::Variant3,
        }
    }

    fn from_rust(rust_enum: RustEnumName) -> Self {
        match rust_enum {
            RustEnumName::Variant1 => PyEnumName::Variant1,
            RustEnumName::Variant2 => PyEnumName::Variant2,
            RustEnumName::Variant3 => PyEnumName::Variant3,
        }
    }
}
```

### Pattern: Vector Conversion (Python list ↔ Rust Vec)

**Python list of primitives → Rust Vec**:
```rust
fn method(&self, values: Vec<i64>) -> i64 {
    self.inner.method(&values)
}
```

**Python list of PyClasses → Rust Vec**:
```rust
fn method(&self, items: Vec<PyRef<PyClassName>>) -> PyResult<i64> {
    let rust_items: Vec<&RustClassName> = items.iter()
        .map(|py_item| &py_item.inner)
        .collect();
    Ok(self.inner.method(&rust_items))
}
```

**Rust Vec → Python list of PyClasses**:
```rust
fn method(&self) -> Vec<PyClassName> {
    self.inner.method()
        .into_iter()
        .map(|item| PyClassName { inner: item })
        .collect()
}
```

### Pattern: Optional Parameters

```rust
#[pyo3(signature = (required, optional=None))]
fn method(&self, required: i64, optional: Option<i64>) -> i64 {
    let opt_val = optional.unwrap_or(42);  // Default value
    self.inner.method(required, opt_val)
}
```

**Python usage**:
```python
obj.method(10)       # Uses default
obj.method(10, 20)   # Uses provided value
```

### Pattern: Keyword Arguments

```rust
#[pyo3(signature = (a, b, *, c=None, d=100))]
fn method(&self, a: i64, b: i64, c: Option<i64>, d: i64) -> i64 {
    // a, b are positional
    // c, d are keyword-only (after *)
    let c_val = c.unwrap_or(0);
    self.inner.method(a, b, c_val, d)
}
```

**Python usage**:
```python
obj.method(1, 2)              # c=None, d=100
obj.method(1, 2, c=3)         # d=100
obj.method(1, 2, c=3, d=4)    # All specified
```

---

## **STEP 3: REGISTER IN MODULE**

### Tool
`Edit` tool

### File
`/home/user/QMNF_System/hcvlang/src/ffi.rs`

### Location
Find the `#[pymodule]` function (around line 105-140)

### Action
Add class registration inside the function:

```rust
#[pymodule]
fn hcvlang(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // ... existing registrations ...

    // YOUR NEW CLASSES HERE:
    m.add_class::<PyClassName>()?;
    m.add_class::<PyEnumName>()?;

    Ok(())
}
```

### Checklist
- [ ] Add `m.add_class::<PyClassName>()?;` for each struct
- [ ] Add `m.add_class::<PyEnumName>()?;` for each enum
- [ ] Maintain alphabetical order (optional, for organization)
- [ ] Don't forget the `?` operator (error propagation)

### Common Error
**Forgetting to register**:
```
ImportError: cannot import name 'ClassName' from 'hcvlang'
```

**Fix**: Add `m.add_class::<PyClassName>()?;`

---

## **STEP 4: BUILD**

### Tool
`Bash` tool

### Commands (EXACT)

```bash
cd /home/user/QMNF_System/hcvlang
cargo build --release --features python --lib
```

**Breakdown**:
- `cd /home/user/QMNF_System/hcvlang` - Navigate to hcvlang directory
- `cargo build` - Rust build command
- `--release` - Optimized build (faster runtime, slower compile)
- `--features python` - Enable PyO3 Python bindings (CRITICAL)
- `--lib` - Build library only (not binaries)

**⚠️ CRITICAL**: Use `--features python` NOT `--features pyo3`

The feature is named `python` in Cargo.toml:
```toml
[features]
python = ["pyo3"]
```

### Expected Output (Success)

```
   Compiling hcvlang v0.1.0 (/home/user/QMNF_System/hcvlang)
warning: unused function `some_internal_fn`
  --> src/fhe/something.rs:123:1
   |
123 | fn some_internal_fn() { }
   | ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

... (64 warnings about unused FHE functions - BENIGN) ...

    Finished `release` profile [optimized] target(s) in 0.34s
```

### Expected Timings
- **Clean build**: 15-20 seconds
- **Incremental build** (after first): 0.28-0.45 seconds

### Expected Errors
**ZERO** ✅

### Expected Warnings
~64 warnings about unused functions in FHE modules - **BENIGN** (by design)

### Build Output Location
**File**: `/home/user/QMNF_System/hcvlang/target/release/libhcvlang.so`
- Linux: `libhcvlang.so`
- macOS: `libhcvlang.dylib`
- Windows: `hcvlang.pyd`

This is the Python extension module.

### Common Build Errors

#### Error 1: Feature Not Found
```
error: none of the selected packages contains the `pyo3` feature
```
**Fix**: Use `--features python` not `--features pyo3`

#### Error 2: Bound/Borrow Issues
```
error[E0277]: the trait bound `PyClassName: FromPyObject<'_>` is not satisfied
```
**Fix**: Use `&Bound<'_, PyClassName>` and `.borrow()` pattern

#### Error 3: Missing Registration
```
warning: unused struct `PyClassName`
```
**Fix**: Add `m.add_class::<PyClassName>()?;` to `hcvlang()` function

#### Error 4: Type Mismatch
```
error[E0308]: mismatched types
expected `PyResult<i64>`
found `i64`
```
**Fix**: Wrap return value in `Ok()` or change return type

---

## **STEP 5: CREATE TEST FILE**

### Tool
`Write` tool (new file)

### Location
`/home/user/QMNF_System/tests/python/test_[module]_ffi.py`

### Naming Convention
- File: `test_[module]_ffi.py` (lowercase, underscores)
- Example: `test_prime_operations_ffi.py`

### Template

```python
"""
FFI Tests for [ModuleName]

Tests the Python bindings for Rust [description] operations.
"""

import sys
import os

# Add hcvlang to Python path
sys.path.insert(0, os.path.join(os.path.dirname(__file__),
                                '../../hcvlang/target/release'))

from hcvlang import ClassName, EnumName


def test_construction():
    """Test basic construction."""
    obj = ClassName(10, 20)
    assert obj is not None
    print(f"✓ Construction: {obj}")


def test_method_basic():
    """Test basic method call."""
    obj = ClassName(10, 20)
    result = obj.method(5)
    assert result == 50  # Expected value
    print(f"✓ Basic method: {result}")


def test_method_with_pyclass():
    """Test method taking another PyClass."""
    obj1 = ClassName(10, 20)
    obj2 = ClassName(30, 40)
    result = obj1.method_with_ref(obj2)
    assert result == 1200  # Expected value
    print(f"✓ Method with PyClass: {result}")


def test_enum():
    """Test enum usage."""
    variant = EnumName.Variant1
    obj = ClassName.with_enum(variant)
    assert obj is not None
    print(f"✓ Enum: {variant}")


def test_error_handling():
    """Test error conditions."""
    obj = ClassName(10, 20)
    try:
        obj.fallible_method(-1)  # Should fail
        assert False, "Should have raised exception"
    except RuntimeError as e:
        print(f"✓ Error handling: {e}")


def test_edge_cases():
    """Test boundary conditions."""
    obj = ClassName(0, 0)
    result = obj.method(0)
    assert result == 0

    obj = ClassName(2**31 - 1, 2**31 - 1)  # Max values
    result = obj.method(1)
    print(f"✓ Edge cases: max values handled")


def test_repr():
    """Test string representation."""
    obj = ClassName(10, 20)
    repr_str = repr(obj)
    assert "ClassName" in repr_str
    assert "10" in repr_str
    print(f"✓ __repr__: {repr_str}")


if __name__ == "__main__":
    print("=" * 60)
    print(f"Testing [ModuleName] FFI")
    print("=" * 60)

    test_construction()
    test_method_basic()
    test_method_with_pyclass()
    test_enum()
    test_error_handling()
    test_edge_cases()
    test_repr()

    print("=" * 60)
    print("✅ All [ModuleName] FFI tests passed!")
    print("=" * 60)
```

### Test Categories

1. **Construction Tests**
   - Default construction
   - Construction with parameters
   - Static constructors (if any)

2. **Method Tests**
   - Basic method calls
   - Methods with PyClass arguments
   - Methods returning PyClasses
   - Static methods
   - Class methods

3. **Property Tests**
   - Getters
   - Setters (if mutable)

4. **Enum Tests** (if applicable)
   - All variants
   - Enum comparisons
   - Enum conversions

5. **Error Handling Tests**
   - Invalid arguments
   - Out-of-bounds access
   - Division by zero
   - Other domain-specific errors

6. **Edge Case Tests**
   - Zero values
   - Maximum values (2^31-1, 2^63-1, etc.)
   - Negative values
   - Empty collections

7. **Integration Tests**
   - Using multiple classes together
   - Real-world use cases

### Test Execution (NOT DONE YET)

**Current Practice**: Tests are **created but NOT executed** per user directive:
> "Build first, test later" - velocity sprint philosophy

**Future Execution**:
```bash
cd /home/user/QMNF_System
python3 -m pytest tests/python/test_[module]_ffi.py -v
```

---

## **STEP 6: COMMIT**

### Tool
`Bash` tool

### Commands

```bash
git add -A
git commit -m "feat: Add [ModuleName] FFI - [one-line summary]

[Detailed description]

[Key features as bullet points]

Test: test_[module]_ffi.py ([test status])"
```

### Commit Message Format

**Structure**:
```
feat: Add [ModuleName] FFI - [one-line summary]

[Blank line]

[Detailed description of what was implemented]

[Blank line]

[Key features/classes as bullet points]
- ClassName1: Key methods
- ClassName2: Key methods

[Blank line]

[Important notes, if any]

[Blank line]

Test: test_[module]_ffi.py ([test status])
```

**Example** (Actual commit from this project):
```
feat: Add PrimeOperations & NumberTheoryOps FFI - Phase 2 P2

Implements FFI bindings for prime number and number theory operations:

- PrimeOperations: Miller-Rabin primality testing, Pollard's rho
  factorization, segmented sieve, prime counting
- NumberTheoryOps: Fibonacci, Euler's totient, Möbius function,
  Chinese Remainder Theorem, Jacobi symbol, continued fractions

All operations use exact integer arithmetic (zero floating-point).

Test: test_primes_numbertheory_ffi.py (370 lines, not run yet)
```

### Commit Type Prefixes
- `feat:` - New feature (FFI implementation)
- `fix:` - Bug fix
- `docs:` - Documentation only
- `refactor:` - Code refactoring (no behavior change)
- `test:` - Adding/updating tests
- `perf:` - Performance improvements

### Commit Signing (If Enabled)

If commit signing fails:
```
Error: signing failed: Signing failed: signing operation failed
```

**Fix**: Retry with shorter commit message after 2-second delay:
```bash
sleep 2
git commit -m "feat: Add [ModuleName] FFI"
```

---

## **STEP 7: PUSH**

### Tool
`Bash` tool

### Command

```bash
git push -u origin claude/analyze-ffi-bridge-modules-01T3NT3SyZXpGfLTRFqm9Yx4
```

**⚠️ CRITICAL**: Branch name MUST:
- Start with `claude/`
- End with session ID (e.g., `01T3NT3SyZXpGfLTRFqm9Yx4`)
- Otherwise: 403 Forbidden error

### Expected Output (Success)

```
Enumerating objects: 7, done.
Counting objects: 100% (7/7), done.
Delta compression using up to 8 threads
Compressing objects: 100% (4/4), done.
Writing objects: 100% (4/4), 1.23 KiB | 1.23 MiB/s, done.
Total 4 (delta 3), reused 0 (delta 0), pack-reused 0
To http://127.0.0.1:[port]/git/Skyelabz210/QMNF_System
   abc1234..def5678  claude/analyze-ffi-bridge-modules-01T3NT3SyZXpGfLTRFqm9Yx4 -> claude/analyze-ffi-bridge-modules-01T3NT3SyZXpGfLTRFqm9Yx4
```

### Network Retry Policy

If push fails due to network errors:
```
fatal: unable to access 'http://...': Failed to connect to 127.0.0.1
```

**Retry Strategy**: Exponential backoff
1. Wait 2 seconds → retry
2. Wait 4 seconds → retry
3. Wait 8 seconds → retry
4. Wait 16 seconds → retry
5. After 4 failures, report error

**Implementation**:
```bash
for i in 1 2 4 8; do
    git push -u origin claude/analyze-ffi-bridge-modules-01T3NT3SyZXpGfLTRFqm9Yx4 && break
    echo "Push failed, retrying in ${i}s..."
    sleep $i
done
```

---

## **STEP 8: UPDATE TRACKER**

### Tool
`Edit` tool

### File
`/home/user/QMNF_System/PRIMARY_AGENT_TASK_TRACKER.md`

### Action
Mark task as completed and update status.

**Example**:
```markdown
## 📋 TASK A1: DoubleHelix FFI

**Status**: ✅ COMPLETED (2025-11-14)
**Time Taken**: 4.5 hours
**Lines Added**: 385 lines FFI code
**Classes Implemented**: 8
**Test File**: test_double_helix_ffi.py (420 lines)
**Commit**: abc1234 - "feat: Add DoubleHelix FFI..."
```

---

## 🎯 COMPLETE WORKFLOW EXAMPLE

Here's a real example from this project:

### MultiPrimeRNS Implementation

```bash
# STEP 1: Read source
Read: /home/user/QMNF_System/hcvlang/src/multi_prime_rns.rs
# Identified: RNSValue, MultiPrimeRNS structs
# Methods: encode, decode, add, mul, add_prime

# STEP 2: Write FFI bindings
Edit: /home/user/QMNF_System/hcvlang/src/ffi.rs
# Added PyRNSValue (lines 3721-3790, ~70 lines)
# Added PyMultiPrimeRNS (lines 3791-3984, ~194 lines)
# Total: 264 lines

# STEP 3: Register classes
Edit: /home/user/QMNF_System/hcvlang/src/ffi.rs
# Added to hcvlang() function:
# m.add_class::<PyRNSValue>()?;
# m.add_class::<PyMultiPrimeRNS>()?;

# STEP 4: Build
Bash: cd hcvlang && cargo build --release --features python --lib
# Output: Finished in 0.32s, 0 errors, 64 benign warnings

# STEP 5: Create test
Write: /home/user/QMNF_System/tests/python/test_multiprime_rns_ffi.py
# Created 400 lines of tests

# STEP 6: Commit
Bash: git add -A && git commit -m "feat: Add MultiPrimeRNS FFI bindings..."

# STEP 7: Push
Bash: git push -u origin claude/analyze-ffi-bridge-modules-01T3NT3SyZXpGfLTRFqm9Yx4

# STEP 8: Update tracker
Edit: /home/user/QMNF_System/PRIMARY_AGENT_TASK_TRACKER.md
# Marked Phase 2 P2 as in-progress

# Total Time: ~3-4 hours
```

---

## 🚫 WHAT WE DO **NOT** USE

### ❌ Maturin
**NOT USED** in this project.

**What it is**: A build tool for PyO3 projects that handles building and publishing Python wheels.

**Why we don't use it**: The project uses direct `cargo build` for development.

**If you were using maturin**:
```bash
# Don't do this:
maturin init
maturin develop
maturin build
```

**What we do instead**:
```bash
cargo build --release --features python --lib
```

### ❌ Running Tests
**NOT DONE** during implementation phase.

**Philosophy**: "Build first, test later" - velocity sprint

**Tests are created but not executed** until all FFI implementations are complete.

**Later (after all FFI complete)**:
```bash
cd /home/user/QMNF_System
python3 -m pytest tests/python/ -v
```

### ❌ Python Virtual Environment
**NOT USED** during FFI implementation.

**Why**: System Python is sufficient for building the Rust extension.

**In production deployment**, you would:
```bash
python3 -m venv venv
source venv/bin/activate
pip install -e .
```

### ❌ Type Checking (mypy)
**NOT DONE** during implementation.

**Later**, you could add type stubs:
```python
# hcvlang.pyi
class ClassName:
    def __init__(self, param1: int, param2: int) -> None: ...
    def method(self, arg: int) -> int: ...
```

### ❌ Documentation Generation
**NOT DONE** during implementation (manual docstrings only).

**Later**, you could generate API docs:
```bash
pdoc hcvlang --html --output-dir docs/
```

---

## 📊 METRICS & BENCHMARKS

### Typical Implementation Times
- **Simple struct** (3-5 methods): 1-2 hours
- **Medium struct** (5-10 methods): 2-4 hours
- **Complex struct** (10+ methods, enums, error handling): 4-6 hours
- **Large system** (multiple structs, complex interactions): 6-8 hours

### Code Volume
- **FFI code per class**: 50-150 lines (average ~100)
- **Test code per class**: 50-150 lines (average ~100)
- **Total per class**: 100-300 lines

### Build Times
- **Clean build**: 15-20 seconds
- **Incremental build**: 0.28-0.45 seconds
- **Full rebuild after ffi.rs change**: 0.3-0.5 seconds

---

## 🎓 COMMON PATTERNS CHEATSHEET

### Pattern 1: Simple Method
```rust
fn method(&self, arg: i64) -> i64 {
    self.inner.method(arg)
}
```

### Pattern 2: Method with PyClass Argument (PyO3 0.22+)
```rust
fn method(&self, other: &Bound<'_, PyOtherClass>) -> PyResult<i64> {
    let other_inner = &other.borrow().inner;
    Ok(self.inner.method(other_inner))
}
```

### Pattern 3: Method Returning PyClass
```rust
fn method(&self) -> PyClassName {
    PyClassName {
        inner: self.inner.method(),
    }
}
```

### Pattern 4: Method with Vec of PyClasses
```rust
fn method(&self, items: Vec<PyRef<PyClassName>>) -> i64 {
    let rust_items: Vec<&RustClass> = items.iter()
        .map(|item| &item.inner)
        .collect();
    self.inner.method(&rust_items)
}
```

### Pattern 5: Method Returning Vec of PyClasses
```rust
fn method(&self) -> Vec<PyClassName> {
    self.inner.method()
        .into_iter()
        .map(|item| PyClassName { inner: item })
        .collect()
}
```

### Pattern 6: Static Constructor
```rust
#[staticmethod]
fn from_value(value: i64) -> Self {
    PyClassName {
        inner: RustClassName::from_value(value),
    }
}
```

### Pattern 7: Fallible Method
```rust
fn fallible(&self, arg: i64) -> PyResult<i64> {
    self.inner.fallible(arg)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
}
```

### Pattern 8: Getter/Setter
```rust
#[getter]
fn value(&self) -> i64 {
    self.inner.value
}

#[setter]
fn set_value(&mut self, value: i64) {
    self.inner.value = value;
}
```

---

## ⚠️ CRITICAL GOTCHAS

### 1. Feature Flag Name
**WRONG**: `--features pyo3`
**RIGHT**: `--features python`

### 2. PyO3 0.22+ Bound Pattern
**WRONG**: `&PyClass`
**RIGHT**: `&Bound<'_, PyClass>` + `.borrow()`

### 3. Module Registration
**Don't forget**: `m.add_class::<PyClassName>()?;` in `hcvlang()` function

### 4. Error Propagation
**WRONG**: `fn method(&self) -> i64`
**RIGHT**: `fn method(&self) -> PyResult<i64>` (when calling fallible Rust methods)

### 5. Branch Naming
**WRONG**: Any branch name
**RIGHT**: `claude/[description]-[session_id]`

### 6. Import Path
**Python import**: `from hcvlang import ClassName` (NOT `from qmnf_rust`)

### 7. Test File Location
**WRONG**: `/home/user/QMNF_System/tests/test_module.py`
**RIGHT**: `/home/user/QMNF_System/tests/python/test_module_ffi.py`

---

## 📚 REFERENCE

### PyO3 Documentation
- Official Guide: https://pyo3.rs/v0.22/
- API Reference: https://docs.rs/pyo3/0.22/
- Migration Guide: https://pyo3.rs/v0.22/migration

### Rust Documentation
- Cargo Book: https://doc.rust-lang.org/cargo/
- Rust Edition Guide: https://doc.rust-lang.org/edition-guide/

### Project-Specific
- **Main Guide**: `/home/user/QMNF_System/CLAUDE.md`
- **Architecture**: `/home/user/QMNF_System/SYSTEM_DEVELOPER_GUIDE.md`
- **Task Tracker**: `/home/user/QMNF_System/PRIMARY_AGENT_TASK_TRACKER.md`
- **Parallel Plan**: `/home/user/QMNF_System/PARALLEL_EXECUTION_PLAN.md`

---

## ✅ IMPLEMENTATION CHECKLIST

Use this for each new FFI module:

- [ ] **Step 1**: Read source file (`hcvlang/src/[module].rs`)
- [ ] **Step 2**: Write FFI bindings in `hcvlang/src/ffi.rs`
  - [ ] Add `#[pyclass]` structs
  - [ ] Add `#[pymethods]` implementations
  - [ ] Use `Bound<'_, T>` pattern for PyClass arguments
  - [ ] Add comprehensive docstrings
  - [ ] Add `__repr__` methods
- [ ] **Step 3**: Register classes in `hcvlang()` function
  - [ ] Add `m.add_class::<PyClassName>()?;` for each class
- [ ] **Step 4**: Build with correct command
  - [ ] `cd hcvlang`
  - [ ] `cargo build --release --features python --lib`
  - [ ] Verify 0 errors
- [ ] **Step 5**: Create test file
  - [ ] File: `tests/python/test_[module]_ffi.py`
  - [ ] Test construction
  - [ ] Test all methods
  - [ ] Test error handling
  - [ ] Test edge cases
- [ ] **Step 6**: Commit changes
  - [ ] `git add -A`
  - [ ] `git commit -m "feat: Add [Module] FFI..."`
- [ ] **Step 7**: Push to branch
  - [ ] `git push -u origin claude/[branch]`
- [ ] **Step 8**: Update tracker
  - [ ] Mark task as completed in PRIMARY_AGENT_TASK_TRACKER.md

---

**Last Updated**: 2025-11-14
**PyO3 Version**: 0.22
**Rust Edition**: 2021
**Project**: QMNF System (Quantum-Modular Numerical Framework)

---

**This is the definitive FFI implementation guide. Follow it exactly for consistent, correct implementations.**

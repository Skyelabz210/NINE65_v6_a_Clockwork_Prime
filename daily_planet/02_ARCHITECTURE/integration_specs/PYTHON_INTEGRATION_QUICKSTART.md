# Python Integration Quick Start
**30-Minute Path to Working Python Bindings**

## Critical Path (Execute in Order)

### 1. Fix Module Name (5 min) ⚠️ CRITICAL

**File**: `hcvlang/src/ffi.rs` (line ~4310)

```diff
-#[pymodule]
-fn hcvlang(m: &Bound<'_, PyModule>) -> PyResult<()> {
+#[pymodule]
+fn hcvlang_pyo3(m: &Bound<'_, PyModule>) -> PyResult<()> {
```

**Why**: setup.py expects `hcvlang_pyo3`, but ffi.rs defines `hcvlang` → module won't be found

---

### 2. Rebuild FFI Module (5 min)

```bash
cd /home/user/QMNF_System/hcvlang
cargo build --release --features python --lib
```

**Expected**: `Finished release [optimized] target(s) in ~25s` (0 errors, ~158 warnings)

---

### 3. Install Dependencies (2 min)

```bash
pip3 install setuptools-rust
```

---

### 4. Install Python Module (5 min)

```bash
cd /home/user/QMNF_System
pip3 install -e .
```

**Expected**: `Successfully installed qmnf-1.0.0`

---

### 5. Validate Import (3 min)

```bash
python3 -c "import hcvlang_pyo3; print('✅ SUCCESS: hcvlang_pyo3 imported')"
```

**If fails**: Check error message, verify previous steps

---

### 6. Test Basic Operations (10 min)

```bash
python3 << 'EOF'
import hcvlang_pyo3

# Test CRTBigInt
a = hcvlang_pyo3.CRTBigInt(123)
b = hcvlang_pyo3.CRTBigInt(456)
c = a + b
print(f"✅ CRTBigInt: 123 + 456 = {c}")

# Test Rational
r = hcvlang_pyo3.Rational(22, 7)
print(f"✅ Rational: π ≈ {r}")

# Test FHE
ctx = hcvlang_pyo3.FHEContext(hcvlang_pyo3.SecurityLevel(0))
sk, pk = ctx.generate_keypair()
ct = ctx.encrypt(ctx.encode(42), pk)
result = ctx.decode(ctx.decrypt(ct, sk))
assert result == 42
print(f"✅ FHE: encrypt(42) → decrypt = {result}")

# Test QMNF API
from qmnf.api import QMNFRational
qr = QMNFRational(355, 113)
print(f"✅ QMNF API: π ≈ {qr}")

print("\n🎉 All systems operational!")
EOF
```

---

## Troubleshooting

### Error: `ModuleNotFoundError: No module named 'hcvlang_pyo3'`

**Cause**: Module name mismatch or not installed

**Fix**:
1. Check ffi.rs has `fn hcvlang_pyo3()` (not `fn hcvlang()`)
2. Rebuild: `cargo build --release --features python --lib`
3. Reinstall: `pip3 install -e .`

---

### Error: `ModuleNotFoundError: No module named 'setuptools_rust'`

**Cause**: Missing dependency

**Fix**: `pip3 install setuptools-rust`

---

### Error: `ImportError: ... symbol PyInit_hcvlang not found`

**Cause**: Module name mismatch (built as `hcvlang` instead of `hcvlang_pyo3`)

**Fix**: Verify step 1 (rename function in ffi.rs)

---

### Warning: 158 warnings during build

**Status**: ✅ NORMAL (expected per CLAUDE.md)

**Categories**: Unused imports, unused variables, non-snake-case names

**Action**: None required (non-critical warnings)

---

## Next Steps (After Quick Start)

### Immediate (Next Hour)
1. Run Python test suite: `python3 -m pytest tests/python/ -v`
2. Test neural residue: `python3 tests/python/test_neural_residue_ffi.py`
3. Validate FHE operations (multiplication fix)

### Short-term (Next Day)
1. Migrate deprecated float methods → scaled integer
2. Fix failing tests
3. Run compliance validation

### Long-term (Next Week)
1. Performance benchmarking
2. Documentation updates
3. Pre-existing float cleanup (355+ violations)

---

## Success Criteria Checklist

- [ ] ✅ `import hcvlang_pyo3` succeeds
- [ ] ✅ Basic FFI operations work (CRTBigInt, Rational, FHE)
- [ ] ✅ QMNF API wrapper functional
- [ ] ✅ Neural residue tests passing
- [ ] ✅ FHE multiplication returns correct results (not 0)
- [ ] ✅ Python test suite >80% pass rate

---

## Key Files Reference

| File | Purpose |
|------|---------|
| `hcvlang/src/ffi.rs` | FFI bindings (103 classes) |
| `setup.py` | Python package config |
| `qmnf/__init__.py` | Main Python API |
| `qmnf/api.py` | QMNFRational wrapper |
| `qmnf/conversion_boundary.py` | Float→Rational conversion |
| `qmnf/neural_residue.py` | Neural network wrappers |
| `tests/python/test_neural_residue_ffi.py` | Neural FFI tests |

---

**Full Documentation**: See `PYTHON_INTEGRATION_ROADMAP.md` (27 pages, comprehensive)

**Estimated Time**: 30 minutes (quick path) → 2-4 hours (full integration)

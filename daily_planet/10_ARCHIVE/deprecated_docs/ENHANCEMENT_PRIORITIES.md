# QMNF System - Enhancement Priorities

**Analysis Date**: 2025-11-11
**Status**: Ready for implementation

## 🔴 Critical Blockers (Do First)

### 1. Build Rust Library
```bash
cd hcvlang
cargo build --release --features python
cd ..
pip install -e .
# Verify: python3 -c "import hcvlang_pyo3; print('Success!')"
```

### 2. Fix Float Contamination (356 violations)
```bash
# Check violations
python3 tools/check_no_floats.py

# Focus on: qmnf/arithmetic/, qmnf/storage/, qmnf/neural/
# Replace float literals with QMNFRational(num, denom)
```

### 3. Remove Duplicate Test Files
```bash
# Keep: tests/python/test_suite.py
# Remove: tests/python/test_suite(1).py, tests/python/test_suite(2).py
rm tests/python/test_suite\(1\).py
rm tests/python/test_suite\(2\).py
```

### 4. Add Pre-commit Hook
```bash
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
python3 tools/check_no_floats.py --strict
if [ $? -ne 0 ]; then
    echo "❌ Float contamination detected. Commit rejected."
    exit 1
fi
EOF
chmod +x .git/hooks/pre-commit
```

## 🟠 High Priority (Next)

### 5. File Organization
```bash
# Create directories
mkdir -p scripts examples archive

# Move root Python files
mv *_benchmark*.py scripts/
mv *_dashboard*.py scripts/
mv qmnf_consciousness_*.py archive/
mv spider_gwen_*.py archive/

# Consolidate versioned files
mv QMNF_Unified_Arithmetic_Framework_v4*.py archive/
```

### 6. Documentation Consolidation
```bash
# Create structure
mkdir -p docs/{getting-started,architecture,api,guides,mathematical,reports}

# Move files (selective - review first)
mv PHASE*.md docs/reports/
mv *PERFORMANCE*.md docs/reports/
mv ARCHITECTURE*.md docs/architecture/
```

### 7. Fix CI/CD
Edit `.github/workflows/rust.yml` - add Python integration tests:
```yaml
- name: Build Python package
  run: pip install -e .
- name: Run Python tests
  run: pytest tests/python/ -v
```

## 🟢 Enhancements (After Stabilization)

### 8. MANA Python Wrapper
Create `qmnf/mana_kernel.py` wrapping Rust MANA orchestration

### 9. Storage Backend Unification
Define `StorageInterface` ABC and consolidate backends

### 10. Performance Optimization
- Profile CRTBigInt bottlenecks
- Implement lazy GCD for rationals
- Add batch operations API

## Quick Reference

**Float Violations by Module**:
- `qmnf/arithmetic/`: ~150 violations
- `qmnf/storage/`: ~80 violations
- `qmnf/neural/`: ~50 violations
- `qmnf/frameworks/`: ~40 violations
- Root Python files: ~36 violations

**Key Files to Review**:
- `qmnf_boundary_fixed.py` - Use QMNFRational from here
- `qmnf/conversion_boundary.py` - All float conversions go here
- `qmnf/api.py` - Clean Python API wrapper
- `hcvlang/src/ffi.rs` - Rust↔Python boundary

**Performance Targets**:
- CRTBigInt: Match num-bigint (currently 3x slower)
- Rational: 10x improvement (currently 173x slower)
- Batch operations: 5-10x vs individual calls

---

## Session Handoff Notes

**What's Done**:
- ✅ Comprehensive gap analysis complete
- ✅ Enhancement plan documented
- ✅ Path handling fix committed (e9fb2e0)

**What's Next**:
1. Build Rust library (blocks everything else)
2. Fix float violations (architectural integrity)
3. Organize files (developer experience)

**Estimated Timeline**:
- Week 1-2: Critical path (items 1-4)
- Week 3-4: High priority (items 5-7)
- Week 5+: Enhancements (items 8-10)

Good luck! 🚀

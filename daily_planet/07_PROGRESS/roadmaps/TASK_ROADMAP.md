# QMNF SYSTEM - TASK ROADMAP
## Based on Forensic Audit Findings (2025-11-15)

**Total Tasks:** 56
**Estimated Timeline:** 1 week (Critical) → 18 months (Full Production)

---

## 🔴 CRITICAL PRIORITY - Fix This Week (11 tasks)

**Estimated Time:** 8-12 hours total

### Float Contamination Fixes (4 tasks)
1. ✅ Fix float literal in `qmnf/harmonic_primitives.py` line 678 (`* 2.0 / MODULUS`)
   - **Fix:** Change to `* 2 // MODULUS`
   - **Time:** 5 minutes

2. ✅ Fix float literal in `qmnf/unified_config.py` line 144 (`5**0.5`)
   - **Fix:** Use integer sqrt function
   - **Time:** 10 minutes

3. ✅ Delete duplicate `qmnf/neural/primitives.rs` with float contamination
   - **Issue:** Xavier init uses `f64.sqrt()`
   - **Time:** 2 minutes

4. ✅ Fix float operations in `qmnf/neural/gso.py` metrics functions
   - **Fix:** Move to conversion_boundary or use fixed-point
   - **Time:** 30 minutes

### Critical Bugs (2 tasks)
5. ✅ Fix FFI Polynomial naming collision
   - **Issue:** Two `Polynomial` classes, NNT version shadowed
   - **Fix:** Rename to `NNTPolynomial` or `RationalPolynomial`
   - **Time:** 30 minutes
   - **File:** `hcvlang/src/ffi.rs` lines 3036, 9118

6. ✅ Fix validation tools path resolution bugs
   - **Files:** `tools/check_no_floats.py`, `tools/boundary_validator.py`
   - **Issue:** `relative_to(Path.cwd())` fails on relative paths
   - **Time:** 1 hour

### Documentation Accuracy (5 tasks)
7. ✅ Update CLAUDE.md: Remove or clarify Reed-Solomon claim
   - **Reality:** Only P/Q parity, not full Reed-Solomon
   - **Time:** 15 minutes

8. ✅ Update CLAUDE.md: Remove Energy Systems subsystem
   - **Reality:** Does not exist (vaporware)
   - **Time:** 10 minutes

9. ✅ Update CLAUDE.md: Mark GPU/FPGA domains as 'PLANNED'
   - **Reality:** Not implemented (enum values only)
   - **Time:** 10 minutes

10. ✅ Update CLAUDE.md: FFI class count from 103 to 123
    - **Reality:** 123 classes discovered (+20)
    - **Time:** 5 minutes

11. ✅ Update CLAUDE.md: MANA components from 6 to 9+
    - **Reality:** 9+ components implemented
    - **Time:** 5 minutes

---

## 🟠 HIGH PRIORITY - Fix This Month (14 tasks)

**Estimated Time:** 4-6 weeks

### FFI Expansion (High Impact)
12. 🔥 **Add FFI bindings for RationalMath transcendental functions**
    - **Impact:** CRITICAL - Unlocks sin, cos, exp, ln, sqrt for Python!
    - **Lines:** 1,248 lines currently unreachable
    - **Functions:** ~15 transcendental operations
    - **Time:** 2-3 weeks
    - **Priority:** HIGHEST

13. ✅ Add FFI bindings for MathConstants (π, e, φ, √2)
    - **Time:** 1 week
    - **Impact:** Pre-computed constants for Python users

### Architectural Cleanup
14. ✅ Consolidate QMNFRational exports to single canonical implementation
    - **Issue:** Multiple implementations cause confusion
    - **Fix:** Export only `QMNFRational` from `qmnf/api.py`
    - **Time:** 2-3 days

15. ✅ Locate actual `src/math/rational.rs` implementation
    - **Issue:** Current `rational.rs` is patch file
    - **Time:** 1 day investigation

16. ✅ Create architecture decision record (ADR) for QMNFRational implementations
    - **Time:** 4 hours

### Code Quality
17. ✅ Audit all 15 files using `math` module for float function usage
    - **Replace:** `from math import gcd` with Rust `gcd`
    - **Time:** 1 week

18. ✅ Audit all 8 files using NumPy for dtype compliance
    - **Add:** `assert arr.dtype == np.int64` checks
    - **Time:** 1 week

19. ✅ Add runtime warning for direct `hcvlang_pyo3` imports outside boundary
    - **Security:** Prevent boundary bypass
    - **Time:** 2-3 hours

20. ✅ Add missing operators to QMNFRational API
    - **Missing:** `__floordiv__`, `__mod__`, `sqrt()`, `gcd()`, `lcm()`
    - **Time:** 1 week

### Minor Float Fixes
21. ✅ Fix CoprimeCascade float in stats (`avg_cascade_depth: f64` → `u64`)
    - **Time:** 30 minutes

22. ✅ Fix DynamicalOracle float rates (fixed-point scaled integers)
    - **Fields:** `learning_rate: f32`, `exploration_rate: f32`
    - **Time:** 1 hour

### Integration
23. ✅ Add pre-commit hooks for validation tools
    - **Tool:** `check_no_floats.py`
    - **Time:** 2 hours

24. ✅ Integrate test suite into CI/CD pipeline
    - **Tests:** 450+ Rust, 70 modules, Python suite
    - **Time:** 1 week

25. ✅ Complete AdaptiveCRT integration stubs
    - **Functions:** `generate_primes_for_tier()`, `value_to_residues()`
    - **Time:** 1-2 weeks

---

## 🟡 MEDIUM PRIORITY - Fix This Quarter (17 tasks)

**Estimated Time:** 3-6 months

### FHE Production Hardening (Critical for FHE Production)
26. 🔥 **Implement FHE bootstrapping**
    - **Impact:** CRITICAL - Enables deep circuit evaluation
    - **Status:** Currently stub only
    - **Time:** 2-3 months
    - **Blocker:** Production FHE deployment

27. ✅ Increase FHE modulus size from 2^31-1 to >2^100
    - **Security:** Current modulus too small for production
    - **Time:** 1-2 months
    - **Requires:** Multi-prime RNS implementation

28. ✅ Add FHE rotation/automorphism operations for SIMD
    - **Impact:** Enable SIMD slot operations
    - **Time:** 1-2 months

29. ✅ Replace FHE deterministic LCG with cryptographic RNG
    - **Security:** Current LCG not cryptographically secure
    - **Suggestion:** ChaCha20 or mix with Shadow Entropy
    - **Time:** 2-3 weeks

30. ✅ Validate Shadow Entropy claims with NIST SP 800-22 tests
    - **Claims:** "10-25× faster", "3-7 bits/cycle"
    - **Time:** 1 month

31. ✅ Add benchmarks for FHE operations
    - **Metrics:** Encryption, decryption, homomorphic ops latency
    - **Time:** 2 weeks

32. ✅ Benchmark Shadow Entropy vs ChaCha20
    - **Validate:** Speed claims
    - **Time:** 1 week

### Neural Network Expansion
33. ✅ Implement CNN layers
    - **Impact:** Enable computer vision applications
    - **Time:** 1-2 months

34. ✅ Implement RNN/LSTM/GRU layers
    - **Impact:** Enable sequence modeling
    - **Time:** 1-3 months

35. ✅ Implement Transformer attention mechanisms
    - **Impact:** Enable LLM support
    - **Time:** 2-3 months

36. ✅ Complete CUDA neural network kernels
    - **Missing:** `batch_dense_forward_kernel`, `batch_dense_backward_kernel`
    - **Time:** 2-3 months

37. ✅ Fix IntegerMLP backpropagation
    - **Issue:** Currently only updates last layer
    - **Time:** 1-2 weeks

38. ✅ Add batch FFI operations for neural primitives
    - **Impact:** 4-8× speedup
    - **Time:** 2-3 weeks

### Execution Domains
39. ✅ Implement GPU execution domain for MANA
    - **Status:** Currently enum value only
    - **Time:** 2-4 months

40. ✅ Implement SwarmEPRAM execution engine
    - **Status:** Framework exists, needs engine
    - **Time:** 2-3 months

### Performance Optimization
41. ✅ Optimize HCVLangBigInt division
    - **Current:** O(bits²) bit-level
    - **Target:** Barrett reduction or Newton-Raphson
    - **Time:** 1-2 months

42. ✅ Implement Karatsuba multiplication for HCVLangBigInt
    - **Current:** O(n·m) schoolbook
    - **Target:** O(n^1.58) Karatsuba
    - **Time:** 1-2 months

---

## 🟢 LOW PRIORITY - Future Enhancements (8 tasks)

**Estimated Time:** 6-18 months

43. ✅ Add 3D geometry support
    - **Current:** 2D only
    - **Time:** 1-2 months

44. ✅ Add constant-time crypto variants
    - **Security:** Side-channel resistance
    - **Time:** 2-3 months

45. ✅ Add integration examples for Quantum Modular Superposition
    - **Documentation:** Usage patterns
    - **Time:** 1-2 weeks

46. ✅ Add integration examples for Fractal Hierarchy
    - **Documentation:** Usage patterns
    - **Time:** 1-2 weeks

47. ✅ Add Python type hints and enforce with mypy
    - **Quality:** Type safety
    - **Time:** 2-3 weeks

48. ✅ Create "Which QMNFRational should I use?" guide
    - **Documentation:** User guidance
    - **Time:** 1 week

49. ✅ Add FPGA execution domain implementation
    - **Status:** Planned
    - **Time:** 4-6 months

50. ✅ Add formal verification for critical algorithms
    - **Algorithms:** Garner reconstruction, Binary GCD
    - **Time:** 3-6 months

---

## 📊 BENCHMARKING - After All Issues Resolved (6 tasks)

**Run After:** Critical + High priority fixes complete
**Estimated Time:** 2-3 weeks

51. ✅ Run comprehensive benchmark suite
    - **Scope:** All subsystems
    - **Baseline:** Compare to milestone benchmarks
    - **Time:** 1 week

52. ✅ Validate overall system performance targets
    - **Target:** 40,184 ops/sec baseline (current average)
    - **Components:** Rational, Geometric, GCD intensive
    - **Time:** 3 days

53. ✅ Benchmark FFI overhead
    - **Comparison:** Individual vs batch operations
    - **Expected:** 4-8× speedup for batch
    - **Time:** 2 days

54. ✅ Benchmark adaptive CRT tier transitions
    - **Metrics:** Transition frequency, overhead
    - **Expected:** <0.1% amortized overhead
    - **Time:** 2 days

55. ✅ Benchmark neural networks vs PyTorch integer quantization
    - **Comparison:** Integer-only training performance
    - **Time:** 1 week

56. ✅ Generate final performance comparison report
    - **Format:** Detailed analysis with graphs
    - **Publish:** Update PROJECT_METRICS.md
    - **Time:** 2 days

---

## TIMELINE SUMMARY

| Phase | Duration | Tasks | Deliverable |
|-------|----------|-------|-------------|
| **Critical Fixes** | 1 week | 11 | Bug-free, accurate docs |
| **High Priority** | 1 month | 14 | Enhanced FFI, clean architecture |
| **Medium Priority** | 3 months | 17 | FHE production, neural expansion |
| **Low Priority** | 6 months | 8 | Advanced features |
| **Benchmarking** | 2 weeks | 6 | Performance validation |
| **TOTAL** | **~6-12 months** | **56** | **Full production system** |

---

## EFFORT DISTRIBUTION

```
Critical (Week 1):       8-12 hours
High (Month 1):          4-6 weeks
Medium (Quarter 1):      3-6 months
Low (Future):            6-18 months
Benchmarking:            2-3 weeks
────────────────────────────────────
TOTAL:                   6-18 months (depending on priority selection)
```

---

## DEPENDENCIES

**Critical Path:**
1. Fix critical bugs → Fix float contamination → Update documentation
2. Add RationalMath FFI → Update Python API → Run benchmarks
3. Implement FHE bootstrapping → Increase modulus → Production FHE

**Parallel Workstreams:**
- FFI expansion (RationalMath, MathConstants)
- Neural network architectures (CNN, RNN, Transformers)
- Execution domains (GPU, FPGA, SwarmEPRAM)
- Performance optimization (division, multiplication, batch ops)

---

## QUICK WINS (Do These First)

**Under 1 Hour Each:**
1. Fix FFI naming collision (30 min)
2. Fix float literals (3 × 5-10 min = 30 min)
3. Delete duplicate file (2 min)
4. Update CLAUDE.md counts (5 × 5-10 min = 40 min)

**Total Quick Wins:** ~2 hours, 9 tasks completed

---

## RESOURCE ALLOCATION

**1 Developer, Full-Time:**
- Week 1: Critical fixes ✅
- Month 1-2: High priority (FFI, architecture) ✅
- Month 3-6: FHE production + Neural expansion ✅
- Month 7-12: Execution domains + Performance ✅
- Month 13-18: Advanced features + Polish ✅

**2 Developers, Full-Time:**
- Developer 1: FFI expansion + Neural networks
- Developer 2: FHE production + Execution domains
- **Timeline:** 6-9 months to full production

**Team of 3-4:**
- **Timeline:** 3-6 months to full production

---

## SUCCESS METRICS

**After Critical Fixes (Week 1):**
- ✅ Zero float contamination violations
- ✅ All validation tools working
- ✅ Documentation 100% accurate
- ✅ All critical bugs fixed

**After High Priority (Month 1):**
- ✅ RationalMath accessible from Python
- ✅ Single canonical QMNFRational
- ✅ CI/CD integrated
- ✅ Math/NumPy audited

**After Medium Priority (Quarter 1):**
- ✅ FHE production-ready (with bootstrapping)
- ✅ Neural networks support CNN/RNN
- ✅ GPU execution domain working
- ✅ Performance optimizations complete

**After Benchmarking:**
- ✅ Performance targets validated
- ✅ System ready for production deployment
- ✅ Comprehensive performance documentation

---

**Generated:** 2025-11-15
**Based On:** Master Forensic Audit Report
**Next Review:** After critical fixes (1 week)

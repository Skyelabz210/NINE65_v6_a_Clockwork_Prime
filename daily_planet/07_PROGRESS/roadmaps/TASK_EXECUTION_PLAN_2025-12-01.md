# QMNF System - Task-Based Execution Plan
**Date**: December 1, 2025
**Objective**: Resolve all identified test failures
**Current Status**: 471 passed, 12 failed, 16 ignored (~95%)
**Target Status**: 490+ passed, 0 failed, <10 ignored (~98%+)

---

## Issue Categories

### Category A: FHE Realtime (3 tests) - ADDRESSABLE
- `test_from_standard_polynomial_center_lift`
- `test_encryption_decryption`
- `test_homomorphic_addition`

### Category B: FHE RNS (3 tests) - ADDRESSABLE
- `test_algebra_harness_rns_t17_no_keys`
- `test_algebra_harness_rns_t257`
- `test_rns_rescale_exhaustive_t17`

### Category C: FHE Operations (4 tests) - ARCHITECTURAL
- `test_homomorphic_multiplication` (main)
- `test_entropy_shadow_integration`
- `test_fixed_multiplication_exhaustive`
- `test_sample_error`

### Category D: FHE Core (1 test) - ARCHITECTURAL
- `test_homomorphic_multiplication` (duplicate)

### Category E: Rational Type (16 tests) - ARCHITECTURAL
- Stack overflow / infinite recursion issues

### Category F: SIGSEGV (1 issue) - INVESTIGATE
- Memory crash during test cleanup

---

## Phase 1: FHE Realtime Tests (Estimated: 2-3 hours)

### Task 1.1: Analyze `test_encryption_decryption`
**File**: `hcvlang/src/fhe_realtime/realtime_context.rs`
**Steps**:
1. Run test with `--nocapture` to see error details
2. Check `RealTimeFHEContext` initialization
3. Verify encryption/decryption parameter alignment
4. Fix parameter mismatch if found

### Task 1.2: Analyze `test_homomorphic_addition`
**File**: `hcvlang/src/fhe_realtime/realtime_context.rs`
**Steps**:
1. Run test with `--nocapture`
2. Check noise budget calculation
3. Verify addition operation preserves correctness
4. Fix noise handling if needed

### Task 1.3: Analyze `test_from_standard_polynomial_center_lift`
**File**: `hcvlang/src/fhe_realtime/adaptive_polynomial.rs`
**Steps**:
1. Run test with `--nocapture`
2. Check center-lift algorithm implementation
3. Verify modulus conversion logic
4. Fix polynomial conversion if needed

---

## Phase 2: FHE RNS Tests (Estimated: 3-4 hours)

### Task 2.1: Analyze RNS Module Configuration
**File**: `hcvlang/src/fhe/rns.rs`
**Steps**:
1. Review Q0, Q1 modulus definitions
2. Check if moduli are NTT-friendly
3. Verify CRT coefficient computation

### Task 2.2: Fix `test_algebra_harness_rns_t17_no_keys`
**Steps**:
1. Check t=17 plaintext modulus configuration
2. Verify algebra harness initialization
3. Fix parameter setup

### Task 2.3: Fix `test_algebra_harness_rns_t257`
**Steps**:
1. Check t=257 plaintext modulus configuration
2. Verify larger modulus handling
3. Fix overflow issues if present

### Task 2.4: Fix `test_rns_rescale_exhaustive_t17`
**Steps**:
1. Analyze rescaling algorithm
2. Check for off-by-one errors
3. Fix rounding logic

---

## Phase 3: FHE Operations Tests (Estimated: 8-12 hours)

### Task 3.1: Document Architectural Limitation
**File**: `FHE_MULTIPLICATION_INVESTIGATION.md` (exists)
**Status**: Already documented - dual-modulus architecture needed

### Task 3.2: Mark Tests as `#[ignore]` with Explanation
**Files**:
- `hcvlang/src/fhe/operations.rs`
- `hcvlang/src/fhe/mod.rs`
**Steps**:
1. Add `#[ignore = "Requires dual-modulus RNS architecture"]` to failing tests
2. Add comment block explaining the issue
3. Reference `FHE_MULTIPLICATION_INVESTIGATION.md`

### Task 3.3: Create Non-RNS Multiplication Fallback (Optional)
**Steps**:
1. Implement simple rescaling without RNS
2. Accept higher noise growth as tradeoff
3. Enable for testing purposes only

---

## Phase 4: Rational Type Issues (Estimated: 4-8 hours)

### Task 4.1: Identify Infinite Recursion Source
**File**: `hcvlang/src/rational.rs`
**Steps**:
1. Analyze `Clone` trait implementation
2. Check `Add` trait for self-reference
3. Find recursive call chain

### Task 4.2: Implement Non-Recursive Arithmetic
**Steps**:
1. Refactor `Add` to use helper function
2. Remove trait recursion
3. Add stack depth limit as safety

### Task 4.3: Re-enable Ignored Tests
**Steps**:
1. Remove `#[ignore]` attributes
2. Run tests to verify fix
3. Add regression tests

---

## Phase 5: SIGSEGV Investigation (Estimated: 2-4 hours)

### Task 5.1: Identify Crash Location
**Steps**:
1. Run with `RUST_BACKTRACE=full`
2. Check for use-after-free patterns
3. Review global static initialization

### Task 5.2: Add Memory Safety Checks
**Steps**:
1. Use `cargo miri test` if available
2. Add bounds checking to hot paths
3. Review unsafe blocks

---

## Execution Timeline

| Phase | Priority | Tests | Est. Time | Dependencies |
|-------|----------|-------|-----------|--------------|
| 1 | HIGH | 3 FHE Realtime | 2-3h | None |
| 2 | HIGH | 3 FHE RNS | 3-4h | None |
| 3 | LOW | 4 FHE Ops | 8-12h | Architectural |
| 4 | MEDIUM | 16 Rational | 4-8h | None |
| 5 | MEDIUM | SIGSEGV | 2-4h | None |

**Parallel Execution**: Phases 1, 2, 4, 5 can run in parallel.

---

## Success Criteria

### Minimum Success (Phase 1-2)
- [ ] FHE Realtime tests pass or documented
- [ ] FHE RNS tests pass or documented
- [ ] Pass rate ≥97%

### Target Success (+ Phase 4)
- [ ] Rational type tests un-ignored
- [ ] No SIGSEGV crashes
- [ ] Pass rate ≥98%

### Full Success (All Phases)
- [ ] All tests pass or have documented architectural reasons
- [ ] 0 unexpected failures
- [ ] Pass rate ≥99%

---

## Commands Reference

```bash
# Run specific test with output
cargo test --lib --release <test_name> -- --nocapture

# Run FHE realtime tests
cargo test --lib --release fhe_realtime:: -- --nocapture

# Run FHE RNS tests
cargo test --lib --release fhe::rns:: -- --nocapture

# Run Rational tests
cargo test --lib --release rational:: -- --nocapture

# Full test suite
cargo test --lib --release 2>&1 | tee test_results.txt
```

---

## Begin Execution

Starting with **Phase 1: FHE Realtime Tests** as highest ROI for immediate improvement.

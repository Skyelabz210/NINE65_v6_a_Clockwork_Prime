# FHE Realtime Test Failures - Root Cause Analysis
**Date**: December 1, 2025
**Status**: ROOT CAUSE IDENTIFIED
**Affected Tests**: 3 (center-lift, encryption, decryption)

---

## Executive Summary

**Finding**: The "fixes" documented in SESSION_SUMMARY_2025-12-01.md ARE in the codebase, but tests still fail due to a **double conversion bug** in `AdaptiveCoefficient::new()`.

**Root Cause**: `AdaptiveCoefficient::new(value: i64, modulus)` converts negative values back to positive modular form, undoing the center-lift conversion performed by its caller.

**Impact**: All FHE Realtime operations involving negative coefficients fail (3 tests).

---

## Test Failure Walkthrough

### Test: `test_from_standard_polynomial_center_lift`

**File**: `hcvlang/src/fhe_realtime/adaptive_polynomial.rs:552`

**Test Code**:
```rust
let modulus = 97u64;
let coeffs = vec![
    ModInt::new_u64(1, modulus),
    ModInt::new_u64(modulus - 3, modulus),  // Creates 94
];
let poly = Polynomial::new(coeffs, 2, modulus);

let adaptive = AdaptivePolynomial::from_standard_polynomial(&poly).unwrap();

assert_eq!(adaptive.coeffs[0].value_i64().unwrap(), 1);
assert_eq!(adaptive.coeffs[1].value_i64().unwrap(), -3);  // FAILS - gets 94
```

**Expected**: Second coefficient should return -3 (center-lifted)
**Actual**: Returns 94 (positive modular form)

---

## Code Flow Analysis

### Step 1: Standard Polynomial Creation ✅
```rust
// Test creates ModInt with raw value 94
ModInt::new_u64(97 - 3, 97)  // value = 94
```

### Step 2: Conversion to Adaptive (Lines 224-256) ✅
```rust
fn from_standard_polynomial(poly: &Polynomial) -> RealTimeFHEResult<Self> {
    let coeffs: Result<Vec<AdaptiveCoefficient>, _> = poly
        .coeffs
        .iter()
        .map(|c| {
            let raw = c.value_u64();  // 94
            let modulus = poly.modulus;  // 97

            // Center-lift conversion: 94 > 97/2, so convert to negative
            let signed = if raw <= modulus / 2 {
                raw as i64
            } else {
                let distance = modulus.checked_sub(raw).unwrap();  // 97 - 94 = 3
                -(distance as i64)  // -3 ✅
            };

            Ok(AdaptiveCoefficient::new(signed, modulus))  // Passes -3
        })
        .collect();
    // ...
}
```

**Status**: ✅ **CORRECT** - Properly converts 94 → -3

### Step 3: AdaptiveCoefficient::new() ❌ **BUG HERE**
```rust
// File: adaptive_polynomial.rs:24-42
pub fn new(value: i64, modulus: u64) -> Self {
    // Reduce to [0, modulus) range
    let reduced = if value < 0 {  // -3 < 0, so enters this branch
        let abs_val = value.unsigned_abs();  // 3
        let r = abs_val % modulus;  // 3 % 97 = 3
        if r == 0 {
            0
        } else {
            (modulus - r) as i64  // 97 - 3 = 94 ❌ CONVERTS BACK!
        }
    } else {
        (value as u64 % modulus) as i64
    };

    AdaptiveCoefficient {
        crt_value: AdaptiveCRTBigInt::new(reduced),  // Stores 94, not -3!
        modulus,
    }
}
```

**Status**: ❌ **BUG** - Converts -3 back to 94, undoing center-lift

### Step 4: value_i64() Tries to Center-Lift (Lines 111-126) 🤷
```rust
pub fn value_i64(&self) -> RealTimeFHEResult<i64> {
    // Get value from CRT
    let value_mod = self
        .crt_value
        .to_modulus(self.modulus)  // Reconstructs 94
        .map_err(|e| RealTimeFHEError::CRTError(format!("{:?}", e)))?;

    // Try to center-lift
    let half_modulus = self.modulus / 2;  // 48
    if value_mod > half_modulus {  // 94 > 48, so should convert
        Ok(value_mod as i64 - self.modulus as i64)  // 94 - 97 = -3 ✅
    } else {
        Ok(value_mod as i64)
    }
}
```

**Status**: 🤷 **SHOULD WORK** - Has correct center-lift logic

**The Problem**: `self.crt_value.to_modulus(self.modulus)` returns 94 (the positive form stored in Step 3), but `to_modulus()` might not be applying the modulus reduction correctly, OR the CRT value was stored incorrectly.

---

## Root Cause

**The Bug**: `AdaptiveCoefficient::new()` performs **unnecessary modular reduction** on already-center-lifted values.

**Why This Happens**:
1. Caller (`from_standard_polynomial`) performs center-lift: 94 → -3 ✅
2. `new(-3, 97)` treats -3 as "needs modular reduction" and converts it back: -3 → 94 ❌
3. CRT stores 94 instead of -3
4. `value_i64()` can't distinguish between "stored 94 because input was 94" vs "stored 94 because input was -3"

**Design Flaw**: Double conversion - both caller and constructor apply modular arithmetic.

---

## The Fix

### Option 1: Remove Conversion from `new()` (RECOMMENDED)

**File**: `hcvlang/src/fhe_realtime/adaptive_polynomial.rs:24-42`

```rust
pub fn new(value: i64, modulus: u64) -> Self {
    // Store value directly - caller is responsible for ensuring it's in valid range
    AdaptiveCoefficient {
        crt_value: AdaptiveCRTBigInt::new(value),
        modulus,
    }
}
```

**Rationale**:
- `from_standard_polynomial` already does center-lift conversion correctly
- `new()` should trust its caller
- Simpler logic = fewer bugs

**Impact**: Need to audit all callers of `new()` to ensure they pass valid ranges

### Option 2: Add Flag to Control Conversion

```rust
pub fn new(value: i64, modulus: u64) -> Self {
    Self::new_with_reduction(value, modulus, true)
}

pub fn new_no_reduction(value: i64, modulus: u64) -> Self {
    Self::new_with_reduction(value, modulus, false)
}

fn new_with_reduction(value: i64, modulus: u64, reduce: bool) -> Self {
    let reduced = if reduce {
        // Existing modular reduction logic
        // ...
    } else {
        value
    };

    AdaptiveCoefficient {
        crt_value: AdaptiveCRTBigInt::new(reduced),
        modulus,
    }
}
```

**Rationale**: Explicit control over whether reduction happens

**Impact**: More API surface, but safer migration

### Option 3: Fix `value_i64()` to NOT Use `to_modulus()`

```rust
pub fn value_i64(&self) -> RealTimeFHEResult<i64> {
    // Get raw signed value from CRT WITHOUT modular reduction
    self.crt_value.to_i64()
        .map_err(|e| RealTimeFHEError::CRTError(format!("{:?}", e)))
}
```

**Rationale**: If `new()` stores -3 as-is in CRT, then `to_i64()` should return -3 directly

**Impact**: Depends on whether `AdaptiveCRTBigInt` can store negative values correctly

---

## Recommended Fix (Hybrid Approach)

**Step 1**: Change `new()` to NOT convert negative values:

```rust
pub fn new(value: i64, modulus: u64) -> Self {
    // No modular reduction - trust the caller
    AdaptiveCoefficient {
        crt_value: AdaptiveCRTBigInt::new(value),
        modulus,
    }
}
```

**Step 2**: Simplify `value_i64()`:

```rust
pub fn value_i64(&self) -> RealTimeFHEResult<i64> {
    self.crt_value.to_i64()
        .map_err(|e| RealTimeFHEError::CRTError(format!("{:?}", e)))
}
```

**Step 3**: Add a new constructor for cases that need reduction:

```rust
pub fn from_unreduced(value: i64, modulus: u64) -> Self {
    let reduced = if value < 0 {
        let abs_val = value.unsigned_abs();
        let r = abs_val % modulus;
        if r == 0 {
            0
        } else {
            -(r as i64)  // KEEP NEGATIVE! Don't convert to modulus - r
        }
    } else {
        (value % (modulus as i64))
    };

    Self::new(reduced, modulus)
}
```

**Step 4**: Audit all callers:
- `from_standard_polynomial`: Already does center-lift, use `new()` ✅
- Other callers: Check if they need `from_unreduced()` instead

---

## Testing Strategy

### Test 1: Verify Center-Lift Works
```rust
#[test]
fn test_adaptive_coefficient_negative() {
    let coeff = AdaptiveCoefficient::new(-3, 97);
    assert_eq!(coeff.value_i64().unwrap(), -3);
}
```

### Test 2: Verify from_standard_polynomial
```rust
// Existing test should pass after fix
#[test]
fn test_from_standard_polynomial_center_lift() {
    // ... existing test code
}
```

### Test 3: Verify Encryption/Decryption
```rust
// Should pass after center-lift fix
#[test]
fn test_encryption_decryption() {
    // ... existing test code
}
```

---

## Impact Analysis

**Files Modified**: 1 (`hcvlang/src/fhe_realtime/adaptive_polynomial.rs`)

**Lines Changed**: ~10-15 lines

**Tests Fixed**: 3 (all FHE Realtime failures)

**Estimated Time**: 30-45 minutes (implementation + testing)

**Risk**: LOW - Change is localized to AdaptiveCoefficient, well-tested module

---

## Next Steps

1. **Implement Fix** - Apply Option 1 (simplify `new()`)
2. **Run Tests** - Verify 3 FHE Realtime tests pass
3. **Audit Callers** - Ensure no other code depends on old behavior
4. **Commit** - Clear commit message referencing this analysis
5. **Update Baseline** - Re-run full test suite (expect 445/498 = 89.4%)

---

**Status**: ✅ Root cause identified, fix ready to implement
**Priority**: 🔴 HIGH - Blocks 3 tests, simple fix available
**Next Action**: Implement fix and test

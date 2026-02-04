#!/bin/bash
# FPD Regression Scanner
# Detects stdlib patterns that indicate innovation regression
# Usage: ./scan.sh src/

set -e

SRC_DIR="${1:-src/}"
SCRIPT_DIR="$(dirname "$0")"

echo "═══════════════════════════════════════════════════════════════════"
echo "  FPD REGRESSION SCANNER"
echo "  Scanning: $SRC_DIR"
echo "═══════════════════════════════════════════════════════════════════"
echo ""

FAILED=0

# ═══════════════════════════════════════════════════════════════════
# FORBIDDEN PATTERNS (must NOT appear)
# ═══════════════════════════════════════════════════════════════════

echo "Checking FORBIDDEN patterns..."
echo ""

# Float types
if grep -rn "f64\|f32" "$SRC_DIR" --include="*.rs" 2>/dev/null | grep -v "//.*f64\|//.*f32\|#\[cfg\|test" | head -5; then
    echo "  ✗ FORBIDDEN: Float types (f64, f32) found"
    FAILED=1
else
    echo "  ✓ No float types found"
fi

# Stdlib transcendentals
if grep -rn "\.exp()\|\.ln()\|\.log()\|\.sin()\|\.cos()" "$SRC_DIR" --include="*.rs" 2>/dev/null | grep -v "//\|#\[cfg\|test" | head -5; then
    echo "  ✗ FORBIDDEN: Stdlib transcendentals found"
    FAILED=1
else
    echo "  ✓ No stdlib transcendentals found"
fi

# Float conversion
if grep -rn "\.to_f64()\|\.as_f64()\|\.to_f32()\|\.as_f32()" "$SRC_DIR" --include="*.rs" 2>/dev/null | grep -v "//\|#\[cfg\|test" | head -5; then
    echo "  ✗ FORBIDDEN: Float conversion found"
    FAILED=1
else
    echo "  ✓ No float conversion found"
fi

# Panic in hot paths (non-test code)
if grep -rn "panic!\|\.expect(\|\.unwrap()" "$SRC_DIR" --include="*.rs" 2>/dev/null | grep -v "test\|#\[cfg(test)\|mod tests" | head -5; then
    echo "  ⚠ WARNING: Potential panics in non-test code (review manually)"
else
    echo "  ✓ No obvious panic paths in non-test code"
fi

echo ""

# ═══════════════════════════════════════════════════════════════════
# REQUIRED PATTERNS (MUST appear after implementation)
# ═══════════════════════════════════════════════════════════════════

echo "Checking REQUIRED patterns..."
echo ""

# These are checked after implementation begins
# For now, just note which are missing

check_required() {
    PATTERN="$1"
    NAME="$2"
    if grep -rn "$PATTERN" "$SRC_DIR" --include="*.rs" 2>/dev/null | head -1 > /dev/null; then
        echo "  ✓ Found: $NAME"
    else
        echo "  ○ Missing: $NAME (implement in corresponding task)"
    fi
}

check_required "binary_gcd" "Binary GCD (T-002)"
check_required "ModResidue" "ModResidue type (T-001)"
check_required "DivStatus" "DivStatus enum (T-001)"
check_required "AnchorSet" "AnchorSet type (T-003)"
check_required "mod_div" "Unified API (T-010)"
check_required "crt_reconstruct" "CRT Reconstruction (T-009)"

echo ""

# ═══════════════════════════════════════════════════════════════════
# SUMMARY
# ═══════════════════════════════════════════════════════════════════

echo "═══════════════════════════════════════════════════════════════════"
if [ $FAILED -eq 0 ]; then
    echo "  REGRESSION STATUS: ✓ CLEAN"
    echo "  No forbidden patterns detected"
else
    echo "  REGRESSION STATUS: ✗ REGRESSION DETECTED"
    echo "  Fix forbidden patterns before proceeding"
fi
echo "═══════════════════════════════════════════════════════════════════"

exit $FAILED

#!/bin/bash
#
# NTT Optimization Regression Scanner
#
# Scans source files for forbidden patterns and verifies required patterns
#
# Usage: ./scan.sh [directory]
#

set -e

TARGET_DIR="${1:-crates/nine65/src/arithmetic}"

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║              NTT OPTIMIZATION REGRESSION SCAN                 ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Scanning: $TARGET_DIR"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

FAILED=0

# ============================================================================
# FORBIDDEN PATTERNS
# ============================================================================

echo "═══════════════════════════════════════════════════════════════"
echo "FORBIDDEN PATTERNS (must NOT appear in NTT code)"
echo "═══════════════════════════════════════════════════════════════"

check_forbidden() {
    local pattern="$1"
    local description="$2"
    
    local count=$(grep -r "$pattern" "$TARGET_DIR" --include="*.rs" 2>/dev/null | grep -v "test" | grep -v "//" | wc -l)
    
    if [ "$count" -gt 0 ]; then
        echo -e "[ ${RED}FAIL${NC} ] $description"
        echo "         Found $count occurrences of: $pattern"
        grep -rn "$pattern" "$TARGET_DIR" --include="*.rs" | grep -v "test" | grep -v "//" | head -3
        FAILED=1
    else
        echo -e "[ ${GREEN}PASS${NC} ] $description"
    fi
}

check_forbidden "f64" "No f64 floating point types"
check_forbidden "f32" "No f32 floating point types"
check_forbidden "as u128 \* .* as u128) % .* as u128" "No naive u128 modular reduction in hot path"
check_forbidden "\.clone()" "No .clone() in hot path (check NTT loops)"

echo ""

# ============================================================================
# REQUIRED PATTERNS
# ============================================================================

echo "═══════════════════════════════════════════════════════════════"
echo "REQUIRED PATTERNS (MUST appear)"
echo "═══════════════════════════════════════════════════════════════"

check_required() {
    local pattern="$1"
    local description="$2"
    
    local found=$(grep -r "$pattern" "$TARGET_DIR" --include="*.rs" 2>/dev/null | head -1)
    
    if [ -n "$found" ]; then
        echo -e "[ ${GREEN}PASS${NC} ] $description"
        echo "         Found in: $(echo "$found" | cut -d: -f1)"
    else
        echo -e "[ ${RED}FAIL${NC} ] $description"
        echo "         Pattern not found: $pattern"
        FAILED=1
    fi
}

check_required "montgomery_mul" "Montgomery multiplication present"
check_required "lazy_reduce\|harvey" "Lazy reduction or Harvey butterfly"
check_required "bit_rev\|bit_reverse" "Bit-reversal implementation"
check_required "twiddle" "Twiddle factors"

echo ""

# ============================================================================
# OPTIONAL PATTERNS (warnings only)
# ============================================================================

echo "═══════════════════════════════════════════════════════════════"
echo "OPTIONAL PATTERNS (recommendations)"
echo "═══════════════════════════════════════════════════════════════"

check_optional() {
    local pattern="$1"
    local description="$2"
    
    local found=$(grep -r "$pattern" "$TARGET_DIR" --include="*.rs" 2>/dev/null | head -1)
    
    if [ -n "$found" ]; then
        echo -e "[ ${GREEN}FOUND${NC} ] $description"
    else
        echo -e "[ ${YELLOW}WARN${NC}  ] $description"
        echo "         Consider adding: $pattern"
    fi
}

check_optional "#\[inline(always)\]" "Inline hints on hot functions"
check_optional "target_feature.*avx512" "AVX-512 SIMD support"
check_optional "debug_assert" "Debug assertions for safety"

echo ""

# ============================================================================
# SUMMARY
# ============================================================================

echo "═══════════════════════════════════════════════════════════════"
if [ "$FAILED" -eq 0 ]; then
    echo -e "RESULT: ${GREEN}✓ CLEAN${NC}"
    echo "All checks passed."
else
    echo -e "RESULT: ${RED}✗ REGRESSION DETECTED${NC}"
    echo "Fix the issues above before proceeding."
fi
echo "═══════════════════════════════════════════════════════════════"

exit $FAILED

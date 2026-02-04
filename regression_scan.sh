#!/bin/bash
# NINE65 MANA Regression Scanner
# Detects patterns that indicate stdlib regression after gap closure

echo "═══════════════════════════════════════════════════════════════"
echo "          NINE65 MANA Regression Scan - Gap Closure            "
echo "═══════════════════════════════════════════════════════════════"

NINE65_SRC="crates/nine65/src"

# Check for FORBIDDEN patterns (indicate regression)
echo ""
echo "🔴 CHECKING FORBIDDEN PATTERNS..."

FORBIDDEN_COUNT=0

# Old naive division patterns in encrypt.rs (actual code, not comments)
if grep -v "//" "$NINE65_SRC/ops/encrypt.rs" 2>/dev/null | grep -q "numerator / denominator"; then
    echo "   ❌ FOUND: naive division in encrypt.rs"
    FORBIDDEN_COUNT=$((FORBIDDEN_COUNT + 1))
fi

if grep -v "//" "$NINE65_SRC/ops/encrypt.rs" 2>/dev/null | grep -qE "/ \(2u128 \* self\.q"; then
    echo "   ❌ FOUND: old degree-2 decode pattern"
    FORBIDDEN_COUNT=$((FORBIDDEN_COUNT + 1))
fi

# Floating point in critical paths
if grep -qE "f64|f32" "$NINE65_SRC/ops/"*.rs 2>/dev/null | grep -v test | grep -v "//"; then
    echo "   ❌ FOUND: floating point in ops/"
    FORBIDDEN_COUNT=$((FORBIDDEN_COUNT + 1))
fi

if [ $FORBIDDEN_COUNT -eq 0 ]; then
    echo "   ✅ No forbidden patterns found"
fi

# Check for REQUIRED patterns (must be present)
echo ""
echo "🟢 CHECKING REQUIRED PATTERNS..."

REQUIRED_COUNT=0
REQUIRED_MISSING=0

# K-Elimination in BFVEncoder
if grep -q "ke: KElimination" "$NINE65_SRC/ops/encrypt.rs" 2>/dev/null; then
    echo "   ✅ KElimination field in BFVEncoder"
    REQUIRED_COUNT=$((REQUIRED_COUNT + 1))
else
    echo "   ❌ MISSING: KElimination field in BFVEncoder"
    REQUIRED_MISSING=$((REQUIRED_MISSING + 1))
fi

# K-Elimination usage in decode
if grep -q "ke.scale_and_round" "$NINE65_SRC/ops/encrypt.rs" 2>/dev/null; then
    KE_COUNT=$(grep -c "ke.scale_and_round" "$NINE65_SRC/ops/encrypt.rs")
    echo "   ✅ ke.scale_and_round used $KE_COUNT times in encrypt.rs"
    REQUIRED_COUNT=$((REQUIRED_COUNT + 1))
else
    echo "   ❌ MISSING: ke.scale_and_round in encrypt.rs"
    REQUIRED_MISSING=$((REQUIRED_MISSING + 1))
fi

# K-Elimination in homomorphic.rs
if grep -q "ke.scale_and_round\|self.ke" "$NINE65_SRC/ops/homomorphic.rs" 2>/dev/null; then
    echo "   ✅ K-Elimination present in homomorphic.rs"
    REQUIRED_COUNT=$((REQUIRED_COUNT + 1))
else
    echo "   ❌ MISSING: K-Elimination in homomorphic.rs"
    REQUIRED_MISSING=$((REQUIRED_MISSING + 1))
fi

# Shadow Entropy
if grep -q "ShadowHarvester" "$NINE65_SRC/entropy/"*.rs 2>/dev/null; then
    echo "   ✅ ShadowHarvester present"
    REQUIRED_COUNT=$((REQUIRED_COUNT + 1))
fi

# MobiusInt
if grep -q "MobiusInt" "$NINE65_SRC/arithmetic/"*.rs 2>/dev/null; then
    echo "   ✅ MobiusInt present"
    REQUIRED_COUNT=$((REQUIRED_COUNT + 1))
fi

# NTTEngine
if grep -q "NTTEngine\|NTTEngineFFT" "$NINE65_SRC/arithmetic/"*.rs 2>/dev/null; then
    echo "   ✅ NTTEngine present"
    REQUIRED_COUNT=$((REQUIRED_COUNT + 1))
fi

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "                         SUMMARY                               "
echo "═══════════════════════════════════════════════════════════════"
echo "   Forbidden patterns found: $FORBIDDEN_COUNT"
echo "   Required patterns found:  $REQUIRED_COUNT"
echo "   Required patterns missing: $REQUIRED_MISSING"

if [ $FORBIDDEN_COUNT -eq 0 ] && [ $REQUIRED_MISSING -eq 0 ]; then
    echo ""
    echo "   ✅ REGRESSION SCAN PASSED"
    exit 0
else
    echo ""
    echo "   ❌ REGRESSION SCAN FAILED"
    exit 1
fi

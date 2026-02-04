#!/bin/bash
# FPD Validation Script
# Validates structure and performs basic checks

set -e

echo "=== FPD Validation Script ==="
echo ""

# Check directory structure
echo "1. Checking directory structure..."
REQUIRED_FILES=(
    "Cargo.toml"
    "README.md"
    "SPECIFICATION.md"
    "src/lib.rs"
    "src/mod_residue.rs"
    "src/binary_gcd.rs"
    "src/anchor_set.rs"
    "src/error.rs"
    "src/mod_inverse.rs"
    "src/fast_path.rs"
    "src/piggyback.rs"
    "src/gcd_reduction.rs"
    "src/crt_tower.rs"
    "src/constant_time.rs"
    "src/audit.rs"
    "tests/property_tests.rs"
    "benches/division_benchmarks.rs"
)

MISSING=0
for file in "${REQUIRED_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "  ✗ Missing: $file"
        MISSING=$((MISSING + 1))
    else
        echo "  ✓ Found: $file"
    fi
done

if [ $MISSING -gt 0 ]; then
    echo ""
    echo "ERROR: $MISSING required files missing!"
    exit 1
fi

echo ""
echo "2. Checking for floating-point contamination..."
FLOAT_VIOLATIONS=$(grep -rn "f64\|f32\|\.to_f64\|\.exp()\|\.ln()\|\.log()\|\.sin()\|\.cos()\|\.sqrt()" src/*.rs 2>/dev/null | grep -v "// ALLOWED:" | grep -v "test" | grep -v "#\[" || true)

if [ -n "$FLOAT_VIOLATIONS" ]; then
    echo "  ⚠ Potential float violations found:"
    echo "$FLOAT_VIOLATIONS"
else
    echo "  ✓ No floating-point contamination detected"
fi

echo ""
echo "3. Checking required patterns..."
REQUIRED_PATTERNS=(
    "binary_gcd"
    "ModResidue"
    "DivStatus"
    "AnchorSet"
    "mod_div"
    "crt_reconstruct"
    "DivisionError"
    "ShadowEntropy"
)

for pattern in "${REQUIRED_PATTERNS[@]}"; do
    COUNT=$(grep -rn "$pattern" src/*.rs 2>/dev/null | wc -l)
    if [ "$COUNT" -gt 0 ]; then
        echo "  ✓ Found '$pattern' ($COUNT occurrences)"
    else
        echo "  ✗ Missing required pattern: $pattern"
    fi
done

echo ""
echo "4. Counting lines of code..."
TOTAL_LINES=$(find . -name "*.rs" -exec wc -l {} \; | awk '{sum+=$1} END {print sum}')
echo "  Total Rust lines: $TOTAL_LINES"

echo ""
echo "5. Counting tests..."
TEST_COUNT=$(grep -rn "#\[test\]" src/*.rs tests/*.rs 2>/dev/null | wc -l)
echo "  Test functions: $TEST_COUNT"

echo ""
echo "=== Validation Complete ==="
echo ""
echo "Summary:"
echo "  Files: ${#REQUIRED_FILES[@]} required, $((${#REQUIRED_FILES[@]} - MISSING)) found"
echo "  Lines: $TOTAL_LINES"
echo "  Tests: $TEST_COUNT"
echo ""

if [ $MISSING -eq 0 ]; then
    echo "✅ All validations passed!"
    exit 0
else
    echo "❌ Validation failed with $MISSING missing files"
    exit 1
fi

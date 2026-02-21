#!/bin/bash
# test_cross_build.sh — Verify that the harness compiles against all builds
#
# Usage:
#   ./scripts/test_cross_build.sh           # cargo check all builds
#   ./scripts/test_cross_build.sh --test    # cargo test all builds

set -euo pipefail

HARNESS_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$HARNESS_DIR"

CMD="${1:-check}"
case "$CMD" in
    --test) CARGO_CMD="test --lib" ;;
    *)      CARGO_CMD="check" ;;
esac

ALL_FEATURES=(
    "v01_original"
    "v02_stable"
    "v03_mana"
    "v04_qclassic"
    "v5_live"
    "exact_trans"
)

echo ""
echo "========================================"
echo "  Armada Harness — Cross-Build $CARGO_CMD"
echo "========================================"
echo ""

PASS=0
FAIL=0

for feature in "${ALL_FEATURES[@]}"; do
    echo -n "  [$feature] cargo $CARGO_CMD ... "
    if cargo $CARGO_CMD -p armada-shim --no-default-features --features "$feature" 2>/dev/null; then
        echo "OK"
        PASS=$((PASS + 1))
    else
        echo "FAIL"
        FAIL=$((FAIL + 1))
    fi
done

echo ""
echo "========================================"
echo "  Results: $PASS passed, $FAIL failed"
echo "========================================"

exit $FAIL

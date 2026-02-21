#!/bin/bash
# bench_all.sh — Run cross-build benchmarks against all compatible FHE builds
#
# Usage:
#   ./scripts/bench_all.sh              # Full benchmarks (slow)
#   ./scripts/bench_all.sh --test       # Quick smoke test
#   ./scripts/bench_all.sh --tracked    # Tracked audit mode
#
# Results go to target/criterion/ with per-build HTML reports.

set -euo pipefail

HARNESS_DIR="$(cd "$(dirname "$0")/.." && pwd)"
cd "$HARNESS_DIR"

MODE="${1:---full}"
EXTRA_ARGS=""

case "$MODE" in
    --test)
        EXTRA_ARGS="-- --test"
        echo "=== SMOKE TEST MODE ==="
        ;;
    --tracked)
        BENCH_NAME="tracked_audit"
        echo "=== TRACKED AUDIT MODE ==="
        ;;
    *)
        echo "=== FULL BENCHMARK MODE ==="
        ;;
esac

BENCH_NAME="${BENCH_NAME:-cross_build}"

# BFV-compatible builds (v01, v02, v04, v5)
BFV_FEATURES=(
    "v01_original"
    "v02_stable"
    "v04_qclassic"
    "v5_live"
)

# MANA parallel CRT (v03) — separate benchmark group
MANA_FEATURES=(
    "v03_mana"
)

# Exact transcendentals — separate benchmark group
TRANS_FEATURES=(
    "exact_trans"
)

echo ""
echo "=========================================="
echo "  Homomorphic Armada — Cross-Build Bench"
echo "=========================================="
echo ""

PASS=0
FAIL=0
SKIP=0

run_bench() {
    local feature="$1"
    local bench="$2"
    echo "--- [$feature] cargo bench -p armada-bench --bench $bench ---"

    if cargo bench -p armada-bench --bench "$bench" \
        --no-default-features --features "$feature" \
        $EXTRA_ARGS 2>&1; then
        echo "  => PASS"
        PASS=$((PASS + 1))
    else
        echo "  => FAIL (exit $?)"
        FAIL=$((FAIL + 1))
    fi
    echo ""
}

# Run BFV benchmarks
echo "=== BFV FHE Benchmarks ==="
for feature in "${BFV_FEATURES[@]}"; do
    run_bench "$feature" "$BENCH_NAME"
done

# Run MANA benchmarks
if [ "$BENCH_NAME" = "cross_build" ]; then
    echo "=== MANA Parallel CRT Benchmarks ==="
    for feature in "${MANA_FEATURES[@]}"; do
        run_bench "$feature" "cross_build"
    done

    echo "=== Exact Transcendentals Benchmarks ==="
    for feature in "${TRANS_FEATURES[@]}"; do
        run_bench "$feature" "cross_build"
    done
fi

echo "=========================================="
echo "  Results: $PASS passed, $FAIL failed, $SKIP skipped"
echo "  HTML reports: target/criterion/"
echo "=========================================="

exit $FAIL

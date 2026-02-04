#!/usr/bin/env python3
"""
Rust vs Python Adaptive CRT Performance Benchmark
==================================================

Validates the 50-100× speedup claim by comparing:
1. Prime generation (Python vs Rust)
2. CRT arithmetic operations (Python vs Rust)
3. Tier transitions (Python vs Rust)
4. Memory usage

Expected Results:
- Prime generation: 200-500× faster in Rust
- Basic operations: 50-100× faster in Rust
- Tier transitions: 50-80× faster in Rust
"""

from __future__ import annotations

import sys
import time
import tracemalloc
from dataclasses import dataclass
from fractions import Fraction
from pathlib import Path
from typing import Callable, List, Optional, Sequence, Tuple

# Resolve repository paths dynamically so the benchmark is portable
REPO_PATH = Path(__file__).resolve().parents[1]
RUST_TARGET_PATH = REPO_PATH / "hcvlang" / "target" / "release"

# Add QMNF Python package and the Rust extension module to the import path
sys.path.insert(0, str(REPO_PATH))
sys.path.insert(0, str(RUST_TARGET_PATH))

# Import Python implementation
from qmnf.arithmetic.optimization.dynamic_crt_stacking import (  # type: ignore
    AdaptiveThresholds,
    DynamicCRTBigInt as PyDynamicCRT,
    PrecisionTier as PyTier,
)

# Import Rust implementation (handled gracefully if unavailable)
try:
    from hcvlang_pyo3 import AdaptiveCRTBigInt as RustDynamicCRT  # type: ignore

    RUST_AVAILABLE = True
except ImportError as exc:  # pragma: no cover - informational path
    print(f"⚠️  Rust bindings not available: {exc}")
    print("   Build with: cd", RUST_TARGET_PATH.parent)
    print("   cargo build --release --features=python")
    RUST_AVAILABLE = False
    RustDynamicCRT = None  # type: ignore


# ============================================================================
# Formatting helpers (integer-only)
# ============================================================================


def _format_microseconds(value: int) -> str:
    """Format integer microseconds with thousands separators."""

    return f"{value:,}"


def _format_fraction(value: Fraction, *, decimals: int = 1) -> str:
    """Format a fraction using integer arithmetic with fixed decimals."""

    denominator = value.denominator
    if denominator == 0:
        return "inf"

    numerator = value.numerator
    negative = numerator < 0
    numerator = abs(numerator)

    scale = 10 ** decimals
    scaled = (numerator * scale * 2 + denominator) // (denominator * 2)
    integer_part = scaled // scale
    fractional_part = scaled % scale

    sign = "-" if negative else ""
    if decimals == 0:
        return f"{sign}{integer_part}"
    return f"{sign}{integer_part}.{fractional_part:0{decimals}d}"


def _mean_fraction(values: Sequence[Fraction]) -> Fraction:
    """Compute the arithmetic mean of fractional values."""

    if not values:
        return Fraction(0, 1)
    total = sum(values, start=Fraction(0, 1))
    return total / len(values)


def _median(values: Sequence[int]) -> int:
    """Return the integer median of a sequence."""

    ordered = sorted(values)
    length = len(ordered)
    if length == 0:
        return 0
    midpoint = length // 2
    if length % 2 == 1:
        return ordered[midpoint]
    return (ordered[midpoint - 1] + ordered[midpoint]) // 2


def _median_fraction(values: Sequence[Fraction]) -> Fraction:
    """Return the median of fractional values."""

    ordered = sorted(values)
    length = len(ordered)
    if length == 0:
        return Fraction(0, 1)
    midpoint = length // 2
    if length % 2 == 1:
        return ordered[midpoint]
    return (ordered[midpoint - 1] + ordered[midpoint]) / 2


def _speedup_fraction(python_us: int, rust_us: int) -> Fraction:
    """Compute the fractional speedup, guarding against division by zero."""

    if rust_us == 0:
        return Fraction(0, 1)
    return Fraction(python_us, rust_us)


# ============================================================================
# Benchmark infrastructure
# ============================================================================


@dataclass
class BenchmarkResult:
    """Single benchmark measurement."""

    name: str
    python_time_us: int
    rust_time_us: int
    speedup: Fraction
    python_memory_kb: int
    rust_memory_kb: int
    iterations: int

    def __str__(self) -> str:  # pragma: no cover - presentation helper
        python_time = _format_microseconds(self.python_time_us)
        rust_time = _format_microseconds(self.rust_time_us)
        speedup_text = _format_fraction(self.speedup, decimals=1)
        return (
            f"{self.name:40s} | "
            f"Python: {python_time:>10s} µs | "
            f"Rust: {rust_time:>10s} µs | "
            f"Speedup: {speedup_text:>6s}×"
        )


def benchmark_function(
    func: Callable[[], None],
    *,
    iterations: int = 100,
    warmup: int = 10,
) -> Tuple[int, int]:
    """Benchmark a callable using integer timing and memory tracking."""

    for _ in range(warmup):
        func()

    samples: List[int] = []
    tracemalloc.start()

    for _ in range(iterations):
        start_ns = time.perf_counter_ns()
        func()
        end_ns = time.perf_counter_ns()
        samples.append(end_ns - start_ns)

    _, peak_bytes = tracemalloc.get_traced_memory()
    tracemalloc.stop()

    median_us = _median(samples) // 1_000
    memory_kb = peak_bytes // 1024

    return median_us, memory_kb


# ============================================================================
# Benchmark suites
# ============================================================================


def benchmark_prime_generation() -> Optional[List[BenchmarkResult]]:
    """Benchmark prime generation for tier initialization."""

    print("\n" + "=" * 80)
    print("BENCHMARK 1: Prime Generation")
    print("=" * 80)

    if not RUST_AVAILABLE:
        print("⏭️  Skipped (Rust not available)")
        return None

    results: List[BenchmarkResult] = []

    print("\n🔹 Tier 0 (1 prime):")

    def python_tier0() -> None:
        _ = PyDynamicCRT.create_default_config(PyTier.TIER_0_30BIT)

    def rust_tier0() -> None:
        _ = RustDynamicCRT(42)  # type: ignore[call-arg]

    py_time, py_mem = benchmark_function(python_tier0, iterations=50)
    rust_time, rust_mem = benchmark_function(rust_tier0, iterations=50)

    result = BenchmarkResult(
        name="Prime Generation (Tier 0: 1 prime)",
        python_time_us=py_time,
        rust_time_us=rust_time,
        speedup=_speedup_fraction(py_time, rust_time),
        python_memory_kb=py_mem,
        rust_memory_kb=rust_mem,
        iterations=50,
    )
    print(result)
    results.append(result)

    print("\n🔹 Tier 3 (8 primes):")

    def python_tier3() -> None:
        _ = PyDynamicCRT.create_default_config(PyTier.TIER_3_240BIT)

    def rust_tier3() -> None:
        _ = RustDynamicCRT((1 << 62) - 1)  # type: ignore[call-arg]

    py_time, py_mem = benchmark_function(python_tier3, iterations=20)
    rust_time, rust_mem = benchmark_function(rust_tier3, iterations=20)

    result = BenchmarkResult(
        name="Prime Generation (Tier 3: 8 primes)",
        python_time_us=py_time,
        rust_time_us=rust_time,
        speedup=_speedup_fraction(py_time, rust_time),
        python_memory_kb=py_mem,
        rust_memory_kb=rust_mem,
        iterations=20,
    )
    print(result)
    results.append(result)

    return results


def benchmark_arithmetic_operations() -> Optional[List[BenchmarkResult]]:
    """Benchmark basic CRT arithmetic operations."""

    print("\n" + "=" * 80)
    print("BENCHMARK 2: Arithmetic Operations")
    print("=" * 80)

    if not RUST_AVAILABLE:
        print("⏭️  Skipped (Rust not available)")
        return None

    results: List[BenchmarkResult] = []

    config_py = PyDynamicCRT.create_default_config()
    thresholds = AdaptiveThresholds.default()

    test_cases = [
        (123456, 789012, "Small values (Tier 0)"),
        (1 << 50, 1 << 51, "Medium values (Tier 1)"),
        ((1 << 61) - 1000, (1 << 61) - 2000, "Large values (Tier 2)"),
    ]

    for val_a, val_b, description in test_cases:
        print(f"\n🔹 {description}:")

        py_a = PyDynamicCRT.from_int(val_a, config_py, thresholds)
        py_b = PyDynamicCRT.from_int(val_b, config_py, thresholds)

        rust_a = RustDynamicCRT(val_a)  # type: ignore[call-arg]
        rust_b = RustDynamicCRT(val_b)  # type: ignore[call-arg]

        def python_add() -> None:
            _ = py_a.add(py_b)

        def rust_add() -> None:
            _ = rust_a + rust_b

        py_time, py_mem = benchmark_function(python_add, iterations=1_000)
        rust_time, rust_mem = benchmark_function(rust_add, iterations=1_000)

        add_result = BenchmarkResult(
            name=f"Addition - {description}",
            python_time_us=py_time,
            rust_time_us=rust_time,
            speedup=_speedup_fraction(py_time, rust_time),
            python_memory_kb=py_mem,
            rust_memory_kb=rust_mem,
            iterations=1_000,
        )
        print(f"  Addition: {add_result}")
        results.append(add_result)

        def python_mul() -> None:
            _ = py_a.mul(py_b)

        def rust_mul() -> None:
            _ = rust_a * rust_b

        py_time, py_mem = benchmark_function(python_mul, iterations=1_000)
        rust_time, rust_mem = benchmark_function(rust_mul, iterations=1_000)

        mul_result = BenchmarkResult(
            name=f"Multiplication - {description}",
            python_time_us=py_time,
            rust_time_us=rust_time,
            speedup=_speedup_fraction(py_time, rust_time),
            python_memory_kb=py_mem,
            rust_memory_kb=rust_mem,
            iterations=1_000,
        )
        print(f"  Multiplication: {mul_result}")
        results.append(mul_result)

    return results


def benchmark_tier_transitions() -> Optional[List[BenchmarkResult]]:
    """Benchmark automatic tier promotion/demotion."""

    print("\n" + "=" * 80)
    print("BENCHMARK 3: Tier Transitions")
    print("=" * 80)

    if not RUST_AVAILABLE:
        print("⏭️  Skipped (Rust not available)")
        return None

    results: List[BenchmarkResult] = []

    print("\n🔹 Promotion (repeated squaring):")

    config_py = PyDynamicCRT.create_default_config()
    thresholds = AdaptiveThresholds.default()

    def python_promotion() -> None:
        value = PyDynamicCRT.from_int(1 << 20, config_py, thresholds)
        for _ in range(10):
            value = value.mul(value)

    def rust_promotion() -> None:
        value = RustDynamicCRT(1 << 20)  # type: ignore[call-arg]
        for _ in range(10):
            value = value * value

    py_time, py_mem = benchmark_function(python_promotion, iterations=10)
    rust_time, rust_mem = benchmark_function(rust_promotion, iterations=10)

    result = BenchmarkResult(
        name="Tier Promotion (10× squaring)",
        python_time_us=py_time,
        rust_time_us=rust_time,
        speedup=_speedup_fraction(py_time, rust_time),
        python_memory_kb=py_mem,
        rust_memory_kb=rust_mem,
        iterations=10,
    )
    print(result)
    results.append(result)

    return results


def benchmark_reconstruction() -> Optional[List[BenchmarkResult]]:
    """Benchmark CRT → integer reconstruction."""

    print("\n" + "=" * 80)
    print("BENCHMARK 4: Value Reconstruction")
    print("=" * 80)

    if not RUST_AVAILABLE:
        print("⏭️  Skipped (Rust not available)")
        return None

    results: List[BenchmarkResult] = []

    test_cases = [
        (123_456_789, "Small (30-bit)"),
        ((1 << 50) + 123, "Medium (50-bit)"),
        ((1 << 61) - 1, "Large (61-bit)"),
    ]

    for value, description in test_cases:
        print(f"\n🔹 {description}:")

        config_py = PyDynamicCRT.create_default_config()
        thresholds = AdaptiveThresholds.default()
        py_val = PyDynamicCRT.from_int(value, config_py, thresholds)

        rust_val = RustDynamicCRT(value)  # type: ignore[call-arg]

        def python_reconstruct() -> None:
            _ = py_val.to_int()

        def rust_reconstruct() -> None:
            _ = int(rust_val)

        py_time, py_mem = benchmark_function(python_reconstruct, iterations=1_000)
        rust_time, rust_mem = benchmark_function(rust_reconstruct, iterations=1_000)

        result = BenchmarkResult(
            name=f"Reconstruction - {description}",
            python_time_us=py_time,
            rust_time_us=rust_time,
            speedup=_speedup_fraction(py_time, rust_time),
            python_memory_kb=py_mem,
            rust_memory_kb=rust_mem,
            iterations=1_000,
        )
        print(result)
        results.append(result)

    return results


# ============================================================================
# Reporting
# ============================================================================


def generate_summary_report(all_results: List[Optional[List[BenchmarkResult]]]) -> None:
    """Generate comprehensive summary report."""

    print("\n" + "=" * 80)
    print("PERFORMANCE SUMMARY")
    print("=" * 80)

    results = [result for group in all_results if group for result in group]

    if not results:
        print("⚠️  No benchmark results available (Rust not built)")
        return

    speedups = [result.speedup for result in results]
    memory_ratios = [
        Fraction(result.python_memory_kb, result.rust_memory_kb)
        for result in results
        if result.rust_memory_kb > 0
    ]

    print("\n📊 Overall Statistics:")
    print(f"   Benchmarks run: {len(results)}")
    print(
        f"   Average speedup: {_format_fraction(_mean_fraction(speedups), decimals=1)}×"
    )
    print(
        f"   Median speedup: {_format_fraction(_median_fraction(speedups), decimals=1)}×"
    )
    print(f"   Min speedup: {_format_fraction(min(speedups), decimals=1)}×")
    print(f"   Max speedup: {_format_fraction(max(speedups), decimals=1)}×")

    if memory_ratios:
        print("\n💾 Memory Efficiency:")
        print(
            "   Average memory reduction: "
            f"{_format_fraction(_mean_fraction(memory_ratios), decimals=1)}×"
        )

    categories = {
        "Prime Generation": [],
        "Arithmetic": [],
        "Transitions": [],
        "Reconstruction": [],
    }

    for result in results:
        if "Prime Generation" in result.name:
            categories["Prime Generation"].append(result.speedup)
        elif "Addition" in result.name or "Multiplication" in result.name:
            categories["Arithmetic"].append(result.speedup)
        elif "Tier" in result.name:
            categories["Transitions"].append(result.speedup)
        elif "Reconstruction" in result.name:
            categories["Reconstruction"].append(result.speedup)

    print("\n📈 Performance by Category:")
    for category, values in categories.items():
        if values:
            avg_speedup = _mean_fraction(values)
            print(
                f"   {category:20s}: "
                f"{_format_fraction(avg_speedup, decimals=1):>6s}× average speedup"
            )

    overall_avg = _mean_fraction(speedups)

    print("\n🎯 Target Validation:")
    if overall_avg >= 100:
        status = "✅ EXCEEDED"
    elif overall_avg >= 50:
        status = "✅ MET"
    elif overall_avg >= 25:
        status = "⚠️  PARTIAL"
    else:
        status = "❌ BELOW"

    print("   Target: 50-100× speedup")
    print(f"   Actual: {_format_fraction(overall_avg, decimals=1)}× average")
    print(f"   Status: {status}")

    print("\n📋 Detailed Results:")
    print(f"{'Benchmark':<50s} {'Python':<12s} {'Rust':<12s} {'Speedup'}")
    print("-" * 90)
    for result in results:
        python_time = _format_microseconds(result.python_time_us)
        rust_time = _format_microseconds(result.rust_time_us)
        speedup_text = _format_fraction(result.speedup, decimals=1)
        print(
            f"{result.name:<50s} {python_time:>10s} µs "
            f"{rust_time:>10s} µs {speedup_text:>6s}×"
        )


# ============================================================================
# Entry point
# ============================================================================


def main() -> None:
    """Run complete benchmark suite."""

    print("=" * 80)
    print("RUST vs PYTHON ADAPTIVE CRT BENCHMARK")
    print("=" * 80)
    print(f"\nRust bindings available: {'✅ YES' if RUST_AVAILABLE else '❌ NO'}")

    if not RUST_AVAILABLE:
        print("\n⚠️  To run benchmarks, build Rust bindings:")
        print("   cd", RUST_TARGET_PATH.parent)
        print("   cargo build --release --features=python")
        return

    results = [
        benchmark_prime_generation(),
        benchmark_arithmetic_operations(),
        benchmark_tier_transitions(),
        benchmark_reconstruction(),
    ]

    generate_summary_report(results)

    print("\n" + "=" * 80)
    print("BENCHMARK COMPLETE")
    print("=" * 80)


if __name__ == "__main__":
    main()

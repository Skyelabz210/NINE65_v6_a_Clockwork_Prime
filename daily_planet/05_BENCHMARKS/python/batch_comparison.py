"""
Batch Operation Comparison Benchmarks

Compares individual FFI calls in loops vs batch operations.
Target: 4-8× speedup for batch operations.

Tests various batch sizes: 10, 100, 1000, 10000
"""

import pytest


# Batch size 10
@pytest.mark.benchmark(group="batch_add_10")
def test_individual_add_10(benchmark):
    """Individual additions in loop (n=10)."""
    from hcvlang_pyo3 import CRTBigInt

    n = 10
    values_a = [CRTBigInt(i) for i in range(n)]
    values_b = [CRTBigInt(i * 2) for i in range(n)]

    def individual_loop():
        return [a + b for a, b in zip(values_a, values_b)]

    result = benchmark(individual_loop)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_add_10")
def test_batch_add_10(benchmark):
    """Batch addition (n=10)."""
    from hcvlang_pyo3 import CRTBigInt, batch_add_crtbigint

    n = 10
    values_a = [CRTBigInt(i) for i in range(n)]
    values_b = [CRTBigInt(i * 2) for i in range(n)]

    result = benchmark(batch_add_crtbigint, values_a, values_b)
    assert len(result) == n


# Batch size 100
@pytest.mark.benchmark(group="batch_add_100")
def test_individual_add_100(benchmark):
    """Individual additions in loop (n=100)."""
    from hcvlang_pyo3 import CRTBigInt

    n = 100
    values_a = [CRTBigInt(i) for i in range(n)]
    values_b = [CRTBigInt(i * 2) for i in range(n)]

    def individual_loop():
        return [a + b for a, b in zip(values_a, values_b)]

    result = benchmark(individual_loop)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_add_100")
def test_batch_add_100(benchmark):
    """Batch addition (n=100)."""
    from hcvlang_pyo3 import CRTBigInt, batch_add_crtbigint

    n = 100
    values_a = [CRTBigInt(i) for i in range(n)]
    values_b = [CRTBigInt(i * 2) for i in range(n)]

    result = benchmark(batch_add_crtbigint, values_a, values_b)
    assert len(result) == n


# Batch size 1000
@pytest.mark.benchmark(group="batch_add_1000")
def test_individual_add_1000(benchmark):
    """Individual additions in loop (n=1000)."""
    from hcvlang_pyo3 import CRTBigInt

    n = 1000
    values_a = [CRTBigInt(i) for i in range(n)]
    values_b = [CRTBigInt(i * 2) for i in range(n)]

    def individual_loop():
        return [a + b for a, b in zip(values_a, values_b)]

    result = benchmark(individual_loop)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_add_1000")
def test_batch_add_1000(benchmark):
    """Batch addition (n=1000)."""
    from hcvlang_pyo3 import CRTBigInt, batch_add_crtbigint

    n = 1000
    values_a = [CRTBigInt(i) for i in range(n)]
    values_b = [CRTBigInt(i * 2) for i in range(n)]

    result = benchmark(batch_add_crtbigint, values_a, values_b)
    assert len(result) == n


# Multiplication comparisons
@pytest.mark.benchmark(group="batch_mul_100")
def test_individual_mul_100(benchmark):
    """Individual multiplications in loop (n=100)."""
    from hcvlang_pyo3 import CRTBigInt

    n = 100
    values_a = [CRTBigInt(i + 1) for i in range(n)]
    values_b = [CRTBigInt(i * 2 + 1) for i in range(n)]

    def individual_loop():
        return [a * b for a, b in zip(values_a, values_b)]

    result = benchmark(individual_loop)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_mul_100")
def test_batch_mul_100(benchmark):
    """Batch multiplication (n=100)."""
    from hcvlang_pyo3 import CRTBigInt, batch_mul_crtbigint

    n = 100
    values_a = [CRTBigInt(i + 1) for i in range(n)]
    values_b = [CRTBigInt(i * 2 + 1) for i in range(n)]

    result = benchmark(batch_mul_crtbigint, values_a, values_b)
    assert len(result) == n


# ModInt batch operations
@pytest.mark.benchmark(group="batch_modint_100")
def test_individual_modint_add_100(benchmark):
    """Individual ModInt additions in loop (n=100)."""
    from hcvlang_pyo3 import ModInt

    n = 100
    modulus = 2147483647
    values_a = [ModInt(i, modulus) for i in range(n)]
    values_b = [ModInt(i * 2, modulus) for i in range(n)]

    def individual_loop():
        return [a + b for a, b in zip(values_a, values_b)]

    result = benchmark(individual_loop)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_modint_100")
def test_batch_modint_add_100(benchmark):
    """Batch ModInt addition (n=100)."""
    from hcvlang_pyo3 import ModInt, batch_add_modint

    n = 100
    modulus = 2147483647
    values_a = [ModInt(i, modulus) for i in range(n)]
    values_b = [ModInt(i * 2, modulus) for i in range(n)]

    result = benchmark(batch_add_modint, values_a, values_b)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_modint_mul_100")
def test_individual_modint_mul_100(benchmark):
    """Individual ModInt multiplications in loop (n=100)."""
    from hcvlang_pyo3 import ModInt

    n = 100
    modulus = 2147483647
    values_a = [ModInt(i + 1, modulus) for i in range(n)]
    values_b = [ModInt(i * 2 + 1, modulus) for i in range(n)]

    def individual_loop():
        return [a * b for a, b in zip(values_a, values_b)]

    result = benchmark(individual_loop)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_modint_mul_100")
def test_batch_modint_mul_100(benchmark):
    """Batch ModInt multiplication (n=100)."""
    from hcvlang_pyo3 import ModInt, batch_mul_modint

    n = 100
    modulus = 2147483647
    values_a = [ModInt(i + 1, modulus) for i in range(n)]
    values_b = [ModInt(i * 2 + 1, modulus) for i in range(n)]

    result = benchmark(batch_mul_modint, values_a, values_b)
    assert len(result) == n


# Rational operations
@pytest.mark.benchmark(group="batch_rational_100")
def test_individual_rational_add_100(benchmark):
    """Individual Rational additions in loop (n=100)."""
    from hcvlang_pyo3 import Rational, CRTBigInt

    n = 100
    values_a = [Rational(CRTBigInt(i + 1), CRTBigInt(7)) for i in range(n)]
    values_b = [Rational(CRTBigInt(i + 2), CRTBigInt(11)) for i in range(n)]

    def individual_loop():
        return [a + b for a, b in zip(values_a, values_b)]

    result = benchmark(individual_loop)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_rational_100")
def test_batch_rational_add_100(benchmark):
    """Batch Rational addition (n=100)."""
    from hcvlang_pyo3 import Rational, CRTBigInt, batch_add_rational

    n = 100
    values_a = [Rational(CRTBigInt(i + 1), CRTBigInt(7)) for i in range(n)]
    values_b = [Rational(CRTBigInt(i + 2), CRTBigInt(11)) for i in range(n)]

    result = benchmark(batch_add_rational, values_a, values_b)
    assert len(result) == n


# Transcendental functions
@pytest.mark.benchmark(group="batch_sin_50")
def test_individual_sin_50(benchmark):
    """Individual sin computations in loop (n=50)."""
    from hcvlang_pyo3 import Rational, CRTBigInt, sin

    n = 50
    angles = [Rational(CRTBigInt(i), CRTBigInt(100)) for i in range(n)]
    precision = 20

    def individual_loop():
        return [sin(angle, precision) for angle in angles]

    result = benchmark(individual_loop)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_sin_50")
def test_batch_sin_50(benchmark):
    """Batch sin computation (n=50)."""
    from hcvlang_pyo3 import Rational, CRTBigInt, batch_sin

    n = 50
    angles = [Rational(CRTBigInt(i), CRTBigInt(100)) for i in range(n)]
    precision = 20

    result = benchmark(batch_sin, angles, precision)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_sqrt_50")
def test_individual_sqrt_50(benchmark):
    """Individual sqrt computations in loop (n=50)."""
    from hcvlang_pyo3 import Rational, CRTBigInt, sqrt

    n = 50
    values = [Rational(CRTBigInt(i + 1), CRTBigInt(1)) for i in range(n)]
    precision = 20

    def individual_loop():
        return [sqrt(val, precision) for val in values]

    result = benchmark(individual_loop)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_sqrt_50")
def test_batch_sqrt_50(benchmark):
    """Batch sqrt computation (n=50)."""
    from hcvlang_pyo3 import Rational, CRTBigInt, batch_sqrt

    n = 50
    values = [Rational(CRTBigInt(i + 1), CRTBigInt(1)) for i in range(n)]
    precision = 20

    result = benchmark(batch_sqrt, values, precision)
    assert len(result) == n


# FastModInt performance
@pytest.mark.benchmark(group="batch_fastmodint_100")
def test_individual_fastmodint_mul_100(benchmark):
    """Individual FastModInt multiplications in loop (n=100)."""
    from hcvlang_pyo3 import FastModInt

    n = 100
    modulus = 2147483647
    values_a = [FastModInt(i + 1, modulus) for i in range(n)]
    values_b = [FastModInt(i * 2 + 1, modulus) for i in range(n)]

    def individual_loop():
        return [a * b for a, b in zip(values_a, values_b)]

    result = benchmark(individual_loop)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_fastmodint_100")
def test_batch_fastmodint_mul_100(benchmark):
    """Batch FastModInt multiplication (n=100)."""
    from hcvlang_pyo3 import FastModInt, batch_mul_fastmodint

    n = 100
    modulus = 2147483647
    values_a = [FastModInt(i + 1, modulus) for i in range(n)]
    values_b = [FastModInt(i * 2 + 1, modulus) for i in range(n)]

    result = benchmark(batch_mul_fastmodint, values_a, values_b)
    assert len(result) == n

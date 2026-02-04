"""
FFI Boundary Overhead Benchmarks

Measures Python-Rust boundary crossing costs for QMNF System.
Tests construction, arithmetic operations, type conversion, and data sizes.

Target: FFI overhead <1µs for simple operations
"""

import pytest


@pytest.mark.benchmark(group="ffi_construction")
def test_crtbigint_construction_small(benchmark):
    """Benchmark CRTBigInt construction from small Python int."""
    from hcvlang_pyo3 import CRTBigInt

    result = benchmark(CRTBigInt, 42)
    assert result is not None


@pytest.mark.benchmark(group="ffi_construction")
def test_crtbigint_construction_large(benchmark):
    """Benchmark CRTBigInt construction from large Python int."""
    from hcvlang_pyo3 import CRTBigInt

    large_val = 1267650600228229401496703205376
    result = benchmark(CRTBigInt, large_val)
    assert result is not None


@pytest.mark.benchmark(group="ffi_construction")
def test_modint_construction(benchmark):
    """Benchmark ModInt construction via FFI."""
    from hcvlang_pyo3 import ModInt

    result = benchmark(ModInt, 123456789, 2147483647)
    assert result is not None


@pytest.mark.benchmark(group="ffi_construction")
def test_rational_construction(benchmark):
    """Benchmark Rational construction via FFI."""
    from hcvlang_pyo3 import Rational, CRTBigInt

    num = CRTBigInt(22)
    den = CRTBigInt(7)

    result = benchmark(Rational, num, den)
    assert result is not None


@pytest.mark.benchmark(group="ffi_arithmetic")
def test_crtbigint_addition(benchmark):
    """Benchmark CRTBigInt addition via FFI."""
    from hcvlang_pyo3 import CRTBigInt

    a = CRTBigInt(123456789)
    b = CRTBigInt(987654321)

    result = benchmark(lambda: a + b)
    assert result is not None


@pytest.mark.benchmark(group="ffi_arithmetic")
def test_crtbigint_multiplication(benchmark):
    """Benchmark CRTBigInt multiplication via FFI."""
    from hcvlang_pyo3 import CRTBigInt

    a = CRTBigInt(123456789)
    b = CRTBigInt(987654321)

    result = benchmark(lambda: a * b)
    assert result is not None


@pytest.mark.benchmark(group="ffi_arithmetic")
def test_modint_addition(benchmark):
    """Benchmark ModInt addition via FFI."""
    from hcvlang_pyo3 import ModInt

    a = ModInt(123456789, 2147483647)
    b = ModInt(987654321, 2147483647)

    result = benchmark(lambda: a + b)
    assert result is not None


@pytest.mark.benchmark(group="ffi_arithmetic")
def test_modint_multiplication(benchmark):
    """Benchmark ModInt multiplication via FFI."""
    from hcvlang_pyo3 import ModInt

    a = ModInt(123456789, 2147483647)
    b = ModInt(987654321, 2147483647)

    result = benchmark(lambda: a * b)
    assert result is not None


@pytest.mark.benchmark(group="ffi_arithmetic")
def test_rational_addition(benchmark):
    """Benchmark Rational addition via FFI."""
    from hcvlang_pyo3 import Rational, CRTBigInt

    a = Rational(CRTBigInt(22), CRTBigInt(7))
    b = Rational(CRTBigInt(355), CRTBigInt(113))

    result = benchmark(lambda: a + b)
    assert result is not None


@pytest.mark.benchmark(group="ffi_arithmetic")
def test_rational_multiplication(benchmark):
    """Benchmark Rational multiplication via FFI."""
    from hcvlang_pyo3 import Rational, CRTBigInt

    a = Rational(CRTBigInt(22), CRTBigInt(7))
    b = Rational(CRTBigInt(355), CRTBigInt(113))

    result = benchmark(lambda: a * b)
    assert result is not None


@pytest.mark.benchmark(group="ffi_conversion")
def test_crtbigint_to_int(benchmark):
    """Benchmark CRTBigInt to Python int conversion."""
    from hcvlang_pyo3 import CRTBigInt

    a = CRTBigInt(123456789)

    result = benchmark(int, a)
    assert result == 123456789


@pytest.mark.benchmark(group="ffi_conversion")
def test_crtbigint_to_string(benchmark):
    """Benchmark CRTBigInt to string conversion."""
    from hcvlang_pyo3 import CRTBigInt

    a = CRTBigInt(123456789)

    result = benchmark(str, a)
    assert "123456789" in result


@pytest.mark.benchmark(group="ffi_data_structures")
def test_integer_matrix_construction(benchmark):
    """Benchmark IntegerMatrix construction."""
    from hcvlang_pyo3 import IntegerMatrix

    result = benchmark(IntegerMatrix, 32, 32)
    assert result is not None


@pytest.mark.benchmark(group="ffi_data_structures")
def test_hypervector_construction(benchmark):
    """Benchmark HyperVector construction."""
    from hcvlang_pyo3 import HyperVector

    result = benchmark(HyperVector, 1024)
    assert result is not None


@pytest.mark.benchmark(group="ffi_batch")
def test_batch_add_crtbigint(benchmark):
    """Benchmark batch addition vs individual FFI calls."""
    from hcvlang_pyo3 import CRTBigInt, batch_add_crtbigint

    n = 100
    values_a = [CRTBigInt(i) for i in range(n)]
    values_b = [CRTBigInt(i * 2) for i in range(n)]

    result = benchmark(batch_add_crtbigint, values_a, values_b)
    assert len(result) == n


@pytest.mark.benchmark(group="ffi_batch")
def test_batch_mul_crtbigint(benchmark):
    """Benchmark batch multiplication."""
    from hcvlang_pyo3 import CRTBigInt, batch_mul_crtbigint

    n = 100
    values_a = [CRTBigInt(i + 1) for i in range(n)]
    values_b = [CRTBigInt(i * 2 + 1) for i in range(n)]

    result = benchmark(batch_mul_crtbigint, values_a, values_b)
    assert len(result) == n


@pytest.mark.benchmark(group="ffi_batch")
def test_individual_vs_batch_comparison(benchmark):
    """Compare individual loop vs batch operation performance."""
    from hcvlang_pyo3 import CRTBigInt

    n = 100
    values_a = [CRTBigInt(i) for i in range(n)]
    values_b = [CRTBigInt(i * 2) for i in range(n)]

    def individual_loop():
        return [a + b for a, b in zip(values_a, values_b)]

    result = benchmark(individual_loop)
    assert len(result) == n


@pytest.mark.benchmark(group="ffi_neural")
def test_dense_layer_construction(benchmark):
    """Benchmark DenseLayer construction."""
    from hcvlang_pyo3 import DenseLayer

    result = benchmark(DenseLayer, 784, 128)
    assert result is not None


@pytest.mark.benchmark(group="ffi_neural")
def test_integer_mlp_construction(benchmark):
    """Benchmark IntegerMLP construction."""
    from hcvlang_pyo3 import IntegerMLP

    layer_sizes = [784, 512, 256, 128, 10]
    result = benchmark(IntegerMLP, layer_sizes)
    assert result is not None


@pytest.mark.benchmark(group="ffi_crypto")
def test_fhe_context_construction(benchmark):
    """Benchmark FHEContext construction."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    result = benchmark(FHEContext, SecurityLevel.Toy)
    assert result is not None


@pytest.mark.benchmark(group="ffi_crypto")
def test_fhe_keypair_generation(benchmark):
    """Benchmark FHE keypair generation."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)

    result = benchmark(ctx.generate_keypair)
    assert result is not None


@pytest.mark.benchmark(group="ffi_crypto")
def test_fhe_encryption(benchmark):
    """Benchmark FHE encryption via FFI."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()
    plaintext = ctx.encode(42)

    result = benchmark(ctx.encrypt, plaintext, pk)
    assert result is not None


@pytest.mark.benchmark(group="ffi_crypto")
def test_fhe_decryption(benchmark):
    """Benchmark FHE decryption via FFI."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()
    plaintext = ctx.encode(42)
    ciphertext = ctx.encrypt(plaintext, pk)

    result = benchmark(ctx.decrypt, ciphertext, sk)
    assert result is not None


@pytest.mark.benchmark(group="ffi_storage")
def test_holographic_encoder_construction(benchmark):
    """Benchmark HolographicEncoder construction."""
    from hcvlang_pyo3 import HolographicEncoder

    result = benchmark(HolographicEncoder, 1024)
    assert result is not None


@pytest.mark.benchmark(group="ffi_math")
def test_math_constants_pi(benchmark):
    """Benchmark MathConstants pi access."""
    from hcvlang_pyo3 import MathConstants

    result = benchmark(MathConstants.pi, 100)
    assert result is not None


@pytest.mark.benchmark(group="ffi_math")
def test_batch_sin(benchmark):
    """Benchmark batch sine computation."""
    from hcvlang_pyo3 import Rational, CRTBigInt, batch_sin

    n = 50
    angles = [Rational(CRTBigInt(i), CRTBigInt(100)) for i in range(n)]

    result = benchmark(batch_sin, angles, 20)
    assert len(result) == n


@pytest.mark.benchmark(group="ffi_math")
def test_batch_sqrt(benchmark):
    """Benchmark batch square root computation."""
    from hcvlang_pyo3 import Rational, CRTBigInt, batch_sqrt

    n = 50
    values = [Rational(CRTBigInt(i + 1), CRTBigInt(1)) for i in range(n)]

    result = benchmark(batch_sqrt, values, 20)
    assert len(result) == n

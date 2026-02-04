"""
Core QMNF Benchmarks - Working API Tests Only

Focused benchmarks testing verified working FFI APIs.
Tests core arithmetic, batch operations, and basic FHE.
"""

import pytest


# =============================================================================
# CRTBigInt Benchmarks
# =============================================================================

@pytest.mark.benchmark(group="crtbigint")
def test_crtbigint_construction_small(benchmark):
    """Benchmark CRTBigInt construction from small int."""
    from hcvlang_pyo3 import CRTBigInt
    result = benchmark(CRTBigInt, 42)
    assert result is not None


@pytest.mark.benchmark(group="crtbigint")
def test_crtbigint_construction_large(benchmark):
    """Benchmark CRTBigInt construction from large int."""
    from hcvlang_pyo3 import CRTBigInt
    large_val = 1267650600228229401496703205376
    result = benchmark(CRTBigInt, large_val)
    assert result is not None


@pytest.mark.benchmark(group="crtbigint")
def test_crtbigint_addition(benchmark):
    """Benchmark CRTBigInt addition."""
    from hcvlang_pyo3 import CRTBigInt
    a = CRTBigInt(123456789)
    b = CRTBigInt(987654321)
    result = benchmark(lambda: a + b)
    assert result is not None


@pytest.mark.benchmark(group="crtbigint")
def test_crtbigint_multiplication(benchmark):
    """Benchmark CRTBigInt multiplication."""
    from hcvlang_pyo3 import CRTBigInt
    a = CRTBigInt(123456789)
    b = CRTBigInt(987654321)
    result = benchmark(lambda: a * b)
    assert result is not None


@pytest.mark.benchmark(group="crtbigint")
def test_crtbigint_subtraction(benchmark):
    """Benchmark CRTBigInt subtraction."""
    from hcvlang_pyo3 import CRTBigInt
    a = CRTBigInt(987654321)
    b = CRTBigInt(123456789)
    result = benchmark(lambda: a - b)
    assert result is not None


# =============================================================================
# ModInt Benchmarks
# =============================================================================

@pytest.mark.benchmark(group="modint")
def test_modint_construction(benchmark):
    """Benchmark ModInt construction."""
    from hcvlang_pyo3 import ModInt
    result = benchmark(ModInt.from_u64, 123456789, 2147483647)
    assert result is not None


@pytest.mark.benchmark(group="modint")
def test_modint_addition(benchmark):
    """Benchmark ModInt addition."""
    from hcvlang_pyo3 import ModInt
    a = ModInt.from_u64(123456789, 2147483647)
    b = ModInt.from_u64(987654321, 2147483647)
    result = benchmark(lambda: a + b)
    assert result is not None


@pytest.mark.benchmark(group="modint")
def test_modint_multiplication(benchmark):
    """Benchmark ModInt multiplication."""
    from hcvlang_pyo3 import ModInt
    a = ModInt.from_u64(123456789, 2147483647)
    b = ModInt.from_u64(987654321, 2147483647)
    result = benchmark(lambda: a * b)
    assert result is not None


@pytest.mark.benchmark(group="modint")
def test_modint_montgomery_mul(benchmark):
    """Benchmark ModInt Montgomery multiplication."""
    from hcvlang_pyo3 import ModInt
    a = ModInt.from_u64(123456789, 2147483647)
    b = ModInt.from_u64(987654321, 2147483647)
    result = benchmark(a.montgomery_mul, b)
    assert result is not None


@pytest.mark.benchmark(group="modint")
def test_modint_modular_inverse(benchmark):
    """Benchmark ModInt modular inverse."""
    from hcvlang_pyo3 import ModInt
    a = ModInt.from_u64(123456789, 2147483647)
    result = benchmark(a.modular_inverse)
    assert result is not None


# =============================================================================
# Batch Operation Benchmarks
# =============================================================================

@pytest.mark.benchmark(group="batch_crtbigint")
def test_batch_add_crtbigint_10(benchmark):
    """Benchmark batch CRTBigInt addition (n=10)."""
    from hcvlang_pyo3 import CRTBigInt, batch_add_crtbigint
    n = 10
    vals_a = [CRTBigInt(i) for i in range(n)]
    vals_b = [CRTBigInt(i * 2) for i in range(n)]
    result = benchmark(batch_add_crtbigint, vals_a, vals_b)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_crtbigint")
def test_batch_add_crtbigint_100(benchmark):
    """Benchmark batch CRTBigInt addition (n=100)."""
    from hcvlang_pyo3 import CRTBigInt, batch_add_crtbigint
    n = 100
    vals_a = [CRTBigInt(i) for i in range(n)]
    vals_b = [CRTBigInt(i * 2) for i in range(n)]
    result = benchmark(batch_add_crtbigint, vals_a, vals_b)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_crtbigint")
def test_batch_add_crtbigint_1000(benchmark):
    """Benchmark batch CRTBigInt addition (n=1000)."""
    from hcvlang_pyo3 import CRTBigInt, batch_add_crtbigint
    n = 1000
    vals_a = [CRTBigInt(i) for i in range(n)]
    vals_b = [CRTBigInt(i * 2) for i in range(n)]
    result = benchmark(batch_add_crtbigint, vals_a, vals_b)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_crtbigint")
def test_batch_mul_crtbigint_100(benchmark):
    """Benchmark batch CRTBigInt multiplication (n=100)."""
    from hcvlang_pyo3 import CRTBigInt, batch_mul_crtbigint
    n = 100
    vals_a = [CRTBigInt(i + 1) for i in range(n)]
    vals_b = [CRTBigInt(i * 2 + 1) for i in range(n)]
    result = benchmark(batch_mul_crtbigint, vals_a, vals_b)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_modint")
def test_batch_add_modint_100(benchmark):
    """Benchmark batch ModInt addition (n=100)."""
    from hcvlang_pyo3 import ModInt, batch_add_modint
    n = 100
    modulus = 2147483647
    vals_a = [ModInt.from_u64(i, modulus) for i in range(n)]
    vals_b = [ModInt.from_u64(i * 2, modulus) for i in range(n)]
    result = benchmark(batch_add_modint, vals_a, vals_b)
    assert len(result) == n


@pytest.mark.benchmark(group="batch_modint")
def test_batch_mul_modint_100(benchmark):
    """Benchmark batch ModInt multiplication (n=100)."""
    from hcvlang_pyo3 import ModInt, batch_mul_modint
    n = 100
    modulus = 2147483647
    vals_a = [ModInt.from_u64(i + 1, modulus) for i in range(n)]
    vals_b = [ModInt.from_u64(i * 2 + 1, modulus) for i in range(n)]
    result = benchmark(batch_mul_modint, vals_a, vals_b)
    assert len(result) == n


# =============================================================================
# Individual vs Batch Comparison
# =============================================================================

@pytest.mark.benchmark(group="individual_vs_batch")
def test_individual_add_loop_100(benchmark):
    """Individual CRTBigInt additions in loop (n=100) - baseline."""
    from hcvlang_pyo3 import CRTBigInt
    n = 100
    vals_a = [CRTBigInt(i) for i in range(n)]
    vals_b = [CRTBigInt(i * 2) for i in range(n)]

    def individual_loop():
        return [a + b for a, b in zip(vals_a, vals_b)]

    result = benchmark(individual_loop)
    assert len(result) == n


@pytest.mark.benchmark(group="individual_vs_batch")
def test_batch_add_optimized_100(benchmark):
    """Batch CRTBigInt addition (n=100) - optimized."""
    from hcvlang_pyo3 import CRTBigInt, batch_add_crtbigint
    n = 100
    vals_a = [CRTBigInt(i) for i in range(n)]
    vals_b = [CRTBigInt(i * 2) for i in range(n)]

    result = benchmark(batch_add_crtbigint, vals_a, vals_b)
    assert len(result) == n


# =============================================================================
# FHE Benchmarks
# =============================================================================

@pytest.mark.benchmark(group="fhe_setup")
def test_fhe_context_creation(benchmark):
    """Benchmark FHEContext creation."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel
    result = benchmark(FHEContext, SecurityLevel.TOY)
    assert result is not None


@pytest.mark.benchmark(group="fhe_keygen")
def test_fhe_keypair_generation(benchmark):
    """Benchmark FHE keypair generation."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel
    ctx = FHEContext(SecurityLevel.TOY)
    result = benchmark(ctx.generate_keypair)
    assert result is not None


@pytest.mark.benchmark(group="fhe_keygen")
def test_fhe_evaluation_key_generation(benchmark):
    """Benchmark FHE evaluation key generation."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel
    ctx = FHEContext(SecurityLevel.TOY)
    sk, pk = ctx.generate_keypair()
    result = benchmark(ctx.generate_evaluation_key, sk)
    assert result is not None


@pytest.mark.benchmark(group="fhe_encode")
def test_fhe_encode_small(benchmark):
    """Benchmark FHE encoding (small value)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel
    ctx = FHEContext(SecurityLevel.TOY)
    result = benchmark(ctx.encode, 42)
    assert result is not None


@pytest.mark.benchmark(group="fhe_encrypt")
def test_fhe_encryption(benchmark):
    """Benchmark FHE encryption."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel
    ctx = FHEContext(SecurityLevel.TOY)
    sk, pk = ctx.generate_keypair()
    pt = ctx.encode(42)
    result = benchmark(ctx.encrypt, pt, pk)
    assert result is not None


@pytest.mark.benchmark(group="fhe_decrypt")
def test_fhe_decryption(benchmark):
    """Benchmark FHE decryption."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel
    ctx = FHEContext(SecurityLevel.TOY)
    sk, pk = ctx.generate_keypair()
    pt = ctx.encode(42)
    ct = ctx.encrypt(pt, pk)
    result = benchmark(ctx.decrypt, ct, sk)
    assert result is not None


@pytest.mark.benchmark(group="fhe_homomorphic")
def test_fhe_homomorphic_addition(benchmark):
    """Benchmark FHE homomorphic addition."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel
    ctx = FHEContext(SecurityLevel.TOY)
    sk, pk = ctx.generate_keypair()
    ct1 = ctx.encrypt(ctx.encode(10), pk)
    ct2 = ctx.encrypt(ctx.encode(32), pk)
    result = benchmark(ctx.add, ct1, ct2)
    assert result is not None


@pytest.mark.benchmark(group="fhe_homomorphic")
def test_fhe_homomorphic_multiplication(benchmark):
    """Benchmark FHE homomorphic multiplication."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel
    ctx = FHEContext(SecurityLevel.TOY)
    sk, pk = ctx.generate_keypair()
    ek = ctx.generate_evaluation_key(sk)
    ct1 = ctx.encrypt(ctx.encode(6), pk)
    ct2 = ctx.encrypt(ctx.encode(7), pk)
    result = benchmark(ctx.mul, ct1, ct2, ek)
    assert result is not None


# =============================================================================
# End-to-End Workflows
# =============================================================================

@pytest.mark.benchmark(group="e2e")
def test_encrypted_computation_pipeline(benchmark):
    """Benchmark end-to-end encrypted computation: (a + b) * c."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    def pipeline():
        ctx = FHEContext(SecurityLevel.TOY)
        sk, pk = ctx.generate_keypair()
        ek = ctx.generate_evaluation_key(sk)

        # Encrypt
        ct_a = ctx.encrypt(ctx.encode(10), pk)
        ct_b = ctx.encrypt(ctx.encode(20), pk)
        ct_c = ctx.encrypt(ctx.encode(3), pk)

        # Compute (10 + 20) * 3 = 90
        ct_sum = ctx.add(ct_a, ct_b)
        ct_result = ctx.mul(ct_sum, ct_c, ek)

        # Decrypt
        pt_result = ctx.decrypt(ct_result, sk)
        value = ctx.decode(pt_result)

        return value

    result = benchmark(pipeline)
    assert result == 90


@pytest.mark.benchmark(group="e2e")
def test_batch_encryption_workflow_50(benchmark):
    """Benchmark batch encryption workflow (n=50)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    def batch_encrypt():
        ctx = FHEContext(SecurityLevel.TOY)
        sk, pk = ctx.generate_keypair()

        plaintexts = [ctx.encode(i) for i in range(50)]
        ciphertexts = [ctx.encrypt(pt, pk) for pt in plaintexts]

        return len(ciphertexts)

    result = benchmark(batch_encrypt)
    assert result == 50


# =============================================================================
# Performance Summary Test
# =============================================================================

def test_performance_summary(benchmark):
    """Quick performance summary across multiple operations."""
    from hcvlang_pyo3 import CRTBigInt, ModInt, batch_add_crtbigint

    def mixed_operations():
        # CRT operations
        crt_vals = [CRTBigInt(i) for i in range(100)]
        crt_result = batch_add_crtbigint(crt_vals, crt_vals)

        # ModInt operations
        mod_vals = [ModInt.from_u64(i, 2147483647) for i in range(50)]

        # Mixed arithmetic
        a = CRTBigInt(12345)
        b = CRTBigInt(67890)
        c = a + b
        d = a * b

        return len(crt_result) + len(mod_vals)

    result = benchmark(mixed_operations)
    assert result == 150

"""
Cryptography Workflow Benchmarks

FHE encryption pipelines, batch encryption, homomorphic operations.
Tests key generation, encryption, decryption, and homomorphic arithmetic.

Target: FHE encrypt <5ms, batch encryption 8× speedup on 8 cores
"""

import pytest


@pytest.mark.benchmark(group="crypto_setup")
def test_fhe_context_creation_toy(benchmark):
    """Benchmark FHEContext creation (Toy security)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    result = benchmark(FHEContext, SecurityLevel.Toy)
    assert result is not None


@pytest.mark.benchmark(group="crypto_setup")
def test_fhe_context_creation_128bit(benchmark):
    """Benchmark FHEContext creation (128-bit security)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    result = benchmark(FHEContext, SecurityLevel.Bits128)
    assert result is not None


@pytest.mark.benchmark(group="crypto_keygen")
def test_fhe_keypair_generation_toy(benchmark):
    """Benchmark FHE keypair generation (Toy)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)

    result = benchmark(ctx.generate_keypair)
    assert result is not None


@pytest.mark.benchmark(group="crypto_keygen")
def test_fhe_keypair_generation_128bit(benchmark):
    """Benchmark FHE keypair generation (128-bit)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Bits128)

    result = benchmark(ctx.generate_keypair)
    assert result is not None


@pytest.mark.benchmark(group="crypto_keygen")
def test_fhe_evaluation_key_generation(benchmark):
    """Benchmark FHE evaluation key generation."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()

    result = benchmark(ctx.generate_evaluation_key, sk)
    assert result is not None


@pytest.mark.benchmark(group="crypto_encoding")
def test_integer_encoding_small(benchmark):
    """Benchmark encoding small integer."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)

    result = benchmark(ctx.encode, 42)
    assert result is not None


@pytest.mark.benchmark(group="crypto_encoding")
def test_integer_encoding_large(benchmark):
    """Benchmark encoding large integer."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)

    result = benchmark(ctx.encode, 1234567890)
    assert result is not None


@pytest.mark.benchmark(group="crypto_encrypt")
def test_fhe_encryption_toy(benchmark):
    """Benchmark FHE encryption (Toy security)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()
    plaintext = ctx.encode(42)

    result = benchmark(ctx.encrypt, plaintext, pk)
    assert result is not None


@pytest.mark.benchmark(group="crypto_encrypt")
def test_fhe_encryption_128bit(benchmark):
    """Benchmark FHE encryption (128-bit security)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Bits128)
    sk, pk = ctx.generate_keypair()
    plaintext = ctx.encode(42)

    result = benchmark(ctx.encrypt, plaintext, pk)
    assert result is not None


@pytest.mark.benchmark(group="crypto_decrypt")
def test_fhe_decryption_toy(benchmark):
    """Benchmark FHE decryption (Toy security)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()
    plaintext = ctx.encode(42)
    ciphertext = ctx.encrypt(plaintext, pk)

    result = benchmark(ctx.decrypt, ciphertext, sk)
    assert result is not None


@pytest.mark.benchmark(group="crypto_decrypt")
def test_fhe_decryption_128bit(benchmark):
    """Benchmark FHE decryption (128-bit security)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Bits128)
    sk, pk = ctx.generate_keypair()
    plaintext = ctx.encode(42)
    ciphertext = ctx.encrypt(plaintext, pk)

    result = benchmark(ctx.decrypt, ciphertext, sk)
    assert result is not None


@pytest.mark.benchmark(group="crypto_homomorphic")
def test_fhe_homomorphic_addition(benchmark):
    """Benchmark FHE homomorphic addition."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()

    ct1 = ctx.encrypt(ctx.encode(10), pk)
    ct2 = ctx.encrypt(ctx.encode(32), pk)

    result = benchmark(ctx.add, ct1, ct2)
    assert result is not None


@pytest.mark.benchmark(group="crypto_homomorphic")
def test_fhe_homomorphic_subtraction(benchmark):
    """Benchmark FHE homomorphic subtraction."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()

    ct1 = ctx.encrypt(ctx.encode(100), pk)
    ct2 = ctx.encrypt(ctx.encode(42), pk)

    result = benchmark(ctx.sub, ct1, ct2)
    assert result is not None


@pytest.mark.benchmark(group="crypto_homomorphic")
def test_fhe_homomorphic_multiplication(benchmark):
    """Benchmark FHE homomorphic multiplication."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()
    ek = ctx.generate_evaluation_key(sk)

    ct1 = ctx.encrypt(ctx.encode(6), pk)
    ct2 = ctx.encrypt(ctx.encode(7), pk)

    result = benchmark(ctx.mul, ct1, ct2, ek)
    assert result is not None


@pytest.mark.benchmark(group="crypto_batch_encrypt")
def test_batch_encryption_10(benchmark):
    """Benchmark batch encryption (n=10)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()

    plaintexts = [ctx.encode(i) for i in range(10)]

    def batch_encrypt():
        return [ctx.encrypt(pt, pk) for pt in plaintexts]

    result = benchmark(batch_encrypt)
    assert len(result) == 10


@pytest.mark.benchmark(group="crypto_batch_encrypt")
def test_batch_encryption_100(benchmark):
    """Benchmark batch encryption (n=100)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()

    plaintexts = [ctx.encode(i) for i in range(100)]

    def batch_encrypt():
        return [ctx.encrypt(pt, pk) for pt in plaintexts]

    result = benchmark(batch_encrypt)
    assert len(result) == 100


@pytest.mark.benchmark(group="crypto_batch_decrypt")
def test_batch_decryption_10(benchmark):
    """Benchmark batch decryption (n=10)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()

    ciphertexts = [ctx.encrypt(ctx.encode(i), pk) for i in range(10)]

    def batch_decrypt():
        return [ctx.decrypt(ct, sk) for ct in ciphertexts]

    result = benchmark(batch_decrypt)
    assert len(result) == 10


@pytest.mark.benchmark(group="crypto_batch_decrypt")
def test_batch_decryption_100(benchmark):
    """Benchmark batch decryption (n=100)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()

    ciphertexts = [ctx.encrypt(ctx.encode(i), pk) for i in range(100)]

    def batch_decrypt():
        return [ctx.decrypt(ct, sk) for ct in ciphertexts]

    result = benchmark(batch_decrypt)
    assert len(result) == 100


@pytest.mark.benchmark(group="crypto_batch_homomorphic")
def test_batch_homomorphic_addition_50(benchmark):
    """Benchmark batch homomorphic additions (n=50)."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()

    cts_a = [ctx.encrypt(ctx.encode(i), pk) for i in range(50)]
    cts_b = [ctx.encrypt(ctx.encode(i * 2), pk) for i in range(50)]

    def batch_add():
        return [ctx.add(a, b) for a, b in zip(cts_a, cts_b)]

    result = benchmark(batch_add)
    assert len(result) == 50


@pytest.mark.benchmark(group="crypto_batch_processor")
def test_batch_fhe_processor_construction(benchmark):
    """Benchmark BatchFHEProcessor construction."""
    from hcvlang_pyo3 import BatchFHEProcessor, BatchConfig

    config = BatchConfig(batch_size=256, parallel_enabled=True)

    result = benchmark(BatchFHEProcessor, config)
    assert result is not None


@pytest.mark.benchmark(group="crypto_batch_processor")
def test_batch_fhe_processor_encrypt_100(benchmark):
    """Benchmark BatchFHEProcessor encryption (n=100)."""
    from hcvlang_pyo3 import BatchFHEProcessor, BatchConfig, FHEContext, SecurityLevel

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()

    config = BatchConfig(batch_size=256, parallel_enabled=True)
    processor = BatchFHEProcessor(config)

    plaintexts = [ctx.encode(i) for i in range(100)]

    result = benchmark(processor.batch_encrypt, plaintexts, pk, ctx)
    assert len(result) == 100


@pytest.mark.benchmark(group="crypto_end_to_end")
def test_encrypted_computation_pipeline_simple(benchmark):
    """Benchmark end-to-end encrypted computation: (a + b) * c."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    def encrypted_pipeline():
        ctx = FHEContext(SecurityLevel.Toy)
        sk, pk = ctx.generate_keypair()
        ek = ctx.generate_evaluation_key(sk)

        # Encrypt inputs
        ct_a = ctx.encrypt(ctx.encode(10), pk)
        ct_b = ctx.encrypt(ctx.encode(20), pk)
        ct_c = ctx.encrypt(ctx.encode(3), pk)

        # Compute (a + b) * c = 90
        ct_sum = ctx.add(ct_a, ct_b)
        ct_result = ctx.mul(ct_sum, ct_c, ek)

        # Decrypt
        pt_result = ctx.decrypt(ct_result, sk)
        value = ctx.decode(pt_result)

        return value

    result = benchmark(encrypted_pipeline)
    assert result is not None


@pytest.mark.benchmark(group="crypto_end_to_end")
def test_encrypted_computation_pipeline_complex(benchmark):
    """Benchmark complex encrypted computation: ((a + b) * c) - d."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    def encrypted_pipeline():
        ctx = FHEContext(SecurityLevel.Toy)
        sk, pk = ctx.generate_keypair()
        ek = ctx.generate_evaluation_key(sk)

        # Encrypt inputs
        ct_a = ctx.encrypt(ctx.encode(15), pk)
        ct_b = ctx.encrypt(ctx.encode(25), pk)
        ct_c = ctx.encrypt(ctx.encode(2), pk)
        ct_d = ctx.encrypt(ctx.encode(10), pk)

        # Compute ((a + b) * c) - d = (40 * 2) - 10 = 70
        ct_sum = ctx.add(ct_a, ct_b)
        ct_mul = ctx.mul(ct_sum, ct_c, ek)
        ct_result = ctx.sub(ct_mul, ct_d)

        # Decrypt
        pt_result = ctx.decrypt(ct_result, sk)
        value = ctx.decode(pt_result)

        return value

    result = benchmark(encrypted_pipeline)
    assert result is not None


@pytest.mark.benchmark(group="crypto_end_to_end")
def test_encrypted_vector_dot_product(benchmark):
    """Benchmark encrypted vector dot product."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel

    def encrypted_dot_product():
        ctx = FHEContext(SecurityLevel.Toy)
        sk, pk = ctx.generate_keypair()
        ek = ctx.generate_evaluation_key(sk)

        # Vectors
        vec_a = [1, 2, 3, 4, 5]
        vec_b = [10, 20, 30, 40, 50]

        # Encrypt
        cts_a = [ctx.encrypt(ctx.encode(x), pk) for x in vec_a]
        cts_b = [ctx.encrypt(ctx.encode(x), pk) for x in vec_b]

        # Compute dot product: sum(a_i * b_i)
        products = [ctx.mul(a, b, ek) for a, b in zip(cts_a, cts_b)]

        # Sum products
        result = products[0]
        for prod in products[1:]:
            result = ctx.add(result, prod)

        # Decrypt
        pt_result = ctx.decrypt(result, sk)
        value = ctx.decode(pt_result)

        return value

    result = benchmark(encrypted_dot_product)
    assert result is not None


@pytest.mark.benchmark(group="crypto_noise_tracking")
def test_noise_tracker_construction(benchmark):
    """Benchmark NoiseTracker construction."""
    from hcvlang_pyo3 import NoiseTracker

    result = benchmark(NoiseTracker, 100)
    assert result is not None


@pytest.mark.benchmark(group="crypto_noise_tracking")
def test_noise_tracker_estimate(benchmark):
    """Benchmark NoiseTracker noise estimation."""
    from hcvlang_pyo3 import FHEContext, SecurityLevel, NoiseTracker

    ctx = FHEContext(SecurityLevel.Toy)
    sk, pk = ctx.generate_keypair()

    ct = ctx.encrypt(ctx.encode(42), pk)
    tracker = NoiseTracker(100)

    result = benchmark(tracker.estimate_noise, ct)
    assert result is not None


@pytest.mark.benchmark(group="crypto_security")
def test_encrypted_task_state_creation(benchmark):
    """Benchmark EncryptedTaskState creation."""
    from hcvlang_pyo3 import EncryptedTaskState

    result = benchmark(EncryptedTaskState, 1)
    assert result is not None


@pytest.mark.benchmark(group="crypto_security")
def test_encrypted_memory_region_creation(benchmark):
    """Benchmark EncryptedMemoryRegion creation."""
    from hcvlang_pyo3 import EncryptedMemoryRegion

    result = benchmark(EncryptedMemoryRegion, 0, 1024)
    assert result is not None

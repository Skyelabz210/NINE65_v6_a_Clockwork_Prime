"""
Integration Workflow Benchmarks

End-to-end workflows across multiple subsystems.
Tests neural + FHE, MANA + storage, and complete application scenarios.

Target: Integration workflows <500ms
"""

import pytest


@pytest.mark.benchmark(group="integration_neural_crypto")
def test_encrypted_neural_inference(benchmark):
    """Benchmark encrypted neural network inference."""
    from hcvlang_pyo3 import IntegerMLP, FHEContext, SecurityLevel

    def encrypted_inference():
        # Create neural network
        mlp = IntegerMLP([128, 64, 10])

        # Create FHE context
        ctx = FHEContext(SecurityLevel.Toy)
        sk, pk = ctx.generate_keypair()

        # Encrypt input
        input_vec = list(range(128))
        encrypted_input = [ctx.encrypt(ctx.encode(x), pk) for x in input_vec]

        # In a real system, forward pass would be on encrypted data
        # For now, decrypt, compute, re-encrypt (placeholder)
        decrypted = [ctx.decode(ctx.decrypt(ct, sk)) for ct in encrypted_input[:128]]
        output = mlp.forward(decrypted)

        # Encrypt output
        encrypted_output = [ctx.encrypt(ctx.encode(x), pk) for x in output]

        return len(encrypted_output)

    result = benchmark(encrypted_inference)
    assert result == 10


@pytest.mark.benchmark(group="integration_storage")
def test_holographic_storage_encode_decode(benchmark):
    """Benchmark holographic storage encode/decode pipeline."""
    from hcvlang_pyo3 import HolographicEncoder, IntegerMatrix

    encoder = HolographicEncoder(1024)

    # Create test data
    data = IntegerMatrix(32, 32)
    for i in range(32):
        for j in range(32):
            data.set(i, j, i * 32 + j)

    def encode_decode():
        encoded = encoder.encode(data)
        decoded = encoder.decode(encoded)
        return decoded

    result = benchmark(encode_decode)
    assert result is not None


@pytest.mark.benchmark(group="integration_storage")
def test_dual_stream_holographic_storage(benchmark):
    """Benchmark dual-stream holographic storage."""
    from hcvlang_pyo3 import DualStreamHolographicStorage

    def dual_stream_workflow():
        storage = DualStreamHolographicStorage(1024, 512)

        # Write data to both streams
        data = list(range(512))
        storage.write_primary(data)
        storage.write_shadow(data)

        # Read from both streams
        primary = storage.read_primary()
        shadow = storage.read_shadow()

        return len(primary) + len(shadow)

    result = benchmark(dual_stream_workflow)
    assert result > 0


@pytest.mark.benchmark(group="integration_mana")
def test_mana_kernel_task_scheduling(benchmark):
    """Benchmark MANA kernel task scheduling."""
    from hcvlang_pyo3 import MANAKernel, HelixTask, ExecutionDomain

    kernel = MANAKernel()

    # Create tasks
    tasks = [HelixTask(i, 100, ExecutionDomain.LinearCPU) for i in range(10)]

    def schedule_tasks():
        for task in tasks:
            kernel.submit_task(task)
        return kernel.get_queue_length()

    result = benchmark(schedule_tasks)
    assert result >= 0


@pytest.mark.benchmark(group="integration_mana")
def test_double_helix_execution(benchmark):
    """Benchmark dual-lane execution via DoubleHelixEngine."""
    from hcvlang_pyo3 import DoubleHelixEngine

    def dual_execution():
        engine = DoubleHelixEngine()

        # Execute on both lanes
        result_a = engine.execute_lane_a(42)
        result_b = engine.execute_lane_b(58)

        return result_a + result_b

    result = benchmark(dual_execution)
    assert result == 100


@pytest.mark.benchmark(group="integration_hyperdimensional")
def test_hyperdimensional_classification_pipeline(benchmark):
    """Benchmark complete hyperdimensional classification."""
    from hcvlang_pyo3 import HyperVector

    def hd_classification():
        # Create class prototypes
        prototypes = {i: HyperVector(1024) for i in range(10)}

        # Create test samples
        samples = [HyperVector(1024) for _ in range(20)]

        # Classify each sample
        predictions = []
        for sample in samples:
            best_class = 0
            best_similarity = -1

            for class_id, prototype in prototypes.items():
                similarity = sample.similarity(prototype)
                if similarity > best_similarity:
                    best_similarity = similarity
                    best_class = class_id

            predictions.append(best_class)

        return len(predictions)

    result = benchmark(hd_classification)
    assert result == 20


@pytest.mark.benchmark(group="integration_math")
def test_mathematical_constants_computation(benchmark):
    """Benchmark mathematical constants computation pipeline."""
    from hcvlang_pyo3 import MathConstants, Rational, CRTBigInt

    def compute_constants():
        # Compute various mathematical constants
        pi = MathConstants.pi(100)
        phi = MathConstants.phi(100)
        e = MathConstants.e(100)
        sqrt2 = MathConstants.sqrt2(100)

        # Perform computation: (pi + e) * phi / sqrt2
        pi_plus_e = pi + e
        result = pi_plus_e * phi
        # Division would be: result / sqrt2

        return result is not None

    result = benchmark(compute_constants)
    assert result is True


@pytest.mark.benchmark(group="integration_math")
def test_polynomial_ring_operations(benchmark):
    """Benchmark polynomial ring operations."""
    from hcvlang_pyo3 import PolynomialRing

    ring = PolynomialRing(1024, 65537)

    # Create polynomials
    coeffs_a = list(range(512))
    coeffs_b = list(range(512, 1024))

    poly_a = ring.from_coeffs(coeffs_a)
    poly_b = ring.from_coeffs(coeffs_b)

    def polynomial_ops():
        # Add, multiply, and reduce
        sum_poly = ring.add(poly_a, poly_b)
        prod_poly = ring.multiply(poly_a, poly_b)
        return ring.reduce(prod_poly)

    result = benchmark(polynomial_ops)
    assert result is not None


@pytest.mark.benchmark(group="integration_nnt")
def test_nnt_polynomial_multiplication(benchmark):
    """Benchmark NNT-based polynomial multiplication."""
    from hcvlang_pyo3 import NNTEngine

    engine = NNTEngine(1024, 65537)

    poly_a = list(range(1024))
    poly_b = list(range(1024))

    result = benchmark(engine.multiply, poly_a, poly_b)
    assert len(result) == 1024


@pytest.mark.benchmark(group="integration_geometric")
def test_geometric_operations_pipeline(benchmark):
    """Benchmark geometric operations pipeline."""
    from hcvlang_pyo3 import GeomPoint2D, Line2D

    def geometric_workflow():
        # Create points
        points = [GeomPoint2D(i * 100, i * 200) for i in range(20)]

        # Create lines between consecutive points
        lines = [Line2D(points[i], points[i + 1]) for i in range(19)]

        # Compute total length (using distance calculations)
        total_length = 0
        for line in lines:
            # In real implementation: line.length()
            # Placeholder: just count
            total_length += 1

        return total_length

    result = benchmark(geometric_workflow)
    assert result == 19


@pytest.mark.benchmark(group="integration_apollonian")
def test_apollonian_circle_generation(benchmark):
    """Benchmark Apollonian circle generation."""
    from hcvlang_pyo3 import ApollonianCircle

    def generate_circles():
        # Create initial circles
        c1 = ApollonianCircle(0, 0, 100)
        c2 = ApollonianCircle(200, 0, 100)
        c3 = ApollonianCircle(100, 173, 100)

        # In real implementation: generate Apollonian gasket
        # Placeholder: just create circles
        circles = [c1, c2, c3]

        return len(circles)

    result = benchmark(generate_circles)
    assert result == 3


@pytest.mark.benchmark(group="integration_swarm")
def test_swarm_optimization_workflow(benchmark):
    """Benchmark swarm optimization workflow."""
    from hcvlang_pyo3 import GravitationalSwarmOptimizer, GSOConfig

    config = GSOConfig(swarm_size=50, dimensions=10)
    optimizer = GravitationalSwarmOptimizer(config)

    def optimization_step():
        # Run one optimization iteration
        optimizer.step()
        best = optimizer.get_best_position()
        return len(best)

    result = benchmark(optimization_step)
    assert result == 10


@pytest.mark.benchmark(group="integration_attractor")
def test_attractor_memory_workflow(benchmark):
    """Benchmark attractor-based memory operations."""
    from hcvlang_pyo3 import AttractorMemoryCell

    def attractor_workflow():
        cells = [AttractorMemoryCell(i, 100) for i in range(20)]

        # Update all cells
        for cell in cells:
            cell.update_state(50)

        # Check convergence
        converged = sum(1 for cell in cells if cell.is_converged())

        return converged

    result = benchmark(attractor_workflow)
    assert result >= 0


@pytest.mark.benchmark(group="integration_fibonacci")
def test_fibonacci_scheduling(benchmark):
    """Benchmark Fibonacci-based scheduling."""
    from hcvlang_pyo3 import FibonacciScheduler

    scheduler = FibonacciScheduler(100)

    def schedule_workflow():
        # Get scheduling sequence
        sequence = []
        for _ in range(20):
            next_time = scheduler.next_interval()
            sequence.append(next_time)

        return len(sequence)

    result = benchmark(schedule_workflow)
    assert result == 20


@pytest.mark.benchmark(group="integration_codex")
def test_codex_gear_manifold_workflow(benchmark):
    """Benchmark Codex gear manifold operations."""
    from hcvlang_pyo3 import CodexManifold

    manifold = CodexManifold()

    def codex_workflow():
        # Add gears to manifold
        for i in range(10):
            manifold.add_gear(i)

        # Process manifold
        result = manifold.process()

        return result

    result = benchmark(codex_workflow)
    assert result is not None


@pytest.mark.benchmark(group="integration_end_to_end")
@pytest.mark.slow
def test_complete_application_workflow(benchmark):
    """Benchmark complete application workflow."""
    from hcvlang_pyo3 import (
        IntegerMLP,
        FHEContext,
        SecurityLevel,
        HolographicEncoder,
        IntegerMatrix,
    )

    def complete_workflow():
        # 1. Neural network inference
        mlp = IntegerMLP([128, 64, 10])
        input_vec = list(range(128))
        nn_output = mlp.forward(input_vec)

        # 2. Encrypt results
        ctx = FHEContext(SecurityLevel.Toy)
        sk, pk = ctx.generate_keypair()
        encrypted_output = [ctx.encrypt(ctx.encode(x), pk) for x in nn_output]

        # 3. Store encrypted results
        encoder = HolographicEncoder(512)
        storage_matrix = IntegerMatrix(4, 4)
        for i in range(min(10, 16)):
            storage_matrix.set(i // 4, i % 4, nn_output[i])

        encoded = encoder.encode(storage_matrix)

        # 4. Retrieve and decrypt
        decoded = encoder.decode(encoded)
        decrypted = [ctx.decode(ctx.decrypt(ct, sk)) for ct in encrypted_output]

        return len(decrypted)

    result = benchmark(complete_workflow)
    assert result == 10


@pytest.mark.benchmark(group="integration_end_to_end")
@pytest.mark.slow
def test_privacy_preserving_learning_pipeline(benchmark):
    """Benchmark privacy-preserving learning pipeline."""
    from hcvlang_pyo3 import OneShotLearner, FHEContext, SecurityLevel

    def privacy_learning():
        # 1. Create learner
        learner = OneShotLearner(128, 10)

        # 2. Create FHE context
        ctx = FHEContext(SecurityLevel.Toy)
        sk, pk = ctx.generate_keypair()

        # 3. Encrypted training data
        for class_id in range(5):
            sample = [(i + class_id * 20) % 256 for i in range(128)]
            # Encrypt sample (in real system)
            encrypted_sample = [ctx.encrypt(ctx.encode(x), pk) for x in sample]

            # Decrypt for training (in real FHE system, train on encrypted)
            decrypted_sample = [ctx.decode(ctx.decrypt(ct, sk)) for ct in encrypted_sample]
            learner.encode(decrypted_sample[:128], class_id)

        # 4. Encrypted inference
        query = list(range(128))
        encrypted_query = [ctx.encrypt(ctx.encode(x), pk) for x in query]

        # Decrypt for inference (placeholder)
        decrypted_query = [ctx.decode(ctx.decrypt(ct, sk)) for ct in encrypted_query]
        prediction = learner.classify(decrypted_query[:128])

        return prediction

    result = benchmark(privacy_learning)
    assert result is not None


@pytest.mark.benchmark(group="integration_end_to_end")
def test_multi_domain_computation(benchmark):
    """Benchmark multi-domain computation workflow."""
    from hcvlang_pyo3 import (
        CRTBigInt,
        ModInt,
        Rational,
        HyperVector,
        GeomPoint2D,
    )

    def multi_domain():
        # 1. CRT arithmetic
        crt_a = CRTBigInt(123456789)
        crt_b = CRTBigInt(987654321)
        crt_result = crt_a + crt_b

        # 2. Modular arithmetic
        mod_a = ModInt(int(crt_result), 2147483647)
        mod_b = ModInt(42, 2147483647)
        mod_result = mod_a * mod_b

        # 3. Rational arithmetic
        rat_a = Rational(CRTBigInt(22), CRTBigInt(7))
        rat_b = Rational(CRTBigInt(int(mod_result)), CRTBigInt(1))
        rat_result = rat_a + rat_b

        # 4. Hyperdimensional encoding
        hv = HyperVector(1024)

        # 5. Geometric computation
        point = GeomPoint2D(int(crt_result), int(mod_result))

        return point is not None

    result = benchmark(multi_domain)
    assert result is True

"""
Neural Network Workflow Benchmarks

End-to-end neural network training and inference benchmarks.
Tests forward/backward passes, batch training, and SIMD acceleration.

Target: Training epoch (32 samples) <100ms
"""

import pytest


@pytest.mark.benchmark(group="neural_construction")
def test_dense_layer_construction_small(benchmark):
    """Benchmark small DenseLayer construction (128→64)."""
    from hcvlang_pyo3 import DenseLayer

    result = benchmark(DenseLayer, 128, 64)
    assert result is not None


@pytest.mark.benchmark(group="neural_construction")
def test_dense_layer_construction_large(benchmark):
    """Benchmark large DenseLayer construction (784→512)."""
    from hcvlang_pyo3 import DenseLayer

    result = benchmark(DenseLayer, 784, 512)
    assert result is not None


@pytest.mark.benchmark(group="neural_construction")
def test_integer_mlp_construction_small(benchmark):
    """Benchmark small MLP construction (3 layers)."""
    from hcvlang_pyo3 import IntegerMLP

    layer_sizes = [128, 64, 32]
    result = benchmark(IntegerMLP, layer_sizes)
    assert result is not None


@pytest.mark.benchmark(group="neural_construction")
def test_integer_mlp_construction_mnist(benchmark):
    """Benchmark MNIST-sized MLP construction (784→128→10)."""
    from hcvlang_pyo3 import IntegerMLP

    layer_sizes = [784, 512, 256, 128, 10]
    result = benchmark(IntegerMLP, layer_sizes)
    assert result is not None


@pytest.mark.benchmark(group="neural_forward")
def test_dense_layer_forward_small(benchmark):
    """Benchmark forward pass through small layer (128→64)."""
    from hcvlang_pyo3 import DenseLayer

    layer = DenseLayer(128, 64)
    input_vec = list(range(128))

    result = benchmark(layer.forward, input_vec)
    assert len(result) == 64


@pytest.mark.benchmark(group="neural_forward")
def test_dense_layer_forward_large(benchmark):
    """Benchmark forward pass through large layer (784→512)."""
    from hcvlang_pyo3 import DenseLayer

    layer = DenseLayer(784, 512)
    input_vec = list(range(784))

    result = benchmark(layer.forward, input_vec)
    assert len(result) == 512


@pytest.mark.benchmark(group="neural_forward")
def test_integer_mlp_forward_mnist(benchmark):
    """Benchmark MLP forward pass (MNIST-sized)."""
    from hcvlang_pyo3 import IntegerMLP

    mlp = IntegerMLP([784, 512, 256, 128, 10])
    input_vec = list(range(784))

    result = benchmark(mlp.forward, input_vec)
    assert len(result) == 10


@pytest.mark.benchmark(group="neural_batch_forward")
def test_dense_layer_batch_forward_10(benchmark):
    """Benchmark batch forward pass (n=10)."""
    from hcvlang_pyo3 import DenseLayer

    layer = DenseLayer(128, 64)
    batch = [list(range(128)) for _ in range(10)]

    def batch_forward():
        return [layer.forward(input_vec) for input_vec in batch]

    result = benchmark(batch_forward)
    assert len(result) == 10


@pytest.mark.benchmark(group="neural_batch_forward")
def test_dense_layer_batch_forward_32(benchmark):
    """Benchmark batch forward pass (n=32)."""
    from hcvlang_pyo3 import DenseLayer

    layer = DenseLayer(784, 256)
    batch = [list(range(784)) for _ in range(32)]

    def batch_forward():
        return [layer.forward(input_vec) for input_vec in batch]

    result = benchmark(batch_forward)
    assert len(result) == 32


@pytest.mark.benchmark(group="neural_batch_forward")
def test_integer_mlp_batch_forward_32(benchmark):
    """Benchmark MLP batch forward pass (n=32)."""
    from hcvlang_pyo3 import IntegerMLP

    mlp = IntegerMLP([784, 512, 256, 10])
    batch = [list(range(784)) for _ in range(32)]

    def batch_forward():
        return [mlp.forward(input_vec) for input_vec in batch]

    result = benchmark(batch_forward)
    assert len(result) == 32


@pytest.mark.benchmark(group="neural_activation")
def test_activation_lut_construction(benchmark):
    """Benchmark ActivationLUT construction."""
    from hcvlang_pyo3 import ActivationLUT

    result = benchmark(ActivationLUT, 1000000)
    assert result is not None


@pytest.mark.benchmark(group="neural_activation")
def test_activation_lut_apply_batch(benchmark):
    """Benchmark ActivationLUT application to batch."""
    from hcvlang_pyo3 import ActivationLUT

    lut = ActivationLUT(1000000)
    values = list(range(-50, 51))

    result = benchmark(lut.apply_batch, values)
    assert len(result) == len(values)


@pytest.mark.benchmark(group="neural_oneshot")
def test_oneshot_learner_construction(benchmark):
    """Benchmark OneShotLearner construction."""
    from hcvlang_pyo3 import OneShotLearner

    result = benchmark(OneShotLearner, 128, 10)
    assert result is not None


@pytest.mark.benchmark(group="neural_oneshot")
def test_oneshot_learner_encode(benchmark):
    """Benchmark OneShotLearner encode operation."""
    from hcvlang_pyo3 import OneShotLearner

    learner = OneShotLearner(128, 10)
    sample = list(range(128))

    result = benchmark(learner.encode, sample, 0)
    assert len(result) == 128


@pytest.mark.benchmark(group="neural_oneshot")
def test_oneshot_learner_similarity(benchmark):
    """Benchmark OneShotLearner similarity computation."""
    from hcvlang_pyo3 import OneShotLearner

    learner = OneShotLearner(128, 10)

    # Add some prototypes
    for i in range(5):
        sample = [(j + i * 10) % 256 for j in range(128)]
        learner.encode(sample, i)

    query = list(range(128))

    result = benchmark(learner.classify, query)
    assert result is not None


@pytest.mark.benchmark(group="neural_hyperdimensional")
def test_hypervector_construction_1024(benchmark):
    """Benchmark HyperVector construction (1024 dims)."""
    from hcvlang_pyo3 import HyperVector

    result = benchmark(HyperVector, 1024)
    assert result is not None


@pytest.mark.benchmark(group="neural_hyperdimensional")
def test_hypervector_construction_10000(benchmark):
    """Benchmark HyperVector construction (10000 dims)."""
    from hcvlang_pyo3 import HyperVector

    result = benchmark(HyperVector, 10000)
    assert result is not None


@pytest.mark.benchmark(group="neural_hyperdimensional")
def test_hypervector_similarity(benchmark):
    """Benchmark HyperVector similarity computation."""
    from hcvlang_pyo3 import HyperVector

    hv1 = HyperVector(1024)
    hv2 = HyperVector(1024)

    result = benchmark(hv1.similarity, hv2)
    assert result is not None


@pytest.mark.benchmark(group="neural_hyperdimensional")
def test_hypervector_bind(benchmark):
    """Benchmark HyperVector binding operation."""
    from hcvlang_pyo3 import HyperVector

    hv1 = HyperVector(1024)
    hv2 = HyperVector(1024)

    result = benchmark(hv1.bind, hv2)
    assert result is not None


@pytest.mark.benchmark(group="neural_hyperdimensional")
def test_hypervector_bundle_many(benchmark):
    """Benchmark HyperVector bundling of many vectors."""
    from hcvlang_pyo3 import HyperVector

    vectors = [HyperVector(1024) for _ in range(20)]

    def bundle_all():
        result = vectors[0]
        for v in vectors[1:]:
            result = result.bundle(v)
        return result

    result = benchmark(bundle_all)
    assert result is not None


@pytest.mark.benchmark(group="neural_consensus")
def test_consensus_classifier_construction(benchmark):
    """Benchmark ConsensusClassifier construction."""
    from hcvlang_pyo3 import ConsensusClassifier

    result = benchmark(ConsensusClassifier, 128, 10)
    assert result is not None


@pytest.mark.benchmark(group="neural_consensus")
def test_consensus_classifier_add_example(benchmark):
    """Benchmark adding example to classifier."""
    from hcvlang_pyo3 import ConsensusClassifier

    classifier = ConsensusClassifier(128, 10)
    example = list(range(128))

    result = benchmark(classifier.add_example, example, 0)


@pytest.mark.benchmark(group="neural_consensus")
def test_consensus_classifier_predict(benchmark):
    """Benchmark consensus classification."""
    from hcvlang_pyo3 import ConsensusClassifier

    classifier = ConsensusClassifier(128, 10)

    # Add training examples
    for i in range(50):
        example = [(j + i * 5) % 256 for j in range(128)]
        classifier.add_example(example, i % 10)

    query = list(range(128))

    result = benchmark(classifier.predict, query)
    assert result is not None


@pytest.mark.benchmark(group="neural_end_to_end")
@pytest.mark.slow
def test_mlp_training_simulation_10_epochs(benchmark):
    """Benchmark MLP training simulation (10 epochs, 32 samples)."""
    from hcvlang_pyo3 import IntegerMLP

    mlp = IntegerMLP([128, 64, 32, 10])

    # Training data
    batch_size = 32
    training_data = [list(range(128)) for _ in range(batch_size)]

    def training_simulation():
        """Simulate 10 training epochs."""
        for epoch in range(10):
            for sample in training_data:
                _ = mlp.forward(sample)
                # In real training: compute loss, backprop, update weights
        return epoch

    result = benchmark(training_simulation)
    assert result == 9


@pytest.mark.benchmark(group="neural_end_to_end")
@pytest.mark.slow
def test_oneshot_learning_workflow(benchmark):
    """Benchmark complete one-shot learning workflow."""
    from hcvlang_pyo3 import OneShotLearner

    def oneshot_workflow():
        """Complete one-shot learning workflow."""
        learner = OneShotLearner(256, 10)

        # Few-shot learning: 1 example per class
        for class_id in range(10):
            example = [(i + class_id * 30) % 256 for i in range(256)]
            learner.encode(example, class_id)

        # Test classification on 20 queries
        correct = 0
        for i in range(20):
            query = [(j + (i % 10) * 30) % 256 for j in range(256)]
            pred = learner.classify(query)
            if pred == i % 10:
                correct += 1

        return correct

    result = benchmark(oneshot_workflow)
    assert result >= 0


@pytest.mark.benchmark(group="neural_end_to_end")
def test_hyperdimensional_encoding_pipeline(benchmark):
    """Benchmark complete hyperdimensional encoding pipeline."""
    from hcvlang_pyo3 import HyperVector

    def hd_pipeline():
        """Encode, bind, and bundle workflow."""
        # Create feature vectors
        color_red = HyperVector(1024)
        color_blue = HyperVector(1024)
        shape_circle = HyperVector(1024)
        shape_square = HyperVector(1024)

        # Bind features to create objects
        red_circle = color_red.bind(shape_circle)
        blue_square = color_blue.bind(shape_square)

        # Bundle objects into memory
        memory = red_circle.bundle(blue_square)

        # Test similarity
        query = color_red.bind(shape_circle)
        similarity = query.similarity(memory)

        return similarity

    result = benchmark(hd_pipeline)
    assert result is not None

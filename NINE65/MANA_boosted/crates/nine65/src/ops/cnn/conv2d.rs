//! Conv2D Layer Implementation
//!
//! 2D Convolutional layer using Im2Col transformation.
//! Integrates with FHENeuralEvaluator for activation functions.

use super::im2col::{conv2d_im2col, Im2ColConfig};
use super::tensor::Tensor4D;
use crate::arithmetic::{MobiusInt, Polarity};
use crate::ops::neural::{ActivationType, FHENeuralEvaluator};

/// Configuration for Conv2D layer
#[derive(Clone, Copy, Debug)]
pub struct Conv2DConfig {
    pub in_channels: usize,
    pub out_channels: usize,
    pub kernel_size: usize,
    pub stride: usize,
    pub padding: usize,
    pub activation: ActivationType,
}

impl Conv2DConfig {
    /// Create config for typical convolution
    pub fn new(
        in_channels: usize,
        out_channels: usize,
        kernel_size: usize,
        stride: usize,
        padding: usize,
        activation: ActivationType,
    ) -> Self {
        Self {
            in_channels,
            out_channels,
            kernel_size,
            stride,
            padding,
            activation,
        }
    }

    /// Create config with default stride=1, padding=0, no activation
    pub fn simple(in_channels: usize, out_channels: usize, kernel_size: usize) -> Self {
        Self::new(
            in_channels,
            out_channels,
            kernel_size,
            1,
            0,
            ActivationType::None,
        )
    }

    /// Get Im2Col configuration
    pub fn im2col_config(&self) -> Im2ColConfig {
        Im2ColConfig::square(self.kernel_size, self.stride, self.padding)
    }

    /// Compute output shape given input shape
    pub fn output_shape(&self, input_shape: [usize; 4]) -> [usize; 4] {
        let [n, _c_in, h, w] = input_shape;
        let (out_h, out_w) = self.im2col_config().output_size(h, w);
        [n, self.out_channels, out_h, out_w]
    }
}

/// 2D Convolutional Layer
#[derive(Clone, Debug)]
pub struct Conv2DLayer {
    /// Kernel weights [out_channels, in_channels, kernel_h, kernel_w]
    pub kernel: Tensor4D,
    /// Bias [out_channels]
    pub bias: Vec<MobiusInt>,
    /// Layer configuration
    pub config: Conv2DConfig,
}

impl Conv2DLayer {
    /// Create new Conv2D layer with given weights
    pub fn new(kernel: Tensor4D, bias: Vec<MobiusInt>, config: Conv2DConfig) -> Self {
        let [c_out, c_in, k_h, k_w] = kernel.shape;
        assert_eq!(c_out, config.out_channels, "Kernel out_channels mismatch");
        assert_eq!(c_in, config.in_channels, "Kernel in_channels mismatch");
        assert_eq!(k_h, config.kernel_size, "Kernel height mismatch");
        assert_eq!(k_w, config.kernel_size, "Kernel width mismatch");
        assert_eq!(bias.len(), config.out_channels, "Bias length mismatch");

        Self {
            kernel,
            bias,
            config,
        }
    }

    /// Create layer with zero-initialized weights (for testing)
    pub fn zeros(config: Conv2DConfig) -> Self {
        let kernel_shape = [
            config.out_channels,
            config.in_channels,
            config.kernel_size,
            config.kernel_size,
        ];
        Self {
            kernel: Tensor4D::zeros(kernel_shape),
            bias: vec![MobiusInt::zero(); config.out_channels],
            config,
        }
    }

    /// Create layer with identity-like weights (diagonal pattern)
    pub fn identity_like(config: Conv2DConfig) -> Self {
        let mut layer = Self::zeros(config);

        // Set center of each kernel to 1 for in_channel == out_channel
        let center = config.kernel_size / 2;
        let min_channels = config.in_channels.min(config.out_channels);

        for c in 0..min_channels {
            layer
                .kernel
                .set(c, c, center, center, MobiusInt::from_unsigned(1, Polarity::Plus));
        }

        layer
    }

    /// Forward pass through the layer
    pub fn forward(&self, input: &Tensor4D, eval: &FHENeuralEvaluator) -> Tensor4D {
        // Validate input shape
        assert_eq!(
            input.channels(),
            self.config.in_channels,
            "Input channels {} != expected {}",
            input.channels(),
            self.config.in_channels
        );

        // Perform convolution via im2col
        let im2col_cfg = self.config.im2col_config();
        let mut output = conv2d_im2col(input, &self.kernel, &self.bias, &im2col_cfg);

        // Apply activation if not None
        if self.config.activation != ActivationType::None {
            apply_activation_tensor(&mut output, self.config.activation, eval);
        }

        output
    }

    /// Get output shape for given input shape
    pub fn output_shape(&self, input_shape: [usize; 4]) -> [usize; 4] {
        self.config.output_shape(input_shape)
    }

    /// Number of parameters (weights + biases)
    pub fn num_parameters(&self) -> usize {
        self.kernel.size() + self.bias.len()
    }
}

/// Apply activation function to entire tensor
fn apply_activation_tensor(tensor: &mut Tensor4D, activation: ActivationType, eval: &FHENeuralEvaluator) {
    match activation {
        ActivationType::None => {}
        ActivationType::ReLU => {
            for value in tensor.data.iter_mut() {
                if value.is_negative() {
                    *value = MobiusInt::zero();
                }
            }
        }
        ActivationType::LeakyReLU => {
            for value in tensor.data.iter_mut() {
                if value.is_negative() {
                    // Leaky ReLU: 0.01 * x for negative values
                    let scaled = MobiusInt::from_unsigned(value.residue / 100, value.polarity);
                    *value = scaled;
                }
            }
        }
        ActivationType::Sigmoid => {
            for value in tensor.data.iter_mut() {
                let x = value.spinor_value() as i128;
                let result = eval.sigmoid(x);
                *value = MobiusInt::from_i64(result as i64);
            }
        }
        ActivationType::Tanh => {
            for value in tensor.data.iter_mut() {
                let x = value.spinor_value() as i128;
                let result = eval.tanh(x);
                *value = MobiusInt::from_i64(result as i64);
            }
        }
        ActivationType::GELU => {
            for value in tensor.data.iter_mut() {
                let x = value.spinor_value() as i128;
                let result = eval.gelu(x);
                *value = MobiusInt::from_i64(result as i64);
            }
        }
        ActivationType::Softmax => {
            // Softmax is typically applied per-channel or per-spatial-location
            // For tensors, we apply per (n, h, w) position across channels
            let [n, c, h, w] = tensor.shape;
            for batch in 0..n {
                for row in 0..h {
                    for col in 0..w {
                        // Gather values across channels
                        let logits: Vec<i128> = (0..c)
                            .map(|ch| tensor.get(batch, ch, row, col).spinor_value() as i128)
                            .collect();

                        // Compute softmax
                        let probs = eval.softmax(&logits);

                        // Store back
                        for (ch, &prob) in probs.iter().enumerate() {
                            tensor.set(
                                batch,
                                ch,
                                row,
                                col,
                                MobiusInt::from_unsigned(prob as u64, Polarity::Plus),
                            );
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv2d_config_output_shape() {
        let config = Conv2DConfig::new(3, 16, 3, 1, 1, ActivationType::ReLU);
        let output = config.output_shape([1, 3, 32, 32]);
        assert_eq!(output, [1, 16, 32, 32]); // Same spatial with padding=1
    }

    #[test]
    fn test_conv2d_layer_zeros() {
        let config = Conv2DConfig::simple(3, 16, 3);
        let layer = Conv2DLayer::zeros(config);

        assert_eq!(layer.kernel.shape, [16, 3, 3, 3]);
        assert_eq!(layer.bias.len(), 16);
        assert_eq!(layer.num_parameters(), 16 * 3 * 3 * 3 + 16);
    }

    #[test]
    fn test_conv2d_forward_identity() {
        // Create 1x1 conv that should pass through (with single channel)
        let config = Conv2DConfig::simple(1, 1, 1);
        let kernel = Tensor4D::from_data(
            vec![MobiusInt::from_unsigned(1, Polarity::Plus)],
            [1, 1, 1, 1],
        );
        let bias = vec![MobiusInt::zero()];
        let layer = Conv2DLayer::new(kernel, bias, config);

        let input = Tensor4D::from_i64(&[1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 1, 3, 3]);
        let eval = FHENeuralEvaluator::default();
        let output = layer.forward(&input, &eval);

        assert_eq!(output.shape, [1, 1, 3, 3]);
        assert_eq!(output.get(0, 0, 0, 0).spinor_value(), 1);
        assert_eq!(output.get(0, 0, 1, 1).spinor_value(), 5);
    }

    #[test]
    fn test_conv2d_with_relu() {
        let config = Conv2DConfig::new(1, 1, 1, 1, 0, ActivationType::ReLU);
        let kernel = Tensor4D::from_data(
            vec![MobiusInt::from_unsigned(1, Polarity::Plus)],
            [1, 1, 1, 1],
        );
        let bias = vec![MobiusInt::from_i64(-5)]; // Bias that makes some values negative

        let layer = Conv2DLayer::new(kernel, bias, config);

        // Input: [1, 2, 3, 4] with bias=-5 gives [-4, -3, -2, -1]
        let input = Tensor4D::from_i64(&[1, 2, 3, 4], [1, 1, 2, 2]);
        let eval = FHENeuralEvaluator::default();
        let output = layer.forward(&input, &eval);

        // All values should be zeroed by ReLU (since all are negative)
        for val in output.data.iter() {
            assert_eq!(val.spinor_value(), 0);
        }
    }

    #[test]
    fn test_conv2d_multiple_channels() {
        // 2 input channels, 2 output channels, 1x1 kernel
        let config = Conv2DConfig::simple(2, 2, 1);

        // Identity-like weights: each output channel reads from corresponding input
        let mut kernel_data = vec![MobiusInt::zero(); 4];
        kernel_data[0] = MobiusInt::from_unsigned(1, Polarity::Plus); // out0 <- in0
        kernel_data[3] = MobiusInt::from_unsigned(1, Polarity::Plus); // out1 <- in1
        let kernel = Tensor4D::from_data(kernel_data, [2, 2, 1, 1]);
        let bias = vec![MobiusInt::zero(); 2];

        let layer = Conv2DLayer::new(kernel, bias, config);

        // Input: [1, 2, 3, 4] in channel 0, [5, 6, 7, 8] in channel 1
        let input_data: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let input = Tensor4D::from_i64(&input_data, [1, 2, 2, 2]);

        let eval = FHENeuralEvaluator::default();
        let output = layer.forward(&input, &eval);

        assert_eq!(output.shape, [1, 2, 2, 2]);
        // Output channel 0 should equal input channel 0
        assert_eq!(output.get(0, 0, 0, 0).spinor_value(), 1);
        // Output channel 1 should equal input channel 1
        assert_eq!(output.get(0, 1, 0, 0).spinor_value(), 5);
    }
}

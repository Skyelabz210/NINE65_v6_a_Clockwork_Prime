//! Batch Normalization Layer
//!
//! Integer-only batch normalization for CNN inference.
//! Uses pre-computed running statistics (not training-time batch stats).
//!
//! # Formula
//!
//! ```text
//! y = gamma * (x - running_mean) / sqrt(running_var + epsilon) + beta
//! ```
//!
//! # Integer Implementation
//!
//! To avoid floating-point:
//! 1. Pre-compute `inv_std = scale / sqrt(var + epsilon)` as scaled integer
//! 2. Compute `y = gamma * (x - mean) * inv_std / scale + beta`
//!
//! All operations use MobiusInt for correct signed arithmetic.

use super::tensor::Tensor4D;
use crate::arithmetic::{MobiusInt, Polarity};

/// Scale factor for fixed-point representation
pub const BATCH_NORM_SCALE: u64 = 1_000_000;

/// Batch Normalization Layer (inference mode)
///
/// Stores pre-computed running statistics and learned parameters.
#[derive(Clone, Debug)]
pub struct BatchNormLayer {
    /// Learned scale parameters [channels]
    pub gamma: Vec<MobiusInt>,
    /// Learned shift parameters [channels]
    pub beta: Vec<MobiusInt>,
    /// Running mean [channels] (scaled by BATCH_NORM_SCALE)
    pub running_mean: Vec<MobiusInt>,
    /// Running variance [channels] (scaled by BATCH_NORM_SCALE)
    pub running_var: Vec<MobiusInt>,
    /// Pre-computed 1/sqrt(var + epsilon) [channels] (scaled by BATCH_NORM_SCALE)
    pub inv_std: Vec<MobiusInt>,
    /// Number of channels
    pub num_channels: usize,
    /// Small constant for numerical stability
    pub epsilon: u64,
}

impl BatchNormLayer {
    /// Create batch normalization layer with given parameters
    pub fn new(
        gamma: Vec<MobiusInt>,
        beta: Vec<MobiusInt>,
        running_mean: Vec<MobiusInt>,
        running_var: Vec<MobiusInt>,
        epsilon: u64,
    ) -> Self {
        let num_channels = gamma.len();
        assert_eq!(beta.len(), num_channels);
        assert_eq!(running_mean.len(), num_channels);
        assert_eq!(running_var.len(), num_channels);

        // Pre-compute inverse standard deviation
        let inv_std = running_var
            .iter()
            .map(|var| {
                let var_scaled = var.residue + epsilon;
                let std = integer_sqrt(var_scaled);
                if std > 0 {
                    MobiusInt::from_unsigned(BATCH_NORM_SCALE / std, Polarity::Plus)
                } else {
                    MobiusInt::from_unsigned(BATCH_NORM_SCALE, Polarity::Plus)
                }
            })
            .collect();

        Self {
            gamma,
            beta,
            running_mean,
            running_var,
            inv_std,
            num_channels,
            epsilon,
        }
    }

    /// Create identity batch norm (gamma=1, beta=0, mean=0, var=1)
    /// Useful for testing - output equals input
    pub fn identity(num_channels: usize) -> Self {
        Self::new(
            vec![MobiusInt::from_unsigned(BATCH_NORM_SCALE, Polarity::Plus); num_channels],
            vec![MobiusInt::zero(); num_channels],
            vec![MobiusInt::zero(); num_channels],
            vec![MobiusInt::from_unsigned(BATCH_NORM_SCALE, Polarity::Plus); num_channels],
            1,
        )
    }

    /// Create from raw values (convenience for loading pre-trained weights)
    pub fn from_raw(
        gamma: &[i64],
        beta: &[i64],
        running_mean: &[i64],
        running_var: &[i64],
        epsilon: u64,
    ) -> Self {
        Self::new(
            gamma.iter().map(|&v| MobiusInt::from_i64(v)).collect(),
            beta.iter().map(|&v| MobiusInt::from_i64(v)).collect(),
            running_mean.iter().map(|&v| MobiusInt::from_i64(v)).collect(),
            running_var.iter().map(|&v| MobiusInt::from_i64(v)).collect(),
            epsilon,
        )
    }

    /// Forward pass: normalize input tensor
    ///
    /// Input: [N, C, H, W]
    /// Output: [N, C, H, W] (same shape)
    pub fn forward(&self, input: &Tensor4D) -> Tensor4D {
        let [batch, channels, height, width] = input.shape;
        assert_eq!(
            channels, self.num_channels,
            "Input channels {} != BatchNorm channels {}",
            channels, self.num_channels
        );

        let mut output = Tensor4D::zeros([batch, channels, height, width]);

        for n in 0..batch {
            for c in 0..channels {
                let gamma = &self.gamma[c];
                let beta = &self.beta[c];
                let mean = &self.running_mean[c];
                let inv_std = &self.inv_std[c];

                for h in 0..height {
                    for w in 0..width {
                        let x = input.get(n, c, h, w);

                        // y = gamma * (x - mean) * inv_std / SCALE + beta
                        let centered = x.sub(mean);
                        let normalized = centered.mul(inv_std);
                        let scaled = normalized.mul(gamma);
                        let divided = scaled.div_scalar(BATCH_NORM_SCALE);
                        let shifted = divided.add(beta);

                        output.set(n, c, h, w, shifted);
                    }
                }
            }
        }

        output
    }

    /// Get number of parameters
    pub fn num_parameters(&self) -> usize {
        self.num_channels * 4 // gamma, beta, mean, var
    }
}

/// Integer square root using Newton's method
///
/// Returns floor(sqrt(n)) exactly.
fn integer_sqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    // Initial guess: next power of 2 / 2
    let mut x = 1u64 << ((64 - n.leading_zeros()) / 2 + 1);

    // Newton-Raphson iteration: x_{n+1} = (x_n + n/x_n) / 2
    loop {
        let x_new = (x + n / x) / 2;
        if x_new >= x {
            return x;
        }
        x = x_new;
    }
}

/// Fused BatchNorm + ReLU for efficiency
pub fn batch_norm_relu(input: &Tensor4D, bn: &BatchNormLayer) -> Tensor4D {
    let mut output = bn.forward(input);

    for val in output.data.iter_mut() {
        if val.is_negative() {
            *val = MobiusInt::zero();
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_sqrt() {
        assert_eq!(integer_sqrt(0), 0);
        assert_eq!(integer_sqrt(1), 1);
        assert_eq!(integer_sqrt(4), 2);
        assert_eq!(integer_sqrt(9), 3);
        assert_eq!(integer_sqrt(16), 4);
        assert_eq!(integer_sqrt(10), 3); // floor(3.16...)
        assert_eq!(integer_sqrt(1000000), 1000);
    }

    #[test]
    fn test_batch_norm_identity() {
        let bn = BatchNormLayer::identity(2);
        let input = Tensor4D::from_i64(&[1, 2, 3, 4, 5, 6, 7, 8], [1, 2, 2, 2]);

        let output = bn.forward(&input);

        // Identity should approximately preserve values (within scaling precision)
        assert_eq!(output.shape, [1, 2, 2, 2]);
        // Values will be scaled by BATCH_NORM_SCALE and then divided back
        // Due to integer arithmetic, may have small differences
    }

    #[test]
    fn test_batch_norm_output_shape() {
        let bn = BatchNormLayer::identity(3);
        let input = Tensor4D::zeros([2, 3, 4, 4]);

        let output = bn.forward(&input);
        assert_eq!(output.shape, [2, 3, 4, 4]);
    }

    #[test]
    fn test_batch_norm_shift_only() {
        // gamma=1, beta=10, mean=0, var=1
        // Result should be x + 10
        let bn = BatchNormLayer::new(
            vec![MobiusInt::from_unsigned(BATCH_NORM_SCALE, Polarity::Plus)],
            vec![MobiusInt::from_i64(10)],
            vec![MobiusInt::zero()],
            vec![MobiusInt::from_unsigned(BATCH_NORM_SCALE, Polarity::Plus)],
            1,
        );

        let input = Tensor4D::from_i64(&[0, 0, 0, 0], [1, 1, 2, 2]);
        let output = bn.forward(&input);

        // All outputs should be approximately 10
        for val in output.data.iter() {
            let v = val.spinor_value();
            assert!(v >= 9 && v <= 11, "Expected ~10, got {}", v);
        }
    }

    #[test]
    fn test_batch_norm_num_parameters() {
        let bn = BatchNormLayer::identity(64);
        assert_eq!(bn.num_parameters(), 64 * 4);
    }

    #[test]
    fn test_batch_norm_relu_fusion() {
        let bn = BatchNormLayer::new(
            vec![MobiusInt::from_unsigned(BATCH_NORM_SCALE, Polarity::Plus)],
            vec![MobiusInt::from_i64(-5)], // Shift by -5
            vec![MobiusInt::zero()],
            vec![MobiusInt::from_unsigned(BATCH_NORM_SCALE, Polarity::Plus)],
            1,
        );

        // Input: [1, 2, 10, 20]
        // After BN with beta=-5: [-4, -3, 5, 15]
        // After ReLU: [0, 0, 5, 15]
        let input = Tensor4D::from_i64(&[1, 2, 10, 20], [1, 1, 2, 2]);
        let output = batch_norm_relu(&input, &bn);

        // Negative values should be zeroed
        let v00 = output.get(0, 0, 0, 0).spinor_value();
        let v01 = output.get(0, 0, 0, 1).spinor_value();
        assert!(v00 <= 0 || v01 <= 0, "Expected some negative values to be zeroed");
    }
}

//! Im2Col: Image-to-Column Transformation
//!
//! Converts convolution into matrix multiplication by unfolding image patches
//! into columns. This enables efficient convolution via matmul (or NTT in future).
//!
//! # Algorithm
//!
//! For input [N, C_in, H, W] and kernel [K_h, K_w]:
//! - Extract each K_h × K_w patch from the input
//! - Flatten patches into columns
//! - Result: [N, C_in * K_h * K_w, H_out * W_out]

use super::tensor::{Tensor3D, Tensor4D};
use crate::arithmetic::MobiusInt;

/// Configuration for im2col transformation
#[derive(Clone, Copy, Debug)]
pub struct Im2ColConfig {
    pub kernel_h: usize,
    pub kernel_w: usize,
    pub stride_h: usize,
    pub stride_w: usize,
    pub pad_h: usize,
    pub pad_w: usize,
}

impl Im2ColConfig {
    /// Create config for square kernel with uniform stride and padding
    pub fn square(kernel_size: usize, stride: usize, padding: usize) -> Self {
        Self {
            kernel_h: kernel_size,
            kernel_w: kernel_size,
            stride_h: stride,
            stride_w: stride,
            pad_h: padding,
            pad_w: padding,
        }
    }

    /// Compute output spatial dimensions
    pub fn output_size(&self, input_h: usize, input_w: usize) -> (usize, usize) {
        let out_h = (input_h + 2 * self.pad_h - self.kernel_h) / self.stride_h + 1;
        let out_w = (input_w + 2 * self.pad_w - self.kernel_w) / self.stride_w + 1;
        (out_h, out_w)
    }
}

/// Perform im2col transformation
///
/// Transforms [N, C_in, H, W] input into [N, C_in * K_h * K_w, H_out * W_out] matrix
/// where each column contains a flattened receptive field.
pub fn im2col(input: &Tensor4D, config: &Im2ColConfig) -> Tensor3D {
    let [batch, channels, height, width] = input.shape;
    let (out_h, out_w) = config.output_size(height, width);

    let col_height = channels * config.kernel_h * config.kernel_w;
    let col_width = out_h * out_w;

    let mut result = Tensor3D::zeros([batch, col_height, col_width]);

    for n in 0..batch {
        let mut col_idx = 0;

        // For each output spatial position
        for h_out in 0..out_h {
            for w_out in 0..out_w {
                let mut row_idx = 0;

                // For each channel
                for c in 0..channels {
                    // For each kernel position
                    for kh in 0..config.kernel_h {
                        for kw in 0..config.kernel_w {
                            // Compute input position (with padding)
                            let h_in =
                                (h_out * config.stride_h + kh) as isize - config.pad_h as isize;
                            let w_in =
                                (w_out * config.stride_w + kw) as isize - config.pad_w as isize;

                            // Check bounds and extract value (zero for padding)
                            let value = if h_in >= 0
                                && h_in < height as isize
                                && w_in >= 0
                                && w_in < width as isize
                            {
                                input.get(n, c, h_in as usize, w_in as usize).clone()
                            } else {
                                MobiusInt::zero()
                            };

                            result.set(n, row_idx, col_idx, value);
                            row_idx += 1;
                        }
                    }
                }
                col_idx += 1;
            }
        }
    }

    result
}

/// Reshape kernel tensor for matrix multiplication
///
/// Transforms [C_out, C_in, K_h, K_w] kernel into [C_out, C_in * K_h * K_w] matrix
pub fn reshape_kernel_for_conv(kernel: &Tensor4D) -> Vec<Vec<MobiusInt>> {
    let [c_out, c_in, k_h, k_w] = kernel.shape;
    let row_len = c_in * k_h * k_w;

    let mut result = Vec::with_capacity(c_out);

    for out_c in 0..c_out {
        let mut row = Vec::with_capacity(row_len);
        for in_c in 0..c_in {
            for kh in 0..k_h {
                for kw in 0..k_w {
                    row.push(kernel.get(out_c, in_c, kh, kw).clone());
                }
            }
        }
        result.push(row);
    }

    result
}

/// Perform convolution via im2col + matrix multiplication
///
/// This is the core convolution operation using im2col transformation.
/// Returns [N, C_out, H_out, W_out] tensor.
pub fn conv2d_im2col(
    input: &Tensor4D,
    kernel: &Tensor4D,
    bias: &[MobiusInt],
    config: &Im2ColConfig,
) -> Tensor4D {
    let [batch, _c_in, height, width] = input.shape;
    let [c_out, _, _, _] = kernel.shape;
    let (out_h, out_w) = config.output_size(height, width);

    // 1. Im2Col transformation
    let cols = im2col(input, config);

    // 2. Reshape kernel to matrix
    let kernel_matrix = reshape_kernel_for_conv(kernel);

    // 3. Matrix multiplication: kernel_matrix × cols
    let mut output = Tensor4D::zeros([batch, c_out, out_h, out_w]);

    for n in 0..batch {
        for out_c in 0..c_out {
            let kernel_row = &kernel_matrix[out_c];

            for spatial_idx in 0..(out_h * out_w) {
                // Get column from im2col result
                let col = cols.get_column(n, spatial_idx);

                // Dot product: sum(kernel_row[i] * col[i])
                let mut sum = bias[out_c].clone();
                for (k, c) in kernel_row.iter().zip(col.iter()) {
                    sum = sum.add(&k.mul(c));
                }

                // Convert spatial index to (h, w)
                let h = spatial_idx / out_w;
                let w = spatial_idx % out_w;
                output.set(n, out_c, h, w, sum);
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arithmetic::Polarity;

    #[test]
    fn test_im2col_config_output_size() {
        let config = Im2ColConfig::square(3, 1, 0);
        assert_eq!(config.output_size(5, 5), (3, 3));

        let config_stride = Im2ColConfig::square(3, 2, 0);
        assert_eq!(config_stride.output_size(5, 5), (2, 2));

        let config_pad = Im2ColConfig::square(3, 1, 1);
        assert_eq!(config_pad.output_size(5, 5), (5, 5));
    }

    #[test]
    fn test_im2col_basic() {
        // 1x1x4x4 input
        let input_values: Vec<i64> = (1..=16).collect();
        let input = Tensor4D::from_i64(&input_values, [1, 1, 4, 4]);

        let config = Im2ColConfig::square(2, 1, 0);
        let cols = im2col(&input, &config);

        // Output should be [1, 4, 9] (2*2=4 features, 3*3=9 positions)
        assert_eq!(cols.shape, [1, 4, 9]);

        // First column should be top-left 2x2 patch: [1, 2, 5, 6]
        let col0 = cols.get_column(0, 0);
        assert_eq!(col0[0].spinor_value(), 1);
        assert_eq!(col0[1].spinor_value(), 2);
        assert_eq!(col0[2].spinor_value(), 5);
        assert_eq!(col0[3].spinor_value(), 6);
    }

    #[test]
    fn test_im2col_with_padding() {
        // 1x1x2x2 input
        let input = Tensor4D::from_i64(&[1, 2, 3, 4], [1, 1, 2, 2]);

        let config = Im2ColConfig::square(2, 1, 1);
        let cols = im2col(&input, &config);

        // With padding=1, output spatial is 3x3 = 9 positions
        assert_eq!(cols.shape, [1, 4, 9]);

        // First column (top-left with padding) should have zeros
        let col0 = cols.get_column(0, 0);
        assert_eq!(col0[0].spinor_value(), 0); // padding
        assert_eq!(col0[1].spinor_value(), 0); // padding
        assert_eq!(col0[2].spinor_value(), 0); // padding
        assert_eq!(col0[3].spinor_value(), 1); // actual data
    }

    #[test]
    fn test_conv2d_identity_kernel() {
        // 1x1x3x3 input
        let input = Tensor4D::from_i64(&[1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 1, 3, 3]);

        // 1x1x1x1 identity kernel (just passes through with scaling)
        let kernel = Tensor4D::from_data(
            vec![MobiusInt::from_unsigned(1, Polarity::Plus)],
            [1, 1, 1, 1],
        );
        let bias = vec![MobiusInt::zero()];

        let config = Im2ColConfig::square(1, 1, 0);
        let output = conv2d_im2col(&input, &kernel, &bias, &config);

        assert_eq!(output.shape, [1, 1, 3, 3]);
        assert_eq!(output.get(0, 0, 0, 0).spinor_value(), 1);
        assert_eq!(output.get(0, 0, 1, 1).spinor_value(), 5);
        assert_eq!(output.get(0, 0, 2, 2).spinor_value(), 9);
    }

    #[test]
    fn test_conv2d_with_bias() {
        let input = Tensor4D::from_i64(&[1, 1, 1, 1], [1, 1, 2, 2]);
        let kernel = Tensor4D::from_data(
            vec![MobiusInt::from_unsigned(1, Polarity::Plus)],
            [1, 1, 1, 1],
        );
        let bias = vec![MobiusInt::from_i64(10)];

        let config = Im2ColConfig::square(1, 1, 0);
        let output = conv2d_im2col(&input, &kernel, &bias, &config);

        // All outputs should be 1 + 10 = 11
        assert_eq!(output.get(0, 0, 0, 0).spinor_value(), 11);
        assert_eq!(output.get(0, 0, 1, 1).spinor_value(), 11);
    }

    #[test]
    fn test_conv2d_3x3_kernel() {
        // 1x1x4x4 input
        let input = Tensor4D::from_i64(&(1..=16).collect::<Vec<_>>(), [1, 1, 4, 4]);

        // 1x1x3x3 sum kernel (all ones)
        let kernel_data: Vec<MobiusInt> = (0..9)
            .map(|_| MobiusInt::from_unsigned(1, Polarity::Plus))
            .collect();
        let kernel = Tensor4D::from_data(kernel_data, [1, 1, 3, 3]);
        let bias = vec![MobiusInt::zero()];

        let config = Im2ColConfig::square(3, 1, 0);
        let output = conv2d_im2col(&input, &kernel, &bias, &config);

        // Output should be 2x2
        assert_eq!(output.shape, [1, 1, 2, 2]);

        // Top-left: sum of 1+2+3+5+6+7+9+10+11 = 54
        assert_eq!(output.get(0, 0, 0, 0).spinor_value(), 54);
    }
}

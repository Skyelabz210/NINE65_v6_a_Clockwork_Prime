//! Pooling Layers for CNN
//!
//! MaxPool2D and AvgPool2D using sliding window operations.
//! Uses MobiusInt for correct signed comparisons and arithmetic.

use super::tensor::Tensor4D;
use crate::arithmetic::{MobiusInt, Polarity};

/// Type of pooling operation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PoolType {
    /// Maximum pooling (takes max value in window)
    Max,
    /// Average pooling (takes mean value in window)
    Average,
}

/// Generic pooling layer configuration
#[derive(Clone, Copy, Debug)]
pub struct PoolLayer {
    pub pool_type: PoolType,
    pub kernel_size: usize,
    pub stride: usize,
    pub padding: usize,
}

impl PoolLayer {
    /// Create new pooling layer
    pub fn new(pool_type: PoolType, kernel_size: usize, stride: usize, padding: usize) -> Self {
        Self {
            pool_type,
            kernel_size,
            stride,
            padding,
        }
    }

    /// Create max pooling layer with stride = kernel_size (typical usage)
    pub fn max_pool(kernel_size: usize) -> Self {
        Self::new(PoolType::Max, kernel_size, kernel_size, 0)
    }

    /// Create average pooling layer with stride = kernel_size
    pub fn avg_pool(kernel_size: usize) -> Self {
        Self::new(PoolType::Average, kernel_size, kernel_size, 0)
    }

    /// Compute output spatial dimensions
    pub fn output_size(&self, input_h: usize, input_w: usize) -> (usize, usize) {
        let out_h = (input_h + 2 * self.padding - self.kernel_size) / self.stride + 1;
        let out_w = (input_w + 2 * self.padding - self.kernel_size) / self.stride + 1;
        (out_h, out_w)
    }

    /// Compute output shape given input shape
    pub fn output_shape(&self, input_shape: [usize; 4]) -> [usize; 4] {
        let [n, c, h, w] = input_shape;
        let (out_h, out_w) = self.output_size(h, w);
        [n, c, out_h, out_w]
    }

    /// Forward pass through the pooling layer
    pub fn forward(&self, input: &Tensor4D) -> Tensor4D {
        match self.pool_type {
            PoolType::Max => max_pool2d(input, self.kernel_size, self.stride, self.padding),
            PoolType::Average => avg_pool2d(input, self.kernel_size, self.stride, self.padding),
        }
    }
}

/// 2D Max Pooling layer
#[derive(Clone, Copy, Debug)]
pub struct MaxPool2D {
    pub kernel_size: usize,
    pub stride: usize,
    pub padding: usize,
}

impl MaxPool2D {
    /// Create new max pooling layer
    pub fn new(kernel_size: usize, stride: usize, padding: usize) -> Self {
        Self {
            kernel_size,
            stride,
            padding,
        }
    }

    /// Create with stride equal to kernel size (no overlap)
    pub fn simple(kernel_size: usize) -> Self {
        Self::new(kernel_size, kernel_size, 0)
    }

    /// Forward pass
    pub fn forward(&self, input: &Tensor4D) -> Tensor4D {
        max_pool2d(input, self.kernel_size, self.stride, self.padding)
    }

    /// Get output shape
    pub fn output_shape(&self, input_shape: [usize; 4]) -> [usize; 4] {
        let [n, c, h, w] = input_shape;
        let out_h = (h + 2 * self.padding - self.kernel_size) / self.stride + 1;
        let out_w = (w + 2 * self.padding - self.kernel_size) / self.stride + 1;
        [n, c, out_h, out_w]
    }
}

/// 2D Average Pooling layer
#[derive(Clone, Copy, Debug)]
pub struct AvgPool2D {
    pub kernel_size: usize,
    pub stride: usize,
    pub padding: usize,
}

impl AvgPool2D {
    /// Create new average pooling layer
    pub fn new(kernel_size: usize, stride: usize, padding: usize) -> Self {
        Self {
            kernel_size,
            stride,
            padding,
        }
    }

    /// Create with stride equal to kernel size
    pub fn simple(kernel_size: usize) -> Self {
        Self::new(kernel_size, kernel_size, 0)
    }

    /// Forward pass
    pub fn forward(&self, input: &Tensor4D) -> Tensor4D {
        avg_pool2d(input, self.kernel_size, self.stride, self.padding)
    }

    /// Get output shape
    pub fn output_shape(&self, input_shape: [usize; 4]) -> [usize; 4] {
        let [n, c, h, w] = input_shape;
        let out_h = (h + 2 * self.padding - self.kernel_size) / self.stride + 1;
        let out_w = (w + 2 * self.padding - self.kernel_size) / self.stride + 1;
        [n, c, out_h, out_w]
    }
}

/// Perform 2D max pooling
///
/// Takes maximum value in each kernel_size × kernel_size window.
/// Uses MobiusInt::max for correct signed comparison.
pub fn max_pool2d(
    input: &Tensor4D,
    kernel_size: usize,
    stride: usize,
    padding: usize,
) -> Tensor4D {
    let [batch, channels, height, width] = input.shape;
    let out_h = (height + 2 * padding - kernel_size) / stride + 1;
    let out_w = (width + 2 * padding - kernel_size) / stride + 1;

    let mut output = Tensor4D::zeros([batch, channels, out_h, out_w]);

    for n in 0..batch {
        for c in 0..channels {
            for h_out in 0..out_h {
                for w_out in 0..out_w {
                    // Initialize with minimum possible value
                    let mut max_val = MobiusInt::from_i64(i64::MIN / 2);

                    // Scan the pooling window
                    for kh in 0..kernel_size {
                        for kw in 0..kernel_size {
                            let h_in = (h_out * stride + kh) as isize - padding as isize;
                            let w_in = (w_out * stride + kw) as isize - padding as isize;

                            if h_in >= 0
                                && h_in < height as isize
                                && w_in >= 0
                                && w_in < width as isize
                            {
                                let val = input.get(n, c, h_in as usize, w_in as usize);
                                max_val = max_val.max(val);
                            }
                            // Padding is treated as negative infinity (skip in max)
                        }
                    }

                    output.set(n, c, h_out, w_out, max_val);
                }
            }
        }
    }

    output
}

/// Perform 2D average pooling
///
/// Computes mean of each kernel_size × kernel_size window.
/// Uses integer division (truncates).
pub fn avg_pool2d(
    input: &Tensor4D,
    kernel_size: usize,
    stride: usize,
    padding: usize,
) -> Tensor4D {
    let [batch, channels, height, width] = input.shape;
    let out_h = (height + 2 * padding - kernel_size) / stride + 1;
    let out_w = (width + 2 * padding - kernel_size) / stride + 1;

    let mut output = Tensor4D::zeros([batch, channels, out_h, out_w]);
    let pool_area = kernel_size * kernel_size;

    for n in 0..batch {
        for c in 0..channels {
            for h_out in 0..out_h {
                for w_out in 0..out_w {
                    let mut sum = MobiusInt::zero();
                    let mut count = 0usize;

                    // Scan the pooling window
                    for kh in 0..kernel_size {
                        for kw in 0..kernel_size {
                            let h_in = (h_out * stride + kh) as isize - padding as isize;
                            let w_in = (w_out * stride + kw) as isize - padding as isize;

                            if h_in >= 0
                                && h_in < height as isize
                                && w_in >= 0
                                && w_in < width as isize
                            {
                                let val = input.get(n, c, h_in as usize, w_in as usize);
                                sum = sum.add(val);
                                count += 1;
                            }
                            // Padding contributes 0 to sum
                        }
                    }

                    // Integer division for average
                    let divisor = if padding > 0 { count } else { pool_area };
                    let avg = if divisor > 0 {
                        sum.div_scalar(divisor as u64)
                    } else {
                        MobiusInt::zero()
                    };

                    output.set(n, c, h_out, w_out, avg);
                }
            }
        }
    }

    output
}

/// Global average pooling (reduces spatial dimensions to 1×1)
pub fn global_avg_pool2d(input: &Tensor4D) -> Tensor4D {
    let [batch, channels, height, width] = input.shape;
    let mut output = Tensor4D::zeros([batch, channels, 1, 1]);
    let total = height * width;

    for n in 0..batch {
        for c in 0..channels {
            let mut sum = MobiusInt::zero();
            for h in 0..height {
                for w in 0..width {
                    sum = sum.add(input.get(n, c, h, w));
                }
            }
            let avg = sum.div_scalar(total as u64);
            output.set(n, c, 0, 0, avg);
        }
    }

    output
}

/// Global max pooling (reduces spatial dimensions to 1×1)
pub fn global_max_pool2d(input: &Tensor4D) -> Tensor4D {
    let [batch, channels, height, width] = input.shape;
    let mut output = Tensor4D::zeros([batch, channels, 1, 1]);

    for n in 0..batch {
        for c in 0..channels {
            let mut max_val = MobiusInt::from_i64(i64::MIN / 2);
            for h in 0..height {
                for w in 0..width {
                    max_val = max_val.max(input.get(n, c, h, w));
                }
            }
            output.set(n, c, 0, 0, max_val);
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_layer_output_shape() {
        let pool = PoolLayer::max_pool(2);
        assert_eq!(pool.output_shape([1, 3, 4, 4]), [1, 3, 2, 2]);

        let pool_stride1 = PoolLayer::new(PoolType::Max, 2, 1, 0);
        assert_eq!(pool_stride1.output_shape([1, 3, 4, 4]), [1, 3, 3, 3]);
    }

    #[test]
    fn test_max_pool_2x2() {
        // Input: 4x4 with values 1-16
        let input = Tensor4D::from_i64(&(1..=16).collect::<Vec<_>>(), [1, 1, 4, 4]);

        let output = max_pool2d(&input, 2, 2, 0);

        assert_eq!(output.shape, [1, 1, 2, 2]);
        // Max of [1,2,5,6] = 6
        assert_eq!(output.get(0, 0, 0, 0).spinor_value(), 6);
        // Max of [3,4,7,8] = 8
        assert_eq!(output.get(0, 0, 0, 1).spinor_value(), 8);
        // Max of [9,10,13,14] = 14
        assert_eq!(output.get(0, 0, 1, 0).spinor_value(), 14);
        // Max of [11,12,15,16] = 16
        assert_eq!(output.get(0, 0, 1, 1).spinor_value(), 16);
    }

    #[test]
    fn test_avg_pool_2x2() {
        // Input: 2x2 with values [4, 8, 12, 16]
        let input = Tensor4D::from_i64(&[4, 8, 12, 16], [1, 1, 2, 2]);

        let output = avg_pool2d(&input, 2, 2, 0);

        assert_eq!(output.shape, [1, 1, 1, 1]);
        // Average of [4, 8, 12, 16] = 40/4 = 10
        assert_eq!(output.get(0, 0, 0, 0).spinor_value(), 10);
    }

    #[test]
    fn test_max_pool_with_negatives() {
        // Test that signed comparison works correctly
        let input = Tensor4D::from_i64(&[-5, -3, -10, -1], [1, 1, 2, 2]);

        let output = max_pool2d(&input, 2, 2, 0);

        // Max should be -1 (least negative)
        assert_eq!(output.get(0, 0, 0, 0).spinor_value(), -1);
    }

    #[test]
    fn test_global_avg_pool() {
        let input = Tensor4D::from_i64(&[1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 1, 3, 3]);

        let output = global_avg_pool2d(&input);

        assert_eq!(output.shape, [1, 1, 1, 1]);
        // Average of 1-9 = 45/9 = 5
        assert_eq!(output.get(0, 0, 0, 0).spinor_value(), 5);
    }

    #[test]
    fn test_global_max_pool() {
        let input = Tensor4D::from_i64(&[1, 5, 3, 9, 2, 4, 7, 8, 6], [1, 1, 3, 3]);

        let output = global_max_pool2d(&input);

        assert_eq!(output.shape, [1, 1, 1, 1]);
        assert_eq!(output.get(0, 0, 0, 0).spinor_value(), 9);
    }

    #[test]
    fn test_pool_layer_forward() {
        let input = Tensor4D::from_i64(&(1..=16).collect::<Vec<_>>(), [1, 1, 4, 4]);

        let max_pool = PoolLayer::max_pool(2);
        let max_out = max_pool.forward(&input);
        assert_eq!(max_out.shape, [1, 1, 2, 2]);

        let avg_pool = PoolLayer::avg_pool(2);
        let avg_out = avg_pool.forward(&input);
        assert_eq!(avg_out.shape, [1, 1, 2, 2]);
    }

    #[test]
    fn test_maxpool2d_layer() {
        let layer = MaxPool2D::simple(2);
        let input = Tensor4D::from_i64(&(1..=16).collect::<Vec<_>>(), [1, 1, 4, 4]);

        let output = layer.forward(&input);
        assert_eq!(output.shape, [1, 1, 2, 2]);
        assert_eq!(layer.output_shape([1, 1, 4, 4]), [1, 1, 2, 2]);
    }
}

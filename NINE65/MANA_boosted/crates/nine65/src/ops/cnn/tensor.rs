//! Tensor Data Structures for CNN Operations
//!
//! NCHW (Channel-First) memory layout for optimal convolution performance.
//! Uses MobiusInt for correct signed arithmetic throughout.

use crate::arithmetic::{MobiusInt, Polarity};

/// Memory layout for tensors
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    /// Batch-Channel-Height-Width (default for convolution)
    NCHW,
    /// Batch-Height-Width-Channel (better for dense layers)
    NHWC,
}

/// 4D Tensor for CNN operations [batch, channels, height, width]
#[derive(Clone, Debug)]
pub struct Tensor4D {
    /// Flattened storage in NCHW order
    pub data: Vec<MobiusInt>,
    /// Shape [N, C, H, W]
    pub shape: [usize; 4],
    /// Memory layout
    pub layout: Layout,
}

impl Tensor4D {
    /// Create new tensor with zeros
    pub fn zeros(shape: [usize; 4]) -> Self {
        let size = shape.iter().product();
        Self {
            data: vec![MobiusInt::zero(); size],
            shape,
            layout: Layout::NCHW,
        }
    }

    /// Create tensor from existing data
    pub fn from_data(data: Vec<MobiusInt>, shape: [usize; 4]) -> Self {
        let expected_size: usize = shape.iter().product();
        assert_eq!(
            data.len(),
            expected_size,
            "Data length {} doesn't match shape {:?} (expected {})",
            data.len(),
            shape,
            expected_size
        );
        Self {
            data,
            shape,
            layout: Layout::NCHW,
        }
    }

    /// Create tensor from i64 values (convenience)
    pub fn from_i64(values: &[i64], shape: [usize; 4]) -> Self {
        let data: Vec<MobiusInt> = values.iter().map(|&v| MobiusInt::from_i64(v)).collect();
        Self::from_data(data, shape)
    }

    /// Create tensor from u64 values with polarity
    pub fn from_u64(values: &[u64], shape: [usize; 4]) -> Self {
        let data: Vec<MobiusInt> = values
            .iter()
            .map(|&v| MobiusInt::from_unsigned(v, Polarity::Plus))
            .collect();
        Self::from_data(data, shape)
    }

    /// Get batch size (N)
    #[inline]
    pub fn batch_size(&self) -> usize {
        self.shape[0]
    }

    /// Get number of channels (C)
    #[inline]
    pub fn channels(&self) -> usize {
        self.shape[1]
    }

    /// Get height (H)
    #[inline]
    pub fn height(&self) -> usize {
        self.shape[2]
    }

    /// Get width (W)
    #[inline]
    pub fn width(&self) -> usize {
        self.shape[3]
    }

    /// Total number of elements
    #[inline]
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Compute linear index for NCHW layout
    #[inline]
    fn linear_index(&self, n: usize, c: usize, h: usize, w: usize) -> usize {
        debug_assert!(n < self.shape[0], "n={} >= N={}", n, self.shape[0]);
        debug_assert!(c < self.shape[1], "c={} >= C={}", c, self.shape[1]);
        debug_assert!(h < self.shape[2], "h={} >= H={}", h, self.shape[2]);
        debug_assert!(w < self.shape[3], "w={} >= W={}", w, self.shape[3]);

        let [_, c_dim, h_dim, w_dim] = self.shape;
        n * c_dim * h_dim * w_dim + c * h_dim * w_dim + h * w_dim + w
    }

    /// Get element at position (n, c, h, w)
    #[inline]
    pub fn get(&self, n: usize, c: usize, h: usize, w: usize) -> &MobiusInt {
        let idx = self.linear_index(n, c, h, w);
        &self.data[idx]
    }

    /// Get mutable element at position (n, c, h, w)
    #[inline]
    pub fn get_mut(&mut self, n: usize, c: usize, h: usize, w: usize) -> &mut MobiusInt {
        let idx = self.linear_index(n, c, h, w);
        &mut self.data[idx]
    }

    /// Set element at position (n, c, h, w)
    #[inline]
    pub fn set(&mut self, n: usize, c: usize, h: usize, w: usize, value: MobiusInt) {
        let idx = self.linear_index(n, c, h, w);
        self.data[idx] = value;
    }

    /// Get a single channel as a slice (zero-copy for NCHW)
    pub fn channel_slice(&self, n: usize, c: usize) -> &[MobiusInt] {
        let [_, c_dim, h_dim, w_dim] = self.shape;
        let start = n * c_dim * h_dim * w_dim + c * h_dim * w_dim;
        let end = start + h_dim * w_dim;
        &self.data[start..end]
    }

    /// Get a single channel as mutable slice
    pub fn channel_slice_mut(&mut self, n: usize, c: usize) -> &mut [MobiusInt] {
        let [_, c_dim, h_dim, w_dim] = self.shape;
        let start = n * c_dim * h_dim * w_dim + c * h_dim * w_dim;
        let end = start + h_dim * w_dim;
        &mut self.data[start..end]
    }

    /// Pad tensor with zeros to specified dimensions
    pub fn pad(&self, pad_h: usize, pad_w: usize) -> Self {
        let [n, c, h, w] = self.shape;
        let new_h = h + 2 * pad_h;
        let new_w = w + 2 * pad_w;
        let mut result = Self::zeros([n, c, new_h, new_w]);

        for batch in 0..n {
            for chan in 0..c {
                for row in 0..h {
                    for col in 0..w {
                        let val = self.get(batch, chan, row, col).clone();
                        result.set(batch, chan, row + pad_h, col + pad_w, val);
                    }
                }
            }
        }

        result
    }

    /// Flatten tensor to 1D vector (for dense layer input)
    pub fn flatten(&self) -> Vec<MobiusInt> {
        self.data.clone()
    }

    /// Reshape tensor (must have same total elements)
    pub fn reshape(&self, new_shape: [usize; 4]) -> Self {
        let expected: usize = new_shape.iter().product();
        assert_eq!(
            self.size(),
            expected,
            "Cannot reshape {} elements to shape {:?}",
            self.size(),
            new_shape
        );
        Self {
            data: self.data.clone(),
            shape: new_shape,
            layout: self.layout,
        }
    }

    /// Create tensor filled with a constant value
    pub fn fill(shape: [usize; 4], value: MobiusInt) -> Self {
        let size = shape.iter().product();
        Self {
            data: vec![value; size],
            shape,
            layout: Layout::NCHW,
        }
    }
}

/// 3D Tensor for intermediate results [batch, features, spatial]
#[derive(Clone, Debug)]
pub struct Tensor3D {
    pub data: Vec<MobiusInt>,
    pub shape: [usize; 3],
}

impl Tensor3D {
    /// Create new 3D tensor with zeros
    pub fn zeros(shape: [usize; 3]) -> Self {
        let size = shape.iter().product();
        Self {
            data: vec![MobiusInt::zero(); size],
            shape,
        }
    }

    /// Create from data
    pub fn from_data(data: Vec<MobiusInt>, shape: [usize; 3]) -> Self {
        let expected: usize = shape.iter().product();
        assert_eq!(data.len(), expected);
        Self { data, shape }
    }

    /// Get element at (i, j, k)
    #[inline]
    pub fn get(&self, i: usize, j: usize, k: usize) -> &MobiusInt {
        let idx = i * self.shape[1] * self.shape[2] + j * self.shape[2] + k;
        &self.data[idx]
    }

    /// Set element at (i, j, k)
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, k: usize, value: MobiusInt) {
        let idx = i * self.shape[1] * self.shape[2] + j * self.shape[2] + k;
        self.data[idx] = value;
    }

    /// Get a column (for im2col result access)
    pub fn get_column(&self, batch: usize, col: usize) -> Vec<MobiusInt> {
        let [_, rows, _] = self.shape;
        (0..rows).map(|r| self.get(batch, r, col).clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor4d_zeros() {
        let t = Tensor4D::zeros([2, 3, 4, 5]);
        assert_eq!(t.size(), 2 * 3 * 4 * 5);
        assert_eq!(t.batch_size(), 2);
        assert_eq!(t.channels(), 3);
        assert_eq!(t.height(), 4);
        assert_eq!(t.width(), 5);
    }

    #[test]
    fn test_tensor4d_indexing() {
        let mut t = Tensor4D::zeros([1, 2, 3, 4]);
        let val = MobiusInt::from_unsigned(42, Polarity::Plus);
        t.set(0, 1, 2, 3, val.clone());
        assert_eq!(t.get(0, 1, 2, 3).residue, 42);
    }

    #[test]
    fn test_tensor4d_channel_slice() {
        let values: Vec<i64> = (0..24).collect();
        let t = Tensor4D::from_i64(&values, [1, 2, 3, 4]);

        let ch0 = t.channel_slice(0, 0);
        assert_eq!(ch0.len(), 12);
        assert_eq!(ch0[0].spinor_value(), 0);

        let ch1 = t.channel_slice(0, 1);
        assert_eq!(ch1.len(), 12);
        assert_eq!(ch1[0].spinor_value(), 12);
    }

    #[test]
    fn test_tensor4d_padding() {
        let t = Tensor4D::from_i64(&[1, 2, 3, 4], [1, 1, 2, 2]);
        let padded = t.pad(1, 1);

        assert_eq!(padded.shape, [1, 1, 4, 4]);
        // Original values should be in center
        assert_eq!(padded.get(0, 0, 1, 1).spinor_value(), 1);
        assert_eq!(padded.get(0, 0, 1, 2).spinor_value(), 2);
        assert_eq!(padded.get(0, 0, 2, 1).spinor_value(), 3);
        assert_eq!(padded.get(0, 0, 2, 2).spinor_value(), 4);
        // Padding should be zero
        assert_eq!(padded.get(0, 0, 0, 0).spinor_value(), 0);
    }

    #[test]
    fn test_tensor3d_column_access() {
        let t = Tensor3D::from_data(
            (0..12).map(|v| MobiusInt::from_i64(v)).collect(),
            [1, 3, 4],
        );

        let col = t.get_column(0, 0);
        assert_eq!(col.len(), 3);
        assert_eq!(col[0].spinor_value(), 0);
        assert_eq!(col[1].spinor_value(), 4);
        assert_eq!(col[2].spinor_value(), 8);
    }
}

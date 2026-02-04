//! CNN Operations for NINE65 FHE
//!
//! NTT-accelerated 2D convolution via Im2Col + frequency-domain multiplication.
//! Uses QMNF integer-only arithmetic with MobiusInt for signed values.
//!
//! # Innovation Integration
//!
//! - **MobiusInt**: Signed arithmetic for weights/activations
//! - **NTT/FFT**: O(N log N) convolution via frequency domain
//! - **Persistent Montgomery**: Minimize modular reductions
//! - **MQ-ReLU**: O(1) activation functions
//!
//! # Architecture
//!
//! ```text
//! Input [N,C,H,W] → Im2Col → NTT Convolution → Activation → Output [N,C',H',W']
//! ```

pub mod tensor;
pub mod im2col;
pub mod conv2d;
pub mod pooling;
pub mod batch_norm;

pub use tensor::{Tensor4D, Tensor3D, Layout};
pub use im2col::{im2col, Im2ColConfig};
pub use conv2d::{Conv2DLayer, Conv2DConfig};
pub use pooling::{PoolLayer, PoolType, MaxPool2D, AvgPool2D};
pub use batch_norm::BatchNormLayer;

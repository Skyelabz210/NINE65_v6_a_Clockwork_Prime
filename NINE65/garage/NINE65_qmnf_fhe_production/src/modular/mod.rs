//! Modular layer traits for QMNF V2 composition.

pub mod compute;
pub mod dispatcher;
pub mod domain;
pub mod entropy;

pub use compute::{ComputeBackend, ComputeOp};
pub use dispatcher::{ComputeDispatcher, ComputeTarget};
pub use domain::{DomainCapability, DomainModule};
pub use entropy::EntropySource;

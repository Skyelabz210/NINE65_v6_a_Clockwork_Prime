//! Compute routing trait for multi-engine dispatch.

use super::compute::ComputeOp;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComputeTarget {
    Primary,
    Secondary,
}

pub trait ComputeDispatcher {
    fn route(&self, op: ComputeOp) -> ComputeTarget;
}

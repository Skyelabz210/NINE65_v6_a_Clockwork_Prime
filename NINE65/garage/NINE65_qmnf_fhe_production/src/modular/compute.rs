//! Compute backend traits for modular composition.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComputeOp {
    Add,
    Sub,
    Mul,
    PolyMul,
    NttForward,
    NttInverse,
    Reduce,
    Compare,
    Divide,
    Sign,
    Mod,
}

pub trait ComputeBackend {
    fn add(&self, a: &[u64], b: &[u64], modulus: u64) -> Vec<u64>;
    fn sub(&self, a: &[u64], b: &[u64], modulus: u64) -> Vec<u64>;
    fn mul(&self, a: &[u64], b: &[u64], modulus: u64) -> Vec<u64>;
    fn poly_mul(&self, a: &[u64], b: &[u64], modulus: u64) -> Vec<u64>;
    fn ntt_forward(&self, a: &[u64], modulus: u64) -> Vec<u64>;
    fn ntt_inverse(&self, a: &[u64], modulus: u64) -> Vec<u64>;
}

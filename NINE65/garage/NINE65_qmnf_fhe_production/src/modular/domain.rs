//! Domain module traits for layer-3 composition.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DomainCapability {
    Fhe,
    Ahop,
    Acc,
    Gso,
    Maa,
    Tco,
    Hcvlang,
    Protocol,
}

pub trait DomainModule {
    fn name(&self) -> &'static str;
    fn capabilities(&self) -> &'static [DomainCapability];
}

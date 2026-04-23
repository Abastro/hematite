use crate::scalar::modulus::Mod64;

/// Denotes modular arithmetic with barrett reduction.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Barrett {
    pub lazy: BarrettLazy,
}

/// Lazy reduction version of Barrett reduction.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BarrettLazy {
    pub modulus: u64,
    barrett_shift: u64,
    barrett_const: u64,
}

impl BarrettLazy {
    pub fn reduce(&self, x: Mod64) -> Mod64 {
        let ratio = (((x.representative as u128) * (self.barrett_const as u128))
            >> self.barrett_shift) as u64;
        Mod64 {
            representative: x.representative - ratio * self.modulus,
        }
    }
}

impl Barrett {}

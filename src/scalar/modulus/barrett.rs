use crate::{
    engine::handle::{AddGroupOps, RingOps},
    scalar::modulus::Mod64,
    utils::bits::bitwidth,
};

/// Denotes modular arithmetic with barrett reduction.
///
/// Require 6 P < 2^64 for P = modulus.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Barrett {
    pub lazy: BarrettLazy,
}

/// Lazy reduction version of Barrett reduction.
///
/// Require 6 P < 2^64 for P = modulus.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct BarrettLazy {
    pub modulus: u64,
    barrett_pre_shift: u64,
    barrett_shift: u64,
    barrett_const: u64,
}

impl BarrettLazy {
    /// Creates a new `BarrettLazy` for the given modulus.
    pub fn new(modulus: u64) -> Self {
        debug_assert!(modulus as u128 * 6 < (1u128 << 64));
        let k = bitwidth(modulus) as u64;
        let barrett_pre_shift = k - 2;
        let barrett_shift = k + 62;
        let barrett_const = ((1u128 << barrett_shift) / modulus as u128) as u64;
        Self {
            modulus,
            barrett_pre_shift,
            barrett_shift,
            barrett_const,
        }
    }

    /// Barrett reduction of Mod64 input, reducing ? to [0, 2P).
    ///
    /// Slightly inefficient since this barrett reduction is geared towards multiplication.
    pub fn reduce(&self, inp: Mod64) -> Mod64 {
        let inter = (inp.representative as u128) * (self.barrett_const as u128);
        let ratio = (inter >> self.barrett_shift) as u64;
        Mod64 {
            representative: inp.representative - ratio * self.modulus,
        }
    }
}

impl AddGroupOps<Mod64> for BarrettLazy {
    fn zero(&self) -> Mod64 {
        Mod64 { representative: 0 }
    }

    /// Input [0, P), Output [0, 2P)
    fn add(&self, lhs: Mod64, rhs: Mod64) -> Mod64 {
        Mod64 {
            representative: lhs.representative + rhs.representative,
        }
    }

    /// Input [0, P), Output [0, P+1)
    fn neg(&self, arg: Mod64) -> Mod64 {
        Mod64 {
            representative: self.modulus - arg.representative,
        }
    }

    /// Input [0, P), Output [0, 2P)
    fn sub(&self, lhs: Mod64, rhs: Mod64) -> Mod64 {
        Mod64 {
            representative: lhs.representative + self.modulus - rhs.representative,
        }
    }
}

impl RingOps<Mod64> for BarrettLazy {
    fn one(&self) -> Mod64 {
        Mod64 { representative: 0 }
    }

    /// Input [0, P), Output [0, 2P)
    fn mul(&self, lhs: Mod64, rhs: Mod64) -> Mod64 {
        let prod = lhs.representative as u128 * rhs.representative as u128;
        let c1 = (prod >> self.barrett_pre_shift) as u64;
        let c2 = ((c1 as u128 * self.barrett_const as u128) >> 64) as u64;
        let c3 = (prod as u64).wrapping_sub(c2.wrapping_mul(self.modulus));
        Mod64 { representative: c3 }
    }
}

impl Barrett {
    /// Reduces `r` from `[0, 2P)` to `[0, P)` with a single conditional subtract.
    #[inline]
    pub fn cond_reduce(&self, inp: Mod64) -> Mod64 {
        let r = inp.representative;
        let red = if r >= self.lazy.modulus {
            r - self.lazy.modulus
        } else {
            r
        };
        Mod64 {
            representative: red,
        }
    }

    /// Fully reduce
    pub fn reduce(&self, inp: Mod64) -> Mod64 {
        let lazy_red = self.lazy.reduce(inp);
        self.cond_reduce(lazy_red)
    }
}

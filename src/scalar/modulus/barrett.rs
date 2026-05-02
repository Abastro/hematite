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
    modulus: u64,
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

    pub fn modulus(&self) -> u64 {
        self.modulus
    }

    /// Barrett reduction of Mod64 input, reducing [0, 2^64) to [0, 2P).
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
        Mod64 { representative: 1 }
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
    pub fn new(modulus: u64) -> Self {
        Barrett {
            lazy: BarrettLazy::new(modulus),
        }
    }

    pub fn modulus(&self) -> u64 {
        self.lazy.modulus
    }

    /// Reduces `r` from `[0, 2P)` to `[0, P)` with a single conditional subtract.
    #[inline]
    pub fn cond_reduce(&self, inp: Mod64) -> Mod64 {
        let r = inp.representative;
        let red = if r >= self.modulus() {
            r - self.modulus()
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

impl AddGroupOps<Mod64> for Barrett {
    fn zero(&self) -> Mod64 {
        Mod64 { representative: 0 }
    }

    fn add(&self, lhs: Mod64, rhs: Mod64) -> Mod64 {
        self.cond_reduce(self.lazy.add(lhs, rhs))
    }

    fn neg(&self, arg: Mod64) -> Mod64 {
        self.cond_reduce(self.lazy.neg(arg))
    }

    fn sub(&self, lhs: Mod64, rhs: Mod64) -> Mod64 {
        self.cond_reduce(self.lazy.sub(lhs, rhs))
    }
}

impl RingOps<Mod64> for Barrett {
    fn one(&self) -> Mod64 {
        Mod64 { representative: 1 }
    }

    fn mul(&self, lhs: Mod64, rhs: Mod64) -> Mod64 {
        self.cond_reduce(self.lazy.mul(lhs, rhs))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use proptest::prelude::*;

    use crate::scalar::modulus::{Mod64, exact::ModExact};

    const MAX_BARRETT_MODULUS: u64 = u64::MAX / 6;

    pub fn barrett_lazy() -> impl Strategy<Value = BarrettLazy> {
        (2u64..=MAX_BARRETT_MODULUS).prop_map(|m| BarrettLazy::new(m))
    }

    pub fn barrett() -> impl Strategy<Value = Barrett> {
        (2u64..=MAX_BARRETT_MODULUS).prop_map(|m| Barrett::new(m))
    }

    fn modular_value(modulus: u64) -> impl Strategy<Value = Mod64> {
        (0u64..modulus).prop_map(|representative| Mod64 { representative })
    }

    proptest! {
        #[test]
        fn barrett_reduce(barr in barrett(), val in any::<u64>()) {
            let exact = ModExact { modulus: barr.modulus() };
            prop_assert_eq!(
                barr.reduce(Mod64 { representative: val }),
                exact.embed(val),
            );
        }

        #[test]
        fn barrett_lazy_reduce(barr in barrett_lazy(), val in any::<u64>()) {
            let reduced = barr.reduce(Mod64 { representative: val });
            prop_assert!(reduced.representative < 2 * barr.modulus(), "modulus range");
            prop_assert_eq!(reduced.representative % barr.modulus(), val % barr.modulus());
        }

        #[test]
        fn barrett_add(
            (barr, (a, b)) in barrett().prop_ind_flat_map2(|barr| {
                (modular_value(barr.modulus()), modular_value(barr.modulus()))
            })
        ) {
            let exact = ModExact { modulus: barr.modulus() };
            prop_assert_eq!(barr.add(a, b), exact.add(a, b));
        }

        #[test]
        fn barrett_sub(
            (barr, (a, b)) in barrett().prop_ind_flat_map2(|barr| {
                (modular_value(barr.modulus()), modular_value(barr.modulus()))
            })
        ) {
            let exact = ModExact { modulus: barr.modulus() };
            prop_assert_eq!(barr.sub(a, b), exact.sub(a, b));
        }

        #[test]
        fn barrett_mul(
            (barr, (a, b)) in barrett().prop_ind_flat_map2(|barr| {
                (modular_value(barr.modulus()), modular_value(barr.modulus()))
            })
        ) {
            let exact = ModExact { modulus: barr.modulus() };
            prop_assert_eq!(barr.mul(a, b), exact.mul(a, b));
        }

        #[test]
        fn barrett_lazy_add(
            (barr, (a, b)) in barrett_lazy().prop_ind_flat_map2(|barr| {
                (modular_value(barr.modulus()), modular_value(barr.modulus()))
            })
        ) {
            let exact = ModExact { modulus: barr.modulus() };
            let sum = barr.add(a, b);
            prop_assert!(sum.representative < 2 * barr.modulus(), "modulus range");
            prop_assert_eq!(
                sum.representative % barr.modulus(),
                exact.add(a, b).representative,
            );
        }

        #[test]
        fn barrett_lazy_sub(
            (barr, (a, b)) in barrett_lazy().prop_ind_flat_map2(|barr| {
                (modular_value(barr.modulus()), modular_value(barr.modulus()))
            })
        ) {
            let exact = ModExact { modulus: barr.modulus() };
            let subtracted = barr.sub(a, b);
            prop_assert!(subtracted.representative < 2 * barr.modulus(), "modulus range");
            prop_assert_eq!(
                subtracted.representative % barr.modulus(),
                exact.sub(a, b).representative,
            );
        }

        #[test]
        fn barrett_lazy_mul(
            (barr, (a, b)) in barrett_lazy().prop_ind_flat_map2(|barr| {
                (modular_value(barr.modulus()), modular_value(barr.modulus()))
            })
        ) {
            let exact = ModExact { modulus: barr.modulus() };
            let product = barr.mul(a, b);
            prop_assert!(product.representative < 2 * barr.modulus(), "modulus range");
            prop_assert_eq!(
                product.representative % barr.modulus(),
                exact.mul(a, b).representative,
            );
        }
    }
}

use num_traits::Euclid;

use crate::{
    engine::handle::{AddGroupOps, FieldOps, RingOps},
    scalar::modulus::Mod64,
};

/// Denotes modular arithmetic with exact reduction.
///
/// Works for modulus < 2^63.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModExact {
    pub modulus: u64,
}

impl ModExact {
    pub fn embed(&self, val: u64) -> Mod64 {
        Mod64 {
            representative: val % self.modulus,
        }
    }

    pub fn pow(&self, base: Mod64, exp: usize) -> Mod64 {
        let mut out = Mod64 { representative: 1 };
        let mut cur_base = base;
        let mut e = exp;

        while e != 0 {
            if e & 1 == 1 {
                out = self.mul(out, cur_base);
            }

            cur_base = self.mul(cur_base, cur_base);
            e >>= 1;
        }

        out
    }
}

impl AddGroupOps<Mod64> for ModExact {
    fn zero(&self) -> Mod64 {
        return Mod64 { representative: 0 };
    }

    fn add(&self, lhs: Mod64, rhs: Mod64) -> Mod64 {
        return Mod64 {
            representative: (lhs.representative + rhs.representative) % self.modulus,
        };
    }

    fn neg(&self, arg: Mod64) -> Mod64 {
        return Mod64 {
            representative: (self.modulus - arg.representative) % self.modulus,
        };
    }

    fn sub(&self, lhs: Mod64, rhs: Mod64) -> Mod64 {
        return Mod64 {
            representative: (lhs.representative + self.modulus - rhs.representative) % self.modulus,
        };
    }
}

impl RingOps<Mod64> for ModExact {
    fn one(&self) -> Mod64 {
        return Mod64 { representative: 1 };
    }

    fn mul(&self, lhs: Mod64, rhs: Mod64) -> Mod64 {
        let prod = (lhs.representative as u128) * (rhs.representative as u128);
        return Mod64 {
            representative: (prod % (self.modulus as u128)) as u64,
        };
    }
}

impl FieldOps<Mod64> for ModExact {
    fn inv(&self, arg: Mod64) -> Mod64 {
        let gcd_res = ExtEuclid::compute(self.modulus, arg.representative);
        assert!(gcd_res.gcd == 1);
        let rep = gcd_res.coeff_right.rem_euclid(self.modulus as i64) as u64;
        Mod64 {
            representative: rep,
        }
    }

    fn div(&self, lhs: Mod64, rhs: Mod64) -> Mod64 {
        self.mul(lhs, self.inv(rhs))
    }
}

/// Result of the Extended Euclidean algorithm,
/// which computes the gcd and coefficients satisfying:
///
/// left * coeff_left + right * coeff_right = gcd.
#[derive(Clone, Copy)]
pub struct ExtEuclid {
    pub gcd: u64,
    pub coeff_left: i64,
    pub coeff_right: i64,
}

impl ExtEuclid {
    /// Computes Extended Euclidean Algorithm
    pub fn compute(left: u64, right: u64) -> ExtEuclid {
        // TODO Make this give unique gcd value
        let mut pre = ExtEuclid {
            gcd: left,
            coeff_left: 1,
            coeff_right: 0,
        };
        let mut post = ExtEuclid {
            gcd: right,
            coeff_left: 0,
            coeff_right: 1,
        };
        // Terminates when post becomes zero
        while post.gcd != 0 {
            let (q, r) = pre.gcd.div_rem_euclid(&post.gcd);
            (pre, post) = (
                post,
                ExtEuclid {
                    gcd: r,
                    coeff_left: pre.coeff_left - (q as i64) * post.coeff_left,
                    coeff_right: pre.coeff_right - (q as i64) * post.coeff_right,
                },
            )
        }
        // When post is zero, pre contains the gcd information
        pre
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use proptest::prelude::*;

    use crate::{engine::handle::tests::*, utils::math::primes::tests::prime_strategy};

    const MAX_EXACT_MODULUS: u64 = (1u64 << 63) - 1;

    pub fn exact_modulus() -> impl Strategy<Value = ModExact> {
        (2u64..=MAX_EXACT_MODULUS).prop_map(|modulus| ModExact { modulus })
    }

    pub fn prime_modulus() -> impl Strategy<Value = ModExact> {
        prop_oneof![
            prop_oneof![Just(2u64), Just(3), Just(5), Just(7), Just(11),],
            prime_strategy(12, MAX_EXACT_MODULUS)
        ]
        .prop_map(|modulus| ModExact { modulus })
    }

    pub fn modular_value(modulus: u64) -> impl Strategy<Value = Mod64> {
        (0u64..modulus).prop_map(|representative| Mod64 { representative })
    }

    // Additive group tests

    #[test]
    fn add_assoc() {
        ops_add_assoc(exact_modulus(), |p| modular_value(p.modulus));
    }

    #[test]
    fn add_comm() {
        ops_add_comm(exact_modulus(), |p| modular_value(p.modulus));
    }

    #[test]
    fn add_identity() {
        ops_add_identity(exact_modulus(), |p| modular_value(p.modulus));
    }

    #[test]
    fn add_inverse() {
        ops_add_inverse(exact_modulus(), |p| modular_value(p.modulus));
    }

    #[test]
    fn sub_compat() {
        ops_sub_compat(exact_modulus(), |p| modular_value(p.modulus));
    }

    // Ring tests

    #[test]
    fn mul_assoc() {
        ops_mul_assoc(exact_modulus(), |p| modular_value(p.modulus));
    }

    #[test]
    fn mul_comm() {
        ops_mul_comm(exact_modulus(), |p| modular_value(p.modulus));
    }

    #[test]
    fn mul_identity() {
        ops_mul_identity(exact_modulus(), |p| modular_value(p.modulus));
    }

    // Field tests

    /// Multiplicative inverse of the prime field.
    #[test]
    fn mul_inverse() {
        ops_mul_inverse(prime_modulus(), |p| modular_value(p.modulus));
    }

    #[test]
    fn div_compat() {
        ops_div_compat(prime_modulus(), |p| modular_value(p.modulus));
    }
}

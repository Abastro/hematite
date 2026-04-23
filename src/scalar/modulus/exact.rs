use num_traits::Euclid;

use crate::{
    engine::handle::{AddGroupOps, FieldOps, RingOps},
    scalar::modulus::Mod64,
};

/// Denotes modular arithmetic with exact reduction.
///
/// Can overflow for modulus over 32 bit.
#[derive(Debug, PartialEq, Eq)]
pub struct ModExact {
    pub modulus: u64,
}

impl ModExact {
    pub fn into(&self, val: u64) -> Mod64 {
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
        return Mod64 {
            representative: (lhs.representative * rhs.representative) % self.modulus,
        };
    }
}

impl FieldOps<Mod64> for ModExact {
    fn inv(&self, arg: Mod64) -> Mod64 {
        let gcd_res = ExtEuclid::compute(self.modulus, arg.representative);
        assert!(gcd_res.gcd == 1);
        Mod64 {
            representative: gcd_res.coeff_right,
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
    pub coeff_left: u64,
    pub coeff_right: u64,
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
            // Step post
            let (q, r) = pre.gcd.div_rem_euclid(&post.gcd);
            post.gcd = r;
            post.coeff_left = pre.coeff_left - q * post.coeff_left;
            post.coeff_right = post.coeff_right - q * post.coeff_right;
            // Step pre
            pre = post;
        }
        // When post is zero, pre contains the gcd information
        pre
    }
}

use num_traits::Euclid;

use crate::{
    engine::handle::{AddGroupHandle, FieldHandle, RingHandle},
    scalar::modulus::Mod64,
};

/// Denotes modular arithmetic with straightforward reduction.
/// The representative is chosen as the unique remainder.
///
/// Can overflow for modulus over 32 bit.
#[derive(Debug, PartialEq, Eq)]
pub struct ModulusSimple {
    pub modulus: u64,
}

impl ModulusSimple {
    pub fn into(&self, val: u64) -> Mod64 {
        Mod64 {
            representative: val % self.modulus,
        }
    }

    pub fn h_pow(&self, base: Mod64, exp: usize) -> Mod64 {
        let mut out = Mod64 { representative: 1 };
        let mut cur_base = base;
        let mut e = exp;

        while e != 0 {
            if e & 1 == 1 {
                self.h_mul_assign(&mut out, &cur_base);
            }

            let cur_base_ = cur_base;
            self.h_mul_assign(&mut cur_base, &cur_base_);
            e >>= 1;
        }

        out
    }

    pub fn invert(&self, value: Mod64) -> Mod64 {
        let gcd_res = ExtEuclid::compute(self.modulus, value.representative);
        assert!(gcd_res.gcd == 1);
        Mod64 {
            representative: gcd_res.coeff_right,
        }
    }
}

impl AddGroupHandle<Mod64> for ModulusSimple {
    fn h_set_zero(&self, val: &mut Mod64) {
        val.representative = 0
    }

    fn h_is_zero(&self, val: &Mod64) -> bool {
        val.representative == 0
    }

    fn h_add(&self, lhs: &Mod64, rhs: &Mod64, out: &mut Mod64) {
        out.representative = (lhs.representative + rhs.representative) % self.modulus
    }

    fn h_sub(&self, lhs: &Mod64, rhs: &Mod64, out: &mut Mod64) {
        out.representative = (lhs.representative + self.modulus - rhs.representative) % self.modulus
    }

    fn h_add_assign(&self, lhs: &mut Mod64, rhs: &Mod64) {
        lhs.representative = (lhs.representative + rhs.representative) % self.modulus
    }

    fn h_sub_assign(&self, lhs: &mut Mod64, rhs: &Mod64) {
        lhs.representative = (lhs.representative + self.modulus - rhs.representative) % self.modulus
    }
}

impl RingHandle<Mod64> for ModulusSimple {
    fn h_set_one(&self, val: &mut Mod64) {
        val.representative = 1
    }

    fn h_mul(&self, lhs: &Mod64, rhs: &Mod64, out: &mut Mod64) {
        out.representative = (lhs.representative * rhs.representative) % self.modulus
    }

    fn h_mul_assign(&self, lhs: &mut Mod64, rhs: &Mod64) {
        lhs.representative = (lhs.representative * rhs.representative) % self.modulus
    }
}

impl FieldHandle<Mod64> for ModulusSimple {
    fn h_div(&self, lhs: &Mod64, rhs: &Mod64, out: &mut Mod64) {
        self.h_mul(lhs, &self.invert(*rhs), out);
    }

    fn h_div_assign(&self, lhs: &mut Mod64, rhs: &Mod64) {
        self.h_mul_assign(lhs, &self.invert(*rhs));
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

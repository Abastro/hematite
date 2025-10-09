use num_traits::{Euclid, one, zero};

use crate::engine::{handle::{AddGroupHandle, FieldHandle, RingHandle}, native::NativeRing};

use super::modulus::Modulus;

/**
 * Denotes simple modulus on a euclidean ring.
 * The representative is chosen as the unique remainder.
 * */
#[derive(Debug, PartialEq, Eq)]
pub struct SimpleModulus<R> {
    modulus: R,
}

impl<R: Euclid> Modulus<R> for SimpleModulus<R> {
    fn representative(&self, value: R) -> R {
        value.rem_euclid(&self.modulus)
    }
}

/**
 * Result of the Extended Euclidean algorithm,
 * which computes the gcd and coefficients satisfying
 * left * coeff_left + right * coeff_right = gcd.
 */
#[derive(Clone, Copy)]
pub struct ExtEuclid<R> {
    pub gcd: R,
    pub coeff_left: R,
    pub coeff_right: R,
}

impl<R: NativeRing + Euclid> ExtEuclid<R> {
    /**
     * Computes Extended Euclidean Algorithm.
     */
    pub fn compute(left: R, right: R) -> ExtEuclid<R> {
        // TODO Make this give unique gcd value
        let mut pre = ExtEuclid {
            gcd: left,
            coeff_left: one(),
            coeff_right: zero(),
        };
        let mut post = ExtEuclid {
            gcd: right,
            coeff_left: zero(),
            coeff_right: one(),
        };
        // Terminates when post becomes zero
        while !post.gcd.is_zero() {
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

impl<R: NativeRing + Euclid> SimpleModulus<R> {
    /**
     * Inverts a modular value - this requires the extended Euclidean algorithm.
     * */
    pub fn invert(&self, value: R) -> R {
        let modulus = self.modulus;
        let gcd_res = ExtEuclid::compute(modulus, value);

        // Requires gcd to be 1
        assert!(gcd_res.gcd.is_one());

        // Has modulus * coeff_left + value * coeff_right = 1
        gcd_res.coeff_right
    }
}

impl<R: NativeRing + Euclid> AddGroupHandle<R> for SimpleModulus<R> {
    fn h_add(&self, lhs: &R, rhs: &R, out: &mut R) {
        *out = self.representative(*lhs + *rhs);
    }

    fn h_sub(&self, lhs: &R, rhs: &R, out: &mut R) {
        *out = self.representative(*lhs - *rhs);
    }

    fn h_add_assign(&self, lhs: &mut R, rhs: &R) {
        *lhs = self.representative(*lhs + *rhs);
    }

    fn h_sub_assign(&self, lhs: &mut R, rhs: &R) {
        *lhs = self.representative(*lhs - *rhs);
    }

    fn h_set_zero(&self, val: &mut R) {
        *val = zero();
    }

    fn h_is_zero(&self, val: &R) -> bool {
        self.representative(*val).is_zero()
    }
}

impl<R: NativeRing + Euclid> RingHandle<R> for SimpleModulus<R> {
    fn h_mul(&self, lhs: &R, rhs: &R, out: &mut R) {
        *out = self.representative(*lhs * *rhs);
    }

    fn h_mul_assign(&self, lhs: &mut R, rhs: &R) {
        *lhs = self.representative(*lhs * *rhs);
    }

    fn h_set_one(&self, val: &mut R) {
        *val = one();
    }
}

impl<R: NativeRing + Euclid> FieldHandle<R> for SimpleModulus<R> {
    fn h_div(&self, lhs: &R, rhs: &R, out: &mut R) {
        *out = self.representative(*lhs * self.invert(*rhs))
    }

    fn h_div_assign(&self, lhs: &mut R, rhs: &R) {
        *lhs = self.representative(*lhs * self.invert(*rhs))
    }
}

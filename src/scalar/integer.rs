use num::{BigInt, One, Zero};

use crate::engine::handle::{AddGroupHandle, RingHandle};

pub type Int = BigInt;

/// Handle for integers.
/// 
/// Since integers are not light, we pass it by reference.
pub struct IntHandle {}

impl AddGroupHandle<Int> for IntHandle {
    fn h_set_zero(&self, val: &mut Int) {
        val.set_zero();
    }

    fn h_is_zero(&self, val: &Int) -> bool {
        val.is_zero()
    }

    fn h_add(&self, lhs: &Int, rhs: &Int, out: &mut Int) {
        *out = lhs + rhs
    }

    fn h_add_assign(&self, lhs: &mut Int, rhs: &Int) {
        *lhs += rhs
    }

    fn h_neg(&self, arg: &Int, out: &mut Int) {
        *out = -arg
    }

    fn h_neg_assign(&self, arg: &mut Int) {
        *arg = -std::mem::take(arg)
    }

    fn h_sub(&self, lhs: &Int, rhs: &Int, out: &mut Int) {
        *out = lhs - rhs
    }

    fn h_sub_assign(&self, lhs: &mut Int, rhs: &Int) {
        *lhs -= rhs
    }
}

impl RingHandle<Int> for IntHandle {
    fn h_set_one(&self, val: &mut Int) {
        val.set_one();
    }

    fn h_mul(&self, lhs: &Int, rhs: &Int, out: &mut Int) {
        *out = lhs * rhs
    }

    fn h_mul_assign(&self, lhs: &mut Int, rhs: &Int) {
        *lhs *= rhs
    }
}

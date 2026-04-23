use num::{Complex, One, Zero};

use crate::engine::handle::{AddGroupHandle, FieldHandle, RingHandle};

pub type Cx128 = Complex<f64>;

pub struct ComplexHandle {}

impl AddGroupHandle<Cx128> for ComplexHandle {
    fn h_set_zero(&self, val: &mut Cx128) {
        val.set_zero();
    }

    fn h_is_zero(&self, val: &Cx128) -> bool {
        val.is_zero()
    }

    fn h_add(&self, lhs: &Cx128, rhs: &Cx128, out: &mut Cx128) {
        *out = *lhs + *rhs
    }

    fn h_sub(&self, lhs: &Cx128, rhs: &Cx128, out: &mut Cx128) {
        *out = *lhs - *rhs
    }

    fn h_add_assign(&self, lhs: &mut Cx128, rhs: &Cx128) {
        *lhs += *rhs
    }

    fn h_sub_assign(&self, lhs: &mut Cx128, rhs: &Cx128) {
        *lhs -= *rhs
    }
}

impl RingHandle<Cx128> for ComplexHandle {
    fn h_set_one(&self, val: &mut Cx128) {
        val.set_one();
    }

    fn h_mul(&self, lhs: &Cx128, rhs: &Cx128, out: &mut Cx128) {
        *out = *lhs * *rhs
    }

    fn h_mul_assign(&self, lhs: &mut Cx128, rhs: &Cx128) {
        *lhs *= *rhs
    }
}

impl FieldHandle<Cx128> for ComplexHandle {
    fn h_div(&self, lhs: &Cx128, rhs: &Cx128, out: &mut Cx128) {
        *out = *lhs / *rhs
    }

    fn h_div_assign(&self, lhs: &mut Cx128, rhs: &Cx128) {
        *lhs /= *rhs
    }
}

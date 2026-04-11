use crate::engine::handle::{AddGroupHandle, FieldHandle, RingHandle};

/// Assume f64 is enough precision
pub struct RealHandle {}

impl AddGroupHandle<f64> for RealHandle {
    fn h_set_zero(&self, val: &mut f64) {
        *val = 0.0;
    }

    fn h_is_zero(&self, val: &f64) -> bool {
        *val == 0.0
    }

    fn h_add(&self, lhs: &f64, rhs: &f64, out: &mut f64) {
        *out = *lhs + *rhs
    }

    fn h_sub(&self, lhs: &f64, rhs: &f64, out: &mut f64) {
        *out = *lhs - *rhs
    }

    fn h_add_assign(&self, lhs: &mut f64, rhs: &f64) {
        *lhs += *rhs
    }

    fn h_sub_assign(&self, lhs: &mut f64, rhs: &f64) {
        *lhs -= *rhs
    }
}

impl RingHandle<f64> for RealHandle {
    fn h_set_one(&self, val: &mut f64) {
        *val = 1.0
    }

    fn h_mul(&self, lhs: &f64, rhs: &f64, out: &mut f64) {
        *out = *lhs * *rhs
    }

    fn h_mul_assign(&self, lhs: &mut f64, rhs: &f64) {
        *lhs *= *rhs
    }
}

impl FieldHandle<f64> for RealHandle {
    fn h_div(&self, lhs: &f64, rhs: &f64, out: &mut f64) {
        *out = *lhs / *rhs
    }

    fn h_div_assign(&self, lhs: &mut f64, rhs: &f64) {
        *lhs /= *rhs
    }
}

// TODO Complex numbers
pub struct ComplexHandle {}

use num::Complex;

use crate::engine::handle::{AddGroupOps, FieldOps, RingOps};

pub type Cx128 = Complex<f64>;

pub struct ComplexHandle {}

impl AddGroupOps<Cx128> for ComplexHandle {
    fn zero(&self) -> Cx128 {
        Cx128::new(0.0, 0.0)
    }

    fn add(&self, lhs: Cx128, rhs: Cx128) -> Cx128 {
        lhs + rhs
    }

    fn neg(&self, arg: Cx128) -> Cx128 {
        -arg
    }

    fn sub(&self, lhs: Cx128, rhs: Cx128) -> Cx128 {
        lhs - rhs
    }
}

impl RingOps<Cx128> for ComplexHandle {
    fn one(&self) -> Cx128 {
        Cx128::new(1.0, 0.0)
    }

    fn mul(&self, lhs: Cx128, rhs: Cx128) -> Cx128 {
        lhs * rhs
    }
}

impl FieldOps<Cx128> for ComplexHandle {
    fn inv(&self, arg: Cx128) -> Cx128 {
        1.0 / arg
    }

    fn div(&self, lhs: Cx128, rhs: Cx128) -> Cx128 {
        lhs / rhs
    }
}

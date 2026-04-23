use crate::engine::handle::{AddGroupOps, FieldOps, RingOps};

pub struct RealHandle {}

impl AddGroupOps<f64> for RealHandle {
    fn zero(&self) -> f64 {
        0.0
    }

    fn add(&self, lhs: f64, rhs: f64) -> f64 {
        lhs + rhs
    }

    fn neg(&self, arg: f64) -> f64 {
        -arg
    }

    fn sub(&self, lhs: f64, rhs: f64) -> f64 {
        lhs - rhs
    }
}

/// Assume f64 is enough precision
impl RingOps<f64> for RealHandle {
    fn one(&self) -> f64 {
        1.0
    }

    fn mul(&self, lhs: f64, rhs: f64) -> f64 {
        lhs * rhs
    }
}

impl FieldOps<f64> for RealHandle {
    fn inv(&self, arg: f64) -> f64 {
        1.0 / arg
    }

    fn div(&self, lhs: f64, rhs: f64) -> f64 {
        lhs / rhs
    }
}

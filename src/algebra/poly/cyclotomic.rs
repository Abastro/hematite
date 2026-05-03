use crate::{
    tensor::tensor::TensorShape,
};

/// Common trait for cyclotomic ring shapes.
///
/// A cyclotomic ring is a ring isomorphic to `Z[X]/(\Phi_M(X))` for some integer M,
/// which is the ring of integers of `Q(\zeta_M)` where `\zeta_M` is a `M`-th root of unity.
///
/// The same shape can be used to represent the rings with different base ring,
/// such as `R[X]/(\Phi_M(X))` or `Z_Q[X]/(\Phi_M(X))`.
pub trait Cyclo: TensorShape {
    /// The degree of the cyclotomic polynomial.
    fn degree(&self) -> usize;

    /// The order where X^order = 1.
    fn order(&self) -> usize;
}

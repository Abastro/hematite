use crate::tensor::tensor::TensorShape;

/// Denotes a cyclotomic ring and the corresponding tensor shape.
///
/// A cyclotomic ring is a ring isomorphic to `Z[X]/(\Phi_M(X))` for some integer M,
/// which is the ring of integers of `Q(\zeta_M)` where `\zeta_M` is a `M`-th root of unity.
///
/// The same shape can be used to represent the rings with different base ring,
/// such as `R[X]/(\Phi_M(X))` or `Z_Q[X]/(\Phi_M(X))`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cyclo {
    /// The degree of the cyclotomic polynomial.
    degree: usize,
    /// The order where X^order = 1.
    order: usize,
}

impl TensorShape for Cyclo {
    type Coord = usize;

    fn coord_index(&self, coord: Self::Coord) -> usize {
        coord
    }

    fn total_size(&self) -> usize {
        self.degree
    }
}

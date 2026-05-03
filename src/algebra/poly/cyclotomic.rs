use crate::{
    algebra::rns::rns::{RNSModulus, RNSShape},
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

/// Denotes cyclotomic ring over RNS modulus.
///
/// Coord order is level * degree.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RNSCyclo<Var> {
    pub modulus: RNSModulus,
    pub variable: Var,
}

impl<Var> TensorShape for RNSCyclo<Var>
where
    Var: Cyclo,
{
    type Coord = (usize, Var::Coord);

    fn coord_index(&self, coord: Self::Coord) -> usize {
        let (level, deg) = coord;
        level * self.variable.total_size() + self.variable.coord_index(deg)
    }

    fn total_size(&self) -> usize {
        self.modulus.total_size() * self.variable.total_size()
    }
}

impl<Var> RNSShape for RNSCyclo<Var>
where
    Var: Cyclo,
{
    fn rns_modulus(&self) -> RNSModulus {
        self.modulus
    }

    fn len_per_prime(&self) -> usize {
        self.variable.degree()
    }
}

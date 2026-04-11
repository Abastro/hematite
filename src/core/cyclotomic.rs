use crate::{
    core::rns::{RNSModulus, RNSShape},
    tensor::tensor::TensorShape,
};

/// Shape for cyclotomic rings.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cyclo {
    pub degree: usize,
    /// The order where X^order = 1.
    pub order: usize,
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

/// Denotes cyclotomic ring over RNS modulus.
///
/// Coord order is level * degree.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RNSCyclo {
    pub modulus: RNSModulus,
    pub variable: Cyclo,
}

impl TensorShape for RNSCyclo {
    type Coord = (usize, usize);

    fn coord_index(&self, coord: Self::Coord) -> usize {
        let (level, deg) = coord;
        level * self.variable.total_size() + deg
    }

    fn total_size(&self) -> usize {
        self.modulus.total_size() * self.variable.total_size()
    }
}

impl RNSShape for RNSCyclo {
    fn rns_modulus(&self) -> RNSModulus {
        self.modulus
    }

    fn len_per_prime(&self) -> usize {
        self.variable.degree
    }
}

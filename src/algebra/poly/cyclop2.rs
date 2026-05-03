use crate::{
    algebra::poly::cyclotomic::Cyclo, tensor::tensor::TensorShape, utils::math::powers::is_pow2,
};

/// Denotes the standard power-of-two cyclotomic ring, `Z[X]/(X^N + 1)`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CycloP2 {
    degree: usize,
}

impl CycloP2 {
    pub fn new(degree: usize) -> Self {
        assert!(is_pow2(degree as u64));
        CycloP2 { degree }
    }
}

impl TensorShape for CycloP2 {
    type Coord = usize;

    fn coord_index(&self, coord: Self::Coord) -> usize {
        coord
    }

    fn total_size(&self) -> usize {
        self.degree
    }
}

impl Cyclo for CycloP2 {
    fn degree(&self) -> usize {
        self.degree
    }

    fn order(&self) -> usize {
        2 * self.degree
    }
}

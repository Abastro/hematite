use crate::{poly::cyclotomic::Cyclo, tensor::tensor::TensorShape};

/// Denotes the standard power-of-two cyclotomic ring, `Z[X]/(X^N + 1)`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CycloP2 {
    pub degree: usize,
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

use crate::tensor::tensor::TensorShape;

/// Shape for cyclotomic rings.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Cyclo {
    pub degree: usize,
    /// The power where X^root_power = 1.
    pub root_power: usize,
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

use crate::tensor::tensor::TensorShape;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vector {
    length: usize,
}

impl TensorShape for Vector {
    type Coord = usize;

    fn coord_index(&self, coord: Self::Coord) -> usize {
        coord
    }

    fn total_size(&self) -> usize {
        self.length
    }
}

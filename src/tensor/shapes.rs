use crate::tensor::tensor::TensorShape;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vector {
    pub length: usize,
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

/// Denotes a shape for product of same objects.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Product {
    pub length: usize
}

impl TensorShape for Product {
    type Coord = usize;

    fn coord_index(&self, coord: Self::Coord) -> usize {
        coord
    }

    fn total_size(&self) -> usize {
        self.length
    }
    
}
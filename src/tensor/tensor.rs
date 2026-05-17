use std::ops::{Index, IndexMut};

/// A lightweight representation of the shape of a tensor.
pub trait TensorShape: Copy {
    type Coord: Copy;

    fn coord_index(&self, coord: Self::Coord) -> usize;
    fn total_size(&self) -> usize;
}

/// A tensor with a fixed shape.
///
/// The data should have length total_size specified by the shape.
#[derive(Clone)]
pub struct Tensor<Shape, T> {
    shape: Shape,
    data: Vec<T>,
}

impl<Shape: TensorShape, T> Tensor<Shape, T> {
    pub fn shape(&self) -> Shape {
        self.shape
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Internal data of the tensor.
    pub fn data_mut(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    pub fn new<F>(shape: Shape, init: F) -> Self
    where
        F: FnMut() -> T,
    {
        let mut data = Vec::with_capacity(shape.total_size());
        data.resize_with(shape.total_size(), init);
        Tensor { shape, data }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }
}

impl<Shape: TensorShape, T> IntoIterator for Tensor<Shape, T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<Shape: TensorShape, T> Index<Shape::Coord> for Tensor<Shape, T> {
    type Output = T;

    fn index(&self, index: Shape::Coord) -> &Self::Output {
        &self.data[self.shape.coord_index(index)]
    }
}

impl<Shape: TensorShape, T> IndexMut<Shape::Coord> for Tensor<Shape, T> {
    fn index_mut(&mut self, index: Shape::Coord) -> &mut Self::Output {
        &mut self.data[self.shape.coord_index(index)]
    }
}

/// A shaped handle, which is used to handle a slice with a shape.
/// 
/// Since slice [T] cannot be constructed in rust, we cannot have (Shape, [T]) for non-owned variant of tensor,
/// so we embed the shape into the handle instead.
pub struct Shaped<Shape, H> {
    pub shape: Shape,
    pub handle: H,
}

use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use crate::engine::handle::{AddGroupHandle, RingHandle};

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

/// Handle for tensors with pointwise operations.
pub struct PointwiseHandle<H> {
    handle_scalar: H,
}

impl<Shape, T, H> AddGroupHandle<Tensor<Shape, T>> for PointwiseHandle<&H>
where
    Shape: TensorShape + Eq + Debug,
    H: AddGroupHandle<T>,
{
    fn h_set_zero(&self, val: &mut Tensor<Shape, T>) {
        for entry in val.data.iter_mut() {
            self.handle_scalar.h_set_zero(entry);
        }
    }

    fn h_is_zero(&self, val: &Tensor<Shape, T>) -> bool {
        val.data
            .iter()
            .all(|entry| self.handle_scalar.h_is_zero(entry))
    }

    fn h_add(&self, lhs: &Tensor<Shape, T>, rhs: &Tensor<Shape, T>, out: &mut Tensor<Shape, T>) {
        debug_assert_eq!(lhs.shape, out.shape);
        debug_assert_eq!(rhs.shape, out.shape);

        for idx in 0..out.shape.total_size() {
            self.handle_scalar
                .h_add(&lhs.data[idx], &rhs.data[idx], &mut out.data[idx]);
        }
    }

    fn h_sub(&self, lhs: &Tensor<Shape, T>, rhs: &Tensor<Shape, T>, out: &mut Tensor<Shape, T>) {
        debug_assert_eq!(lhs.shape, out.shape);
        debug_assert_eq!(rhs.shape, out.shape);

        for idx in 0..out.shape.total_size() {
            self.handle_scalar
                .h_sub(&lhs.data[idx], &rhs.data[idx], &mut out.data[idx]);
        }
    }

    fn h_add_assign(&self, lhs: &mut Tensor<Shape, T>, rhs: &Tensor<Shape, T>) {
        debug_assert_eq!(lhs.shape, rhs.shape);

        for idx in 0..rhs.shape.total_size() {
            self.handle_scalar
                .h_add_assign(&mut lhs.data[idx], &rhs.data[idx]);
        }
    }

    fn h_sub_assign(&self, lhs: &mut Tensor<Shape, T>, rhs: &Tensor<Shape, T>) {
        debug_assert_eq!(lhs.shape, rhs.shape);

        for idx in 0..rhs.shape.total_size() {
            self.handle_scalar
                .h_sub_assign(&mut lhs.data[idx], &rhs.data[idx]);
        }
    }
}

impl<Shape, T, H> RingHandle<Tensor<Shape, T>> for PointwiseHandle<&H>
where
    Shape: TensorShape + Eq + Debug,
    H: RingHandle<T>,
{
    fn h_set_one(&self, val: &mut Tensor<Shape, T>) {
        for entry in val.data.iter_mut() {
            self.handle_scalar.h_set_one(entry);
        }
    }

    fn h_mul(&self, lhs: &Tensor<Shape, T>, rhs: &Tensor<Shape, T>, out: &mut Tensor<Shape, T>) {
        debug_assert_eq!(lhs.shape, out.shape);
        debug_assert_eq!(rhs.shape, out.shape);

        for idx in 0..out.shape.total_size() {
            self.handle_scalar
                .h_mul(&lhs.data[idx], &rhs.data[idx], &mut out.data[idx]);
        }
    }

    fn h_mul_assign(&self, lhs: &mut Tensor<Shape, T>, rhs: &Tensor<Shape, T>) {
        debug_assert_eq!(lhs.shape, rhs.shape);

        for idx in 0..rhs.shape.total_size() {
            self.handle_scalar
                .h_mul_assign(&mut lhs.data[idx], &rhs.data[idx]);
        }
    }
}

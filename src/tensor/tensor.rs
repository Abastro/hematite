use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

use crate::engine::handle::{AddGroupHandle, RingHandle};

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

    /// Internal data of the tensor.
    pub fn data(&mut self) -> &mut Vec<T> {
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

impl<Shape: TensorShape + Eq + Debug, T, H: AddGroupHandle<T>> AddGroupHandle<Tensor<Shape, T>>
    for PointwiseHandle<&H>
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

impl<Shape: TensorShape + Eq + Debug, T, H: RingHandle<T>> RingHandle<Tensor<Shape, T>>
    for PointwiseHandle<&H>
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

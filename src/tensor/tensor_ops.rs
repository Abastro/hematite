use crate::engine::handle::AddGroupHandle;
use crate::engine::handle::RingHandle;
use crate::tensor::tensor::Tensor;
use crate::tensor::tensor::TensorShape;
use std::fmt::Debug;

/// Handle for tensors with pointwise operations.
///
/// Works for both tensors and slices.
pub struct PointwiseHandle<H> {
    handle_scalar: H,
}

impl<H> PointwiseHandle<H> {
    pub fn new(handle_scalar: H) -> Self {
        PointwiseHandle { handle_scalar }
    }
}

impl<Shape, T, H> AddGroupHandle<Tensor<Shape, T>> for PointwiseHandle<&H>
where
    Shape: TensorShape + Eq + Debug,
    H: AddGroupHandle<T>,
{
    fn h_set_zero(&self, val: &mut Tensor<Shape, T>) {
        for entry in val.iter_mut() {
            self.handle_scalar.h_set_zero(entry);
        }
    }

    fn h_is_zero(&self, val: &Tensor<Shape, T>) -> bool {
        val.iter().all(|entry| self.handle_scalar.h_is_zero(entry))
    }

    fn h_add(&self, lhs: &Tensor<Shape, T>, rhs: &Tensor<Shape, T>, out: &mut Tensor<Shape, T>) {
        let shape = crate::the_debug!(lhs.shape(), rhs.shape(), out.shape());

        for idx in 0..shape.total_size() {
            self.handle_scalar
                .h_add(&lhs.data()[idx], &rhs.data()[idx], &mut out.data_mut()[idx]);
        }
    }

    fn h_sub(&self, lhs: &Tensor<Shape, T>, rhs: &Tensor<Shape, T>, out: &mut Tensor<Shape, T>) {
        let shape = crate::the_debug!(lhs.shape(), rhs.shape(), out.shape());

        for idx in 0..shape.total_size() {
            self.handle_scalar
                .h_sub(&lhs.data()[idx], &rhs.data()[idx], &mut out.data_mut()[idx]);
        }
    }

    fn h_add_assign(&self, lhs: &mut Tensor<Shape, T>, rhs: &Tensor<Shape, T>) {
        let shape = crate::the_debug!(lhs.shape(), rhs.shape());

        for idx in 0..shape.total_size() {
            self.handle_scalar
                .h_add_assign(&mut lhs.data_mut()[idx], &rhs.data()[idx]);
        }
    }

    fn h_sub_assign(&self, lhs: &mut Tensor<Shape, T>, rhs: &Tensor<Shape, T>) {
        let shape = crate::the_debug!(lhs.shape(), rhs.shape());

        for idx in 0..shape.total_size() {
            self.handle_scalar
                .h_sub_assign(&mut lhs.data_mut()[idx], &rhs.data()[idx]);
        }
    }

    fn h_neg(&self, arg: &Tensor<Shape, T>, out: &mut Tensor<Shape, T>) {
        let shape = crate::the_debug!(arg.shape(), out.shape());

        for idx in 0..shape.total_size() {
            self.handle_scalar
                .h_neg(&arg.data()[idx], &mut out.data_mut()[idx]);
        }
    }

    fn h_neg_assign(&self, arg: &mut Tensor<Shape, T>) {
        for idx in 0..arg.shape().total_size() {
            self.handle_scalar.h_neg_assign(&mut arg.data_mut()[idx]);
        }
    }
}

impl<Shape, T, H> RingHandle<Tensor<Shape, T>> for PointwiseHandle<&H>
where
    Shape: TensorShape + Eq + Debug,
    H: RingHandle<T>,
{
    fn h_set_one(&self, val: &mut Tensor<Shape, T>) {
        for entry in val.iter_mut() {
            self.handle_scalar.h_set_one(entry);
        }
    }

    fn h_mul(&self, lhs: &Tensor<Shape, T>, rhs: &Tensor<Shape, T>, out: &mut Tensor<Shape, T>) {
        let shape = crate::the_debug!(lhs.shape(), rhs.shape(), out.shape());

        for idx in 0..shape.total_size() {
            self.handle_scalar
                .h_mul(&lhs.data()[idx], &rhs.data()[idx], &mut out.data_mut()[idx]);
        }
    }

    fn h_mul_assign(&self, lhs: &mut Tensor<Shape, T>, rhs: &Tensor<Shape, T>) {
        let shape = crate::the_debug!(lhs.shape(), rhs.shape());

        for idx in 0..shape.total_size() {
            self.handle_scalar
                .h_mul_assign(&mut lhs.data_mut()[idx], &rhs.data()[idx]);
        }
    }
}

impl<T, H> AddGroupHandle<[T]> for PointwiseHandle<&H>
where
    H: AddGroupHandle<T>,
{
    fn h_set_zero(&self, val: &mut [T]) {
        for entry in val {
            self.handle_scalar.h_set_zero(entry);
        }
    }

    fn h_is_zero(&self, val: &[T]) -> bool {
        val.iter().all(|entry| self.handle_scalar.h_is_zero(entry))
    }

    fn h_add(&self, lhs: &[T], rhs: &[T], out: &mut [T]) {
        let size = crate::the_debug!(lhs.len(), rhs.len(), out.len());

        for idx in 0..size {
            self.handle_scalar
                .h_add(&lhs[idx], &rhs[idx], &mut out[idx]);
        }
    }

    fn h_add_assign(&self, lhs: &mut [T], rhs: &[T]) {
        let size = crate::the_debug!(lhs.len(), rhs.len());

        for idx in 0..size {
            self.handle_scalar.h_add_assign(&mut lhs[idx], &rhs[idx]);
        }
    }

    fn h_sub(&self, lhs: &[T], rhs: &[T], out: &mut [T]) {
        let size = crate::the_debug!(lhs.len(), rhs.len(), out.len());

        for idx in 0..size {
            self.handle_scalar
                .h_sub(&lhs[idx], &rhs[idx], &mut out[idx]);
        }
    }

    fn h_sub_assign(&self, lhs: &mut [T], rhs: &[T]) {
        let size = crate::the_debug!(lhs.len(), rhs.len());

        for idx in 0..size {
            self.handle_scalar.h_sub_assign(&mut lhs[idx], &rhs[idx]);
        }
    }

    fn h_neg(&self, arg: &[T], out: &mut [T]) {
        let size = crate::the_debug!(arg.len(), out.len());

        for idx in 0..size {
            self.handle_scalar.h_neg(&arg[idx], &mut out[idx]);
        }
    }

    fn h_neg_assign(&self, arg: &mut [T]) {
        for entry in arg {
            self.handle_scalar.h_neg_assign(entry);
        }
    }
}

impl<T, H> RingHandle<[T]> for PointwiseHandle<&H>
where
    H: RingHandle<T>,
{
    fn h_set_one(&self, val: &mut [T]) {
        for entry in val {
            self.handle_scalar.h_set_one(entry);
        }
    }

    fn h_mul(&self, lhs: &[T], rhs: &[T], out: &mut [T]) {
        let size = crate::the_debug!(lhs.len(), rhs.len(), out.len());

        for idx in 0..size {
            self.handle_scalar
                .h_mul(&lhs[idx], &rhs[idx], &mut out[idx]);
        }
    }

    fn h_mul_assign(&self, lhs: &mut [T], rhs: &[T]) {
        let size = crate::the_debug!(lhs.len(), rhs.len());

        for idx in 0..size {
            self.handle_scalar.h_mul_assign(&mut lhs[idx], &rhs[idx]);
        }
    }
}

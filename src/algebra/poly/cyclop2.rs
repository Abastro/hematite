use crate::{
    engine::handle::{AddGroupHandle, RingHandle},
    tensor::tensor_ops::PointwiseHandle,
    utils::math::powers,
};

/// Handle for generic power-of-two cyclotomic operations.
pub struct P2Handle<H> {
    handle_scalar: H,
}

impl<H> P2Handle<H> {
    pub fn new(handle_scalar: H) -> Self {
        P2Handle { handle_scalar }
    }
}

// TODO Implement vectorization for Mod64

/// Generic additive implementation
impl<H, T> AddGroupHandle<[T]> for P2Handle<&H>
where
    H: AddGroupHandle<T>,
{
    fn h_set_zero(&self, val: &mut [T]) {
        PointwiseHandle::new(self.handle_scalar).h_set_zero(val);
    }

    fn h_is_zero(&self, val: &[T]) -> bool {
        PointwiseHandle::new(self.handle_scalar).h_is_zero(val)
    }

    fn h_add(&self, lhs: &[T], rhs: &[T], out: &mut [T]) {
        PointwiseHandle::new(self.handle_scalar).h_add(lhs, rhs, out);
    }

    fn h_add_assign(&self, lhs: &mut [T], rhs: &[T]) {
        PointwiseHandle::new(self.handle_scalar).h_add_assign(lhs, rhs);
    }

    fn h_sub(&self, lhs: &[T], rhs: &[T], out: &mut [T]) {
        PointwiseHandle::new(self.handle_scalar).h_sub(lhs, rhs, out);
    }

    fn h_sub_assign(&self, lhs: &mut [T], rhs: &[T]) {
        PointwiseHandle::new(self.handle_scalar).h_sub_assign(lhs, rhs);
    }

    fn h_neg(&self, arg: &[T], out: &mut [T]) {
        PointwiseHandle::new(self.handle_scalar).h_neg(arg, out);
    }

    fn h_neg_assign(&self, arg: &mut [T]) {
        PointwiseHandle::new(self.handle_scalar).h_neg_assign(arg);
    }
}

impl<H> P2Handle<&H> {
    /// Computes negacyclic convolution in Z[X]/(X^N + 1).
    pub fn compute_mul<T: Clone>(&self, lhs: &[T], rhs: &[T]) -> Vec<T>
    where
        H: RingHandle<T>,
    {
        let degree = crate::the_debug!(lhs.len(), rhs.len());
        debug_assert!(powers::is_pow2(degree as u64));

        let mut out: Vec<T> = lhs.to_vec();
        for entry in &mut out {
            self.handle_scalar.h_set_zero(entry);
        }

        for i in 0..degree {
            for j in 0..degree {
                let mut tmp = rhs[j].clone();
                self.handle_scalar.h_mul_assign(&mut tmp, &lhs[i]);
                let target = (i + j) % degree;
                if i + j < degree {
                    self.handle_scalar.h_add_assign(&mut out[target], &tmp);
                } else {
                    // X^N = -1: wrap folds back
                    self.handle_scalar.h_sub_assign(&mut out[target], &tmp);
                }
            }
        }

        out
    }
}

/// Less efficient generic implementation of cyclotomic product
impl<T: Clone, H> RingHandle<[T]> for P2Handle<&H>
where
    H: RingHandle<T>,
{
    fn h_set_one(&self, val: &mut [T]) {
        for entry in val {
            self.handle_scalar.h_set_one(entry);
        }
    }

    fn h_mul(&self, lhs: &[T], rhs: &[T], out: &mut [T]) {
        let result = self.compute_mul(lhs, rhs);
        out.clone_from_slice(&result[..]);
    }

    fn h_mul_assign(&self, lhs: &mut [T], rhs: &[T]) {
        let result = self.compute_mul(lhs, rhs);
        lhs.clone_from_slice(&result[..]);
    }
}

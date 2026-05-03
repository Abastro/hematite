use crate::{
    algebra::poly::cyclotomic::Cyclo,
    engine::handle::AddGroupHandle,
    tensor::{tensor::TensorShape, tensor_ops::PointwiseHandle},
    utils::math::powers::is_pow2,
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

/// Handle for generic power-of-two cyclotomic operations.
pub struct CycloP2Handle<H> {
    handle_scalar: H,
}

impl<H> CycloP2Handle<H> {
    pub fn new(handle_scalar: H) -> Self {
        CycloP2Handle { handle_scalar }
    }
}

// TODO Implement vectorization

impl<H, T> AddGroupHandle<[T]> for CycloP2Handle<&H>
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

// TODO inefficient h_mul

// impl<T, H> RingHandle<[T]> for CycloP2Handle<&H>
// where
//     H: RingHandle<T>,
// {
//     fn h_set_one(&self, val: &mut [T]) {
//         for entry in val {
//             self.handle_scalar.h_set_one(entry);
//         }
//     }

//     fn h_mul(&self, lhs: &[T], rhs: &[T], out: &mut [T]) {
//         todo!()
//     }

//     fn h_mul_assign(&self, lhs: &mut [T], rhs: &[T]) {
//         todo!()
//     }
// }

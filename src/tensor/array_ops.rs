use std::ops::Range;

use crate::engine::handle::{AddGroupHandle, RingHandle};

use super::array::NArray;

/**
 * Array handle for fixed size arrays.
 * The handle should be a reference.
 * */
pub struct FixedArrayHandle<H> {
    handle_scalar: H,
    length: usize,
}

/**
 * Additive group implementation for slices to aid with implementations.
 * */
impl<A, H: AddGroupHandle<A>> AddGroupHandle<[A]> for FixedArrayHandle<&H> {
    fn h_set_zero(&self, val: &mut [A]) {
        debug_assert_eq!(self.length, val.len());

        for idx in 0..self.length {
            self.handle_scalar.h_set_zero(&mut val[idx]);
        }
    }

    fn h_is_zero(&self, val: &[A]) -> bool {
        val.iter().all(|entry| self.handle_scalar.h_is_zero(entry))
    }

    fn h_add(&self, lhs: &[A], rhs: &[A], out: &mut [A]) {
        debug_assert_eq!(self.length, lhs.len());
        debug_assert_eq!(self.length, rhs.len());
        debug_assert_eq!(self.length, out.len());

        for idx in 0..self.length {
            self.handle_scalar
                .h_add(&lhs[idx], &rhs[idx], &mut out[idx]);
        }
    }

    fn h_sub(&self, lhs: &[A], rhs: &[A], out: &mut [A]) {
        debug_assert_eq!(self.length, lhs.len());
        debug_assert_eq!(self.length, rhs.len());
        debug_assert_eq!(self.length, out.len());

        for idx in 0..self.length {
            self.handle_scalar
                .h_sub(&lhs[idx], &rhs[idx], &mut out[idx]);
        }
    }

    fn h_add_assign(&self, lhs: &mut [A], rhs: &[A]) {
        debug_assert_eq!(self.length, lhs.len());
        debug_assert_eq!(self.length, rhs.len());

        for idx in 0..self.length {
            self.handle_scalar.h_add_assign(&mut lhs[idx], &rhs[idx]);
        }
    }

    fn h_sub_assign(&self, lhs: &mut [A], rhs: &[A]) {
        debug_assert_eq!(self.length, lhs.len());
        debug_assert_eq!(self.length, rhs.len());

        for idx in 0..self.length {
            self.handle_scalar.h_sub_assign(&mut lhs[idx], &rhs[idx]);
        }
    }
}

/**
 * Ring implementation for slices.
 * */
impl<R, H: RingHandle<R>> RingHandle<[R]> for FixedArrayHandle<&H> {
    fn h_set_one(&self, val: &mut [R]) {
        debug_assert_eq!(self.length, val.len());

        for idx in 0..self.length {
            self.handle_scalar.h_set_one(&mut val[idx]);
        }
    }

    fn h_mul(&self, lhs: &[R], rhs: &[R], out: &mut [R]) {
        debug_assert_eq!(self.length, lhs.len());
        debug_assert_eq!(self.length, rhs.len());
        debug_assert_eq!(self.length, out.len());

        for idx in 0..self.length {
            self.handle_scalar
                .h_mul(&lhs[idx], &rhs[idx], &mut out[idx]);
        }
    }

    fn h_mul_assign(&self, lhs: &mut [R], rhs: &[R]) {
        debug_assert_eq!(self.length, lhs.len());
        debug_assert_eq!(self.length, rhs.len());

        for idx in 0..self.length {
            self.handle_scalar.h_mul_assign(&mut lhs[idx], &rhs[idx]);
        }
    }
}

/**
 * Product of additive groups is an additive group.
 * */
impl<A, H: AddGroupHandle<A>> AddGroupHandle<NArray<A>> for FixedArrayHandle<&H> {
    fn h_set_zero(&self, val: &mut NArray<A>) {
        self.h_set_zero(&mut val[..]);
    }

    fn h_is_zero(&self, val: &NArray<A>) -> bool {
        self.h_is_zero(&val[..])
    }

    fn h_add(&self, lhs: &NArray<A>, rhs: &NArray<A>, out: &mut NArray<A>) {
        self.h_add(&lhs[..], &rhs[..], &mut out[..]);
    }

    fn h_sub(&self, lhs: &NArray<A>, rhs: &NArray<A>, out: &mut NArray<A>) {
        self.h_sub(&lhs[..], &rhs[..], &mut out[..]);
    }

    fn h_add_assign(&self, lhs: &mut NArray<A>, rhs: &NArray<A>) {
        self.h_add_assign(&mut lhs[..], &rhs[..]);
    }

    fn h_sub_assign(&self, lhs: &mut NArray<A>, rhs: &NArray<A>) {
        self.h_sub_assign(&mut lhs[..], &rhs[..]);
    }
}

/**
 * Product of rings is a ring.
 * On the other hand, product of fields is not a field,
 * so we do not include an implementation.
 * */
impl<R, H: RingHandle<R>> RingHandle<NArray<R>> for FixedArrayHandle<&H> {
    fn h_set_one(&self, val: &mut NArray<R>) {
        self.h_set_one(&mut val[..]);
    }

    fn h_mul(&self, lhs: &NArray<R>, rhs: &NArray<R>, out: &mut NArray<R>) {
        self.h_mul(&lhs[..], &rhs[..], &mut out[..]);
    }

    fn h_mul_assign(&self, lhs: &mut NArray<R>, rhs: &NArray<R>) {
        self.h_mul_assign(&mut lhs[..], &rhs[..]);
    }
}

/**
 * Handle which partitions the vector into equal-sized chunks,
 * then handles each part.
 * Each handle should be a reference.
 * */
pub struct ChunksHandle<H> {
    chunk_handle: Vec<H>,
    chunk_length: usize,
}

impl<H> ChunksHandle<H> {
    /// Create a new handle for chunks.
    pub fn new(chunk_handle: Vec<H>, chunk_length: usize) -> Self {
        ChunksHandle { chunk_handle, chunk_length }
    }
}

impl<H> ChunksHandle<&H> {
    /**
     * Returns iterator of FixedArrayHandle with corresponding range to the chunk.
     * */
    fn chunks_iter(&self) -> impl Iterator<Item = (FixedArrayHandle<&H>, Range<usize>)> {
        let chunk_range = (0 as usize..)
            .step_by(self.chunk_length)
            .map(|start| (start..start + self.chunk_length));
        self.chunk_handle
            .iter()
            .map(|handle| FixedArrayHandle {
                handle_scalar: *handle,
                length: self.chunk_length,
            })
            .zip(chunk_range)
    }
}

impl<A, H: AddGroupHandle<A>> AddGroupHandle<NArray<A>> for ChunksHandle<&H> {
    fn h_set_zero(&self, val: &mut NArray<A>) {
        for (handle, range) in self.chunks_iter() {
            handle.h_set_zero(&mut val[range]);
        }
    }

    fn h_is_zero(&self, val: &NArray<A>) -> bool {
        self.chunks_iter()
            .all(|(handle, range)| handle.h_is_zero(&val[range]))
    }

    fn h_add(&self, lhs: &NArray<A>, rhs: &NArray<A>, out: &mut NArray<A>) {
        for (handle, range) in self.chunks_iter() {
            handle.h_add(&lhs[range.clone()], &rhs[range.clone()], &mut out[range]);
        }
    }

    fn h_sub(&self, lhs: &NArray<A>, rhs: &NArray<A>, out: &mut NArray<A>) {
        for (handle, range) in self.chunks_iter() {
            handle.h_sub(&lhs[range.clone()], &rhs[range.clone()], &mut out[range]);
        }
    }

    fn h_add_assign(&self, lhs: &mut NArray<A>, rhs: &NArray<A>) {
        for (handle, range) in self.chunks_iter() {
            handle.h_add_assign(&mut lhs[range.clone()], &rhs[range]);
        }
    }

    fn h_sub_assign(&self, lhs: &mut NArray<A>, rhs: &NArray<A>) {
        for (handle, range) in self.chunks_iter() {
            handle.h_sub_assign(&mut lhs[range.clone()], &rhs[range]);
        }
    }
}

impl<R, H: RingHandle<R>> RingHandle<NArray<R>> for ChunksHandle<&H> {
    fn h_set_one(&self, val: &mut NArray<R>) {
        for (handle, range) in self.chunks_iter() {
            handle.h_set_one(&mut val[range]);
        }
    }

    fn h_mul(&self, lhs: &NArray<R>, rhs: &NArray<R>, out: &mut NArray<R>) {
        for (handle, range) in self.chunks_iter() {
            handle.h_mul(&lhs[range.clone()], &rhs[range.clone()], &mut out[range]);
        }
    }

    fn h_mul_assign(&self, lhs: &mut NArray<R>, rhs: &NArray<R>) {
        for (handle, range) in self.chunks_iter() {
            handle.h_mul_assign(&mut lhs[range.clone()], &rhs[range]);
        }
    }
}

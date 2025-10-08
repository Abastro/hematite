use crate::engine::handle::AddGroupHandle;

use super::array::NArray;

/**
 * Array handle for fixed size arrays.
 * */
pub struct FixedArrayHandle<H> {
    handle_scalar: H,
    length: usize,
}

/**
 * Product of additive groups is an additive group.
 * */
impl<A, H: AddGroupHandle<A>> AddGroupHandle<NArray<A>> for FixedArrayHandle<H> {
    fn h_zero(&self) -> NArray<A> {
        NArray::new(self.length, || self.handle_scalar.h_zero())
    }

    fn h_is_zero(&self, val: &NArray<A>) -> bool {
        val.internal()
            .iter()
            .all(|v| self.handle_scalar.h_is_zero(v))
    }

    fn h_add(&self, mut lhs: NArray<A>, rhs: &NArray<A>) -> NArray<A> {
        self.h_add_assign(&mut lhs, rhs);
        lhs
    }

    fn h_sub(&self, mut lhs: NArray<A>, rhs: &NArray<A>) -> NArray<A> {
        self.h_sub_assign(&mut lhs, rhs);
        lhs
    }

    fn h_add_assign(&self, lhs: &mut NArray<A>, rhs: &NArray<A>) {
        debug_assert_eq!(lhs.len(), self.length);
        debug_assert_eq!(rhs.len(), self.length);

        for i in 0..self.length {
            self.handle_scalar.h_add_assign(&mut lhs[i], &rhs[i]);
        }
    }

    fn h_sub_assign(&self, lhs: &mut NArray<A>, rhs: &NArray<A>) {
        debug_assert_eq!(lhs.len(), self.length);
        debug_assert_eq!(rhs.len(), self.length);

        for i in 0..self.length {
            self.handle_scalar.h_sub_assign(&mut lhs[i], &rhs[i]);
        }
    }
}

// TODO Handle which acts at part of the array?

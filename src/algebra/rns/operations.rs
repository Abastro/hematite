use crate::{
    algebra::rns::rns::RNS,
    engine::handle::{AddGroupHandle, RingHandle},
    scalar::modulus::Mod64,
    tensor::tensor::{Tensor, TensorShape},
};
use std::fmt::Debug;

/// Generic handle for RNS tensors.
pub struct RNSHandle<H> {
    /// Handle the modulus.
    pub handle_mod: dyn Fn(u64) -> H,
}

// TODO Take out subvectors to process

impl<Shape, H> AddGroupHandle<Tensor<RNS<Shape>, Mod64>> for RNSHandle<H>
where
    Shape: TensorShape + Eq + Debug,
    H: AddGroupHandle<Mod64>,
{
    /// Hardcoded
    fn h_set_zero(&self, val: &mut Tensor<RNS<Shape>, Mod64>) {
        for v in val.iter_mut() {
            v.representative = 0
        }
    }

    /// Hardcoded
    fn h_is_zero(&self, val: &Tensor<RNS<Shape>, Mod64>) -> bool {
        val.iter().all(|v| v.representative == 0)
    }

    fn h_add(
        &self,
        lhs: &Tensor<RNS<Shape>, Mod64>,
        rhs: &Tensor<RNS<Shape>, Mod64>,
        out: &mut Tensor<RNS<Shape>, Mod64>,
    ) {
        let shape = crate::the_debug!(lhs.shape(), rhs.shape(), out.shape());
        let rns = shape.rns_modulus;

        for level in 0..rns.total_size() {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.per_level.total_size() {
                let coord = shape.rns_coord(level, index);

                handle.h_add(
                    &lhs.data()[coord],
                    &rhs.data()[coord],
                    &mut out.data_mut()[coord],
                );
            }
        }
    }

    fn h_sub(
        &self,
        lhs: &Tensor<RNS<Shape>, Mod64>,
        rhs: &Tensor<RNS<Shape>, Mod64>,
        out: &mut Tensor<RNS<Shape>, Mod64>,
    ) {
        let shape = crate::the_debug!(lhs.shape(), rhs.shape(), out.shape());
        let rns = shape.rns_modulus;

        for level in 0..rns.total_size() {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.per_level.total_size() {
                let coord = shape.rns_coord(level, index);

                handle.h_sub(
                    &lhs.data()[coord],
                    &rhs.data()[coord],
                    &mut out.data_mut()[coord],
                );
            }
        }
    }

    fn h_add_assign(&self, lhs: &mut Tensor<RNS<Shape>, Mod64>, rhs: &Tensor<RNS<Shape>, Mod64>) {
        let shape = crate::the_debug!(lhs.shape(), rhs.shape());
        let rns = shape.rns_modulus;

        for level in 0..rns.total_size() {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.per_level.total_size() {
                let coord = shape.rns_coord(level, index);

                handle.h_add_assign(&mut lhs.data_mut()[coord], &rhs.data()[coord]);
            }
        }
    }

    fn h_sub_assign(&self, lhs: &mut Tensor<RNS<Shape>, Mod64>, rhs: &Tensor<RNS<Shape>, Mod64>) {
        let shape = crate::the_debug!(lhs.shape(), rhs.shape());
        let rns = shape.rns_modulus;

        for level in 0..rns.total_size() {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.per_level.total_size() {
                let coord = shape.rns_coord(level, index);

                handle.h_sub_assign(&mut lhs.data_mut()[coord], &rhs.data()[coord]);
            }
        }
    }

    fn h_neg(&self, arg: &Tensor<RNS<Shape>, Mod64>, out: &mut Tensor<RNS<Shape>, Mod64>) {
        let shape = crate::the_debug!(arg.shape(), out.shape());
        let rns = shape.rns_modulus;

        for level in 0..rns.total_size() {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.per_level.total_size() {
                let coord = shape.rns_coord(level, index);

                handle.h_neg(&arg.data()[coord], &mut out.data_mut()[coord]);
            }
        }
    }

    fn h_neg_assign(&self, arg: &mut Tensor<RNS<Shape>, Mod64>) {
        let shape = arg.shape();
        let rns = shape.rns_modulus;

        for level in 0..rns.total_size() {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.per_level.total_size() {
                let coord = shape.rns_coord(level, index);

                handle.h_neg_assign(&mut arg.data_mut()[coord]);
            }
        }
    }
}

impl<Shape, H> RingHandle<Tensor<RNS<Shape>, Mod64>> for RNSHandle<H>
where
    Shape: TensorShape + Eq + Debug,
    H: RingHandle<Mod64>,
{
    /// Hardcoded
    fn h_set_one(&self, val: &mut Tensor<RNS<Shape>, Mod64>) {
        for v in val.iter_mut() {
            v.representative = 1;
        }
    }

    fn h_mul(
        &self,
        lhs: &Tensor<RNS<Shape>, Mod64>,
        rhs: &Tensor<RNS<Shape>, Mod64>,
        out: &mut Tensor<RNS<Shape>, Mod64>,
    ) {
        let shape = crate::the_debug!(lhs.shape(), rhs.shape(), out.shape());
        let rns = shape.rns_modulus;

        for level in 0..rns.total_size() {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.total_size() {
                let coord = shape.rns_coord(level, index);

                handle.h_mul(
                    &lhs.data()[coord],
                    &rhs.data()[coord],
                    &mut out.data_mut()[coord],
                );
            }
        }
    }

    fn h_mul_assign(&self, lhs: &mut Tensor<RNS<Shape>, Mod64>, rhs: &Tensor<RNS<Shape>, Mod64>) {
        let shape = crate::the_debug!(lhs.shape(), rhs.shape());
        let rns = shape.rns_modulus;

        for level in 0..rns.total_size() {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.per_level.total_size() {
                let coord = shape.rns_coord(level, index);

                handle.h_mul_assign(&mut lhs.data_mut()[coord], &rhs.data()[coord]);
            }
        }
    }
}

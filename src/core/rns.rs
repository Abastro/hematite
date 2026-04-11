use crate::{
    engine::handle::{AddGroupHandle, RingHandle},
    modulus::modulus::Mod64,
    tensor::tensor::{Tensor, TensorShape},
    the_debug,
};
use std::{cmp::Ordering, collections::BTreeSet, fmt::Debug, ops::Index};

pub const MAX_MODULUS_COUNT: usize = 64;

/// Denotes a modulus as Residue Number System (RNS).
/// RNS decomposition is used to represent large modulus,
/// where the large modulus is a product of basis modulus.
///
/// The list of modulus should have no duplicates.
/// The ordering is given by inclusion relation,
/// which corresponds to divisibility relation of modulus.
///
/// Also serves as a shape for RNS modular numbers.
#[derive(Clone, Copy, Debug)]
pub struct RNSModulus {
    modulus_count: usize,
    modulus: [u64; MAX_MODULUS_COUNT],
}

impl RNSModulus {
    pub fn new(modulus: &[u64]) -> Self {
        let mut arr = [0u64; MAX_MODULUS_COUNT];
        for i in 0..modulus.len() {
            arr[i] = modulus[i]
        }

        RNSModulus {
            modulus_count: modulus.len(),
            modulus: arr,
        }
    }

    /// Basis modulus indices for the level.
    pub fn modulus(&self) -> &[u64] {
        &self.modulus[..self.modulus_count]
    }
}

impl PartialEq for RNSModulus {
    fn eq(&self, other: &Self) -> bool {
        self.modulus() == other.modulus()
    }
}

impl Eq for RNSModulus {}

impl PartialOrd for RNSModulus {
    /// Slower implementation, but does the job for now
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let set_a: BTreeSet<u64> = self.modulus().iter().copied().collect();
        let set_b: BTreeSet<u64> = other.modulus().iter().copied().collect();

        if set_a == set_b {
            Some(Ordering::Equal)
        } else if set_a.is_subset(&set_b) {
            Some(Ordering::Less)
        } else if set_b.is_subset(&set_a) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl Index<usize> for RNSModulus {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.modulus()[index]
    }
}

impl TensorShape for RNSModulus {
    type Coord = usize;

    fn coord_index(&self, coord: Self::Coord) -> usize {
        coord
    }

    fn total_size(&self) -> usize {
        self.modulus_count
    }
}

/// Shape with RNS structure.
///
/// Each prime (level) is given continguous memory.
pub trait RNSShape: TensorShape {
    fn rns_modulus(&self) -> RNSModulus;
    fn len_per_prime(&self) -> usize;

    /// Coordinates in RNS vs. index format
    fn rns_coord(&self, level: usize, index: usize) -> usize {
        level * self.len_per_prime() + index
    }
}

/// Denotes shapes which are pointwise.
///
/// Note that product of rings like Z_Q \otimes R is not pointwise,
/// hence they need to be excluded.
pub trait RNSPointwiseShape: RNSShape {}

impl RNSShape for RNSModulus {
    fn rns_modulus(&self) -> RNSModulus {
        *self
    }

    fn len_per_prime(&self) -> usize {
        1
    }
}

/// Generic handle for RNS tensors.
pub struct RNSHandle<H> {
    pub handle_mod: dyn Fn(u64) -> H,
}

impl<Shape, H> AddGroupHandle<Tensor<Shape, Mod64>> for RNSHandle<H>
where
    Shape: RNSShape + Eq + Debug,
    H: AddGroupHandle<Mod64>,
{
    /// Hardcoded
    fn h_set_zero(&self, val: &mut Tensor<Shape, Mod64>) {
        for v in val.iter_mut() {
            v.representative = 0
        }
    }

    /// Hardcoded
    fn h_is_zero(&self, val: &Tensor<Shape, Mod64>) -> bool {
        val.iter().all(|v| v.representative == 0)
    }

    fn h_add(
        &self,
        lhs: &Tensor<Shape, Mod64>,
        rhs: &Tensor<Shape, Mod64>,
        out: &mut Tensor<Shape, Mod64>,
    ) {
        let shape = the_debug!(lhs.shape(), rhs.shape(), out.shape());
        let rns = shape.rns_modulus();

        for level in 0..rns.modulus_count {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.len_per_prime() {
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
        lhs: &Tensor<Shape, Mod64>,
        rhs: &Tensor<Shape, Mod64>,
        out: &mut Tensor<Shape, Mod64>,
    ) {
        let shape = the_debug!(lhs.shape(), rhs.shape(), out.shape());
        let rns = shape.rns_modulus();

        for level in 0..rns.modulus_count {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.len_per_prime() {
                let coord = shape.rns_coord(level, index);

                handle.h_sub(
                    &lhs.data()[coord],
                    &rhs.data()[coord],
                    &mut out.data_mut()[coord],
                );
            }
        }
    }

    fn h_add_assign(&self, lhs: &mut Tensor<Shape, Mod64>, rhs: &Tensor<Shape, Mod64>) {
        let shape = the_debug!(lhs.shape(), rhs.shape());
        let rns = shape.rns_modulus();

        for level in 0..rns.modulus_count {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.len_per_prime() {
                let coord = shape.rns_coord(level, index);

                handle.h_add_assign(&mut lhs.data_mut()[coord], &rhs.data()[coord]);
            }
        }
    }

    fn h_sub_assign(&self, lhs: &mut Tensor<Shape, Mod64>, rhs: &Tensor<Shape, Mod64>) {
        let shape = the_debug!(lhs.shape(), rhs.shape());
        let rns = shape.rns_modulus();

        for level in 0..rns.modulus_count {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.len_per_prime() {
                let coord = shape.rns_coord(level, index);

                handle.h_sub_assign(&mut lhs.data_mut()[coord], &rhs.data()[coord]);
            }
        }
    }
}

impl<Shape, H> RingHandle<Tensor<Shape, Mod64>> for RNSHandle<H>
where
    Shape: RNSPointwiseShape + Eq + Debug,
    H: RingHandle<Mod64>,
{
    /// Hardcoded
    fn h_set_one(&self, val: &mut Tensor<Shape, Mod64>) {
        for v in val.iter_mut() {
            v.representative = 1;
        }
    }

    fn h_mul(
        &self,
        lhs: &Tensor<Shape, Mod64>,
        rhs: &Tensor<Shape, Mod64>,
        out: &mut Tensor<Shape, Mod64>,
    ) {
        let shape = the_debug!(lhs.shape(), rhs.shape(), out.shape());
        let rns = shape.rns_modulus();

        for level in 0..rns.modulus_count {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.len_per_prime() {
                let coord = shape.rns_coord(level, index);

                handle.h_mul(
                    &lhs.data()[coord],
                    &rhs.data()[coord],
                    &mut out.data_mut()[coord],
                );
            }
        }
    }

    fn h_mul_assign(&self, lhs: &mut Tensor<Shape, Mod64>, rhs: &Tensor<Shape, Mod64>) {
        let shape = the_debug!(lhs.shape(), rhs.shape());
        let rns = shape.rns_modulus();

        for level in 0..rns.modulus_count {
            let handle = (self.handle_mod)(rns[level]);
            for index in 0..shape.len_per_prime() {
                let coord = shape.rns_coord(level, index);

                handle.h_mul_assign(&mut lhs.data_mut()[coord], &rhs.data()[coord]);
            }
        }
    }
}

/// Shape of vector of RNS modulus.
///
/// For compatibility with e.g. RNSCyclo,
/// the coord order is level * length.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RNSVector {
    pub modulus: RNSModulus,
    pub length: usize,
}

impl TensorShape for RNSVector {
    type Coord = (usize, usize);

    fn coord_index(&self, coord: Self::Coord) -> usize {
        let (level, idx) = coord;
        level * self.length + idx
    }

    fn total_size(&self) -> usize {
        self.modulus.total_size() * self.length
    }
}

impl RNSShape for RNSVector {
    fn rns_modulus(&self) -> RNSModulus {
        self.modulus
    }

    fn len_per_prime(&self) -> usize {
        self.length
    }
}

impl RNSPointwiseShape for RNSVector {}

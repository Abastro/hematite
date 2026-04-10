use std::{cmp::Ordering, collections::BTreeSet};

use crate::tensor::tensor::TensorShape;

pub const MAX_MODULUS_COUNT: usize = 64;

/// Denotes a modulus as Residue Number System (RNS).
/// RNS decomposition is used to represent large modulus,
/// where the large modulus is a product of basis modulus.
///
/// The list of modulus should be in ascending order with no duplicates.
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

impl TensorShape for RNSModulus {
    type Coord = usize;

    fn coord_index(&self, coord: Self::Coord) -> usize {
        coord
    }

    fn total_size(&self) -> usize {
        self.modulus_count
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

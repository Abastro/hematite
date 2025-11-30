use crate::{
    engine::handle::{AddGroupHandle, RingHandle},
    tensor::{array::NArray, array_ops::ChunksHandle},
    the,
    transform::number_theoretic::NTT,
};

use super::levelled::Level;

/// Metadata for the ring elements.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CycloMeta {
    length: usize,
    level: Level,
}

/// Denotes the cyclotomic elements.
///
/// The constituents are in 2-dimensional format,
/// where elements for each modulus are grouped together.
#[derive(Clone)]
pub struct Cyclo<const NTT: bool> {
    metadata: CycloMeta,
    constituents: NArray<u64>,
}

impl<const NTT: bool> Cyclo<{ NTT }> {
    pub fn get_metadata(&self) -> CycloMeta {
        self.metadata
    }
}

/// Parameter for the modular cyclotomic rings.
pub struct Parameter<Mod> {
    /// Length of the constituents in each ring element.
    length: usize,
    /// Order of the roots of unity in the cyclotomic ring.
    order: usize,
    /// The modulus basis.
    modulus: Vec<Mod>,
}

/// Handle for the modular cyclotomic rings.
///
/// At this level, the metadata should be the same between operands for arithmetic operations.
pub struct CycloHandle<Mod> {
    parameter: Parameter<Mod>,
    ntt: Vec<NTT<Mod, u64>>,
}

impl<Mod> CycloHandle<&Mod> {
    pub fn parameter(&self) -> &Parameter<&Mod> {
        &self.parameter
    }

    /// Obtaining the chunk handle is not optimized for now
    fn chunk_handle(&self, metadata: CycloMeta) -> ChunksHandle<&Mod> {
        let param = &self.parameter;
        let level = metadata.level;
        let handles: Vec<_> = level
            .modulus_basis()
            .iter()
            .map(|idx| param.modulus[*idx])
            .collect();
        ChunksHandle::new(handles, param.length)
    }
}

/// Additive handle for cyclotomic rings.
impl<Mod: AddGroupHandle<u64>, const NTT: bool> AddGroupHandle<Cyclo<{ NTT }>>
    for CycloHandle<&Mod>
{
    fn h_set_zero(&self, val: &mut Cyclo<{ NTT }>) {
        self.chunk_handle(val.metadata)
            .h_set_zero(&mut val.constituents);
    }

    fn h_is_zero(&self, val: &Cyclo<{ NTT }>) -> bool {
        self.chunk_handle(val.metadata).h_is_zero(&val.constituents)
    }

    fn h_add(&self, lhs: &Cyclo<{ NTT }>, rhs: &Cyclo<{ NTT }>, out: &mut Cyclo<{ NTT }>) {
        let metadata = the!(lhs.metadata, rhs.metadata, out.metadata);

        self.chunk_handle(metadata).h_add(
            &lhs.constituents,
            &rhs.constituents,
            &mut out.constituents,
        );
    }

    fn h_sub(&self, lhs: &Cyclo<{ NTT }>, rhs: &Cyclo<{ NTT }>, out: &mut Cyclo<{ NTT }>) {
        let metadata = the!(lhs.metadata, rhs.metadata, out.metadata);

        self.chunk_handle(metadata).h_sub(
            &lhs.constituents,
            &rhs.constituents,
            &mut out.constituents,
        );
    }

    fn h_add_assign(&self, lhs: &mut Cyclo<{ NTT }>, rhs: &Cyclo<{ NTT }>) {
        let metadata = the!(lhs.metadata, rhs.metadata);

        self.chunk_handle(metadata)
            .h_add_assign(&mut lhs.constituents, &rhs.constituents);
    }

    fn h_sub_assign(&self, lhs: &mut Cyclo<{ NTT }>, rhs: &Cyclo<{ NTT }>) {
        let metadata = the!(lhs.metadata, rhs.metadata);

        self.chunk_handle(metadata)
            .h_sub_assign(&mut lhs.constituents, &rhs.constituents);
    }
}

/// Ring handle for modular cyclotomic rings.
/// NTT is required for efficient product.
impl<Mod: RingHandle<u64>> RingHandle<Cyclo<true>> for CycloHandle<&Mod> {
    fn h_set_one(&self, val: &mut Cyclo<true>) {
        self.chunk_handle(val.metadata)
            .h_set_one(&mut val.constituents);
    }

    fn h_mul(&self, lhs: &Cyclo<true>, rhs: &Cyclo<true>, out: &mut Cyclo<true>) {
        let metadata = the!(lhs.metadata, rhs.metadata, out.metadata);

        self.chunk_handle(metadata).h_mul(
            &lhs.constituents,
            &rhs.constituents,
            &mut out.constituents,
        );
    }

    fn h_mul_assign(&self, lhs: &mut Cyclo<true>, rhs: &Cyclo<true>) {
        let metadata = the!(lhs.metadata, rhs.metadata);

        self.chunk_handle(metadata)
            .h_mul_assign(&mut lhs.constituents, &rhs.constituents);
    }
}

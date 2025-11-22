use crate::{
    engine::handle::{AddGroupHandle, RingHandle},
    tensor::{array::NArray, array_ops::ChunksHandle},
    transform::number_theoretic::NTT,
};

/// Metadata for the ring elements.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CycloMeta {
    length: usize,
    level: usize,
}

/// Denotes the cyclotomic elements.
#[derive(Clone)]
pub struct Cyclo<const NTT: bool> {
    metadata: CycloMeta,
    constituents: NArray<u64>,
}

impl<const NTT: bool> Cyclo<{ NTT }> {
    pub fn get_metadata(&self) -> CycloMeta {
        self.metadata
    }

    /// Set metadata of the ring element without changing the internal data.
    /// The length should be the same for the operation to make sense.
    pub fn set_metadata(&mut self, new_meta: CycloMeta) {
        assert_eq!(self.metadata.length, new_meta.length);

        if self.metadata.level != new_meta.level {
            self.constituents.resize((new_meta.level + 1) * new_meta.length, || 0);
        }

        self.metadata = new_meta;
    }
}

/// Parameter for the modular cyclotomic rings.
pub struct Parameter<Mod> {
    /// Length of the constituents in each ring element.
    length: usize,
    /// Order of the roots of unity in the cyclotomic ring.
    order: usize,
    /// Chain of modulus.
    modulus: Vec<Mod>,
}

/// Handle for the modular cyclotomic rings.
pub struct Handle<Mod> {
    parameter: Parameter<Mod>,
    /// Chunk handles for each level
    chunk_handles: Vec<ChunksHandle<Mod>>,
    ntt: Vec<NTT<Mod, u64>>,
}

impl<Mod> Handle<Mod> {
    pub fn parameter(&self) -> &Parameter<Mod> {
        &self.parameter
    }

    fn chunk_handle(&self, metadata: CycloMeta) -> &ChunksHandle<Mod> {
        &self.chunk_handles[metadata.level]
    }
}

impl<Mod: AddGroupHandle<u64>, const NTT: bool> AddGroupHandle<Cyclo<{ NTT }>> for Handle<&Mod> {
    fn h_set_zero(&self, val: &mut Cyclo<{ NTT }>) {
        self.chunk_handle(val.metadata)
            .h_set_zero(&mut val.constituents);
    }

    fn h_is_zero(&self, val: &Cyclo<{ NTT }>) -> bool {
        self.chunk_handle(val.metadata).h_is_zero(&val.constituents)
    }

    fn h_add(&self, lhs: &Cyclo<{ NTT }>, rhs: &Cyclo<{ NTT }>, out: &mut Cyclo<{ NTT }>) {
        // TODO Match level
        // Seems like simply matching level does not make sense without context.
        // Need to think about how to handle this.
        let metadata = lhs.metadata;
        out.metadata = metadata;

        self.chunk_handle(metadata).h_add(
            &lhs.constituents,
            &rhs.constituents,
            &mut out.constituents,
        );
    }

    fn h_sub(&self, lhs: &Cyclo<{ NTT }>, rhs: &Cyclo<{ NTT }>, out: &mut Cyclo<{ NTT }>) {
        // TODO Match level
        let metadata = lhs.metadata;
        out.metadata = metadata;

        self.chunk_handle(metadata).h_sub(
            &lhs.constituents,
            &rhs.constituents,
            &mut out.constituents,
        );
    }

    fn h_add_assign(&self, lhs: &mut Cyclo<{ NTT }>, rhs: &Cyclo<{ NTT }>) {
        // TODO Match level
        self.chunk_handle(lhs.metadata)
            .h_add_assign(&mut lhs.constituents, &rhs.constituents);
    }

    fn h_sub_assign(&self, lhs: &mut Cyclo<{ NTT }>, rhs: &Cyclo<{ NTT }>) {
        // TODO Match level
        self.chunk_handle(lhs.metadata)
            .h_sub_assign(&mut lhs.constituents, &rhs.constituents);
    }
}

/// Ring handle for modular cyclotomic rings.
/// NTT is required for efficient product.
impl<Mod: RingHandle<u64>> RingHandle<Cyclo<true>> for Handle<&Mod> {
    fn h_set_one(&self, val: &mut Cyclo<true>) {
        todo!()
    }

    fn h_mul(&self, lhs: &Cyclo<true>, rhs: &Cyclo<true>, out: &mut Cyclo<true>) {
        // TODO Match level
        self.chunk_handle(lhs.metadata).h_mul(
            &lhs.constituents,
            &rhs.constituents,
            &mut out.constituents,
        );
    }

    fn h_mul_assign(&self, lhs: &mut Cyclo<true>, rhs: &Cyclo<true>) {
        // TODO Match level
        self.chunk_handle(lhs.metadata)
            .h_mul_assign(&mut lhs.constituents, &rhs.constituents);
    }
}

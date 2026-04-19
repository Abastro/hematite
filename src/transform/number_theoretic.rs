use crate::modulus::modulus::Mod64;

/// Number Theoretic Transform (NTT) is an analogue of DFT for modular integers.
///
/// In our case, usually it is the isomorphism Z_q[X]/(Phi_M(X)) = Z_q^N.
pub trait NTT {
    /// Ring to product
    fn forward(&self, vec: &mut [Mod64]);

    /// Product to ring
    fn backward(&self, vec: &mut [Mod64]);
}

/// NTT given by the isomorphism Z_q[X]/(X^N+1) = Z_q^N.
pub struct NTTP2 {
    // TODO Roots
}

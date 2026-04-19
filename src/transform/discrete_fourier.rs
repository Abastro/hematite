use crate::real_cx::complexes::Cx128;

/// Discrete Fourier Transform (DFT) transforms ring C[X]/(X^M - 1) to vector C^M.
///
/// We adapt it to the case of ring C[X]/(f(X)) where f(X) is a factor of X^M - 1.
/// 
/// To perform it in-place, the ring coefficients are in bit-reversal order.
pub trait DFT {
    /// Ring to product
    fn forward(&self, vec: &mut [Cx128]);

    /// Product to ring
    fn backward(&self, vec: &mut [Cx128]);
}

/// DFT given by isomorphism C[X]/(X^n - i) = C^n,
/// with evaluation points zeta^(5^j).
pub struct DFTP2 {
    // TODO Roots
}
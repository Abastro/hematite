use crate::real_cx::complexes::Cx128;

/// Discrete Fourier Transform (DFT) transforms ring \bC[X]/(X^M - 1) to vector \bC^M.
///
/// We adapt it to the case of ring \bC[X]/(f(X)) where f(X) is a factor of X^M - 1.
/// 
/// To perform it in-place, the ring coefficients are in bit-reversal order.
pub trait DFT {
    /// Ring to vector
    fn forward(&self, vec: &mut [Cx128]);

    /// Vector to ring
    fn backward(&self, vec: &mut [Cx128]);
}

/// DFT given by isomorphism \bC[X]/(X^N - i) = C^N,
/// with evaluation points zeta^(5^j).
pub struct DFTP2 {
    pub roots: Vec<Cx128>
}
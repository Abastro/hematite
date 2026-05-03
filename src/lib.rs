//! Homomorphic encryption library written in rust.

/// Utilities.
pub mod utils;

/// Underlying functionalities for implementations.
pub mod engine;

/// Arrays and tensors, with operations on them.
pub mod tensor;

/// Scalars relevant to homomorphic operations.
///
/// Namely, integers, real/complex numbers, and modular arithmetic.
pub mod scalar;

/// Transformations like Number-Theoretic Transform or Discrete Fourier Transform.
pub mod transform;

/// Algebra for homomorphic encryption, specifically Cyclotomic rings and RNS.
pub mod algebra;

/// Encryption schemes.
pub mod encryption;

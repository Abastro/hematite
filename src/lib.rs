//! Homomorphic encryption library written in rust.

/// Underlying functionalities for implementations.
pub mod engine;

/// Modular operations.
pub mod modulus;

/// Arrays and tensors, with operations on them.
pub mod tensor;

/// Transformations like Number-Theoretic Transform or Discrete Fourier Transform.
pub mod transform;

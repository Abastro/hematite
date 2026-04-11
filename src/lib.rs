//! Homomorphic encryption library written in rust.

/// Utilities.
pub mod utils;

/// Underlying functionalities for implementations.
pub mod engine;

/// Mathematical basis, like finding primes.
pub mod math;

/// Modular arithmetic.
pub mod modulus;

pub mod real_cx;

/// Arrays and tensors, with operations on them.
pub mod tensor;

/// Transformations like Number-Theoretic Transform or Discrete Fourier Transform.
pub mod transform;

pub mod core;

/// Encryption schemes.
pub mod encryption;

//! Homomorphic encryption library written in rust.

/// Utilities.
pub mod utils;

/// Underlying functionalities for implementations.
pub mod engine;

/// Mathematical basis, like finding primes.
pub mod math;

/// Arrays and tensors, with operations on them.
pub mod tensor;

/// Scalars relevant to homomorphic operations.
///
/// Namely, integers, real/complex numbers, and modular arithmetic.
pub mod scalar;

/// Transformations like Number-Theoretic Transform or Discrete Fourier Transform.
pub mod transform;

/// Residue number system for representing large composite modulus.
pub mod rns;

/// Polynomial rings, especially cyclotomic rings.
pub mod poly;

pub mod core;

/// Encryption schemes.
pub mod encryption;

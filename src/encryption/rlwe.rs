use crate::tensor::array::NArray;

/// Metadata for the ciphertext.
pub struct Metadata {
}

/// Ciphertext of rank K,
/// with decryption circuit given as m = b_0 + a_0 s_0 ... + a_K s_K.
pub struct CiphertextRank<const K: usize> {
    b: NArray<u64>,
    ai: [NArray<u64>; K],
}

/// We use 'rank 0 ciphertext' as plaintext.
pub type Plaintext = CiphertextRank<0>;

/// The usual ciphertext is a 'rank 1 ciphertext'.
pub type Ciphertext = CiphertextRank<1>;

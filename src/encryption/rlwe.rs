use crate::tensor::array::NArray;

/// Ciphertext of rank K,
/// with decryption circuit given as m = b_0 + a_0 s_0 ... + a_K s_K.
pub struct Ciphertext<const K: usize> {
    b: NArray<u64>,
    ai: [NArray<u64>; K],
}

/// We use 'rank 0 ciphertext' as plaintext.
pub type Plaintext = Ciphertext<0>;

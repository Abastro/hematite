/// Returns the number of bits needed to represent `n`.
/// Returns 0 for n == 0.
#[inline]
pub const fn bitwidth(n: u64) -> u32 {
    u64::BITS - n.leading_zeros()
}
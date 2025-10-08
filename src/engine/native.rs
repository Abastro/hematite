use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use num_traits::{One, Zero};

/**
 * Native/cheap ring types.
 */
pub trait NativeRing:
    Copy
    + Eq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + Zero
    + One
{
}

impl<
    R: Copy
        + Eq
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + AddAssign
        + SubAssign
        + MulAssign
        + Zero
        + One,
> NativeRing for R
{
}

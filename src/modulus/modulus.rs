/**
 * Modulus denotes the submodule or ideal I of R
 * which is used to construct R/I.
 */
pub trait Modulus<R> {
    /**
     * Representative of the value as element in R/I.
    * */
    fn representative(&self, value: R) -> R;
}

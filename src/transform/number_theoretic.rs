/**
 * Number-Theoretic Transformation,
 * decomposing R_q[X] / \phi_power(X) into R_q^length.
 * */
pub struct NTT<R, Mod> {
    modulus: Mod,
    length: usize,
    power: usize,
    primitive: R,
    roots_forward: Vec<R>,
    roots_backward: Vec<R>,
}


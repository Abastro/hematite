use crate::modulus::simple::ModulusSimple;

impl ModulusSimple {   
    /// Finds primitive root of a prime.
    pub fn primitive_root(&self) -> u64 {
        let q = self.modulus;
        // let factors: Vec<u64>;

        if let Some(g) = (2..q - 2).find(|_cand| todo!()) {
            g
        } else {
            panic!("Cannot find primitive root");
        }
    }
}

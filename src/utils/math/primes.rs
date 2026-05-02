/// NTT primes generators.
/// An NTT prime is a prime q such that
/// \bZ_q[X] / \Phi_order (X) decomposes fully into \bZ_q^degree.
pub mod primes_ntt {
    use std::iter;

    fn inside_bit_size(val: u64, bit_size: usize) -> bool {
        return ((val as f64).log2() - bit_size as f64).abs() < 0.5;
    }

    /// Ascending iterator for NTT primes.
    pub fn ascending_iterator(power: usize, bit_size: usize) -> impl Iterator<Item = u64> {
        let base = 1u64 << bit_size;
        let power_u64 = power as u64;
        // multiple larger than base
        let mut current = ((base - 1) / power_u64 + 1) * power_u64 + 1;

        iter::from_fn(move || {
            // Find larger prime
            while !primal::is_prime(current) {
                current += power_u64;

                // Prime too large
                if !inside_bit_size(current, bit_size) {
                    return None;
                }
            }

            Some(current)
        })
    }

    /// Descending iterator for NTT primes.
    pub fn descending_iterator(power: usize, bit_size: usize) -> impl Iterator<Item = u64> {
        let base = 1u64 << bit_size;
        let power_u64 = power as u64;
        // multiple smaller than base
        let mut current = (base / power_u64) * power_u64 + 1;

        iter::from_fn(move || {
            // Find smaller prime
            while !primal::is_prime(current) {
                current -= power_u64;

                // Prime too large
                if !inside_bit_size(current, bit_size) {
                    return None;
                }
            }

            Some(current)
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use proptest::prelude::Strategy;

    pub fn prime_strategy(min: u64, max: u64) -> impl Strategy<Value = u64> {
        (min..max).prop_filter("not prime", |&n| primal::is_prime(n))
    }
}

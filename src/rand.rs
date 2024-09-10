use crate::traits::Digit;
use crate::BigUint;

use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::{thread_rng, Rng};

fn gen_n_random_values<T>(n: usize) -> Vec<T>
where
    Standard: Distribution<T>,
{
    let mut ret = Vec::<T>::with_capacity(n);
    for _ in 0..n {
        ret.push(thread_rng().gen::<T>());
    }
    ret
}

/// Generates a random BigUint with n bits
///
/// The only non random bits is the nth one, which guaranteed to be 1
pub fn gen_random_biguint<T: Digit>(n: usize) -> BigUint<T>
where
    Standard: Distribution<T>,
{
    if n == 0 {
        return BigUint::default();
    }

    let mut ret = BigUint::<T>::from(gen_n_random_values::<T>(n / T::NB_BITS + 1));
    let nb_bits = ret.nb_bits();

    if nb_bits == 0 {
        ret = BigUint::from(1u32) << (n - 1);
    } else if nb_bits > n {
        ret >>= nb_bits - n;
    }
    ret.set_bit(n - 1, true);

    debug_assert_eq!(ret.nb_bits(), n);
    ret
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_random_biguint() {
        for i in 1..20 {
            let a = super::gen_random_biguint::<u32>(i * 10);
            let b = super::gen_random_biguint::<u32>(i * 10);
            assert_eq!(a.nb_bits(), i * 10);
            assert_eq!(b.nb_bits(), i * 10);
        }
    }
}

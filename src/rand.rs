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
pub fn gen_random_biguint<T: Digit>(n: usize) -> BigUint<T>
where
    Standard: Distribution<T>,
{
    BigUint::<T>::from(gen_n_random_values::<T>(n / T::NB_BITS))
}

use crate::BigUint;

use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::{thread_rng, Rng};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

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
#[cfg_attr(feature = "pyo3", pyfunction)]
pub fn gen_random_biguint(n: usize) -> BigUint {
    biguint!(gen_n_random_values::<u32>(n / 32))
}

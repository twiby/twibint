use core::cmp::Ordering;
use digits_vec::Digits;

use pyo3::prelude::*;

#[macro_export]
macro_rules! biguintvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            BigUint::from(temp_vec)
        }
    };
}

#[macro_export]
macro_rules! biguint {
    ( $( $x:expr ),* ) => {
        {
            $(
                BigUint::from($x)
            )*
        }
    };
}

mod digits_vec;
pub(crate) mod fmt;
pub(crate) mod froms;
pub(crate) mod ops;

#[cfg(test)]
mod test;

#[derive(Clone, Debug, PartialEq, Eq)]
#[pyclass]
pub struct BigUint {
    pub val: Vec<u32>,
}

impl BigUint {
    pub fn new(val: u32) -> BigUint {
        BigUint { val: vec![val] }
    }

    #[inline]
    pub fn nb_bits(&self) -> usize {
        let mut last = *self.val.iter().last().unwrap();

        let mut nb_bits_last = 0;
        while last > 0 {
            last >>= 1;
            nb_bits_last += 1;
        }

        32 * (self.val.len() - 1) + nb_bits_last
    }

    #[inline]
    pub fn bit(&self, b: usize) -> bool {
        (self.val[b / 32] >> b % 32) & 1 != 0
    }

    #[inline]
    pub fn bits<'a>(&'a self) -> impl DoubleEndedIterator<Item = bool> + 'a {
        (0..self.nb_bits()).map(|b| self.bit(b))
    }

    #[inline]
    pub(crate) fn remove_trailing_zeros(&mut self) {
        while self.val.len() > 1 && self.val.last() == Some(&0) {
            self.val.pop();
        }
    }
}

impl Default for BigUint {
    fn default() -> BigUint {
        BigUint::new(0)
    }
}

impl std::hash::Hash for BigUint {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.val.hash(state);
    }
}

impl PartialOrd<BigUint> for BigUint {
    fn partial_cmp(&self, other: &BigUint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigUint {
    fn cmp(&self, other: &BigUint) -> Ordering {
        match self.val.len().cmp(&other.val.len()) {
            Ordering::Equal => (),
            o => return o,
        };
        for (a, b) in self.val.iter().zip(other.val.iter()).rev() {
            match a.cmp(b) {
                Ordering::Equal => continue,
                o => return o,
            };
        }
        Ordering::Equal
    }
}

//! biguint: declares the BigUint type and implements all its operations.

use core::cmp::Ordering;
use digits_vec::Digits;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

mod digits_vec;
pub(crate) mod fmt;
pub(crate) mod froms;
pub(crate) mod ops;

#[cfg(test)]
mod test;

// TODO: implement ilog2 and that sort of things

/// Representation of an unsigned integer with an infinite number of bits (above
/// a certain position, they are all 0).
///
/// The internal representation is a Vec of u32 as a radix representation. For zero,
/// we cheat a little and use a vector with a single element: 0.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct BigUint {
    pub val: Vec<u32>,
}

impl BigUint {
    /// Trivial constructor: from a single `u32` \
    /// Integers higher than `u32::MAX` are supposed to be constructed using the
    /// various `From<T>` implementations.
    pub fn new(val: u32) -> BigUint {
        BigUint { val: vec![val] }
    }

    /// Returns the minimal number of bits necessary for a binary representation.
    #[inline]
    pub fn nb_bits(&self) -> usize {
        32 * (self.val.len() - 1) + (32 - self.val.iter().last().unwrap().leading_zeros() as usize)
    }

    /// Returns the bth bit as a bool. Since we represent an infinite number of bits,
    /// b could be higher than `self.nb_bits()`
    /// (but realistically to be other than 0 it will fit in a usize)
    #[inline]
    pub fn bit(&self, b: usize) -> bool {
        if b >= self.val.len() * 32 {
            false
        } else {
            (self.val[b / 32] >> b % 32) & 1 != 0
        }
    }

    /// Return an iterator on the bits, returning `bool` values. The iterator will
    /// stop as soon as all the infinitely remaining bits are 0
    #[inline]
    pub fn bits<'a>(&'a self) -> impl DoubleEndedIterator<Item = bool> + 'a {
        (0..self.nb_bits()).map(|b| self.bit(b))
    }

    /// (private) clean trailing zeros of the representation, if any, after an
    /// operation has been performed.
    #[inline]
    pub(crate) fn remove_trailing_zeros(&mut self) {
        let count = self.val.len() - self.val.iter().rev().take_while(|n| **n == 0).count();
        self.val.resize(std::cmp::max(count, 1), 0u32);
    }
}

/// Default implementation for BigUint: returns 0.
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

//! biguint: declares the BigUint type and implements all its operations.

use crate::traits::Digit;

use core::cmp::Ordering;

pub(crate) mod fmt;
pub(crate) mod froms;
pub(crate) mod ops;

mod digits_vec;
use digits_vec::Digits;

#[cfg(test)]
mod test;

// TODO: implement ilog2 and that sort of things

/// Representation of an unsigned integer with an infinite number of bits (above
/// a certain position, they are all 0).
///
/// The internal representation is a Vec of a type that implements Digit as a radix representation.
/// For zero, we cheat a little and use a vector with a single element: 0.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BigUint<T: Digit> {
    pub(crate) val: Vec<T>,
}

impl<T: Digit> BigUint<T> {
    /// Trivial constructor: from a single digit \
    /// Integers higher than `T::MAX` are supposed to be constructed using the
    /// various `From<_>` implementations.
    pub fn new(val: T) -> BigUint<T> {
        BigUint { val: vec![val] }
    }

    /// Returns the minimal number of bits necessary for a binary representation.
    #[inline]
    pub fn nb_bits(&self) -> usize {
        T::NB_BITS * (self.val.len() - 1)
            + (T::NB_BITS - self.val.iter().last().unwrap().leading_zeros() as usize)
    }

    /// Returns the bth bit as a bool. Since we represent an infinite number of bits,
    /// b could be higher than `self.nb_bits()`
    /// (but realistically to be other than 0 it will fit in a usize)
    #[inline]
    pub fn bit(&self, b: usize) -> bool {
        if b >= self.val.len() * T::NB_BITS {
            false
        } else {
            (self.val[b / T::NB_BITS] >> b % T::NB_BITS) & T::ONE != T::ZERO
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
        let count = self.val.len() - self.val.iter().rev().take_while(|n| **n == T::ZERO).count();
        self.val.resize(std::cmp::max(count, 1), T::ZERO);
    }
}

/// Default implementation for BigUint: returns 0.
impl<T: Digit> Default for BigUint<T> {
    fn default() -> BigUint<T> {
        BigUint::new(T::ZERO)
    }
}

impl<T: Digit> std::hash::Hash for BigUint<T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.val.hash(state);
    }
}

impl<T: Digit> PartialOrd<BigUint<T>> for BigUint<T> {
    fn partial_cmp(&self, other: &BigUint<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Digit> Ord for BigUint<T> {
    fn cmp(&self, other: &BigUint<T>) -> Ordering {
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

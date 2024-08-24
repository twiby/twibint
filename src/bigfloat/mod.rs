//! Experimental implementation of a BigFloat type: a floating point signed
//! number, able to represent a subset of rational numbers.
//!
//! These numbers are not meant to be approximations, and every operation
//! must be implemented in a lossless manner
use crate::BigInt;
use crate::BigUint;

use crate::traits::Digit;

mod froms;
mod ops;

#[cfg(test)]
mod tests;

/// The BigFloat type represents a signed floating point number.
/// It is composed of a `BigInt` represeting the mantissa, and a scale
/// specifying by how many digits it is supposed to be shifted
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BigFloat<T: Digit> {
    int: BigInt<T>,
    scale: isize,
}

impl<T: Digit> BigFloat<T> {
    fn new(val: T) -> Self {
        Self::from(BigUint::<T>::new(val))
    }

    /// Remove zero-digits at the beginning
    fn simplify(&mut self) {
        let nb_zeros: usize = self
            .int
            .uint
            .val
            .iter()
            .take_while(|n| **n == T::ZERO)
            .count()
            .try_into()
            .unwrap();
        self.scale += nb_zeros as isize;
        self.int.uint.val.drain(..nb_zeros);
    }

    #[inline]
    fn equal_int(&self, other_sign: bool, other: &[T]) -> bool {
        let positive_scale = if self.scale >= 0isize {
            self.scale as usize
        } else {
            return false;
        };

        if positive_scale >= other.len() {
            return false;
        }

        let mut equal = other[..positive_scale].iter().all(|n| *n == T::ZERO);
        equal &= self.int.signed_eq(other_sign, &other[positive_scale..]);
        equal
    }
}

impl<T: Digit> Default for BigFloat<T> {
    fn default() -> Self {
        BigFloat {
            int: Default::default(),
            scale: 0,
        }
    }
}

impl<T: Digit> std::hash::Hash for BigFloat<T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.int.hash(state);
        self.scale.hash(state);
    }
}

impl<T: Digit> PartialEq<BigInt<T>> for BigFloat<T> {
    fn eq(&self, other: &BigInt<T>) -> bool {
        self.equal_int(other.sign, &other.uint.val)
    }
}

impl<T: Digit> PartialEq<BigUint<T>> for BigFloat<T> {
    fn eq(&self, other: &BigUint<T>) -> bool {
        self.equal_int(true, &other.val)
    }
}

impl<T: Digit> PartialEq<BigFloat<T>> for BigInt<T> {
    fn eq(&self, other: &BigFloat<T>) -> bool {
        other.equal_int(self.sign, &self.uint.val)
    }
}

impl<T: Digit> PartialEq<BigFloat<T>> for BigUint<T> {
    fn eq(&self, other: &BigFloat<T>) -> bool {
        other.equal_int(true, &self.val)
    }
}

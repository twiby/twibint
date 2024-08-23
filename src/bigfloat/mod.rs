//! Experimental implementation of a BigFloat type: a floating point signed
//! number, able to represent a subset of rational numbers.
//!
//! These numbers are not meant to be approximations, and every operation
//! must be implemented in a lossless manner
use crate::BigInt;

use crate::traits::Digit;

mod froms;
mod ops;

#[cfg(test)]
mod tests;

/// The BigFloat type represents a signed floating point number.
/// It is composed of a `BigInt` represeting the mantissa, and a scale
/// specifying by how many digits it is supposed to be shifted
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct BigFloat<T: Digit> {
    int: BigInt<T>,
    scale: isize,
}

// impl<T: Digit> BigFloat<T> {
//     fn
// }

impl<T: Digit> std::hash::Hash for BigFloat<T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.int.hash(state);
        self.scale.hash(state);
    }
}
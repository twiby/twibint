//! bigint: declares the BigInt type and implements all its operations.

use crate::traits::{Digit, SignedDigit};
use crate::BigUint;
use core::cmp::Ordering;

mod fmt;
mod froms;
mod ops;

#[cfg(test)]
mod test;

// TODO: implement ilog2 and that sort of things

/// Representation of an signed integer with an infinite number of bits.
///
/// The internal representation has 2 members: a BigUint for the absolute value, and
/// a boolean for the sign.
#[derive(Clone, Debug, Eq)]
pub struct BigInt<T: Digit> {
    pub uint: BigUint<T>,
    pub sign: bool,
}

impl<T: Digit> BigInt<T> {
    /// Trivial constructor: from a single `i32` \
    /// Integers higher than `i32::MAX` or lowar than `i32::MIN` are supposed
    /// to be constructed using the various `From<T>` implementations.
    pub fn new(val: T::Signed) -> BigInt<T> {
        BigInt::<T> {
            uint: BigUint::<T>::new(val.abs()),
            sign: val.is_positive(),
        }
    }

    /// Returns true if the integer is strictly higher than 0, false otherwise
    pub fn is_sign_positive(&self) -> bool {
        self.uint != Default::default() && self.sign
    }
    /// Returns true if the integer is strictly lower than 0, false otherwise
    pub fn is_sign_negative(&self) -> bool {
        self.uint != Default::default() && !self.sign
    }
}

/// Default implementation for BigUint: returns 0 with positive sign.
impl<T: Digit> Default for BigInt<T> {
    fn default() -> BigInt<T> {
        BigInt::<T> {
            uint: Default::default(),
            sign: true,
        }
    }
}

impl<T: Digit> std::hash::Hash for BigInt<T> {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.uint.hash(state);
        self.sign.hash(state);
    }
}

impl<T: Digit> PartialOrd<BigInt<T>> for BigInt<T> {
    fn partial_cmp(&self, other: &BigInt<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Digit> Ord for BigInt<T> {
    fn cmp(&self, other: &BigInt<T>) -> Ordering {
        match (self.sign, other.sign) {
            (true, true) => self.uint.cmp(&other.uint),
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => self.uint.cmp(&other.uint).reverse(),
        }
    }
}

/// Test for equality. Returns true if the integers are equal.
///
/// In particular, handles the case where the 2 operands are 0, but with opposite
/// sign. In that case the test still returns true.
impl<T: Digit> PartialEq for BigInt<T> {
    fn eq(&self, other: &Self) -> bool {
        self.uint == other.uint && ((self.sign == other.sign) || (self.uint.val == vec![T::ZERO]))
    }
}

//! bigint: declares the BigInt type and implements all its operations.

use crate::BigUint;
use core::cmp::Ordering;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// macro that allows easy construction of a BigInt from any type T for
/// which From<T> is implemented. Particularly useful for base 10 digit
/// string: `let uint = biguint!["-123456789101112131415"];`
#[macro_export]
macro_rules! bigint {
    ( $( $x:expr ),* ) => {
        {
            $(
                BigInt::from($x)
            )*
        }
    };
}

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
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct BigInt {
    pub uint: BigUint,
    pub sign: bool,
}

impl BigInt {
    /// Trivial constructor: from a single `i32` \
    /// Integers higher than `i32::MAX` or lowar than `i32::MIN` are supposed
    /// to be constructed using the various `From<T>` implementations.
    pub fn new(val: i32) -> BigInt {
        BigInt {
            uint: BigUint::new(val.abs().try_into().unwrap()),
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
impl Default for BigInt {
    fn default() -> BigInt {
        BigInt {
            uint: Default::default(),
            sign: true,
        }
    }
}

impl std::hash::Hash for BigInt {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        self.uint.hash(state);
        self.sign.hash(state);
    }
}

impl PartialOrd<BigInt> for BigInt {
    fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &BigInt) -> Ordering {
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
impl PartialEq for BigInt {
    fn eq(&self, other: &Self) -> bool {
        self.uint == other.uint && ((self.sign == other.sign) || (self.uint.val == vec![0]))
    }
}

//! Experimental implementation of a BigFloat type: a floating point signed
//! number, able to represent a subset of rational numbers.
//!
//! These numbers are not meant to be approximations, and every operation
//! must be implemented in a lossless manner
use crate::BigInt;
use crate::BigUint;
use std::cmp::Ordering;

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
    pub(crate) int: BigInt<T>,
    pub(crate) scale: isize,
}

impl<T: Digit> BigFloat<T> {
    fn new(val: T) -> Self {
        Self::from(BigUint::<T>::new(val))
    }

    #[inline]
    pub(crate) fn with_capacity(mut self, capcity: usize) -> Self {
        self.int.uint.set_capacity(capcity);
        self
    }

    /// Copies data from other into self, keeping self's allocation if possible
    pub fn copy_from(&mut self, other: &Self) {
        self.int.copy_from(&other.int);
        self.scale = other.scale;
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

    fn float_unsigned_ord(&self, other_scale: isize, other: &[T]) -> Ordering {
        if self.scale == other_scale {
            return self.int.uint.ord(other);
        }

        let self_size = self.scale + (self.int.uint.val.len() as isize);
        let other_size = other_scale + (other.len() as isize);

        if self_size < other_size {
            return Ordering::Less;
        }
        if self_size > other_size {
            return Ordering::Greater;
        }

        for (a, b) in self.int.uint.val.iter().rev().zip(other.iter().rev()) {
            match a.cmp(b) {
                Ordering::Equal => (),
                o => return o,
            };
        }

        self.int.uint.val.len().cmp(&other.len())
    }

    fn float_ord(&self, other_scale: isize, other_sign: bool, other: &[T]) -> Ordering {
        match (self.int.sign, other_sign) {
            (true, true) => self.float_unsigned_ord(other_scale, other),
            (false, false) => self.float_unsigned_ord(other_scale, other).reverse(),
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
        }
    }

    /// Consumes this `BigFloat` and returns the closest `BigInt` to it. In case of
    /// 2 options, returns the lowest
    pub(crate) fn round(self) -> BigInt<T> {
        if self.scale >= 0 {
            let scale = self.scale as usize;
            let nb_digits = self.int.uint.val.len();
            let mut int = self.int;
            int.uint.val.resize(nb_digits + scale, T::ZERO);
            int.uint.val.copy_within(0..nb_digits, scale);
            int.uint.val[0..scale].fill(T::ZERO);
            return int;
        } else {
            let scale = (-self.scale) as usize;
            if scale > self.int.uint.val.len() {
                let mut int = self.int;
                int.uint.val.clear();
                int.uint.val.push(T::ZERO);
                int.sign = true;
                return int;
            } else {
                let one_half = BigFloat::from(T::ONE) >> 1;
                let adjust =
                    match one_half.float_unsigned_ord(self.scale, &self.int.uint.val[0..scale]) {
                        Ordering::Less => true,
                        _ => false,
                    };

                let mut int = self.int;
                int.uint.val.drain(0..scale);
                if int.uint.val.is_empty() {
                    int.uint.val.push(T::ZERO);
                }

                if adjust {
                    int.uint += T::ONE;
                }

                return int;
            }
        }
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

impl<T: Digit> PartialOrd<BigFloat<T>> for BigFloat<T> {
    fn partial_cmp(&self, other: &BigFloat<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Digit> PartialOrd<BigUint<T>> for BigFloat<T> {
    fn partial_cmp(&self, other: &BigUint<T>) -> Option<Ordering> {
        if self.int.sign {
            Some(self.float_unsigned_ord(0, &other.val))
        } else {
            Some(Ordering::Less)
        }
    }
}

impl<T: Digit> PartialOrd<BigFloat<T>> for BigUint<T> {
    fn partial_cmp(&self, other: &BigFloat<T>) -> Option<Ordering> {
        other.partial_cmp(self).map(|o| o.reverse())
    }
}

impl<T: Digit> PartialOrd<BigInt<T>> for BigFloat<T> {
    fn partial_cmp(&self, other: &BigInt<T>) -> Option<Ordering> {
        Some(self.float_ord(0, other.sign, &other.uint.val))
    }
}

impl<T: Digit> PartialOrd<BigFloat<T>> for BigInt<T> {
    fn partial_cmp(&self, other: &BigFloat<T>) -> Option<Ordering> {
        other.partial_cmp(self).map(|o| o.reverse())
    }
}

impl<T: Digit> Ord for BigFloat<T> {
    fn cmp(&self, other: &BigFloat<T>) -> Ordering {
        self.float_ord(other.scale, other.int.sign, &other.int.uint.val)
    }
}

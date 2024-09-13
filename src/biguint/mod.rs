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

    /// Allocates for at least `capacity` bits, total.
    ///
    /// ```
    /// use twibint::BigUint;
    /// let n = BigUint::<u64>::default().with_capacity(100);
    /// assert!(n.capacity() >= 100);
    /// ```
    #[inline]
    pub fn with_capacity(mut self, capacity: usize) -> BigUint<T> {
        self.set_capacity(capacity);
        self
    }

    /// Allocates for at least `capacity` bits, total.
    ///
    /// ```
    /// use twibint::BigUint;
    /// let mut n = BigUint::<u64>::default();
    /// n.set_capacity(100);
    /// assert!(n.capacity() >= 100);
    /// ```
    #[inline]
    pub fn set_capacity(&mut self, capacity: usize) {
        if capacity > 0 {
            let target_length = (capacity - 1) / T::NB_BITS + 1;
            let reserve = target_length.max(self.val.len()) - self.val.len();
            self.val.reserve(reserve);
        }
    }

    /// Returns the number of bits this integer can store without reallocating
    ///
    /// ```
    /// use twibint::BigUint;
    /// let n = BigUint::<u64>::default().with_capacity(100);
    /// assert!(n.capacity() >= 100);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.val.capacity() * T::NB_BITS
    }

    /// Returns the minimal number of bits necessary for a binary representation.
    #[inline]
    pub fn nb_bits(&self) -> usize {
        nb_bits(&self.val)
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

    /// Sets the bth bit of the integer. Since we represent an infinite number of bits,
    /// b could be higher than `self.nb_bits()`
    /// (but realistically to be other than 0 it will fit in a usize)
    #[inline]
    pub fn set_bit(&mut self, b: usize, val: bool) {
        if val {
            self.val
                .resize((b / T::NB_BITS + 1).max(self.val.len()), T::ZERO);
            let mask = T::ONE << (b % T::NB_BITS);
            self.val[b / T::NB_BITS] |= mask;
        } else {
            let mask = T::MAX - (T::ONE << (b % T::NB_BITS));
            self.val[b / T::NB_BITS] &= mask;
            self.remove_leading_zeros();
        }
    }

    /// Return an iterator on the bits, returning `bool` values. The iterator will
    /// stop as soon as all the infinitely remaining bits are 0
    #[inline]
    pub fn bits(&self) -> impl DoubleEndedIterator<Item = bool> + '_ {
        (0..self.nb_bits()).map(|b| self.bit(b))
    }

    /// Copies data from other into self, keeping self's allocation if possible
    pub fn copy_from(&mut self, other: &Self) {
        self.val.resize(other.val.len(), T::ZERO);
        self.val[..].copy_from_slice(&other.val[..]);
    }

    /// (private) clean trailing zeros of the representation, if any, after an
    /// operation has been performed.
    #[inline]
    pub(crate) fn remove_leading_zeros(&mut self) {
        let count = self.val.len() - self.val.iter().rev().take_while(|n| **n == T::ZERO).count();
        self.val.truncate(std::cmp::max(count, 1));
    }

    #[inline]
    pub(crate) fn ord(&self, other: &[T]) -> Ordering {
        ord(&self.val, other)
    }
}

#[inline]
pub(crate) fn nb_bits<T: Digit>(a: &[T]) -> usize {
    match a.last() {
        None => 0,
        Some(last) => T::NB_BITS * (a.len() - 1) + (T::NB_BITS - last.leading_zeros() as usize),
    }
}

#[inline]
pub(crate) fn ord<T: Digit>(a: &[T], b: &[T]) -> Ordering {
    match a.len().cmp(&b.len()) {
        Ordering::Equal => (),
        o => return o,
    };
    for (a, b) in a.iter().zip(b.iter()).rev() {
        match a.cmp(b) {
            Ordering::Equal => continue,
            o => return o,
        };
    }
    Ordering::Equal
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
        self.ord(&other.val)
    }
}

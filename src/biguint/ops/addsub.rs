use crate::biguint::nb_bits;
use crate::biguint::ops::add_assign;
use crate::biguint::ops::rsub_assign;
use crate::biguint::ops::sub_assign;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use std::cmp::Ordering;

use crate::traits::Digit;
use crate::BigUint;

impl<T: Digit> BigUint<T> {
    pub(crate) fn clone_for_addition_with(&self, other_len: usize) -> Self {
        let mut data = Vec::<T>::with_capacity(self.val.len().max(other_len) + 1);
        data.extend_from_slice(&self.val);
        BigUint::<T> { val: data }
    }

    #[inline]
    pub(crate) fn add_assign(&mut self, other: &[T]) {
        let target_nb_bits = self.nb_bits().max(nb_bits(other)) + 1;
        let target_length = (target_nb_bits - 1) / T::NB_BITS + 1;
        self.val.resize(target_length, T::ZERO);

        let carry = add_assign(&mut self.val, other);
        debug_assert!(!carry);

        self.remove_leading_zeros();
    }

    #[inline]
    pub(crate) fn sub_assign(&mut self, other: &[T]) {
        match self.ord(other) {
            Ordering::Equal => {
                self.val.clear();
                self.val.push(T::ZERO);
            }
            Ordering::Less => panic!("Attempt at subtraction with underflow"),
            Ordering::Greater => {
                sub_assign(&mut self.val, other);
                self.remove_leading_zeros();
            }
        }
    }

    pub(crate) fn _rsub_assign(&mut self, other: &[T]) {
        match self.ord(other) {
            Ordering::Equal => {
                self.val.clear();
                self.val.push(T::ZERO);
            }
            Ordering::Greater => panic!("Attempt at subtraction with underflow"),
            Ordering::Less => {
                debug_assert!(other.len() >= self.val.len());
                let prev_len = self.val.len();
                self.val.resize(other.len(), T::ZERO);
                rsub_assign(&mut self.val, other, prev_len);
                self.remove_leading_zeros();
            }
        }
    }

    /// Computes the subtraction of `self` from `other`, and stores the result in `self`
    ///
    /// Very much like a subtraction where the operands are reversed. On this
    /// unsigned type this can avoid an unnecessary allocation in some cases.
    #[inline]
    pub fn rsub_assign(&mut self, other: &BigUint<T>) {
        self._rsub_assign(&other.val);
    }
}

impl<T: Digit> Add<T> for &BigUint<T> {
    type Output = BigUint<T>;

    fn add(self, other: T) -> Self::Output {
        let mut ret: BigUint<T> = self.clone_for_addition_with(1);
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<T> for BigUint<T> {
    type Output = BigUint<T>;

    fn add(mut self, other: T) -> Self::Output {
        self += &other;
        self
    }
}

impl<T: Digit> Add<&T> for &BigUint<T> {
    type Output = BigUint<T>;

    fn add(self, other: &T) -> Self::Output {
        let mut ret: BigUint<T> = self.clone_for_addition_with(1);
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<&T> for BigUint<T> {
    type Output = BigUint<T>;

    fn add(mut self, other: &T) -> Self::Output {
        self += other;
        self
    }
}
impl<T: Digit> Add<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;

    fn add(self, other: &BigUint<T>) -> Self::Output {
        let mut ret = self.clone_for_addition_with(other.val.len());
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;

    fn add(mut self, other: BigUint<T>) -> Self::Output {
        self += &other;
        self
    }
}
impl<T: Digit> Add<&BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;

    fn add(mut self, other: &BigUint<T>) -> Self::Output {
        self += other;
        self
    }
}
impl<T: Digit> Add<BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;

    fn add(self, mut other: BigUint<T>) -> Self::Output {
        other += self;
        other
    }
}

impl<T: Digit> AddAssign<T> for BigUint<T> {
    fn add_assign(&mut self, other: T) {
        self.add_assign(&[other]);
    }
}
impl<T: Digit> AddAssign<&T> for BigUint<T> {
    fn add_assign(&mut self, other: &T) {
        self.add_assign(&[*other]);
    }
}

impl<T: Digit> AddAssign<BigUint<T>> for BigUint<T> {
    fn add_assign(&mut self, other: BigUint<T>) {
        *self += &other;
    }
}

impl<T: Digit> AddAssign<&BigUint<T>> for BigUint<T> {
    fn add_assign(&mut self, other: &BigUint<T>) {
        self.add_assign(&other.val);
    }
}

impl<T: Digit> SubAssign<T> for BigUint<T> {
    fn sub_assign(&mut self, other: T) {
        *self -= &BigUint::<T>::new(other);
    }
}
impl<T: Digit> SubAssign<&T> for BigUint<T> {
    fn sub_assign(&mut self, other: &T) {
        *self -= *other;
    }
}
impl<T: Digit> SubAssign<BigUint<T>> for BigUint<T> {
    fn sub_assign(&mut self, other: BigUint<T>) {
        *self -= &other;
    }
}
impl<T: Digit> SubAssign<&BigUint<T>> for BigUint<T> {
    fn sub_assign(&mut self, other: &BigUint<T>) {
        self.sub_assign(&other.val);
    }
}
impl<T: Digit> Sub<T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn sub(self, other: T) -> BigUint<T> {
        let mut ret = self.clone();
        ret -= other;
        return ret;
    }
}
impl<T: Digit> Sub<&T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn sub(self, other: &T) -> BigUint<T> {
        let mut ret = self.clone();
        ret -= other;
        return ret;
    }
}
impl<T: Digit> Sub<T> for BigUint<T> {
    type Output = BigUint<T>;
    fn sub(mut self, other: T) -> BigUint<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<&T> for BigUint<T> {
    type Output = BigUint<T>;
    fn sub(mut self, other: &T) -> BigUint<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn sub(mut self, other: BigUint<T>) -> BigUint<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<&BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn sub(mut self, other: &BigUint<T>) -> BigUint<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn sub(self, other: BigUint<T>) -> BigUint<T> {
        self - &other
    }
}
impl<T: Digit> Sub<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn sub(self, other: &BigUint<T>) -> BigUint<T> {
        let mut ret = self.clone();
        ret -= other;
        ret
    }
}

impl<T, T2> Sum<T> for BigUint<T2>
where
    T2: Digit,
    BigUint<T2>: AddAssign<T>,
{
    fn sum<I>(iter: I) -> BigUint<T2>
    where
        I: Iterator<Item = T>,
    {
        let mut ret = BigUint::<T2>::new(T2::ZERO);
        for el in iter {
            ret += el;
        }
        ret
    }
}

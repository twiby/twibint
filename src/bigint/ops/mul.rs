use core::iter::Product;
use core::ops::{Mul, MulAssign};

use crate::traits::Digit;
use crate::{BigInt, BigUint};

impl<T: Digit> BigInt<T> {
    /// Use this integer to store the multiplication of `a` and `b`
    ///
    /// This is handy when one wants to keep allocations around to store
    /// the result of a multiplication
    ///
    /// ```
    /// use twibint::BigInt;
    ///
    /// let mut n = -BigInt::from(vec![u64::MAX; 4]);
    /// let a = BigInt::from(-2i64);
    /// let b = BigInt::from(-4i64);
    ///
    /// n.set_to_mul(&a, &b);
    /// assert_eq!(n, BigInt::from(8));
    /// ```
    #[inline]
    pub fn set_to_mul(&mut self, a: &BigInt<T>, b: &BigInt<T>) {
        self._set_to_mul(a.sign, &a.uint.val, b.sign, &b.uint.val);
    }

    #[inline]
    pub(crate) fn _set_to_mul(&mut self, a_sign: bool, a: &[T], b_sign: bool, b: &[T]) {
        self.uint._set_to_mul(a, b);
        self.sign = a_sign == b_sign;
    }
}

impl<T: Digit> MulAssign<T> for BigInt<T> {
    fn mul_assign(&mut self, other: T) {
        self.uint *= other;
    }
}
impl<T: Digit> MulAssign<&T> for BigInt<T> {
    fn mul_assign(&mut self, other: &T) {
        self.uint *= *other;
    }
}
impl<T: Digit> MulAssign<&BigInt<T>> for BigInt<T> {
    fn mul_assign(&mut self, other: &BigInt<T>) {
        *self = &*self * other;
    }
}
impl<T: Digit> MulAssign<BigInt<T>> for BigInt<T> {
    fn mul_assign(&mut self, other: BigInt<T>) {
        *self = &*self * &other;
    }
}
impl<T: Digit> Mul<T> for &BigInt<T> {
    type Output = BigInt<T>;
    fn mul(self, other: T) -> BigInt<T> {
        let mut ret = self.clone();
        ret *= other;
        ret
    }
}
impl<T: Digit> Mul<&T> for &BigInt<T> {
    type Output = BigInt<T>;
    fn mul(self, other: &T) -> BigInt<T> {
        let mut ret = self.clone();
        ret *= *other;
        ret
    }
}
impl<T: Digit> Mul<T> for BigInt<T> {
    type Output = BigInt<T>;
    fn mul(self, other: T) -> BigInt<T> {
        &self * other
    }
}
impl<T: Digit> Mul<&T> for BigInt<T> {
    type Output = BigInt<T>;
    fn mul(self, other: &T) -> BigInt<T> {
        &self * *other
    }
}
impl<T: Digit> Mul<&BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn mul(self, other: &BigInt<T>) -> BigInt<T> {
        let mut ret = BigInt::<T>::default()
            .with_capacity((self.uint.val.len() + other.uint.val.len()) * T::NB_BITS);
        ret._set_to_mul(self.sign, &self.uint.val, other.sign, &other.uint.val);
        ret
    }
}
impl<T: Digit> Mul<BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn mul(self, other: BigInt<T>) -> BigInt<T> {
        &self * &other
    }
}
impl<T: Digit> Mul<BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn mul(self, other: BigInt<T>) -> BigInt<T> {
        self * &other
    }
}
impl<T: Digit> Mul<&BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn mul(self, other: &BigInt<T>) -> BigInt<T> {
        &self * other
    }
}

impl<T, D: Digit> Product<T> for BigInt<D>
where
    BigInt<D>: MulAssign<T>,
{
    fn product<I>(iter: I) -> BigInt<D>
    where
        I: Iterator<Item = T>,
    {
        let mut ret = BigInt::<D>::from(BigUint::<D>::new(D::ONE));
        for el in iter {
            ret *= el;
        }
        ret
    }
}

use core::iter::Product;
use core::ops::{Mul, MulAssign};

use crate::traits::Digit;
use crate::BigUint;

impl<T: Digit> BigUint<T> {
    /// Use this integer to store the multiplication of `a` and `b`
    ///
    /// This is handy when one wants to keep allocations around to store
    /// the result of a multiplication
    ///
    /// ```
    /// use twibint::BigUint;
    ///
    /// let mut n = BigUint::from(vec![u64::MAX; 4]);
    /// let a = BigUint::from(2u64);
    /// let b = BigUint::from(4u64);
    ///
    /// n.set_to_mul(&a, &b);
    /// assert_eq!(n.try_into_digit().unwrap(), 8);
    /// ```
    #[inline]
    pub fn set_to_mul(&mut self, a: &BigUint<T>, b: &BigUint<T>) {
        self._set_to_mul(&a.val, &b.val);
    }

    #[inline]
    pub(crate) fn _set_to_mul(&mut self, a: &[T], b: &[T]) {
        self.val.resize(a.len() + b.len(), T::ZERO);
        super::implem_choices::mul(&mut self.val, a, b);
        self.remove_leading_zeros();
    }

    #[inline]
    pub(crate) fn mul_assign_digit(&mut self, b: T) {
        self.val.push(T::ZERO);
        super::implem_choices::mul_assign_digit(&mut self.val, b);
        self.remove_leading_zeros();
    }
}

impl<T: Digit> MulAssign<T> for BigUint<T> {
    fn mul_assign(&mut self, other: T) {
        self.mul_assign_digit(other);
    }
}
impl<T: Digit> MulAssign<&T> for BigUint<T> {
    fn mul_assign(&mut self, other: &T) {
        *self *= *other;
    }
}
impl<T: Digit> MulAssign<&BigUint<T>> for BigUint<T> {
    fn mul_assign(&mut self, other: &BigUint<T>) {
        *self = &*self * other;
    }
}
impl<T: Digit> MulAssign<BigUint<T>> for BigUint<T> {
    fn mul_assign(&mut self, other: BigUint<T>) {
        *self = &*self * &other;
    }
}
impl<T: Digit> Mul<T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn mul(self, other: T) -> BigUint<T> {
        let mut ret = self.clone();
        ret *= other;
        ret
    }
}
impl<T: Digit> Mul<&T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn mul(self, other: &T) -> BigUint<T> {
        self * *other
    }
}
impl<T: Digit> Mul<T> for BigUint<T> {
    type Output = BigUint<T>;
    fn mul(mut self, other: T) -> BigUint<T> {
        self *= other;
        self
    }
}
impl<T: Digit> Mul<&T> for BigUint<T> {
    type Output = BigUint<T>;
    fn mul(mut self, other: &T) -> BigUint<T> {
        self *= other;
        self
    }
}

impl<T: Digit> Mul<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn mul(self, other: &BigUint<T>) -> BigUint<T> {
        let mut ret =
            BigUint::<T>::default().with_capacity((self.val.len() + other.val.len()) * T::NB_BITS);
        ret._set_to_mul(&self.val, &other.val);
        ret
    }
}
impl<T: Digit> Mul<BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn mul(self, other: BigUint<T>) -> BigUint<T> {
        &self * &other
    }
}
impl<T: Digit> Mul<BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn mul(self, other: BigUint<T>) -> BigUint<T> {
        self * &other
    }
}
impl<T: Digit> Mul<&BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn mul(self, other: &BigUint<T>) -> BigUint<T> {
        &self * other
    }
}

impl<T, D: Digit> Product<T> for BigUint<D>
where
    BigUint<D>: MulAssign<T>,
{
    fn product<I>(iter: I) -> BigUint<D>
    where
        I: Iterator<Item = T>,
    {
        let mut ret = BigUint::<D>::new(D::ONE);
        for el in iter {
            ret *= el;
        }
        ret
    }
}

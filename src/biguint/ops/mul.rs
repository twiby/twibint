use core::iter::Product;
use core::ops::{Mul, MulAssign};

use crate::traits::Digit;
use crate::BigUint;

impl<T: Digit> MulAssign<T> for BigUint<T> {
    fn mul_assign(&mut self, other: T) {
        *self *= BigUint::<T>::new(other);
    }
}
impl<T: Digit> MulAssign<&T> for BigUint<T> {
    fn mul_assign(&mut self, other: &T) {
        *self *= *other;
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
impl<T: Digit> Mul<T> for BigUint<T> {
    type Output = BigUint<T>;
    fn mul(self, other: T) -> BigUint<T> {
        &self * other
    }
}

impl<T: Digit> Mul<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn mul(self, other: &BigUint<T>) -> BigUint<T> {
        super::implem_choices::mul(&self.val, &other.val).into()
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
impl<T: Digit> Mul<BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn mul(self, other: BigUint<T>) -> BigUint<T> {
        &self * &other
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

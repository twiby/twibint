use core::iter::Product;
use core::ops::{Mul, MulAssign};

use crate::traits::Digit;
use crate::{BigInt, BigUint};

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
impl<T: Digit> Mul<T> for BigInt<T> {
    type Output = BigInt<T>;
    fn mul(self, other: T) -> BigInt<T> {
        &self * other
    }
}
impl<T: Digit> Mul<&BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn mul(self, other: &BigInt<T>) -> BigInt<T> {
        BigInt::<T> {
            uint: &self.uint * &other.uint,
            sign: self.sign == other.sign,
        }
    }
}
impl<T: Digit> Mul<BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn mul(self, other: BigInt<T>) -> BigInt<T> {
        &self * &other
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

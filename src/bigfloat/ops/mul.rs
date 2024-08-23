use crate::BigFloat;
use std::iter::Product;
use std::ops::Mul;
use std::ops::MulAssign;

use crate::traits::Digit;

impl<T: Digit> MulAssign<T> for BigFloat<T> {
    fn mul_assign(&mut self, other: T) {
        *self *= BigFloat::<T>::new(other);
    }
}
impl<T: Digit> MulAssign<&T> for BigFloat<T> {
    fn mul_assign(&mut self, other: &T) {
        *self *= *other;
    }
}
impl<T: Digit> MulAssign<&BigFloat<T>> for BigFloat<T> {
    fn mul_assign(&mut self, other: &BigFloat<T>) {
        *self = &*self * other;
    }
}
impl<T: Digit> MulAssign<BigFloat<T>> for BigFloat<T> {
    fn mul_assign(&mut self, other: BigFloat<T>) {
        *self = &*self * &other;
    }
}
impl<T: Digit> Mul<T> for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn mul(self, other: T) -> BigFloat<T> {
        self * BigFloat::<T>::new(other)
    }
}
impl<T: Digit> Mul<&T> for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn mul(self, other: &T) -> BigFloat<T> {
        self * *other
    }
}
impl<T: Digit> Mul<T> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn mul(self, other: T) -> BigFloat<T> {
        &self * other
    }
}
impl<T: Digit> Mul<&T> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn mul(self, other: &T) -> BigFloat<T> {
        &self * other
    }
}

impl<T: Digit> Mul<&BigFloat<T>> for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn mul(self, other: &BigFloat<T>) -> BigFloat<T> {
        let int = (&self.int * &other.int).into();
        let mut ret = BigFloat {
            int,
            scale: self.scale + other.scale,
        };
        ret.simplify();
        ret
    }
}
impl<T: Digit> Mul<BigFloat<T>> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn mul(self, other: BigFloat<T>) -> BigFloat<T> {
        &self * &other
    }
}
impl<T: Digit> Mul<BigFloat<T>> for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn mul(self, other: BigFloat<T>) -> BigFloat<T> {
        self * &other
    }
}
impl<T: Digit> Mul<&BigFloat<T>> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn mul(self, other: &BigFloat<T>) -> BigFloat<T> {
        &self * other
    }
}

impl<T, D: Digit> Product<T> for BigFloat<D>
where
    BigFloat<D>: MulAssign<T>,
{
    fn product<I>(iter: I) -> BigFloat<D>
    where
        I: Iterator<Item = T>,
    {
        let mut ret = BigFloat::<D>::new(D::ONE);
        for el in iter {
            ret *= el;
        }
        ret
    }
}

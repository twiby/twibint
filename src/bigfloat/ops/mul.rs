use crate::BigFloat;
use std::iter::Product;
use std::ops::Mul;
use std::ops::MulAssign;

use crate::traits::Digit;

impl<T: Digit> BigFloat<T> {
    #[inline]
    pub(crate) fn set_to_mul(&mut self, a: &BigFloat<T>, b: &BigFloat<T>) {
        self._set_to_mul(
            a.int.sign,
            a.scale,
            &a.int.uint.val,
            b.int.sign,
            b.scale,
            &b.int.uint.val,
        );
    }

    #[inline]
    pub(crate) fn _set_to_mul(
        &mut self,
        a_sign: bool,
        a_scale: isize,
        a: &[T],
        b_sign: bool,
        b_scale: isize,
        b: &[T],
    ) {
        self.int._set_to_mul(a_sign, a, b_sign, b);
        self.scale = a_scale + b_scale;
    }
}

impl<T: Digit> MulAssign<T> for BigFloat<T> {
    fn mul_assign(&mut self, other: T) {
        self.int *= other;
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
        let mut ret = self.clone();
        ret *= other;
        ret
    }
}
impl<T: Digit> Mul<&T> for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn mul(self, other: &T) -> BigFloat<T> {
        let mut ret = self.clone();
        ret *= other;
        ret
    }
}
impl<T: Digit> Mul<T> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn mul(mut self, other: T) -> BigFloat<T> {
        self *= other;
        self
    }
}
impl<T: Digit> Mul<&T> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn mul(mut self, other: &T) -> BigFloat<T> {
        self *= other;
        self
    }
}

impl<T: Digit> Mul<&BigFloat<T>> for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn mul(self, other: &BigFloat<T>) -> BigFloat<T> {
        let mut ret = BigFloat::default()
            .with_capacity((self.int.uint.val.len() + other.int.uint.val.len()) * T::NB_BITS);
        ret._set_to_mul(
            self.int.sign,
            self.scale,
            &self.int.uint.val,
            other.int.sign,
            other.scale,
            &other.int.uint.val,
        );
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

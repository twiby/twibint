use core::iter::Product;
use core::ops::{Mul, MulAssign};

use crate::BigInt;

impl MulAssign<u32> for BigInt {
    fn mul_assign(&mut self, other: u32) {
        self.uint *= other;
    }
}
impl MulAssign<&u32> for BigInt {
    fn mul_assign(&mut self, other: &u32) {
        self.uint *= *other;
    }
}
impl MulAssign<i32> for BigInt {
    fn mul_assign(&mut self, other: i32) {
        *self *= BigInt::from(other);
    }
}
impl MulAssign<&i32> for BigInt {
    fn mul_assign(&mut self, other: &i32) {
        *self *= *other;
    }
}
impl MulAssign<&BigInt> for BigInt {
    fn mul_assign(&mut self, other: &BigInt) {
        *self = &*self * other;
    }
}
impl MulAssign<BigInt> for BigInt {
    fn mul_assign(&mut self, other: BigInt) {
        *self = &*self * &other;
    }
}
impl Mul<u32> for &BigInt {
    type Output = BigInt;
    fn mul(self, other: u32) -> BigInt {
        let mut ret = self.clone();
        ret *= other;
        ret
    }
}
impl Mul<u32> for BigInt {
    type Output = BigInt;
    fn mul(self, other: u32) -> BigInt {
        &self * other
    }
}
impl Mul<&BigInt> for u32 {
    type Output = BigInt;
    fn mul(self, other: &BigInt) -> BigInt {
        other * self
    }
}
impl Mul<i32> for &BigInt {
    type Output = BigInt;
    fn mul(self, other: i32) -> BigInt {
        let mut ret = self.clone();
        ret *= other;
        ret
    }
}
impl Mul<i32> for BigInt {
    type Output = BigInt;
    fn mul(self, other: i32) -> BigInt {
        &self * other
    }
}
impl Mul<&BigInt> for i32 {
    type Output = BigInt;
    fn mul(self, other: &BigInt) -> BigInt {
        other * self
    }
}
impl Mul<&BigInt> for &BigInt {
    type Output = BigInt;
    fn mul(self, other: &BigInt) -> BigInt {
        BigInt {
            uint: &self.uint * &other.uint,
            sign: self.sign == other.sign,
        }
    }
}
impl Mul<BigInt> for BigInt {
    type Output = BigInt;
    fn mul(self, other: BigInt) -> BigInt {
        &self * &other
    }
}

impl<T> Product<T> for BigInt
where
    BigInt: MulAssign<T>,
{
    fn product<I>(iter: I) -> BigInt
    where
        I: Iterator<Item = T>,
    {
        let mut ret = BigInt::new(1);
        for el in iter {
            ret *= el;
        }
        ret
    }
}

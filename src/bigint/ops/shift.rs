use crate::traits::Digit;
use crate::BigInt;

use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

impl<T: Digit> Shr<usize> for &BigInt<T> {
    type Output = BigInt<T>;
    fn shr(self, other: usize) -> BigInt<T> {
        let mut ret = self.clone();
        ret >>= other;
        ret
    }
}
impl<T: Digit> Shr<usize> for BigInt<T> {
    type Output = BigInt<T>;
    fn shr(mut self, other: usize) -> BigInt<T> {
        self >>= other;
        self
    }
}
impl<T: Digit> ShrAssign<usize> for BigInt<T> {
    fn shr_assign(&mut self, other: usize) {
        if *self == BigInt::<T>::default() {
            return;
        } else if self.sign {
            self.uint >>= other;
        } else {
            self.sign = !self.sign;
            *self -= T::ONE;
            self.uint >>= other;
            self.sign = !self.sign;
            *self -= T::ONE;
        }
    }
}

impl<T: Digit> Shl<usize> for &BigInt<T> {
    type Output = BigInt<T>;
    fn shl(self, other: usize) -> BigInt<T> {
        let mut ret = self.clone();
        ret <<= other;
        ret
    }
}
impl<T: Digit> Shl<usize> for BigInt<T> {
    type Output = BigInt<T>;
    fn shl(mut self, other: usize) -> BigInt<T> {
        self <<= other;
        self
    }
}
impl<T: Digit> ShlAssign<usize> for BigInt<T> {
    fn shl_assign(&mut self, other: usize) {
        self.uint <<= other;
    }
}

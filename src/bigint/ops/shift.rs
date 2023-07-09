use crate::BigInt;

use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

impl Shr<usize> for &BigInt {
    type Output = BigInt;
    fn shr(self, other: usize) -> BigInt {
        let mut ret = self.clone();
        ret >>= other;
        ret
    }
}
impl Shr<usize> for BigInt {
    type Output = BigInt;
    fn shr(mut self, other: usize) -> BigInt {
        self >>= other;
        self
    }
}
impl ShrAssign<usize> for BigInt {
    fn shr_assign(&mut self, other: usize) {
        if *self == BigInt::default() {
            return;
        } else if self.sign {
            self.uint >>= other;
        } else {
            self.sign = !self.sign;
            *self -= 1;
            self.uint >>= other;
            self.sign = !self.sign;
            *self -= 1;
        }
    }
}

impl Shl<usize> for &BigInt {
    type Output = BigInt;
    fn shl(self, other: usize) -> BigInt {
        let mut ret = self.clone();
        ret <<= other;
        ret
    }
}
impl Shl<usize> for BigInt {
    type Output = BigInt;
    fn shl(mut self, other: usize) -> BigInt {
        self <<= other;
        self
    }
}
impl ShlAssign<usize> for BigInt {
    fn shl_assign(&mut self, other: usize) {
        self.uint <<= other;
    }
}

use crate::traits::Digit;
use crate::BigInt;

use std::ops::Neg;

impl<T: Digit> Neg for &BigInt<T> {
    type Output = BigInt<T>;
    fn neg(self) -> BigInt<T> {
        let mut ret = self.clone();
        ret.sign = !self.sign;
        ret
    }
}
impl<T: Digit> Neg for BigInt<T> {
    type Output = BigInt<T>;
    fn neg(mut self) -> BigInt<T> {
        self.sign = !self.sign;
        self
    }
}

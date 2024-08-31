use crate::traits::Digit;
use crate::BigFloat;

use std::ops::Neg;

impl<T: Digit> Neg for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn neg(self) -> BigFloat<T> {
        let mut ret = self.clone();
        ret.int.sign = !self.int.sign;
        ret
    }
}
impl<T: Digit> Neg for BigFloat<T> {
    type Output = BigFloat<T>;
    fn neg(mut self) -> BigFloat<T> {
        self.int.sign = !self.int.sign;
        self
    }
}

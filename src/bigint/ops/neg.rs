use std::ops::Neg;

use crate::BigInt;

impl Neg for &BigInt {
    type Output = BigInt;
    fn neg(self) -> BigInt {
        let mut ret = self.clone();
        ret.sign = !self.sign;
        ret
    }
}
impl Neg for BigInt {
    type Output = BigInt;
    fn neg(self) -> BigInt {
        -&self
    }
}

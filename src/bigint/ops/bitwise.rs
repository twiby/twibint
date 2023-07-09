use crate::BigInt;
use std::ops::Not;

impl Not for BigInt {
    type Output = BigInt;
    fn not(mut self) -> BigInt {
        self.sign = !self.sign;
        self.uint -= 1;
        self
    }
}
impl Not for &BigInt {
    type Output = BigInt;
    fn not(self) -> BigInt {
        let mut ret = -self;
        ret -= 1;
        ret
    }
}

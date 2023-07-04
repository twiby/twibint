use crate::traits::Pow;
use crate::BigInt;

impl Pow for BigInt {
    fn pow(&self, exp: usize) -> BigInt {
        Self {
            uint: self.uint.pow(exp),
            sign: self.sign || exp & 1 == 0,
        }
    }
}

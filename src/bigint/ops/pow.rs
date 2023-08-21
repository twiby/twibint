use crate::traits::{Digit, Pow};
use crate::BigInt;

impl<T: Digit> Pow for BigInt<T> {
    fn pow(&self, exp: usize) -> BigInt<T> {
        Self {
            uint: self.uint.pow(exp),
            sign: self.sign || exp & 1 == 0,
        }
    }
}

use crate::BigInt;

impl BigInt {
    pub fn pow(&self, exp: usize) -> BigInt {
        Self {
            uint: self.uint.pow(exp),
            sign: self.sign || exp & 1 == 0,
        }
    }
}

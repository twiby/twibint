use crate::BigInt;
use std::ops::{BitXor, BitXorAssign, Not};

impl Not for BigInt {
    type Output = BigInt;
    fn not(mut self) -> BigInt {
        self.sign = !self.sign;
        self -= 1;
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

impl BitXor<&BigInt> for &BigInt {
    type Output = BigInt;
    fn bitxor(self, other: &BigInt) -> BigInt {
        match (self.sign, other.sign) {
            (true, true) => BigInt::from(&self.uint ^ &other.uint),
            (false, false) => BigInt::from(&!self ^ &!other),
            (true, false) => !BigInt::from(self ^ &!other),
            (false, true) => !BigInt::from(&!self ^ other),
        }
    }
}
impl BitXorAssign<&BigInt> for BigInt {
    fn bitxor_assign(&mut self, other: &BigInt) {
        *self = &*self ^ other;
    }
}

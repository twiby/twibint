use crate::BigInt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

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
            (false, false) => &!self ^ &!other,
            (true, false) => !(self ^ &!other),
            (false, true) => !(&!self ^ other),
        }
    }
}
impl BitXorAssign<&BigInt> for BigInt {
    fn bitxor_assign(&mut self, other: &BigInt) {
        *self = &*self ^ other;
    }
}

impl BitAnd<&BigInt> for &BigInt {
    type Output = BigInt;
    fn bitand(self, other: &BigInt) -> BigInt {
        match (self.sign, other.sign) {
            (true, true) => BigInt::from(&self.uint & &other.uint),
            (false, false) => !(&!self | &!other),
            (true, false) => &(self ^ &!other) & self,
            (false, true) => &(&!self ^ other) & other,
        }
    }
}
impl BitAndAssign<&BigInt> for BigInt {
    fn bitand_assign(&mut self, other: &BigInt) {
        *self = &*self & other;
    }
}

impl BitOr<&BigInt> for &BigInt {
    type Output = BigInt;
    fn bitor(self, other: &BigInt) -> BigInt {
        match (self.sign, other.sign) {
            (true, true) => BigInt::from(&self.uint | &other.uint),
            _ => !(&!self & &!other),
        }
    }
}
impl BitOrAssign<&BigInt> for BigInt {
    fn bitor_assign(&mut self, other: &BigInt) {
        *self = &*self | other;
    }
}

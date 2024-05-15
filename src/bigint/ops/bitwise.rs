use crate::traits::Digit;
use crate::BigInt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

impl<T: Digit> Not for BigInt<T> {
    type Output = BigInt<T>;
    fn not(mut self) -> BigInt<T> {
        self.sign = !self.sign;
        self -= T::ONE;
        self
    }
}
impl<T: Digit> Not for &BigInt<T> {
    type Output = BigInt<T>;
    fn not(self) -> BigInt<T> {
        let mut ret = -self;
        ret -= T::ONE;
        ret
    }
}

impl<T: Digit> BitXor<&BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn bitxor(self, other: &BigInt<T>) -> BigInt<T> {
        match (self.sign, other.sign) {
            (true, true) => BigInt::<T>::from(&self.uint ^ &other.uint),
            (false, false) => &!self ^ &!other,
            (true, false) => !(self ^ &!other),
            (false, true) => !(&!self ^ other),
        }
    }
}
impl<T: Digit> BitXor<&BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn bitxor(self, other: &BigInt<T>) -> Self::Output {
        &self ^ other
    }
}
impl<T: Digit> BitXor<BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn bitxor(self, other: BigInt<T>) -> Self::Output {
        self ^ &other
    }
}
impl<T: Digit> BitXor<BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn bitxor(self, other: BigInt<T>) -> Self::Output {
        &self ^ &other
    }
}
impl<T: Digit> BitXor<T> for BigInt<T> {
    type Output = BigInt<T>;
    fn bitxor(self, other: T) -> Self::Output {
        &self ^ BigInt::from_unsigned(other)
    }
}
impl<T: Digit> BitXor<&T> for BigInt<T> {
    type Output = BigInt<T>;
    fn bitxor(self, other: &T) -> Self::Output {
        &self ^ BigInt::from_unsigned(*other)
    }
}
impl<T: Digit> BitXor<T> for &BigInt<T> {
    type Output = BigInt<T>;
    fn bitxor(self, other: T) -> Self::Output {
        self ^ BigInt::from_unsigned(other)
    }
}
impl<T: Digit> BitXor<&T> for &BigInt<T> {
    type Output = BigInt<T>;
    fn bitxor(self, other: &T) -> Self::Output {
        self ^ BigInt::from_unsigned(*other)
    }
}
impl<T: Digit> BitXorAssign<&BigInt<T>> for BigInt<T> {
    fn bitxor_assign(&mut self, other: &BigInt<T>) {
        *self = &*self ^ other;
    }
}
impl<T: Digit> BitXorAssign<BigInt<T>> for BigInt<T> {
    fn bitxor_assign(&mut self, other: BigInt<T>) {
        *self = &*self ^ &other;
    }
}
impl<T: Digit> BitXorAssign<T> for BigInt<T> {
    fn bitxor_assign(&mut self, other: T) {
        *self = &*self ^ BigInt::from_unsigned(other);
    }
}
impl<T: Digit> BitXorAssign<&T> for BigInt<T> {
    fn bitxor_assign(&mut self, other: &T) {
        *self = &*self ^ BigInt::from_unsigned(*other);
    }
}

impl<T: Digit> BitAnd<&BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn bitand(self, other: &BigInt<T>) -> BigInt<T> {
        match (self.sign, other.sign) {
            (true, true) => BigInt::<T>::from(&self.uint & &other.uint),
            (false, false) => !(&!self | &!other),
            (true, false) => &(self ^ &!other) & self,
            (false, true) => &(&!self ^ other) & other,
        }
    }
}
impl<T: Digit> BitAnd<BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn bitand(self, other: BigInt<T>) -> Self::Output {
        &self & &other
    }
}
impl<T: Digit> BitAnd<&BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn bitand(self, other: &BigInt<T>) -> Self::Output {
        &self & other
    }
}
impl<T: Digit> BitAnd<BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn bitand(self, other: BigInt<T>) -> Self::Output {
        self & &other
    }
}
impl<T: Digit> BitAndAssign<&BigInt<T>> for BigInt<T> {
    fn bitand_assign(&mut self, other: &BigInt<T>) {
        *self = &*self & other;
    }
}
impl<T: Digit> BitAndAssign<BigInt<T>> for BigInt<T> {
    fn bitand_assign(&mut self, other: BigInt<T>) {
        *self = &*self & &other;
    }
}
impl<T: Digit> BitAndAssign<T> for BigInt<T> {
    fn bitand_assign(&mut self, other: T) {
        *self = &*self & BigInt::<T>::from_unsigned(other);
    }
}
impl<T: Digit> BitAndAssign<&T> for BigInt<T> {
    fn bitand_assign(&mut self, other: &T) {
        *self = &*self & BigInt::<T>::from_unsigned(*other);
    }
}
impl<T: Digit> BitAnd<T> for BigInt<T> {
    type Output = BigInt<T>;
    fn bitand(mut self, other: T) -> Self::Output {
        self &= other;
        self
    }
}
impl<T: Digit> BitAnd<&T> for BigInt<T> {
    type Output = BigInt<T>;
    fn bitand(mut self, other: &T) -> Self::Output {
        self &= other;
        self
    }
}
impl<T: Digit> BitAnd<T> for &BigInt<T> {
    type Output = BigInt<T>;
    fn bitand(self, other: T) -> Self::Output {
        let mut ret = self.clone();
        ret &= other;
        ret
    }
}
impl<T: Digit> BitAnd<&T> for &BigInt<T> {
    type Output = BigInt<T>;
    fn bitand(self, other: &T) -> Self::Output {
        let mut ret = self.clone();
        ret &= other;
        ret
    }
}

impl<T: Digit> BitOr<&BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn bitor(self, other: &BigInt<T>) -> BigInt<T> {
        match (self.sign, other.sign) {
            (true, true) => BigInt::<T>::from(&self.uint | &other.uint),
            _ => !(&!self & &!other),
        }
    }
}
impl<T: Digit> BitOr<BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn bitor(self, other: BigInt<T>) -> Self::Output {
        self | &other
    }
}
impl<T: Digit> BitOr<&BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn bitor(self, other: &BigInt<T>) -> Self::Output {
        &self | other
    }
}
impl<T: Digit> BitOr<BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn bitor(self, other: BigInt<T>) -> Self::Output {
        &self | &other
    }
}
impl<T: Digit> BitOr<T> for BigInt<T> {
    type Output = BigInt<T>;
    fn bitor(self, other: T) -> Self::Output {
        &self | BigInt::from_unsigned(other)
    }
}
impl<T: Digit> BitOr<T> for &BigInt<T> {
    type Output = BigInt<T>;
    fn bitor(self, other: T) -> Self::Output {
        self | BigInt::from_unsigned(other)
    }
}
impl<T: Digit> BitOr<&T> for BigInt<T> {
    type Output = BigInt<T>;
    fn bitor(self, other: &T) -> Self::Output {
        &self | BigInt::from_unsigned(*other)
    }
}
impl<T: Digit> BitOr<&T> for &BigInt<T> {
    type Output = BigInt<T>;
    fn bitor(self, other: &T) -> Self::Output {
        self | BigInt::from_unsigned(*other)
    }
}
impl<T: Digit> BitOrAssign<&BigInt<T>> for BigInt<T> {
    fn bitor_assign(&mut self, other: &BigInt<T>) {
        *self = &*self | other;
    }
}
impl<T: Digit> BitOrAssign<BigInt<T>> for BigInt<T> {
    fn bitor_assign(&mut self, other: BigInt<T>) {
        *self = &*self | &other;
    }
}
impl<T: Digit> BitOrAssign<T> for BigInt<T> {
    fn bitor_assign(&mut self, other: T) {
        *self = &*self | BigInt::from_unsigned(other);
    }
}
impl<T: Digit> BitOrAssign<&T> for BigInt<T> {
    fn bitor_assign(&mut self, other: &T) {
        *self = &*self | BigInt::from_unsigned(*other);
    }
}

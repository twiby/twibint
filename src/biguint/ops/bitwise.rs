use core::cmp::Ordering;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

use crate::traits::Digit;
use crate::BigUint;

impl<T: Digit> BitAnd<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn bitand(self, other: &BigUint<T>) -> BigUint<T> {
        let (big, small) = match self.cmp(other) {
            Ordering::Equal => return self.clone(),
            Ordering::Less => (other, self),
            Ordering::Greater => (self, other),
        };
        let mut ret = small.clone();
        ret &= big;
        ret
    }
}
impl<T: Digit> BitAnd<&BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn bitand(mut self, other: &BigUint<T>) -> BigUint<T> {
        self &= other;
        self
    }
}
impl<T: Digit> BitAnd<BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn bitand(self, mut other: BigUint<T>) -> BigUint<T> {
        other &= self;
        other
    }
}
impl<T: Digit> BitAnd<BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn bitand(mut self, other: BigUint<T>) -> BigUint<T> {
        self &= &other;
        self
    }
}
impl<T: Digit> BitAndAssign<&BigUint<T>> for BigUint<T> {
    fn bitand_assign(&mut self, other: &BigUint<T>) {
        self.val
            .resize(std::cmp::min(self.val.len(), other.val.len()), T::ZERO);
        self.val
            .iter_mut()
            .zip(other.val.iter())
            .for_each(|(a, b)| *a &= *b);
    }
}
impl<T: Digit> BitAndAssign<BigUint<T>> for BigUint<T> {
    fn bitand_assign(&mut self, other: BigUint<T>) {
        *self &= &other;
    }
}
impl<T: Digit> BitAndAssign<T> for BigUint<T> {
    fn bitand_assign(&mut self, other: T) {
        *self &= BigUint::new(other)
    }
}
impl<T: Digit> BitAndAssign<&T> for BigUint<T> {
    fn bitand_assign(&mut self, other: &T) {
        *self &= BigUint::new(*other)
    }
}
impl<T: Digit> BitAnd<T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn bitand(self, other: T) -> Self::Output {
        let mut ret = self.clone();
        ret &= other;
        ret
    }
}
impl<T: Digit> BitAnd<&T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn bitand(self, other: &T) -> Self::Output {
        let mut ret = self.clone();
        ret &= other;
        ret
    }
}
impl<T: Digit> BitAnd<T> for BigUint<T> {
    type Output = BigUint<T>;
    fn bitand(mut self, other: T) -> Self::Output {
        self &= other;
        self
    }
}
impl<T: Digit> BitAnd<&T> for BigUint<T> {
    type Output = BigUint<T>;
    fn bitand(mut self, other: &T) -> Self::Output {
        self &= other;
        self
    }
}

impl<T: Digit> BitOr<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn bitor(self, other: &BigUint<T>) -> BigUint<T> {
        let (big, small) = match self.cmp(other) {
            Ordering::Equal => return self.clone(),
            Ordering::Less => (other, self),
            Ordering::Greater => (self, other),
        };

        let mut ret = big.clone();
        ret |= small;
        ret
    }
}
impl<T: Digit> BitOr<&BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn bitor(self, other: &BigUint<T>) -> Self::Output {
        &self | other
    }
}
impl<T: Digit> BitOr<BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn bitor(self, other: BigUint<T>) -> Self::Output {
        self | &other
    }
}
impl<T: Digit> BitOr<BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn bitor(self, other: BigUint<T>) -> Self::Output {
        &self | &other
    }
}
impl<T: Digit> BitOr<T> for BigUint<T> {
    type Output = BigUint<T>;
    fn bitor(self, other: T) -> Self::Output {
        &self | BigUint::new(other)
    }
}
impl<T: Digit> BitOr<T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn bitor(self, other: T) -> Self::Output {
        self | BigUint::new(other)
    }
}
impl<T: Digit> BitOr<&T> for BigUint<T> {
    type Output = BigUint<T>;
    fn bitor(self, other: &T) -> Self::Output {
        &self | BigUint::new(*other)
    }
}
impl<T: Digit> BitOr<&T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn bitor(self, other: &T) -> Self::Output {
        self | BigUint::new(*other)
    }
}
impl<T: Digit> BitOrAssign<&BigUint<T>> for BigUint<T> {
    fn bitor_assign(&mut self, other: &BigUint<T>) {
        self.val
            .resize(std::cmp::max(self.val.len(), other.val.len()), T::ZERO);
        self.val
            .iter_mut()
            .zip(other.val.iter())
            .for_each(|(a, b)| *a |= *b);
    }
}
impl<T: Digit> BitOrAssign<BigUint<T>> for BigUint<T> {
    fn bitor_assign(&mut self, other: BigUint<T>) {
        *self |= &other;
    }
}
impl<T: Digit> BitOrAssign<T> for BigUint<T> {
    fn bitor_assign(&mut self, other: T) {
        *self |= BigUint::new(other)
    }
}
impl<T: Digit> BitOrAssign<&T> for BigUint<T> {
    fn bitor_assign(&mut self, other: &T) {
        *self |= BigUint::new(*other)
    }
}

impl<T: Digit> BitXor<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn bitxor(self, other: &BigUint<T>) -> BigUint<T> {
        let (big, small) = match self.cmp(other) {
            Ordering::Equal => return self.clone(),
            Ordering::Less => (other, self),
            Ordering::Greater => (self, other),
        };

        let mut ret = big.clone();
        ret ^= small;
        ret
    }
}
impl<T: Digit> BitXor<&BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn bitxor(self, other: &BigUint<T>) -> Self::Output {
        &self ^ other
    }
}
impl<T: Digit> BitXor<BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn bitxor(self, other: BigUint<T>) -> Self::Output {
        self ^ &other
    }
}
impl<T: Digit> BitXor<BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn bitxor(self, other: BigUint<T>) -> Self::Output {
        &self ^ &other
    }
}
impl<T: Digit> BitXor<T> for BigUint<T> {
    type Output = BigUint<T>;
    fn bitxor(self, other: T) -> Self::Output {
        &self ^ BigUint::new(other)
    }
}
impl<T: Digit> BitXor<&T> for BigUint<T> {
    type Output = BigUint<T>;
    fn bitxor(self, other: &T) -> Self::Output {
        &self ^ BigUint::new(*other)
    }
}
impl<T: Digit> BitXor<T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn bitxor(self, other: T) -> Self::Output {
        self ^ BigUint::new(other)
    }
}
impl<T: Digit> BitXor<&T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn bitxor(self, other: &T) -> Self::Output {
        self ^ BigUint::new(*other)
    }
}
impl<T: Digit> BitXorAssign<&BigUint<T>> for BigUint<T> {
    fn bitxor_assign(&mut self, other: &BigUint<T>) {
        self.val
            .resize(std::cmp::max(self.val.len(), other.val.len()), T::ZERO);
        self.val
            .iter_mut()
            .zip(other.val.iter())
            .for_each(|(a, b)| *a ^= *b);
        self.remove_leading_zeros();
    }
}
impl<T: Digit> BitXorAssign<BigUint<T>> for BigUint<T> {
    fn bitxor_assign(&mut self, other: BigUint<T>) {
        *self ^= &other;
    }
}
impl<T: Digit> BitXorAssign<T> for BigUint<T> {
    fn bitxor_assign(&mut self, other: T) {
        *self ^= BigUint::new(other);
    }
}
impl<T: Digit> BitXorAssign<&T> for BigUint<T> {
    fn bitxor_assign(&mut self, other: &T) {
        *self ^= BigUint::new(*other);
    }
}

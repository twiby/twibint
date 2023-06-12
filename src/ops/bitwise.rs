use core::cmp::Ordering;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

use crate::BigInt;

impl BitAnd<&BigInt> for &BigInt {
    type Output = BigInt;
    fn bitand(self, other: &BigInt) -> BigInt {
        let mut ret = BigInt::from(
            self.val
                .iter()
                .zip(other.val.iter())
                .map(|(a, b)| a & b)
                .collect::<Vec<_>>(),
        );
        ret.remove_trailing_zeros();
        ret
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
        let (big, small) = match self.cmp(other) {
            Ordering::Equal => return self.clone(),
            Ordering::Less => (other, self),
            Ordering::Greater => (self, other),
        };

        let mut ret = big.clone();
        ret.val
            .iter_mut()
            .zip(small.val.iter())
            .for_each(|(b, s)| *b |= *s);
        ret
    }
}
impl BitOrAssign<&BigInt> for BigInt {
    fn bitor_assign(&mut self, other: &BigInt) {
        *self = &*self | other;
    }
}

impl BitXor<&BigInt> for &BigInt {
    type Output = BigInt;
    fn bitxor(self, other: &BigInt) -> BigInt {
        let (big, small) = match self.cmp(other) {
            Ordering::Equal => return self.clone(),
            Ordering::Less => (other, self),
            Ordering::Greater => (self, other),
        };

        let mut ret = big.clone();
        ret.val
            .iter_mut()
            .zip(small.val.iter())
            .for_each(|(b, s)| *b ^= *s);
        ret.remove_trailing_zeros();
        ret
    }
}
impl BitXorAssign<&BigInt> for BigInt {
    fn bitxor_assign(&mut self, other: &BigInt) {
        *self = &*self ^ other;
    }
}

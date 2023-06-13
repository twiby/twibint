use core::cmp::Ordering;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

use crate::BigUint;

impl BitAnd<&BigUint> for &BigUint {
    type Output = BigUint;
    fn bitand(self, other: &BigUint) -> BigUint {
        let mut ret = BigUint::from(
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
impl BitAndAssign<&BigUint> for BigUint {
    fn bitand_assign(&mut self, other: &BigUint) {
        *self = &*self & other;
    }
}

impl BitOr<&BigUint> for &BigUint {
    type Output = BigUint;
    fn bitor(self, other: &BigUint) -> BigUint {
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
impl BitOrAssign<&BigUint> for BigUint {
    fn bitor_assign(&mut self, other: &BigUint) {
        *self = &*self | other;
    }
}

impl BitXor<&BigUint> for &BigUint {
    type Output = BigUint;
    fn bitxor(self, other: &BigUint) -> BigUint {
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
impl BitXorAssign<&BigUint> for BigUint {
    fn bitxor_assign(&mut self, other: &BigUint) {
        *self = &*self ^ other;
    }
}

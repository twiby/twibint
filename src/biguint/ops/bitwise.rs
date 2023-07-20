use core::cmp::Ordering;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

use crate::BigUint;

impl BitAnd<&BigUint> for &BigUint {
    type Output = BigUint;
    fn bitand(self, other: &BigUint) -> BigUint {
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
impl BitAndAssign<&BigUint> for BigUint {
    fn bitand_assign(&mut self, other: &BigUint) {
        self.val
            .resize(std::cmp::min(self.val.len(), other.val.len()), 0u32);
        self.val
            .iter_mut()
            .zip(other.val.iter())
            .for_each(|(a, b)| *a &= *b);
    }
}
impl BitAndAssign<BigUint> for BigUint {
    fn bitand_assign(&mut self, other: BigUint) {
        *self &= &other;
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
        ret |= small;
        ret
    }
}
impl BitOrAssign<&BigUint> for BigUint {
    fn bitor_assign(&mut self, other: &BigUint) {
        self.val
            .resize(std::cmp::max(self.val.len(), other.val.len()), 0u32);
        self.val
            .iter_mut()
            .zip(other.val.iter())
            .for_each(|(a, b)| *a |= *b);
    }
}
impl BitOrAssign<BigUint> for BigUint {
    fn bitor_assign(&mut self, other: BigUint) {
        *self |= &other;
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
        ret ^= small;
        ret
    }
}
impl BitXorAssign<&BigUint> for BigUint {
    fn bitxor_assign(&mut self, other: &BigUint) {
        self.val
            .resize(std::cmp::max(self.val.len(), other.val.len()), 0u32);
        self.val
            .iter_mut()
            .zip(other.val.iter())
            .for_each(|(a, b)| *a ^= *b);
        self.remove_trailing_zeros();
    }
}
impl BitXorAssign<BigUint> for BigUint {
    fn bitxor_assign(&mut self, other: BigUint) {
        *self ^= &other;
    }
}

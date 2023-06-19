use core::iter::Product;
use core::ops::{Mul, MulAssign};

use crate::BigUint;

impl MulAssign<u32> for BigUint {
    fn mul_assign(&mut self, other: u32) {
        *self *= other as u64;
    }
}
impl MulAssign<&u32> for BigUint {
    fn mul_assign(&mut self, other: &u32) {
        *self *= *other;
    }
}
impl Mul<u32> for &BigUint {
    type Output = BigUint;
    fn mul(self, other: u32) -> BigUint {
        let mut ret = self.clone();
        ret *= other;
        ret
    }
}
impl Mul<u32> for BigUint {
    type Output = BigUint;
    fn mul(self, other: u32) -> BigUint {
        &self * other
    }
}
impl Mul<&BigUint> for u32 {
    type Output = BigUint;
    fn mul(self, other: &BigUint) -> BigUint {
        other * self
    }
}

impl MulAssign<u64> for BigUint {
    fn mul_assign(&mut self, other: u64) {
        let mut carry = 0u64;

        for val in self.val.iter_mut() {
            let full = other * (*val as u64) + carry;
            (*val, carry) = (full as u32, full >> 32);
        }

        self.val.push(carry as u32);
        self.remove_trailing_zeros();
    }
}
impl MulAssign<&u64> for BigUint {
    fn mul_assign(&mut self, other: &u64) {
        *self *= *other;
    }
}
impl Mul<u64> for &BigUint {
    type Output = BigUint;
    fn mul(self, other: u64) -> BigUint {
        let mut ret = self.clone();
        ret *= other;
        ret
    }
}
impl Mul<u64> for BigUint {
    type Output = BigUint;
    fn mul(self, other: u64) -> BigUint {
        &self * other
    }
}
impl Mul<&BigUint> for u64 {
    type Output = BigUint;
    fn mul(self, other: &BigUint) -> BigUint {
        other * self
    }
}

impl Mul<&BigUint> for &BigUint {
    type Output = BigUint;
    fn mul(self, other: &BigUint) -> BigUint {
        let lhs = &self.val;
        let rhs = &other.val;

        let mut ret = vec![0u32; self.val.len() + other.val.len()];

        for (idx_1, b) in rhs.iter().enumerate() {
            let mut carry = 0u64;

            for (a, r) in lhs.iter().zip(&mut ret[idx_1..]) {
                let full = (*a as u64) * (*b as u64) + (*r as u64) + carry;
                (*r, carry) = (full as u32, (full >> 32));
            }

            ret[idx_1 + lhs.len()] = carry as u32;
        }

        biguint!(ret)
    }
}
impl MulAssign<&BigUint> for BigUint {
    fn mul_assign(&mut self, other: &BigUint) {
        *self = &*self * other;
    }
}
impl MulAssign<BigUint> for BigUint {
    fn mul_assign(&mut self, other: BigUint) {
        *self = &*self * &other;
    }
}
impl Mul<BigUint> for BigUint {
    type Output = BigUint;
    fn mul(self, other: BigUint) -> BigUint {
        &self * &other
    }
}

impl<T> Product<T> for BigUint
where
    BigUint: MulAssign<T>,
{
    fn product<I>(iter: I) -> BigUint
    where
        I: Iterator<Item = T>,
    {
        let mut ret = BigUint::new(1);
        for el in iter {
            ret *= el;
        }
        ret
    }
}

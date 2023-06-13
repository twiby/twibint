use core::iter::Product;
use core::ops::{Mul, MulAssign};

use crate::BigUint;

fn pure_mul(a: u32, b: u32) -> (u32, u32) {
    let full = (a as u64) * (b as u64);
    return (
        ((full << 32) >> 32).try_into().unwrap(),
        (full >> 32).try_into().unwrap(),
    );
}
#[test]
fn pure_mul_test() {
    let (a, b) = pure_mul(u32::MAX, u32::MAX);
    assert_eq!(a, 1);
    assert_eq!(b, 4294967294);
}

impl MulAssign<u32> for BigUint {
    fn mul_assign(&mut self, other: u32) {
        let mut c: bool;
        let (mut c1, mut c2, mut v): (u32, u32, u32);

        (self.val[0], c1) = pure_mul(self.val[0], other);
        for val in self.val.iter_mut().skip(1) {
            (v, c2) = pure_mul(*val, other);
            (*val, c) = v.overflowing_add(c1);
            c1 = c2 + (c as u32);
        }

        if c1 > 0 {
            self.val.push(c1);
        } else {
            self.remove_trailing_zeros();
        }
    }
}
impl MulAssign<&u32> for BigUint {
    fn mul_assign(&mut self, other: &u32) {
        *self *= *other;
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
impl Mul<&BigUint> for &BigUint {
    type Output = BigUint;
    fn mul(self, other: &BigUint) -> BigUint {
        if self.val.len() == 0 || other.val.len() == 0 {
            return BigUint::new(0);
        }

        let mut ret = BigUint::new(0);
        for i in 0..other.val.len() {
            ret += &((self * other.val[i]) << (i * 32));
        }

        ret
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

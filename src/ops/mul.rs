use core::ops::{Mul, MulAssign};

use crate::BigInt;

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

impl MulAssign<u32> for BigInt {
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
impl MulAssign<&BigInt> for BigInt {
    fn mul_assign(&mut self, other: &BigInt) {
        *self = &*self * other;
    }
}
impl Mul<u32> for &BigInt {
    type Output = BigInt;
    fn mul(self, other: u32) -> BigInt {
        let mut ret = self.clone();
        ret *= other;
        ret
    }
}
impl Mul<&BigInt> for u32 {
    type Output = BigInt;
    fn mul(self, other: &BigInt) -> BigInt {
        other * self
    }
}
impl Mul<&BigInt> for &BigInt {
    type Output = BigInt;
    fn mul(self, other: &BigInt) -> BigInt {
        if self.val.len() == 0 || other.val.len() == 0 {
            return BigInt::new(0);
        }

        let mut ret = BigInt::new(0);
        for i in 0..other.val.len() {
            ret += &((self * other.val[i]) << (i * 32));
        }

        ret
    }
}

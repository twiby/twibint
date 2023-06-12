use core::cmp::Ordering;
use core::ops::{Div, DivAssign, Rem, RemAssign};

use crate::BigInt;

trait RemDiv<T> {
    type DivOutput;
    type RemOutput;
    fn rem_div(&self, other: &T) -> Option<(Self::DivOutput, Self::RemOutput)>;
    fn div(&self, other: &T) -> Option<Self::DivOutput> {
        self.rem_div(other).map(|ret| ret.0)
    }
    fn rem(&self, other: &T) -> Option<Self::RemOutput> {
        self.rem_div(other).map(|ret| ret.1)
    }
}

impl RemDiv<u32> for BigInt {
    type DivOutput = BigInt;
    type RemOutput = u32;
    fn rem_div(&self, other: &u32) -> Option<(BigInt, u32)> {
        if *other == 0 {
            return None;
        }

        let other_64 = *other as u64;
        let mut msb = 0u64;
        let mut div: u64;

        let mut ret = BigInt::new(0);
        for idx in (0..self.val.len()).rev() {
            let lsb = self.val[idx] as u64;

            let current = (msb << 32) | lsb;
            (div, msb) = (current / other_64, current % other_64);

            ret += BigInt::from(div) << (32 * idx);
        }

        ret.remove_trailing_zeros();
        Some((ret, msb.try_into().unwrap()))
    }

    fn rem(&self, other: &u32) -> Option<u32> {
        let other_64 = *other as u64;
        let mut msb = 0u64;

        for val in self.val.iter().rev() {
            let current = (msb << 32) | (*val as u64);
            msb = current % other_64;
        }

        Some(msb.try_into().unwrap())
    }
}

impl RemAssign<u32> for BigInt {
    fn rem_assign(&mut self, other: u32) {
        let value = &*self % other;
        *self = BigInt::new(value);
    }
}
impl Rem<u32> for &BigInt {
    type Output = u32;
    fn rem(self, other: u32) -> u32 {
        RemDiv::rem(self, &other).unwrap()
    }
}

impl DivAssign<u32> for BigInt {
    fn div_assign(&mut self, other: u32) {
        *self = &*self / other;
    }
}
impl Div<u32> for &BigInt {
    type Output = BigInt;
    fn div(self, other: u32) -> BigInt {
        RemDiv::div(self, &other).unwrap()
    }
}
impl Div<&BigInt> for u32 {
    type Output = u32;
    fn div(self, other: &BigInt) -> u32 {
        if other == &BigInt::new(0) {
            panic!("Attempt at division by zero.");
        }

        let big_self = BigInt::new(self);
        match big_self.cmp(other) {
            Ordering::Equal => 1u32,
            Ordering::Less => 0u32,
            Ordering::Greater => big_self.val[0] / other.val[0],
        }
    }
}
impl DivAssign<&BigInt> for u32 {
    fn div_assign(&mut self, other: &BigInt) {
        *self = *self / other;
    }
}
impl Rem<&BigInt> for u32 {
    type Output = u32;
    fn rem(self, other: &BigInt) -> u32 {
        if other == &BigInt::new(0) {
            panic!("Attempt at division by zero.");
        }

        let big_self = BigInt::new(self);
        match big_self.cmp(other) {
            Ordering::Equal => 0u32,
            Ordering::Less => self,
            Ordering::Greater => big_self.val[0] % other.val[0],
        }
    }
}
impl RemAssign<&BigInt> for u32 {
    fn rem_assign(&mut self, other: &BigInt) {
        *self = *self / other;
    }
}

impl RemDiv<BigInt> for BigInt {
    type DivOutput = BigInt;
    type RemOutput = BigInt;
    fn rem_div(&self, other: &BigInt) -> Option<(BigInt, BigInt)> {
        if other == &BigInt::new(0) {
            return None;
        }

        match self.cmp(other) {
            Ordering::Equal => return Some((BigInt::new(1), BigInt::new(0))),
            Ordering::Less => return Some((BigInt::new(0), self.clone())),
            _ => (),
        }

        if self.val.len() == 1 {
            return Some((
                BigInt::new(self.val[0] / other),
                BigInt::new(self.val[0] % other),
            ));
        }

        assert!(self.val.len() >= other.val.len());

        let mut ret = BigInt::new(0);
        let mut remainder = BigInt::new(0);
        for idx in (0..self.val.len()).rev() {
            remainder = &(remainder << 32) ^ &BigInt::new(self.val[idx]);

            match remainder.cmp(other) {
                Ordering::Less => continue,
                Ordering::Equal => {
                    remainder -= other;
                    ret += BigInt::new(1) << 32 * idx;
                }
                Ordering::Greater => {
                    let mut quotient = 0u32;
                    let mut product = BigInt::new(0);

                    // We add to the current product power of 2 by power of 2
                    for bit in (0..32).rev() {
                        let temp = (1 << bit) * other;
                        if &product + &temp <= remainder {
                            quotient += 1 << bit;
                            product += temp;
                        }
                    }

                    remainder -= &product;
                    ret += BigInt::new(quotient) << 32 * idx;
                }
            };
        }

        Some((ret, remainder))
    }
}
impl RemAssign<&BigInt> for BigInt {
    fn rem_assign(&mut self, other: &BigInt) {
        *self = &*self % other;
    }
}
impl Rem<&BigInt> for &BigInt {
    type Output = BigInt;
    fn rem(self, other: &BigInt) -> BigInt {
        RemDiv::rem(self, other).unwrap()
    }
}
impl DivAssign<&BigInt> for BigInt {
    fn div_assign(&mut self, other: &BigInt) {
        *self = &*self / other;
    }
}
impl Div<&BigInt> for &BigInt {
    type Output = BigInt;
    fn div(self, other: &BigInt) -> BigInt {
        RemDiv::div(self, other).unwrap()
    }
}

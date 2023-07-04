use core::cmp::Ordering;
use core::ops::{Div, DivAssign, Rem, RemAssign};

use crate::errors::DivisionByZero;
use crate::traits::{DivisionResult, RemDiv};
use crate::BigUint;

impl RemDiv<u32> for BigUint {
    type DivOutput = BigUint;
    type RemOutput = u32;
    fn rem_div(&self, other: &u32) -> DivisionResult<(BigUint, u32)> {
        if *other == 0 {
            return Err(DivisionByZero());
        }

        let other_64 = *other as u64;
        let mut msb = 0u64;
        let mut div: u64;

        let mut ret = BigUint::new(0);
        for idx in (0..self.val.len()).rev() {
            let lsb = self.val[idx] as u64;

            let current = (msb << 32) | lsb;
            (div, msb) = (current / other_64, current % other_64);

            ret += BigUint::from(div) << (32 * idx);
        }

        ret.remove_trailing_zeros();
        Ok((ret, msb.try_into().unwrap()))
    }

    fn rem(&self, other: &u32) -> DivisionResult<u32> {
        if *other == 0 {
            return Err(DivisionByZero());
        }
        let other_64 = *other as u64;
        let mut msb = 0u64;

        for val in self.val.iter().rev() {
            let current = (msb << 32) | (*val as u64);
            msb = current % other_64;
        }

        Ok(msb.try_into().unwrap())
    }
}

impl RemAssign<u32> for BigUint {
    fn rem_assign(&mut self, other: u32) {
        let value = &*self % other;
        *self = BigUint::new(value);
    }
}
impl Rem<u32> for &BigUint {
    type Output = u32;
    fn rem(self, other: u32) -> u32 {
        RemDiv::rem(self, &other).unwrap()
    }
}

impl DivAssign<u32> for BigUint {
    fn div_assign(&mut self, other: u32) {
        *self = &*self / other;
    }
}
impl Div<u32> for &BigUint {
    type Output = BigUint;
    fn div(self, other: u32) -> BigUint {
        RemDiv::div(self, &other).unwrap()
    }
}
impl Div<&BigUint> for u32 {
    type Output = u32;
    fn div(self, other: &BigUint) -> u32 {
        if other == &BigUint::new(0) {
            panic!("Attempt at division by zero.");
        }

        let big_self = BigUint::new(self);
        match big_self.cmp(other) {
            Ordering::Equal => 1u32,
            Ordering::Less => 0u32,
            Ordering::Greater => big_self.val[0] / other.val[0],
        }
    }
}
impl DivAssign<&BigUint> for u32 {
    fn div_assign(&mut self, other: &BigUint) {
        *self = *self / other;
    }
}
impl Rem<&BigUint> for u32 {
    type Output = u32;
    fn rem(self, other: &BigUint) -> u32 {
        if other == &BigUint::new(0) {
            panic!("Attempt at division by zero.");
        }

        let big_self = BigUint::new(self);
        match big_self.cmp(other) {
            Ordering::Equal => 0u32,
            Ordering::Less => self,
            Ordering::Greater => big_self.val[0] % other.val[0],
        }
    }
}
impl RemAssign<&BigUint> for u32 {
    fn rem_assign(&mut self, other: &BigUint) {
        *self = *self % other;
    }
}

impl RemDiv<BigUint> for BigUint {
    type DivOutput = BigUint;
    type RemOutput = BigUint;
    fn rem_div(&self, other: &BigUint) -> DivisionResult<(BigUint, BigUint)> {
        if other == &BigUint::new(0) {
            return Err(DivisionByZero());
        }

        match self.cmp(other) {
            Ordering::Equal => return Ok((BigUint::new(1), BigUint::new(0))),
            Ordering::Less => return Ok((BigUint::new(0), self.clone())),
            _ => (),
        }

        if self.val.len() == 1 {
            return Ok((
                BigUint::new(self.val[0] / other),
                BigUint::new(self.val[0] % other),
            ));
        }

        assert!(self.val.len() >= other.val.len());

        let mut ret = BigUint::new(0);
        let mut remainder = BigUint::new(0);
        for idx in (0..self.val.len()).rev() {
            remainder = &(remainder << 32) ^ &BigUint::new(self.val[idx]);

            match remainder.cmp(other) {
                Ordering::Less => continue,
                Ordering::Equal => {
                    remainder -= other;
                    ret += BigUint::new(1) << 32 * idx;
                }
                Ordering::Greater => {
                    let mut quotient = 0u32;
                    let mut product = BigUint::new(0);

                    // We add to the current product power of 2 by power of 2
                    for bit in (0..32).rev() {
                        let temp = (1u32 << bit) * other;
                        if &product + &temp <= remainder {
                            quotient += 1 << bit;
                            product += temp;
                        }
                    }

                    remainder -= &product;
                    ret += BigUint::new(quotient) << 32 * idx;
                }
            };
        }

        Ok((ret, remainder))
    }
}
impl RemAssign<&BigUint> for BigUint {
    fn rem_assign(&mut self, other: &BigUint) {
        *self = &*self % other;
    }
}
impl Rem<&BigUint> for &BigUint {
    type Output = BigUint;
    fn rem(self, other: &BigUint) -> BigUint {
        RemDiv::rem(self, other).unwrap()
    }
}
impl DivAssign<&BigUint> for BigUint {
    fn div_assign(&mut self, other: &BigUint) {
        *self = &*self / other;
    }
}
impl Div<&BigUint> for &BigUint {
    type Output = BigUint;
    fn div(self, other: &BigUint) -> BigUint {
        RemDiv::div(self, other).unwrap()
    }
}

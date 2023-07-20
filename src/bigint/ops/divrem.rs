use core::ops::{Div, DivAssign, Rem, RemAssign};

use crate::traits::{DivisionResult, RemDiv};
use crate::{BigInt, BigUint};

impl RemDiv<u32> for BigInt {
    type DivOutput = BigInt;
    type RemOutput = u32;
    fn rem_div(&self, other: &u32) -> DivisionResult<(BigInt, u32)> {
        let (mut q, mut r) = self.uint.rem_div(other)?;

        if !self.sign {
            q += 1;
            r = other - r;
        }

        Ok((
            BigInt {
                uint: q,
                sign: self.sign,
            },
            r,
        ))
    }
    fn div(&self, other: &u32) -> DivisionResult<BigInt> {
        let mut q = RemDiv::div(&self.uint, other)?;

        if !self.sign {
            q += 1;
        }

        Ok(BigInt {
            uint: q,
            sign: self.sign,
        })
    }
    fn rem(&self, other: &u32) -> DivisionResult<u32> {
        let mut r = RemDiv::rem(&self.uint, other)?;

        if !self.sign {
            r = other - r;
        }

        Ok(r)
    }
}

impl RemDiv<i32> for BigInt {
    type DivOutput = BigInt;
    type RemOutput = i32;
    fn rem_div(&self, other: &i32) -> DivisionResult<(BigInt, i32)> {
        let (mut q, mut r) = self.uint.rem_div(&(other.abs() as u32))?;

        if self.sign ^ other.is_positive() {
            q += 1;
            r = other.abs() as u32 - r;
        }

        Ok((
            BigInt {
                uint: q,
                sign: !(self.sign ^ other.is_positive()),
            },
            if other.is_positive() {
                r as i32
            } else {
                -(r as i32)
            },
        ))
    }
    fn div(&self, other: &i32) -> DivisionResult<BigInt> {
        let mut q = RemDiv::<u32>::div(&self.uint, &other.abs().try_into().unwrap())?;

        if self.sign ^ other.is_positive() {
            q += 1;
        }

        Ok(BigInt {
            uint: q,
            sign: !(self.sign ^ other.is_positive()),
        })
    }
    fn rem(&self, other: &i32) -> DivisionResult<i32> {
        let mut r = RemDiv::<u32>::rem(&self.uint, &other.abs().try_into().unwrap())?;

        if self.sign ^ other.is_positive() {
            r = other.abs() as u32 - r;
        }

        Ok(if other.is_positive() {
            r as i32
        } else {
            -(r as i32)
        })
    }
}

impl RemDiv<BigInt> for BigInt {
    type DivOutput = BigInt;
    type RemOutput = BigInt;
    fn rem_div(&self, other: &BigInt) -> DivisionResult<(BigInt, BigInt)> {
        let (mut q, mut r) = self.uint.rem_div(&other.uint)?;

        if self.sign ^ other.sign {
            q += 1;
            r = &other.uint - &r;
        }

        Ok((
            BigInt {
                uint: q,
                sign: !(self.sign ^ other.sign),
            },
            BigInt {
                uint: r,
                sign: other.sign,
            },
        ))
    }
    fn div(&self, other: &BigInt) -> DivisionResult<BigInt> {
        let mut q = RemDiv::<BigUint<u32>>::div(&self.uint, &other.uint)?;

        if self.sign ^ other.sign {
            q += 1;
        }

        Ok(BigInt {
            uint: q,
            sign: !(self.sign ^ other.sign),
        })
    }
    fn rem(&self, other: &BigInt) -> DivisionResult<BigInt> {
        let mut r = RemDiv::<BigUint<u32>>::rem(&self.uint, &other.uint)?;

        if self.sign ^ other.sign {
            r = &other.uint - &r;
        }

        Ok(BigInt {
            uint: r,
            sign: other.sign,
        })
    }
}

impl RemAssign<u32> for BigInt {
    fn rem_assign(&mut self, other: u32) {
        let value = &*self % other;
        *self = BigInt::from(value);
    }
}
impl Rem<u32> for &BigInt {
    type Output = u32;
    fn rem(self, other: u32) -> u32 {
        RemDiv::rem(self, &other).unwrap()
    }
}
impl Rem<u32> for BigInt {
    type Output = u32;
    fn rem(self, other: u32) -> u32 {
        RemDiv::rem(&self, &other).unwrap()
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
impl Div<u32> for BigInt {
    type Output = BigInt;
    fn div(self, other: u32) -> BigInt {
        RemDiv::div(&self, &other).unwrap()
    }
}

impl RemAssign<i32> for BigInt {
    fn rem_assign(&mut self, other: i32) {
        let value = &*self % other;
        *self = BigInt::from(value);
    }
}
impl Rem<i32> for &BigInt {
    type Output = i32;
    fn rem(self, other: i32) -> i32 {
        RemDiv::rem(self, &other).unwrap()
    }
}
impl Rem<i32> for BigInt {
    type Output = i32;
    fn rem(self, other: i32) -> i32 {
        RemDiv::rem(&self, &other).unwrap()
    }
}

impl DivAssign<i32> for BigInt {
    fn div_assign(&mut self, other: i32) {
        *self = &*self / other;
    }
}
impl Div<i32> for &BigInt {
    type Output = BigInt;
    fn div(self, other: i32) -> BigInt {
        RemDiv::div(self, &other).unwrap()
    }
}
impl Div<i32> for BigInt {
    type Output = BigInt;
    fn div(self, other: i32) -> BigInt {
        RemDiv::div(&self, &other).unwrap()
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
impl Rem<BigInt> for BigInt {
    type Output = BigInt;
    fn rem(self, other: BigInt) -> BigInt {
        RemDiv::rem(&self, &other).unwrap()
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
impl Div<BigInt> for BigInt {
    type Output = BigInt;
    fn div(self, other: BigInt) -> BigInt {
        RemDiv::div(&self, &other).unwrap()
    }
}

use core::ops::{Div, DivAssign, Rem, RemAssign};

use crate::biguint::ops::divrem::RemDiv;
use crate::errors::DivisionByZero;
use crate::{BigInt, BigUint};

type DivisionResult<T> = Result<T, DivisionByZero>;

impl RemDiv<u32> for BigInt {
    type DivOutput = BigInt;
    type RemOutput = u32;
    fn rem_div(&self, other: &u32) -> DivisionResult<(BigInt, u32)> {
        let (q, r) = self.uint.rem_div(other)?;
        Ok((
            BigInt {
                uint: q,
                sign: self.sign,
            },
            r,
        ))
    }
    fn div(&self, other: &u32) -> DivisionResult<BigInt> {
        let q = RemDiv::div(&self.uint, other)?;
        Ok(BigInt {
            uint: q,
            sign: self.sign,
        })
    }
    fn rem(&self, other: &u32) -> DivisionResult<u32> {
        let r = RemDiv::rem(&self.uint, other)?;
        Ok(r)
    }
}

impl RemDiv<i32> for BigInt {
    type DivOutput = BigInt;
    type RemOutput = i32;
    fn rem_div(&self, other: &i32) -> DivisionResult<(BigInt, i32)> {
        let (q, r) = self
            .uint
            .rem_div(&<i32 as TryInto<u32>>::try_into(other.abs()).unwrap())?;
        Ok((
            BigInt {
                uint: q,
                sign: !(self.sign ^ other.is_positive()),
            },
            if self.is_sign_positive() {
                r.try_into().unwrap()
            } else {
                -<u32 as TryInto<i32>>::try_into(r).unwrap()
            },
        ))
    }
    fn div(&self, other: &i32) -> DivisionResult<BigInt> {
        let q = RemDiv::<u32>::div(&self.uint, &other.abs().try_into().unwrap())?;
        Ok(BigInt {
            uint: q,
            sign: !(self.sign ^ other.is_positive()),
        })
    }
    fn rem(&self, other: &i32) -> DivisionResult<i32> {
        let r = RemDiv::<u32>::rem(&self.uint, &other.abs().try_into().unwrap())?;
        Ok(if self.is_sign_positive() {
            r.try_into().unwrap()
        } else {
            -<u32 as TryInto<i32>>::try_into(r).unwrap()
        })
    }
}

impl RemDiv<BigInt> for BigInt {
    type DivOutput = BigInt;
    type RemOutput = BigInt;
    fn rem_div(&self, other: &BigInt) -> DivisionResult<(BigInt, BigInt)> {
        let (q, r) = self.uint.rem_div(&other.uint)?;
        Ok((
            BigInt {
                uint: q,
                sign: !(self.sign ^ other.sign),
            },
            if self.is_sign_positive() {
                r.into()
            } else {
                -<BigUint as Into<BigInt>>::into(r)
            },
        ))
    }
    fn div(&self, other: &BigInt) -> DivisionResult<BigInt> {
        let q = RemDiv::<BigUint>::div(&self.uint, &other.uint)?;
        Ok(BigInt {
            uint: q,
            sign: !(self.sign ^ other.sign),
        })
    }
    fn rem(&self, other: &BigInt) -> DivisionResult<BigInt> {
        let r = RemDiv::<BigUint>::rem(&self.uint, &other.uint)?;
        Ok(if self.is_sign_positive() {
            r.into()
        } else {
            -<BigUint as Into<BigInt>>::into(r)
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

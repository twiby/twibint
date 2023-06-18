use core::ops::{Div, DivAssign, Rem, RemAssign};

use crate::biguint::ops::divrem::RemDiv;
use crate::{BigInt, BigUint};

impl RemDiv<u32> for BigInt {
    type DivOutput = BigInt;
    type RemOutput = u32;
    fn rem_div(&self, other: &u32) -> Option<(BigInt, u32)> {
        let (q, r) = self.uint.rem_div(other)?;
        Some((
            BigInt {
                uint: q,
                sign: self.sign,
            },
            r,
        ))
    }
    fn div(&self, other: &u32) -> Option<BigInt> {
        let q = RemDiv::div(&self.uint, other)?;
        Some(BigInt {
            uint: q,
            sign: self.sign,
        })
    }
    fn rem(&self, other: &u32) -> Option<u32> {
        let r = RemDiv::rem(&self.uint, other)?;
        Some(r)
    }
}

impl RemDiv<i32> for BigInt {
    type DivOutput = BigInt;
    type RemOutput = i32;
    fn rem_div(&self, other: &i32) -> Option<(BigInt, i32)> {
        let (q, r) = self
            .uint
            .rem_div(&<i32 as TryInto<u32>>::try_into(other.abs()).unwrap())?;
        Some((
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
    fn div(&self, other: &i32) -> Option<BigInt> {
        let q = RemDiv::<u32>::div(&self.uint, &other.abs().try_into().unwrap())?;
        Some(BigInt {
            uint: q,
            sign: !(self.sign ^ other.is_positive()),
        })
    }
    fn rem(&self, other: &i32) -> Option<i32> {
        let r = RemDiv::<u32>::rem(&self.uint, &other.abs().try_into().unwrap())?;
        Some(if self.is_sign_positive() {
            r.try_into().unwrap()
        } else {
            -<u32 as TryInto<i32>>::try_into(r).unwrap()
        })
    }
}

impl RemDiv<BigInt> for BigInt {
    type DivOutput = BigInt;
    type RemOutput = BigInt;
    fn rem_div(&self, other: &BigInt) -> Option<(BigInt, BigInt)> {
        let (q, r) = self.uint.rem_div(&other.uint)?;
        Some((
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
    fn div(&self, other: &BigInt) -> Option<BigInt> {
        let q = RemDiv::<BigUint>::div(&self.uint, &other.uint)?;
        Some(BigInt {
            uint: q,
            sign: !(self.sign ^ other.sign),
        })
    }
    fn rem(&self, other: &BigInt) -> Option<BigInt> {
        let r = RemDiv::<BigUint>::rem(&self.uint, &other.uint)?;
        Some(if self.is_sign_positive() {
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

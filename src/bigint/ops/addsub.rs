use core::cmp::Ordering;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::BigInt;

impl Add<u32> for &BigInt {
    type Output = BigInt;

    fn add(self, other: u32) -> Self::Output {
        let mut ret: BigInt = self.clone();
        ret += other;
        return ret;
    }
}
impl Add<i32> for &BigInt {
    type Output = BigInt;

    fn add(self, other: i32) -> Self::Output {
        let mut ret: BigInt = self.clone();
        ret += other;
        return ret;
    }
}
impl Add<&BigInt> for u32 {
    type Output = BigInt;
    fn add(self, other: &BigInt) -> Self::Output {
        other + self
    }
}
impl Add<&BigInt> for i32 {
    type Output = BigInt;
    fn add(self, other: &BigInt) -> Self::Output {
        other + self
    }
}
impl Add<&BigInt> for &BigInt {
    type Output = BigInt;

    fn add(self, other: &BigInt) -> Self::Output {
        let mut ret = self.clone();
        ret += other;
        return ret;
    }
}
impl Add<BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, other: BigInt) -> Self::Output {
        &self + &other
    }
}

impl AddAssign<u32> for BigInt {
    fn add_assign(&mut self, other: u32) {
        let other = BigInt::from(other);
        *self += other;
    }
}
impl AddAssign<&u32> for BigInt {
    fn add_assign(&mut self, other: &u32) {
        *self += *other;
    }
}
impl AddAssign<i32> for BigInt {
    fn add_assign(&mut self, other: i32) {
        let other = BigInt::from(other);
        *self += other;
    }
}
impl AddAssign<&i32> for BigInt {
    fn add_assign(&mut self, other: &i32) {
        *self += *other;
    }
}

impl AddAssign<BigInt> for BigInt {
    fn add_assign(&mut self, other: BigInt) {
        *self += &other;
    }
}
impl AddAssign<&BigInt> for BigInt {
    fn add_assign(&mut self, other: &BigInt) {
        // Case same sign: pure addition of components
        match (self.sign, other.sign) {
            (some_bool, other_bool) if some_bool == other_bool => {
                self.uint += &other.uint;
                return;
            }
            _ => (),
        };

        match self.uint.cmp(&other.uint) {
            Ordering::Equal => *self = BigInt::default(),
            Ordering::Greater => self.uint -= &other.uint,
            Ordering::Less => {
                self.uint = &other.uint - &self.uint;
                self.sign = !self.sign;
            }
        }
    }
}

impl SubAssign<u32> for BigInt {
    fn sub_assign(&mut self, other: u32) {
        *self -= &BigInt::from(other);
    }
}
impl SubAssign<&BigInt> for BigInt {
    fn sub_assign(&mut self, other: &BigInt) {
        *self += -other;
    }
}
impl Sub<u32> for &BigInt {
    type Output = BigInt;
    fn sub(self, other: u32) -> BigInt {
        let mut ret = self.clone();
        ret -= other;
        return ret;
    }
}
impl Sub<&BigInt> for &BigInt {
    type Output = BigInt;
    fn sub(self, other: &BigInt) -> BigInt {
        let mut ret = self.clone();
        ret -= other;
        ret
    }
}
impl Sub<BigInt> for BigInt {
    type Output = BigInt;
    fn sub(self, other: BigInt) -> BigInt {
        &self - &other
    }
}

impl<T> Sum<T> for BigInt
where
    BigInt: AddAssign<T>,
{
    fn sum<I>(iter: I) -> BigInt
    where
        I: Iterator<Item = T>,
    {
        let mut ret = BigInt::new(0);
        for el in iter {
            ret += el;
        }
        ret
    }
}

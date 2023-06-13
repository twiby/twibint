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
impl Add<&BigInt> for u32 {
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
        let other = BigInt::new(other);
        *self += other;
    }
}
impl AddAssign<&u32> for BigInt {
    fn add_assign(&mut self, other: &u32) {
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
        while self.val.len() < other.val.len() {
            self.val.push(0);
        }

        let mut partial_carry_1: bool;
        let mut partial_carry_2: bool;
        let mut full_carry = false;

        for (a, b) in self.val.iter_mut().zip(other.val.iter()) {
            (*a, partial_carry_1) = a.overflowing_add(*b);
            (*a, partial_carry_2) = a.overflowing_add(full_carry as u32);
            full_carry = partial_carry_1 | partial_carry_2;
        }

        for val in self.val.iter_mut().skip(other.val.len()) {
            (*val, full_carry) = val.overflowing_add(full_carry as u32);
        }

        if full_carry {
            self.val.push(1);
        } else {
            self.remove_trailing_zeros();
        }
    }
}

impl SubAssign<u32> for BigInt {
    fn sub_assign(&mut self, other: u32) {
        *self -= &BigInt::new(other);
    }
}
impl SubAssign<&BigInt> for BigInt {
    fn sub_assign(&mut self, other: &BigInt) {
        if &*self < other {
            panic!("Attempt at subtraction with underflow");
        }

        let mut partial_carry_1: bool;
        let mut partial_carry_2: bool;
        let mut carry = false;
        for (a, b) in self.val.iter_mut().zip(other.val.iter()) {
            (*a, partial_carry_1) = a.overflowing_sub(*b);
            (*a, partial_carry_2) = a.overflowing_sub(carry as u32);
            carry = partial_carry_1 | partial_carry_2;
        }

        for val in self.val.iter_mut().skip(other.val.len()) {
            (*val, carry) = val.overflowing_sub(carry as u32);
        }

        self.remove_trailing_zeros();
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

use core::iter::Sum;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::BigUint;

impl Add<u32> for &BigUint {
    type Output = BigUint;

    fn add(self, other: u32) -> Self::Output {
        let mut ret: BigUint = self.clone();
        ret += other;
        return ret;
    }
}
impl Add<&BigUint> for u32 {
    type Output = BigUint;
    fn add(self, other: &BigUint) -> Self::Output {
        other + self
    }
}
impl Add<&BigUint> for &BigUint {
    type Output = BigUint;

    fn add(self, other: &BigUint) -> Self::Output {
        let mut ret = self.clone();
        ret += other;
        return ret;
    }
}
impl Add<BigUint> for BigUint {
    type Output = BigUint;

    fn add(self, other: BigUint) -> Self::Output {
        &self + &other
    }
}

impl AddAssign<u32> for BigUint {
    fn add_assign(&mut self, other: u32) {
        let other = BigUint::new(other);
        *self += other;
    }
}
impl AddAssign<&u32> for BigUint {
    fn add_assign(&mut self, other: &u32) {
        *self += *other;
    }
}

impl AddAssign<BigUint> for BigUint {
    fn add_assign(&mut self, other: BigUint) {
        *self += &other;
    }
}
impl AddAssign<&BigUint> for BigUint {
    fn add_assign(&mut self, other: &BigUint) {
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

impl SubAssign<u32> for BigUint {
    fn sub_assign(&mut self, other: u32) {
        *self -= &BigUint::new(other);
    }
}
impl SubAssign<BigUint> for BigUint {
    fn sub_assign(&mut self, other: BigUint) {
        *self -= &other;
    }
}
impl SubAssign<&BigUint> for BigUint {
    fn sub_assign(&mut self, other: &BigUint) {
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
impl Sub<u32> for &BigUint {
    type Output = BigUint;
    fn sub(self, other: u32) -> BigUint {
        let mut ret = self.clone();
        ret -= other;
        return ret;
    }
}
impl Sub<&BigUint> for &BigUint {
    type Output = BigUint;
    fn sub(self, other: &BigUint) -> BigUint {
        let mut ret = self.clone();
        ret -= other;
        ret
    }
}

impl<T> Sum<T> for BigUint
where
    BigUint: AddAssign<T>,
{
    fn sum<I>(iter: I) -> BigUint
    where
        I: Iterator<Item = T>,
    {
        let mut ret = BigUint::new(0);
        for el in iter {
            ret += el;
        }
        ret
    }
}

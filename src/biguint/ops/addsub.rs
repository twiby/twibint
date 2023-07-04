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
        if self.val.len() < other.val.len() {
            self.val.resize(other.val.len(), 0u32)
        }

        let carry = super::implem_choices::add_assign(&mut self.val, &other.val);

        self.val.push(carry as u32);
        self.remove_trailing_zeros();
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

        super::implem_choices::sub_assign(&mut self.val, &other.val);

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

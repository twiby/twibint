use core::cmp::Ordering;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::traits::Digit;
use crate::{BigInt, BigUint};

impl<T: Digit> BigInt<T> {
    fn clone_for_addition_with(&self, other_len: usize) -> Self {
        let uint = self.uint.clone_for_addition_with(other_len);
        BigInt {
            uint,
            sign: self.sign,
        }
    }
}

impl<T: Digit> Add<T> for BigInt<T> {
    type Output = BigInt<T>;

    fn add(mut self, other: T) -> Self::Output {
        self += other;
        self
    }
}
impl<T: Digit> Add<&T> for BigInt<T> {
    type Output = BigInt<T>;

    fn add(mut self, other: &T) -> Self::Output {
        self += other;
        self
    }
}
impl<T: Digit> Add<T> for &BigInt<T> {
    type Output = BigInt<T>;

    fn add(self, other: T) -> Self::Output {
        let mut ret: BigInt<T> = self.clone_for_addition_with(1);
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<&T> for &BigInt<T> {
    type Output = BigInt<T>;

    fn add(self, other: &T) -> Self::Output {
        let mut ret: BigInt<T> = self.clone_for_addition_with(1);
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<&BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;

    fn add(self, other: &BigInt<T>) -> Self::Output {
        let mut ret = self.clone_for_addition_with(other.uint.val.len());
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;

    fn add(self, mut other: BigInt<T>) -> Self::Output {
        other += self;
        other
    }
}
impl<T: Digit> Add<BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn add(mut self, other: BigInt<T>) -> Self::Output {
        self += other;
        self
    }
}
impl<T: Digit> Add<&BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn add(mut self, other: &BigInt<T>) -> Self::Output {
        self += other;
        self
    }
}

impl<T: Digit> AddAssign<T> for BigInt<T> {
    fn add_assign(&mut self, other: T) {
        let other = BigInt::<T>::from(BigUint::<T>::new(other));
        *self += other;
    }
}
impl<T: Digit> AddAssign<&T> for BigInt<T> {
    fn add_assign(&mut self, other: &T) {
        *self += *other;
    }
}

impl<T: Digit> AddAssign<BigInt<T>> for BigInt<T> {
    fn add_assign(&mut self, other: BigInt<T>) {
        *self += &other;
    }
}
impl<T: Digit> AddAssign<&BigInt<T>> for BigInt<T> {
    fn add_assign(&mut self, other: &BigInt<T>) {
        // Case same sign: pure addition of components
        if self.sign == other.sign {
            self.uint += &other.uint;
            return;
        }

        match self.uint.cmp(&other.uint) {
            Ordering::Equal => {
                self.uint.val.clear();
                self.uint.val.push(T::ZERO);
            }
            Ordering::Greater => self.uint -= &other.uint,
            Ordering::Less => {
                self.uint = &other.uint - &self.uint;
                self.sign = !self.sign;
            }
        }
    }
}

impl<T: Digit> SubAssign<&T> for BigInt<T> {
    fn sub_assign(&mut self, other: &T) {
        *self -= *other;
    }
}
impl<T: Digit> SubAssign<T> for BigInt<T> {
    fn sub_assign(&mut self, other: T) {
        *self -= &BigInt::<T>::from(BigUint::<T>::new(other));
    }
}
impl<T: Digit> SubAssign<&BigInt<T>> for BigInt<T> {
    fn sub_assign(&mut self, other: &BigInt<T>) {
        if self.sign != other.sign {
            self.uint += &other.uint;
            return;
        }

        match self.uint.cmp(&other.uint) {
            Ordering::Equal => {
                self.uint.val.clear();
                self.uint.val.push(T::ZERO);
            }
            Ordering::Greater => self.uint -= &other.uint,
            Ordering::Less => {
                self.uint = &other.uint - &self.uint;
                self.sign = !self.sign;
            }
        }
    }
}
impl<T: Digit> SubAssign<BigInt<T>> for BigInt<T> {
    fn sub_assign(&mut self, other: BigInt<T>) {
        *self -= &other;
    }
}
impl<T: Digit> Sub<T> for &BigInt<T> {
    type Output = BigInt<T>;
    fn sub(self, other: T) -> BigInt<T> {
        let mut ret = self.clone_for_addition_with(1);
        ret -= other;
        return ret;
    }
}
impl<T: Digit> Sub<&T> for &BigInt<T> {
    type Output = BigInt<T>;
    fn sub(self, other: &T) -> BigInt<T> {
        let mut ret = self.clone_for_addition_with(1);
        ret -= other;
        return ret;
    }
}
impl<T: Digit> Sub<T> for BigInt<T> {
    type Output = BigInt<T>;
    fn sub(mut self, other: T) -> BigInt<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<&T> for BigInt<T> {
    type Output = BigInt<T>;
    fn sub(mut self, other: &T) -> BigInt<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<&BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn sub(self, other: &BigInt<T>) -> BigInt<T> {
        let mut ret = self.clone_for_addition_with(other.uint.val.len());
        ret -= other;
        ret
    }
}
impl<T: Digit> Sub<BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn sub(mut self, other: BigInt<T>) -> BigInt<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<&BigInt<T>> for BigInt<T> {
    type Output = BigInt<T>;
    fn sub(mut self, other: &BigInt<T>) -> BigInt<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<BigInt<T>> for &BigInt<T> {
    type Output = BigInt<T>;
    fn sub(self, mut other: BigInt<T>) -> BigInt<T> {
        other -= self;
        -other
    }
}

impl<T, D: Digit> Sum<T> for BigInt<D>
where
    BigInt<D>: AddAssign<T>,
{
    fn sum<I>(iter: I) -> BigInt<D>
    where
        I: Iterator<Item = T>,
    {
        let mut ret = BigInt::<D>::default();
        for el in iter {
            ret += el;
        }
        ret
    }
}

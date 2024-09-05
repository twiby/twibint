use core::cmp::Ordering;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::traits::Digit;
use crate::BigInt;

impl<T: Digit> BigInt<T> {
    fn clone_for_addition_with(&self, other_len: usize) -> Self {
        let uint = self.uint.clone_for_addition_with(other_len);
        BigInt {
            uint,
            sign: self.sign,
        }
    }

    #[inline]
    pub(crate) fn add_assign(&mut self, other_sign: bool, other: &[T]) {
        // Case same sign: pure addition of components
        if self.sign == other_sign {
            self.uint.add_assign(other);
            return;
        }

        match self.uint.ord(other) {
            Ordering::Equal => {
                self.uint.val.clear();
                self.uint.val.push(T::ZERO);
            }
            Ordering::Greater => self.uint.sub_assign(other),
            Ordering::Less => {
                self.uint._rsub_assign(other);
                self.sign = !self.sign;
            }
        }
    }

    #[inline]
    pub(crate) fn sub_assign(&mut self, other_sign: bool, other: &[T]) {
        self.add_assign(!other_sign, other);
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
        self.add_assign(true, &[other])
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
        self.add_assign(other.sign, &other.uint.val);
    }
}

impl<T: Digit> SubAssign<&T> for BigInt<T> {
    fn sub_assign(&mut self, other: &T) {
        *self -= *other;
    }
}
impl<T: Digit> SubAssign<T> for BigInt<T> {
    fn sub_assign(&mut self, other: T) {
        self.sub_assign(true, &[other]);
    }
}
impl<T: Digit> SubAssign<&BigInt<T>> for BigInt<T> {
    fn sub_assign(&mut self, other: &BigInt<T>) {
        self.sub_assign(other.sign, &other.uint.val);
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

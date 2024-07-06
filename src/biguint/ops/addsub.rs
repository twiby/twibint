use core::iter::Sum;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::traits::Digit;
use crate::BigUint;

impl<T: Digit> BigUint<T> {
    pub(crate) fn clone_for_addition_with(&self, other_len: usize) -> Self {
        let mut data = Vec::<T>::with_capacity(self.val.len().max(other_len) + 1);
        data.extend_from_slice(&self.val);
        BigUint::<T> { val: data }
    }
}

impl<T: Digit> Add<T> for &BigUint<T> {
    type Output = BigUint<T>;

    fn add(self, other: T) -> Self::Output {
        let mut ret: BigUint<T> = self.clone_for_addition_with(1);
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<T> for BigUint<T> {
    type Output = BigUint<T>;

    fn add(mut self, other: T) -> Self::Output {
        self += &other;
        self
    }
}

impl<T: Digit> Add<&T> for &BigUint<T> {
    type Output = BigUint<T>;

    fn add(self, other: &T) -> Self::Output {
        let mut ret: BigUint<T> = self.clone_for_addition_with(1);
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<&T> for BigUint<T> {
    type Output = BigUint<T>;

    fn add(mut self, other: &T) -> Self::Output {
        self += other;
        self
    }
}
impl<T: Digit> Add<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;

    fn add(self, other: &BigUint<T>) -> Self::Output {
        let mut ret = self.clone_for_addition_with(other.val.len());
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;

    fn add(mut self, other: BigUint<T>) -> Self::Output {
        self += &other;
        self
    }
}
impl<T: Digit> Add<&BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;

    fn add(mut self, other: &BigUint<T>) -> Self::Output {
        self += other;
        self
    }
}
impl<T: Digit> Add<BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;

    fn add(self, mut other: BigUint<T>) -> Self::Output {
        other += self;
        other
    }
}

impl<T: Digit> AddAssign<T> for BigUint<T> {
    fn add_assign(&mut self, other: T) {
        let other = BigUint::<T>::new(other);
        *self += other;
    }
}
impl<T: Digit> AddAssign<&T> for BigUint<T> {
    fn add_assign(&mut self, other: &T) {
        *self += *other;
    }
}

impl<T: Digit> AddAssign<BigUint<T>> for BigUint<T> {
    fn add_assign(&mut self, other: BigUint<T>) {
        *self += &other;
    }
}
impl<T: Digit> AddAssign<&BigUint<T>> for BigUint<T> {
    fn add_assign(&mut self, other: &BigUint<T>) {
        let target_length = self.val.len().max(other.val.len()) + 1;
        self.val.resize(target_length, T::ZERO);

        let carry = super::implem_choices::add_assign(&mut self.val, &other.val);
        debug_assert!(!carry);

        self.remove_trailing_zeros();
    }
}

impl<T: Digit> SubAssign<T> for BigUint<T> {
    fn sub_assign(&mut self, other: T) {
        *self -= &BigUint::<T>::new(other);
    }
}
impl<T: Digit> SubAssign<&T> for BigUint<T> {
    fn sub_assign(&mut self, other: &T) {
        *self -= *other;
    }
}
impl<T: Digit> SubAssign<BigUint<T>> for BigUint<T> {
    fn sub_assign(&mut self, other: BigUint<T>) {
        *self -= &other;
    }
}
impl<T: Digit> SubAssign<&BigUint<T>> for BigUint<T> {
    fn sub_assign(&mut self, other: &BigUint<T>) {
        if &*self < other {
            panic!("Attempt at subtraction with underflow");
        }

        super::implem_choices::sub_assign(&mut self.val, &other.val);

        self.remove_trailing_zeros();
    }
}
impl<T: Digit> Sub<T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn sub(self, other: T) -> BigUint<T> {
        let mut ret = self.clone();
        ret -= other;
        return ret;
    }
}
impl<T: Digit> Sub<&T> for &BigUint<T> {
    type Output = BigUint<T>;
    fn sub(self, other: &T) -> BigUint<T> {
        let mut ret = self.clone();
        ret -= other;
        return ret;
    }
}
impl<T: Digit> Sub<T> for BigUint<T> {
    type Output = BigUint<T>;
    fn sub(mut self, other: T) -> BigUint<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<&T> for BigUint<T> {
    type Output = BigUint<T>;
    fn sub(mut self, other: &T) -> BigUint<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn sub(mut self, other: BigUint<T>) -> BigUint<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<&BigUint<T>> for BigUint<T> {
    type Output = BigUint<T>;
    fn sub(mut self, other: &BigUint<T>) -> BigUint<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn sub(self, other: BigUint<T>) -> BigUint<T> {
        self - &other
    }
}
impl<T: Digit> Sub<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;
    fn sub(self, other: &BigUint<T>) -> BigUint<T> {
        let mut ret = self.clone();
        ret -= other;
        ret
    }
}

impl<T, T2> Sum<T> for BigUint<T2>
where
    T2: Digit,
    BigUint<T2>: AddAssign<T>,
{
    fn sum<I>(iter: I) -> BigUint<T2>
    where
        I: Iterator<Item = T>,
    {
        let mut ret = BigUint::<T2>::new(T2::ZERO);
        for el in iter {
            ret += el;
        }
        ret
    }
}

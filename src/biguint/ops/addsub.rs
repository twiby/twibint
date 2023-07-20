use core::iter::Sum;
use core::ops::{Add, AddAssign, Sub, SubAssign};

use crate::traits::Digit;
use crate::BigUint;

impl<T: Digit> Add<T> for &BigUint<T> {
    type Output = BigUint<T>;

    fn add(self, other: T) -> Self::Output {
        let mut ret: BigUint<T> = self.clone();
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<&BigUint<T>> for &BigUint<T> {
    type Output = BigUint<T>;

    fn add(self, other: &BigUint<T>) -> Self::Output {
        let mut ret = self.clone();
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
impl<T: Digit> Add<T> for BigUint<T> {
    type Output = BigUint<T>;

    fn add(mut self, other: T) -> Self::Output {
        self += &other;
        self
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
        if self.val.len() < other.val.len() {
            self.val.resize(other.val.len(), T::ZERO)
        }

        let carry = super::implem_choices::add_assign(&mut self.val, &other.val);

        self.val.push(T::from(carry));
        self.remove_trailing_zeros();
    }
}

impl<T: Digit> SubAssign<T> for BigUint<T> {
    fn sub_assign(&mut self, other: T) {
        *self -= &BigUint::<T>::new(other);
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

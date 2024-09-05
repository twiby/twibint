use crate::biguint::ops::add_assign;
use crate::biguint::ops::rsub_assign;
use crate::biguint::ops::sub_assign;
use crate::traits::Digit;
use crate::BigInt;
use std::cmp::Ordering;
use std::iter::Sum;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::BigFloat;

impl<T: Digit> BigFloat<T> {
    fn clone_for_addition_with(&self, other_len: usize) -> Self {
        let uint = self.int.uint.clone_for_addition_with(other_len);
        BigFloat {
            scale: self.scale,
            int: BigInt {
                uint,
                sign: self.int.sign,
            },
        }
    }

    fn unsigned_add_bigger_scale(&mut self, other_scale: isize, other: &[T]) {
        let scale_diff = other_scale - self.scale;
        assert!(scale_diff >= 0);
        let scale_diff = scale_diff as usize;

        let target_length = self.int.uint.val.len().max(scale_diff + other.len()) + 1;
        self.int.uint.val.resize(target_length, T::ZERO);

        let carry = add_assign(&mut self.int.uint.val[scale_diff..], other);
        debug_assert!(!carry);

        self.int.uint.remove_leading_zeros();
        self.simplify();
    }

    fn unsigned_add_smaller_scale(&mut self, other_scale: isize, other: &[T]) {
        let scale_diff = self.scale - other_scale;
        assert!(scale_diff > 0);
        let scale_diff = scale_diff as usize;

        let target_length = self.int.uint.val.len().max(scale_diff + other.len()) + 1;
        let shift = target_length - self.int.uint.val.len();
        self.int <<= shift * T::NB_BITS;
        self.scale -= shift as isize;
        self.unsigned_add_bigger_scale(other_scale, other);
    }

    fn unsigned_add(&mut self, other_scale: isize, other: &[T]) {
        if self.scale > other_scale {
            self.unsigned_add_smaller_scale(other_scale, other);
        } else {
            self.unsigned_add_bigger_scale(other_scale, other);
        }
    }

    fn unsigned_sub_bigger_scale(&mut self, other_scale: isize, other: &[T]) {
        let scale_diff = other_scale - self.scale;
        assert!(scale_diff >= 0);
        let scale_diff = scale_diff as usize;

        let carry = sub_assign(&mut self.int.uint.val[scale_diff..], other);
        debug_assert!(!carry);

        self.int.uint.remove_leading_zeros();
        self.simplify();
    }

    fn unsigned_sub_smaller_scale(&mut self, other_scale: isize, other: &[T]) {
        let scale_diff = self.scale - other_scale;
        assert!(scale_diff > 0);
        let scale_diff = scale_diff as usize;

        let target_length = self.int.uint.val.len().max(scale_diff + other.len()) + 1;
        let shift = target_length - self.int.uint.val.len();
        self.int <<= shift * T::NB_BITS;
        self.scale -= shift as isize;
        self.unsigned_sub_bigger_scale(other_scale, other);
    }

    fn unsigned_sub(&mut self, other_scale: isize, other: &[T]) {
        if self.scale > other_scale {
            self.unsigned_sub_smaller_scale(other_scale, other);
        } else {
            self.unsigned_sub_bigger_scale(other_scale, other);
        }
    }

    fn unsigned_rsub_bigger_scale(&mut self, other_scale: isize, other: &[T]) {
        let scale_diff = other_scale - self.scale;
        assert!(scale_diff >= 0);
        let scale_diff = scale_diff as usize;

        let prev_len = if self.int.uint.val.len() >= scale_diff {
            self.int.uint.val.len() - scale_diff
        } else {
            0
        };

        self.int.uint.val.resize(other.len() + scale_diff, T::ZERO);
        let carry = rsub_assign(&mut self.int.uint.val[scale_diff..], other, prev_len);
        debug_assert!(!carry);

        // We need an addition correction for digits "below" the subtraction
        let mut remaining = T::ONE;
        for d in &mut self.int.uint.val[..scale_diff] {
            *d = (T::MAX - *d).wrapping_add(remaining);
            remaining = T::from_bool(*d == T::ZERO && !(remaining == T::ZERO));
        }

        if remaining == T::ZERO {
            sub_assign(&mut self.int.uint.val[scale_diff..], &[T::ONE]);
        }

        self.int.uint.remove_leading_zeros();
        self.simplify();
    }

    fn unsigned_rsub_smaller_scale(&mut self, other_scale: isize, other: &[T]) {
        let scale_diff = self.scale - other_scale;
        assert!(scale_diff > 0);
        let scale_diff = scale_diff as usize;

        let target_length = self.int.uint.val.len().max(scale_diff + other.len()) + 1;
        let shift = target_length - self.int.uint.val.len();
        self.int <<= shift * T::NB_BITS;
        self.scale -= shift as isize;
        self.unsigned_rsub_bigger_scale(other_scale, other);
    }

    fn unsigned_rsub(&mut self, other_scale: isize, other: &[T]) {
        if self.scale > other_scale {
            self.unsigned_rsub_smaller_scale(other_scale, other);
        } else {
            self.unsigned_rsub_bigger_scale(other_scale, other);
        }
    }

    pub(crate) fn add_assign(&mut self, other_sign: bool, other_scale: isize, other: &[T]) {
        if self.int.sign == other_sign {
            self.unsigned_add(other_scale, other);
            return;
        }

        match self.float_unsigned_ord(other_scale, other) {
            Ordering::Equal => {
                self.int.uint.val.clear();
                self.int.uint.val.push(T::ZERO);
                self.int.sign = true;
                self.scale = 0;
            }
            Ordering::Greater => self.unsigned_sub(other_scale, other),
            Ordering::Less => {
                self.int.sign = !self.int.sign;
                self.unsigned_rsub(other_scale, other);
            }
        }
    }

    #[inline]
    pub(crate) fn sub_assign(&mut self, other_sign: bool, other_scale: isize, other: &[T]) {
        self.add_assign(!other_sign, other_scale, other);
    }
}

impl<T: Digit> Add<T> for BigFloat<T> {
    type Output = BigFloat<T>;

    fn add(mut self, other: T) -> Self::Output {
        self += other;
        self
    }
}
impl<T: Digit> Add<&T> for BigFloat<T> {
    type Output = BigFloat<T>;

    fn add(mut self, other: &T) -> Self::Output {
        self += other;
        self
    }
}
impl<T: Digit> Add<T> for &BigFloat<T> {
    type Output = BigFloat<T>;

    fn add(self, other: T) -> Self::Output {
        let mut ret: BigFloat<T> = self.clone_for_addition_with(1);
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<&T> for &BigFloat<T> {
    type Output = BigFloat<T>;

    fn add(self, other: &T) -> Self::Output {
        let mut ret: BigFloat<T> = self.clone_for_addition_with(1);
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<&BigFloat<T>> for &BigFloat<T> {
    type Output = BigFloat<T>;

    fn add(self, other: &BigFloat<T>) -> Self::Output {
        let mut ret = self.clone_for_addition_with(other.int.uint.val.len());
        ret += other;
        return ret;
    }
}
impl<T: Digit> Add<BigFloat<T>> for &BigFloat<T> {
    type Output = BigFloat<T>;

    fn add(self, mut other: BigFloat<T>) -> Self::Output {
        other += self;
        other
    }
}
impl<T: Digit> Add<BigFloat<T>> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn add(mut self, other: BigFloat<T>) -> Self::Output {
        self += other;
        self
    }
}
impl<T: Digit> Add<&BigFloat<T>> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn add(mut self, other: &BigFloat<T>) -> Self::Output {
        self += other;
        self
    }
}

impl<T: Digit> AddAssign<T> for BigFloat<T> {
    fn add_assign(&mut self, other: T) {
        self.add_assign(true, 0, &[other])
    }
}
impl<T: Digit> AddAssign<&T> for BigFloat<T> {
    fn add_assign(&mut self, other: &T) {
        *self += *other;
    }
}

impl<T: Digit> AddAssign<BigFloat<T>> for BigFloat<T> {
    fn add_assign(&mut self, other: BigFloat<T>) {
        *self += &other;
    }
}
impl<T: Digit> AddAssign<&BigFloat<T>> for BigFloat<T> {
    fn add_assign(&mut self, other: &BigFloat<T>) {
        println!("ADD ASSIGN");
        println!("{:?}", self);
        println!("{:?}", other);
        self.add_assign(other.int.sign, other.scale, &other.int.uint.val);
        println!("{:?}", self);
    }
}

impl<T: Digit> SubAssign<&T> for BigFloat<T> {
    fn sub_assign(&mut self, other: &T) {
        *self -= *other;
    }
}
impl<T: Digit> SubAssign<T> for BigFloat<T> {
    fn sub_assign(&mut self, other: T) {
        self.sub_assign(true, 0, &[other]);
    }
}
impl<T: Digit> SubAssign<&BigFloat<T>> for BigFloat<T> {
    fn sub_assign(&mut self, other: &BigFloat<T>) {
        self.sub_assign(other.int.sign, other.scale, &other.int.uint.val);
    }
}
impl<T: Digit> SubAssign<BigFloat<T>> for BigFloat<T> {
    fn sub_assign(&mut self, other: BigFloat<T>) {
        *self -= &other;
    }
}
impl<T: Digit> Sub<T> for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn sub(self, other: T) -> BigFloat<T> {
        let mut ret = self.clone_for_addition_with(1);
        ret -= other;
        return ret;
    }
}
impl<T: Digit> Sub<&T> for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn sub(self, other: &T) -> BigFloat<T> {
        let mut ret = self.clone_for_addition_with(1);
        ret -= other;
        return ret;
    }
}
impl<T: Digit> Sub<T> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn sub(mut self, other: T) -> BigFloat<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<&T> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn sub(mut self, other: &T) -> BigFloat<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<&BigFloat<T>> for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn sub(self, other: &BigFloat<T>) -> BigFloat<T> {
        let mut ret = self.clone_for_addition_with(other.int.uint.val.len());
        ret -= other;
        ret
    }
}
impl<T: Digit> Sub<BigFloat<T>> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn sub(mut self, other: BigFloat<T>) -> BigFloat<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<&BigFloat<T>> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn sub(mut self, other: &BigFloat<T>) -> BigFloat<T> {
        self -= other;
        self
    }
}
impl<T: Digit> Sub<BigFloat<T>> for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn sub(self, mut other: BigFloat<T>) -> BigFloat<T> {
        other -= self;
        -other
    }
}

impl<T, D: Digit> Sum<T> for BigFloat<D>
where
    BigFloat<D>: AddAssign<T>,
{
    fn sum<I>(iter: I) -> BigFloat<D>
    where
        I: Iterator<Item = T>,
    {
        let mut ret = BigFloat::<D>::default();
        for el in iter {
            ret += el;
        }
        ret
    }
}

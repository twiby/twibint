//! This module introduces some ergonomics for multiplying floats
//! when only caring for some amount of most significant digits.

use crate::traits::Digit;
use crate::BigFloat;
use crate::BigUint;

/// Meant to only be implemented by BigFloat, for the prupose of the
/// Newton-Raphson division
pub(super) trait SmartMul<T: Digit> {
    type MSDRep<'a>
    where
        Self: 'a;
    fn msd(&self, nb_digits: usize) -> Self::MSDRep<'_>;
    fn smart_mul<A: OpArgument<T>, B: OpArgument<T>>(&mut self, _a: A, _b: B) {
        unimplemented!()
    }
    fn smart_add_assign<A: OpArgument<T>>(&mut self, _a: A) {
        unimplemented!()
    }
}

impl<T: Digit> SmartMul<T> for BigFloat<T> {
    type MSDRep<'a> = BigFloatMSD<'a, T>;

    #[inline]
    fn msd(&self, nb_digits: usize) -> BigFloatMSD<'_, T> {
        BigFloatMSD {
            float: &self,
            nb_digits: nb_digits,
        }
    }

    #[inline]
    fn smart_mul<A: OpArgument<T>, B: OpArgument<T>>(&mut self, a: A, b: B) {
        let (a_sign, a_scale, a_data) = a.data();
        let (b_sign, b_scale, b_data) = b.data();
        self._set_to_mul(a_sign, a_scale, a_data, b_sign, b_scale, b_data);
    }

    #[inline]
    fn smart_add_assign<A: OpArgument<T>>(&mut self, a: A) {
        let (a_sign, a_scale, a_data) = a.data();
        self.add_assign(a_sign, a_scale, a_data);
    }
}

impl<T: Digit> SmartMul<T> for BigUint<T> {
    type MSDRep<'a> = BigUintMSD<'a, T>;

    #[inline]
    fn msd(&self, nb_digits: usize) -> BigUintMSD<'_, T> {
        BigUintMSD {
            uint: &self,
            nb_digits: nb_digits,
        }
    }
}

pub(super) trait OpArgument<T: Digit> {
    fn data(&self) -> (bool, isize, &[T]);
}

impl<T: Digit> OpArgument<T> for BigFloat<T> {
    #[inline]
    fn data(&self) -> (bool, isize, &[T]) {
        (self.int.sign, self.scale, &self.int.uint.val)
    }
}

impl<T: Digit> OpArgument<T> for &BigFloat<T> {
    #[inline]
    fn data(&self) -> (bool, isize, &[T]) {
        (self.int.sign, self.scale, &self.int.uint.val)
    }
}

impl<T: Digit> OpArgument<T> for BigUint<T> {
    #[inline]
    fn data(&self) -> (bool, isize, &[T]) {
        (true, 0, &self.val)
    }
}

impl<T: Digit> OpArgument<T> for &BigUint<T> {
    #[inline]
    fn data(&self) -> (bool, isize, &[T]) {
        (true, 0, &self.val)
    }
}

/// Represents the most significant digits of a BigFloat
pub(super) struct BigFloatMSD<'a, T: Digit> {
    float: &'a BigFloat<T>,
    nb_digits: usize,
}

impl<'a, T: Digit> OpArgument<T> for BigFloatMSD<'a, T> {
    #[inline]
    fn data(&self) -> (bool, isize, &[T]) {
        let offset = self.nb_digits.min(self.float.int.uint.val.len());
        let cutoff = self.float.int.uint.val.len() - offset;
        (
            self.float.int.sign,
            self.float.scale + (cutoff as isize),
            &self.float.int.uint.val[cutoff..],
        )
    }
}

/// Represents the most significant digits of a BigUint
pub(super) struct BigUintMSD<'a, T: Digit> {
    uint: &'a BigUint<T>,
    nb_digits: usize,
}

impl<'a, T: Digit> OpArgument<T> for BigUintMSD<'a, T> {
    #[inline]
    fn data(&self) -> (bool, isize, &[T]) {
        let offset = self.nb_digits.min(self.uint.val.len());
        let cutoff = self.uint.val.len() - offset;
        (true, cutoff as isize, &self.uint.val[cutoff..])
    }
}

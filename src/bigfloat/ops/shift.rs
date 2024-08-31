use std::ops::Shl;
use std::ops::ShlAssign;
use std::ops::Shr;
use std::ops::ShrAssign;

use crate::traits::Digit;
use crate::BigFloat;

impl<T: Digit> BigFloat<T> {
    pub(crate) fn shl(&mut self, b: usize) {
        self.int <<= b % T::NB_BITS;
        let scale_offset: isize = (b / T::NB_BITS).try_into().unwrap();
        self.scale += scale_offset;
    }

    fn shr(&mut self, b: usize) {
        let small_shift = b % T::NB_BITS;
        let big_shift: isize = (b / T::NB_BITS).try_into().unwrap();
        self.scale -= big_shift;

        if (self.int.uint.val[0].trailing_zeros() as usize) < small_shift {
            self.int <<= T::NB_BITS;
            self.scale -= 1;
        }
        self.int >>= small_shift;
    }
}

impl<T: Digit> Shl<usize> for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn shl(self, other: usize) -> BigFloat<T> {
        let mut ret = self.clone();
        ret <<= other;
        ret
    }
}

impl<T: Digit> Shl<usize> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn shl(mut self, other: usize) -> BigFloat<T> {
        self <<= other;
        self
    }
}

impl<T: Digit> ShlAssign<usize> for BigFloat<T> {
    fn shl_assign(&mut self, b: usize) {
        self.shl(b);
        self.simplify();
    }
}

impl<T: Digit> Shr<usize> for &BigFloat<T> {
    type Output = BigFloat<T>;
    fn shr(self, other: usize) -> BigFloat<T> {
        let mut ret = self.clone();
        ret >>= other;
        ret
    }
}

impl<T: Digit> Shr<usize> for BigFloat<T> {
    type Output = BigFloat<T>;
    fn shr(mut self, other: usize) -> BigFloat<T> {
        self >>= other;
        self
    }
}

impl<T: Digit> ShrAssign<usize> for BigFloat<T> {
    fn shr_assign(&mut self, b: usize) {
        self.shr(b)
    }
}

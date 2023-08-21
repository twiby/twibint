use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::traits::Digit;
use crate::BigUint;

impl<T: Digit> Shl<usize> for &BigUint<T> {
    type Output = BigUint<T>;
    fn shl(self, other: usize) -> BigUint<T> {
        let mut ret = self.clone();
        ret <<= other;
        ret
    }
}
impl<T: Digit> Shl<usize> for BigUint<T> {
    type Output = BigUint<T>;
    fn shl(mut self, other: usize) -> BigUint<T> {
        self <<= other;
        self
    }
}
impl<T: Digit> ShlAssign<usize> for BigUint<T> {
    fn shl_assign(&mut self, mut b: usize) {
        if self == &BigUint::<T>::new(T::ZERO) {
            return;
        }

        // First apply whole word shifts (by decreasing b by steps of T::NB_BITS)
        let u32_shifts = b / T::NB_BITS;
        let mut temp = vec![T::ZERO; u32_shifts];
        temp.append(&mut self.val);
        self.val = temp;
        b %= T::NB_BITS;

        // Early exit
        if b == 0 {
            return;
        }

        // remaining: shift by less than T::NB_BITS
        let mut overflowing_bits: T;
        let mut carry = T::ZERO;
        for val in &mut self.val[(u32_shifts)..] {
            overflowing_bits = *val >> (T::NB_BITS - b);
            *val <<= b;
            *val |= carry;
            carry = overflowing_bits
        }

        if carry > T::ZERO {
            self.val.push(carry);
        }
    }
}

fn small_shr_assign<T: Digit>(n: &mut BigUint<T>, b: usize) {
    debug_assert!(b < T::NB_BITS);
    // Early exit
    if b == 0 {
        return;
    }

    // remaining: shift by less than T::NB_BITS
    let mut underflowing_bits: T;
    let mut carry = T::ZERO;
    for val in n.val.iter_mut().rev() {
        underflowing_bits = *val << (T::NB_BITS - b);
        *val >>= b;
        *val |= carry;
        carry = underflowing_bits
    }

    n.remove_trailing_zeros();
}

impl<T: Digit> Shr<usize> for &BigUint<T> {
    type Output = BigUint<T>;
    fn shr(self, other: usize) -> BigUint<T> {
        if other >= self.nb_bits() {
            return BigUint::<T>::default();
        }

        let mut ret = BigUint::<T>::from(self.val[other / T::NB_BITS..].to_vec());
        small_shr_assign(&mut ret, other % T::NB_BITS);
        ret
    }
}
impl<T: Digit> Shr<usize> for BigUint<T> {
    type Output = BigUint<T>;
    fn shr(mut self, other: usize) -> BigUint<T> {
        self >>= other;
        self
    }
}
impl<T: Digit> ShrAssign<usize> for BigUint<T> {
    fn shr_assign(&mut self, b: usize) {
        if b >= self.nb_bits() {
            self.val = vec![T::ZERO];
            return;
        }

        // First apply whole word shifts (by decreasing b by steps of 32)
        self.val.drain(..b / T::NB_BITS);

        // shift by less than 32
        small_shr_assign(self, b % T::NB_BITS)
    }
}

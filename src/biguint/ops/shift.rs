use core::ops::{Shl, ShlAssign, Shr, ShrAssign};

use crate::BigUint;

impl Shl<usize> for &BigUint {
    type Output = BigUint;
    fn shl(self, other: usize) -> BigUint {
        let mut ret = self.clone();
        ret <<= other;
        ret
    }
}
impl Shl<usize> for BigUint {
    type Output = BigUint;
    fn shl(mut self, other: usize) -> BigUint {
        self <<= other;
        self
    }
}
impl ShlAssign<usize> for BigUint {
    fn shl_assign(&mut self, mut b: usize) {
        // First apply whole word shifts (by decreasing b by steps of 32)
        let u32_shifts = b / 32;
        let mut temp = vec![0u32; u32_shifts];
        temp.append(&mut self.val);
        self.val = temp;
        b %= 32;

        // Early exit
        if b == 0 {
            return;
        }

        // remaining: shift by less than 32
        let mut overflowing_bits: u32;
        let mut carry: u32 = 0;
        for val in &mut self.val[(u32_shifts)..] {
            overflowing_bits = *val >> (32 - b);
            *val <<= b;
            *val |= carry;
            carry = overflowing_bits
        }

        if carry > 0 {
            self.val.push(carry);
        }
    }
}

fn small_shr_assign(n: &mut BigUint, b: usize) {
    debug_assert!(b < 32);
    // Early exit
    if b == 0 {
        return;
    }

    // remaining: shift by less than 32
    let mut underflowing_bits: u32;
    let mut carry: u32 = 0;
    for val in n.val.iter_mut().rev() {
        underflowing_bits = *val << (32 - b);
        *val >>= b;
        *val |= carry;
        carry = underflowing_bits
    }

    n.remove_trailing_zeros();
}

impl Shr<usize> for &BigUint {
    type Output = BigUint;
    fn shr(self, other: usize) -> BigUint {
        if other >= self.nb_bits() {
            return BigUint::default();
        }

        let mut ret = BigUint::from(self.val[other / 32..].to_vec());
        small_shr_assign(&mut ret, other % 32);
        ret
    }
}
impl Shr<usize> for BigUint {
    type Output = BigUint;
    fn shr(mut self, other: usize) -> BigUint {
        self >>= other;
        self
    }
}
impl ShrAssign<usize> for BigUint {
    fn shr_assign(&mut self, b: usize) {
        if b >= self.nb_bits() {
            self.val = vec![0];
            return;
        }

        // First apply whole word shifts (by decreasing b by steps of 32)
        self.val.drain(..b / 32);

        // shift by less than 32
        small_shr_assign(self, b % 32)
    }
}

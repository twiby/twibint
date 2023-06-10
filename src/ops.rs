use core::ops::{
    Add, AddAssign, Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::BigInt;

const U32_RADIX: u64 = 1 << 32;

impl Add<u32> for &BigInt {
    type Output = BigInt;

    fn add(self, other: u32) -> Self::Output {
        let mut ret: BigInt = self.clone();
        ret += other;
        return ret;
    }
}
impl Add<&BigInt> for u32 {
    type Output = BigInt;
    fn add(self, other: &BigInt) -> Self::Output {
        other + self
    }
}
impl Add<&BigInt> for &BigInt {
    type Output = BigInt;

    fn add(self, other: &BigInt) -> Self::Output {
        let mut ret = self.clone();
        ret += other;
        return ret;
    }
}

impl AddAssign<u32> for BigInt {
    fn add_assign(&mut self, other: u32) {
        let other = BigInt::new(other);
        *self += other;
    }
}

impl AddAssign<BigInt> for BigInt {
    fn add_assign(&mut self, other: BigInt) {
        *self += &other;
    }
}
impl AddAssign<&BigInt> for BigInt {
    fn add_assign(&mut self, other: &BigInt) {
        while self.val.len() < other.val.len() {
            self.val.push(0);
        }

        for idx in 0..other.val.len() {
            let mut b = other.val[idx];
            let mut c = false;

            for val in &mut self.val[idx..] {
                (*val, c) = val.overflowing_add(b);
                b = 1;
                if !c {
                    break;
                }
            }

            if c {
                self.val.push(1);
            }
        }
    }
}

impl SubAssign<u32> for BigInt {
    fn sub_assign(&mut self, other: u32) {
        *self -= &BigInt::new(other);
    }
}
impl SubAssign<&BigInt> for BigInt {
    fn sub_assign(&mut self, other: &BigInt) {
        if &*self < other {
            panic!("Attempt at subtraction with underflow");
        }

        for idx in 0..other.val.len() {
            let mut b = other.val[idx];
            let mut c: bool;

            for val in &mut self.val[idx..] {
                (*val, c) = val.overflowing_sub(b);
                b = 1;
                if !c {
                    break;
                }
            }

            self.remove_trailing_zeros();
        }
    }
}
impl Sub<u32> for &BigInt {
    type Output = BigInt;
    fn sub(self, other: u32) -> BigInt {
        let mut ret = self.clone();
        ret -= other;
        return ret;
    }
}
impl Sub<&BigInt> for &BigInt {
    type Output = BigInt;
    fn sub(self, other: &BigInt) -> BigInt {
        let mut ret = self.clone();
        ret -= other;
        ret
    }
}

impl Shl<usize> for &BigInt {
    type Output = BigInt;
    fn shl(self, other: usize) -> BigInt {
        let mut ret = self.clone();
        ret <<= other;
        ret
    }
}
impl Shl<usize> for BigInt {
    type Output = BigInt;
    fn shl(mut self, other: usize) -> BigInt {
        self <<= other;
        self
    }
}
impl ShlAssign<usize> for BigInt {
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

impl Shr<usize> for &BigInt {
    type Output = BigInt;
    fn shr(self, other: usize) -> BigInt {
        let mut ret = self.clone();
        ret >>= other;
        ret
    }
}
impl Shr<usize> for BigInt {
    type Output = BigInt;
    fn shr(mut self, other: usize) -> BigInt {
        self >>= other;
        self
    }
}
impl ShrAssign<usize> for BigInt {
    fn shr_assign(&mut self, mut b: usize) {
        if b >= self.nb_bits() {
            self.val = vec![0];
            return;
        }

        // First apply whole word shifts (by decreasing b by steps of 32)
        let u32_shifts = b / 32;
        self.val = self.val[u32_shifts..].to_vec();
        b %= 32;

        // Early exit
        if b == 0 {
            return;
        }

        // remaining: shift by less than 32
        let mut underflowing_bits: u32;
        let mut carry: u32 = 0;
        for val in self.val.iter_mut().rev() {
            underflowing_bits = *val << (32 - b);
            *val >>= b;
            *val |= carry;
            carry = underflowing_bits
        }

        self.remove_trailing_zeros();
    }
}

pub(crate) fn pure_mul(a: u32, b: u32) -> (u32, u32) {
    let full = (a as u64) * (b as u64);
    return (
        (full % U32_RADIX).try_into().unwrap(),
        (full / U32_RADIX).try_into().unwrap(),
    );
}

impl MulAssign<u32> for BigInt {
    fn mul_assign(&mut self, other: u32) {
        let mut c: bool;
        let (mut c1, mut c2, mut v): (u32, u32, u32);

        (self.val[0], c1) = pure_mul(self.val[0], other);
        for val in self.val.iter_mut().skip(1) {
            (v, c2) = pure_mul(*val, other);
            (*val, c) = v.overflowing_add(c1);
            c1 = c2 + (c as u32);
        }

        if c1 > 0 {
            self.val.push(c1);
        }
    }
}
impl MulAssign<&BigInt> for BigInt {
    fn mul_assign(&mut self, other: &BigInt) {
        *self = &*self * other;
    }
}
impl Mul<u32> for &BigInt {
    type Output = BigInt;
    fn mul(self, other: u32) -> BigInt {
        let mut ret = self.clone();
        ret *= other;
        ret
    }
}
impl Mul<&BigInt> for u32 {
    type Output = BigInt;
    fn mul(self, other: &BigInt) -> BigInt {
        other * self
    }
}
impl Mul<&BigInt> for &BigInt {
    type Output = BigInt;
    fn mul(self, other: &BigInt) -> BigInt {
        if self.val.len() == 0 || other.val.len() == 0 {
            return BigInt::new(0);
        }

        let mut ret = BigInt::new(0);
        for i in 0..other.val.len() {
            ret += &((self * other.val[i]) << (i * 32));
        }

        ret
    }
}

impl RemAssign<u32> for BigInt {
    fn rem_assign(&mut self, other: u32) {
        let value = &*self % other;
        *self = BigInt::new(value);
    }
}
impl Rem<u32> for &BigInt {
    type Output = u32;
    fn rem(self, other: u32) -> u32 {
        let other_64 = other as u64;

        let mut base_mod: u64 = 1;
        let base_mod_multiplier: u64 = U32_RADIX % other_64;

        let mut ret: u64 = 0;
        for val in &self.val {
            ret += (*val as u64) * base_mod;
            ret %= other_64;
            base_mod = (base_mod * base_mod_multiplier) % other_64;
        }
        ret.try_into().unwrap()
    }
}

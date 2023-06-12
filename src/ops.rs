use core::cmp::Ordering;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::BigInt;

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

        let mut partial_carry_1: bool;
        let mut partial_carry_2: bool;
        let mut full_carry = false;

        for (a, b) in self.val.iter_mut().zip(other.val.iter()) {
            (*a, partial_carry_1) = a.overflowing_add(*b);
            (*a, partial_carry_2) = a.overflowing_add(full_carry as u32);
            full_carry = partial_carry_1 | partial_carry_2;
        }

        for val in self.val.iter_mut().skip(other.val.len()) {
            (*val, full_carry) = val.overflowing_add(full_carry as u32);
        }

        if full_carry {
            self.val.push(1);
        } else {
            self.remove_trailing_zeros();
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

        let mut partial_carry_1: bool;
        let mut partial_carry_2: bool;
        let mut carry = false;
        for (a, b) in self.val.iter_mut().zip(other.val.iter()) {
            (*a, partial_carry_1) = a.overflowing_sub(*b);
            (*a, partial_carry_2) = a.overflowing_sub(carry as u32);
            carry = partial_carry_1 | partial_carry_2;
        }

        for val in self.val.iter_mut().skip(other.val.len()) {
            (*val, carry) = val.overflowing_sub(carry as u32);
        }

        self.remove_trailing_zeros();
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

impl BitAnd<&BigInt> for &BigInt {
    type Output = BigInt;
    fn bitand(self, other: &BigInt) -> BigInt {
        let mut ret = BigInt::from(
            self.val
                .iter()
                .zip(other.val.iter())
                .map(|(a, b)| a & b)
                .collect::<Vec<_>>(),
        );
        ret.remove_trailing_zeros();
        ret
    }
}
impl BitAndAssign<&BigInt> for BigInt {
    fn bitand_assign(&mut self, other: &BigInt) {
        *self = &*self & other;
    }
}

impl BitOr<&BigInt> for &BigInt {
    type Output = BigInt;
    fn bitor(self, other: &BigInt) -> BigInt {
        let (big, small) = match self.cmp(other) {
            Ordering::Equal => return self.clone(),
            Ordering::Less => (other, self),
            Ordering::Greater => (self, other),
        };

        let mut ret = big.clone();
        ret.val
            .iter_mut()
            .zip(small.val.iter())
            .for_each(|(b, s)| *b |= *s);
        ret
    }
}
impl BitOrAssign<&BigInt> for BigInt {
    fn bitor_assign(&mut self, other: &BigInt) {
        *self = &*self | other;
    }
}

impl BitXor<&BigInt> for &BigInt {
    type Output = BigInt;
    fn bitxor(self, other: &BigInt) -> BigInt {
        let (big, small) = match self.cmp(other) {
            Ordering::Equal => return self.clone(),
            Ordering::Less => (other, self),
            Ordering::Greater => (self, other),
        };

        let mut ret = big.clone();
        ret.val
            .iter_mut()
            .zip(small.val.iter())
            .for_each(|(b, s)| *b ^= *s);
        ret.remove_trailing_zeros();
        ret
    }
}
impl BitXorAssign<&BigInt> for BigInt {
    fn bitxor_assign(&mut self, other: &BigInt) {
        *self = &*self ^ other;
    }
}

pub(crate) fn pure_mul(a: u32, b: u32) -> (u32, u32) {
    let full = (a as u64) * (b as u64);
    return (
        ((full << 32) >> 32).try_into().unwrap(),
        (full >> 32).try_into().unwrap(),
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

trait RemDiv<T> {
    type DivOutput;
    type RemOutput;
    fn rem_div(&self, other: &T) -> Option<(Self::DivOutput, Self::RemOutput)>;
    fn div(&self, other: &T) -> Option<Self::DivOutput> {
        self.rem_div(other).map(|ret| ret.0)
    }
    fn rem(&self, other: &T) -> Option<Self::RemOutput> {
        self.rem_div(other).map(|ret| ret.1)
    }
}

impl RemDiv<u32> for BigInt {
    type DivOutput = BigInt;
    type RemOutput = u32;
    fn rem_div(&self, other: &u32) -> Option<(BigInt, u32)> {
        if *other == 0 {
            return None;
        }

        let other_64 = *other as u64;
        let mut msb = 0u64;
        let mut div: u64;

        let mut ret = BigInt::new(0);
        for idx in (0..self.val.len()).rev() {
            let lsb = self.val[idx] as u64;

            let current = (msb << 32) | lsb;
            (div, msb) = (current / other_64, current % other_64);

            ret += BigInt::from(div) << (32 * idx);
        }

        ret.remove_trailing_zeros();
        Some((ret, msb.try_into().unwrap()))
    }

    fn rem(&self, other: &u32) -> Option<u32> {
        let other_64 = *other as u64;
        let mut msb = 0u64;

        for val in self.val.iter().rev() {
            let current = (msb << 32) | (*val as u64);
            msb = current % other_64;
        }

        Some(msb.try_into().unwrap())
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
        RemDiv::rem(self, &other).unwrap()
    }
}

impl DivAssign<u32> for BigInt {
    fn div_assign(&mut self, other: u32) {
        *self = &*self / other;
    }
}
impl Div<u32> for &BigInt {
    type Output = BigInt;
    fn div(self, other: u32) -> BigInt {
        RemDiv::div(self, &other).unwrap()
    }
}
impl Div<&BigInt> for u32 {
    type Output = u32;
    fn div(self, other: &BigInt) -> u32 {
        if other == &BigInt::new(0) {
            panic!("Attempt at division by zero.");
        }

        let big_self = BigInt::new(self);
        match big_self.cmp(other) {
            Ordering::Equal => 1u32,
            Ordering::Less => 0u32,
            Ordering::Greater => big_self.val[0] / other.val[0],
        }
    }
}
impl DivAssign<&BigInt> for u32 {
    fn div_assign(&mut self, other: &BigInt) {
        *self = *self / other;
    }
}

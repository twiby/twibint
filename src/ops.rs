use core::ops::{Add, AddAssign, Shl, ShlAssign, Mul, MulAssign, Rem, RemAssign};

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
		if add_assign_byte(&mut self.val, other) {
			self.val.push(1);
		}
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

		for b in 0..other.val.len() {
			if add_assign_byte(&mut self.val[b..], other.val[b]) {
				self.val.push(1);
			}
		}
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
		let mut u32_shift = 0;
		while b >= 32 {
			self.val.insert(0, 0);
			b -= 32;
			u32_shift += 1;
		}
		if b == 0 { return; }

		let c = shl_assign(&mut self.val[u32_shift..], b);
		if c > 0 {
			self.val.push(c);
		}
	}
}

impl MulAssign<u32> for BigInt {
	fn mul_assign(&mut self, other: u32) {
		let c = mul_assign(&mut self.val, other);
		if c > 0 {
			self.val.push(c);
		}
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
			ret += &((self * other.val[i]) << (i*32));
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

const U32_RADIX: u64 = 1 << 32;
pub fn pure_mul(a: u32, b: u32) -> (u32, u32) {
	let full = (a as u64) * (b as u64);
	return ((full%U32_RADIX).try_into().unwrap(), (full/U32_RADIX).try_into().unwrap());
}
pub fn mul_assign(a: &mut [u32], b: u32) -> u32 {
	let mut c: bool;
	let (mut c1, mut c2, mut v): (u32,u32,u32);

	(a[0], c1) = pure_mul(a[0], b);
	for val in a.iter_mut().skip(1) {
		(v, c2) = pure_mul(*val, b);
		(*val, c) = v.overflowing_add(c1);
		c1 = c2 + (c as u32);
	}
	return c1;
}

pub fn shl_assign(a: &mut [u32], b: usize) -> u32 {
	let mut carry_1: u32;
	let mut carry_2: u32 = 0;
	for val in a {
		carry_1 = *val >> (32-b);
		*val <<= b;
		*val |= carry_2;
		carry_2 = carry_1
	}
	return carry_2;
}

pub fn add_assign_byte(a: &mut [u32], mut b: u32) -> bool {
	let mut c = false;

	for val in a {
		(*val,c) = val.overflowing_add(b);
		b = 1;
		if !c { return false; }
	}

	return c
}
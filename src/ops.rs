use core::ops::{Add, AddAssign};

use crate::BigInt;

impl<'a> Add<u32> for &'a BigInt {
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
impl<'a> Add<&'a BigInt> for &'a BigInt {
	type Output = BigInt;

	fn add(self, other: &'a BigInt) -> Self::Output {
		let mut ret = self.clone();
		ret += other;
		return ret;
	}
}

impl AddAssign<u32> for BigInt {
	fn add_assign(&mut self, other: u32) {
		*self += &BigInt::new(other);
	}
}

impl<'a> AddAssign<&'a BigInt> for BigInt {
	fn add_assign(&mut self, other: &'a BigInt) {
		while self.val.len() < other.val.len() {
			self.val.push(0);
		}

		let (mut c1, mut c2) = (false, false);
		for b in 0..other.val.len() {
			let c = (c1 as u32) + (c2 as u32);
			c1 = add_assign_byte(&mut self.val[b..], c);
			c2 = add_assign_byte(&mut self.val[b..], other.val[b]);
		}
		let c = (c1 as u32) + (c2 as u32);
		if c > 0 {
			self.val.push(c);
		}
	}
}

pub fn add_assign_byte(a: &mut [u32], b: u32) -> bool {
	let (v, c) = a[0].overflowing_add(b);
	a[0] = v;

	if a.len() > 1 {
		return add_assign_byte(&mut a[1..], c as u32);
	} else {
		return c;
	}
}
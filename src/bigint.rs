


use core::cmp::Ordering;
use crate::digits_vec::Digits;

#[derive(Clone, Debug, PartialEq)]
pub struct BigInt {
	pub val: Vec<u32>
}

impl BigInt {
	pub fn new(val: u32) -> BigInt {
		BigInt{val: vec![val]}
	}

	#[inline]
	pub fn nb_bits(&self) -> usize {
		32 * self.val.len()
	}

	#[inline]
	pub fn bit(&self, b: usize) -> bool {
		(self.val[b/32] >> b%32) & 1 != 0
	}

	#[inline]
	pub fn bits<'a>(&'a self) -> impl DoubleEndedIterator<Item = bool> + 'a {
		(0..self.nb_bits()).map(|b| self.bit(b))
	}
}

impl From<u64> for BigInt {
	fn from(n: u64) -> BigInt {
		BigInt{val: vec![(n%4294967296).try_into().unwrap(), (n/4294967296).try_into().unwrap()]}
	}
}

impl From<Vec<u32>> for BigInt {
	fn from(v: Vec<u32>) -> BigInt {
		BigInt{val:v}
	}
}

impl From<&str> for BigInt {
	fn from(s: &str) -> BigInt {
		let mut ret = BigInt::new(0);

		let mut base = BigInt::new(1);
		for c in s.chars().rev() {
			let v:u32 = c.to_digit(10).unwrap();

			ret += v * &base;
			base *= 10;
		}

		return ret;
	}
}

impl From<&BigInt> for Digits {
	fn from(b: &BigInt) -> Digits {
		let mut digits = Digits::new(0);

		for bit in b.bits().rev() {
			digits.times_2();
			if bit {
				digits.add_n_at_k(1, 0);
			}
		}

		digits
	}
}

impl From<&BigInt> for String {
	fn from(b: &BigInt) -> String {
		String::from(&Digits::from(b))
	}
}
impl From<BigInt> for String {
	fn from(b: BigInt) -> String {
		String::from(&Digits::from(&b))
	}
}

impl PartialOrd<BigInt> for BigInt {
	fn partial_cmp(&self, other: &BigInt) -> Option<Ordering> {
		match self.val.len().cmp(&other.val.len()) {
			Ordering::Equal => (),
			o => return Some(o)
		};
		for (a,b) in self.val.iter().zip(other.val.iter()).rev() {
			match a.cmp(b) {
				Ordering::Equal => continue,
				o => return Some(o)
			};
		}
		Some(Ordering::Equal)
	}
}

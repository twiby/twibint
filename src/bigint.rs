#[derive(Clone, Debug, PartialEq)]
pub struct BigInt {
	pub val: u32
}

impl BigInt {
	pub fn new(val: u32) -> BigInt {
		BigInt{val:val}
	}
}

impl From<&str> for BigInt {
	fn from(s: &str) -> BigInt {
		let mut ret = BigInt::new(0);

		let mut base = 1;
		for c in s.chars().rev() {
			let v = c.to_digit(10).unwrap();

			ret += v * base;
			base *= 10;
		}

		return ret;
	}
}
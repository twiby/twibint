use crate::ops::add_assign_byte;
use crate::BigInt;

#[test]
fn new() {
	let val:u32 = 100;

	let bg = BigInt::new(val);

	assert_eq!(bg.val, val);
}

// #[test]
// fn from_str() {
// 	let s = "123";

// 	let bg = BigInt::from(s);

// 	assert_eq!(bg.val, 123);
// }

#[test]
fn to_str() {
	let bg = BigInt::new(123);

	assert_eq!(String::from(&bg), "123");
}

#[test]
fn add_assign() {
	let mut bg = BigInt::new(0);
	bg += 100u32;

	assert_eq!(bg.val, 100);

	bg += &BigInt::new(100);

	assert_eq!(bg.val, 200);
}

#[test]
fn add() {
	let b1 = BigInt::new(100);
	let b2 = BigInt::new(50);

	assert_eq!(&b1 + &b2, BigInt::new(150));
	assert_eq!(&b1 + 50, BigInt::new(150));
}

#[test]
fn bits() {
	let b1 = BigInt::new(0);
	let b2 = BigInt::new(4294967295);
	let b3 = BigInt::new(2147483648);

	assert_eq!(b1.nb_bits(), 32);
	assert_eq!(b2.nb_bits(), 32);
	assert_eq!(b3.nb_bits(), 32);

	for b in 0..b1.nb_bits() {
		assert!(!b1.bit(b));
	}

	for b in 0..b2.nb_bits() {
		assert!(b2.bit(b));
	}

	for b in 0..b3.nb_bits()-1 {
		assert!(!b3.bit(b));
	}
	assert!(b3.bit(31));

	let mut count = 0;
	for bit in b1.bits() {
		assert!(!bit);
		count += 1;
	}
	for bit in b2.bits().rev() {
		assert!(bit);
		count += 1;
	}
	assert_eq!(count, 64);
}

#[test]
fn add_assign_byte_test() {
	let mut b1: Vec<u32> = vec![u32::MAX, u32::MAX];
	let c = add_assign_byte(&mut b1, 2);

	assert_eq!(c, true);
	assert_eq!(b1, vec![1, 0]);
}
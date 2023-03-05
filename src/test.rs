use crate::BigInt;

#[test]
fn new() {
	let val:u32 = 100;

	let bg = BigInt::new(val);

	assert_eq!(bg.val, val);
}

#[test]
fn from_str() {
	let s = "123";

	let bg = BigInt::from(s);

	assert_eq!(bg.val, 123);
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
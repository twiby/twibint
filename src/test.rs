use crate::ops::mul_assign;
use crate::ops::pure_mul;
use crate::ops::add_assign_byte;
use crate::BigInt;

#[test]
fn new() {
	let val:u32 = 100;

	let bg = BigInt::new(val);

	assert_eq!(bg.val, vec![val]);
}

#[test]
fn from_str() {
	let s = "1234567891011121314151617181920";

	let bg = BigInt::from(s);

	assert_eq!(String::from(bg), "1234567891011121314151617181920");
}

#[test]
fn to_str() {
	let bg = BigInt::new(123);

	assert_eq!(String::from(&bg), "123");
}

#[test]
fn to_str_overflow() {
	let mut bg = BigInt::new(u32::MAX);
	bg += 1;

	assert_eq!(String::from(&bg), "4294967296");
}

#[test]
fn add_assign() {
	let mut bg = BigInt::new(0);
	bg += 100u32;

	assert_eq!(bg.val, vec![100]);

	bg += &BigInt::new(100);

	assert_eq!(bg.val, vec![200]);
}

#[test]
fn add_assign_overflow() {
	let mut bg = BigInt::new(u32::MAX);
	bg += 1u32;

	assert_eq!(bg.val, vec![0, 1]);

	bg += &BigInt::new(100);

	assert_eq!(bg.val, vec![100, 1]);
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

#[test]
fn add_assign_full_test() {
	let mut b1 = BigInt::from(vec![u32::MAX, u32::MAX, u32::MAX]);
	b1 += 1;

	assert_eq!(b1.val, vec![0, 0, 0, 1]);

	let b = &BigInt::new(u32::MAX) + &BigInt::new(u32::MAX);
	assert_eq!(b.val, vec![4294967294, 1]);
}

#[test]
fn fibonacci() {
	let mut n1 = BigInt::new(0);
	let mut n2 = BigInt::new(1);
	let mut n = 500;

	while n > 1 {
		let temp = n2.clone();
		n2 += &n1;
		n1 = temp;
		n -= 1;
	}

	assert_eq!(String::from(&n2), "1394232245616978801397243828704\
		072839500702565876973072641089629483255716228632906915\
		57658876222521294125");
}

#[test]
fn fibonacci_5() {
	let mut n1 = BigInt::new(0);
	let mut n2 = BigInt::new(1);
	let mut n = 500;

	while n > 1 {
		let temp = n2.clone();
		n2 += &n1;
		n1 = temp;
		n -= 1;
	}

	n2 *= 5u32;

	assert_eq!(String::from(&n2), "6971161228084894006986219143520\
		364197503512829384865363205448147416278581143164534577\
		88294381112606470625");
}

#[test]
fn fibonacci_5_bis() {
	let mut n1 = BigInt::new(0);
	let mut n2 = BigInt::new(1);
	let mut n = 500;

	while n > 1 {
		let temp = n2.clone();
		n2 += &n1;
		n1 = temp;
		n -= 1;
	}


	let n3 = &n2 * &BigInt::new(5);
	assert_eq!(String::from(&n3), "6971161228084894006986219143520\
		364197503512829384865363205448147416278581143164534577\
		88294381112606470625");
	let n3 = &BigInt::new(5) * &n2;
	assert_eq!(String::from(&n3), "6971161228084894006986219143520\
		364197503512829384865363205448147416278581143164534577\
		88294381112606470625");
}

#[test]
fn fibonacci_square() {
	let mut n1 = BigInt::new(0);
	let mut n2 = BigInt::new(1);
	let mut n = 500;

	while n > 1 {
		let temp = n2.clone();
		n2 += &n1;
		n1 = temp;
		n -= 1;
	}
	let n1 = n2.clone();
	assert_eq!(String::from(&n2), "1394232245616978801397243828704\
		072839500702565876973072641089629483255716228632906915\
		57658876222521294125");
	assert_eq!(String::from(&n1), "1394232245616978801397243828704\
		072839500702565876973072641089629483255716228632906915\
		57658876222521294125");

	let n3 = &n1 * &BigInt::from(vec![1,2]);
	assert_eq!(String::from(&n3), "1197636379730135883426986149904088164401882181843047237880919768151177099109732530421226027034957340830083465166125");

	assert_eq!(String::from(&n1*505575602), "70488980690561591893764998983784650568241707916619531913830861936195395765223225155085259704056844689235065938250");

	let n3 = &n2 * &n1;
	assert_eq!(String::from(&n3), "19438835547181635041596396415865294747559575831069137016545616216954503763688963053816123829809193659535915661080641869480232607381532114794348172492750360328240298056819461819264536306335141533339064759515625");
}

#[test]
fn factorial_100() {
	let mut n = BigInt::new(1);

	for i in 1..=100u32 {
		n *= i;
	}

	assert_eq!(String::from(n), "93326215443944152681699238856266700490715968264381621468592963895217599993229915608941463976156518286253697920827223758251185210916864000000000000000000000000");
}

#[test]
fn mul_test() {
	let n1 = BigInt::new(4294967295);
	let n2 = BigInt::new(4294967295);
	let n3 = &n1 * &n2;

	assert_eq!(String::from(&n3), "18446744065119617025");	
}

#[test]
fn pure_mul_test() {
	let (a,b) = pure_mul(u32::MAX, u32::MAX);
	assert_eq!(a, 1);
	assert_eq!(b, 4294967294);

	let n1 = BigInt::from(vec![4294967295, 4294967295, 4294967295]);
	let n2 = &n1 * 4294967295;
	assert_eq!(String::from(&n2), "340282366841710300949110269833929293825");
}

#[test]
fn shr_assign_test() {
	let b = BigInt::new(2147483648);
	let b2 = &b << 33;
	assert_eq!(b2.val, vec![0,0,1]);
}

#[test]
fn mul_assign_u32() {
	let mut b = BigInt::new(2147483648);
	let c = mul_assign(&mut b.val, 5);
	assert_eq!(c, 2);
	assert_eq!(String::from(&b), "2147483648");
}


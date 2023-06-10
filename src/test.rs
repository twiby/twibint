use crate::ops::pure_mul;
use crate::BigInt;

#[test]
fn new() {
    let val: u32 = 100;

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
fn from_u64() {
    let n = BigInt::from(18446744073709551614u64);
    assert_eq!(n.val, vec![4294967294, 4294967295]);
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
fn cmp() {
    let n1 = BigInt::from(vec![u32::MAX, u32::MAX, u32::MAX]);
    assert!(n1 == n1);
    assert!(n1 <= n1);
    assert!(n1 >= n1);

    let n2 = BigInt::from(vec![u32::MAX - 1, u32::MAX, u32::MAX]);
    assert!(n2 < n1);
    assert!(n1 > n2);

    let n3 = BigInt::from(vec![u32::MAX, u32::MAX, u32::MAX - 1]);
    assert!(n3 < n2);
    assert!(n3 < n1);

    let n4 = BigInt::from(vec![u32::MAX, u32::MAX]);
    assert!(n4 <= n1);
    assert!(n4 <= n2);
    assert!(n4 <= n3);
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

    for b in 0..b3.nb_bits() - 1 {
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
fn add_assign_full_test() {
    let mut b1 = BigInt::from(vec![u32::MAX, u32::MAX, u32::MAX]);
    b1 += 1;

    assert_eq!(b1.val, vec![0, 0, 0, 1]);

    let b = &BigInt::new(u32::MAX) + &BigInt::new(u32::MAX);
    assert_eq!(b.val, vec![4294967294, 1]);
}

#[test]
fn sub() {
    let mut b1 = BigInt::from(vec![u32::MAX, u32::MAX]);
    b1 -= u32::MAX;

    assert_eq!(b1, BigInt::from(vec![0, u32::MAX]));
    b1 -= 1;
    assert_eq!(b1, BigInt::from(vec![u32::MAX, u32::MAX - 1]));
    b1 -= &BigInt::from(vec![u32::MAX, u32::MAX - 1]);
    assert_eq!(b1, BigInt::new(0));
}

#[test]
fn sub_full() {
    let n1 = BigInt::from("12345678910111213");
    let n2 = BigInt::from("987654321");

    let n3 = &n1 - &n2;
    assert_eq!(String::from(&n3), "12345677922456892");
}

#[test]
fn fibonacci() {
    let mut n1 = BigInt::new(0);
    let mut n2 = BigInt::new(1);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        assert_eq!(&n2 - &n1, temp);
        assert_eq!(&n2 - &(&n1 + &temp), BigInt::new(0));
        n1 = temp;
        n -= 1;
    }

    assert_eq!(
        String::from(&n2),
        "1394232245616978801397243828704\
		072839500702565876973072641089629483255716228632906915\
		57658876222521294125"
    );
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

    assert_eq!(
        String::from(&n2),
        "6971161228084894006986219143520\
		364197503512829384865363205448147416278581143164534577\
		88294381112606470625"
    );
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
    assert_eq!(
        String::from(&n3),
        "6971161228084894006986219143520\
		364197503512829384865363205448147416278581143164534577\
		88294381112606470625"
    );
    let n3 = &BigInt::new(5) * &n2;
    assert_eq!(
        String::from(&n3),
        "6971161228084894006986219143520\
		364197503512829384865363205448147416278581143164534577\
		88294381112606470625"
    );
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
    assert_eq!(
        String::from(&n2),
        "1394232245616978801397243828704\
		072839500702565876973072641089629483255716228632906915\
		57658876222521294125"
    );
    assert_eq!(
        String::from(&n1),
        "1394232245616978801397243828704\
		072839500702565876973072641089629483255716228632906915\
		57658876222521294125"
    );

    let n3 = &n1 * &BigInt::from(vec![1, 2]);
    assert_eq!(
        String::from(&n3),
        "1197636379730135883426986149904\
		088164401882181843047237880919768151177099109732530421\
		226027034957340830083465166125"
    );

    assert_eq!(
        String::from(&n1 * 505575602),
        "704889806905615918937\
		649989837846505682417079166195319138308619361953957652\
		23225155085259704056844689235065938250"
    );

    let n3 = &n2 * &n1;
    n2 *= &n1;
    assert_eq!(
        String::from(&n3),
        "1943883554718163504159639641586\
		529474755957583106913701654561621695450376368896305381\
		612382980919365953591566108064186948023260738153211479\
		434817249275036032824029805681946181926453630633514153\
		3339064759515625"
    );
    assert_eq!(
        String::from(&n2),
        "1943883554718163504159639641586\
		529474755957583106913701654561621695450376368896305381\
		612382980919365953591566108064186948023260738153211479\
		434817249275036032824029805681946181926453630633514153\
		3339064759515625"
    );
}

#[test]
fn factorial_100() {
    let mut n = BigInt::new(1);

    for i in 1..=100u32 {
        n *= i;
    }

    assert_eq!(
        String::from(n),
        "933262154439441526816992388562667\
		004907159682643816214685929638952175999932299156089414\
		639761565182862536979208272237582511852109168640000000\
		00000000000000000"
    );
}

#[test]
fn fact_mod() {
    let mut n1 = BigInt::new(0);
    let mut n2 = BigInt::new(1);
    let mut n = 500;

    while n > 1 {
        let temp = n2.clone();
        n2 += &n1;
        n1 = temp;
        n -= 1;
    }
    let n3 = &n2 * &n2;

    assert_eq!(&n3 % 13, 9);
    assert_eq!(&n3 % 4294967295, 637285095);
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
    let (a, b) = pure_mul(u32::MAX, u32::MAX);
    assert_eq!(a, 1);
    assert_eq!(b, 4294967294);

    let n1 = BigInt::from(vec![4294967295, 4294967295, 4294967295]);
    let n2 = &n1 * 4294967295;
    assert_eq!(String::from(&n2), "340282366841710300949110269833929293825");
}

#[test]
fn shl_assign_test() {
    let b = BigInt::new(2147483648);
    let b2 = &b << 33;
    assert_eq!(b2.val, vec![0, 0, 1]);
}

#[test]
fn shr_assign_test() {
    let b = BigInt::from(vec![u32::MAX; 3]);
    let b2 = &b >> 33;
    assert_eq!(b2.val, vec![u32::MAX, u32::MAX >> 1]);
}

#[test]
fn mul_assign_u32() {
    let mut b = BigInt::new(2147483648);
    b *= 5;

    assert_eq!(String::from(&b), "10737418240");
}

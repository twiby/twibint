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
fn binary() {
    let a = BigInt::from(vec![256, 1024]);
    assert_eq!(
        format!("{:b}", a),
        "0000000000000000000001000000000000000000000000000000000100000000"
    );
}

#[test]
fn hex() {
    let a = BigInt::from(vec![256, 1024]);
    assert_eq!(format!("{:x}", a), "0000040000000100");
}

#[test]
fn default() {
    assert_eq!(BigInt::default(), BigInt::new(0));
}

#[test]
fn fromstr() {
    let a: BigInt = "124".parse().unwrap();
    assert_eq!(a, BigInt::new(124));
}

#[test]
#[should_panic]
fn fromstr_fail() {
    let _: BigInt = "124test".parse().unwrap();
}

#[test]
fn f64() {
    let a = BigInt::from(vec![u32::MAX, u32::MAX]);
    let f: f64 = From::from(&a);
    assert_eq!(f, 1.8446744073709552e+19);

    assert_eq!(format!("{:e}", a), format!("{:e}", f));
}

#[test]
fn hash() {
    use std::collections::HashMap;
    let mut map = HashMap::<BigInt, String>::new();

    map.insert(BigInt::from(vec![1, 2, 3]), "first".to_string());
    map.insert(BigInt::from(vec![3, 2, 1]), "second".to_string());

    assert!(map.contains_key(&BigInt::from(vec![1, 2, 3])));
    assert!(map.contains_key(&BigInt::from(vec![3, 2, 1])));
    assert_eq!(map[&BigInt::from(vec![1, 2, 3])], "first");
    assert_eq!(map[&BigInt::from(vec![3, 2, 1])], "second");
}

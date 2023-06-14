use crate::{BigInt, BigUint};

#[test]
fn creation() {
    let n1 = bigint!(128u32);
    let n2 = bigint!(-129i32);
    let n3 = bigint!(129u64);
    let n4 = bigint!(-128i64);

    assert_eq!(
        n1,
        BigInt {
            uint: biguintvec![128],
            sign: true
        }
    );
    assert_eq!(
        n2,
        BigInt {
            uint: biguintvec![129],
            sign: false
        }
    );
    assert_eq!(
        n3,
        BigInt {
            uint: biguintvec![129],
            sign: true
        }
    );
    assert_eq!(
        n4,
        BigInt {
            uint: biguintvec![128],
            sign: false
        }
    );

    assert!(n1 > n2);
    assert!(n1 < n3);
    assert!(n1 > n4);
    assert!(n2 < n3);
    assert!(n2 < n4);
    assert!(n3 > n4);
}

#[test]
fn hash() {
    use std::collections::HashMap;
    let mut map = HashMap::<BigInt, String>::new();

    let n1 = bigintvec![1, 2, 3];
    let mut n2 = n1.clone();
    n2.sign = false;

    map.insert(n1.clone(), "first".to_string());
    map.insert(n2.clone(), "second".to_string());

    assert!(map.contains_key(&n1));
    assert!(map.contains_key(&n2));
    assert_eq!(map[&n1], "first");
    assert_eq!(map[&n2], "second");
}

#[test]
fn f64() {
    let mut a = bigintvec![u32::MAX, u32::MAX];
    a.sign = false;
    let f: f64 = From::from(&a);
    assert_eq!(f, -1.8446744073709552e+19);

    assert_eq!(format!("{:e}", a), format!("{:e}", f));
}

#[test]
fn f32() {
    let mut a = bigintvec![u32::MAX, u32::MAX];
    a.sign = false;
    let f: f32 = From::from(&a);
    assert_eq!(f, -1.8446744e+19);
}

#[test]
fn binary() {
    let mut a = bigintvec![256, 1024];
    assert_eq!(
        format!("{:b}", a),
        "0000000000000000000001000000000000000000000000000000000100000000"
    );
    a.sign = false;
    assert_eq!(
        format!("{:b}", a),
        "-0000000000000000000001000000000000000000000000000000000100000000"
    );
}

#[test]
fn hex() {
    let mut a = bigintvec![256, 1024];
    assert_eq!(format!("{:x}", a), "0000040000000100");
    a.sign = false;
    assert_eq!(format!("{:x}", a), "-0000040000000100");
}

#[test]
fn parse() {
    let n1: BigInt = "-12345678901234567".parse().unwrap();
    let n2: BigInt = "12345678901234567".parse().unwrap();
    assert_eq!(n1.uint, n2.uint);
    assert_ne!(n1.sign, n2.sign);

    assert_eq!(String::from(&n1), "-12345678901234567");
    assert_eq!(String::from(&n2), "12345678901234567");
}
#[test]
#[should_panic]
fn parse_fail() {
    let _: BigInt = "-123456789012-34567".parse().unwrap();
}

#[test]
fn from_f64() {
    // Test zero
    let f = 0f64;
    let n = BigInt::try_from(f).unwrap();
    assert_eq!(n, BigInt::default());

    // Test positive exponent
    let f: f64 = 1.8446744073709552e+19;
    let n = BigInt::try_from(f).unwrap();
    assert_eq!(n, bigint!("18446744073709551616"));

    // Test negative exponent
    let f: f64 = -1.8446744073709552e+3;
    let n = BigInt::try_from(f).unwrap();
    assert_eq!(n, bigint!("-1844"));
}
#[test]
#[should_panic]
fn from_f64_fail2() {
    let f: f64 = f64::INFINITY;
    let _ = BigInt::try_from(f).unwrap();
}
#[test]
#[should_panic]
fn from_f64_fail3() {
    let f: f64 = f64::NAN;
    let _ = BigInt::try_from(f).unwrap();
}
#[test]
fn from_f32() {
    // Test zero
    let f = 0f32;
    let n = BigInt::try_from(f).unwrap();
    assert_eq!(n, BigInt::default());

    // Test positive exponent
    let f: f32 = -1.8446744e+19;
    let n = BigInt::try_from(f).unwrap();
    assert_eq!(n, bigint!("-18446744073709551616"));

    // Test negative exponent
    let f: f32 = 1.8446744e+3;
    let n = BigInt::try_from(f).unwrap();
    assert_eq!(n, bigint!("1844"));
}
#[test]
#[should_panic]
fn from_f32_fail2() {
    let f: f32 = f32::INFINITY;
    let _ = BigInt::try_from(f).unwrap();
}
#[test]
#[should_panic]
fn from_f32_fail3() {
    let f: f32 = f32::NAN;
    let _ = BigInt::try_from(f).unwrap();
}

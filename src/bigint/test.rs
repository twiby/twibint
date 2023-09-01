use crate::traits::Digit;
use crate::{BigInt, BigUint};

use typed_test_gen::test_with;

#[test_with(u32, u64)]
fn creation<T: Digit>() {
    let n1 = BigInt::<T>::from(128u32);
    let n2 = BigInt::<T>::from(-129i32);
    let n3 = BigInt::<T>::from(129u64);
    let n4 = BigInt::<T>::from(-128i64);

    assert_eq!(
        n1,
        BigInt::<T> {
            uint: BigUint::<T>::from(128u32),
            sign: true
        }
    );
    assert_eq!(
        n2,
        BigInt::<T> {
            uint: BigUint::<T>::from(129u32),
            sign: false
        }
    );
    assert_eq!(
        n3,
        BigInt::<T> {
            uint: BigUint::<T>::from(129u32),
            sign: true
        }
    );
    assert_eq!(
        n4,
        BigInt::<T> {
            uint: BigUint::<T>::from(128u32),
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

#[test_with(u32, u64)]
fn sign<T: Digit>() {
    let n1 = BigInt::<T>::from(1i32);
    let n2 = BigInt::<T>::from(0i32);
    let n3 = BigInt::<T>::from(-1i32);
    let n4 = BigInt::<T> {
        uint: BigUint::<T>::default(),
        sign: false,
    };

    assert!(n1.is_sign_positive());
    assert!(!n1.is_sign_negative());
    assert!(!n2.is_sign_positive());
    assert!(!n2.is_sign_negative());
    assert!(!n4.is_sign_positive());
    assert!(!n4.is_sign_negative());
    assert!(!n3.is_sign_positive());
    assert!(n3.is_sign_negative());

    assert_eq!(n2, n4);
}

#[test_with(u32, u64)]
fn hash<T: Digit>() {
    use std::collections::HashMap;
    let mut map = HashMap::<BigInt<T>, String>::new();

    let n1 = BigInt::<T>::from(u64::MAX);
    let mut n2 = n1.clone();
    n2.sign = false;

    map.insert(n1.clone(), "first".to_string());
    map.insert(n2.clone(), "second".to_string());

    assert!(map.contains_key(&n1));
    assert!(map.contains_key(&n2));
    assert_eq!(map[&n1], "first");
    assert_eq!(map[&n2], "second");
}

#[test_with(u32, u64)]
fn f64<T: Digit>() {
    let mut a = BigInt::<T>::from(u64::MAX);
    a.sign = false;
    let f: f64 = From::from(&a);
    assert_eq!(f, -1.8446744073709552e+19);

    assert_eq!(format!("{:e}", a), format!("{:e}", f));
}

#[test_with(u32, u64)]
fn f32<T: Digit>() {
    let mut a = BigInt::<T>::from(u64::MAX);
    a.sign = false;
    let f: f32 = From::from(&a);
    assert_eq!(f, -1.8446744e+19);
}

#[test_with(u32, u64)]
fn binary<T: Digit>() {
    let mut a = BigInt::<T>::from(BigUint::<T>::from(vec![T::MAX >> 1, T::MAX >> 1]));
    let mut s = "0".to_string();
    for _ in 0..T::NB_BITS - 1 {
        s.push('1');
    }
    s.push('0');
    for _ in 0..T::NB_BITS - 1 {
        s.push('1');
    }
    assert_eq!(format!("{:b}", a), s);
    a.sign = false;
    let mut s2 = "-".to_string();
    s2.push_str(&s);
    assert_eq!(format!("{:b}", a), s2);
}

#[test_with(u32, u64)]
fn hex<T: Digit>() {
    let mut a = BigInt::<T>::from(BigUint::<T>::from(vec![T::MAX >> 4, T::MAX >> 4]));
    let mut s = "0".to_string();
    for _ in 0..(T::NB_BITS >> 2) - 1 {
        s.push('f');
    }
    s.push('0');
    for _ in 0..(T::NB_BITS >> 2) - 1 {
        s.push('f');
    }
    assert_eq!(format!("{:x}", a), s);
    a.sign = false;
    let mut s2 = "-".to_string();
    s2.push_str(&s);
    assert_eq!(format!("{:x}", a), s2);
}

#[test_with(u32, u64)]
fn parse<T: Digit>() {
    let n1: BigInt<T> = "-12345678901234567".parse().unwrap();
    let n2: BigInt<T> = "12345678901234567".parse().unwrap();
    assert_eq!(n1.uint, n2.uint);
    assert_ne!(n1.sign, n2.sign);

    assert_eq!(String::from(&n1), "-12345678901234567");
    assert_eq!(String::from(&n2), "12345678901234567");
}
#[test_with(u32, u64)]
#[should_panic]
fn parse_fail<T: Digit>() {
    let _: BigInt<T> = "-123456789012-34567".parse().unwrap();
}

#[test_with(u32, u64)]
fn from_f64<T: Digit>() {
    // Test zero
    let f = 0f64;
    let n = BigInt::<T>::try_from(f).unwrap();
    assert_eq!(n, BigInt::<T>::default());

    // Test positive exponent
    let f: f64 = 1.8446744073709552e+19;
    let n = BigInt::<T>::try_from(f).unwrap();
    assert_eq!(n.to_string(), "18446744073709551616");

    // Test negative exponent
    let f: f64 = -1.8446744073709552e+3;
    let n = BigInt::<T>::try_from(f).unwrap();
    assert_eq!(n.to_string(), "-1844");
}

#[test_with(u32, u64)]
#[should_panic]
fn from_f64_fail2<T: Digit>() {
    let f: f64 = f64::INFINITY;
    let _ = BigInt::<T>::try_from(f).unwrap();
}

#[test_with(u32, u64)]
#[should_panic]
fn from_f64_fail3<T: Digit>() {
    let f: f64 = f64::NAN;
    let _ = BigInt::<T>::try_from(f).unwrap();
}
#[test_with(u32, u64)]
fn from_f32<T: Digit>() {
    // Test zero
    let f = 0f32;
    let n = BigInt::<T>::try_from(f).unwrap();
    assert_eq!(n, BigInt::<T>::default());

    // Test positive exponent
    let f: f32 = -1.8446744e+19;
    let n = BigInt::<T>::try_from(f).unwrap();
    assert_eq!(n.to_string(), "-18446744073709551616");

    // Test negative exponent
    let f: f32 = 1.8446744e+3;
    let n = BigInt::<T>::try_from(f).unwrap();
    assert_eq!(n.to_string(), "1844");
}

#[test_with(u32, u64)]
#[should_panic]
fn from_f32_fail2<T: Digit>() {
    let f: f32 = f32::INFINITY;
    let _ = BigInt::<T>::try_from(f).unwrap();
}

#[test_with(u32, u64)]
#[should_panic]
fn from_f32_fail3<T: Digit>() {
    let f: f32 = f32::NAN;
    let _ = BigInt::<T>::try_from(f).unwrap();
}

// test_panic_functions!(
//     parse_fail, parse_fail_u32, parse_fail_u64;
//     from_f64_fail2, from_f64_fail2_u32, from_f64_fail2_u64;
//     from_f64_fail3, from_f64_fail3_u32, from_f64_fail3_u64;
//     from_f32_fail2, from_f32_fail2_u32, from_f32_fail2_u64;
//     from_f32_fail3, from_f32_fail3_u32, from_f32_fail3_u64;
// );

// test_functions!(
//     creation, creation_u32, creation_u64;
//     sign, sign_u32, sign_u64;
//     hash, hash_u32, hash_u64;
//     f32, f32_u32, f32_u64;
//     f64, f64_u32, f64_u64;
//     binary, binary_u32, binary_u64;
//     parse, parse_u32, parse_u64;
//     hex, hex_u32, hex_u64;
//     from_f32, from_f32_u32, from_f32_u64;
//     from_f64, from_f64_u32, from_f64_u64;
// );

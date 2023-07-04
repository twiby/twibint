use crate::traits::TrueDiv;
use crate::BigUint;

#[test]
fn new() {
    let val: u32 = 100;

    let bg = BigUint::new(val);

    assert_eq!(bg.val, vec![val]);
}

#[test]
fn zero() {
    let val = BigUint::default();
    assert_eq!(val.to_string(), "0");
}

#[test]
fn from_str() {
    let s = "1234567891011121314151617181920";

    let bg = biguint!(s);

    assert_eq!(String::from(bg), "1234567891011121314151617181920");
}

#[test]
fn from_u64() {
    let n = BigUint::from(18446744073709551614u64);
    assert_eq!(n.val, vec![4294967294, 4294967295]);
}

#[test]
fn to_str() {
    let bg = BigUint::new(123);

    assert_eq!(String::from(&bg), "123");
}

#[test]
fn to_str_overflow() {
    let mut bg = BigUint::new(u32::MAX);
    bg += 1;

    assert_eq!(String::from(&bg), "4294967296");
}

#[test]
fn cmp() {
    let n1 = biguint![vec![u32::MAX, u32::MAX, u32::MAX]];
    assert!(n1 == n1);
    assert!(n1 <= n1);
    assert!(n1 >= n1);

    let n2 = biguint![vec![u32::MAX - 1, u32::MAX, u32::MAX]];
    assert!(n2 < n1);
    assert!(n1 > n2);

    let n3 = biguint![vec![u32::MAX, u32::MAX, u32::MAX - 1]];
    assert!(n3 < n2);
    assert!(n3 < n1);

    let n4 = biguint![vec![u32::MAX, u32::MAX]];
    assert!(n4 <= n1);
    assert!(n4 <= n2);
    assert!(n4 <= n3);
}

#[test]
fn bits() {
    let b1 = BigUint::new(0);
    let b2 = BigUint::new(4294967295);
    let b3 = BigUint::new(2147483648);
    let b4 = &b3 >> 1;

    assert_eq!(b1.nb_bits(), 0);
    assert_eq!(b2.nb_bits(), 32);
    assert_eq!(b3.nb_bits(), 32);
    assert_eq!(b4.nb_bits(), 31);

    assert!(!b1.bit(100));
    assert!(!b2.bit(100));
    assert!(!b3.bit(100));
    assert!(!b4.bit(100));

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
    assert_eq!(count, 32);
}

#[test]
fn binary() {
    let a = biguint![vec![256, 1024]];
    assert_eq!(
        format!("{:b}", a),
        "0000000000000000000001000000000000000000000000000000000100000000"
    );
}

#[test]
fn hex() {
    let a = biguint![vec![256, 1024]];
    assert_eq!(format!("{:x}", a), "0000040000000100");
}

#[test]
fn default() {
    assert_eq!(BigUint::default(), BigUint::new(0));
}

#[test]
fn fromstr() {
    let a: BigUint = "124".parse().unwrap();
    assert_eq!(a, BigUint::new(124));
}

#[test]
#[should_panic]
fn fromstr_fail() {
    let _: BigUint = "124test".parse().unwrap();
}

#[test]
fn f64() {
    let a = biguint![vec![u32::MAX, u32::MAX]];
    let f: f64 = From::from(&a);
    assert_eq!(f, 1.8446744073709552e+19);

    assert_eq!(format!("{:e}", a), format!("{:e}", f));
}

#[test]
fn f32() {
    let a = biguint![vec![u32::MAX, u32::MAX]];
    let f: f32 = From::from(&a);
    assert_eq!(f, 1.8446744e+19);
}

#[test]
fn hash() {
    use std::collections::HashMap;
    let mut map = HashMap::<BigUint, String>::new();

    map.insert(biguint![vec![1, 2, 3]], "first".to_string());
    map.insert(biguint![vec![3, 2, 1]], "second".to_string());

    assert!(map.contains_key(&biguint![vec![1, 2, 3]]));
    assert!(map.contains_key(&biguint![vec![3, 2, 1]]));
    assert_eq!(map[&biguint![vec![1, 2, 3]]], "first");
    assert_eq!(map[&biguint![vec![3, 2, 1]]], "second");
}

#[test]
fn from_f64() {
    // Test zero
    let f = 0f64;
    let n = BigUint::try_from(f).unwrap();
    assert_eq!(n, BigUint::default());

    // Test positive exponent
    let f: f64 = 1.8446744073709552e+19;
    let n = BigUint::try_from(f).unwrap();
    assert_eq!(n, biguint!("18446744073709551616"));

    // Test negative exponent
    let f: f64 = 1.8446744073709552e+3;
    let n = BigUint::try_from(f).unwrap();
    assert_eq!(n, biguint!("1844"));
}

#[test]
#[should_panic]
fn from_f64_fail() {
    let f: f64 = -1.8446744073709552e+19;
    let _ = BigUint::try_from(f).unwrap();
}
#[test]
#[should_panic]
fn from_f64_fail2() {
    let f: f64 = f64::INFINITY;
    let _ = BigUint::try_from(f).unwrap();
}
#[test]
#[should_panic]
fn from_f64_fail3() {
    let f: f64 = f64::NAN;
    let _ = BigUint::try_from(f).unwrap();
}
#[test]
fn from_f32() {
    // Test zero
    let f = 0f32;
    let n = BigUint::try_from(f).unwrap();
    assert_eq!(n, BigUint::default());

    // Test positive exponent
    let f: f32 = 1.8446744e+19;
    let n = BigUint::try_from(f).unwrap();
    assert_eq!(n, biguint!("18446744073709551616"));

    // Test negative exponent
    let f: f32 = 1.8446744e+3;
    let n = BigUint::try_from(f).unwrap();
    assert_eq!(n, biguint!("1844"));
}

#[test]
#[should_panic]
fn from_f32_fail() {
    let f: f32 = -1.8446744e+19;
    let _ = BigUint::try_from(f).unwrap();
}
#[test]
#[should_panic]
fn from_f32_fail2() {
    let f: f32 = f32::INFINITY;
    let _ = BigUint::try_from(f).unwrap();
}
#[test]
#[should_panic]
fn from_f32_fail3() {
    let f: f32 = f32::NAN;
    let _ = BigUint::try_from(f).unwrap();
}

#[test]
fn truediv() {
    let n1 = biguint!("123456678890123345567789");
    let n2 = biguint!("12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = 10000000270550.242f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);

    let n1 = biguint!("123456678890123345567789") << 15;
    let n2 = biguint!("12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = 3.2768000886539034e+17f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);

    let n1 = biguint!("123456678890123345567789") << 3030;
    let n2 = biguint!("12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = f64::INFINITY;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);

    let n2 = biguint!("123456678890123345567789");
    let n1 = biguint!("12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = 9.999999729449765e-14f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);

    let n2 = biguint!("12345667889012334556778900000000");
    let n1 = biguint!("12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = 9.999999729449765e-22f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);

    let n1 = biguint!("12345667889012334556778900000000");
    let n2 = biguint!("12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = 1.0000000270550242e+21f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);
}

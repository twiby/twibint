use crate::traits::Digit;
use crate::BigUint;

use typed_test_gen::test_with;

#[test_with(u32, u64)]
fn new<T: Digit>() {
    let val = T::ONE;

    let bg = BigUint::<T>::new(val);

    assert_eq!(bg.val, vec![val]);
}

#[test_with(u32, u64)]
fn zero<T: Digit>() {
    let val = BigUint::<T>::default();
    assert_eq!(val.to_string(), "0");
}

#[test_with(u32, u64)]
fn from_str<T: Digit>() {
    let s = "1234567891011121314151617181920";

    let bg = BigUint::<T>::from(s);

    assert_eq!(String::from(bg), "1234567891011121314151617181920");
}

#[test_with(u32, u64)]
fn from_u64<T: Digit>() {
    let n = BigUint::from(T::decomposition_from_u64(18446744073709551614u64));
    assert_eq!(n.to_string(), "18446744073709551614");
}

#[test_with(u32, u64)]
fn to_str<T: Digit>() {
    let bg = BigUint::<T>::from("123");

    assert_eq!(String::from(&bg), "123");
}

#[test_with(u32, u64)]
fn to_str_overflow<T: Digit>() {
    let mut bg = BigUint::<T>::from(T::decomposition_from_u32(u32::MAX));
    bg += T::ONE;

    assert_eq!(String::from(&bg), "4294967296");
}

#[test_with(u32, u64)]
fn cmp<T: Digit>() {
    let n1 = BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]);
    assert!(n1 == n1);
    assert!(n1 <= n1);
    assert!(n1 >= n1);

    let n2 = BigUint::<T>::from(vec![T::MAX - T::ONE, T::MAX, T::MAX]);
    assert!(n2 < n1);
    assert!(n1 > n2);

    let n3 = BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX - T::ONE]);
    assert!(n3 < n2);
    assert!(n3 < n1);

    let n4 = BigUint::<T>::from(vec![T::MAX, T::MAX]);
    assert!(n4 <= n1);
    assert!(n4 <= n2);
    assert!(n4 <= n3);
}

#[test_with(u32, u64)]
fn bits<T: Digit>() {
    let b1 = BigUint::<T>::new(T::ZERO);
    let b2 = BigUint::<T>::new(T::MAX);
    let mut b3 = BigUint::<T>::new(T::ONE);
    b3 <<= T::NB_BITS - 1;
    let b4 = &b3 >> 1;

    assert_eq!(b1.nb_bits(), 0);
    assert_eq!(b2.nb_bits(), T::NB_BITS);
    assert_eq!(b3.nb_bits(), T::NB_BITS);
    assert_eq!(b4.nb_bits(), T::NB_BITS - 1);

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
    assert!(b3.bit(T::NB_BITS - 1));

    let mut count = 0;
    for bit in b1.bits() {
        assert!(!bit);
        count += 1;
    }
    for bit in b2.bits().rev() {
        assert!(bit);
        count += 1;
    }
    assert_eq!(count, T::NB_BITS);
}

#[test_with(u32, u64)]
fn binary<T: Digit>() {
    let a = BigUint::<T>::from(vec![T::MAX >> 1, T::MAX >> 1]);
    let mut s = "0".to_string();
    for _ in 0..T::NB_BITS - 1 {
        s.push('1');
    }
    s.push('0');
    for _ in 0..T::NB_BITS - 1 {
        s.push('1');
    }
    assert_eq!(format!("{:b}", a), s);
}

#[test_with(u32, u64)]
fn hex<T: Digit>() {
    let a = BigUint::<T>::from(vec![T::MAX >> 4, T::MAX >> 4]);
    let mut s = "0".to_string();
    for _ in 0..(T::NB_BITS >> 2) - 1 {
        s.push('f');
    }
    s.push('0');
    for _ in 0..(T::NB_BITS >> 2) - 1 {
        s.push('f');
    }
    assert_eq!(format!("{:x}", a), s);
}

#[test_with(u32, u64)]
fn default<T: Digit>() {
    assert_eq!(BigUint::<T>::default(), BigUint::<T>::new(T::ZERO));
}

#[test_with(u32, u64)]
fn fromstr<T: Digit>() {
    let a: BigUint<T> = "124".parse().unwrap();
    assert_eq!(a, BigUint::<T>::from(124u32));
}

#[test_with(u32, u64)]
fn fromstr_fail<T: Digit>() {
    assert!("124test".parse::<BigUint<T>>().is_err());
}

#[test_with(u32, u64)]
fn f64<T: Digit>() {
    let a = BigUint::<T>::from(u64::MAX);
    let f: f64 = From::from(&a);
    assert_eq!(f, 1.8446744073709552e+19);

    assert_eq!(format!("{:e}", a), format!("{:e}", f));
}

#[test_with(u32, u64)]
fn f32<T: Digit>() {
    let a = BigUint::<T>::from(u64::MAX);
    let f: f32 = From::from(&a);
    assert_eq!(f, 1.8446744e+19);
}

#[test_with(u32, u64)]
fn hash<T: Digit>() {
    use std::collections::HashMap;
    let mut map = HashMap::<BigUint<T>, String>::new();

    map.insert(BigUint::<T>::from(u32::MAX), "first".to_string());
    map.insert(BigUint::<T>::from(u64::MAX), "second".to_string());

    assert!(map.contains_key(&BigUint::<T>::from(u32::MAX)));
    assert!(map.contains_key(&BigUint::<T>::from(u64::MAX)));
    assert_eq!(map[&BigUint::<T>::from(u32::MAX)], "first");
    assert_eq!(map[&BigUint::<T>::from(u64::MAX)], "second");
}

#[test_with(u32, u64)]
fn from_f64<T: Digit>() {
    // Test zero
    let f = 0f64;
    let n = BigUint::<T>::try_from(f).unwrap();
    assert_eq!(n, BigUint::<T>::default());

    // Test positive exponent
    let f: f64 = 1.8446744073709552e+19;
    let n = BigUint::<T>::try_from(f).unwrap();
    assert_eq!(n.to_string(), "18446744073709551616");

    // Test negative exponent
    let f: f64 = 1.8446744073709552e+3;
    let n = BigUint::<T>::try_from(f).unwrap();
    assert_eq!(n.to_string(), "1844");
}

#[test_with(u32, u64)]
fn from_f64_fail<T: Digit>() {
    let f: f64 = -1.8446744073709552e+19;
    assert!(BigUint::<T>::try_from(f).is_err());
}
#[test_with(u32, u64)]
fn from_f64_fail2<T: Digit>() {
    let f: f64 = f64::INFINITY;
    assert!(BigUint::<T>::try_from(f).is_err());
}
#[test_with(u32, u64)]
fn from_f64_fail3<T: Digit>() {
    let f: f64 = f64::NAN;
    assert!(BigUint::<T>::try_from(f).is_err());
}
#[test_with(u32, u64)]
fn from_f32<T: Digit>() {
    // Test zero
    let f = 0f32;
    let n = BigUint::<T>::try_from(f).unwrap();
    assert_eq!(n, BigUint::<T>::default());

    // Test positive exponent
    let f: f32 = 1.8446744e+19;
    let n = BigUint::<T>::try_from(f).unwrap();
    assert_eq!(n.to_string(), "18446744073709551616");

    // Test negative exponent
    let f: f32 = 1.8446744e+3;
    let n = BigUint::<T>::try_from(f).unwrap();
    assert_eq!(n.to_string(), "1844");
}

#[test_with(u32, u64)]
fn from_f32_fail<T: Digit>() {
    let f: f32 = -1.8446744e+19;
    assert!(BigUint::<T>::try_from(f).is_err());
}
#[test_with(u32, u64)]
fn from_f32_fail2<T: Digit>() {
    let f: f32 = f32::INFINITY;
    assert!(BigUint::<T>::try_from(f).is_err());
}
#[test_with(u32, u64)]
fn from_f32_fail3<T: Digit>() {
    let f: f32 = f32::NAN;
    assert!(BigUint::<T>::try_from(f).is_err());
}

#[test_with(u32, u64)]
fn to_primitive<T: Digit>() {
    let n1 = BigUint::<T>::from(T::decomposition_from_u64(u64::MAX));
    assert_eq!(TryInto::<u64>::try_into(&n1).unwrap(), u64::MAX);

    let n2 = BigUint::<T>::from(T::decomposition_from_u32(u32::MAX >> 16));
    assert_eq!(TryInto::<u16>::try_into(&n2).unwrap(), u16::MAX);
    assert_eq!(TryInto::<u32>::try_into(&n2).unwrap(), u16::MAX as u32);
    assert_eq!(TryInto::<u64>::try_into(&n2).unwrap(), u16::MAX as u64);
}
#[test_with(u32, u64)]
fn to_primitive_fail<T: Digit>() {
    assert!(
        TryInto::<u16>::try_into(&BigUint::<T>::from(T::decomposition_from_u32(u32::MAX))).is_err()
    );
}
#[test_with(u32, u64)]
fn to_primitive_fail2<T: Digit>() {
    let n1 = BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]);
    assert!(TryInto::<u64>::try_into(&n1).is_err());
}
#[test_with(u32, u64)]
fn to_primitive_fail3<T: Digit>() {
    let n1 = BigUint::<T>::from(vec![T::MAX >> 16; 3]);
    assert!(TryInto::<u32>::try_into(&n1).is_err());
}

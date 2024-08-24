use crate::traits::Digit;
use crate::BigUint;
use typed_test_gen::test_with;

use crate::BigInt;

use crate::bigfloat::BigFloat;

#[test_with(u32, u64)]
fn test_big_float_from_int<T: Digit>() {
    let int = BigInt::<T>::default();
    let f = BigFloat::<T>::from(int.clone());

    assert_eq!(f.int, int);
    assert_eq!(f.scale, 0);
}

#[test_with(u32, u64)]
fn test_big_float_from_uint<T: Digit>() {
    let int = BigInt::<T>::default();
    let f = BigFloat::<T>::from(int.uint.clone());

    assert_eq!(f.int, int);
    assert_eq!(f.scale, 0);
}

#[test_with(u32, u64)]
fn test_big_float_from_big_uint<T: Digit>() {
    let uint = BigUint::<T>::from(vec![T::ZERO, T::ZERO, T::MAX, T::MAX]);
    let f = BigFloat::<T>::from(uint.clone());

    assert_eq!(f.int, BigInt::from(BigUint::from(vec![T::MAX, T::MAX])));
    assert_eq!(f.scale, 2);
}

#[test_with(u32, u64)]
fn test_trailing_zeros<T: Digit>() {
    let u = T::MAX - T::ONE;
    assert_eq!(u.leading_zeros(), 0);
    assert_eq!(u.trailing_zeros(), 1);
}

#[test_with(u32, u64)]
fn test_eq_bigint<T: Digit>() {
    let n1 = BigFloat::<T>::default();
    let mut n2 = BigFloat::<T>::default();
    n2.int.sign = false;
    let n3 = BigInt::<T>::default();
    let mut n33 = BigInt::<T>::default();
    n33.sign = false;
    let n333 = BigUint::<T>::default();

    let n4 = BigFloat::<T>::from(BigInt::<T>::from_unsigned(T::MAX)) << T::NB_BITS;
    let n5 = BigFloat::<T>::from(BigInt::from_unsigned(T::MAX)) >> T::NB_BITS;
    let n6 = BigInt::<T>::from(vec![T::ZERO, T::MAX]);
    let n7 = BigInt::<T>::from(vec![T::MAX]);
    let n8 = BigUint::<T>::new(T::MAX);
    let n9 = BigUint::<T>::from(vec![T::ZERO, T::MAX]);

    assert_eq!(n1, n2);
    assert_eq!(n1, n3);
    assert_eq!(n1, n33);
    assert_eq!(n2, n3);
    assert_eq!(n2, n33);
    assert_eq!(n1, n333);
    assert_eq!(n2, n333);

    assert_eq!(n4, n6);
    assert_eq!(n4, n9);
    assert_ne!(n4, n5);
    assert_ne!(n4, n7);
    assert_ne!(n4, n8);
    assert_ne!(n5, n6);
    assert_ne!(n5, n7);
}

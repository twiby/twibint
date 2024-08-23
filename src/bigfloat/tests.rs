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

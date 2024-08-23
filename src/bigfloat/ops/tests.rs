use crate::traits::Digit;
use crate::BigFloat;
use crate::BigInt;
use typed_test_gen::test_with;

use crate::BigUint;

#[test_with(u32, u64)]
fn shl_1<T: Digit>() {
    let int = BigInt::<T>::from(BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]));

    let mut f = BigFloat::from(int.clone());

    f <<= T::NB_BITS;
    assert_eq!(f.int.uint.val, vec![T::MAX, T::MAX, T::MAX]);
    assert_eq!(f.scale, 1);

    f <<= 20 * T::NB_BITS;
    assert_eq!(f.int.uint.val, vec![T::MAX, T::MAX, T::MAX]);
    assert_eq!(f.scale, 21);
}

#[test_with(u32, u64)]
fn shl_2<T: Digit>() {
    let int = BigInt::<T>::from(BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]));

    let mut f = BigFloat::from(int.clone());
    f <<= 1;

    assert_eq!(
        f.int.uint.val,
        vec![T::MAX - T::ONE, T::MAX, T::MAX, T::ONE]
    );
    assert_eq!(f.scale, 0);
}

#[test_with(u32, u64)]
fn shl_3<T: Digit>() {
    let int = BigInt::<T>::from(BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]));

    let mut f = BigFloat::from(int.clone());
    f <<= T::NB_BITS + 1;

    assert_eq!(
        f.int.uint.val,
        vec![T::MAX - T::ONE, T::MAX, T::MAX, T::ONE]
    );
    assert_eq!(f.scale, 1);
}

#[test_with(u32, u64)]
fn shl_4<T: Digit>() {
    let int = BigInt::<T>::from(BigUint::<T>::from(vec![T::MAX - T::ONE, T::MAX, T::MAX]));

    let mut f = BigFloat::from(int.clone());
    f <<= T::NB_BITS - 1;

    assert_eq!(f.int.uint.val, vec![T::MAX, T::MAX, T::MAX >> 1]);
    assert_eq!(f.scale, 1);
}

#[test_with(u32, u64)]
fn shr_0<T: Digit>() {
    let int = BigInt::<T>::from(BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]));

    let mut f = BigFloat::from(int.clone());

    f >>= 0;
    assert_eq!(f.int.uint.val, vec![T::MAX, T::MAX, T::MAX]);
    assert_eq!(f.scale, 0);
}

#[test_with(u32, u64)]
fn shr_1<T: Digit>() {
    let int = BigInt::<T>::from(BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]));

    let mut f = BigFloat::from(int.clone());

    f >>= T::NB_BITS;
    assert_eq!(f.int.uint.val, vec![T::MAX, T::MAX, T::MAX]);
    assert_eq!(f.scale, -1);

    f >>= 20 * T::NB_BITS;
    assert_eq!(f.int.uint.val, vec![T::MAX, T::MAX, T::MAX]);
    assert_eq!(f.scale, -21);
}

#[test_with(u32, u64)]
fn shr_2<T: Digit>() {
    let int = BigInt::<T>::from(BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]));

    let mut f = BigFloat::from(int.clone());
    f >>= 1;

    assert_eq!(
        f.int.uint.val,
        vec![T::ONE << T::NB_BITS - 1, T::MAX, T::MAX, T::MAX >> 1]
    );
    assert_eq!(f.scale, -1);
}

#[test_with(u32, u64)]
fn shr_3<T: Digit>() {
    let int = BigInt::<T>::from(BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]));

    let mut f = BigFloat::from(int.clone());
    f >>= T::NB_BITS + 1;

    assert_eq!(
        f.int.uint.val,
        vec![T::ONE << T::NB_BITS - 1, T::MAX, T::MAX, T::MAX >> 1]
    );
    assert_eq!(f.scale, -2);
}

#[test_with(u32, u64)]
fn shr_4<T: Digit>() {
    let int = BigInt::<T>::from(BigUint::<T>::from(vec![T::MAX, T::MAX, T::MAX]));

    let mut f = BigFloat::from(int.clone());

    f <<= 1;
    assert_eq!(
        f.int.uint.val,
        vec![T::MAX - T::ONE, T::MAX, T::MAX, T::ONE]
    );
    assert_eq!(f.scale, 0);

    f >>= 1 + T::NB_BITS;
    assert_eq!(f.int.uint.val, vec![T::MAX, T::MAX, T::MAX]);
    assert_eq!(f.scale, -1);
}

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

#[test_with(u32, u64)]
fn shift_coherence<T: Digit>() {
    let mut n1 = BigInt::from(vec![T::ZERO, T::ZERO, T::MAX, T::MAX >> 5, T::ONE << 10]);
    n1.sign = false;
    let n2 = BigFloat::from(n1.clone());

    assert_eq!(n1, n2);
    assert_eq!(&n1 << T::NB_BITS, &n2 << T::NB_BITS);
    assert_eq!(&n1 << 5, &n2 << 5);
    assert_eq!(&n1 << T::NB_BITS + 5, &n2 << T::NB_BITS + 5);
}

#[test_with(u32, u64)]
fn mul<T: Digit>() {
    let a = BigFloat::from(vec![T::ONE, T::ONE]);
    let b = BigFloat::from(vec![T::ONE, T::ONE]);

    let c = a * b;
    assert_eq!(c.scale, 0);
    assert_eq!(c.int.uint.val, vec![T::ONE, T::ONE + T::ONE, T::ONE]);
}

#[test_with(u32, u64)]
fn mul_2<T: Digit>() {
    let a = BigFloat::from(vec![T::ONE, T::ONE]) << 2 * T::NB_BITS;
    let b = BigFloat::from(vec![T::ONE, T::ONE]);

    let c = a * b;
    assert_eq!(c.scale, 2);
    assert_eq!(c.int.uint.val, vec![T::ONE, T::ONE + T::ONE, T::ONE]);
}

#[test_with(u32, u64)]
fn mul_3<T: Digit>() {
    let a = BigFloat::from(vec![T::ONE, T::ONE]);
    let b = BigFloat::from(vec![T::ONE, T::ONE]) << 2 * T::NB_BITS;

    let c = a * b;
    assert_eq!(c.scale, 2);
    assert_eq!(c.int.uint.val, vec![T::ONE, T::ONE + T::ONE, T::ONE]);
}

#[test_with(u32, u64)]
fn mul_4<T: Digit>() {
    let a = BigFloat::from(vec![T::ONE, T::ONE]) << 3 * T::NB_BITS;
    let b = BigFloat::from(vec![T::ONE, T::ONE]) << 2 * T::NB_BITS;

    let c = a * b;
    assert_eq!(c.scale, 5);
    assert_eq!(c.int.uint.val, vec![T::ONE, T::ONE + T::ONE, T::ONE]);
}

#[test_with(u32, u64)]
fn mul_5<T: Digit>() {
    let a = BigFloat::from(vec![T::ONE, T::ONE]) >> 3 * T::NB_BITS;
    let b = BigFloat::from(vec![T::ONE, T::ONE]) << 2 * T::NB_BITS;

    let c = a * b;
    assert_eq!(c.scale, -1);
    assert_eq!(c.int.uint.val, vec![T::ONE, T::ONE + T::ONE, T::ONE]);
}

#[test_with(u32, u64)]
fn mul_6<T: Digit>() {
    let a = BigFloat::from(vec![T::ONE << T::NB_BITS - 1]);
    let b = BigFloat::from(vec![T::ONE + T::ONE]);

    let c = a * b;
    assert_eq!(c.scale, 1);
    assert_eq!(c.int.uint.val, vec![T::ONE]);
}

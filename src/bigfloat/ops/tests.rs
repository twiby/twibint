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
fn shl_5<T: Digit>() {
    let int = BigInt::<T>::from(BigUint::<T>::from(vec![T::MAX - T::ONE, T::MAX, T::MAX]));

    let mut f = BigFloat::from(int.clone());
    f.shl(T::NB_BITS - 1);

    assert_eq!(f.int.uint.val, vec![T::ZERO, T::MAX, T::MAX, T::MAX >> 1]);
    assert_eq!(f.scale, 0);
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

#[test_with(u32, u64)]
fn unsigned_add_bigger_scale<T: Digit>() {
    let a = BigFloat::from(vec![T::MAX, T::MAX]);
    let b = vec![T::ONE];

    let mut c1 = a.clone();
    c1.add_assign(true, 0, &b);
    assert_eq!(c1.int.uint.val, vec![T::ONE]);
    assert_eq!(c1.scale, 2);

    let mut c2 = a.clone();
    c2.add_assign(true, 1, &b);
    assert_eq!(c2.int.uint.val, vec![T::MAX, T::ZERO, T::ONE]);
    assert_eq!(c2.scale, 0);

    let mut c3 = a.clone();
    c3.add_assign(true, 2, &b);
    assert_eq!(c3.int.uint.val, vec![T::MAX, T::MAX, T::ONE]);
    assert_eq!(c3.scale, 0);
}

#[test_with(u32, u64)]
fn unsigned_add_bigger_scale_2<T: Digit>() {
    let mut a = BigFloat::from(vec![T::MAX, T::MAX]);
    a.scale = -10;
    let b = vec![T::ONE];

    let mut c1 = a.clone();
    c1.add_assign(true, -10, &b);
    assert_eq!(c1.int.uint.val, vec![T::ONE]);
    assert_eq!(c1.scale, -8);

    let mut c2 = a.clone();
    c2.add_assign(true, -9, &b);
    assert_eq!(c2.int.uint.val, vec![T::MAX, T::ZERO, T::ONE]);
    assert_eq!(c2.scale, -10);

    let mut c3 = a.clone();
    c3.add_assign(true, -8, &b);
    assert_eq!(c3.int.uint.val, vec![T::MAX, T::MAX, T::ONE]);
    assert_eq!(c3.scale, -10);
}

#[test_with(u32, u64)]
fn unsigned_add_smaller_scale<T: Digit>() {
    let a = BigFloat::from(vec![T::MAX, T::MAX]);
    let b = vec![T::ONE, T::ONE];

    let mut c1 = a.clone();
    c1.add_assign(true, -1, &b);
    assert_eq!(c1.int.uint.val, vec![T::ONE, T::ZERO, T::ZERO, T::ONE]);
    assert_eq!(c1.scale, -1);

    let mut c2 = a.clone();
    c2.add_assign(true, -2, &b);
    assert_eq!(c2.int.uint.val, vec![T::ONE, T::ONE, T::MAX, T::MAX]);
    assert_eq!(c2.scale, -2);

    let mut c3 = a.clone();
    c3.add_assign(true, -3, &b);
    assert_eq!(
        c3.int.uint.val,
        vec![T::ONE, T::ONE, T::ZERO, T::MAX, T::MAX]
    );
    assert_eq!(c3.scale, -3);
}

#[test_with(u32, u64)]
fn unsigned_sub_bigger_scale<T: Digit>() {
    let a = BigFloat::from(vec![T::MAX, T::MAX]);
    let b = vec![T::ONE];

    let mut c1 = a.clone();
    c1.sub_assign(true, 0, &b);
    assert_eq!(c1.int.uint.val, vec![T::MAX - T::ONE, T::MAX]);
    assert_eq!(c1.scale, 0);

    let mut c2 = a.clone();
    c2.sub_assign(true, 1, &b);
    assert_eq!(c2.int.uint.val, vec![T::MAX, T::MAX - T::ONE]);
    assert_eq!(c2.scale, 0);

    let mut c2 = a.clone();
    c2.sub_assign(true, -1, &b);
    assert_eq!(c2.int.uint.val, vec![T::MAX, T::MAX - T::ONE, T::MAX]);
    assert_eq!(c2.scale, -1);

    let mut c2 = a.clone();
    c2.sub_assign(true, -2, &b);
    assert_eq!(
        c2.int.uint.val,
        vec![T::MAX, T::MAX, T::MAX - T::ONE, T::MAX]
    );
    assert_eq!(c2.scale, -2);
}

#[test_with(u32, u64)]
fn add<T: Digit>() {
    let a = BigFloat::from(vec![T::MAX, T::MAX]);
    let b = BigFloat::from(vec![T::ONE]);

    let c1 = &a + &b;
    assert_eq!(c1.int.uint.val, vec![T::ONE]);
    assert_eq!(c1.scale, 2);

    let c2 = &a + (&b << T::NB_BITS);
    assert_eq!(c2.int.uint.val, vec![T::MAX, T::ZERO, T::ONE]);
    assert_eq!(c2.scale, 0);

    let c3 = &a + (&b << (2 * T::NB_BITS));
    assert_eq!(c3.int.uint.val, vec![T::MAX, T::MAX, T::ONE]);
    assert_eq!(c3.scale, 0);
}

#[test_with(u32, u64)]
fn add_2<T: Digit>() {
    let mut a = BigFloat::from(vec![T::MAX, T::MAX]);
    a.scale = -10;
    let b = BigFloat::from(vec![T::ONE]);

    let c1 = &a + (&b >> (10 * T::NB_BITS));
    assert_eq!(c1.int.uint.val, vec![T::ONE]);
    assert_eq!(c1.scale, -8);

    let c2 = &a + (&b >> (9 * T::NB_BITS));
    assert_eq!(c2.int.uint.val, vec![T::MAX, T::ZERO, T::ONE]);
    assert_eq!(c2.scale, -10);

    let c3 = &a + (&b >> (8 * T::NB_BITS));
    assert_eq!(c3.int.uint.val, vec![T::MAX, T::MAX, T::ONE]);
    assert_eq!(c3.scale, -10);
}

#[test_with(u32, u64)]
fn add_3<T: Digit>() {
    let a = BigFloat::from(vec![T::MAX, T::MAX]);
    let b = BigFloat::from(vec![T::ONE, T::ONE]);

    let c1 = &a + (&b >> T::NB_BITS);
    assert_eq!(c1.int.uint.val, vec![T::ONE, T::ZERO, T::ZERO, T::ONE]);
    assert_eq!(c1.scale, -1);

    let c2 = &a + (&b >> (2 * T::NB_BITS));
    assert_eq!(c2.int.uint.val, vec![T::ONE, T::ONE, T::MAX, T::MAX]);
    assert_eq!(c2.scale, -2);

    let c3 = &a + (&b >> (3 * T::NB_BITS));
    assert_eq!(
        c3.int.uint.val,
        vec![T::ONE, T::ONE, T::ZERO, T::MAX, T::MAX]
    );
    assert_eq!(c3.scale, -3);
}

#[test_with(u32, u64)]
fn sub<T: Digit>() {
    let a = BigFloat::from(vec![T::MAX, T::MAX]);
    let b = BigFloat::from(vec![T::ONE]);

    let c1 = &a - &b;
    assert_eq!(c1.int.uint.val, vec![T::MAX - T::ONE, T::MAX]);
    assert_eq!(c1.scale, 0);

    let c2 = &a - (&b << T::NB_BITS);
    assert_eq!(c2.int.uint.val, vec![T::MAX, T::MAX - T::ONE]);
    assert_eq!(c2.scale, 0);

    let c2 = &a - (&b >> T::NB_BITS);
    assert_eq!(c2.int.uint.val, vec![T::MAX, T::MAX - T::ONE, T::MAX]);
    assert_eq!(c2.scale, -1);

    let c2 = &a - (&b >> (2 * T::NB_BITS));
    assert_eq!(
        c2.int.uint.val,
        vec![T::MAX, T::MAX, T::MAX - T::ONE, T::MAX]
    );
    assert_eq!(c2.scale, -2);
}

#[test_with(u32, u64)]
fn sub2<T: Digit>() {
    let a = BigFloat::from(vec![T::MAX, T::MAX]);
    let b = BigFloat::from(vec![T::ONE, T::ZERO, T::ONE]) >> 3 * T::NB_BITS;

    let c1 = &a - b.clone();
    assert_eq!(
        c1.int.uint.val,
        vec![T::MAX, T::MAX, T::MAX - T::ONE, T::MAX - T::ONE, T::MAX]
    );
    assert_eq!(c1.scale, -3);
}

#[test_with(u32, u64)]
fn sub3<T: Digit>() {
    let a = BigFloat::from(vec![T::MAX, T::MAX]);
    let b = BigFloat::from(vec![T::ONE, T::MAX, T::ONE]) >> 3 * T::NB_BITS;

    let c1 = &a - b.clone();
    assert_eq!(
        c1.int.uint.val,
        vec![T::MAX, T::ZERO, T::MAX - T::ONE, T::MAX - T::ONE, T::MAX]
    );
    assert_eq!(c1.scale, -3);
}

use crate::traits::{Digit, Pow, TrueDiv};
use crate::{BigInt, BigUint};

fn neg<T: Digit>() {
    let n1: BigInt<T> = "-12345678901234567".parse().unwrap();
    let n2: BigInt<T> = "12345678901234567".parse().unwrap();
    assert_eq!(n1, -&n2);
    assert_eq!(-n1, n2);
}

fn add_full_test<T: Digit>() {
    assert_eq!(
        BigInt::<T>::from("4294967295") + BigInt::<T>::from("4294967295"),
        BigInt::<T>::from("8589934590")
    );
    assert_eq!(
        BigInt::<T>::from("4294967295") + BigInt::<T>::from("-4294967295"),
        BigInt::<T>::from("0")
    );
    assert_eq!(
        BigInt::<T>::from("4294967295") + BigInt::<T>::from("-4294967295"),
        BigInt::<T>::from("-0")
    );
    assert_eq!(
        BigInt::<T>::from("-4294967295") + BigInt::<T>::from("-4294967295"),
        BigInt::<T>::from("-8589934590")
    );
    assert_eq!(
        BigInt::<T>::from("4294967296") + BigInt::<T>::from("-4294967295"),
        BigInt::<T>::from("1")
    );
    assert_eq!(
        BigInt::<T>::from("-4294967296") + BigInt::<T>::from("4294967295"),
        BigInt::<T>::from("-1")
    );
    assert_eq!(
        BigInt::<T>::from("4294967295") + BigInt::<T>::from("-4294967296"),
        BigInt::<T>::from("-1")
    );
    assert_eq!(
        BigInt::<T>::from("-4294967295") + BigInt::<T>::from("4294967296"),
        BigInt::<T>::from("1")
    );
}

fn sub_full_test<T: Digit>() {
    assert_eq!(
        BigInt::<T>::from("4294967295") - BigInt::<T>::from("4294967295"),
        BigInt::<T>::from("0")
    );
    assert_eq!(
        BigInt::<T>::from("4294967295") - BigInt::<T>::from("4294967295"),
        BigInt::<T>::from("-0")
    );

    assert_eq!(
        BigInt::<T>::from("-4294967295") - BigInt::<T>::from("4294967295"),
        BigInt::<T>::from("-8589934590")
    );
    assert_eq!(
        BigInt::<T>::from("4294967296") - BigInt::<T>::from("4294967295"),
        BigInt::<T>::from("1")
    );
    assert_eq!(
        BigInt::<T>::from("-4294967296") - BigInt::<T>::from("-4294967295"),
        BigInt::<T>::from("-1")
    );
    assert_eq!(
        BigInt::<T>::from("-4294967295") - BigInt::<T>::from("-4294967296"),
        BigInt::<T>::from("1")
    );
    assert_eq!(
        BigInt::<T>::from("4294967295") - BigInt::<T>::from("4294967296"),
        BigInt::<T>::from("-1")
    );
}

fn sum<T: Digit>() {
    let values = vec![T::MAX; 10];
    let mut ret = T::MAX.to_string();
    ret.push('0');

    let n1: BigInt<T> = values.iter().sum();
    println!("{:?}", n1.to_string());
    assert_eq!(n1.to_string(), ret);

    let n2: BigInt<T> = values
        .iter()
        .map(|n| -BigInt::<T>::from(BigUint::<T>::new(*n)))
        .sum();
    let mut ret2 = "-".to_string();
    ret2.push_str(&ret);
    assert_eq!(n2.to_string(), ret2);
}

fn mul_full_test<T: Digit>() {
    assert_eq!(
        BigInt::<T>::from("4294967295") * BigInt::<T>::from("4294967295"),
        BigInt::<T>::from("18446744065119617025")
    );
    assert_eq!(
        BigInt::<T>::from("-4294967295") * BigInt::<T>::from("-4294967295"),
        BigInt::<T>::from("18446744065119617025")
    );
    assert_eq!(
        BigInt::<T>::from("-4294967295") * BigInt::<T>::from("4294967295"),
        BigInt::<T>::from("-18446744065119617025")
    );
    assert_eq!(
        BigInt::<T>::from("4294967295") * BigInt::<T>::from("-4294967295"),
        BigInt::<T>::from("-18446744065119617025")
    );
}

fn div_rem_1<T: Digit>() {
    let two = T::ONE + T::ONE;
    let three = two + T::ONE;

    let (q, r) = (
        BigInt::<T>::from("10") / three,
        BigInt::<T>::from("10") % three,
    );
    assert_eq!(q, BigInt::<T>::from(3));
    assert_eq!(r, T::ONE);

    let (q, r) = (
        BigInt::<T>::from("-10") / three,
        BigInt::<T>::from("-10") % three,
    );
    assert_eq!(q, BigInt::<T>::from(-4));
    assert_eq!(r, two);
}

fn div_rem_2<T: Digit>() {
    let (q, r) = (
        BigInt::<T>::from("10") / BigInt::<T>::from(3),
        BigInt::<T>::from("10") % BigInt::<T>::from(3),
    );
    assert_eq!(q, BigInt::<T>::from(3));
    assert_eq!(r, BigInt::<T>::from(1));

    let (q, r) = (
        BigInt::<T>::from("-10") / BigInt::<T>::from(-3),
        BigInt::<T>::from("-10") % BigInt::<T>::from(-3),
    );
    assert_eq!(q, BigInt::<T>::from(3));
    assert_eq!(r, BigInt::<T>::from(-1));

    let (q, r) = (
        BigInt::<T>::from("-10") / BigInt::<T>::from(3),
        BigInt::<T>::from("-10") % BigInt::<T>::from(3),
    );
    assert_eq!(q, BigInt::<T>::from(-4));
    assert_eq!(r, BigInt::<T>::from(2));

    let (q, r) = (
        BigInt::<T>::from("10") / BigInt::<T>::from(-3),
        BigInt::<T>::from("10") % BigInt::<T>::from(-3),
    );
    assert_eq!(q, BigInt::<T>::from(-4));
    assert_eq!(r, BigInt::<T>::from(-2));
}

fn pow<T: Digit>() {
    let n = BigInt::<T>::from(-5i32);
    assert_eq!(n.pow(0), BigInt::<T>::from(1i32));
    assert_eq!(n.pow(1), BigInt::<T>::from(-5i32));
    assert_eq!(n.pow(2), BigInt::<T>::from(25i32));
    assert_eq!(n.pow(3), BigInt::<T>::from(-125i32));

    let n = BigInt::<T>::from(128i32);
    let n2 = n.pow(50);
    assert_eq!(
        n2,
        BigInt::<T>::from(
            "2293498615990071511610820895302086940796564989168281\
            123737588839386922876088484808070018553110125686554624"
        )
    );

    let n = BigInt::<T>::from(-128i32);
    let n2 = n.pow(16);
    assert_eq!(n2, BigInt::<T>::from("5192296858534827628530496329220096"));

    let n = BigInt::<T>::from(-128i32);
    let n2 = n.pow(15);
    assert_eq!(n2, BigInt::<T>::from("-40564819207303340847894502572032"));
}

fn truediv<T: Digit>() {
    let n1 = BigInt::<T>::from("123456678890123345567789");
    let n2 = BigInt::<T>::from("-12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = -10000000270550.242f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);

    let n2 = BigInt::<T>::from("-123456678890123345567789");
    let n1 = BigInt::<T>::from("-12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = 9.999999729449765e-14f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);
}

fn not<T: Digit>() {
    let n1 = BigInt::<T>::from(-10);
    let n2 = BigInt::<T>::from(9);
    assert_eq!(!&n1, n2);
    assert_eq!(!n2, n1);
}

fn xor<T: Digit>() {
    let n1 = BigInt::<T>::from(10);
    let n2 = BigInt::<T>::from(8);

    assert_eq!(&n1 ^ &n2, BigInt::<T>::from(2));
    assert_eq!(&-&n1 ^ &-&n2, BigInt::<T>::from(14));
    assert_eq!(&n1 ^ &-&n2, BigInt::<T>::from(-14));
    assert_eq!(&-&n1 ^ &n2, BigInt::<T>::from(-2));
}

fn and<T: Digit>() {
    let n1 = BigInt::<T>::from(10);
    let n2 = BigInt::<T>::from(3);

    assert_eq!(&n1 & &n2, BigInt::<T>::from(2));
    assert_eq!(&-&n1 & &-&n2, BigInt::<T>::from(-12));
    assert_eq!(&n1 & &-&n2, BigInt::<T>::from(8));
    assert_eq!(&-&n1 & &n2, BigInt::<T>::from(2));
}

fn or<T: Digit>() {
    let n1 = BigInt::<T>::from(10);
    let n2 = BigInt::<T>::from(3);

    assert_eq!(&n1 | &n2, BigInt::<T>::from(11));
    assert_eq!(&-&n1 | &-&n2, BigInt::<T>::from(-1));
    assert_eq!(&n1 | &-&n2, BigInt::<T>::from(-1));
    assert_eq!(&-&n1 | &n2, BigInt::<T>::from(-9));
}

fn shifts<T: Digit>() {
    assert_eq!(BigInt::<T>::from(10) >> 2, BigInt::<T>::from(2));
    assert_eq!(BigInt::<T>::from(-10) >> 2, BigInt::<T>::from(-3));

    let mut n = BigInt::<T>::from(10);
    n >>= 2;
    assert_eq!(n, BigInt::<T>::from(2));

    let mut n = BigInt::<T>::from(-10);
    n >>= 2;
    assert_eq!(n, BigInt::<T>::from(-3));

    assert_eq!(BigInt::<T>::from(10) << 2, BigInt::<T>::from(40));
    assert_eq!(BigInt::<T>::from(-10) << 2, BigInt::<T>::from(-40));

    let mut n = BigInt::<T>::from(10);
    n <<= 2;
    assert_eq!(n, BigInt::<T>::from(40));

    let mut n = BigInt::<T>::from(-10);
    n <<= 2;
    assert_eq!(n, BigInt::<T>::from(-40));
}

test_functions!(
    neg, neg_u32, neg_u64;
    add_full_test, add_full_test_u32, add_full_test_u64;
    sub_full_test, sub_full_test_u32, sub_full_test_u64;
    sum, sum_u32, sum_u64;
    mul_full_test, mul_full_test_u32, mul_full_test_u64;
    div_rem_1, div_rem_1_u32, div_rem_1_u64;
    div_rem_2, div_rem_2_u32, div_rem_2_u64;
    pow, pow_u32, pow_u64;
    truediv, truediv_u32, truediv_u64;
    not, not_u32, not_u64;
    xor, xor_u32, xor_u64;
    and, and_u32, and_u64;
    or, or_u32, or_u64;
    shifts, shifts_u32, shifts_u64;
);

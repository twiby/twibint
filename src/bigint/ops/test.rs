use crate::traits::{Pow, TrueDiv};
use crate::BigInt;

#[test]
fn neg() {
    let n1: BigInt = "-12345678901234567".parse().unwrap();
    let n2: BigInt = "12345678901234567".parse().unwrap();
    assert_eq!(n1, -&n2);
    assert_eq!(-n1, n2);
}

#[test]
fn add_full_test() {
    assert_eq!(
        bigint!("4294967295") + bigint!("4294967295"),
        bigint!("8589934590")
    );
    assert_eq!(bigint!("4294967295") + bigint!("-4294967295"), bigint!("0"));
    assert_eq!(
        bigint!("4294967295") + bigint!("-4294967295"),
        bigint!("-0")
    );
    assert_eq!(
        bigint!("-4294967295") + bigint!("-4294967295"),
        bigint!("-8589934590")
    );
    assert_eq!(bigint!("4294967296") + bigint!("-4294967295"), bigint!("1"));
    assert_eq!(
        bigint!("-4294967296") + bigint!("4294967295"),
        bigint!("-1")
    );
    assert_eq!(
        bigint!("4294967295") + bigint!("-4294967296"),
        bigint!("-1")
    );
    assert_eq!(bigint!("-4294967295") + bigint!("4294967296"), bigint!("1"));
}
#[test]
fn sub_full_test() {
    assert_eq!(bigint!("4294967295") - bigint!("4294967295"), bigint!("0"));
    assert_eq!(bigint!("4294967295") - bigint!("4294967295"), bigint!("-0"));

    assert_eq!(
        bigint!("-4294967295") - bigint!("4294967295"),
        bigint!("-8589934590")
    );
    assert_eq!(bigint!("4294967296") - bigint!("4294967295"), bigint!("1"));
    assert_eq!(
        bigint!("-4294967296") - bigint!("-4294967295"),
        bigint!("-1")
    );
    assert_eq!(
        bigint!("-4294967295") - bigint!("-4294967296"),
        bigint!("1")
    );
    assert_eq!(bigint!("4294967295") - bigint!("4294967296"), bigint!("-1"));
}

#[test]
fn sum() {
    let values = vec![u32::MAX; 10];

    let n1: BigInt = values.iter().sum();
    println!("{:?}", n1.to_string());
    assert_eq!(n1, bigint!("42949672950"));

    let big_values = values.iter().map(|n| -BigInt::from(*n)).collect::<Vec<_>>();
    let n2: BigInt = big_values.iter().sum();
    assert_eq!(n2, bigint!("-42949672950"));
}

#[test]
fn mul_full_test() {
    assert_eq!(
        bigint!("4294967295") * bigint!("4294967295"),
        bigint!("18446744065119617025")
    );
    assert_eq!(
        bigint!("-4294967295") * bigint!("-4294967295"),
        bigint!("18446744065119617025")
    );
    assert_eq!(
        bigint!("-4294967295") * bigint!("4294967295"),
        bigint!("-18446744065119617025")
    );
    assert_eq!(
        bigint!("4294967295") * bigint!("-4294967295"),
        bigint!("-18446744065119617025")
    );
}

#[test]
fn div_rem_u32() {
    let (q, r) = (bigint!("10") / 3u32, bigint!("10") % 3u32);
    assert_eq!(q, bigint!(3));
    assert_eq!(r, 1);

    let (q, r) = (bigint!("-10") / 3u32, bigint!("-10") % 3u32);
    assert_eq!(q, bigint!(-4));
    assert_eq!(r, 2);
}

#[test]
fn div_rem_i32() {
    let (q, r) = (bigint!("10") / 3i32, bigint!("10") % 3i32);
    assert_eq!(q, bigint!(3));
    assert_eq!(r, 1);

    let (q, r) = (bigint!("-10") / -3i32, bigint!("-10") % -3i32);
    assert_eq!(q, bigint!(3));
    assert_eq!(r, -1);

    let (q, r) = (bigint!("-10") / 3i32, bigint!("-10") % 3i32);
    assert_eq!(q, bigint!(-4));
    assert_eq!(r, 2);

    let (q, r) = (bigint!("10") / -3i32, bigint!("10") % -3i32);
    assert_eq!(q, bigint!(-4));
    assert_eq!(r, -2);
}

#[test]
fn div_rem() {
    let (q, r) = (bigint!("10") / bigint!(3), bigint!("10") % bigint!(3));
    assert_eq!(q, bigint!(3));
    assert_eq!(r, bigint!(1));

    let (q, r) = (bigint!("-10") / bigint!(-3), bigint!("-10") % bigint!(-3));
    assert_eq!(q, bigint!(3));
    assert_eq!(r, bigint!(-1));

    let (q, r) = (bigint!("-10") / bigint!(3), bigint!("-10") % bigint!(3));
    assert_eq!(q, bigint!(-4));
    assert_eq!(r, bigint!(2));

    let (q, r) = (bigint!("10") / bigint!(-3), bigint!("10") % bigint!(-3));
    assert_eq!(q, bigint!(-4));
    assert_eq!(r, bigint!(-2));
}

#[test]
fn pow() {
    let n = bigint!(-5i32);
    assert_eq!(n.pow(0), bigint!(1i32));
    assert_eq!(n.pow(1), bigint!(-5i32));
    assert_eq!(n.pow(2), bigint!(25i32));
    assert_eq!(n.pow(3), bigint!(-125i32));

    let n = bigint!(128i32);
    let n2 = n.pow(50);
    assert_eq!(
        n2,
        bigint!(
            "2293498615990071511610820895302086940796564989168281\
            123737588839386922876088484808070018553110125686554624"
        )
    );

    let n = bigint!(-128i32);
    let n2 = n.pow(16);
    assert_eq!(n2, bigint!("5192296858534827628530496329220096"));

    let n = bigint!(-128i32);
    let n2 = n.pow(15);
    assert_eq!(n2, bigint!("-40564819207303340847894502572032"));
}
#[test]
fn truediv() {
    let n1 = bigint!("123456678890123345567789");
    let n2 = bigint!("-12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = -10000000270550.242f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);

    let n2 = bigint!("-123456678890123345567789");
    let n1 = bigint!("-12345667555");
    let f = n1.truediv(&n2).unwrap();
    let true_div = 9.999999729449765e-14f64;
    println!("{:b}", f.to_bits());
    println!("{:b}", true_div.to_bits());
    assert_eq!(f, true_div);
}

#[test]
fn not() {
    let n1 = bigint!(-10);
    let n2 = bigint!(9);
    assert_eq!(!&n1, n2);
    assert_eq!(!n2, n1);
}

#[test]
fn xor() {
    let n1 = bigint!(10);
    let n2 = bigint!(8);

    assert_eq!(&n1 ^ &n2, bigint!(2));
    assert_eq!(&-&n1 ^ &-&n2, bigint!(14));
    assert_eq!(&n1 ^ &-&n2, bigint!(-14));
    assert_eq!(&-&n1 ^ &n2, bigint!(-2));
}

#[test]
fn and() {
    let n1 = bigint!(10);
    let n2 = bigint!(3);

    assert_eq!(&n1 & &n2, bigint!(2));
    assert_eq!(&-&n1 & &-&n2, bigint!(-12));
    assert_eq!(&n1 & &-&n2, bigint!(8));
    assert_eq!(&-&n1 & &n2, bigint!(2));
}

#[test]
fn or() {
    let n1 = bigint!(10);
    let n2 = bigint!(3);

    assert_eq!(&n1 | &n2, bigint!(11));
    assert_eq!(&-&n1 | &-&n2, bigint!(-1));
    assert_eq!(&n1 | &-&n2, bigint!(-1));
    assert_eq!(&-&n1 | &n2, bigint!(-9));
}

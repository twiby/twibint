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

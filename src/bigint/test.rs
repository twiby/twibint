use crate::{BigInt, BigUint};

#[test]
fn creation() {
    let n1 = bigint!(128u32);
    let n2 = bigint!(-129i32);
    let n3 = bigint!(129u64);
    let n4 = bigint!(-128i64);

    assert_eq!(
        n1,
        BigInt {
            uint: biguintvec![128],
            sign: true
        }
    );
    assert_eq!(
        n2,
        BigInt {
            uint: biguintvec![129],
            sign: false
        }
    );
    assert_eq!(
        n3,
        BigInt {
            uint: biguintvec![129],
            sign: true
        }
    );
    assert_eq!(
        n4,
        BigInt {
            uint: biguintvec![128],
            sign: false
        }
    );

    assert!(n1 > n2);
    assert!(n1 < n3);
    assert!(n1 > n4);
    assert!(n2 < n3);
    assert!(n2 < n4);
    assert!(n3 > n4);
}

#[test]
fn hash() {
    use std::collections::HashMap;
    let mut map = HashMap::<BigInt, String>::new();

    let n1 = bigintvec![1, 2, 3];
    let mut n2 = n1.clone();
    n2.sign = false;

    map.insert(n1.clone(), "first".to_string());
    map.insert(n2.clone(), "second".to_string());

    assert!(map.contains_key(&n1));
    assert!(map.contains_key(&n2));
    assert_eq!(map[&n1], "first");
    assert_eq!(map[&n2], "second");
}

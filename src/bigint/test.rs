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

    map.insert(bigintvec![1, 2, 3], "first".to_string());
    map.insert(bigintvec![3, 2, 1], "second".to_string());

    assert!(map.contains_key(&bigintvec![1, 2, 3]));
    assert!(map.contains_key(&bigintvec![3, 2, 1]));
    assert_eq!(map[&bigintvec![1, 2, 3]], "first");
    assert_eq!(map[&bigintvec![3, 2, 1]], "second");
}

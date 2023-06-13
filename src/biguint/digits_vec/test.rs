use crate::biguint::digits_vec::Digits;

#[test]
fn new() {
    let d = Digits::new(123);

    assert_eq!(d.val, vec![3, 2, 1]);
}

#[test]
fn add() {
    let mut d = Digits::new(123);

    d.add_n_at_k(9, 0);
    assert_eq!(d.val, vec![2, 3, 1]);
}

#[test]
fn double() {
    let mut d = Digits::new(193);

    d.times_2();
    assert_eq!(d.val, vec![6, 8, 3]);

    d = Digits::new(922);

    d.times_2();
    assert_eq!(d.val, vec![4, 4, 8, 1]);
}

#[test]
fn string() {
    let d = Digits::new(1943);
    assert_eq!(String::from(&d), "1943");
}

#[test]
fn from_str() {
    let s = "123";

    let bg = Digits::from(s);

    assert_eq!(bg.val, vec![3, 2, 1]);
}

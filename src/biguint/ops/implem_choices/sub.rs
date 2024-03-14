use crate::traits::Digit;

pub(super) fn schoolbook_sub_assign<T: Digit>(rhs: &mut [T], lhs: &[T]) {
    let mut c1: bool;
    let mut c2: bool;
    let mut carry = false;
    for (a, b) in rhs.iter_mut().zip(lhs.iter()) {
        (*a, c1) = a.overflowing_sub(*b);
        (*a, c2) = a.overflowing_sub(T::from(carry));
        carry = c1 | c2;
    }

    // Potential carry propagation
    let mut n = lhs.len();
    while carry && n < rhs.len() {
        (rhs[n], carry) = rhs[n].overflowing_sub(T::from(carry));
        n += 1;
    }
}

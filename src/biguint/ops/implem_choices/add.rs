use crate::traits::{Digit, DoubleDigit};

pub(super) fn schoolbook_add_assign<T: Digit>(rhs: &mut [T], lhs: &[T]) -> bool {
    let mut carry = T::ZERO;

    for (a, b) in rhs.iter_mut().zip(lhs.iter()) {
        let full = a.to_double() + b.to_double() + carry.to_double();
        (*a, carry) = full.split();
    }

    // Potential carry propagation
    for val in rhs.iter_mut().skip(lhs.len()) {
        let full = val.to_double() + carry.to_double();
        (*val, carry) = full.split();
    }

    carry != T::ZERO
}

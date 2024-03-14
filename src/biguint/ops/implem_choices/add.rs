use crate::traits::{Digit, DoubleDigit};

pub(super) fn schoolbook_add_assign<T: Digit>(rhs: &mut [T], lhs: &[T], carry: bool) -> bool {
    let mut carry = T::from(carry);
    for (a, b) in rhs.iter_mut().zip(lhs.iter()) {
        let full = a.to_double() + b.to_double() + carry.to_double();
        (*a, carry) = full.split();
    }

    propagate_carry(&mut rhs[lhs.len()..], carry != T::ZERO)
}

pub(super) fn propagate_carry<T: Digit>(rhs: &mut [T], mut carry: bool) -> bool {
    let mut it = rhs.iter_mut();
    while carry {
        match it.next() {
            None => return carry,
            Some(val) => (*val, carry) = val.overflowing_add(T::from(carry)),
        };
    }
    carry
}

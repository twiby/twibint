use crate::traits::Digit;

pub(super) fn schoolbook_sub_assign<T: Digit>(rhs: &mut [T], lhs: &[T]) {
    let mut partial_carry_1: bool;
    let mut partial_carry_2: bool;
    let mut carry = false;
    for (a, b) in rhs.iter_mut().zip(lhs.iter()) {
        (*a, partial_carry_1) = a.overflowing_sub(*b);
        (*a, partial_carry_2) = a.overflowing_sub(T::from(carry));
        carry = partial_carry_1 | partial_carry_2;
    }

    for val in rhs.iter_mut().skip(lhs.len()) {
        (*val, carry) = val.overflowing_sub(T::from(carry));
    }
}

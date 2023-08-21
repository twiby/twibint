use crate::traits::{Digit, DoubleDigit};

use core::cmp::max;

pub(super) fn schoolbook_add_assign_mul<T: Digit>(ret: &mut [T], rhs: &[T], lhs: &[T]) {
    for (idx_1, b) in rhs.iter().enumerate() {
        let mut carry = T::ZERO;

        for (a, r) in lhs.iter().zip(&mut ret[idx_1..]) {
            let full = a.to_double() * b.to_double() + r.to_double() + carry.to_double();
            (*r, carry) = full.split();
        }

        ret[idx_1 + lhs.len()] = carry;
    }
}

pub(super) fn karatsuba<const THRESHOLD: usize, T: Digit>(rhs: &[T], lhs: &[T]) -> Vec<T> {
    let target_length = max(rhs.len(), lhs.len()).next_power_of_two();
    assert!(target_length < usize::MAX >> 1);

    let mut x = rhs.to_vec();
    let mut y = lhs.to_vec();
    x.resize(target_length, T::ZERO);
    y.resize(target_length, T::ZERO);

    let mut ret = vec![T::ZERO; target_length << 1];
    let mut buff = vec![T::ZERO; target_length << 1];
    _karatsuba::<THRESHOLD, _>(&mut ret, &x, &y, &mut buff);
    ret
}
fn _karatsuba<const THRESHOLD: usize, T: Digit>(
    ret: &mut [T],
    rhs: &[T],
    lhs: &[T],
    buff: &mut [T],
) {
    debug_assert!(rhs.len() == lhs.len());
    debug_assert!(rhs.len().is_power_of_two());
    debug_assert_eq!(ret.len(), 2 * rhs.len());
    debug_assert_eq!(buff.len(), 2 * rhs.len());

    let size = rhs.len();
    let half_size = size >> 1;

    // Early exit
    if size < THRESHOLD {
        schoolbook_add_assign_mul(ret, rhs, lhs);
        return;
    }

    let (x0, x1) = rhs.split_at(half_size);
    let (y0, y1) = lhs.split_at(half_size);

    // Compute (x0+x1) and (y0+y1), using ret as a buffer,
    // but specifically handle their last bit
    let (x_temp, y_temp) = ret[..size].split_at_mut(half_size);
    x_temp.copy_from_slice(x0);
    y_temp.copy_from_slice(y0);
    let x_carry = super::add::schoolbook_add_assign(x_temp, x1);
    let y_carry = super::add::schoolbook_add_assign(y_temp, y1);

    // compute z1 in a separate buffer
    // but specifically handle its last bit
    let (z1, new_buff) = buff.split_at_mut(size);
    let mut z1_last_bit = T::ZERO;
    _karatsuba::<THRESHOLD, _>(&mut z1[..size], x_temp, y_temp, new_buff);
    if x_carry {
        z1_last_bit += T::from(super::add::schoolbook_add_assign(
            &mut z1[half_size..],
            &y_temp,
        ));
    }
    if y_carry {
        z1_last_bit += T::from(super::add::schoolbook_add_assign(
            &mut z1[half_size..],
            &x_temp,
        ));
    }
    z1_last_bit += T::from(x_carry && y_carry);

    // z0 and z2
    ret[..size].fill(T::ZERO);
    new_buff.fill(T::ZERO);
    _karatsuba::<THRESHOLD, _>(&mut ret[..size], x0, y0, new_buff);
    new_buff.fill(T::ZERO);
    _karatsuba::<THRESHOLD, _>(&mut ret[size..], x1, y1, new_buff);

    // subtract z0 and z2 from z1
    let mut partial_carry_1: bool;
    let mut partial_carry_2: bool;
    let mut partial_carry_3: bool;
    let mut carry = T::ZERO;
    for i in 0..size {
        (z1[i], partial_carry_1) = z1[i].overflowing_sub(ret[i + size]);
        (z1[i], partial_carry_2) = z1[i].overflowing_sub(ret[i]);
        (z1[i], partial_carry_3) = z1[i].overflowing_sub(carry);
        carry = T::from(partial_carry_1) + T::from(partial_carry_2) + T::from(partial_carry_3);
    }
    (z1_last_bit, _) = z1_last_bit.overflowing_sub(carry);

    // add z1
    super::add::schoolbook_add_assign(&mut ret[half_size..], z1);
    super::add::schoolbook_add_assign(&mut ret[half_size + size..], &[z1_last_bit]);
}

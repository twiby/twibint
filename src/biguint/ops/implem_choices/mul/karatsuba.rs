use crate::traits::Digit;

use super::super::add_assign;
use super::super::sub_assign;
use super::schoolbook_mul;

// Below this number of digits, multiplication is schoolbook
#[cfg(debug_assertions)]
const KARATSUBA_INTERNAL_THRESHOLD: usize = 2;
#[cfg(debug_assertions)]
const KARATSUBA_EXTERNAL_THRESHOLD: usize = 2;

#[cfg(not(debug_assertions))]
const KARATSUBA_INTERNAL_THRESHOLD: usize = 20;
#[cfg(not(debug_assertions))]
const KARATSUBA_EXTERNAL_THRESHOLD: usize = 156;

pub(super) const KARATSUBA_EXTERNAL_THRESHOLD_SQUARED: usize =
    KARATSUBA_EXTERNAL_THRESHOLD * KARATSUBA_EXTERNAL_THRESHOLD;

pub(super) fn karatsuba<T: Digit>(rhs: &[T], lhs: &[T]) -> Vec<T> {
    let target_length = rhs.len().max(lhs.len()).next_power_of_two();
    assert!(target_length < usize::MAX >> 1);

    let mut x = rhs.to_vec();
    let mut y = lhs.to_vec();
    x.resize(target_length, T::ZERO);
    y.resize(target_length, T::ZERO);

    let mut ret = vec![T::ZERO; target_length << 1];
    let mut buff = vec![T::ZERO; target_length << 1];
    _karatsuba(&mut ret, &x, &y, &mut buff);
    ret.resize(rhs.len() + lhs.len(), T::ZERO);
    ret
}

fn _karatsuba<T: Digit>(ret: &mut [T], x: &[T], y: &[T], buff: &mut [T]) {
    debug_assert!(x.len() == y.len());
    debug_assert!(x.len().is_power_of_two());
    debug_assert_eq!(ret.len(), 2 * x.len());
    debug_assert_eq!(buff.len(), 2 * x.len());

    let size = x.len();
    let half_size = size >> 1;

    // Early exit
    if size < KARATSUBA_INTERNAL_THRESHOLD {
        schoolbook_mul(ret, x, y);
        return;
    }

    let (x0, x1) = x.split_at(half_size);
    let (y0, y1) = y.split_at(half_size);

    // Compute (x0+x1) and (y0+y1), using ret as a buffer,
    // but specifically handle their last bit
    let (x_temp, y_temp) = ret[..size].split_at_mut(half_size);
    x_temp.copy_from_slice(x0);
    y_temp.copy_from_slice(y0);
    let x_carry = add_assign(x_temp, x1);
    let y_carry = add_assign(y_temp, y1);

    // compute z1 in a separate buffer
    // but specifically handle its last bit
    let (z1, new_buff) = buff.split_at_mut(size);
    let mut z1_last_bit = T::ZERO;
    _karatsuba(&mut z1[..size], x_temp, y_temp, new_buff);
    if x_carry {
        z1_last_bit += T::from(add_assign(&mut z1[half_size..], &y_temp));
    }
    if y_carry {
        z1_last_bit += T::from(add_assign(&mut z1[half_size..], &x_temp));
    }
    z1_last_bit += T::from(x_carry && y_carry);

    // z0 and z2
    _karatsuba(&mut ret[..size], x0, y0, new_buff);
    _karatsuba(&mut ret[size..], x1, y1, new_buff);

    // subtract z0 and z2 from z1
    if sub_assign(z1, &ret[..size]) {
        z1_last_bit -= T::ONE;
    }
    if sub_assign(z1, &ret[size..size * 2]) {
        z1_last_bit -= T::ONE;
    }

    // add z1
    add_assign(&mut ret[half_size..], z1);
    add_assign(&mut ret[half_size + size..], &[z1_last_bit]);
}

use crate::traits::Digit;

use super::super::add_assign;
use super::super::sub_assign;
use super::schoolbook_mul;

// Below this number of digits, multiplication is schoolbook
#[cfg(debug_assertions)]
const KARATSUBA_INTERNAL_THRESHOLD: usize = 7;

#[cfg(not(debug_assertions))]
const KARATSUBA_INTERNAL_THRESHOLD: usize = 20;

fn allocate_buffer<T: Digit>(big: &[T], small: &[T]) -> Vec<T> {
    debug_assert!(big.len() >= small.len());
    let target_length = small.len().next_power_of_two();
    assert!(target_length < usize::MAX >> 1);
    vec![T::ZERO; target_length << 1]
}

pub(super) fn karatsuba<T: Digit>(ret: &mut [T], rhs: &[T], lhs: &[T]) {
    if rhs.len() < lhs.len() {
        return karatsuba(ret, lhs, rhs);
    }

    if exit_karatsuba(lhs.len()) {
        schoolbook_mul(ret, rhs, lhs);
    } else if rhs.len() == lhs.len() {
        let mut buff = allocate_buffer(rhs, lhs);
        symetric_karatsuba(ret, rhs, lhs, &mut buff);
    } else {
        let mut buff_1 = allocate_buffer(rhs, lhs);
        let mut buff_2 = allocate_buffer(rhs, lhs);
        asymetric_karatsuba(ret, rhs, lhs, &mut buff_1, &mut buff_2);
    }
}

/// multiplies big and small, puts the result in ret.
///
/// we assume big is larger than small, and that ret is filled with zeros
fn asymetric_karatsuba<'a, T: Digit>(
    mut ret: &mut [T],
    mut big: &'a [T],
    mut small: &'a [T],
    buff_1: &mut [T],
    buff_2: &mut [T],
) {
    debug_assert!(big.len() >= small.len());
    let mut half_size = small.len();
    let mut size = half_size << 1;
    let mut write_counter = 0;

    while !exit_karatsuba(small.len()) {
        symetric_karatsuba(&mut buff_1[..size], &big[..half_size], small, buff_2);

        if size > write_counter {
            ret[write_counter..size].copy_from_slice(&buff_1[write_counter..size]);
            add_assign(ret, &buff_1[..write_counter]);
        } else {
            add_assign(ret, &buff_1[..size]);
        }

        big = &big[half_size..];
        ret = &mut ret[half_size..];
        write_counter = write_counter.max(size + 1) - half_size;

        if big.len() < small.len() {
            (small, big) = (big, small);
            half_size = small.len();
            size = half_size << 1;
        }
    }

    if half_size > 0 {
        size = big.len() + small.len();
        schoolbook_mul(&mut buff_1[..size], big, small);
        if size > write_counter {
            ret[write_counter..size].copy_from_slice(&buff_1[write_counter..size]);
            add_assign(ret, &buff_1[..write_counter]);
        } else {
            add_assign(ret, &buff_1[..size]);
        }
    }
}

#[inline]
fn exit_karatsuba(size: usize) -> bool {
    size < KARATSUBA_INTERNAL_THRESHOLD
}

/// multiplies big and small, puts the result in ret.
///
/// we assume x and y have the same size
/// ret doesn't have to be filled with zeros
fn symetric_karatsuba<T: Digit>(ret: &mut [T], x: &[T], y: &[T], buff: &mut [T]) {
    // Early exit
    if exit_karatsuba(x.len()) {
        schoolbook_mul(ret, x, y);
        return;
    }

    let size = x.len();
    let half_size = (size >> 1) + (size % 2);
    let small_half_size = size >> 1;
    let size = half_size << 1;

    debug_assert_eq!(x.len(), y.len());
    debug_assert_eq!(ret.len(), x.len() + y.len());
    debug_assert!(buff.len() >= 2 * size);

    let (buff, sub_buff) = buff.split_at_mut(size);
    let (x0, x1) = x.split_at(half_size);
    let (y0, y1) = y.split_at(half_size);

    // Compute x0 + x1 and y0 + y1 in buff
    let (x_cross, y_cross) = buff.split_at_mut(half_size);
    x_cross.copy_from_slice(x0);
    y_cross.copy_from_slice(y0);
    let x_carry = add_assign(x_cross, x1);
    let y_carry = add_assign(y_cross, y1);

    // Compute z1 in ret
    let z1 = &mut ret[half_size..half_size + size + 2];
    symetric_karatsuba(&mut z1[..size], x_cross, y_cross, sub_buff);
    z1[size] = T::from(x_carry && y_carry);
    if x_carry {
        add_assign(&mut z1[half_size..], y_cross);
    }
    if y_carry {
        add_assign(&mut z1[half_size..], x_cross);
    }

    // Compute z2 in buff
    let z2 = &mut buff[..2 * small_half_size];
    symetric_karatsuba(z2, x1, y1, sub_buff);
    ret[half_size + size + 1..].copy_from_slice(&z2[half_size + 1..]);
    add_assign(&mut ret[size..], &z2[..half_size + 1]);
    sub_assign(&mut ret[half_size..], &z2);

    // Compute z0 in buff
    let z0 = &mut buff[..size];
    symetric_karatsuba(z0, x0, y0, sub_buff);
    ret[..half_size].copy_from_slice(&z0[..half_size]);
    add_assign(&mut ret[half_size..], &z0[half_size..]);
    sub_assign(&mut ret[half_size..], &z0);
}

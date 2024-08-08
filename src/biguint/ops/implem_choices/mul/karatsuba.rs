use crate::traits::Digit;

use super::super::add_assign;
use super::super::sub_assign;
use super::schoolbook_mul;

const DEBUG: bool = false;
fn debug(msg: String) {
    if DEBUG {
        println!("{:?}", msg);
    }
}
macro_rules! debug {
    ($($arg:tt)*) => {{
        debug(format! { $($arg)* })
    }};
}

// Below this number of digits, multiplication is schoolbook
#[cfg(debug_assertions)]
const KARATSUBA_INTERNAL_THRESHOLD: usize = 7;
#[cfg(debug_assertions)]
const KARATSUBA_EXTERNAL_THRESHOLD: usize = 7;

#[cfg(not(debug_assertions))]
const KARATSUBA_INTERNAL_THRESHOLD: usize = 20;
#[cfg(not(debug_assertions))]
const KARATSUBA_EXTERNAL_THRESHOLD: usize = 156;

pub(super) const KARATSUBA_EXTERNAL_THRESHOLD_SQUARED: usize =
    KARATSUBA_EXTERNAL_THRESHOLD * KARATSUBA_EXTERNAL_THRESHOLD;

pub(super) fn karatsuba<T: Digit>(rhs: &[T], lhs: &[T]) -> Vec<T> {
    if rhs.len() < lhs.len() {
        return karatsuba(lhs, rhs);
    }
    debug_assert!(rhs.len() >= lhs.len());

    let target_length = rhs.len().next_power_of_two();
    assert!(target_length < usize::MAX >> 1);

    let x = rhs;
    let mut y = lhs.to_vec();
    y.resize(rhs.len(), T::ZERO);
    debug_assert_eq!(x.len(), y.len());

    let mut ret = vec![T::ZERO; x.len() + y.len()];
    let mut buff = vec![T::ZERO; target_length << 1];
    symetric_karatsuba(&mut ret, &x, &y, &mut buff);
    ret
}

#[inline]
fn exit_karatsuba(size: usize) -> bool {
    size < KARATSUBA_INTERNAL_THRESHOLD
}

fn symetric_karatsuba<T: Digit>(ret: &mut [T], x: &[T], y: &[T], buff: &mut [T]) {
    // Early exit
    if exit_karatsuba(x.len()) {
        debug!("EXIT");
        schoolbook_mul(ret, x, y);
        return;
    }

    debug!("ret {:?}", ret.len());
    debug!("x {:?}", x.len());
    debug!("y {:?}", y.len());
    debug!("buff {:?}", buff.len());

    debug_assert_eq!(x.len(), y.len());
    debug_assert_eq!(ret.len(), x.len() + y.len());
    debug_assert!(buff.len() >= x.len() + y.len());

    let size = x.len();
    let half_size = (size >> 1) + (size % 2);
    let small_half_size = size >> 1;
    let size = half_size << 1;

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
    z1[size] = T::ZERO;
    z1[size + 1] = T::ZERO;
    if x_carry {
        debug_assert!(!add_assign(&mut z1[half_size..], y_cross))
    }
    if y_carry {
        debug_assert!(!add_assign(&mut z1[half_size..], x_cross))
    }
    z1[size] += T::from(x_carry && y_carry);

    // Compute z2 in buff
    let z2 = &mut buff[..2 * small_half_size];
    symetric_karatsuba(z2, x1, y1, sub_buff);
    debug_assert_eq!(ret[half_size + size + 1], T::ZERO);
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

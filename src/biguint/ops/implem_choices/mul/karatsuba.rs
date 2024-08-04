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
    debug!("ENTER");
    symetric_karatsuba(&mut ret, &x, &y, &mut buff);
    ret
}

/// Rule is :
/// - if both sizes are below threshold, exit
/// - if one of them is 1 (or 0), exit
#[inline]
fn exit_karatsuba(size: usize) -> bool {
    size < KARATSUBA_INTERNAL_THRESHOLD
}

fn symetric_karatsuba<T: Digit>(ret: &mut [T], x: &[T], y: &[T], buff: &mut [T]) {
    debug!("ret {:?}", ret.len());
    debug!("x {:?}", x.len());
    debug!("y {:?}", y.len());
    debug!("buff {:?}", buff.len());

    debug_assert_eq!(x.len(), y.len());
    debug_assert_eq!(ret.len(), x.len() + y.len());
    debug_assert!(buff.len() >= x.len() + y.len());

    let size = x.len();
    let half_size = (size >> 1) + (size % 2);
    debug!("size: {size}");
    debug!("half_size: {half_size}");

    // Early exit
    if exit_karatsuba(size) {
        debug!("EXIT");
        schoolbook_mul(ret, x, y);
        return;
    }

    let (x0, x1) = x.split_at(half_size);
    let (y0, y1) = y.split_at(half_size);
    assert!(x0.len() >= x1.len());
    assert!(y0.len() >= y1.len());

    // Compute (x0+x1) and (y0+y1), using ret as a buffer,
    // but specifically handle their last bit
    let (mut x_temp, mut y_temp) = ret.split_at_mut(size);
    x_temp = &mut x_temp[..half_size];
    y_temp = &mut y_temp[..half_size];
    debug_assert_eq!(x_temp.len(), half_size);
    debug_assert_eq!(y_temp.len(), half_size);
    x_temp.copy_from_slice(x0);
    y_temp.copy_from_slice(y0);
    debug!("FIRST ADDS");
    let x_carry = add_assign(x_temp, x1);
    let y_carry = add_assign(y_temp, y1);

    // compute z1 in a separate buffer
    // but specifically handle its last bit
    let (z1, new_buff) = buff.split_at_mut(half_size * 2);
    let mut z1_last_bit = T::ZERO;
    debug!("MIXED KARATSUBE");
    symetric_karatsuba(z1, x_temp, y_temp, new_buff);
    debug!("Z1 COMPLETION");
    if x_carry {
        z1_last_bit += T::from(add_assign(&mut z1[half_size..], &y_temp));
    }
    if y_carry {
        z1_last_bit += T::from(add_assign(&mut z1[half_size..], &x_temp));
    }
    z1_last_bit += T::from(x_carry && y_carry);

    // z0 and z2
    debug!("MAIN KARATSUBAS");
    symetric_karatsuba(&mut ret[..half_size * 2], x0, y0, new_buff);
    symetric_karatsuba(&mut ret[half_size * 2..], x1, y1, new_buff);

    // subtract z0 and z2 from z1
    debug!("SUBS");
    if sub_assign(z1, &ret[..half_size * 2]) {
        z1_last_bit -= T::ONE;
    }
    if sub_assign(z1, &ret[half_size * 2..]) {
        z1_last_bit -= T::ONE;
    }

    // add z1
    debug!("FINAL ADDS");
    if half_size + z1.len() >= ret.len() {
        debug_assert_eq!(z1_last_bit, T::ZERO);
    }
    add_assign(&mut ret[half_size..], z1);
    if z1_last_bit != T::ZERO {
        add_assign(&mut ret[half_size + z1.len()..], &[z1_last_bit]);
    }
}

#![cfg_attr(not(feature = "unsafe"), forbid(unsafe_code))]

use crate::traits::{Digit, DoubleDigit};

#[cfg(feature = "unsafe")]
use crate::traits::ToPtr;

/// Specialization of addition for x86_64 machines
#[cfg(all(feature = "unsafe", target_arch = "x86_64"))]
mod x86_64;

/// Current implementation of add_assign, returning the carry
/// Assumes rhs has at least the size of lhs
pub(crate) fn add_assign<T: Digit>(rhs: &mut [T], lhs: &[T]) -> bool {
    // Specifically for u32 digits, we accelerate by reinterpreting arrays as u64
    #[cfg(feature = "unsafe")]
    if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_mut_ptr::<u32>(), lhs.to_ptr::<u32>()) {
        let size = lhs.len() / 2;

        let carry: bool = unsafe {
            let rhs_64: &mut [u64] = std::slice::from_raw_parts_mut(rhs_cast.cast(), size);
            let lhs_64: &[u64] = std::slice::from_raw_parts(lhs_cast.cast(), size);
            add_assign(rhs_64, lhs_64)
        };

        return schoolbook_add_assign(&mut rhs[size*2..], &lhs[size*2..], carry);
    }

    #[allow(unused_mut)]
    let mut done = 0;
    #[allow(unused_mut)]
    let mut carry = false;

    #[cfg(all(feature = "unsafe", target_arch = "x86_64"))]
    'x86_u64_spec: {
        if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_mut_ptr::<u64>(), lhs.to_ptr::<u64>()) {
            debug_assert_eq!(T::NB_BITS, 64);

            let size = lhs.len().min(rhs.len());
            if size <= 5 {
                break 'x86_u64_spec;
            }
            let (c, d) = unsafe { x86_64::schoolbook_add_assign_x64_64(rhs_cast, lhs_cast, size - 5) };
            debug_assert!(size - d < 5);
            done += d;
            carry = c;
        }
    }

    schoolbook_add_assign(&mut rhs[done..], &lhs[done..], carry)
}

fn schoolbook_add_assign<T: Digit>(rhs: &mut [T], lhs: &[T], carry: bool) -> bool {
    let mut carry = if carry { T::ONE } else { T::ZERO };

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

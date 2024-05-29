#![cfg_attr(not(feature = "unsafe"), forbid(unsafe_code))]

use crate::traits::{Digit, DoubleDigit};

#[cfg(feature = "unsafe")]
use crate::traits::ToPtr;

/// Specialization of addition for x86_64 machines
#[cfg(all(feature = "unsafe", target_arch = "x86_64"))]
mod x86_64;

// #[cfg(feature = "unsafe")]
// fn add_assign_u64(rhs: *mut u64, lhs: *const u64, size: usize) -> bool {
//     #[allow(unused_mut)]
//     let mut done = 0;
//     #[allow(unused_mut)]
//     let mut carry = false;

//     #[cfg(target_arch = "x86_64")]
//     if size > 5 {
//         let (c, d) = unsafe { x86_64::schoolbook_add_assign_x64_64(rhs, lhs, size - 5) };
//         done += d;
//         carry = c;
//     }

//     assert!(lhs.is_aligned());

//     carry
// }

/// Current implementation of add_assign, returning the carry
/// Assumes rhs has at least the size of lhs
pub(crate) fn add_assign<T: Digit>(rhs: &mut [T], lhs: &[T]) -> bool {
    // Specifically for u32 digits, we accelerate by reinterpreting arrays as u64
    // #[cfg(feature = "unsafe")]
    // if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_mut_ptr::<u32>(), lhs.to_ptr::<u32>()) {
    //     let size = lhs.len() / 2;

    //     let carry: bool = unsafe {
    //         // assert_eq!(lhs_cast.align_offset(std::mem::align_of::<u64>()), 0);
    //         // assert_eq!(rhs_cast.align_offset(std::mem::align_of::<u64>()), 0);

    //         let rhs_64: &mut [u64] = std::slice::from_raw_parts_mut(rhs_cast.cast(), size);
    //         let lhs_64: &[u64] = std::slice::from_raw_parts(lhs_cast.cast(), size);
    //         add_assign(rhs_64, lhs_64)
    //     };

    //     return schoolbook_add_assign(&mut rhs[size*2..], &lhs[size*2..], carry);
    // }

    #[cfg(feature = "unsafe")]
    if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_mut_ptr::<u64>(), lhs.to_ptr::<u64>()) {

        let rhs_64: &mut[u64] = unsafe { std::slice::from_raw_parts_mut(rhs_cast, rhs.len()) };
        let lhs_64: &[u64] = unsafe { std::slice::from_raw_parts(lhs_cast, lhs.len()) };

        return unsafe { schoolbook_add_assign_u64(rhs_64, lhs_64, false) };
    }

    schoolbook_add_assign(rhs, lhs, false)
}

#[cfg(feature = "unsafe")]
unsafe fn schoolbook_add_assign_u64(rhs: &mut [u64], lhs: &[u64], _carry: bool) -> bool {
    let size = lhs.len().min(rhs.len());
    if size <= 5 {
        return schoolbook_add_assign(rhs, lhs, false);
    }
    
    #[cfg(target_arch = "x86_64")]
    let (carry, done) = x86_64::schoolbook_add_assign_x64_64(rhs.as_mut_ptr(), lhs.as_ptr(), size - 5);
    #[cfg(not(target_arch = "x86_64"))]
    let (carry, done) = (false, 0);

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

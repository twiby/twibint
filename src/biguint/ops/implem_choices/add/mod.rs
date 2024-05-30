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
        let carry: bool = unsafe { add_assign_u64(rhs_cast.cast(), size, lhs_cast.cast(), size) };
        return schoolbook_add_assign(&mut rhs[size*2..], &lhs[size*2..], carry);
    }

    #[cfg(feature = "unsafe")]
    if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_mut_ptr::<u64>(), lhs.to_ptr::<u64>()) {
        return unsafe { add_assign_u64(rhs_cast, rhs.len(), lhs_cast, lhs.len()) };
    }

    schoolbook_add_assign(rhs, lhs, false)
}

#[cfg(feature = "unsafe")]
unsafe fn add_assign_u64(rhs: *mut u64, rhs_size: usize, lhs: *const u64, lhs_size: usize) -> bool {
    debug_assert!(rhs_size >= lhs_size);

    #[cfg(target_arch = "x86_64")]
    let (carry, done) = x86_64::schoolbook_add_assign_x64_64(rhs, lhs, lhs_size);
    #[cfg(not(target_arch = "x86_64"))]
    let (carry, done) = (false, 0);

    schoolbook_add_assign_u64(rhs.wrapping_add(done), rhs_size - done, lhs.wrapping_add(done), lhs_size - done, carry)
}

/// Unsafe version operates directly in pointers
#[cfg(feature = "unsafe")]
unsafe fn schoolbook_add_assign_u64(mut rhs: *mut u64, mut rhs_size: usize, mut lhs: *const u64, mut lhs_size: usize, carry: bool) -> bool {
    debug_assert!(rhs_size >= lhs_size);
    rhs_size -= lhs_size;

    let mut carry = if carry { 1u64 } else { 0u64 };

    while lhs_size > 0 {
        let full = (*rhs as u128) + (*lhs as u128) + (carry as u128);
        (*rhs, carry) = full.split();
        rhs = rhs.offset(1);
        lhs = lhs.offset(1);
        lhs_size -= 1;
    }

    while rhs_size > 0 && carry > 0 {
        let full = (*rhs as u128) + (carry as u128);
        (*rhs, carry) = full.split();
        rhs = rhs.offset(1);
        rhs_size -= 1;
    }

    carry != 0u64
}

/// Safe version of add_assign uses generic slices
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

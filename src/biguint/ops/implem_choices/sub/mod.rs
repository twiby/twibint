#![cfg_attr(not(feature = "unsafe"), forbid(unsafe_code))]

use crate::traits::Digit;

#[cfg(feature = "unsafe")]
use crate::traits::ToPtr;

/// Specialization of subtraction for x86_64 machines
#[cfg(all(feature = "unsafe", target_arch = "x86_64"))]
mod x86_64;

/// Assumes rhs > lhs
pub(crate) fn sub_assign<T: Digit>(rhs: &mut [T], lhs: &[T]) -> bool {
    // Specifically for u32 digits, we accelerate by reinterpreting arrays as u64
    #[cfg(feature = "unsafe")]
    if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_mut_ptr::<u32>(), lhs.to_ptr::<u32>()) {
        let size = lhs.len() / 2;
        let carry: bool = unsafe { sub_assign_u64(rhs_cast.cast(), size, lhs_cast.cast(), size) };
        return schoolbook_sub_assign(&mut rhs[size*2..], &lhs[size*2..], carry);
    }

    #[cfg(feature = "unsafe")]
    if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_mut_ptr::<u64>(), lhs.to_ptr::<u64>()) {
        return unsafe { sub_assign_u64(rhs_cast, rhs.len(), lhs_cast, lhs.len()) };
    }

    schoolbook_sub_assign(rhs, lhs, false)
}

/// Tries hardware acceleration before classical algorithm
#[cfg(feature = "unsafe")]
unsafe fn sub_assign_u64(rhs: *mut u64, rhs_size: usize, lhs: *const u64, lhs_size: usize) -> bool {
    debug_assert!(rhs_size >= lhs_size);


    #[cfg(target_arch = "x86_64")]
    let (carry, done) = x86_64::schoolbook_sub_assign_x64_64(rhs, lhs, lhs_size);
    #[cfg(not(target_arch = "x86_64"))]
    let (carry, done) = (false, 0);

    schoolbook_sub_assign_u64(rhs.wrapping_add(done), rhs_size - done, lhs.wrapping_add(done), lhs_size - done, carry)
}

/// Unsafe version operates directly on pointers
#[cfg(feature = "unsafe")]
unsafe fn schoolbook_sub_assign_u64(mut rhs: *mut u64, mut rhs_size: usize, mut lhs: *const u64, mut lhs_size: usize, mut carry: bool) -> bool {
    debug_assert!(rhs_size >= lhs_size);
    rhs_size -= lhs_size;

    let mut carry_1: bool;
    let mut carry_2: bool;
    while lhs_size > 0 {
        (*rhs, carry_1) = (*rhs).overflowing_sub(*lhs);
        (*rhs, carry_2) = (*rhs).overflowing_sub(carry as u64);
        carry = carry_1 | carry_2;

        rhs = rhs.offset(1);
        lhs = lhs.offset(1);
        lhs_size -= 1;
    }

    while rhs_size > 0 && carry {
        (*rhs, carry) = (*rhs).overflowing_sub(carry as u64);
        rhs = rhs.offset(1);
        rhs_size -= 1;
    }

    carry
}

/// Safe version of sub_assign uses generic slices
fn schoolbook_sub_assign<T: Digit>(rhs: &mut [T], lhs: &[T], mut carry: bool) -> bool {
    let mut partial_carry_1: bool;
    let mut partial_carry_2: bool;
    for (a, b) in rhs.iter_mut().zip(lhs.iter()) {
        (*a, partial_carry_1) = a.overflowing_sub(*b);
        (*a, partial_carry_2) = a.overflowing_sub(T::from(carry));
        carry = partial_carry_1 | partial_carry_2;
    }

    for val in rhs.iter_mut().skip(lhs.len()) {
        (*val, carry) = val.overflowing_sub(T::from(carry));
    }

    carry
}

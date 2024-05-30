use crate::traits::{Digit, DoubleDigit, ToPtr};

mod karatsuba;

#[cfg(all(feature = "unsafe", target_arch = "x86_64"))]
mod x86_64;

/// adds the multiplicatin of rhs and lhs to ret
fn schoolbook_add_assign_mul<T: Digit>(ret: &mut [T], rhs: &[T], lhs: &[T]) {
    for i in 0..lhs.len() {
        single_digit_add_assign_mul(&mut ret[i..], rhs, lhs[i]);
    }
}

/// Compute the multiplication of `rhs` by `b` and adds the result to `ret`
fn single_digit_add_assign_mul<T: Digit>(ret: &mut [T], rhs: &[T], b: T) {
    debug_assert!(ret.len() > rhs.len());

    // Specifically for u32 digits, we accelerate by reinterpreting arrays as u64
    #[cfg(feature = "unsafe")]
    if let (Some(ret_cast), Some(rhs_cast), Some(b_cast)) = (
        ret.to_mut_ptr::<u32>(), 
        rhs.to_ptr::<u32>(), 
        b.to_ptr::<u32>()
    ) {
        let size = rhs.len() / 2;
        let carry: T;
        unsafe {
            let c_64 = single_digit_add_assign_mul_u64(ret_cast.cast(), rhs_cast.cast(), size, *b_cast as u64);
            let c_32 = c_64 as u32; // Guaranteed to fit, the carry will have the same size as b
            carry = *T::from_ptr::<u32>(&c_32).unwrap();
        }
        schoolbook_single_digit_add_assign_mul(&mut ret[size*2..], &rhs[size*2..], b, carry);
        return;
    }

    #[cfg(feature = "unsafe")]
    if let (Some(ret_cast), Some(rhs_cast), Some(b)) = (
        ret.to_mut_ptr::<u64>(), 
        rhs.to_ptr::<u64>(), 
        b.to_ptr::<u64>()
    ) {
        unsafe {
            let carry = single_digit_add_assign_mul_u64(ret_cast, rhs_cast, rhs.len(), *b);
            *ret_cast.wrapping_add(rhs.len()) = carry
        }
        return;
    }

    schoolbook_single_digit_add_assign_mul(ret, rhs, b, T::ZERO);
}

#[cfg(feature = "unsafe")]
unsafe fn single_digit_add_assign_mul_u64(ret: *mut u64, rhs: *const u64, rhs_size: usize, b: u64) -> u64 {
    #[cfg(target_arch = "x86_64")]
    let (carry, done) = x86_64::single_digit_add_assign_mul_x86_64(ret, rhs, b, rhs_size);
    #[cfg(not(target_arch = "x86_64"))]
    let (carry, done) = (false, 0);

    schoolbook_single_digit_add_assign_mul_u64(ret.wrapping_add(done), rhs.wrapping_add(done), rhs_size - done, b, carry)
}

#[cfg(feature = "unsafe")]
unsafe fn schoolbook_single_digit_add_assign_mul_u64(mut ret: *mut u64, mut rhs: *const u64, mut rhs_size: usize, b: u64, mut carry: u64) -> u64 {
    while rhs_size > 0 {
        let full = (*rhs as u128) * (b as u128) + (*ret as u128) + carry as u128;
        (*ret, carry) = full.split();
        ret = ret.offset(1);
        rhs = rhs.offset(1);
        rhs_size -= 1;
    }
    carry
}

fn schoolbook_single_digit_add_assign_mul<T: Digit>(ret: &mut [T], rhs: &[T], b: T, mut carry: T) {
    for (a, r) in rhs.iter().zip(ret.iter_mut()) {
        let full = a.to_double() * b.to_double() + r.to_double() + carry.to_double();
        (*r, carry) = full.split();
    }

    ret[rhs.len()] = carry;
}


/// Current implementation of multiplication
pub(crate) fn mul<T: Digit>(rhs: &[T], lhs: &[T]) -> Vec<T> {
    // Specifically for u32 digits, we accelerate multiplication by reinterpreting
    // arrays as u64 (and add a correction if length is odd)
    if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_ptr::<u32>(), lhs.to_ptr::<u32>()) {
        // Do the multiplication on u64 arrays (with half length)
        let mut ret: Vec<T> = unsafe {
            let rhs_size = rhs.len() / 2;
            let lhs_size = lhs.len() / 2;
            let rhs_64: &[u64] = std::slice::from_raw_parts(rhs_cast.cast(), rhs_size);
            let lhs_64: &[u64] = std::slice::from_raw_parts(lhs_cast.cast(), lhs_size);
            let mut ret_64: Vec<u64> = mul(rhs_64, lhs_64);
            let ret = Vec::<T>::from_raw_parts(
                ret_64.as_mut_ptr().cast(),
                ret_64.len() * 2,
                ret_64.capacity() * 2,
            );
            std::mem::forget(ret_64);
            ret
        };

        // if the array lengths are odd, then the multiplication is not over
        let additional_rhs_term = (rhs.len() % 2 == 1).then_some(rhs.last().unwrap());
        let additional_lhs_term = (lhs.len() % 2 == 1).then_some(lhs.last().unwrap());

        match (additional_rhs_term, additional_lhs_term) {
            (Some(&a), None) => {
                ret.push(T::ZERO);
                single_digit_add_assign_mul(&mut ret[rhs.len() - 1..], &lhs, a);
            }
            (None, Some(&b)) => {
                ret.push(T::ZERO);
                single_digit_add_assign_mul(&mut ret[lhs.len() - 1..], &rhs, b);
            }
            (Some(&a), Some(&b)) => {
                ret.push(T::ZERO);
                ret.push(T::ZERO);
                single_digit_add_assign_mul(&mut ret[rhs.len() - 1..], &lhs[..lhs.len() - 1], a);
                single_digit_add_assign_mul(&mut ret[lhs.len() - 1..], &rhs, b);
            }
            (None, None) => (),
        }

        return ret;
    }

    // Arrays are not big enough for karatsuba to be worth it
    if rhs.len() * lhs.len() < karatsuba::KARATSUBA_EXTERNAL_THRESHOLD_SQUARED {
        let mut ret = vec![T::ZERO; rhs.len() + lhs.len()];
        schoolbook_add_assign_mul(&mut ret, rhs, lhs);
        return ret;
    }

    karatsuba::karatsuba(rhs, lhs)
}

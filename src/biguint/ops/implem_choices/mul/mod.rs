#![cfg_attr(not(feature = "unsafe"), forbid(unsafe_code))]

use crate::traits::{Digit, DoubleDigit};

#[cfg(feature = "unsafe")]
use super::u32_ptrs_aligned;
#[cfg(feature = "unsafe")]
use crate::traits::ToPtr;

mod karatsuba;

#[cfg(all(feature = "unsafe", target_arch = "x86_64"))]
mod x86_64;

/// adds the multiplicatin of rhs and lhs to ret
fn schoolbook_mul<T: Digit>(ret: &mut [T], rhs: &[T], lhs: &[T]) {
    if lhs.len() == 0 {
        ret[..rhs.len()].fill(T::ZERO);
        return;
    }

    single_digit_mul(ret, rhs, lhs[0]);
    for i in 1..lhs.len() {
        single_digit_add_assign_mul(&mut ret[i..], rhs, lhs[i]);
    }
}

/// Compute the multiplication of `rhs` by `b` and writes the result to `ret`
fn single_digit_mul<T: Digit>(ret: &mut [T], rhs: &[T], b: T) {
    debug_assert!(ret.len() > rhs.len());

    // Specifically for u32 digits, we accelerate by reinterpreting arrays as u64
    #[cfg(feature = "unsafe")]
    if let (Some(ret_cast), Some(rhs_cast), Some(b_cast)) = (
        ret.to_mut_ptr::<u32>(),
        rhs.to_ptr::<u32>(),
        b.to_ptr::<u32>(),
    ) {
        // TODO: in this very case, alignment can be fixed via doing one step "manually"
        if u32_ptrs_aligned(ret_cast, rhs_cast) {
            let size = rhs.len() / 2;
            let carry: T;
            unsafe {
                let c_64 =
                    single_digit_mul_u64(ret_cast.cast(), rhs_cast.cast(), size, *b_cast as u64);
                let c_32 = c_64 as u32; // Guaranteed to fit, the carry will have the same size as b
                carry = *T::from_ptr::<u32>(&c_32).unwrap();
            }
            schoolbook_single_digit_mul(&mut ret[size * 2..], &rhs[size * 2..], b, carry);
        } else {
            schoolbook_single_digit_mul(ret, rhs, b, T::ZERO);
        }
        return;
    }

    #[cfg(feature = "unsafe")]
    if let (Some(ret_cast), Some(rhs_cast), Some(b)) = (
        ret.to_mut_ptr::<u64>(),
        rhs.to_ptr::<u64>(),
        b.to_ptr::<u64>(),
    ) {
        unsafe {
            let carry = single_digit_mul_u64(ret_cast, rhs_cast, rhs.len(), *b);
            *ret_cast.wrapping_add(rhs.len()) = carry
        }
        return;
    }

    schoolbook_single_digit_mul(ret, rhs, b, T::ZERO);
}

/// Compute the multiplication of `rhs` by `b` and adds the result to `ret`
fn single_digit_add_assign_mul<T: Digit>(ret: &mut [T], rhs: &[T], b: T) {
    debug_assert!(ret.len() > rhs.len());

    // Specifically for u32 digits, we accelerate by reinterpreting arrays as u64
    #[cfg(feature = "unsafe")]
    if let (Some(ret_cast), Some(rhs_cast), Some(b_cast)) = (
        ret.to_mut_ptr::<u32>(),
        rhs.to_ptr::<u32>(),
        b.to_ptr::<u32>(),
    ) {
        // TODO: in this very case, alignment can be fixed via doing one step "manually"
        if u32_ptrs_aligned(ret_cast, rhs_cast) {
            let size = rhs.len() / 2;
            let carry: T;
            unsafe {
                let c_64 = single_digit_add_assign_mul_u64(
                    ret_cast.cast(),
                    rhs_cast.cast(),
                    size,
                    *b_cast as u64,
                );
                let c_32 = c_64 as u32; // Guaranteed to fit, the carry will have the same size as b
                carry = *T::from_ptr::<u32>(&c_32).unwrap();
            }
            schoolbook_single_digit_add_assign_mul(
                &mut ret[size * 2..],
                &rhs[size * 2..],
                b,
                carry,
            );
        } else {
            schoolbook_single_digit_add_assign_mul(ret, rhs, b, T::ZERO);
        }
        return;
    }

    #[cfg(feature = "unsafe")]
    if let (Some(ret_cast), Some(rhs_cast), Some(b)) = (
        ret.to_mut_ptr::<u64>(),
        rhs.to_ptr::<u64>(),
        b.to_ptr::<u64>(),
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
unsafe fn single_digit_mul_u64(ret: *mut u64, rhs: *const u64, rhs_size: usize, b: u64) -> u64 {
    #[cfg(target_arch = "x86_64")]
    let (carry, done) = x86_64::single_digit_mul_x86_64(ret, rhs, b, rhs_size);
    #[cfg(not(target_arch = "x86_64"))]
    let (carry, done) = (0, 0);

    schoolbook_single_digit_mul_u64(
        ret.wrapping_add(done),
        rhs.wrapping_add(done),
        rhs_size - done,
        b,
        carry,
    )
}

#[cfg(feature = "unsafe")]
unsafe fn single_digit_add_assign_mul_u64(
    ret: *mut u64,
    rhs: *const u64,
    rhs_size: usize,
    b: u64,
) -> u64 {
    #[cfg(target_arch = "x86_64")]
    let (carry, done) = x86_64::single_digit_add_assign_mul_x86_64(ret, rhs, b, rhs_size);
    #[cfg(not(target_arch = "x86_64"))]
    let (carry, done) = (0, 0);

    schoolbook_single_digit_add_assign_mul_u64(
        ret.wrapping_add(done),
        rhs.wrapping_add(done),
        rhs_size - done,
        b,
        carry,
    )
}

#[cfg(feature = "unsafe")]
unsafe fn schoolbook_single_digit_mul_u64(
    mut ret: *mut u64,
    mut rhs: *const u64,
    mut rhs_size: usize,
    b: u64,
    mut carry: u64,
) -> u64 {
    while rhs_size > 0 {
        let full = (*rhs as u128) * (b as u128) + (carry as u128);
        (*ret, carry) = full.split();
        ret = ret.offset(1);
        rhs = rhs.offset(1);
        rhs_size -= 1;
    }
    carry
}

#[cfg(feature = "unsafe")]
unsafe fn schoolbook_single_digit_add_assign_mul_u64(
    mut ret: *mut u64,
    mut rhs: *const u64,
    mut rhs_size: usize,
    b: u64,
    mut carry: u64,
) -> u64 {
    while rhs_size > 0 {
        let full = (*rhs as u128) * (b as u128) + (*ret as u128) + (carry as u128);
        (*ret, carry) = full.split();
        ret = ret.offset(1);
        rhs = rhs.offset(1);
        rhs_size -= 1;
    }
    carry
}

fn schoolbook_single_digit_mul<T: Digit>(ret: &mut [T], rhs: &[T], b: T, mut carry: T) {
    for (a, r) in rhs.iter().zip(ret.iter_mut()) {
        let full = a.to_double() * b.to_double() + carry.to_double();
        (*r, carry) = full.split();
    }

    ret[rhs.len()] = carry;
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
    #[cfg(feature = "unsafe")]
    if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_ptr::<u32>(), lhs.to_ptr::<u32>()) {
        return unsafe { mul_u32(rhs, rhs_cast, lhs, lhs_cast) };
    }

    let mut ret = vec![T::ZERO; rhs.len() + lhs.len()];
    karatsuba::karatsuba(&mut ret, rhs, lhs);
    ret
}

/// Special code for efficiently handling u32 case
#[cfg(feature = "unsafe")]
unsafe fn mul_u32<T: Digit>(
    rhs: &[T],
    rhs_cast: *const u32,
    lhs: &[T],
    lhs_cast: *const u32,
) -> Vec<T> {
    // Do the multiplication on u64 arrays (with half length)
    let rhs_size = rhs.len() / 2;
    let lhs_size = lhs.len() / 2;

    if !u32_ptrs_aligned(rhs_cast, lhs_cast) {
        // TODO: in this very case, alignment can be fixed via doing one step "manually"
        let mut ret = vec![T::ZERO; rhs.len() + lhs.len()];
        schoolbook_mul(&mut ret, rhs, lhs);
        return ret;
    }

    debug_assert_eq!(rhs_cast.align_offset(std::mem::align_of::<u64>()), 0);
    debug_assert_eq!(lhs_cast.align_offset(std::mem::align_of::<u64>()), 0);

    let rhs_64: &[u64] = std::slice::from_raw_parts(rhs_cast.cast(), rhs_size);
    let lhs_64: &[u64] = std::slice::from_raw_parts(lhs_cast.cast(), lhs_size);
    let mut ret_64: Vec<u64> = mul(rhs_64, lhs_64);
    let mut ret = Vec::<T>::from_raw_parts(
        ret_64.as_mut_ptr().cast(),
        ret_64.len() * 2,
        ret_64.capacity() * 2,
    );
    std::mem::forget(ret_64);

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

    ret
}

/// We want to test misalignments here
#[cfg(all(test, feature = "unsafe"))]
mod tests {
    use super::mul;

    #[test]
    fn misaligned_mul() {
        let a = vec![u32::MAX, u32::MAX, u32::MAX];
        let b = vec![u32::MAX, u32::MAX, u32::MAX];
        let ret_full = vec![1, 0, 0, u32::MAX - 1, u32::MAX, u32::MAX];
        let ret_part = vec![1, 0, u32::MAX - 1, u32::MAX];

        assert_eq!(mul(&a[..], &b[..]), ret_full);
        assert_eq!(mul(&a[..2], &b[..2]), ret_part);
        assert_eq!(mul(&a[..2], &b[1..]), ret_part);
        assert_eq!(mul(&a[1..], &b[..2]), ret_part);
        assert_eq!(mul(&a[1..], &b[1..]), ret_part);
    }

    /// Randomize some tests to compare the result with num-bigint
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint(n: usize) {
        use num_bigint::BigUint;
        use rand::distributions::Standard;
        use rand::prelude::Distribution;
        use rand::{thread_rng, Rng};

        fn gen_n_random_values<T>(n: usize) -> Vec<T>
        where
            Standard: Distribution<T>,
        {
            let mut ret = Vec::<T>::with_capacity(n);
            for _ in 0..n {
                ret.push(thread_rng().gen::<T>());
            }
            ret
        }

        println!("STEP {n}");

        const SIZE1: usize = 100;
        const SIZE2: usize = 1000;
        let size_1 = SIZE1 + rand::thread_rng().gen_range(0..100);
        let size_2 = SIZE2 + rand::thread_rng().gen_range(0..100);

        let vec_a = gen_n_random_values::<u32>(size_1);
        let vec_b = gen_n_random_values::<u32>(size_2);

        let a = BigUint::new(vec_a.clone());
        let b = BigUint::new(vec_b.clone());
        let c = a * b;
        let should_get = c.to_u32_digits();

        let biguinta = crate::BigUint::from(vec_a.clone());
        let biguintb = crate::BigUint::from(vec_b.clone());
        let got_biguint = biguinta * biguintb;
        let got_main = got_biguint.val;
        let mut got_schoolbook = vec![0u32; size_1 + size_2];
        super::schoolbook_mul(&mut got_schoolbook, &vec_a, &vec_b);

        if should_get != got_main {
            assert_eq!(got_schoolbook, should_get);
            assert_eq!(should_get.len(), got_main.len());
            for (i, (a, b)) in should_get.iter().zip(got_main.iter()).enumerate() {
                if a > b {
                    println!("digit {i}, diff {}", a - b);
                } else if b > a {
                    println!("digit {i}, diff {}", b - a);
                }
            }
        }

        assert_eq!(should_get, got_main);
    }

    /// Randomize some tests to compare the result with num-bigint
    #[test]
    #[cfg(feature = "rand")]
    fn coherence_with_num_bigint_many() {
        for n in 0..100 {
            coherence_with_num_bigint(n);
        }
    }
}

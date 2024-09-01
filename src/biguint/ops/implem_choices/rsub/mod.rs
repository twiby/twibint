#![cfg_attr(not(feature = "unsafe"), forbid(unsafe_code))]

use crate::traits::Digit;

#[cfg(feature = "unsafe")]
use super::u32_ptrs_aligned;
#[cfg(feature = "unsafe")]
use crate::traits::ToPtr;

/// Specialization of subtraction for x86_64 machines
#[cfg(all(feature = "unsafe", target_arch = "x86_64"))]
mod x86_64;

/// Assumes len(rhs) == len(lhs)
pub(crate) fn rsub_assign<T: Digit>(rhs: &mut [T], lhs: &[T], rhs_len: usize) -> bool {
    // Specifically for u32 digits, we accelerate by reinterpreting arrays as u64
    // #[cfg(feature = "unsafe")]
    // if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_mut_ptr::<u32>(), lhs.to_ptr::<u32>()) {
    //     if u32_ptrs_aligned(rhs_cast, lhs_cast) {
    //         // Case pointers correctly aligned: pretend they are u64
    //         let size = lhs.len() / 2;
    //         let carry: bool =
    //             unsafe { sub_assign_u64(rhs_cast.cast(), size, lhs_cast.cast(), size) };
    //         return schoolbook_sub_assign(&mut rhs[size * 2..], &lhs[size * 2..], carry);
    //     } else {
    //         // Case pointers are misaligned: base safe algo
    //         return schoolbook_sub_assign(rhs, lhs, false);
    //     }
    // }

    // #[cfg(feature = "unsafe")]
    // if let (Some(rhs_cast), Some(lhs_cast)) = (rhs.to_mut_ptr::<u64>(), lhs.to_ptr::<u64>()) {
    //     return unsafe { sub_assign_u64(rhs_cast, rhs.len(), lhs_cast, lhs.len()) };
    // }

    schoolbook_rsub_assign(rhs, lhs, rhs_len, false)
}

// /// Tries hardware acceleration before classical algorithm
// #[cfg(feature = "unsafe")]
// unsafe fn sub_assign_u64(rhs: *mut u64, rhs_size: usize, lhs: *const u64, lhs_size: usize) -> bool {
//     debug_assert!(rhs_size >= lhs_size);

//     #[cfg(target_arch = "x86_64")]
//     let (carry, done) = x86_64::schoolbook_rsub_assign_x86_64(rhs, lhs, lhs_size);
//     #[cfg(not(target_arch = "x86_64"))]
//     let (carry, done) = (false, 0);

//     schoolbook_sub_assign_u64(
//         rhs.wrapping_add(done),
//         rhs_size - done,
//         lhs.wrapping_add(done),
//         lhs_size - done,
//         carry,
//     )
// }

// /// Unsafe version operates directly on pointers
// #[cfg(feature = "unsafe")]
// unsafe fn schoolbook_sub_assign_u64(
//     mut rhs: *mut u64,
//     mut rhs_size: usize,
//     mut lhs: *const u64,
//     mut lhs_size: usize,
//     mut carry: bool,
// ) -> bool {
//     debug_assert!(rhs_size >= lhs_size);
//     rhs_size -= lhs_size;

//     let mut carry_1: bool;
//     let mut carry_2: bool;
//     while lhs_size > 0 {
//         (*rhs, carry_1) = (*rhs).overflowing_sub(*lhs);
//         (*rhs, carry_2) = (*rhs).overflowing_sub(carry as u64);
//         carry = carry_1 | carry_2;

//         rhs = rhs.offset(1);
//         lhs = lhs.offset(1);
//         lhs_size -= 1;
//     }

//     while rhs_size > 0 && carry {
//         (*rhs, carry) = (*rhs).overflowing_sub(carry as u64);
//         rhs = rhs.offset(1);
//         rhs_size -= 1;
//     }

//     carry
// }

/// Safe version of rsub_assign uses generic slices
fn schoolbook_rsub_assign<T: Digit>(
    rhs: &mut [T],
    lhs: &[T],
    rhs_len: usize,
    mut carry: bool,
) -> bool {
    debug_assert!(rhs.len() == lhs.len());

    let mut partial_carry_1: bool;
    let mut partial_carry_2: bool;
    for (a, b) in rhs.iter_mut().zip(lhs.iter()).take(rhs_len) {
        (*a, partial_carry_1) = b.overflowing_sub(*a);
        (*a, partial_carry_2) = a.overflowing_sub(T::from(carry));
        carry = partial_carry_1 | partial_carry_2;
    }

    rhs[rhs_len..].copy_from_slice(&lhs[rhs_len..]);
    for val in rhs.iter_mut().skip(rhs_len) {
        (*val, carry) = val.overflowing_sub(T::from(carry));
    }

    carry
}

/// We want to test misalignments here
#[cfg(all(test, feature = "unsafe"))]
mod tests {
    use super::rsub_assign;

    #[test]
    fn misaligned_rsub() {
        let a = vec![1u32, 1, 1, 1];
        let b = vec![u32::MAX, u32::MAX, u32::MAX, 0u32];
        let b2 = vec![u32::MAX, u32::MAX, 0u32, 0u32];
        let ret_full = vec![2, 1, 1, 0];
        let ret_part = vec![2, 1, 0];

        let mut temp = b.clone();
        rsub_assign(&mut temp[..], &a[..], 3);
        assert_eq!(temp, ret_full);

        let mut temp = b2[..3].to_vec();
        rsub_assign(&mut temp, &a[..3], 2);
        assert_eq!(temp, ret_part);

        let mut temp = b[1..].to_vec();
        rsub_assign(&mut temp, &a[..3], 2);
        assert_eq!(temp, ret_part);

        let mut temp = b2[..3].to_vec();
        rsub_assign(&mut temp, &a[1..], 2);
        assert_eq!(temp, ret_part);

        let mut temp = b[1..].to_vec();
        rsub_assign(&mut temp, &a[1..], 2);
        assert_eq!(temp, ret_part);
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

        const SIZE: usize = 1000;
        let vec_a = gen_n_random_values::<u32>(SIZE);
        let vec_b = gen_n_random_values::<u32>(SIZE);

        let a = BigUint::new(vec_a.clone());
        let b = BigUint::new(vec_b.clone());
        let c = if &a < &b {
            let mut c = b.clone();
            c -= &a;
            c
        } else {
            let mut c = a.clone();
            c -= &b;
            c
        };

        let should_get = c.to_u32_digits();

        let got = if &a < &b {
            let mut got = vec_a.clone();
            rsub_assign(&mut got, &vec_b, vec_a.len());
            got
        } else {
            let mut got = vec_b.clone();
            rsub_assign(&mut got, &vec_a, vec_b.len());
            got
        };

        if should_get != got {
            assert_eq!(should_get.len(), got.len());
            for (i, (a, b)) in should_get.iter().zip(got.iter()).enumerate() {
                if a > b {
                    println!("digit {i}, diff {}", a - b);
                } else if b > a {
                    println!("digit {i}, diff {}", b - a);
                }
            }
        }

        assert_eq!(should_get, got);
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

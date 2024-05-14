use crate::traits::{Digit, DoubleDigit, ToPtr};

#[cfg(targe_arch = "x86_64")]
use std::arch::asm;

// TODO: some of these may be simd-able

#[cfg(target_arch = "x86_64")]
unsafe fn single_digit_add_assign_mul_x86_64(
    ret: *mut u64,
    rhs: *const u64,
    b: u64,
    len: usize,
) -> (u64, usize) {
    let mut carry = 0;
    let mut idx = 0;

    asm!(
        "3:",

        // Get data
        "mov rax, qword ptr[{a} + 8*{i}]",
        "mov {temp_a_2}, qword ptr [{a} + 8*{i} + 8]",
        "mov {temp_a_3}, qword ptr [{a} + 8*{i} + 16]",
        "mov {temp_a_4}, qword ptr [{a} + 8*{i} + 24]",
        "mov {temp_r_1}, qword ptr[{r} + 8*{i}]",
        "mov {temp_r_2}, qword ptr[{r} + 8*{i} + 8]",
        "mov {temp_r_3}, qword ptr[{r} + 8*{i} + 16]",
        "mov {temp_r_4}, qword ptr[{r} + 8*{i} + 24]",

        // Multiply
        "mul {b}",
        // Handle carry from previous
        "add rax, {c}",
        "adc rdx, 0",
        // Add to the current number
        "add rax, {temp_r_1}",
        "adc rdx, 0",
        // Get results
        "mov {c}, rdx",
        "mov qword ptr [{r} + 8*{i}], rax",

        // Next mul
        "mov rax, {temp_a_2}",
        "mul {b}",
        // Handle carry from previous
        "add rax, {c}",
        "adc rdx, 0",
        // Add to the current number
        "add rax, {temp_r_2}",
        "adc rdx, 0",
        // Get results
        "mov {c}, rdx",
        "mov qword ptr [{r} + 8*{i} + 8], rax",

        // Next mul
        "mov rax, {temp_a_3}",
        "mul {b}",
        // Handle carry from previous
        "add rax, {c}",
        "adc rdx, 0",
        // Add to the current number
        "add rax, {temp_r_3}",
        "adc rdx, 0",
        // Get results
        "mov {c}, rdx",
        "mov qword ptr [{r} + 8*{i} + 16], rax",

        // Next mul
        "mov rax, {temp_a_4}",
        "mul {b}",
        // Handle carry from previous
        "add rax, {c}",
        "adc rdx, 0",
        // Add to the current number
        "add rax, {temp_r_4}",
        "adc rdx, 0",
        // Get results
        "mov {c}, rdx",
        "mov qword ptr [{r} + 8*{i} + 24], rax",

        // Increment loop counter
        "add {i}, 4",
        "cmp {i}, {len}",
        "jle 3b",

        out("rax") _,
        out("rdx") _,
        b = in(reg) b,
        a = in(reg) rhs,
        r = in(reg) ret,
        c = inout(reg) carry,
        i = inout(reg) idx,
        len = in(reg) len,

        temp_r_1 = out(reg) _,
        temp_r_2 = out(reg) _,
        temp_r_3 = out(reg) _,
        temp_r_4 = out(reg) _,
        temp_a_2 = out(reg) _,
        temp_a_3 = out(reg) _,
        temp_a_4 = out(reg) _,
    );

    (carry, idx)
}

/// adds the multiplicatin of rhs and lhs to ret
fn schoolbook_add_assign_mul<T: Digit>(ret: &mut [T], rhs: &[T], lhs: &[T]) {
    for i in 0..lhs.len() {
        single_digit_add_assign_mul(&mut ret[i..], rhs, lhs[i]);
    }
}

fn single_digit_add_assign_mul<T: Digit>(ret: &mut [T], rhs: &[T], b: T) {
    #[allow(unused_mut)]
    let mut done = 0;
    let mut carry = T::ZERO;

    #[cfg(target_arch = "x86_64")]
    'x86_u64_spec: {
        if let (Some(ret_cast), Some(rhs_cast), Some(b)) = (
            ret.to_mut_ptr::<u64>(),
            rhs.to_ptr::<u64>(),
            b.to_ptr::<u64>(),
        ) {
            assert_eq!(T::NB_BITS, 64);

            let size = rhs.len();
            if size <= 4 {
                break 'x86_u64_spec;
            }
            let (c, d) =
                unsafe { single_digit_add_assign_mul_x86_64(ret_cast, rhs_cast, *b, size - 4) };
            debug_assert!(size - d < 4);
            done = d;
            carry = unsafe { *T::from_ptr::<u64>(&c).unwrap() };
        }

        if let (Some(ret_cast), Some(rhs_cast), Some(b)) = (
            ret.to_mut_ptr::<u32>(),
            rhs.to_ptr::<u32>(),
            b.to_ptr::<u32>(),
        ) {
            assert_eq!(T::NB_BITS, 32);

            let size = rhs.len() / 2;
            if size <= 4 {
                break 'x86_u64_spec;
            }
            let (c, d) = unsafe {
                single_digit_add_assign_mul_x86_64(
                    ret_cast.cast(),
                    rhs_cast.cast(),
                    *b as u64,
                    size - 4,
                )
            };
            debug_assert!(size - d < 4);
            done = d * 2;
            let c_32 = c as u32;
            carry = unsafe { *T::from_ptr::<u32>(&c_32).unwrap() };
        }
    }

    for (a, r) in rhs[done..].iter().zip(ret[done..].iter_mut()) {
        let full = a.to_double() * b.to_double() + r.to_double() + carry.to_double();
        (*r, carry) = full.split();
    }

    ret[rhs.len()] = carry;
}

fn karatsuba<const THRESHOLD: usize, T: Digit>(rhs: &[T], lhs: &[T]) -> Vec<T> {
    let target_length = rhs.len().max(lhs.len()).next_power_of_two();
    assert!(target_length < usize::MAX >> 1);

    let mut x = rhs.to_vec();
    let mut y = lhs.to_vec();
    x.resize(target_length, T::ZERO);
    y.resize(target_length, T::ZERO);

    let mut ret = vec![T::ZERO; target_length << 1];
    let mut buff = vec![T::ZERO; target_length << 1];
    _karatsuba::<THRESHOLD, _>(&mut ret, &x, &y, &mut buff);
    ret.resize(rhs.len() + lhs.len(), T::ZERO);
    ret
}
fn _karatsuba<const THRESHOLD: usize, T: Digit>(
    ret: &mut [T],
    rhs: &[T],
    lhs: &[T],
    buff: &mut [T],
) {
    debug_assert!(rhs.len() == lhs.len());
    debug_assert!(rhs.len().is_power_of_two());
    debug_assert_eq!(ret.len(), 2 * rhs.len());
    debug_assert_eq!(buff.len(), 2 * rhs.len());

    let size = rhs.len();
    let half_size = size >> 1;

    // Early exit
    if size < THRESHOLD {
        schoolbook_add_assign_mul(ret, rhs, lhs);
        return;
    }

    let (x0, x1) = rhs.split_at(half_size);
    let (y0, y1) = lhs.split_at(half_size);

    // Compute (x0+x1) and (y0+y1), using ret as a buffer,
    // but specifically handle their last bit
    let (x_temp, y_temp) = ret[..size].split_at_mut(half_size);
    x_temp.copy_from_slice(x0);
    y_temp.copy_from_slice(y0);
    let x_carry = super::add_assign(x_temp, x1);
    let y_carry = super::add_assign(y_temp, y1);

    // compute z1 in a separate buffer
    // but specifically handle its last bit
    let (z1, new_buff) = buff.split_at_mut(size);
    let mut z1_last_bit = T::ZERO;
    _karatsuba::<THRESHOLD, _>(&mut z1[..size], x_temp, y_temp, new_buff);
    if x_carry {
        z1_last_bit += T::from(super::add_assign(&mut z1[half_size..], &y_temp));
    }
    if y_carry {
        z1_last_bit += T::from(super::add_assign(&mut z1[half_size..], &x_temp));
    }
    z1_last_bit += T::from(x_carry && y_carry);

    // z0 and z2
    ret[..size].fill(T::ZERO);
    new_buff.fill(T::ZERO);
    _karatsuba::<THRESHOLD, _>(&mut ret[..size], x0, y0, new_buff);
    new_buff.fill(T::ZERO);
    _karatsuba::<THRESHOLD, _>(&mut ret[size..], x1, y1, new_buff);

    // subtract z0 and z2 from z1
    let mut partial_carry_1: bool;
    let mut partial_carry_2: bool;
    let mut partial_carry_3: bool;
    let mut carry = T::ZERO;
    for i in 0..size {
        (z1[i], partial_carry_1) = z1[i].overflowing_sub(ret[i + size]);
        (z1[i], partial_carry_2) = z1[i].overflowing_sub(ret[i]);
        (z1[i], partial_carry_3) = z1[i].overflowing_sub(carry);
        carry = T::from(partial_carry_1) + T::from(partial_carry_2) + T::from(partial_carry_3);
    }
    (z1_last_bit, _) = z1_last_bit.overflowing_sub(carry);

    // add z1
    super::add_assign(&mut ret[half_size..], z1);
    super::add_assign(&mut ret[half_size + size..], &[z1_last_bit]);
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

const KARATSUBA_EXTERNAL_THRESHOLD_SQUARED: usize =
    KARATSUBA_EXTERNAL_THRESHOLD * KARATSUBA_EXTERNAL_THRESHOLD;

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
    if rhs.len() * lhs.len() < KARATSUBA_EXTERNAL_THRESHOLD_SQUARED {
        let mut ret = vec![T::ZERO; rhs.len() + lhs.len()];
        schoolbook_add_assign_mul(&mut ret, rhs, lhs);
        return ret;
    }

    karatsuba::<KARATSUBA_INTERNAL_THRESHOLD, _>(rhs, lhs)
}
